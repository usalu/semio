# Rust source admission throughput (2026-11-19)

## Where the time went (measured on the busy machine, CPU about 100 %)
Real data: the 503 remembered unit admissions of `.cache/b/r/m/u/a` (2952 probed paths, 2002 distinct files, 38.5 MB), replayed through the real reader (`source-admission-bench.ts`, `syscall-count.ts`, `tok-bench.ts`, `cfg-bench.ts`, `memo-miss-probe.ts`). The real compilation graph could not be captured: planning waits behind the running build's cargo/host-capacity locks (stopped after 40 min).

| Phase | Finding |
| --- | --- |
| Ancestry lstats + rechecks | old reader: 38.9 `lstat` per read (189 reads, 7357 lstats, parent chains of 14-25 emoji directories, a full chain re-walk at 5 points per file: start, between chunks, after read, between replay chunks, after replay, after EOF probe). Dominant I/O cost; every `lstat` is a libuv threadpool round trip through Defender. |
| open/stat/read | cheap by comparison (about 1-3 ms each idle); replay re-read doubles reads but not opens. |
| yieldContinuation | `setImmediate`, 25 ms for 3400 calls: negligible. |
| cfg (`rustc --print cfg`) | 180 ms each idle (more under load), sequential per unit; the port keyed its cache on args **including every `--cfg feature=...`**, so units differing only in features each spawned rustc. The 503 remembered units carry only 7 distinct cfg sets. |
| Tokenize/select/inspect | CPU bound: 0.85 MB/s end to end (rustTokens 7 MB/s, but a file is tokenized 4 times: delimiter fault, select, inspect references, module scope walk). 55 MB is about 65 s idle, minutes under contention. Closures barely overlap between units (322 distinct files of 341 reads), so cross-unit sharing helps little. |
| Storage recheck | already gated to once per 15 s. |
| Memo | 443 of 503 remembered admissions still validate (88 %); the 60 misses are genuinely edited files (mtime/ino/size) from peers. No memo was written in the last 2 h (every cold unit completing needs all probed files older than the start minus 2 s, and the shared replicated worktree keeps touching files). A memo-hit unit already costs one `lstat` per probed path (36-59 ms for 60 units). |

## Changes
- `📖️read/🟦️.ts`: new run-scoped `createPhysicalCargoSourceReaderV1()` (`read`, `sweep`). Each directory is verified no-follow once (parallel for fresh chains) and re-verified by `sweep` (dev/ino/directory/not-symlink) at every unit boundary and at completion; a file read keeps its own guarantees: lstat, O_NOFOLLOW open, descriptor identity before, fstat identity between chunks, descriptor byte replay, EOF probe, final lstat+fstat identity, ConcurrentEdit retry. Verified bytes are shared with later reads of the same path while its lstat identity is unchanged (not for files younger than 2 s, 256 MiB cap); the request bounds (maximumBytes, remainingFiles, expectedBytes) are applied to every read, cached or not. Any failure drops all caches. `readPhysicalCargoSourceV1` stays as the one-shot (fresh reader, sweep, retry).
- `🎟️admission/🟦️.ts`: the actual port owns one reader and sweeps it in `recheck` (so every read is followed by a sweep before admission returns); cfg is keyed without features and merged exactly (`mergeCargoCfgFeaturesV1`, compiler order by name then value); new optional port method `prefetch(units, control)` starts the distinct cfg invocations of the not-remembered units 4 at a time before the reads (recall now runs once up front per unit); a run-scoped `RustSourceAnalysis` is passed to the closure reader.
- `📥️source/🟦️.ts`: `RustSourceAnalysisV1`/`createRustSourceAnalysisV1` (selected text and references per content digest and cfg key, replaying generated-token observers; 64 Mi chars cap) and an optional 6th parameter of `readRustSourceClosureV1`.

## Before / after
- Reader on 150 units / 885 real reads (cold, busy machine): 16.5 and 21.1 files/s old vs 62.6 and 88.1 files/s new (about 4x); 301 vs 598 KB/s on the 60-unit run.
- Syscalls on 189 real reads: lstat 7357 to 1026 (38.9 to 5.4 per read), opens 189 to 170 (shared reads).
- cfg: one spawn per distinct (compiler arguments, flags, target, kind) instead of per feature set, started 4 ahead; sequential 180 ms+ per distinct invocation before.
- Not changed: tokenizer cost (follow-up: tokenize once per file; delimiter fault, select, inspect and scope walk each re-tokenize; a content-keyed token memo in `🦀️rust/🟦️.ts` would cut about 3/4 of the CPU but changes a shared module).

