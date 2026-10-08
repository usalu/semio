# 📓️ exec-norm-a — en1991, en1995, en1996 + norm-wide seams

Status: **WRITTEN BUT UNVERIFIED (no compile, no test run).** Every `cargo check` attempt died before reaching the norm crates:
(1) the shared build-dir lock queue (30-45 min waits, two `Killed: 9`), (2) the repo-wide `✏️s` workspace restructuring (plugins are now
per-artifact workspaces: `cd ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996 && gate cargo check --target wasm32-wasip2`), (3) `semio-framework-replication`
does not compile right now (12 errors, peer mid-change in `🌱️value/🗂️ordered`). Last attempt log: `T/🗑️generated/norm-a/check-1996.txt`.
Policy gate `bun ./📜️script.ts verify mutation-outcome-law` ran (it crashed late in its own bun code) and printed **0** breaches in the three artifacts.

## Shared seams

| seam | before | after |
|---|---|---|
| `commit_value_tree_edit` (+ `handle_set_field/insert_item/remove_item/apply_remedy*`, `dispatch_*`) | clone value tree, mutate, decode, `from_snapshot(base,target)` | DELETED. Handlers take `resolve: FnOnce(&D,&NormEdit)->Result<Vec<M>,String>`; `NormEdit::{SetField,InsertItem,RemoveItem}`; `NormEditRules{set_field,insert_item,remove_item}.resolve(doc,edit)` maps a value-tree path template (`walls[].lengthM`) + resolved selectors to ONE concrete kind via `Mutation::from_payload_value`. `remedy_edit` returns (path,value) without touching a tree; `apply_remedy_edit` kept (remedy-law tests) on top of it. |
| en1998 `🩹apply-remedy` (5th caller) | custom tree edit | uniform `dispatch_apply_remedy(.., \|d,e\| crate::mutations::EDIT_RULES.resolve(d,e))` — **en1998 (norm-b/c) must define `EDIT_RULES`**, as must every family calling `dispatch_*` (replace the `\|base,target\| X::from_snapshot(base,target)` closure; table rows are mechanical: scratch generator `T/🗑️generated/norm-a/gen_leaves.py::edit_rules_source`). |
| `results` `change-selected-check-index` | diff = whole `NormResultsWindowConfig` | `NormResultsWindowConfigDiff{selected_check: Option<SelectedCheck>}` (sparse, `absorb`, `DiffAlgebra`), leaf split into `🔺️diff`/`↩️inverse`/`🧪️tests`, law test, config test uses `apply_diff`. Contract dev-dep `protocol-laws` added. |
| `En*Mutation::from_snapshot` | snapshot differencing | `replacement(base,target)`: per-setting setters + lists cleared and rebuilt (en1991 already was); used by `set-snapshot` and `import_media`. |
| `apply_to_artifact`, `diff_set_snapshot`, `artifact: Option<Box<Artifact>>` diff field | whole-snapshot replacement | deleted |
| `🪡️list-delta` (new, `📇️registry/🧬️contract/🪡️list-delta`) | — | `Keyed`, `RowPatch`, `Parts<R,Q>` (`commit_onto`, `inverse`, `between`, base-free `absorb`), macros `norm_list_delta!` / `norm_row_patch!` (concrete wire types). Keyed delta = `removed` keys, `added{after,row}` (anchor = key of preceding row, tombstone-safe), `modified{key,patch}`; absorb: patch∘patch, create∘modify fold, create∘delete cancels (re-anchoring), delete∘create replaces. Randomized sequence-law test in `🪡️list-delta/🧪️tests/🔬️unit`. |

## Artifacts

| artifact | kinds | before | after |
|---|---|---|---|
| 🪨️en1996 | 54 list kinds (set/insert/remove at wall/opening/load-case/concentrated depth) | V1-GENERIC-DIFF + V3-LEAF-APPLY (whole `walls` clone) | sparse keyed deltas (`En1996WallDelta` → `WallPatch` → nested deltas); inverse leaves unchanged (already concrete absolute setters / index insert-remove) |
| | 4 doc settings | clean | clean |
| 🪵️en1995 | 58 list kinds (member/connection/actions) | same | keyed `Member`/`Connection` deltas with nested `MemberAction`/`ConnectionAction` deltas |
| 🏋️en1991 | 14 list kinds + `change-accidental-assumed-force` | same | `Floor/SelfWeightElement/Roof/WindFace/AccidentalCase` deltas; accidental force patches the row-local `impact` block (no ids inside) |
| | 65 doc settings | clean | clean (diff type now builds snapshot field-wise in `apply`, no clone-and-write) |

All three diff types: `MutationDiff::apply(&self,base,ApplyCapability)`, field-wise `absorb`, concrete `DiffAlgebra::{inverse,between,is_empty}`.
Behaviour changes: inserts now reject a duplicate row id (`mutation.duplicate-id`, fatal) in en1991/en1996/nested en1995 (keyed deltas need unique keys).
Wave-2 ruling: remove inverses already restore at the ORIGINAL index (`InsertX{index}`); new `🧪️tests/🔬️middle-row` per artifact asserts the inverse-sum law for middle-row remove/insert of every list and index restoration.

## Files touched (counts)

