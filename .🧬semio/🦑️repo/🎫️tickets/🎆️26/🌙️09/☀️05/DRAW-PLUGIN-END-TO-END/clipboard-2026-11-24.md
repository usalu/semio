# Draw clipboard: copy never finishes in the browser guest r36 (2026-11-24)

Status: root cause NOT proven. A native browser-grant reproducer is written but has never run, because no native build could start (section 5). Nothing in the clipboard production code was changed.

## 1. What is proven from code (read, not run)

- Registration: `copy`, `cut` and `paste` are framework reserved routes (`framework_reserved_job!(FrameworkCopyJob ...)`, `...CutJob`, `...PasteJob` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs`, ordinals 7, 8, 9; copy lane `HostOnly`, cut and paste `Artifact, Child, Interaction`). Draw overrides the body: `DrawingPlayApp::build_reserved_tool_job` (`D/✏️editor/🖥️play-app/🦀️.rs:226`) admits `crate::clipboard_job::DrawingClipboardJob` through `admit_reserved_work` for the ids in `DRAWING_CLIPBOARD_RESERVED_IDS`. So copy, cut and paste are one job type, not Draw commands. The factory is registered (no `interactive-job.missing-factory`), and the job is admitted, because the command answers `command-complete` with the typed operation id.
- Puzzle's clipboard job (`🧩️puzzle/.../✏️editor/✂️clipboard-job/🦀️.rs`, 8 KB) is a plain outbox job. It has no `SnapshotRead` custody. Draw's job (`D/✏️editor/📋️clipboard/🧵️job/🦀️.rs`, 64 KB) is the only one that first duplicates and seals the captured `SnapshotRead<DrawingSnapshot>` (`seal_source`), and that is the structural difference.
- `DrawingClipboardJob::step` has two guard returns that give `InteractiveJobStep::Outcome(None)` (a yield with no work, no fuel spent, no effect, no error):
  1. input capture (first step): `cx.fuel_remaining()==0 || retained_grant().maximum_items==0 || maximum_copy_bytes < 2*size_of::<Option<ArtifactReservedToolInput>>()`;
  2. `seal_source`: `retained_clone_grant_admits(grant, SnapshotRead::duplicate_original_demands())` (copy bytes, depth 2), then `retained_clone_grant_admits(grant, sealing.next_demand())`, and inside `SnapshotReadSourceStaging::funded` also `effect.remaining_work_units() >= next_advance_work_demand()`; when any of them fails, `step` answers `progress(0)` without an error.
  Both are silent: nothing faults, nothing is emitted, and the only symptom is a drain that runs turns with the cost of an idle turn. The measured drain (about 150 fuel and 2.3 items per turn, 29,000 turns, no effect, no wake, in `local-query-stall-2026-11-24.md` section 4) matches an unfunded silent yield (a funded job would spend fuel and items in proportion to work). It does not match a job that runs and loops: `turn()` would then burn fuel until `should_yield`.
- The clipboard-write effect is only created in publish phases 37 and 38 (`workspace.effects.push(Effect::ClipboardWrite ...)`), which need the sealed source (phases 2 and 3 read from `workspace.source`) and the JSON encoder (phase 4). So a job that is stuck before phase 2 produces exactly the observed picture: command-complete, no `clipboard-write`, no `spawn-job`. (`spawn-job` is not expected: reserved jobs run inside the guest's mounted typed operation, not as a host-stepped spawned job.)
- `cut` and `paste` use the same step entry (seal first, then the phase machine), so a stuck seal would hold them too. The report that they "complete without changing rows" is not explained by this; see not verified.

## 2. Inferred, not proven

- The most likely cause is an unfunded demand in `seal_source` (or its input capture) that is silently refused under the browser's per-turn grant, as opposed to the native fixture's. The candidates, in order: the sealing projection quote (`SnapshotReadSourceStaging::next_demand`, `projection_stage_quote`), the work-unit gate in `funded`, the capture gate. I did not measure any of these numbers.
- Why this would differ from native: the native fixture renews each turn from the fixture caller (16,777,216 items, 528 MiB copy, 2,000,000 fuel), narrowed by `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` (64 items, 512 KiB copy, 16 MiB capacity, 1 MiB release). The browser shard grants 50,000,000 fuel and an actor-source remainder that shrinks per turn. `arm_host_turn_grant` / `arm_host_turn_fuel` / `arm_host_lifetime` exist to model exactly that and are used by the PDF, Procedural and Sourcing native laws.
- Not the cause (read): Draw's registration, the `DrawingClipboardJob::turn` loop (it would spend fuel), the host drain (it only submits idle turns and re-arms on `more-work`).

## 3. Reproducer written (never compiled)

File: `D/✏️editor/📋️clipboard/🧪️tests/🪟️runtime/🦀️.rs` (included as `clipboard_runtime` by `D/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, crate `semio-s-artifact-draw-drawing-editor-interaction-tests`). Added, below the existing `registered_clipboard_copy_paste_cut_preserve_atomic_inverse_and_live_selection`:

