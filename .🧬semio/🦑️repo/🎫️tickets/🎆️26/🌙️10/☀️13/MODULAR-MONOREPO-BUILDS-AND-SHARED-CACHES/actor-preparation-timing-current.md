# Actor Preparation Timing and Owned Process Observations

## Collected Actor 49501 facts

The reviewed provenance Source ran through one authorized actual prototype Original, 49501. It is terminal exit 1 at 289116 ms. Pack completed 0/0. RetainedTurn reached Rust compiler execution and then stopped at the preserved `cargo:test timeout 134716ms`. No release ran, no Actor retry followed, and no recensus mismatch occurred. The same-Original metadata observer drained 516 samples / 26 events / 28367 encoded record bytes and closed its actual image before terminal.

| Retained evidence | Observation | Limit of inference |
| --- | --- | --- |
| Cargo admission | 240000 ms is created at launcher entry | Covers preparation plus compiler, not compiler alone |
| Actual Cargo child allowance | 134716 ms | About 105284 ms was already consumed before spawn |
| Whole Original | 600000 ms, terminal around 289116 ms | Whole Original deadline was not exhausted |
| Core gate | Ten of ten cores admitted for both launches | Earlier gate wait duration is not recorded |
| Broker memory grant | 26 GiB | This is admission policy, not measured RSS or CPU activity |
| RetainedTurn physical free bytes | 4297216000, floor 4294967296 | Initial admission passed; this is not a permanent free-space guarantee |
| Store/profile exclusion | Existing maintenance writer, physical Cargo locks and complete descriptor checks passed | Their individual wait durations are absent |
| Actual compiler output | Dependencies progressed through syn/serde and DSL into RetainedTurn test Source warnings | No `Finished` or test execution receipt; no Rust error reported before enforced stop |
| Process snapshot | Cargo/rustc ancestry was present once | One PID snapshot proves neither CPU nor I/O rate |
| Recensus | No mismatch in this cohort | Does not attribute or disprove earlier timestamp reversals |

`launchRepositoryCargoV1` creates its finite admission before `withRepositoryCompilerStoreV1`, dependencies, unit graph, Source admission, writer/profile exclusion, CPU capacity and scheduler setup. `execute` reads only the remaining control time and passes that to `runOwnedCommand`. The timeout preserves that original grant. Existing generic running heartbeats cannot split the approximately 105.3-second preparation interval. No SDK contribution, CPU bottleneck, profile-lock bottleneck, filesystem stall or compiler hang is established by this retained evidence.

## Wired next-run observations

The Actor prerequisite now passes only an opt-in `SEMIO_CARGO_TIMING=1` diagnostic flag in its existing environment. Its command, selected manifests, mode, actual Original, 240000 ms per-call limit and 600000 ms whole limit are unchanged. The launcher emits fixed begin/ready/end/error points at actual compile-route boundaries: store, inputs, unit graph, Source admission, writer, profile lease, capacity, scheduler/broker, and child execution. Acquisition readiness is distinct from later retained lifetime. An absent ready/end point identifies an open measured interval; it is not a claim that a specific lower resource is blocked.

The existing one-second-throttled control publication still awaits the original admission publisher first. It then copies the already-available resource/mode/wait-elapsed fields into the bounded timing stream. These are progress data, not writer custody. Malformed diagnostic data emits typed unavailable information and does not add a new operation refusal. Monotonic process timestamps correlate phase/resource records with the prerequisite's subtree samples. The original private control, cancellation, requested grant and all exclusion scopes remain the execution authority.

The prerequisite's already accepted observer scope also takes a bounded Unix process snapshot every five seconds. Each ps child uses the actual carried issued control narrowed to at most two seconds. It selects the actual executing Bun PID and its observed descendants, collecting CPU time, process age, state, RSS, bounded command name and observed birth string. A linear bounded traversal excludes unrelated processes. Raw host snapshots are not logged or retained.

