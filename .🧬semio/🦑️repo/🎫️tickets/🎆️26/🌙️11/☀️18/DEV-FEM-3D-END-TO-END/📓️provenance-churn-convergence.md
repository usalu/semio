# Provenance churn convergence

## Where the time goes

Source: `🗑️generated/attempts/_semio-tech_os-plugin-describe-rs_build-1795022990406.txt` (started 18:29:50, ended with exit 75 at 19:09), phase ledger at its end (total 2280 s):

| phase | seconds | note |
| --- | --- | --- |
| provenance | 1274.6 | 37,861 file operations, 2.79 GB hashed (units' outputs, staged copies, dep-info, dep-info sources) |
| consume | 511.1 | retained compilation copy, source digests 4.8 s |
| cargo | 299.6 | real compile of 321 units (not fully cached) |
| plan:facts + recheck:facts | 11.6 + 31.2 | 8.5 MB of planning records, read 4 times |
| plan/recheck graph, sources | about 51 | cheap |
| setup | 42 | |

Provenance dominates (56 percent). Each file is hashed once, then again in the final `recheck`, and nothing outlives an owner: `CurrentPhysicalFilesV1` (the claim custody) is created per `writeCompletedCargoInvocationProvenanceV1` snapshot attempt (`physical()` makes a fresh owner each time), per build-layer attempt and per restarted process. The "build" profile wraps planning, cargo, provenance and consumption (`withCargoArtifactPlanLedgeredV1`, `retryOnConcurrentEditV1("build", ...)`), so one edit of `key-frame.rs` anywhere in that window re-runs all of it, and exit 75 restarts the process and re-hashes 2.8 GB.

No existing per-file memo was reusable: `🕸️units/📥️capture/🧠️memo` memoizes Cargo planning answers (text/presence/list questions, racy window), `sourceMemo` serves source admission; neither covers file identity to digest for provenance.

## Change

- New `📁️filesystem/🧾️observation/📁️current/🧠️memo` (schema first: `🧬️schema/🔣️.json`, `DigestMemoV1`, `digestMemoV1(directory)`): maps absolute path plus identity (dev, ino, size, mtimeNs, ctimeNs) to sha256 and byte length. Shared per directory in the process, persisted atomically as `digests.json` (flushed every 2048 entries and on process exit, including the exit 75 path). A digest is remembered only when mtime and ctime are older than the racy window (2000 ms, schema) at hash start, so a same-second rewrite is never mistaken for old bytes.
- `CurrentPhysicalFilesV1` takes an optional memo. `CurrentPhysicalOwnerV1.measure/digest` answer from it (no body read counted) and remember what they hash; `recheck` skips re-hashing a file whose identity equals the observed one and whose memo entry agrees, and still re-hashes (and fails on) anything touched. `memoizedPhysicalFilesV1(buildDirectory)` builds the custody; the memo file lives in `<buildDirectory>/semio-digest-memo/`.
- Wired into the four provenance writers: `native-build/🟦️.ts` (`observeCompleted` owner and `sharedProvenanceCustodyV1`, plus `recheckCompletedFiles`), `native-build/🌊️relay`, `describe/🏭️fresh-component`, repo library `🏃️process`.
- Effect: a snapshot or build retry in the same process, or a restarted process, lstat-checks the 37k files and hashes only edited or racy ones (the peer-edited source plus freshly written cargo outputs older than 2 s).

## Not changed (follow-ups)

- The "build" retry still wraps planning, cargo, provenance and consume. With the memo a retry costs planning (about 50 s) plus lstat sweep rather than 21+ min; narrowing the loop to re-observe only the changed unit needs a restructure of `withCargoArtifactPlanLedgeredV1` (dense one-liners) and was not attempted here.
- Retention copies (`copyCompilationFileV1`, `q/<attempt>`) hash while copying and are repeated per snapshot attempt: the 511 s consume phase is not covered by the memo.
- The 8.5 MB planning-record read (`plan:facts`, `recheck:facts`, about 43 s per attempt) is not memoized.

## Tests

- `📁️current/🧠️memo/🧪️tests/🟦️.ts` (bun, 5 pass): racy decisions from the language-agnostic `🔣️.json` equal the Python oracle `🐍️.py` (hashlib digests of the fixture files also compared), exact-identity lookup, restart persistence and damaged-file refusal, a retried owner reads only the peer-edited file and its digests equal hashlib, recheck skips unchanged remembered files and still rejects a changed one. Registered in `🧾️observation/📜️script.ts` (test and tsc lists).
- Existing: `runCurrentFileRecheckChecksV1` passes (10), tsc strict clean for `📁️current`; receipt tests 22 pass, 1 fails (`...repeated from a fresh owner...`: its sleep list shows a quiet-window wait of about 1975 ms because the test does not stub `modifiedAgeMs`; unrelated to the memo, the test file has peer edits). Churn test file: its Python-model scenario test fails in this shell, unrelated.

## Round 2: converging build layer and planning facts

Measured by the coordinator after the memo: attempt about 16 min (cargo about 5 min, provenance 473 s); any peer edit in that window still exhausted the 4 build attempts and ended the command with exit 75.

### Build layer converges inside the process

- Schema (`🔁️churn/🧬️schema/🔣️.json`): `convergence.wallClockMilliseconds` = 7,200,000 (2 h).
- `retryOnConcurrentEditV1` / `...SyncV1` take `converge: true` (and a test clock `now`). For the `build` profile such a call is bounded by the wall clock instead of 4 attempts; the pause is the last declared backoff (30 s) once the list is used up; the announcement reads "attempt N of ∞". Without `converge` nothing changes (the Python-model scenarios are untouched). Exhaustion after the wall clock fails as before with the changed path, so the outer restart still exists as the last resort.
- Opted in: `withCargoArtifactPlanLedgeredV1` (native-build) and `freshRun` (describe fresh-component).
- What a round now costs: re-plan (about 50 s), incremental cargo (rebuilds only what the edit invalidates), provenance where the digest memo answers every unchanged file by identity and the final recheck skips them, so only the edited files are re-hashed. Progress bound: the memo makes each round touch only files changed since the previous round; the wall clock is the hard stop.
- Coherence: unchanged. Every round re-plans, re-checks (`recheck()` of recipe, graph, sources, facts) and the receipt/exe is published only after the final recheck passes; a staged exe of a failed round is discarded and re-staged by the next round. A file edited inside the 2 s racy window is never answered from the memo.

### Planning facts

`captureCargoArtifactPlanFactsV1` (`📋️plan`, the "cargo:artifact-plan" phase) reads no files: the 8.5 MB are the in-memory graph hashed token by token with a lease check per token and a progress publish plus yield every 64 tokens (about 50k publishes per pass, 4 passes per attempt, 43 s in the ledger). It now builds canonical text synchronously below the unit level and hashes it in 256 KiB chunks with one checkpoint per chunk; the digest is byte-identical (the existing independent-oracle test `📋️plan/🧪️tests/🧵️control` passes). A file memo does not apply there; there is no per-file read to cache.

### Tests

- `🔁️churn/🧪️tests/🟦️.ts`: new test (fake clock) that a converging build retries past 4 attempts until the 2 h clock, uses the last pause, announces "∞", succeeds when a later round is clean, and that `converge:false` still stops at 4. 14 pass; the one failure (Python-model replay, "german" language protocol) was already failing before.
- Plan tests 17 pass, plan control test 2 pass, tsc strict clean for churn and plan.
- Not run: a real describe build (not started, per constraints).
