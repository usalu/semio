# Audit: every test/fixture/story/doc pinning the old "+N" panel paging

Read-only sweep of the whole repo (excluding `node_modules`, `target`, `⚡️cache`, `🗑️generated`, `dist`,
`storybook-static`) for the framework paging primitives (`panel_continuation_row`, `paged_panel_section`,
`panel_page_rows`, `PanelRowBudget`, `UI_VALUE_PAGE_ROWS`, `PANEL_RECONCILE_NODE_BUDGET`, `SECTION_ROWS`,
`IDS_ROWS`, `LIST_ROWS_MAX`, `CATALOGUE_GROUP_ROWS`, `HISTORY_COMMAND_ROWS`, `continuation_row`,
`continuation_row_from`, `paged_section_from`, `section_page`, `setPanelPage`, `panel_pages`,
`.more`-suffixed keys, `+{N}` label formatting, `more-horizontal` icon, `PANEL_PAGE_GUARD`, `entityRows`)
plus Storybook Tree/Panel stories and framework recipe docs. No files were modified.

## 0. Primitives being retired (definitions, for reference)

| Path | Line | Symbol |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | 5888 | `pub fn panel_page_rows() -> usize` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | 5893 | `pub struct PanelRowBudget(usize)` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | 5933 | `pub fn panel_continuation_row(section_id, omitted) -> UiAssemblyResult<BuiltNode>` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | 5952 | `pub fn paged_panel_section<T>(...)` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | 10117 | `pub const HISTORY_COMMAND_ROWS: usize = 16` (history panel pager) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | 39485 | `pub use app::{paged_panel_section, panel_continuation_row, panel_page_rows};` (public re-export) |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs` | 54 | `pub const UI_VALUE_PAGE_ROWS: usize = crate::UI_BUILT_CHILDREN_MAX - 1;` |

None of these definitions are behind a Cargo feature gate — they compile under default features in both
crates. `paged_panel_section`'s continuation row (`panel_continuation_row`) is a `Component::TreeItem` with
key `"{section_id}.more"` and label `"+{omitted}"`, dispatching `setPanelPage` where the caller supplies a
next-page action.

## 1. Rust unit tests (grouped by crate; all are `#[cfg(test)]` modules compiled INTO the lib crate — no
separate `tests/` integration binaries pin this pattern, see §2)

### Crate `semio-s-artifact-energy-model` (energy plugin, no extra feature needed — see §5)
File: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`

| Line | Symbol | What it pins |
|---|---|---|
| 208 | `async fn an_oversized_document_pages_with_continuation_rows()` | `assert!(json.contains(&format!("{TREE_NAMESPACE}.materials.more")))` — pins the `.more` continuation-row key |
| 60, 106, 118, 143, 146 | (setup) `PanelRowBudget::new(rows/32/DEMO_ZONES_DEMAND/4)` | seeds a fixed row budget per test — any budget/row-count rework invalidates these fixture sizes |

### Crate `semio-s-artifact-fem-2d` (needs `--features component-app-assembly`, see §5)
File: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`

| Line | Symbol | What it pins |
|---|---|---|
| 47 | `fn omitted(parent, suffix) -> usize` | shared test helper: looks up child keyed `"{TREE_NAMESPACE}.{suffix}.more"` and parses its label with `trim_start_matches('+')` — pins BOTH the `.more` key and the `+N` label format. Used by 3 call sites (lines 285, 286, 288 — inside the two tests below). |
| 55 | `fn placed(parent, suffix) -> usize` | counts children whose key ≠ the `.more` continuation key |
| 257 | `async fn section_quotas_are_max_min_fair()` | asserts the max-min-fair row-quota split across 9 sections (`section_quotas([...], 31)`) — pins the shared-budget allocation law itself, not just the row format |
| 274 | `async fn an_oversized_document_pages_with_continuation_rows_instead_of_faulting()` | drives a 60-node/40-load document past one page and asserts every truncated section closes with a `.more`/`+N` row via `omitted()`/`placed()` |

