# RESULT: BASELINE-DEFECTS-12 (`bd12`)

Executor `bd12`, 2026-10-18 (session 1 00:00 to 00:20 staged the `poll_kernel_turn` decomposition and items 1 and 2; session 2 02:10 to 03:10 verified, finished and closed). Foreground, no sub-agent, no modifying git command. Tools: `bd12-tools/` (pre-edit copies in `bd12-tools/orig/`), lock tools in `bd12-tools/locks/`. Nothing was compiled: `COMPILE ADMITTED` is absent from `📓️FLEET.md`; every Rust claim is static.

## 0. Verdict

DONE for items 1, 2, 3, 6 and the facade alias; the `time-travel-turns` E0616 does not reproduce (section 8). Items 4 and 5 are diagnosed; both need a compile/wasm window and are listed in section 7.

| # | Item | Status |
| --- | --- | --- |
| 1 | `runtime_lifecycle_grant()` in `🚪️opening` | DONE (grant taken from the owner) |
| 2 | dead `pump_runtime_live_cooperative_turn` | DONE (helper, test, mount, fixture dir, probe counter, fault variant) |
| 3 | `poll_kernel_turn` 962 lines | DONE (about 45-line phase driver + 14 named phase functions in 6 children); differential oracle clean |
| 4 | empty `plugins.json` | DIAGNOSED: not a generator defect; law moved to the application catalog (section 4) |
| 5 | 12 stale descriptor packs | DIAGNOSED: only the describe step (wasm) re-emits them (section 5) |
| 6 | `semio-framework-2d` in consumer locks | DONE textually for 174 of 180 locks, validated (section 6) |
| P3-6 | `time-travel-turns` E0616; facade alias | alias DONE (facade test and fixture follow it); E0616 NOT REPRODUCIBLE, no change (section 8) |

## 1. runtime_lifecycle_grant (ruling: take the grant from the owner)

`📦️instances/🚪️opening/🦀️.rs`: the cancelled-opening close step now passes `cx.retained_grant()` (the owner-lent `RetainedCloneGrant` of the borrowed `StepContext`) to `app.close_step(cx, loan)`. `runtime_lifecycle_grant()`, its `RUNTIME_CLOSE_BYTES_PER_STEP` constant and the `RetainedCloneGrant` import are deleted. The mounted-owner TS law (`🏇️mounted-owner/🧪️tests/🟦️.ts`: `pluginFamilyText().includes("runtime_lifecycle_grant") === false`) already reads the whole plugin family, so it now covers `🚪️opening` too. `git grep` over the whole repo: no source file names the function any more.

## 2. Dead cooperative pump

Removed: `pump_runtime_live_cooperative_turn` and `runtime_close_pool` (`🔁️cleanup-turns`), `RuntimeAppCell::maintenance_probe_turns` (wasm-only, only the pump wrote it), the Rust test `🧪️tests/🤝️cooperative-pump/🦀️.rs` with its `tests-09` mount and the TS test (both deleted, tracked as ` D`), the empty fixture directory `🧫️fixtures/🤝️cooperative-pump` (this session), and the now unreachable fault `RuntimeCleanupFault::CooperativeClock` (enum, the two 19-entry tables now 18 entries, repr numbers 13..17 renumbered, `🩺️runtime-fault-vectors.json` entry `clock-cooperative`, renderer `🩺️window-fault` test: the removed code line and the count 19 to 18). Module doc of `🔁️cleanup-turns` no longer says "cooperative". `git grep` finds no remaining source reference (ticket notes excepted).

## 3. poll_kernel_turn decomposition

`🔄️turn/🌀️poll-kernel-turn/🦀️.rs` is now the phase driver (about 45 body lines): `admit_turn_entry`, `admit_lifecycle_focus`, `drive_turn_retirement`, `select_turn_event`, `dispatch_turn_event` (+ `ui_intent_event`, `job_completed_event`), `drive_extension_turn`, `drive_cold_pair_ingress`, `step_restored_document_loads`, `drive_command_ingress`, `advance_original_receiving`, `step_pending_patch_close`, `render_dirty_surfaces`, `drive_turn_reconcile`, `drive_turn_executors`, `settle_turn_work` (`TurnWorkInputs`), `fill_turn_patch_page`, `publish_turn_result`. New children (all wired in the root with `#[path]` + `use`): `🪜️entry-lifecycle-and-retirement`, `📨️event-dispatch`, `⌨️command-ingress-turn`, `🎨️dirty-surface-render`, `🔚️turn-finish`; the extension, cold-pair and more-work children received their phase functions by append. `bd12-tools/gen_rt.py` built every phase body from verbatim line ranges of the pre-edit snapshot `bd12-tools/orig/rt/` with declared mechanical substitutions (`install_rt.py` refused to run if a live file differed from the snapshot).

