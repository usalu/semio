# Source Boundary Probe Parity

## Cause of the disagreement

The old `native-source-boundary-preflight.ts` reimplemented a thin slice of admission. Against `admitCargoSourceBoundariesV1` (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts`) it differed in five ways:

1. **Cfg.** It called `readRustSourceClosureV1` with an empty cfg. Admission passes the real `rustc --print cfg` merged with the unit's features (`mergeCargoCfgFeaturesV1`). Every `#[cfg(target_pointer_width = ...)]`, `cfg(unix)` or `cfg(feature = ...)` module mount was therefore skipped by the probe and read by admission. This is why the snapshot crate (16 files, 263979 bytes) looked within limits.
2. **Features.** The probe enabled none. Admission's units carry the crate's feature set.
3. **Per-read budget.** Admission asks the physical reader for `maximumBytes = maxSourceBytes - used`. A file crossing the remaining budget raises `process.source.read-bytes-exceeded` (observed 54421, maximum 52586 for the window file), and `sourceBoundaryRefusalV1` wraps it as `process.source.boundary-exceeded {crate, files, maxFiles, bytes: used + observed, maxBytes}`. The probe only compared the totals afterwards and never reported the read-level refusal.
4. **Units.** Admission also admits the build script of a crate. The probe looked at the lib only.
5. **Limits.** The probe never checked `maxActiveLines`. It also parsed `[package.metadata.semio.rust-source-boundary]` with a regex instead of TOML.

## Fix

`native-source-boundary-preflight.ts` now builds the same workspace `CargoCompilationUnitV1` rows admission receives (lib plus build script, default-feature closure, boundary from the manifest TOML via `Bun.TOML`). It then calls the real `admitCargoSourceBoundariesV1` per crate, with:

- the real `createPhysicalCargoSourceReaderV1` (no-follow reads, per-read `maximumBytes`/`remainingFiles`),
- `rustc --print cfg` (release: `-C debug-assertions=off`; `PREFLIGHT_PROFILE=debug` and `PREFLIGHT_TARGET=<triple>` override) merged with features via the real `mergeCargoCfgFeaturesV1`,
- the repository policy (`sourceBoundary` in `caching/🔣️policy.json`).

No measurement or limit logic is reimplemented. Each refusal prints the error admission throws:

`OVER<TAB>crate<TAB>process.source.boundary-exceeded {crate,files,maxFiles,bytes,maxBytes}<TAB>cause key + values (read-bytes-exceeded / read-files-exhausted, or -)<TAB>manifest`

The active-lines refusal prints admission's own message. Other refusals print `ERROR<TAB>manifest<TAB>key values`. `native-play.sh boundaries` greps `^OVER`, so it needs no change. Because the code calls admission itself, a later change to admission is picked up automatically.

Residual gap: cargo metadata cannot run outside the repo's rustc ingress, so features are the crate's own default closure. Features that dependents enable (Cargo feature unification) are not modelled.

## Test

`native-source-boundary-preflight.test.ts` (bun test, synthetic crates in `🗑️generated`, removed afterwards). Each case first asserts the old probe's count (empty cfg) is within limits:

1. A lib of about 200 kB mounting a 70 kB module behind `cfg(target_pointer_width = "64")`. Expected: the `boundary-exceeded` line with the `read-bytes-exceeded` cause and the exact observed and maximum bytes.
2. 32 modules mounted behind a default feature. Expected: `files: 33, maxFiles: 32` with the `read-files-exhausted` cause.
3. A crate-authored boundary (`maxSourceBytes = 8`) is honoured.

Result: 3 pass, 0 fail.

Run:
`bun test ./.🧬semio/.../BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/native-source-boundary-preflight.test.ts`

## Run against `✏️s/🔌️plugins/🧩️puzzle`

`snapshot/📦️packages/🦀️rust/🦀️.rs` was rewritten at 06:26 by the concurrent split, so the crate now passes admission and the probe reports nothing for it. I could not reproduce the original 263979 bytes. With tightened policies the probe reports admission-shaped numbers on the current tree: 7 files at `maxFiles 6` (28945 bytes counted), and 161146 bytes at `maxBytes 150000` (file `🪶️sqlite/📸️snapshot/🦀️.rs`, observed 34533, maximum 23387). A full run over `✏️s`, `🌎️hub` and `🧰️framework` takes about 26 s and prints no `OVER` or `ERROR` line.

## Files

- `native-source-boundary-preflight.ts` (rewritten; exports the pieces the test uses, main guarded by `import.meta.main`)
- `native-source-boundary-preflight.test.ts` (new)
- `boundary-probe-parity.md` (this report)
