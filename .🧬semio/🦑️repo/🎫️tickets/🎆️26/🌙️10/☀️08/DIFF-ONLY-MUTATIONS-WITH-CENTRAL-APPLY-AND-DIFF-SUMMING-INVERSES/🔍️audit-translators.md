# 🔍️ Translator Audit: Whole-State Diffing and Whole-Document Kinds

Read-only audit of `✏️s/🔌️plugins/**`, `🌎️hub/**`, `🧰️framework/**` (Rust, TS, TSX). No source edited, no builds, no git writes. This file is the only write.

Rulings applied (from `📋️design.md`): **No whole-document mutations**, **Snapshot-to-kinds translators are deleted**, **Host-scene reconciliation is a translator**, plus **No feature loss** (natural-file import = load/genesis path; details-pane field edit = concrete kind for that field) and **Replace kinds** (entity replace is legit).

## 0. Legend and Path Convention

- **TRANSLATOR**: derives concrete kinds by comparing two whole states (base vs next, before vs after), or builds a whole next state then dispatches the difference.
- **WHOLE-DOC-KIND**: a command row, mutation kind, or effect that carries or replaces a whole document/snapshot (includes name-banned kinds such as `set-snapshot`, `patch-snapshot`, `replace-document`, `setDocument`, `replaceSnapshotSource`).
- **BETWEEN-OUTSIDE-SYNC**: `DiffAlgebra::between` (or a `between`-style whole-state comparison) called outside a diff-type impl and outside sync/load code.
- **LEGIT-LOAD-PATH**: genesis/load/repair-on-load/sync. Allowed by the rulings, flagged where it needs an owner check.
- **FALSE-POSITIVE**: name match only (UI tree reconcile, job reconcile, render effects, concrete per-gesture diff, dead reader, comments).
- Path convention: `⋯` in a path stands for the standard artifact prefix `🗿️artifacts/<artifact>/🏅️standards/<std>/🪆️subsets/✳️any/` (or the subset variant). Line numbers were verified against the working tree at audit time. Resolve any row with `rg -n '<fn>' ✏️s/🔌️plugins/<plugin>`.
- "Callers" counts are production call sites; test call sites are noted separately because they must be rewritten with the translator.

## 1. Summary

- TRANSLATOR: 15 rows (T1, T2, T3, T5, T6, T7, T8, T10, T11, T12, T13, T15, T16, T17, T19). They cover 88 stdio snapshot-edit implementers (50 of them delegate to the shared `snapshot_edit_net(_exact)` helpers), 38 stdio `net_mutations(base, next)` definitions, and the editor/command call sites in raster, wfc, remodel, block (2d/3d/5d), procedural (2d/3d), flow, vcs, gis, playbook. T4 (docx) and T9 (block helper) sit inside translator chains and are counted with them.
- WHOLE-DOC-KIND: 13 rows. W1-W7, W11, W13 (command rows, mutation kinds, source replace), T4 (docx whole XML inside a part diff), T23 (flow VCS `ReplaceDocument`), plus W9: about 25 effect-based whole-document resets (example switches and loads) that need an owner ruling.
- BETWEEN-OUTSIDE-SYNC: 1 production site (`🏙️bim/✏️editor/🔮️inference/🦀️.rs:73`).
- LEGIT-LOAD-PATH: 4 rows that need an owner check (workflow repair, space/collection repair, sourcing `setDocument` load effect, sync `FrontierDelta::between`).
- FALSE-POSITIVE: about 20 name-only matches (listed in section 4).
- Hub (`🌎️hub/**`): no translator, no whole-document kind. Only job-reconcile and admin-reconcile name matches.
- Host/TS messages: no `sceneReport`, `reportScene`, `replaceScene` producers or consumers in scope. The shell's `setDocument` reader (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14373`) is a dead reader of a deleted operation.
- Puzzle 2d/3d/5d: excluded but listed in section 5.

## 2. Per-Plugin Table

