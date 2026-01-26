# Design Assistance System — Refactor Plan v2 (Measures + Sample Report + Rulesets)

This revision **adjusts the architecture and refactor plan** to incorporate:

- A **measure catalog** that defines how generic actions are executed via **design MCP tools** (example target: `semio`)
- A **sample aggregated report** structure produced by validators and the assistant
- Two **sample rulesets** (`BerlinBuildingCode`, `RoomProgram`) that bind **rules → measures**

> Goal: make the *changer* implementation **measure-driven**: it fixes violations by selecting measures prescribed by the violated rule(s), then executing the corresponding MCP tool sequences.

---

## 0. Inputs incorporated

### Measure catalog (sample)

Supported measure IDs in the sample catalog:

- area-increase, area-decrease, storey-removal

Excerpt (verbatim JSON for traceability):

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

### Sample aggregated report (sample)

Observed rule statuses in the sample report:

- staircase-in-separate-stairwell=violated; enough-buro-space=violated; building-height-limit=compliant; smoething-to-be-irrelevant=not-applicable

Full sample report:

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

### Rulesets (samples)

**RoomProgram** ruleset:

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

**BerlinBuildingCode** ruleset:

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

---

## 1. Key adjustment: Measure-driven remediation

### 1.1 Measure as the “bridge” between violations and MCP actions

A *measure* is no longer just an explanation — it becomes a **prescriptive remediation primitive**:

- **Ruleset** assigns `measures: ["..."]` to a rule.
- **Measure catalog** provides per-target tool sequences:
  - `tools.analyze[]` (read-only inspection / evaluation)
  - `tools.change[]` (mutations)
  - plus human-readable `instructions`

The **changer** consumes:
1. Violations from the aggregated report
2. Ruleset mappings (rule → measures)
3. Measure catalog entries (measure → MCP tool plan for `semio`)
4. Design context from MCP analyze tools
5. Performs changes using MCP change tools

### 1.2 Important mismatch to resolve: naming/IDs

In the provided samples, the rulesets refer to measures like:

- `"increase-floor-area"`
- `"decrease-floor-area"`
- `"remove-storey"`

But the measure catalog uses IDs like:

- `"area-increase"`
- `"area-decrease"`
- `"storey-removal"`

**Refactor requirement:** introduce a **measure ID normalization layer**:

- either update rulesets to match the measure catalog IDs
- or add an alias map in the assistant/changer configuration

Example alias map:

```json
{
  "increase-floor-area": "area-increase",
  "decrease-floor-area": "area-decrease",
  "remove-storey": "storey-removal"
}
```

---

## 2. Updated architecture (what changes)

### What stays the same

- `assistant` fans out concurrently to translator+validator pairs per target.
- Translation output is piped to validator.
- Validator outputs are aggregated into one report.
- If violations exist, `changer` edits the design via MCP and requests re-validation.

### What changes

1. **Assistant aggregates at “rule/clauses” level**
   - It must preserve:
     - rule id, rule status
     - per-clause status / actual / should (where provided)
2. **Assistant enriches violations with measures**
   - It uses the ruleset to attach the measures for any violated rule.
3. **Changer uses measures as the action plan**
   - It executes MCP tool sequences defined in the measure catalog.
4. **Auditability**
   - Each applied change is associated with:
     - target, ruleId, clauseId
     - selected measure(s)
     - MCP tool calls executed
     - before/after design revision ID (if available)

---

## 3. Updated data model

### 3.1 Normalize the report format into an “assistant report”

Your sample report is rule-centric. For the changer to be effective, the assistant should convert it to an enriched structure:

```json
{
  "designId": "semio:project:abc",
  "iteration": 1,
  "targets": [
    {
      "target": "BerlinBuildingCode",
      "rules": [
        {
          "id": "staircase-in-separate-stairwell",
          "status": "violated",
          "clauses": [
            { "id": "not-connecting-two-storeys-within-same-usage-unit-with-total-gross-floor-area-of-no-more-than-200-m²-if-different-escape-route-can-be-reached-on-each-storey", "status": "violated" }
          ],
          "measures": ["area-decrease"],        // after normalization/aliasing
          "violations": [
            {
              "ruleId": "staircase-in-separate-stairwell",
              "clauseId": "not-connecting-two-storeys-within-same-usage-unit-with-total-gross-floor-area-of-no-more-than-200-m²-if-different-escape-route-can-be-reached-on-each-storey",
              "message": "Clause violated",
              "measures": ["area-decrease"]
            }
          ]
        }
      ]
    }
  ]
}
```

### 3.2 Why “violations” should exist even if validators only return rule/clauses

Even when the validator output is only:

- rule status
- clause status
- actual/should

… the assistant should synthesize a normalized **Violation** object because:

- the changer needs a uniform input type
- the assistant needs stable IDs for deduping across iterations
- output tooling (reports, dashboards) benefits from consistent structure

---

## 4. Updated changer strategy (measure-driven)

### 4.1 High-level algorithm

For each iteration:

1. **Extract actionable violations**
   - Consider only rules where `status == "violated"`
2. **Attach measures via ruleset**
   - Lookup `(target.rules[].id == violatedRuleId) → measures[]`
3. **Normalize/alias measures**
4. **Create a measure execution plan**
   - For each measure, select the `targets[]` entry for `id == "semio"`
5. **Run MCP analyze tools first**
   - Use the catalog’s `tools.analyze[]` as *minimum required reads*
6. **Apply MCP change tools**
   - Execute `tools.change[]` in a transaction
7. **Record change log**
   - measure id, tool calls, changed node IDs, errors
8. **Stop early if stuck**
   - no diff in design revision OR repeated failures

