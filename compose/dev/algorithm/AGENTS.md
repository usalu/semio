---
technology: compose
bundle:
 name: algorithms
 emoji: 🧬
 description: The algorithms bundle for compose.
 kind: library
---

# 🧾 Specification

## Strict layering (algorithms Storybook)

All algorithm story paths use **`@compose/js` `Session`** (`openSessionInMemory` + `Store.installProjection`) as the browser-facing `compose/rs` GraphQL wire. For **UI** and shared DTO shapes, use **`@compose/react`** and **`@compose/ui`** re-exports.

## 🕸️ Systems

- **Algorithm Visualization System**: Storybook-based UI shell that renders algorithm inputs and outputs using the standardized `AlgorithmApp` from `@compose/ui`.

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
- **Single Source Runner**: Storybook helpers call `@compose/js` `KitStore`, which re-exports the `compose/rs` WASM implementation. Do not add native bridge, REST, stdin, or multi-language adapter paths here.

## 📛 Entities
