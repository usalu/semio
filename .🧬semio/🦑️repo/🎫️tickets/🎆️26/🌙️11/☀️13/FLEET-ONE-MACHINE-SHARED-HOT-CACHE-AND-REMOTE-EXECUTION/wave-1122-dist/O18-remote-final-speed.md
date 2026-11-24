# O18: a remote final that is faster, survives a store restart and never meets a foreign linker

L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, R = `L🌐️remote-execution/`. Lock `lock.offload-link` released. Restarted nothing, no scratch build on the coordinator (one 160-module wasm crate in the session scratchpad, deleted), nothing written under `/Users/ueli/.sinc/play`. Landing record in `T/landings.md`.

## 1. Why the grant was 1 (established)

Which side prints it: the requester prints `, N codegen threads` from the worker's `auxiliary_metadata` entry (`R🚀️offload/🔌️client/🟦️.ts:111` reads `type.semio.dev/codegen-threads`, `R👷️worker/🟦️.ts` writes it from `granted`). A pre-O16 worker returns no entry, so the suffix proves that m06, m12 and m10 ran O16 code. Not the cause.

The cause is the load term. The O16 grant was `busy = max(others, ceil(load))`, `threads = cores - reserve - busy`. Live reading now (`repo fleet status --json`, 08:20): loads on 10-core Macs are m01 16.1 (12 cores), m02 18.4, m03 9.7, m06 14.9, m07 10.3, m08 3.8, m10 8.4, m11 19.6, m12 12.9, coordinator 16.6. Every machine is a builder as well as a worker (its Ready stage, local cargo, offloaded units in and out), so the premise "workers with about 10 idle cores" was wrong: with reserve 1 the old formula grants more than 1 thread only at load <= 7 (m08 alone). The memory term is not binding: with the ledger value (2.6 GB) and a spare of 4.9 GB it allows 5 threads; only the unmeasured family estimate (7.9 GB) forces 1, and that affects the first run only. The jobserver is not the cause: the worker builds a private pool in the action's `.tmp` and hands it through `CARGO_MAKEFLAGS`/`MAKEFLAGS`; the stub-rustc sandbox test (worker tests, O16) shows grant 4 puts 3 tokens in the fifo, and a real rustc takes them (below).

Evidence that more threads help on a busy machine (a real wasm32-wasip2 ThinLTO cdylib, 160 modules x 40 exported functions, `-C lto=thin -C codegen-units=16 -C opt-level=s -Z threads=1`, nightly-2026-07-20, on the coordinator at load 16 of 10 cores; jobserver fifo with N-1 tokens exactly as the worker makes it; same sha256 `74303c2f...` in all three runs):

| back end | wall | user CPU | CPU / wall |
|---|---|---|---|
| serial (`-Z no-parallel-backend`, what a grant of 1 runs) | 10.22 s | 8.59 s | 0.84 |
| pool of 4 | 4.33 s | 8.27 s | 1.91 |
| pool of 8 | 3.82 s | 8.64 s | 2.26 |

The operating system divides a busy machine by threads: a unit with one thread gets about 1/(L+1) of the cores, one with four about 4/(L+4). The final is the longest unit of the build, so taking that share from short units shortens the build; total work is unchanged. For the real final (31 CPU-minutes) the grant of 4 turns about 0.8 core into about 2.5 cores at load 12.

## 2. Fix

Schema first: `WorkerPolicy.codegen.fairShareThreads` (value 4) in `L🏃️execution/🧬️schema/🌐️remote-execution.json` / `L🔣️policy.json`.

`R👷️worker/🧵️codegen/🟦️.ts` `remoteExecutionCodegenThreadsV1`:
- `cores' = cores - reserve`, `idle = cores' - max(others, ceil(load))`, `byCores = min(cores' - others, max(idle, fairShareThreads))`: load can no longer take the fair share away, but the cores granted to the worker's other remote actions still count (two finals never share the same cores).
- `byMemory = 1 + floor((spare - declared peak) / 512)` unchanged; the maximum 8 unchanged.
- New: `pressured` (memory pressure raised) grants 1. `R🛰️agent/🟦️.ts` sets it from the memory session's own gate (`remoteExecutionPressureGateV1`, the one the admission uses: `pressure-warn`/`pressure-critical`), so the memory term uses their interface and nothing is duplicated.

Vectors (`R👷️worker/🧵️codegen/🧪️tests/🔣️.json`, 17 cases, independent Python oracle `🐍️.py` extended): `busy-stage-p4` (cores 10, load 12.9, spare 4.9 GB, peak 2.6 GB) gives 4 (was 1); `busy-memory-binds` 2; `busy-family-estimate` (peak 7,938) 1; `others-leave-two` 2; `others-take-all` 1; `saturated-fair-share` 4; `pressured` 1. The cluster test (two real agents with a simulated host view) now asserts: idle 16-core worker grants >1, a worker at load 30 grants exactly the fair share (4) and fewer than the idle one.