- `browser_clipboard_settled` / `browser_clipboard`: dispatch a verb through the real `DrawingAppFixture`, with the shard's turn fuel (50,000,000), the interactive per-turn grant (`INTERACTIVE_COMPONENT.preparation`) and a host lifetime armed, then settle the typed operation with `settle_registered_typed_operation_within` and a hard bound of 4,096 turns. A stall surfaces as the fixture's own "original fixture settlement stopped after N turns (fuel, stage, demand, grant, ...)" refusal, which names the unfunded demand and the grant. That message is the missing measurement.
- `browser_grants_copy_emits_clipboard_write_and_paste_inserts_the_layer`: select the rectangle, copy, require exactly one `Effect::ClipboardWrite`, paste that fragment, require one more layer, a new selected id that exists in the document.
- `browser_grants_copy_without_selection_is_refused_within_the_turn_limit`: copy with an empty selection must be refused, and must not stall.

No assertion was weakened or removed. The existing test is unchanged.

## 4. Verification of the earlier PagedUtf8 controlled-decode fix (`native-tests-2026-11-23d.md` item 9)

Still unverified. No native test of the crate ran to a result in this pass (section 5).

## 5. Commands and exact results

All from the drawing workspace `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing`, `SEMIO_OFFLOAD=never SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en repo cargo --store=isolated --configuration=prototype -- test --no-fail-fast -p semio-s-artifact-draw-drawing-editor-interaction-tests clipboard -- --test-threads=1` (logs in `🗑️generated/clip-native-1..10.txt`).

