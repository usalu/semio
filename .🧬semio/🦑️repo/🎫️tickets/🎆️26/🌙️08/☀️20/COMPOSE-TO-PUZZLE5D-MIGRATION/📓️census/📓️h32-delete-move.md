# DELETE and MOVE/DRAG Census — Compose H32/H33

## Delete

### 1. Fixture Breakdown

**File:** `/Users/ueli/Documents/semio/compose/fixture/delete.cases.compose.json`
- **Case count:** 1 case
- **Case name:** `nakagin_capsule_tower_third_tambour_and_first_small_tower_connection`
- **Design name:** Nakagin Capsule Tower
- **Selection asset:** `nakagin-capsule-tower.deleted.selection.compose.json`
- **Expected diff asset:** `nakagin-capsule-tower.deleted.design.diff.compose.json`

**File:** `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.deleted.selection.compose.json`
- **Input shape:** Object with `pieces` (array) and `connections` (array)
- **JSON keys:** `pieces`, `connections`, each containing objects with `id` field
- **Encodes:** Selection for deletion
  - 1 piece to delete: `8ade0ad2-14e3-469d-822b-6ba08b4f2f2b`
  - 1 connection to delete: `8684b51c-94f5-4a50-be16-526ac8e0892a`

**File:** `/Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.deleted.design.diff.compose.json`
- **Output shape:** Diff with pieces (removed + updated) and connections (removed)
- **JSON keys:** `pieces.removed`, `pieces.updated`, `connections.removed`
- **Encodes:** Result of deletion
  - **Pieces removed:** 1 piece
  - **Pieces updated:** 10 pieces (all with both `plane` and `center` changes, indicating they became fixed)
  - **Connections removed:** 11 connections (1 explicit + 10 stale connections derived from the deleted piece)

### 2. Rust Implementation Signatures

**File:** `/Users/ueli/Documents/semio/compose/client/lib/rs/lib.rs`

- **Line 4912:** `delete_piece_by_external_id(&self, piece_id: &Id) -> Result<(), ComposeError>`
  - Low-level utility that removes piece from design's pieces list and external-id index
  - Returns error if piece not found (idempotent: not idempotent, errors on missing piece)

- **Line 18154:** `delete_piece(&self, ctx, id) -> ResponseInterface`
  - GraphQL mutation stub (marked not_implemented)
  - Single piece delete

- **Line 18158:** `delete_pieces(&self, ctx, ids: Vec<Id>) -> ResponseInterface`
  - GraphQL mutation stub (marked not_implemented)
  - Multiple pieces delete

- **Line 18163:** `delete_pieces_and_connections(&self, ctx, piece_ids: Vec<Id>, connection_ids: Vec<Id>) -> ResponseInterface`
  - GraphQL mutation stub (marked not_implemented)
  - Paired piece and connection delete

### 3. Go Implementation: Cascade Logic

**File:** `/Users/ueli/Documents/semio/compose/client/lib/go/main.go` lines 2491–2637

**Function:** `DeletePiecesAndConnectionsInDesign(kit *Kit, design Design, pieceIds []string, connectionIds []string) ComposeReport[DesignDiff]`

**Cascade order and logic:**

1. **CONNECTIONS REMOVED FIRST** (lines 2500–2515):
   - Identify "stale connections": any connection touching a deleted piece (parent OR child)
   - Combine stale connections with explicitly requested deletions
   - Result: `allRemovedConnectionIds` (union of explicit + stale)

2. **PIECES REMOVED** (lines 2550–2552):
   - Remove the explicitly requested pieces from the diff

3. **DERIVED PIECES UPDATE** (lines 2517–2610):
   - Identify pieces that "become fixed": pieces whose parent connection was removed
   - A piece becomes fixed only if:
     - It had a parent connection being removed (line 2520–2530)
     - It is NOT itself being deleted (line 2531–2533)
     - It has no other parent connection outside the removal set (line 2535–2541)
   - For each piece becoming fixed:
     - Flatten the design to get absolute plane/center (line 2556–2566)
     - Use flattened plane and center as the new fixed values (line 2584–2609)

