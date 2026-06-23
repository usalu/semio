# Merkle Hash Optimization for flattenDesign

## 🔑 Idea

`flattenDesign` walks the piece graph with BFS from fixed pieces and propagates
absolute planes and 2D centers along each connection. The computation per piece
is:

- `plane(piece)` depends only on the *placement chain* from the nearest fixed
  ancestor: fixed plane of the component root and, for each edge in the chain,
  the parent connector geometry, the child connector geometry and the
  `gap / shift / rise / rotation / turn / tilt` of the connection.
- `center(piece)` depends only on the fixed center of the component root and,
  for each edge in the chain, the parent connector's `direction.z`, the parent
  connector's `t` and the connection's `u / v`.

Nothing else matters. Therefore the plane and the center of every piece can be
individually represented as the leaf of a merkle tree whose chain hash encodes
exactly those inputs.

When a second `flattenDesign` call comes in with a cache from the previous run,
any piece whose merkle inputs are identical reuses the cached plane / center;
only pieces whose chain changed are recomputed.

## 🪟 Hash Inputs (per language, identical across all)

All hashes are lower-case hex `sha256`. Numbers are serialised with
`formatNumberForHash` (integers without decimals, other numbers via the
shortest round-trip decimal representation) so every implementation produces
byte-identical bytes before hashing.

### PlaneHash(piece)

- If piece is a component root:
  `sha256("plane.root" || guid || plane.origin.x,y,z || plane.xAxis.x,y,z || plane.yAxis.x,y,z)`
  (if piece has no fixed plane: `"plane.root.identity"` + guid).
- Else:
  `sha256("plane.chain" || parent.PlaneHash || parentConnector.point.x,y,z || parentConnector.direction.x,y,z || childConnector.point.x,y,z || childConnector.direction.x,y,z || conn.gap || conn.shift || conn.rise || conn.rotation || conn.turn || conn.tilt)`.

### CenterHash(piece)

- If piece is a component root:
  `sha256("center.root" || guid || center.u || center.v)` (identity `0,0` when missing).
- Else:
  `sha256("center.chain" || parent.CenterHash || parentConnector.direction.z || parentConnector.t || conn.u || conn.v)`.

## 🧪 Tests

All five implementations add the same unit that loads
`compose/assets/compose/flatten-merkle.cases.compose.json` and for each case:

1. Runs `flattenDesign` once, capturing the hash map.
2. Mutates the kit in a deterministic way described in the case, runs again.
3. Asserts exactly the listed piece guids have a changed `planeHash` /
   `centerHash`, the rest stay byte-identical.
