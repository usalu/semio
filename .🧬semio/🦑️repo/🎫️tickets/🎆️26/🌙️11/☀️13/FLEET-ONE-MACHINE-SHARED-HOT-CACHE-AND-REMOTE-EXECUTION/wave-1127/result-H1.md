# H1 result: workspace-stage failures on g1-m01, m03, m08, m14 (2026-11-27 00:15)

No code edit was needed. All causes were transient peer half-edits or capacity waits, and the tree is consistent now.

- m01/m14 (Nx exit 130): the real error is `E0425 cannot find type OriginalCancelRootIdentity in the crate root` while compiling `semio-framework-job-context` (referenced by `job-context/🎟️source/🏭️admission/🦀️.rs:17`).
  - A peer had not yet re-exported the type in `job-context/🦀️.rs` (last edit 23:22) and `job-context/🪟️facade/🦀️.rs` (23:42). The failures were at 23:31 and 23:34.
  - Both re-exports exist now, and `semio_framework_async` exports the type. Exit 130 is Nx killing its sibling tasks (`generate:prototype`, `typegen:prototype`, `build:prototype`) after the first cargo failure. I found no SIGINT from the warden or a stall guard in the logs.
  - Not verified: a cargo check of `semio-framework-job-context`. Load was 60+, so I skipped it.
- m03 (`nativeImageShardMountedV1 is not defined`): the authorizer bundle `nx-authorizers/60fe0e...` on m03 was built from a mid-edit snapshot of the `native-build/.../image/shard/🟦️.ts` source. That file was edited again at 23:52.
  - The bundle digest is content-addressed on the sources, so a rebuild yields a new bundle. Today every consumer imports the function and the shard module defines and exports it. A local `bun build` of the budget `images/🟦️.ts` entry resolves and defines it.
  - Not verified: whether m03's own retry builds a fresh bundle. Its ready.json tail still shows the old error, and the last retry was due around 00:10.
- m08/m14 (running, not failed): both are waiting for host-capacity grants on 16 GB hosts.
  - m08: the project-graph-slice read asks for a non-shrinkable 5918 MB minimum, and the gate reports 0 to 574 MB spare above the 2048 MB floor. Earlier `run-many` requests (2663 MB each) are queued ahead of it.
  - m14 waits on a `run-many` that is short by about 700 MB.
  - Not fixed. This is the 16 GB capacity-sizing problem, not a workspace-stage bug. Candidates: the graph-slice minimum, and queue entries left by dead attempts. The relevant code is under `🌡️host-capacity/` and the sidecar `🚦️ready/`.
- Not verified: no worker's ready.json flipped to ready within my window.

## Round 2 (00:40)
- `repo cargo check -p semio-framework-job-context --lib` (prototype, shared store): exit 0 after 14 min under load 60. This confirms the E0425 cause is gone. m01 is Ready (ready.json `ready:true`, refreshing).
- The `source-transient` failure class already existed (wave 1125 F1: classification, tree-stamp debounce 15 s, EN/DE refusal). Its patterns missed the three failures seen here, so they never took the fast path.
  - Added to `sourceTransientPatterns`: `^ReferenceError: \w+ is not defined`; rustc `error[E0…]: cannot find (type|value|…)`; `Bundle failed: No matching export in "semio-captured-source:` (this was m03's newest failure at 00:33, `nativeImageVolumeUsedBytesV1`); and the new bundle-changed refusal below.
  - Test: `🚦️ready/🧪️tests/🧱️source-transient/🟦️.ts` has 9 new cases checked against the Python oracle. 4 of 4 pass.
- Systemic gap: `nxAuthorizerSourcesHoldV1` existed but nothing called it.
  - `ensureNxAuthorizerBundleV1` (`⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts`) now re-captures the authorizer and compiler source closures after compile and preflight, before the atomic rename. If either differs, it throws `concurrentEditV1("bytes", …, "Authorizer sources changed while the … bundle was built, it is not published", "build")`. Staging is removed, nothing is published, and the supervisor classifies it as source-transient and retries.
  - Test: `🛠️tools/🧪️tests/🧱️bundle-settled/🟦️.ts`. `repo typecheck` of both tools files passed (0 errors).
- Landing is in `landings.md` ("Wave 1127 H1"). Script: `🧰️scripts/H1/source-settle.py`.
- Not verified: m03, m08 and m14 are still not Ready. m03 was in backoff on the `No matching export` bundle error, though that export has existed in the tree since 00:14, so its replica or bundle lagged. m08 and m14 are waiting on host-capacity grants (swap crowded). The final state is in the coordinator's message.
- Not verified: the bundle guard has no live end-to-end test. It is covered by the placement test only, because racing a real capture is not safe live.

## Round 3 (00:50): m03 `No matching export … nativeImageVolumeUsedBytesV1`
- Not replica lag. m03's replica cursors equal m01's (segment 92, 696976 bytes), and the attest `manifest` digest is identical on m01, m03 and m08.
- The importer is legitimate: `🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts` (edited 00:15:07) imports `nativeImageVolumeUsedBytesV1` from the `⚖️availability` package index. That index (edited 00:14:15) exports it, and the artifact file defines it (00:14:10).
- Reproduced locally with the fleet Bun 1.4.2 (`.cache/b/t/m/bun/…/bun`). A bundle of both authorizer closures (`📦️runtime/🟦️.ts`, `📜️script.ts`) on the current tree succeeds (7.4 MB and 13.4 MB). The current sources are therefore consistent, and m03's failing bundle input differs from them.
- Cache keys, read through:
  - The bundle directory is content-addressed on the captured source digests, so a bundle cannot outlive its sources by key.
  - The source memo is validated by file digest, and a changed file is re-read.
  - The capture reads disk directly with a descriptor stamp check.
  - I found no stale-key path in this code.
- I cannot read m03's disk. I therefore landed a diagnostic and a recheck, not a guessed key change (see the next three bullets).
- Fix, `⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts`:
  - Any `Bundle failed` or `compilation refused` error during bundle publication is rechecked. If a capture no longer holds against the tree, it becomes the concurrent-edit refusal and is retried as source-transient.
  - Otherwise the message gains `[captured against disk: <path> captured sha256 …, on disk …]` for every module the error names. The next m03 log will show whether its captured bytes of the index or the runner differ from its disk, which decides between a stale snapshot and a divergent tree.
- Tests: `🛠️tools/🧪️tests/🧱️bundle-settled/🟦️.ts` (3 pass: evidence format, the guard before publish, and the catch routing). `repo typecheck` of both files passed.
- Not proven: m03 has not been seen Ready, and its failure persists as of 00:45. The coordinator should read m03's next `ready-log-workspace.txt` for the `captured against disk` line. If captured and on-disk digests differ with no concurrent edit, the capture or memo on m03 is stale. If they match, m03's tree content differs from the manifest digest, i.e. replica corruption of one file, and a `repo` repair or resync of that path is needed.
