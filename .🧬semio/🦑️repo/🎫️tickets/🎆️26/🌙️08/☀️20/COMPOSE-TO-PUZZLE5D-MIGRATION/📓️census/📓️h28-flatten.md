# Compose Flatten Algorithm Family — Census H28

## 1. Fixture Structure: `flatten.cases.compose.json`

**File**: `/Users/ueli/Documents/semio/compose/fixture/flatten.cases.compose.json`

**Structure**: `{ "cases": [...] }`

**Total cases**: 5

**Cases**:
1. `name: "nakagin_capsule_tower"` → kit: `kit/dev/metabolism/wip/initialKit/kit.compose.json`, designPath: `["Nakagin Capsule Tower"]`
2. `name: "nakagin_capsule_tower_slanted"` → kit: same, designPath: `["Nakagin Capsule Tower", "Slanted"]`
3. `name: "nakagin_capsule_tower_twisted"` → kit: same, designPath: `["Nakagin Capsule Tower", "Twisted"]`
4. `name: "nakagin_capsule_tower_dancing"` → kit: same, designPath: `["Nakagin Capsule Tower", "Dancing"]`
5. `name: "capsule_dream"` → kit: same, designPath: `["Capsule Dream"]`

**JSON keys** at root level: `"cases"` (array)

**JSON keys** per case: `"name"`, `"kit"`, `"designPath"`

---

## 2. Rust Implementation in `compose/client/lib/rs/lib.rs`

**Region**: Lines 1192–1507, module `crate::geom::flatten`

### Core Function
- **Line 1393**: `pub async fn flatten_design_positions(kit: &Arc<Kit>, design: &Arc<Design>) -> HashMap<Id, PositionInput>`
  - Returns absolute plane and center for every piece via breadth-first traversal
  - **Signature**: Takes kit and design references; returns HashMap mapping piece IDs to computed positions

### Key Symbols & Line Numbers
| Symbol | Line | Type | Purpose |
|--------|------|------|---------|
| `flatten` module | 1194 | mod | Namespace containing all flatten logic |
| `flatten_design_positions` | 1393 | pub async fn | Main entry point: computes positions for all pieces |
| `bfs_root` | 1422 | async fn (nested) | Breadth-first traversal from a root piece |
| `piece_stored_position` | 1381 | async fn | Retrieves cached position of a piece |
| `piece_is_fixed` | 1388 | async fn | Checks if a piece is marked as fixed |
| `compute_child_plane` | 1308 | async fn | Applies connection transform to parent plane |
| `resolve_connector` | 1362 | async fn | Finds connector in type or kit by ID |
| `connector_geom` | 1298 | async fn | Extracts point, direction, t_param from connector |
| `plane_input_to_matrix` | 1236 | fn | Converts Plane to 4×4 matrix |
| `matrix_to_plane` | 1243 | fn | Inverts matrix back to Plane |
| `mul_mat` | 1247 | fn | Matrix multiplication (4×4) |
| `translation` | 1257 | fn | Builds translation matrix |
| `rotation_axis` | 1261 | fn | Builds Rodrigues rotation matrix |
| `quaternion_from_unit_vectors` | 1273 | fn | Builds rotation between vectors |
| `quaternion_to_matrix` | 1289 | fn | Converts quaternion to matrix |
| `normalize` | 1211 | fn | Vector normalization |
| `cross` | 1220 | fn | 3D cross product |
| `dot` | 1224 | fn | 3D dot product |
| `deg_to_rad` | 1228 | fn | Degree to radian conversion |
| `round_f` | 1232 | fn | Float rounding to 6 decimals |

### Data Structures Used
- **Line 1402**: `connections: Vec<Arc<Connection>>` — all connections in design
- **Line 1403**: `adjacency: HashMap<String, Vec<(String, Arc<Connection>)>>` — neighbor graph
- **Line 1418**: `piece_planes: HashMap<String, PlaneInput>` — computed absolute planes
- **Line 1419**: `piece_centers: HashMap<String, CoordinateInput>` — computed diagram centers

