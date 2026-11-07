# 1105b Audit: Temporary Debug Artifacts Added 2026-11-05

Scope: `🧰️framework/`, `✏️s/`, `🌎️hub/`, `♻️mit-bestand/`. Read-only audit.
Sources: `git log --since="2026-11-05 00:00" -p` (commits 0454b0b6be9, 3904c31bd05, a15c8956e6c, be9cb5028db) and the uncommitted working tree (`git diff`, `git diff --cached`).
Patterns: `[DEBUG]`, `[TRACE]`, `dbg!(`, `println!`/`eprintln!`, `console.*`, `log::*!`, `todo!()`, `unimplemented!()`.

Line numbers: working-tree hits are current; committed hits are the line in that commit's new file and may have shifted.

## 1. Temporary `[DEBUG]` prints in non-test code (uncommitted, working tree)

These are new in the working tree and not yet committed. They match the aggregator-gate debugging described in the brief.

| # | file:line | snippet | owning lane | non-test |
|---|---|---|---|---|
| 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/📨️action-dispatch/🦀️.rs:27` | `eprintln!("[DEBUG] aggregator-gate A capture_not_ready {}", address.action_id)` | aggregator (uncommitted) | yes |
| 2 | `…/🔌️plugin-app-impl/📨️action-dispatch/🦀️.rs:32` | `eprintln!("[DEBUG] aggregator-gate B dispatch_action_none {}", address.action_id)` | aggregator (uncommitted) | yes |
| 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:134` | `eprintln!("[DEBUG] aggregator-gate C stage_action_source {}", action)` | aggregator (uncommitted) | yes |
| 4 | `…/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:135` | `eprintln!("[DEBUG] aggregator-gate D loan_source {}", action)` | aggregator (uncommitted) | yes |
| 5 | `…/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:137` | `eprintln!("[DEBUG] aggregator-gate E received_none {} resumes_in_flight={} typing_pending={} typed_prod_pending={} tool_dispatch={:?} typing_ready={} emission_pending={}", …)` | aggregator (uncommitted) | yes |

All five are inside `dispatch_action` / its helpers and print on every early `return Ok(None)`. They must be removed before commit. Paths are relative to the repo root.

## 2. Temporary `[TRACE]` prints in non-test code (committed today)

Both are gated by runtime-diagnostics flags, but they are still temporary tracing text in production code.

| # | file:line | snippet | owning lane | non-test |
|---|---|---|---|---|
| 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs:56` (commit a15c8956e6c) | `eprintln!("[TRACE] typed-operation slots instance={instance} live={live}/{} peak={peak}", …)`, gated by `runtime_diagnostics_enabled()` or a drain flag | typed-operations / reactor runtime (a15c8956e6c) | yes |
| 7 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🏁️exchange-finish/🦀️.rs:75` (commit a15c8956e6c) | `debug_runtime_line(format_args!("[TRACE] typed-operation inline leftover operation={} merged-into-reply={merged}", …))` | plugin-exchange / exchange-finish (a15c8956e6c) | yes |

## 3. `[DEBUG]` and `[TRACE]` prints in test code (committed today, informational)

These are in test files and are not production debug code. They should still be removed before the tests are finalized.

