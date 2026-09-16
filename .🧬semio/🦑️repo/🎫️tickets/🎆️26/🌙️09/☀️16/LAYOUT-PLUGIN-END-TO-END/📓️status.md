# 📏️ Layout plugin end to end — status

## 2026-09-16

- Ticket opened on disk (repo MCP timed out).
- Playground pin: `app = "s.layout.layout@1/*#editor"` added to the layout `Cargo.toml` playground block (same fault family as forms: `primary plugin layout does not declare pinned app ""`).
- Audit of the editor root (`✏️editor/🦀️.rs`) before any runtime run:
  1. No `command_from_action` override → every shell action would be refused as "not framework-reserved" (forms fault 3). Added `args_bridge` (snake-case fold, integral-float restore, `value` stringify for `patchPage`/`patchFrame`/engagement rows, flat `{x,y,zoom}` → `camera` nest, `shiftKey`→`extend`, palette default `kind=rect`) + `command_from_action` override; two unit tests (`command_from_action_round_trips_every_command_id` also asserts payload equality, `…_bridges_host_control_contracts`).
  2. Six verbs still `BatchOnlyPendingRewrite` (`addFrame`, `addPage`, `patchPage`, `patchFrame`, `canvasPointerDown`, `canvasPointerMove`) → hard-dead at dispatch (`interactive-job.not-ui-safe`) — the very verbs that persist a document mutation. Migrated: added to `LAYOUT_RETAINED_TOOL_IDS`, `PUBLICATION_CONTRACTS` (`addFrame`/`patchPage`/`patchFrame` → `[Artifact]`, `addPage` → `[Artifact, WindowConfig]`, pointer down/move → `[HostOnly]`), the proofs `tools` row, `LayoutWindowWork::step` arms (AddPage mirrors `handle`'s active-page switch when a window is addressed), manifest flipped to `Migrated`. `extent`/`step` no longer require a view state for document-lane verbs (window-lane verbs still fail closed with `layout-window-view-required`).
  3. Panels/inspector rows are keyed via `try_id` (key ← id), so the forms `#0` DuplicateSiblingKey family does not apply here.
- Scripts: `📜️activate-layout-react.sh` (activate-layout-react-dev, 6079), `📜️serve-layout-react.sh` (`?plugin=layout`), `🐍️layout-console-dump-probe.mjs`.
- 00:03Z `cargo check -p semio-s-plugin-layout --lib` blocked: `semio-framework-ui-contract` fails (8 errors, `TreeItemProps` missing `window`/`granularity` in the `ui_typed_field_catalog!` mirrors) — a peer's in-flight ARTIFACT-TREE-VIRTUALISED-STREAMING refactor (9 unstaged files under `🖱️ui/🧬️contract` modified in the last 30 min). Polling for quiescence before re-running.
