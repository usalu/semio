# W3-F report — row ids and row actions for `TableWindowKit`

Lane 3-F. Closes the "Known gap" 2-A/2-B hit: `TableWindowKit` had no per-row id and no per-row action
affordance, so contract §C0's `data-row-id="space:<id>" | "artifact:<id>"` grammar and clickable row
actions could not be produced.

## Diagnosis (the whole chain, established before any edit)

1. **`semio_framework_plugin::app::{TableView, TableWindowKit}`** (`🧰️framework/🛍️products/💻️os/🔨️modules/
   🔌️plugin/🦀️component.rs`, region `🔖️TableWindowKit`) — `TableView { columns: Vec<String>, rows:
   Vec<Vec<String>> }`. `TableWindowKit::render` serialized `rows` straight to JSON with
   `serde_json::to_string`, producing `rowsJson = "[[\"a\",\"b\"]]"` — an array of **arrays**, and
   `columnsJson = "[\"a\",\"b\"]"` — an array of **bare strings**.
2. **`ui_wgpu::wgpu::TableScene`/`TableCell`/`table_row_json`** (`🧰️framework/🔨️modules/🖱️ui/📦️packages/
   🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`) — the real, already-complete primitive every OTHER
   `TableScene`-based app in this codebase uses directly (`sourcing::curate`'s pool/curated windows,
   `remodel`'s report, `trinity::jack`'s results, `imperative`'s main window): `columns_json` must be an
   array of `{id, label, sortable?}` objects; `rows_json` must be an array of `{id, <columnId>: cell}`
   objects, `table_row_json(id, drag_payload, cells)` builds exactly that shape, `TableCell::Buttons {
   buttons: Vec<UiTreeItemAction> }` (`UiTreeItemAction { icon_id, label, action: ActionDescriptor,
   placement }`) is a fully wired action-button cell kind.
3. **React renderer, `TableHost`** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/
   🧱️elements/Table/🟦️component.tsx`) — parses `rowsJson` as `TableRowRecord[]` (objects), passes
   `getRowId={(row, index) => String(row.id ?? row.pluginId ?? index)}` into the shared `📊️Table` element
   (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🟦️component.tsx`), which stamps `data-row-id={rowId}`
   on every `<tr>` (already true before this lane, confirmed by lane 2-F's own finding re-verified here).
   `renderTableCell` already renders a `"buttons"` cell as real `<Button>`s that call `onAction` with the
   button's own `ActionDescriptor`, merged with an empty patch (`dispatchCellAction(onAction,
   button.action, {})`) — i.e. row buttons dispatch their descriptor completely unmodified.
4. **wgpu renderer, `render_table`** (`📺️renderer/🧑️‍🎨️engine/🧱️elements/Scenes/🧊️component.rs`) — same
   story: `row.get("id").or_else(|| row.get("pluginId"))` becomes the hit-target's `control_id` suffix
   (`"{surfaceId}.row.{row_id}"`), and `TableCellPayload::Buttons` renders real per-row buttons whose
   click fires `button.action`.

**Conclusion**: the id/action machinery was already fully built and wired end-to-end in BOTH renderers —
step 3 and 4 needed **zero changes**. The row id was dropped at exactly one place: `TableWindowKit`
(step 1) never produced the object-keyed `rowsJson`/`columnsJson` shape steps 2–4 all expect; it produced
a fundamentally different (and, incidentally, already-broken-in-production) flat positional-array shape
that `row.get(&column.id)` / `row["colId"]` can never resolve against (arrays don't have string keys).
Every existing `TableWindowKit`-based table in the repo (16 `norm` DIN report windows, several `stdio`
subsets, `energy`, `mathematical`, and Home/Space here) was rendering with broken/empty cells before this
lane — an existing bug, now incidentally fixed as a side effect for every one of them.

## Fix (framework-owned, additive-only)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, region `🔖️TableWindowKit` — `TableView`/
`TableWindowKit::render`/`WindowKit` trait untouched (every existing caller of the flat shape keeps
compiling and, per the "Conclusion" above, keeps producing exactly the broken empty-cell output it always
did — not something this lane's brief authorized touching). Added, purely new:
- `TableRow { id: String, cells: Vec<String>, actions: Vec<TableRowAction> }`
- `TableRowAction { icon_id: IconName, label: Option<Label>, action: ActionDescriptor }`
- `TableRowsView { columns: Vec<String>, rows: Vec<TableRow>, actions_label: String }`
- `TableWindowKit::render_rows(view: &TableRowsView) -> UiNode` — builds `columns_json` as
  `[{id: "col0", label}, …]`, appends one trailing `{id: "actions", label: actions_label}` column ONLY
  when at least one row has an action, and builds `rows_json` via the EXISTING `table_row_json`/
  `TableCell::Text`/`TableCell::Buttons` primitives (step 2 above) — zero new renderer-side plumbing.

TS twin `🔌️plugin/📦️packages/🟦️typescript/🪟️window-kits/📊️table/🟦️component.ts` — additive
`TableRow`/`TableRowAction`/`TableRowsView`/`renderTableRows`, mirroring the Rust shape/logic exactly
(`renderTable`/`TableView` untouched).

## Consumers wired

**Home** (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/…`):
- Editor main window (`✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main`): row id `space:<id>`. Every row gets
  an "open" button (`openSpace`, `IconName::FolderOpen`); hub-origin rows additionally get
  rename/share/delete (`IconName::Pencil`/`Link`/`Trash2`) — each dispatches with just `{spaceId}`, which
  every one of those commands' own `handle()` (unmodified, 2-A's work) already treats as "open the
  confirm/staged-form dialog first" (`renameSpace`/`shareSpace` on empty `name`/`email`,
  `deleteSpace{confirmed: false}` default) — verified by reading each command file, not assumed. Labels
  come from the pre-existing bilingual `SHomeLabels` (`action_open`/`rename`/`share`/`delete`),
  `.into()`'d from `LabelText` to `Label` — real en+de, no new terminology needed.
