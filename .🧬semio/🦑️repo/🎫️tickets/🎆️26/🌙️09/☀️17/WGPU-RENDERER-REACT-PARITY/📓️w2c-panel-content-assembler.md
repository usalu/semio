# 🧾 W2c — the 8-anchor dock's CONTENT in the wgpu Shell (panel records assembler)

Packet W2c of ticket 26/09/17 WGPU-RENDERER-REACT-PARITY. Closes W1h's remaining gaps 1–7, W1c's
`expanded_command_id`-has-no-reader gap, W1j's chat-transcript gap, the shell audit's recommendation 7
(panel content-projection escape hatch) and `📓️audit-plugin-react-only-surfaces.md` §6.1's
"~471 React lines of keyboard-shortcut registry with zero wgpu counterpart". All paths absolute under
`/Users/ueli/Documents/semio`.

Primary file: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
(referred to below as **the shell file**). Line numbers are this packet's final revision.

---

## 0. The one thing that unblocked everything else: a real `UiNode → UiNodeRecord` assembler

W1h §5 recorded the shape of the problem exactly: *"no `UiNode → UiNodeRecord` assembler exists in the
shell"*, so the Commands panel had to be **re-expressed as `WindowMeasure`s** to be publishable at all.
That vocabulary is `Select`/`Toggle`/`Slider`/`Number`/`Group` — no text, no button, no nesting — which
is why every other shell-owned panel (Settings' five leaves, the Tool leaves, Marketplace, the chat
transcript, the staged command form) was structurally unpublishable, not merely unwritten.

**New** (shell file, region `//#region 🧾️PanelRecordsAssembler`):

| Symbol | Line | What it is |
|---|---|---|
| `panel_ui_records(surface_id, root)` | 3899 | `UiNode` subtree → `Vec<ui_contract::UiNodeRecord>`, pre-order (parent-first, densely numbered from id 1) |
| `PanelProjection` / `PanelRecord` | 3917 / 3906 | the reserve-then-place idiom `WindowMeasuresProjection` already used, so record order is parent-first |
| `ShellState::publish_surface_records(surface, records)` | 3781 | the lease mint extracted out of `publish_window_measures`: content-hash revision → `ui_document_ingress_generation` → `UiDocumentLease::try_publish`. One ingress for every shell-owned body, so an unchanged panel pays nothing |
| `ShellState::shell_owned_panel_leaves()` | 13897 | every leaf id the shell publishes a body for, derived from `default_dock`'s own sources |
| `ShellState::publish_shell_panel_document(tab_id)` | 13910 | routes a leaf id to its builder and publishes it |

Projected variants: `Stack`, `Section`, `Field`, `Text`, `Separator`, `Button`, `Select`, `Toggle`,
`Input`, `KeyValue`, `Progress`, `Slider`. A variant no shell builder authors (`Tree`, `Image`,
`ComponentScene`, `ExternalSlot`, `Group`, `Ring`, `IconSelect`, `NumberStepper`) is **refused loudly**
(`panel_ui_node_tag`), so a builder that grows one fails at publication instead of painting a hole —
pinned by `the_panel_assembler_refuses_an_unprojectable_node`.

`refresh_ui`'s panel loop (5285-5300) now iterates `shell_owned_panel_leaves()` instead of only the
command categories, and keeps all of them in `retained` so `retire_documents_outside` never sweeps them.

`command_panel_measures` is **deleted**: the Command dock publishes through the same assembler now
(`publish_command_panel_document`, 13884). `build_command_panel_ui` went back to `#[cfg(test)]` with an
honest docstring — it is the *all-categories oracle* the command-registry law asserts against, and
`build_command_category_ui` (13939) is what the anchor actually publishes.

---

## 1. Framework Settings branch on `bottom-right`, with five children

### 1.1 The branch

`ShellState::default_dock` (6570) now builds `bottom-right` as React's `🧭️DockAssembly` does
(`🏛️ShellHost/🟦️.tsx:9438-9439`):

