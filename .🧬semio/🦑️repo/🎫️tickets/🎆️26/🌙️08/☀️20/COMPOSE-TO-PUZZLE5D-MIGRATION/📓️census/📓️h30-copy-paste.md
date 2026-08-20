# COMPOSE Copy/Paste/Duplicate/Selection Transfer Census — H30/H31 Migration

## 1. copy-paste.cases.compose.json

**1 test case**: `nakagin_capsule_tower`

**Case structure** (`copy-paste.cases.compose.json:1–19`):
- `name`: `nakagin_capsule_tower` (string)
- `kit`: Reference to initial kit at `kit/dev/metabolism/wip/initialKit/kit.compose.json`
- `designName`: `"Nakagin Capsule Tower"` (target design name)
- `selectionAsset`: Points to selection fixture `nakagin-capsule-tower.copy.design.selection.compose.json`
- `expectedCopyAsset`: Points to `nakagin-capsule-tower.copy.design.compose.json`
- `pasteTargetAsset`: Points to `nakagin-capsule-tower.paste.design.compose.json`
- `expectedPasteDiffAsset`: Points to `nakagin-capsule-tower.paste.design.diff.compose.json`
- `pasteCoordinate`: `{"u": 10, "v": 10}`
- `expectedPasteWithCoordinateDiffAsset`: Points to `nakagin-capsule-tower.paste.with-coordinate.design.diff.compose.json`
- `designFamilies`: Empty array

**Input shapes**:
- Selection input: `pieces` array (10 piece UUIDs) + `connections` array (9 connection UUIDs)
- Copy output: Design with `pieces` and `connections` arrays
- Paste input: Source design (copy output) + target design + anchoring="original" + coordinate (optional)
- Paste output: DesignDiff with `pieces.added` and `connections.added` arrays

**JSON keys**: `id`, `name`, `description`, `type`, `plane`, `center`, `scale`, `mirrorPlane`, `isHidden`, `isLocked`, `color`, `props`, `attributes`

---

## 2. Committed Fixture Files — Chaining & Representations

### nakagin-capsule-tower.copy.design.selection.compose.json
**Path**: `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.copy.design.selection.compose.json`

**Represents**: A user selection from a design — which pieces and connections are selected for copying.

**Structure** (lines 1–63):
- `pieces`: Array of 10 pieces, each with only `id` field (UUID-4 style)
  - Examples: `"31be08e1-e75c-4024-86b4-c3c6d3939fbb"`, `"3b0444ad-2307-442e-972a-7f1c2cc7dcb9"`, etc.
- `connections`: Array of 9 connections, each with only `id` field
  - Examples: `"06e4aa42-4a30-4149-9f0d-0eea397c0120"`, etc.

**Selection representation**: MINIMAL — only IDs. No types, ports, representations, or attributes.

### nakagin-capsule-tower.copy.design.compose.json
**Path**: `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.copy.design.compose.json`

**Represents**: The clipboard state after copying the selection.

**Structure** (lines 1–100 visible):
- `id`: Empty string `""`
- `name`: Empty string `""`
- `parent`: null
- `pieces`: Array of 10 pieces with **full payload**:
  - `id`, `name`, `description`, `type` (ref), `design`, `plane`, `center`, `scale`, `mirrorPlane`, `isHidden`, `isLocked`, `color`, `props`, `attributes`
  - **Key attributes for external pieces**: `"compose.center"` (JSON string of coordinate) and `"compose.plane"` (JSON string of plane)
  - Example: piece `"31be08e1-e75c-4024-86b4-c3c6d3939fbb"` (fixed) has both `compose.center` and `compose.plane` attributes
  - Example: piece `"3b0444ad-2307-442e-972a-7f1c2cc7dcb9"` (connected, no plane) has empty attributes
- `connections`: Array of 9 connections

**Classifies pieces as**:
1. **Internal-fixed**: plane set, both endpoints selected → copied as-is
2. **Internal-connected**: both endpoints selected, connection selected → copied as-is
3. **Parent-excl-child-incl** (pp-excl-pc-incl): parent not selected, connection selected → copied with `compose.center` + `compose.plane` attributes
4. **External**: Referenced by non-internal connections → added with `compose.piece.origin = "external"` + `compose.center` attributes

### nakagin-capsule-tower.paste.design.compose.json
**Path**: `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.paste.design.compose.json`

**Represents**: The target design into which paste will merge pieces and connections.

**Structure**:
- Design with `id`, `name: "Nakagin Capsule Tower Second Storey"`, `pieces`, `connections`
- Pieces reference types and may have `pose` (plane + center for existing pieces)
- Used to match external-origin clipboard pieces by name (exact match required to remap connections)