Documentation: the 27 statement docs that an earlier hoist had piled above the 962-line function are now on the phase function that owns the code they describe (`bd12-tools/route_docs.py`, text moved verbatim, pre-edit files in `orig/docs/`); the driver keeps its own summary and the M2 presence paragraph.

Verification (static):

* `rustfmt +nightly-2026-07-20 --emit stdout` parses all 18 turn children, the root and every other Rust file touched here: 0 errors.
* Differential typeck oracle (`bd12-tools/posrun-overlay.py`, round-2 union sandbox with stub layer; baseline = live tree with the turn folder replaced by `orig/rt`; lib and test): baseline 1,284 errors, current 1,283. Non-stub NEW rows: privacy 0, trait-scope 0, resolve 0, type 1 = `E0282` at `🔚️turn-finish` `with_pending_patches(|pending| {` — the stub of `with_state` takes 0 arguments, so the closure parameter has no expected type; the original has the identical `E0061` + `E0308` pair at the same call (snapshot line 926) and only the downstream `E0282` is not reported there. Stub artifact, not a code difference. Logs: `🗑️generated/c5a-bd12-diff-*` (swept by peers; command in section 7).
* The earlier name-resolution oracle (`oracle.py rt`) is dominated by the unlinked-crate noise (every new function signature names `semio_framework_job::…`), so it was not used as evidence.
* Source gate (static census): lib closure 20 files (limit 32), 3,034 lines (limit 4,000), 238,182 bytes (limit 262,144; ratio 0.91, in the 0.90 band of CRATE-HEADROOM — the next growth needs a split).

## 4. plugins.json is `[]` by design

`📇️registry/🤖️generated/🔌️plugins.json` is rendered by `📽️projection renderCatalogFiles` from the neutral `📣️publication/🔣️.json`, whose `contributions` is `[]` (the Oct 14 redesign: "publication authority is supplied by the application that owns the resulting catalog"). The framework registry therefore must be empty; the generator is correct. The real registry is `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json` (70 owners, 50,525 bytes), produced by `bun repo run @semio-tech/s-dev-composition:generate`; its inputs are the committed hub descriptor pairs and the built component deliverables (`componentDeliverableV1(...).wasm`), so a regeneration needs the wasm components (section 7).

Defect fixed: the identity law read the neutral registry and passed vacuously. It moved to the owner of the catalog, `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🪪️plugin-identity/🟦️.ts` (same body, imports re-pointed, generated path `../../🤖️generated/🔌️plugins.json`), and gained a non-vacuity expectation (`registry.length > 0`). Not run: `bun test ./<path>` stops at `Cannot find package 'ajv'` (the dependency image is missing, section 5). Note: a bare path makes Bun 1.3.14 treat it as a filter and crash while scanning the repository; always pass `./<path>`.

## 5. Twelve stale descriptor packs

`🧬️semantic-source-pairs` compares `🧩️component/🔣️.json` with the binary `🛂️.descriptor.semio` (canonical pack equal to the JSON). The 12 hub plugins (writer, wfc, vcs, animate, sequence, fem, reasoning, remodel, dag, block, space, sourcing) have regenerated JSON (ioEntries, topic contributions) and old packs. Only the describe step (`bun repo run workspace:components describe --all`) emits both forms from the guest `describe()` answer; a TS JSON-to-pack inverse would have to guess integer widths and enum tags, so no pack was hand-encoded. Also not re-checkable right now: the TS checkers import `@semio-tech/*` through the dependency image, which is missing (`Cannot find module '@semio-tech/framework/packs'`).

## 6. `semio-framework-2d` in consumer locks

