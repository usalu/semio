# SDK Capture Critical Path

## Source-Only Finding and Proposed Scope

Current `sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts` hashes every regular file with `openSync`, two `fstatSync` calls, positional `readSync` calls and `closeSync`. The existing 16-worker parent-layer executor keeps multiple descriptor lifetimes active because hashing yields between chunks, but each actual synchronous descriptor call still blocks that process's main thread. It does not provide asynchronous disk I/O.

The retained failed drain capture measured **12.362 seconds in those synchronous descriptor intervals** inside its **57.197-second instrumented capture callback**. That is a real bounded target, about 21.6% of that measured interval, not proof that all 57.197 seconds were synchronous hashing or that this change alone makes every operation finish within 120 seconds.

Proposed next implementation, pending Root review: replace only the private descriptor read/hash helper with genuine asynchronous built-in FileHandle open/stat/positional-read/close, still under the **same** layer executor's maximum 16 workers and unchanged 64 KiB chunk, Original/control checkpoints, real base custody, complete SHA, exact extent checks, descriptor pre/post identity, rooted namespace recensus and accepted failure drainage. Preserve synchronous short enforcing-prefix guards; do not call the result “zero synchronous filesystem work” globally.

This report is the required attribution/effect-scope handback **before any Source/test/schema edit or probe**. No runtime/copy retry, Original, tools/Rust image scope, Cargo command, maintenance, publication or Source clone was started for this analysis. The failed SIGKILL stage remains unknown and untouched.

## Exact Retained Scalar Evidence

All three current five-fence operations consumed the same **11,701 entries / 10,189 files / 168,402,658 bytes / 12,155 chunks**. Metrics are rounded milliseconds from the actual retained child stdout files.

| Logged scalar | Runtime release 99215 | Runtime prototype 68153 | Failed drain prototype 32974 |
| --- | ---: | ---: | ---: |
| Capture callback elapsed ms | 10,571 | 16,106 | 57,197 |
| `read.openMs` | 1,677 | 2,470 | 4,011 |
| `read.statMs` | 88 | 121 | 406 |
| `read.readMs` | 2,858 | 4,239 | 7,708 |
| `read.closeMs` | 41 | 60 | 237 |
| Sum of synchronous descriptor intervals | **4,664** | **6,890** | **12,362** |
| Sum / logged capture interval | 44.12% | 42.78% | 21.61% |
| `read.checkMs` | 31 | 71 | 327 |
| `read.yieldMs` | 40,240 | 70,637 | 148,169 |
| `walk.parentMs` | 75,543 | 104,904 | 501,520 |
| `walk.lstatMs` | 36,903 | 54,828 | 166,458 |
| `walk.controlMs` | 17 | 36 | 98 |
| `walk.publishMs` | 31 | 14 | 18 |
| `walk.yieldMs` | 222 | 320 | 433 |
| `walk.recheckMs` | 2,924 | 2,299 | 10,162 |

The synchronous intervals are disjoint executions of the same main thread, so their sum identifies observed time during which this helper does not yield to other JavaScript. It includes system-call waiting and host descheduling, not merely CPU or raw-device service time. The close interval also includes its final control check. These are rounded measurements, not a profiler's exact CPU breakdown.

The asynchronous parent/stat/control-yield/store totals overlap across workers. They include scheduler/event-loop waiting, work by other workers, and potentially actual I/O. They are **not serial stages** and must not be added to 57.197 seconds or used to blame a resource holder. In particular, 148.169 accumulated yield seconds does not mean an authored 148-second sleep. The helper retains one real Original continuation after every consumed chunk; removing those checks/yields is outside scope.

Compared with release, the failed capture interval grew by 46.626 seconds while the synchronous descriptor sum grew by 7.698 seconds. At least the remaining 38.928-second difference is not explained by these synchronous counters as a disjoint timing category. Other work overlaps, so this subtraction is a limitation on attribution, not a causal decomposition or guaranteed available speedup. SHA `hash.update`/digest CPU, metadata/index parsing/sorting/encoding and some control calls are not independently timed by these counters. No disk, CPU, queue, PID, agent or capacity cause is established by this report.

## Boundary Labels Are Different Operations

The logged `SDK source index captured` is the capture of the actual **pristine dependency tree's `node_modules`**, including complete regular-file hashes and actual namespace discovery. It is not the first-party authorizer/compiler Source capture performed by normal `captureNxToolingInputsV1` in the tools publisher. The private runtime/drain fixture selects the authentic existing pristine base and invokes the derivative projection; it does not run a fresh normal SDK authorizer build.

The capture timer starts **after** initial SourceTree acquisition and reading/parsing any existing sealed index. It logs before returning from the SourceTree callback. SourceTree's final rooted directory recensus runs **after** that log and before the capture function returns. Therefore 57.197 seconds is not the entire projection duration or even every outer guard cost around capture.

For the two runtime operations with stage points:

| Actual outer stage | Release | Prototype |
| --- | ---: | ---: |
| Before projection | 0.260 s | 0.618 s |
| Projection held | 12.352 s | 18.643 s |
| Difference | 12.092 s | 18.025 s |
| Logged inner capture | 10.571 s | 16.106 s |
| Remaining outer/setup/final-recensus/fact-derivation interval | 1.521 s | 1.919 s |
| Quote held | 12.550 s | 20.430 s |
| Actual copy settled | 44.681 s | 89.064 s |
| Child terminal | 59.60 s, exit 0 | 115.53 s, exit 0 |

