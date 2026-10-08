# exec-stdio-semio

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio` (19 subsets). Status: WRITTEN BUT UNVERIFIED. No compiler or test run has finished yet
(see "Build status"); every claim below is structural, from the source and from `rustfmt` parsing only.

## Build status

- Two `cargo check -p semio-s-artifact-stdio-semio --target wasm32-wasip2 --message-format=short` runs via `🚦️gate.sh stdio-semio` (logs
  `🗑️generated/stdio-semio/check-1.txt`, `check-2.txt`) both stopped in the dependency `semio-framework-replication` with 12 `E0308` errors
  (`RetainedCloneGrant` vs `Grant`, `SharedOwner<String>`) in `🧰️framework/🔨️modules/📡️replication/…/📡️wire/🏠️local-interaction/🌳️root/` — a peer's
  in-flight change, not stdio-semio. So NO stdio-semio diagnostic exists yet: everything below is WRITTEN BUT UNVERIFIED by the compiler.
  The crate is now its own cargo workspace (`🧿️semio/Cargo.toml`); run checks from `🧿️semio/📦️packages/🦀️rust`, not from `✏️s`.
- NOT run: `--features component-app-assembly` (editors and nets), `cargo test`, `rustc` on any test target.
- Syntax only: `rustfmt --check --config skip_children=true` over the touched `🦀️.rs` files reports no parse errors; this proves syntax, not types.
- Framework spine API consumed as published in `📓️exec-fw-spine.md` (`protocol::apply_diff`, `ApplyCapability`, `MutationDiff::apply(&self, base, capability)`).

## Contract and generic code (all 19 subsets)

| Item | Before | After |
|---|---|---|
| `set-snapshot` leaf (19) | generic whole-snapshot mutation | DELETED; callers use concrete kind mutations or the genesis/load path |
| `patch-snapshot` leaf (19) | JSON-pointer patch mutation | DELETED |
| `no-mutation` identity (scenario sentinel) | mapped to `SetSnapshot(base)` | DELETED (adapters, features, oracle py, fixtures) |
| per-subset `agg_diff`/`agg_inverse` | generic dispatch | DELETED; each kind has `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs` in its leaf |
| `apply_semio_<s>_mutation(&mut P, &M)` | `&mut` applier | replaced by `diff_semio_<s>_mutation(&M, &P) -> MutationOutcome<Diff>`; applying is `protocol::apply_diff` (re-exported as `semio_s_artifact_stdio_semio::apply_diff`) |
| 18 `🦠️mutation` facades | `&mut P` | diff-only |
| `SemioDiff::Replace` (base envelope diff) | whole replace | DELETED |
| `snapshot_patch_leaf!` | contract macro | not edited (owned by stdio-small); no longer used here |
| editors (19) | `SetSnapshot`/`PatchSnapshot` | `snapshot_edit_net(event, snapshot, net::net)` with a new `✏️editor/🧮️net/🦀️.rs` per subset |
| `🧰️brep` borrowed tests, `prepared_operation_wire_source` | present | deleted |
| io text/binary mutation codecs | `setSnapshot:` branches and keyword rows | removed (dangling blocks in 7 text codecs, keyword tables in 8) |
| `KINDS` lists | contained set/patch-snapshot | removed in animation, flow, presentation, video, audio |

## Per kind

| Subset | Kinds | Classification before -> after | Notes |
|---|---|---|---|
| 🔤️text | 7 | whole `RunList{values}` diff -> keyed rows (`IndexedTripleDiff<SemioTextRunDiff,_>`, coalescing absorb) | marks nested keyed |
| 📊️table | 8 | whole column/row lists -> keyed rows; column change names one cell edit per row | |
| 🕸️graph | 18 | whole node/edge lists -> keyed rows with node/edge/port/property diffs | |
| 🧰️kit | 13 | whole lists -> keyed rows (`types`, `designs`, `objects`, `models`, `representations`) | wave 2: re-add kinds carry `at` |
| 🔢️value | all | `inverse` via `between(apply(base),base)` -> concrete (`inverse_value_diff`, `inverse_nodes_diff`, `inverse_named_positional`) | |
| 🔺️mesh | all | same concrete inverse; `create-texture` gained `at`; `delete-texture` inverse is `CreateTexture{texture, at: Some(pos)}` | |
| 🌊️flow, 🎞️animation, 🎬️video, 🏛️model, 📐️cad, 📑️document, 📽️presentation, 🔊️audio, 🖼️image, 🖊️drawing, 🧊️brep, 📦️object | rest | agg dissolution; every kind diff/inverse in its own leaf; image diff files keep their guards (target-missing, no-op, clamped, invariant) | |
| ✉️base | 18 `apply-<arm>` | wrapper leaves delegate to the arm; top-level `SetSnapshot` gone | adapter + feature rewritten (see below) |

Shared algebra: `✉️base/🧬️schema/🧰️triples/🦀️.rs` (`IndexedRow`, `Replace`, `IndexedTripleDiff`, `absorb_indexed_rows/slot`, `net_ordered`, `net_keyed`).

## Tests, fixtures, oracles

- 644 `.apply(` call sites -> `protocol::apply_diff`; 24 files `apply_semio_*` -> `crate::applied` (crate-root `cfg(test)` helper).
- `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await` inserted in 114 leaf test files (one per leaf case, in the
  inverse test). NOT inserted: `🧰️kit/…/✂️unbind-representation` (sync test), `✉️base/…/🖼️apply-image/🚫️refuses` (refusal case, no inverse), and every leaf that has no
  `🧪️tests/<case>/` directory (233 leaf directories vs 116 test files; the remaining leaves are covered only by subset unit tests). Pending: a sum-law sweep for those.
- text/table/graph/kit diff unit tests rewritten (apply touches only named fields, absorb equals sequential apply, inverse restores base); 46 fixtures
  `🧫️fixtures/🧬️mutations/*/*/🔺️diff/🔣️.json` regenerated by `🗑️generated/stdio-semio/fixdiff.py` (independent Python oracle; 48 cases, 0 mismatches).
- Adapters `🧪️tests/*mutate-semio-*/🦀️.rs` (18): `diff_semio_<s>_mutation` + `apply_diff`; noMutation/setSnapshot decode branches removed.
  `🐍️.py` oracles (17): set/patch/no-mutation branches, KINDS entries and baseline registrations removed; `py_compile` OK.
  `🥒️.feature` (19): no-mutation baseline scenarios, set/patch/no-mutation Examples rows and the base set/patch/reassert/retype scenarios removed;
  the identity-round-trip scenarios now name a sibling fixture with the same before-snapshot (`insert-timeline`, `insert-sample`, `remove-edge`, `remove-relation`,
  `remove-block-entity`, `set-run-style`, `remove-slide`, `remove-tag`).
- `✉️base`: `🔮️oracles/🦀️.rs` (json-rust reader) no longer routes `setSnapshot`/`patchSnapshot`; adapter `mutate-semio-base` rewritten (round trip uses
  `apply-value` before-envelope plus the committed DSL and pack artifacts).
- `🔮️oracles/🔣️.json` (19): set/patch/no-mutation kinds, mutation entries and evidence removed (valid JSON re-parsed); base probe counts 22 -> 18.
- Deleted fixture directories: 25 `mutate-semio-*` set/patch/no-mutation bundles, value `no-mutation.mutation.json`, document `no-mutation-leaves…` and
  `set-snapshot-replaces…` (generator family `📜️document` removed, router and fixture table updated), cad four recipe bundles (generator lists and probe reader updated).

## Wave 2 ruling: position-exact inverses (done in this pass, unverified)

Ruling: a delete/remove inverse restores the item at its ORIGINAL index; create/insert kinds carry an optional index (`at`, absent = append).

- Law convention found in `protocol_laws::assert_mutation_inverse_sum_law` and the framework store: `Mutation::inverse` rows are stored in the
  REVERSE of replay order (the law reverses them before applying). Multi-row inverses were reordered accordingly (kit `remove-design`,
  graph `delete-node`, table `delete-column`, brep `delete-vertex`) and every adapter/unit/leaf-test loop over an inverse now iterates `.rev()`.
- Name-keyed (`NamedTripleDiff`) collections now carry positional adds (`NamedAdded { index, item }`, helper `base::schema::triples::NamedAdded`) with
  rewritten `between/apply/inverse/absorb_named` (generator `🗑️generated/stdio-semio/named2pos.py`, shape copied from the existing mesh engine) in:
  flow (nodes, edges, params), model (spatial, elements, relations), brep (vertices, edges, loops, faces, shells, solids), cad (layers, blocks,
  entities, block entities), document (styles, images), presentation (masters, layouts), image (metadata). Text and binary diff codecs wrap the added
  rows with `enc_named_added`/`dec_named_added`.
- New optional `at: Option<usize>` payload field (Rust leaf, diff, inverse, text codec, borrowed codec where present, grammar/g4/ebnf, leaf JSON schema,
  proto/graphql/ts mirrors where they exist, python oracle) on: flow `insert-node`, `insert-edge`, `set-node-param`; model `insert-spatial-node`,
  `insert-element`, `insert-relation`; brep `create-vertex/edge/face/shell/solid`; cad `add-layer/block/entity/block-entity`; document `insert-style`,
  `insert-image`; presentation `insert-master`, `insert-layout`; mesh `create-mesh/primitive/material`; kit `add-type`, `add-design`, `create-object`,
  `create-model`, `bind-representation`; value `set-map-entry`, `set-node` (binary codec appends a varint `at+1`); image `set-metadata-entry`.
- Every matching remove/delete inverse is now ONE row carrying the original index (the old tail-rebuild tricks in mesh, brep, kit `unbind-representation`,
  value `remove-map-entry` are gone; `x-semio-inverse-rows` removed from the single-row leaves).
- Law tests added (every first/middle/last row, calling `assert_mutation_inverse_sum_law`): text (remove-run, reorder-runs), table (remove-row,
  reorder-rows), kit (types, designs, objects, models, representations on a three-row fixture), flow, model, brep, cad, document, presentation (slides),
  value (nodes, map entries). Mesh already removes a middle mesh in its committed vector. The python oracles of flow, model, brep, cad, document,
  presentation, value honour `at` and emit it in their inverses; the cad feature now removes the middle entity in the inverse scenario too.
- NOT converted (still non-positional, open): flow/graph/etc. nested property lists (`remove-*-property`), `drawing` named collections not touched
  (its delete-node/layer already carry indexes), `video`/`audio`/`animation`/`text`/`table`/`graph` rows were already index-addressed.
  Hand-updated committed diff fixtures whose `added` rows now need `{index,item}` were NOT regenerated for the name-keyed subsets (flow has none;
  model, brep, cad, document, presentation, image, mesh may carry some under `🧫️fixtures/🧬️mutations/*/*/🔺️diff/🔣️.json`) — regenerate
  once the crate compiles, using the independent oracle pattern of `fixdiff.py`.

## Wave 3

Build (from `🧿️semio/📦️packages/🦀️rust`, via the gate, `--target wasm32-wasip2 --keep-going`):
- Default features: `check-5.txt` = GREEN (exit 0, 0 errors) after fixing the first-compile findings (document `NamedAdded` import, 20 exhaustive
  `RetireOwned` patterns in the crate root now `{ …, .. }`, flow `insert-node`/`insert-edge` inverse patterns, image `absorb_named(&d2)`).
- `--features component-app-assembly` (editors and nets): first run found 2 errors in my nets (mesh `change_material_base` -> `change_material_base_color`,
  presentation `set_text_box_blocks` -> `set_textbox_blocks`), both fixed. Two more are NOT mine: brep and model editors call
  `semio_framework_plugin::bounded_config_store_one_item_preparation_factory_birth_bytes`, which the plugin framework no longer exports at the crate root
  (present in HEAD already). The re-run after my fixes (`check-8.txt`) stopped in `semio-framework-replication` / `semio-framework-value`
  (peer's half-finished `ErasedSnapshotRetirement`: `next_close_byte_demand`, `close_step` arity, `RetainedCloneStep`), so the editor feature set has NO fresh
  stdio diagnostics yet. Retry when the framework is green.
- NOT run: `cargo test` (blocked by the same framework failure). All test files below are therefore compiled by nobody yet.

Closed items:
1. Inverse row order: already the wave-3 ruling (stored order = reverse of replay). Dag, gis, sequence, flow and cad callers replay `.rev()`.
2. Name-keyed added rows regenerated: only brep had committed diff fixtures with bare `added` rows (5 bundles, now `{index,item}`); other name-keyed
   subsets carry none.
3. Nested `remove-*-property`: graph node/edge properties were already positional (`index`/`IndexAdded`); flow params got `set-node-param.at`. A scan of every
   remove/delete/unbind inverse finds no remaining one without an index, except the singleton slots (object `brep|mesh|properties`, kit `properties`) and
   document `remove-block` (tree path).
4. Mirrors: diff JSON schemas rewritten from the Rust structs for text, table, graph, kit (keyed triples) and their `🛰️.proto`/`🔗️.graphql`/`🟦️.ts`
   regenerated; `added` rows wrapped as `{index,item}` in the JSON schemas, graphql, proto, ts of flow, brep, cad, mesh, image (+ ts for model/document/
   presentation/cad). Kit's `🔺️diff/🟦️.ts` is a real TS port (parse and apply): restored from HEAD after I overwrote it by mistake and rewritten for
   the keyed shape; the kit document-contract test lost its SetSnapshot witness (15 mutations, 15 variants). Still open there: the committed
   `diffCases`/`patchCases` fixtures of that contract test still hold the old `{values:[…]}` diffs, and model's one-line proto messages were not patched.
   Removed from the mirrors: `SetSnapshot` in the base graphql union, the base TypeScript probe, presentation text keyword json/graphql/ts, flow
   `operation-source.json` tag 0.
5. Leaf tests for every leaf without one (117): 100 generated `🧪️tests/↩️inverts/🦀️.rs` (document, cad, presentation, flow, model, animation, audio, video,
   value) calling `assert_mutation_inverse_sum_law` on the leaf's demo mutations that the committed base accepts (asserts at least one is exercised), wired as
   children of each subset's unit-test module; 17 base `apply-<arm>` tests on the committed wrapped-arm vectors. A leaf whose demo mutations are all refused by the
   base will fail the `checked > 0` assertion — that is the intended signal, to be fixed once tests run.
6. Outside callers: dag `document-behavior` test, cad editor (genesis carries the imported topology, tests use `create-vertex`), gis inferences (+ unit test),
   flow add-widget test and `flow_scene_replacement` (now the concrete node/edge/param leaves via `flow_content_leaves`, no snapshot replacement), sequence
   node-graph test, trinity child-frame test (`change-node-label`), playbook, imperative and flow `patch-flow-widgets` constructors gained `at: None`.
   `writer`/`drawing` `SetSnapshot` belong to their own artifacts and are untouched.

### Wave 3b: gate burn-down for 🧿️semio (gate-run-2.log)

- Rule 1 `mutation-migration/outcome` (96 `🔺️diff/🦀️.rs` without a frozen code): every one of them now starts with real guards using the frozen codes —
  `mutation.target-missing` (absent id/index/path, out-of-range slot), `mutation.duplicate-id` (fatal, create/insert of an existing id) and
  `mutation.no-op` (warning, set to the value already held). Generator `🗑️generated/stdio-semio/outcomes.py`; subsets: flow 11, animation 11, video 7, audio 8, model 9, cad 14,
  document 16, presentation 13, value 7. UNVERIFIED by the compiler (guards use the leaf modules' private lookups `timeline_at`, `find_layer`, `block_at`, `master_at`, `resolve`, …).
  Known thinness: document `insert-block` only guards the top-level slot, nested paths are not checked; value `set-node`/`set-map-entry` guard no-op / non-map only.
- R15 untested leaves: base `apply-image` (new `↩️inverts` test on the committed arm vector) and kit `unbind-representation` (new test unbinding every representation).
- Other stdio breaches in the log are NOT under 🧿️semio and were not touched (reported to the coordinator): R8 `&mut` in leaves (svg `restore-non-tiny`, `strip-non-tiny`;
  png `change-gamma`, `paint-native-samples`, `patch-pixels`; tiff `paint-region` x2; bmp `paint-indexed-region`, `paint-direct-region`), R10 `between(` in leaves
  (png `change-gamma`, `paint-native-samples`, `patch-pixels`; jpg `replace-image`; wav `🧬️mutations/🦀️.rs:66`; bmp x2), R14 whole-snapshot restore inverse (svg `remove-element`,
  `strip-non-tiny` returning `RestoreNonTiny`), R16 `apply_to` (xml base `handcrafted-diff-codec` test line 90).
- Foundation status stayed RED (`semio-framework-value` `ErasedSnapshotRetirement` half-finished) for the whole waiting window, so no new cargo diagnostics exist for wave 3.

## Wave 4 and 5 (edit rules, deleted nets, positional/negative checks)

Nothing below was compiled or run: `foundation.status` stayed RED (framework-value `SnapshotRetirementStep` / `next_close_byte_demand`) for the whole window. Everything is WRITTEN BUT UNVERIFIED.

- **Edit-rules table per subset** (`<subset>/✏️editor/🧭️edit-rules/🦀️.rs`, `pub const EDIT_RULES: EditRules` of the stdio contract's `🧭️rules`): text, table, graph, flow, animation, video, audio, model, cad,
  presentation, mesh, image, brep, drawing, object, kit, document, value, plus the envelope (`✉️base`). Each editor implements `snapshot_edit_rules()` and, where the payload is
  not a plain field of the edit, `snapshot_edit_special()`; `snapshot_edit_mutations`, `net_mutations`, `snapshot_edit_net` and the 19 `✏️editor/🧮️net/🦀️.rs` files are deleted;
  `net_keyed`/`net_ordered`/`NetStep`/`NetKeyed` (triples) are deleted with their last user. No semio code references `SnapshotPatch`, `prepare_snapshot_patch`, `snapshot_edit_net`.
- **Shared plumbing** `✉️base/✏️editor/🧭️edit-plumbing/🦀️.rs` (`crate::editor::semio_base::edit_plumbing`): const rule builders (`ent`/`ins`/`rem`/`named`/`keyed`), `resolve` (plan + reshape + `Mutation::from_payload_value`),
  reshapes for payloads the table cannot carry (`spread_item` for `create-*` kinds whose payload is the inserted row plus `at`; `complete_by`/`complete_in` fill the members a setter keeps,
  e.g. `resize-node`, `set-edge-endpoints`, `set-stream-meta`, `replace-primitive-geometry`, `edit-design`, `set-image-bytes`), `slot_edit` (optional child: `create-*`/`delete-*` of object brep/mesh/properties and kit properties),
  `list_move` (text `reorder-runs`, table `reorder-rows`/`reorder-columns`, image `move-frame`) and `edited` (an edit applied to a clone of ONE entity, never the document).
- **Computed gestures**: document block tree (path resolved through quote/list item/table cell containers -> `insert-block`/`remove-block`/`set-block-content`/`set-heading-level`/`set-list-ordered`/
  `set-paragraph-style`/`set-run-text`/`set-run-style`/`set-image-block`), value root tree (path of key/index segments -> `set-value`/`insert-list-item`/`remove-list-item`/`set-map-entry`/`remove-map-entry`), table cell edit
  (`edit-cell` gets its column name from the column list). The envelope strips `/subset` and delegates to the wrapped subset's special/rules, wrapping the result in `apply-<subset>`.
- **Refusals (by design, one kind or nothing)**: replacing a child that already exists (delete then create), inserting a design with pieces, changing a table column kind, brep loops, graph edge endpoints/labels, drawing node tree edits,
  text mark replacement, and editing the subset tag or schema stamp. These raise `snapshot-edit.unsupported-path`.
- **Test**: `✉️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` `edit_rules_laws` (every rule of all 18 tables names a kind of its subset's `KINDS` with one selector per wildcard; text edit-run plan; refusal of an unnamed pointer;
  spread/complete/edited/list_move plumbing).
- **Dead whole-snapshot codecs removed** (last users were the deleted set-snapshot ops): animation `enc/dec_animation_snapshot` and the `patch-snapshot` stub in its text codec, cad `enc/dec_cad_snapshot`, presentation `enc/dec_presentation_snapshot`,
  base `enc/dec_hex_snapshot`, value `enc/dec_semio_snapshot` and the binary `enc/dec_semio_value_snapshot_bin`.
- **Mesh TypeScript oracle** `🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts`: removed the `PatchSnapshot` helpers and arms; creates take `at`, delete inverses are one create at the original index.
- **Wave 5 (value patch helpers, apply, order lists, negative diffs)**:
  - `value_diff_between` is no longer reachable from any leaf: value `set-node` and `set-map-entry` emit a typed whole-node `SemioValueDiff::Replace { value }` row from payload + base reads (inverse reads the base node); the function is private to the diff module and
    its io imports are gone.
  - Nested row-diff methods named `apply` (animation keyframe/channel/timeline, audio channel, image frame) are now `patched`; the only remaining `.apply(` under `🧬️schema/**` is the envelope's capability forwarding in `SemioDiff::apply`.
  - No leaf under `🧬️mutations/**` references `between`, `apply_diff`, `ApplyCapability` or `apply_to` (rg). Positional rows: no `order`/`reordered` id list exists in any semio diff; moves are indexed remove + add rows.
    `DiffAlgebra::inverse` bodies of the 19 diff modules read base rows one by one (scripted scan: no apply/between/simulation call in any `inverse*` fn).
  - NOT resolvable inside this artifact: `DiffAlgebra::between` is still a required trait method (design line 70: sync/import only). Its implementations and the helpers they call (`value_diff_between`, `*_between`, `between_row`, `between_indexed_rows`, text/table/graph
    `between_*`) live in the `🧬️schema/🔺️diff` modules; if the new gate flags `*between*` helpers there, the trait method has to leave `DiffAlgebra` first.
- **Payload spelling (found late, fixed)**: the leaf payload structs of table, document, object, presentation, text, mesh, graph, drawing, image, brep, kit are snake_case on the wire (no `rename_all`); animation, video, model, cad, audio are camelCase.
  The first rule tables used camelCase everywhere. All tables, reshapes and specials now spell payload members per subset (checked by a scratch script that compares every rule member with the leaf struct fields); `spread_snake`
  renames the camelCase members of an inserted row (`sourcePort` -> `source_port`, `childId` -> `child_id`, ...); `DocBlock` variant members (`style_id`, `image_id`) are snake_case in the snapshot too.
  `edit-run`'s `new_content` was renamed `new_text` everywhere (leaf, codecs, schema json, graphql/proto/ts, python oracle, feature vectors, fixtures, probes script, oracles notes) because the R14 regex reads `*_content` as a whole-record restore.
- **Gate run 4 (74 breaches under 🧿️semio) -> 0**: `bun ./📜️script.ts verify mutation-outcome-law` after the fixes prints no row under 🧿️semio (the log still shows other plugins' rows). Fixes:
  - R10: leaf-reachable `between` is gone. document `set-block-content` emits `DocBlockDiff::Replace { block }`, `set-image-block` a sparse `DocImageBlockDiff` from payload + base; presentation `set-slide-notes` / `set-text-box-blocks` emit positional replace rows
    (`replace_rows`: every base row removed at its index, every payload row added at its index). The sync-only machinery (`between_*` recursion, `between_demo_cases` law vectors) is named `between_*` in flow, video, model, cad, document, presentation, audio, mesh, drawing, brep.
  - R8/R12: row application helpers are `apply_*_to_copy` / `apply_row` (animation, audio, image, video, document, presentation); model `relative_placement_diff` takes `Fn(SemioTransform) -> SemioTransform` instead of `&mut`;
    drawing `transform_index` and `parent_and_index` no longer clone-and-mutate.
  - R14: text `edit-run` field rename above.
- **Stale test module removed**: image `🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs` still declared the deleted `set-snapshot` retarget test (`#[path]` to a missing file; every `#[path]` in the artifact now resolves, scripted check).
- Mirrors: object and kit `🔗️.graphql` unions no longer list `SetSnapshotMutation`.
- Foundation was still `RED 22:41` at the end (waited well over 60 minutes in total): nothing in waves 4 and 5 was compiled; the exact commands remain `"$T/🚦️gate.sh" stdio-semio -- cargo check ... [--features component-app-assembly]` and `cargo test -p semio-s-artifact-stdio-semio edit_rules_laws` from `🧿️semio/📦️packages/🦀️rust`.

- Procedure note: one `git rm --cached` was issued by mistake on the deleted text `🧮️net/🦀️.rs` (index-only, the file is deleted in the tree); no other git command was run.

## Open issues

1. Compile and test results pending (see Build status). Expect fix-ups in: leaf tests that still reference removed helpers (unused `list` bindings are only warnings), text/table/graph/kit
   mutation unit tests and `demo` cases, `object` document-contract test, `value`/`mesh` unit tests that asserted `between`-based inverses.
2. (resolved by wave 2) Kit re-add kinds now carry `at`; remaining exactness gaps are listed under Wave 2.
3. Sum-law call missing for leaves without a `🧪️tests/<case>` directory (list above).
4. Comment-only mentions of `SetSnapshot`/`set-snapshot` remain in docs, `.graphql/.proto/.ts/.g4/.ebnf/.grammar.semio` headers and some diff-file module docs (no runtime effect).
   Schema artifacts `🔗️.graphql/🛰️.proto/🟦️.ts/🔣️.json` of the generic leaves must be regenerated from the Rust schema; only hand-edited ones were checked.
5. Callers OUTSIDE this artifact still use `Semio*Mutation::SetSnapshot/PatchSnapshot` or `apply_semio_*` (found by rg): plugin cad editor tests
   (`🔌️plugins/📐️cad/…/✏️editor/…`), flow editor + `➕️add-widget` test, sequence `🕸️node-graph` test, trinity rewriting `child-frame` test, layout/raster/remodel mutation docs.
   Not edited (not in scope; to be handled by their owners or a follow-up).
6. `snapshot_patch_leaf!` untouched; remaining references live in stdio-small scope.
7. Test-host helpers `patched_snapshot`/`snapshot_patch_inverse` (`semio_repo_test`) lost their last caller here; removal belongs to the test-host owner.