| Plugin | TRANSLATOR | WHOLE-DOC-KIND | BETWEEN-OUTSIDE-SYNC | LEGIT-LOAD | FALSE-POSITIVE | Notes |
|---|---|---|---|---|---|---|
| 🗄️stdio (88 editors + contract + 38 schema diffs) | T1, T2, T3 | W13, T4 (docx XML) | - | - | `set_document_id` | Generic snapshot-edit lane = JSON-pointer patch on whole snapshot, then net diff. Largest translator family. |
| 🖨️raster | T5 | - | - | - | - | `set-active-example` diffs whole layer forest. |
| 🀄️wfc | T6 | - | - | - | - | `set-active-example` diff plus reset effect (W9). |
| 📸️remodel | T7 | - | - | - | - | Diff plus per-frame media ops. |
| 🧱️block (2d/3d/5d) | T8, T9 | - | - | - | - | Six copies of `replace_document_operations`: `edit` and `set-active-example` per dimension. |
| 🌀️procedural (2d/3d) | T10, T11, T12, T13 | - | - | - | T14 | Example switch, import, add-widget, host mutate-then-diff. |
| 🌊️flow | T15 | - | - | - | - | `flow_scene_replacement` for example switch. Flow VCS: T23. |
| 🌿️vcs | T16 | W5 | - | - | - | `patch-snapshot` string-keyed field dispatch (name banned). |
| ✒️writer | - | W1 | - | - | - | `set-snapshot`, `set-snapshot-json`, `load-document-json` (+ W9 reset). |
| 🖍️draw | - | W2 | - | - | `reconcile` (geometry) | `set-snapshot` (full block), `load-document-json`. |
| 🗒️note | - | W3 | - | - | - | `load-document-json` and `set-active-example` reset effects. |
| 🔱️trinity | - | W4 | - | - | - | `load-document-json` plus reset effects. |
| 🏛️architect | - | W6 | - | - | `replace` entity kinds (legit) | `ReplaceDocument` program mutation; reset effects in exchange/example. |
| ➗️mathematical | - | W7 | - | - | - | `setDocument` replaces graph and geometry (compare-then-push). |
| 🪵️sourcing | - | W8 | - | (W8 load effect) | - | `setDocument` / `set-artifact-json` is `Effect::LoadDocument`. |
| 🎞️animate | - | W9 (reset) | - | - | `diff_set_presentation` (concrete) | Example reset only. |
| 🌍️gis | T17 | - | - | - | `patch_routes_operations` | `edited_collection_operations` diffs before/after collection. |
| 🏗️fem (2d/3d) | - | W9 (reset) | - | - | `reconcile` (render) | Example reset. |
| 💠️lowpoly | - | W9 (reset) | - | - | `lowpoly_selection_motion_diff` | Reset effects in `document` and `media` commands. |
| 🎥️shooting | - | W9 (reset) | - | - | `replace-*` entity kinds | Reset in `document` command. |
| 📖️playbook | T19 | - | - | - | - | `playbook_edit_blocks_leaves` emits whole block list per step. |
| 🎬️sequence | - | W11 (candidate) | - | - | - | `replace_snapshot(&mut self, SequenceHostSnapshot)`; callers not verified. |
| 🏙️bim | - | - | BT1 | - | - | `ModelDiff::between(previous, snapshot)` for inference cache. |
| 🔋️energy | - | - | - | - | `create-room-air-model` diff | Concrete from payload. |
| 🎪️demonstrator | - | - | - | - | `ChangeSchema` | Concrete kind, not a diff. |
| 📕️norm | - | - | - | - | `replace_document` (cache setter) | Not a mutation. `DiffAlgebra` doc says sync-only. |
| 🕸️dag, 📏️layout, 🏭️process, 🪐️space (plugin), 🔋️energy schema | - | `patch-document` (layout), unverified | - | - | - | Not individually verified (see section 6). |
| 🧩️puzzle | EXCLUDED | EXCLUDED | EXCLUDED | - | - | Section 5. |
| 🧰️framework (os: flow vcs, host, space, store, db sync, shell, ui) | T23 | W10 (= T23) | - | T24, T25, T26 | store root swap, `reconcile_alternative`, ui reconcile, shell dead reader, repo library reconcile | See rows. |
| 🌎️hub | - | - | - | - | job/admin/gis reconcile, space `set-active-example` (Navigate) | No translator. |

## 3. Rows

Each row: class, location, production callers, the user action that should emit concrete kinds instead.

### 3.1 Translators (T)

**T1. Stdio snapshot-edit lane (generic path).** TRANSLATOR.
- Contract: `📇️registry/🧬️contract/✏️editing/🦀️.rs:1354` (`snapshot_edit_net`), `:1364` (`snapshot_edit_net_exact`), trait decl `:810` (`snapshot_edit_mutations`), default `snapshot_edit_emit` (builds whole `expected` via `generic_snapshot_edit_expected` → `apply_snapshot_edit` at `:450`, then calls `snapshot_edit_mutations`, then `validate_snapshot_edit_publication` `:1468`).
- Callers: 88 stdio editor impls (appendix A). 50 files call `snapshot_edit_net(_exact)`; the rest call `apply_snapshot_edit` + `net_mutations` or `edit_mutations` inline. All 88 pass through the whole-document apply-then-diff pattern.
- Action: the details-pane / JSON-pointer edits `setSnapshotValue`, `insertSnapshotValue`, `removeSnapshotValue`, `moveSnapshotValue`, `renameSnapshotKey` must dispatch the concrete kind for the addressed field (ruling: details-pane edit = concrete kind). They must not patch the whole document and diff it back to leaves.

