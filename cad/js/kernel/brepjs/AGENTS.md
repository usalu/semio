# Kernel (brepjs)

Private brepjs persistence graph used inside `@cad/js/kernel/brepjs`. Not part of the public spatial framework vocabulary.

## Raw (editable, kernel-private)

Vertex: zero-dimensional point.
Edge: one-dimensional entity between two vertices; geometry may be a multi-control-point curve.
Wire: contiguous edges sharing vertices; open or closed; manifold or non-manifold.
Face: two-dimensional region from closed wires; flat or undulating geometry.
Shell: contiguous faces sharing edges; open or closed.
Cell: three-dimensional region from closed shells.
CellComplex: contiguous cells sharing faces.
Cluster: arbitrary nested membership of brepjs entities.

Anchor: parametric point on vertex, edge/wire (t), face (u,v), or cell (u,v,w).

## Analytic (non-editable, kernel-private)

Surfaces: derived faces from exposure and stance (external/internal, horizontal/vertical).
Parts: derived closed shells from overlap semantics (none, difference, intersection).
Volumes: boolean union of closed shells in a cell group.

## Construct query (kernel)

Raw brepjs entities are matched only inside kernel query adapters. Public construct queries use `MATCH (Object …)` and `CALL view.<viewId>.<derivedObjectId>({})`.
