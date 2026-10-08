# 🔎 Round 2 Rules Audit: Dashboard And TUI Working Tree

Read-only audit against `AGENTS.md`. Scope: every changed or untracked file under `D` and `UI` (162 entries from `git status`, 106 `.rs`, 10 `.ts`, 20 `.feature`, 2 `Cargo.toml`, 23 `.json`, 1 `.md`). Raw helper output: `🗑️generated/audit-hits.txt`, `🗑️generated/question-mark-files.txt`, `🗑️generated/clippy-full.txt`.

Abbreviations: `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`, `UI` = `🧰️framework/🔨️modules/🖱️ui`, `T` = ticket folder. Line numbers refer to the working tree as read at audit time.

Slices: T-A ansi/backend/windows/host/pty (TUI native), T-B event/scene/layout/engine/chrome/overlays, T-C vt + terminal widget, T-D text/cell/theme/rows/elements/List/Wizard/Table/Log/Input/Tree/unit tests, A-1 registry/command-tree/inventory/cli/usage/entrypoint/repo-domain/root-delegation/packages, A-2 daemon/connection/pty-daemon, A-3 terminal view/preferences.

Note: the brief's clippy grep (`🎛️dashboard|⌨️tui|🧱️elements`) returns nothing because clippy prints `..\..` backslash paths. The full output is in `🗑️generated/clippy-full.txt`; the table below is filtered from it.

## 1. Docstrings without an emoji (placeholders `???`/`??`/`?`)

Pre-existing at HEAD in most files, still a rule violation in the changed files. Placeholders replaced emoji; repair with the owner's real emoji.

| file:line | rule | finding | suggested fix | slice |
|---|---|---|---|---|
| UI/⌨️tui/🎬️scene/🦀️.rs:10,31,40,48,81,146,259,288 | docstring must start with emoji | `/// ???` on 8 docstrings | restore emoji | T-B |
| UI/⌨️tui/📏️layout/🦀️.rs:4,18,27,95,178,186,206,210,214,289,304,320,382,387,399,413,458,504,533,538 (23 lines in total) | docstring must start with emoji | `/// ???` / `/// ??` / `/// ?` | restore emoji | T-B |
| UI/⌨️tui/🔌️backend/🦀️.rs:24 | region marker | `//#region ???Clipboard` | restore emoji | T-A |
| UI/⌨️tui/🔌️backend/🦀️.rs:65,199,223 | docstring | `/// ??` | restore emoji | T-A |
| UI/⌨️tui/🏃️host/🦀️.rs:8,63 | docstring | `/// ???` | restore emoji | T-A |
| UI/⌨️tui/🦀️.rs:1 (`//!`), :49 | module and item docstring | `//! ???`, `/// ??` | restore emoji | T-D |
| UI/⌨️tui/🪀️widget/🦀️.rs:35,89,96,129,191,308 | docstring | `/// ???` / `/// ??` | restore emoji | T-C |
| UI/⌨️tui/🧪️tests/🔬️unit/🦀️.rs:475,516,518,546,548 (26 lines in total) | region markers | `//#region ???Geometry`, `//#region ???Text`, and similar | restore emoji or delete the markers (see §3) | T-D |

## 2. Duplicated emoji within the same file

AGENTS: "start all docstrings with a unique and fitting emoji". Counted per file over changed `.rs` files: 224 duplicate docstring blocks now, versus 79 at HEAD. The rule is broken across most of the changed files; the increase comes mainly from new files.