CPU deltas are printed only when observed PID, parent and birth string agree and CPU/age clocks do not move backwards. Unix lstart has second precision: this is diagnostic correlation, not a cryptographic PID-generation identity or retirement proof. Short-lived children can be missed; a process leaving the observed subtree is not proof of process death. The root's own CPU includes observer overhead. State/RSS and CPU-time changes do not prove filesystem I/O progress. These limits are essential when interpreting the next cohort.

## Finite policy and settlement

| New quantity | Closed maximum |
| --- | ---: |
| Raw OS snapshot | 1048576 bytes / 8192 rows |
| Logged observed subtree | 64 rows, trimmed further to record byte limit |
| Process-name display | 64 UTF-16 units |
| Snapshot interval / count | 5000 ms / 120 |
| Single JSON observation record | 16384 bytes |
| Subtree log including fixed prefix/newline | 1048576 bytes |
| Phase events per Cargo call | 96 |
| Progress-resource events per Cargo call | 256 |
| Combined launcher timing stream per OS process, prefix included | 1048576 bytes |
| One ps child grant | At most 2000 ms, intersected with the same Original |

No control, invocation, physical loan or registry is minted from any timing record. The parser/formatters accept ordinary data only. No new process receiver, global authority registry, structural bridge, directory writer, deletion API or namespace enrollment was added. The Core process/lease/provider owners are unchanged. The default launcher is silent and performs no sampling; the Actor diagnostic prerequisite opts in explicitly.

The process sampler shares the registered observer operation with the existing exact-unit metadata loop. Completion stops the loop, wakes its timer, awaits any accepted ps child and closes the actual Native image before the Original settles. A requested-child deadline cannot extend the parent. Existing primary plus observer failure aggregation is preserved. Unsupported native process snapshot platforms emit an explicit unavailable record; macOS runtime is tested, Linux input grammar is portable-only evidence, and Windows has no claimed provider runtime qualification.

## Test-driven qualification

A genuine missing-owner RED preceded the timing owner (`actor-timing-red.log`). A separate missing resource-formatter RED preceded its implementation (`actor-timing-resource-red.log`). The permanent language-neutral fixture has Unix clock variants, before/after process snapshots with a foreign subtree, new births, phase records and resource/truncation records. Independent Python/SQLite recursively selects descendants, computes CPU differences and produces the same phase/resource JSON. Hostile tests reject duplicate PIDs, oversized captures, malformed clocks and nonfinite event data; observed birth changes suppress deltas and oversized log records are bounded.

The actual host law runs under one real finite carried Original per Bun process. It captures two genuine system snapshots around bounded CPU work and confirms positive CPU-time movement and RSS on that exact still-live test root. This qualifies the macOS measurement provider; it is not a retrospective measurement of Actor 49501. The actual Actor sampling path and phase points are wired for the next authorized cohort and have not yet been exercised by an Actor run.

Final results and Source handback are appended after collection. No further Actor runtime is authorized by this report.

## Exact changed owners

- `library/🏃️process/🦀️cargo/⏱️timing/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/{🟦️.ts,🔣️.json}}`
- `library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`: opt-in observations only; actual arguments, grants and exclusion calls retained.
- Ticket `actor-current/📜️script.ts`: same-Original bounded subtree sampling and opt-in flag.
- Ticket `issuer-bootstrap/📜️script.ts`: cheap timing/strict prerequisite route.
- Both editor catalogs: timing prototype/release adjacent to existing issuer gates.

The new `issuer-bootstrap timing prototype|release` routes require no Rust compiler headroom, Native image, tools publication or Cargo invocation. Root's independent Source review is retained in `cargo-timing-subtree-independent-current.md` and is explicitly not a runtime test.


## Collected Source handback

All timing/observer Source is now frozen. The final cheap prerequisite is BOTH GREEN, one test / nineteen assertions: prototype 398 ms and release 392 ms (`actor-timing-frozen-prototype.log`, `actor-timing-frozen-release.log`). Independent SQLite matches descendant selection, CPU deltas, phase output and resource/truncation output. Actual macOS root CPU movement is measured at 50 ms and 40 ms respectively; these belong to the small controlled diagnostic tests, not Actor 49501. Expanded ES2022/noUnchecked strict is collected exit 0 (`actor-timing-frozen-types.log`). All own handles are terminal.