### 4.2 Example: fixing `enough-buro-space` (RoomProgram)

From the sample report, the rule `enough-buro-space` is violated with:

- `actual: 255m²`
- `should: 300m²`

Ruleset indicates a measure: `increase-floor-area` (normalized to `area-increase`).

Measure catalog for `area-increase` on `semio` prescribes:

- Analyze: `evaluate_quality`
- Change: `add_piece`, `add_connection`, `change_piece_type`, `update_type`

**Concrete plan the changer should attempt:**

1. Analyze:
   - `evaluate_quality` (to identify weak areas / candidate expansion zones)
2. Change:
   - add new floor pieces (e.g., extend a room zone)
   - connect new pieces to existing pieces
   - optionally change piece types to “buro” / office-eligible
   - update types / metadata
3. Re-check:
   - request re-validation; confirm `actual >= should`

### 4.3 Example: fixing staircase rule (BerlinBuildingCode)

Sample report shows `staircase-in-separate-stairwell` violated due to a specific clause.
Ruleset attaches `decrease-floor-area` (normalized to `area-decrease`).

Measure catalog for `area-decrease` on `semio` prescribes:

- Analyze: `evaluate_quality`
- Change: `remove_piece`, `change_piece_type`, `update_type`

**Concrete plan the changer should attempt:**

- Identify the areas contributing to the clause condition (MCP analyze)
- Reduce gross floor area or reclassify usage units via:
  - removing pieces
  - changing piece types
  - updating metadata/types

> Note: This is a **generic** action. If the clause requires “separate stairwell”, a richer catalog would include a staircase-specific measure (e.g., `add-stairwell`), but with current inputs the system will apply the prescribed generic remediation.

---

## 5. Updated assistant responsibilities

The assistant must now:

1. Load rulesets for targets (static files or via registry)
2. Load the measure catalog for the design platform (`semio`)
3. On each iteration:
   - run translators+validators concurrently
   - parse validator outputs into a uniform internal structure
   - enrich violated rules with measures from rulesets
   - normalize measure IDs via alias map
   - emit an enriched report for the changer

---

## 6. Refactor plan for `go/assistant/main.go` (updated)

This plan preserves the single-file constraint while introducing measure-driven remediation cleanly.

### 6.1 New config blocks

Add:

- `MeasureCatalog` (loaded from JSON like `measures.json`)
- `Rulesets` per target (loaded from JSON like `BerlinBuildingCode.json`, `RoomProgram.json`)
- `MeasureAliases` (map string → string)
- `DesignPlatformID` (e.g. `"semio"` to select correct target tool plan)

### 6.2 New/updated types (in-file)

Introduce these types (or equivalents):

- `Ruleset`:
  - `id`
  - `rules[]` where each has `id`, `description`, `clauses[]?`, `measures[]`
- `MeasureCatalogEntry`:
  - `id`, `description`, `targets[]` each with `id`, `tools.analyze[]`, `tools.change[]`, `instructions`
- `EnrichedViolation`:
  - `target`, `ruleId`, `clauseId`, `status`, `actual?`, `should?`, `measures[]`, `instructions[]`, `location?`
- `ChangePlan`:
  - grouped by `(target, ruleId)` then list of `MeasureExecStep`
- `ChangeLog`:
  - list of executed tool calls, affected nodes, errors, durations

### 6.3 New helper functions (core)

**Ruleset / catalog loading**
- `loadRuleset(path string) (Ruleset, error)`
- `loadMeasureCatalog(path string) ([]MeasureCatalogEntry, error)`
- `normalizeMeasureID(id string) string`

**Enrichment**
- `attachMeasures(report TargetValidatorOutput, ruleset Ruleset) EnrichedTargetResult`
- `buildChangePlan(enrichedReport Report) ChangePlan`

**Execution**
- `executeMeasure(ctx, m MeasureCatalogEntry, designRef) (ChangeLogEntry, error)`
- `executeChangePlan(ctx, plan ChangePlan) (ChangeLog, error)`

### 6.4 Changer invocation changes

Your earlier plan treated changer as a generic agent that “tries to fix violations”.
With measure-driven remediation, you have two viable designs:

#### Option A (recommended): assistant computes plan, changer executes

- Assistant produces `ChangePlan` with explicit steps + tool lists.
- Changer becomes a deterministic executor with minimal discretion.

Pros:
- More predictable
- Easier to test
- Strong audit trail

Cons:
- Less “agent creativity”

#### Option B: changer computes plan from enriched report + catalogs

- Assistant provides enriched report + raw catalogs.
- Changer decides sequencing.

Pros:
- More flexible
- Better for complex domain reasoning

Cons:
- Less reproducible unless changer is constrained

**Recommended implementation compromise:**
- Assistant builds a *default plan* (deterministic).
- Changer can reorder within a rule group, but must remain within measure tool allowances.

---

## 7. Testing strategy updates

### Unit tests (in Go, even if separate later)

- Measure normalization:
  - `"increase-floor-area" → "area-increase"`
- Enrichment:
  - violated rule gets measures attached from ruleset
- Plan construction:
  - correct semio tool lists chosen
- Stability:
  - identical input yields identical change plan

### Integration tests

- Use fake translator/validator binaries
- Use a mock MCP server that records tool calls
- Verify:
  - correct tools invoked for each measure
  - report iteration loop terminates correctly

---

## 8. Concrete next steps

1. Decide where rulesets and measure catalog are loaded from:
   - embedded, local files, or remote registry
2. Add measure aliasing (temporary until IDs are harmonized)
3. Implement enrichment path in assistant
4. Implement measure-driven changer execution
5. Add audit logs + report artifacts

---

**End of Refactor Plan v2**