| run | result |
|---|---|
| 1 | `semio-plugin-instance-typed-ops` did not compile (`match settled` on a `bool` against `ToolSettlementUnit`, three E0308): another agent's in-flight edit of the framework typed-operation settlement; later fixed by its owner. Also ENOSPC. |
| 2, 4 | ENOSPC at 660 to 710 of about 1,300 crates. The isolated Rust image `.cache/b/r/fixed.dmg` is 26 GiB (declared 96 GiB); the incremental directory is about half of what is written. Every failed run leaves the lane at 4.3 GiB (evicted), so a retry recompiles everything (about 20 minutes). |
| 3 | refused: `CARGO_PROFILE_*` env is not allowed ("profile environment belongs to issued consumption"). |
| 5 | with `CARGO_INCREMENTAL=0` (accepted; incremental dir stays 0 B, about 6 MB per crate, so a full build fits): reached `semio-s-artifact-draw-drawing-editor-interaction` and failed with E0599, `PagedUtf8::as_str` missing in `✏️editor/📌️panels/🗂️layers/🦀️.rs:65` (the model's `shape_kind` is a `PagedUtf8` now). I fixed it (see files). |
| 6 | failed before compiling: `Internal error: Rust crate Source boundary exceeded before compiler: semio-s-artifact-stdio-pdf-lift (active lines 4010/4000) (reference 1f43f6c1)`. |
| 7 to 10 | the same boundary refusal on every attempt (`clip-native-10.txt.attempts` counts 11 attempts over about 30 minutes; the script `retry.sh` in the session scratchpad retries every minute and starts the real run as soon as the boundary clears). |

The blocker is the PDF base lift crate: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (modified 16:59, +289/-84 lines of an ExtGState / page-resources change in the working tree). The crate is 10 active lines over the 4000 line boundary. The `repo cargo` source admission checks every crate of the test's dependency closure (`semio-framework-job-registry` pulls the whole plugin set), so no Draw native test can start until that crate is split or trimmed by whoever owns it. Because only I may compile, that owner cannot see this failure. I did not touch the PDF crate.

## 6. Files

- Changed: `D/✏️editor/📋️clipboard/🧪️tests/🪟️runtime/🦀️.rs` (three new tests and helpers), `D/✏️editor/📌️panels/🗂️layers/🦀️.rs` (`layer_icon` uses `PagedUtf8::eq_str`; one-line compile fix required to build the crate).
- Read: `D/✏️editor/📋️clipboard/🧵️job/🦀️.rs`, `🛫️encode`, `🧪️tests`, `D/✏️editor/🖥️play-app/🦀️.rs`, `🧰️framework/.../🔌️plugin/👷️job/🏛️reserved/🦀️.rs`, `.../📜️declaration/👷️job/🎟️request/🏭️reserved/🦀️.rs`, `🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🪜️staging/🦀️.rs` and `.../👥️duplicate/🦀️.rs`, `🧰️framework/.../🧪️testing/🧪️artifact-app-laws/.../🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`, Puzzle `✂️clipboard-job`.

## 7. Not verified

- Every statement in sections 1 and 2 about runtime behaviour; the reproducer has not compiled, let alone run.
- Whether `copy` with an armed browser grant stalls natively at all. If it passes natively, the remaining differences are wasm32 struct sizes, the host wall clock deadline per turn, and the shrinking actor stock, none of which a native test models.
- The `PagedUtf8` fix, the three earlier remaining interaction failures, memory of a full Draw native run.
- No fix was made: with no measured demand there is no honest root-cause change. The next step, once the PDF boundary is cleared, is to run the reproducer and read the refusal message (stage, demand, grant); if the demand exceeds the grant, raise nothing at the host but quote the sealing demand into the grant the clipboard job funds, or split the seal into fundable steps.

## 8. Update 18:55: first native reproducer results (runs 10 to 13, crates `semio-s-artifact-draw-drawing-editor-interaction-tests` and `...-editor-tests`)

- Run 10 (interaction-tests, `clipboard`): 8 pass, 1 fail. The `PagedUtf8` controlled-decode fix is verified (`drawing_clipboard_native_codec_matches_serde_and_resumes_large_text` passes). The failing `hydration_preserves_admitted_identity...` ("funded final payload extent cannot extend or reallocate") is the coordinator's `decode::layer()` reservation bug, fixed by the coordinator; not rerun yet.
- Run 12 (editor-tests, `clipboard`; `clipboard_runtime` is mounted only there): all three runtime tests, including the existing `registered_clipboard_copy_paste_cut_...`, stopped at the `copy` dispatch with `UnsupportedOwner` "native Window transient capture requires an already admitted original partition". Cause (product): `TypedWindowTransientStoreOwner::capture_original_header` (`🔌️plugin/🪟️window/🫧️transient/🦀️.rs`) required an existing partition, while the non-original `capture()` creates it. A reserved framework job (copy/cut/paste) captures the window transient of a window that was never touched. Fix: `capture_original_header` now takes `&mut self`, adds the partition's birth bytes to its construction demand when the partition is absent, and creates it inside the funded `built` unit. No fixture seeding. A dedicated framework law is still to write; the clipboard runtime tests are the law for now.
- Run 13 (same crate, after that fix): the dispatch passes; the three tests now fail with the stall the browser shows:
  - `browser_grants_copy_emits_clipboard_write_and_paste_inserts_the_layer`: "original fixture settlement stopped after 4096 turns (fuel 50000000, deadline exceeded false, pending true, maintenance empty false, stage 18, demand copy 875008 capacity 437888 depth 2, grant items 61 copy 523215 capacity 16777216 release 1048576 depth 4096, close owner Begin)". The quoted maintenance demand (875,008 copy bytes) is above the grant (523,215), so that quote is unfundable under the interactive grant.
  - `browser_grants_copy_without_selection_...`: the same stall (an empty selection is not refused either).
  - The existing `registered_clipboard_copy_paste_cut_...` (fixture grant, no browser narrowing) also stalls: "stopped after 4104 turns (fuel 1999999, pending true, stage 0, demand copy 1 depth 1, grant items 64 copy 524288)". Fuel 1,999,999 of 2,000,000 means each turn spends one unit: the job yields at once without work, exactly the silent `Outcome(None)` guard of section 1. So copy stalls natively with the fixture's own grant. It is a product bug, not wasm32 or the shard, and it predates the browser narrowing.
- Not yet known: which silent guard (input capture, `duplicate_original` admission, `sealing.next_demand()` against the grant, or the work-unit gate) refuses. Next run: `[DEBUG]` prints in `seal_source` and the capture step (grant, demand, remaining work units), then fix at that guard. No further native run is started until the coordinator's r37 guest build has finished.

## 9. Update 19:20: the stall quote and the size law

- The quote that stops the settlement (copy 875,008, capacity 437,888, depth 2) is the close/retirement quote of the reserved job body: `admit_reserved_work` allocates one `ArtifactReservedRequestJob<_, DrawingClipboardJob>` (`size_of` = 437,504 bytes by the capacity quote) and retires it by moving it twice. 875,008 > the interactive copy grant of 524,288, so the body can neither close nor (together with the teardown) settle. The job's inline fields are the offender; which one is not measured. Candidates: `Workspace` fields (`DrawingScenePlacementJob`, the JSON read/write cursors, `SnapshotReadSourceStaging`, `ClipboardHydration`) and the clone authorities. `DrawingLayerLocator`, `DrawingLayerCloneWorkAuthority`, `PagedList` and `metadata::Table` are all a few KiB and are not it (read).
- Law added (not compiled, r37 owns the compiler): `clipboard_job_body_fits_a_small_share_of_the_interactive_grant` in the clipboard runtime tests; it prints the `size_of` of the job and its candidate components. The first run after r37 names the large one, which then moves behind funded heap admission (step-wise, like Puzzle's job) instead of raising the grant.
- The existing `registered_clipboard_...` test's stall (stage 0, quote 1 byte, fuel 1,999,999 per turn) is a different wait: the job yields on its first guard each turn; its cause is not found. Check after the size fix.

## 10. Prepared, not compiled (19:30)

`D/✏️editor/📋️clipboard/🧵️job/🦀️.rs`: `Workspace.placement` is now `Option<Box<DrawingScenePlacementJob>>`. Phase 34 yields (no partial effect) unless fuel, items, copy bytes, capacity bytes and depth admit `size_of::<DrawingScenePlacementJob>()`, then boxes the job and charges it (`copied_items 1`, `copied_bytes` and `retained_capacity_bytes` = its size) next to the existing work receipt. No grant raised. JSON cursor and hydration boxing waits for `native-size.txt`. Not compiled; compile after r37's final unit.

## 11. Browser trace on r38 (01:40)
`probe-clipboard-debug.mjs`: sealing works (16 funded steps, work demand 1 to 5 of 256, source published), then `turn` runs at phase 0 with 256 fuel left and never advances (26 turns seen before the 60-line cap). `turn` returns before `advance` only when an outcome is already decided, and the first turn spent one fuel unit at phase 0, so `prepare` failed there and the fault is not handed to the operation. Suspicions in section 1 (capture and seal guards, 256-unit contract) are refuted. r39 prints the prepare error, every `lend`, and the worker outcome in `drive_worker_step` (all `[DEBUG] clip`, to be removed).
Reserved copy without a selection must fail visibly through the same hand-over.
