# 🪟️ WGPU Shell Chrome Geometry Audit

## Parent Runtime Follow-up: Cap Ambiguity Resolved

After this source audit, Astra measured the requested exact selectors in the fresh React puzzle3d reference at 1280×720. Both `[data-slot="mode-dock-tabbar"]` and `[data-slot="mode-dock-tab-cap"]` are **28.765625px** tall, starting at y=31.984375. This is the actual cap, not a larger ancestor. The `[data-slot="mode-dock-tab"]` inside has `p-single` (computed 3.2px all sides), `min-h-medium` (22.4px), and contains focus/close buttons with `h-medium` (used 22.390625px). The outer content-driven cap height is therefore **C + 2P**, which happens to equal B for the default tokens but must not be tied to navbar height for custom themes. The label's inner button is 16px high and vertically centered; action buttons are C high, inset by P from the cap edges. The stack-body has top padding equal to the measured cap depth (28.7656px), and the live tabpanel starts at y=60.75.

This resolves the source audit's P2 ambiguity: retain C for the internal focus/close actions and ordinary pane chips, but derive the dock cap depth from its padded content, and use that same outer depth for cap paint, silhouette safe body, tabbar and drop zones. Add a fixture with independently varied navbar height so C+2P is not accidentally replaced by B. Do not preserve the prior C-only cap assumption after this DOM evidence.

Measured first stack: tabbar `[3.1875,31.984375,423.46875,28.765625]`; cap `[3.1875,31.984375,110.796875,28.765625]`; focus `[47.640625,35.171875,22.390625,22.390625]`; close `[73.21875,35.171875,22.390625,22.390625]`; body `[3.1875,31.984375,423.46875,656.03125]`, with tabpanel `[6.375,60.75,417.09375,627.265625]`. Second cap has the same depth and a longer label. These measurements were obtained through read-only DOM geometry/computed styles; no page state or stylesheet was injected.

**Scope.** Read-only audit of the current sources on 2026-09-20. This packet concerns shell/navbar/footer/window chrome geometry, paint, and pointer geometry only. It does not claim a fresh WGPU runtime observation; the React DOM measurements below are the current-source evidence already recorded in `📓️astra-coordination.md`.

## 📐️ Shared geometry contract

The authoritative token source is `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🔣️.json:321-328`. Let `S = chrome.uiSpacingCompactPx`, `B = S * chrome.navbarHeightUiSpacing` (also footer), `C = S * chrome.controlHeightUiSpacing`, and `P = G = S * 1` (panel inset and standard gap/padding). The current values give nominal `B = 9S` and `C = 7S`; browser used values may be fractional. Implement from `Theme`/generated tokens, never from the recorded 1280px screenshot values.

The targets already consume this contract in the expected places:

- React exposes `h-large` for the navbar/footer and `h-medium` for chip controls; see `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️.tsx` and `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🟦️.tsx`.
- WGPU initializes these generated theme values in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs:261-281`; `ShellState::body_rect` uses `B` at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20441-20445`.
- The current React DOM evidence at 1280×720 reports a `B`-sized shell bar (about 28.77px), controls around `C` (about 22.39px), and `P` about 3.2px. It is confirmation of the token equations, not a replacement for them.

## P0 — navbar must be laid out as a bounded centered band

### Evidence

React declares three distinct flow roles in `🏛️ShellHost/🟦️.tsx:10297-10345`:

