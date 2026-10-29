# Linux Setup Memory Failure

Full setup7334 terminated1 after its final native ELF install completed0. The signal sender is proven: host earlyoom PID1587 logged SIGTERM at2026-10-28 21:32:08 to captured Bun PID356472, uid1000, VmRSS4178MiB, oom_score930 and oom_score_adj200. Host memory available was308MiB and swapfree0 of8947MiB. The captured Bun exited5.021seconds later. This is an OS low-memory termination, not an inferred timeout or private Original failure. Sourceoutput and bounded journal evidence are retained temporarily under generated.

The farm summary records ONE worker process,464 adopted projects,1 failed,0 lost and23 restarts after1381seconds. Progress listing eight running projects reflects asynchronous inference loops, not eight OS worker processes. The current schema starts one own-grant worker and requests additional capacity separately. The additional3584MiB exec waited behind workspace:setup and never admitted before the kill. Existing guards remained active.

Diagnosis is now checking total inherited grant accounting for parent and its own-grant worker, the fixed2048MiB JavaScriptCore memory-size hint, and parent-retained memory. No worker override, floor/swap weakening or setup retry has been introduced. Full provisioning remains incomplete; the native install milestone remains valid.


## Actual Reservation and Shared Worker Accounting

The frozen memory correction reads the same-id current ledger reservation, qualified by the existing PID, boot and heartbeat law. Propagated tool-envelope growth cannot enlarge graph memory accounting. The measured 3831 MiB reservation allocates 1915 MiB to the graph parent, 461 MiB to fixed overhead and at most 1455 MiB to its owned worker pool. Tiny reservations never acquire an artificial minimum that exceeds their actual size; an insufficient worker remainder starts no owned worker.

Parent pressure samples the maximum of physical RSS and managed heap plus external bytes. The production governor charges its retained baseline and refreshes the current reservation before every inference admission. Before farm dispatch, a smaller live allocation disables the prior owned pool, drains accepted inference and adoption work, closes the pool and starts only a currently funded replacement. Supplemental host grants retain their independent custody.

The retained-pressure fallback still allows one inference to make progress when irreducible parent memory exceeds its budget. This correction is not a hard RSS cap or a proven resolution of the actual host earlyoom event. A fresh full workspace run remains necessary. No dominant retained object leak was established.

Validation: terminal handle 57271, 37 tests, 1777 assertions, three suites, zero failures. Neutral JSON fixtures are checked with Ajv and independent SQLite arithmetic, and existing Python farm oracle cases now distinguish unauthenticated plans from actual live ledger grants. Tests cover real current grants, same-id mismatches, dead PIDs, stale heartbeats, previous boot, quiet grants, small reservations, explicit budget limits, RSS, per-admission shrink and owned-pool drain ordering. The actual farm fixture retains a positive 2048 MiB worker funded by a current 7168 MiB reservation. The unrelated concurrent closure-publisher race happened to pass this run and remains separately owned by Root.

The public Source target `test-graph-memory-reservation` is registered with its runtime Source entries and runs the existing memory suite, reservation suite and project-farm suite under caller-owned test artifacts. The direct bounded suite log is `🗑️generated/memory-reservation-final.log`. Root owns the next full workspace retry; no duplicate setup was launched.