| file | now (dup blocks) | HEAD (changed files) | suggested fix | slice |
|---|---|---|---|---|
| D/🎮️registry/🦀️.rs | 21 | 12 | assign unique emoji per docstring | A-1 |
| UI/⌨️tui/🖥️chrome/🦀️.rs | 21 | 6 | assign unique emoji (🏷️, 🚦️, 🎯, 🪟, 🧱 each repeated) | T-B |
| UI/⌨️tui/📜️rows/🦀️.rs | 16 | new | assign unique emoji | T-D |
| UI/⌨️tui/🚇️pty/🦀️.rs | 14 | 13 | assign unique emoji (📥, 🪜, 🛎️, 🧵, 🌱, 🧾, 📤, ✍️, 🏁, 🔁, 📣 repeated) | T-A |
| UI/⌨️tui/⚙️engine/🦀️.rs | 13 | new | assign unique emoji | T-B |
| D/🏛️repo-domain/🦀️.rs | 11 | new | assign unique emoji | A-1 |
| D/🌀️daemon/✉️ipc/🦀️.rs | 9 | 4 | assign unique emoji | A-2 |
| D/🌀️daemon/🚚️transport/🦀️.rs | 8 | 8 | assign unique emoji (🔌, 🤝, 📥, 📤, 👂, ⏰, ⏳, ⏱️) | A-2 |
| D/⚙️preferences/⌨️keymap/🦀️.rs | 7 | new | assign unique emoji | A-3 |
| D/🌀️daemon/📼️replay/🦀️.rs | 6 | 5 | assign unique emoji | A-2 |
| UI/⌨️tui/🔌️backend/🦀️.rs | 6 | 2 | assign unique emoji (📋️ repeated 4x) | T-A |
| D/🧪️tests/🧭️journeys/🖥️terminal/🦀️.rs | 6 | new | assign unique emoji | A-3 |
| D/🖥️terminal/🪟️windows/🦀️.rs | 6 | new | assign unique emoji | A-3 |
| D/🖥️terminal/🚀️launcher/🦀️.rs | 5 | new | assign unique emoji | A-3 |
| D/🌳️command-tree/🦀️.rs | 4 | 16 | assign unique emoji (🌳️ repeated) | A-1 |
| D/📎️connection/🦀️.rs | 4 | new | assign unique emoji | A-2 |
| D/🌀️daemon/🧠️supervisor/🦀️.rs | 4 | new | assign unique emoji | A-2 |
| D/🌀️daemon/🕹️control/🦀️.rs | 4 | new | assign unique emoji | A-2 |
| UI/⌨️tui/📝️text/🔤️tables/🦀️.rs | 4 | new | assign unique emoji | T-D |
| UI/🧱️elements/📊️Table/🎯️targets/⌨️tui/🦀️.rs | 3 | new | assign unique emoji | T-D |
| UI/🧱️elements/🌳️Tree/🎯️targets/⌨️tui/🦀️.rs | 3 | new | assign unique emoji | T-D |
| UI/⌨️tui/🎨️theme/🦀️.rs (🎭️, 🔣️, 🔤️), UI/⌨️tui/📏️layout/🦀️.rs (📏️, 🪟), UI/⌨️tui/📝️text/🦀️.rs (📐️, 🪡️), UI/⌨️tui/🔡️ansi/🦀️.rs (🖍️, 📋️, 🏷️), UI/⌨️tui/🔲️cell/🦀️.rs (🧮️), UI/⌨️tui/🪟️windows/🦀️.rs (🔒), UI/🧱️elements/📃️List, 🧙️Wizard, 🪟️Window | 3 or fewer each | | unique emoji per docstring | T-B / T-D / T-A |
| D/📦️packages/🦀️rust/🦀️.rs (3), D/🧭️cli/🦀️.rs (3), D/🖥️terminal/⌨️controls/🦀️.rs (3), D/🌀️daemon/👥️clients/🦀️.rs (3) | 3 each | | unique emoji | A-1 / A-2 / A-3 |

Full per-line list: run the helper and read `🗑️generated/audit-hits.txt` (`DUP-EMOJI`).

## 3. Comments inside definitions

AGENTS: "You MUST NOT comment inside definitions." Region markers `//#region` / `//#endregion` sit inside `impl` bodies and are comments.

| file:line | rule | finding | suggested fix | slice |
|---|---|---|---|---|
| UI/⌨️tui/⚙️engine/🦀️.rs:183,328,330,349,351,415,417,462,464,581,583,895,897,964 (14 markers) | no comments in definitions | `//#region` / `//#endregion` inside `impl` | delete markers; split by file if the grouping is needed | T-B |
| UI/⌨️tui/📏️layout/🦀️.rs:468 | no comments in definitions | real `//` comment inside a fn body | move to the fn docstring or delete | T-B |
| UI/⌨️tui/📟️vt/🧱️screen/🦀️.rs:274,367,369,447,449,660,662,728,730,851,853,1024,1026,1104,1106,1123,1125,1250 (18 markers) | no comments in definitions | `//#region` markers in `impl Screen` | delete; split the impl if needed | T-C |
| UI/⌨️tui/📟️vt/🖥️pane/🦀️.rs:151,238,240,285,287,465,504,584 (8 markers) | no comments in definitions | `//#region` markers | delete | T-C |
| D/🌀️daemon/🧠️supervisor/🦀️.rs:202,264,266,349,351,629,631,758,760,768 (10 markers) | no comments in definitions | `// #region` markers inside `impl` | delete | A-2 |
| D/📦️packages/🦀️rust/🦀️.rs:166,205,207,256 (4 markers) | no comments in definitions | `//#region` markers inside a fn body | delete | A-1 |
| UI/⌨️tui/🧪️tests/🔬️unit/🦀️.rs:93,143,293,315 | no comments in definitions | `//` explanations inside test fn bodies | move to the test's docstring | T-D |

