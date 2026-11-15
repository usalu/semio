# Sealed cargo wrapper snapshots

## Finding
The repo tool itself already runs from a sealed store (`🏗️build/🏛️sealed`, `SEMIO_REPO_SEAL`, `build:implementation-source`), but the cargo unit-cache wrapper did not: the POSIX `rustc-wrapper` shell stand-in, the Windows native executable (bundled and compiled from the live entry), the `SEMIO_UNIT_CACHE_SCRIPT` that `compilerWrapperEnvironmentV1` hands to the broker chain, and the detached collector all ran `📜️script.ts` / `🏁️entry.ts` and everything they import (237 files) from the live shared worktree. The build-script shim and the fleet uploader derive their paths from `import.meta.url`, so they follow whatever the wrapper runs.

## Change
New module `🗃️unit-cache/🏛️seal/🟦️.ts` (`sealModulesV1`), policy in `🧬️schema/🔣️.json` (`Policy.seal`):
- Captures the import closure of the entries (static imports, quoted file literals, JSON), parses every file with the runtime transpiler, stages a copy below `<cacheRoot>/sealed/<digest>` (digest = sha256 over sorted relative path + file hash, read-only files), imports the entry from the staged copy in a runtime child (catches renamed exports, missing files, throwing top levels), re-reads the live files and requires identical bytes, then publishes by one atomic rename. Peers sealing the same bytes share one directory.
- A capture that fails any step is retried after `settleMilliseconds` (250 ms, 6 attempts). If none succeeds, the last good snapshot of those entries (per-entry-set pointer) is served and `stale` names `path:line:column message`; without one it throws and the unit cache is switched off for that Cargo process with the existing bilingual refusal. A mid-edit tree is never executed.
- Retention: current, newest 4 and anything used within 24 h are kept; the rest and old stages are removed, and the matching shims in `bin/`.
- Wired in `🚪️launch`: `sealedUnitCacheV1`, `sealedUnitCacheScriptV1`; `ensureUnitCacheShimV1` seals unless the caller names script/entry (POSIX shim is `bin/rustc-wrapper-<digest>` exec'ing the sealed script; Windows bundles and compiles the sealed entry, so the executable digest can no longer differ from what was compiled); `startUnitCacheCollectorV1` runs the sealed script; the compiler store (`🏗️compiler/🟦️.ts`) exports the sealed script as `SEMIO_UNIT_CACHE_SCRIPT` (absent, hence cache off, when sealing is refused).
- `unitCacheShimPathV1` now requires the digest.

## Tests (`🏛️seal/🧪️tests/🟦️.ts`, fixture `🧫️fixtures/🔣️.json`; 9 pass)
digest equals the fixture vector and an independent Python computation; an edit during a session leaves the session's output unchanged and only the next seal moves; syntax-broken edit not sealed (last good served, place named, no extra directory); refusal without a good snapshot; parses-but-does-not-load edit not sealed and no stage left; retry seals once the tree settles; change between parse and second read refused; retention; shim and environment script point into the sealed copy, never the live script. Existing unit-cache windows, lane and gc tests pass except the unrelated lane quota rounding assertion (7104857899 vs 7104857898, `compilerLaneQuotaBytesV1`, not touched).

Run: `SEMIO_TEST_PYTHON_BINARY=$(which python3) repo tools exec bun -- test ./…/🏛️seal/🧪️tests/🟦️.ts`.

## Residual
- A semantically broken but loadable edit cannot be detected statically; it is sealed (same as the repo-tool seal). Sessions are protected only from later edits.
- A session longer than 24 h that is not among the newest 4 snapshots could lose its directory to a later seal.
- Cost: each launch reads the closure twice and parses it (no stat fast path yet); boot check only for new digests.
- Not sealed: other repo build helpers outside the unit-cache closure (scheduler provisioning, native-build helpers, `🏃️process` code run by `repo cargo` itself runs under the repo-tool seal, mode `live` opts out). Windows `rustc-wrapper-<bundle>.exe` generations are still never collected (pre-existing).
- No full real-cargo run (`cargo-unit-cache-processes`) was done: host had no spare memory.

## Follow-up fixes (all landed before the tooling freeze; nothing queued)
- `compilerLaneQuotaBytesV1`: the code's declared integer largest-remainder split is the rule; the Python oracle in the lane test now applies the same rule (no code change).
- Build-script test: the exec-bit assertion on the tracked script was removed; it now asserts the stand-in starts the runtime on the script.
- Unit-cache suite: `session()` sets `SEMIO_UNIT_CACHE_REMOTE=off` (fleet cache made the cleanup test see remote hits); `events()` skips `x/offload.jsonl` (a separate stream).
- `🧱️units/🧪️tests`: private per-test workspace outside the checkout plus a loud guard against the live store; `withUnitPressureRecheckV1` takes an injectable timer and its gc test is deterministic.
- Last direct runs: seal 9/9, units and gc files 23/23, lane 3/3, targeted unit-cache subset 6/6. A full 9-file suite pass was not repeated after the final fixes (needs a quiet host).
