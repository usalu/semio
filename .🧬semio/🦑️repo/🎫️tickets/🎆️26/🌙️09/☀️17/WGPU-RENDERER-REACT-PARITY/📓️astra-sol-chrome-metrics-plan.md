# Astra Sol — Chrome Metrics Activation Plan

## Scope held behind checkpoint 17

This review prepares the remaining Shell chrome metrics packet while the Native62 capture remains sealed. It changes no React or WGPU runtime source. The shared neutral fixture and strict Draft-07 schema are already present at `🧰️framework/🔨️modules/🖱️ui/{🧫️fixtures,🧬️schema}/🔝️navbar-centered-band/🔣️.json`; activation must first register laws against those rows, then change production.

## Example control source oracle

React places the example trigger inside a flexible wrapper and gives the trigger `min-w-[12rem] max-w-md` at `NavbarExampleSelect/🟦️.tsx:67-75`. The measured activation-17 trigger is 192 px although the WGPU intrinsic chip is 110.179 px. With the browser's 16 px root rem, the authored bounds are exactly 192 and 448 px.

WGPU currently has no width policy on `ShellNavbarControl` (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:17204-17209`). `shell_example_control` creates the same shape as every mode and role control at lines 17286-17291. Reservation calls `retained_chrome_group_item_width` through `navbar_control_band_width` at lines 23161-23169; paint independently calls the same intrinsic width helper at lines 24495-24504. Adding a late x offset would move paint after reservation and preserve the overlap defect.

The bounded production shape should be one optional closed width policy on `ShellNavbarControl`, populated only by `shell_example_control`. A pure resolver should compute

`clamp(intrinsicPixels, minimumRem × rootRemPixels, maximumRem × rootRemPixels)`.

Both `navbar_control_band_width` and `render_navbar_cluster_step` must invoke that same resolver before summing or constructing the rectangle. The law also passes 20 px and requires 240 px for both reservation and paint, proving the implementation does not substitute `Theme::font_size_emphasized` (14.4 px), `font_size_small` (11.2 px), device scale, or a post-layout offset.

## Root-rem authority audit

There is no existing native root-rem or general UI-scale authority to reuse:

- React's `html` rule does not author `font-size` (`ui/🎨️styling/🖌️ui/🎨️.css:180-183`), so the live 16 px value measured by Chromium is the browser's initial root font size.
- `WindowMetrics::logical_size` divides physical extent by device scale (`ui/🖥️host/🪟️window/🦀️.rs:28-46`). The renderer then uses logical pixels for layout and sends scale only to raster atlases (`renderer/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12303-12314`). The dedicated DPI law explicitly forbids device scale from moving chrome (`Shell/🧪️tests/📐️wgpu-dpi-logical-units/🦀️.rs:1-16`).
- `Theme` owns logical layout metrics but currently has no root-rem field (`ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs:92-140`). Generated typography includes `TEXT_LG_PX = 16` (`ui/🎨️styling/🔤️tokens/🦀️.rs:375-385`), but that value describes large text. Its numerical equality to the browser root is not semantic authority.

The clean activation shape is therefore a generated `dom.rootRemPx = 16` metric projected as `ui_styling::dom::ROOT_REM_PX`, plus `Theme::root_rem_pixels` initialized from that metric. This keeps the value in the existing logical-layout carrier while separating it from text size and device density. The custom 20 px fixture row mutates that explicit metric in the native law and must reach both real width consumers.

Required registered Rust law in the existing `wgpu-navbar-footer-parity` module:

- load and validate every `exampleControlMetrics` row;
- resolve 110.179 → 192 and 720 → 448 at rem 16;
- resolve 110.179 → 240 at rem 20;
- call the same reservation and paint-width functions and assert both equal `expectedReservationPixels` and `expectedPaintPixels`;
- assert mode and role controls remain intrinsic and no x offset enters the law.

The existing React neutral suite should consume the same rows with a small dependency-free clamp twin, while the mounted React oracle retains the real class receipt. The schema is the language-neutral contract; the real Tailwind class is the third-party/browser oracle.

## Task Manager label source oracle

React builds `os.task-manager` with `shellLabel("ui.panelToggle.taskManager")` at `ChromePanels/🟦️.tsx:1443-1448`. The live locale bundle resolves that semantic key to `Tasks` and `Aufgaben` at `ui/🎯️targets/⚛️react/🟦️.tsx:3541-3545` and `:2655-2659`.

WGPU builds the same dock leaf from `taskManager.title` at `Shell/🎯️targets/🧊️wgpu/🦀️.rs:8786-8789`, whose string table returns `Task Manager` and `Task-Manager` at lines 27533-27534. That key has no other consumer. The production repair should give the dock leaf a dedicated `panelToggle.taskManager` string pair `Tasks` / `Aufgaben`; body heading and command copy remain separate affordances. The neutral fixture now pins its complete footer identity: id `os.task-manager`, anchor `bottomRight`, order `2`, icon `cpu`, and the two locale rows. React's actual source declares those first four fields together at `ChromePanels/🟦️.tsx:1443-1448`.

The Rust law must assert exact id, icon, order, and EN/DE label after `sync_dock_tabs`. The React law must call the actual `uiI18n` lookup for the fixture key in both locales. Geometry receipts should assert text and rect together so a shorter wrong label cannot pass from width alone.

## Activation order

1. Register neutral React and Rust laws and capture their focused RED results.
2. Add the closed example width policy and shared resolver; route reservation and paint through it.
3. Add the WGPU `panelToggle.taskManager` locale pair and use it only for the dock leaf.
4. Run the focused React neutral/mounted oracle, focused native chrome filters, then the root-owned full native census.
5. Capture activation evidence at root rem 16: example width 192 px, task labels `Tasks` / `Aufgaben`, and no regression in centred-band overlap.

Presence remains a painted noninteractive status and is outside this packet.

## Held neutral and native drafts

The shared fixture and Draft-07 schema now pin five named width rows rather than accepting interchangeable records. They reject unknown width members, a missing/substituted named row, a changed expected reservation/paint width, duplicate locale rows, and wrong Task Manager identity/order/icon/label data. The ticket-only neutral Bun law is `🔬️chrome-metrics/🧪️draft/🧭️neutral.🟦️.ts`.

The ticket-only native draft is `🔬️chrome-metrics/🧪️draft/🦀️.rs`. Its width law first drives the exact 110.179→192, 720→448, and custom-rem 110.179→240 rows through the production resolver. It then drives the actual `navbar_control_band_width` reservation and `render_navbar_cluster_step` painter, reading the registered `playground.navbar.fixture` hit rectangle as painted geometry. Its footer law installs a real session and reads `default_dock()` for both locales; it never constructs a fake dock row. The draft is intentionally unregistered and has not been compiled while checkpoint 17 is capturing the current repaired binary. Once registered before production, current source should be RED because the width policy/root-rem seam does not exist and `default_dock` still emits `Task Manager` / `Task-Manager`.

The scoped neutral receipt is:

```text
bun nx exec --projects=workspace -- bun test ./…/🔬️chrome-metrics/🧪️draft/🧭️neutral.🟦️.ts
4 pass, 0 fail, 26 assertions, 284 ms, exit 0
```

## Prepared live React oracle

The ticket input `🔬️chrome-metrics/{🧪️draft,🎚️config}/🟦️.ts` is an actual Chromium law against the current-source React app at port 6313. It waits for the real `playground.navbar.fixture` trigger, gives its existing host the fixture's available width, changes the document root font between 16 and 20 px, and reads computed `min-width`, computed `max-width`, and `getBoundingClientRect().width`. It contains no geometry mock. The page-local style changes are only an isolated CSS sizing oracle and do not establish app interaction parity.

The first scoped Nx browser oracle passed on 2026-09-21: one Chromium test passed in 20.2 seconds (19.3 seconds test time), exit 0. It exercised the initial three fixture rows against the actual mounted trigger and proved exact widths 192 px, 448 px, and 240 px for root-rem inputs 16, 16, and 20.

After adding the responsive interior and scaled-maximum rows, the same mounted-source oracle passed again: one Chromium test passed in 13.1 seconds (12.3 seconds test time), exit 0. It drove all five rows through Chromium's computed `min-width`, `max-width`, and actual trigger rectangle. The exact observed contract is therefore `110.179→192` and `720→448` at a 16 px root, then `110.179→240`, `360→360`, and `720→560` at a 20 px root. The first retry after generated-output cleanup was inconclusive because port 6313 was not listening and Playwright exited 137 before any assertion; the passing receipt came from the restored ticket-owned no-Cargo React source server. The command was:

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true \
PLAYWRIGHT_BASE_URL=http://127.0.0.1:6313/ \
PLAYWRIGHT_BROWSERS_PATH='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright' \
bun nx exec --projects=workspace -- \
  bunx playwright test \
  --config '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️chrome-metrics/🎚️config/🟦️.ts'
```

The `workspace` project scope is significant: it invokes Playwright through Nx without requiring the unrelated cyclic generated stdio/docx project chain. The earlier unscoped daemon-free retry stopped before Playwright on the existing `value-derive-rs ↔ framework-os-kernel` graph cycle and is infrastructure evidence, not a renderer failure. No direct-Bun bypass or workspace graph suppression was used. Future Playwright artifacts are directed into the ticket's generated runtime directory.
