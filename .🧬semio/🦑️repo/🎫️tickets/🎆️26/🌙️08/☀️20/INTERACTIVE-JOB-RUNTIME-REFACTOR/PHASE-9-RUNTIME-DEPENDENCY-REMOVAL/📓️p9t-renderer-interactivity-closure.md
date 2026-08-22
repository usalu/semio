# Renderer Interactivity Closure

## Exact residue before

The authoritative audit reported 14 findings: one renderer binary `block_on`, twelve synchronous filesystem findings, and one synchronous process invocation. One filesystem finding belongs to the excluded UI-contract typegen test. The renderer-owned eleven filesystem findings were: three kernel plugin/extension discovery and byte-read sites, four scale registry/WASM/report sites, hot-swap metadata, and three ProgramBridge plugin/descriptor discovery sites. The process finding was scale RSS sampling through synchronous `ps`.

## Owned I/O job boundary

`semio-framework-os-services::NativeIoJob` is an owned, schema/region-structured `InteractiveJob` with requests for byte reads, directory scans, modified times, chunked writes, and resident-memory observation. Reads/writes are sliced at 32 KiB; scans process at most 32 entries per step. Every step passes through the standard watchdog, 4 ms interactive budget, cancellation token, and shared renderer `WorkerPool` `Lane::Io`. `RendererIoHandle` is a non-blocking completion future/`try_take` handle whose `Drop` cancels unfinished work.

Renderer integration now routes:

- kernel parent/extension WASM reads and extension scans;
- ProgramBridge plugin directory, WASM artifact, and descriptor discovery;
- hot-swap modified-time scans, with the UI frame only submitting/polling a retained handle;
- scale registry/WASM reads and report writes;
- scale RSS observation through owned macOS/Linux platform probes, with no spawned `ps` process.

Plugin reload waits for discovery on the app task, then revalidates the live runtime handle before committing the new entries. Prepared-render revision/generation gates are unchanged.

## Audit and tests

- `📝️p9t-renderer-interactivity-audit-3.txt`: zero renderer violations. The only remaining synchronous finding is the explicitly excluded UI-contract typegen test. The renderer binary's single owned async driver is permanently allowlisted as a true process entrypoint.
- `📝️p9t-os-services-native-io-tests-3.txt`: 3/3 pass, including `Send`, resident-memory, and a 96 KiB-plus chunked write/read/scan/modified-time round trip.
- `📝️p9t-os-services-native-io-release-check-3.txt`: native release check passes.
- `📝️p9t-ui-host-wasm-check.txt`: relevant wasm platform boundary passes.
- `📝️p9t-dependency-ratchet.txt`: 211/238, clean.
- `📝️p9t-owned-fmt-check-2.txt` and `📝️p9t-renderer-static-census.txt`: owned formatting passes; renderer direct dependency/runtime-call census is zero except the single sanctioned binary driver.

## Full renderer gate boundary

`📝️p9r-wgpu-native-check-2.txt` is the exact attempted native boundary. Cargo stops before WGPU on 14 `semio-framework-os-flow` errors and 2,740 `semio-s-plugin-puzzle` async-migration errors. Therefore no claim is made that full WGPU native/release/WASM compilation ran to completion. The source/static renderer gate is clean and all newly owned leaf boundaries compile/test; Flow and Puzzle are deliberately untouched.
