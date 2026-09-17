# 🔱️ Trinity plugin end to end — status

## 2026-09-17

- Ticket opened on disk (repo MCP timed out). Pattern follows 26/09/16/LAYOUT-PLUGIN-END-TO-END.
- Playgrounds already declared in `📦️packages/🦀️rust/Cargo.toml`: `trinity-jack` (react 6054) and `trinity-rewriting` (react 6056), both with `app` pins.
- Audit before any runtime run (same generic fault family as forms/shooting/layout):
  1. Neither editor overrides `command_from_action` → every shell action refused as "not framework-reserved".
  2. Jack: `patchNodes`/`deleteSelection`/`setActiveExample`/`setFixtureJson` `BatchOnlyPendingRewrite`; rewriting: all eight document verbs `BatchOnlyPendingRewrite` → hard-dead at dispatch.
  3. Rewriting has no `build_artifact_store_one_item_preparation_factory` (jack has one).
  4. Jack `reset_document_effect` minted a live `ArtifactEnvelope` only to print its spr → the process3d Drop trap on `setActiveExample`.
- Native `cargo check -p semio-s-plugin-trinity --lib` was already red before any edit (committed peer work):
  - jack: `📝️editor` window used `fixture` instead of `snapshot`, `📊️results` the reverse; `TRINITY_JACK_PLAY_CONTROLLER_ID` private but used by the A8 artifact panel `interaction_domain`; both inspection panels used `Label::try_from(&str)` on the plugin `Label` (→ `ui_label`).
  - rewriting: commit 3250e6cb90 (09-15) renamed callers to `Graph::host_snapshot_json` but jack's method stayed `fixture_json` → renamed in jack (+ its unit test).
- Fixes applied:
  - Jack: `args_bridge::command_from_action` (all 11 verbs; camelCase keys, stringified control values, `nodeIds` list/JSON/bare id, nested or flat viewport); `JackRetainedDocumentJobFactory` (`patchNodes`/`deleteSelection` → `Artifact`, `setActiveExample`/`setFixtureJson` → `HostOnly` LoadDocument) with proofs, `build_tool_job` routing and manifest flipped to `Migrated`; `reset_document_effect` now uses `store::empty_document_spr`.
  - Rewriting: `args_bridge::command_from_action` (all 10 verbs; `operations` array → `operations_json`); `RewritingDocumentJobFactory` for the eight document verbs (`resetRule` → `HostOnly`, rest `Artifact`); one-item artifact preparation factory (60 kB mutation bound for `edit-before-fixture`); manifest flipped to `Migrated`.
- Scripts: `📜️check-trinity.sh`, `📜️activate-trinity-{jack,rewriting}-react.sh`, `📜️serve-trinity-{jack,rewriting}-react.sh`, `🐍️trinity-console-dump-probe.mjs`.
