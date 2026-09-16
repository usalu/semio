# Audit — `setPanelPage` plumbing and Tree contract schema pipeline

Read-only audit. Companion to the other `📓️audit-*` files already in this ticket folder
(`📓️audit-app-panels-a.md`, `📓️audit-host-tree-pipeline.md`, `📓️audit-paging-tests.md`,
`📓️design-virtualised-tree.md`) — this one focuses specifically on (a) how `setPanelPage` is wired
end to end today, so a framework-owned replacement's non-registration can be verified against a
concrete precedent, and (b) the exact file-by-file schema pipeline for the Tree contract.

---

## 1. Tree contract types, serialisation, and schema pipeline

### 1.1 Rust types

All in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs`:

- `RowActionPlacement` enum — line 106 (`Row | Menu`, `#[serde(rename_all="camelCase")]`, has
  `ToValue`/`FromValue`).
- `RowAction` struct — line 152: `{ icon: UiText, label: Option<Label>, action: ActionBinding,
  placement: RowActionPlacement }`. **No** `ToValue`/`FromValue` — deliberately, because `action:
  ActionBinding` embeds `UiValue`, which is the one DslValue-free exception the crate carries (see
  the file's header comment, lines 10-14). `credited_clone` is hand-written (line ~163).
- `TreeProps` struct — line 420: `{ interaction_domain: Option<UiText> }` only. Sections/items are
  children, not inline fields (comment right above, lines 411-419, documents this old→new shape
  change explicitly).
- `TreeSectionProps` struct — line 432: `{ label: Option<Label>, default_open: Option<bool> }`.
- `TreeItemProps` struct — line 451: `{ label: Label, description: Option<UiText>, icon:
  Option<UiText>, default_open: Option<bool>, draggable: Option<bool>, drag_data:
  Option<UiFixedMap<UiText>>, dimmed: Option<bool>, row_actions: UiFixedList<RowAction> }`. Also
  **no** `ToValue`/`FromValue` (same reason: `row_actions: UiFixedList<RowAction>` needs `RowAction:
  ToValue`). Has a hand-written `credited_clone` (line ~466).
- `Component` enum — line 526, with `Tree(TreeProps)` (540), `TreeSection(TreeSectionProps)` (541),
  `TreeItem(TreeItemProps)` (542) among ~19 variants.

### 1.2 Builder

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs`, region `🌲️Tree` (roughly lines
1360-1543, matches the task's cited range):

- `tree()` / `TreeBuilder` (≈1360-1394): only field is `interaction_domain`; `From<TreeBuilder> for
  BuiltNode` at line 1393 assembles `Component::Tree(TreeProps { interaction_domain: … })`.
- `tree_section(label)` / `TreeSectionBuilder` (≈1396-1432): `label`, `default_open`; assembled at
  line 1432 into `Component::TreeSection(TreeSectionProps { … })`.
- `tree_item(label)` / `TreeItemBuilder` (≈1435-1543): every `TreeItemProps` field has a builder
  setter (`description`, `icon`, `default_open`, `draggable`, `drag_data`, `dimmed`,
  `try_row_action`); `From<TreeItemBuilder> for BuiltNode` (≈1520-1542) assembles
  `Component::TreeItem(TreeItemProps { … })` field-by-field.

**Adding a field here means**: add the struct field (with `#[serde(default, skip_serializing_if=…)]`
+ matching `#[value(...)]` attr unless the struct opts out like `TreeItemProps`/`RowAction` do),
add a builder field + setter, and update the two `credited_clone` bodies if the struct has one.

### 1.3 Serialisation (ToValue / Serialize)

- `TreeProps`/`TreeSectionProps` derive both `Serialize/Deserialize` (serde, wire JSON) **and**
  `ToValue/FromValue` (`semio_framework_value_derive`, the `UiValue`/DslValue-adjacent in-process
  value form used by the plugin ABI) — see the `#[value(crate = "::protocol::value", …)]` attrs.
- `TreeItemProps` and `RowAction` derive **only** `Serialize/Deserialize`, never `ToValue/FromValue`,
  because they transitively hold `UiValue`/`ActionBinding`. This is a structural constraint, not an
  oversight — a new field on `TreeItemProps` does not go through the `ToValue` derive path at all.

### 1.4 JSON Schema / generated TS pipeline

