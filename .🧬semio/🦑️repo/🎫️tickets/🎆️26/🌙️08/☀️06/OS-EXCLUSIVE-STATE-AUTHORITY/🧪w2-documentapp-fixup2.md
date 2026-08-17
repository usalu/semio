# W2 — DocumentApp fixup pass 2

**Ticket:** OS-EXCLUSIVE-STATE-AUTHORITY  
**Date:** 2026-08-06  
**Scope:** Receiverless leftovers in `✏️s/🔌️plugins/**` + `cargo check -p … --lib` on priority crates.

## `cargo check -p … --lib` (DEVELOPER_DIR=/Library/Developer/CommandLineTools)

| Crate | Status | Blocker (if not green) |
|-------|--------|-------------------------|
| `semio-s-plugin-fem` | **green** | — |
| `semio-s-plugin-norm` | **green** | — |
| `semio-s-plugin-layout` | **red** | `semio-framework-os-infinite` (~186 errors: glue/`self::`/`crate::infinite` module wiring) |
| `semio-s-plugin-flow` | **red** | same `semio-framework-os-infinite` |
| `semio-s-plugin-draw` | **red** | `semio-framework-os` (`E0432`: `crate::space`, `crate::workflow`, `crate::workflow_kernel`) |
| `semio-s-plugin-procedural` | **red** | same `semio-framework-os` as draw |

Re-run:

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools \
  cargo check -p semio-s-plugin-{fem,norm,layout,draw,flow,procedural} --lib
```

## Plugin edits this pass

### Session globals (rule 3)

- **No** `DRAWPLAYAPP_SESSION` / `FLOWPLAYAPP_EVAL_SESSION` / `PROCEDURAL*PLAYAPP_EVAL_SESSION` / `LazyLock<Mutex<…Session>>` remain under play apps.
- **Draw / flow / procedural:** `handle` uses `DrawSession::default()` or `FlowEvalSession::default()` per dispatch; **layout** uses `LayoutEngine::new()` in `render`.
- Removed stale `use std::sync::Mutex` from layout + procedural 2d/3d play `component.rs`.
- Updated flow config doc: eval is per-call local session, not `FlowPlayApp::eval_session`.

### Receiverless `self` in associated fns (rule 2)

- **Puzzle 2d/3d/5d** `DocumentApp`: `render`, `window_engagements`, `window_measures`, `tool_measures` no longer use bare `self` inside associated fns; use `let app = Puzzle*PlayApp::default()` (same interim model as `handle` on 3d).
- **Puzzle 2d/3d/5d** `context_menu`: dropped erroneous `&self` parameter (trait is receiverless).

### Duplicated trait items (rule 1)

- No `fn app_id(&self)` / `fn document_schema(&self)` left in plugins.
- Norm apps still override **`fn config_schema() -> &'static str`** (associated fn — not a duplicate of `APP_ID` / `DOCUMENT_SCHEMA`).

### Emit / draft (rule 4)

- Play apps on the priority list already use `Emit<_, _, Self::DraftOperation>` at the `DocumentApp` boundary; command handlers return `Emit<Op, CfgOp>` and rely on `From` where applicable.

### Misc

- Layout test: `LayoutPlayApp::io()` instead of `LayoutPlayApp::default().io()`.

## Remaining top error classes (batch log + fresh checks)

From `🧪w2-documentapp-cargo-check.err` (aggregate) and 2026-08-06 re-check:

| Class | ~count (batch) | Notes |
|-------|----------------|--------|
| `E0433` unresolved module fns | 1009 | Mostly `semio-framework-os-infinite` glue (`infinite`, `os_store`, `os_dsl`, `self::canvas`, …) |
| `E0425` cannot find value | 429 | Includes puzzle `self` in associated fns (partially fixed this pass) |
| `E0432` unresolved import | 405 | `ui_wgpu::wgpu::*`, `framework_surface_terrain`, `semio-framework-os` workflow modules |
| `E0308` type mismatch | 198 | Often `command_id` `&str` vs `&'static str` in apps not yet aligned |
| `E0559` variant/field | 58 | Command enum / manifest drift |
| `E0603` private crate | 32 | `semio_format` via OS package glue |
| `E0277` trait bound | 32 | Config `ConfigRecord` / `OperationDiff` (fem was in this bucket; **now green**) |

## Structural follow-up (not plugin-only)

1. **`semio-framework-os-infinite`** — restore/include modules so `crate::infinite`, board canvas shims, and asset `include_bytes!` paths resolve (blocks layout, flow, puzzle, gis, dag, …).
2. **`semio-framework-os`** — wire `space` / `workflow` / `workflow_kernel` (blocks draw, procedural).
3. **Fieldful play apps** — `Puzzle3dPlayApp`, `CadPlayApp`, `LowpolyPlayApp`: move `RefCell`/`Mutex` caches into typed `Draft` + `draft_operations`; stop `Puzzle*PlayApp::default()` per `handle`/`render` (stateless but wrong for geometry/precompute caches).
4. **`command_id` → `&'static str`** — sweep apps where generated `command_id()` returns `&str`.

## Files touched (plugins)

- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎚️config/🦀️component.rs`
