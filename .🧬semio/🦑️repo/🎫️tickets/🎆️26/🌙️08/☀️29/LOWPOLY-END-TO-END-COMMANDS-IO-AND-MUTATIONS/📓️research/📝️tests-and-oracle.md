# Lowpoly Test Surface, Oracle Mechanism, and Test Inventory

## 1. Existing Test Coverage: mutate-lowpoly-1

The lowpoly plugin declares ONE committed test case: **mutate-lowpoly-1**, located at:
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/mutate-lowpoly-1/`

### 1.1 Scenario Outline Coverage (Feature: Cross-Language Differential)

The Gherkin feature (🥒️.feature) defines TWO scenario outlines with 17 examples each — 34 parameterized scenarios total. These test the 17 KINDS of mutations that the LowpolyMutation enum declares:

#### Scenario Outline 1: `mutate-<id>`
Tests that the committed vector for each mutation kind:
- Declares its own kind correctly (EXTERNALLY TAGGED, single key)
- Moves the before-snapshot to the after-snapshot exactly

**Mutations tested (17 kinds, 1 fixture each):**
1. `create-object`: inserts-obj-mast-between-hull-and-fin
2. `delete-object`: removes-obj-fin-without-touching-the-order
3. `reorder-objects`: moves-obj-fin-in-front-of-obj-hull
4. `rename-object`: retitles-obj-hull
5. `change-object-smooth-shading`: turns-on-smooth-shading-for-obj-hull
6. `move-object`: translates-obj-hull-along-x-and-z
7. `rotate-object`: yaws-obj-hull-about-the-y-axis
8. `scale-object`: halves-obj-hull-uniformly
9. `create-mesh`: attaches-a-mesh-child-handle-to-obj-fin
10. `delete-mesh`: detaches-the-mesh-child-handle-from-obj-hull
11. `insert-paint-layer`: stacks-a-detail-layer-above-the-base-layer
12. `remove-paint-layer`: drops-the-detail-layer-at-index-1
13. `rename-paint-layer`: retitles-the-base-layer-to-undercoat
14. `change-paint-layer-visible`: hides-the-base-layer
15. `change-paint-layer-opacity`: fades-the-base-layer-to-half
16. `change-paint-layer-blend-mode`: switches-the-base-layer-to-multiply
17. `edit-paint-layer`: paints-red-over-the-second-half-of-the-base-layer

#### Scenario Outline 2: `inverse-<id>`
Tests that the committed diff's footprint is COMPLETE:
- Every field that changed between before/after is declared in the diff
- Every field the diff declares actually differs
- Verifies the PRECONDITION for undoability (weaker than full inverse law)

Uses the same 17 kinds and fixtures as Scenario 1.

#### Scenario 3: `identity-round-trip`
Tests the identity property for the specific JSON encoding:
- Parses the committed before-snapshot (two-object document with stacked paint layers)
- Re-serializes it through platform's dependency-free JSON reader/writer
- Asserts the re-serialized bytes DIFFER from committed (committed is pretty-printed, writer is compact)
- Asserts re-parsed document equals original document (JSON identity preserved)

**Total committed test cases: 35 scenarios (34 parameterized + 1 identity)**

### 1.2 Test Implementation Files

Three implementation roles in mutate-lowpoly-1:

#### 🦀️.rs (Rust Subject Adapter)
- **Path**: `🗿️artifacts/💠️lowpoly/🧪️tests/mutate-lowpoly-1/🦀️component.rs`
- **Role**: SUBJECT (tests the Rust implementation)
- **Coverage**: 17 mutate-{kind} + 17 inverse-{kind} scenarios (34 total), plus identity-round-trip (1)
- **Strategy**: REPLAYS committed vectors WITHOUT running the plugin codec
  - Reads JSON vectors from committed fixture files
  - Asserts that vectors declare claimed kind
  - Asserts observability law (mutation moves the document, unless outcome declares no-op)
  - Asserts footprint completeness law
  - Uses shared metamorphic law helpers from `🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs`
- **Constraint**: Does NOT link the plugin crate — vectors are the oracle, not the codec

#### 🐍️.py (Python Oracle Adapter)
- **Path**: `🗿️artifacts/💠️lowpoly/🧪️tests/mutate-lowpoly-1/🐍️component.py`
- **Role**: ORACLE (independent second implementation)
- **Coverage**: 17 mutate-{kind} + 17 inverse-{kind} scenarios (34 total), plus identity-round-trip (1)
- **Strategy**: ACTUALLY APPLIES mutations
  - Written from specification alone: the schema snapshot (object/layer shape), derivation rules (rules 2, 3, 7), and the committed vectors
  - No Rust code read; no plugin crate imports
  - Implements all 17 mutation verbs in closed vocabulary
  - Asserts applied result equals committed after-snapshot
  - Asserts observability (applied mutation moves the document)
  - Implements the FULL inverse law: apply(inverse(m), apply(m, base)) == base
  - Handles base64 RUNS in edit-paint-layer by byte-offset overwrites (never resizing)
- **Key details**:
  - EXTERNALLY TAGGED mutations: payload is `{"MoveObject": {...}}`
  - meshWorkspace argument only in create-mesh (not in schema snapshot)
  - All paint layers addressed by INDEX, objects by ID

#### 🥒️.feature (Gherkin Feature)
- **Path**: `🗿️artifacts/💠️lowpoly/🧪️tests/mutate-lowpoly-1/🥒️.feature`
- **Role**: Specification and scenario declaration
- **Tags**: 
  - `@capability-lowpoly-1-mutate`: capability being tested
  - `@oracle-lowpoly-python-independent`: oracle registration tag
  - `@comparison-ordered-json-v1`: comparison profile (ordered JSON with no tolerance)
  - `@mutations-lowpoly-1-any`: mutation catalog reference
  - `@id-mutate`, `@id-inverse`, `@id-identity-round-trip`: scenario classification
  - `@level-exhaustive`, `@level-long`: test levels
  - `@mode-differential`, `@mode-round-trip`: test modes

### 1.3 Fixture Locations

All 85 fixtures (17 mutations × 5 files each + 0 shared) are handcrafted specification vectors in:
`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{emoji}{mutation-name}/🧪️tests/{test-name}/`

Each mutation directory holds a SINGLE fixture:
- `⬅️before/🔣️component.json`: before-snapshot
- `🦠️mutation/🔣️component.json`: mutation payload (EXTERNALLY TAGGED)
- `🔺️diff/🔣️component.json`: diff (footprint declaration)
- `🎯️outcome/🔣️component.json`: outcome (status, messages)
- `➡️after/🔣️component.json`: after-snapshot

Example vectors:
- `🌱️create-object/🧪️tests/inserts-obj-mast-between-hull-and-fin/`
- `🎨️edit-paint-layer/🧪️tests/paints-red-over-the-second-half-of-the-base-layer/`

---

## 2. Test Discovery and Naming Convention

### 2.1 Language-Agnostic File Kind Registration

From repo taxonomy (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`):