The Tree contract's TypeScript mirror is **hand-maintained as string literals**, not derived by
macro reflection:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs` (this file **is** the crate's
  `schema_metadata` module — the root `🦀️.rs` declares it via `#[path = "🧬️schema/🦀️.rs"] pub mod
  schema_metadata;`, lines 22-23 of `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🦀️.rs`).
  - `pub const TYPES: &[SchemaMetadata]` (line 132) is a flat array of `{name, version,
    typescript}` entries, one per exported wire type, **80 entries total** (asserted by the
    typegen test — see below). `TreeProps`/`TreeSectionProps`/`TreeItemProps`/`RowAction`/
    `RowActionPlacement`/`Component` each have their own hand-written entry containing the literal
    TS `export type …` string (e.g. `TreeItemProps` at line 801, `TreeProps` at 819,
    `TreeSectionProps` at 835, `RowAction` at 586, `RowActionPlacement` at 600, the `Component`
    discriminated union at line 263).
  - `validate()` (line 1076) only checks version≠0, no duplicate names, and that each entry's
    `typescript` string actually contains `export type <Name>`/`export interface <Name>` — it does
    **not** check the TS shape matches the Rust struct. That correspondence is entirely on the
    author.
  - `render_typescript()` (line 1096) just concatenates all `TYPES[i].typescript` in array order
    with a generated-file banner.
- **Regeneration**: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts`
  registers nx targets on `@semio-tech/ui-contract-rs` (project file: `📋️project.json` next to it):
  - `generate` → `GenerateScript` → runs `cargo test -p semio-framework-ui-contract --features
    typegen --test typegen_export` with `SEMIO_TYPEGEN_OUT=<path>` and writes the rendered TS to
    `generatedUiContractPath()`, i.e.
    `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` (committed, 700
    lines; contains `TreeProps`/`TreeSectionProps`/`TreeItemProps`/`RowAction`/`RowActionPlacement`
    at lines 492/505/515/337/346 respectively, byte-for-byte the same strings as in `🧬️schema/🦀️.rs`).
  - `check` → re-runs the same test **without** `SEMIO_TYPEGEN_OUT`, which makes the test assert
    `rendered == include_str!(".../🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts")` (the actual
    test file: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧬️typegen-export/🦀️.rs`, which
    also asserts `schema_metadata::TYPES.len() == 80` and calls `schema_metadata::validate()`).
  - `check-wasm` also runs `cargo check -p semio-framework-ui-contract --target wasm32-wasip2
    --features typegen` (typegen is feature-gated so it never has to compile for real guest builds).
  - Command line: `bun nx run @semio-tech/ui-contract-rs:generate` (this is literally the banner
    string baked into the generated file's first line).

**So adding/renaming a Tree field is a 3-step manual edit, not a one-shot codegen**: (1) edit the
Rust struct in `🧩️component/🦀️.rs`, (2) hand-edit the matching TS string literal inside the
`SchemaMetadata` entry in `🧬️schema/🦀️.rs` (and bump `version` per that struct's convention if the
codebase does so elsewhere — check neighbouring entries), (3) run the `generate` nx target to
refresh the committed mirror, then `check` (or CI) enforces the two stay byte-identical.

### 1.5 Fixtures that pin the Tree contract shape

Grepping the contract crate's fixtures for `"tree"`/`"treeItem"`/`"treeSection"` turns up (non-
exhaustive but the material ones):

- `🧫️fixtures/🧪️conformance/🧩️component/🌳️tree/📸️snapshot.json` +
  `🧫️fixtures/🧪️conformance/🧩️component/🌳️tree/🎯️expect.json` — the conformance corpus case for
  a bare `Component::Tree`, driven by `📇️catalog.json` and consumed by
  `🧪️tests/🔬️conformance-unit/🦀️.rs` (mounted as `conformance` in the crate root) and the
  `conformance` nx target (`ConformanceScript` in the same `📜️script.ts`, filters on
  `conformance::`).
- `🧫️fixtures/🧪️conformance/🖥️composite/🌳️tree-nested-sections/📸️snapshot.json` +
  `…/🎯️expect.json` — a composite case nesting `TreeSection`/`TreeItem`.
- `♻️retirement/🌳️typed/🧬️schema/🔣️.json` (lines 43-45, 290-302) and
  `♻️retirement/🌳️typed/🧩️components.json` (lines 17-19) — the **byte-size** fixtures for the
  "built tree retirement" mechanism (credited-clone / patch retirement byte accounting). Line 19 of
  `🧩️components.json` hard-codes a `treeItem` literal with an expected `"bytes": 18` — a new
  `TreeItemProps` field that is ever non-default on a retired/foreign-credited node will change this
  number and the retirement law tests (`built_tree_retirement_*`, run via the
  `built-tree-retirement-check` nx target / `BuiltTreeRetirementScript`) will need updating.
- The top-level `🧬️schema/🔣️.json` (draft-07 JSON Schema) in the same dir is **not** the Tree
  contract's schema — it only carries the crate's three separately-registered named exports
  (`ConformanceCatalogFixture`, `ContractFixture`, `PresenceUpdate`; see
  `register_scope_exports()` at `🧬️schema/🦀️.rs` lines ~31-38). The Tree/Component wire shape's
  only normative projection is the hand-written TS in `TYPES`, not a JSON-Schema `$defs` entry.

### 1.6 TS-side decode / React target

- `builtNodeToSnapshot` — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx:379`.
  This is the OS renderer engine's own decode of a `BuiltNode` (the framework wire tree) into a flat
  `UiSnapshot` the store retains — it is **not** inside
  `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react`; that `🎯️targets/⚛️react` package under the
  contract module tree is a much smaller, mostly styling-adjacent target. The real per-component
  React decode/renderer lives in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`:
  - `🧱️elements/📃️UiDocumentStore/🟦️.tsx` — the retained document/store (holds `UiSnapshot`,
    revision, hashes).
  - `🧱️elements/🗣️Interpreter/🟦️.tsx` — the actual per-`Component` React renderer (interprets a
    decoded node into JSX; this is where a new Tree prop's read-side lives on the React consumer).
  - `🧱️elements/🏛️ShellHost/🟦️.tsx` — the shell/session driver (session state, `refreshUi`,
    tutorial recorder — see §3/§4 below).
  - `🎯️targets/⚛️react/🟦️.tsx` — the top-level React target entry.

## 2. `setPanelPage` end to end (per-app duplication)

Traced through puzzle3d (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/`) and confirmed
identically shaped in CAD, puzzle2d, and (structurally, same pattern) fem3d/fem2d/energy/etc.

