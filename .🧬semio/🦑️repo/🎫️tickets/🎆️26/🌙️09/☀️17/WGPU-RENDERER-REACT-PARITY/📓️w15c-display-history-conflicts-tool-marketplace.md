# 🖥️ W15c — Display / History check-in / Conflicts / Tool measures + Marketplace actions

Packet W15c of ticket 26/09/17 WGPU-RENDERER-REACT-PARITY. Closes items **1 (B1, P0)**, **4 (C2)**,
**5 (B6)** and **6 (B4 + B5)** of `📓️audit-w14-shell-residual.md`. Source + tests only — no wasm
build, no activation, no live verification (W14d owns that). All paths absolute under
`/Users/ueli/Documents/semio`.

Files touched:

| File | What |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | the four bodies, their state, their dispatch arms, the router |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | the two missing readers (tool measures, conflicts) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | the shared history tree's `#s-checkin` row |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | that row's contract assertions |
| `🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs` | **new** — six laws, one per body |
| `🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` | `host_test_shell` → `pub(super)` (fixture reuse only) |

---

## 1. Display panel bodies (`framework.display.windows` / `framework.display.layout`) — audit B1, **P0**

**React source.** `buildDisplayWindowsTree` (`📌️ChromePanels/🟦️.tsx:157-186`),
`buildDisplayLayoutTree` (`:190-249`), `groupNamedLayoutsToTreeItems` (`:85-127`),
`createFrameworkDisplayPanelTabs` (`:243-275`), `createWorldProjectionTemplates`
(`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3097-3145`), and `DisplayHostApi`'s implementation
(`📌️ChromePanels/🟦️.tsx:1138-1156`).

**The gap.** Both tabs are real dock leaves (`framework_display_tabs`, shell file), but
`shell_owned_panel_leaves` never listed them and `publish_shell_panel_document`'s match had no arm, so
selecting either published nothing — `Ok(None)`, a tab with no body.

