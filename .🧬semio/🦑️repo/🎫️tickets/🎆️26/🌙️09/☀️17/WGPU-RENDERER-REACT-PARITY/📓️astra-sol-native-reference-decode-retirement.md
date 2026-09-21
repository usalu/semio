# Native Reference Decode Retirement

## Observed owner boundary

The native renderer keeps one `Arc<NativeReferenceDecodeJob>` in `RuntimeMailboxInner::native_reference_decode`. The job owns three potentially large payloads:

- encoded input in `Mutex<Option<Vec<u8>>>`;
- a decoded `DecodedReferenceImage`, whose `pixels` are a `Vec<u8>` bounded by the renderer's 64 MiB reference-image authority;
- an output that can itself be `Waiting(DecodedReferenceImage)` or `Ready(SceneRasterLease)`.

Current `NativeReferenceDecodeJob::cancel` performs `take()` on input and decoded storage when the worker has not started. Both `close_renderer_asset_step` and the component close path clear the mailbox slot as soon as phase 2 is observed. `pump_native_reference_decode` also clears the slot when the World owner disappeared. These paths can therefore destroy the input, a waiting decoded pixel vector, or a ready raster lease in one caller turn.

The worker-running case has a separate ownership boundary: phase 1 means the maintenance job may hold local encoded or decoded values. Close must set cancellation and wait for the worker's phase-2 handback; it cannot steal or drop that local owner.

## Fail-first laws

The bounded packet should replace the old synchronous-cancellation assertion with two direct production-owner laws in `wgpu-renderer-async-boundary`:

1. `native_reference_decode_cancellation_keeps_encoded_and_decoded_owners_until_bounded_close` creates a real job with encoded input larger than two `JOB_PAYLOAD_PAGE_BYTES` pages and an actual `DecodedReferenceImage` whose pixels are also larger than two pages. Cancellation must leave both owners present for an explicit close lane.
2. `native_reference_decode_phase_two_waiting_output_stays_in_the_mailbox_after_one_close_turn` places a real decoded image in `NativeReferenceDecodeOutput::Waiting`, advances the job to phase 2, installs it in the actual `RuntimeMailbox`, and calls the production whole-runtime close step once. The mailbox/job owner must remain live because its pixels exceed one page.

The existing cancellation law still checks the exact World request owner is returned and the worker uses only the bounded maintenance lane. A mailbox-level assertion should keep the `native_reference_decode` slot occupied before terminal close, proving that phase 2 is not equivalent to retirement completion.

Proposed focused filter:

```text
test(native_reference_decode_cancellation_keeps_encoded_and_decoded_owners_until_bounded_close) | test(native_reference_decode_phase_two_waiting_output_stays_in_the_mailbox_after_one_close_turn)
```

## Narrow production seam

Use a closed result:

```rust
enum NativeReferenceDecodeCloseStep {
    Busy,
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}
```

`NativeReferenceDecodeJob::begin_close` only sets the cancellation flag and changes an idle or handed-back phase into a private closing phase. It leaves encoded and decoded buffers owned by the job. A running worker remains phase 1 until it publishes its exact phase-2 handback.

`NativeReferenceDecodeJob::close_step(maximum_items, maximum_bytes)` owns the bounded retirement order:

1. wait while the worker is phase 1;
2. take a phase-2 output into retirement ownership without dropping it;
3. release a `Ready` lease as one item, discard `Failed` as one item, or move `Waiting` into the decoded retirement field;
4. truncate at most `maximum_bytes` from decoded pixels;
5. remove empty decoded metadata as one item;
6. truncate at most `maximum_bytes` from encoded input;
7. remove the empty encoded allocation as one item and report terminal.

The caller grants `maximum_items = 1` and `maximum_bytes = JOB_PAYLOAD_PAGE_BYTES`, so each turn releases at most one item or one page. A zero grant remains pending. No loop belongs in cancellation, pump, full close, or component close.

`RuntimeMailbox` then needs one exact helper that accepts either the expected `WorldAssetRequestToken` for component close or the currently admitted token for whole-runtime close. A token mismatch is already terminal for that caller and must not affect a sibling. The helper begins close, advances one page, and clears the mailbox slot only on `Complete`.

`pump_native_reference_decode` must use the same seam when cancellation is observed or the World state no longer exists. In particular, a phase-2 `Waiting` value cannot be rearmed after cancellation and cannot fall out of a local `output` binding. Normal publication keeps the existing `Ready`/`Waiting` behavior.

