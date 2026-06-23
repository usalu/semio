---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Bulk close
## Plan

1. Map out all pieces and connections from design.json
2. Identify selected (dragged) pieces from pieces.json
3. Build parent-child tree from connections
4. Trace which pieces are "root movers" (selected + have center)
5. Trace which pieces are "implicitly moving" (descendants of root movers)
6. Determine which connections need adjustment
7. Verify against expected diff_design.json output

## Analysis

### Data Model

In compose, connections define parent-child relationships:
- **connecting** piece = parent
- **connected** piece = child
- The connection's `u`/`v` encode the child's relative diagram position to its parent

### Piece Inventory (design.json)

| GUID (short) | Name | Has Center? |
|---|---|---|
| `e2faf536` | b0 | Yes (u:4.82, v:0.18) |
| `e72975ab` | t_fs_b0_c0 | No |
| `ade12a05` | t_b0_c0 | No |
| `ed7b6b3c` | ci_bs_b0_c0 | No |
| `a7a6c527` | t_fs_b0_c1 | No |
| `3910be91` | b1 | Yes (u:12.75, v:0.04) |
| `f0b8dd4a` | b2 | Yes (u:11.11, v:4.11) |
| `b6aff18a` | t_fs_b2_c0 | No |
| `91b889a0` | b3 | Yes |
| `c52879d5` | t_f0_b0_c0 | No |
| `d885ae2c` | b4 | Yes |
| `99969c34` | t_fs_b4_c1 | No |

### Connection Tree (parent → child)

| Connection GUID | Child | Parent |
|---|---|---|
| `7296fbd8` | t_fs_b0_c0 | **b0** |
| `8f3dd453` | t_b0_c0 | t_fs_b0_c0 |
| `1fc6cbb2` | ci_bs_b0_c0 | t_b0_c0 |
| `23886666` | t_fs_b0_c1 | **b0** |
| `b9c2feb0` | t_fs_b2_c0 | **b2** |
| `cd1fa979` | t_f0_b0_c0 | t_b0_c0 |
| `7f946e5e` | t_fs_b4_c1 | **b4** |

This gives the following subtrees:
```
b0
├── t_fs_b0_c0
│   └── t_b0_c0
│       ├── ci_bs_b0_c0
│       └── t_f0_b0_c0
└── t_fs_b0_c1

b1 (isolated, no connections)

b2
└── t_fs_b2_c0

b4
└── t_fs_b4_c1
```

### Selected (Dragged) Pieces (pieces.json)

- `e2faf536` (**b0**) — has center
- `ed7b6b3c` (ci_bs_b0_c0) — no center
- `a7a6c527` (t_fs_b0_c1) — no center
- `3910be91` (**b1**) — has center
- `b6aff18a` (t_fs_b2_c0) — no center
- `c52879d5` (t_f0_b0_c0) — no center

### Offset

`{u: -1, v: 2}`

---

### Algorithm: `dragPiecesInDesign(design, pieces, offset) -> DesignDiff`

#### Step 1: Identify "Root Movers"

Root movers = selected pieces that have a `center`.

A piece with a center has an explicit diagram position. When dragged, its center is offset directly.

**Root movers:** b0, b1

#### Step 2: Offset Centers of Root Movers

For each root mover, emit a piece update with center offset `{u: -1, v: 2}`.

→ `diff.pieces.updated`: b0 (center +offset), b1 (center +offset) ✓

#### Step 3: Compute the "Moving Set"

The moving set = all pieces that will change absolute position as a result of the drag.

This includes:
1. Root movers themselves (b0, b1)
2. All descendants of root movers in the connection tree

Descendants of **b0**: t_fs_b0_c0 → t_b0_c0 → ci_bs_b0_c0, t_f0_b0_c0; t_fs_b0_c1
Descendants of **b1**: (none)

**Moving set:** {b0, t_fs_b0_c0, t_b0_c0, ci_bs_b0_c0, t_f0_b0_c0, t_fs_b0_c1, b1}

