# Staffage Phone Verification Follow-up

## Catalog Validation

The full Staffage React run returned 136 ready and 16 invalid records out of 152. The failures come from the authored geometry currently tracked by the open [Perfect Handcrafted 3D Models per Scale and Mode for Every Staffage ticket](../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🎫️ticket.json), ID `2026/11/14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`. That ticket has independent model sources and check reports, with Bamboo Clump and Lombardy Poplar explicitly marked in progress.

- Ivy on Trellis still draws `crown-l` and `crown-r` at 1:500. Direct inspection of the current asset shows that its 1:500 body contains only `panel` supports and has no weighted solids. This reproduces the missing-body-coverage findings independently of the browser. The previous suite also reported 70 mm top and bottom rails below the 75 mm threshold and thermal and structural solids copied from finer scales.
- The full suite reported Multi-stem Birch stem widths of 72.3, 53.2 and 62.7 mm against the 75 mm threshold at 1:500. Qualifying slender exceptions require a measured ratio of at least 20:1. Choosing new geometry or declaring that per-solid exception requires the author's per-scale model and verification.

These findings concern geometry authoring and the explicit print and scale contracts. No asset geometry or validation rules changed in the phone ticket.

## Phone Recovery

The detail had two actions named Try again when an individual model failed to load. The fallback action reloaded the entire category. The phone detail now presents only its selected-asset retry or cancellation notice while a request is loading, cancelled or failed. A record rejected by validation still shows the existing localized issue count and first five findings, without offering a network retry for persistent geometry errors. The regression expects exactly one retry action and verifies that both attempts request the same asset ID and never request the category.

## Additional Verification

[staffage-phone-stage.mjs](./staffage-phone-stage.mjs) sends a native touchscreen gesture through Chromium, checks that the detail scroller moves, observes pointer cancellation on the stage and verifies that the asset address is unchanged.

The repository launcher stalled on its pinned Bun receipt and later recovered. The registered Staffage React target then passed all 11 grid and phone tests, including cancellation and selected-asset retry. The registered viewer component test passed with a cancelled touch pointer followed by a completed drag. The first gesture cleared the temporary transform and published no pan. The second published a pan. The Staffage React TypeScript check also passed.

The playground composition suite passed all six cases, including the `svgelements` geometry oracle, when uv used a ticket-local cache.

The complete registered Staffage React suite passed 294 of 295 tests with the ticket-local uv cache. Its sole failing assertion expected all 152 authored assets to be ready and received 136 ready and 16 invalid, matching the independent catalog validator. The final post-gesture-fix rerun again passed 294 of 295. The same authored-readiness test reached its explicit 120-second limit during that run. The final Staffage TypeScript check passed.

The actual WebKit phone smoke passed through the isolated Vite middleware after its configuration prebundled the React entry points. At 320 × 640, the overview had 658 pixels of content inside a 532-pixel scroller and no horizontal overflow. The test tapped through the category and detail, requested exactly one Walking Commuter asset, started playback, recovered the catalog scroll position with Back and opened the all-assets filter controls. At 844 × 260 with 200% text and a French locale, it scrolled the language gate, selected German and opened the localized phone overview. [Browser harness](./staffage-phone-browser.mjs) · [Original Chromium verification](./staffage-phone.md)

The independent Chromium stage swipe first revealed that the two touch utilities computed to `pinch-zoom` alone. Chromium kept the on-stage pointer, saved a pan in the address and left the detail scroller at the same offset. The stage now declares `pan-y pinch-zoom` as one value. All 18 canonical grid and viewer tests passed after this change, including the combined touch-action assertion and the cancelled-versus-completed drag regression. The Chromium phone rerun verified the computed declaration and sent a genuine touch gesture over the drawing. The browser cancelled the stage pointer, the detail scroll offset increased from 465 to 577 pixels, and the drawing position and address stayed the same. [Stage gesture harness](./staffage-phone-stage.mjs)

An isolated Node worker ran the repository's production `validateAsset` against all 152 current authored records. It confirmed 136 ready and 16 invalid. The failures span five vegetation records and eleven animal records.

| Category | Invalid records |
| --- | --- |
| Vegetation | Hornbeam Hedge, Hosta Clump, Ivy on Trellis, Miscanthus Grass, Multi-stem Birch |
| Animals | Dairy Cow, Goat, Hedgehog, Horse, Labrador Dog, Mallard Duck, Pig, Pigeon, Red Fox, Roe Deer, Sheep |

The failing rules cover bone or thermal-region coverage, minimum thickness, and solids copied across authored scales. [Catalog validator entry point](./staffage-catalog-check.ts)
