# R2 Slice Q-1b: Rules Cleanup Of The Dashboard Crate

Date: 2026-10-08. `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`. Behaviour-preserving apart from the items marked "new".

## Verified results

| Check | Result |
| --- | --- |
| `cargo test -p semio-framework-repo-dashboard --lib` (target `target-fleet-q1b`) | 190 passed, 0 failed (was 180). One earlier run ended with `STATUS_CONTROL_C_EXIT` (0xc000013a) after 76 tests: a console Ctrl+C reached the test process (the daemon interrupt tests share the console with other agents); the rerun was green. |
| `cargo +nightly-2026-07-07 clippy -p semio-framework-repo-dashboard --all-targets` | 0 warnings in the dashboard crate (was 35 lib / 32 lib-test). Warnings that remain belong to `semio-framework-ui` (T-A/T-B/T-D). |
| `bun test` `🧪️tests/⌨️controls` | 5 pass |
| `bun test` `🧪️tests/🌀️control-plane` | 11 pass |
| `bun test` `🧪️tests/🎮️registry` (with `SEMIO_DASHBOARD_BIN`) | 8 pass |
| `bun test` `🧪️tests/🧊️execution` (with `SEMIO_TEST_ARTIFACT_DIR`) | 4 pass |
| `r2-q1-emoji-check.ts` on `D` (without V-1 folders) and on `⌨️tui/🚇️pty` | clean (was 120 problems in `D`, 14 in `🚇️pty`) |

Not run: the V-1 folders (`🧪️tests/🧭️journeys`, `🗺️coverage`) and native PTY tests.

## 1. Docstring emoji

`r2-q1-emoji-check.ts [dir]` reports, per file, every docstring block whose first word is no emoji (`MISSING`, `#️⃣` keycaps count) or repeats an earlier emoji of the same file (`DUP`, variation selectors ignored). `r2-q1-emoji-apply.ts <spec>` replaces the emoji of named lines (`file:line emoji`). All 120 findings in `D` were repaired with a fitting, unique emoji, and the 14 in the PTY module (Unix/Windows twins) too. The two `MISSING` rows were the keycap `#️⃣` of the XXH3 docstrings (checker false positive, fixed in the checker).

## 2. Comments inside definitions

Only `//#region` markers existed (208). The 14 inside an `impl` or fn body (`🌀️daemon/🧠️supervisor` 10, `📦️packages/🦀️rust` 4) are deleted. Top-level `//#region 🔖️X` markers between items stay: the convention of the whole repo (thousands of uses outside `D`).

## 3. Debug output

Deleted: `println!("[DEBUG] ...")` in `⚙️preferences` tests and four in `🌀️daemon/🧪️tests/🧊️integration` (plus the unused `seconds`/`started` they needed), the `SEMIO_TEST_PRINT_JOURNEYS` printing in the launcher tests, the `[DEBUG]` prefix of the fixture program `🧫️fixtures/🌀️control-plane/📜️script.ts`. Decision on the two env-gated traces: both removed (`SEMIO_DASHBOARD_TRACE` in `🖥️terminal/🦀️.rs` and the `trace()` helper with its seven calls in `📚️inventory/🦀️.rs`), and the README sentence that documented it. They only printed elapsed microseconds and no test read them; the first-frame timing is measured through the PTY test.

## 4. Dead code

`#[allow(dead_code)] mcp: Option<serde_json::Value>` is gone. `Tool.mcp` is typed (`registry::Mcp { server, clients: BTreeMap<String, McpClient> }`, `AGENT_CLIENTS`, same shape as `#/$defs/McpExposure`), validated (server slug, at least one client, client in `AGENT_CLIENTS`, every stated parameter is one the tool accepts) and used: `Entry::mcp` is public and `entry_json` / `semio commands --json` lists it; the `RegistryEntry` schema gained `mcp`. New: test `a_tool_that_is_an_mcp_server_lists_the_clients_that_start_it_and_a_wrong_exposure_is_a_problem`, and `bun-test` of the registry fixture workspace carries an exposure (the Ajv/oracle test in `🧪️tests/🎮️registry` still passes). The last `#[allow(clippy::too_many_arguments)]` (`Entry::new`, 11 arguments) is replaced by a `Draft` struct; no `allow(` is left in `D`.

## 5. CLI text in the catalogue

New `cli_*` labels (en and de, placeholders checked) in `🖥️terminal/🌐️labels`: daemon usage, serve/start/stop failures and success lines, "not running", the status line, "warning: …", the "daemon startup failed" error, "unexpected arguments", "failed to attach to the terminal", "unknown verb", "failed to run". `preferences::cli_locale(root, parsed)` picks the language (journals, environment, flag; English when they cannot be read). `DashboardLabels::skew` replaces the view-local `skew_text` and the English `Skew::describe()` in `daemon status`. `supervisor::{start_detached, status, stop}` take the catalogue. Test `command_line_messages_follow_the_language_of_the_preference`. Still English by design: details that come from lower layers (preference parse errors, registry resolve errors that the oracle tests match, `Skew::describe()` inside `RunError`) and the `usage:` syntax lines of `semio run|logs|open` in `🧭️cli`.

## 6. Clippy (all fixed)