**T2. Stdio `net_mutations(base, next)` family.** TRANSLATOR (38 definitions; representative set):
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:187`
- `…/📕️xlsx/…/🧬️mutations/🦀️.rs:116`, `…/📽️pptx/…/🧬️mutations/🦀️.rs:124`, `…/📐️step/…/🧬️mutations/🦀️.rs:123-161`, `…/🏗️ifc/…/🧬️mutations/🦀️.rs:65,108`, `…/📰️xml/…/🧬️mutations/🦀️.rs:40`, `…/🧾️json/…/🧬️mutations/🦀️.rs:37`, `…/📊️csv/…/🧬️mutations/🦀️.rs:84`, `…/📑️tsv/…/🧬️mutations/🦀️.rs:80`, `…/🎒️zip/…/🧬️mutations/🦀️.rs:72,125`, `…/🎨️svg/…/🧬️mutations/🦀️.rs:43,165,206`, `…/🖋️dxf/…/🧬️mutations/🦀️.rs:226`, `…/🎞️gif/…/🧬️mutations/🦀️.rs:125,238`, `…/🧱️ply/…/🧬️mutations/🦀️.rs:120`, `…/🔺️stl/…/🧬️mutations/🦀️.rs:91`, `…/🎵️mp3/…/🧬️mutations/🦀️.rs:51`, `…/🎥️mp4/…/🧬️mutations/🦀️.rs:86`, `…/📼️avi/…/🧬️mutations/🦀️.rs:82`, `…/🔊️wav/…/🧬️mutations/🦀️.rs:64`, `…/☁️las/…/🧬️mutations/🦀️.rs:119`, `…/🖊️dwg/…/🧬️mutations/🦀️.rs:109`, `…/💬️bcf/…/🧬️mutations/🦀️.rs:192`, `…/🌦️epw/…/🧬️mutations/🦀️.rs:123`, `…/🖼️tiff/…/🧬️mutations/🦀️.rs:43,63`, `…/📦️obj/…/📐️geometry/🧬️schema/🧬️mutations/🦀️.rs:201`, `…/🌐️iso21320/…/🧬️mutations/🦀️.rs:125`.
- Helpers in stdio editor files: `binary_net_mutations` `…/💾️binary/…/✏️editor/🦀️.rs:174`, `txt_net_mutations` `…/🔤️txt/…/✏️editor/🦀️.rs:269`, `md_net_mutations` `…/📝️md/…/✏️editor/🦀️.rs:259`, `html_net_mutations` `…/🌐️html/…/✏️editor/🦀️.rs:253`, `deflate_net_mutations` `…/🗜️deflate/…/✏️editor/🦀️.rs:195`.
- Action: each format editor gesture (cell edit, node edit, text change) must map to a concrete leaf kind (for example `set-cell`, `set-node-attr`, `set-text-run`). It must not rebuild the whole document and derive leaves.

**T3. Stdio binary byte-range replacement.** TRANSLATOR.
- `…/💾️binary/…/✏️editor/🦀️.rs:180` (`binary_net_replacement`), callers `:169` (`Emit::mutations(binary_net_replacement(&snapshot.bytes, &parsed))`) and `:175`.
- Callers: 2 production.
- Action: the byte-range edit is the concrete kind (one `replace-range` with offset and bytes); the common-prefix/suffix search should move out of the editor into the gesture.

**T4. Docx part-level whole XML document inside a mutation.** Classed WHOLE-DOC-KIND inside a translator.
- `…/📜️docx/…/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:574` (`xml_replace_document`), called at `:524` (`document: (!unchanged).then(|| xml_replace_document(document))`). The enclosing part diff compares existing vs. next part.
- Callers: 1 production (inside the docx part diff); 4 `pub fn net_mutations` profiles reach it through `✏️editor` `net_mutations(snapshot, &next)`.
- Action: a node-level edit in a docx part becomes node-addressed kinds (existing `xml-address` family), not `document: Some(whole XML)`.

**T5. 🖨️raster set-active-example.** TRANSLATOR.
- `✏️s/🔌️plugins/🖨️raster/⋯/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs:32` (`replace_document_operations(current, next, example_id)`), called `:122`. Also compares `doc.snapshot.layers == example.layers` (`:~44`) and calls `release_layer_forest`.
- Callers: 1 production; 1 unit test (`🧪️tests/🔬️unit/🦀️.rs:22`).
- Action: example switch (a user action) must emit the concrete layer/asset kinds it needs, or be a genesis/load (see W9 decision). It must not diff the layer forest.

**T6. 🀄️wfc set-active-example.** TRANSLATOR (plus effect, W9).
- `✏️s/🔌️plugins/🀄️wfc/⋯/🖼️bitmap/…/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs:66` (`replace_document_operations(current: &BitmapSnapshot, next)`), call `:56`.
- Also `✏️s/🔌️plugins/🀄️wfc/⋯/✏️editor/🦀️.rs:326-332`: compares `example_snapshot` to the document, then `reset_document_effect`.
- Callers: 1 production (+ 3 unit tests in `🧪️tests/🔬️unit/🦀️.rs:10-36`).
- Action: example switch → concrete tile/rule/slot kinds, or load (W9).

**T7. 📸️remodel set-active-example.** TRANSLATOR.
- `✏️s/🔌️plugins/📸️remodel/⋯/📸️remodeling/…/🎬️set-active-example/🦀️.rs:74` (`replace_document_operations`), call `:38` (`mutations.extend(replace_document_operations(doc.snapshot, &next))`), plus `example_media_operations` (one `create-asset` per committed frame).
- Callers: 1 production (+ 1 unit test `🧪️tests/🔬️unit/🦀️.rs:36,39`).
- Action: example switch → concrete frame/asset/scene kinds.

**T8. 🧱️block set-active-example and edit (2d, 3d, 5d).** TRANSLATOR; six copies of `replace_document_operations`.
- `replace_document_operations` definitions: `…/◻️2d/…/✏️editor/🎮️commands/🎨️edit/🦀️.rs:11`, `…/◻️2d/…/🎬️set-active-example/🦀️.rs:11`, `…/🧊️3d/…/🎨️edit/🦀️.rs:11`, `…/🧊️3d/…/🎬️set-active-example/🦀️.rs:11`, `…/🖐️5d/…/🎨️edit/🦀️.rs:11`, `…/🖐️5d/…/🎬️set-active-example/🦀️.rs:11`.
- Callers (production): `edit` → 2d `:158`, 3d `:212`, 5d `:223` (builds whole next document from text, then `Emit::mutations(replace_document_operations(...))`). `set-active-example` → 2d `:163`, 3d `:217`, 5d `:228`.
- Callers: 6 production, 0 tests in the same files.
- Action: the text `edit` gesture must dispatch the concrete kinds for the edited block/handle/attribute (the parser output is the input, the kinds are the output). Example switch → concrete kinds or load (W9). Retire all six copies together; one copy is a duplicate (AGENTS: repeated code must sit close together, not be duplicated across dimensions).

**T9. 🧱️block shared row diff.** Diff-type helper (FALSE-POSITIVE as a helper, but part of the T8 chain).
- `…/🧱️block/🧬️schema/🧱️shared/🦀️.rs:268` (`block_rows_between`), callers: 2d/3d/5d `🔺️diff/🦀️.rs:113-117` (diff-type impls). Retire with T8.

**T10. 🌀️procedural generation3d `generation3d_document_replacement(before, after)`.** TRANSLATOR.
- Definition: `✏️s/🔌️plugins/🌀️procedural/⋯/🧊️generation3d/…/🧬️schema/🧬️mutations/🦀️.rs:684`.
- Production callers: `🎨️set-active-example` `:59` and `📥️import-document` `:104`. Test callers: `🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:142,154,185-209`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs:81`, `🔬️fold-contract/🦀️.rs:26,33`.
- Action: example switch → concrete widget/graph/camera kinds (or load, W9). Import → genesis/load path (ruling: "natural-file import is the load/genesis path"). The import command currently diffs the existing document against the imported one; it must instead load/genesis. Note the comment at `📥️import-document/🦀️.rs:11-17` says this is deliberate; the ruling supersedes it.

