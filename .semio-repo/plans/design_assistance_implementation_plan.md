# Design Assistance System — Implementation Plan

Implement a Go binary **assistant** that validates a single **design** (source: `semio` over MCP) against multiple compliance **targets** via per-target **translator** and **validator** pairs. The assistant aggregates results into a **report** and, when violations exist, invokes a **changer agent** that edits the design through the design MCP server until the design becomes compliant or iteration limits are reached.

Target examples in this plan:
- `BerlinBuildingCode`
- `RoomProgram`

---

## 1. System Responsibilities

### 1.1 Assistant (Go binary)
- Reads the current design (and revision/snapshot if supported) from the **design MCP server**
- For each target, concurrently:
  - runs the target’s **translator agent**
  - pipes translator output into the target’s **validator binary**
  - parses validator output into a uniform internal structure
- Aggregates per-target results into a single **assistant report**
- If the report contains violations:
  - enriches violations with **measures** derived from the target ruleset
  - builds a deterministic **change plan**
  - invokes the **changer agent** (which uses MCP analyze+change tools)
- Repeats validate → change → validate until exit condition

### 1.2 Translators (agents)
- Convert the source design (`semio`) into the target-specific input format required by the validator
- Must be deterministic for a given design revision and configuration

### 1.3 Validators (binaries)
- Read translator output from `stdin`
- Emit a structured JSON result to `stdout` describing:
  - rule status: compliant / violated / not-applicable / error
  - clause status and any actual/should values
  - optional location references back to design elements

### 1.4 Changer (agent)
- Receives the assistant report and a change plan
- Uses design MCP:
  - **analyze tools** to locate and understand the context of the violation
  - **change tools** to perform modifications
- Produces:
  - whether any changes were applied
  - optional new design revision/snapshot id
  - a change log that can be embedded into the assistant report

---

## 2. Execution Semantics

### 2.1 Concurrency
- **One goroutine per target**
- Each goroutine performs:
  1) translator execution  
  2) validator execution using translator output as stdin  
  3) parse result into `TargetResult`

The assistant **waits for all targets** (or times out) before aggregating.

### 2.2 Piping translator → validator
Implementation choices:
- **Buffered pipe** (recommended default): capture translator stdout to memory (with size limit) or temp file, then pass to validator stdin.
- **Streaming pipe** (optional): connect translator stdout directly to validator stdin with `io.Pipe` and run both processes concurrently.

This plan uses buffered-by-default to match the described “translator returns, then validator starts” behavior while still “piping” translator output into validator.

### 2.3 Cancellation and timeouts
- Root context: `--run-timeout`
- Per-target:
  - `--translate-timeout`
  - `--validate-timeout`
- If one target fails:
  - default: mark target as error but still aggregate other targets
  - if target is `Required`, fail the iteration (no changer call)

---

## 3. Domain Model and Report Format

### 3.1 Concepts
- **Rule**: compliance requirement
- **Clause**: atomic check for a rule
- **Measure**: prescriptive remediation primitive that maps violations to MCP tool sequences
- **Violation**: generated when a rule is violated (often per clause) and enriched with measures

### 3.2 Normalized (assistant) report
The assistant emits one report per iteration.

```json
{
  "runId": "uuid",
  "design": {
    "source": "semio",
    "designId": "semio:project:…",
    "revision": "…"
  },
  "iteration": 1,
  "generatedAt": "RFC3339",
  "targets": [
    {
      "target": "BerlinBuildingCode",
      "status": "ok",
      "validatorVersion": "…",
      "rules": [
        {
          "id": "staircase-in-separate-stairwell",
          "status": "violated",
          "clauses": [
            { "id": "…", "status": "violated", "actual": "…", "should": "…" }
          ],
          "measures": ["area-decrease"],
          "violations": [
            {
              "id": "BerlinBuildingCode:staircase-in-separate-stairwell:…",
              "target": "BerlinBuildingCode",
              "ruleId": "staircase-in-separate-stairwell",
              "clauseId": "…",
              "status": "violated",
              "actual": "…",
              "should": "…",
              "location": { "designNodeId": "…" },
              "measures": [
                {
                  "id": "area-decrease",
                  "platform": "semio",
                  "analyzeTools": ["evaluate_quality"],
                  "changeTools": ["remove_piece","change_piece_type","update_type"],
                  "instructions": "Decrease the floor area by removing pieces or changing the type of existing pieces."
                }
              ]
            }
          ]
        }
      ]
    }
  ],
  "violations": [ /* flattened across targets for convenience */ ],
  "changeLog": {
    "applied": true,
    "steps": [ /* per measure/tool call */ ]
  },
  "summary": {
    "targetsOk": 2,
    "targetsError": 0,
    "violationsTotal": 3
  }
}
```