Finding: the facade `semio-framework-2d` lost its `compute` alias and its `semio-framework-2d-compute` dependency (peer edit 00:08, facade lib + manifest). Consumers were re-pointed in manifests by 2D-COMPUTE-REPOINT, but 174 of 180 non-ticket `Cargo.lock` files still carried the old edges.

Method (`bd12-tools/locks/lock-sync.ts`): the per-package dependency delta is derived from the manifests (HEAD against working tree, family names only): 235 packages swapped facade for compute, 4 gained compute, the facade lost compute. For each lock the same edge edit is applied to every package block that exists in it (dev edges only for workspace members, optional edges never added; none occurred), insertion in byte order, unchanged blocks kept byte-exact, compute block pruned where it became unreachable. Result: drop-edge facade 1,181, add-edge compute 1,212, drop-edge compute on the facade 174, pruned compute blocks 8; 174 files rewritten, second run reports 0 changes (idempotent), `2dc-tools/validate-locks.py` over the touched files: sorted, no duplicates, no new dangling edge (4 dangling edges remain, all pre-existing and unrelated: `os-directory` ×3, layout editor-commands-tests to `io-ports`). Backup of the 174 originals: `GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/bd12-locks-before.tgz`.

Not exact, left for an offline refresh (`cargo update --workspace --offline` in these workspaces): pre-existing unreachable blocks the tool does not touch (`semio-framework-plugin-declared-verdict`, `…-testing-caller`, 27 font blocks in the animate/raster bridges, pack-schema blocks in note/shooting/bmp), and the 6 locks that were already consistent. `📦️units` test not run: `Cannot find module '@semio-tech/framework-compiler-ts/syntax/module-tree'` (dependency image missing).

## 7. Compile / window queue

1. `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-plugin-reactor-turn --lib --tests --message-format=short` (item 3; expected 0 errors; the statics above cannot see borrow checking or privacy of the new `pub(super)` items).
2. Same for `-p semio-framework-plugin-runtime-instances -p semio-framework-plugin-reactor-lifecycle -p semio-framework-2d --tests` (items 1, 2, facade test), then `cargo test -p semio-framework-plugin-reactor-lifecycle` cleanup-fault vector test (18 vectors).
3. `cargo update --workspace --offline` in `🧰️framework`, `✏️s` and the plugin workspaces; diff against the hand-patched locks (they must be identical on the family names).
4. `bun repo run workspace:components describe --all`, then `bun repo run @semio-tech/s-dev-composition:generate`, then the moved identity law and `bun repo run @semio-tech/plugin-registry:test-semantic-source-pairs`.
5. Oracle re-run (read-only, about 10 minutes): `cd F/bd12-tools && POS_PREFIX=c5a python3 posrun-overlay.py run families/bd12-diff.json --cfgs lib,test --label diff`.

## 8. time-travel-turns E0616 and open items

* E0616: the typeck oracle (`bd12-tools/families/bd12-tt.json`: `semio-framework-plugin-time-travel`, `-turns`, `-panel`; lib; `posrun.py`) reports privacy 0 and trait-scope 0 for the three crates. The four fields named in CENSUS5-A (`retirements`, `discarded`, `discarded_pending`, `discarded_factory`) are `pub(crate)` on `TimeTravelStoreState`, and every reader (`♻️retirement`, `🎮️decision/🗑️discard`, `🧱️store-owners-and-member-visitors`, the struct's own impl) is a module of the same crate; `git grep` finds no read in `🧵️instance` (the turns crate) or `🪟️panel`. The earlier rows came from the five-crate union family of CENSUS5-A, where the struct and its readers were flattened into different crates. No accessor API was added because no cross-crate reader exists; the owner already exposes `discard_issuer_missing`, `adopt_discard_issuer` and `queue_retirement`. A compile of `-p semio-framework-plugin-time-travel-turns` in the window (section 7) is the final proof.
* Fault-vector doc line "thirteen unrelated conditions" in `🧹️cleanup-vocabulary` is stale (18 now); left.
* Crates to compile: `semio-framework-plugin-reactor-turn` (lib + tests), `semio-framework-plugin-runtime-instances`, `semio-framework-plugin-reactor-lifecycle` (+ its fault-vector test crate `plugin-runtime-runtime-cleanup-fault-vector` and `…-runtime-close-budget`), `semio-framework-2d`, `semio-framework-plugin-time-travel-turns`.