**T11. 🌀️procedural generation3d `config_replacement(base, next)`.** TRANSLATOR (config lane).
- Definition: `…/🧊️generation3d/…/✏️editor/🎚️config/🦀️.rs:182`. Callers: `🎨️set-active-example` `:64`, `📥️import-document` `:107`.
- Action: config fields (camera, LOD, show mode, sun) → concrete config kinds per changed field from the example's declared values.

**T12. 🌀️procedural generation2d `generation2d_host_snapshot_operations(before, after)`.** TRANSLATOR.
- Definition: `…/🌀️generation2d/…/🧬️schema/🧬️mutations/🦀️.rs:179`.
- Production callers: `✏️s/🔌️plugins/🌀️procedural/⋯/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs:35` (example switch), `🎮️commands/🧩️add-widget/🦀️.rs:57` (add widget: runs `host.add_widget(...)`, then diffs `baseline` vs host), `🧬️schema/🦀️.rs:131` (inside `host_operations`, T13).
- Action: add-widget → the concrete `add-widget` kind with the descriptor and position (the host method already knows them). Example switch → concrete widget/synapse/layout kinds.

**T13. 🌀️procedural `host_operations(host_snapshot, mutate: FnOnce(&mut FlowHost))`.** TRANSLATOR (generic mutate-then-diff).
- Definition: `…/🌀️generation2d/…/🧬️schema/🦀️.rs:127`. Production callers: 5 (`rg -n 'host_operations\(' ✏️s/🔌️plugins/🌀️procedural` lists them). Each caller passes a closure that mutates a host and expects the diff.
- Action: every host gesture returns the concrete kinds from the host API (the host method already names the change), not a mutate-then-diff. This retires the generic helper.

**T15. 🌊️flow set-active-example.** TRANSLATOR.
- `✏️s/🔌️plugins/🌊️flow/⋯/✏️editor/🦀️.rs:2591` (`flow_scene_replacement(composed, widgets, synapses, layout)`), caller `🎮️commands/🎨️set-active-example/🦀️.rs:30` (`set_active_example_edit` at `:21`).
- Callers: 1 production.
- Action: example switch → concrete widget, synapse and layout kinds (or load, W9).

**T16. 🌿️vcs demo edit.** TRANSLATOR.
- `✏️s/🔌️plugins/🌿️vcs/⋯/✏️editor/🎮️commands/🩹️edit/🦀️.rs:9` (`vcs_demo_projection_diff_operations(current, next)`), caller `:56` (`Emit::mutations(vcs_demo_projection_diff_operations(current, &next_projection))`).
- Callers: 1 production.
- Action: each edit field (title, counter, status, notes, tags) → its concrete kind from the gesture (rename-vcs, change-counter, change-status, change-notes, add-tag / remove-tag).

**T17. 🌍️gis feature edit.** TRANSLATOR (scoped to one collection).
- `✏️s/🔌️plugins/🌍️gis/⋯/✏️editor/🎮️commands/🗺️features/🦀️.rs:116` (`edited_collection_operations(document, collection, feature_id, edit)`), callers `:176`, `:210`. It builds `after` for the collection and calls `collection_operations(collection, before, &after)` (`:37`, a before/after diff).
- Action: a feature field edit → a concrete `set-feature-field` (or per-field) kind. The closure edit already knows the key and value.

**T19. 📖️playbook block list edit.** TRANSLATOR (emits whole list).
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:275` (`playbook_edit_blocks_leaves(content, step_id, edit)`): clones `step.blocks`, runs the edit closure over the whole list, then emits `playbook_blocks_leaf` (`:244`, a `SetNodeParam` carrying the entire blocks list) if `blocks != step.blocks`.
- Callers: `:284` (add block), `:292` (remove), `:306` (move). 3 production.
- Action: add/remove/move block → concrete insert-block / remove-block / move-block kinds with index (position-exact inverses per ruling). Remove the whole-list `SetNodeParam` write path.

**T23. Framework flow VCS `begin_replace_document` and `flow_vcs_step_document_replacement`.** WHOLE-DOC-KIND (VCS `ReplaceDocument` action).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:654` (public), `:1572` (step machine, phases `ReserveReplacement` → `ReplaceSchema` → camera/widgets/synapses/layout). Writes a version slot.
- Callers: public API 1 (`:654`); test callers `🧪️tests/🔬️flow-vcs/🦀️.rs:622,704,1600,1694`.
- Action: a whole-document import or checkout is a load/genesis path, not a history row. Verify it does not create a history version; otherwise decompose to concrete flow kinds.

**T24. Framework workflow repair `reconcile_workflow_snapshot`.** LEGIT-LOAD-PATH (verify).
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:657` returns `(WorkflowSnapshot, Vec<MutationMessage>)` after orphan-edge drop and other repair rules. Called at `:871` (inside a `#[cfg(test)]`-adjacent replay block; confirm whether production). Action: keep as load/genesis repair only; its mutation messages must never become history rows.