**Idempotency:** NOT idempotent. No error handler shown for nonexistent pieces; the function does not validate that all pieceIds exist before processing.

**Quote from code (lines 2500–2506):**
```go
// Find stale connections: connections referencing any deleted piece
staleConnectionIds := make(map[string]bool)
for _, conn := range design.Connections {
    if deletedPieceSet[conn.Parent.Piece.Id] || deletedPieceSet[conn.Child.Piece.Id] {
        staleConnectionIds[conn.Id] = true
    }
}
```

**Test evidence** (lines 1345–1497 in `main_test.go`):
- Test calls `DeletePiecesAndConnectionsInDesign` with 1 piece and 1 connection
- Verifies removed pieces match selection
- Verifies 11 total connections removed (1 explicit + 10 stale)
- Verifies 10 pieces updated with both plane and center changes

## Move / Drag

### 6. Fixture Files and Content

**Directory:** `/Users/ueli/Documents/semio/compose/fixture/move/`
- `diff.design.compose.json`: Result of move operation with plane/shift/rise updates
- `story.design.compose.json`: Reference design with pieces and their poses
- `vector.compose.json`: Move input { gap: 2, shift: -1, rise: 0 }

**Directory:** `/Users/ueli/Documents/semio/compose/fixture/drag/`
- `design.compose.json`: Full design before drag
- `pieces.compose.json`: Pieces to drag (6 pieces total; b0, b1, and connection/type pieces)
- `offset.compose.json`: Drag offset { u: -1, v: 2 }
- `diff.design.compose.json`: Result with center updates on pieces and u/v updates on connections
- `diff.design.free.compose.json`: Empty object (edge case for drag with no changes)
- `pieces.png`, `design.png`: Visual references

### 7. Rust Implementation Signatures

**File:** `/Users/ueli/Documents/semio/compose/client/lib/rs/lib.rs`

**Single piece drag:**
- **Line 18193:** `drag(&self, ctx: &Context, offset: OffsetInput) -> ResponseInterface`
  - Scope: `Scope::PieceInDesign { design_id, piece_id }`
  - Input: `Input::Offset { offset }` (u, v coordinates)
  - Operation: `DragPieceInDesign`

**Multiple pieces drag:**
- **Line 18248:** `drag(&self, ctx: &Context, offset: OffsetInput) -> ResponseInterface`
  - Scope: `Scope::PiecesInDesign { design_id, piece_ids }`
  - Input: `Input::Offset { offset }` (u, v coordinates)
  - Operation: `DragPiecesInDesign`

**Single piece move (not implemented):**
- **Line 18210:** `r#move(&self, ctx: &Context, position: PositionInput) -> ResponseInterface`
  - Scope: `Scope::PieceInDesign`
  - Input: `Input::Position`
  - Marked: not_implemented

**Multiple pieces move (not implemented):**
- **Line 18265:** `r#move(&self, ctx: &Context, offset: OffsetInput) -> ResponseInterface`
  - Scope: `Scope::PiecesInDesign`
  - Input: `Input::Offset`
  - Marked: not_implemented

### 8. Coordinate System Distinction: 2D Diagram vs 3D World