## Tests (`bun test` from the module folders, SEMIO_TEST_ARTIFACT_DIR set)
- `📖️read/🧪️tests/🟦️.ts` (6 pass): old 3 plus: one run opens each unchanged file once and stays under 4 lstats per file over a 18-deep tree (spy on `fs/promises`, fixture `throughput` in `🔣️.json`, schema `Throughput`); a directory replaced after a shared read is refused by the next sweep (ConcurrentEdit ancestry) and a junction ancestor is refused; a changed file is re-read, cached files still obey bytes/files/extent bounds. Feature scenario added to `🥒️refusal.feature`.
- `🎟️admission/🧪️tests/🏎️throughput/🟦️.ts` (3 pass, fixture `🔣️.json`): merged cfg equals the real `rustc --print cfg` with those features (rustc is the oracle); admission hands exactly the not-remembered units to `prefetch` once, first; analysis is shared per cfg and never across cfg.
- Pre-existing failures, not from this change (also present with the peer's uncommitted `closures` dedupe): receiveScriptProcessInvocation tests need the repo envelope under bare `bun test`; `createPhaseLedgerV1 is not defined`; the recheck-sequence test expects reads for identical units that the peer's dedupe now skips; shared-engine fixture `NativeSharedSlots`.

## Files
`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/`: `🟦️.ts`, `🎟️admission/🟦️.ts`, `🎟️admission/📖️read/🟦️.ts`, `🎟️admission/📖️read/🧬️schema/🔣️.json`, `🎟️admission/📖️read/🧪️tests/{🟦️.ts,🔣️.json,🥒️refusal.feature}`, `🎟️admission/🧪️tests/🏎️throughput/{🟦️.ts,🔣️.json}`. Ticket: benchmarks `source-admission-bench.ts`, `source-admission-baseline-read.ts`, `syscall-count.ts`, `tok-bench.ts`, `cfg-bench.ts`, `closure-bench.ts`, `select-bench.ts`, `memo-miss-probe.ts`.

## Follow-up (coordinator round 2)

### (1) Admission memo: cause proven, fixed
- The racy-file hypothesis is disproven for this worktree: remembering all 503 stored closures against their current files succeeds 503/503 (`racy-probe.ts`; youngest probed file is 16 min old, so none would be refused now). Newest memo on disk is 2026-11-18 16:01, i.e. about 18 h, not 2 h.
- Real cause: the fem component build never uses the memo. `runRepositoryComponentDriverV1` passes no memo to `runNativeBuildDriverV1`; the component pass admits through `admitCargoSelectionV1`, which called `admitCargoSourceBoundariesV1` with no 6th argument, so nothing is recalled or remembered (also `buildSchedulerV1` and the relay, not changed). Only repo cargo and native-artifact paths pass `repositoryCargoSourceMemoV1`. This matches the live numbers: 4133 reads for 2306 files means every unit ran cold.
- Fix: `admitCargoSelectionV1` takes `sourceMemo`; `NativeBuildDriverOptionsV1.sourceMemo` feeds it; the driver passes `repositoryCargoSourceMemoV1(store)` (exported from the typescript cargo package). Wiring test `🧩️component-pass/🧪️tests/🧠️memo-wiring/🟦️.ts` (AST: driver, pass, selection, unit admission).
- Racy files properly: `unit-cache/📖️admission/🟦️.ts` no longer refuses a unit with a recently touched file. Such a stamp is stored `racy:true` (schema updated) and the memo records the digest of the bytes read; recall re-hashes only racy files and misses on mismatch. A racy path without a digest is still refused. Tests: `📖️admission/🧪️tests/🏁️racy/` (fixture `🔣️.json`, sha256 oracle from Bun's hasher; same-size and longer rewrite under an unchanged coarse stamp both miss), existing memo test updated.
- Effect on the stored memos now: 443 of 503 units recall in 5.5 s total on this busy box (`recall-bench.ts`), skipping 31.1 MB of Source reads and every cfg spawn for them; the 60 others were genuinely edited and are re-admitted once and then remembered.

### (2) Token reuse in the shared Rust syntax module
`📚️compiler/📖️syntax/🦀️rust/🟦️.ts`: `rustTokens` memoizes the token stream by source text (8 Mi characters cap, sources over half the cap bypass it); each call returns its own array of the shared immutable tokens; `rustTokenPairs` memoizes the pairing per issued array and only while the array still equals the issued one (edited copies are paired fresh). Delimiter fault, cfg select, reference inspection and the module scope walk now share one stream per distinct text (source and selected text: 2 tokenizations instead of 4, 2 pairings instead of 4). On 404 real files (5.4 MB): delimiter fault 1295 to 179 ms, tokens+pairs 1256 to 219 ms, inspect 2427 to 926 ms (warm); select+inspect+tokens 6.35 s to 3.68 s end to end (0.85 to 1.47 MB/s); closure analysis of 200 real units 5.3 s to 0.7-2.6 s. New test in `🧪️tests/🟦️.ts` (9 pass incl. the existing tree-sitter oracle tests): shared tokens across equal text from distinct strings, own arrays, pairing equal to an independent stack pairing, edited copy not served the shared pairing, warm under half of cold.

### (3) `createPhaseLedgerV1 is not defined`
Not a broken production reference: `native-build/🟦️.ts` imports it and uses it in `withCargoArtifactPlanV1`. A peer split that function into a ledger wrapper plus `withCargoArtifactPlanLedgeredV1`; the memo test extracted the wrapper by AST and ran it with stubs lacking the ledger. Fixed in `📥️source/🎟️admission/🧪️tests/🧠️memo/🟦️.ts`: it now extracts `withCargoArtifactPlanLedgeredV1` and stubs `phaseLedgerV1`/`measureV1`; the test passes.

### Test state
Pass: read 6, admission throughput 3, memo wiring 1, racy 2, unit-cache admission memo test, syntax 9. Remaining failures are not from this work: tests that need the repo script envelope under bare `bun test` (e.g. component-pass suite, 10 of them), the `Source refused` message text check, the peer's `closures` dedupe vs the recheck-sequence fixture, `NativeSharedSlots` fixture, and load-dependent unit-cache compiler-wrapper tests.
