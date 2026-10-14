# 📓 Compile Baseline 2026-10-14

Audit run, no source edits. Commands ran with `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` pointed at the private `⚡️cache/bl` directory, through `🚦️cargo-slot.sh baseline --`, with `--keep-going --message-format=short`. Logs are in `🗑️generated/baseline/` (`check1-hub-wasm.log`, `check2a-plugin-sdk.log`, `check2b-os-kernel.log`, `timestamps.txt`). Each check ran once to completion, so no resume was needed.

Error counts are the number of `: error` lines. Build-script failures (`failed to run custom build command`) are listed separately and are not in those counts. Warning counts are the number of path-located `: warning` lines. Error groups are attributed to the `could not compile` line that follows them in the log.

## Summary

| # | Check | Start-End | Result | Errors | Warnings |
|---|-------|-----------|--------|--------|----------|
| 1 | hub, `wasm32-wasip2`, `--profile wasm-release` (workspace) | 16:49:09-16:53:16 | FAIL (exit 101) | 46 (+2 build scripts) | 92 |
| 2a | plugin SDK `--lib` (native) | 16:53:43-16:56:38 | FAIL (exit 101) | 38 | 88 |
| 2b | os-kernel `semio-framework-os-kernel` `--lib` (native) | 16:56:44-16:57:17 | FAIL (exit 101) | 21 | 82 |
| 3 | `manifest-sweep.sh` | - | SKIPPED | - | - |

Profile note: `wasm-release` exists in `🌎️hub/Cargo.toml`, so it was used as specified.

Manifest paths:
- plugin SDK: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`
- os-kernel: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` (package `semio-framework-os-kernel`)

Manifest sweep: skipped. `🔎️manifest-sweep.sh` hardcodes `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/sweep/target` and writes `🗑️generated/manifest-sweep.tsv`. It does not honor the private `bl` target dir, so it was not run.

## Check 1: hub, wasm32-wasip2, wasm-release (FAIL, 46 errors, 92 warnings)

Path prefix `🧰️framework/...` is shortened to the crate dir. Line numbers are the first occurrence in log order.

| Crate | Code / cause | Count | First file:line |
|---|---|---|---|
| semio-framework-ui-theme | E0433, `ui_contract` unresolved | 9 | `🖱️ui/🎨️styling/🎭️theme/📦️packages/🦀️rust/../../🧩️component/🦀️.rs:373:13` |
| tokio 1.52.3 | `Only features sync,macros,io-util,rt,time are supported on wasm` (a dependent enables an unsupported feature) | 1 | `tokio-1.52.3/src/lib.rs:478:1` |
| semio-framework-canvas | E0117, orphan rule (foreign trait for foreign type) | 2 | `🖼️canvas/📦️packages/🦀️rust/../../🌉️icon-name-value/🦀️.rs:10:1` |
| semio-framework-job | E0502, `*cx` borrowed mutably and immutably | 2 | `🧵️job/📦️packages/🦀️rust/../../⏱️context/📸️capture/../🎟️source/👶️child/🦀️.rs:13:576` |
| semio-framework-os-store-message-ledger | E0405, `ToValue` not in scope | 1 | `🏪️store/📒️ledger/📨️messages/📦️packages/🦀️rust/../../🦀️.rs:504:6` |
| semio-framework-os-store-message-ledger | E0425, `DslValue` not in scope | 1 | `...messages/📦️packages/🦀️rust/../../🦀️.rs:505:27` |
| semio-framework-os-store-message-ledger | E0433, `DslValue`/`ToValue` undeclared | 2 | `...messages/📦️packages/🦀️rust/../../🦀️.rs:506:9` |
| semio-framework-os-pack | E0433, `serde_json` unresolved (bridge) | 11 | `🎒️pack/📦️packages/🦀️rust/../../🌉️bridge/🦀️.rs:128:34` |
| semio-framework-os-command | E0433, `ToValue`/`Terminology`/`Locale` not in crate root | 4 | `📡️spr/🎮️command/📦️packages/🦀️rust/../../🦀️.rs:859:35` |
| io-lifetimes 2.0.4 and 3.0.1 (registry) | E0658, unstable library feature `wasip2` | 4 | `io-lifetimes-2.0.4/src/lib.rs:52:18` |
| wasmtime-internal-cache 47.0.3 | E0425, `nice` not in `rustix::process` | 1 | `wasmtime-internal-cache-47.0.3/src/worker.rs:254:32` |
| semio-framework-ui-layout | E0433, `semio_framework_ui_contract` unresolved | 8 | `🖱️ui/📜️declaration/📐️layout/📦️packages/🦀️rust/../../🦀️.rs:1900:9` |
| build script wasmtime-internal-jit-debug 47.0.3 | `gdbjit.c: fatal error: 'stdlib.h' file not found` (no wasi sysroot for C) | build script | log line 424 |
| build script libsqlite3-sys 0.36.0 | `sqlite3.c: fatal error: 'stdio.h' file not found` (no wasi sysroot for C) | build script | log line 572 |

