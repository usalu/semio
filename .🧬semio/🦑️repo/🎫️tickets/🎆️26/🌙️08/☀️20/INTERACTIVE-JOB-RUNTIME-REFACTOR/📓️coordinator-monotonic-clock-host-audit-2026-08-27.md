# Monotonic Clock Host Audit

## Current Verified Source and Runtime Scope

The two plugin outer callbacks now use the shared checked optional microsecond clock, not unconditional std::time::Instant. Their deterministic four-law cohort is actually4/4; strict shared7999/8000/8001 boundary and clock-factory authority tests are2/2. Bare framework-async wasm32 compilation is actuallyPASS after7m31. Native owned-WASI host clock execution remains unrun.

The generated Flow initializer and package-sibling loader gate pass against the old binary. The new consumed clock gate actually fails after real catalogue/close because the old binary reports zero startup clock samples. The demonstrator owner will rebuild canonical Flow :wasm and run :test-browser-clock after its active catalog/native queue. No concurrent shared package publication is performed by this fleet. Detailed recovery evidence is in `📓️coordinator-recovery-and-owned-read-review-2026-08-27.md`. Historical source findings below are superseded only at these exact repaired boundaries; all-app/fresh-browser timing remains open.

## Latest Native and Cross-Platform Boundary

The coordinator reviewed actual native core microsecond **5/5** output and the complete five-law source. Registered exact factory dispatch now separately passes **1/1 with a fake microsecond clock** and **1/1 with the actual native monotonic clock**, using the unchanged 500-microsecond contract. Both run real target supersession/rebase, count97 publication, ACK, UTF-8 lost-slot retirement and exact app close. Logs: `🧪️microsecond-driver-green-r2-native-2026-08-27.txt`, `🧪️microsecond-plugin-registered-r2-native-2026-08-27.txt`, `🧪️microsecond-plugin-registered-real-r1-native-2026-08-27.txt`. The earlier registered R1 reached publication then aborted on a missing test-owner forwarding join; its failure evidence remains retained.

Actual consumed host/WASI and bare-Wasm startup are still separate gates. The shared clock is optional and no longer uses synthetic bare ticks. The raw Flow loader's actual built module imports generated flow_core_bg.js while its default raw instantiate supplies no imports; generated initializer integration is assigned without introducing a new import namespace into all rlib consumers.

Two outer plugin callbacks still used unconditional std::time::Instant at this source inspection: run_runtime_live_cleanup_turn and run_runtime_close_turn. Their module is Wasm-reachable and plugin_step_close_cleanup pumps the callbacks on Wasm. They must use the same checked optional real microsecond clock as job deadlines, faulting before work if unavailable and retaining exact owners; a native-only outer clock is not sufficient browser integration. The transport executor owns those two callbacks in coordination with the shared clock owner.

The plan's hard boundary is strict: duration at or above 8,000 microseconds must fail. Earlier predicates used greater-than and even the new close fixture admitted equality. Trace and plugin callback equality laws are assigned; no threshold is raised. Current cold-close timing remains RED as recorded in `📓️coordinator-instance-close-native-r1-review-2026-08-27.md`.

## Source-Confirmed Defects

The coordinator inspected the live sources on 2026-08-27. This is a source audit, not a runtime timing pass. The publication executor owns implementation and native verification; the unchanged 8,000-microsecond hard ceiling remains required.

| Boundary | Observed implementation | Required correction |
| --- | --- | --- |
| Native owned-Wasm host | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`, `reply_owned_host`, returns `I64(0)` for `wasi:clocks/monotonic-clock@0.2.0.now` | Real monotonic nanoseconds at the WASI boundary, with checked unit/range handling |
| Live owned operation | The same file's `resume_owned_operation` calls `reply_owned_host` when the actual resumable interpreter yields a host call | A native outer watchdog cannot substitute for the guest's missing cooperative clock |
| Native/WASI trace | `🧰️framework/🔨️modules/⏱️trace/🦀️component.rs` uses `Instant.elapsed().as_micros()` | Preserve microsecond precision through job admission, absolute deadline, checks and watchdog |
| Bare Wasm trace | The same trace module uses `AtomicU64.fetch_add(1)` as a fallback; `install_clock` has no callers in the inspected framework/app trees | Install a real owned host monotonic-clock seam; synthetic ticks are not elapsed time |
| Browser WASI shim | Existing `node_modules/@bytecodealliance/preview2-shim/dist/browser/clocks.js` reads `performance.now()` and converts milliseconds to nanoseconds | Confirm actual generated component wiring and unit conversion; browser resolution is not a hard real-time guarantee |

The separate `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️component.rs` describe host also returns zero, but it is not the live host above. Descriptor-only behavior must not obscure the production defect. The pure `now-ms` host callback and generated browser `nowMs` shim use wall-clock epoch milliseconds; they are not replacements for monotonic cooperative deadlines.

## Required Verification

Additional source scout: the owned raw Flow browser entry `🌊️flow/🌉️wasm/📦️packages/🟨️javascript/🟨️flow-browser.js` defaults instantiate imports to an empty object. The adjacent Flow host defaults to `Date.now` and passes a frozen millisecond timestamp/deadline for each raw ABI call; repeatedly checking that stored timestamp cannot measure time consumed inside the call. Its default repeated `queueMicrotask` scheduling also needs a paint/input-fair task-yield audit. The publication and Flow executors have the exact source locations and ownership boundary. The Puzzle standalone Board loader uses existing wasm-bindgen package initialization and needs its own real clock binding/fresh generated bundle. None of these source observations is a runtime benchmark.

The publication executor is now unifying job deadlines and trace watchdogs behind one installed real optional clock, eliminating the bare-Wasm synthetic fallback and deriving coarse milliseconds explicitly. This source change remains under implementation; actual consumed-entry installations and registered plugin verification are still required. The core driver's four native laws are independently source/log-reviewed in `📓️coordinator-source-r16-2026-08-27.md`.

Schema-first exact cases cover 1, 499, 500, 999 and 1,000 microseconds, expiry at equality, rejection on deadline overflow, missing-clock admission rejection, and a real registered 500-microsecond job that can progress and yield. Host-clock installation must be executed on each supported Wasm path, not merely defined or made fail-closed. A missing-clock guard is safe failure but is not finished app functionality. No larger runtime grant, rounding up to milliseconds, synthetic tick or zero clock is accepted.

The coordinator sent the exact live-host findings to the sole compiler executor and notified the transport executor about overlapping host ownership. The executor is migrating job consumers while preserving separately named coarse actor/backbone millisecond timers. Native, fresh Wasm and browser runtime evidence remain pending.
