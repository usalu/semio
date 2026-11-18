# 🔎 Round 3 Audit — Native

# Native border audit: shared-border consolidation (read-only)

Scope: Rust UI under `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/` (wgpu targets, flex/layout, shell, TUI, elements, border-presentation, CardStack). C# and Python: the only `.cs` file in the UI module is `.../🖱️ui/🎨️styling/🎨️palette/🔷️.cs`, which I did not inspect. No Python UI code was found in that module. Nothing was built, run or probed. All findings are from reading the code.

Short answer: the wgpu double-seam problem is already handled by the shared-border finalizer. The real gaps are (a) theme gaps between bordered siblings in the kit and generic stacks, (b) a stale kit `Field`, (c) no native CardStack, (d) JSON cases that exist for native but are never consumed by Rust, and (e) several TUI frame junctions that are drawn as `│─` instead of `├─`.

Path shorthand used below: `UI` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui`, `WGPU` = `UI/🎯️targets/🧊️wgpu`, `TUI` = `UI/⌨️tui`.

---

## Findings (ordered by severity)

### F1. Kit `Field` still uses the pre-round-2 label/gap/control model (High)

- `WGPU/🪀️widgets/🦀️.rs:70-75` (measure):
  ```rust
  WidgetNode::Field { label, child, .. } => {
      let label_h = theme.font_size_small;
      let gap = gap_for_token(theme, Some("standard"));
      let (cw, ch) = measure_control(atlas, theme, child);
      (cw.max(atlas.measure_text(label, theme.font_size_small).0), label_h + gap + ch)
  ```
- `WGPU/🪀️widgets/🦀️.rs:188-194` (render): draws the label at `bounds.y + label_h`, then the control at `bounds.y + label_h + gap`.
- **Why:** the round-2 model (`field_chrome_metrics`, `WGPU/📐️flex/🎨️style-mapping-and-chrome-metrics/🦀️.rs:63-80`) makes label, description and error into cells with block padding and one hairline divider each, and puts the control at 69.2 with a total of 132.6. The kit widget has no cells, no dividers, no padding and no error band. Its height disagrees with the retained Field, so a Field rendered through the kit and one rendered through the retained tree will not line up.
- **Fix:** route both measure and render through `field_chrome_metrics` (or a shared helper) and draw the cell dividers with `push_shared_border_edges`. Delete the `gap_for_token(.., "standard")` call. Add a kit-vs-retained height test built from `🧪️tests/🧫️fixtures/📐️section-field-presentation/🔣️.json`.

### F2. Kit containers stack bordered children with `gap_standard` (High)

- `WGPU/🪀️widgets/🦀️.rs:61`: `WidgetNode::Separator => (theme.control_height.max(1.0), 1.0 + theme.gap_standard)`. The hairline gets a theme gap below it.
- `WGPU/🪀️widgets/🦀️.rs:85`, `:108` (measure) and `:217`, `:289` (render):
  ```rust
  height += h + theme.gap_standard;          // Section and Group measure
  y += h + ctx.theme.gap_standard;           // Section and Group render
  ```
- **Why:** children of a Section or Group are controls and fields (bordered). Every sibling pair gets a theme gap between their borders. This is exactly the "gap between bordered siblings" the brief forbids, and it is the kit-side twin of the React `SharedBorderRow`/`Field` vertical stack.
- **Fix:** stack children with zero gap. Move the spacing inside the child cell as padding, or use a shared-border row container. For `Separator`, return height `1.0` and let the neighbouring cell own the seam.

### F3. Generic stacks default to a theme gap (High, systemic)

- `UI/🧮️layout/🦀️.rs:60`: `None | Some("standard") => theme.gap_standard,` (in `gap_for_token`).
- `WGPU/📌️mounted_layout/🧱️job-state-and-admission/🦀️.rs:237`: `gap: gap_for_token(&self.theme, stack.gap.as_deref()),`. A stack with no `gap` token inherits `gap_standard`.
- `WGPU/🐚️shell/🦀️.rs:220` (`build_stack`) and `:184` (`root_stack`): both are `gap: None`, so they inherit the theme gap. `build_stack` is the tab-group column whose children are window-cap buttons (bordered). `root_stack` is the `shell.root` column.
- **Why:** the round-2 fixes only set `Some("none")` on specific stacks (`Tabs` at `WGPU/📑️Tabs/.../🦀️.rs:165`, `PresenceBar` at `WGPU/👥️PresenceBar/.../🦀️.rs:79`, shell axes at `shell/🦀️.rs:206`). Any other authored stack whose children are bordered (button rows, IconSelector rows, cap chips) still gets a gap.
- **Fix:** make the default for bordered children `none`, or add an explicit `shared` / `bordered-children` flag on the stack spec that resolves to zero gap. Set `gap: Some("none")` at `shell/🦀️.rs:220`. Check `shell/🦀️.rs:184` only if the root can hold more than one bordered child.

### F4. Section gap and trailing margin between bordered sections (High, verify)

- `WGPU/📌️mounted_layout/🧱️job-state-and-admission/🦀️.rs:242`: `UiNode::Section(_) => LayoutNodeKind::Section { gap: self.theme.gap_standard, top: 0.0, bottom: 0.0 },`
- `WGPU/📐️flex/🌊️flow-style/🦀️.rs:222`: `LayoutNodeKind::Section { gap, top, bottom } => FlowStyle { gap_main: gap, ... }`. Children of a Section (Fields) are separated by `gap_standard`.
- `WGPU/📐️flex/🎨️style-mapping-and-chrome-metrics/🦀️.rs:13`: `SECTION_TRAILING_MARGIN = ROOT_REM_PX * 2.0` (32 px). It is added at `:49` (`section_chrome_metrics`), and the section fixture carries `"sectionTrailingMargin": 32.0`.
- **Why:** the brief rules out a "page margin" between bordered things. If Section is itself bordered (or its Fields are), both the inter-field gap and the 32 px trailing margin break the rule. Round 2 deliberately kept these numbers, so it is unclear whether Section counts as a bordered surface. I did not check the React `Section` TSX.
- **Fix:** confirm the React Section's border and gap classes first. If the Section is bordered, set the Section gap to 0 and remove the trailing margin (or move it into the cell padding). Update `section-field-presentation/🔣️.json` and its tests together.

### F5. No native CardStack: `bands.rs` has no caller (High)

- `UI/🧱️elements/🃏️CardStack/` contains only `bands.rs` and `bands.test.rs`. There is no `🎯️targets/🧊️wgpu` or `⌨️tui` twin.
- `UI/🎯️targets/🧊️wgpu/🪶️light/🦀️.rs:16`: `pub mod card_bands;`. This is the only reference to `resolve_card_bands` outside its own test (`bands.rs`, `bands.test.rs:10`).
- **Why:** React `CardStack` is gapless by contract (`UI/🧱️elements/🃏️CardStack/🟦️.tsx:86`, and `🟦️.test.tsx:42-45` asserts no gap class). A native card stack falls back to generic stacks (F3), so cards get a `gap_standard`. The ported band decision is never used to position anything. `bands.rs` returns only `mode/lift/cap/footer`; it does not emit the line geometry that `compositor.cases.json` `interlock` expects (for example the line at y=172 spanning 0..100 and the line at y=200 spanning 300..400).
- **Fix:** add a native CardStack layout/paint that calls `resolve_card_bands`, applies `lift` to the next card, and draws the shared band with `push_shared_border_edges`. Port the `interlock` line geometry into a native test. Until then, the note in `round2-native.md` that native has no CardStack should be a tracked gap, not a closed item.

### F6. Shared JSON cases meant for native are not consumed by Rust (Medium)

- `UI/🧱️elements/🗂️WindowChrome/compositor.cases.json` has a `nativeSeams` array (4 cases: "active next RTL", "loading beats active vertical", "introduced beats loading", "nested pressed next"). Its only consumer is the TypeScript test `UI/🔨️modules/📏️border-presentation/segments.test.ts:154`.
- `UI/🔨️modules/📏️border-presentation/segments.test.rs:7` loads only `cases.json` in the same folder. That file has 4 cases with no RTL, no `dir` and no nested-content case.
- `SharedBorderSegment` (`UI/🔨️modules/📏️border-presentation/🦀️.rs`, struct around lines 13-20) has no direction. RTL seams cannot be represented natively, so "active next RTL" has no native counterpart.
- `cellFrames` and `interlock` in the same JSON are web-only in practice. `cellFrames.seams` is consumed at `segments.test.ts:159`. The round-2 note justifies skipping them, but `nativeSeams` does not get the same justification.
- **Fix:** either copy `nativeSeams` into `border-presentation/cases.json` and add a Rust test that mirrors any RTL coordinate before resolving, or point `include_str!` at the compositor file. For the nested-content case, assert the winning colour through the resolver with the nested segment list. Add a `dir` field to the schema.

### F7. TUI frame rules do not join the frame's vertical edges (Medium-High)

- `UI/🧱️elements/🖱️ContextMenu/🎯️targets/⌨️tui/🦀️.rs:159-161`:
  ```rust
  let line = Rect::new(rect.x + 1, y, rect.width - 2, 1);
  if item.separator {
      buf.hline(Pos { x: line.x, y }, line.width, '\u{2500}', border, bg);
  ```
  The frame's `│` sits at `rect.x` and `rect.x + rect.width - 1` (`UI/🧱️elements/💬️Dialog/🎯️targets/⌨️tui/🦀️.rs:73-90`, `paint_frame`). The separator therefore renders as `│──────│` instead of `├──────┤`.
- `UI/🧱️elements/⌨️Command/🎯️targets/⌨️tui/🦀️.rs:176-180`: `let inner = Rect::new(rect.x + 1, rect.y + 1, rect.width - 2, 1);` then `buf.hline(Pos { x: inner.x, y: rect.y + 2 }, inner.width, '\u{2500}', ...)`. Same defect under the palette query row.
- **Why:** the junction table in `TUI/⚙️engine/🖌️paint-and-gutters/🦀️.rs:6-23` already defines `├` (mask 7) and `┤` (mask 13). The overlay painters bypass it.
- **Fix:** add one helper, for example `frame_rule(buf, rect, y, theme, surface)`, which writes `├` at `rect.x`, `─` across, and `┤` at the right edge, and use it from `paint_menu` and `paint_palette`. Overlays are isolated by design, so a local junction is enough here.

### F8. TUI Divider inside a Window meets the window's `│` without a junction (Medium, verify)

- `UI/🧱️elements/➖️Divider/🎯️targets/⌨️tui/🦀️.rs:14`: `buf.hline(Pos { x: rect.x, y: rect.y }, rect.width, '\u{2500}', fg, bg);`
- `TUI/🖥️chrome/🦀️.rs:701-716` (`window_content_padding`) returns `[1, 1, 1, 1]` for compact windows, and for full windows the inset is at least 1. Children therefore start at `window.x + 1`, directly against the outline column `window.x` written by `paint_window` (`UI/🧱️elements/🪟️Window/🎯️targets/⌨️tui/🦀️.rs:284-302`).
- **Why:** a Divider that spans the content width renders `│─────` at the window's edge, with no `├`. `compose_shared_borders` only merges glyphs from `Window`, `Navbar` and `Footer` outlines (`paint-and-gutters/🦀️.rs:88`), not from widgets inside a window.
- **Fix:** have the window painter record divider rows and write `├`/`┤` on its own outline at those rows, or let the Divider extend into the inset column when its parent is a Window. Confirm the layout first; I did not trace the Divider's placement in every window kind.

### F9. TUI table rules stop short of the scroll track (Low)

- `UI/🧱️elements/📊️Table/🎯️targets/⌨️tui/🦀️.rs:227` and `:275`: `buf.hline(Pos { x: content.x, y }, content.width, '\u{2500}', ...)`. `content.width` is reduced by one when `overflow` is true (`:213`), so the rule ends one cell before the scroll column at `rect.x + rect.width - 1` (`:291`). That leaves a gap cell between the rule and the track.
- **Fix:** extend the rule to `rect.width` and let the track cell take the junction glyph, or leave as is if the track is not treated as a bordered element.

### F10. Only Window, Navbar and Footer participate in shared-border composition (Info)

- `TUI/⚙️engine/🖌️paint-and-gutters/🦀️.rs:88`: `let outlined = matches!(c, ChromeState::Window(_) | ChromeState::Navbar(_) | ChromeState::Footer(_));`
- Dialog, ContextMenu and Command draw their own corners through `paint_frame` and are not composed. This is correct for overlays, which must sit above content. It would become a gap if any bordered tile is added later without `begin_borders`. No change needed now.

---

## Verified correct (no action)

- **Border-presentation resolver and cases:** `UI/🔨️modules/📏️border-presentation/🦀️.rs:328-340` and `segments.test.rs:7-23` consume `cases.json`, which covers hover precedence, partial child hover, equal-state tie-break and touching endpoints. Shared-edge ownership is resolved exactly. I did not re-execute the tests.
- **Card band logic:** `bands.rs:33-36` (`touch`, `ABUT = 0.25`, `CARD_BAND_MIN_GAP = 40`) matches the constants the file documents as mirroring `bands.ts`. `bands.test.rs:10` consumes all 10 `bands.cases.json` cases.
- **Field arithmetic:** `field_chrome_metrics` (`style-mapping-and-chrome-metrics/🦀️.rs:63-80`) produces the fixture values 28.2 / 69.2 / 132.6, as the round-2 notes state. The retained Field is correct. Only the kit twin (F1) is stale.
- **wgpu control borders:** `push_control_border` for Button, Select, Toggle and IconSelector can draw coincident edges. Coincident segments are resolved by the shared-border finalizer, so the seam is painted once. `Stepper` (`WGPU/Stepper .../🦀️.rs:66`) uses `push_shared_border_edges` with an explicit `[false, !left, false, left]` mask.
- **Gap-none stacks:** `Tabs` (`WGPU/📑️Tabs/.../🦀️.rs:165`) and `PresenceBar` (`.../🦀️.rs:79`) are correct.
- **TUI tiling:** `shell()` sets the root `shared: true` (`TUI/🖥️chrome/🦀️.rs:734`), and both `layout` axes use shared boundaries (`TUI/📏️layout/🦀️.rs:132` and `:144`). Adjacent windows, the navbar and the footer share one cell. The bit-mask glyph table in `paint-and-gutters` is correct for every glyph I checked.
- **TUI text gaps:** `Navbar` `BAND_GAP` and `Footer` `STATUS_GAP` separate text, not borders. They are acceptable.
- **Corner glyphs on the window top border:** the one-row tab chrome comment at `TUI/🖥️chrome/🦀️.rs:678` (`┌ ◐ dev ✕ │ ✓ Tasks ✕ ──── ⤢ ┐`) is the window's own outline, not an adjacent pair.

---

## Not verified / limits

- Static read only. I did not run cargo, vitest, the TUI tests or the browser probe. F8 and F9 depend on layout placement I did not trace end to end.
- I did not read the TUI window tab-chip code (`TUI/🖥️chrome/🦀️.rs` around 678-800, and `Window/🎯️targets/⌨️tui/🦀️.rs` 150-400) in full.
- I did not read the React `Section` TSX, so F4's bordered status is unconfirmed.
- The C# palette file and the Python modules were located but not inspected for stroke or border code.
- I did not check whether `segments.test.rs` assertions cover all four `cases.json` entries. The round-2 note says it passes, which I took at face value.