## 4. Leftover debug output and `[DEBUG]` logs

AGENTS: temporary logs must carry `[DEBUG] ` and be removed; no debug printing left behind. No `todo!`, `unimplemented!`, `dbg!`, or `console.log` debug output was found in library code.

| file:line | rule | finding | suggested fix | slice |
|---|---|---|---|---|
| UI/⌨️tui/🧪️tests/🔬️backend-native-windows-unit/🦀️.rs:71 | no leftover debug | `println!("[DEBUG] wake ended a 10 s wait ...")` | delete or make it an assertion | T-A |
| UI/⌨️tui/🧪️tests/🔬️backend-native-common/🦀️.rs:74,78,81 | no leftover debug | `panic!("[DEBUG] ... thread panic")` used as probes | replace with real assertions on the thread outcome | T-A |
| UI/⌨️tui/📟️vt/🧪️tests/🔬️unit/🦀️.rs:85,139,182 | no leftover debug | `println!("[DEBUG] terminal streams/keys/pointer: ... shared ...")` | delete | T-C |
| UI/⌨️tui/🧪️tests/⌨️input-decoding/🦀️.rs:74 | no leftover debug | `println!("[DEBUG] input decoding: ...")` | delete | T-D |
| D/⚙️preferences/🧪️tests/🔬️unit/🦀️.rs:33 | no leftover debug | `println!("[DEBUG] preference journal replay ...")` | delete | A-3 |
| D/🖥️terminal/🦀️.rs:81 | no leftover debug | `[DEBUG] dashboard usable first frame` behind `SEMIO_DASHBOARD_TRACE` (documented at `D/README.md:240`) | keep only as a deliberate diagnostic without the `[DEBUG]` tag, or delete; decide with the owner | A-3 |
| D/📚️inventory/🦀️.rs:297 | no leftover debug | `[semio inventory] {stage} ...us` trace behind `SEMIO_DASHBOARD_TRACE` | same decision as above | A-1 |
| D/🖥️terminal/🚀️launcher/🧪️tests/🔬️unit/🦀️.rs:44,57 | no leftover debug | `[DEBUG] entries ...` / `[DEBUG] {} => {}` behind `SEMIO_TEST_PRINT_JOURNEYS` | delete the env-gated printing | A-3 |
| D/🌀️daemon/🧪️tests/🧊️integration/🦀️.rs:322,440,522,579 | no leftover debug | four `println!("[DEBUG] ...")` in integration tests | delete or assert the values | A-2 |
| D/📦️packages/🦀️rust/🦀️.rs:70,263,280 and other `[semio]`/`[dashboard]` eprintln and println lines in `🧭️cli`, `🌀️daemon/🦀️.rs`, `📜️root-delegation` | none | product CLI output, not debug; see §10 for the locale issue | none | A-1 / A-2 |

## 5. Dead code hidden by attributes

| file:line | rule | finding | suggested fix | slice |
|---|---|---|---|---|
| D/🎮️registry/🦀️.rs:301 | no `#[allow(dead_code)]` stubs | `#[allow(dead_code)] mcp: Option<serde_json::Value>` on `ToolDeclaration` accepts a field and never uses it | remove the field from the struct and the schema (`deny_unknown_fields` then rejects it), or implement it | A-1 |
| UI/⌨️tui/🪟️windows/🦀️.rs:33,227 | no unused stubs | clippy: `WAIT_FAILED` and `PeekNamedPipe` never used | delete both | T-A |
| UI/⌨️tui/🪟️windows/🦀️.rs:3 | acceptable, note | file-level `#![allow(non_camel_case_types, non_snake_case)]` for Win32 FFI names | keep; name the acronym types per clippy in one place if desired | T-A |