**T25. Space/collection repair on load.** LEGIT-LOAD-PATH (verify).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs:1083` (`reconcile_collection_integrity`), `…/🪐️space/🦀️.rs:1147` (`reconcile_space_atelier_invariant`). Both return `(snapshot, messages)`. Confirm they run only on load.

**T26. Sync frontier delta.** LEGIT (sync code exception).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:190` (`FrontierDelta::between(from, to)`). Permitted: sync code.

### 3.2 Whole-Document Kinds and Effects (W)

**W1. ✒️writer.** WHOLE-DOC-KIND.
- `✏️s/🔌️plugins/✒️writer/⋯/✏️editor/🎮️commands/📸️set-snapshot/🦀️.rs:49` (struct `SetSnapshot`, `#[dsl(keyword = "set-snapshot")]`, field `json: String`), handle `:53` returns `parse_document_json` → `reset_document_effect` (writer editor `:215`).
- `…/🔣️set-snapshot-json/🦀️.rs:44` (`SetSnapshotJson`), `…/📄️load-document-json/🦀️.rs:44` (`LoadDocumentJson`). Same reset path.
- Callers: dev chrome / fixture injection (no production user action). Action: `set-snapshot` is a debugging injector; keep as load (genesis) or delete; do not expose as a history-producing kind.

**W2. 🖍️draw.** WHOLE-DOC-KIND.
- `…/🖍️drawing/…/✏️editor/🎮️commands/📸️set-snapshot/🦀️.rs:12` (`SetSnapshot { #[dsl(block)] snapshot: DrawingSnapshot }`), handle `:17-23` → `drawing_reset_document_effect`.
- `…/📄️load-document-json/🦀️.rs:12` (`LoadDocumentJson`), handle `:18-30`.
- Action: same as W1.

**W3. 🗒️note.** WHOLE-DOC-KIND.
- `…/🗒️note/…/✏️editor/🎮️commands/📄️load-document-json/🦀️.rs:11` (`LoadDocumentJson`), handle `:14-30`, reset at `✏️editor/🦀️.rs:104`.
- `…/🗃️set-active-example/🦀️.rs:19-23` → `reset_document_effect(&next_document)`.
- Action: example switch → load or concrete kinds (W9 decision).

**W4. 🔱️trinity.** WHOLE-DOC-KIND.
- `…/🔱️trinity/…/✏️editor/🎮️commands/📄️load-document-json/🦀️.rs` (`LoadDocumentJson`, reset effect); `…/🎯️set-active-example/🦀️.rs` (reset effect); `…/♻️reset-rule/🦀️.rs` (reset effect). Reset defined `✏️editor/🦀️.rs:122`.
- Action: as W1; `reset-rule` is a rule-level reset, verify its scope.

**W5. 🌿️vcs `patch-snapshot`.** WHOLE-DOC-KIND (name banned; stringly dispatch).
- `…/🌿️vcs/…/✏️editor/🎮️commands/🩺️patch-snapshot/🦀️.rs:35` (`PatchSnapshot { field: String, value: String }`), handle `:40-45`, dispatch `vcs_patch_operation_for_field` (`:11`) → one concrete `VcsDemoMutation`.
- Callers: UI details pane (string field). Action: per-field concrete kind (`rename-vcs`, `change-counter`, `change-status`, `change-notes`). Delete the `patch-snapshot` kind; the details pane dispatches the concrete kind for the field (ruling).

**W6. 🏛️architect `ReplaceDocument`.** WHOLE-DOC-KIND.
- Mutation kind: `✏️s/🔌️plugins/🏛️architect/⋯/🧬️schema/🧬️mutations/📃️document/♻️replace/🦀️.rs:20-34` (`impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceDocument`, `diff` at `:22`). Variant: `…/🧬️mutations/🦀️.rs:141` (`ReplaceDocument(super::replace…)`); proto `🛰️.proto:111,741`; graphql `🔗️.graphql:471`; TS `🟦️.ts:211-212`; oracle `🔮️oracles/🔣️.json:4726`.
- Entity-level `♻️replace` kinds (validation-record, human, requirement, etc.) are replace-entity (legit per ruling). Only `📃️document/♻️replace` is whole-document.
- Reset effect callers: architect `✏️editor/🦀️.rs:163` (`reset_document_effect`), used by `📤️exchange` (2 calls) and `📚️example` (1 call).
- Action: exchange/import → load path; example → load or concrete kinds (W9). Remove the `ReplaceDocument` variant from the mutation union once exchange/example are migrated.

**W7. ➗️mathematical `setDocument` (`set-artifact`).** WHOLE-DOC-KIND.
- `✏️s/🔌️plugins/➗️mathematical/⋯/✏️editor/🎮️commands/🗿️set-artifact/🦀️.rs:20-27` (`handle`: decodes graph and geometry, `if graph != doc.snapshot.graph { operations.push(...) }`, etc.). Registration `✏️editor/🦀️.rs:248`, `:261`, `:1298-1306` (`action_destructive("setDocument")`), describe `:1351` ("Replaces the equation's whole graph and point geometry").
- Example switch `🎬️set-active-example/🦀️.rs:24-30` → `reset_equation_document_effect` (W9).
- Action: graph edit → concrete node/edge kinds; geometry edit → concrete point kinds. Rename the action; a whole-graph replacement is not a gesture.

**W8. 🪵️sourcing `setDocument` (`set-artifact-json`).** LEGIT-LOAD-PATH (name banned).
- `…/🪵️sourcing/…/✏️editor/🎮️commands/🗿️set-artifact-json/🦀️.rs:14` (struct), handle `:19-26`, host-only lane (`✏️editor/🦀️.rs:423`, comment `:418` "`Effect::LoadDocument` rather than a store edit").
- Action: keep as load effect; rename to `load-document-json` for consistency with W1-W4.
- Also `🎬️set-active-example/🦀️.rs:21` → `reset_document_effect(&next)` (W9). Reset def `✏️editor/🦀️.rs:1121`.