Root owns wiring the helper into full and component surface close. This agent's production boundary remains `NativeReferenceDecodeJob`, the exact mailbox helper, and `pump_native_reference_decode`.

## Status

Native 121 executed both fail-first laws. The phase-zero law observed `None` instead of the retained 32,785-byte encoded owner. The phase-two law observed that one whole-runtime close turn cleared a waiting output whose pixels exceeded two pages. The run executed 58 tests: 55 passed and these two laws plus an unrelated Map fixture failed.

Production now has one closed `Busy`/`Pending`/`Complete` close result. Cancellation fences publication without taking buffers. A running worker must return its owner; a checked-out output cannot be stolen; `Waiting` transfers to the decoded retirement field; a ready raster lease releases as one item; decoded pixels and encoded input truncate by at most one 16 KiB page per turn. The exact mailbox slot is cleared only after terminal completion. Pump cancellation, missing-World cleanup, component close, and whole-runtime close all call the same seam.

The language-neutral native-asset fixture is schema v2 and pins the page releases `[16384, 16384, 32]` followed by `[16384, 16384, 17]`, exact byte total 65,585, and nine phase-two mailbox turns. Its Ajv and independent TypeScript oracle passed through the canonical browser-worker target: 11 files and 152 tests passed in 5.86 seconds, Nx 11.4 seconds. Receipt: `🗑️generated/astra-runtime/native-reference-oracle/run.log`.

The production and native law sources both parse with `rustfmt --edition 2021`. Native 122 executed 60 tests: 59 passed and the remaining failure was the independently owned Map fixture. Both phase-zero and phase-two NativeReference retirement laws passed, including the exact nine-turn mailbox-retention sequence. Receipt: `🗑️generated/astra-runtime/renderer-native122-handoff-reference-green/run.log`.

## Phase-one worker handback

Native 122 proves that owners already resident in the job retire by one bounded page per turn. A running worker still takes both fields into closure locals. The current closure drops the encoded input on every return and drops decoded pixels when cancellation wins, raster admission is refused, or moved-raster preparation returns the pixels with an error.

`native_reference_decode_running_worker_returns_its_encoded_and_decoded_owners_before_bounded_close` uses the actual maintenance worker with a test-only barrier after it checks out both owners. It cancels while phase one is live, proves close reports `Busy`, then releases the worker and requires the exact encoded and decoded sizes to be resident in the job before bounded close. The production handback remains held until this law records the expected failure.

The narrow repair adds one private pre-publication retirement phase. Every worker return restores encoded input to the job. Cancellation and raster refusal restore decoded pixels; moved-raster preparation reconstructs the decoded owner from returned pixels on error; pool reuse retains the unused decoded pixels; backpressure keeps the decoded owner in the existing `Waiting` output. Successful moved preparation transfers pixels into the raster lease and retains no duplicate decoded allocation.

The pre-publication phase advances the same one-page payload retirement helper used by close. Only after decoded/input payloads are terminal does it publish phase two, so successful decode cannot synchronously drop encoded bytes. Cancellation can atomically redirect either phase one's eventual handback or the pre-publication phase into bounded close. A cancelled exact World request closes without `mark_world3d_asset_miss`; only a still-live request whose URL became stale may publish a miss.

Focused filter:

```text
test(native_reference_decode_running_worker_returns_its_encoded_and_decoded_owners_before_bounded_close)
```

Native 125 recorded the intended phase-one failure: the real worker returned with the encoded owner absent instead of retaining all 32,785 bytes. The same run recorded the separate stalled-body transport RED and executed 64 selected tests, with 59 passing and five expected/independently owned failures. Receipt: `🗑️generated/astra-runtime/renderer-native125-retained-owner-red/run.log`.

The phase-one production repair is now applied. Non-cancelled workers enter private phase six, which advances exactly one existing payload-retirement grant per decode turn before phase-two publication. Cancel redirects phase six into phase-three close. Exact cancellation does not publish a missing-image result into the retiring World. The prepared and applied patch is retained at `🧩️native-reference-phase1.patch`.

Native 126 executed 68 selected ownership laws. The phase-one worker law passed with the exact 32,785-byte encoded and decoded owners handed back before bounded retirement. The run recorded 67 passes; its only failure was the independently owned Map cleanup fixture. Receipt: `🗑️generated/astra-runtime/renderer-native126-integrated-owner-green/run.log`.
