# Affine Input Commit Across The Existing Crate Boundary

## Exact Source Constraint

The actual EventQueue is defined in UI-host and exposed through OsHost.events; the actual frame generation, scheduler, surface lane and RuntimeMailbox receivers are WGPU-owned. UI-host currently has no direct trace dependency (only its native job dependency reaches trace). A WGPU-private InputCommitTurn cannot call a UI-host-private commit method. Making an unrestricted public PreparedInput.commit or accepting Copy CallbackVerdict would reopen the authority hole. Conversely, moving the WGPU receiver types into UI-host would invert their current dependency direction and is outside this bounded packet.

## Narrow Proposed Placement

Keep one real Watchdog, not a second timer: a shared UI-host **affine measured input-turn facade** owns the actual trace Watchdog with the fixed UiEvent stage. This requires only a direct internal-framework trace dependency, not an external runtime dependency or ABI change. Its private fields retain the exact queue/candidate borrow. It has no constructor from a verdict, no guard replacement, no DerefMut, no unrestricted mutable owner, and no public queue commit apart from consuming this measured facade.

WGPU's private InputCommitTurn owns that measured facade together with the freshly reacquired actual metrics receiver guards/reservations. Its finish consumes the same measured facade and commits the queue only on that actual watchdog's accepted finish; on acceptance it performs the already-prevalidated fixed scheduler/frame/surface/mailbox writes before releasing any guard. There is no fallible work, user callback, waker, generic payload Drop, reserve, coalescing search, or generation arithmetic between those writes. Superseded owned values remain structurally retained for later granted close. Refusal writes none of the receivers and leaves every original reservation/source in place. The single UI thread plus held mailbox guards supplies the existing visibility boundary; this is not a fake rollback after partial publication.

The simple UI-host event-only delegate can use the same measured facade without WGPU receivers. That is the one canonical API, not a compatibility bypass. WGPU metrics must never call the event-only finalizer independently of its private all-receiver wrapper.

## Required Caller Scope Before Mount

The facade must begin before actual callback admission, backing preparation and source copy; it cannot merely time a final commit after earlier public reserve/write calls already did work. A producer writer is borrowed from that live measured facade, not constructed by an unmeasured queue accessor. Resumed work creates a new facade around the same persistent candidate each turn, retaining the candidate outside the facade and outside unwind. The platform caller must hold one facade for its whole bounded input turn, including receiver try-lock/preflight, and must not perform normalized payload allocation before acquiring its retained candidate.

This tightens the earlier reserve/input_writer/drive sketch: those operations must belong to one measured turn, not accidentally allow several independently timed subcalls to exempt their combined callback duration. Concrete native, CanvasHost and worker caller signatures will be joined together with the old void/consuming APIs removed; no source change is made by this report.

## Review/Gate Scope

Root identity R6 is only missing-API compile RED, and its eventual private queue root helper can remain staged until this complete caller cutover. The already-accepted writer containment and checked-root design remain unchanged. Tests must exercise the real WGPU private wrapper, actual mutex contention/poison, prior successful same-op/gen results versus a later8000us callback, all-three refusal, and no active child cancellation before accepted publication. No native timing or all-three publication proof exists yet.

