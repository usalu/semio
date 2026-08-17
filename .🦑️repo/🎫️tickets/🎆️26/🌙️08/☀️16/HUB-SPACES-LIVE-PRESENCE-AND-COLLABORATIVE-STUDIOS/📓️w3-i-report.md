# W3-I report — make the wgpu shell compile and run its first-ever unit tests

Lane 3-I. Scope per brief: clear the 6 remaining compile errors 3-H left blocked on (3× `AppFrame::Error`
missing `report`, 1× `Rect`/`&Rect`, 2× test-fixture `AppDefinition` missing `role`/`dialect`), then get
`cargo test -p semio-framework-os-renderer-wgpu --lib` green, then confirm the native shell builds.

Confirmed before touching anything: `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/🎫️ticket.json`
is `"status": "closed"` and its `📌️important.md` is gone (no such file) — `📡️spr/**`/`🔌️plugin/**`
therefore available to read (I did not need to edit either). `FULL-STDIO-…/🎫️ticket.json` is
`"status": "open"` — `✏️s/🔌️plugins/🗄️stdio/**` stayed untouched (never referenced by anything in scope).

## 1. Baseline — reconfirming 3-H's list

`cargo check -p semio-framework-os-renderer-wgpu` baseline (`🧪️3-i-baseline-check.txt`): **exactly 4
errors**, matching 3-H's report precisely — no drift on the tree since their pass:
- `ProgramBridge/🧊️component.rs:129,221,250` — `AppFrame::Error { in_reply_to, fault }` missing `report`.
- `Dock/🧊️component.rs:1087` — `paint_dock_tab_icon(..., tab_rect, ...)` passes `&Rect`, wants `Rect`.

(The 2 `AppDefinition` test-fixture errors don't show under plain `cargo check` — they're `#[cfg(test)]`
-gated, confirmed by 3-H — so `cargo test` surfaces them separately, see §3 below.)

