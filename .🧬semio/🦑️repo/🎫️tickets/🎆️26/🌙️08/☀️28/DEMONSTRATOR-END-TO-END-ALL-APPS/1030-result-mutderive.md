STATUS: PORTED to the peer 🌱️value API, UNVERIFIED (upstream value crates still red: `value-retained-map` `next_return_demand` missing). derive(Mutations) output needed no change (it calls only the leaf port traits; grep for receive_birth_demand/close_step/RetirementStep in the derive: none). Transients green earlier: gen2d, gen3d-view, gen3d, fem2d, kernel. Pending the upstream: wfc2d/3d/bitmap, Home stepper test, epw roster law, hub. Driver ready: `1030-tools/mutderive/finish-all.sh`.

# 1030 mutderive result

## What the derive emitted (root cause)
`derive(Mutations)` (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔁️mutations-expansion/🦀️.rs`) built the whole roster validation inside ONE associated const `DESCRIPTORS`:
- per leaf an inline `const {}` block (own interpreter budget; fine) but ALSO, in the shared body:
- `component_completeness`: for every registered domain operation a `while` over ALL descriptors with `str_eq` on ~300-byte owner paths = O(operations x leaves x path) -> quadratic, the dominant cost (energy: 306 x 306 pairs);
- `validate_mutation_leaf_descriptor_roster_uniqueness`: hashed per field, but hashed the whole ~300-byte owner path of every leaf (about 0.5M terminators) in the same body as the rest.
The step budget (2M terminators) is per const body, so the sum of these overran it; every crate root that carried the (blanket, template-copied) allow was suppressing that.

## Redesign (no limit raise, no allow)
1. Completeness is O(1) per component: each leaf's inline const block now yields the INDEX of the scope that validated it; the body counts per scope and asserts `count == registered operations`. Sound because `validate_leaf` (identity matcher) pins each leaf to exactly one registered (owner, kind) entry, the roster check proves owners unique, so `#leaves(c) == #entries(c)` implies a bijection.
2. Roster uniqueness runs in its own inline `const {}` block (own budget) instead of sharing the body.
3. `leaf-descriptor-validation` hashes at most the last 48 bytes of every key (`IDENTITY_HASH_TAIL_BYTES`): per-leaf cost is bounded independent of path length; equality still compares full strings (`same_identity`).
4. No API change (no re-export chain touched).

## Law tests
- `mutation_descriptor_roster_failures` (new, `os/spr/command`): every `M::DESCRIPTORS[i]` equals field-for-field the 14-field JSON record (`descriptor_filename`) of its owner dir, parsed by the independent pack-json parser. Wired into the generated `semio_payload_law_<aggregate>` test of EVERY non-generic aggregate (so energy, bim, gltf, ... all run it).
- `roster_uniqueness_hashes_a_bounded_tail_yet_compares_full_identities` (`replication/mutation/tests/mutation-leaf-metadata`): 300 owners sharing a 96-byte tail (hash collisions by construction) must pass; an injected duplicate must report exactly (first 7, index 299).

## Removal
`C/1030-tools/mutderive/strip_allow.py [--apply]` (git ls-files incl. untracked, tickets excluded, line-exact `#![allow(long_running_const_eval)]`, collapses the one adjacent blank line, atomic temp+rename, re-reads each file at edit time): 541 `.rs` crate roots + the hub stdio composition TS generator template (`🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts`). Repo-wide `git grep long_running_const_eval` outside the tickets tree: 0 hits.

## Checks
| batch | result |
|---|---|
| energy model mutations (306 leaves) native `--lib` | rc=0 |
| gltf mutations (120 leaves) native `--lib` | rc=0, 0 const-eval diagnostics |
| bim model mutations (216) | BLOCKED, unrelated: `semio-s-artifact-stdio-ifc-2x3-base` Rust crate Source extent (files) refused before compiler |
| deflate + deflate-mutations native `--lib` | rc=0 |
| law test epw (`semio_payload_law`) | rc=1, UNRELATED: epw test target does not compile (EpwMutation/EpwDiff `diff`,`parse_op`,... trait methods missing in test code); 0 diagnostics mention the new helper |
| law tests gltf, energy | queued, chained |
| wasm hub energy+puzzle components | queued, chained |

## Addendum 16:12 — generic retained "set one field" leaf (UNCOMPILED, check queued in check-hub.sh)
Framework (new, `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🎛️set-field/🦀️.rs`, mounted as `retained::set_field`):
- `RetainedSetField<R,P,M>` (leaf names authored value, displaced original value, sparse patch, exact inverse), `RetainedSetFieldLeaf<L,R,P,M>` = generic `RetainedMutationLeafPreparation` (bind payload+original, `RetainedFieldPreparation` copy of the authored value under grants, one-field patch; inverse copies the displaced original value into the single inverse row; full custody/close chain incl. `FactoryVectorRetirement`; no unwrap/expect, invariant failures surface as `Refused(mutation.invariant)`/empty rows).
- `RetainedPatchDiffApply<R,P>` = generic `RetainedDiffApply` over `RetainedRowPatchApply` (lifted from the energy branches hand impl).
- macros (re-exported through replication packages and `os/packages` next to `retained_row_patch`): `retained_patch_diff!(Patch => Row)` and `retained_set_field!(Leaf => Snapshot, Patch, Mutation { payload_field: Value => snapshot_field @ index })`.
Apps migrated: gen2d `Generation2dTransient` and gen3d `Generation3dViewTransient`: patch field is `Option<Option<String>>` (the `...TextChange`/`...EvalChange` wrappers are deleted), patch derives `RetireOwned` + `retained_row_patch!` + `retained_patch_diff!`, aggregate `#[mutations(.., retained)]`, leaf one `retained_set_field!` line. Caveat: nested options collapse in `ToValue` (documented on the patch); transient state is ephemeral local-only so patches never cross a wire.
NOT done / honest gaps: (1) nothing compiled or run yet (`check-hub.sh` now runs gen2d + gen3d transient `--lib` checks, then the hub batch); (2) no runtime law yet that the generic prep drives a full decision/preflight/inverse/commit cycle (the framework harness in `retained/🧪️tests` is typed to its own Projection/Operation; a set-field instance of it is the next test); (3) idml/drawing/layout hand preparations are NOT set-field leaves (drag layers, diagnostics, row patches) so the "generated equals hand" law and their deletion do not apply; the other 11 `bounded_transient_preparation_factory` apps still need their transient aggregates moved to the same three lines once gen2d/gen3d prove green.

## Review item (set-field Drop) — PREPARED, blocked by edit freeze
`1030-tools/mutderive/set-field-drop-safe.py --apply` (run after the freeze lifts): `Drop` and `terminal_is_empty` no longer require `closing` (a leaf that never took custody drops silently); a leaf dropped while still holding custody leaks its `ManuallyDrop` payloads (never double-frees) and trips `debug_assert!` only (no production panic); the close obligation stays enforced through `terminal_is_empty` and the owner's close loop. Abandon-path test to add with it: construct `RetainedSetFieldLeaf` over the gen2d transient types, drop unprepared (must not panic), then drive decision to Ready and close fully (law: `terminal_is_empty` after `close_step` loop).


## Final check table (30 Oct)
| batch | rc |
|---|---|
| energy / gltf / deflate native | 0 / 0 / 0 |
| bim | blocked (ifc extent) |
| law tests epw / gltf / energy | blocked by unrelated stale test code in those crates (not the new helper) |
| gen2d transient, gen3d transient, hub energy+puzzle | refused before compiler (see STATUS) |

## Value crate source-extent refusal (diagnosed, NOT fixed: peer-owned code)
- Not a policy change and not my code: limit is unchanged (`rust-source-boundary` maxSourceBytes 262144 in `value/📦️packages/🦀️rust/Cargo.toml`); "maximumBytes 4078" is the REMAINING budget when the walker reached `📶️sort/🦀️.rs` (5376 B), i.e. closure already at 258066 B.
- Exact numbers (`1030-tools/mutderive/measure-value.ts`, gatecensus `measure`): closure of `semio-framework-value` = 259162 B / 29 files without feature `testkit` (fits, 2982 B spare) but 263442 B / 31 files WITH `testkit` (over by 1298 B). `testkit` is enabled in the real builds because 53 crates list `semio-framework-value ... features=["testkit"]` under normal `[dependencies]` (os kernel, ~45 store crates, dsl, io, semio, ...), already at HEAD.
- What pushed it over (vs HEAD, file by file): new `value/🗂️ordered/🔢️numeric/🧮️scratch/🎟️work/🦀️.rs` +9732 B (`NumericWorkInsertCursor/NumericWorkRemoveCursor`, staged by a peer, only consumer `◻️2d/🔍️trace`), `🧮️scratch/🦀️.rs` +122 B (mount), `value/📦️packages/🦀️rust/🦀️.rs` +49 B (`pub use semio_framework_value_persistent::work`). My set-field leaf lives in `replication-mutation-retained`, outside the value closure: 0 B contribution.
- Fix options (need the owning peer; the work cursors use private `NumericIndex`/`Node` of the scratch module, so a carve needs those exposed): (1) move the work cursors to a new crate `semio-framework-value-numeric-work` next to `value-persistent` (frees 9.9 KB, closure 253.5 KB), or (2) drop `testkit` from the 53 normal-dependency edges to dev-dependencies (frees 4280 B, enough alone). Option 2 is mechanical but touches 53 manifests across frozen-sensitive trees; I did not apply either.
- Consequence: gen3d, gen2d (also lock problem) and the hub batch cannot compile until one of these lands; check-hub.sh is ready to rerun.

## testkit layering fix (coordinator decision: option 2) — applied
- `1030-tools/mutderive/testkit_to_dev.py --apply`: 52 manifests: `semio-framework-value` in `[dependencies]` loses `features=["testkit"]`; the testkit entry now lives in `[dev-dependencies]` (region-safe, atomic). Left: 0 normal-dep testkit edges to value.
- Real layering leak found and fixed: `store/🧰️test-support` (a normal dependency of the os kernel) kept value/testkit alive only for `pub use semio_framework_value::testing::assert_wire_witness`. The 10-line helper now lives in test-support (its owner), value's copy is removed, test-support's value dep has no testkit, and the one direct user (`store/👥️presence/♻️retirement` test) gets it from test-support via a new presence dev-dependency. No non-test code used any value testkit item.
- Not done: the kernel still depends on `os-store-test-support` and re-exports it (`pub use os_store::test_support`) in production; 1179 test usages depend on it. Reported, not changed.
- Re-measure (gatecensus on hub energy+puzzle components, wasm32-wasip2): `semio-framework-value` 259162 B / 29 files (limit 262144/32), 2982 B spare; none of 1079 closures over any limit.
- Relock: manifests changed in 52 crates + presence dev-dep; the repo cargo path refreshes each affected Cargo.lock on the next check (check-hub.sh running: gen2d, gen3d, hub).

## Final (30 Oct night)
| batch | rc |
|---|---|
| gen2d transient `--lib` | 0 |
| gen3d transient `--lib` (after dropping a duplicate RetireOwned derive) | 0 |
| hub energy+puzzle wasm | 1: `semio-framework-2d-booleans` 7 errors (peer) |

## Resume 30 Oct night: remaining transient apps
- Reality check: none of the 11 apps has a hand-written retained transient preparation to delete (all were red for lack of `RetainedMutation`); idml/drawing/layout are not transient set-field leaves and stay.
- Classes: (a) single-field set leaf: gen2d, gen3d-view GREEN; gen3d `Generation3dTransient` migrated (uncompiled; `check-gen3d.sh` hit `CargoLockRefused`: generation3d Cargo.lock cannot resolve `semio-framework-2d-booleans`, coordinator lock needed). (b) one-field whole-root `transient_root!` (fem2d `gestures`): new `retained_root_set_field!` (RetainedRootMutation wraps the same leaf prep; mutation is its own leaf), `sparse_record_diff!` gained `derives: [..]` (opt-in, RetireOwned), state/gesture/entry derive RetainedClone, SnapshotProjection impl, manifest dep added; fem2d check reaches only `semio-framework-2d-booleans` (peer, 3-4 errors: BooleanJob::advance arity / last_turn_work) and stops before the transient crate. (c) multi-field leaf (`SetSolve` sets assignments+contradiction / output pixels+width+height in wfc2d, wfc3d, bitmap) and multi-field `transient_root!` roots (flow, bim, layout, cad, raster, draw, ...): need the N-field generalisation of `RetainedSetFieldLeaf` (one `RetainedFieldPreparation` per field, loop phases, inverse as one row); not started. (d) Home `ApplyDirectoryPage` folds a page into a directory projection: not a field set; needs its own preparation.
- Roster law: energy/gltf/epw test targets fail on unrelated stale test code; the tail-hash test is green. Hub: `semio-framework-2d-booleans` (peer) breaks the wasm batch before our crates.

## Update 31 Oct: N-field generalisation
- `RetainedSetField` now has `Authored`/`Displaced` types. New macros (definition first, then re-export in replication packages + os packages; kernel `--lib` check rc=0): `retained_set_record!` (leaf whose authored record is cloned whole, patch sets every field, inverse built from the cloned whole original snapshot) and `retained_root_record!`; `transient_root!` gained the arm `retained: true, fields: {..}` (adds `RetireOwned` to the sparse diff via the new `derives:` arm of `sparse_record_diff!`, plus RetainedMutation/RetainedDiff/row patch). The single-field `retained_root_set_field!` was removed (superseded). Caveat: the patch sets every listed field even when unchanged (no per-field no-op detection on the retained path; final state and inverse are exact).
- Apps: fem2d `FemGumballTransient` = `transient_root!{ retained: true }` (derives RetireOwned+RetainedClone, SnapshotProjection, dep); wfc2d/wfc3d `SetSolve`, wfc-bitmap `SetSolve` via `retained_set_record!` (bitmap's `BitmapTransientText` wrapper deleted: diff field is `Option<Option<String>>`); script `1030-tools/mutderive/migrate_wfc.py`.
- Verified green: `fem2dtransient`, `gen3dtransient` (both packages), `gen2dtransient`, `gen3d viewer`, kernel. Blocked: wfc* (errors only in `semio-framework-2d-regions` / `flow-drawing` from the ◻️2d boolean migration: `advance_work`, `into_result`, `boolean_paths`), rerun `check-transients.sh` (re-enable wfc3d/bitmap lines) when ◻️2d is quiet.
- Home `ApplyDirectoryPage`: NOT implemented. `HomeTransient` is an `Arc<HomeDirectoryProjection>` with its own byte-grant retirement (`retained_bytes`, `HomeTransientRetirementFactory`), not RetainedClone; a correct preparation needs the projection's incremental fold/admit API quoted under grants (parse page -> admit -> fold) which does not exist yet. Needs a design decision (incremental fold stepper in the directory crate) rather than a blind macro.

## Update 31 Oct: Home fold stepper, roster-law blockers
- `home/transient/🪜️fold-stepper/🦀️.rs`: `HomeDirectoryFold` (new/next_quote/step(budget)/finish): copies the base spaces then users row by row (quote = entry + key bytes), then folds the page event by event (quote = 1024 + estimated bytes of the addressed row, via the new shared `row_bytes`), whole units only, a unit that does not fit alone makes no progress. Origin page = rebuild from nothing. Law test `the_fold_stepper_equals_the_one_shot_fold_in_bounded_turns` (300 spaces, budgets 2 KiB/16 KiB/1 MiB) compares against the one-shot `fold_page` oracle and asserts each turn charged <= budget and small budgets need several turns. UNRUN (compile blocked by `semio-framework-value-retirement` mid-edit: `RetirementStep::Failure` now takes 2 args).
- Still needed for ApplyDirectoryPage (not started): (1) `RetireOwned` for `HomeTransient`/`HomeTransientDiff` built on the existing `HomeTransientRetirementFactory`/page-wise release, `RetainedClone` for `Arc<HomeDirectoryProjection>` (alias) so `retained_row_patch!`/apply can swap the root; (2) the leaf preparation: bind payload+original, parse (<= 64 KiB, one quoted turn), `admit_page` (Held -> empty diff + no-op warning, Err -> error message), then `HomeDirectoryFold::step` per turn, close chain = chunked release of the partial maps; inverse = empty rows (non-invertible by declaration).
- Roster-law blockers: epw tests lacked trait imports (`ColdMutation`, `OpText/OpBinary/DiffText/DiffBinary`): added. Then the test build stops in `semio-framework-value-retirement` (peer, 16 errors), so the law is still unrun. Energy (`tests/optics` uses removed `energy_snapshot_with_state`, `EntityId`, `.kind()`) belongs to the energy lane (AMBER, mid-migration) and gltf-model-snapshot (123 errors) to the stdio lane: not touched.

## Update 31 Oct (port wave)
- Set-field leaf call sites mirror the peer API: `RetainedCloneRef::bind`, `RetainedCloneBinding::close_one`, `RetainedCloneClose::{step_granted,begin_granted}` and `FactoryVectorRetirement::close_step` now return `RetainedWorkOutcome`; I take `.result` (the replication retained traits `decide/inverse/close_step` still return `Result<_,ValueError>` and have no work channel, so `work` is not forwarded; if the peer extends those traits the leaf forwards it in one place per method). `RetainedFieldPreparation` and `RetainedRowPatchApply` (replication-owned) are unchanged and will need their own port by whoever ports replication-retained.
- Check status: `check-retained.sh` (`-p semio-framework-replication-mutation-retained`) stops upstream in `semio-framework-value-retained-map` (`RetainedCloneBorrowAuthorityAdmission::next_return_demand` missing; value lane). Nothing of mine was reached yet.
- Run order when value is green: `finish-all.sh` = retained, transients (wfc2d/3d/bitmap/fem2d), gen3d, home stepper law, epw roster law, hub.
