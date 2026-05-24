# Factory

A factory is

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

## Non-editable

Surfaces are derived faces that are a combination of Exposure (External or Internal) and Stance (Horizontal or Vertical).
e.g. two coplanar faces are merged into a single surface
e.g. when two cells intersect the surface will not be the complete face but it is split into external and internal faces.
Surfaces are just a different way of "splitting the faces semantically". e.g. the total area or shape doesnt change

Parts are derived cells that are a combination of Overlap (None, Difference, Intersection)
Parts are just a different way of "splitting the cells semantically". e.g. the total volume or shape doesnt change
