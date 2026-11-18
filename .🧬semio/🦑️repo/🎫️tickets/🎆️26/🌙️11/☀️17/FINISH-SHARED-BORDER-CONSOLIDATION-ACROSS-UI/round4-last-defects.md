# Round 4 last two defects (play landing seam at 768, OS shell footer overflow at 768)

## 1. semio-tech play landing at 768x1024: 1.0 px pairs between 59 px cards

Root cause (measured, not the earlier guess about the compositor): `createWindowSilhouetteGeometry` builds the logical outline at inset 0 and the paint outline at inset 1. A title chip that ends less than about 1.5 px short of a card side (or starts less than that from the other side) gives a 0.5-1.5 px step at inset 0 that collapses at inset 1, because the chip is clamped to the inset bounds there. The two outlines then have different lengths (6 vs 4, 8 vs 6 points). The compositor's `degenerate` guard (`geometry.outline.length !== windowSilhouetteOutline(metrics).length`) then painted the default `borderPath` (left line at x+1) while the neighbour merged the shared seam on the centreline (x): the 1.0 px pair. A sweep of left 0..3 and right 56..59 on a 59 px card reproduced it for every margin up to 1.5 px.

Design decision: a chip that reaches within two border insets of a card side reaches that side. Then the cap row IS the card top, the silhouette is a plain rectangle at every inset, outline lengths always agree, and the compositor resolves the coincident segments normally (the `degenerate` branch no longer triggers for this case). Chips with a real margin keep their step.

Implementation
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/🟦️.tsx`: `WINDOW_SILHOUETTE_FULL_SPAN_SNAP` (2 x inset) and `snapWindowSilhouetteChips`, applied to top and bottom chips in `normalizeWindowSilhouetteMetrics`, so every consumer (outline, clip path, regions, glass) sees the same snapped chips.
- Tests (language-agnostic): `🔲️WindowSilhouette/chip-span.cases.json` + `chip-span.schema.json` (4 cases: full width, 1 px short, starts 1.75 px in, real margin keeps step; logical and paint outlines), wired in `🔲️WindowSilhouette/shared-border.test.ts` with an Ajv schema check and a sweep asserting equal outline lengths for every near-full span. Run: WindowSilhouette, WindowChrome (`shared-border.test.ts`), CardStack and border-presentation `segments.test.ts` suites green (64 of 65; the single failure is the Playwright browser test, chromium binary not installed on this machine, unrelated).
- Rust mirror `🔨️modules/📏️border-presentation/📦️packages/🦀️rust`: no change needed, the crate mirrors the segment/priority resolver, not the silhouette geometry (no silhouette symbol in it). The compositor behaviour itself is unchanged.

Live (in-app browser, own tab, fronted before every probe, only `pending:false`): play introduction, landing, opened card, back: 768 count 0 (landing was 1-2), 375 count 0, 1440 count 0; no horizontal overflow (docScroll equals viewport).

## 2. OS shell footer at 768 with credits: 972 px of items in a 768 px footer

Measured: tab bars (147 + 221), local-only chip 71, two 48 px gap cells, leading credits 299, trailing credits 136 (three wordmark logos). The tablet logos-only mode (earlier fix) was still 435 px of credits and cannot fit next to the shell's own chrome; at 375 the same logos were wider than the phone.

Design decision: below desktop the credits fold into ONE shared-border footer cell: an icon button (`ui.footer.credits`) with an accessible name built from the localized captions (en and de, no new strings, no default language) that opens a popover with the full captions, separators and logo links. The gap cells and the second credit cell disappear in that mode; the filler keeps the cell right-aligned. Desktop is unchanged (full captions and logos).

Implementation
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellFooter/🟦️.tsx`: new `shellFooterCreditsMenuItem`; `shellFooterCreditsCompact` doc updated.
- `🏛️ShellHost/🖼️canvas-footer/🟠️footer-items/🟦️.tsx`: compact (phone or tablet) pushes the single credits cell, otherwise the existing leading/trailing cells.
- Test `🏛️ShellFooter/🟦️.test.tsx` (3/3): one `<button>`, `aria-label` equal to the captions joined, in en and de.

Live (demonstrator opened pane = OS shell footer with credits):
| viewport | footer row scroll/client | gap probe |
|---|---|---|
| 1440x900 | 1440/1440 | 0 |
| 768x1024 | 768/768 (was 972/768) | 0, also with the credits popover open |
| 375x812 | 375/375 | 0, also with the popover open |

## Full gate (count of violating pairs)

| Scope | 1440 | 768 | 375 |
|---|---|---|---|
| play: intro, landing, opened card, back | 0 | 0 | 0 |
| demonstrator: intro, overview, opened pane, back | 0 | 0 | 0 (opened pane also 0 with popover open at 768 and 375) |
| OS harness, 72 parts at 768 | not rerun (0 before, geometry only affects chips) | 62 parts 0; 10 parts non-zero, exactly the previously classified set: settings-general/theme/keys/defaults/conflicts, display, marketplace, panel-bottom (39 px, floating panel over canvas, class A), layout-bottom (12 px, class B), window-rich-many (39.7 px designed U-gap, Open 3) | not rerun (0 before) |

No horizontal overflow on any probed view (`documentElement.scrollWidth` equals the viewport; the only element beyond the right edge is the overview carousel's own scroller content).

## Notes
- First attempt to read two probes in one blocking `javascript_exec` hung the page (45 s timeout) once, with the credits popover open and a sleep in the same call; reproduced nothing afterwards, split calls (start, then read) were stable.
- Open items 3 and 4 of `round4-final-reprobe.md` (designed U-gap owner decision, focus rings) are unchanged and not part of this task.
- Servers on 5700-5702 killed, caches and logs of this pass deleted; ticket-local tooling reused: `r5-footer.js` (new, footer measurement plus gate), `lv-final-play.js`, `lv-final-demo.js`, `round3-os-run.js`.
