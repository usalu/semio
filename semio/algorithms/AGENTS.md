---
technology: semio
bundle:
 name: algorithms
 emoji: 🧬
 description: The algorithms bundle for semio.
 kind: library
---

# 🧾 Specification

## Strict layering (algorithms Storybook)

For in-browser **TypeScript** algorithm paths, prefer **`@semio/js` `KitStore`** (then GraphQL → `semio/rs`) for kit reads/writes. For **UI** and shared DTO shapes, use **`@semio/react`** re-exports — do not skip react when the surface is hook-oriented.

Native REST / stdin bridges MUST target **`semio/rs`** implementations, not duplicated graph math in TypeScript.

## 🕸️ Systems

- **Algorithm Visualization System**: Storybook-based UI shell that renders algorithm inputs and outputs using the standardized `AlgorithmApp` from `@semio/ui`.

## 🧮 Algorithms

- **Cluster**: Clusters multiple selected pieces into a single design-id designPiece.
- **Copy/Paste**: Copies selected pieces and connections into a clipboard design, then pastes them onto a target design.
- **Delete**: Removes selected pieces from a design.
- **Drag**: Offsets center for all fixed pieces (pieces with center and plane) by drag offset (piece diff). If a selected piece is a descendant of another selected piece then it is ignored. Otherwise adds the drag offset to the parent connection (connection diff).
- **Find Replaceable Types In Designs**: Suggests compatible replacement types and descendant designs for a piece selection.
- **Flatten**: Flattens a nested design into a flat collection of pieces and connections.
- **Move**: Repositions selected pieces to a 2D location defined by a vector.

## 🛠️ Mechanisms

- **AlgorithmApp Shell**: Standardized golden-layout configuration that wires context state to specialized windows (VecInput, PiecesSelectionInput, etc.).
- **Language Provider**: Storybook global decorator that injects the selected implementation language into the algorithm context.
- **Native Rust bridge** ([`native-bridges/rs`](native-bridges/rs)): stdin JSON ops against the **`semio`** crate use **`KitGraph` / `KitGraphRef`** (in-memory graph); `flatten` calls **`KitGraph::flatten_design_async`**. This matches the split where **`KitStore`** names the async control plane in `semio-store`, not the graph handle.

## 📛 Entities
