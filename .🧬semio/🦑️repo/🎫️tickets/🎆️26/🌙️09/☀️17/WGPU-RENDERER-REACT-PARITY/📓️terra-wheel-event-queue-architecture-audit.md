# Wheel Event Queue Architecture Audit

## Scope and evidence limit

Read-only source audit. No build, native test, browser action, or server command was run for this report. Native86 is external execution evidence supplied by the ticket owner: an opposite-sign pair produced no settled action and two physical wheel events produced one factor. The owner also reports a possible first-render camera-seed defect in the same-sign fixture; this report does not use that fixture baseline to infer a camera formula.

## Confirmed loss before the application handler

The browser and native host sources initially construct one `DispatchEvent::Scroll` for each physical wheel callback, including pointer coordinates, both deltas, and the then-current modifier snapshot:

- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📡️event/🦀️.rs:118-124` decodes a browser wheel event into one `DispatchEvent::Scroll`; its web wheel normalization is at `:410-420`.
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs:491-495` does the same for each `WindowEvent::MouseWheel` after native line-unit normalization at `:403-407`.

That fidelity is lost by the generic host queue, before any WGPU `AppWheel` work:

- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️enqueue/🦀️.rs:131-141` defines `ScrollSample` with position, both deltas, modifiers, and generation.
- `:152-200`, especially `coalesce_scroll` at `:173-186`, stores only one sample. A later wheel adds its deltas but replaces the coordinates, modifiers, and generation.
- `EventQueue::enqueue` at `:276-302` sends every `DispatchEvent::Scroll` to that coalescer at `:292-294` rather than the bounded discrete queue.
- `drain_page` at `:334-347` emits the lone coalesced scroll before discrete events. That changes inter-event order as well as event count: a later scroll can precede an earlier queued key, press, or other discrete event.

`DISCRETE_EVENT_BYTE_CAPACITY` is 4 KiB and `event_owned_bytes` charges scroll zero bytes (`enqueue/🦀️.rs:212` onward). Scroll can therefore occupy the existing bounded event-count queue without increasing an owned-byte grant. Its finite count remains subject to the existing page/capacity result; overflow must be explicit rather than represented by summed deltas.

The renderer loses information a second time:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12172-12178` `AppWheelNotch` retains only `delta`, `x`, and `y`.
- `:12180-12183` fixes the batch at eight notches. `AppWheel::accumulate` (`:12213-12231`) merges a stationary or saturated burst; `take` (`:12233-12246`) omits zero sums. It consequently removes the separate factors for same-position wheels and erases an opposite-sign pair.
- `FrameWheelCursor` at `:12742-12748` retains only one `delta`, position, ctrl boolean, and target. The later `WheelStart`/`WheelScene` phases (`:12891-12892`) cannot recover `delta_x`, shift/alt/meta, event order, or original identity.

By contrast, `RuntimeApply::start_dispatch` (`renderer/🦀️.rs:10183-10216`) already checks out an interaction and runs exactly one owned `DispatchEvent` per future; its resume path continues that dispatch cursor (`:10345-10356`). That is the correct bounded serialized execution point, provided the host ingress presents each scroll separately and in original order.

## Required contract

For every admitted physical `WheelEvent`, after host normalization and before any input scheduling:

1. exactly one ordered `DispatchEvent::Scroll { x, y, delta_x, delta_y, modifiers }` exists;
2. its coordinate, both deltas, and full modifier snapshot belong to that same physical event;
3. it is applied once by one `RuntimeApply` dispatch turn, in arrival order relative to every non-replaceable input event;
4. its scene owner is resolved at application time from its own coordinate and the live exact target identity; no old frame cursor or current-global modifier value substitutes for it;
5. capacity refusal is observable as refusal/backpressure. It must not merge, cancel, or silently discard input semantics.

This agrees with the mounted React oracle’s fixed factor per original `WheelEvent`. In particular, `+d` followed by `-d` must reach the handler twice in that order. It is not equivalent to one zero delta, and identical-coordinate events are not one larger factor.

## Minimal production routing