1. **Row construction** —
   `…/🧊️3d/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs` (puzzle3d artifact-tree panel builder,
   `BODY_KEY = "puzzle.3d.play.artifact"` at line 27):
   - `page_action(section_id, page)` (≈line 258-260) builds the action tuple via
     `ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action("setPanelPage", Some(ui_value_map([("page", …), ("section", …)])))`.
   - `continuation_row_from(section_id, omitted, next_page)` (≈261-275) turns that into a
     `selectable_item(format!("{section_id}.more"), "+{omitted}", "more-horizontal", page_action(…))`
     tree row — the literal "+N" continuation row the ticket is replacing.
   - The generic paging machinery (`page_rows`, `PanelRowBudget`, `continuation_row`,
     `paged_panel_section`) is explicitly **not** duplicated — the file's own comment (≈205-210)
     says it "lives in the SDK (`semio_framework_plugin::{panel_page_rows, PanelRowBudget,
     panel_continuation_row, paged_panel_section}`) so every panel that outgrows one page … pages
     identically." Only the *action id* (`"setPanelPage"`) and its dispatch/registration are
     per-app.
   - Same shape in
     `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:96` (own
     `ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID)` call) and
     `✏️s/🔌️plugins/📐️cad/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs:163`.

2. **App-level command decode** —
   `…/🧊️3d/…/✏️editor/🎮️commands/📄set-panel-page/🦀️.rs`:
   ```rust
   pub fn set_panel_page(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
       let section = args.and_then(|v| v.get("section")).and_then(|v| v.as_str()).unwrap_or("").to_string();
       let page = args.and_then(|v| v.get("page")).and_then(|v| v.as_u64()).unwrap_or(0) as u32;
       if section.is_empty() { return; }
       ctx.scene.runtime.panel_pages.insert(section, page);
   }
   ```
   `ctx.scene.runtime.panel_pages` is the app's own `BTreeMap<String, u32>` (mirrored in puzzle2d as
   `…/🎚️config/🦀️.rs:63` / `…/🪟️window/🦀️.rs:168` doc comments: "Per-section page cursor of the
   virtualised panels").

3. **Dispatch wiring, all per-app, all hand-written**, e.g. puzzle3d's monolithic
   `…/🧊️3d/…/✏️editor/🦀️.rs`:
   - `SetPanelPage = "setPanelPage"` constant (line 2618).
   - Listed in a `toolIds`/action-name array (line 2688, and again 3864, 7696).
   - `match action { … "setPanelPage" => set_panel_page::set_panel_page(ctx, args), … }` (line
     3744) — the literal `command_from_action`/dispatch match arm.
   - Manifest registration: `.view_action("setPanelPage", LocalizedLabel::native("Set Panel Page",
     "Panel-Seite festlegen"))` (line 8575) and
     `.action_interactive_job("setPanelPage", InteractiveJobClassification::Migrated)` (line 8735)
     — the mandatory classification (see `InteractiveJobClassification` below) that
     `validate_interactive_job_classification` (manifest crate) will fail closed on if omitted
     (default is `Unclassified`, rejected before a release catalog can activate — manifest
     `🦀️.rs` lines 726-733/850-860).
   - `ArtifactToolPublicationContract { tool_id: "setPanelPage", lanes: &[…WindowConfig] }` (line
     7290) — publication-lane declaration.
   - CAD's editor repeats every one of these independently (own `SetPanelPage` command type at
     `…/📐️cad/…/✏️editor/🦀️.rs:1202/1230`, own `.action_with(…).action_interactive_job(…)` at
     lines 2427/2500, own fixtures at `…/🧫️fixtures/🗄️retained-jobs/🔣️.json:43,360`); puzzle2d
     repeats it a third time (`…/◻️2d/…/✏️editor/🦀️.rs:1230,1268,1373,1932,3869,4172,4227`).
   - Also present in each app's static `📦️packages/🟦️typescript/📜️script.ts` allow-list of tool
     ids (e.g. CAD's `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts:97`) and in the
     CAD plugin's own top-level `🔣️.json` manifest at 4 separate line numbers (954, 4201, 7448,
     10695 — one per subset/standard variant).

**Conclusion for §2**: `setPanelPage` is entirely app-declared — a constant, a dispatch match arm,
a manifest `ActionDefinition` + `InteractiveJobClassification`, a publication-lane entry, and a
`toolIds` allow-list entry, each duplicated verbatim (modulo controller id) in every app that pages
a panel (puzzle3d, puzzle2d, CAD, and by the same shape fem3d/fem2d/energy/layout/space/vcs/note/
draw/raster/forms/process3d/block/lowpoly/gis/shooting/generation3d — all of these showed up in the
`Component::TreeItem(props)` grep in §6 below with their own paged-panel builder file). A
framework-owned replacement removes all of steps 2-3 for every one of these apps; only the row-
construction call in step 1 (which action id to bind, if any at all) changes.

## 3. Framework-owned actions that need **no** per-app registration

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (the `semio_framework_plugin`/OS plugin
runtime crate — 39,745 lines):

- `PluginApp::dispatch_action` (≈line 26069-26082, doc comment "B1: FRAMEWORK-reserved verbs only
  … an app's own behavior is dispatched exclusively through `dispatch_typed_command` now"):
  ```rust
  if A::ROLE == AppRole::Viewer && VIEWER_REJECTED_ACTION_IDS.contains(&action) { … }
  if is_tool_run_action_id(action) { return self.dispatch_tool_run_action(…).await; }
  if is_framework_reserved_action_id(action) { return self.dispatch_framework_reserved_action(…).await; }
  let definition = self.registry.get(action).ok_or_else(|| Fault::new(…"unknown action key"…))?;
  ```
  This is the **exact fork point**: `is_framework_reserved_action_id` (line 20976) is checked
  *before* the app's own `self.registry.get(action)` lookup. An action id in that set is fully
  handled by the framework and never needs to exist in any app's manifest/registry at all.
- `is_framework_reserved_action_id` (line 20976-20983) is the union of:
  - `HISTORY_ACTION_IDS` (6: undo/redo/commitCheckpoint/createAlternative/switchAlternative/
    checkoutCheckpoint),
  - `CLIPBOARD_ACTION_IDS` (3: copy/cut/paste),
  - `INTERACTION_ACTION_IDS` (line 20974, **6**: `INTERACTION_SELECT_ACTION_ID`,
    `INTERACTION_HOVER_ACTION_ID`, `CLEAR_SELECTION_ACTION_ID`, `SELECT_ALL_ACTION_ID`,
    `SET_SELECTION_MODE_ACTION_ID`, `SET_INTERACTION_GRANULARITY_ACTION_ID`),
  - tool-run ids, plus `REVERT_TO_COMMAND_ACTION_ID`/`SET_HISTORY_COMMAND_FILTER_ACTION_ID`/
    `NOTE_SHELL_COMMAND_ACTION_ID`/`RECORD_TUTORIAL_ACTION_ID`.
- `framework_reserved_route_job(action, raw, work_items)` (line 15717) is the ONE match that turns a
  reserved action id into its `ArtifactReservedToolJob` (e.g. `INTERACTION_SELECT_ACTION_ID =>
  ArtifactReservedToolJob::new(FrameworkInteractionSelectJob::new(raw, work_items))`, line 15734),
  each backed by a `framework_reserved_job!` macro invocation a few lines above (line
  15691-15694). **This is the template for adding a brand-new framework-owned action**: declare a
  new `framework_reserved_job!(...)`, add a match arm here, add the id to the relevant
  `*_ACTION_IDS` array (or a new one, wired into `is_framework_reserved_action_id`), and optionally
  add it to `FRAMEWORK_SHARED_ACTION_DESCRIPTOR_ROUTES` (line ≈25742, pairs an action id with an
  agent-facing `schema_id` — 12 entries currently) if it should also be MCP/agent-addressable.
- `setActiveUtility`/`setActiveTool` are a **second, weaker** pattern — not reserved-routed, but
  auto-injected into the app's own registry by the shared manifest builder rather than hand-declared
  per app: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5303`:
  ```rust
  if !self.utilities.is_empty() && declared_action_ids.insert(SET_ACTIVE_UTILITY_ACTION_ID.to_string()) { … }
  ```
  (mirrored for `SET_ACTIVE_TOOL_ACTION_ID`, doc comment at manifest `🦀️.rs:1343/1359`). These
  still flow through `self.registry.get(action)` and the ordinary `dispatch_emit` path (plugin
  `🦀️.rs:26095`, `matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID)`), so
  they are framework-DECLARED but not framework-DISPATCHED — a middle ground worth knowing about but
  not the strongest precedent for a per-panel viewport action (the `is_framework_reserved_action_id`
  short-circuit in `dispatch_action` is the stronger one, since it never touches the app's registry
  at all).
