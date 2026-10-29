# Output Observer Service and Immutable SDK Review

Read-only in-progress review, 2026-10-29. No Source edits, compiler, tests, graph or native watcher launches by reviewer. Bootstrap has not frozen this bridge.

Authority: exported output observer factory uses the acquired service birth.workspaceRoot and retained usePhase control, without accepting a caller replacement Original. Marker factory separately confines its absolute marker path below authenticated birth.directory, then observes only its containing directory; this permits selected workspace roots whose namespace is outside that workspace. Factory is private to active ownedDaemon and requires server-open/non-draining state. Runtime barrel explicitly exports the two service factories through the package facade.

Bridge ordering observed in the checked-in immutable patch: raw Source events enter receiveSource before semioExcluded/native ignored admission; zero-source-event flush still flushes output and marker channels; RECORD_OUTPUTS_HASH_BATCH awaits scope registration then flush before native expansion/baseline; warm match awaits the same flush. Handler and d.ts signatures now await/declare Promise results. The previous whole-workspace output Watcher is removed in favor of exact scoped recursive adapters and marker membership.

Concrete findings sent directly to owner:

1. Recursive bridge adapter allocates Native Watcher then calls watch without startup try/catch rollback. If watch throws, no handle reaches observer core, so it cannot close that allocation. The existing actual kernel adapter already demonstrates cleanup on startup failure. Preserve original and stop errors.
2. Native callback calls project(events) outside a protective try/catch. Projection errors must reach receive(error,[]) and fail the channel closed, rather than escape native callback execution. Validate root-relative projection containment as the kernel adapter does.
3. Service resourcesStopped catches observer.stop failures, then marks stopped/finishes and awaits root release before rethrowing. Observer core retains records whose stop failed, so actual resource settlement is uncertain. Preserve physical custody until confirmed closure, or use existing bounded fatal service termination policy; aggregating only after release is insufficient proof of closed subscriptions.

Bootstrap independently identified the old two-second output timestamp immunity and is adding a causal invalidation regression/fix. This report does not accept that unchanged behavior. Final integration/closure proofs and frozen-source review remain pending.


## Corrected Intermediate Checkpoint

Re-read actual Source after owner’s correction: native startup now catches watch failure, awaits stop and aggregates independent cleanup errors; callback projection is guarded and routes failures to receive(error,[]). Service resourcesStopped now throws observer-stop failures before stopped=true/finish/released, preserving root custody while settlement is unconfirmed. These specific findings are corrected in current Source. Owner is adding bridge regressions; frozen integration verdict remains pending.