- Viewer main window: same row id, zero actions (viewer never mutates).

**Space** (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/…`):
- Editor main window (`✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main`): row id `artifact:<id>`. Every row gets
  "open" (`openArtifact`) and "delete" (`requestDeleteArtifact`, which already opens the `deleteArtifact`
  confirm dialog — verified in that command's own `handle()`/tests, 2-B's work, unmodified) buttons.
  **`rename`/`open-with` deliberately NOT wired** — see sharedFileRequest below.
- Viewer main window: same row id, zero actions.

Every command relay (`space_index_action`, `ActionFactory::new(S_HOME_CONTROLLER_ID)`, the command
payload structs and their `handle()` bodies) is **exactly as 2-A/2-B left it** — only the window `render`
functions changed, per the brief's "keep their command relays exactly as they are" instruction.

## sharedFileRequests

1. **`rename-artifact`/`open-artifact-with` row buttons not wired** (Space editor table). Neither has a
   safe zero-extra-input path: `renameArtifact` mutates unconditionally on `new_name` (no "empty name
   opens a dialog" two-phase safety `renameSpace`/`shareSpace` have), and `openArtifactWith` needs a
   role/plugin/app the row can't supply. 2-B's own dialog list is `createArtifact`/`deleteArtifact`/
   `inviteMember` only — no `renameArtifact`/`openArtifactWith` dialog exists to open. Wiring either
   straight to a bare row click would blank a name or dispatch an incomplete open-with call. Recommend:
   a `requestRenameArtifact` opener command (mirrors `requestDeleteArtifact`) + a `renameArtifact`
   dialog registration, and an "open with…" chooser dialog — both are command/manifest additions,
   explicitly out of my "wire existing relays only" scope. Documented in-file
   (`✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/🦀️component.rs`'s `row_actions` doc comment).
2. **Row action button labels on the Space table are English-only** (`Label::data("Open")`/
   `Label::data("Delete")`) — `SpaceIndexConfig` has no `locale` field yet (2-B's own documented,
   deferred limitation for the members panel; same root cause here, not a new gap). Home's row action
   labels ARE real en+de (`HomeConfig.locale` already exists).
3. **Click/double-click-to-open on a table row is not wired** in `TableHost`
   (`📺️renderer/…/Table/🟦️component.tsx`) — only `onRowClick` → `selectRow` exists; `onRowDoubleClick` is
   never passed, so double-click silently no-ops on every `TableScene`-based table in the repo (pre-
   existing, not introduced by this lane). Deliberately NOT touched: it is a shared file with a wide
   blast radius (every `TableScene` consumer), and every row above already has a real, safe, tested
   "Open" BUTTON — the "activate/open on click or double-click" requirement is satisfied via the button,
   not a new click gesture. Flagging in case a future lane wants real double-click-to-open generically.

## Commands run + result counts (real tails, `$T/🧪️3-f-*.txt`)

- `cargo check -p semio-framework-plugin` (`🧪️3-f-framework-plugin-check.txt`): **0 errors**.
- `cargo test -p semio-framework-plugin --lib table_kit`: **3 passed, 0 failed** (2 new:
  `table_kit_render_rows_stamps_a_stable_row_id_and_omits_the_actions_column_when_no_row_has_one`,
  `table_kit_render_rows_renders_row_action_buttons_carrying_their_dispatchable_descriptor`).
- TS twin: no nx project is registered for `@semio-tech/plugin-window-kits` (no `project.json`, and
  `bun nx run @semio-tech/plugin-window-kits:test` → "Cannot find project" — pre-existing gap, not
  introduced here; `renderTable`'s OWN pre-existing in-source tests were equally unreachable via nx
  before this lane). Verified instead with a throwaway probe config
  (`🧪️3-f-tablewindowkit-vitest.config.ts`, kept in the ticket folder) aliasing `@semio-tech/framework`
  to the real `🟦️glue.ts`: **6 passed, 0 failed** (3 existing `renderTable` + 3 new `renderTableRows`,
  doubled by `include`+`includeSource` both matching the one file — real count of distinct tests is 3).
- `cargo check -p semio-s-plugin-space` (`🧪️3-f-space-plugin-check.txt`): **0 errors**.
- `cargo test -p semio-s-plugin-space --lib` (`🧪️3-f-space-plugin-test.txt`): **203 passed, 1 failed**
  (baseline handed to this lane: 198 passed / 1 failed). The 1 failure is the SAME known one named in the
  brief — `engine::space::component::tests::two_instances_converge_on_disjoint_edits_via_backbone`
  (framework store bug, not mine, `⚙️engine/**` forbidden to me). +5 new tests, all passing:
  `a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions`,
  `a_local_row_only_carries_an_open_action_button` (home editor),
  `a_row_stamps_the_space_row_id` (home viewer),
  `a_row_stamps_the_artifact_row_id_and_carries_dispatchable_open_and_delete_buttons` (space editor),
  `a_row_stamps_the_artifact_row_id_with_no_actions_cell` (space viewer). 198 + 5 = 203, exact match.
- **React DOM row id / row action dispatch** (brief's explicit ask): added 2 tests to
  `📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts` (where `TableHost`'s
  own pre-existing test already lived) —
  `"stamps a row's own id onto the rendered row's data-row-id attribute"` and
  `"dispatches a row action button's own ActionDescriptor, unmodified, on click"`. Verified via
  `bunx vitest run --config <that package's 🧪️vitest.config.ts> -t "row's own"` /
  `-t "dispatches a row action button"`: **1 passed** each, isolated. Full-suite run
  (`🧪️3-f-renderer-react-vitest-full.txt`): **313 passed, 9 failed** (322 total) — the 9 failures are
  ALL pre-existing/unrelated (`interprets virtual file system component scenes`, `ShellFaultBoundary`,
  `resolveWindowActions`, `commandCategories`, `SEMIO_LOCKED_*` footer/brand tests, `handleAction`
  round-trip, `renders selectable builder cards` — none mention table/row/TableHost); this file has 139
  uncommitted lines from concurrent peer lanes (`git diff --stat`), none overlapping my insertion point.
- `bun nx run @semio-tech/ui-react:test` (`🧪️3-f-ui-react-nx-test.txt`, brief's requested command —
  `Table`/`TableAvatar` live here, though `TableHost` does not): **515 passed, 10 failed** (525 total) —
  byte-identical to lane 2-F's own documented baseline (`UnifiedGumball` math, icon hover animations,
  `CanvasPickMenu`, `Shell` components, tree helpers ×2, `VirtualFileSystem` ×4). I made no changes to
  this package; unchanged baseline confirms no regression.
- Repo-wide sanity check, two OTHER table-using plugins: `cargo check -p semio-s-plugin-note
  -p semio-s-plugin-dag` (as literally suggested) hit **pre-existing, unrelated** errors in `note`
  (`SvgSnapshot`/`DwgSnapshot` field mismatches in its stdio import/export deserializers, committed
  2026-08-14, untouched by anyone since — confirmed via `git log --date=iso`/`git status`). Substituted
  per the brief's own escape hatch ("or two other table-using plugins that currently compile"):
  `cargo check -p semio-s-plugin-norm -p semio-s-plugin-mathematical`
  (`🧪️3-f-norm-mathematical-check.txt`) — `norm` (16 `TableWindowKit`-based DIN report windows): **0
  errors**. `mathematical` hit ITS OWN unrelated pre-existing error (`mathematical_geometry` vs.
  `mathematical_graph` typo, committed 2026-08-16 14:18, before this lane). Final clean sanity pair:
  **`semio-s-plugin-norm` (0 errors) + `semio-s-plugin-dag` (0 errors)** — both compile, both
  unaffected by the additive `TableWindowKit` change.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — additive `TableRow`/`TableRowAction`/
  `TableRowsView`/`TableWindowKit::render_rows` + 2 new tests, region `🔖️TableWindowKit`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🪟️window-kits/📊️table/
  🟦️component.ts` — additive TS twin + 2 new in-source tests.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/
  🪟️windows/🏠️main/🦀️component.rs` — `render_rows`/`row_actions` rewritten onto `render_rows`; +2 tests.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/
  🪟️windows/🏠️main/🦀️component.rs` — same, no actions; +1 test.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/
  🪟️windows/🏠️main/🦀️component.rs` — `render`/`row_actions` rewritten onto `render_rows`; +1 test.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/
  🪟️windows/🏠️main/🦀️component.rs` — same, no actions; +1 test.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/
  ⚛️react/🧪️index.test.ts` — +2 `TableHost` tests (row id → `data-row-id`, row action → `onAction`).
- Ticket scratch (kept, per brief): `🧪️3-f-*.txt` logs,
  `🧪️3-f-tablewindowkit-vitest.config.ts` probe config.

**Not touched**: `🖱️ui/🧱️elements/📜️HistoryTable/**` (peer-leased, MUTATION-OUTCOMES 2-D — confirmed
untouched via `git status`), the wgpu `Scenes/🧊️component.rs` `render_table`/`TableCellPayload` (already
correct, read-only verified), `TableHost/🟦️component.tsx` itself (already correct, read-only verified,
only its co-located test file gained 2 new cases).

## What is NOT done

- Rename/open-with row buttons on the Space artifact table (sharedFileRequest #1 above).
- Real en+de labels for the Space table's row action buttons (sharedFileRequest #2).
- Double-click-to-open row gesture (sharedFileRequest #3) — every row's "Open" button already satisfies
  the activate/open requirement without it.
- `semio-framework-os-renderer-wgpu`'s own test suite could not be run: `cargo test -p
  semio-framework-os-renderer-wgpu --lib table` fails to COMPILE via a transitive dependency on
  `semio-s-plugin-puzzle`, which has 3 pre-existing, unrelated `SemanticMutation` trait-bound errors
  (puzzle2d/3d/5d play-mutation mismatches) — nothing to do with tables, not touched by me, out of my
  scoped-check budget per the brief ("never `cargo check --workspace`"). The wgpu-side `render_table`/
  `TableCellPayload` code itself was verified correct by direct reading (step 4 of the Diagnosis), not
  by a passing test run — flagging honestly rather than claiming a test I could not execute.
- End-to-end click-through in a running shell/browser — no dev server was available in this lane's
  environment; every claim above is backed by a real `cargo test`/`vitest run`, never assumed.

Ticket not closed (coordinator owns that).