- `treeExpansion` (found in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6073`,
  `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs:1511`,
  `…/🏛️ShellHost/🟦️.tsx:1071-1072`, `…/🛠️ShellHelpers/🟦️.tsx:2691`) is **not** a live
  host→guest or guest→host action at all — it is one variant of `TutorialUiChange`, the shell's
  own tutorial-recorder diff alphabet. `ShellHost`'s `diffTutorialUiSnapshot` (≈`🛠️ShellHelpers/🟦️.tsx:2703-2706`)
  diffs a `Set<string>` of `expandedTreeIds` between two `TutorialUiSnapshot`s purely client-side and
  emits `{kind:"treeExpansion", id, expanded}` deltas for tutorial playback; `applyTutorialUiChange`'s
  `case "treeExpansion":` (≈2691) replays it into `dispatch({type: "SET_TREE_OPEN_STATE", …})`, a
  React reducer action local to the shell. **This is nonetheless directly useful precedent**: the
  React host already maintains a per-tree-id expanded/collapsed set (`expandedTreeIds`) entirely on
  its own side today, for an unrelated purpose (tutorial recording). The new virtualisation protocol
  needs exactly this data (which containers are expanded) — the host doesn't have to invent tracking
  it, only relay what `SET_TREE_OPEN_STATE` already knows to the guest.
- `INTERACTION_SELECT_ACTION_ID`-driven per-domain selection is the concrete example of a
  **framework-owned per-panel state an app's `Present`/render reads without ever registering
  anything**: `InteractionView<'a>` (plugin `🦀️.rs:9485`) wraps `state: &InteractionState`
  (persisted) + `hover: &InteractionHoverState` (ephemeral) + `peers`, exposing `.selection(domain)`
  / `.hover(domain, channel)`. `ArtifactApp::render_with_request_context` (trait decl at plugin
  `🦀️.rs:11583`; fem3d impl at
  `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs:1085-1096`) takes `interaction:
  &InteractionView<'_>` as a parameter alongside `doc`/`cfg`/`view_state`; fem3d's doc comment on
  that impl says it plainly: *"Reads the framework-owned `"fem3d"` selection/hover once per render
  and threads it through every body — the windows paint it, the artifact tree marks it, the
  inspector edits it."* It builds a `Fem3dInteractionSnapshot::from_interaction(interaction)` (line
  1096) and threads that into `render_body` → `build_artifact_tree(document, interaction, labels)`
  (`📌️panels/🗿️artifact/🦀️.rs:365,396`).

