# Compiler Storage Maintenance Readiness

Read-only investigation using the build skill, repository README Recovery section, declared maintenance project, bootstrap recovery schema and actual compiler launch/headroom source. No preview, cleanup, retirement, mount, receipt mutation, graph or compiler command was executed. No peer was interrupted.

## Exact Declared Routes

- `repo run repo-cache-retire:inactive-preview --output=stream`
- `repo run repo-cache-retire:inactive --output=stream`

Owner: repo library/caching/pruning/budget project.json. Preview is cache:false diagnostic and forwards no caller arguments; retirement is side-effecting. Native bootstrap CLI recovery schema explicitly recognizes these exact owner/target pairs before graph admission and dispatches the original issued control into runInactiveCompilerMaintenanceV1. No guessed flag or raw command is needed.

Preview promises no deletion of compiler units, not zero filesystem effects. Its script opens the current rust partition, obtains the actual compiler reservation, queues exclusive compiler writer custody with wait:true, creates a diagnostic artifact directory and writes plan.json. Returned JSON names plannedUnits,plannedLogicalBytes,unmetBytes,planPath and freshly observed availableBytes. Completed deletion units/logicalBytes remain zero in preview. Logical reclaim estimates and physical free bytes are separate evidence.

There is no genuine current-store wait:false preview target or flag in this declaration. forwardAllArgs:false and closed operation parsing do not provide such an option. The internal historical-preview path requires a different exact historical owner/explicit artifact directory and is not a nonblocking current-store workaround. Starting the declared current preview now can queue exclusive writer custody behind active shared compilers and delay subsequent shared entrants; do not launch it casually during this cohort.

## Current Wait and Existing Recovery

The exact log message originates in caching/cargo/unit-cache/gc/headroom, not the simpler fixed compiler reservation headroom gate. In repository process/cargo/launcher, withAgentLaneV1 line98 awaits admitImageHeadroomV1 before pooled lane acquisition line100. Actual compiler-writer shared custody is entered later at195 during admitted compilation. The wait retains its opened compiler partition/reservation lifetime; it has not reached that later compiler-writer shared section.

admitImageHeadroomV1 computes free minus floor, running commands outstanding estimates and earlier waiting reservations. On reason=space it invokes the default collectGarbageSingleFlightV1 with actual shortfall bytes, at the authored reclaim interval, before continuing its wait. This includes reservations-driven shortages, not only raw physical fullness. A pure queue reason waits for predecessors. Collection is single-flight and targets idle incremental sessions/lanes/cold units under their lock protocol, preserving running holders. Once no other command runs or is ahead and collection ran, the defining policy admits an alone command so the existing real out-of-space retry can handle any remaining shortfall. Cancellation withdraws its reservation.

There is also recoverCompactCompilerHeadroomV1 registered by the actual compiler-store lifetime for its lower physical reservation gate; it uses genuine writer custody, bounded inactive retirement and fresh physical rechecks. That mechanism is distinct from the observed learned-command headroom wait. Its presence must not be conflated with evidence of bytes reclaimed in this particular run.

## Safe Next Action

Keep the original command waiting while the active cohort drains. Its own admission loop already attempts the relevant collector without introducing a queued maintenance writer. If authoritative diagnostics are needed after shared activity settles, root can choose the exact declared preview and inspect its plan plus actual availableBytes. This reviewer obtained no current reclaimable-byte inventory and makes no claim that maintenance would free a particular amount. No exported helper that reaps stale ledger rows was invoked as a supposedly pure read. Custody refusal remains an owner diagnostic; it does not authorize raw deletion or cache receipt changes.