### nakagin-capsule-tower.paste.design.diff.compose.json
**Path**: `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.paste.design.diff.compose.json`

**Represents**: The diff result from `PasteDesign(kit, copy_output, target_design, "original", null)`.

**Structure** (lines 1–200 visible):
- `pieces.added`: Array of 10 pieces, **same IDs as copy output**
  - Piece `"31be08e1-e75c-4024-86b4-c3c6d3939fbb"`: has `plane`, `center` explicitly set (was pp-excl-pc-incl)
  - Piece `"7dc5b737-3b6b-4068-b315-b7bacc91c2e1"`: has `center` = `{"u": 0.0, "v": 0.0}` (was fixed, anchor-adjusted)
  - Piece `"3b0444ad-2307-442e-972a-7f1c2cc7dcb9"`: no plane/center (was internal-connected)
- `connections.added`: Array of connections

**Critical observation**: Pasted piece IDs are IDENTICAL to clipboard source IDs.

### nakagin-capsule-tower.paste.with-coordinate.design.diff.compose.json
**Path**: `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.paste.with-coordinate.design.diff.compose.json`

**Represents**: The diff result from `PasteDesign(kit, copy_output, target_design, "original", {u: 10, v: 10})`.

**Key difference from plain paste**:
- Piece `"31be08e1-e75c-4024-86b4-c3c6d3939fbb"`: 
  - Without coordinate: `center` = `{"u": 1.907067, "v": 5.867067}`
  - With coordinate: `center` = `{"u": 11.907067, "v": 15.867066999999999}` (+ 10 offset)
- Piece `"7dc5b737-3b6b-4068-b315-b7bacc91c2e1"` (type `"b"`):
  - Without coordinate: `center` = `{"u": 0.0, "v": 0.0}`
  - With coordinate: `center` = `{"u": 10.0, "v": 10.0}` (exact coordinate)

**Coordinate effect**: Adds `{u: 10, v: 10}` offset to all piece centers (after anchor adjustment).

---

## 3. Rust Implementation (lib.rs)

**File**: `/Users/ueli/Documents/semio/compose/client/lib/rs/lib.rs` (21,677 lines)

**Status**: No `CopyDesign` or `PasteDesign` functions found in Rust lib.rs via grep. Copy/paste likely implemented in GraphQL schema or via WASM wrapper, not in core Rust domain logic library.

**Checked for**: `copy_design`, `paste_design`, `CopyDesign`, `PasteDesign` — all returned no matches.

---

## 4. Go Implementation (main.go) — Core Copy/Paste Logic

**File**: `/Users/ueli/Documents/semio/compose/client/lib/go/main.go` (18,121 lines)

### CopyDesign Signature
**Line 12451**:
```go
func CopyDesign(kit *Kit, design Design, pieceIds []string, connectionIds []string) Design
```

**Behavior** (lines 12451–12607):
1. Classifies each selected piece into one of three categories using parent/connection info
2. Flattens design to compute absolute planes/centers (for pp-excl-pc-incl pieces)
3. **Returns a Design** with copied pieces and connections — **NO ID REMAPPING**
4. Pieces retain original IDs; external pieces get `compose.piece.origin = "external"` + `compose.center`/`compose.plane` attributes

**Key classifications**:
- **isInternalFixed**: piece has plane AND is selected → copy as-is
- **isInternalConnected**: parent AND connection both selected → copy as-is
- **isPpExclPcIncl**: parent NOT selected BUT connection selected → copy with attributes

### PasteDesign Signature
**Line 12617**:
```go
func PasteDesign(kit *Kit, source Design, target Design, anchoring string, coordinate *Coordinate) DesignDiff
```

**Behavior** (lines 12617–13017):
1. Identifies external-origin pieces in source
2. Computes bounding box from non-external pieces
3. Calculates anchor point based on anchoring mode: `"original"`, `"middle"`, `"centroid"`, `"bottomLeft"`, `"topLeft"`, `"topRight"`, `"bottomRight"`
4. For each piece:
   - If fixed & not connected: apply `-anchor` offset, then `+coordinate` offset if given
   - If connected with external parent: try to match target piece by name + connector; if match found, remap connection; else treat as fixed
   - If connected with non-external parent: add as-is
5. **Returns DesignDiff** with `pieces.added` and `connections.added` — **IDs UNCHANGED from source**

**Key line 12821**:
```go
center = Coordinate{U: center.U - anchor.U, V: center.V - anchor.V}
if coordinate != nil {
    center = Coordinate{U: center.U + coordinate.U, V: center.V + coordinate.V}
}
```

**Connection remapping** (lines 12876–12887):
- If external parent found in target by name match: update connection to point to matched target piece

---

## 5. ID Remapping — THE CRITICAL DETERMINISM ISSUE