### Crate `semio-s-artifact-fem-3d` (needs `--features component-app-assembly`, also pulls fem-2d's flag transitively, see §5)
File: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`

| Line | Symbol | What it pins |
|---|---|---|
| 47 | `fn omitted(...)` | identical helper to fem-2d's, same `.more`/`+N` pin |
| 55 | `fn placed(...)` | same |
| 239 | `async fn section_quotas_are_max_min_fair()` | same law as fem-2d |
| 253 | `async fn an_oversized_document_pages_with_continuation_rows_instead_of_faulting()` | same shape as fem-2d, 3d nodes |

### Crate `semio-s-artifact-cad-cad` (no extra feature needed — see §5)
File: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`

| Line | Symbol | What it pins |
|---|---|---|
| 29 | `async fn document_lists_every_pane_object_bound_to_the_cad_domain()` | per-pane loop asserts `placed + omitted == objects.len()` where `omitted` is parsed from a `"{section_key}.more"` row's `"+N"` label (inline logic, lines ~40–46) |
| 128 | `async fn set_panel_page_advances_the_structure_section()` | drives `setPanelPage`, asserts `objects.len() > CAD_SECTION_ROWS` (line 133), `json.contains("setPanelPage")` (141), and `second_rows.first() == objects[CAD_SECTION_ROWS].id` (148) after seeding `CadPlayRuntime.panel_pages` |
| 154 | `async fn a_stale_panel_page_cursor_clamps_to_the_last_page()` | `section_page(&pages, "section", 0) == 0` (156), `section_page(&pages, "section", CAD_SECTION_ROWS+1) == 1` (157), `section_page(&BTreeMap::new(), "section", 100) == 0` (158) — pins `CAD_SECTION_ROWS` clamp arithmetic directly |

