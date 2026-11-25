# 🔍 Audit 2: Native UI, OS Shell, TUI and Plugin Chrome against the Frame Laws

Read-only audit. No source file was edited and no git write or build ran. Scope: Rust wgpu UI and its targets, OS shell and dock, TUI, drawing plugins, C#, Go, Swift, Kotlin, C++ and the VS Code extension. Laws are taken from `design.md`, `summary.md` (this ticket) and law 1, 2, 4, 7, 8 and 12 of `UI-LAWS-FOR-CHIPS-HOVER-BANDS-DRAG-TOGGLES-ICONS-AND-LINES/summary.md`.

## Path aliases

| Alias | Expands to |
| --- | --- |
| `UI` | `🧰️framework/🔨️modules/🖱️ui` |
| `UIW` | `UI/🎯️targets/🧊️wgpu` |
| `OSW` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging` |
| `DOCK` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu` |

All paths are relative to `/Users/ueli/Documents/semio`. Line numbers are from `grep -n` on the current worktree.

## Verdicts by area

| Area | Verdict |
| --- | --- |
| Chip titles and content (TUI) | Pass. Titles sit in chip cells on the first or last interior row (`⌨️tui/🖥️chrome`, `Window/🎯️targets/⌨️tui`). No title above a border line. |
| Chip hover (TUI) | Pass. Hover changes the chip fill and the emphasis of existing walls only (`chip_border_role`). No line is added. |
| Gaps between sibling frames (TUI, wgpu CardStack) | Pass. `card_stack_frames` abuts frames on shared edges and paints each shared line once. |
| Rounded corners and shadows | Fail. Several explicit non-zero radii and `push_rounded` calls remain (Lane 3). |
| Chip glass (wgpu) | Partial. Every wgpu chip site uses `theme.chip_glass`, but the content level is hard-coded to `Level::Base` (Lane 1). |
| Navbar and footer bands (wgpu shell) | Fail against law 4. The navbar is an opaque solid, and the main body ends above the footer (Lane 2). |
| Select rows (wgpu, law 12) | Partial. Rows are cells in the sense of fills and hairlines, but hover and row height differ from the web contract (Lane 4). |
| Card stack leftovers | The sliver and gap rule still exists in `bands.rs` (Lane 5). |
| Schema and token drift | Three drifts found (Lanes 6 and 7). |
| Silhouette names | Clean in UI and OS chrome code. Remaining hits are 3D, staffage, map and icon meanings, which are correct. |
| C# | No UI code. The 9 `.cs` files are repo test fixtures, a generated icon set and a palette. No Grasshopper or Rhino UI. |
| Go | `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/` holds CLI and MCP packages only. No Go dashboard TUI. The dashboard is Rust (`🎛️dashboard/🖥️terminal`). |
| Swift, Kotlin, C++, VS Code extension | None found in the repo. |

## Findings by lane

Each lane touches a disjoint set of files, so lanes can run in parallel. The owning package names come from the handoffs of this ticket where they are recorded; entries marked "verify" must be checked with `repo` before use.

### Lane 1: chip glass uses the wrong content level

Problem: `Theme::chip_glass(content)` returns the glass one level above `content` with the 10 % veil (`🎨️styling/🎭️theme/🧩️component/🦀️.rs:458`). The fold math is correct. The call sites pass `Level::Base` whatever surface the chip covers. A chip over a Window or Pane level frame then takes the glass of Window, which is the surface it sits on. This is the bug of law 1 that the ticket fixed on the web.