1. `top-left` is leading flow content.
2. The centered item is the single cluster `logoAndTitle`, optional example/mode/role controls, **then `top-middle`**. Its inter-child spacing is `gap-double`; the logo/title’s own children use `gap-single`.
3. `top-right` is ordinary trailing flow content. `Navbar` also has its measured fullscreen/trailing slot (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:9495-9537`). Open panel hosts retain their previous width rather than changing this occupied geometry (`…/🟦️.tsx:7480-7545`).

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️.tsx:113-182` derives the widest free interval after collecting the occupied row rectangles, clamps the centered content into that interval, and applies its width as `maxWidth`. The language-neutral React fixture `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔝️navbar-centered-band/🟦️.ts` explicitly exercises occupied spans `[0,173]`, `[947,1185]`, free band `[173,947]`, and a 753px cluster to prevent role-control occlusion.

The current WGPU path contradicts that structure:

- `ShellState::navbar_trailing_tab_row_item` at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20484-20492` still places both `TopMiddle` and `TopRight` in a trailing row.
- `render_navbar_step` at `…/🦀️.rs:21348+` first advances `cursor.x` through leading tabs, logo/title/badge, example, modes, and roles; the same rectangles are painted and registered as `HitTarget`s. It then walks the stale combined trailing row right-to-left and places fullscreen from `cursor.right`.

This explains the fresh React/WGPU offset documented in `📓️astra-coordination.md`: the React app-name begins near the physical centered-band start while WGPU begins it immediately after leading tabs. It is a paint and interaction error, not merely a text alignment error.

### Implementable change

Before the retained paint phases, derive a `NavbarChromeLayout` from measured WGPU item widths and the shared token metrics:

- Measure the leading `TopLeft` band, the trailing `TopRight` band, and fullscreen/trailing-slot width. Preserve panel-host placeholder widths while their panels are open.
- Build the occupied intervals including the horizontal padding and applicable `G` separators. Normalize/merge them, select the largest remaining interval, and use the React centered-left rule: center the actual constrained cluster in the viewport, then clamp its left edge to the free interval. Constrain the cluster’s renderable width to that interval before publishing hit rectangles.
- Treat the center as one measured group in this order: logo/title/role badge, example selector, mode switcher, role switcher, `TopMiddle` panel tabs. Use `G` inside logo/title and `2G` between these siblings. Do not put `TopMiddle` in the right band.
- Paint/register leading, centered, and trailing groups from that plan. Pointer targets must be only each control’s constrained physical rectangle; do not use the whole free interval as a hit target.

The retained cursor can remain incremental: store the completed rectangle plan and per-item positions before phase zero, rather than deriving `cursor.x` from the previous item. This is necessary because full cluster width and trailing occupancy are both inputs to its first x-coordinate.

### Acceptance fixtures

- Extend the existing `navbar-centered-band` fixture with the native-layout input/output contract, rather than a 1280px constant. At the documented 1280px case, its constraints must hold: cluster bounds lie in `[173,947]`; no role/example hit rect intersects `TopRight` or fullscreen; `TopMiddle` is within the center group.
- Add equivalent WGPU unit coverage beside `🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs`: assert geometry and `HitTarget` rectangles for 1280px, the first desktop width, and a narrow desktop width. At every width, every published center control must lie within the selected free band and must not overlap a leading/trailing hit rect.
- Keep the mobile breakpoint behavior as an explicit fixture. React omits the desktop center controls and panel chrome tabs on mobile; WGPU must select the same compact composition before applying desktop free-band logic.

## P1 — footer must reserve its actual trailing sequence and center the middle tab band safely

### Evidence

React’s active footer source is `🏛️ShellHost/🟦️.tsx:10850-10884`:

1. bottom-left and centered bottom-middle panel hosts (desktop),
2. a flexible fill,
3. presence (`#s-presence-peers`, desktop),
4. `HubConnectionIndicator` on every device,
5. bottom-right panel tabs (desktop).

`Footer` uses the same bounded-centered-band mechanism as `Navbar`. The center item therefore cannot simply be placed at `width / 2` if the leading/footer trailing occupied rectangles close in.

WGPU instead has two independent layout paths:

- `footer_pill_anchors` at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20517-20522` only measures bottom-left/right tab bands, then derives direct sync/presence positions. It does not reserve a hub indicator or the centered tab group.
- `footer_tab_row_rect` at `…/🦀️.rs:20544-20561` locates `BottomMiddle` at the raw viewport midpoint; it can overlap the direct pills, while paint/hit ordering lets later tab hits win.
- `render_footer_step` at `…/🦀️.rs:21859-21933` paints direct `#s-sync-status` and presence in phase 3, then footer tabs in phase 5. React still declares `s-sync-status` as a panel utility node (`🏛️ShellHost/🟦️.tsx:9310-9320`), but it is absent from `footerItems`; its footer successor in the current composition is `HubConnectionIndicator`.

### Implementable change

Make the footer consume the same generic occupied-interval/centered-band planner as the navbar, with a footer sequence descriptor:

- leading bottom-left tabs;
- centered bottom-middle host;
- normal trailing children in their React order: presence, hub connection, bottom-right tabs;
- device gates matching React.

The planner should place the centered bottom-middle host in the largest free band after all normal items are measured. Place every normal footer child exactly once, then derive paint and hit rectangles from the same plan. This removes collision-dependent draw/hit ordering.

Align the semantic/status projection before changing the visual: WGPU currently exposes a direct sync pill and has no source-visible counterpart for React’s direct hub indicator. The owner must identify the available native hub state and action contract. If it is unavailable, record that as a product-state gap; do not relabel the sync pill as hub merely to imitate its rectangle.

### Acceptance fixtures

- A footer geometry fixture with non-empty bottom-left, bottom-middle, presence, hub, and bottom-right content; verify no pair of paint or hit rects intersects, and that the center group remains in its computed free interval.
- A compact/mobile fixture confirms no desktop panel tabs/presence while the hub state remains present, matching `footerItems`.
- A rendered interaction check should press each resulting item’s geometric center and verify its corresponding control id, so z-order cannot mask an overlap.

## P1 — active dock tabs need the React active-base fill

### Evidence

React applies `modeDockActiveTabFillClass` to the globally active window at `🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1034-1044`; its exported definition is `interactiveActiveFillClass` and documents a primary fill at `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:7819-7823`.

WGPU identifies `stack_active_tab` at `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1637-1705`, but changes only glyph tint before drawing text/icons. The surrounding group glass remains unchanged. `Theme::selected` is already the generated `chrome.active_base` (`🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs:261`), so no new color token is required.

### Implementable change

Within the existing tab group glass content, paint a `theme.selected` solid rectangle for the globally active tab before its label/actions, restricted to `tab.rect`. Keep inactive tabs transparent and keep the silhouette outline responsible for the outer border. Preserve the existing distinction: a selected tab in an inactive stack does not receive the globally-active fill.

### Acceptance fixtures

- WGPU Dock test: two stacks with independently selected tabs and one globally active stack. Assert exactly one selected-fill rectangle, at the active tab’s geometry, with the generated selected color.
- React/WGPU parity fixture: active tab’s label/action targets still have their existing `C`-high rectangles and are fully contained by the fill; inactive stack tabs do not use selected fill.

## P2 — cap, window body, pane, and utility geometry: validate the premise before widening

### Current-source outcome

The proposed change from `C` to `B` for every WGPU dock cap is **not supported by the current React source**.

- React `WindowChrome`’s cap row has no `h-large`; its physical chip cells use `min-h-medium` in both normal and `chipOnly` variants (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:7994-8168`). Mode dock tabs similarly declare `min-h-medium` (`…/🟦️.tsx:7770-7823`; `🧱️elements/🎨️Canvas/🟦️.tsx:1034-1044`). The recorded 28.765625px “top cap wrapper” is therefore not enough to conclude that its actual chip/cap depth is `B`; it may be a measured wrapper/stack box or a state-specific ancestor.
- WGPU’s cap, hit band, silhouette depths, and safe body all consistently use `C`: `stack_tab_bar_rect` (`🛰️Dock/…/🦀️.rs:1106`), `render_stack` (`:1637+`), `layout_stack_cap` (`:1897+`), and `stack_window_silhouette` (`:1935+`). `stack_body_rects_with_silhouettes` returns the silhouette safe body (`:367-384`). A blind change to `B` would move body geometry and all tab/drop targets beyond what current React source specifies.
- Shell canvas inset is already aligned. WGPU’s shell body starts after `B` and its main window bounds are inset by `P`; React’s mode canvas uses `p-single`. The documented body origin at `B + P` agrees with this. No global shell body offset change is warranted.
- Pane chips also match the token contract: WGPU `window_pane_chip_rect` (`🐚️Shell/…/🦀️.rs:15517-15531`) places a `C`-high chip at `P` from the supplied safe body’s relevant edges. React `chromePanelSafeAreaStyle` has the same default `--spacing-single` base (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:9402-9413`). WGPU utility rails begin right of the bottom-left chip and use a `C`-high row (`🐚️Shell/…/🦀️.rs:15724-15820`), consistent with React’s `utilityBarBodyClass` `min-h-medium` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:8262-8264`).

### Required disambiguation fixture before any cap-height edit

At the same viewport and app state used for `📓️astra-coordination.md`, capture React rectangles for:

- `[data-slot="mode-dock-tabbar"]`;
- every `[data-slot="mode-dock-tab-cap"]` and its tab button;
- `[data-slot="mode-dock-stack-body"]`;
- its `window-chrome-footer` chip rows when present.

Compare cap and body boundaries against the WGPU silhouette/tab-bar/body rectangles using `B`, `C`, and `P` from the live theme. Widen WGPU cap depth only if the actual cap **chip** rectangle is `B`; if only an ancestor is `B`, preserve `C` hits/silhouette depth and identify the ancestor’s surrounding layout rule separately. This prevents a large body/hit regression based on an ambiguous wrapper measurement.

## 🧭️ Prioritized implementation order

1. Introduce one measured occupied-band layout helper/fixture and use it for navbar first. Move `TopMiddle` from WGPU trailing data to the center plan; change center sibling gaps to `2G`.
2. Apply the same helper to the footer with the current React sequence, while resolving the hub-state projection rather than retaining a misleading direct sync visual.
3. Paint the active dock-tab selected fill with the existing `Theme::selected` token.
4. Run the cap DOM rectangle disambiguation fixture. Keep current `C` cap/body/pane/utility geometry unless that fixture contradicts current source.

## ⚠️ Ambiguities and ownership boundaries

- The audit intentionally makes no claim about popup/select state or window lifecycle. Those areas are owned by other active work.
- Exact native truncation policy for a center cluster narrower than its free band is not encoded in the WGPU source. React uses `min-w-0` plus a measured `maxWidth`; the WGPU plan must constrain children before emitting their hit rects, then validate no-overlap at the first desktop and narrow desktop widths.
- The current WGPU source comments describe `TopMiddle` as trailing and a direct footer sync pill as React parity, but the current `ShellHost` composition contradicts both. The current executable source and current DOM-measurement record take precedence over those historical comments.
- No source change, test change, build, activation, ticket lifecycle operation, or Git operation was performed for this audit.
