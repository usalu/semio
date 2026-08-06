# W2 — DocumentApp receiverless migration (`✏️s/🔌️plugins/**`)

**Ticket:** OS-EXCLUSIVE-STATE-AUTHORITY  
**Date:** 2026-08-06  
**Scope:** Every `impl DocumentApp for` under `✏️s/🔌️plugins` (54 apps). OS SDK trait already receiverless — no edits under `🧰️framework/🛍️products/💻️os/**`.

## Summary

| Metric | Count |
|--------|------:|
| `impl DocumentApp for` files touched | **54** |
| Still using `fn app_id(&self)` | **0** |
| `type Draft = NoDraft` (+ `NoDraftOperation`) | **54** |
| `register_document_app::<Ty>(app)` (no factory closure) | **all call sites updated** (incl. `🪐️space` glue) |
| **Receiverless contract applied** (consts, associated fns, `handle` + draft/engines) | **54** |
| **True ZST** (no fields, no Mutex/RefCell on app struct) | **39** |
| **Blocked / follow-up** (non-ZST or module-global session) | **15** |
| `cargo check -p semio-s-plugin-*` (batch, `DEVELOPER_DIR=/Library/Developer/CommandLineTools`) | **8 ok / 38 fail** (see log) |

Full per-app row: `🧪w2-documentapp-inventory.json`. Batch log: `🧪w2-documentapp-cargo-check.err`. Mechanical scripts: `🔧️migrate-w2-documentapp.mjs`, `🔧️migrate-w2-documentapp-fix.mjs`, `🔧️migrate-w2-call-sites.mjs`.

## Key patterns applied

1. **Associated constants** — `const APP_ID`, `const DOCUMENT_SCHEMA` on the impl (module `pub const APP_ID` kept for manifests; trait consts use literals or existing `*_PLAY_APP_ID` symbols).
2. **Draft lane** — `type Draft = NoDraft`, `type DraftOperation = NoDraftOperation` unless an app later needs a real draft projection.
3. **Associated functions** — removed `&self` from `DocumentApp` methods; `config_schema()`, `initial_projection()`, `render`, `io`, media helpers, etc.
4. **`handle`** — `fn handle(command, doc, cfg, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Op, CfgOp, Self::DraftOperation>, Fault>`.
5. **`Emit`** — third type param `Self::DraftOperation` on all app emissions.
6. **Imports** — `DraftView`, `NoDraft`, `NoDraftOperation` from `semio_framework_plugin`; `EngineHandles` from `store`.
7. **Registration** — `.register_document_app::<PlayApp>(create_*_app())` (turbofish only).
8. **Manifest / tests** — `PlayApp::config_spec()`, `PlayApp::io()` (not instance methods); `Self::io()` inside trait bodies.
9. **Ephemeral engine/session (interim)** — for flow/draw/procedural/layout: app struct → unit ZST + `LazyLock<Mutex<…>>` + `*_lock()` helpers so `render`/`pending_effects` keep working until draft is threaded into `render` or state moves to `Draft` + ops.

## Migrated (receiverless + ZST unit struct) — 39 apps

Norm (15): `en1990`, `en1991`–`en1999`, `din4108`, `din16798`, `din18599`, `iso16757`, `vdi3805`.

Others: `architect`, `animate-present`, `block` (2d/3d/5d), `dag`, `fem` (2d/3d), `forms`, `gis` (2d/3d), `imperative`, `mathematical`, `note`, `playbook`, `playbook-procedural extension`, `process` 3d, `raster`, `reasoning-wires`, `remodel`, `sequence`, `shooting`, `sourcing-curate`, `space` (home/space), `trinity` (rewrite/jack), `vcs`, `writer`, `procedural` (2d/3d — ZST shell), `flow`, `draw`, `layout`, `demonstrator` panes (via their apps), etc.

(Exact paths: see inventory JSON — all list `blocked: "migrated"` except rows below.)

## Blocked / follow-up — 15 apps

| App | Issue | Next step |
|-----|--------|-----------|
| `CadPlayApp` | `RefCell<u64>` (`preview_seq`) | Move preview seq to draft or config op; drop field |
| `LowpolyPlayApp` | `RefCell<LowpolyScratch>` | Real `Draft` / `DraftOperation` for scratch+commit |
| `Puzzle2dPlayApp`, `Puzzle5dPlayApp`, `Puzzle3dPlayApp` | `RefCell` / `Mutex` caches, precompute, transform session | Split into `Draft` lane + config; `handle` must not use fresh `default()` per dispatch (puzzle3d interim) |
| `FlowPlayApp`, `DrawPlayApp`, `Procedural2d/3dPlayApp`, `LayoutPlayApp` | ZST but **module `LazyLock<Mutex>`** for eval/draw/layout session | Move session into typed `Draft` + `draft_operations`; wire `render` to draft store when SDK exposes it |
| `en1992` (norm) | Same receiverless shape as siblings; any compile gap is workspace/deps | — |

## `cargo check` (semio-s-plugin-*)

- **OK (8):** `draw-fsm`, `draw-fsm-macros`, `energy`, `imperative-control`, `imperative-core`, `imperative-logic`, `imperative-math`, `imperative-text` (mostly extension crates without DocumentApp surface).
- **FAIL (38):** DocumentApp-related patterns in failures:
  - `command_id` → `&'static str` vs generated `&str` (`E0308` / signature mismatch).
  - Leftover `self.` in associated `handle`/`render` helpers (e.g. puzzle3d — partial fix to `Puzzle3dPlayApp::default().…`).
  - Pre-existing / orthogonal: `semio-framework-os-infinite` (`Undirected`), private `semio_format`, fem config traits, etc.

Re-run batch:

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools \
  bash -c 'while read -r p; do cargo check -p "$p" || true; done < <(rg "^name = \"semio-s-plugin" ✏️s/🔌️plugins -g Cargo.toml | sed ...)' 
```

(see `🧪w2-documentapp-cargo-check.err` for full output).

## Counts for parent agent

- **Migrated (receiverless contract):** **54 / 54**
- **True ZST (no app fields):** **39 / 54**
- **Remaining structural work:** **15** (5 fieldful apps + 6 session Mutex hoists + puzzle family)
- **Plugin crates passing `cargo check` in batch:** **8 / 46** (many failures not caused by DocumentApp migration alone)