**W9. Effect-based whole-document resets (example switches and loads).** WHOLE-DOC-KIND (effect; not a mutation row).
- Reset definitions (`reset_document_effect` or equivalent) and callers:
  - writer `✏️editor/🦀️.rs:215`; callers `🧺️set-active-example/🦀️.rs:36`, `…set-snapshot*`, `…load-document-json`.
  - wfc `✏️editor/🦀️.rs:167` (grid3d), `:456` (grid2d); callers `✏️editor/🦀️.rs:326-332`.
  - lowpoly `✏️editor/🦀️.rs:2027`; callers `🎮️commands/📄️document/🦀️.rs` (1), `📤️media/🦀️.rs` (1), editor (3).
  - fem2d `✏️editor/🦀️.rs:1276`, fem3d `:1291`; callers `📚️set-active-example` (2d and 3d).
  - note `✏️editor/🦀️.rs:104`; callers `🗃️set-active-example`, `📄️load-document-json`.
  - sourcing `✏️editor/🦀️.rs:1121`; callers `🎬️set-active-example/🦀️.rs:21`, `📇️stock-from-catalogue/🦀️.rs`, `🗿️set-artifact-json/🦀️.rs`.
  - shooting `✏️editor/🦀️.rs:936`; callers `🎮️commands/📄️document/🦀️.rs` (3), editor (2).
  - architect `✏️editor/🦀️.rs:163`; callers `📤️exchange/🦀️.rs` (2), `📚️example/🦀️.rs` (1).
  - mathematical: `reset_equation_document_effect` via `🎬️set-active-example/🦀️.rs:30`.
  - animate: `reset_presentation_document_effect` via `🎬️set-active-example/🦀️.rs` (demo only).
  - trinity `✏️editor/🦀️.rs:140`; callers `🎯️set-active-example`, `📄️load-document-json`, `♻️reset-rule`.
  - Action (owner decision): if the example/load replaces the whole document and is not undoable, it is a genesis/load path and stays. If it is undoable (history row), it must emit concrete kinds. Current code emits no history row, so these are LEGIT-LOAD-PATH pending an owner ruling; I classify them WHOLE-DOC-KIND because they bypass the mutation algebra.

**W10.** See T23 (flow VCS `ReplaceDocument`).

**W11. 🎬️sequence `replace_snapshot`.** WHOLE-DOC-KIND (candidate; callers not verified).
- `✏️s/🔌️plugins/🎬️sequence/⋯/✏️editor/🦀️.rs:583` (`pub fn replace_snapshot(&mut self, snapshot: SequenceHostSnapshot)`). Verify callers; if production, action is a host load.

**W13. 🗄️stdio `replaceSnapshotSource` (`ReplaceSource`).** WHOLE-DOC-KIND (whole source text, then T1 net diff).
- `…/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:55` (variant), `:492-504` (`if let ReplaceSource { source } = event` → parse full source), `:736` (action id mapping), `:790` (encode), `:1346` (size guard). Action id `REPLACE_SNAPSHOT_SOURCE_ACTION_ID` (`:40`).
- Action: a source-text editor should dispatch concrete leaves for the changed range, or a load. A whole-source replace inside a history-producing edit violates "No whole-document mutations".

