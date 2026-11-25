# Presentation and hub handoff: no gap ring in the clip presentation, hub admin tests, dead card rule

## 1 Clip presentation contract: no gap ring, content beneath the chips, frame tiles the canvas

Round 2 (coordinator): the clip scene is picture content, so the body is the COMPLETE frame and the chips are glass overlays on top; the focus frame is the whole canvas band (no outer inset, no gutter).

Contract now (`🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json`, `🧬️schema.json`):
- `chrome.inset` removed (also the dead `control.inset` of `🖱️interaction`); new required `canvas` rect `0, 29, 1600, 842` (band between navbar and footer).
- `focus.rect = canvas`, `focus.viewport = focus.rect` (was inset 3.2 px frame margin and 3.2 px body inset with the chip row cut off: 6.4, 59.4, 1587.2 x 805.2).
- New `focus.safeClearance { top: 24, right: 0, bottom: 0, left: 0 }` (= `chrome.titleHeight`): the text clearance as a separate value for text-like content and for where scenes may place titles. Scene content is not moved.
- Python geometry (`📐️geometry/🐍️.py`, schema): `inset` metric removed; `window_geometry().body` is the complete frame; `safe_clearances` (already there) carries the chip row depth; chip text padding is the spacing token.
- Browser pane (`🌐️browser/🟦️.tsx`): body `position:absolute; inset:0` (no `--window-frame-top-clearance`), `band()` and the window top read `contract.canvas`.
- Hierarchy renderer (`🥞️hierarchy/🖌️render/🐍️.py`): `_camera` and `_nodes` read `contract.canvas` / `contract.viewport`; no 1600, 900, nav or footer maths left.
- The overview grid was already gapless in the real UI (`.course-home-grid` in the course `chrome-and-home` CSS has no gap and no padding, cards fill cells); the stale `grid.outerInset` and `grid.gap` (3.2) cases are deleted, and a test now guards the rule.

Tests on both sides:
- Python cases and tests: `🧪️tests/🔣️.cases.json` (canvas, focus, safeClearance), `📐️geometry/🧪️tests/🔣️.cases.json` (`window.body` = frame, `underChipProbes` replace `floorPixel`) and `🐍️.py` (body equals frame, no `inset` metric, focus equals canvas; real-browser parity test now also asserts that the pixels beneath the top chip row and at the left frame edge equal the plain body pixels), `🧪️tests/🐍️.py` (`test_frame_content_runs_beneath_the_chips_and_the_frame_tiles_the_canvas`).
- TypeScript vitest `🌐️browser/🧪️tests/🪟️frame-body/🟦️.ts` + `🔣️.json` (Ajv 2020 schema validation as the third-party oracle, rejects an `inset`, canvas/focus/safeClearance derivation, gapless grid CSS, pane source has `inset:0` and no 3.2 or 900): 4 tests OK, `presentation-hub-vitest.config.ts` under `lockf -k /private/tmp/claude-501/semio-vitest.lock`.
- Python run (`presentation-hub-py.sh`, course env with the output root inside the workspace because the browser render refuses work dirs outside it; under the manim lock): first run of the final state 53 tests, 49 OK; the 4 errors are all `Presentation UI sources changed during handoff capture`, i.e. other agents edited ui sources during the capture, not assertions. Those 4 were rerun alone in two runs (3 + 1) under the same lock: all 4 OK. Net: all 53 pass; no assertion failed. A complete run before round 2 (inset removed, viewport = rect minus chip row) was 53 OK.

### Do published clips and posters depend on the viewport numbers?
- Posters (`🖼️posters`): no, Manim frames of the scene that never read `focus.*`. Their placement inside the opened pane changes (body box 1587.2 x 805.2 at 6.4, 59.4 becomes 1600 x 842 at 0, 29).
- Clips: one dependency, `🖱️interaction/🎬️scene/🐍️.py` `cursor_unit = frame_height / focus.viewport.height` for authored cursor clicks inside a scene: 8 / 805.2 becomes 8 / 842, so the cursor in a re-rendered clip is 4.4 % smaller. Nothing else stored reads the numbers. Open/restore transition frames come from the browser pane and change by the body box above. Clips are re-rendered from source (not regenerated here); the scene file reads `🔣️.json` directly, the presentation `source_files()` includes it.