Boxed `ServerMsg::SessionChanged(session)` and `ClientMsg::Spawn(command)` (wire unchanged), `Window::Body::Output.session`; `FrameBuffer::next` -> `next_frame`; `Input` is `Copy`; `while let`, `map_or_else`, `is_multiple_of`, `push('\x18')`, iterator loops, `watch`/`attach` return nothing, `spawn_group(&str)`, `serve(&AtomicBool)`, `inventory::start(&Path)`, `send(&ClientMsg)` in the integration harness, struct-literal test data, `size_of` without path, char-array pattern, `Escape::OperatingEnd`.

## 7. Feature coverage

Convention (A-1's, now shared): a Rust test `every_scenario_of_the_X_feature_is_proved_by_a_test` calls `crate::tests::assert_proved(feature, sources, proofs)` (`🧪️tests/🔬️unit/🦀️.rs`): the scenario titles must equal the proof titles in order, every proof names at least one `#[test]` that exists. Bun adapters check the same by title.

- Keymap: new `🧪️tests/⌨️keymap/🥒️.feature` (5 scenarios; the three keymap scenarios moved out of `⌨️controls`). Wired to the bun oracle `🧪️tests/⌨️controls/🟦️.ts` (new test checks each scenario against a bun test of that file and a named Rust test) and to `⚙️preferences/⌨️keymap/🧪️tests`. Fixture `🧫️fixtures/⌨️keymap/🔣️.json`, oracles: Ajv, an independent canonicalizer and builder, Node readline.
- The four runnerless features now have a runner: `⚙️preferences` (8 scenarios, Rust in `⚙️preferences/🧪️tests`, bun in `🌀️control-plane`), `📼️replay` (7), `🟢️ready` (5), `🧩️groups` (6), all proved in `🌀️daemon/🧪️tests/🔬️unit`.
- New features for the new modules, each proved by existing tests: `✉️ipc` (6), `🚚️transport` (3), `📜️journal` (3), `🧠️supervisor` (6), `🕹️control` (4) in `🌀️daemon`; `🚀️launcher` (5), `🌐️labels` (5), `🪟️windows` (9), `📡️sessions` (7), `📋️panes` (4) in `🖥️terminal`; `🧭️cli` (5); `🏛️repo-domain` (4). `👥️clients` is covered by the replay scenarios "Never disconnect a stalled view" and "List many sessions to a fresh view".
- Bun side (`🧪️tests/🌀️control-plane/🟦️.ts`, new test): the independent oracles of that file are tied to scenarios of `⚙️preferences`, `🚀️launcher`, `🟢️ready`, `✉️ipc` and `⌨️controls`.
- Not wired (not in scope, no runner change): the `🥒️.feature` files under `🧫️fixtures` (`🌳️inferred-targets`, `🗣️launch-axes`), and the `⌨️controls` scenario "Windows owns UTF-8 code pages only while attached" (its test lives in the TUI crate).
- Gaps that stay: `🧪️tests/🧭️journeys` and `🗺️coverage` (V-1); no new fixtures were added for the module features (their tests build data inline); `🟢️ready` bytewise scenario is proved by the table test that also feeds one byte at a time.

## 8. Files

Edited: all `D/**/🦀️.rs` for the emoji, regions and clippy; `D/🎮️registry/🦀️.rs`, `🧬️schema/🎮️registry/🔣️.json`, `🧫️fixtures/🎮️registry/🏗️workspace.json`, `🎮️registry/🧪️tests/🔬️unit/🦀️.rs`; `D/🖥️terminal/{🦀️,🌐️labels,📡️sessions,🚀️launcher,🪟️windows}`; `D/🌀️daemon/{🦀️,🧠️supervisor,✉️ipc,📜️journal,🕹️control,👥️clients}` and its tests; `D/⚙️preferences/🦀️.rs` and tests; `D/📚️inventory/🦀️.rs`; `D/📜️root-delegation/🦀️.rs`; `D/📦️packages/🦀️rust/🦀️.rs`; `D/📇️playground-catalog` test; `D/🧪️tests/🔬️unit/🦀️.rs`; `D/README.md`; `D/🧫️fixtures/🌀️control-plane/📜️script.ts`; `D/🧪️tests/⌨️controls/{🟦️.ts,🥒️.feature}`, `🌀️control-plane/🟦️.ts`; `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🚇️pty/🦀️.rs` (docstring emoji only). Created: the 13 feature files under `D/🧪️tests/{⌨️keymap,✉️ipc,🚚️transport,📜️journal,🧠️supervisor,🕹️control,🚀️launcher,🌐️labels,🪟️windows,📡️sessions,📋️panes,🧭️cli,🏛️repo-domain}`, and in `T`: `r2-q1-emoji-check.ts`, `r2-q1-emoji-apply.ts`, `r2-q1-clippy.sh`, `r2-q1-check.sh`, `r2-q1-sub.ts`, `r2-q1-draft.ts` plus one-off edit specs `r2-q1-e*.json|ts`, `r2-q1-emoji-spec-*.txt`.

Incident: a stray line `s` at the end of `🎮️registry/🦀️.rs` (appeared while several agents wrote the file) broke the build for a moment; removed.
