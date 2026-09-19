# 🎨️♿️ W15d — settings-panel depth and control accessibility parity

Packet W15d of 26/09/17/WGPU-RENDERER-REACT-PARITY. Source + tests only: **no wasm build, no
activation** — W14d owns live verification. Audit input: `📓️audit-w14-shell-residual.md` items 7
(§B8), 8 (§B2 + §B3) and 10 (§C4 + §B14) plus its §E keybindings table.

All line numbers are as of this write-up; the shell file is edited concurrently by W14c/W15a/W15c, so
anchor by symbol name rather than by line.

---

## 0. What changed, in one paragraph

The wgpu shell had no theme DOCUMENT at all: its custom themes were a five-slot
`CustomChromeTheme { base, light{background,panel,navbar,text,accent}, dark{…} }` override record that
React cannot read, and its theme leaf offered select/reset/delete. This packet gives the target the
same `UiTheme` document React edits (parsed from the authored `🖱️ui/🎨️styling/🔣️.json`, which is what
React's `STYLING_SEMIO_THEME` is generated from), all eight editor sections over it with React's own
control ids, a runtime re-tokenisation of `Theme` from that document (including the formula-derived
level ramp), the merge-policy selector, theme export/import through the existing download and
file-open doors, and the first PRODUCTION accessible-name publication for shell chrome.

---

## 1. Item 7 — the theme editor's eight unported sections (audit §B8, P1)

### React source

- `📌️ChromePanels/🟦️.tsx:693-828` `buildSettingsThemeTree` — the selector section plus
  `colorItems` / `spacingItems` / `fontItems` / `strokeItems` / `radiusItems` / `opacityItems` /
  `metricSections` / `appearanceItems`, each row id `framework.settings.theme.<section>.<key>`.
- `📌️ChromePanels/🟦️.tsx:610-676` `themeColorInputRow` / `themeTextInputRow` / `themeNumberInputRow` /
  `buildThemeAppearanceGroupItems` (the paint + `<paint>.alpha` pair).
- `🏛️ShellHost/🟦️.tsx:8230-8301` `draftThemePatch` and the eight mutators.
- `🖱️ui/🎨️styling/🌓️theme/🟦️.ts:63` `UiTheme`, `:109` `resolveThemePaint`, `:278` `parseUiTheme`,
  `:323` `serializeUiTheme`; `🖱️ui/🎨️styling/📽️projection/🟦️.ts:129` `injectLevelPaints`
  (`bg(k) = oklabMix(base, foreground, k · shadeStepPercent/100)`).

### wgpu change

All in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
unless stated.

| What | Where |
|---|---|
| `//#region 🎨️ThemeDocumentModel` — `ThemeDocument` (Rust twin of `UiTheme`, `BTreeMap` throughout so rows enumerate in React's `Object.keys().sort()` order), `ThemeNumber` (`number \| number[]`), `ThemePaintRef`, `SHELL_THEME_DOCUMENT_JSON` (`include_str!` of `🖱️ui/🎨️styling/🔣️.json`), `shell_theme_document_base()` | `🦀️.rs:24949-25065` |
| `theme_resolve_paint` — port of `resolveThemePaint` incl. the `"transparent"` mix alpha rule | `🦀️.rs:25092-25108` |
| `theme_from_document(document, dark) -> Theme` — the runtime twin of the ui crate's compile-time `from_chrome`: 13 chrome paints → their `Theme` fields, the six `level*` surfaces recomputed through `ui_styling::color::oklab_mix`, outcome (4) and diagram (10) groups, `radii.chrome`, `strokes.chromeBorder*`, `opacities.chromeFocusRingAlpha` | `🦀️.rs:25120-25200` |
| `apply_theme_document_metrics` — every `Theme` scalar that is a `metrics.chrome/dom/typography` entry | `🦀️.rs:25205-25250` |
| `//#region 🎨️ThemeEditorPanel` — `SHELL_THEME_EDITOR_SECTIONS` (React's eight, in React's order), `SHELL_THEME_APPEARANCE_GROUPS` (React's four), `theme_editor_row`, `theme_editor_section_toggle`, `ShellState::theme_document` / `theme_draft_patch` / `clear_theme_draft` / `theme_editor_page_rows` / `build_settings_theme_editor_sections`, `theme_paint_editor_values` | `🦀️.rs:25253-25470` |
| `//#region 🎨️ThemeDocumentDoors` — `set_active_theme_document`, `save_custom_theme_document`, `document_theme_for_ids` (cached on `(theme id, appearance, draft generation)`), `custom_theme_id_for_label`, `merge_committed_args` | `🦀️.rs:25473-25560` |
| `//#region ⚖️MergePolicyAndThemeCommits` — `ShellState::apply_theme_editor_commit` (the eight verbs) | `🦀️.rs:25600-25700` |
| `build_settings_theme_ui` now appends the eight sections (and its doc comment no longer claims the editor is out of scope) | `🦀️.rs:6690-6790` |
| `ShellState` fields `theme_draft`, `theme_editor_open_section`, `theme_editor_page`, `theme_save_label` | `🦀️.rs:2916-2932` |
| `ChromePrefsState` fields `theme_document_draft`, `theme_document_generation`, `theme_document_resolved` + `Default`/`project_chrome_prefs` initialisers | `🦀️.rs:~22630` |
| `resolve_theme_for_ids` consults the document first | `🦀️.rs:~22870` |
| Dispatch arms `setThemeColor` … `setThemeAppearancePaint`, `toggleThemeSection`, `stepThemePage`, `setThemeSaveLabel`, `saveTheme`; `setThemeId`/`resetThemeId` now drop the draft like React's do | `🦀️.rs:8490-8620` |

Three things worth calling out.

1. **The section header is a `Button`, not a `Section`.** A retained `Section` container registers no
   hit on this target (the audit's own §F3), so a React `TreeDataSection` header has no twin that can
   be pressed. The header carries React's *section id* (`framework.settings.theme.colors`) and its
   intent ("open this section"), which is the closest honest mapping; §F3's fix direction says the
   same thing.
2. **The open section is PAGED (16 rows/page).** This is the one deliberate divergence from React and
   it is a ceiling, not a preference: `ui_contract::UI_DOCUMENT_NODES` is **128** records and a
   labelled row costs two (`Field` + control), while the document is ~450 rows (43 colors, 67 strokes,
   ~150 metrics, 152 appearance paints…). The pager adds two wgpu-only controls,
   `framework.settings.theme.page.previous` / `.next`, and a page counter row. A law publishes the
   leaf with each section open and asserts the record count stays under the ceiling.
3. **A retained commit used to DROP the row's authored args.** `commit_focused_input` and the select
   `.item.` path both dispatched `{ value }` alone. That is why the theme editor could not have worked
   with one verb per section — and it is also a **pre-existing bug I found in passing**: the Default
   Apps leaf authors `{ row: "<dialect>/<role>" }` on its `Select` (`settings_select_row_with`) and its
   own `setDefaultApp` handler reads `args.row`, which therefore never arrived, so every pin wrote the
   empty key. `merge_committed_args` merges the committed value INTO the authored object (keys sorted
   ascending) at both sites; `setDefaultApp` is fixed by that alone.

### Live re-tokenisation

`resolve_theme_for_ids` (the one call site in `frame()`) now answers `document_theme_for_ids` first, so
a draft edit repaints on the next frame. Editing `colors.primary` moves every paint that references
that token (`chrome.accent` is `{token: "primary"}`); editing `chrome.base` moves the background AND —
through the recomputed oklab ramp — the navbar, pane, panel, dialog and menu surfaces; editing
`metrics.chrome.navbarHeightUiSpacing` re-measures the chrome. Resolution is cached on
`(theme id, appearance, draft generation)` so the frame loop never walks the document twice.

A theme SAVED here is written as React's canonical `serializeUiTheme` JSON into
`customThemes[id].config`, the same slot React writes, through the existing preference-persistence
lane (`canonical_custom_themes` diffing). The legacy five-slot `CustomChromeTheme` record still
resolves — `ThemeDocument::is_document` tells the two apart — so nothing a previous session saved
breaks.

### Laws

`🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs` (registered at the end of the wgpu
shell file as `mod theme_editor_and_accessibility_tests`):

1. `the_embedded_theme_document_carries_every_section_react_enumerates`
2. `the_colors_section_mints_reacts_row_ids_in_ascending_key_order`
3. `the_spacing_and_fonts_sections_mint_reacts_row_ids`
4. `the_number_sections_mint_reacts_row_ids_and_number_text` (strokes/radii/opacities + the
   `number | number[]` text and parse rules)
5. `the_metrics_section_is_two_levels_and_names_both_coordinates`
6. `the_appearance_section_offers_reacts_paint_and_alpha_rows`
7. `editing_the_document_re_tokenises_the_painted_theme` (token → paint, `chrome.base` → level ramp,
   metric → measurement)
8. `every_open_theme_section_publishes_within_the_retained_document_ceiling`
9. `an_editor_commit_keeps_the_rows_authored_coordinates`

---

## 2. Item 8 — General/Theme settings completeness (audit §B2, §B3)

### React source

- merge policy: `📌️ChromePanels/🟦️.tsx:339` `MERGE_POLICY_OPTIONS = ["LaissezFaire","Normal","Vigilant"]`,
  `:467-486` the `framework.settings.mergePolicy` `Select`.
- export/import: `📌️ChromePanels/🟦️.tsx:801,806`; `🏛️ShellHost/🟦️.tsx:8338-8347`
  `exportTheme` = `downloadMediaExport(`${id}.theme.dsl`, "text/plain", serializeUiTheme(theme))`,
  `importTheme` = `requestFileOpen(".theme.dsl,.dsl,text/plain")` → `parseUiTheme` → `saveTheme`.

### wgpu change

- `framework.settings.mergePolicy` row added to `build_settings_general_ui` (`🦀️.rs:~6870`), options
  from `SHELL_MERGE_POLICY_OPTIONS`, dispatching `setMergePolicy`; `ShellState::merge_policy` defaults
  to `Normal` (`protocol::MergePolicy::default()`). The value is kept as the variant NAME string
  rather than importing `protocol` into this crate for three labels.
- `framework.settings.theme.export` → `exportTheme` arm → `download_media_export("<id>.theme.dsl",
  "text/plain", &serde_json::to_string_pretty(&document), None)`. This is the **same
  `DownloadMediaExport` lane** every plugin export uses on this target (the coordinator's
  "Draw PDF Export Path" note): in the browser the bytes go through `host_io_call`'s
  `download-media-export` op on the page, because the frame Worker owns no `document`.
- `framework.settings.theme.import` → `importTheme` arm. **The file-open door does exist on wgpu**
  (`request_file_open`, both a real browser picker via `host_io_call` and a native dialog), so import
  is wired, not deferred. The picker is gated on USER ACTIVATION, so — exactly like
  `pending_file_opens` — the request parks in `ShellState::pending_theme_import` and
  `drain_gesture_bound_work` opens it inside the gesture; the picked text comes back as an ordinary
  `applyImportedTheme` action, which parses it and saves it under `custom.<slug>`. The native branch
  goes through `submit_shell_io_future` and produces the same action.
- `framework.settings.theme.saveLabel` + `framework.settings.theme.save` were added with them: without
  a save the editor could mutate a draft nothing could keep.

**Not ported** (and not in this packet's brief): §B7, the seven-axis DRIVER editor. The
`build_settings_general_ui` doc comment still describes that gap accurately.

### Laws

10. `the_general_leaf_offers_reacts_merge_policy_selector`
11. `the_theme_leaf_offers_reacts_export_import_and_save_controls` (control ids + the
    serialize/parse round trip + React's save slug)

---

## 3. Item 10 — accessible names, inline hotkey badges, keybinding drift (audit §C4, §B14, §E)

### React source

- `🖱️ui/🔨️modules/⌨️control-hotkey-presentation/🟦️.tsx:32` `ControlHotkeyBadge` — rendered by
  `ButtonGroup` (`:117`), `Toggle` (`:201`), `ToggleGroup` (`:229`) for any control with an id and
  inline text while the driver's `hotkeys` axis is `"inline"` (the default driver's).
- `🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:163` `SHELL_KEYBINDINGS`,
  `composeControlKeybindings` (shell defaults + app action bindings + user overrides),
  `useControlHotkey`.
- The names themselves: React writes `aria-label`/`aria-keyshortcuts` onto the element it renders.

### wgpu change

- **§C4 accessible names.** `//#region ♿️ChromeAccessibleNames` (`🦀️.rs:~25760`): the two central
  chrome item painters (`render_retained_chrome_group_item_step` and `render_chrome_group`, the ones
  every navbar chip, footer pill, panel tab, pane chip and surface control goes through) now call
  `note_chrome_control_name(item.control_id, item.label)` beside their `register_hit`, so the
  published NAME is the text that was actually painted rather than a second table that could drift.
  `publish_retained_hit_registry` promotes those names with the hit registry in one step (the same
  double-buffer discipline `retained_hit_windows` follows) and hands
  `ShellState::chrome_accessibility_nodes` to a new
  `crate::interpreter::note_chrome_accessibility` door
  (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`, beside `note_chrome_hit_registry`).
  `build_accessibility_dump` announces them as an extra `shell.chrome` window, so
  `🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`'s existing `accessibilityMirror` gives every chrome control
  a real ARIA element **with no TypeScript change at all** — the mirror is already generic over
  `AccessibilityProjectionNode`.
  Crucially this door is NOT diagnostics-gated, unlike `CHROME_LEDGER` beside it: an assistive
  technology must read the chrome whether or not `SEMIO_RUNTIME_DIAGNOSTICS` is armed. Bounded at 512
  rows. Controls owned by a retained body are skipped, because the document projection already
  announces those.
- **§B14 hotkey badges + `aria-keyshortcuts`.** `ShellState::chrome_control_label(control_id, label)`
  appends this session's own remapped chord to any bound control's label and is idempotent (the role
  chips already appended their own). It is applied in `render_navbar_cluster_step` and the surface
  overlay control step — BEFORE `retained_chrome_group_item_width` measures, or the chip would lay out
  narrower than the text it paints (the W3b failure mode). The `shortcut` field of every published
  chrome accessibility node is the `aria-keyshortcuts` twin the audit asked for, and it covers every
  control whose id carries a chord, not just those two.
  Scope note: React badges a control whose DOM id is a binding id. The panel-anchor chips' ids are
  `framework.panelTab.*`, not `ui.shell.panelAnchor.*`, so React does not badge them either and
  neither does this.
- **§E keybinding drift — both rows were already fixed, and are now pinned.** §E2 and §E3 were flagged
  "not independently re-verified". They are verified here: `SHELL_SHORTCUT_ROWS` already carries
  `("os.toggleFullscreen", "mod+shift+f")`, matching React's `SHELL_KEYBINDINGS`, and `mod+y` already
  resolves to `ShellEditVerb::Redo` beside `mod+shift+z` (W10b). No change was needed; a law now
  fails if either drifts back.

### Laws

12. `chrome_controls_publish_accessible_names_and_shortcuts`
13. `a_bound_chrome_control_carries_its_chord_inline_exactly_once`
14. `the_flagged_keybinding_rows_match_reacts_registry` (§E2 + §E3)

---

## 4. Gates

PENDING — see §6.

## 5. What I did NOT run

- **No wasm build, no activation, no live boot, no probe.** W14d owns puzzle3d activation; nothing
  here was verified on screen. Every claim above is a source or unit-test claim.
- No TypeScript gate: the packet changed **no** TypeScript. The accessibility mirror needed none.
- No `ui-rs` / `infinite` suites: nothing in those crates changed.
- I did not port §B7 (the driver editor), §B1 (Display panel bodies — W14c/W15a's lane), §B4/§B5/§B6.

## 6. Hand-offs for W14d

1. **Live-verify the theme editor on 6213.** Open `framework.settings.theme`, press
   `framework.settings.theme.colors`, edit `primary` to a loud hex, blur — the accent of every chrome
   chip must move within one frame. Then press `framework.settings.theme.appearances` → `light` →
   `chrome`, edit `base`, and confirm the navbar/panel/menu surfaces move together (the oklab ramp),
   not just the page background.
2. **The pager is wgpu-only.** A parity probe comparing control rosters will see
   `framework.settings.theme.page.previous`/`.next` and a counter row that React has no twin for, and
   will see only 16 rows of an open section. That is the 128-record ceiling, not a gap.
3. **Export writes a real file.** `framework.settings.theme.export` should present
   `semio.theme.dsl` through the page's download door; if nothing appears, the fault is in
   `host_io_call`'s `download-media-export` op, not in this packet.
4. **Import needs a gesture.** The picker opens from `drain_gesture_bound_work`; if it never appears,
   check that the press reached `drain_gesture_bound_work` (the same door `Import Document…` uses).
5. **Accessible names are now live in the DOM.** `document.getElementById("…accessibility mirror…")`
   should carry a `[data-window="shell.chrome"]` element per chrome control with `aria-label` and,
   for bound ones, `aria-keyshortcuts`. This is a new, cheap parity signal for the probe: React's
   chrome controls can be compared name-for-name against it.
6. **`setDefaultApp` should now pin correctly** (the args-merge fix). Worth one probe step, since it
   silently wrote the empty key before.
7. **Float canonical form.** `serde_json` writes `3.0` where JavaScript writes `3` for an integral
   theme number. Round-tripping is unaffected, but a byte-for-byte JSON oracle against React's
   `serializeUiTheme` output would differ (the ticket's own "fixture float canonical form" trap).
