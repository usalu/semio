# Six Layout and Dock Widget Coverage Preparation

## Current Evidence

The current eight-case physical Dock packet covers split-left/right/top/bottom, merge, reorder, Escape cancellation and configured template creation. React now passes all eight against the fresh app. WGPU passes the seven topology cases; the actual Display template path remains red. Those facts do not cover every `LayoutSpec` or window widget.

The six contract variants are `leaf`, `stack`, `grid`, `overlay`, `scroll`, `absolute` in `ui/🧬️contract/📐️layout/🦀️.rs`. React consumes them in `Interpreter/🟦️.tsx::layoutSpecStyle` and `ContainerView`; native consumes them in `ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs::flow_from_spec`, dimension resolution and grid-track resolution. Searching current plugin authored Rust/TS found no direct `LayoutSpec::Grid/Overlay/Absolute/Scroll` or `UiNode::Grid/Overlay/Scroll` production producer. A source switch is not a mounted six-variant runtime receipt; an existing lawful producer/host route must first be established for each record form.

## Exact Display Premeasure Boundary

`Shell::render_panel_step` phase 0 computes its anchor rectangle from the previously retained intrinsic extent. Its `cursor.flag` skips phase 1 premeasure whenever an intrinsic extent already exists. A disclosure changes retained layout while that older extent still exists, so phase 8 can lay out and paint expanded children inside the previous compact host box. The current paint then clips the new branch and publishes that clipped registry. Subsequent frame invalidation/reflow is not proven by the fixture.

The proposed repair boundary is phase 1: drive ingress/reconcile/layout to `UiDocumentFrameCursor::layout_is_accepted` before painting the panel glass and tab bar, recompute `anchor_rect`, and restart viewport resolution only when that host rectangle changed. The bounded document cursor already has the required restart operation. This would apply the existing first-frame premeasure to changed intrinsic content, without a second frame obligation. `surface_content_height` chooses candidate intrinsic data after interaction rebase is ready; it chooses presented data while a candidate cannot safely contribute. The new native single-paint law drives real Shell publication, normalized pointer input and `seal_presented_input_candidate`/acknowledgement through the existing test helper. Production is unchanged until its executable red.

## Concrete Customization Gap in Six-Variant Native Flow

React `spaceTokenRem` resolves `xs/sm/md/...` via `uiSpacingLen`, which consumes live `--ui-spacing`. Native `flow_from_spec`, `dim_of` and `grid_track` call `SpaceToken::px`/`EdgeSpace::px`, which multiply the generated canonical `UI_SPACING_COMPACT_PX` constant. Thus the new Shell eighteen-field projection does not cover authored retained-node padding/gaps/fixed dimensions. With explicit compact `5px`, `xs` must become 5px in both targets; the native current source computes 3.2px. The same fixed authority appears in native named component gap/padding resolution. This is a source-identified gap, not yet a runtime confirmed failure.

A repair must pass the resolved authored spacing step into all layout token resolution, including fixed grid tracks, rather than infer it from a customizable gap/control multiplier. A derived native Theme spacing-step field can carry the canonical compact/root projection without restoring a second editable `uiSpacingCompactPx` authority. Canonical defaults remain derived from authored compact/root tokens.

## Prepared Neutral Acceptance Packet

`🔬️layout-coverage/🧫️fixtures/🔣️.json` contains strict six-variant expectations over two explicit compact/root themes. Every row requires independent browser CSS measurements and native accepted-layout bounds, owning controller/window identity, EN/DE accessible activation, and exact event counts. The companion schema rejects unknown variants, missing actions and malformed measurement requirements. This is prepared input, not an executed or passing gate.

Additional physical Dock widget rows require divider displacement with conserved extent, maximize/restore with preserved identities, close-active/background with correct focus and retired hits, close-last into an empty stack, reopening with fresh body ownership, and split/join corner cancellation. Existing normalized pointer and maximize-resync fixtures cover portions in isolation; they need current paired physical receipts through ordinary app controls. Root metadata/routes remain coordinator-owned.

## Existing Mounted Native Route

The existing `ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs` contains a lawful production route: `mount` attaches each authored LayoutSpec to the retained tree, and `layout_tree_now` drives MountedLayoutJob before reading published bounds. Its current grid law verifies two fractional columns but uses Theme::default; existing overlay-flow fixture validates placement on canonical spacing. The flex fixture likewise drives the same production measure/arrange ladder, but its token-ramp law hardcodes canonical3.2px. These are appropriate fail-first seams for a custom5px/20px-root packet, without exposing a test-only UI to a plugin. A paired real Chromium React ContainerView/Interpreter oracle can consume the same retained record layout fixture. Parent ownership of executable metadata remains required.

The focused Display green attempt `sol-native-theme-display-introduction-green.txt` was rejected by a concurrent nonliteral import in actor lifetime test source before Cargo; missing graph producers were cascading errors. No new native result was inferred.

## Root-Rem Projection Coverage Gap

The native Theme already has `root_rem_pixels`, consumed by `resolve_shell_navbar_control_width` for root-relative clamps. `Theme::default` initializes it from canonical `dom::ROOT_REM_PX`; `apply_theme_document_metrics` currently assigns the18 mapped geometry fields but leaves this existing reference field unchanged. A custom-root20px case consequently resolves compact spacing correctly for the18 fields while retaining16px for independent root-rem control clamps. This source gap belongs in the same derived compact/root transport packet; it needs an additional direct root-reference assertion in the neutral native projection law before repair. The running focused gate intentionally tests its current18-field contract and does not claim this uncovered reference is customized.