**wgpu change** (all in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`):

| Symbol | What it is |
|---|---|
| `ShellState::build_display_windows_ui` | one `UiTreeSectionNode` per window kind (`framework.display.windows.<kind>`), its plain `…​.kind` leaf, and for a `world-3d` surface the whole projection taxonomy |
| `ShellState::build_display_layout_ui` | React's `…​.layout.save` section (name box `framework.display.saveLabel` + `framework.display.save`, disabled on a blank name) and `…​.layout.list` section |
| `world_projection_template_rows` | rebuilds React's NESTED id grammar from `WORLD_PROJECTION_TEMPLATES`' own `depth` column — `…​.projection.parallel.axonometric.axonometric-isometric` |
| `named_layout_rows` / `place_named_layout_row` | React's `groupNamedLayoutsToTreeItems`, incl. `framework.display.layout.group.<a/b>` folders and the `framework.display.delete.<id>` action on a user row |
| `display_panel_body` / `display_unavailable_section` | the one-`Tree`-in-a-`Stack` body shape every shell-owned leaf publishes |
| `"framework"` dispatch arms | `setLayoutSaveLabel`, `saveCurrentLayout`, `applyNamedLayout`, `deleteUserLayout`, `openDisplayWindow` |
| `ShellState::current_named_layout` / `apply_named_layout` / `open_display_window` | React's `saveCurrentLayout` (`user-<millis>` id), `applyNamedLayout`, and the press that replaces React's drag |
| `shell_owned_panel_leaves` + `publish_shell_panel_document` | the two leaves are now shell-owned and routed |

**Deliberate divergences** (both recorded in the code's own doc comments):

1. React's Windows rows are drag SOURCES (`COMPOSE_WINDOW_TEMPLATE_MIME`) with **no** click handler.
   This renderer has no panel→dock drag lane, so each row is pressable and opens the window it names
   (`openDisplayWindow`, same right-split placement as `mod+shift+n`). The **ids are React's**, so a
   probe addressing `framework.display.windows.<kind>.kind` finds the same control on both.
2. React pre-reverses each template level to cancel its bottom-anchored `Tree`'s own per-level
   reversal; what it RENDERS is kind-leaf → Parallel → Perspective. This renderer's panel tree does
   not reverse, so that reading order is written directly.

**Law.** `both_display_leaves_are_shell_owned_and_publish_a_body`,
`the_display_windows_leaf_publishes_reacts_kind_and_projection_ids`,
`the_display_layout_leaf_publishes_the_save_box_and_both_layout_rosters`.

## 4. History panel check-in row (`#s-checkin`) — audit C2

**React source.** `frameworkUtilitiesHistoryTab` (`🏛️ShellHost/🟦️.tsx:9016-9109`) — the
`framework.history.checkin` row whose control is the `#s-checkin` button, sibling of
`framework.history.checkpoint`, gated by `canCheckIn(session.app.role)` (removed for a viewer, never
disabled), label suffixed ` (N)` with `uncommittedEditCount`.

**Why it landed in the shared plugin tree, as the audit prescribed.** React's OS shell renders the
history tab ITSELF (`shellRendersPanelTabItself`); the wgpu shell keeps React's DOCK placement but the
BODY still comes from the guest (`ui_history_panel`, `🔌️plugin/🦀️.rs`). The wgpu panel assembler
cannot reproduce that body (windowed command rows, row-action menus), so the row goes where both
renderers already share one source.

**Change** (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `ui_history_panel`):
`FRAMEWORK_CHECKIN_CONTROLLER_ID` (`"framework.checkin"`, the shell-owned twin of `framework.sync`,
already routed by the wgpu shell's `dispatch_action` → `handle_checkin_action`),
`uncommitted_command_count(history)` (the oldest-first fold React's `uncommittedEditCount` applies),
and the row itself between Commit Checkpoint and Create Alternative, `read_only`-gated.

**Deliberate divergence.** React's `#s-checkin` press opens a message dialog whose typed text rides
the checkpoint. That message is SHELL state a plugin assembler cannot see, and the wgpu shell paints
no dialog for `checkin_dialog_draft` (native keyboard routing only). The row therefore dispatches
`framework.checkin`/`submit` directly and the shell falls back to its own `"check-in"` message —
React's own blank-draft fallback. The typed-message half stays open (see §Hand-offs).

**Law.** The contract test's own history assertions
(`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`,
`ui_history_panel_filters_rows_and_gates_the_backwards_action`): six action rows, the check-in row at
index 3, and five rows for a viewer.

## 5. Conflicts leaf projection — audit B6

**React source.** `buildConflictsTree` (`📌️ChromePanels/🟦️.tsx:995-1031`) over
`selectOpenConflicts(shellState)` (`🐚️Shell/🟦️.tsx:1152`), with `conflictKindLabel`/
`conflictMessageText` (`🏛️ShellHost/🟦️.tsx:8063-8072`) and `dispatchResolveConflict`.

**The gap.** `build_settings_conflicts_ui` always returned React's *unavailable* row: "the wgpu shell
has no conflict projection reader yet" (W2c gap 3).

**wgpu change.**

| Symbol | File | What |
|---|---|---|
| `wasm_program_exchange::read_conflicts` / `resolve_conflict` | `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | `AppCommand::ReadConflicts` / `AppCommand::ResolveConflict` + the `AppFrame::Conflicts` decode, same shape as `read_history` |
| `ProgramBridgeEntry::read_conflicts` / `resolve_conflict` | same | the native-backend doors |
| `ShellConflictRow` | `🐚️Shell/…/🦀️.rs` | the target-neutral projection (id, `quarantined`, worst message's code + text) — NOT the wire type, which only exists on the native backend |
| `ShellState::{open_conflicts, selected_conflict_id, conflicts_seeded}` | same | React's `merge.conflicts` + `selectedConflictId` |
| `ShellState::conflict_rows` / `seed_open_conflicts` / `resolve_open_conflict` | same | React's kind/message reduction, its `ReadConflicts` session seed (hung off `refresh_history_snapshot`, the same session/document edge) and its `onResolve` |
| `build_settings_conflicts_ui` | same | the real roster; the unavailable row only when there genuinely is none |
| `"framework"` arms `selectConflict` / `resolveConflict` | same | React's `onSelect` / `onResolve` |

**Deliberate divergences.** (a) React nests Accept/Discard as two inline buttons in the row's single
control slot; a row here carries ONE control, so each verb is its own addressable child row under
React's own `….accept` / `….discard` id, expanded for the selected conflict exactly as React expands
its diff for the selected one. (b) No `ConflictDiffPreview`: React's own `currentDocumentText` is `""`
on this lease too ("no local snapshot"), so neither renderer has a second side to diff.

**Law.** `the_conflicts_leaf_paints_the_live_roster_and_reacts_resolution_pair`.

## 6. Tool measures + Marketplace actions — audit B4 + B5

**React source.** `buildToolTree` + `windowMeasuresToTreeItems` (`🛠️ShellHelpers/🟦️.tsx:4762`,
`:3617-3662`), `buildToolTabs`' `toolMeasuresByToolIdRef` (`:4784`); `buildMarketplaceTree` +
`marketplacePluginItem` (`📌️ChromePanels/🟦️.tsx:1266-1370`).

**The gap.** Both were "program-bridge missing reader" shaped. The guest already publishes its tool
measures on the reserved `UiRefreshSection::Tools` section (`🔌️plugin/🦀️.rs`, the `Tools` arm of
`canonical_section_json`); nothing on this renderer read it.

**wgpu change.**

| Symbol | File | What |
|---|---|---|
| `ProgramBridgeEntry::tool_measures_section` + `tool_measures_from_section` | `🌉️ProgramBridge/…/🦀️.rs` | the TOOL twin of `window_measures_section` / `window_measures_from_section`, target-neutral (it rides `render_with_document`, not `exchange`) |
| `ShellState::tool_measures` + `refresh_tool_measures` | `🐚️Shell/…/🦀️.rs` | React's `toolMeasuresByToolId` ref, refreshed on `UiDirtySection::Tools` beside the window-measures pass |
| `window_measure_tree_rows` / `format_measure_number` | same | React's `windowMeasuresToTreeItems`: row id = the measure's OWN id, control = the measure's own editor (`Group`/`Select`/`Slider`/`Number`/`Toggle`) |
| `build_tool_panel_ui` | same | the headerless `tool.<id>.options` section over those rows |
| `build_marketplace_ui` | same | a plugin row per resident plugin AND per plugin the activation catalogue claims, with React's `….install` / `….reload` / `….uninstall` verbs |
| `ShellState::uninstall_plugin` + `"framework"` arms `installPlugin` / `reloadPlugin` / `uninstallPlugin` | same | the verbs, over the lazy-install lane `ShellState::install_plugin` already owned |

**Deliberate divergences.** (a) Marketplace actions are child rows rather than inline buttons, for the
same one-control reason as the Conflicts pair — React's ids unchanged. (b) React's extension ledger
(`marketplaceExtensionItem`, enable/disable/uninstall) and its install-from-URL/file section are NOT
ported: this target has no extension ledger and no package source, so a row for either would be a
decoration. (c) `canUninstall` is "not the session's own program", which is the only uninstall this
shell can refuse meaningfully.

**Law.** `a_tool_leaf_publishes_its_own_measures_under_their_own_ids`,
`the_marketplace_leaf_offers_reacts_install_reload_and_uninstall_verbs`.
---

## 7. What was NOT run — and what W14d owns

**Not run by this packet, deliberately** (the packet is source + tests only):

* **No wasm build, no activation, no serve, no probe.** No wgpu shell was booted, so every claim about
  painted chrome rests on the unit laws below. In particular the Display leaves' rows, the Conflicts
  pair, the Marketplace verbs and the tool measure editors have **never been observed as pixels**.
* **No `cargo test -p semio-framework-os-kernel`.** The check-in row's own contract assertions live in
  that crate's test target, which is outside this packet's gate set; the row and its test edit are
  type-checked through the renderer's dependency build only. Running
  `cargo test -p <the os-kernel crate> --lib -- ui_history_panel` is the missing proof.
* **No React-side re-verification.** React's OS shell renders its history tab itself
  (`shellRendersPanelTabItself`), so the shared tree's new row does not appear in React's shell; that
  reasoning was read off the source, not observed.

**Hand-offs for W14d (live verification):**

1. **Display → Windows.** Open `bottom-left` → Windows. Expect one section per window kind and, for
   puzzle3d's world pane, `framework.display.windows.<kind>.projection.parallel…` rows. A press should
   split a second instance right of the active stack; a projection row should additionally seed that
   instance's projection template. The row is a PRESS here, not a drag — do not score it as missing
   because the drop never happens.
2. **Display → Layout.** Type a name, press `framework.display.save`, reload: the saved layout must
   come back (it round-trips through `namedLayouts` in `semio.os.config`). Pressing a builtin row must
   move the dock.
3. **Conflicts.** Only the NATIVE program bridge can read the roster (`read_conflicts` rides
   `exchange`, which the browser's JS backend does not have — the same limit `read_history` already
   carries). On a browser serve the leaf will still show the unavailable row; that is expected, not a
   regression. Score this leaf on the native target or not at all.
4. **Tool measures.** Arm a tool whose app publishes `tool_measures`; its leaf must now paint the
   measures' own editors. An app with no tool measures still paints the bare options section, which is
   what React renders with none in hand.
5. **Marketplace.** Rows for every plugin the activation catalogue claims; Install on a non-resident,
   Reload/Uninstall on a resident, Uninstall disabled for the session's own program.
6. **`#s-checkin`.** It is now a row of the History panel (`framework.history.checkin`, button id
   `s-checkin`), NOT a footer chip. Pressing it commits a checkpoint with the message `"check-in"` —
   there is still no message dialog on this renderer.

**Still open after this packet** (unchanged or newly recorded):

* The check-in MESSAGE dialog: `ShellState::checkin_dialog_draft` has keyboard routing
  (`cfg(not(wasm32))`) but **no paint step**, so `framework.checkin`/`open` would open an invisible
  draft. A follow-up owes either a chrome card for it or an inline input on the row.
* Conflicts/history on the BROWSER wgpu target: both need a JS-backend `exchange` door.
* Marketplace extensions and the install-from-URL/file lane (React's `MarketplaceExtensionEntry`,
  `installExtensionFromUrl`/`FromFile`) — no extension ledger and no package source on this target.
* Audit items B2/B3/B7/B8 (merge-policy select, theme export/import, driver editor, theme sections)
  were **not** in this packet's scope and remain open.
