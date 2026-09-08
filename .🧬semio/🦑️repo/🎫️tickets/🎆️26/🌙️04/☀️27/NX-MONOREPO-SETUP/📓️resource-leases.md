# Resource Leases

## Design and Verification Plan — 2026-09-08

Existing artifact publication uses an exclusive-create PID file. A crash can leave it behind, and readers/Nx cache restoration do not participate. Session records reject stale UUIDs but their read/check/write sequence has no cross-process transaction. These are remaining concurrency gaps, not completed safeguards.

The proposed primitive holds a rollback-mode SQLite transaction for a resource’s lifetime: shared reads coexist; an exclusive transaction excludes other readers and writers. The operating system releases file locks on process exit. Each resource uses a stable hash-named database, outside cached deliverables, so invocation IDs do not create an unbounded namespace. No heartbeat or age-based lock stealing is involved. The [SQLite locking specification](https://www.sqlite.org/lockingv3.html) describes the kernel-backed shared/exclusive behavior and Unix/Windows backends.

Bun’s [built-in SQLite interface](https://bun.sh/docs/runtime/sqlite) and Node’s [built-in SQLite interface](https://nodejs.org/api/sqlite.html) will sit behind a repository-owned interface. No new package dependency is needed. Native Python SQLite will act as an independent process oracle; language-neutral cases will cover sharing, exclusivity, unrelated resources, waiting cancellation, process death and repeat-use storage.

Required boundaries: the database must stay in rollback mode; an acquired lease is released after the protected operation stops, never merely when cancellation is requested. Waiting must be cancellable and report progress. Multiple resources must be acquired in stable order. The implementation must reject symlinked database/store paths and foreign database identities. Native Nx cache restoration and every relevant producer/consumer still need explicit integration before this primitive can claim to protect the whole monorepo. Network filesystems and platform-specific qualification remain separate gates.

## Implemented Primitive and First Integration

The missing-module red test was observed after correcting the probe’s fixture-root typo. The complete primitive suite now passes ten native-process cases across Bun/Node/Python: shared readers, writer exclusion, unrelated resources, waiter cancellation, Python-owner death, Bun-owner death and Node cold initialization. Twenty repetitions retained exactly three constant-size databases. Additional checks passed for deadlines, cancellation after acquisition (the lock stays held until release), throwing progress callbacks, stable multi-resource ordering, throwing consumers, foreign/mismatched databases, rejection of WAL, hard links and directory links. Runtime SQLite versions observed: Bun 3.51.0; Node 3.53.4. No package dependency was added.

Service-session open, readiness publication and close now take the common exclusive transaction. A deterministic red test demonstrated that each operation previously ignored a held lease. All three mutations now wait, preserve the old record while waiting, and perform their change after release. APIs and their Demonstrator/native-Nx fixture callers are asynchronous; Vite awaits its readiness publication callback. The native two-invocation Nx + real Vite cancellation scenario passed again.

A separate delayed-response test reproduced stale readiness acceptance: an old HTTP response arrived after a replacement session was committed. Readiness now rechecks the prepared session immediately before returning the URL; the green check is running.

The generic and integrated tests are registered in `repo:test`. The earlier daemon-backed full repo/registry attempt was stopped through its own root wrapper after over fifteen minutes without task execution; continuous concurrent source rewrites kept invalidating the shared graph. Its wrapper exited 143 and the shared daemon was preserved. The next full run explicitly uses `NX_DAEMON=false`. This is a diagnostic choice for this qualification run, not a repository-wide daemon-disable policy.

Remaining integration includes artifact publication/restoration, installation leases, long-lived artifact consumers and cleanup. Lock databases must not be removed or replaced while participants can use them. The primitive coordinates cooperating processes on a local filesystem; concurrent hostile path replacement and network-filesystem semantics have not been qualified.

## Ticket Storage Reclamation

At 12 GiB free, the current Demonstrator attempt was stopped through its own root wrapper after Flow artifact compilation had already failed on three ordered-value imports. No unrelated process or shared Cargo store was stopped or deleted. The completed private `scale-native-consumer/cargo-target` store accounted for 2.015 GiB allocated. Its retained 06:01 receipt reports 6 passed exact laws for `semio-framework-plugin-host`, executable SHA-256 `d9d5022223ee42cf384520ded186569ddd81263e0df5641fe5846da18a641c99`. A recursive lsof check reported no open handles; no lease/active marker remained. Only that compiled store was reclaimed; its receipts/law outputs were retained for the ticket.

The delayed-response green test passed. The complete primitive suite was rerun with an explicit stdin cancellation command for platform-neutral waiter cancellation and fallback discovery of `python3`/`python`; all ten process cases and additional lifecycle/path checks passed again. Actual Windows/Linux execution is still pending.

## Integrated Test Checkpoint — 2026-09-08 18:30

The full repo/registry invocation exited 1. `repo:test` reached the native continuous-service tests and exposed an inconsistent harness deadline: the first consumer gate expired at 15 seconds although the parent could wait longer for the second Nx startup. The language-neutral fixture now owns coordinated wait/gate/service/health deadlines (60/120/180/5 seconds), with their ordering validated. All four native modes passed with that correction: independent owners, cancellation, default sharing and real Vite cancellation. The full run still needs repetition.

The registry failure was separate: Vitest reported “No test files found” because its config had no include pattern for taxonomy-named files. Its owning config now explicitly includes the direct test-category `🟦️.ts` leaves; verification is pending.

The focused Flow release retry was stopped through its own wrapper before target execution when available space fell below 5 GiB. It exited 143. At the next checkpoint only 3.2 GiB remained available, so further native compiler jobs remain paused. The already-running Print document verification is still progressing. No shared compiler cache or another worker’s files were deleted.

## Host Restart Checkpoint — 2026-09-08 19:31

The previous tool session handles are missing. The OS reports boot time 18:31:22, after the preceding test launch; a process inspection finds neither the Print probe/TeX descendants nor the repo/registry invocation. Their logs contain no terminal result. They are interrupted, not failed or passed. Logs were retained with a before-reboot suffix. Available disk headroom is now 69 GiB; the reason for the space change was not determined. Print collection verification and the repo/registry checks are being restarted from the current sources and surviving task caches.