## 6. Compatibility layers, legacy, fallbacks

No compatibility layer, deprecation, migration, or adapter for old data was found in scope. Terms reviewed and judged not violations:

- `UI/⌨️tui/🔡️ansi/🦀️.rs:895,922` `legacy: bool` is the X10 mouse encoding (protocol term).
- `UI/⌨️tui/🧪️tests/🔬️unit/🦀️.rs:362` `mouse_legacy_encodings` names the same protocol mode.
- `D/🎮️registry/🦀️.rs:29,142,145,147,1040,1068` `FALLBACK_VERB = "task"` is a deliberate default verb rule, not a legacy shim. Owner may still want it named as a rule, not a fallback.
- `D/🖥️terminal/🌐️labels` and `🖥️terminal/🪟️windows` use "adapter" only in test docs; `D/🏛️repo-domain/🦀️.rs:123,430` `// #region Adapters` is a region name, not migration code.

## 7. Runtime dependencies

| file | rule | finding | fix | slice |
|---|---|---|---|---|
| D/📦️packages/🦀️rust/Cargo.toml | no new runtime dep | unchanged in working tree; `[dependencies]` only path crates plus `serde`/`serde_json` workspace | none | A-1 |
| UI/📦️packages/🦀️rust/Cargo.toml (diff hunk at line 148) | no new runtime dep | only `[dev-dependencies]` added: `unicode-width = "0.2.2"`, `unicode-segmentation = "1.13.2"` (oracles, allowed) | none | T-D |
| D/🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.toml | dev-only | `portable-pty = "0.9"`, `vt100 = "0.16"` in `[dev-dependencies]` of a separate test package | none | A-3 |

## 8. Feature, fixture and oracle coverage

AGENTS: at least one language-agnostic test per feature, with a third-party oracle. This audit checked file presence and path references; oracle content was not verified (UNVERIFIED).

| file | rule | finding | suggested fix | slice |
|---|---|---|---|---|
| D/⚙️preferences/⌨️keymap/ (new src), D/🧬️schema/⚙️preferences/⌨️keymap/, D/🧫️fixtures/⌨️keymap/🔣️.json | feature needs `🥒️.feature` + fixture + oracle | no `🥒️.feature` anywhere for keymap | add `D/🧪️tests/⌨️keymap/🥒️.feature` with a runner and oracle, or fold scenarios into `⌨️controls` explicitly | A-3 |
| D/🧪️tests/⚙️preferences/🥒️.feature | runner must exist | no test file in the folder; no code references its path (UNVERIFIED that a glob picks it up) | add runner or delete | A-3 |
| D/🧪️tests/📼️replay/🥒️.feature | runner must exist | no test file in folder; no path reference | add runner (fixture folder missing too) or delete | A-2 |
| D/🧪️tests/🧩️groups/🥒️.feature | runner must exist | no test file in folder; no path reference; no fixture folder | add runner and fixture | A-2 |
| D/🧪️tests/🟢️ready/🥒️.feature | runner must exist | no test file in folder; referenced only indirectly | add runner | A-2 |
| D/🧫️fixtures/🔎️launcher/🔣️.json | fixture needs feature | fixture with no feature under `tests`; launcher covered only by unit tests | add `🥒️.feature` + oracle | A-3 |
| D/🏛️repo-domain/, D/🧭️cli/, D/🌀️daemon/✉️ipc/, 🚚️transport/, 👥️clients/, 📜️journal/, 🕹️control/, 🧠️supervisor/, D/🖥️terminal/🌐️labels/, 📋️panes/, 📡️sessions/, 🪟️windows/, 🚀️launcher/ | feature per feature | new modules with no own feature folder; covered only by unit tests (UNVERIFIED mapping) | map each to a feature or add one | A-1 / A-2 / A-3 |
| UI new `🧪️tests/*` folders (e.g. `🔬️chrome`, `🔬️engine`, `🔬️overlays`, `🚦️status-roles`) | feature + oracle | not checked in this pass | UNVERIFIED | T-B / T-C / T-D |

## 9. Progress and cancellation for expensive operations