**fileKinds (test-related):**
```
"gherkin-feature": {
  "emoji": "🥒️",
  "extensionChains": [".feature"],
  "role": "test"
}

"rust-source": {
  "emoji": "🦀️",
  "extensionChains": [".rs"],
  "role": "source"
}

"python-source": {
  "emoji": "🐍️",
  "extensionChains": [".py"],
  "role": "source"
}

"json-document": {
  "emoji": "🔣️",
  "extensionChains": [".json"],
  "role": "data"
}
```

**Test Configuration:**
```
"testsDirName": "🧪️tests"
"testFixturesDirName": "🧫️fixtures"
"testFeatureFileKindId": "gherkin-feature"
"testCaseSlugPattern": "^[a-z0-9]+(?:-[a-z0-9]+)*$"

"testAdapterFileKinds": {
  "🦀️rust": "rust-source",
  "🟦️typescript": "typescript-source",
  "🐹️go": "go-source",
  "🐍️python": "python-source",
  "🔷️dotnet": "dotnet-source"
}

"testImplementationIds": {
  "🦀️rust": "rust",
  "🟦️typescript": "typescript",
  "🐹️go": "go",
  "🐍️python": "python",
  "🔷️dotnet": "dotnet"
}
```

### 2.2 Test Case Naming Convention

