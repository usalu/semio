# Wave B16 — leftover export download and distinct import

Implementation pass, 2026-09-11. Picks up #45b `export-only FAIL download=none` / `import-distinct FAIL before=1 after=1`, B6 leftover-payload hop, B9 "excluded by construction", and B13's proof that `#action.exportFixture` already downloads when the Actions pane is actually clicked.

Every command tail quoted below is real output from this pass.

---

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`. No `git commit`/`stash`/`checkout`, no worktree, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, everything foreground. The ticket was not opened/closed here.
- The repo MCP server is not connected in this session (`GetDynamicTools` catalog has no repo namespace), so the ticket folder is managed on disk.
- `:6014` returned HTTP 200. `:6013` was not used.
- `[DEBUG] ` logs: **none added**; none removed. B6's `puzzle3d.import.ingress/parsed/apply` taps stayed and fired on the leftover distinct-import law (`payload_len=809` `objects=2` `before=1` `ops=1` `after_objects=2`).
- Peers were live in the same files (B9–B14 leftover/guest/Actions). No peer hunk was reverted. `importFixture => Puzzle3dWindowCommandWork` is kept verbatim so the #44 source-string law still matches.

---

## 1 Remaining hop — factory fallthrough, not a guest fold

### Root cause

`dispatch()` already downloaded `puzzle-3d.json` and leftover `importFixture` already replaced a one-object live fixture with a two-object JSON (B9/W-AB laws green). Browser leftover does not call `dispatch()` — it admits a retained job from the factory match, then leftover-commits.

The factory match at `✏️editor/🦀️.rs` routed only `importFixture` through `Puzzle3dWindowCommandWork`. `exportFixture` and `openImportFixture` fell through to `BoundedFirstStepCommandWork`. That is the leftover hop B6/B13 named: HostOnly download / file-picker effects never entered the leftover-commit job that the browser actually runs.

B13 already proved the user-visible Actions row `#action.exportFixture` downloads 7542-byte `puzzle-3d.json` when the pane stays open. #45b's `download=none` is the canvas-menu ordinal / Actions-fold probe, not a missing `DownloadMediaExport`. Distinct import still needed the leftover factory path for the picker (`openImportFixture`) plus the already-routed `importFixture` payload.

### Fix

Three explicit leftover arms next to the existing `importFixture` arm (not a combined `|` arm — the #44 law asserts the exact `importFixture` string):

```
"exportFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
"openImportFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
"importFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
```

`exportFixture` is **not** added to `puzzle3d_shell_only_emit` — it needs the live fixture and goes through Scene → `dispatch_step`. `openImportFixture` stays in `puzzle3d_shell_only_emit` (`RequestFileOpen` accept JSON, `import_action=importFixture`), so leftover Scene short-circuits the picker without a scene.

No leftover HostOnly publication change. WindowCommandWork already publishes `requested_effects` the same way leftover `importFixture` does.

### What it is deliberately NOT

- Not a probe edit. B13's Actions-pane click already downloads; #45b's menu ordinal / last-Actions-button fold is a locator lie.
- Not a guest-fold fix. B9's `effects:0` on a same-file reimport is identity, and the leftover distinct-import law now records `after_objects=2` + `history_patch`.
- Not a revert of B9–B14.

---

## 2 Laws (3 new, `✏️editor/🧪️tests/🔬️unit/🦀️.rs`)

- `leftover_export_fixture_downloads_puzzle_3d_json` — source-string `exportFixture => Puzzle3dWindowCommandWork`; leftover emit is `DownloadMediaExport` filename `puzzle-3d.json`.
- `leftover_open_import_fixture_requests_file_open` — source-string `openImportFixture => Puzzle3dWindowCommandWork`; leftover emit is `RequestFileOpen` accept JSON / `import_action=importFixture`.
- `leftover_import_fixture_replaces_live_document_with_distinct_two_object_json` — leftover `importFixture` of a two-object payload against a one-object live document → `after_objects=2` and `history_patch`.

Filter `leftover_` (11 tests, includes peer leftover copy/paste/inspection):

```
running 11 tests
test editor::puzzle3d::component::tests::leftover_export_fixture_downloads_puzzle_3d_json ... ok
test editor::puzzle3d::component::tests::leftover_import_fixture_replaces_live_document_with_distinct_two_object_json ... ok
test editor::puzzle3d::component::tests::leftover_open_import_fixture_requests_file_open ... ok
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 694 filtered out; finished in 1.03s
```

Distinct-import DEBUG from that run:

```
[DEBUG] puzzle3d.import.ingress args=true payload_len=809
[DEBUG] puzzle3d.import.parsed objects=2 before=1
[DEBUG] puzzle3d.import.apply ops=1 after_objects=2
```

Existing export/import laws plus the three new ones (8 tests):

```
running 8 tests
test editor::puzzle3d::component::tests::leftover_open_import_fixture_requests_file_open ... ok
test editor::puzzle3d::component::tests::export_fixture_downloads_round_trippable_json ... ok
test editor::puzzle3d::component::tests::leftover_export_fixture_downloads_puzzle_3d_json ... ok
test editor::puzzle3d::component::tests::leftover_import_fixture_replaces_live_document_with_distinct_two_object_json ... ok
test editor::puzzle3d::component::tests::import_fixture_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations ... ok
test editor::puzzle3d::component::tests::open_import_fixture_requests_file_open_then_import_applies_payload ... ok
test editor::puzzle3d::component::tests::import_fixture_reproduces_the_exported_document ... ok
test editor::puzzle3d::component::tests::exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 697 filtered out; finished in 12.02s
```

Full tails: `🗑️generated/b16-leftover-laws.txt`, `🗑️generated/b16-related-laws.txt`.

Crate `semio-s-artifact-puzzle-3d`, feature `component-app-assembly`, `RUST_MIN_STACK=134217728`, `--test-threads=1`. Factory arms now at editor.rs ~7777.

---

## 3 LAND

- Leftover `exportFixture` downloads `puzzle-3d.json`.
- Leftover `openImportFixture` opens the JSON picker that completes as `importFixture`.
- Leftover distinct `importFixture` replaces objects (`after_objects=2`) and upserts history.
- No git. No `:6013`. Ticket left open.