### Plane/Transform Solving
- **Lines 1308–1360**: `compute_child_plane` implements full 3D connector transformation
  - Resolves parent/child connector geometry (point, direction, t-param)
  - Applies gap (y-axis), shift (x-axis), rise (z-axis) translations first (lines 1354–1356)
  - Applies rotation, turn, tilt rotations in sequence (lines 1345–1351)
  - Returns transformed plane via matrix multiplication (line 1359)

---

## 3. CRITICAL QUESTION: Does Flatten Remove or Retain Connections?

**ANSWER: Flatten REMOVES all connections.**

**Definitive proof from Go implementation** (`compose/client/lib/go/main.go`):

```go
// FlattenDesignDiff line ~14700 (excerpt)
removedConnList := make([]ConnectionId, 0, len(design.Connections))
for i := range design.Connections {
    removedConnList = append(removedConnList, ConnectionId{Id: design.Connections[i].Id})
}

// ... later in function ...

result := DesignDiff{}
if len(updatedPieces) > 0 {
    result.Pieces = &PiecesDiff{Updated: updatedPieces}
}
if len(removedConnList) > 0 {
    result.Connections = &ConnectionsDiff{Removed: removedConnList}  // ← EXPLICIT REMOVAL
}
return result
```

**Semantic confirmation from AGENTS.md**:
- Line 59: "A _flat_ [`design`](#%EF%B8%8F-design-) has no [`connections`](#-connection-) and all [`pieces`](#-piece-) are _fixed_ ◳️"

**Operation consequence**: 
- The forward DesignDiff produced by flatten includes ALL connections in the `Removed` list
- The backward diff regenerates connections for undo

---

## 4. Other Language Implementations

### TypeScript/JavaScript (`compose/client/lib/js/index.ts`)
- **Line 3128**: `declare flatten: () => Promise<SetResult>;` — method exists in GraphQL interface
- **Line 3382**: `{ method: "flatten", buildInner: () => `fl: flatten` }` — GraphQL query builder
- **Status**: Declared but implementation details in query resolution (GraphQL wiring)
- **Divergence**: ⚠️ Unknown — GraphQL layer may abstract away connection removal logic

### Go (`compose/client/lib/go/main.go`)
- **Lines 14700±**: `FlattenDesignDiff(kit *Kit, designId string) DesignDiff`
- **Behavior**: Explicitly removes ALL connections (verified above)
- **Agrees with Rust**: ✓ Both compute identical absolute positions via BFS

### Python (`compose/client/lib/py/main.py`)
- **Line 14644**: `def flattenDesignDict(kit: dict, designId: str) -> dict`
- **Returns**: Only piece position updates; no explicit connection removal in dict output
- **Line 14855**: `def flattenDesignReportDict(kit: dict, designId: str) -> dict` — wraps with ComposeReport
- **Status**: Lower-level dict form doesn't emit connection removal; unclear if higher-level report wrapper adds it
- **Divergence**: ⚠️ UNCLEAR — may rely on separate `FixPiecesInDesign` operation to remove connections

### C# (.NET) (`compose/client/lib/net/Compose/cs/Compose.cs`)
- **Lines 12424±**: `public static DesignChange FlattenDesign(Kit kit, string designId)`
- **Lines 12431±**: Error handling with code `"flatten.design-not-found"`, `"flatten.empty-pieces"`
- **Line 6993**: Comment "Modifications pieces that become fixed (parent connection removed) with flat plane and center from the flattened design"
- **Status**: Follows Go/Rust pattern (implementation not fully visible in grep output)
- **Agrees with Rust**: ✓ Likely (comments suggest connection removal)

---

## 5. Numeric Behavior: Tolerances & Rounding