```
bottom-right = [ framework.settings (branch) , framework.marketplace , framework.panel.history ]
```

* The app's own Settings-group tabs are **partitioned** out of the anchor and nested as the branch's
  FIRST children, then the five framework leaves follow with their order shifted past them — a faithful
  port of `integrateAppSettingsPanelTabsIntoFrameworkBranch` (`🛠️ShellHelpers/🟦️.tsx:1391-1397`,
  `order: (tab.order ?? index) + appSettingsTabs.length`).
* `framework.panel.history` is pulled out of the app's tabs and re-added as a **sibling** of the
  branch, which is React's `shellRendersPanelTabItself`/`frameworkUtilitiesHistoryTab` pair
  (`🛠️ShellHelpers/🟦️.tsx:1408-1413`, `🏛️ShellHost/🟦️.tsx:8982`) — mounting it inside would put two
  tabs with one id in the anchor, the exact defect React's own comment there documents.
* `framework_settings_tabs()` (6641) emits general / theme / keybindings / default-apps / conflicts in
  React's order, and **omits the theme leaf outright** when the boot descriptor locks the theme id —
  React's `locks.themeId` render gate (`📌️ChromePanels/🟦️.tsx:1047`), which removes the tab rather
  than disabling it.

Ids are byte-identical to `📌️ChromePanels/🟦️.tsx:71-82` (`FRAMEWORK_SETTINGS_PANEL_ID` … 
`FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID`), declared at shell file 126-140.

### 1.2 The five bodies

| Leaf | Builder | Line | React reference | Content |
|---|---|---|---|---|
| `framework.settings.general` | `build_settings_general_ui` | 6087 | `buildSettingsGeneralTree`, `📌️ChromePanels/🟦️.tsx:348-578` | `framework.settings.app` section (name/id/controller/plugin text rows) + `framework.settings.general` section: appearance, layout, driver, language, terminology selects and the Reset Dock button. Every row id and every option `value` is React's |
| `framework.settings.theme` | `build_settings_theme_ui` | 6034 | `buildSettingsThemeTree`, `:693-828` | **un-gated** from `#[cfg(test)]`. Theme select + Reset + Delete (gated on a `custom.` id) |
| `framework.settings.keybindings` | `build_settings_keybindings_ui` | 6186 | `buildSettingsKeybindingsTree`, `:844-896` | one row per shortcut: the chord in force through `format_keybinding_shortcut`, a conflict suffix, a capture control, and a reset control on an overridden row |
| `framework.settings.default-apps` | `build_settings_default_apps_ui` | 6248 | `buildSettingsDefaultAppsTree`, `:929-974` | one `Select` per (dialect × role) the loaded manifests can open, first option `"none"` (React's `DEFAULT_APP_NONE_VALUE`), values encoded `"<pluginId> <appId>"` (`encodeDefaultAppValue`, `:902`) |
| `framework.settings.conflicts` | `build_settings_conflicts_ui` | 6331 | `buildConflictsTree`, `:995-1031` | React's own unavailable row — see gap 3 |

Locks are honoured the way React's are: `locks.appearance`/`locks.locale`/`locks.terminology` **remove**
their rows (`crate::boot_locks()`), never grey them out.

New `"framework"` dispatch arms (6745-6835): `setLayout`, `setDefaultApp`, `captureKeybinding`,
`resetKeybinding`, `stageCommandArg`, `resetStagedCommand`, `executeStagedCommand`, `setChatDraft`,
`sendChatDraft`, `clearChat`. `setLayout` logs `os.setLayout` through `note_shell_setting_command` like
every sibling setting, so a direct row press and a ⌘️K command fold into one History row.

### 1.3 The keybinding REGISTRY — the remappable one, not W1c's fixed list

`📓️audit-plugin-react-only-surfaces.md` §6.1 was right that W1c's `SHELL_SHORTCUT_ROWS` is *not* this:
it was a `const &[(&str, &str)]`, **unremappable by construction**. The override lane already existed
end-to-end on this side (`ChromePrefsState.keybinding_overrides` at 17128, persisted through
`set_keybinding_override` at 17706-17714) and had **no reader at all**.

| Symbol | Line | What it is |
|---|---|---|
| `shell_shortcut_table(overrides)` | 13237 | the resolved table — `SHELL_SHORTCUT_ROWS` with the override applied per control id, React's `composeControlKeybindings` (`🕹️control-keybinding-context/🟦️.tsx:193-202`). A chordless override is dropped rather than blanking the row (React's `parseUiKeybindingOverrides` gate) |
| `shell_shortcut_for_in(table, …)` | 13249 | the resolver over a resolved table, first matching row wins |
| `is_reserved_shell_chord_in(table, …)` | 12482 | reservation over the same table, so a remapped chord is reserved at its NEW spelling and the default it replaced becomes an app's to claim — React's `reservedShellChordsV1(SHELL_KEYBINDINGS, uiKeybindingOverrides)` |
| `ShellState::shortcut_table()` | 11540 | this session's resolved table; **every production key reader goes through it** (10883, 11064, and both `is_reserved_shell_chord_in` rungs) |
| `shell_keybinding_rows(overrides)` | 13279 | the settings rows, control-id sorted, each carrying `keys` / `overridden` / `conflicted` |
| `ShellKeybindingRow` | 13266 | that row |
| `ShellState::capture_keybinding_chord` | 11548 | one-gesture remap: an armed row swallows the next key event and writes its chord. Modifier order is React's `chordFromKeyboardEvent` exactly (`ctrl, meta, alt, shift`, then the key); `Escape` cancels and still consumes |
| `keybinding_capture_key_token` | 13116 | the inverse of `key_event_matches_chord`'s key arm, so a captured chord is one this renderer can match again |

