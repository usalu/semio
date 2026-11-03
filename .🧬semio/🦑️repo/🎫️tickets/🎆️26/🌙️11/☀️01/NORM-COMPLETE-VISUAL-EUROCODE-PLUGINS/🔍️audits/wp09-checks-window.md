# WP-09 Checks window and viewer wiring (A9)

SURFACE = `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface`.

## Delivered
- `SURFACE/🪟️visual-windows/✅️checks/🦀️.rs`: `ChecksViewState` (grouping entity/clause/part, sort worst-first/report/clause/title, status + text filter, selected check, pending progress) with `encode`/`decode` wire form; pure `plan_checks`; `check_selection` (entity id + field path); `render_checks` (drop-in, same four arguments as the retired `render_report`) and `render_checks_with` (view state + `ChecksWindowSpec`); `check_detail_rows` (shared by window rows and inspection).
- `📶️utilisation-bar`: `UtilisationBand` (tone + fill pattern `█ ▓ ▒ ·` + glyph `✓ ⚠ ✗ ∅`), 12-cell bar on a 0-120 % scale with `┃` tick at 100 %, locale percent, determinate `ui::progress` node (worst utilisation above the list, per-check in inspection).
- `🩹️remedy-chips`: one chip row per remedy or choice option (`applyRemedy` / `setField`), change text `12 mm → ≥ 22 mm`.
- `🔗️clause-link`: localized clause text (`§6.2.9` / `Abschn. 6.2.9`), natural clause ordering, optional `openClause` row.
- Rows: windowed (existing `TreeWindows` ledger), accessible name sentence per row, tone, `selected`, `dimmed` while pending; entity group rows use the raw entity id with `granularity("entity")` when an interaction domain is set (selection sync); activation fires existing `setSelectedCheckIndex`.
- Quantities: `quantity_text` (units from `format_quantity`, German decimal comma).
- `render_report` and its helpers deleted from `SURFACE/🌳️report-tree/🦀️.rs` (no shim); `report_table_columns/rows` kept as the table twin. `NORM_REPORT_*` ids renamed `NORM_CHECKS_*`. `SURFACE/📚️catalogue-and-inspection/🦀️.rs::render_inspection` now uses `check_detail_rows`.
- All 15 `📊️results` windows switched with one-line `render_report` to `render_checks` change; existing report tests renamed to the new ids.
- Tests: `✅️checks/🧪️tests/{🦀️.rs,🥒️.feature}` (bar patterns, locale, view-state round trip, natural clause order, plan vs `std` sort, filters, selection, DE/EN rendering parsed with `serde_json`, action binding only for editors, optional actions, inspection sharing, table twin).
- Mount applied: see `📌️important/mounts.md`.

## Not done / hand-offs (see `📌️important/requests.md`)
- NDP provenance / unverified-basis status: kernel field not present yet, not invented.
- `setChecksView`, `cancelEvaluation`, `openClause` commands and window-config persistence: window renders them only when `ChecksActions` flags are set.
- Ghost remedy preview (`Trigger::HoverPreview`, `previewRemedy`) belongs to WP-10; chips are ready to take it.
- Colour of the bar fill is carried by `Tone`; the patterned text bar is the redundancy.
- Viewer shims: the viewer `📊️report` windows (if any call `render_checks`) need no change; viewport/diagrams tabs are WP-07/08/12.

## Verification
NOT COMPILED: A9 stood down from building; the integrator compiles via `@semio-tech/norm-artifact-contract-rs` (see requests.md). Likeliest fixes: `interaction_domain(controller, String)` arg type, a `Progress` node as sibling of the tree in a `column`, unused `tree_group` import warning.