**NONDETERMINISTIC — RANDOM IDs NOT REMAPPED**

### Evidence

**Go's `Id()` function** (`main.go:274–278`):
```go
func Id() string {
    bytes := make([]byte, 16)
    rand.Read(bytes)
    return hex.EncodeToString(bytes)
}
```

**ID generation**: Generates random 16 bytes → converts to 32-char hex string. **Cryptographically random, not deterministic.**

### Paste Behavior

When `PasteDesign` returns, pasted pieces retain **the exact same IDs as the clipboard source**. Examples from fixture:

| Piece ID | Copy Output | Paste Diff | Paste+Coordinate Diff |
|----------|------------|-----------|----------------------|
| `31be08e1...` | ✓ Present | ✓ Same ID | ✓ Same ID |
| `3b0444ad...` | ✓ Present | ✓ Same ID | ✓ Same ID |
| `7dc5b737...` | ✓ Present | ✓ Same ID | ✓ Same ID |

**No ID map returned**; no new IDs generated during paste.

### Migration Implication

**If you paste the same clipboard twice, you get duplicate IDs.** The caller must handle ID remapping BEFORE or AFTER applying the DesignDiff. This is likely the responsibility of:
1. **Client UI layer** (remap IDs on diff before `ApplyDesignDiff`)
2. **Or server validation** (reject/heal duplicates via `IdUniquenessConstraint`)
3. **No serialized ID map** is provided in the paste result

---

## 6. Selection Payload Shape

**copy-paste.cases.compose.json & fixtures show**:

Selection is a **minimal structure**:
```json
{
  "pieces": [{"id": "..."}, {"id": "..."}, ...],
  "connections": [{"id": "..."}, {"id": "..."}, ...]
}
```

**What is carried**:
- Piece IDs only (no types, ports, representations)
- Connection IDs only (no endpoints, port info)

**Dependency closure NOT computed in copy**:
- User explicitly selects pieces AND connections
- Copy extracts only those selections
- External pieces (referenced by non-internal connections) are added automatically — but only for connections that are in the selection

**Example from fixture** (`nakagin-capsule-tower.copy.design.selection.compose.json`):
- 10 piece IDs selected
- 9 connection IDs selected
- Copy output includes 10 pieces (including auto-added external pieces not in original selection)
- Copy output includes 9 connections

---

## 7. Paste.with-coordinate vs Plain Paste

**Plain paste** (`"original"` anchor, no coordinate):
```
newCenter.u = center.u - anchor.u
newCenter.v = center.v - anchor.v
```

**Paste with coordinate** (`"original"` anchor, `{u:10, v:10}`):
```
newCenter.u = (center.u - anchor.u) + coordinate.u
newCenter.v = (center.v - anchor.v) + coordinate.v
```

**In fixtures**:
- Plain: piece `"7dc5b737..."` → `center = {u: 0.0, v: 0.0}`
- With coord: piece `"7dc5b737..."` → `center = {u: 10.0, v: 10.0}`

**Coordinate shifts all pasted piece centers by the given offset** (after anchor adjustment).

**For remapped stub-bridge connections**: Per line 12888–12945 (Go), if both coordinate AND stub-child edge match, connection `u`/`v` is recalculated based on target parent center minus the anchor+coordinate offset.

---

## 8. Other Language Implementations

### Python (main.py)
**Functions**: `copyDesignDict` (line 12407), `pasteDesignDict` (line 12538)
- **Status**: Identical logic to Go implementation
- **File size**: 21,571 lines
- **IDs**: Also nondeterministic (`uuid.uuid4()` or equivalent in Python)
- **Tests**: `test_copy()` (line 20209), `test_paste_without_coordinate()` (line 20267), `test_paste_with_coordinate()` (line 20305)

### JavaScript/TypeScript (index.ts)
**Status**: GraphQL transport layer only; no copy/paste functions found in `compose/client/lib/js/index.ts`
- File is primarily type definitions and GraphQL wire protocol (WASM worker or inline)
- Copy/paste likely called via GraphQL mutations to Rust backend

### .NET / C# (net/)
**Directory**: `/Users/ueli/Documents/semio/compose/client/lib/net/`
**Status**: Present but not examined in depth; implementation likely mirrors Go/Python logic

### Ruby (rb/)
**Directory**: `/Users/ueli/Documents/semio/compose/client/lib/rb/`
**Status**: Present but not examined in detail

---

## 9. Selection Shape Across Fixtures

### capsule-dream.dragged.selection.compose.json
**Path**: `/Users/ueli/Documents/semio/compose/fixture/capsule-dream.dragged.selection.compose.json`

**Structure** (lines 1–7):
```json
{
  "pieces": [{"id": "277768b5-9220-4312-bf0d-ab82d9fb6a73"}],
  "connections": []
}
```