**Conflict model**, ported verbatim from `📌️ChromePanels/🟦️.tsx:845-865`: last writer of a row's FIRST
chord owns it, so exactly the *earlier* row of a colliding pair is flagged, the marker is cosmetic, and
alternatives past the first comma are never compared. Pinned by
`keybinding_rows_flag_the_earlier_owner_of_a_shared_chord`.

**Two dead predicates deleted.** `shell_role_chord` and `shell_mode_step_chord` were hand-rolled
ladders run AHEAD of the table, which (a) made `playground.navbar.roles.*` and `ui.shell.mode.*`
unremappable even once the table became remappable, and (b) matched a chord held with Shift that
React's own `keyboardEventMatchesChord` refuses. Both rows are ordinary table rows now (shell file
11670 carries the dedup note). `shell_shortcut_for` / `is_reserved_shell_chord` survive as
`#[cfg(test)]` default-table projections — that is what the existing shortcut LAWS assert against.

`format_keybinding_shortcut` gained the missing `arrowup`/`arrowdown`/`arrowleft`/`arrowright` glyph
arms (11486-11489); it previously only knew the bare `up`/`down`/`left`/`right` spellings while the
table declares `mod+alt+arrowright`, so the mode-cycle chord rendered as `⌘️⌥️Arrowright`.

---

## 2. Tool branch leaves, and the Marketplace leaf

**Tool** (`tool_panel_tabs`, 6656; branch emitted at 6614): one `tool.<id>` leaf per tool the ACTIVE
MODE declares, in declared order, via `semio_framework::manifest::resolve_mode_tools` — the Rust twin
`buildToolTabs` reads through (`🛠️ShellHelpers/🟦️.tsx:4756`, `🏛️ShellHost/🟦️.tsx:9413-9422`). Label and
icon come off the tool's own manifest entry through the live terminology and locale. The branch sits
**left of Command** (order 0 vs 1) with React's `"hammer"` fallback icon, not Command's `"wrench"`.

The leaf tab IS the activation control, which is React's whole `reconcileToolTabSelection` point
(`🛠️ShellHelpers/🟦️.tsx:4790`): `ShellState::arm_tool_for_panel_tab` (11524) dispatches `setActiveTool`
whenever a selected path ends on a tool leaf, from **both** routes into the path (the desktop tab row
and the mobile one), and a path that selects no tool leaf is left alone rather than deactivating — the
"skip while not the active root" rule that keeps a tool live while another category is browsed.

