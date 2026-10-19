# macOS Shared Bootstrap Stall

Read-only investigation on October 19. No process, image, lease, dependency binding or cache was modified. Samples and image inventory are temporary ticket-generated files.

## Concrete Blocking Resource

Both the native dashboard invocation (PID 99788) and the dashboard-control regression runner (PID 2966) are waiting for shared access to:

`native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d`

Its resource database is `.cache/b/l/m/data/aa5bbea09cfe074fa78d6a26040daaef3d3c8824b1f2f3c6764dbe3f128eb95b.sqlite`. The first live queue ticket is `00mvf5fi9i-0000vyn--GDTeipjAe3rGCe8f0e89w`, declaring PID 41423, exclusive access and no shared cohort. Following live tickets include PIDs 98539, 99788, 99823 and 2966, each requesting shared cohort access.

A normal read-only SQLite read of this database returned `database is locked`. Read-only immutable inspection of the stable resource-identity table confirmed the dependency-image logical key. `lsof` showed only PID 41423 holding the resource database open; all waiters hold their own independent queue claims. This is a live exclusive dependency-image maintenance holder, not a stale queue ticket that may be deleted.

PID 41423 is the existing `bun repo run @teaching/architecture-course:test-e2e -- --serial` build process, running for more than 33 minutes at inspection. Its stdout/stderr point to the separate ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/e2e-1.log`. That log reached `build:dependencies` at line 4879 and subsequently records only repository command progress; it has not reached the course test body. No child process currently belongs to PID 41423. The dependency-binding resource `javascript-dependency-bindings:/Users/ueli/Documents/semio` is also held only by PID 41423.

## Source Path and Samples

The ordinary build entry first opens the lease and diagnostics stores, then calls `ensureBuildEnvironmentV1` before build inference or selected task execution. A missing/stale parser witness causes that owner to publish `build:dependencies` and call the dependency producer. `withJavascriptDependencyStorageV1` acquires queued whole-image dependency maintenance and the exclusive JavaScript binding guard. The producer inspects/publishes workspace ownership and performs fenced dependency Source observation and publication under these guards.

The waiting process sample shows repeated SQLite open, metadata checks and locking calls, consistent with the implemented queue-claim liveness probes. The holder sample instead shows sustained deeply recursive JIT JavaScript activity. It does not identify a JavaScript function name, so the exact producer substep and any possible algorithmic defect remain unproven. There is no evidence of a second owner blocking the holder on an additional lease.

`hdiutil info` confirms the dependencies image is already mounted at `.cache/b/d/m`; its fixed image, lease image and diagnostics image are present. The evidence does not support treating the image as orphaned, detaching it, deleting its cache, removing queue files or bypassing the dependency witness.

## Safe Next Step

The dashboard task may reduce its own redundant waiters and retain one native/test invocation. Unblocking the shared resource requires the live dependency producer to finish, or its owning task to cancel it through its existing invocation and allow cleanup to settle. Restarting dashboard/test waiters alone cannot release PID 41423's exclusive resource. The other task's process and leases must be preserved absent authorization from its owner.

No dashboard-scoped bootstrap repair is justified by the current evidence. A performance investigation of the dependency producer would need to instrument or profile that owner's actual operation, then restart its invocation after the owning task authorizes that work. This report makes no test pass, test failure or runtime readiness claim.

## Later Observation

The original exclusive holder PID 41423 subsequently disappeared. The dashboard-control invocation progressed to its own `build:dependencies` stage (runner log line 5082), then waited for the same dependency image again. The new first live queue ticket is `00mvf6f8p0-000250v-lRJhS-MvHjMUCikDoFdHfw`, PID 99823, exclusive access. That existing process runs `bun repo dependencies lock --all`. `lsof` confirms PID 99823 is now the sole holder of the dependency-image resource database; shared waiters include PIDs 27717, 98539 and 2966. This is a successive live producer, not evidence that the previous lock failed to release. No test body or test failure is present in the dashboard-control log yet.

## Owned Runner Cleanup

The parent requested cancellation of this slice's own lib-test waiter. SIGINT was sent only to PID 2966; session 88398 settled with exit 1 and the canonical `Original Script process interrupted` error. Its dependency-image queue ticket disappeared through normal cancellation cleanup. PID 99823 remained live at more than 25 minutes with its original exclusive ticket. No producer process, producer lease, mounted image or dependency payload was changed by this investigation. The suite never reached its test body, and the ticket remains open for runtime validation after the shared producer settles.
