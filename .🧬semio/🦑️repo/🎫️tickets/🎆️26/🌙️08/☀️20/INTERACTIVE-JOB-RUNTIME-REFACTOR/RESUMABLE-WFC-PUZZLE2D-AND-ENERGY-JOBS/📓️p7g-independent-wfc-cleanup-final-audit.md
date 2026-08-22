# P7g Independent WFC Cleanup Final Audit

Date: 2026-08-22  
Scope: Current P7 cold guest relay cleanup-race repair, hot-shard cancellation, production
ActionBus/freshness route, inference bridge, and WFC checkpoint/commit bounds.  
Method: Fresh read-only source/static audit. Read root `AGENTS.md`, all of
`p7a-wfc-job.md`, `p7e-independent-wfc-panic-postrepair-audit.md`, and
`p7f-independent-wfc-cancel-final-audit.md`, including their appended dispositions. No Cargo,
Bun, build, test, runtime, Wasm, cache/target, ticket-status/JSON, git, or production-source
mutation was performed.

## Verdict: REJECT

P7f's cited cold-relay cleanup ownership P0 is repaired in the current source. The new state
machine, lease dispositions, retained-panic path, admission checks, one-shot cancellation gates,
and three deterministic race regressions withstand static review. The source no longer exposes the
guest as `Available` after a failed/panicked start or step and before cleanup resolution.

However, the current WFC maximum-checkpoint test fixture is deterministically malformed by eight
bytes. It cannot pass the production restore header's exact-length validation, and the associated
allocation test admits `MAX_CHECKPOINT_BYTES - 8` while asserting it is the exact maximum. This
invalidates the claimed exact-maximum checkpoint coverage required by P7; no executable result was
run to conceal the issue.

## P0 — Maximum Checkpoint Fixture And Exact-Maximum Bound Test Are Incorrect

The production checkpoint header is exactly 168 bytes:

- `CHECKPOINT_MAGIC` is 8 bytes (`wfc job:18`);
- `CheckpointBuild::new` budgets 168 plus the variable sections (`:320-339`), and
  `checkpoint_one` reserves/writes that same 168-byte header (`:964-990`);
- `WfcRestore::decode_header` reads that header and computes
  `expected_bytes = 168 + domains + trail + decisions + observed`, rejecting when it differs from
  the input length (`:1241-1287`).

In contrast, the test helper `maximum_checkpoint` truncates a valid checkpoint to **176** bytes and
uses `(MAX_CHECKPOINT_BYTES - 176) / 8` observations (`:1682-1695`). It leaves an extra eight-byte
gap after the 168-byte encoded header. With `observed_count = (MAX - 176) / 8`, production decode
computes `expected_bytes = 168 + observed_count * 8 = MAX - 8`, while the helper asserts and passes
an input of `MAX`. `WfcRestore::new(...).expect("maximum admitted restore")` in
`maximum_checkpoint_restore_is_bounded_and_cancellable_in_every_phase` (`:1871-1875`) therefore
cannot reach the claimed restore/cancellation coverage: the header parser returns
`wfc-checkpoint-input-mismatch` first.

The allocation test repeats the same incorrect 176-byte premise (`:1909-1937`): its admitted
state has `(MAX - 176) / 8` observed entries, so `CheckpointBuild::new` budgets and reserves
`MAX - 8`, not `MAX`; yet the test asserts `checkpoint.byte_limit == MAX`. It tests allocation
reservation alone and does not drive `checkpoint_one` through an exact valid maximum. The following
one-item-over assertion is correspondingly measured from the wrong baseline.

This is a source-deterministic failure of the requested exact-maximum checkpoint test contract,
not a timing-dependent concern. Repair must make the fixture and exact-max test use the single
production header constant/contract (rather than divergent literals) and drive a valid maximum
checkpoint through serialization and restore. This audit does not implement that repair.

## Cleanup-Race Repair: Static PASS Evidence