**DRAG operates in 2D diagram plane:**
- Input: `offset.compose.json` contains `{ u: -1, v: 2 }`
- These are diagram coordinates (u, v on the piece's local 2D plane)
- Result in `diff.design.compose.json`: Only `center` field updated (u, v)
- Connections also update only `u`, `v` fields

**Example from fixture:**
```json
// Drag input
{ "u": -1, "v": 2 }

// Piece update result
{ "piece": { "id": "e2faf536..." }, "diff": { "center": { "u": 3.82491, "v": 2.1833 } } }

// Connection update result
{ "connection": { "id": "b9c2feb0..." }, "diff": { "u": -1, "v": 2 } }
```

**MOVE operates in 3D world coordinates:**
- Input: `vector.compose.json` contains `{ gap: 2, shift: -1, rise: 0 }`
- These are world coordinates (shift/rise in world space, gap for layout)
- Result in `diff.design.compose.json`: Full `plane` (origin + xAxis + yAxis) updated
- Connections update `shift` and `rise` fields

**Example from fixture:**
```json
// Move input
{ "gap": 2, "shift": -1, "rise": 0 }

// Piece update result
{ "piece": { "id": "e2faf536..." }, "diff": { "plane": { "origin": { "x": 6.5, "y": 9.7, "z": -7.5 }, "xAxis": {...}, "yAxis": {...} } } }

// Connection update result
{ "connection": { "id": "b9c2feb0..." }, "diff": { "shift": -1, "rise": -2 } }
```

### 9. FIXED vs LINKED Pieces: Movement and Legality Rules

**File:** `/Users/ueli/Documents/semio/compose/client/lib/rs/lib.rs`

**Fixed piece definition** (line 1388–1390):
```rust
async fn piece_is_fixed(piece: &Arc<Piece>) -> bool {
    matches!(*piece.connection_kind.read().await, Some(crate::kit::design::piece::PieceConnectionKind::Fixed))
}
```

**Piece types and their behavior:**

1. **FIXED pieces** (marked `PieceConnectionKind::Fixed`):
   - Have explicit, anchored pose (plane + center)
   - Created via `addFixedPiece` operation (line 18087)
   - When a fixed piece moves via drag, only its `center` (u, v) changes
   - When a fixed piece moves via pose, full plane and center change
   - Quote from line 6053–6055:
     ```rust
     if pdiff.fix_piece {
         *piece.connection_kind.write().await = Some(design::piece::PieceConnectionKind::Fixed);
         return Ok(());
     }
     ```

2. **LINKED/DERIVED pieces** (not marked Fixed):
   - Position is derived from parent piece via connection geometry
   - In flattening (line 1437–1443), fixed pieces use stored plane; derived pieces get default plane but derived center
   - When parent piece moves, derived pieces' positions recompute via connection math
   - When a derived piece is moved directly via drag (line 6057–6072), it updates its center coordinate
   - Quote from flatten logic (line 1437–1443):
     ```rust
     if piece_is_fixed(root_piece).await {
         piece_planes.insert(root_id.to_string(), pos.plane);  // Use stored plane
         piece_centers.insert(root_id.to_string(), pos.center);
     } else {
         piece_planes.insert(root_id.to_string(), PlaneInput::default());  // Default plane
         piece_centers.insert(root_id.to_string(), pos.center);
     }
     ```

**Movement rules:**

- **Moving a FIXED piece:** LEGAL. Directly updates its plane/center.
- **Moving a LINKED piece directly:** LEGAL (via drag). Updates its center on the 2D plane. This breaks the connection-derived computation (piece no longer aligned to parent).
- **Parent move cascades to children:** When a fixed piece moves, its linked children recompute their positions via connection geometry (line 1472 in flatten: `compute_child_plane`).
- **Connection becomes anchor when parent is deleted:** A linked piece that loses its parent connection via deletion becomes fixed, getting its absolute plane/center from the flattened design (lines 2517–2610 in main.go).

### 10. Tests Exercising Delete and Move/Drag

**Go tests** (file: `/Users/ueli/Documents/semio/compose/client/lib/go/main_test.go`):

- **TestDelete** (lines 1345–1497):
  - Tests `DeletePiecesAndConnectionsInDesign` with Nakagin Capsule Tower
  - Loads selection from `nakagin-capsule-tower.deleted.selection.compose.json`
  - Verifies removed pieces, updated pieces (with plane + center), and removed connections
  - Asserts 1 piece removed, 11 connections removed, 10 pieces updated

- **TestDrag** (lines 1499–1544):
  - Tests `DragPiecesInDesign` with drag/design.compose.json
  - Loads pieces, design, offset
  - Verifies piece center updates and connection u/v updates
  - Asserts expected center diffs match computed

**Integration tests** (file: `/Users/ueli/Documents/semio/compose/client/ui/desktop/test/suite/index.mjs`):
- Electron integration suite (lines 1–120+)
- Tests renderer load and console message validation
- No specific delete/drag test visible in excerpt

---

**Report generated:** 2026-08-20