---

## 4. Inputs the Assistant Loads

### 4.1 Rulesets (per target)
A ruleset maps **rule → measures** (and optionally clause metadata).

- RoomProgram ruleset: `RoomProgram.json`
- BerlinBuildingCode ruleset: `BerlinBuildingCode.json`

Ruleset schema (minimum):
```json
{
  "id": "room-program",
  "rules": [
    {
      "id": "enough-buro-space",
      "description": "…",
      "measures": ["increase-floor-area"]
    }
  ]
}
```

### 4.2 Measure catalog (per design platform)
The measure catalog maps **measure → tool plan** by platform (`semio`, etc.). The changer executes these tool plans.

Minimum schema:
```json
{
  "id": "area-increase",
  "description": "Increase the floor area",
  "targets": [
    {
      "id": "semio",
      "tools": {
        "analyze": ["evaluate_quality"],
        "change": ["add_piece","add_connection","change_piece_type","update_type"]
      },
      "instructions": "Increase the floor area by either adding pieces or changing the type of existing pieces. Connect the new pieces to the existing pieces."
    }
  ]
}
```

### 4.3 Normalization maps (required with provided samples)
Two normalizations are required for robust operation:

#### Measure ID normalization
Rulesets reference measure IDs that differ from the catalog IDs. Use a map:
```json
{
  "increase-floor-area": "area-increase",
  "decrease-floor-area": "area-decrease",
  "remove-storey": "storey-removal"
}
```

#### Rule ID normalization
Validator outputs (sample report) can use rule IDs that differ from ruleset rule IDs. Use a map per target:
```json
{
  "BerlinBuildingCode": {
    "staircase-in-separate-stairwell": "staircase-located"
  }
}
```

> Best long-term fix: align IDs across validator and ruleset, but the assistant should tolerate mismatches via aliases.

---

## 5. Measure-Driven Remediation (Changer Contract)

### 5.1 Measure selection
For each violated rule:
1. Map validator rule id → ruleset rule id (rule alias)
2. Read ruleset measures[]
3. Normalize measure IDs to catalog IDs (measure alias)
4. Look up measure entry for `platform=semio`
5. Execute `tools.analyze[]` then `tools.change[]`

### 5.2 semio measure table (from provided catalog)
| Measure ID | Intent | semio analyze tools | semio change tools |
|---|---|---|---|
| `area-increase` | Increase the floor area | `evaluate_quality` | `add_piece, add_connection, change_piece_type, update_type` |
| `area-decrease` | Decrease the floor area | `evaluate_quality` | `remove_piece, change_piece_type, update_type` |
| `storey-removal` | Remove a storey | `` | `remove_piece` |

### 5.3 Changer I/O contract
Recommended:
- assistant calls changer as a process
- assistant writes an **enriched report** + **change plan** to changer `stdin`
- changer returns `stdout` JSON:

```json
{
  "modified": true,
  "newRevision": "rev-18",
  "appliedMeasures": ["area-increase"],
  "toolCalls": [
    { "tool":"evaluate_quality", "args":{}, "ok":true },
    { "tool":"add_piece", "args":{}, "ok":true }
  ],
  "notes": ["…"]
}
```

---

## 6. Orchestrator Algorithm

### 6.1 One validation iteration (`runOnce`)
1. Read design metadata (id, optional revision)
2. For each target concurrently:
   - run translator
   - run validator with translator output piped to stdin
   - parse validator JSON into `TargetRulesResult`
3. Aggregate into `Report`
4. Enrich violated rules with measures and tool plans
5. Write artifacts:
   - `report.iter-N.json`
   - `report.iter-N.md`