The independent read-only review found no actionable deadline, exclusion, authority, parser or drain issue, including the final resource formatter correction (`cargo-timing-subtree-independent-current.md`). It ran no tests or Actor probes. The sampler's second-resolution observed birth strings remain diagnostic correlation only.

The proposed next command is the existing editor route: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/actor-current/📜️script.ts test prototype`. It retains one 600000 ms Original, six sequential existing owners, each unchanged 240000 ms Cargo grant, the exact-unit bounded observer, and the now-wired fixed phase/resource plus owned-root CPU observations. Only existing authentic retirement may create headroom. No next cohort or release was launched at handback; Root authorization/coordination is the next step.


## Authorized diagnostic prototype 92270: collected terminal

Root authorized exactly one current-Source prototype using the frozen diagnostics. The existing `actor-current/📜️script.ts test prototype` route is collected exit 1 after 4385 ms (`🗑️generated/actor-timed-prototype.log`, actual Bun PID 59967). Its one Original remains 600000 ms and its first Cargo admission remains 240000 ms. No grant was renewed.

This run stopped before the first compiler at the genuine physical initial-headroom guard. The fixed Rust image had 4293222400 available bytes against 4294967296 required, a deficit of 1744896 bytes. Existing managed recovery scanned three prototype units totalling 27929380 logical bytes and zero release units; neither scan selected a retirement. Available physical bytes remained unchanged. No manual retirement, cap increase, row adoption or witness rewrite occurred.

The actual launcher timing path now executed: store admission consumed 2970 ms; inputs began at elapsed 2970 ms; the actual prototype cache-scan shared-resource observation reported a 10 ms wait at elapsed 3948 ms, with 236050 ms remaining. Thus this stop is a measured physical admission refusal, not a Cargo child timeout, observed compiler CPU stall, or recensus mismatch. No child-begin phase or Cargo compiler was reached.

The observer stopped and drained before terminal: eight exact-unit samples, one metadata event / 342 bytes, one process snapshot / 528 bytes, stopped true. The observed root CPU was 250 ms with no earlier comparable sample, so no CPU delta or I/O progress can be inferred. The snapshot included only the actual root and its short-lived ps child. Process inventory counts from the enforcing recovery are diagnostic data; they do not authenticate another process, its liveness, a contention cause, or retirement authority. The real image closed before Original settlement.

Release remains held and no retry was launched. Current Source stays frozen. The next dependency is lawful physical headroom under the existing managed retirement protocol; no complete current Actor composite is claimed.

## Post-Retirement Prototype 12456

Root scheduled one actual `actor-current/📜️script.ts test prototype` after guarded retirement 83670 and pinned graph oracle 36011 settled. Session 12456 was collected terminal exit 1. The unchanged 600000 ms whole Original and 240000 ms first Cargo grant remained in force. Actual physical headroom passed at 4,302,499,840 bytes against 4,294,967,296 required. Store admission took 2159 ms, then inputs began. Actual `cargo fetch --locked` refused stale `🧰️framework/Cargo.lock` before compiler execution. No crate/test completion is claimed.

The actual observer drained at 4876 ms: nine metadata samples / one event / 342 bytes, one process sample / 532 bytes, stopped true. All own runtime/image/control scopes closed before terminal. No release or retry followed. The current UI render-owner transplant has still-settling workspace/direct-consumer manifests; its owner reported this explicitly. The next lock refresh is deferred until that Source manifest handback, then actual managed offline refresh and locked fetch must pass before another Actor cohort.

Exact settled output `🗑️generated/root-actor-post-retirement-prototype.log`: 3947 bytes, SHA256 `0865d21110c2e58c81ad944576bfe3c3eb87a9628a855ad544ab684d935235d4`. This failure supersedes neither the historical partial Actor successes nor earlier capacity refusals.
