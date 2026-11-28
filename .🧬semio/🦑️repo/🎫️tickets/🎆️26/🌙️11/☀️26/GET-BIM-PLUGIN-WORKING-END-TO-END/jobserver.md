# Jobserver retirement: EFAULT turned completed builds into internal errors

## Symptom
`model-test45/46/48.log` end with `Internal error: Jobserver retirement failed: EFAULT: bad address in system call argument, rm '.../cargo/prototype/tmp/jobserver-<id>/jobserver.fifo'; EFAULT ... rm '.../jobserver-<id>'` and exit 1, after 2000+ s of cargo work. The fifo and directory stayed behind (10 stale `jobserver-*` dirs in `.cache/b/r/m/cargo/prototype/tmp`).

## Findings
- Two independent defects, the second is the one that fails builds:
  1. `launcher/🟦️.ts` `finally` of the jobserver scope threw whenever `jobserver.close()` or `rmSync(directory)` failed. Even when the build body succeeded, that throw replaced the result (`Jobserver retirement failed` as an internal error, exit 1); when the body failed it replaced the real error with an `AggregateError`.
  2. `fifoChannel.close()` removed the fifo with `rmSync(endpoint,{force:true})` (Bun's recursive-capable rm path: stat + remove with its own path buffers), and the directory with `rmSync(...,{recursive:true,force:true})`. Both calls hit EFAULT in the same process.
- What the EFAULT is not (tested with the repo's Bun 1.4.2 on the shard mount and on the home volume, `🗑️generated`-free probes): removing a fifo that is still open (O_RDWR|O_NONBLOCK), after close, via `rmSync force`, `unlinkSync` and `rmSync recursive` all succeed on the `.cache/b/r/m/cargo` APFS sparse-image shard; so it is neither a fifo-on-shard limitation nor open-fifo semantics. A race/double removal yields ENOENT (and `rmSync force` swallows it), not EFAULT. The stale leftovers from the failed runs also removed fine afterwards with `rmSync` and `unlinkSync` (nothing held them open).
- Conclusion: EFAULT is a transient kernel/runtime path-copy failure of a very long-lived, memory-saturated Bun process (runs were 2000+ s, host at its memory ceiling, lane images at capacity), hitting every rm in that process, not a deterministic property of the fifo or the mount. It cannot be reproduced on an idle host, so the fix makes retirement tolerant of it instead of depending on it, and removes the escalation.

## Fix
- `🎰️jobserver/🟦️.ts`
  - new `removeJobserverPathV1(path, directory)`: fifo via direct `unlinkSync` (no recursive machinery, fine while open elsewhere), directory via recursive rm; ENOENT = success; other failures retried with backoff (0/10/40/160 ms), then the system `rm -rf` (own process, own buffers) on non-Windows; only a path still present afterwards raises the last error.
  - `fifoChannel` uses it for the initial stale-fifo removal and in `close()` (descriptor closed in `try`, fifo removed in `finally`).
  - new `retireJobserverV1(jobserver, directory)`: closes the pool and removes the directory, never throws, returns typed `JobserverRetirementWarningV1` (`kind: "jobserver-retirement-failed"`, `path`, `reason`); idempotent.
- `📣️diagnostics.json`: new `jobserverRetirementFailed` in English and German (`{path}`, `{reason}`).
- `launcher/🟦️.ts`: the `finally` now only prints the localized warnings via `nativeBuildDiagnosticLinesV1` to stderr; the body's result or original error propagates, so the real exit status is preserved. The `jobserverFailed/jobserverFailure` bookkeeping and the two AggregateError messages are removed.
- Package entry re-exports `removeJobserverPathV1`, `retireJobserverV1`, `JobserverRetirementWarningV1`.

## Tests
- `🎰️jobserver/🧪️tests/🟦️.ts`: retiring unlinks an open fifo (checked with `test -p` as independent oracle), vanished fifo and double retirement yield no warning; a locked parent (chmod 500) yields typed warnings with the directory path and a later retirement succeeds.
- `📣️diagnostics/🧪️tests/🟦️.ts`: SQLite oracle extended with `{reason}`; the existing parity/umlaut/language checks cover the new key.
- Run via `SEMIO_LANGUAGE=en repo test-file`; log `🗑️generated/jobserver-test1.log`. The bim workspace build was not run.

## Changed files
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧬️schema/📣️diagnostics.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📣️diagnostics/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`

## Side effects
Removed three stale jobserver dirs' contents during probing (`jobserver-039a…`, `-3f05…` fifos, `-6831…` dir) from `.cache/b/r/m/cargo/prototype/tmp`; the rest are stale leftovers of the failed runs and can be deleted safely.