Learned peak: unchanged and correct as it stands. The worker measures the process group's peak, the requester writes it to the crate ledger (up at once, halfway down), so a parallel run raises the declared peak of the next one and the memory term then grants fewer threads on a tight machine (O12/O16). The first parallel run of a never-measured final is bounded by the 512 MB per thread margin, the warden and the exit-137 path. Need reported to the memory session (not done by me): the warden ranks `remote-unit` rustc as cheapest to redo, cancelled newest-first within the rank; a 20-minute final should be protected by elapsed time (it is, as "least work lost" sorts the oldest last) but a final that has run 15 minutes deserves a rank above cheap units.

## 3. The 8.3-minute queue (explained, not a defect in the natural path)

- It was a pinned run. A pin uses `semio-offload=queue` and waits for a worker with room (`waitMs`). The natural path is bounded: O17's gate waits `finals.waitMilliseconds` (30 s) for a worker that fits the placed peak, else `final-wait` and the unit compiles here (`R🚀️offload/🚦️admission/🟦️.ts:193`; seen once in the fleet counters, `reasons.final-wait` = 1).
- Why no worker fitted: the placed peak of an unmeasured final is the family p90 (7,938 MB; the cold bound is 6,144 MB), larger than every Mac's reported spare (m12 had 4.9 GB). The lane limit (W4: a worker whose Ready stage runs lends at most `stageConcurrency` = 1 unit) was not the reason; it only means a final occupies that worker's one remote lane for its whole run, so the worker declines cheap units meanwhile (intended trade-off, one number in the schema).
- After the first measurement the ledger holds 2.6 GB and the next placement waited 0.9 s. O12's single charge (landed) removes the second overcount. What remains is the first run of each of the 45 unmeasured `semio_hub_*_component` finals: it needs the family estimate or a size-based predictor (output size correlates only loosely: 157 MB hub final 2.6 GB, 214 MB cad 6.5 GB, 88 MB puzzle 3.3 to 6.1 GB), so I did not invent one; the first runs go to a worker with 8 GB spare or compile locally after 30 s. Not changed.

## 4. g1-m06: linker identity (landed)

What was covered: the toolchain digest is the hash of the `rustc -vV` text; the SDK is covered for native links (`semio-sdk`); nothing covered the linker that consumes the bitcode and objects of a wasm final: `rust-lld`, `wasm-component-ld` and the `gcc-ld` wrappers in `<sysroot>/lib/rustlib/<host>/bin`. A worker whose install of those files differs from its compiler's (a damaged or partial component install, a different LLVM) reads the same `rustc -vV` and so was a valid placement. The m06 error (`rust-lld: unknown file type` on exactly 10 `.rcgu.o.rcgu.o` bitcode objects) fits that: a linker that cannot read the bitcode version of the compiler that wrote it. I cannot read m06's sysroot from here, so this is the best explanation, not a proof; the fix does not depend on the exact mismatch.

Change (schema first, `R🚀️offload/🧬️schema/🔣️.json` `property.linker` = `semio-linker`):
- `R🚀️offload/🧰️toolchain/🟦️.ts` `offloadLinkerIdentityV1(sysroot, memoRoot?)`: 16 hex of the SHA-256 over the relative path and content digest of `rust-lld`, `wasm-component-ld` (`.exe` too) and every `gcc-ld/*` file (the optional LLVM tools next to them do not count, so a machine with `llvm-tools` and one without agree); memoised by size and time in the process and in `offload-linker.json` of the unit cache root. 17 MB total, read once.
- `detectWorkerRustcV1` adds `semio-linker=<id>` to the worker's offer (a worker without those files offers none and never gets a linked unit).
- The requester asks for it for linked units only (`R🚀️offload/🟦️.ts`, `offloadSysrootV1` memoised ten minutes like the version text); it does not enter the unit key, only the action's platform properties. A linked unit whose own linker identity is unknown stays local (`not-portable`).
- Test `R🚀️offload/🧪️tests/🔗️linker/🟦️.ts` 3/0 with independent Python oracle `🐍️.py`: identity equals the oracle; optional tools do not change it; a mismatching fixture (other `rust-lld`, truncated `rust-lld`) has another identity; the real `detectWorkerRustcV1` offer carries it; the scheduler's `placementCompatibleV1` accepts the matching worker and refuses the mismatching one (the g1-m06 case), and a unit that links nothing asks for no linker.
- Rollout: workers advertise after their own restart; until then a new requester's linked units find no matching worker and stay local. The play requester (snapshot) does not send the property and is unaffected.

## 5. g1-m03: the lost 22.8-minute result (landed)

Why it was lost: the worker handed over its outputs through `run.content.write`; a large file (the 157 MB component) is streamed with `retries=0` (a consumed stream cannot be replayed), and a byte write retries 3 times with 50, 100, 200 ms, so a store that was down for minutes failed at once with code 14 and the requester fell back after the work was done.

