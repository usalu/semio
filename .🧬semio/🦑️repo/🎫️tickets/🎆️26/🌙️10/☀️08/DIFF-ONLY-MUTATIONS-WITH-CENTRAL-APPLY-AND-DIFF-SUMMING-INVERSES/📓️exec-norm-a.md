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

## Wave 4 — verification round: BLOCKED, nothing compiled yet (17:32)

Requested order: (1) contract `cargo check --target wasm32-wasip2`, (2) en1996/en1995/en1991 `cargo check --target wasm32-wasip2`, (3) `cargo test` mutation
modules, (4) contract `cargo test list_delta` + `cargo test --test config_mutation`. Commands (cwd = the crate's workspace dir:
`📕️norm/📇️registry/🧬️contract`, `📕️norm/🗿️artifacts/{🪨️en1996,🪵️en1995,🏋️en1991}`), each wrapped: `"$T/🚦️gate.sh" norm-a -- cargo check --target wasm32-wasip2 --message-format=short`.

Result: step 1 was queued at 16:38 and never started. At 17:32 the gate shows two holders — `stdio-semio` (pid 75954, cargo 2863,
`check -p semio-s-artifact-stdio-semio --features component-app-assembly`) and `norm-b` (pid 81186, cargo 2879, `check --target wasm32-wasip2`) — both
alive 20+ min at 0% CPU with ZERO rustc processes on the machine: a cargo lock deadlock between the two (the known fine-grain-locking deadlock),
not a compile in progress. Per the rules I did not kill another executor's cargo. My gate process (pid 49663) keeps waiting for a slot.
Needed from the coordinator: free those two cargos (or restart them serially), then my queued contract check starts; contract first, then the three artifacts.
Nothing in Wave 4 is verified; the Wave 1-3 "written but unverified" status stands.

### Wave 4 update (17:35) — gate released, contract check ran, still no norm crate reached
`cd ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract && gate.sh norm-a -- cargo check --target wasm32-wasip2 --message-format=short` →
`could not compile semio-framework-replication (lib) due to 14 previous errors` (log `T/🗑️generated/norm-a/check-contract.txt`). All errors are in
`🧰️framework/🔨️modules/📡️replication/…/🔗️causal/🔀️transition/🔁️fold/🦀️.rs`: `semio_framework_value::retirement::owned_retirement` no longer exists,
`ErasedSnapshotRetirement` grew `next_copy_byte_demand`/`next_capacity_byte_demand`/… and `close_step` changed arity, `RetainedCloneStep` vs
`SnapshotRetirementStep` mismatches — i.e. the `🌱️value` retirement refactor landed without its replication callers. Not in norm scope; 0 errors in
the contract, artifacts or list-delta (they were not reached). Counts: contract 0/0 checked, en1996/en1995/en1991 not run, tests not run.

### Outcome-code audit (17:48)
Contract `🪡️list-delta` raises only `mutation.apply.{missing-target,duplicate-target}` (regex `^mutation\.apply\.[a-z0-9-]+$`, matches the pattern; neither is in the
`accepted` list but the pattern governs). Leaves in en1991/en1995/en1996 use only `mutation.{invariant,duplicate-id,target-missing,no-op}` (all frozen). The shared
app-surface raises Fault codes `norm.*` (not outcome messages). `gate-run-2.log` has 0 lines for en1991/en1995/en1996/contract/results. The non-frozen `diff.order-mismatch`,
`diff.duplicate-id`, `diff.target-missing`, `diff.index-out-of-range`, `diff.index-order` come from norm-b/norm-c artifacts' own diff files (en1994, en1999 …), not from the shared
contract. If they want a shared apply vocabulary they can reuse `list_delta` (it already raises `mutation.apply.*`).

### 18:21 — foundation.status still RED (native=101 wasm=101, last update 18:17); waited 38 min per the build rule, ran no cargo. Resume with: contract check → en1996/en1995/en1991 wasm checks → mutation tests → contract list_delta + config_mutation tests.

## Positional list-delta API (wave 5, AMB-1) — supersedes the anchor (`after`) design above

`📕️norm/📇️registry/🧬️contract/🪡️list-delta` (`semio_s_artifact_norm_contract::list_delta`). A list delta never carries an `order`/`reordered` id list and no
`after` anchors: every ordered change is a positional row, and the indices are COORDINATES, never evolving positions:

| row | wire | index meaning |
|---|---|---|
| remove | `removed: [{id, index}]` | `index` = position in the BASE list (what the inverse reinserts at) |
| insert | `inserted: [{index, row}]` | `index` = position in the AFTER list |
| move | `moved: [{id, from, to}]` | `from` = BASE position, `to` = AFTER position |
| patch | `modified: [{id, patch}]` | keyed by id, position-free (sparse typed `RowPatch`) |