### 6.2 Full loop (`runLoop`)
```
for iter in 1..MaxIterations:
  report = runOnce()
  if report.Violations == 0: exit compliant
  changeResult = runChanger(report)
  if !changeResult.modified: exit stuck
  update designRef revision (if provided)
exit budget_exhausted
```

### 6.3 Exit conditions
- **compliant**: no violations
- **stuck**: changer reports no modifications (or repeated identical report)
- **budget_exhausted**: max iterations reached
- **fatal_error**: MCP unavailable or required target failed

---

## 7. Example: Sample Report Interpretation

Sample validator rule statuses:

- `staircase-in-separate-stairwell`: **violated**
- `enough-buro-space`: **violated**
- `building-height-limit`: **compliant**
- `smoething-to-be-irrelevant`: **not-applicable**

Assistant behavior:
- Only rules with `status == "violated"` produce `violations[]` and remediation steps.
- `not-applicable` rules are recorded for audit but never remediated.
- Rules with `actual/should` are preserved verbatim and surfaced in `violation.actual` and `violation.should`.

---

## 8. Implementation Blueprint (Single File)

All code in:
```
go/assistant/main.go
```

### 8.1 Sections (recommended ordering)
1. Imports + constants
2. Config + CLI flags
3. Types (rulesets, measure catalog, report)
4. Loading helpers (JSON load, alias normalization)
5. Process execution helpers (translator/validator/changer)
6. Orchestrator (`runOnce`, `runLoop`)
7. Report writing (JSON + MD)
8. `main()`

### 8.2 Core structs (minimum set)
- `TargetSpec`
- `DesignRef`
- `Ruleset`, `RulesetRule`
- `MeasureCatalogEntry`, `MeasureTargetPlan`
- `ValidatorOutput` (raw)
- `TargetResult` (parsed + normalized)
- `Violation` (normalized, enriched)
- `Report`
- `ChangePlan`, `ChangeStep`
- `ChangerResult`

### 8.3 Core functions (minimum set)
Loading:
- `loadRuleset(path string) (Ruleset, error)`
- `loadMeasureCatalog(path string) (map[string]MeasureCatalogEntry, error)`
- `loadAliases(path string) (map[string]string, error)` (optional)
Normalization:
- `normalizeRuleID(target, ruleID string) string`
- `normalizeMeasureID(measureID string) string`
Orchestration:
- `runTranslator(ctx, spec, designRef) ([]byte, error)`
- `runValidator(ctx, spec, translated []byte) (ValidatorOutput, error)`
- `enrichTargetResult(target, validatorOutput, ruleset, catalog) TargetResult`
- `aggregate(runId, designRef, iter, targetResults) Report`
Loop:
- `buildChangePlan(report) ChangePlan`
- `runChanger(ctx, report, plan) (ChangerResult, error)`
- `runOnce(ctx, iter, designRef) (Report, error)`
- `runLoop(ctx, designRef) (Report, error)`
Artifacts:
- `writeReportJSON(path, report)`
- `writeReportMD(path, report)`

### 8.4 Process execution details
- Use `exec.CommandContext`
- Capture:
  - `stdout` (primary output)
  - `stderr` (for logs and error propagation)
- Size limits:
  - translator output max bytes (`--max-translation-bytes`)
  - validator output max bytes (`--max-validator-bytes`)

---

## 9. Testing Plan

### 9.1 Unit tests (logic)
- Alias resolution:
  - rule id mapping per target
  - measure id mapping to catalog
- Enrichment correctness:
  - violated rule attaches measures and semio tool plans
- Report stability:
  - stable violation IDs across iterations for same (target, rule, clause)

### 9.2 Integration tests (process)
- Fake translator that prints deterministic JSON
- Fake validator that returns sample report
- Mock MCP (for changer) that records tool calls
- Verify:
  - concurrency works
  - report artifacts created
  - changer invoked only when violations exist

---

## Appendix A — Provided JSON Inputs (Reference)