**Yes — a new framework action `panelViewport`/`treeWindow` fits this exact mould**: add it (and a
job) to the `is_framework_reserved_action_id` family in §3 so no app ever registers it; store the
per-panel (offset, count) window in a new framework-owned state type analogous to
`InteractionState`/`InteractionHoverState`; expose it through a new `*View<'_>` parameter threaded
into `render_with_request_context` exactly like `InteractionView`; each app's `render_body` reads
it and passes it into its panel-tree builder instead of calling `paged_panel_section`. The
re-present trigger is the same mechanism selection changes already use — see §5.

## 4. Panel geometry the guest can see today — none below whole-window size

- `BROWSER_EVENT_METRICS` = `1_801` (`🧰️framework/🔨️modules/🖱️ui/🖥️host/📡️event/🦀️.rs:35`),
  decoded at line 103 into `BrowserHostEvent::Metrics(BrowserCanvasMetrics { width, height,
  scale_factor })` — this is the **whole canvas/window** pixel size and DPI scale, nothing
  panel-scoped.
- `WindowMetrics` (`🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs:32`): `{ physical:
  PhysicalSize, scale_factor: f32 }`, plus `logical_size()` — again whole-window only.
- No `rows_visible`/`panel_height`/`visible` concept exists anywhere in
  `🧰️framework/🔨️modules/🖱️ui/🖥️host` or the os plugin crate — confirmed by grep across both
  trees; the terms return zero hits outside test names for scroll/viewport in the 3D world sense
  (camera viewport, not panel row window). **The guest currently has zero visibility into panel
  scroll position, panel pixel height, or which rows of a tree are actually on screen.** Any
  virtualisation protocol has to introduce this from scratch; there is no existing partial signal to
  extend.
