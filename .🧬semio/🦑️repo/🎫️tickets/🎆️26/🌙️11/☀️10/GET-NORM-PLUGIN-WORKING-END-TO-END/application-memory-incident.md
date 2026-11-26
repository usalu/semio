# Application-memory incident on the 16 GiB Mac

## What happened

macOS paused Codex while Cargo builds, project graph work, Vite and Chromium overlapped. The host recorded 8,637 MiB compressed, 2,325 MiB available and 103 MiB/s of swap traffic. Swap use reached 7.2 GiB. Chromium was killed or suspended during attempted browser verification. After the affected repository jobs were stopped, the filesystem event daemon alone still had readings above 5 GiB RSS. The observations establish host-wide memory exhaustion. No available diagnostic identifies the single allocation that prompted macOS to pause Codex.

Five macOS Bun diagnostic reports on November 25 recorded excessive disk writes. Two morning reports showed Bun footprints of 3,826.59 MB and 3,664.98 MB. Each report listed the OS action as “none.” These records document additional resource use that day. They do not identify the command or allocation behind the Codex pause.

## What changed

| Reproduced failure | Correction and evidence |
|---|---|
| A first worker or an expired deadline could override critical pressure or active swapping. | [Machine-wide admission](hard-memory-admission-2026-11-25.md) now keeps these stops active. An explicit 6,144 MiB request had actually been offered 1,958 MiB. The [exact-reservation check](codex-exact-capacity-2026-11-25.md) now validates a live grant of at least the requested size before spawning. |
| The warden suspended tasks without necessarily reclaiming their memory during sustained pressure. | [The warden](warden-memory-observation.md) can end eligible repository workloads after its grace period during deep compression and active swap. Its updated enforcement code ran on this Mac. [Cancellation tests](capacity-run-cancellation-2026-11-25.md) verified descendant cleanup before grant release. |
| A fleet graph worker could start with 1 MiB of spare room or after a failed host sample. | [Graph workers](graph-worker-memory-2026-11-25.md) respect the desktop reserve, retry sampling, and drain busy workers during critical pressure. Real child processes exited and resumed when capacity recovered. |
| Native and browser fallback paths could launch children without a sufficient grant. Distributed Cargo could plan locally outside the ledger. | The [browser launcher](codex-guarded-play-diagnostics-2026-11-25.md), [native helper](native-exec-admission-2026-11-25.md) and [fleet Cargo controller](codex-fleet-cargo-orchestration-2026-11-25.md) now account for the local work before it starts. |
| The per-unit compiler broker had its own unsafe first-worker and timeout exceptions. Build-script forecasts charged unrelated source files. | [Compiler admission](codex-native-unit-broker-2026-11-25.md) checks pressure and swap before both exceptions. The [build-script estimate](codex-build-script-memory-2026-11-25.md) for libc now uses its 605-line script and falls from 5,959 to 639 MiB. Unknown closures keep their conservative fallback. |

## What was verified

During the EN 1990 retry the cold compiler broker forecast 5,711 MiB for the native libc library. The actual compiler peaked at 142.27 MiB. The whole-tree estimate remains conservative for an unfamiliar build. Successful native measurements can now inform subsequent matching builds on that host through the compiler fingerprint and package version. Failure, cancellation, remote execution and unknown identities supply no local measurement. [Forecast investigation and controlled tests](native-libc-forecast-2026-11-26.md)

An ordinary one-core native release build verified the measurement path through the updated Rust scheduler and broker. It exited successfully, recorded a 68,091,904-byte peak under its exact compiler-input key and left a durable host-specific history entry with cache replay disabled.

Fleet compiler-output scans previously allocated a buffer the size of each binary. A full scan of a 215 MiB Eurocode binary with the bounded reader increased process peak RSS by 752 KiB. [Measurement and regression tests](compiler-output-scan-memory.md)

The one-worker, locked WebAssembly builds succeeded for the EN 1995 mutation library, the EN 1996 mutation, schema and editor packages, and the EN 1999 diff facade and extracted detail rows. The EN 1995 target built locally. Its highest observed 2.5-second sample was about 1.20 GiB RSS. That lower bound cannot establish compliance with the 1,536 MiB component-unit ceiling. The previous component build reported 2,258 MiB. Seven earlier source-boundary violations were also carved under the existing limits. [Package carves](source-boundary-carve-2026-11-25.md) · [EN 1995](en1995-mutations-memory-boundary-2026-11-25.md) · [EN 1996](en1996-mutation-carve-2026-11-25.md) · [EN 1999](en1999-carves-low-memory.md)

The new EN 1999 visual-law crate ran all 11 Rust tests successfully through the guarded compiler. The guarded Norm JavaScript suite passed 12,093 tests under a 2,048 MiB grant. Both signed 15-route browser attempts exercised the denial path. The first ended with 2,936 MiB available above the desktop reserve. The second ended after 1,801 seconds with 2,091 MiB available against the required 6,144 MiB. Chromium never launched. The separate 4,916 MiB component publication request also timed out with 3,125 MiB available. Current-source signed browser journeys and fresh publication of all five components remain open. The [verification record](current-norm-verification.md) includes the attempted runs and results.

These changes protect work launched through the repository. macOS and processes outside that control can still consume memory, so an OS-wide guarantee against future application pauses is unavailable.

During the later one-worker DIN 4108 build, a live status sample reported normal pressure and 3,433 MiB of swap in use. The enforcing warden reported 2,276 MiB compressed and recorded two suspensions and two resumptions. That sample still left only 2,255 MiB available for new work after reservations.

On November 26, between compiler runs, the ledger showed zero running grants and zero waiters. The machine then had 5,792 MiB available, leaving 3,744 MiB after the 2,048 MiB desktop reserve. Even with the repository queue empty, that was below the browser requirement and the first publication attempt’s extra envelope. The standard component controller uses 2,048 MiB for its own work and retains compiler admission and warden checks. The 4,916 MiB outer request was specific to that attempt.

The warden also stopped a real EN 1990 mutation library compile at 1,719 MiB against its 1,536 MiB library ceiling. The scheduler exited with status 143 and the repository command reported failure. The physical replay test could not execute in that run.

After the editor rules and 31 remaining inverse computations moved to their owners, the same native library compiled with a recorded peak of **763,954,832 bytes (729 MiB)**. The comparison covers the two observed builds in the shared working tree. All 134 mutation identities and the GraphQL, JSON Schema, Protobuf and TypeScript contracts were preserved. [Inverse differential](en1990-inverse-carve.md)

A later single-worker EN 1990 diagnostic encountered host-wide critical pressure while still building dependencies. The warden terminated the repository’s Cargo controller and the command returned failure. This exercised pressure cancellation independently of the per-library ceiling. [Native run record](codex-en1990-geometry-replay-2026-11-25.md)

Materials also has a distinct final-link requirement. Eight historical WebAssembly final links peaked between 2,162 and 4,786 MiB. The standard component path successfully refreshed its descriptor emitter and admitted 876 source units. It subsequently declined a local final exceeding its grant and checked the fleet for an eligible linked-final worker. [Component controller and link accounting](materials-controller-admission-2026-11-26.md)