| File:line | Shape | Verdict | Fix |
| --- | --- | --- | --- |
| `UIW/🖌️paint/🎨️node-painting-with-driver/📝️text-and-structure-nodes/🦀️.rs:311` | Card stack chip glass | Fix: pass the level of the card's content (the web card body level, verify against `windowFrameChipLevel` in `UI/🧱️elements/🃏️CardStack/🟦️.tsx`). | Pass the covered level. |
| `DOCK/🧮️axis-solving-and-stack-rendering/🦀️.rs:247` | Dock group glass | Fix: the group lies on the dock canvas level. | Pass the dock content level. |
| `OSW/🗨️view-overlays/⏪️time-travel/🦀️.rs:319` and `:365` | Time-travel indicator and reprojection chips | Fix: chips over the window pane. | Pass the pane level. |
| `OSW/🗨️view-overlays/🧱️chrome-paint-and-navbar/🦀️.rs:62` | Chrome group item glass | Fix. | Pass the level of the bar it lies in. |

Owning packages: `semio-ui-wgpu-paint` (verify; the crate of `UIW/🖌️paint`), `semio-renderer-wgpu-dock` (`DOCK`, a dock test exists), `semio-renderer-wgpu-shell-view-overlays` (verify). Test: `repo cargo --store=shared --configuration=prototype -- test -p <crate>`. Add a test in `UIW/🖍️draw/🔧️pipeline/🎬️execution/🧪️tests/🔬️unit/🦀️.rs` that asserts each chip glass equals `theme.chip_glass(level)` for the covered level.

### Lane 2: navbar and footer bands are opaque, and the main body stops above the footer

Law 4: navbar and footer are glass one layer above the canvas, and content runs beneath them. The web side now does this. The wgpu shell does not.

| File:line | Shape | Verdict | Fix |
| --- | --- | --- | --- |
| `OSW/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:15` | `push_solid([0, 0, width, navbar_height], theme.navbar)`: opaque navbar band | Fail (law 4). | Push `theme.glass(Level::Panel)` with `data-glass-chip` semantics over the canvas, and begin the glass content at the top edge. |
| `OSW/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:22` | Navbar bottom hairline as `push_solid` | Pass in colour; it changes state colour on hover only, which law 2 allows. | Draw as a shared-border segment when Lane 2 changes the band. |
| `OSW/🚶️session/🖼️chrome-frame-and-main-window/🦀️.rs:11` | Main body rect `top .. screen_h - top - footer_height - subfooter` | Fail (law 4). Content ends above the footer and never runs beneath it. | Body runs from `top` to `screen_h` minus the subfooter, and the scroll clearance is applied as padding, as in the web `--course-band-*` approach. |
| Footer paint site | Not located by grep. The footer status chips are painted in `OSW/...` (`footer_status_chips`, `🪀️view-panels/🎨️theme-file-import-and-accessibility/🦀️.rs:376`). | Verify before fixing. | Locate `render_footer` and apply the same glass band. |

Owning packages: `semio-renderer-wgpu-shell-view-chrome` (verify), `semio-renderer-wgpu-shell-session` (verify). Tests: `repo cargo --store=shared --configuration=prototype -- test -p <crate>` for the chrome crates `shell-tests-chrome-2` and `shell-tests-chrome-4` from `shell-all-handoff.md`.

### Lane 3: rounded corners and non-zero radii in chrome paint (law 7)

The token `radii::CHROME` is 0 (`🎨️styling/🔤️tokens/🦀️.rs:149`), so the radius argument of `push_glass` and `push_rounded` is zero for chrome. But the call sites still pass the radius, and several pass explicit non-zero values.