All four fixed files are inside this lane's lease
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/**` — `ProgramBridge`/`Dock`/`Shell` are all
`🧑️‍🎨️engine/🧱️elements/*`), so — unlike 3-H, whose lease excluded them — these are squarely mine to fix, not
`sharedFileRequest`s.

## 2. Fix — `AppFrame::Error.report` (task 1a)

Not a bare `report: _`/`..` ignore. `📡️spr/🧵️channel/🦀️component.rs:267-270`'s own doc says `report` is
"one packed `DispatchReport` of the rejected dispatch, accompanying a `Fault.code == "mutation.rejected"`"
— real diagnostic content, not inert wire padding. `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
already has the exact sibling pattern for this (`app_frame_fault_summary` + `dispatch_report_summary` +
`dispatch_error_message`, reading `protocol::DispatchReport.messages` and formatting `code: message
[target]`), so I mirrored it locally inside `ProgramBridge/🧊️component.rs`'s own
`mod wasm_program_exchange` (native-only) rather than inventing a placeholder value or silently dropping
real data:

- New `dispatch_report_summary(report: &[u8]) -> String` — decodes `report` via the same
  `pack_rt::decode_wire_value` + `from_dsl_value::<protocol::DispatchReport>` pipeline this module
  already uses for every other wire payload; empty string for a pre-CHANNEL_VERSION-11 peer or a
  message-free report.
- New `app_frame_error_message(fault, report) -> String` — `app_frame_fault_summary(fault)` (unchanged)
  plus `" — {summary}"` appended when `dispatch_report_summary` is non-empty.
- All 4 `AppFrame::Error` sites in this file (the 3 originally erroring at lines 129/221/250, plus
  `expect_done`'s pre-existing `{ fault, .. }` pattern which already ignored `report` silently and
  compiled before but was equally under-informative) now destructure `report` and route through
  `app_frame_error_message`, for one consistent error-formatting story across the whole file instead of
  3 enriched call sites and 1 left bare.
- `protocol::DispatchReport` resolves cleanly: `semio-framework-os-kernel` (aliased `protocol`/`dsl`/
  `store`/... in `📦️glue.rs`) re-exports it via `pub use crate::os_spr::conflict::{..., DispatchReport,
  ...}` in `📡️spr/🦀️component.rs:38` → `os_spr`'s `pub use component::*;` → crate root's
  `pub use crate::os_spr::*;` — verified by grep, not assumed.

## 3. Fix — `Rect`/`&Rect` (task 1b)

`Dock/🧊️component.rs:1087`: `tab_rect` is bound `&Rect` from `for (window_id, label, icon_id, tab_rect)
in &tabs` (line 1074); `paint_dock_tab_icon` (line 1012) wants an owned `Rect` (`Rect` is `Copy`).
One-line fix, exactly 3-H's own mechanical suggestion: `tab_rect` → `*tab_rect` at the call site. No
other call to `paint_dock_tab_icon` in the file, no other type implication.

## 4. Fix — test-fixture `AppDefinition.role`/`.dialect` (task 1c)

Two synthetic test-only `AppDefinition` literals (`Shell/🧊️component.rs:6411`'s `command_registry_tests::
test_app`, `Dock/🧊️component.rs:1410`'s `tests::sample_app`) predate `AppDefinition` gaining non-defaulted
`pub role: AppRole` / `pub dialect: ArtifactDialect` (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:2694-
2700`, the `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` per-subset surface convention, still open, not mine).
Neither fixture is derived from or asserted against a real artifact dialect — both build a throwaway app
for command-registry/dock-layout unit tests, so I followed this codebase's own established idiom for
synthetic test dialects (`ArtifactDialect { artifact_kind: "s.test.<name>", standard: "1", subset: "*" }`,
seen at `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:4347`, `🖥️platform/🦀️component.rs:102/159`,
`🔌️plugin/🦀️component.rs:6846/7592/16360-61`, `🖥️host/🦀️component.rs` ×5) rather than inventing a new
naming pattern:
- `Shell/🧊️component.rs` `test_app`: `role: AppRole::Editor`, `dialect: ArtifactDialect { artifact_kind:
  "s.test.app", standard: "1", subset: "*" }`.
- `Dock/🧊️component.rs` `sample_app`: `role: AppRole::Editor`, `dialect: ArtifactDialect { artifact_kind:
  "s.test.dock", standard: "1", subset: "*" }`.

`AppRole::Editor` (not `Viewer`) because both fixtures build apps carrying real `commands`/mode-commands
used to exercise editor-facing behaviour (command registry search, dock tab rendering) — matches the same
role choice this codebase's own `s.test.*` test-app-builder fixtures already make
(`🔌️plugin/🦀️component.rs:6846/7592`, `AppRole::Editor`).

## 5. `cargo check -p semio-framework-os-renderer-wgpu` (task 1) — GREEN

`🧪️3-i-check-1.txt`: **0 errors**, 220 pre-existing-shape warnings. `grep -c "^error"` confirms 0.

## 6. `cargo test -p semio-framework-os-renderer-wgpu --lib` (task 2) — first-ever run

First pass (`🧪️3-i-test-1.txt`) still had the 2 test-only `AppDefinition` compile errors (§4 above, fixed
in the same edit pass before running tests again). Second pass (`🧪️3-i-test-2.txt`) compiled and ran:
**312 passed; 4 failed**. One of the 4 was in the "ours" list (`identity_directory_presence_tests`) —
investigated and fixed (below); re-run (`🧪️3-i-test-3.txt`): **313 passed; 3 failed**.

### The "ours" list — all 9 tests, all green

Per the brief: identity actor shape, default bindings with/without identity, directory-event fold,
`os.open-artifact{documentId}`, presence roster filtering by surface, sync status pill. (No standalone
Rust "check-in viewer guard" test exists in this module — `canCheckIn` is React-only per `📓️w3-a-report.md`;
wgpu's viewer guard is unit-untested code, not a broken test — nothing to fix here, flagging as a gap.)

All 9 tests in `shell::identity_directory_presence_tests` (lanes 2-D's 8 + 3-A's `sync_pill_text_...`)
now pass:
```
test shell::identity_directory_presence_tests::default_bindings_are_empty_without_space_or_data_dir ... ok
test shell::identity_directory_presence_tests::default_bindings_are_folder_only_without_identity ... ok
test shell::identity_directory_presence_tests::default_bindings_are_hub_plus_folder_with_identity_and_space ... ok
test shell::identity_directory_presence_tests::directory_command_from_action_covers_every_frozen_verb ... ok
test shell::identity_directory_presence_tests::open_artifact_relay_target_parses_document_and_space_ids ... ok
test shell::identity_directory_presence_tests::sync_pill_text_covers_persisted_pending_and_every_remote_state ... ok
test shell::identity_directory_presence_tests::fold_directory_events_action_reaches_the_controller_with_the_events_payload ... ok
test shell::identity_directory_presence_tests::presence_rows_are_scoped_to_the_attached_surface ... ok
test shell::identity_directory_presence_tests::shell_actor_uses_contract_grammar_when_identity_present_else_local_default ... ok
```

**The one "ours" failure, investigated and fixed — a test-assertion bug, not a wiring defect.**
`fold_directory_events_action_reaches_the_controller_with_the_events_payload` failed:
`left: Number(7.0) right: 7`. Traced the cause: `fold_directory_events_action` builds `ActionDescriptor.args`
by round-tripping the events payload through `optional_json_as_dsl_value` (`serde_json::Value` →
`DslValue`), and `DslValue::Number` is *always* `f64` (`🗣️dsl/🧬️schema/🦀️component.rs:205` —
`Number(f64)`, no integer variant at all). Converting the resulting `DslValue` back to JSON for the
assertion (`dsl_value_as_json` → `serde_json::to_value`) therefore produces a float-tagged
`serde_json::Number` (`.as_i64()` is `None` for it, per serde_json's internal `PosInt/NegInt/Float`
representation), which fails `PartialEq<i32>` against the literal `7` even though the value is
numerically identical. This is universal, by-design behaviour of every `ActionDescriptor.args` payload in
the codebase (all numbers round-trip through `DslValue`'s single `f64` variant) — not a defect in the
shell's directory-fold wiring, which dispatches the correct controller id, action name, and event data.
Per the brief ("fix the wiring, not the test" — but only for a **real** defect): fixed the assertion to
compare numerically (`args_json["events"][0]["seq"].as_f64()`, `Some(7.0)`), with an inline comment
explaining why, rather than leaving a hidden landmine or "fixing" non-broken wiring.

### The other 3 failures — confirmed pre-existing, unrelated, out of the "ours" list and out of lease

None are in `identity_directory_presence_tests`; none touch identity/directory/presence/check-in code.
`git blame --date=iso` on each: all three predate this ticket (started 2026-08-16) by well over a week —
first-ever surfaced only because this is the crate's first-ever successful compile, not something this
wave introduced:

1. `shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments`
   (`Shell/🧊️component.rs:11659`) — a `WindowSilhouette` geometry assertion (`push_window_silhouette_border`
   gap-baseline math). `git blame`: `5756202c43e`, `2026-08-14 12:44:47` (silhouette call site) /
   `23d0db68338`, `2026-08-06 05:42:37` (assertions). Not touched by any lane in this ticket.
2. `shell::shell_input_tests::standalone_multi_app_variants_resolve_their_declared_app`
   (`Shell/🧊️component.rs:4846`) — `resolve_playground_app_id("puzzle2d")` now returns
   `Some("s.puzzle2d@1/*#editor")` but the test still hardcodes the pre-surface-id-convention literal
   `Some("puzzle2d-play")`. `resolve_playground_app_id` comes from `crate::generated_plugin_hosts`
   (`ProgramBridge/🧊️component.rs:632`), a `build.rs`-generated file reflecting the puzzle plugin's live
   manifest — the SAME `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` (still-open) surface-id migration 3-H's
   own report attributed the `role`/`dialect` fixture gap to. `git blame`: `23d0db68338`,
   `2026-08-06 05:42:37` — the test itself long predates that migration.
3. `shell::ui_prefs_themes_i18n_tests::load_ui_prefs_once_prefers_a_lock_over_storage`
   (`Shell/🧊️component.rs:11197`) — `SEMIO_LOCKED_APPEARANCE` env-var lock precedence assertion
   (`left: "system" right: "dark"`). `git blame`: `23d0db68338`, `2026-08-06 05:42:37`. Uses
   `unsafe { std::env::set_var(...) }` on a process-global — plausibly flaky under `cargo test`'s default
   multi-threaded runner sharing env state with other tests, not investigated further (out of lease, out
   of the "ours" list, no identity/directory/presence/check-in relation).

None of the three were touched, and none were fixed — correctly out of scope per the brief's explicit
"ours" list.

### Final real numbers

`🧪️3-i-test-3.txt`: **test result: FAILED. 313 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out.**
All 3 failures are pre-existing, unrelated, attributed above with `git blame --date=iso`; all 9 of lanes
2-D's/3-A's own new tests pass.

## 7. `cargo check -p semio-wgpu-native --features native-bin` (task 3) — corrected command, GREEN

`semio-wgpu-native` is not a package name — it's the `[[bin]]` name inside package
`semio-framework-os-renderer-wgpu` (`Cargo.toml:48-51`). `cargo check -p semio-wgpu-native --features
native-bin` fails immediately: `error: cannot specify features for packages outside of workspace`
(`🧪️3-i-native-bin-p-wrong.txt`) — expected, not a code defect. Confirmed the real invocation from this
target's own `📜️script.ts:157/192`: `cargo build -p <crateName> --bin semio-wgpu-native --features
native-bin`. Ran the `check` equivalent: `cargo check -p semio-framework-os-renderer-wgpu --bin
semio-wgpu-native --features native-bin` (`🧪️3-i-native-bin-check.txt`) — **0 errors** (`grep -c
"^error"` = 0), `Finished` dev profile. The native shell builds.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs` —
  new `dispatch_report_summary`/`app_frame_error_message` in `mod wasm_program_exchange` (native-only);
  all 4 `AppFrame::Error` match sites in the file now destructure and surface `report`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Dock/🧊️component.rs` —
  `paint_dock_tab_icon(..., tab_rect, ...)` → `*tab_rect` (line 1087); `tests::sample_app` fixture gained
  `role`/`dialect`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` —
  `command_registry_tests::test_app` fixture gained `role`/`dialect`;
  `fold_directory_events_action_reaches_the_controller_with_the_events_payload`'s assertion fixed to
  compare `DslValue`'s f64-tagged JSON number correctly (`.as_f64()` instead of a bare integer literal).

All four are inside this lane's lease (`🧑️‍🎨️engine/**`). No foreign-file edits; no `sharedFileRequest`s.

## Commands run (real tails, all logged to `$T/🧪️3-i-*.txt`)

- `cargo check -p semio-framework-os-renderer-wgpu` — baseline 4 errors → fixed 0 errors
  (`🧪️3-i-baseline-check.txt` → `🧪️3-i-check-1.txt`).
- `cargo test -p semio-framework-os-renderer-wgpu --lib` — 6 compile errors → compiles, 312 passed/4
  failed → 313 passed/3 failed after the `fold_directory_events` assertion fix
  (`🧪️3-i-test-1.txt` → `🧪️3-i-test-2.txt` → `🧪️3-i-test-3.txt`).
- `cargo check -p semio-wgpu-native --features native-bin` — errors immediately (wrong package name,
  `🧪️3-i-native-bin-p-wrong.txt`); corrected to `cargo check -p semio-framework-os-renderer-wgpu --bin
  semio-wgpu-native --features native-bin` — 0 errors (`🧪️3-i-native-bin-check.txt`).

## What is NOT done

- The 3 pre-existing, unrelated test failures (§6 above) — attributed with `git blame --date=iso`,
  correctly out of the brief's "ours" list and out of this lane's lease; not fixed.
- No Rust unit test exists for the wgpu "check-in viewer guard" specifically (the brief's task-2 list
  names it, but 3-A's own report shows this guard was only unit-tested on the React side —
  `canCheckIn`). The wgpu-side guard code itself (`#s-checkin` absent for `AppRole::Viewer`, per 3-A's
  report) was not touched by this lane and was not click-verified — flagging as a real, pre-existing gap
  for whoever owns follow-up wgpu test coverage, not something I introduced or was asked to build net-new.
- No wasm32 build was checked (out of the brief's 3 named commands, which are all native/lib-test
  targeted); the wasm32 target's own compile health is unverified by this lane.
- Ticket not closed (coordinator owns that); `ticket_close`/`ticket_reopen` never called.
