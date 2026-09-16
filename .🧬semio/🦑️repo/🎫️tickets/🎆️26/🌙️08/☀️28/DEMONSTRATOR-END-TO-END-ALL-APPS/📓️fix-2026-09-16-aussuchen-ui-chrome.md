# Aussuchen (`s.sourcing.curation@1/*#editor`) — UI chrome audit verification + fix

Date: 2026-09-16. Repo root `/Users/ueli/Documents/semio`.

## 0. Path drift — the 08-28 audit no longer addresses real files

`📓️app-aussuchen.md` (08-28) cites `🗿️artifacts/🗂️curate/…/✏️editor/🦀️component.rs`. Neither the
artifact folder nor the file name exists any more:

| audit path | current path |
|---|---|
| `🗿️artifacts/🗂️curate/` | `🗿️artifacts/🗂️curation/` |
| `…/✏️editor/🦀️component.rs` | `…/✏️editor/🦀️.rs` |
| `🎮️commands/📄️set-active-example/` | `🎮️commands/🎬️set-active-example/` |
| `🎮️commands/🧺️curate-add/` | `🎮️commands/➕️curation-add/` |
| `🎮️commands/🔍️sort-table/` | `🎮️commands/↕️sort-table/` |
| `CurateSnapshot` / `SourcingCurateConfig` / `curateAdd` | `CurationSnapshot` / `SourcingCurationConfig` / `curationAdd` |

Crate: `semio-s-artifact-sourcing-curation`
(`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml:2`).

## 1. Findings — the audit's §4/§7 chrome claims are STALE (verified against current code)

Audit claim vs. reality, with file:line as read on 2026-09-16 **before** any edit in this pass:

| audit claim (08-28) | current reality | evidence |
|---|---|---|
| "`sourcing_action()` … defined but never called anywhere in the plugin" | **FALSE.** Called 4× in the pool filter bar. | `…/🪟️windows/🏊️pool/🦀️.rs:89,95,104,112` |
| "`SOURCING_DRAG_MIME` … declared and never referenced again" | **FALSE.** Set on every sourcing table scene. | `…/✏️editor/🦀️.rs:100` (`scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into())`) |
| "No search box … anywhere in the pool/curated/grid render code" | **FALSE.** `filter_bar()` builds a text input, one toggle per module, a typology `select` and a min-availability number input, each bound to its `sourcing_action` command. | `…/🏊️pool/🦀️.rs:84-116`, mounted at `:129` |
| "no sortable column headers" | **FALSE.** Pool declares `sortable: true` on name/module/availability; the renderer turns a sortable header click into `action: "sortTable"`. | `…/🏊️pool/🦀️.rs:119-125`; `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx:202-209` |
| "no drag source/drop target" | **FALSE.** Every row carries `_drag: {objectId}`; `drop_action_json` is `dropOnPool`/`dropOnCurated`; the React host wires `dataTransfer.setData(rowDragMime, …)` and a matching drop handler. | `…/✏️editor/🦀️.rs:83-106`; `📊️Table/🟦️.tsx:167-188,214-218` |
| "`TableView` cells are plain strings with zero row actions" | **FALSE.** The windows use `TableScene` + typed `TableCell` (`Text`/`Number`/`Stepper`/`Buttons`); Curated already had a per-row `trash-2` remove button. | `…/🧺️curated/🦀️.rs:45-53` |
| "Preview permanently stuck on 'No selection'" | **FIXED by a peer.** `render_with_request_context` now reads `interaction.selection(SOURCING_ROWS_DOMAIN)` and passes the ids to `preview::render`. | `…/✏️editor/🦀️.rs:1042-1055` (left untouched by this pass) |
| §2 `demo-stock` is real | **STILL TRUE** (fixture now at `…/🧫️fixtures/📦️expected-stock.json`). | `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:109` |

**Net:** the audit's headline ("fully implemented commands with ZERO UI chrome") is wrong for the
current tree. Almost all of the chrome landed between 08-28 and today.

### 1b. Real gaps that *were* still open

1. **Pool had no curate/uncurate row action.** Curating from the Pool was only possible by nudging
   the `curated` stepper or by drag-and-drop — no one-click button. `curationAdd`/`curationRemove`
   were dispatched from Curated only.
2. **Curated had no sortable headers and ignored the sort state** — `sourcing_table(..., None)` at
   `…/🧺️curated/🦀️.rs:57`, and all four columns `sortable: false` at `:39`. `curated::render` did not
   even receive `&SourcingCurationConfig`.
3. **Pool's sort fell through to name for any unknown column** (`_ => a.name.cmp(&b.name)`,
   `…/🏊️pool/🦀️.rs:47`), so sorting Curated by its own `count` column would silently re-sort the Pool
   by name.
4. **Contributed `sourcing.module` topics do NOT reach the Pool rows on their own.** `filtered_stock`
   reads `document.stock_extra` (`…/🧬️schema/🦀️.rs:295-308`); `setContributions` only writes
   `config.contributions_json` (`…/🎮️commands/🧩️set-contributions/🦀️.rs:15`), which feeds
   `available_modules()` — i.e. the filter toggles and the typology options. Contributed kinds become
   Pool **rows** only after `stockFromCatalogue` merges them into the document
   (`…/🎮️commands/📇️stock-from-catalogue/🦀️.rs:20-31`). That command was palette-only — no chrome.
   **This is by design, not a bug** (stock is VCS'd document content, contributions are session
   config), but it needed a one-click affordance and a regression test.

## 2. Edits

See §3 below.

## 3. Commands & results

See §4 below.