| file (committed in) | snippet | non-test |
|---|---|---|
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/dag.ts:17` (3904c31bd05) | `console.log("[DEBUG] public control DAG real-clock counts …")` | no (test) |
| `…/🧪️tests/⏳️admission/freshness.ts:32, :54` (3904c31bd05) | `console.log("[DEBUG] public control DAG freshness …")`, `"[DEBUG] public open-wait freshness …"` | no (test) |
| `🧰️framework/🔨️modules/🔲️pixels/✍️editing/🪣️flood/🧪️tests/large-close.test.ts:24` (3904c31bd05) | `console.log("[DEBUG] large flood close probe …")` | no (test) |
| `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎥️shooting-route/🟦️.ts:40` (a15c8956e6c) | `console.log("[DEBUG] shooting authored route …")` | no (test) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/📄️natural/🦀️.rs:83, :249` (be9cb5028db) | `eprintln!("[DEBUG] semio-current-example …")`, `eprintln!("[DEBUG] semio-current-article …")` | no (test) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs:32` (be9cb5028db) | `for index in 0..1048576 { if index<512 { eprintln!("[DEBUG] native opening turn={index}") } …}` | no (test; loop prints heavily) |
| `…/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:238` (be9cb5028db) | `eprintln!("[TRACE] actual typed ingress pre-admitted …")` | no (test) |
| `…/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧪️tests/original-view-read.rs:220` (a15c8956e6c) | `println!("[DEBUG] actual actor view pin job aliases …")` | no (test) |
| `…/🌊️flow/🖥️host/📤️publication/🧵️work/🧪️tests/🦀️.rs:63` (a15c8956e6c) | `assert_eq!(…, "[DEBUG] original=…")` (assertion message) | no (test) |
| `…/📦️packages/…/🧪️tests/…` and other `🧪️tests/` hits (about 60 lines in total) | mostly `console.log`/`println!` in fixtures and test scripts | no (test) |

## 4. Intentional tool or CLI output (not debug, listed for completeness)

These are product or tooling output, not temporary debugging. Review only if the brief wants them removed.

| file:line (commit) | snippet | non-test |
|---|---|---|
| `🧰️framework/…/🎒️pack/⌨️cli/💾️binary/🦀️.rs:5-6`, `🚦️control/🦀️.rs:77`, `📡️spr/⌨️cli/…:5-6`, `📡️spr/⌨️cli/🦀️.rs:654` (a15c8956e6c) | `eprintln!` for CLI progress and errors | yes, intentional |
| `🖨️describe/🛂️descriptor-emission/🦀️.rs:633-736` (a15c8956e6c) | `eprintln!("usage: …")`, error reporting | yes, intentional |
| `…/🧑‍💻dev/♻️activation/🚀️launch/🟦️.ts:16` (a15c8956e6c) | `console.log("[dev] activating …")` | yes, dev tool output |
| `…/📚️library/🎛️commands/🛠️dev/🟦️.ts:43` (a15c8956e6c) | `console.error("[dev] unknown route …")` | yes, dev tool output |
| `…/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:467` (a15c8956e6c) | `console.log("[runtime graph] acquiring actual native browser …")` | yes, verification progress; check if it should stay |
| `📜️script.ts` build scripts: `🖌️render/…/📜️script.ts:74`, `wgpu/…/📜️script.ts:61,188`, `📦️rust/📜️script.ts:208,417`, `🖥️host/…/📜️script.ts:79`, `mcp/…/📜️script.ts:22` | `console.log` / `console.error` build progress | yes, build tooling output |
| `…/🌉️mcp/📡️stdio/📦️binary/🟦️.ts:134-140` (staged, uncommitted) | `console.error("[semio-os-mcp] staging binary …")` | yes, CLI progress |
| `…/IconRenderHost/📤️export/🟦️.ts:40, :83` (a15c8956e6c) | `console.error` in listener and announce catch paths | yes, error reporting |
| `…/📨️/ShellHost/…/🟦️.tsx:186, :368-371`, `shell-uri-documents/…/🟦️.ts:29, :268`, `🎨️Canvas/…/🟦️.tsx:81` (be9cb5028db) | `console.error` / `console.warn` for failed admissions | yes, error reporting |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📣️diagnostics/🟦️.ts:13` (3904c31bd05) | `cargoProvenanceConsoleSinkV1` default sink with `console.log` | yes, diagnostics sink; intentional |

## 5. Not found

- `todo!()` and `unimplemented!()`: none added today in non-test code.
- `dbg!(`: none added today.
- `🌎️hub/` and `♻️mit-bestand/`: no debug artifacts added today.

## 6. Recommended action

1. Delete items 1 to 5 (aggregator-gate prints) before committing. Owner: aggregator lane.
2. Decide on items 6 and 7 (`[TRACE]` in typed-operations and exchange-finish). Remove them or keep them behind an explicit diagnostic flag that is documented.
3. Clean test-code `[DEBUG]` prints from section 3, especially the 1048576-iteration loop in `🌱️initialization/🎟️registration/🧪️tests/🦀️.rs:32`.