| file | rule | finding | suggested fix | slice |
|---|---|---|---|---|
| D/🌀️daemon/📼️replay/🦀️.rs | progress + cancel | no `progress` or cancel token in source; replay can be large | add progress events and a cancel flag like `📚️inventory::discover(root, cancelled: &AtomicBool)` | A-2 |
| D/📦️installation/ | progress | cancel present, no progress reporting | add progress | A-1 |
| D/🌊️workflow/, D/🌀️daemon/📜️journal/, D/🖥️terminal/🚀️launcher/ | progress + cancel | none present; expensive status UNVERIFIED | confirm cost; add progress and cancel if expensive | A-1 / A-2 / A-3 |
| D/📚️inventory/ and D/🌀️daemon/🧠️supervisor/ | none | cancel and progress present | none | A-1 / A-2 |

## 10. Hard-coded English in the dashboard view and CLI

| file:line | rule | finding | suggested fix | slice |
|---|---|---|---|---|
| D/🖥️terminal/🌐️labels/🦀️.rs:26-144 | catalogue | all view strings are in the en/de catalogue; each key carries both languages | none | A-3 |
| D/🌀️daemon/🦀️.rs:69 | catalogue for CLI | `eprintln!("usage: semio daemon start|stop|status|attach|serve")` is hard-coded English | route through `ui_locale` en/de | A-2 |
| D/🖥️terminal/🦀️.rs:66 | catalogue for CLI | `eprintln!("[dashboard] unexpected arguments: ...")` hard-coded | route through catalogue | A-3 |
| D/📜️root-delegation/🦀️.rs:16 | catalogue for CLI | `eprintln!("[semio] unknown verb {:?}")` hard-coded | route through catalogue | A-1 |
| D/📦️packages/🦀️rust/🦀️.rs:147 | catalogue for CLI | `eprintln!("[semio] failed to run {cmd}: {e}")` hard-coded | route through catalogue | A-1 |
| D/🏛️repo-domain/🦀️.rs:109,276 | persisted text | `DASHBOARD_CLOSE_SUMMARY`, `"Reopened from the semio dashboard"` written into ticket records in English | low; decide whether persisted text is localized | A-1 |

## 11. Clippy (`cargo +nightly-2026-07-07-x86_64-pc-windows-msvc clippy -p semio-framework-repo-dashboard`)

Clippy was run with `nightly-2026-07-07` because the active `nightly-2026-07-20` has no clippy component. Stable fails on `trim-paths`. Lib target only; test targets were not linted (UNVERIFIED). Output: `🗑️generated/clippy-full.txt`.

Dashboard crate: 35 warnings. Notable:

| file:line | finding | suggested fix | slice |
|---|---|---|---|
| D/🌀️daemon/✉️ipc/🦀️.rs:37,62 | large enum size difference (336 / 456 bytes) | box the large variants | A-2 |
| D/🌀️daemon/👥️clients/🦀️.rs:34 | large enum size difference (336 bytes) | same | A-2 |
| D/📎️connection/🦀️.rs:25,133 | large enum difference (456 bytes); `while let` loop | box variants; use `while let` | A-2 |
| D/🌀️daemon/🧠️supervisor/🦀️.rs:318,327 | unnecessary return values | drop return | A-2 |
| D/🌀️daemon/🧠️supervisor/🦀️.rs:632,803 | by-value argument not consumed | take a reference | A-2 |
| D/🌀️daemon/📼️replay/🦀️.rs:382 | `push_str` with one char | `push('\x18')` | A-2 |
| D/🌀️daemon/📼️replay/🦀️.rs:622,653 | manual `is_multiple_of` | use `is_multiple_of` | A-2 |
| D/🌀️daemon/🚚️transport/🦀️.rs:539 | unnecessary qualification | simplify path | A-2 |
| D/🌀️daemon/✉️ipc/🦀️.rs:510 | method `next` confuses `Iterator::next` | rename | A-2 |
| D/🎮️registry/🦀️.rs:864 | `else if` without `else` | add `else` or restructure | A-1 |
| D/🎮️registry/🦀️.rs:730 | loop variable indexes `lanes` | iterate `lanes` directly | A-1 |
| D/🌊️workflow/🦀️.rs:13,14,47,50,241 | `map(...).unwrap_or(...)` on Option/Result | `map_or` / `is_ok_and` | A-1 |
| D/🛝️playground-session/🦀️.rs:20,44-48 | `map(...).unwrap_or(...)` on Option | `map_or` | A-1 |
| D/⚙️preferences/🦀️.rs:142 | `map(...).unwrap_or_else` | `map_or_else` | A-3 |
| D/📚️inventory/🦀️.rs:403 | by-value argument not consumed | take a reference | A-1 |
| D/🧭️cli/🦀️.rs:175 | variant name ends with enum name | rename variant | A-1 |
| D/🖥️terminal/🚀️launcher/🦀️.rs:109,248,290 | loop index; by-value arguments | iterate directly; take references | A-3 |
| D/🖥️terminal/🪟️windows/🦀️.rs:32 | large enum size difference (464 bytes) | box variants | A-3 |
| D/🖥️terminal/⌨️controls/🦀️.rs:180 | by-value argument not consumed | take a reference | A-3 |

