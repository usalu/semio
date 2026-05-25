# Command

# Geometry

Topology: A Topology is an abstract superclass.

## Editable

Vertex: A Vertex is a zero-dimensional entity equivalent to a geometry point.
Edge: An Edge is a one-dimensional entity defined by two vertices. It is important to note that while a topologic edge is made of two vertices, its geometry can be a curve with multiple control vertices.
Wire: A Wire is a contiguous collection of Edges where adjacent Edges are connected by shared Vertices. It may be open or closed and may be manifold or non-manifold.
Face: A Face is a two-dimensional region defined by a collection of closed Wires. The geometry of a face can be flat or undulating.
Shell: A Shell is a contiguous collection of Faces, where adjacent Faces are connected by shared Edges. It may be open or closed and may be manifold or non-manifold.
Cell: A Cell is a three-dimensional region defined by a collection of closed Shells. It may be manifold or non- manifold.
CellComplex: A CellComplex is a contiguous collection of Cells where adjacent Cells are connected by shared Faces. It is non-manifold.
Cluster: A Cluster is a collection of any topologic entities. It may be contiguous or not and may be manifold or non-manifold. Clusters can be nested within other Clusters.

Anchor: An Anchor is a parameteric point. It can be attached to a Vertex (no parameter needed), an Edge or Wire (parameter t needed), a Face (parameter u,v needed), a Cell (parameter u,v,w needed).

## Non-editable

Surfaces are derived faces that are a combination of Exposure (External or Internal) and Stance (Horizontal or Vertical).
e.g. two coplanar faces are merged into a single surface
e.g. when two cells intersect the surface will not be the complete face but it is split into external and internal faces.
Surfaces are just a different way of "splitting the faces semantically". They are shape-invariant.

Parts are derived cells that are a combination of Overlap (None, Difference, Intersection)
Parts are just a different way of "splitting the cells semantically". They are shape-invariant.

Run one boolean intersection on all cells at the same time. Return them as intersection parts. Run for every cell boolean difference where the cutters are all the other intersecting cells. Return them as difference parts.
Explode all parts and check wheater they are internal (inside other parts) or external, and check wheather the vertices of the parts are mostly horizontal or vertical. All parts and surfaces are made out of vertices that already exist.

Volumes are derived CellGroup

Skin is the