- 204 leaves: `🔺️diff/🦀️.rs` regenerated for 134 list kinds; `🧪️tests/✅apply/🦀️.rs` carries `assert_mutation_inverse_sum_law` for all 204 (en1996 rewritten, en1995 patched + `⛔dupe` `apply_diff`, en1991 `const VECTOR` + law test).
- diff schemas: `🧬️schema/🔺️diff/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql}` ×3 (JSON schema hand-structured with `$defs`, TS twins regenerated with a patched scratch copy of `🧪️s2-norm-ts-twins.ts` at `T/🗑️generated/norm-a/twins.ts`; the original generator is stale: reads removed `📝️text` files).
- fixtures: 208 `🧫️fixtures/🧬️mutations/*/*/🔺️diff/🔣️.json` regenerated by `T/🗑️generated/norm-a/gen_fixtures.py` (independent Python emulation of the leaf semantics) — not yet checked by the Rust vector test.
- aggregates: `🧬️mutations/🦀️.rs` ×3 (replacement, `EDIT_RULES` module `🧭️edit-rules`, `apply_diff` bridge), fixture/unit tests ×3, `🚪️io/🦀️.rs` ×3 (`apply_diff`), editor commands ×5 per artifact, window-ownership test en1996.
- contract: `🦀️.rs`, `🖥️app-surface/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`, `🪡️list-delta/**`, results config files.
- Python oracle `semio_norm_vocabulary` does not model diffs — unchanged.

## Open issues / not run

- Nothing compiled: expect first-compile errors (macro-derive interplay: `ArtifactSchema` on non-`Option` delta fields, `DslRecord` on macro-generated structs, `protocol::DiffAlgebra` export).
- Commands to run when the build is healthy (inside each artifact workspace): `cargo check --target wasm32-wasip2`, `cargo test --lib mutations` ; contract: `cargo test list_delta` and `cargo test --test config_mutation`.
- en1991 `middle-row` test assumes the default snapshot holds a row in every list.
- Other families (norm-b/c) must migrate their `dispatch_*` closures and results-window-ownership tests (`.apply(&before)` → `protocol::apply_diff`).

## Wave 3 — no whole-document mutations (still WRITTEN BUT UNVERIFIED: nothing compiled)

There was no `set-snapshot` mutation KIND in en1991/en1995/en1996 (`MutationKind`); the whole-document replacement lived in the
editor command `set-snapshot` (`ReplaceSnapshot`), `set-active-example`, the shared `import_media`, and the `replacement(base, target)`
differ. All of it is deleted:

| change | files |
|---|---|
| `replacement` deleted (+ its tests) | `{en1991,en1995,en1996}/…/🧬️schema/🧬️mutations/🦀️.rs`, `…/🧪️tests/🔬️unit/🦀️.rs` |
| `Artifact::set_snapshot(&mut self, …)` deleted | `{en1991,en1995,en1996}/…/🧬️schema/🦀️.rs` (no callers) |
| editor command `set-snapshot` and `set-active-example` deleted (module dirs, `#[path]` mods in `{artifact}/🦀️.rs`, `app_commands!` rows, `tools:` lists, `ActionDefinition`/destructive/interactive-job/describe registrations) | `{artifact}/🦀️.rs`, `{artifact}/…/✏️editor/🦀️.rs`, `✏️editor/🎮️commands/{📤️set-snapshot,🎨️set-active-example}` removed |
| editor tests that dispatched them | `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` ×3: command-id/keyword tables, host-backed-report test now evaluates, undo/redo test now uses a concrete `SetField` (en1996 `storeys`, en1995 `annex`, en1991 `altitude`); en1995 `set_active_example_loads_every_bundled_example` removed |
| `import_media` | shared `import_media(port, media)` (no `wrap`): `model:in` inert, `artifact:in` refused (whole-document import is the shell's load path); the three editors' `ArtifactEditor::import_media` call it |
| example loading | `setActiveExample` is no longer declared by these editors; the framework catalogue route (`EditorApp::catalogue_example_document` → `Effect::LoadDocument`) loads examples (genesis/load path, not a mutation, no history row) |
| shared macro `norm_command_from_action!` | `setSnapshot` and `setActiveExample` arms removed from both shapes (`$decode` is now unused) |
| shared tool surface | `NORM_RETAINED_TOOL_IDS` and `NORM_PUBLICATION_CONTRACTS` lose both ids (8 → 6 routes); fixture `🧫️fixtures/🧫️retained-command-dispositions/🔣️.json` (routes, publicationContracts, expected counts 6/90/90) and `🔬️retained-disposition-oracle` updated |
| en1995 `change-annex` leaf module | `mutations::set_snapshot` (legacy directory-name alias of `ChangeAnnex`) renamed `mutations::change_annex` — callers in `{en1995}/🦀️.rs`, mutations aggregate, change-annex leaf diff/inverse, io text mutations + 2 io tests, unit/op-round-trip/diff tests; leaf doc comment cleaned |

Callers searched with grep over the three artifacts for `set.?snapshot|SetSnapshot|ReplaceSnapshot|setActiveExample|SetActiveExample|replacement(` : none remain
(codecs, io, MCP-visible tool lists, tests, fixtures, TS/py, features included). Other norm families still reference the removed shared
pieces (command enums with `ReplaceSnapshot`/`SetActiveExample` rows, `import_media(port, media, wrap)`, 8-route `tools:` lists).