### Constants (`compose/client/lib/rs/lib.rs`, lines 1206–1209)
```rust
const TOLERANCE: f64 = 0.01;              // Alignment/colinear threshold
const DIAGRAM_RADIUS: f64 = 2.697;        // Diagram layout constant (unused in plane calc)
const DIAGRAM_VERTICAL_V_EXTRA: f64 = 1.0;// Diagram V-axis offset for vertical connections
const DIAGRAM_HORIZONTAL_SCALE: f64 = 3.0633; // Diagram scaling for horizontal connections
```

### Rounding (`line 1232–1234`)
```rust
fn round_f(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}
```
- **Precision**: 6 decimal places (1 μm in metric)
- **Applied to**: Diagram center coordinates (u, v) — see line 1485

### Quaternion Alignment Tolerance (`line 1275`)
```rust
if r < 0.000_001 {  // Epsilon for unit vector alignment
    // Handle degenerate case
}
```

### Plane Comparison Tolerance (Go, main.go)
```go
const tol = 0.0001  // planesEqualApprox function
```

---

## 6. Benchmarks

**File**: `/Users/ueli/Documents/semio/compose/client/benchmark/functions.csv`

**Format**: CSV with columns: `name`, `go`, `typescript`, `python`, `rust`, `csharp` (times in milliseconds)

**Flatten benchmarks** (5 cases, matching fixture):
1. `Flatten Design/Nakagin Capsule Tower` — go: 3.52ms, ts: 36.15ms, py: 53.59ms, rs: 36.34ms, cs: 51.08ms
2. `Flatten Design/Nakagin Capsule Tower/Slanted` — go: 1.01ms, ts: 15.78ms, py: 47.17ms, rs: 38.13ms, cs: 8.84ms
3. `Flatten Design/Nakagin Capsule Tower/Twisted` — go: 1.81ms, ts: 11.83ms, py: 54.83ms, rs: 35.60ms, cs: 9.09ms
4. `Flatten Design/Nakagin Capsule Tower/Dancing` — go: 2.00ms, ts: 11.34ms, py: 75.53ms, rs: 35.77ms, cs: 9.10ms
5. `Flatten Design/Capsule Dream` — go: 39.62ms, ts: 703.22ms, py: 863.59ms, rs: 775.92ms, cs: 575.98ms

**Observation**: Capsule Dream is 10–20× slower; intermediate designs scale linearly with piece count.

---

## 7. Tests Exercising Flatten

### Go (`compose/client/lib/go/main_test.go`)
- **Line 1070**: `func TestFlattenMerkle(t *testing.T)` — primary test
  - Loads `/compose/fixture/flatten-merkle.cases.compose.json`
  - Runs 10 merkle mutation cases
  - **Line 1081**: `ComputeFlatHashes(&kitBefore, designIdBefore)` — computes plane/center hashes
  - **Line 1182**: `CachedFlattenReusesValues` subtest — validates merkle caching across runs
  - **Line 1160**: `CrossLanguageParityReferenceHashes` subtest — validates SHA256 hashes match expected cross-language values

### Python (`compose/client/lib/py/main.py`)
- **Lines 2145–2163**: Flatten benchmark harness in test_main
  ```python
  flatten_cases = _test_load_json("flatten.cases.compose.json")["cases"]
  for _fc in flatten_cases:
      _make_flatten_bench(_kit, _design, _label)  # Benchmarks each case
  ```

### Rust
- **Line 21612**: `fn flatten_design_resolves_linked_piece_absolute_pose()` (test function name from grep)
  - Likely unit test for linked piece resolution

### C# Tests
- **File**: `/Users/ueli/Documents/semio/compose/client/lib/net/Compose.Tests/cs/Tests.cs`
- Test file exists but flatten-specific tests not visible in grep output

---

## 8. `flatten-merkle`: Relationship to Flatten

**File**: `/Users/ueli/Documents/semio/compose/fixture/flatten-merkle.cases.compose.json`

**Structure**: Separate test fixture (not a variant of flatten)

