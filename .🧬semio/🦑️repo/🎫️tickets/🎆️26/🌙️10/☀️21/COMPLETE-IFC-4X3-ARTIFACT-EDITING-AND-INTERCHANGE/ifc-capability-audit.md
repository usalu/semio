# IFC Capability Audit

Focus: existing IFC4 source; read-only exploration. No source files changed.

## Source Roots

- Artifact: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc.
- IFC4 subset: 🏅️standards/4️⃣4/🪆️subsets/✳️any.
- IFC2x3: 🏅️standards/🔖️2x3/🪆️subsets/{🧱️base,🏢️cobie,🤝️cv20,🧮️sav}.

## Existing Interfaces

IfcSnapshot (🧬️schema/📸️snapshot/🦀️.rs) owns schema, IfcHeader, entities: Vec<IfcEntity>. IfcEntity contains id, name, args, complex. IfcValue is recursive (unset, derived, integer, real, string, enum, reference, aggregate, typed value). to_part21_document converts the own model to shared syntax only at the boundary.

IfcDiff (🧬️schema/🔺️diff/🦀️.rs) supports id-keyed entity removed/modified/added triples, positional argument triples, sparse header fields, and IfcEntityDiff.complex: Option<Vec<IfcComplexType>>. Existing complex whole-vector diff already applies, inverses, and absorbs; no new algebra is needed for a SetEntityComplex mutation.

IfcMutation (🧬️schema/🧬️mutations/🦀️.rs) has NINE current variants: set-file-description, set-file-name, set-file-schema, insert-entity, remove-entity, set-entity-name, set-entity-arg, insert-entity-arg, remove-entity-arg. Stale prose claims ten. Each leaf supplies handcrafted diff and structured outcome. The mutation edit_rules::EDIT_RULES maps pointer edits to concrete kinds; explicitly excludes complex constituents, entity IDs, document schema. Argument-descendant edits replace an entire argument.

Ifc4AnyEditor implements editing::SnapshotEditingEditor with snapshot_edit_event and snapshot_edit_rules; Editor::handle routes EditSnapshot to snapshot_edit_emit. The details window uses editing::render_snapshot_details and the main window uses MeshWindowKit. A SetEntityComplex mutation plus pointer rule can expose the currently retained but uneditable complex field.

Root declaration() registers artifact schemas, formats, inference descriptors, composers and document_codec_bare<IfcSnapshot,IfcMutation> for standard 4 and IFC2x3 subsets. Interfaces are assembled through semio_framework_plugin and semio_s_artifact_stdio_contract; preserve these boundaries.

## Geometry And Layout Gaps

All ten main surfaces (viewer/editor for IFC4 and four IFC2x3 subsets) render synthetic boxes. Main-window entity_count converts the document to Value, chooses a maximum array length, clamps it to 1..6, and world_instances_json makes index-spaced boxes. Mesh contents do not derive from IFC representation geometry, product placement or real element identity. Empty snapshots still show one box. Selection is always empty and camera fixed.

Exact current rendering interface: main::render(&IfcSnapshot) -> UiAssemblyResult<BuiltNode>, producing MeshView { camera_json, meshes_json, instances_json, selection_json } and calling MeshWindowKit::render(&view). meshes_json currently contains {id:box,data:mesh_from_kind(box)}. Instance shape: {id,meshId,position:[x,y,z],rotation:[x,y,z,w],scale:[x,y,z],label,smoothShading}. Viewer definition uses MeshWindowKit::viewer_window_kind(); editor uses window_kind().

Shared projection opportunity: place an IFC-derived scene model and projection function beside the model/analyzer, independent of viewer/editor. Both surfaces should consume the same projection; neither should import its sibling surface. Keep any tessellator behind a codebase-owned interface. Generate meshes and stable instance IDs from actual IFC products and representation graphs; apply resolved product placements. Real unsupported representation diagnostics should be explicit, rather than fake geometry. Define the scene schema first and test semantic mesh output independently. Expensive projection should use existing job progress/cancellation interfaces.

Existing spatial analyzer (🚪️io/🏛️spatial/🦀️.rs) exposes analyze_spatial(&Part21Document) -> SpatialAnalysis and IfcArtifact::spatial(). SpatialAnalysis has roots:Vec<SpatialNode>, placements:HashMap<u64,Mat4>, property_sets:HashMap<u64,Vec<PropertySet>>, issues:Vec<String>. Mat4 is row-major and transform_point uses M*p. It resolves LocalPlacement -> Axis2Placement3D chains using parent*local composition and detects cycles. Traversal currently handles RelAggregates, RelContainedInSpatialStructure, IFCPROJECT roots, and PropertySingleValue via RelDefinesByProperties. It does not tessellate representations. Geometry projection can reuse matrices and the common Part21 boundary, but should extend placement handling for 2D placements and supported representation families with honest diagnostics.

Editor layout is snapshot_details_split_layout(main::WINDOW_KIND_ID, Main); viewer is a single main pane. No relationship to a separate layout artifact was found inside IFC sources beyond these UI WindowLayout functions.

## Tests And Completion Opportunities

Language-neutral scenarios: 🧪️tests/🏗️mutate-ifc-4/🥒️.feature and 🧪️tests/🔺️differential-ifc-4/🥒️.feature. Ruststep 0.4 supplies independent Part21 reading; Python IfcOpenShell 0.8.4.post1 supplies independent production for schema-expressible operations. Differential Python KINDS actually lists FIVE variants despite stale prose saying six. Existing real Nakagin fixture has approximately 2.5 MB and 24792 instances.

Recommended mutation completion: add SetEntityComplex using existing diff field, update owned schemas and edit rules, add codec/roundtrip/inverse/absorb/edit-pointer fixtures, and validate legal complex records through the existing independent reader. Entity IDs should stay stable identity unless a dedicated graph-wide renumber verb is intentionally designed.

Recommended visual completion: replace synthetic boxes with a shared schema-owned scene projection, test known extrusion/tessellation and mapped placement examples against IfcOpenShell geometry through its test-only boundary, and ensure empty and unsupported documents render honestly. Actual geometry support is substantially larger than the complex mutation gap.
