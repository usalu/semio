# Input Metrics Concrete Receiver Join

This is the next live cutover plan, not mounted behavior. It supplements the private writer/root primitive; no numeric tag or physical work check is resident funding.

## Actual Receiver and Caller Boundary

- UI-host window::WindowDelegate::handle_event/handle_metrics currently consume/return void. NativeHost::normalize clones IME and allocates key strings before delegate entry. CanvasHost::step takes its structural pending_dispatch/latest_metrics/latest_pointer before the void callback and then ACKs. Clipboard acceptance makes multiple text clones. Each must instead keep its original source installed and pass a narrow borrowed ingress facade; no mutable source/root escape and no ACK on refusal or CommittedFault.
- WGPU winit_app::enqueue_host_metrics currently mutates frame generation, scheduler and EventQueue. OsHost::handle_metrics then always calls surface_resize.enqueue and runtime.enqueue_apply. All authored callers must move to one typed result; no void compatibility delegate remains.
- SurfaceResizeLane has exclusive &mut authority, not a mutex. Current enqueue immediately calls session.begin_close before replacing its scalar pending request. The accepted fixed commit must install only the checked scalar request and a deferred-close marker. The existing session stays structurally installed; its actual begin_close runs in the subsequent bounded lane step, never between publication and terminal Watchdog timing.
- RuntimeMailboxInner::completions is a real mutex. Its current enqueue uses blocking lock, allocates completion revision with unchecked fetch_add, may remove/drop a generic RuntimeApply when full, then increments presentation scene revision and calls an arbitrary waker under another blocking mutex. These calls cannot be reused inside the affine fixed commit.

## Required Private Prepared State

The WGPU private commit wrapper owns the same measured UI-host turn (one real Watchdog), mutable scheduler/frame-generation references, exclusive surface-lane borrow, and fresh actual mailbox receiver guard. Across yielded copy turns it retains only exact reservation identities, never MutexGuards. Preparation must preflight occupied/full/poison/closing states, all checked successor counters and the actual complete header move cost before admission_checkpoint.

Runtime mailbox reservation must identify a real fixed destination and prevent concurrent producers from consuming it. It must not remove an existing generic completion to make room. A full ready queue refuses the entire metrics commit and preserves source, generations, active surface child and all receiver contents. Superseded completions can only move into a preadmitted retained retirement owner and later close bytewise; they cannot be silently dropped.

The existing mailbox next_revision and presentation scene_revision are independently updated atomics. Two separately successful CAS operations are not an atomic all-receiver reservation: the second can fail after the first changed visible state. The live patch must either put their mutation authority under the same actual short-lived receiver guard, with every authored producer joined, or supply an already-existing exact composite reservation. Merely reading both and later storing over concurrent writers is invalid. No such composite authority is currently exposed. This is a concrete remaining source join, not solved by WatchdogAdmission.

The post-admission fixed commit cannot call the existing enqueue, enqueue_apply, make_room_for, mark_scene_changed or waker path. Wake intent must remain a fixed retained scalar request for a later bounded scheduler-owned action; a generic user callback after the terminal clock is not part of a proven callback. The existing scheduler invalidate operation is a fixed bitset write and can be included in the guarded fixed commit.

## Required Actual Laws

Future native selectors must exercise actual source receivers (not a mocked verdict or receiver model): metrics event-full, mailbox-full, input/frame/lane/mailbox generation exhaustion; held actual completions mutex and poisoned mutex; active SurfaceResizeSession pointer/identity preservation; equal counters in different queue roots and stale candidate identity; zero and short physical backing/descriptor grant; cancellation and partial-copy unwind; actual clock7999/8000/8001; prior successful same-operation verdict followed by8000; terminal overrun after fixed publication yielding CommittedFault without retry/rollback.

Original admission5 remain decisive queue regressions. New primitive root5 and writer4 cannot substitute for any receiver law. Exact live metadata sizes, all simultaneous original/candidate/output backing, funded parent ownership and actual publication tail are required before claiming the cutover complete. No WGPU production source has changed in this packet.

