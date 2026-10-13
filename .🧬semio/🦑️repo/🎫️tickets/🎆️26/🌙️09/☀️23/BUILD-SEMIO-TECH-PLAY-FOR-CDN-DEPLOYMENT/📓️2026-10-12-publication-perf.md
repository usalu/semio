# 2026-10-12 publication performance after the grant-based retained-clone migration

Executor: pub-perf (Sonnet). All cargo ran in the foreground of its own slot through `🚦️cargo-slot.sh` with `SLOTS=2` and the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/pub-perf/target` (deleted, with every binary copy and result directory, at the end). No git modification, no ticket operation, no sub-agent.

## Result

| proof | before | after |
| --- | --- | --- |
| hub `editor_catalog`, 177 tests, one process each (`each-catalog.sh`, 5 parallel, 240 s alarm) | 88 ok (60 of the failures end at the 30 s `settle_registered_typed_operation` deadline) | **160 ok / 16 FAILED / 1 TIMEOUT**, 0 regressions; none of the 17 is a performance stop (below) |
| slowest settle of the catalog (debug) | 30 s deadline hit by 60 editors | **1.1 s** (pptx, 5.4 MB document, 34 370 units); json_any 0.24 s (609 units) |
| small JSON edit (json_any_editor), host iterations of one `settle` | never retired; once the livelock was fixed 1 402 | **64** |
| config edit, one-item turns to `Prepared` + close (shooting `SetCamera`, grant items 1) | ~2.9 k (brief) | **813 + 111** (about 30 host turns at the shipped item budget 32) |
| document edit, one-item turns (shooting `CreateShot`) | not measured | **1 069 + 126** |
| `RetainedPayloadBuilder` init turns per publication | ~256 | **2** |
| plugin `--lib`, one process per test (1 230 tests, 6 parallel, 180 s) vs `pl-tests/r15/results.tsv` (951 ok) | 951 ok | **952 ok, 0 regressions** (+1 new law green) |
| os-kernel `canonical_edit` | 31 | **31 passed** |
| os-kernel `--lib`, one process per test (1 465 tests, 6 parallel, 120 s) vs `fw-tests/kernel-each/results.tsv` (1 346 ok) | 1 346 ok | **1 353 ok, 0 regressions** (the `hot_path_tests::local_steps_cost_o_change_however_long_the_history` law needs 125 s alone, 151 s at the fw-tests baseline; it hit the 120 s alarm under load; two names vanished because peers renamed them) |
| replication `--lib` | 340 | **346 passed** (seal and authority cursors changed) |
| stdio-semio `preparation_law` | red 5/5, `(544, 0)` close accounting | **5 passed** (the 544 bytes were the allocation of my own `[DEBUG]` timing table inside the heap witness, now removed) |
| shooting `preparation_law` (doc + config) and `authoring_law` | - | **2 passed + 1 passed** (permanent turn bounds added) |
| png `preparation_law` / bmp `preparation_law` | 4 / 2 | **4 passed / 2 passed**; png 1 MiB image edited in 773 turns, 4 MiB in 867 |
| `cargo check --lib` native: os-kernel, plugin, value, job; `--target wasm32-wasip2`: os-kernel, plugin, value | - | all Finished, 0 errors |

The 17 catalog stops that remain: 11 editors without an edit fixture (docx x3, las, svg x3, tiff x2, xml x2), 5 `snapshot-edit.lossy-conversion` on the saved-source reopen after the publication succeeded (jpg x2, pdf17 ua/vt/x), and the known 300 s debug timeout of `sqlite_snapshot_semio_any_editor`.

## Root causes (in the order they were found)

1. **An unfundable demand that yields silently.** `admission::admit_raw` reserved the 16 MiB maximum raw extent for every retained command; the closing job then demanded a 16 MiB `release_bytes` turn while the mounted grant releases 1 MiB. `advance_retirement_turn` answers an unfunded demand with an empty receipt, so the close ladder spun until the 30 s deadline, forever, with no fault. `admit_raw(declared_bytes, maximum_raw_bytes)` now reserves exactly the declared extent (refusing a declaration above the maximum or an allocation failure), `admit_limits` carries the pure limit check; the language-agnostic fixture `🎟️admission/🧫️fixtures/🔣️.json` (version 2: `limits` + `raw` arrays) is read by the Rust test and the TS oracle.
2. **A job that retired nothing it borrowed.** `ArtifactRetainedCommandJob::close_step` refused every job that still held its command, snapshot, config, history, interaction and operation inputs. They are now retired through `input_retirement` lanes: owned inputs through `artifact_retirement_admit_owned`, the shared roots through the new `store::artifact_retirement_admit_shared_lease` / `artifact_retirement_shared_lease_birth_demands` (the duplicate test helper in `🧪️tests/⏳️completion` now uses them).
3. **Per-unit overhead that scaled with nothing.** `RetainedPayloadBuilder::advance_initialization` initialised its 256 page bindings one per turn: now in pages priced by `copy / size_of::<Option<JobPayloadPage>>()`, the header joins the finishing turn (256 -> 2 turns). Bounded work (`BoundedArtifactCommandWork`, `ArtifactCommandWork::reports_progress() == false`) no longer emits previews and checkpoints. The worker handshake fuses poll Outcome -> semantic -> ack -> resume when the outcome is an empty yield.
4. **One ownership receipt per host turn.** The migration made the pump return after every receipt, so a publication cost one host turn per unit. `drive_typed_operation_worker`, `publish_mounted_typed_operation_run_admitted` (new `publish_mounted_typed_operation_pump`) and `maintenance_ladder_step` (a batching wrapper over `maintenance_ladder_unit_step`) now loop units inside the host turn, **bounded by the unchanged host grant**: every unit is quoted against `remaining_plugin_turn_grant(turn, spent)`, the receipts are summed exactly, and the loop leaves on exhausted items, a unit without progress, a wall guard (`INTERACTIVE_TURN_WORKER_WALL_US`) or an unchanged witness. The stage-0 maintenance worker and the worker pump no longer build a thread pool per turn on native. Accounting, cancellation and progress semantics are those of the single-unit path; the grant's `maximum_items` is the control knob.
5. **Two passes where one suffices, and byte pairs compared whole.** The canonical seal cursor (`🔏️seal`) counted in one pass and hashed in a second; it now keeps the counted bytes in its 1 024-byte page and jumps to the owner take when everything fits (falls back to the hash pass otherwise). The semantic authority compare (`🛂️authority`) pages its byte pairs by `maximum_copy_bytes / 2` across fields.
6. **A read lease nobody returned.** Every typed operation takes a lease on the plugin's `TransientStore` for its `ArtifactOwnedToolJobContext`; after the job closed, the returned root waited for a pump that did not exist for that store (document, config and interaction stores have one), so 47 otherwise finished editors died at the registry `Drop` assertion and a long-lived app would exhaust its 1 024 slots. Fix: `TransientStore::maintenance_returned_reads_step_root` / `_demands_root` / `returned_read_pending` (root-only retirement factory variant `advance_returned_snapshot_read_root`), a fourth transient stage in the plugin returned-read rotation (`next_snapshot_read_return_stage` is now modulo 4), and `BoundedTransientStoreDisposer` drains the returned reads of the store it displaces before dropping it.
7. **Whole-snapshot apply needs the capacity grant of its clone.** `MutationApplyEdit` Stage::Apply is one snapshot-sized turn quoted from the exact retained capacity of the clone (267 KB .. 5.4 MB for the 28 gltf/pdf/ifc/step/xlsx/pptx/dwg/gif/epw fixtures) and answered an empty step under the 256 KiB capacity grant, which the plugin reports 4 096 units later as `publication-stalled`. The stdio policy now grants 16 MiB of capacity.
8. **Test side:** the final close loop of `editor_catalog` never took the constructor receipts; it now uses `artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy())`.

## The shipped policy

`MountedOwnerPolicyV1` stays explicit per owner. The stdio hub composition (`🌎️hub/🧩️compositions/🗄️stdio`: the generated root `🦀️.rs`, its generator `🧩️composition/🟦️.ts`, the nine `🧩️extensions/*/🦀️.rs`, the `🚢️shipped-fleet` test) and `contract::testing::mounted_policy()` now grant `{ items 32, copy 32 KiB, capacity 16 MiB, release 1 MiB, depth 4096 }` for preparation, maintenance and close (was items 1, capacity 256 KiB). Measured with the same binary: items 1 -> 982 host iterations for json_any, items 32 -> 64.

## Permanent proofs added

- `🎟️admission` fixture version 2 + Rust test + TS oracle (`bun test` 2 pass).
- shooting `preparation_law`: `PREPARATION_TURN_BOUND = 2048` / `CLOSE_TURN_BOUND = 256` turns for the document and the config lane (measured 1069/126 and 813/111; the config lane was ~2.9 k).
- `editor_catalog` itself is the proof of the transient drain (registry `Drop` asserts on any unreturned lease) and of the settle deadline (30 s, restored from my temporary 40 s).

## Remaining floors and follow-ups

- Large documents are structurally unit-bound: pptx (5.4 MB) 34 370 units, pdf17_vt 8 292, i.e. about 6.4 units per KiB (clone, two canonical traversals, hash, fold, item close at 32 KiB copy per unit). Under items 32 that is about 1.1 s in debug. Further cuts are structural (per-phase unit counts measured earlier for one small edit: pclose.source 53, authority source close 49, itemclose.stage 174, identity 47).
- A snapshot above the capacity grant (16 MiB) still stalls at Stage::Apply with `publication-stalled`; the real fix is a paged `apply_diff`, not a larger grant.
- The other ~45 hub compositions and the plugin crates under `✏️s/🔌️plugins` keep the literal `items 1 / capacity 262_144` (not part of the measured stdio hub); they get every structural fix above but not the host-turn batching. Adopting the stdio literal there is a one-line change per root, to be validated per composition.
- `hot_path_tests::local_steps_cost_o_change_however_long_the_history` needs a 150 s alarm in debug.
- Job crate `--lib` tests do not compile (pre-existing old API), so the builder init change is covered by the existing builder cut tests (258/259) through the plugin tests, not by a job-crate run.

## Files touched

Permanent (under `🧰️framework/🛍️products/💻️os/🔨️modules/` unless absolute): `🔌️plugin/🧵️retained-command/🎟️admission/{🦀️.rs, 🧫️fixtures/🔣️.json, 🧪️tests/🦀️.rs, 🧪️tests/🟦️.ts}`, `🔌️plugin/🧵️retained-command/🦀️.rs`, `🔌️plugin/🦀️.rs`, `🔌️plugin/🪜️maintenance-ladder/🦀️.rs`, `🔌️plugin/🫧️transient/🧵️publication/🦀️.rs`, `🔌️plugin/🧪️tests/⏳️completion/🦀️.rs`, `🏪️store/🦀️.rs`, `🏪️store/🧬️snapshot-clone/⚖️law/🦀️.rs`; `🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🦀️.rs`, `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🛂️authority/🦀️.rs`, `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`; `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧰️testing/🦀️.rs`; `🌎️hub/🧩️compositions/🗄️stdio/{🦀️.rs, 🧩️composition/🟦️.ts, 🧩️extensions/*/🦀️.rs (9), 🧪️tests/🚢️shipped-fleet/🦀️.rs, 🧪️tests/✏️editor-catalog/🦀️.rs}`; `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/⚖️preparation-law/🦀️.rs`.

Debug-only touches, all reverted (net diff zero from my side): `🧰️framework/🔨️modules/🌱️value/🦀️.rs` (`debug_turns`), `🌱️value/♻️retirement/🎟️turn/🦀️.rs`, `🧵️job/🦀️.rs`, `🏪️store/{🔗️read/🔒️registry, 📬️paged-one-item, 📬️paged-one-item/🧬️mutation-apply, 🧬️snapshot-clone, 🧵️canonical-edit, 🧵️canonical-edit/🧳️source}/🦀️.rs`, `🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs`. `grep` of `[DEBUG]`, `debug_turns`, `SEMIO_DEBUG` over the diff of every touched file against HEAD finds none of mine (the `[DEBUG]` SQLite lines of `editor_catalog` are the io-sqlite executor's).
