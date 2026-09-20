# Frame Action Batch Transfer Audit

Read-only source audit on 2026-09-20. No build, browser activation, or test was run.

## Verified ownership model

The current transfer path has the right normal-pressure shape.

- `BoundedActionQueue::front_batch_len` validates the head countdown across the whole published batch in [action/🦀️.rs](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs#L1016). `InputState::take_action_batch_len_step` exposes that check without removing an action in [input/🦀️.rs](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs#L392).
- `FrameActionOwners::begin_batch` reserves the entire target slice before the first source action is taken; it returns `Deferred` when 255 of 256 frame credits are occupied. The source remains intact. This is implemented in [renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L9609) and exercised by the two-action terminal fixture at [wgpu-renderer-async-boundary/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs#L805).
- Staging one source action per `InputEvents` opportunity is deliberate and bounded. `FrameActionOwners::try_push` rejects unrelated direct frame actions while `batch` is present, so an old single cannot appear between a staged `canvasDragLeave` and `canvasDrop`.
- `AppRuntime.frame_actions`, rather than a discardable `AppFrameTransaction`, owns both published and staged actions. The pre-phase continuation at [renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L12952) resumes a staged batch after a superseded candidate. This preserves the staged owner across candidates.
- A live deferred cursor checks out the interaction state and drains its FIFO before another frame can use that interaction. `start_frame_deferred` takes one `FrameDeferredWork` and schedules its resume in [renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L10188). `FrameDeferredCursor::take_next` dispatches actions before tutorial flush and settle in [renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L9811). This supports the required old-owner-before-new-batch ordering.
- Target cleanup is bounded: `FrameActionOwners::close_step` removes one staged or published descriptor per call and later retires an empty batch reservation. `FrameDeferredCursor::close_step` uses that owner in [renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L9720).

## Required repair: fault must own and retire the source tail

**Confidence: high.** A fault after staging a prefix currently retires only the target prefix, leaving the still-queued source tail able to transfer as a new singleton.

`InputState::record_action_fault` only records `action_fault`; it does not retire `pending_actions` ([input/🦀️.rs](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs#L462)). On the next transfer attempt, `take_action_batch_len_step` returns that fault, and `fail_frame_action_batch` only marks/steps `FrameActionBatchOwner` ([renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L9739)). It has no source-tail owner.

This is reachable in the runtime. A staged transfer returns `Pending` from `InputEvents` ([renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L13045)). Independent retained-surface ingress records action faults on the shared input authority, including Board/TiledMap/NodeGraph/World pointer routes around [renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L15764). The new fail-first law at [wgpu-renderer-async-boundary/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs#L855) isolates the concrete sequence:

1. Source owns `canvasDragLeave, canvasDrop`, followed by independent `later`.
2. The first call stages `canvasDragLeave`.
3. An input fault arrives.
4. The full terminal batch must retire before `later` is dispatchable.

The current transfer implementation cannot satisfy that law because `canvasDrop` remains in `InputState`. It would subsequently be admitted as a one-item batch ahead of `later`.

The repair needs these invariants:

1. When batch staging begins, retain the source batch identity and its remaining source count in the runtime-owned batch owner. A countdown alone is not an identity: a later standalone action also has `batch_remaining == 1`.
2. In failure mode, retire exactly one descriptor per call across both owners: first any staged target descriptors, then one verified source-tail descriptor per call, then the empty reservation. Do not bulk-drain a terminal pair.
3. Before consuming a source-tail item, require the retained batch identity and expected countdown. A mismatch must fault without consuming the next independent action.
4. Surface the original failure only after both the staged prefix and verified source tail are empty. The following independent singleton must remain FIFO-head and become transferable only then.
5. A capacity `Deferred` is not failure: it retains the entire source batch and must not record a frame fault or enter cleanup.

The existing new law covers the fault-after-first-member case. It should be accompanied by a malformed-source-identity case with an appended singleton, proving cleanup cannot mistake that singleton for a one-member source tail.

## Required dispatch/order laws

The repaired packet should retain these independent laws:

- At 255/256, a two-member batch returns `Deferred`, changes neither target length nor source head, then publishes both members consecutively once a slot is freed.
- Given `single-A, [leave, drop], single-B`, `single-A` may fill the last slot; `leave/drop` defer as a whole and `single-B` cannot bypass them.
- A candidate superseded after staging one member resumes the same runtime owner; it does not drop the staged prefix or materialize the tail into the superseding candidate.
- A complete frame must not replace a live deferred owner. The old cursor dispatches all of its actions before the later runtime ledger is installed.
- Cancellation, stale retirement, and fault cleanup remove one descriptor per grant, then one empty reservation; no path clears the entire owner array.

The first two current fixture tests are source-level evidence only; this audit did not run them.



## Follow-up: source-tail repair review

Read after the source-tail repair landed; still read-only and no test was run by this audit.

**Result: the repaired transfer closes the identified orphan path. Confidence: high.**

`FrameActionBatchOwner` now stores `expected`, staged `len`, and `source_remaining` as bounded `u8` values ([renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L9592)). `consume_batch_source` decrements `source_remaining` only after `take_action_step` has taken the corresponding source descriptor. Consequently a target prefix and unmaterialized source tail are accounted separately.

On failure, `retire_failed_batch_step(input)` now retires one staged descriptor per call, then verifies that the current input head's batch countdown equals `source_remaining` and removes one source-tail descriptor per call ([renderer/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs#L9710)). It surfaces the stored fault only after both are empty. The new law at [wgpu-renderer-async-boundary/🦀️.rs](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs#L855) specifically asserts that `canvasDrop` retires before the appended `later` action becomes transferable.

The countdown check is sufficient for the current runtime ownership boundary: outside test helpers, the only production calls that remove input actions are this transfer function's normal and failed-tail paths. Other surface routes reserve/publish actions or record a fault; none can consume the queue head between the staged prefix and failure cleanup. This audit found no introduced ordering, supersession, or bounded-retirement regression in the repaired path.

The capacity path remains nonfatal: `begin_batch` happens before the first source pop and `Deferred` leaves the source batch in `InputState`. A completed staged batch is still published as one contiguous FIFO slice, and `FrameActionOwners::try_push` continues to refuse unrelated direct actions while the batch owner exists.

The Native46 run mentioned by the coordinating agent was still running when this review finished; this audit does not claim it passed.

