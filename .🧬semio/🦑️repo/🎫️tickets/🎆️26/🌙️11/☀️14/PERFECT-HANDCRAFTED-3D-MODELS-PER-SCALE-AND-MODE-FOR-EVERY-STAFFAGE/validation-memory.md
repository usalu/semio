# Validation memory and process cleanup

The validation host has 16 GiB of RAM and also runs the replica service and work for other chats. Three obsolete staffage validation process trees were stopped. A subsequent process-table check confirmed that their workers had exited. After cleanup the operating system reported normal memory pressure. At the 13:07 CET sample on 21 November, the repository capacity service reported 8,109 MB available and 2,482 MB of swap in use.

Local checks run in isolated process groups. The guard measures the aggregate resident memory of the group on macOS and Linux and uses a job object on Windows. The largest local check has a 1,150 MiB ceiling. Smaller jobs use a lower ceiling and host headroom thresholds. A stopped check can resume from an individual asset, scale, rule or view. On completion and cancellation the guard reaps remaining workers. The regression exercises normal completion, cancellation and concurrent child allocations above the shared memory ceiling. The live replica and other chats' active jobs keep running.

The geometry cache holds up to 32 MiB of retained mesh buffers and keys. A regression streamed over 40 MiB through the cache, retained a repeatedly reused mesh and verified every position and index after rebuilding an evicted mesh. Large files are checked and independently reloaded one at a time. The remote execution status still reported an unconfigured scheduler at 13:03 CET, so these checks used native workers.

During the canopy and cabinet pass, three completed process groups were cleared. Resident-size checks count descendants, cancellation reaps the isolated group, and large tree checks checkpoint at each view or rule. At the 14:13 local review the host reported normal pressure, 6.2 GB available, and 1.57 GB swap in use. A fresh task-owned process scan found active validation workers only.

On 21 November the local validation pool also gained a 3,584 MiB shared reservation ceiling. Each command reserves its full process-tree budget before starting. Host-specific OS file locks release automatically, including when a worker is killed. A language-independent fixture exercised simultaneous 128, 65 and 64 MiB requests in a 192 MiB pool, then verified immediate reuse of a terminated worker's reservation. The separate child-cleanup and aggregate-residency tests passed on macOS. The 14:55 host check reported normal pressure, approximately 6.2 GB available and 1.57 GB swap in use.

The native guard now checks whether its launching process has disappeared while it is running or waiting for memory. A separate process regression killed a scheduler with a live worker. Before the change, the worker outlived the scheduler. Afterward both guard and worker exited and their reservation became available. The normal-exit, explicit-cancellation, concurrent-budget and independent-lock regressions also passed.

At 15:35 CET the host reported normal pressure, 6,719 MB available and 1,565 MB of swap in use. Twelve queued or running checks shared the same 3,584 MiB reservation pool. Checks waiting for a reservation had no geometry worker. A process scan found no orphaned ticket workers.

The local reservation manager also admits a job when enough free 64 MiB slots are separated by other running checks. The fixture held four slots, released the first and third, and admitted a 128 MiB request immediately. The aggregate limit stays at 3,584 MiB. The process regressions also passed for normal completion, cancellation and scheduler death.

The macOS process tests again confirmed scheduler-death cleanup, worker cancellation, descendant cleanup and the aggregate memory ceiling. Windows additionally watches a handle to the launching process and releases its job object when that process exits. The Windows wait-state contract was exercised on macOS. A live Windows run remains unverified on this host.

## November 21 evening process sweep

Three obsolete task worker trees were cleared during the task. A fourth, superseded tower-crane thermal worker was stopped after the bounded replacement was verified. Subsequent process scans at the end of the catalogue run and during the flight-feather study found zero orphaned ticket workers. The local aggregate reservation is 3,584 MiB, with no task permitted an individual reservation over 1,150 MiB. Memory pressure remained normal, including 8,080 MB available while measuring the foliage and crane. Dense checks which reached their own cap were divided by rule and resumed from matching receipts.

At the post-publication sweep, all four structural corrections were installed and all 608 scale checks found zero out-of-body structural members. The host reported normal memory pressure, 7,350 MB available of 16,384 MB, and zero orphaned ticket workers.

The next host process sweep found zero orphaned staffage ticket workers. macOS reported 80% system-wide memory free by its pressure metric. The large active resident-build and replica processes belong to shared infrastructure.

The superseded crane validation run and the sequential whole-catalog export run were stopped before their replacements started. The export replacement uses two disjoint source partitions at 640 MiB per worker and respects the existing 3,584 MiB aggregate admission ceiling. macOS subsequently reported 76% system-wide free by its pressure metric.

At 20:03 CET the process sweep found zero ticket processes adopted by init. The resident build host and replica supervisor were identified by their command lines and active children. macOS reported 72% free on its pressure metric. Both export partitions completed. A city-car document check reached its 540 MiB reservation and terminated with its descendants; its own log and receipt preserve the interruption for a bounded retry.

## November 21 late validation

The repo capacity check reported 6,449 MB available of 16,384 MB, normal pressure and 1,008 MB of swap. The fleet status still had no configured scheduler or worker token. The superseded wheel and foliage validator process trees were retired after checking the program and its actual log descriptor. A subsequent scan found zero orphaned ticket processes. The 120-case volume regression suite now runs in named groups of six in the shared 3,584 MiB admission pool, and the dense mixer check runs one rule group per process.

## Bounded workers and the geometry oracle

At the next check, repo reported 6,848 MB available and normal host pressure. The superseded maple and wheel workers were retired by matching their command and open log file. The process tree had zero adopted ticket workers. The Python trimesh and SciPy oracle now walks its grid in batches of 2,048 points. The same analytical band and area case that exceeded the validation memory budget then passed in 31.55 seconds.

The catalog validation and export sweeps can reserve the first 2,816 MiB of the shared 3,584 MiB local pool. Individual subjects can use the remaining 768 MiB even while both bulk sweeps have work. Reservations use the same host-specific lock. The process audit inspects executable arguments and parent IDs before retiring superseded subject checks.