- `Trigger` enum (the thing `UiIntent`/`ActionBinding` actually carries — the task's "UiIntent
  enum" is `UiIntent` the **struct**, at `🧬️contract/🎬️action/🦀️.rs:1767`; its `trigger` field is
  the enum) — `🧬️contract/🎬️action/🦀️.rs:1691-1702`:
  ```rust
  pub enum Trigger { Activate, Change, Commit, Delta, Drop, Submit, Abort, RepeatLast, HoverPreview }
  ```
  All 9 variants reach the guest identically — `UiIntent` (fields: `surface`, `revision`, `node`,
  `node_key`, `trigger`, `action`, `args`, `input`, `seq`) is the one message shape a renderer emits
  for every user action, dispatched by the headless runtime regardless of which `Trigger` it
  carries; there is no guest-side filtering by trigger kind. A new "viewport changed" signal would
  most naturally be its own new framework action (see §3) carrying `(section_id, offset, count)` as
  `args`, dispatched on `Trigger::Change` or a fresh trigger, rather than overloading an existing
  one.

## 5. `Present`/`render` invocation and panel-body caching

- Trait: `ArtifactApp::render` (`🔌️plugin/🦀️.rs:11583`) — `async fn render(body_key: &str, doc:
  &ArtifactView<'_, Snapshot>, cfg: &ConfigView<'_, Config>, view_state: &ViewModel) ->
  UiAssemblyResult<ComponentTree>`, and the richer `render_with_request_context` (adds `owner`,
  `transient`, `interaction: &InteractionView<'_>`).
- Object-safe wrapper: `PluginApp::render(&mut self, body_key: &str, snapshot_override_json:
  Option<&str>, view_state: &ViewModel) -> Result<ComponentTree, Fault>` (declared at plugin
  `🦀️.rs:12188`, impl at 29948).
- **Call site — this is the caching/lazy answer**: `🔌️plugin/🦀️.rs:35990-36049`, the
  `refresh`/`RefreshRequest` handler:
  ```rust
  for entry in &request.windows {
      let window_view_state = request.view_state.for_window_instance(&entry.key)…;
      let node = resolve_ready(instance.app.render(&entry.body_key, None, &window_view_state))?;
      …
  }
  let panel_view_state = request.view_state.for_panel();
  for entry in &request.panels {
      let node = resolve_ready(instance.app.render(&entry.body_key, None, &panel_view_state))?;
      …
  }
  ```
  `request.panels`/`request.windows` are **caller-supplied** lists of `{key, body_key, hash}` —
  the HOST decides, per refresh call, which body keys get re-rendered at all; a body key absent
  from the request is never rendered that turn (and its previous `hash`/value stands). The TS side
  (`ShellHost/🟦️.tsx`) already has a **partial** refresh scope precedent —
  `refreshUi(currentSession, { kind: "partial", panelBodies })` (`🏛️ShellHost/🟦️.tsx:9914`) and
  `{ kind: "partial", windowBodies }` (line 10428) alongside the default `{ kind: "full" }` calls —
  so "only render panels whose tab is currently visible/expanded" is architecturally exactly what
  the `panels` list construction on the TS side would need to filter, not a new mechanism.
