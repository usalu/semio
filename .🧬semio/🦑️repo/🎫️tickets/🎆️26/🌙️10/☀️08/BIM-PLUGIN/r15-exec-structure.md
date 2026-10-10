# WP21 Structural Analysis Hooks

Implementation in progress. Authored supports, load cases and loads use sparse typed diffs and concrete inverse leaves. Their graph result depends on existing storey, wall layout and solid nodes. No solver state or BIM module was introduced.

The local IfcOpenShell IFC4 schema was inspected for structural entity attribute names, select types and cardinality. StructuralAnalysisModel, CurveMember, SurfaceMember, PointConnection, BoundaryNodeCondition, LoadCase, Point/Curve/SurfaceAction, Single/Linear/PlanarForce and structural member/activity relationships were checked using the installed .venv schema registry.

The IFC projection reports distributed supports reduced to boundary point connections and area moments unsupported by IFC planar-force entities; exact range/moments remain in the solver JSON projection. Validation has not yet run.
