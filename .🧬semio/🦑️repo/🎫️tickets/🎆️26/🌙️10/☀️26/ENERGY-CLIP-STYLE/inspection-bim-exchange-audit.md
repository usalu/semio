# BIM Exchange API Audit

Read-only snapshot of the actual new exchange leaf, current model core/data, schema and cases. No concurrent module was implemented or overwritten.

## Required Public Ports

- step_excerpt(tag, model=None, *, first_id=...) -> dict. The exact default first ID is not specified by existing fixtures. Calls at exchange leaf115 and176 pass B3 and a loaded model, then request a second export using first_id=step_first_id(excerpt)+20. Return lines as list of strings and spans as dictionaries with zero-based line/start/end and role. Spans must address the original lines; _pick filters/remaps line numbers and the panel renderer colors half-open character intervals.
- step_first_id(excerpt) -> int. Must return the first local STEP entity ID used by the excerpt so the second export can shift local references while retaining the same object GlobalId. Core export list currently omits this name; adding only the missing module will not expose it.
- mesh(tag, segments, model=None) -> dict. Leaf builds B3 meshes for segment counts6through16 and reads vertex_count, triangle_count, kept and lost. kept/lost entries may be localized en/de dictionaries or strings. Geometry fields must match the native mesh_view interface; no complete fixture for that new port was found in the inspected old cases.
- ids_check(*, source, model=None) -> dict. source is semantic or mesh. Leaf reads applicable and requirement; semantic applicability is every IfcColumn, and mesh is expected to expose no semantic IfcColumn applicability.

## STEP Story Semantics

Leaf requests lines containing ISO-10303-21, FILE_SCHEMA, IFCCOLUMN, IFCCARTESIANPOINT, IFCPROPERTYSET, IFCPROPERTYSINGLEVALUE, IFCMATERIAL, IFCCIRCLEPROFILEDEF and IFCEXTRUDEDAREASOLID. Required span roles are class, guid, placement, predefined, pset, value, material, profile and extrusion. The first property-single-value token is used for LoadBearing. IFC column class, stable GlobalId and placement must describe the same piloti across renumbered exports; line IDs are file-local. Profile radius and extrusion depth must match actual model geometry rather than unrelated wall constants.

Current model schema_id is IFC4X3_ADD2. B3 is IfcColumn/COLUMN, GUID0yoQlyTGDVUBsRS$cAaoWq, typeTY-PILOTI, axis[10.75,4.75], base ground, top soffit:SL-1. Its type diameter is0.3m, and effective Pset_ColumnCommon LoadBearing is true via the type. Model IDS rule IDS-COLUMN-LOADBEARING requires that property for every IfcColumn. Core helper APIs already supply resolved elements, quantities and building geometry; exchange must use their values.

## Existing Fixture And Asset Evidence

The missing path is model/🔌️exchange/🐍️.py. Core imports step_excerpt, mesh and ids_check at its final export loop; step_first_id is absent. The inspected model schema still requires ["version","languages","revision","schema","building","objects","representations","models","translation","links"] and declares ["version","languages","revision","schema","building","objects","representations","models","translation","links"]. The language-neutral chapter cases retain the old WALL-01 model (6×3×0.3m,18m²,5.4m³), wall thickness event and finish/fire_rating losses. They do not specify B3 STEP numbering/spans, cylinder tessellation or semantic/mesh IDS return shapes. The presentation cases enumerate exchange narrative keys but contain no numeric STEP oracle. No .ifc, .step or .ids source assets were found below the BIM tree in the bounded asset scan. The new Villa Savoye model JSON is the actual value source; old WALL-01 cases are not evidence for a B3 implementation.

Signatures above are inferred directly from actual call sites where no schema contract exists; this report does not invent a default numbering policy or claim complete IFC serialization conformance.