Remove the renderer-owned semantic queue (`AppWheel`, `AppWheelNotch`, `FrameWheelCursor`, and `WheelStart`/`WheelScene`) instead of raising its capacity or teaching it more fields. Route `DispatchEvent::Scroll` directly from the existing normalized-event dispatch into one first-party `AppInteractionState` scroll operation while that event’s checked-out interaction is owned by `RuntimeApply`.

That operation receives the complete immutable event record and does this per event:

1. resolve `ShellState::scene_pointer_target_at(x, y, …)` fresh, which already gives modal/open-panel precedence and returns only a live exact scene target (`🐚️Shell/🦀️.rs:13401-13424`);
2. if present and still `scene_pointer_target_is_published`, call the exact-target scene-wheel effect (`AppInteractionState::apply_scene_wheel`) with the original deltas and modifier snapshot; do not re-use a frame target or scan bounds;
3. otherwise invoke generic `ShellState::handle_pointer_wheel` for retained UI/chrome. Its current `:13466-13500` implementation must accept the event modifier snapshot as an argument: it currently reads `retained_event_modifiers(input)`, which can instead use a later key state. It also currently only receives one delta and scales it by 24, so the direct boundary must preserve both deltas until a named consumer chooses its documented unit conversion.

The invalid-target outcome needs an explicit owner decision. A target that was valid at hit resolution but retired before apply must not be redirected to a newly published scene with the same surface id. The safe result is an inert scene event (or ordinary chrome only when the fresh target lookup says chrome owns that coordinate), never a stale target replay.

## Host queue repair needed before renderer replacement

Deleting `AppWheel` alone leaves the earlier `EventQueue` coalescer and does not meet the contract. `Scroll` must become a non-coalescible event in the bounded ingress queue.

Do not implement that as merely pushing scroll into today’s `discrete` deque while leaving `drain_page`’s “coalesced first, then discrete” shape. The result would retain wrong ordering against coalescible pointer moves/metrics. The bounded clean solution is to assign the existing ingress generation/sequence to every admitted item and merge any replaceable samples into the page by that sequence, while preserving one scroll record per input. A replaceable pointer/metric sample may still discard superseded work, but its retained final sample must appear at its final original sequence position; scroll cannot be a replaceable sample. If existing scheduler constraints make such a merge impossible, put all physical input that must order with scroll into the one bounded FIFO and leave only frame metrics as replaceable side data.

The current source does not establish that this host queue is the only route feeding the WGPU runtime; the two independently confirmed loss points still require both fixes unless direct call-site tracing proves one is bypassed. The implementation owner should trace `EventQueue::drain_page` consumers before deleting only one layer.

## Fail-first laws and existing test route

Extend `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🔬️enqueue-unit/🦀️.rs` (the existing queue oracle) with physical events rather than source strings:

1. enqueue same-point wheels with `ctrl` then `shift`: drain returns two scroll events in that order, each with its own deltas and modifiers;
2. enqueue `+d`, then `-d`: drain returns both, not no event or one zero event;
3. interleave scroll, key/press, and a replaceable pointer move: emitted sequence respects ingress order at the retained pointer-move sequence point;
4. fill the fixed queue: the next physical scroll returns the established overflow/refusal result and does not alter prior deltas, modifiers, or coordinates.

Use the existing WGPU input/camera law that Native86 made red (the owner’s `AppWheel → Shell → camera` test) as the product-level law, but feed its real `DispatchEvent::Scroll`/runtime route rather than directly exercising `AppWheel`. Required cases are two identical-coordinate same-sign wheels (two React factors), opposite signs (two ordered effects), a modifier change between wheels, chrome ownership, modal/panel precedence, and a topmost exact World target. Assert exact target effect/camera state, not an `AppWheel` implementation detail. Retain the React browser oracle for the same fixture input; first establish a published camera seed before using an absolute zoom baseline.

## Confidence and limits

High confidence in the two coalescing defects and the direct dispatcher seam because the cited source shows the data erased. Medium confidence in the exact queue merge form: it depends on the as-yet-unverified caller chain from host `EventQueue::drain_page` into `RuntimeApply`. The next source-only check is that caller chain and the current `apply_scene_wheel` signature, to ensure the direct operation can carry both deltas and full modifier data without another conversion.
