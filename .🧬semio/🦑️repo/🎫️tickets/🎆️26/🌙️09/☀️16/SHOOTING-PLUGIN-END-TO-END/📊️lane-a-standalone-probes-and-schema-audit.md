# Lane A Standalone Probes and Schema Audit

Date: 2026-11-22. Ticket: `🎆️26/🌙️09/☀️16/SHOOTING-PLUGIN-END-TO-END`. Logs: `🗑️generated/`.

## Source closures

`source-boundary-audit.mjs` on every `Cargo.toml` under `✏️s/🔌️plugins/🎥️shooting` and `🌎️hub/🧩️compositions/🎥️shooting`.

| Target | Exit |
| --- | --- |
| `x86_64-unknown-linux-gnu` | 0 |
| `aarch64-apple-darwin` | 0 |
| `x86_64-pc-windows-msvc` | 0 |
| `wasm32-unknown-unknown` | 0 |
| `wasm32-wasip2` | 0 |

Log: `🗑️generated/source-boundary-audit.log` (`ALL_DONE rc=0`). Sixteen crates, one virtual workspace skipped (`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.toml`). Every lib and test closure is within 32 files, 4000 active lines, and 262144 bytes. The tightest closures are `semio-s-artifact-shooting-shooting-mut-any` and `mut-any-2` at 31 files, and `semio-s-artifact-shooting-shooting` test at 27 files, 3423 active lines, 221488 bytes. No crate split was required.

## Standalone probes

Launched with `🚀️launch-shooting-script.ts` and the pinned Bun binary.

| Command | Exit | Result |
| --- | --- | --- |
| `drawing-browser-check.mjs` as `shooting:drawing-probe` | 0 | Chromium recovered the same render after connection reset and HTTP 503, kept sibling cancellation, and resolved relative Three resources. Log: `🗑️generated/drawing-browser-check.log`. |
| `wgpu-world-transport-probe.mjs` | 0 | Worker dropped partial bytes, retried HTTP 503, and sealed the repository GLB. Permanent failure used one attempt. Log: `🗑️generated/wgpu-world-transport-probe.log`. |
| `svg-retry-oracle.cjs` | 0 | RxJS virtual schedule matched `🖼️svg-pane/🧪️retry.json`, and the native `svg-retry-probe` matched the same offsets. Log: `🗑️generated/svg-retry-oracle.log`. |
| `@semio-tech/shooting-shooting-rs:verify-snapshot-sqlite-source` | 0 | `tsc --noEmit --strict` on the public snapshot facade and the SQLite snapshot test. Log: `🗑️generated/verify-snapshot-sqlite-source.log`. |
| `@semio-tech/shooting-shooting-rs:test-snapshot-sqlite-source` | 0 | 43 pass, 0 fail, 155 `expect()` calls, 1 file. Log: `🗑️generated/test-snapshot-sqlite-source.log`. |

## Fix

`verify snapshot-sqlite-source` failed once with `TS6053`: it typechecked `🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`, which is not the owned SQLite test. `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🧪️tests/🔬️rust-package-commands/🟦️.ts` now typechecks `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`. The package router `📜️script.ts` points `snapshotSqliteTests` at that same file. The re-run exited 0.

## Localization and schema

English and German coverage is complete on the scanned surfaces:

- 41 mutation JSON schemas, 0 fields missing a bilingual `x-semio-ui.label` or a glossary key.
- 328 JSON documents under the plugin, hub composition, and dev variant, 0 string labels and 0 labels missing `en` or `de`.
- 686 non-fixture Rust, TypeScript, and JSON sources, 0 empty `native_de` values.

Drawing property order is the same in `🧬️schema/🔣️.json` and in the GraphQL `ShootingDrawing` types under artifact, snapshot, mutations, and diff. Each of those GraphQL roots and their protobuf twins expose `emblem`. SQLite `🗄️.sql` stores the same drawing fields as `drawing_*` columns on `shooting_shot` and stores emblem rows in `shooting_emblem`. The text DSL snapshot (`🚪️io/📝️text/📸️snapshot`) is an opaque `schema` plus `payload` envelope in GraphQL, Protobuf, and `📖️.grammar.semio`, so it has no field-level twin to diverge. Coverage is the passing test `Shooting drawing fields share one order and every document facet owns emblem` inside the 43-test source run.