The drain fixture records `before-drain-projection` at 0.670 seconds but has no subsequent projection-held/quote-held marker. Its complete inner capture is observed, subsequent copy progress occurs, and no copy/drain completion is observed before cancelled/SIGKILL termination under the unchanged 120-second policy. Do not fabricate the missing stage durations. The five physical-fence split reduced demonstrated duplicate work; it does not explain all timing variance in initial capture or later copying.

## Exact Existing Owner and Effect Scope

The private `hashSource(path, stamp, store, metrics?)` is called directly by complete capture. Its exported string-returning wrapper `hashNxSdkSourceFileV1` is also used by full sealed shared-source verification and cold index sealing. An asynchronous replacement therefore affects all three real descriptor consumers. Their current real base SH/EX and actual store/control ownership must remain; no new caller-selectable backend, pool size or authority is needed.

The existing Source layer owner creates at most 16 workers in each parent depth layer, stops new births on the first failure and awaits every accepted worker before throwing the exact value, including `undefined`. The helper must keep descriptor ownership inside that accepted worker: no fire-and-forget open/read/close, no promise that outlives the layer callback, and `await close()` before success/failure settlement. The same 64 KiB buffer per active helper bounds the file read arena at 1 MiB for 16 active helpers; runtime/library overhead is separate. No full-file read buffer, preliminary hash pass, extra index copy or new runtime library is proposed.

Suggested actual descriptor sequence:

1. Check the real Original control, open the exact admitted path using the same read-only/no-follow flags, then check control again.
2. Await actual descriptor stat and compare the same device/inode/size/mtime/mode predicate with the admitted stamp. Do not replace it with a path-only check or public data.
3. For each bounded positional read, await the actual descriptor read, check exact nonzero extent and current control, update the same SHA-256 state, retain the existing continuation and subsequent control check. Keep the same original size/offset and chunk ceiling.
4. Await actual descriptor post-stat and compare the unchanged predicate. The caller's actual leaf/ancestor/root/roster checks and final complete capture/index comparison remain.
5. Always await close; preserve a primary read/stat/control failure even if close also fails. A close failure is fatal when no primary failure exists. No caught failure becomes normal capacity DATA.

Use the existing built-in filesystem API; no thread-pool environment tuning, higher parallelism, runtime dependency or new standalone script. SHA update remains synchronous computation. This is asynchronous descriptor I/O, not a claim of zero synchronous checks or constant I/O latency.

## TDD Plan Before Production Replacement

The existing complete-capture and sealed-verification tiny tests currently observe `openSync`/`closeSync` and inject exact Error/undefined failures through those calls. They must be handcrafted to observe/delegate the new **actual** FileHandle operations; simply letting their old observers see zero opens would drop coverage and produce a false green.

Author the closed neutral file-I/O contract adjacent to the existing capture fixtures: maximum 16 active descriptors, unchanged 64 KiB chunks, exactly one actual open per hashed regular file, zero synchronous descriptor open/stat/read/close calls, complete expected path/byte/SHA rows and precise primary-failure precedence. Ajv and independent SQLite counts validate the data; independent Bun.CryptoHasher/file bytes validate every hash. The schema supplies expectations, never a control/store/lease.

Before editing production, execute a genuine tiny RED with a delegating observer that counts synchronous descriptor file I/O on the owned `.part` files/descriptors. Current code must really exceed the zero-sync descriptor budget while returning the correct complete hashes. Keep short enforcing-prefix/directory checks outside that specifically named descriptor budget. Do not install a fake filesystem port or change any runtime effect to create the RED.

After replacement, observe real asynchronous opens and returned handles. Delegate their actual stat/read/close methods without replacing the descriptor or custody. Assert one open per regular file, bounded concurrency, exact complete bytes/chunks/hash and zero active handles after success. Inject Error and `undefined` at open/read/post-stat and after an actual close; stop births and drain all other accepted handles before propagating the exact primary. A body failure plus close failure preserves the original primary; successful body plus close failure remains failed. Do not use constructor/prototype type casts as a substitute for an actual open.

Retain real hostile identity cases: pre-open replacement, mutation/shortening during an awaited read, descriptor post-stat mismatch, changed ancestor/membership and symlink escape. Keep the existing sealed-index mismatch, foreign-hardlink, read-only mode, exact entry cap, cold sealing, source/destination/nested quote and five-fence/drain tests. The whole root Original must reach its actual designed terminal result.

Only tiny focused/strict BOTH after Root accepts this scope. Label future diagnostics with the actual descriptor backend so asynchronous cumulative operation latency is not compared with the old synchronous sum as if both were serialized blocking time. Any full-size gate remains separately authorized; no lucky retry or deadline increase is part of this proposal.

## Provenance and Handback

Observed current Source hashes:

- Index/hash owner: `71721d022c90837470515368f101205bf6ee1d57e5c1d8ea33fb8816a0c755d3`.
- Layer executor: `432c1c7c1a447f48073b6ddc18eab2eb942b6501f1fddbc0f41424c509399141`.

Exact retained scalar inputs are the three persistent child stdout files identified in [the full-gate report](sdk-five-fence-gates-current.md) and [the successful release handback](sdk-revalidation-boundary-current.md). The failed drain stdout hash is `acb5b4c4429f659cd0c96c57388f22f9f2430e0fe115f5e6df4213e0d7067ad2`. This analysis read only those bounded logs and actual Source. No new physical result or successful drain is inferred.

Recommended next scope is the bounded asynchronous descriptor helper plus truthful replacement of its existing observers and new zero-sync RED, with all current guards retained. The measured component is material but incomplete; wider metadata/scheduler attribution remains open. Source is unchanged pending Root review of this handback.