## Strict Dock Widget Inputs

`🔬️dock-widget-coverage/🧫️fixtures/🔣️.json` and its strict schema now author seven additional physical rows: divider resize, maximize/restore, close active, close background, close last, reopen, and perpendicular corner resize. Every row specifies actual UI selectors, preconditions, public pointer/action input and independent geometry/identity/accepted-owner consequences. The corner control resizes both axes and aligned peer cross separators; it does not merge stacks. Existing React `pointercancel` ends the drag at the last accepted resize, so this packet does not invent an Escape rollback contract for that widget. These are prepared input files without an executor or passing result.

## Permanent Widget Executor Snapshot

The seven widget vectors are now embedded into the domain-owned Dock browser-acceptance fixture/schema, and the existing permanent script accepts `run react|wgpu URL --suite widgets --output ABSOLUTE_DIRECTORY`. Default invocation retains the original eight docking cases. Widget runs iterate explicit English and German persisted locale preference events in isolated Chromium contexts. All setup uses actual template transfer, tab drag and close actions; no direct layout mutation is used. Reopen specifically closes and recreates an extra orthographic template instance, the ordinary producer of unique window-instance identities. Accepted owners are captured from visible window/surface DOM or native published surface/input geometry; the React DOM does not expose parent-document/app-instance identifiers, so those are not invented or claimed by this widget packet.

The updated contract invocation `sol-dock-widget-contract.txt` passed uncached through Nx in2.2s with strict Ajv validation of the complete fixture and trusted Chromium drag/cancel events. The actual EN/DE React widget invocation is running at7300 in `sol-dock-widgets-react.txt`; no complete widget pass is yet claimed.

## Actual React Widget Red

`sol-dock-widgets-react.txt` executed14 physical rows across English and German,10 passed and4 failed in4m49s. Both locales passed divider resize, maximize/restore, active-tab close, background-tab close and last-window close, including actual body geometry, accessible action names, focus and retired-control consequences. Reopen was blocked by the adapter toggling an already-open Display category closed on the second template transfer; source opening is now idempotent based on visible public template controls.

Both corner rows recorded actual production failure: a60px/40px twelve-step pointer drag moved only5px on the first step. The page also emitted the independent resizable primitive error about stacking order requiring a common ancestor. The global corner interceptor retained a DOM element whose callback was deleted when size-keyed group reconciliation remounted it. After this executed red, it now captures the admitted resize function and authored split spec for the pointer lifetime. The stable callback resolves current axis group refs at every movement, so detached element cleanup cannot retire an active gesture. The focused rerun also strengthens setup to four windows with aligned cross-axis peer splits and requires both peer heights to move40px, preserving the original60px/40px consequence.

The focused EN/DE reopen and strengthened corner rerun is pending in `sol-dock-widgets-react-repaired.txt`. No full14-pass result is yet claimed.

The retained `TreeRowMetrics` is already passed into flex construction and is a natural transport for a derived spacing step. Its standard control-width field also uses compiled `TREE_ROW_CONTROL_WIDTH` (canonical `controlValueColumnUiSpacing × UI_SPACING_COMPACT_PX`) instead of an authored compact projection. The next token packet must verify this independent named reference against the React value-column CSS, rather than assume that changing flex gap alone completes customizable component geometry.

React Tree source confirms `uiSpacingLen(STYLING_DOM.controlValueColumnUiSpacing)` for its ordinary value column and `windowMeasureValueColumnUiSpacing` for compact presentation. The live authored spacing step therefore affects React column width; native compiled pixel constants do not. The named multiplier itself is compiled in these React locations, so independent multiplier customization is an additional shared-authority question, not a demonstrated React-versus-native multiplier divergence. The immediate derived-step packet can preserve the current factor while eliminating the spacing mismatch.

## Complete React Widget Green

The first focused repair ran four rows: both reopen rows passed; both strengthened corner geometry checks passed but the gate failed on independent resizable primitive page faults. Reading the installed primary library source showed its pointer-move handler ignores default-prevented events, while its inactive hover path compares the event target stacking order with mounted groups. Our owned corner callback reconciled the target out of the DOM before that later path ran. The interceptor now consumes only its admitted active pointer movements before invoking resize, preserving explicit event ownership. No library code or fault assertion was changed.

`sol-dock-widgets-react-green.txt` then executed the complete seven-case matrix in both locales and passed all14 in5m9s. Both corner receipts moved59.921875px horizontally and39.84375px vertically within the2px authored tolerance, and both aligned upper cross-axis peers gained39.84375px height. Canvas extents, four window identities and stack counts were conserved. Browser faults were absent. Ordinary maximize/restore and close/reopen body/focus/retirement consequences passed. Actual accessible names were Focus/Unfocus/Close in English and Fokussieren/Fokus aufheben/Schließen in German.

Full receipts, public owner/geometry observations, screenshots and console files are under `🗑️generated/sol-dock-widgets-react-green/react/{en,de}`. Fresh WGPU original-eight and widget14 receipts remain owed after the coordinator publishes its repaired WASM source.