**Discovered test cases** follow pattern: `{prefix}-{artifact}-{version}`

Where:
- `{prefix}` = `mutate`, `io`, `codec`, etc. (from test catalog)
- `{artifact}` = artifact slug (e.g., `lowpoly`, `jack`, `fem3d`)
- `{version}` = integer version number (e.g., 1, 2, 3)

**Examples from codebase:**
- `mutate-lowpoly-1` (this artifact's test case)
- `mutate-jack-1` (trinity/jack artifact)
- `mutate-fem3d-1` (fem/3d artifact)
- `mutate-puzzle-3d-1` (puzzle/3d artifact)

**Directory structure** (REQUIRED for discovery):
```
📦️plugins/{plugin}/🗿️artifacts/{artifact}/🧪️tests/
└── {prefix}-{artifact}-{version}/
    ├── 🥒️.feature          (REQUIRED: Gherkin feature file)
    ├── 🦀️.rs               (OPTIONAL: Rust adapter)
    ├── 🐍️.py               (OPTIONAL: Python adapter)
    ├── 🟦️.ts               (OPTIONAL: TypeScript adapter)
    ├── 🐹️.go               (OPTIONAL: Go adapter)
    └── 🔷️.cs               (OPTIONAL: .NET adapter)
```

**File kind emojis MUST match:** repo taxonomy defines these as the ONLY canonical filenames the test runner discovers.

---

## 3. Oracle Mechanism and Third-Party Library Requirement

### 3.1 Lowpoly Oracle Configuration

Location: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`

**Oracle Registration (schema v2):**
```json
{
  "oracles": [
    {
      "id": "lowpoly-python-independent",
      "ecosystem": "python",
      "package": "",
      "version": "",
      "capabilities": ["lowpoly-1-mutate"],
      "comparisonProfiles": ["ordered-json-v1"],
      "license": "AGPL-3.0-only",
      "testOnly": true,
      "rationale": "A second implementation of the s.lowpoly.lowpoly document and all seventeen typed mutations, in Python...",
      "kind": "cross-semio-implementation",
      "engine": {
        "family": "none",
        "implementation": "in-repository second implementation",
        "version": "0"
      },
      "productionReachable": false,
      "networkDuringExecution": false
    }
  ],
  "noOracleDecisions": [],
  "mutationCatalogs": [
    {
      "id": "lowpoly-1-any",
      "capability": "lowpoly-1-mutate",
      ...
    }
  ]
}
```

### 3.2 Oracle Classification and CLAUDE.md Compliance Issue

**Status**: DECLARED SUPPLEMENTAL (cross-semio-implementation) — NOT a third-party library reference

**Kind**: `cross-semio-implementation`
- Written from specification (schema snapshot, derivation rules, committed vectors)
- NO Rust/production code read
- Imports nothing from plugin
- Acts as differential oracle only

**CLAUDE.md Requirement**: Every feature must be validated against at least one third-party library
- **STATUS**: ❌ **UNFULFILLED** — Python implementation is SUPPLEMENTAL only
- **Noted in oracle.json rationale**: "UNDER PROTOCOL V2 this registration is classified `cross-semio-implementation` — a required SUPPLEMENTAL oracle that does not discharge `lowpoly-1-mutate`'s external-oracle requirement for its 17-kind mutation vocabulary; a qualifying third-party reference (`third-party-library` / `third-party-cli` / `standards-reference-tool`) is still owed."

**Why third-party library was declined (not merely absent):**
- Two-level addressing: ID-keyed objects + INDEX-keyed paint layers (unique to this vocab)
- Paint stack buffer edits at byte OFFSET (edit-paint-layer splices base64 RUNS in place)
- No mesh/scene library models this algebra: `networkx`, `igraph`, `petgraph`, Blender, Rhino, etc. all model vertices/edges, not port-addressed property graphs with layer stacks
- None of these libraries read `.dsl.semio`
- Verdict in oracle.json: "A THIRD-PARTY library was declined, not merely absent"

### 3.3 Oracle Comparison Profile

**Profile**: `ordered-json-v1`
- Ordered JSON comparison (field order preserved)
- No tolerance for value divergence
- Member-by-member equality check

---

## 4. Other Plugins' Test Organization Patterns

### 4.1 Trinity Plugin (jack artifact)
- **Test location**: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/`
- **Test count**: 1 case (mutate-jack-1)
- **Pattern**: `mutate-{artifact}-{version}`
- **Oracle**: Python independent implementation (oracle-jack-python-independent)
- **Mutations**: 8 kinds (nodes, edges, property graphs with port-addressed endpoints)

### 4.2 FEM Plugin (fem3d artifact)
- **Test location**: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🧪️tests/`
- **Test count**: 1 case (mutate-fem3d-1)
- **Pattern**: `mutate-{artifact}-{version}`
- **Oracle**: Python independent implementation (oracle-fem3d-python-independent)
- **Reference**: Second implementation approach parallels lowpoly

### 4.3 Puzzle Plugin (3d artifact)
- **Test location**: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🧪️tests/`
- **Test count**: 1 case (mutate-puzzle-3d-1)
- **Pattern**: `mutate-{artifact}-{version}`
- **Coverage**: Block algebra mutations (3D solid geometry)

---

## 5. Untested Lowpoly Behavior (Test Gaps)

### 5.1 Commands and I/O Formats

**UNTESTED:**
1. **No command-line tests**: lowpoly CLI commands (create, edit, export, import) have NO test coverage
   - No verification of command argument parsing
   - No validation of CLI exit codes/error messages
   - No end-to-end pipeline tests (file read → mutation → file write)

2. **No I/O format round-trips**: Lowpoly supports multiple serialization formats
   - No `.dsl.semio` text grammar tests (parse/print round-trip)
   - No `.pack.semio` binary protocol tests (encode/decode round-trip)
   - No format migration/upgrade tests
   - No format validation tests (malformed input rejection)

3. **No import/export tests**: Cross-format conversion
   - No JSON → `.dsl.semio` round-trip
   - No JSON → `.pack.semio` round-trip
   - No `.dsl.semio` ↔ `.pack.semio` equivalence

### 5.2 Mutation Scenarios Not Exercised

**Mutation edge cases:**
1. **Boundary conditions**:
   - Create object with index at/beyond array bounds
   - Remove last paint layer (index edge)
   - Reorder objects to self (no-op variant)
   - Edit paint layer with zero-length runs

2. **Complex mutations**:
   - Multiple mutations in sequence (composite operations)
   - Undo/redo chains (inverse composition law)
   - Mutations on documents with no objects
   - Mutations on objects with no mesh or empty paint stack

3. **Mesh mutations**:
   - Replace mesh (delete then create)
   - Mesh on objects in different order states
   - Create-mesh with non-existent target references

4. **Paint layer mutations**:
   - Insert at HEAD vs TAIL of stack
   - Opacity edge values (0.0, 1.0)
   - All blend modes (currently only tests multiply)
   - Edit runs that touch buffer boundaries

### 5.3 Data Integrity and Error Handling

**UNTESTED:**
1. **Codec robustness**:
   - Malformed JSON input
   - Missing required fields
   - Type mismatches (string vs number)
   - Duplicate object IDs
   - Out-of-bounds paint layer indices

2. **Mutation rejection**:
   - Mutations that would violate schema (should be caught)
   - Invalid transformation matrices
   - Illegal childId references

3. **Round-trip fidelity**:
   - Multiple encode/decode cycles
   - Whitespace/formatting preservation (text grammar)
   - Floating-point precision in transforms

---

## 6. Running Lowpoly Tests

### 6.1 Run All Tests (All Levels)

```bash
cd /Users/ueli/Documents/semio
bun ./📜️script.ts test
```

### 6.2 Run Lowpoly Tests Only

```bash
cd /Users/ueli/Documents/semio
bun ./📜️script.ts test --filter lowpoly
# OR
bun ./📜️script.ts test --owner lowpoly
```

### 6.3 Run Specific Test Case

```bash
cd /Users/ueli/Documents/semio
bun ./📜️script.ts test --case mutate-lowpoly-1
```

### 6.4 Run at Specific Level

```bash
cd /Users/ueli/Documents/semio
# Exhaustive level (all 34 parameterized + 1 identity = 35 scenarios)
bun ./📜️script.ts test --level exhaustive --case mutate-lowpoly-1

# Long level (identity round-trip scenario)
bun ./📜️script.ts test --level long --case mutate-lowpoly-1
```

### 6.5 Run with Coverage

```bash
cd /Users/ueli/Documents/semio
bun ./📜️script.ts coverage
```

### 6.6 Test Framework Invocation

The test runner is implemented by:
- **Discovery**: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/`
- **Adapter registration**: Each 🦀️.rs file exports `pub fn adapter() -> Adapter`
- **Python registration**: Each 🐍️.py file exports `def adapter()`
- **Feature file parsing**: Gherkin scenarios mapped to test cases via full scenario id

---

## 7. Summary: Test Inventory and Coverage

| Category | Count | Status |
|----------|-------|--------|
| **Mutation Kinds Tested** | 17/17 | ✅ Complete |
| **Scenarios (mutate-{id})** | 17 | ✅ Implemented |
| **Scenarios (inverse-{id})** | 17 | ✅ Implemented |
| **Scenarios (identity-round-trip)** | 1 | ✅ Implemented |
| **Total Test Scenarios** | 35 | ✅ Complete |
| **Languages** | 2 (Rust subject + Python oracle) | ✅ Cross-lang differential |
| **Fixtures** | 85 (17 × 5 files) | ✅ Handcrafted vectors |
| **Third-Party Oracle** | 0 | ❌ MISSING (CLAUDE.md violation) |
| **CLI/Command Tests** | 0 | ❌ MISSING |
| **I/O Format Tests** | 0 | ❌ MISSING |
| **Edge Case Tests** | 0 | ❌ MISSING |

---

## 8. Interactive Job Configuration

### 8.1 Component Partition (🔣️component.json)

Located at: `✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️component.json`

**Configuration**: 47 tools, 28 Migrated, 19 BatchOnlyPendingRewrite
- Migrated tools can run in interactive lanes: Artifact, Config, HostOnly, Transient
- Pending rewrite tools have blocker: "Reducer lacks a bounded operation-owned cursor or exact Store publication authority"
- Examples of migrated: patchObject, addPaintLayer, paintStrokeEnd, importSnapshotJson, setFixtureJson
- Examples of pending: addPrimitive, extrude, inset, bevel, loopCut, subdivide, triangulate, mirror, decimate, flipFaces, merge, dissolve, snap, toggleSmooth, etc.

### 8.2 Schema Validation (🔣️schema.json)

Located at: `✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️schema.json`

- JSON Schema draft 2020-12
- Validates partition structure: version, owner, maximumPollMicros, routes array
- 47 routes required, each with: toolId, classification, lanes, preparation, blocker
- Constraints: Migrated routes require non-empty lanes and null blocker; BatchOnlyPendingRewrite require empty lanes and non-empty blocker string

---

## EOF Marker

**Report generated**: 2026-08-29
**Repository**: /Users/semio
**Lowpoly plugin path**: ✏️s/🔌️plugins/💠️lowpoly
**Test path**: 🗿️artifacts/💠️lowpoly/🧪️tests/mutate-lowpoly-1
