# 🗒️ Note plugin end to end — status

## 2026-09-17

- Ticket opened on disk (repo MCP timed out). No prior NOTE-*-END-TO-END ticket.
- Audit of the editor root (`✏️editor/🦀️.rs`) against the layout/forms fault families:
  1. Playground `app` missing in `📦️packages/🦀️rust/Cargo.toml` → added `app = "s.note.note@1/*#editor"` (forms fault 1: `primary plugin note does not declare pinned app ""`).
  2. No `command_from_action` → every shell action refused as "not framework-reserved". Added `args_bridge` (snake fold, integral floats, text/number/bool coercion of control `value`s, panel quick-add `addBlock {kind}` defaults `x/y=0`, `inkApplyEvents` `events`→`events_json` + phase default, flat `{x,y,zoom}`→`camera`, `blockId`/`id`→`block_ids` list, null `engagementSubmit.value` stays `None`) + two unit tests (every row round-trips from camelCase; host control contracts).
  3. 26 of 34 verbs still `BatchOnlyPendingRewrite` (every document verb: `addBlock`, `patchBlocks`, `inkApplyEvents`, `deleteSelection`, nudges, grid/snap/pencil/eraser options, `setActiveExample`, `setFixtureJson`, `saveDownload`) → hard-dead at dispatch. `NoteCommandWork` was already generic over every `NoteCommand`, and the full contract table already existed as the unused `NOTE_AUDITED_*` lists — collapsed them into `NOTE_RETAINED_TOOL_IDS`/`NOTE_RETAINED_PUBLICATION_CONTRACTS`, proofs `tools` now lists all 34 rows, manifest flipped to `Migrated`.
  4. The artifact one-item preparation factory (layout fault 4) already exists in note.
- `cargo check -p semio-s-plugin-note --lib` green (10:26 local, 9m39s); the one new warning (qualified `DslValue`) fixed before restage.
- Restage #1 started 10:27 local: `screen note-activate` → `🗑️generated/activate-note-react.txt`.
- Restage #1 done 11:04 local (32 min under load avg 170, `component-dev` 23 min). Serve: `screen note-serve` (vite 6080, `?plugin=note`).
- Boot probe #1: every module 500 — `🏛️ShellHost/🟦️.tsx` did not parse. Root cause and repair (15 renderer/kernel/actor files damaged by a peer's `[DEBUG]` console strip codemod, including silent behaviour changes in action dispatch): `📓️fix-2026-09-17-fused-debug-strip.md`.
- Boot probe #2 (`🗑️generated/note-boot-2`): `data-semio-os-ready=note`, Canvas + Navigator windows, Artifact/Catalogue/Inspection panels. Boot `setActiveExample` (now dispatchable) trapped: `artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner` → `reset_document_effect` minted a live envelope just to print the spr (process3d fault 1). Now `store::empty_document_spr("note", NOTE_DOCUMENT_SCHEMA)`; also added `build_document_store_initialization_job` (`bounded_document_store_initialization_job`, fem2d shape) — the trait default refuses every `Effect::LoadDocument`. No `type Members` needed: `NoteSnapshot` declares no `#[child]` field, so `visit_child_refs` never reaches the nested text handles.
- Known gap (host, not note): `InkCanvasHost` dispatches `setSelection`/`setHover` to the app; nothing maps them onto the framework-owned `interactionSelect`/`interactionHover`, so canvas-click selection is refused for ink-canvas apps.
- Restage #2 started 11:21 local.
