# Runtime verification — procedural 3d on the React target (port 6018)

Coordinator-run in the Claude browser pane (viewport emulated 1440×900, tab hidden), against the 19:35 build served by
`serve-generation3d-react-dev` (`📓️wave1-boot-report-2026-09-09.md`). URL `http://127.0.0.1:6018/?plugin=generation3d`.

## 19:50 boot #1

| t | observation |
|---|---|
| +10 s | title `semio · os`, root present, body 103 chars, 0 canvases |
| +40 s | title `semio · procedural · 3d`; chrome renders: panel tabs Artifact/Catalogue/Inspection, Fullscreen, mode tabs Edit/Generate, example picker = **Hexagonal Mushroom Column**; **no window bodies, 0 canvases** — in their place one fault card |
| fault card | `{"code":"plugin.internal","message":"surface context exceeds its wire bound","origin":"plugin","retryable":false}` |
| console | Rust panic in the plugin worker: `ordered-map root must be explicitly retired before drop` at `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81:57`, stack: `OrderedMap<WidgetLayout>::drop` ← `FlowFixture` drop glue ← `FlowHost` drop glue ← `Generation3dPlayApp::pending_effects` closure ← `with_scratch_session` → `unreachable` trap; actor `procedural#1` trapped |
| console | after the trap every turn fails: `runtime instance authority is busy` (`plugin.internal`, retryable=false); `action failed setActiveExample {exampleId: hexagonal-mushroom-column}`; `local interaction observation failed`; `[DEBUG] reactor more-work streak=… command_ingress=true` |

Verdict: shell boots, plugin actor dies on its first `pending_effects`. Two defects to fix before any window can be judged:
1. **Retirement panic** — a scratch `FlowHost`/`FlowFixture` built by `with_scratch_session` is dropped without retiring its cold-tracked `OrderedMap` roots (`layout`).
2. **Surface wire bound** — a window surface context exceeds the framework's wire bound; which surface and which bound to be established (mesh payload of the preview vs flow graph scene).

## 00:20 (09-10) boot #2 — build 23:5x (`📓️rebuild-2026-09-09.md`), serve restarted from the main session (`$S/serve-react.sh`)

| t | observation |
|---|---|
| +40 s | title `semio · procedural · 3d`; chrome as before; **no trap, no wire-bound fault** (both boot #1 defects gone) |
| window area | one status line instead of window bodies: `plugin-ui.intake-budget-exhausted:1:procedural-main:74736` — the flow window surface (74 736 B) exhausts the retained-UI intake budget; this is the catalogue-embedding defect (`📓️unit-suite-2026-09-09.md` §3.1), catalogue lane in flight |
| console | `[DEBUG] thunk failed command-ingress:procedural#1 typed-operation failed: validation failed: batched item candidate failed its exact fixed fold contract`; same message on `action failed setActiveExample {exampleId: hexagonal-mushroom-column}` and `local interaction observation failed` |
| console | `[DEBUG] typed-operation publication turn=1024 operations=["27:Retiring:true:true"] latest_wins_empty=true …` — the retained typed operation for the example switch retires without publishing |
| DOM | 0 canvases, no `framework.window.*` ids rendered |

Verdict: actor survives now; two blockers remain — (1) flow window surface over the intake budget (catalogue lane), (2) every typed operation fails the "batched item candidate … exact fixed fold contract" validation, so no action (example switch, interaction observation) publishes.
