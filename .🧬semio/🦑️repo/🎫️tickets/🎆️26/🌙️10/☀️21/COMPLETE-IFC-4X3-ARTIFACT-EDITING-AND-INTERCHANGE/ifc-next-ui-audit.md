# IFC Next UI Audit

Read-only source audit, 2026-10-21. Existing ticket ownership and root/s/stdio instructions apply. No implementation, builds, AGENTS changes or git commands were performed.

## Recommended Next Slice

Implement IFC4 product selection to a semantic inspector, preserving the existing generic snapshot editor as the complete low-level authoring surface. This directly connects the real representation geometry already delivered to useful authoring. Begin with named root/product fields (GlobalId, Name, Description, ObjectType, Tag), placement references, representation references and forward/back reference navigation. Do not claim full EXPRESS validation or editable mesh vertices.

Use artifact-owned commands and existing retained snapshot edit publication: translate semantic property edits to current SetEntityArg mutations addressed by entity id, and publish through the retained action factory. Selection and expanded inspector state belong to ephemeral interaction state; the document stays the existing event-sourced IFC snapshot. Schema-first semantic property descriptors should define version/type/argument index, localized names, value kind, optionality and supported mutation; implement Rust and TypeScript projections from those descriptors. Validate with language-neutral vectors and a test-only IfcOpenShell reference for property extraction and edited file re-read.

## Concrete IFC4 Findings

Paths below are relative to C:/git/semio/✏️s/🔌️plugins/🗄️stdio. Let A = 🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any.

- A/✏️editor/🦀️.rs: Ifc4AnyEditCommand contains SetActiveExample and EditSnapshot only. handle() routes generic snapshot gestures through SnapshotEditingEditor. render_with_request_context receives InteractionView but ignores it; main rendering receives only snapshot. render() sends the details pane the entire document through editing::render_snapshot_details. There is no selected-product inspector or product-specific edit command.
- A/🚪️io/📐️geometry/🦀️.rs: render_with builds the world scene with world3d_selection_json("rectangle", &[], None). Selection is always empty. It exposes original request cancellation through render_in_context, which should remain intact.
- A/🧬️schema/📐️geometry/🟦️.ts: geometryPayloads gives each real representation a stable instance id ifc-PRODUCT-REPRESENTATION and interactionId ifc-PRODUCT. GeometryMesh exposes product and representation ids. These are sufficient identity anchors; do not use list index or a manufactured instance identity for semantic selection.
- A/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs delegates entirely to artifact geometry; this is a good thin surface boundary.
- The sibling main/🟦️.ts still declares meshId: "box", fixed transforms and Ifc4AnyEditInstance. This typed twin is stale relative to real geometry payloads. The viewer equivalent should be audited and corrected with the same first-party scene contract, avoiding a second scene schema.
- A/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs: EDIT_RULES maps header tuples, entity name, complex constituents, argument edits, and entity/argument insert/remove to concrete mutations. complex_edit handles deep complex constituent gestures. This is working structural editing, not semantic attribute labeling. An argument still appears under args by numeric position, and name is freely editable at the grammar layer.
- A/🧬️schema/📸️snapshot/🦀️.rs deliberately models IfcEntity {id,name,args,complex} and IfcValue grammar variants. IFC EXPRESS entity attributes, allowed enum values, units, cardinality and relation roles are not represented as model-specific descriptors. No claim that a grammar-valid edit is an EXPRESS-valid edit is justified.
- A/✏️editor/🎭️modes/✏️edit/🦀️.rs installs snapshot_details_split_layout. A new inspector should extend this layout using existing framework window facilities, not replace the full structural details pane with a narrow special-case form.

## Existing Reusable Details Infrastructure

📇️registry/🧬️contract/✏️editing/🪟️details/📐️detail-model/🦀️.rs defines SnapshotDetailsProvider. It supports lazy child access, presentation(path, locale), enum_values, creation_template, variants, missing properties and item capabilities. There is no selected-product argument in render_snapshot_details; use an artifact-owned selected provider/root or a separate artifact-specific inspector over this shared provider interface.

📇️registry/🧬️contract/✏️editing/🪟️details/🌲️detail-tree-and-window/🦀️.rs already supplies typed toggles, string enum selectors, number text drafts, nullable creation, collection insertion, variants, missing property controls and explicit Apply/Discard source drafts. Its schema typing is transport/JSON-schema typing. Reuse the controls and retained command flow, adding IFC semantics through first-party descriptors rather than a UI-only type switch.

## IFC2x3: Confirmed Placeholder Geometry

Let B = 🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets. All eight Rust main windows under {🧱️base,🤝️cv20,🏢️cobie,🧮️sav}/{✏️editor/🎭️modes/✏️edit,👁️viewer/🎭️modes/👁️view}/🪟️windows/🪟️main/🦀️.rs were inspected. Every one:

- Uses mesh_from_kind("box").
- Computes entity_count by largest top-level array length clamped to 1..6.
- Manufactures id and label from subset/index, positions boxes at index * 2, and renders empty selection.

This is especially broken for current base Ifc2x3Snapshot: B/🧱️base/🧬️schema/📸️snapshot/🦀️.rs stores {schema, document: Part21Document, edmPreamble}; real instances are nested under document.instances. The top-level count thus misses model entities and still renders at least one box for an empty snapshot.

## Safe Common IFC Projection Opportunity

Share first-party representation traversal math and mesh emission, with version-specific graph readers/descriptors. IFC2x3 has its own snapshot identity and uses Part21Document internally; IFC4 uses IfcValue/IfcEntity. Avoid converting IFC2x3 into an IFC4 snapshot, treating it as IFC4 schema, or importing the editor from a viewer. Define a narrow first-party graph-read interface for instance id, type constituents and typed positional values; implement it at each artifact boundary. Geometry routines can then consume that interface with explicit schema profile capabilities (2x3 has no IFC4 tessellated face-set entities). Preserve original source ids, complex constituents, local placement traversal, mapped instances, units, status/diagnostics, progress and cancellation. Use the existing real IFC4 extrusion/faceted BRep fixture coverage as baseline; add pinned IFC2x3 official fixtures and independent IfcOpenShell expected geometry before replacing all eight windows at once.

Do not widen the immediate selection/inspector slice into this commonization while remote IFC4x3/build work is active. It changes package boundaries and should follow a dedicated graph-interface design and test matrix. The eight placeholder windows are exact replacement targets once that shared projection is ready.

## Verification Limits

This is static source evidence. No UI action, native compilation or end-to-end authoring success was executed. Existing ifc4-progress.md records repository-routed Windows native-cache and unrelated facade dependency blockers. Those are not silently treated as passing tests.