File: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (editor-level command tests, separate file from the panel's own tests above)

| Line | Symbol | What it pins |
|---|---|---|
| 1797 | `async fn set_panel_page_records_the_section_cursor_on_the_config_lane()` (region `📄️PanelPaging`) | drives `setPanelPage` action, asserts it never touches the document (`artifact_mutations.is_empty()`), lands on `runtime.panel_pages`, and rejects an empty-section request |

### Crate `semio-s-artifact-puzzle-2d` (needs `--features component-app-assembly`, see §5)
No unit-test file under `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/*/🧪️tests/🔬️unit/🦀️.rs` matched the paging symbols. The `📌️panels/🗿️artifact/🦀️.rs` and `📌️panels/🔍️inspection/🦀️.rs` production files define `section_page`/`continuation_row`/`SECTION_ROWS`/`IDS_ROWS` (see §6) but currently have **no dedicated unit test** exercising them — implementers should add coverage rather than remove any.
File: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs`

| Line | Symbol | What it pins |
|---|---|---|
| 176–197 | `panel_pages: BTreeMap<String,u32>` seeded with `("objects",3)`/`("references",1)` | window-transient round-trip test carries the per-section page cursor map through retire/restore — needs updating if `panel_pages` is removed from window state |

### Crate `semio-s-artifact-puzzle-3d` (needs `--features component-app-assembly`, see §5)

File: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — the main artifact-tree paging law suite

| Line | Symbol | What it pins |
|---|---|---|
| 31–34 | `fn page_guard()` using `super::PANEL_PAGE_GUARD` | serializes every paging test in this binary — `panel_page_rows()` reads a **process-wide** arena, so concurrent tests would race; removing the guard needs a replacement synchronization story if any global-arena law survives virtualisation |
| 126 | (law) `assert!(interactive <= semio_framework_ui_contract::UI_VALUE_PAGE_ROWS, ...)` | pins the global page ceiling |
| 133 | `assert!(object_rows.len() <= super::SECTION_ROWS, ...)` | pins per-section row cap |
| 203 | `assert!(more.2 > 0, "the Nakagin continuation row must advance setPanelPage: {more:?}")` | pins that the continuation row advances `setPanelPage` |
| 340–354 | (Wave U law) builds `rows_per = super::SECTION_ROWS`, finds row `key.ends_with(".more") && key.contains("objects") && bindings > 0` | pins `.more` suffix + non-zero binding count (a clickable `setPanelPage` control) |
| 392–393 | `assert!(nodes <= super::PANEL_RECONCILE_NODE_BUDGET, ...)` | pins the 16-node host reconcile envelope |

File: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs`

| Line | Symbol | What it pins |
|---|---|---|
| 8, 34, 64, 102, 169 | `super::super::artifact::PANEL_PAGE_GUARD.lock()` | same global-arena serialization guard, 5 call sites |
| 33 | `fn the_catalogue_pages_an_over_wide_kind_catalog_without_exceeding_the_fixed_page()` | asserts `objects.children.len() <= UI_BUILT_CHILDREN_MAX` and `objects.children.iter().any(|row| row.key.as_str().ends_with(".more"))` (lines 46–47) |

File: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`

| Line | Symbol | What it pins |
|---|---|---|
| 159 | `fn inspection_ids_page_is_bounded_and_the_continuation_advances()` | seeds `IDS_ROWS*2+3` ids (160), asserts `id_rows <= IDS_ROWS` (167), and computes `last = (ids.len()-1)/IDS_ROWS` (178) — pins the `IDS_ROWS`/pager-cursor arithmetic directly |
| 50 | (setup) `runtime.panel_pages.insert(IDS_SECTION.to_string(), page)` | seeds the section's page cursor |

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs` — no direct hit found here for puzzle-3d (only puzzle-2d's window test above carries `panel_pages`); puzzle-3d's window struct also has a `panel_pages` field (production, `🪟️window/🦀️.rs:40`) but no dedicated round-trip test currently exercises it.

## 2. Rust integration tests

**None found.** Every `🧪️tests` directory that matched is a source file pulled into the owning crate's
lib target as an in-crate `#[cfg(test)] mod` (confirmed for `semio-framework-plugin` via its
`#[cfg(test)] include!("🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs")` at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:38470`, inside `pub mod plugin_runtime { ... }`).
None of the artifact crates' `Cargo.toml` declare a `[[test]]` target pointing at a paging-related file
(the only `[[test]]` entries that exist at all are `semio-s-artifact-procedural-generation3d`'s
`example-geometry`/`io-round-trip`/`incremental-eval`, none of which reference paging, and
`semio-framework-ui-contract`'s `typegen_export`/`catalogue_carrier_map`, also unrelated). One
framework-level "integration-shaped" unit test does exist inside `semio-framework-plugin`:

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
(included into the lib crate, compiled by plain `cargo test -p semio-framework-plugin`)

| Line | Symbol | What it pins |
|---|---|---|
| 41 | `use semio_framework_ui_contract::{..., UI_BUILT_CHILDREN_MAX, UI_VALUE_PAGE_ROWS};` | imports the page-ceiling const |
| 4731, 4778 | `assert!(actions <= UI_VALUE_PAGE_ROWS, "inline reverts must stay inside the arena page the contract declares: {actions} > {UI_VALUE_PAGE_ROWS}")` | pins the global interactive-row ceiling for inline history reverts |
| 4782 | `async fn ui_history_panel_pages_command_rows_from_the_live_count()` | pins that the framework's own history panel pages its command rows |

## 3. TS tests

| Path | Line | Symbol | What it pins |
|---|---|---|---|
| `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts` | 80–99 (`class RetainedAuditScript`, router command `retained-audit`) | reads `🧬️schema/🔣️.json` + `🧫️fixtures/🗄️retained-jobs/🔣️.json`, extracts command ids straight out of `editor/🦀️.rs`'s `app_commands!` macro block via regex, and diffs against a hard-coded `expectedAdmitted` array that **includes `"setPanelPage"`** (line 97) | If `setPanelPage` is removed/renamed, this audit script's `expectedAdmitted` list must be updated or the audit fails. Invoked directly (not wired into the package's `test`/`generate`/`fixture` npm scripts) — run with `bun run 📜️script.ts retained-audit` from `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/`. |

No other TS test files (only production/config references — see §7) directly assert on the pager output
format. Storybook stories render live artifact panels rather than hard-coding rows, so no story-level
snapshot pins the `+N`/`.more` shape (see §6).

## 4. JSON fixtures

| Path | Line | What it pins |
|---|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` | 78, 265 | lists `"setPanelPage"` as a published/admitted tool id for `Puzzle2dPlayApp`/`Puzzle3dPlayApp` owners (schema `semio.puzzle.publication-authority.v1`) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` | 6 | `toolIds` array includes `"setPanelPage"` among the retained-job replay fixture's admitted tools |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` | 52 | same, puzzle-3d's retained-jobs fixture |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` | 43, 360 | `"setPanelPage"` listed among retained tool ids and as a full `"id": "setPanelPage"` route entry — consumed and validated by the TS `RetainedAuditScript` above |
| `✏️s/🔌️plugins/📐️cad/🔣️.json` | 954, 4201, 7448, 10695 | a generated plugin-manifest/descriptor snapshot (`descriptorVersion`, `manifest.apps[].actions[]`) that mirrors the action declared in `editor/🦀️.rs`; 4 near-duplicate `"id": "setPanelPage"` action entries (one per app/pane). Appears to be a checked-in generated artifact, not hand-authored — regenerate rather than hand-edit if `setPanelPage` is retired. |

## 5. Stories

**None found.** Searched every `📖️stories` directory in the repo (framework `🖱️ui` element stories —
including `🧱️elements/🌳️Tree/📖️stories` and `🧱️elements/🖼️Panel/📖️stories` — and every plugin's
`📖️stories`, e.g. `✏️s/🔌️plugins/📸️remodel/📖️stories/🎭️panels`) for `continuation`, `more-horizontal`,
`omitted`, `truncat`, `panel_page`, `setPanelPage`, `paged`, `pagination` — zero matches. Panels are
rendered live from the wasm artifact's own assembly, so no story hard-codes a `+N`/`.more` row.

## 6. Docs (recipes under `🧰️framework` / `✏️s`, excluding ticket notes)

**None found.** No `📓️recipe-plugin.md`-style doc, or any other markdown under `🧰️framework` or `✏️s`,
documents the paging law in prose — the only "documentation" of the convention is the Rust doc-comments
on the primitives themselves (§0) and on each call site (§7). The paging law is therefore ONLY encoded in
code and in the ticket-scoped design note already in this ticket
(`📓️design-virtualised-tree.md`) plus the predecessor ticket
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/PUZZLE-3D-ARTIFACT-TREE-PAGING/📓️ellipsis-continuation-row.md`.

## 7. Ticket-scoped browser probes (paths only — out of scope to detail, listed per the task's carve-out)

Probe/check scripts under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/` that assert on `+N`/`.more`/`entityRows`
paging markers, so the coordinator knows verification tooling already exists for some of these panels:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR/🐍️energy-panels-probe.mjs` — asserts rows matching `/\.more$/i` and text `/^(more|mehr|weitere)\b/i` after picking a surface (energy zones/windows pager)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️b46-selection-nakagin.ts` — logs `entityRows` (outliner rows minus continuation rows) and clicks the first one
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️b44-mutation-latency.ts` — logs `outliner entityRows=${rows.length}`

(Several other files matched a loose `continuation` grep — e.g. `🔍️b22-mesh-probe.ts`,
`🐍️host-ingress-latency-probe.mjs` — but on inspection those pin an unrelated "command ingress
continuation" async/dispatch concept, not the panel row pager; excluded as false positives.)

## 8. Cargo commands per crate (feature flags checked against each crate's own `Cargo.toml`)

| Crate | Needs `component-app-assembly`? | Command |
|---|---|---|
| `semio-s-artifact-energy-model` | No (`ui-contract` is a hard, non-optional dependency) | `cargo test -p semio-s-artifact-energy-model` |
| `semio-s-artifact-cad-cad` | No (no `[features]` section at all — everything required) | `cargo test -p semio-s-artifact-cad-cad` |
| `semio-s-artifact-fem-2d` | Yes (`ui-contract`/`ui-scene` are optional, gated by this feature) | `cargo test -p semio-s-artifact-fem-2d --features component-app-assembly` |
| `semio-s-artifact-fem-3d` | Yes (also flips on `semio-s-artifact-fem-2d/component-app-assembly`) | `cargo test -p semio-s-artifact-fem-3d --features component-app-assembly` |
| `semio-s-artifact-puzzle-2d` | Yes | `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly` |
| `semio-s-artifact-puzzle-3d` | Yes | `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly` |
| `semio-s-artifact-procedural-generation3d` | Yes (for its `[[test]]` lanes; the catalogue pager itself has no dedicated test yet) | `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly` |
| `semio-framework-plugin` | No (primitives + the plugin-builder-contract test compile under default features) | `cargo test -p semio-framework-plugin` |
| `semio-framework-ui-contract` | No (`UI_VALUE_PAGE_ROWS` is unconditional; only its own `typegen` feature exists, unrelated) | `cargo test -p semio-framework-ui-contract` |

## 9. TS test runner command per package

| Package | Command |
|---|---|
| `@semio-tech/cad-js` (`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript`) | `bun nx run @semio-tech/cad-js:test` (the `setPanelPage`-pinning `retained-audit` command is separate — see §3) |
| `@semio-tech/framework-os` (`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript`, owns `ShellHelpers.tsx`) | `bun nx run @semio-tech/framework-os:test` |

## 10. Other production touchpoints worth knowing about (not tests/fixtures/stories/docs, but implementers will hit these while removing the pattern)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:4654` — `WINDOW_CONFIG_RAIL_ACTION_IDS` includes `"setPanelPage"` (production dispatch-scope set, not a test).
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️panel/🦀️.rs` and `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄set-panel-page/🦀️.rs` — the `setPanelPage` command handlers themselves.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧬️schema/🛰️.proto:17` — `map<string, uint32> panel_pages = 5;` wire schema field that will need a migration/removal plan.
- Config lanes carrying the cursor: `CadConfig::panel_pages_json` (`✏️s/🔌️plugins/📐️cad/.../✏️editor/🎚️config/🦀️.rs`) and puzzle's `panel_pages: BTreeMap<String,u32>` on both `WindowConfig`/`WindowTransient` (`✏️s/🔌️plugins/🧩️puzzle/.../✏️editor/🎚️config/🦀️.rs`, `.../✏️editor/🪟️window/🦀️.rs`).

## Excluded false positives (confirmed irrelevant, saved to avoid re-checking)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🦀️.rs` and the ticket copy
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS/{🧩️dwg-io-head-source.rs,🧪️dwg-r2-*.c,🧪️dwg-r2-dwg-header.txt}` — `section_page(s)`/`dwg_section_page_checksum` are unrelated DWG binary-format concepts (on-disk R2004/R2007 section pages), not the panel row pager.
- `♻️mit-bestand/🔎️recherche/_archive/migration/write_current_structure_report.ps1` — `$entityRows` is an unrelated PowerShell report-generator variable.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts` — `homeDirectoryIdentityRowsOracle`/`HomeDirectoryIdentityRowsCheckScript` is an unrelated home-directory identity check ("Identity**Rows**", not panel paging).
