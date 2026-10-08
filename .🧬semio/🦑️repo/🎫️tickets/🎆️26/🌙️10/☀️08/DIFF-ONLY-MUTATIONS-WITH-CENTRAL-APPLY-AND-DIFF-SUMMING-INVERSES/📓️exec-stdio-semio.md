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