**Represents**: A user has dragged one piece; no connections selected.

### representation.selection.compose.json
**Path**: `/Users/ueli/Documents/semio/compose/fixture/representation.selection.compose.json`

**Structure** (lines 1–104):
```json
{
  "cases": [
    {
      "name": "default_representation_when_no_tags_are_selected",
      "selectedTagIds": [],
      "representations": [{"id": "...", "fileId": "...", "tagIds": [...]}],
      "expectedId": "representation-default"
    }
  ]
}
```

**Represents**: NOT a copy/paste selection. This is a **representation selection test** — choosing which 3D/image file to use for a piece type based on tag matching. Different domain.

### Canonical Selection Shape

**ONE canonical minimal selection structure in compose**:
```json
{
  "pieces": [{"id": "..."}, ...],
  "connections": [{"id": "..."}, ...]
}
```

**No variant shapes found**; same structure used everywhere for copy/paste operations.

---

## 10. Tests Exercising Copy/Paste

**Go** (`main_test.go:2029–2233`):
- `TestCopyAndPaste` → `test("Nakagin Capsule Tower", func(t *testing.T))`
  - Loads selection
  - Calls `CopyDesign(kit, design, pieceIds, connectionIds)`
  - Verifies piece count, IDs, attributes (`compose.center`, `compose.plane`, `compose.piece.origin`)
  - Calls `PasteDesign(kit, copyOutput, targetDesign, "original", nil)`
  - Verifies added pieces & connections match expected
  - Calls `PasteDesign(..., {u: 10, v: 10})`
  - Verifies centers are offset by coordinate

**Python** (line 20209+):
- `test_copy(case)` — tests copy operation
- `test_paste_without_coordinate(case)` — tests paste with anchoring="original", no coordinate
- `test_paste_with_coordinate(case)` — tests paste with coordinate offset

**Rust**: No direct tests found; tests likely run via Python/Go harness calling GraphQL backend.

---

## Summary: Critical Findings for Migration

| Item | Finding |
|------|---------|
| **ID Determinism** | **NONDETERMINISTIC**: IDs are random 128-bit hex. Pasted pieces keep source IDs unchanged. Caller must remap IDs before/after applying diff. |
| **ID Map Serialization** | **NOT PROVIDED**: `PasteDesign` returns only `DesignDiff`, no ID remapping table. Migration must store or recompute the mapping. |
| **Selection Payload** | **Minimal**: Only `pieces[]` and `connections[]` with IDs. No types, ports, or dependency closure. |
| **Copy Closure Rule** | **Explicit + Auto-external**: Copy includes selected pieces/connections. External pieces referenced by non-internal connections are auto-added with `compose.piece.origin = "external"`. |
| **Coordinate Behavior** | **Additive offset**: After anchor adjustment, `coordinate` is added to all piece centers. Affects remapped stub-bridge connection u/v. |
| **Canonical Selection** | **One shape**: All fixtures use `{pieces: [{id}], connections: [{id}]}`. No variants. |
| **Language Parity** | **Go/Python identical**; Rust/TS via GraphQL; .NET/Ruby present but logic assumed parallel. |
| **Fixture Chain** | Selection → Copy → Paste → PasteDiff (→ PasteDiff+Coordinate). Each output feeds next operation. |

---

## References (Absolute Paths)

- `/Users/ueli/Documents/semio/compose/fixture/copy-paste.cases.compose.json`
- `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.copy.design.selection.compose.json`
- `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.copy.design.compose.json`
- `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.paste.design.compose.json`
- `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.paste.design.diff.compose.json`
- `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.paste.with-coordinate.design.diff.compose.json`
- `/Users/ueli/Documents/semio/compose/fixture/capsule-dream.dragged.selection.compose.json`
- `/Users/ueli/Documents/semio/compose/fixture/representation.selection.compose.json`
- `/Users/ueli/Documents/semio/compose/client/lib/go/main.go:274–278` (Id() function)
- `/Users/ueli/Documents/semio/compose/client/lib/go/main.go:12451–12607` (CopyDesign)
- `/Users/ueli/Documents/semio/compose/client/lib/go/main.go:12617–13017` (PasteDesign)
- `/Users/ueli/Documents/semio/compose/client/lib/go/main_test.go:2029–2233` (TestCopyAndPaste)
- `/Users/ueli/Documents/semio/compose/client/lib/py/main.py:12407–12535` (copyDesignDict)
- `/Users/ueli/Documents/semio/compose/client/lib/py/main.py:12538–12850+` (pasteDesignDict)
- `/Users/ueli/Documents/semio/compose/client/lib/py/main.py:20209–20305+` (test_copy, test_paste_*)