Apply: validate each `id` at its base index; the after list has `|base| − removed + inserted` slots; inserted rows and moved rows go to their after
slots, every other base row (an unmoved survivor) fills the remaining slots in base order; then patches. A key may be removed and inserted (replacement).
`inverse(base)` is row by row, no simulation: removed ← inserted rows `(row.id, their index)`; inserted ← removed rows `(their base index, row read from base)`;
moved ← `(id, to, from)`; modified ← `patch.inverse(base row)`. `absorb` is base-free and coalesces per id by pure index arithmetic
(`rank`/`nth_free` over the two diffs' removed/moved-from and inserted/moved-to coordinate sets): insert∘remove → nothing (a replacement keeps its base removal),
insert∘move → insert at the final slot, move∘move → one move, move∘remove → remove at the base index, remove∘insert (same id) → replacement, patch∘patch → one patch,
insert∘patch → changed insert. `between(a, b)` (sync/import only) keeps a longest common subsequence in place and moves the rest.

Rust surface (macros generate concrete wire types; generic algebra in `Parts<R, Q>`):

```rust
norm_list_delta! { pub Delta { removal: Removal, insertion: Insertion, relocation: Relocation, modification: Modification, row: Row, patch: Patch, key: id } }
Delta::insertion(index, row)            Delta::removal(base: &[Row], index)     Delta::relocation(base: &[Row], from, to)
Delta::modification(id, patch)          Delta::commit_onto(base)                Delta::absorb(later)   Delta::inverse(base)   Delta::between(a, b)   Delta::is_empty()
```
Leaves build rows from payload + base reads only (`removal` reads `base[index].id`). `norm_row_patch!` is unchanged. Randomized sequence law
(`🪡️list-delta/🧪️tests/🔬️unit`): insert/remove/move/patch sequences, absorb ≡ sequential, `inverse` restores, `between` reproduces.
AMB-2: none of the three artifacts derives data in diffs. AMB-3: every `inverse` here reads base values row by row.

## Wave 5 — positional rows, bridge relocation, gate (still no cargo: foundation.status RED)

Implemented the API recorded above ("Positional list-delta API"):
- `🪡️list-delta/🦀️.rs` rewritten (`Parts{removed:(id,base_idx), inserted:(after_idx,row), moved:(id,from,to), modified}`; `commit_onto`, `inverse` row-by-row from base, base-free
  `absorb` via `rank`/`nth_free` coordinate arithmetic, LCS-based `between`); macro `norm_list_delta!` now takes `removal/insertion/relocation/modification` type names
  and generates `{removed,inserted,moved,modified}` wire types (`{id,index}`, `{index,row}`, `{id,from,to}`, `{id,patch}`). No `order`/`after` anywhere.
- `🪡️list-delta/🧪️tests/🔬️unit/🦀️.rs`: 6000-case randomized sequence law over insert/remove/move/patch (nested too) + per-id coalescing + refusal tests.
- en1991/en1995/en1996: diff schemas, 134 list leaf diffs (`D::insertion(index,row)`, `D::removal(&list,index)`), `🔺️diff/🔣️.json` (+`$defs`), TS twins (regenerated), 208 fixture
  diffs, diff unit tests updated. Inverse leaves unchanged (already position-exact: remove ⇒ `Insert{index}`).
- AMB-2: nothing in the three artifacts derives data in diffs. AMB-3: every `inverse` reads base rows (no simulation, no apply on a copy).
- D-01 / gate: the `apply_en19xx_mutation`/`inverse_en19xx_mutation` oracle bridge lived in `🧬️mutations/🦀️.rs` (R9). Moved to `🚪️io/🦀️.rs` `mutation_bridge` (re-exported
  `…::any::io::{apply_en19xx_mutation, inverse_en19xx_mutation}`); users updated: oracle adapters `🧪️tests/{🏋️mutate-en1991-1,🪵️mutate-en1995-1,🪨️mutate-en1996-1}`, fixture/unit/
  middle-row tests. `bun ./📜️script.ts verify mutation-outcome-law` (log `T/🗑️generated/norm-a/gate-run-5.txt`): **0 breaches** in en1991/en1995/en1996, the contract and results config.

Remaining 14 `📕️norm` breaches (norm-b/norm-c): R9 `apply_diff(` in the `🧬️mutations/🦀️.rs` oracle bridge of ⚖️en1990:132, ⚡️din18599:97, 🌍️en1997:97, 🌬️din16798:145,
🏛️en1992:109, 🏭️vdi3805:150, 📇️iso16757:160, 🔩️en1993:182, 🧩️en1994:115, 🧱️din4108:157, 🪶️en1999:88, 🫨️en1998:120 — same fix as mine (move the bridge fns into
`🚪️io/🦀️.rs`, update adapters/tests); R12 ⚖️en1990 `🧬️schema/🔺️diff/🦀️.rs:191` (`let mut … = <ref param>.clone()`).
Unverified until the foundation is GREEN: contract+artifact `cargo check --target wasm32-wasip2`, mutation tests, `list_delta` tests (the new `absorb` arithmetic is exactly what
the randomized test exists to prove — expect to debug it on first run).