- `PanelTabDefinition.body_key: Option<String>` (manifest `🦀️.rs:3418`; plugin-crate mirror at
  `🔌️plugin/🦀️.rs:713`) is the declaration a panel tab carries; `body_key: def.body_key` is copied
  1:1 through `panel_tab_definition_to_spec`/`panel_tab_spec_to_definition` (plugin `🦀️.rs:754,
  771, 794`) with no separate "is this tab currently mounted" flag on the Rust side — that
  liveness decision is made purely by whether the TS host includes the tab's body key in
  `request.panels` for a given refresh, which in turn is presumably driven by which `PanelTabBar`
  tab is active (not traced further in this pass — the `panels: []` construction site in
  `🛠️ShellHelpers`/`🏛️ShellHost` would be the next place to look to confirm it's filtered to only
  the visible tab, per the ticket's `📓️audit-host-tree-pipeline.md`).

## 6. wgpu Tree target and other non-React consumers of `TreeItemProps`

Two distinct "wgpu tree" things exist and must not be conflated:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs` (257 lines) — this is
  the **old, generic widget-level** `TreeItem<E>`/`TreeSection<E>` painter (measure/render
  functions, `TREE_ROW_HEIGHT` etc., imported from `crate::wgpu::widgets`). It does **not**
  reference `Component`/`TreeItemProps` at all — it operates on its own pre-existing generic widget
  structs, one layer removed from the new contract.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` — this is the **actual
  consumer of `Component::TreeItem`/`TreeProps`/`TreeSectionProps`** from the new contract, and the
  file that bridges old-widget-tree ⇄ new-contract-tree:
  - `row_action(action: &RowAction, controller)` (line ~460) maps `RowAction` → the legacy
    `UiTreeItemAction` (icon/label/`ActionDescriptor`/placement).
  - `tree_item(document, record, surface, controller, depth)` (line 480) pattern-matches `let
    ui_contract::Component::TreeItem(props) = &record.component else { … }` and reads every
    `TreeItemProps` field (`label`, `description`, `icon`, `default_open`, `draggable`,
    `drag_data`, `dimmed`, `row_actions`) to build a `UiTreeItemNode` for the legacy `UiNode::Tree`
    wire shape.
  - `Component::Tree(props)` arm (line 778) reads `props.interaction_domain` and walks children,
    building `UiTreeSectionNode`s from `Component::TreeSection` children.
  - `Component::TreeSection(_) | Component::TreeItem(_)` also has a **second** arm (line 800, in
    the sibling `children_of`-style match) that renders these records as plain `UiNode::Stack` rows
    for the interactive/hit-testing sync path — "the document-path twin of `children_of`'s `Tree`
    arm," per its own comment.
  - **This file is the one non-React file that must be touched for any new `TreeItemProps`/
    `TreeProps`/`TreeSectionProps` field that needs to reach the legacy wgpu-target visual/hit-test
    path.**
- Every app plugin's panel builder destructures `Component::TreeItem(props)` directly (not through
  any shared helper) to rewrite a row's shape for its own paging continuation logic — a
  representative, far-from-exhaustive sample from the repo-wide grep (all pattern `if let
  semio_framework_plugin::Component::TreeItem(props) = &mut …`): CAD
  (`📐️cad/…/✏️editor/🦀️.rs:427,439`, `…/📌️panels/🗿️artifact/🦀️.rs:61,78`), puzzle raster
  (`🖨️raster/…/📌️panels/🎭️masks/🦀️.rs:25`, `…/📌️panels/🗿️artifact/🦀️.rs:45,70`), process3d
  (4 call sites), fem3d/fem2d (`…/📌️panels/🗿️artifact/🦀️.rs:190,274` and `:197,282`), energy (4
  call sites in `📌️panels/🗿️artifact/🦀️.rs`), plus block/lowpoly/space/vcs/gis/note/forms/draw/
  layout/generation3d/procedural-core/shooting — roughly 40+ call sites total. None of these use
  `TreeItemProps { .. }` exhaustive destructuring (all bind `props` and mutate named fields), so
  **adding a new `Option<T>` field to `TreeItemProps` will not break any of these call sites at
  compile time** — but every one of them is exactly the kind of per-app paging/continuation-row
  logic the new framework protocol is meant to delete.

## Extension points

**(i) Files to touch to add a field to `Tree`/`TreeSection`/`TreeItem` props end to end:**
1. `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` — add the field to the relevant
   struct (`TreeProps` L420 / `TreeSectionProps` L432 / `TreeItemProps` L451), with
   `#[serde(default, skip_serializing_if=…)]` (+ `#[value(...)]` unless the struct is one of the
   two `ToValue`-exempt ones), and update its `credited_clone` if present.
2. `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` — add a builder field + `impl
   …Builder` setter method, and extend the `From<…Builder> for BuiltNode` assembly call (region
   `🌲️Tree`, ≈L1360-1543).