| Requirement | Current evidence | Result |
| --- | --- | --- |
| Atomic `Available` / `Leased` / `CleanupPending` / `Quarantined` ownership | `GuestInstanceSlot` owns all four states (`host:2762-2767`). Acquire replaces/restores the slot under the poison-tolerant mutex; Drop installs the disposition while holding that mutex (`:2812-2908`). | PASS |
| Ordinary start/step fault becomes cleanup-pending before permit release and receiver delivery | Error arms set the mounted lease disposition to `CleanupPending` (`:3016-3037`). The request explicitly drops the lease before the semaphore permit, schedules cleanup, then sends completion (`:3043-3051`). | PASS |
| Retained start/step panic has the same ordering | Every mounted lease defaults to `CleanupPending` (`:2813-2819`). `GuestRelayPoolFuture` takes/drops the retained future before its panic handler (`:2660-2670`); future-local destruction drops the later `guest` before the earlier `permit`. Panic recovery then schedules cleanup and sends the one fault (`:3086-3127`). | PASS (static) |
| Mounted preflight and post-preflight race cannot enter the guest | `run_job_on_worker` preflights `CleanupPending`/`Quarantined` (`:3324-3328`). A route that raced that check must subsequently acquire the semaphore and `GuestInstanceLease::acquire`, which returns the stored pending/quarantine detail instead of a guest (`:2813-2834`); its `Rejected` completion clears its own cleanup flag, so it cannot schedule cancellation for an unadmitted job (`:3246-3249`). | PASS |
| Cleanup-only lease remains exclusive | Cleanup must first own the sole semaphore permit, then `acquire_cleanup` accepts only `CleanupPending { instance: Some(_) }` and leaves public state `CleanupPending { instance: None }` during fallible `cancel-job` (`:2837-2855`, `:3068-3082`). Both pending forms preflight/reject mounted routes. | PASS |
| Only success restores `Available`; failure/panic/consumed admission is typed quarantine | `Ok(true)` alone calls `cleanup_resolved`; ordinary error or `Ok(false)` calls `quarantine` (`:2954-2971`, `:3076-3080`). Cleanup lease defaults to quarantine on unwind (`:2843-2846`); its Drop publishes state before the permit releases. | PASS |
| Exactly-one cleanup schedule, guest cancellation admission, and terminal delivery | Schedule and admission use separate AcqRel compare-exchange gates (`:2938-2943`, `:3063-3066`); sender takes its only channel once (`:2744-2757`); job terminal state returns `Yield` thereafter (`:3205-3267`). No fallible production `GuestRuntime::cancel_job` call is discarded in relay or shard paths. | PASS (static) |
| No worker parking/lost normal wake in retained relay | One finite poll runs per closure; pending clears `scheduled` then re-schedules only a recorded wake; complete/panic terminates the retained task (`:2635-2687`). The pending barrier awaits through this retained future, so it releases the worker rather than blocking it. | PASS (static; runtime unrun) |
| Caller cancellation is not replaced by generic driver cancellation | `GuestColdRelayJob` owns/observes its own `cancel` (`host:3129-3185`, `:3205-3220`); `run_job_on_worker` constructs a distinct relay token and separate `BatchJobParams.cancel` (`:3329-3347`). `WorkerJobSession` passes only the latter to `drive_step` (`job framework:752-803`). | PASS (static) |
| Hot-shard cancellation remains safe | `Effect::CancelJob` removes records only after `Ok(())`; error unregisters/retire actor and publishes typed fault (`shard:606-627`). Actor cancellation enumerates the `BTreeSet` in stable order, retains first fault, and always unregisters (`:729-758`). | PASS (static) |
| Production ActionBus/freshness and fixed bridge remain intact | Procedural registration calls the Assembly factory on `ActionBus::production` (`procedural:139-147`); bridge uses exact key/schema and bounded worker session (`infer bridge:171-223`); host removes live authority and validates it immediately before result exposure (`host:4972-4993`). Lossless checkpoint/commit channel is a fixed two-slot array with checked aggregate bytes (`infer bridge:69-127`). | PASS (static) |
| Commit bounds | Commit admission uses checked item/byte arithmetic, exact reservations for both output and side assignment vector, and pre-append item/byte checks (`wfc job:352-373`, `:1050-1081`). | PASS (static) |