| File:line | Shape | Verdict | Fix |
| --- | --- | --- | --- |
| `UIW/🖌️paint/🎨️node-painting-with-driver/🎛️control-nodes/🦀️.rs:91` | Colour swatch `push_rounded(..., 2.0)` | Fail: explicit radius on a control. | Square `push_solid`. |
| `UIW/🖌️paint/🔬️reference/🦀️.rs:302` | Same swatch in the reference painter | Fail. | Same. |
| `UIW/🖌️paint/🖼️ui-image-sources-skeleton-and-overlays/🦀️.rs:492-500` | Media overlay button, track `2.0`, fill `4.0`, thumb `8 x 8` at radius `2.0` | Fail. | Square rects. |
| `UIW/🖌️paint/📶️progress-celebrate-and-widget-painters/🦀️.rs:247-271` | Switch rail and thumb, slider track | Fail for chrome controls. | Square, as in the CSS base layer. |
| `UIW/🖌️paint/📶️progress-celebrate-and-widget-painters/🦀️.rs:257-302` | Face discs and radio dot (`disc`) | Verify: content (presence avatar) or control. | Avatar stays round (content); radio becomes square. |
| `UIW/🖌️paint/🌲️retained-tree-node-painting/🦀️.rs:102,186,188,190` | Tree header and rows via `push_rounded(..., theme.border_radius)` | Token 0, so visually square. Path still rounded. | Use `push_solid` / `push_rect`. |
| `UIW/🖌️paint/🎨️node-painting-with-driver/🎛️control-nodes/🦀️.rs:240` and `reference/🦀️.rs:365,559,561` | Row fills with `border_radius` | Same as above. | Same. |
| `OSW/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:543` | Open panel glass `push_glass(..., theme.border_radius, ...)` | Token 0. Path still takes a radius. | Drop the radius argument. |
| `OSW/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:513,624,798,1061` | Dialog and menu glass with `theme.border_radius` | Same. | Drop the argument. |
| `OSW/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs:141` | Tour and status radius | Same. | Drop the argument. |
| `UI/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:98` | Select popup glass `ctx.theme.border_radius` | Same. | Drop the argument. |

Fix: make `push_glass` and `push_rounded` in the draw contract take no radius for chrome, or remove `Theme::border_radius` from chrome call sites and keep the radius only for content (scene and node radii, `radii::NODE_DEFAULT`). Owning packages: `semio-ui-draw-contract` (verify), `semio-ui-wgpu-paint` (verify), `semio-ui-wgpu-light`, the shell view crates. Test: `repo cargo ... -- test -p semio-ui-wgpu-paint-laws` (named in `wgpu-ui-handoff.md`, 110 tests).

### Lane 4: Select and ContextMenu rows (law 12)

| File:line | Shape | Verdict | Fix |
| --- | --- | --- | --- |
| `UI/🧱️elements/🔽️Select/🧭️behavior/🦀️.rs:17-19` | `select_row_height` returns `theme.panel_header_height`. The doc comment says `--chip-row-height`. | Drift in naming and in the token. Numerically 7 x ui-spacing, which matches `--chip-row-height` only when that is above 1.5 rem (the CSS uses `max(1.5rem, 7 * ui-spacing)`). | Use the chip row token and its `max(1.5rem, ...)` rule. Update the test at `🧱️elements/🔽️Select/🧪️tests/🔬️wgpu-select-keyboard/🦀️.rs:79`. |
| `UI/🧭️behavior/🦀️.rs:283-291` `select_row_colors` | Hover fills the row with `row_hover` and emphasises the text. Chosen row fills with `accent`. | Decision needed. The web native picker hovers the text only (law 12). The React `Select` hovers with a fill (`data-active`). Choose one and apply it to both. | Pick the React `Select` contract and document it. |
| `UI/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:112-113` | Row separators are `push_solid` hairlines, not shared-border cells. | Pass in pixels, not in structure. The compositor law (cells own their lines) is not applied. | Use `push_shared_border_edges` for rows if Select moves to the compositor. |
| `UI/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:98` | Popup glass with the radius token | See Lane 3. | Drop the radius argument. |
| ContextMenu wgpu rows | Not inspected in this pass. Items use `chrome_item_bg_at` hover fill (`OSW/🍱️view-chrome/🧱️chrome-paint-and-navbar/🦀️.rs:810`). | Verify against law 12. | Same contract as Select. |

Owning package: the crate that owns `🔽️Select` in wgpu (verify the name with `repo` before running). Test: `repo test-file -- UI/🧱️elements/🔽️Select/🧪️tests/...` for the React side, and the wgpu keyboard test named above.

### Lane 5: card stack keeps the gap concept

Law: no gap elements and no sliver closing. The handoff of `wgpu-ui-handoff.md` lists this as open.