### Stills (looked at)
- Python: `evidence/presentation-hub/python-focus-frame.png` (`presentation-hub-still.py`, `presentation-hub-run.sh still`): canvas band, frame, chips over the body, body and red viewport outline coincide with the frame on all four sides.
- Browser: `evidence/presentation-hub/browser-focus.png` (`presentation-hub-browser-still.py`, `presentation-hub-run.sh browser`, static render of the real pane): a red-bordered body image fills the frame from navbar line to footer line and from edge to edge; the glass title chip sits over its top-left corner. Measured: window rect and content rect both `0, 29, 1600, 842`.

## 2 Hub admin vitest

Command: `node node_modules/vitest/vitest.mjs run --config 🌎️hub/🔨️modules/🛡️admin/🧪️tests/🎚️config/🟦️.ts --configLoader native` from `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript`.
- `hands table action cells` (ownFrames 1 vs 0): root cause is a stale test. The test (commit 748) asserted that no framed element lives anywhere in `tbody`. Commit 751 made every shared ui Table row (and the header) a compositor band frame (`tableBandBorder`: edges `bottom`, delegate `self`), which is the intended single-line row seam, not a second frame inside the owner. The test's purpose is that the action cell holds flush groups, no inset framed buttons. Fixed: it now asserts `ownFrames(cell)` is empty and that every row frame carries exactly the `bottom` edge. Product unchanged.
- `command-routing`: passes in the same run (Test Files 3 passed, 29 tests; command-routing included). I could not reproduce a failure; the earlier failure was most likely a run against a half-edited tree. Not changed.

### Table rows as compositor cells (D-4): done
Rows and header already were compositor band frames (`tableBandBorder`: edges `bottom`, delegate `self`); what was hand-drawn was the native `border-b` (double paint on the same pixel row). Changes in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🟦️.tsx`: `borderNormalBottomClass` removed from `thead` and rows, the skeleton table header and rows are band frames too, band cells take the seam pixel as padding (`py` of half a hairline on `td`, the header `th` bottom padding grows by half a hairline) so the collapsed border's space stays; in `🎨️styling/🖌️ui/🎨️.css` the framed last row drops its cell padding (`tr[data-shared-border-edges="bottom"]:last-child > td { padding-block: 0 }`, the old `border-bottom-width: 0` equivalent). The generic `table tbody tr` colour rule stays (other tables, e.g. HistoryTable, still use native lines).
- Test first: `📊️Table/🧪️tests/🧩️row-seams/🟦️.tsx` (4 tests: no `border-b`, edges bottom and delegate self, cells hold the hairline, skeleton bands, last-row rule) was red 4 of 4 before the change, green after. Related: row-menu, drag, `shared-border.test.ts`, OverviewCard: 71 OK.
- Pixel acceptance (`table-rows-measure.mjs`, Storybook dev server on my own port 6191, Playwright Chromium, stories WithAvatars, SelectableWithAvatars, CompactWithAvatars, before and after): header 30.875 px, every row 25 px, row top positions and every compositor seam path (`M 1 24 L 1135 24`, header `M 1 29 L 1135 29`) identical; screenshots differ by at most 1 colour level on the header seam row (before: native line plus overlay on the same pixels). Only delta: the table box is 0.5 px shorter (106.375 to 105.875), the half border a collapsed table adds below its last row; no painted pixel moves. Server stopped by pid.
- Not touched: HistoryTable and other tables with their own native lines, the empty-state row (never had a seam).

## 3 Dead card rule

Deleted the `:is([data-card-stack], [data-card-grid], [data-layered-overview]) [data-overview-card-paint] { translate: none; transform: none }` rule and its comment in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`, plus its test ("never lifts a card tiled in a stack...") and the two unused imports in `🧱️elements/🃏️OverviewCard/🧪️tests/🧩️component/🟦️.tsx`. Nothing lifts the paint node any more (no `translate`/`transition-transform` class on it, asserted by the kept test in the same file and by the presentation hover test). OverviewCard tests: 5 OK (`web-ui-vitest.sh`).

## Open
- Table: stills in `evidence/presentation-hub/table/` (before and after, WithAvatars).
- Hub admin `command-routing` and the other suites ran once in the sandbox config only; not run through `repo test`.
- Not run: Rust, Playwright acceptance, a full clip re-render (instructed).