Body: `build_tool_panel_ui` (6467) — one headerless options section reporting the tool's armed state,
which is React's `buildToolTree` (`:4734`) with no measures in hand. See gap 1.

**Marketplace** (`build_marketplace_ui`, 6352): one section per plugin source with a
`label · version · status` row per plugin, React's `buildMarketplaceTree`
(`📌️ChromePanels/🟦️.tsx:1295-1374`). The wgpu shell's only source is the bridges it booted with, so
every row reports `loaded` and the install-from-URL/file section is omitted. See gap 2.

---

## 3. `expanded_command_id`'s reader — the staged command form

`build_command_category_ui` (13939) is the gap W1c recorded (`📓️w1c` §7 item 1: *"`expanded_command_id`
has no reader yet"*). It is React's `buildCommandCategoryTree` (`🛠️ShellHelpers/🟦️.tsx:4612-4679`):

* the **form section first** (`command.category.<id>.form`), so a bottom anchor's reversed section order
  paints it above the list exactly as React's does;
* one control per declared argument through `staged_command_arg_row` (4175), whose control-kind switch
  is React's `renderStagedArgControl` (`:3839`): `Select` → `Select`, `Toggle` → `Toggle`,
  `Slider` → `Slider`, `Number` → numeric `Input`, everything else → text `Input`;
* `command-<key>-execute` (disabled while a required argument is unstaged — React's
  `missing.length > 0`) and `command-<key>-reset`, React's own two section actions (Reset, not Cancel);
* React's `autoExpandedSingleton` rule: a category with exactly one arg-carrying command opens its form
  with no expansion write at all;
* the expanded command is filtered OUT of the list section below.

Staging buffers in `ShellState::staged_command_args` (2770) and leaves it only on Execute, which routes
an `Os`-owned command through `apply_os_command` and any other through `dispatch_command` with a
`CommandInvocation`, then clears both the buffer and `expanded_command_id`.

---

## 4. Content-hug height

React's `<Panel>` never sets a `top`+`bottom` pair: `anchorPositionStyle`
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6864-6884`) bonds ONE vertical edge and clamps with
`maxHeight: calc(100% - (var(--spacing-single) * 2))`, so the box is content-sized up to that clamp
(`🖼️Panel/🟦️.tsx:462-468`, which carries no `height` at all).

Three new seams:

1. `ui_wgpu::wgpu::Ui::surface_content_height(window_id)` —
   `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1718`: the solved height of a surface's
   root node, `None` until its layout has been accepted once.
2. `crate::interpreter::retained_content_height(window_id)` —
   `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:436`: the shell's reader for it.
3. `anchor_panel_rect(anchor, size, content_h, column_slots, column_index, body, theme)` (11894) takes
   the measured height; `ShellState::anchor_content_height` (16195) supplies it as
   `tab bar + document + 2·gap`, and `anchor_rect` (16178) passes it.

Geometry now: `height = content.clamp(0, band)`, and the origin bonds the anchor's own edge — a top
anchor grows DOWN from its inset, a bottom anchor grows UP from the body's bottom inset, a middle anchor
centres on its band. Two hugging anchors in one column are each clamped to their own share, so they
still cannot overlap. An unmeasured surface keeps its full band rather than collapsing, which is
byte-identical to the pre-packet behaviour for a surface's first frame.

⚠️ One float subtlety worth keeping: the slack is computed **before** it is added to the origin
(`slot_top + slack`, not `slot_top + band - height`) — the latter loses a float step on a full-band
panel (693.6 added then subtracted) and drifts a thousandth off the declared inset, which is exactly
how `anchor_rects_sit_at_the_react_anchor_insets` caught it.

---

## 5. Drag drop preview, and `trees`

**Preview** (`dock_tab_insert_preview`, 16543; painted at 16460 and 16490): React's
`panelTabInsertPreviewClass` (`🧭️PanelTabBar/🟦️.tsx:385`) is `"w-0.5 self-stretch rounded-full
bg-accent shrink-0"`, so `TAB_INSERT_PREVIEW_WIDTH_PX = 2.0` (shell file 143) painted as a rounded
accent pill the height of the row, before the chip the drop would land in front of (and after the last
chip when the index is past the end). The index rule is React's `computeTabDockDropZone`
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:7110-7128`): the chip half the pointer is over (`fraction >= 0.5` →
after), or the row's length when the pointer is inside the row but past every chip. The dragged tab's
own chip is excluded, React's `excludedIds`.

`finish_dock_tab_drag` (16385) now **drops where the preview pointed** (`dock_tab_drop_index`, 16407)
instead of always appending, so what lands is what was shown.

**`trees`** (`DockTabSkeleton.trees`, 3216; `dock_leaf_tree_unit_id`, 3247;
`dock_tab_skeleton_of`, 3253): React's `panelTabNodeToSkeleton` (`🧭️PanelTabBar/🟦️.tsx:271-274`) writes
a leaf's `PanelTreeUnit` id list and a branch's children. A wgpu leaf hosts exactly ONE retained
document, so its unit list is always the single `"<leafId>.tree"` id React's `singleTreeLeaf` mints —
written now, so a skeleton this renderer saves round-trips through React's own unit resolution instead
of dropping every leaf's units (W1h gap 5). A branch writes none.

---

## 6. Mobile: eight anchors flatten into one panel

Breakpoint is React's `UI_MOBILE_MEDIA_QUERY` = `(max-width: 767px)`
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1648`), read off the **logical** `screen_w` W1g established via the
same `dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX = 767.0` the window dock already uses
(`ShellState::mobile_panel_active`, 6695).