Totals: 46 error lines (9 + 1 + 2 + 2 + 4 + 11 + 4 + 4 + 1 + 8), plus 2 build-script failures.

## Check 2a: plugin SDK, native `--lib` (FAIL, 38 errors, 88 warnings)

| Crate | Code / cause | Count | First file:line |
|---|---|---|---|
| semio-framework-ui-theme | E0433, `ui_contract` unresolved | 9 | `🖱️ui/🎨️styling/🎭️theme/📦️packages/🦀️rust/../../🧩️component/🦀️.rs:373:13` |
| semio-framework-job | E0502 | 2 | `🧵️job/📦️packages/🦀️rust/../../⏱️context/📸️capture/../🎟️source/👶️child/🦀️.rs:13:576` |
| semio-framework-os-store-message-ledger | E0405 / E0425 / E0433 | 1 / 1 / 2 | `🏪️store/📒️ledger/📨️messages/📦️packages/🦀️rust/../../🦀️.rs:504:6` |
| semio-framework-os-pack | E0433, `serde_json` unresolved | 11 | `🎒️pack/📦️packages/🦀️rust/../../🌉️bridge/🦀️.rs:128:34` |
| semio-framework-os-command | E0433 | 4 | `📡️spr/🎮️command/📦️packages/🦀️rust/../../🦀️.rs:859:35` |
| semio-framework-ui-layout | E0433, `semio_framework_ui_contract` unresolved | 8 | `🖱️ui/📜️declaration/📐️layout/📦️packages/🦀️rust/../../🦀️.rs:1900:9` |

Totals: 9 + 2 + 4 + 11 + 4 + 8 = 38.

## Check 2b: os-kernel, native `--lib` (FAIL, 21 errors, 82 warnings)

| Crate | Code / cause | Count | First file:line |
|---|---|---|---|
| semio-framework-job | E0502 | 2 | `🧵️job/📦️packages/🦀️rust/../../⏱️context/📸️capture/../🎟️source/👶️child/🦀️.rs:13:576` |
| semio-framework-os-store-message-ledger | E0405 / E0425 / E0433 | 1 / 1 / 2 | `🏪️store/📒️ledger/📨️messages/📦️packages/🦀️rust/../../🦀️.rs:504:6` |
| semio-framework-os-pack | E0433, `serde_json` unresolved | 11 | `🎒️pack/📦️packages/🦀️rust/../../🌉️bridge/🦀️.rs:128:34` |
| semio-framework-os-command | E0433 | 4 | `📡️spr/🎮️command/📦️packages/🦀️rust/../../🦀️.rs:859:35` |

Totals: 2 + 4 + 11 + 4 = 21. The ui-theme and ui-layout errors are not in the kernel's native graph.

## Diagnosis notes

- `ui_contract` (ui-layout and ui-theme): ui-layout code refers to `semio_framework_ui_contract::`, but its manifest declares the dependency as `ui_contract = { ..., package = "semio-framework-ui-contract" }`. ui-theme's file `🧩️component/🦀️.rs` uses `ui_contract`, and ui-theme's `[dependencies]` has no `ui_contract` entry.
- `serde_json`, `ToValue`, `DslValue`, `Terminology`, `Locale` (pack, ledger, command): unresolved names in crate or module scope. These look like source inconsistencies, possibly from in-flight edits by other agents. The errors were not checked against manifests.
- Wasm-only failures: tokio feature gate, io-lifetimes `wasip2` feature, and wasmtime-internal-cache `nice` all come from registry crates compiled for wasm32-wasip2. Two C build scripts (wasmtime-internal-jit-debug, libsqlite3-sys) also fail because the wasi sysroot headers are not available to the C compiler on this host.
- Concurrency: the other agents' `cargo check` processes were running during this audit. The slot directory `play-fleet/slots/slot-1` was held by another agent and was not touched.

## Cleanup

`⚡️cache/bl` was removed at the end of the run. Logs and timestamps remain in `🗑️generated/baseline/`.
