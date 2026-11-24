# O16 remote cdylib final: the worker sizes the LLVM back end from its own cores

L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, R = `L🌐️remote-execution/`. Lock `lock.offload-link` released. Restarted nothing, no scratch build on the coordinator (the determinism crate was 120 tiny modules in the session scratchpad, deleted).

## 1. Where the flag lived, and why
Three places carried `-Z no-parallel-backend`:
- Requester action builder `R🚀️offload/📜️action/🟦️.ts` `offloadSerialArgumentsV1` (was :33-38, appended to every `cdylib` final): mirrored the local scheduler flag.
- Worker validator `R🧱️actions/🟦️.ts:47` (allow-list of `-Z` values): only accepts it, adds nothing.
- Worker execution `R👷️worker/🟦️.ts`: ran the validated argv as is, with no jobserver (a rustc without an inherited jobserver would take ALL cores uncapped), so the requester's flag was the only memory bound.
Original reason (the old docstring, introduced in commit 508f457ff1e of 2026-11-22; the auto-commit message carries no text): MEMORY. "peak is the front end plus one module instead of one per core and the worker's memory matches the predicted peak." It was a hard-coded stand-in for memory admission. Same reason for the local scheduler flag (O15). Now the memory bound is explicit (below).

## 2. Change (schema first)
- Schema `WorkerPolicy.codegen {maximumThreads, reserveCores, extraMegabytesPerThread}` (`L🏃️execution/🧬️schema/🌐️remote-execution.json`, values in `L🔣️policy.json`: 8 / 1 / 512).
- NEW `R👷️worker/🧵️codegen/🟦️.ts`: pure `remoteExecutionCodegenThreadsV1`: busy = max(cores granted to the worker's other running actions, ceil(load)); threads = clamp(cores - reserve - busy, 1, cap) and at most 1 + floor((spare memory - the unit's declared peak) / extraMegabytesPerThread). `remoteExecutionCodegenArgumentsV1` adds the serial flag only at 1 (a requester's explicit `-Z no-parallel-backend` is honoured: grant 1). `withRemoteExecutionCodegenPoolV1` makes a job-token pool of N slots (rustc keeps its own slot) in the action's `.tmp` (inside the sandbox-writable root) and hands it through `CARGO_MAKEFLAGS`/`MAKEFLAGS`; a pool that cannot be made falls back to 1.
- `R🛰️agent/🟦️.ts`: `threads()` evaluates the grant from the host view (cores, load, spare memory by the same `evaluateRemoteExecutionAdmissionV1` that admits actions, with the other running units' declared peaks held). `R👷️worker/🟦️.ts`: only `cdylib` rustc units ask; the worker tracks each running task's grant (others count 1 each) so two finals share the cores; the granted number goes back in `auxiliary_metadata` (`type.semio.dev/codegen-threads`, `R🟦️.ts`). A worker without a `threads` provider runs serially (conservative).
- Requester: `offloadSerialArgumentsV1` -> `offloadFrontEndArgumentsV1` (front end stays `threads=1`, no flag). Validator unchanged on purpose: an explicit serial request stays legal (it is the only way a requester can ask for 1).
- Memory adapts as O12: the worker measures the process group's peak, the requester writes it to the crate peak ledger and family. A parallel run raises the recorded peak (up at once); a later run on a busy worker declares that larger peak, gets fewer threads (memory term), and the ledger goes halfway down on a smaller measurement. Caveat: a previously serial-measured final may peak higher in parallel; the 512 MB/thread margin and the exit-137 path cover the first parallel run.
- Observability: the `offload-run` event detail now ends in `, N codegen threads`.

## 3. Determinism (real wasm32-wasip2 cdylib, nightly-2026-07-20, `lto=thin`, `codegen-units=16`, `opt-level=s`, same path/remap)
48-module crate: sha256 `c58950ac...` identical for: `-Z no-parallel-backend` (x2), jobserver pools of 2, 4 and 9 tokens (x2 each), rustc's own default jobserver, `-Z threads=1/4/8`. 120-module crate: `db34f3d3...` identical for serial and pools of 1, 3, 9; wall 2.49 s serial, 1.55 s (1 token), 0.72 s (3), 0.68 s (9): the pool really parallelises. Output bytes do not depend on the codegen thread count; no key change needed. Key neutrality of the flag removal: the flag was added in `offloadRustcCommandV1` after the unit key; the key never saw it.

## 4. Tests (`repo test-file`, one at a time)
- NEW `R👷️worker/🧵️codegen/🧪️tests/` `🟦️.ts` 3/0 + vectors `🔣️.json` (11 cases) + Python oracle `🐍️.py` (independent formula, run by the test).
- `R👷️worker/🧪️tests/🟦️.ts` 8/0, 2 new: stub `rustc` under the real sandbox (sandbox-exec) reads its own jobserver: grant 4 -> 3 tokens in the fifo, no serial flag, result says 4; grant 1 / no provider / provider throws -> serial flag, no pool, result 1; explicit serial request -> 1 and the provider is not asked; a `lib` unit is never asked, no flag, no pool, no entry.
- Cluster `R🚀️offload/🧪️tests/🔗️native-link/🌐️cluster/🟦️.ts` 2/0, new test: two real agents (`--join`) with a simulated host view (16 cores): a native cdylib offloaded through the wrapper returns ">1 codegen threads" at load 0.5, "1 codegen threads" at load 30.
- Also pass: actions 10/0, wasm-cdylib 8/0 (real rustc, worker-filter, bytes equal local), offload 42/0, offload native-link 10/0, server `R🧪️tests/🟦️.ts` 17/0, 🌐️fleet 10/0, 🏋️unit-admission 10/0, 🧮️single-charge 8/0, retirement 4/0. `L🧪️tests/🧬️policy-literals` has 4 failures from other lanes (`.mjs` literals in observation/inference/generations, cache-gc `strayScanMilliseconds` texts), none from this change.

## 5. Restart facts
- Workers: new agent + worker code; they self-restart after the quiet time (drain up to 2 h). Until a worker restarts it runs the OLD code: a new requester sends no flag, the old worker then runs rustc with no jobserver (all cores uncapped, no memory bound). So patch the requester only after most workers restarted, or accept uncapped codegen on old workers (the 16 GB Macs: ~1 module/core extra). Old requester + new worker: the flag is sent, the worker honours it (grant 1): safe.
- Requester (play snapshot): `wave-1122-dist/O16-patched/action.patch` (+ `apply.sh [play-root]`, temp-name + mv): the offload wrapper's sealed closure is read PER RUSTC CALL, no driver restart; verified `patch --dry-run` applies cleanly to `/Users/ueli/.sinc/play/.../🚀️offload/📜️action/🟦️.ts`. Not included: the client/wrapper/root hunks (threads in the event detail): observability only, and the snapshot lacks the phases landing they sit on. Key neutral.

## Open
- Windows worker: pool uses a Win32 semaphore via bun:ffi (untested here); any failure falls back to 1 thread.
- Grant is taken at the action's start; two actions starting in the same instant both see the same others (small over-grant, bounded by the cap and the load term).
- The vector for the real fleet (load 7 to 35 on 10 cores) gives 1 thread almost always today: the grant follows the machines' own load, as asked; the gain shows on idle workers.