| Symbol | Line | What it is |
|---|---|---|
| `mobile_panel_tabs()` | 6679 | every anchor's ROOT tabs flattened in `PanelAnchor::ALL` order, plus the synthetic `framework.mobile.app` leaf (order 99) when the navbar's example/mode clusters have no room — React's `mobilePanelTabs` (`🏛️ShellHost/🟦️.tsx:9472-9497`), which flattens `defaultDock`, not the override-applied dock |
| `mobile_panel_reconcile_path` | 3228 | the flat twin of `ShellDock::reconcile_path` |
| `render_mobile_panel_step` | 16688 | the panel itself: NOT a floating box but an in-flow pane filling the body, React's `LayoutMobilePanel` (`📐️Layout/🟦️.tsx:47-83`) |
| `paint_mobile_tab_bar` | 16757 | its single tab row, registering `shell.panel.tab.mobile.<tabId>` |
| `ShellChromeFramePhase::Panels if mobile` | 15949 | the guard: below the breakpoint the eight anchors do not paint at all |

State: `mobile_panel_visible` / `mobile_panel_path` (2780-2783), React's own pair. The navbar drops all
three anchor tab rows on mobile and paints one `ui.mobilePanel.toggle` chip instead (16880-16903),
React's `navbarItems` gate.

---

## 7. Panel content-projection escape hatch, and the chat transcript