UI crate (`semio-framework-ui`, 61 warnings) that touches the TUI, notable items:

| file:line | finding | slice |
|---|---|---|
| UI/⌨️tui/🪟️windows/🦀️.rs:8,9,39,142,171,184 | non-camel acronym names (`HANDLE`, `HPCON`, `COORD`, `STARTUPINFOW`, `STARTUPINFOEXW`, `OVERLAPPED`) | T-A |
| UI/⌨️tui/🪟️windows/🦀️.rs:33,227 | unused `WAIT_FAILED`, `PeekNamedPipe` | T-A |
| UI/⌨️tui/🔌️backend/🦀️.rs:42,109,112,465,1095 | `div_ceil` manual; `map.unwrap_or(false)`; unneeded `return`; by-value arg | T-A |
| UI/⌨️tui/🚇️pty/🦀️.rs:552,650,693,762,763 | field assignment after default; unnecessary `Option`; by-value arg; `map.unwrap_or` | T-A |
| UI/⌨️tui/🎬️scene/🦀️.rs:32 | large enum difference (608 bytes) | T-B |
| UI/⌨️tui/⚙️engine/🦀️.rs:977 | `map.unwrap_or` | T-B |
| UI/⌨️tui/📏️layout/🦀️.rs:12,71,83,539,563 | derivable impl; manual checked division; `sort_by_key`; by-value args | T-B |
| UI/⌨️tui/🖥️chrome/🦀️.rs:157,713,740,749 | large enum difference (304 bytes); too many args (8/7); `map.unwrap_or`; by-value arg | T-B |
| UI/⌨️tui/🪀️widget/🦀️.rs:130,157 | large enum difference (608 bytes); `map.unwrap_or` | T-C |
| UI/⌨️tui/📟️vt/🧱️screen/🦀️.rs:232,821,836,1030,1203 | `T::default()`; `repeat().take()` x2; `map.unwrap_or`; `map_or` | T-C |
| UI/⌨️tui/📟️vt/🖥️pane/🦀️.rs:193,347,539 | unnecessary `Option` wrap; manual checked division; too many args (8/7) | T-C |
| UI/⌨️tui/📜️rows/🦀️.rs:143,459 | manual `is_multiple_of`; manual checked division | T-D |
| UI/⌨️tui/🔲️cell/🦀️.rs:213 | too many args (8/7) | T-D |
| UI/⌨️tui/📝️text/🦀️.rs:71 | manual `is_multiple_of` | T-D |
| UI/⌨️tui/📝️text/🔤️tables/🦀️.rs:14,22 | acronym names `ZWJ`, `LVT` | T-D |
| UI/🧱️elements/*/🎯️targets/⌨️tui/🦀️.rs (Divider:12, Input:36, Label:14, Select:16,29,49,50, Table:178,182,197,256, Window:104,127,179, Tree:307, Toggle:29, Wizard:96) | unnecessary `Option` wraps; `map.unwrap_or`; too many args (10/7, 8/7); unnecessary closure in `bool::then` | T-D |

Tests (`--tests` not run): UNVERIFIED.

## 12. Non-concise code

A 6-line sliding-window scan of changed `.rs` files found only 6 repeated windows, all trivial (`let after = Instant::now();`, Unicode range tables, `let y = usize::from(self.cursor.y);`). No large duplicated block was found. Size concerns are the Clippy `too_many_arguments` and large enum variants above, not duplication.

## 13. Not checked / UNVERIFIED

- Feature oracles beyond file presence (§8).
- UI feature and oracle coverage (§8).
- Test-target clippy (§11).
- Whether `D/🧪️tests/*/🥒️.feature` files without a path reference are picked up by a runner glob.
- Runtime behaviour of any change (no tests were executed in this read-only pass).
