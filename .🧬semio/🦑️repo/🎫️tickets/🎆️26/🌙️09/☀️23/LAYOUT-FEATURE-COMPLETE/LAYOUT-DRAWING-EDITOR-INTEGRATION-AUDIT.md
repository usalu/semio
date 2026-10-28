# Embedded Drawing Editing Integration Audit

Read-only source audit; native build unavailable. No production changes.

## Confirmed Gap

Layout inspection: C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs. Lines 162–165 expose Drawing text fields; 494–506 expose copy/clear and page canvas width/height. Existing UI law: 🔍️inspection/🧪️tests/📄️drawing/🦀️.rs:4 asserts those four controls in both locales. Geometry/path/style edits are absent.

## Existing Drawing-Owned APIs

Drawing base: C:/git/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor.

- 📌️panels/🔍️properties/🦀️.rs:302 render(&DrawingSnapshot, ids, &DrawingPlayLabels, &TreeWindows): borrowed child property/arrangement/fill/polygon/path UI. :154 node_row edits anchor/control coordinates, node mode, contour opening/closing/joining, split/delete/reverse/conversion. :245 fill_controls and :280 stop_row edit gradient coordinates, stops/color/alpha/offset.
- 🎮️commands/✏️edit-path/🦀️.rs: EditPath { layer_id, edit: Box<PathEdit> }; handle borrows ArtifactView<DrawingSnapshot>, validates lock/target, calls Drawing geometry edit_path and emits update_path_geometry plus selection effects.
- 🎮️commands/🎨️edit-fill/🦀️.rs: EditFill { layer_id, edit: Box<FillEdit> }; borrowed handler uses Drawing edit_fill and emits replace_layer_fill.
- 🎮️commands/🩹️patch-layer/🦀️.rs: PatchLayer uses Drawing-owned parse_layer_field_input, drawing_op_for_layer_field, lock checks and semantic-inverse no-op detection.
- 🎮️commands/🧵️patch-layers/🦀️.rs:19 handle; :45 selected_targets. Polygon UI uses patchLayers.
- Existing fixtures: patch-layer and edit-selection each have 🧪️tests/🔬️unit/🦀️.rs; edit-path simplification has ✏️edit-path/🧵️simplify/🧪️tests/🔬️unit/🦀️.rs.

## Framework Composition Seams

Framework base: C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin.

- 📜️declaration/🪟️surface/✏️artifact-editor-trait/🦀️.rs:23 ArtifactEditor::Members is the closed typed member fleet (default NoMembers); :26 child_restore_projection projects actual loaded-parent child fields; :30 genesis_child_pack provides derivable child genesis. 🧵️instance/🛬️lifted/🌳️tree/🦀️.rs:31 editor_surface<E> retains E::Members in VcsArtifactApp<EditorApp<E>, E::Members>.
- Layout ✏️editor/🕹️layout-play-app/🦀️.rs:56 implements ArtifactEditor without overriding Members, child_restore_projection or genesis_child_pack. Its current inline background payload cannot simply receive a ChildEmit: no declared Drawing member fleet is mounted.
- 🧵️instance/📥️staging/🧬️composer-api/🦀️.rs:238 open_child; :244 child_store; :259 register_child are real member store registration/lookup seams.
- 🪆️child/🧭️member-keys-and-paths/🦀️.rs:20 MemberPath models slot/child-id owner paths with root/join/under.
- 📜️declaration/📨️emission/🛬️lifted/📦️preparation/🦀️.rs:117 ChildEmitPreparation::of_owned<S,M>(slot, child_id, operations) retains original typed mutations for MemberStoreOwnedBatch admission. :37 OwnedChildEmit holds metadata plus actual typed batch. 📜️declaration/📨️emission/🦀️.rs:495 Emit::prepare_child_one incrementally prepares/transfers it using caller grants.
- 🧵️instance/📨️dispatch/📥️staging/📤️emit/🦀️.rs:469 explicitly refuses ChildEmit without a live child store, directing callers to open_child/register_child.
- 🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs dispatch_command validates app/mode ownership, stages original arguments, builds owning app command, admits retained wire and starts typed dispatch. Its owner match rejects non-app/non-mode addresses. It is not a verified generic embedded child command dispatcher.

## Clean Integration Recommendation

Declare Drawing as a framework child with exact Drawing dialect/member roster; reuse Drawing properties and interaction controls over the borrowed member snapshot. Keep selected child address and Drawing selection/session ephemeral. Route the actual Drawing command/payload to its owning handler and commit its original DrawingMutation batch through funded ChildEmitPreparation/publication. Extend a framework-owned child command route where necessary: the inspected dispatcher only supports app/mode ownership. Such a route must verify MemberPath/dialect, retain exact child revision/source and forward original caller progress/cancellation/close. This is an identified missing seam, not a verified existing embedded-editor API.

Do not feed ChildEmitPreparation the current inline background without registering a real member store. Do not duplicate Drawing fields/validators in Layout, reconstruct Drawing from field strings, replace the whole child for scalar edits, or cold-clone before each command. Keeping inline backgrounds would require a separate explicit framework typed subartifact projection/mutation boundary; the inspected ChildEmit applies to stores rather than arbitrary inline fields.

## End-To-End Tests

Extend the current Layout Drawing UI corpus to document/page target selection and reused Drawing controls in English/German. Drive geometry, path-coordinate, gradient and opacity through real routing, then assert exact child mutations, unchanged sibling, undo/redo, lock/invalid-value refusal, original source/revision identity, cancellation and undergrant behavior. Reuse Drawing command fixtures for semantic expectations. Framework laws: 🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/⚖️laws/🦀️.rs (child-group wire and semantic-text-wire corpora), and 🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/👶️child-emit-prepublication-and-ephemeral-lanes/🦀️.rs.
