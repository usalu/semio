# Terra Host EventQueue Admission Audit

**Scope:** read-only review of the host `EventQueue` repair and Winit callback admission boundary. No tests or builds were run.

## EventQueue

`📥️enqueue/🦀️.rs` now establishes the intended queue ownership boundary.

- `EventQueue::new` leaves `VecDeque` at zero capacity ([`📥️enqueue/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️enqueue/🦀️.rs:255)).
- `enqueue` and `enqueue_metrics` compute a candidate generation with `checked_next` before changing queue state, and assign `generation` only after accepted coalescing or discrete insertion ([`📥️enqueue/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️enqueue/🦀️.rs:279)).
- A discrete event first passes logical byte/count limits, then fallibly reserves the bounded initial allocation. A reserve failure produces saturated overflow before `push_back` ([`📥️enqueue/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️enqueue/🦀️.rs:317)).
- `close_step` retires one pending item per call and releases empty `VecDeque` backing in its own later step; terminal requires both no event and zero capacity ([`📥️enqueue/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️enqueue/🦀️.rs:361)).

The matching neutral fixture requires zero initial backing, a minimum bounded allocation, refusal without generation advance, and separate physical release ([`🔣️.json`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧫️fixtures/🔣️.json:14)). The focused admission tests cover full queue and maximum-generation refusal plus retirement ([`🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🎟️admission/🦀️.rs:17)).

**Finding — resolved at the queue boundary, high confidence.** I found no generation, allocation, capacity, or close-step regression in the current `EventQueue` implementation.

## Winit callback boundary

**Finding — repair required in both callback helpers, high confidence.** The Winit helper changes renderer work state before it asks `EventQueue` whether an input event is admitted:

- `enqueue_host_event` advances `frame_generation`, invalidates the scheduler, then calls `events.enqueue` ([`🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:69)). A full queue therefore returns `Overflow` after a frame-generation advance and render invalidation.
- `enqueue_host_metrics` has the same pre-admission ordering and discards the `enqueue_metrics` outcome ([`🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:80)). Metrics refusal has the same regression but lacks an observable helper result.

The new Winit law exercises the actual helper, fills the queue, and requires `Overflow`, no frame-generation delta, and no render scheduling ([`🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-winit-app-callback-latency/🦀️.rs:46)). It correctly catches the event path. It is expected to be red against the current callback order.

The minimal ownership order is: preflight a nonmutating next frame generation when the hold is free; admit to `EventQueue`; and only on `Accepted`, commit the preflight generation and invalidate the scheduler. A frame-generation preflight failure must leave the input queue untouched. The same helper should be used by metrics, with a returned `EnqueueOutcome` so its refusal can be observed.

`OsHost::handle_metrics` currently still performs the physical `surface_resize` and runtime `Resize` after the ignored queue outcome ([`🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:121)). Whether platform resize must continue on queue refusal is an independent policy decision; it must not cause renderer input generation or scheduler admission to commit.

## Required law coverage

1. Keep the new full-event rejection law.
2. Add the equivalent metrics-full rejection law: the helper reports `Overflow`, the EventQueue generation and renderer frame generation stay unchanged, and no scheduler render is requested.
3. Add a frame-generation-exhaustion helper law: an otherwise admissible event is not inserted when the free frame counter cannot advance. This distinguishes callback-level admission from the EventQueue’s existing maximum-generation law.

Confidence is high because these outcomes follow directly from the current mutation order; this audit did not execute the suite.
