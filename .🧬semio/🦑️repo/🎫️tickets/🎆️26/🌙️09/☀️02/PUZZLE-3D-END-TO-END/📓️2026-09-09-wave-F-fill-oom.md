# W-F — Fill tool guest OOM and the 2 MiB test-stack overflow

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave F. Written incrementally while the work ran.

## 0. Assignment

1. A native law that drives the whole app through fill-tool activation plus N ≥ 300 `fillBuildTick`
   cycles **with the bounded job actually stepped**, asserting (i) retained heap stays under a fixed
   bound across cycles and (ii) the fill-count slider's `ready` becomes > 0. It must FAIL before the
   fix and PASS after.
2. The fix itself — architectural, no caps that merely hide the leak.
3. `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` must pass on the
   default 2 MiB test-thread stack.

## 1. Instruments built first

### 1.1 A counting global allocator

`…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` now carries `Puzzle3dHeapWitness`, a `#[global_allocator]`
compiled only under `cfg(test)`. It keeps a **per-thread** signed retained-byte counter
(`retained_heap_bytes()`), because the suite runs laws in parallel and one law's retention must not
be charged to another. The guest heap is a fixed 512 MiB wasm linear memory
(`.cargo/config.toml`'s `--max-memory=536870912`), so an owner the tick loop never frees is a hard
trap in production and invisible in a native suite unless the heap itself is weighed.

### 1.2 A shared fill-registry test guard

`fill_envelope_registry()` is ONE process-wide authority with `FILL_ENVELOPE_MAX_OPERATIONS = 4`
slots. The precompute tests module already had a private `fill_envelope_test_guard()`; the whole-app
laws need the SAME mutex or they do not serialize at all. It moved to
`…/✏️editor/⏳️precompute/🦀️.rs` as `pub(crate) fn fill_envelope_test_guard()`.

### 1.3 A stack high-water probe

Temporary probe (removed before close): a thread with a 64 MiB stack paints 24 MiB below its own
frame with `0xA5`, runs a body, then scans upward for the first byte that changed. That is the exact
peak stack the body consumed.

## 2. Deliverable (c) — the stack overflow

### 2.1 The brief's premise was wrong

Measured peaks (warm, debug profile):

| body | peak stack |
| --- | --- |
| `create_puzzle3d_app()` | 315 KB |
| `AppActionRegistry::from_definition` | 318 KB |
| **`fill_tool::measures` + `main::window_measures` + `main::engagement`, sync only** | **96 KB** |
| `VcsArtifactApp::with_registry` | 1 135 KB |
| `testkit::new_app_with_registry` | 1 334 KB |
| `+ bind_instance_id` | 1 420 KB |
| `app().await` (whole fixture, incl. its `Drop` drain) | 1 966 KB |
| `app().await` + one `dispatch("fillBuildTick")` | 2 575 KB |
| the failing law's body | 2 651 KB |

The measures path costs **96 KB**, not 2 MiB. The cost is the app fixture: ~1.4 MB to build it and
~0.6 MB for one `dispatch`, against a 2 MiB (2 097 152 byte) default test-thread stack. Every
app-fixture law in this crate therefore runs at 70–95 % of its stack, and the one that fails is
simply the one that adds the most on top. It passes under `--test-threads=1` only because libtest
runs a single-threaded suite on the process main thread, which has 8 MB.

### 2.2 The oversized carriers

`size_of` on the precompute session's members:

| type | bytes |
| --- | --- |
| `Puzzle3dPrecomputeSession` | 30 704 |
| `Puzzle3dCollision` | 30 088 |
| **`MountedFillWorker` (`BatchJobSession<SharedFillWorkerJob>`)** | **20 904** |
| **`StepOutcome`** | **8 256** |
| `FillBuilder` | 4 096 |
| `CollisionSpatialIndex` | 104 |

`MountedFillWorker` + `StepOutcome` = 29 160 of `Puzzle3dCollision`'s 30 088 bytes. Both are held BY
VALUE twice over: in `Puzzle3dCollision` (`fill_worker`, `fill_worker_outcome`) and in
`FillEnvelopeAuthority` (`worker`, `worker_outcome`). `Puzzle3dPlayApp` is 30 904 bytes as a result,
and it is built on the stack by `with_puzzle3d_app_for` on **every dispatch and every render** — the
same reason `document_tree_cache` was already boxed in that struct.

Taken over by W-F2 (cursor fleet) at 2026-09-10 00:57 CEST.

W-F landed the instruments (§1) and the stack diagnosis (§2). Boxing of
`MountedFillWorker` / `StepOutcome` (`OwnedFillWorker` / `OwnedFillOutcome`) is already in
`⏳️precompute/🦀️.rs`. `fill_envelope_test_guard` is `pub(crate)` there. The counting allocator
is process-wide (W-F corrected the per-thread counter: pool threads allocate, the tick frees).
The architectural fix was not applied.

## 3. Fix

### 3.1 Root cause (verified in source, not assumed)

Every `ArtifactApp` method builds a **fresh** `Puzzle3dPlayApp::default()` via
`with_puzzle3d_app_for`. Checkout restores only `Puzzle3dCollisionSession` (brush lane + meshes +
scene). The fill cursor — `fill_job`, `fill_admission`, `fill_observation`, `fill_faulted`,
`fill_terminal`, `fill_applied_count` — lives on `Puzzle3dPrecomputeSession` and is **not**
checked in. Then `Drop for Puzzle3dPrecomputeSession` terminalizes whatever `fill_job` that
call just admitted and incrementally closes it.

Consequences, one per 120 ms `fillBuildTick` / every render:

1. `sync_precompute_session` → `SetScene` → `install_scene` always `supersede_admitted_fill`
   (no-op, cursor is already gone) then, because `engine.fill` is absent from the collision
   session, `start_fill_preparation(true)` allocates a new `FillBuilder` (~2.8 MB of
   preparation pages on a real document).
2. `enqueue_fill_job` sees `fill_job == None`, admits a **new** envelope, emits `SpawnJob`.
3. End of `with_puzzle3d_app_for`: Drop closes that envelope before the host can step it.
4. Slider `ready` stays 0 and `loading: true`: `fill_progress_summary` reads a brand-new
   builder (`sequence.len() == 0`, not stalled). Native tests that "plan fine" used
   `drive_enqueued_fill_job_for_test` through `with_puzzle3d_app_mut`, which is a **different
   empty app** than the dispatch session — the bypass is a no-op. Wave J's host job driver
   never gets a live envelope to step: the guest already closed it.

That is both the guest OOM (~2.8 MB retained per tick until 512 MiB traps) and the
native/wasm `ready == 0` asymmetry. It is guest-side (session cursor dropped across
`with_puzzle3d_app_for`), not host-side TypeScript.

`fill_faulted` cannot latch across ticks for the same reason. The four-slot registry does
not cap the leak if Drop's close is incomplete or if each tick's preparation is retained by
the job runtime / a half-closed handle.

Brush-queue re-add (wave D4 §4): `precompute_step_lane` re-queues on `unknown_pending`
regardless of `resume_candidate_index`. The in-source comment documents that gating on
`resume_candidate_index > 0` livelocked targets that never cleared the broad-phase index.
Replacing the cache entry is bounded; this is not the 2.8 MB/tick owner. Left as-is.

### 3.2 What the fix must do

1. Persist a lightweight `Puzzle3dFillSession` cursor in `Puzzle3dSessionState`; take it
   before PlayApp drop so Drop does not close a live envelope.
2. `SetScene` must not supersede or re-prepare on a no-op / applied-projection sync.
   Applied-projection detection must use `read_fill` (registry owner), not `engine.fill`
   (cleared at admission).
3. Drive the bounded job through `start_job` / `step_job` in the whole-app law and in
   `drive_fill_until_ready` (replace the dead bypass). Do not use the bypass for the new law.
4. Keep the already-boxed worker/outcome carriers; box `Puzzle3dPlayApp.precompute` so the
   Default() frame on the 2 MiB test-thread stack is a pointer.
5. Remove W-F's unguarded `[DEBUG]` `println!` in `new` / `supersede` / `enqueue`.

W-U owns command arms / scope table in `✏️editor/🦀️.rs` (live: `relocateTargetVolume`
kind). W-H2 added `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`.
Edits in `editor.rs` stay in session structs + `with_puzzle3d_app_for`.