## Deterministic Barrier Challenge

The three new tests do position the hold at the required interval rather than merely waiting for
final cleanup:

1. Ordinary failure: `run_guest_relay_request` drops `guest`, drops `permit`, then awaits the
   mock release barrier before scheduling cleanup or sending its receiver completion
   (`host:3043-3051`). The first two concurrent-route tests wait for that barrier, assert
   `CleanupPending` and zero premature cancel admission, start a second mounted route, and assert
   unchanged guest admission counts (`:3904-3960`).
2. Retained panic: the panic catcher drops the retained request future first (`:2660-2670`), which
   drops the default cleanup-pending lease before the permit. The test-only panic handler then
   awaits the same barrier before calling recovery/scheduling (`:3096-3117`). The third route test
   asserts pending/rejection before release and successful reuse only after cleanup success
   (`:3963-3986`).
3. The barrier wait is asynchronous (`MockJobStepGate::wait`) inside `GuestRelayPoolFuture`, and
   `wait_for_release_barrier` observes `is_waiting()` before launching the second route
   (`:3580-3609`). It therefore does not hold a worker while manufacturing the concurrent route.

The tests are structurally adequate for the P7f race; they were inspected only and not executed.

## Explicitly Unrun Native, Release, Wasm, And Runtime Gates

No executable pass is claimed for this current tree. Still unrun:

- compile plus relay, hot-shard, Assembly/WFC, and inference-bridge tests in debug and release,
  including the three cleanup-pending barriers, fallible cancel, retained-panic, poison,
  receiver-close, terminal-race, lost/double-wake, and one-worker progress cases;
- a repaired valid exact-maximum checkpoint serialization/restore test and its one-byte-over case,
  plus allocation-pressure and p99/max watchdog measurements;
- public-factory replay at 1/2/4/default worker counts and mounted `semio.infer`
  freshness/document-close integration;
- procedural native development, strict `-D warnings`, and release gates;
- `wasm32-unknown-unknown` and `wasm32-wasip2` build/runtime gates.

Historical command results in earlier P7 notes were neither run nor accepted as evidence for this
audit.

## 2026-08-22 Checkpoint Boundary Repair Disposition

The isolated P0 is repaired in current source and is ready for independent re-audit. No cold-relay
cleanup ownership or hot-shard cancellation source was changed.

Production now declares the checkpoint fixed header from its magic size and grouped count of 20
`u64` fields. Encoding materializes a compiler-checked array of exactly that length. The shared
`CheckpointCounts::checked_bytes` calculation derives domain-word, trail-entry, decision-entry, and
observation-entry bytes with checked arithmetic and is used by both `CheckpointBuild` capacity
admission/reservation and `WfcRestore` exact-length validation. Header and entry append guards use
the same derived constants. The former independent 168/176 literals are gone.

The maximum fixture no longer truncates or rewrites checkpoint offsets. It prepares valid typed
state, derives the observation count from `MAX_CHECKPOINT_BYTES` minus the shared base count, and
drives production `begin_checkpoint` plus incremental `checkpoint_one`. It asserts an exact
`MAX_CHECKPOINT_BYTES` result, then the existing phased restore regression admits, decodes,
verifies, rebuilds, completes, and takes the restored job. The maximum allocation regression uses
the same state helper and rejects one more observation before reservation. Restore admission uses a
checked `MAX_CHECKPOINT_BYTES + 1` boundary and rejects before constructing restore state. Added
source regressions also require a valid zero-domain checkpoint to contain exactly the fixed header
and restore successfully, and require both direct checked-count overflow and malformed-header
overflow to return the typed capacity fault.

Source/static disposition gates on the repaired tree: WFC `rustfmt --check` exit 0; interactivity
verification exit 0 with 775/775 bounded rows and no failure; scoped diff hygiene exit 0; zero
standalone 168/176 checkpoint literals; and zero WFC debug/private-executor/unsafe-retention scan
hits. Cargo, debug/release tests, runtime timing/allocation tests, native procedural gates, and both
Wasm targets were not run by instruction. No executable pass is claimed.
