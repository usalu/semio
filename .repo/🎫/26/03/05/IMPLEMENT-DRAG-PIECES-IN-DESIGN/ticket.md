# Implement Drag Pieces in Design

## Goal
SKETCHPAD-IMPROVEMENTS

## Status
closed

## Prompt
Implement dragPiecesInDesign function across all programming languages (JS/TS, Python, Go, Rust, C#). Add Design/Drag tests using asset files. Update sketchpad to use the drag function for node dragging.

## Plan
1. Read existing code in all languages to understand Design/DesignDiff/Piece structures
2. Read asset files (design.json, pieces.json, offset.json, diff_design.json, diff_design_free.json) for test data
3. Implement dragPiecesInDesign in JS/TS (semio.ts)
4. Implement dragPiecesInDesign in Python (semio.py)
5. Implement dragPiecesInDesign in Go (semio.go)
6. Implement dragPiecesInDesign in Rust (semio.rs)
7. Implement dragPiecesInDesign in C# (Semio.cs / Semio.Grasshopper.cs)
8. Add tests for each language using asset data
9. Update sketchpad Design.tsx to use dragPiecesInDesign for node dragging
10. Run all tests and validate

## TODOs
- [x] Read existing code structures
- [x] Read asset files for test I/O
- [x] Implement JS/TS dragPiecesInDesign
- [x] Implement Python dragPiecesInDesign
- [x] Implement Go dragPiecesInDesign
- [x] Implement Rust dragPiecesInDesign
- [x] Implement C# dragPiecesInDesign
- [x] Add JS/TS test
- [x] Add Python test
- [x] Add Go test
- [x] Add Rust test
- [x] Add C# test
- [x] Update sketchpad drag to use function
- [x] Run all tests

## Changes
- semio/assets/index.ts: Added drag asset exports (DragDesign, DragPieces, DragOffset, DragDiffDesign, DragDiffDesignFree)
- semio/assets/semio/drag/diff_design_free.json: Fixed empty file → `{}`
- semio/js/semio.ts: Added `dragPiecesInDesign(design, pieces, offset): DesignDiff` before `#endregion Design`
- semio/js/semio.test.ts: Added Drag test describe block with test for design diff
- semio/go/semio.go: Added `DragPiecesInDesign` function before `#endregion Flatten Design`
- semio/go/semio_test.go: Added `TestDrag` between TestFlatten and TestDiff
- semio/py/semio.py: Added `dragPiecesInDesignDict` after `flattenDesignDict`
- semio/engine/engine.py: Added `drag_pieces_in_design` MCP tool + import
- semio/engine/engine.test.py: Fixed ASSETS_DIR path, added drag test
- semio/rs/semio.rs: Added `drag_pieces_in_design` function + drag test module
- semio/net/Semio/Semio.cs: Added `Design.DragPiecesInDesign` static method
- semio/net/Semio.Tests/Tests.cs: Added `Drag` test class with xUnit Fact
- semio/js/sketchpad/Design.tsx: Imported `dragPiecesInDesign`, added `dragStartPositionRef`, refactored `onNodeDragStop` to use `dragPiecesInDesign` for piece and connection diff computation

## Findings

### Type Definitions in semio/js/semio.ts

#### Coord (center type) — Line 1259
```typescript
export const CoordSchema = z.object({ u: z.number(), v: z.number() });
export type Coord = z.infer<typeof CoordSchema>;
export const CoordDiffSchema = CoordSchema.partial();
export type CoordDiff = z.infer<typeof CoordDiffSchema>;
```

#### Piece — Line 4900-4920
```typescript
export const PieceSchema = z.object({
  guid: z.string(),
  name: z.string().optional(),
  type: TypeIdSchema.optional(),
  design: DesignIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: CoordSchema.optional(),
  scale: z.number().optional(),
  mirrorPlane: PlaneSchema.optional(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Piece = z.infer<typeof PieceSchema>;
```

#### PieceDiff — Line 4943-4953
```typescript
export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, props: true, attributes: true }).extend({
  plane: PlaneDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
```

#### PiecesDiff — Line 5063-5073
```typescript
export const PiecesDiffSchema = z.object({
  removed: z.array(PieceIdSchema).optional(),
  updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(),
  added: z.array(PieceSchema).optional(),
});
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;
```

#### Connection — Line 5468-5486
```typescript
export const ConnectionSchema = z.object({
  guid: z.string(),
  connected: SideSchema,
  connecting: SideSchema,
  gap: z.number().optional(),
  shift: z.number().optional(),
  rise: z.number().optional(),
  rotation: z.number().optional(),
  turn: z.number().optional(),
  tilt: z.number().optional(),
  u: z.number().optional(),
  v: z.number().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type Connection = z.infer<typeof ConnectionSchema>;
```

#### ConnectionDiff — Line 5493-5502
```typescript
export const ConnectionDiffSchema = ConnectionSchema.partial().omit({ guid: true, connected: true, connecting: true, attributes: true }).extend({
  connected: SideDiffSchema.optional(),
  connecting: SideDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
```

#### Design — Line 5834-5864
```typescript
export const DesignSchema = z.object({
  guid: z.string(),
  name: z.string(),
  parent: DesignIdSchema.optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  pieces: z.array(PieceSchema).optional(),
  connections: z.array(ConnectionSchema).optional(),
  stats: z.array(StatSchema).optional(),
  props: z.array(PropSchema).optional(),
  layers: z.array(LayerSchema).optional(),
  activeLayer: LayerIdSchema.optional(),
  groups: z.array(GroupSchema).optional(),
  canScale: z.boolean().optional(),
  canMirror: z.boolean().optional(),
  unit: z.string().optional(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(ConceptIdSchema).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type Design = z.infer<typeof DesignSchema>;
```

#### DesignDiff — Line 5916-5936
```typescript
export const DesignDiffSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, authors: true, attributes: true }).partial().extend({
  pieces: PiecesDiffSchema.optional(),
  connections: ConnectionsDiffSchema.optional(),
  stats: StatsDiffSchema.optional(),
  props: PropsDiffSchema.optional(),
  layers: LayersDiffSchema.optional(),
  groups: GroupsDiffSchema.optional(),
  authors: AuthorsDiffSchema.optional(),
  attributes: AttributesDiffSchema.optional(),
});
export type DesignDiff = z.infer<typeof DesignDiffSchema>;
```

### Region Structure
- `// #region 🔖Design` starts at **line 5822**
- `// #endregion 🔖Design` ends at **line 7099**
- `flattenDesign` at **line 6425**
- `removePiecesAndConnectionsFromDesign` at **line 6340**
- Last function before `#endregion`: `findStaleConnectionsInDesign` ending at **line 7097**

### Existing Related Functions (no drag/move found)
- `removePieceFromDesignDiff` (line 6071) — removes single piece from diff
- `removePiecesFromDesignDiff` (line 6124) — removes multiple pieces from diff
- `removePiecesAndConnectionsFromDesign` (line 6340) — returns DesignDiff removing pieces + connections
- `flattenDesign` (line 6425) — flattens nested design structure
- `findPieceInDesign` — finds piece by guid
- `findPieceConnectionsInDesign` (line 7060) — finds connections for a piece
- `findStaleConnectionsInDesign` (line 7088) — finds orphaned connections

### Best Insertion Point for `dragPiecesInDesign`
**Before `// #endregion 🔖Design` at line 7099**, after `findStaleConnectionsInDesign`. The function should:
- Take `pieceIds: string[]` and a delta `{ du: number, dv: number }`
- Return `DesignDiff` with `pieces.updated` containing `PieceDiff` entries that update `center`
- Follow the same pattern as `removePiecesAndConnectionsFromDesign`

## Summary

Implemented `dragPiecesInDesign` across all 5 languages (JS/TS, Go, Python, Rust, C#) with tests using drag asset files. The function computes a DesignDiff from a design, selected pieces, and UV offset by: finding root movers (selected pieces with centers), BFS to find moving set (root movers + descendants), and computing offset diffs for root mover centers and orphan connection u/v values. Integrated in the sketchpad's `onNodeDragStop` to replace manual piece/descendant update computation and added connection u/v update support.