### 7.1 The hatch (shell audit recommendation 7)

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:54` gained
`LayoutNodeKind::HostContent { height }`, mapped at `:262` to exactly the host's band, clipped, filling
the parent's cross axis. `📌️mounted_layout/🦀️.rs:474` classifies `UiNode::ExternalSlot` into it, with
the band read off the slot's own `params_json` (`{"hostContentHeight": <px>}`) by `host_content_height`
(`:37`).

This is the minimal correct shape of React's `emptyState` hatch (`🌳️Tree/🟦️.tsx:1042`, used at
`📌️ChromePanels/🟦️.tsx:1376-1391`): **a slot whose pixels the engine does not author**. It used to fall
into `LayoutNodeKind::Leaf`, which measures from arena children — a slot has none, so the band collapsed
to nothing and host-provided content had no box at all. The reservation costs one node and no
measurement, so no other panel's node credit moves.

### 7.2 The chat transcript (W1j gap 1, `P-agent-chat-transcript`)

`build_agent_chat_ui` (6407) + `send_agent_chat_draft` (6383) + `seed_agent_chat_messages` (4122) +
`AgentChatMessage` (4108). Published as the `framework.chat` leaf's retained document through
`panel_ui_records`, beneath W1j's header.

The decisive finding: React's transcript is **local component state**, not bridge data.
`BasicChatPanel` (`🌳️Tree/🟦️.tsx:3982-4085`) seeds two assistant lines, appends the user's line and a
local echo quoting a ≤72-character preview on send, and never reads `useAgentBridge` — the bridge
supplies only the header's status dot. So the wgpu twin is exactly reproducible with no transport, which
is what it now is: two seeded lines, a per-send user line + echo, a draft box, Clear, and Send disabled
on a blank draft.

⚠️ The transcript is assembled by the shell as ordinary `UiNode`s rather than through the `HostContent`
slot, because everything `BasicChatPanel` paints *is* expressible in that vocabulary. The hatch is the
generic mechanism for content that is not (a canvas, a live scene inside a panel); it ships with its own
tests rather than with a contrived first consumer.

---

## 8. Tests

### Shell (`🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs`, +14 tests)

| Test | Asserts |
|---|---|
| `the_settings_branch_nests_the_five_framework_leaves_after_the_apps_own` | bottom-right's three roots, the branch's app-first child order, and React's `+ appSettingsTabs.length` re-ordering |
| `every_shell_owned_leaf_projects_into_retained_records` | the assembler's CLOSURE property — every dockable shell leaf has a body and every body is dockable — plus parent-first dense record numbering |
| `the_panel_assembler_refuses_an_unprojectable_node` | a refusal naming the variant, never a silent drop |
| `a_keybinding_override_remaps_a_shell_chord_and_frees_its_default` | the override dispatches AND reserves at its new spelling; the default stops being the shell's; the default table is untouched |
| `a_blank_keybinding_override_keeps_the_declared_default` | React's chordless-override gate |
| `keybinding_rows_flag_the_earlier_owner_of_a_shared_chord` | the asymmetric last-writer-wins conflict marker, and control-id row order |
| `an_armed_capture_writes_the_next_chord_as_an_override` | `ctrl+alt+shift+arrowup` spelling, one-gesture disarm, Escape cancels without writing |
| `the_keybindings_leaf_paints_one_row_per_shortcut` | one row per `SHELL_SHORTCUT_ROWS` entry, addressable ids, capture+reset on an overridden row |
| `a_panel_hugs_its_measured_content_up_to_the_react_clamp` | per-anchor bonded edge (top grows down, bottom grows up, middle centres), the `100% − 2·spacing` clamp, and the unmeasured fallback |
| `two_hugging_anchors_in_one_column_never_overlap` | the per-share clamp |
| `the_mobile_panel_flattens_every_anchor_in_anchor_order` | the fixture's flattened order, and that 767px itself is mobile |
| `the_mobile_panel_path_drills_a_branch_to_its_first_leaf` | mobile path reconciliation and the vanished-path fallback |
| `the_command_dock_opens_the_expanded_commands_staged_form` | **`expanded_command_id`'s reader**: form section FIRST, one control per arg, Execute+Reset, expanded command not also listed |
| `execute_is_disabled_until_every_required_argument_is_staged` | React's `missing.length > 0` gate |
| `the_chat_panel_paints_a_transcript_and_echoes_each_send` | two seeded lines, blank-draft no-op, trimmed user line + echo, draft cleared, draft box and Send in the published body |
| `a_persisted_skeleton_carries_each_leafs_tree_unit` | every leaf writes its single `<leafId>.tree` unit, every branch writes none |

Shared fixture `🐚️Shell/🧫️fixtures/🧭️default-dock/🔣️.json` **extended** (W1h's own fixture): `expected`
now carries the Settings branch with all six children, the Marketplace leaf, the History sibling and
every leaf's `trees`; new `settingsBranch` and `mobilePanelTabs` blocks. Both renderers can assert
against it.

### ui (`🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs`, +2 tests)

`a_host_content_slot_reserves_the_band_its_host_declared` (the declared band is honoured exactly and the
slot fills its parent's width) and
`a_host_content_slot_without_a_declared_height_falls_back_and_clamps` (no JSON / no key / absurd number
/ negative number).

---

## 9. Verification — what was actually run

Logs under `🗑️generated/w2c-*.txt`. Every command was a single foreground `-j 4` invocation with
`CARGO_INCREMENTAL=0` (the first incremental run died with a rustc ICE in
`evaluate_obligation`, `w2c-check-1.txt`).

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4 --keep-going` | **exit 0**, 0 errors | `w2c-ui-check-1.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4 --keep-going` | **exit 0**, 0 errors, 54 warnings, 54s | `w2c-check-3.txt` |
| `cargo check … --target wasm32-unknown-unknown --lib -j 4 --keep-going` | **exit 0**, 0 errors, 58.5s | `w2c-wasm-check.txt` |
| `cargo test … --lib -- panel_anchor_model_tests` | **38 passed, 1 failed** (the float drift above, then fixed) | `w2c-tests-3.txt` |
| `cargo test … --lib -- panel_anchor_model_tests command_registry_tests shell_chrome_parity shell_shortcuts_palette tutorial_tests` | **128 passed, 2 failed** — both pre-existing, see below | `w2c-tests-4.txt` |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- host_content` | **2 passed, 0 failed** | `w2c-ui-tests-2.txt` |
| `cargo test … --lib -- panel_anchor_model_tests agent_overlays_tests window_measures_tests shell_shortcuts_palette_tests` | **73 passed, 0 failed**, then hung — see below | `w2c-tests-5.txt` |

**Warnings are the proof the type-check actually reached this code** (memory: zero errors is meaningless
if expansion aborted): both checks emit warnings naming the shell file's own lines, and the final
warning set for the shell file / `mounted_layout` / `flex` contains **no new entry from this packet** —
every remaining one (`chord_carries_accelerator_v1`, `reserved_shell_chords_v1`, `mode_layout_stacks_v1`,
`dock_seed_active_window_id_v1`, `WindowScopeStackV1`, `has_display_tabs`,
`shell_directory_request_base_url`, `contributions_reachability_json`, `close_step`) predates it. The
four warnings this packet did introduce (`MeasureSelectItem` unused, and `shell_shortcut_for` /
`is_reserved_shell_chord` / `build_command_panel_ui` becoming test-only) were closed by dropping the
import and `#[cfg(test)]`-gating the three test-only readers.

