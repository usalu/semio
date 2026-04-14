---
technology: semio
bundle:
 name: algorithms
 emoji: 🧬
 description: The algorithms bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

- **Algorithm Visualization System**: Storybook-based UI shell that renders algorithm inputs and outputs using the standardized `AlgorithmApp` from `@semio/ui`.

## 🧮 Algorithms

- **Cluster**: Clusters multiple selected pieces into a single design-guid designPiece.
- **Copy/Paste**: Copies selected pieces and connections into a clipboard design, then pastes them onto a target design.
- **Delete**: Removes selected pieces from a design.
- **Drag**: Offsets center for all fixed pieces (pieces with center and plane) by drag offset (piece diff). If a selected piece is a descendant of another selected piece then it is ignored. Otherwise adds the drag offset to the parent connection (connection diff).
- **Find Replaceable Types In Designs**: Suggests compatible replacement types and descendant designs for a piece selection.
- **Flatten**: Flattens a nested design into a flat collection of pieces and connections.
- **Move**: Repositions selected pieces to a 2D location defined by a vector.

## 🛠️ Mechanisms

- **AlgorithmApp Shell**: Standardized golden-layout configuration that wires context state to specialized windows (VecInput, PiecesSelectionInput, etc.).
- **Language Provider**: Storybook global decorator that injects the selected implementation language into the algorithm context.

## 📛 Entities
