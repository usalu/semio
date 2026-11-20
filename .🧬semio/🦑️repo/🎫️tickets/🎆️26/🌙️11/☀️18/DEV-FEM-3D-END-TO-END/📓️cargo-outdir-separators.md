# Cargo out_dir mixed separators

Cargo's new build-dir layout reports `build\<crate>/<hash>\out`; `canonical()` requires `resolve(v)===v`.

## Change
`📦️retention/📋️inventory/🟦️.ts`: new `reported(value)` (resolve when absolute string, then `canonical`) applied to every cargo-reported path: build script `out_dir`, unit `artifacts[].path` (from `filenames`), `depInfo[].path`, the `eligible` artifact set in `completedCargoOutputClaimsV1`, and the existing `tracked` resource-source normalization. Everything else (invocation inputs, resource read paths, origin roots) is still strict `canonical`. The reader `📥️reading/🟦️.ts` needs no change: it only recomputes the roster through the same function.

## Test
`📦️retention/🧪️tests/🟦️.ts`: "accepts mixed separator cargo paths and records their normalized form" (out_dir and a bin artifact spelled with `/` and `sep`).

## How the tests run
They need an issued envelope with an `artifacts` capability and cwd = checkout root (suite `native-build`, `🧪️testing/🗂️suites/🔣️.json`, run via `repo ./📜️script.ts test native-build`, which covers the whole tree). Single file: `bun retention-run.ts "./<file>" [bun args]` in this ticket folder (issues the envelope, sets `SEMIO_TEST_ARTIFACT_DIR`, cwd root).

## Results
- Plain `bun test ./🟦️.ts` in the tests folder: 11 pass / 8 fail (19 tests, with the new one), the failures are missing envelope (enqueue, 🧪️testing/📥️invocation:91) and cwd-relative paths.
- Under the envelope from the root: 16 pass / 3 fail. The remaining failure is `copies exact real descriptor bytes` (`copied.mode` 0o666 vs expected 0o640, Windows has no POSIX mode); the other two are its cascade (the shared Original closes after the first failure, the proveX steps never run).

## Descriptor copy mode
Production (`📁️copy/🟦️.ts`) intentionally preserves the source's `mode & 0o777` (open, chmod, returned `mode`); Windows has no POSIX modes (source stats as 0o666), so the code is right and the 0o640 assertion was POSIX-only. The test now asserts `copied.mode` equals the source and destination stat modes everywhere, 0o640 only when `process.platform!=="win32"`, and the writable bit on Windows.

Re-run with retention-run.ts: 18 pass / 0 fail (the earlier 3 failures were this one plus its cascade; the earlier "19" included the unnamed cascade entry).