3. `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs` — hand-edit the matching
   `SchemaMetadata` entry's `typescript:` string literal for the struct (`TreeItemProps` L801,
   `TreeProps` L819, `TreeSectionProps` L835) to add the new TS field.
4. Run `bun nx run @semio-tech/ui-contract-rs:generate` to refresh
   `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`; `…:check` (or CI) will
   otherwise fail the byte-identity assertion in
   `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧬️typegen-export/🦀️.rs`.
5. Update conformance/retirement fixtures that hard-code the shape:
   `🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🌳️tree/{📸️snapshot,🎯️expect}.json`,
   `…/🖥️composite/🌳️tree-nested-sections/{📸️snapshot,🎯️expect}.json`,
   `🧬️contract/♻️retirement/🌳️typed/🧬️schema/🔣️.json` and `…/🧩️components.json` (byte-count
   literal at line 19 will change for any new default-diverging `treeItem` field).
6. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` — read the new field inside
   `tree_item()`/the `Component::Tree(props)` arm if the legacy wgpu visual/hit-test target needs to
   honour it (React's `builtNodeToSnapshot`/Interpreter path in
   `…/📺️renderer/🧑‍🎨engine/🧱️elements/{📃️UiDocumentStore,🗣️Interpreter}/🟦️.tsx` is the other
   consumer to check).

**(ii) Adding ONE framework-owned action reaching a framework-owned per-panel state (no per-app
registration), following the `interactionSelect` precedent exactly:**
1. Define a new state type analogous to `InteractionState`/`InteractionHoverState` — e.g. a
   `PanelViewportState` keyed by panel-section id → `{offset, count}` — owned by the framework
   runtime (same home as `InteractionState`, in the plugin crate `🔌️plugin/🦀️.rs`).
2. Register the action id (e.g. `PANEL_VIEWPORT_ACTION_ID = "panelViewport"`) into
   `is_framework_reserved_action_id` (`🔌️plugin/🦀️.rs:20976`) — either add it to a new
   `*_ACTION_IDS` array or extend an existing one if semantically close to `INTERACTION_ACTION_IDS`.
3. Add a `framework_reserved_job!(FrameworkPanelViewportJob, FrameworkPanelViewportJobFactory,
   "panelViewport", <next_route_id>, …)` declaration (pattern at `🔌️plugin/🦀️.rs:15691-15694`) and
   a matching arm in `framework_reserved_route_job` (`🔌️plugin/🦀️.rs:15717-15736`).
4. Optionally add it to `FRAMEWORK_SHARED_ACTION_DESCRIPTOR_ROUTES` (≈L25742-25753) with a
   `framework.reserved.panelViewport.v1` schema id if it should be MCP/agent-addressable.
5. `dispatch_action` (`🔌️plugin/🦀️.rs:26069`) already routes any `is_framework_reserved_action_id`
   hit to `dispatch_framework_reserved_action` before ever touching `self.registry` — **no app
   manifest, dispatch match arm, or `toolIds` entry is needed**, which is the entire point relative
   to `setPanelPage`'s §2 duplication.
6. Expose the new state to apps via a new `PanelViewportView<'_>` parameter threaded into
   `ArtifactApp::render_with_request_context` (trait decl `🔌️plugin/🦀️.rs:11583`), mirroring
   `interaction: &InteractionView<'_>` exactly — construct it once per render call from the
   framework-owned state, same as `InteractionView` is built from `InteractionState`/
   `InteractionHoverState`.

**(iii) How the app re-presents after that state changes:** identical to how `interactionSelect`
already triggers a re-present today — dispatching the framework-reserved action produces a normal
`InvocationResult` from `dispatch_framework_reserved_action` (`🔌️plugin/🦀️.rs:25703`), which
carries a `UiDirtyScope` (see the neighbouring `Self::empty_result(action, meta, …, UiDirtyScope::…)`
calls in the same function, e.g. line 25710) that marks the panel/window body dirty; the next
`refreshUi` call from the TS host (`🏛️ShellHost/🟦️.tsx`) — which is already invoked after every
dispatched action, per the many `void refreshUi(session, …)` call sites following action dispatch —
re-renders exactly the dirtied `body_key`s via the `request.panels`/`request.windows` loop in
`🔌️plugin/🦀️.rs:36037-36049` (§5 above), which calls `instance.app.render(...)` → the app's
`render_body`/panel builder reads the new `PanelViewportView` and materialises only the requested
row window. No new re-present trigger mechanism is needed; the existing dirty-scope +
`refreshUi(partial)` pipeline that already serves `interactionSelect` carries it.