### The three failures that are not this packet's

1. `shell_chrome_parity_tests::the_overlay_row_steps_clear_of_an_open_floating_panel` — fails at
   `flush_controls[0]` with `151.2` vs `7.2`, **the branch that passes `panels: &[]`**, i.e. with no
   panel involved at all. Byte-identical to the failure `📓️w1h-…` §8 already recorded and attributed to
   `world3d_status_pill_for` emitting a pill where that branch assumes none. This packet's geometry
   change only moves a rect when a panel exists.
2. `command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss`
   — `DirectoryClient`/bootstrap-epoch machinery in the 🛂️SpaceAdministration lane, also already recorded
   by W1h.
3. `panel_anchor_model_tests::host_panel_action_is_claimed_before_guest_and_preserves_the_session_roster_and_home_projection`
   — passes in isolation (`w2c-tests-3.txt`, 0.03s) and **hangs past 60s** when run alongside
   `agent_overlays_tests`/`window_measures_tests` (`w2c-tests-5.txt`). Same DirectoryHome lane as (2);
   the run was terminated rather than waited out.

A whole-suite `cargo test --lib` cannot complete at all right now: a peer's
`async_boundary_tests::renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length`
aborts the test binary with a non-unwinding panic in a destructor (SIGABRT), which is why the evidence
above is per-module rather than one full run (`w2c-tests-full.txt`).

