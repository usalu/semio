# Native cargo wasm fallback (2026-11-27)

## Profile
`native-profile.toml` (ticket folder) is the effective `prototype` profile that repo injects, extracted from `.cache/b/r/m/configuration/b6abe340.../view.json` (wasm32-wasip2 beams component build): `inherits=dev, opt-level=s, debug=0, debug-assertions, overflow-checks, codegen-units=16, lto=off, incremental=false`, build-override `opt-level=0 codegen-units=256`, plus 100+ per-package `opt-level=3` overrides. Target rustflags (`--max-memory`, `-zstack-size=8388608`, `-Z threads=0`) and `[unstable]` come from `.cargo/config.toml` unchanged.

## Command (`NATIVE=1 sh native-wasm-build.sh`, ONLY / ONLY_HUB select packages)
`cd <root> && env -u RUSTC_WRAPPER CARGO_TARGET_DIR=/Users/ueli/t/src-wasm CARGO_BUILD_BUILD_DIR=$CARGO_TARGET_DIR/b CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_INCREMENTAL=0 SEMIO_BUILD_MODE=prototype cargo build -j 4 --config native-profile.toml --profile prototype --manifest-path <m> -p <pkg> --lib --target wasm32-wasip2`; the final wasm is copied to `🗑️generated/native-wasm/`. `rebuild-sourcing-retry.sh` passes `NATIVE` through. Logs: `🗑️generated/native-wasm-<pkg>.log`.

## Timings and results (no compile errors; -j 4 sufficed, no OOM)
- hub `semio_hub_sourcing_component.wasm` 293913536 B, about 27 min cold (log was deleted by a peer mid-build; the build kept going).
- beams 52754687 B, 1080 s. slabs 52753913 B, 1105 s. windows 52751550 B, 1150 s.
- Staging, restart of 6081 and journeys via `NATIVE=1 sh rebuild-sourcing-retry.sh ""` (fresh wasm present, 2862 s total, log `native-wasm-journeys.log`).
- Descriptors: beams and slabs descriptor and json refreshed in their `🧩️component` folders; windows and hub descriptor identical; hub `🔣️.json` refreshed.
- `/Users/ueli/t/src-wasm` deleted at the end (disk fell to 13 GB free).

## Journeys (`🗑️generated/all-journeys/summary.txt`)
journey exit 1 (25 pass / 9 fail, all `de`), editing exit 0 (40/0), history exit 1 (44 / 68 fail; `de` click timeouts waiting for the `Pool` tabpanel, 16 `en` fails), surfaces exit 1 (55 / 13 fail; `de` plus the en "settings language switch" test). The failures concentrate in German labels (the tests look for English accessible names such as `Pool`), so the fresh components work and the rest is a locale or test-selector issue, not a build issue.