**W14. Shell wgpu `setDocument` dead reader.** FALSE-POSITIVE (dead reader).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14373` checks `operation == "setDocument"` to set `document_changed`. No producer remains (comment at `:14343-14357` says the CRUD `setDocument` write was deleted). Delete the reader with the next shell cleanup.

### 3.3 Between Outside Sync (BT)

**BT1. 🏙️bim inference cache.** BETWEEN-OUTSIDE-SYNC.
- `✏️s/🔌️plugins/🏙️bim/⋯/✏️editor/🔮️inference/🦀️.rs:73`: `<ModelDiff as DiffAlgebra<ModelSnapshot>>::between(previous, snapshot)` to compute `touched` fields, then `recompute(snapshot, touched)`. Callers: 1.
- Action: the touched set comes from the dispatched concrete kinds (the mutation applied). Compare the snapshot pair only in the sync code path, never in the editor.

## 4. False Positives and Legit Non-Translators

- `🌀️procedural` `generation3d_transform_diff(base, targets, kinds, identity, compose)` (`…/🧊️generation3d/…/🧬️schema/🧬️mutations/🦀️.rs:524`; callers: scale, rotate, drag transforms). Per-target concrete kinds from a gesture. LEGIT.
- `🌀️procedural` `patch_leaves` (`🩹️patch-flow-widgets/🦀️.rs:24`), `mesh_operation_rows` (`🥽️edit-mesh-selection/🦀️.rs:86`, `🔪️knife-mesh-selection/🦀️.rs:34`). Concrete rows from payload. LEGIT.
- `💠️lowpoly` `lowpoly_selection_motion_diff` (`🧬️schema/🧬️mutations/🦀️.rs:88`; 3 callers: move/rotate/scale). Concrete per gesture. LEGIT.
- `🌍️gis` `patch_routes_operations` (`🗺️features/🦀️.rs:245`; callers `:304,:322`). Concrete per route field. LEGIT.
- `🎞️animate` `diff_set_presentation` (`🔺️diff/🦀️.rs:392`; tile/source mutations). Concrete per kind. LEGIT.
- `🔋️energy` `create-room-air-model` `diff(payload, base)` (`🔺️diff/🦀️.rs:8`). Concrete from payload. LEGIT.
- `🎪️demonstrator` `set-active-example` → `ChangeSchema` (`🎨️set-active-example/🦀️.rs:24-29`). Concrete kind. LEGIT.
- `🏗️fem` and `🖍️draw` `reconcile(doc)` (`session/🦀️.rs:2382,3618`; `geometry/🦀️.rs:98`). Render-effect reconciliation, not mutation derivation.
- `🗄️stdio` `set_document_id(permanent, changing)` (`✏️editor/🖼️page/🦀️.rs:1092`). Concrete `set-document-id` kind.
- `📕️norm` `replace_document(&mut self, document)` (`⚖️compliance/🦀️.rs:933`). Cache setter, not a mutation.
- `🧰️framework` store `replace_document_roots_retained` (`🏪️store/🦀️.rs:19882`) and `reconcile_alternative` (`🏪️store/🦀️.rs:12826`). VCS/root swap, not diff derivation.
- `🧰️framework` ui `reconcile` (`🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:301`, `🎯️targets/🧊️wgpu/🔀️reconcile`), renderer `reconcile*` TS (`🌳️wgpu-document-reconcile` test). UI tree reconciliation, not host-scene diffing.
- `🧰️framework` `apply_force_graph_layout_to_board_snapshot_json` (infinite board; layout JSON, not a diff). Puzzle 2d uses it; see section 5.
- `🧰️framework` repo library `reconcile*` (`🧹️normalization`, `🔍️discovery`): journal/WAL reconciliation, not mutations.
- `🌎️hub` job/admin reconcile: `reconcile_gis_map_job` (`💡️inference/🏃️runtime/🦀️.rs:3707`), `reconcile_request` (`💡️inference/🪶️sqlite/🦀️.rs:587`), `reconcile_stale_admin_acceptance` (`🏗️bootstrap/🦀️.rs:9874`), `reconcile_shard_owner` (`🛢️db/🌐️cluster/🦀️.rs:468`). Job/row reconciliation.
- `🌎️hub` space `set-active-example` (`🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🎬️set-active-example/🦀️.rs:14-18`): Navigate effect only.
- `🪐️space` `patch_parameter_operation(projection, parameter_id, patch)` (`🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/⚙️engine/🦀️.rs:156`): concrete parameter op.
- `DiffAlgebra` impls in diff-type modules (`🔺️diff/` in 40+ plugins, `os/🎚️config`, `os/🔁️workflow`, `os/🪐️space`, `os/🌊️flow/🌿️vcs`). Excluded by definition. Their `between` helpers are the mechanism behind T1/T8/T9 and are retired with them.
- Test-only: `spr` `protocol-laws` (`📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs:762-763`), `stdio` gltf/diff docs. Keep the law tests; they check the diff type, not a production path.

## 5. Excluded: Puzzle 2d / 3d / 5d (listed for completeness)

- `🧩️puzzle/🧬️schema/🧬️mutations/🦀️.rs:355` `puzzle2d_snapshot_mutations(before, after)`: callers `🎲️apply-board-events/🦀️.rs:175`, editor `🦀️.rs:289`, `:1484`.
- `🧩️puzzle/🧬️schema/🧬️mutations/🦀️.rs:629` `puzzle3d_snapshot_mutations`: callers editor `🦀️.rs:395,456,651`, `puzzle3d_mutations_between` `:389` (used `:484`).
- `🧩️puzzle/🧬️schema/🧬️mutations/🦀️.rs:254` `puzzle5d_snapshot_mutations`: callers `puzzle5d_operations_between_snapshots` `editor/🦀️.rs:552-564`.
- `DiffAlgebra::between` in puzzle `📸️snapshot/🦀️.rs:97-98` (3d, 5d, 2d).
- `Puzzle3dSceneInvalidation::between(synced, scene)` `⏳️precompute/🦀️.rs:742`.
- Host-scene reconciliation (puzzle 3d/5d scene report) and puzzle 2d `board_snapshot` editing (`apply-board-events`): the ruling names these; not audited further here.
- `delete_target_regions_from_snapshot(&mut Value, ids)` `editor/🦀️.rs:1220` (JSON-level snapshot edit).

## 6. Coverage and Limits

- Method: identifier sweeps (replacement, snapshot_mutations, delta_operations, from_snapshot, reconcile, net_mutations, snapshot_edit_*, setDocument/set-snapshot/patch-snapshot/replace-document/replace-snapshot, DiffAlgebra/between); a two-snapshot-argument signature sweep over Rust (about 986 candidate lines, filtered by mutation-like names, reviewed by hand); a kind/command directory sweep; TS/TSX host message sweep (`sceneReport`, `reportScene`, `replaceScene`, `setDocument`); hub sweep.
- Excluded from classification: `**/🧪️tests/**` and `**/tests/**` (tests that call translators are listed where they matter), `pkg/`, `node_modules/`, `🗑️generated/`, `*.json`, `*.md`.
- Not individually verified (open):
  - `🏭️process` (`commands/🗿️artifact`, 7 hits), `📏️layout` `patch-document`, `🕸️dag` `set_snapshot`, `🎬️sequence` `replace_snapshot` callers, `🔱️trinity` `♻️reset-rule` scope, `💠️lowpoly` `mesh_edit` (`🖌️session/🦀️.rs:121`), `host_operations` 5 caller sites (listed by grep, not individually read), `🌍️gis` `collection_operations` body, `🧰️framework` `apply_workflow_operation` (`🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs:1263`).
  - The 38 stdio `net_mutations` bodies were located but not each read; they share the pattern `fn net_mutations(base, next) -> Vec<Mutation>`.
  - W9 counts are from the `reset_document_effect` call sweep (about 25 definitions and call sites), not a per-line read of every command file.
- Recommendation for the next pass: run `rg -n 'fn (replace_document_operations|generation3d_document_replacement|generation2d_host_snapshot_operations|host_operations|flow_scene_replacement|vcs_demo_projection_diff_operations|edited_collection_operations|playbook_edit_blocks_leaves|config_replacement|binary_net_replacement|snapshot_edit_net(_exact)?|net_mutations)\b' ✏️s/🔌️plugins 🧰️framework` as the removal checklist, and `rg -n 'SetSnapshot|PatchSnapshot|ReplaceSnapshot|ReplaceDocument|LoadDocumentJson|SetSnapshotJson|ReplaceSource|SetArtifactJson' ✏️s/🔌️plugins` for the kind checklist.

## 7. Appendix A: Stdio Snapshot-Edit Implementers (88)

Columns: line of `fn snapshot_edit_mutations` in the file; profile directory; count of delegation calls in the file (`snapshot_edit_net(_exact)`, `apply_snapshot_edit`, `net_mutations`, `edit_mutations`); whether the file also handles `ReplaceSource`. Every row is a TRANSLATOR (T1). Rows with `ReplaceSource` also carry W13.

| line | profile (file under `🗄️stdio/`) | delegation calls | handles ReplaceSource |
|---|---|---|---|
| 640 | ✏️editor | 4 | no |
| 404 | ✏️editor | 3 | no |
| 291 | 📐️geometry | 2 | no |
| 555 | ✏️editor | 3 | no |
| 217 | 🧱️baseline | 2 | no |
| 683 | ✏️editor | 2 | no |
| 363 | 🧾️document | 2 | no |
| 723 | ✏️editor | 2 | no |
| 293 | 🔢️value | 2 | no |
| 513 | ✏️editor | 3 | no |
| 302 | 🏛️model | 2 | no |
| 584 | 🧱️base | 2 | no |
| 590 | ✅️valid | 2 | no |
| 293 | 📑️document | 2 | no |
| 182 | 🌉️transitional | 3 | no |
| 182 | 🔒️strict | 3 | no |
| 293 | 🔊️audio | 2 | no |
| 182 | 🧱️base | 3 | no |
| 293 | ✉️base | 2 | no |
| 296 | 2️⃣cc2 | 3 | no |
| 293 | 🖼️image | 2 | no |
| 296 | 6️⃣cc6 | 3 | no |
| 296 | 3️⃣cc3 | 3 | no |
| 296 | 4️⃣cc4 | 3 | no |
| 296 | 5️⃣cc5 | 3 | no |
| 291 | ♾️any | 1 | no |
| 296 | 1️⃣cc1 | 3 | no |
| 284 | 🧊️brep | 2 | no |
| 296 | 🧱️base | 3 | no |
| 293 | 🧰️kit | 2 | no |
| 224 | 🌐️iso21320 | 3 | no |
| 226 | 🧱️base | 3 | no |
| 293 | 🕸️graph | 2 | no |
| 293 | 🎬️video | 2 | no |
| 225 | ✏️editor | 2 | no |
| 225 | 🧱️base | 2 | no |
| 293 | 📽️presentation | 2 | no |
| 293 | 📦️object | 2 | no |
| 291 | 🎩️header | 2 | no |
| 196 | 🗄️a | 2 | no |
| 196 | 🧱️base | 2 | no |
| 326 | 🔺️mesh | 2 | no |
| 196 | 🖨️x | 2 | no |
| 217 | 🗄️a | 2 | no |
| 293 | 📊️table | 2 | no |
| 217 | ♿️ua | 2 | no |
| 217 | 🧾️vt | 2 | no |
| 293 | 📐️cad | 2 | no |
| 233 | 🧱️base | 2 | no |
| 293 | 🔤️text | 2 | no |
| 293 | 🌊️flow | 2 | no |
| 293 | 🎞️animation | 2 | no |
| 293 | 🖊️drawing | 2 | no |
| 363 | 🧱️baseline | 1 | no |
| 217 | 🖨️x | 2 | no |
| 350 | 🧾️document | 1 | no |
| 217 | ⚕️h | 2 | no |
| 217 | 📐️e | 2 | no |
| 253 | 🔬️tiny | 4 | no |
| 253 | 🔰️basic | 4 | no |
| 241 | 🔄️transitional | 3 | no |
| 241 | 🧱️base | 4 | no |
| 445 | 🧱️base | 3 | no |
| 353 | ✏️editor | 2 | no |
| 233 | 📏️strict | 3 | no |
| 424 | ✏️editor | 2 | no |
| 291 | ✏️editor | 2 | no |
| 703 | ✏️editor | 1 | yes |
| 218 | 🌉️transitional | 3 | no |
| 220 | 🔒️strict | 3 | no |
| 242 | ✏️editor | 2 | no |
| 329 | 🧱️base | 3 | no |
| 296 | 🤝️cv20 | 3 | no |
| 296 | 🧱️base | 3 | no |
| 572 | 🛜️i-json | 2 | no |
| 563 | 🧱️base | 2 | no |
| 296 | 🧮️sav | 3 | no |
| 296 | 🏢️cobie | 3 | no |
| 291 | ✏️editor | 2 | no |
| 299 | ✏️editor | 2 | no |
| 291 | ✏️editor | 2 | no |
| 291 | 📰️header | 2 | no |
| 373 | ✏️editor | 2 | yes |
| 348 | 🎛️hdrl | 2 | no |
| 392 | ✏️editor | 3 | no |
| 291 | ✏️editor | 2 | no |
| 425 | ✏️editor | 2 | no |
| 428 | 🖊️markup | 2 | no |