### A.1 Measure catalog
```json
[
  {
    "id": "area-increase",
    "description": "Increase the floor area",
    "targets": [
      {
        "id": "semio",
        "description": "Increase the floor area by eiter adding pieces or changing the type of existing pieces. Connect the new pieces to the existing pieces.",
        "tools": {
          "analyze": [
            "evaluate_quality"
          ],
          "change": [
            "add_piece",
            "add_connection",
            "change_piece_type",
            "update_type"
          ]
        },
        "instructions": "Increase the floor area by eiter adding pieces or changing the type of existing pieces. Connect the new pieces to the existing pieces."
      },
      {
        "id": "archicad",
        "description": "Increase the floor area by eiter moving or stretching walls.",
        "tools": {
          "analyze": [
            "get_zone_area"
          ],
          "change": [
            "move_wall",
            "stretch_wall"
          ]
        },
        "instructions": "Increase the floor area by eiter moving or stretching walls."
      }
    ]
  },
  {
    "id": "area-decrease",
    "description": "Decrease the floor area",
    "targets": [
      {
        "id": "semio",
        "description": "Decrease the floor area by removing pieces or changing the type of existing pieces.",
        "tools": {
          "analyze": [
            "evaluate_quality"
          ],
          "change": [
            "remove_piece",
            "change_piece_type",
            "update_type"
          ]
        },
        "instructions": "Decrease the floor area by removing pieces or changing the type of existing pieces."
      },
      {
        "id": "archicad",
        "description": "Decrease the floor area by removing walls.",
        "tools": {
          "analyze": [
            "get_zone_area"
          ],
          "change": [
            "remove_wall"
          ]
        },
        "instructions": "Decrease the floor area by removing walls."
      }
    ]
  },
  {
    "id": "storey-removal",
    "description": "Remove a storey",
    "targets": [
      {
        "id": "semio",
        "description": "Remove the storey by removing the pieces that are or form the roof.",
        "tools": {
          "analyze": [],
          "change": [
            "remove_piece"
          ]
        },
        "instructions": "Remove the storey by removing the pieces that are or form the roof."
      }
    ]
  }
]
```

### A.2 Sample validator report
```json
{
  "rules": [
    {
      "id": "staircase-in-separate-stairwell",
      "status": "violated",
      "clauses": [
        {
          "id": "not-external-staircase",
          "status": "compliant"
        },
        {
          "id": "not-in-building-classes-1-and-2",
          "status": "compliant"
        },
        {
          "id": "not-connecting-two-storeys-within-same-usage-unit-with-total-gross-floor-area-of-no-more-than-200-m²-if-different-escape-route-can-be-reached-on-each-storey",
          "status": "violated"
        }
      ]
    },
    {
      "id": "enough-buro-space",
      "status": "violated",
      "clauses": [
        {
          "id": ">300m²",
          "actual": "255m²",
          "should": "300m²"
        }
      ]
    },
    {
      "id": "building-height-limit",
      "status": "compliant",
      "clauses": [
        {
          "id": "<21m",
          "status": "compliant",
          "actual": "20.5m"
        }
      ]
    },
    {
      "id": "smoething-to-be-irrelevant",
      "status": "not-applicable",
      "clauses": [
        {
          "id": "<21m",
          "status": "compliant",
          "actual": "20.5m"
        }
      ]
    }
  ]
}
```

### A.3 RoomProgram ruleset
```json
{
  "id": "room-program",
  "rules": [
    {
      "id": "enough-buro-space",
      "description": "At least 300m² buro spaces.",
      "measures": [
        "increase-floor-area"
      ]
    }
  ]
}
```

### A.4 BerlinBuildingCode ruleset
```json
{
  "id": "berlin-building-code",
  "rules": [
    {
      "id": "staircase-located",
      "description": "Must be located in a separate stairwell",
      "clauses": [
        {
          "id": "not-external-staircase",
          "description": "Not an external staircase"
        },
        {
          "id": "not-in-building-classes-1-and-2",
          "description": "Not in buildings of building classes 1 and 2"
        }
      ],
      "measures": [
        "decrease-floor-area"
      ]
    },
    {
      "id": "building-height-limit",
      "description": "The building height must be less than 21m.",
      "measures": [
        "remove-storey"
      ]
    }
  ]
}
```

