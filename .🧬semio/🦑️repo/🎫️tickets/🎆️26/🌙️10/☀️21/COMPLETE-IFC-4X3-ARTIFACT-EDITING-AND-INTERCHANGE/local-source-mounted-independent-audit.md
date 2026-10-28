# Mounted Local Source Revision Independent Audit

Read-only local hookup checkpoint2026-10-21. No browser interaction/native claim; remote/backbone/archive/extension coverage remains explicitly in progress.

ShellCore retains stable cache+actual owner ref. document-transfer-ref resolves LocalDocumentOwner from exact session and document runtime/client entry,assigns current ref,and binds cache with exact current plugin; cleanup invalidates only that captured owner/plugin. register observer begins opaque observation before latest queue,passes original signal to readLocalInteraction,rechecks active instance/exact handle/current owner after await,publishes admitted scalar identity,and uses isCurrent after reentrant notification before publishing interaction state. finally.fail only affects still-pending same observation,so successful ready state survives. Superseded queued reads see aborted original signal and cannot publish. Prior callback registration hardening is source-added.

program-dispatch acquires exact source hold before handleAction and releases in finally; normal success waits awaitOperationSettle. Direct browser actor route similarly holds across its actual dispatch command promise and refreshes afterward. Multiple holds are managed by source cache rather than one Boolean. Existing source observer activity is suppressed while held,then a new actual observation begins after release.

## Settlement gap flagged for owner triage

The existing !isCurrentEffectOwner(actionOwner) early return precedes awaitOperationSettle. New finally therefore releases the source hold and schedules observation while a started retained operation may still be active,if effect-owner staleness does not also retire the exact document owner. Skipping stale UI effects is appropriate; skipping operation settlement is not sufficient to justify cache readiness. Await settlement before that return or transfer the hold to exact completion lifetime. Parent was notified to confirm effect-owner semantics/fix. This is a source-path concern,not executed reproduction.

Also ensure exception/watchdog paths do not authorize old identity while a native operation still exists: failed dispatch may release a hold only if actual operation settled/retired,otherwise readiness must remain unresolved. Remote and extension holds need shared owner coverage before any complete proof. New hooks alone do not solve all source-change routes.

No heavy suite rerun. Root's schema/cache tests and local source checks are distinct from mounted browser behavior. Needed integration cases are superseding effect owner during long operation,overlapping holds,aborted latest read,same-instance document replacement,plugin replacement,and close during pending observation.