| File | Shape | Verdict | Fix |
| --- | --- | --- | --- |
| `UI/🧱️elements/🃏️CardStack/bands.rs` | `chip_gap_reads_as_sliver`, `CARD_BAND_MIN_GAP` | Fail: a gap rule for a design without gaps. | Delete the sliver rule. Keep `snap_card_height` only if the layout still needs it. |
| `UI/🧱️elements/🃏️CardStack/bands.test.rs` | Tests of the gap and snap rules | Follows the rule above. | Delete with the rule. |
| `UI/🧱️elements/🃏️CardStack/bands.ts` | TypeScript twin of the sliver rule (`chipGapReadsAsSliver`) | Same drift, TS side. | Delete in the same change. |

Owning package: `semio-ui-wgpu-light` (CardStack). Test: `repo cargo --store=shared --configuration=prototype -- test -p semio-ui-wgpu-light` (29 tests in the handoff).

### Lane 6: schema dialect drift (TUI emphasis test)

| File | Shape | Verdict | Fix |
| --- | --- | --- | --- |
| `⌨️tui/🧪️tests/🎨️emphasis/🟦️.ts` (2 failures) | Ajv2020 cannot resolve the `http://json-schema.org/draft-07/schema#` `$schema` of the shared schemas. | Fail. | Use one dialect for all shared schemas. The new frame schemas use `https://json-schema.org/draft/2020-12/schema` (`UI/🧱️elements/🔲️WindowFrame/frame.schema.json`). Move the TUI shared schemas to 2020-12. |

Owning package: the TUI tests (`semio-ui-tui-laws`, verify). Test: `repo test-file -- ⌨️tui/🧪️tests/🎨️emphasis/🟦️.ts` (expect 5 passing).

### Lane 7: token and CSS drift for the chip tint

| File:line | Shape | Verdict | Fix |
| --- | --- | --- | --- |
| `UI/🎨️styling/🖌️ui/🎨️.css:1032` | `--chip-tint: 10%;` hand-written | Drift risk. The value is a token (`levels.chipTintPercent`). | Generate the CSS variable from `🎨️styling/🔣️.json:478` through `🎨️styling/📽️projection/🟦️.ts`. |
| `UI/🎨️styling/🔣️.json:478` | `"chipTintPercent": 10` | Source of truth. | None. |
| `UI/🎨️styling/🎭️theme/🧩️component/🦀️.rs:460` | `levels::CHIP_TINT_PERCENT` read | Correct. | None. |
| `UI/🧱️elements/🔲️WindowFrame/frame.schema.json` (draft 2020-12) | Frame schema | Correct dialect. | See Lane 6. |

Owning packages: the styling projection (TS, `🎨️styling/📽️projection`) and `semio-ui-theme` (verify). Test: `repo test-file -- UI/🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve/🟦️.ts`.

## Items not swept

- Plugin drawings (`✏️s/🔌️plugins/🌊️flow`, `🕸️dag`, `🔱️trinity/🔌️jack`, `🗒️note`, `🎞️animate`, `🖍️draw`): a grep for titles placed above node bounds in the framework found nothing. The plugin drawing code was not swept file by file, so the "node title above the box" law is unverified there.
- `UI/🧱️elements/🗂️WindowChrome/frame-structure.test.tsx:47` (TypeScript, not native) still asserts the absence of `silhouette`. Correct, listed for completeness.
- wgpu ContextMenu rows and the dialog and tour footers (`OSW/🗨️view-overlays/🗨️chrome-overlays-and-tour`) were checked only for radii, not for row and footer geometry.

## Recommended order

1. Lane 5 (delete the gap rule) and Lane 6 (dialect) are small and independent.
2. Lane 7 (generate the CSS tint) and Lane 4 (Select row contract decision) can run in parallel with Lane 1.
3. Lane 1 and Lane 3 touch the same paint files only at the call-site arguments, so run them in one pass per package.
4. Lane 2 is the largest and changes the shell layout; run it last and re-run the shell chrome suites.