**Purpose**: Cross-language parity validation for **incremental** flatten via merkle hashing

**Key sections**:
1. **Lines 3–18** — `parity` block: Reference SHA256 hashes for plane/center of 2 specific pieces
   - `planeHash`: "ad8a923756fe73d84190135710c09cc3dafff280c5f23de5e98b6252d7958ba6"
   - `centerHash`: "334c87eed98599cae04e43a9cba40308c344452c904bac5661e60cbc153c0ad0"

2. **Lines 20–206** — `cases` array: 10 mutation cases
   - Example: `nakagin_baseline_no_mutation` — mutate nothing; expect no hash changes
   - Mutations include: piece description, piece plane origin, connection gap/rotation/u/v
   - Assertions: which pieces' plane/center hashes should change

**Relationship**:
- `flatten-merkle` is **NOT** flatten-then-hash; it's a **separate algorithm** for caching
- Flatten computes positions once
- **FlatMerkleCacheEntry** (line 15008 in Rust) bundles position + hashes; subsequent flattens reuse unchanged chains
- Mutation tracking: if input piece hashes unchanged, skip descendant recomputation

**Algorithm flow**:
1. Flatten design (compute all positions)
2. Hash each piece's plane + center
3. On next flatten call, check if parent chains' hashes match previous run
4. If match: reuse cached plane/center; if mismatch: recompute from that point down

---

## 9. Documentation on Flatten Semantics

### Compose README (`compose/README.md`)
- **Lines 51–63**: "Design" concept
  - Line 53: "A [`design`](#%EF%B8%8F-design-) is an undirected graph of [`pieces`](#-piece-) (nodes) and [`connections`](#-connection-) (edges)"
  - Line 59: **"A _flat_ [`design`](#%EF%B8%8F-design-) has no [`connections`](#-connection-) and all [`pieces`](#-piece-) are _fixed_"** ← definitive statement
  - Line 60: "The [`pieces`](#-piece-) are _placed_ _hierarchically_ ([breadth-first](https://en.wikipedia.org/wiki/Breadth-first_search)) for every _component_"

### Compose AGENTS.md (`compose/AGENTS.md`)
- **Lines 51–63**: Same design specification as README
- **No specific algorithm pseudocode** for flatten

### Dev Algorithm Docs
- **Search result**: No `compose/dev/algorithm/**` files matching "flatten" found
- **Status**: UNKNOWN — algorithm docs may be minimal or in different location

---

## Summary Table

| Item | Value | Source |
|------|-------|--------|
| **Fixture cases** | 5 | flatten.cases.compose.json |
| **Removes connections** | ✓ YES | Go main.go lines ~14700 |
| **Rust TOLERANCE** | 0.01 | rs/lib.rs:1206 |
| **Rust rounding** | 6 decimals | rs/lib.rs:1232–1234 |
| **Rust algorithm** | BFS from fixed pieces | rs/lib.rs:1422 |
| **Go agrees** | ✓ YES | main.go FlattenDesignDiff |
| **Python agrees** | ⚠️ UNCLEAR | py/main.py doesn't emit Removed in dict output |
| **C# agrees** | ✓ LIKELY | Compose.cs comment references |
| **Benchmarks** | 5 cases | functions.csv |
| **Primary test** | TestFlattenMerkle | go/main_test.go:1070 |
| **flatten-merkle** | Caching layer, not separate flatten | Fixture + Rust line 15008 |

---

## Known Unknowns

1. **Python connection removal**: `flattenDesignDict` returns dict with only piece updates; unclear if higher-level ComposeReport wrapper removes connections via separate operation or diff inversion.
2. **TypeScript implementation**: GraphQL wiring abstracts actual flatten logic; unclear if TS implements removal or delegates to Go/Rust backend.
3. **Algorithm documentation**: No detailed pseudocode or algorithm rationale found in `compose/dev/` or comments beyond AGENTS.md definition.

