# Baseline runtime evidence — 2026-09-05

Coordinator-run commands and observations. Logs live in the session scratchpad and are copied into `🗑️generated/` at close.

## Registered gates

| Gate | Command | Result | Evidence |
|---|---|---|---|
| OS dev quick tests | `bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache` | RED | vitest run killed by the 30 s quick budget: `[budget] … --config 🧪️tests/🟦️.ts … exceeded 30000ms — killed`. No test results printed. Lane A owns the repair. |
| Plugin registry check | `bun nx run @semio-tech/plugin-registry:check --skip-nx-cache` | RED (infrastructure) | ~20 min repo walk then `ENOENT: scandir …/target-block/debug/deps/rustcAWEOX6` in `📚️library/🔍️discovery/🟦️.ts:8754` (`discoverCatalogPackages`). A concurrent lane's isolated Cargo target root vanished mid-walk. Lane B owns skipping `target*` roots and tolerating vanished entries. |
| stdio native check (census) | `RUSTC_WRAPPER="" CARGO_TARGET_DIR=target-s-e2e cargo check -p semio-s-plugin-stdio --keep-going` | running | Peer semio-f4 verified the `#[path]` mount drift is gone from the main crate (remaining hits are in the separate `semio-s-plugin-stdio-test-oracle` crate and test-only fixtures). Peer semio-08 reports the gltf mapping mismatch fixed at 03100691d5. Result to be recorded in `📓️stdio-check-census.md`. |

## Served React shell (`dev s served`, port 6070)

Started via `.claude/launch.json` `s-react-served` (`bun ./📜️script.ts dev s served` → `nx run @semio-tech/framework-os-dev:dev -- s` with `SEMIO_RENDERER=react SKIP_PLUGIN_BUILD=1`).

- 00:00 registry refreshed (59 plugin crates, 60 playgrounds, 45 framework packages), `.vscode/launch.json` regenerated.
- 00:00–08:00 the `dev s` process (pid 95024) has no child process, ~7 % CPU, port 6070 not listening. No vite child, no engine `wasm` child. Suspected in-process `generatePluginRegistry` repo walk inside `ensurePluginRegistry` (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1121-1128`) crawling every `target*` root, the same walk that takes ~20 min in the registry check.
- Browser evidence: pending.

## Concurrency on the machine

- An orphaned `cargo rustc -p semio-s-plugin-puzzle --target wasm32-wasip2 --profile wasm-dev` (pid 96183, parent reparented to launchd) holds the shared `target-demonstrator-dev/wasm-dev` lock with a live rustc child compiling stdio; ticket `26/08/28/DEMONSTRATOR` has `-p semio-s-plugin-process` queued behind it. Not killed on purpose; peers semio-08 (PROCESS-END-TO-END) and semio-f4 (PROCEDURAL-3D-END-TO-END) consume its outputs.
- Other live tickets today: `26/09/05/BLOCK-PLUGIN-END-TO-END` (session ⚪2adc84fa) owns the block plugin; this ticket does not dispatch block work.
- `26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY` owns the stdio format-by-format rename; this ticket does not touch stdio Rust sources.