Change:
- Schema `WorkerPolicy.outage` {seconds 600, initialMilliseconds 1000, maximumMilliseconds 15000} (shared by workers and requesters, EN descriptions in the schema).
- `R🗄️content/🟦️.ts` `retryRemoteExecutionContentOutageV1`: repeats an operation while the store is unreachable (code 14), pause doubling to the maximum, bounded by the outage seconds from the first failure; other errors (507, 4xx), abort and an outage of 0 end at once. `outageTolerantRemoteExecutionContentV1` wraps the replayable calls (has, missing, read, byte write, action result).
- Worker (`R👷️worker/🟦️.ts`): `missing` and every output write are retried; a file is re-streamed anew each attempt.
- Requester (`R🚀️offload/🟦️.ts`): for a linked unit the execute budget, the deadline and the download time limit grow by the outage seconds (so it waits for the worker's retries), and the output reads go through the tolerant store. Cheap units keep their short bounds, so an unreachable store at the start still declines at once.
- Tests: content-retry 7/0 (4 new: authored outage, doubling and capping with a fake clock, bound/refusal/abort/zero, tolerant decorator), worker 11/0 (new test: the store refuses the first missing check and two writes with code 14, a 300 kB file streams again, the action completes after the policy's pauses).

## 6. Fleet-cache hits 0 in P4's second run

Not a defect that I can show; the evidence is incomplete because the scratch cache root (and its event log) was deleted by P4. What the numbers say: the first run's units were compiled and published from the coordinator; the second run used a fresh target but the same cache root, so every unit whose key was unchanged was a LOCAL hit (399) before any fleet lookup, which is the correct order. The 760 compiles are units whose key changed between the runs: peers edited framework crates all day (P4 point 8: lock file churn, four unbuildable windows), and a key covers the sources of the crate and of everything below it, so one edit in a low crate re-keys its whole dependent cone. A re-keyed unit can be a fleet hit only if another machine compiled the same tree state and published it; none had. So 0 fleet hits on the changed cone is expected, and 0 fleet hits on unchanged units cannot occur (they hit locally first). What stays unproven is that an unchanged key from run 1 would have been a fleet hit on ANOTHER machine; that needs the proof of the acceptance run (second build on another worker after the first published). The fleet-wide `spool-full` counters (m02 53,450, m06 50,412) are historical (E1: +0 in a 9-minute window after the E74 fix), and U1 is working on that area now. Suggestion for the next measurement: log the key of the first miss per unit with the fleet-cache lookup outcome (`keySamples` exist in the counters) so a re-keyed cone and a true fleet miss separate.

## 7. Files and tests

Changed: `L🏃️execution/🧬️schema/🌐️remote-execution.json`, `L🔣️policy.json`, `R👷️worker/🧵️codegen/🟦️.ts` (+ tests `🔣️.json`, `🐍️.py`, `🟦️.ts`), `R🛰️agent/🟦️.ts`, `R🗄️content/🟦️.ts`, `R👷️worker/🟦️.ts` (+ test), `R🚀️offload/🟦️.ts`, `R🚀️offload/🧬️schema/🔣️.json`, `R🚀️offload/🧰️toolchain/🟦️.ts`, `R🚀️offload/📜️action/🟦️.ts`, new `R🚀️offload/🧪️tests/🔗️linker/{🟦️.ts,🐍️.py}`, `R🚀️offload/🧪️tests/🔗️native-link/🌐️cluster/🟦️.ts`, `R🧪️tests/🔁️content-retry/🟦️.ts`.

`repo test-file`, one run at a time, all pass: codegen 3/0; worker 11/0; content-retry 7/0; linker 3/0; native-link cluster 2/0 (the first run failed only because my own new pressure scenario asked a critical-pressure worker to take a unit, which it rightly refuses; removed, the pure vectors cover pressure); offload 42/0; wasm-cdylib 8/0 (once failed to import because lane U1 was mid-edit in `unit-cache/🛰️fleet`, passed on retry); finals 5/0; gate 4/0; failure 2/0; native-link 10/0; single-charge 8/0; unit-admission 10/0; scheduler 22/0; server 17/0; fleet 10/0. `L🧪️tests/🧬️policy-literals` 59 pass, 4 fail (observation, inference, generations literals, cache-gc texts): the same four as in O16, from other lanes.

## 8. Open

- Real remote run of the actual hub final (to put numbers on 4 threads at worker load 12) needs the 15 GB scratch build rule and a worker restart onto this code; not done. Expected from the table: about 2.4x on the back end, so roughly 8 to 11 minutes instead of 19 to 27.
- The first run of a never-measured final still queues or compiles locally (section 3).
- The memory session may want the warden to treat a long-running final as costlier to redo than a cheap unit (section 2).
- Windows worker: the pool and fair share are untested there.