### Not verified

No runtime or visual confirmation: no wgpu shell was booted, so every claim about painted chrome rests
on the unit tests listed. The mobile panel's paint step, the insert-preview pill and the navbar mobile
toggle in particular are **type-checked and unit-tested at the model level only** — their pixels have
never been observed.

---

## 10. Remaining gaps

1. **Tool leaves have no measures.** React's body is `windowMeasuresToTreeItems(toolMeasuresByToolId[id])`
   — the program's own published tool measures. The wgpu program bridge has no `tool_measures` reader,
   so `build_tool_panel_ui` renders the options section and the tool's armed state and nothing else,
   which is exactly what React renders with no measures in hand. Closing it is a program-bridge packet
   (a `tool_measures_section` fetch beside `window_measures_section`), not a panel one.
2. **Marketplace has no installer lane.** React's host carries
   `installPlugin`/`uninstallPlugin`/`reloadPlugin`/`installExtensionFromUrl`/`installExtensionFromFile`/
   `uninstallExtension`/`setExtensionEnabled` plus a plugin *source* registry and an extension ledger.
   wgpu has only the bridges it booted with, so the leaf lists them and offers no actions. Also: React's
   icon for this leaf is `shellTabIcon("store")` and neither renderer's icon set has a `store`, so both
   fall back to `circle-dot` — the wgpu leaf uses `circle-dot` deliberately.
3. **Conflicts has no projection.** React builds its rows from `selectOpenConflicts(shellState)` over
   🐚️Shell's store, with `ConflictDiffPreview` behind a `🔺️DiffViewHost`. The wgpu shell has no conflict
   projection reader, so the leaf resolves to React's own unavailable row. It exists, is addressable, and
   never paints blank — but it never paints a conflict either.
4. **No driver EDITOR.** React's General tab has a third section: seven per-axis driver selects plus
   save/delete. `set_driver_field`/`save_driver`/`delete_driver` have no `"framework"` dispatch arm on
   this side, so the driver row stays a plain selector — the same scoping `build_settings_theme_ui`
   already applies to React's multi-hundred-token theme editor.
5. **Theme editor still scoped.** `build_settings_theme_ui` remains select/reset/delete; React's eight
   further sections (colors, spacing, fonts, strokes, radii, opacities, metrics, per-appearance paints)
   are unported, as `w3-prefs-i18n-themes` already scoped them.
6. **`framework.settings.general`'s icon diverges deliberately.** React passes
   `shellTabIcon("framework.settings.general")`, which is not an `IconName` and falls back to
   `circle-dot`; wgpu uses `settings-2`, which `panel_tab_icon_id` already returned for that id. Flagged
   rather than "fixed" in either direction.
7. **`mobile_panel_tabs`' synthetic app leaf has no body.** React's `framework.mobile.app` leaf hosts the
   navbar's example/mode/role cluster *elements*; the wgpu leaf is emitted and selectable but publishes
   no document, because those clusters are navbar paint steps here, not reusable nodes.
8. **Default-app pins are in-memory.** `default_app_pins` is not persisted: React reads and writes
   `OpeningPreferences` (`os.config.opening`). The `Select` works for the session and forgets on reload.
9. **The `HostContent` hatch has no paint-side host callback.** The layout reserves the band and the
   solved rect is available, but nothing in `paint` dispatches to a host painter for that node yet — a
   host with genuinely non-`UiNode` content still has to paint over the panel itself (which is what the
   chat panel's shell-assembled body sidesteps entirely). The layout half is what recommendation 7
   named and what is tested; the paint dispatch is a follow-up.
10. **Keybinding capture has no `aria-keyshortcuts` twin** and no inline hotkey badge
    (`⌨️control-hotkey-presentation`'s `ControlHotkeyBadge`); wgpu still folds hotkey text into tooltips
    only, as §6.1 recorded.