Pieces NOT in the moving set: {b2, t_fs_b2_c0, b3, b4, t_fs_b4_c1}

#### Step 4: Find Connections that Need Adjustment

For each **selected** piece that is NOT already in the moving set:
- Find its "parent connection" (the connection where this piece is the `connected`/child)
- Offset that connection's `u`/`v` by the drag offset

Trace each selected piece:

| Selected Piece | In Moving Set? | Action |
|---|---|---|
| **b0** | Yes (root mover) | Skip — center already offset |
| ci_bs_b0_c0 | Yes (descendant of b0) | Skip — moves implicitly with b0 |
| t_fs_b0_c1 | Yes (descendant of b0) | Skip — moves implicitly with b0 |
| **b1** | Yes (root mover) | Skip — center already offset |
| **t_fs_b2_c0** | **No** | **Adjust parent connection** |
| t_f0_b0_c0 | Yes (descendant of b0) | Skip — moves implicitly with b0 |

Only **t_fs_b2_c0** needs connection adjustment. Its parent connection is `b9c2feb0` (parent=b2, which is NOT selected and NOT moving).

→ `diff.connections.updated`: `b9c2feb0` (u/v +offset) ✓

---

### Answers to Specific Questions

#### 1. Which pieces get their centers offset?

**Only b0 and b1** — the selected pieces that have a `center` property. Pieces without centers (ci_bs_b0_c0, t_fs_b0_c1, t_fs_b2_c0, t_f0_b0_c0) don't have diagram positions to offset.

#### 2. Why does connection `b9c2feb0` get offset?

Because its child (**t_fs_b2_c0**) is selected/dragged, but its parent (**b2**) is NOT selected and NOT a descendant of any root mover — b2 is "fixed." Since b2 stays in place, the only way to express that t_fs_b2_c0 moved is to adjust the connection's u/v (which encodes the child's position relative to the parent).

#### 3. Which pieces are ignored as "children of a moving parent"?

Three selected pieces are skipped because they are descendants of **b0** (a root mover):

- **ci_bs_b0_c0**: grandchild of b0 (b0 → t_fs_b0_c0 → t_b0_c0 → ci_bs_b0_c0). Moves implicitly when b0's center moves.
- **t_fs_b0_c1**: direct child of b0 (b0 → t_fs_b0_c1). Moves implicitly.
- **t_f0_b0_c0**: great-grandchild of b0 (b0 → t_fs_b0_c0 → t_b0_c0 → t_f0_b0_c0). Moves implicitly.

Their parent connections are NOT adjusted because both sides of the connection are in the moving set — the relative positions are preserved.

---

### Key Definitions

- **"Fixed" piece**: A piece that is NOT selected AND NOT a descendant of any selected piece with a center. It stays in place during the drag. Example: b2, b3, b4.

- **"Parent connection"**: The unique connection where a given piece appears as the `connected` (child) side. Each non-root piece has exactly one parent connection that defines its position relative to its parent.

- **"Root mover"**: A selected piece that has a `center`. Its center is directly offset. All its descendants in the connection tree move implicitly.

### Pseudocode

```
function dragPiecesInDesign(design, selectedPieces, offset):
  diff = { pieces: { updated: [] }, connections: { updated: [] } }

  rootMovers = selectedPieces.filter(p => p.center != null)

  for each rm in rootMovers:
    diff.pieces.updated.push({ piece: rm.guid, diff: { center: offset } })

  movingSet = new Set(rootMovers)
  for each rm in rootMovers:
    addAllDescendants(design.connections, rm, movingSet)

  for each sp in selectedPieces:
    if sp in movingSet:
      continue
    parentConn = design.connections.find(c => c.connected.piece.guid == sp.guid)
    if parentConn != null:
      diff.connections.updated.push({ connection: parentConn.guid, diff: { u: offset.u, v: offset.v } })

  return diff
```

## Changes

No code changes — analysis only.

## Todos

- [x] Map all pieces and connections
- [x] Build parent-child tree
- [x] Trace root movers and moving set
- [x] Verify against expected diff_design.json
- [x] Document algorithm
