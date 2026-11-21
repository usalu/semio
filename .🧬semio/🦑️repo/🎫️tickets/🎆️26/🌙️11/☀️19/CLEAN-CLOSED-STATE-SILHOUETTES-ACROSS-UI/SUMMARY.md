# 🫧 Clean Closed State Silhouettes Across UI: Summary

Goal: every hover, focus, selected, pressed, open, invalid, loading and celebrated silhouette is one clean, closed outline on every element and in every UI, built on the shared-border system.

## Mechanism ([design.md](design.md))

- **Boxes.** Windows, window chips, frames, cells and borderless boxes are all treated as boxes.
  - Cells are painted by the nearest owner overlay; there are no CSS divider borders any more.
  - Void elements (`input`, `select`, `img`…) are painted by the nearest ancestor that can hold an overlay.
  - Borderless controls (`sharedBorderlessBoxProps` / `useSharedBorderBox`, re-exported via `@semio-tech/ui-react/chrome`) draw nothing at rest and their closed silhouette when stated.
- **State ownership** (`resolveBoxStates`, `states.cases.json`):
  - Hover goes to the innermost interactive box; focus to the innermost interactive box, and only on `:focus-visible`.
  - Markers stay on their own box. Ancestors never inherit hover or focus.
  - Priority: normal < active < invalid < waiting < loading < introduced < celebrated < focus < hover.
- **Edges** (`boxSilhouetteSegments`, `silhouettes.cases.json`). Stated boxes always contribute all four sides. Coincident lines resolve to one line through the existing resolver, with one paint convention: sides flush with an ancestor edge snap onto that edge.
- **One painter.** The compositor is the only state-line painter.
  - State borders, rings, outlines and the browser focus outline (`[data-shared-border-root] :focus-visible`) are removed. Fills stay.
  - Forced colours map overlay strokes to `Highlight` / `CanvasText` / `GrayText`.
- **Compositor runtime:**
  - It attaches automatically wherever a box appears.
  - Its overlays are created lazily.
  - A stated box is never clipped by its own owner.
  - It ignores mutations that cannot affect boxes and keeps cached box lists.
  - It re-measures on every state commit.
- **Native.** Rust `resolve_box_states` / `box_silhouette_segments` consume the same JSON cases.
  - wgpu: a `HOVER_OWNER` flag; the selected ring sits on the box's own seams.
  - TUI: chips close their seam in their state colour.

Details per round: [web-mechanism.md](web-mechanism.md), [native-mechanism.md](native-mechanism.md), [sweep-elements.md](sweep-elements.md), [sweep-products.md](sweep-products.md), [cell-overflow.md](cell-overflow.md), [os-fixes.md](os-fixes.md), [perf-fixes.md](perf-fixes.md). Audits: [audit-state-mechanisms.md](audit-state-mechanisms.md), [inventory-state-lines.md](inventory-state-lines.md).

## Unrelated fundamental fixes

- **Cells overflowing their owner.** Control cells overflowed bordered owners by 2 px (`chromeControlItemBaseClass` min height, Stepper, course touch targets in the navbar), so their bottom lines were clipped. Cells now stretch to their owner.
- **CardStack interlock.** It reset and rewrote styles in a loop. It now writes only changed values and skips settled layouts, and its MutationObserver ignores changes inside cards.
- **LayeredOverview.** Its overview button was a cell without an owner.
- **ToggleGroup.** The root's `data-state` marked the whole group active.
- **Dock test crate.** It did not compile (`split_axis_extent(&[])`).
- **Staffage clock.** Ambient previews share a drawing queue and evaluate scenes in a module worker. The interactive clock still delivers foreground controls synchronously. [Implementation and regression coverage](staffage-followup.md)

## Verification

- **Live probe** [silhouette-probe.js](silhouette-probe.js), run with `sweep-boot.js` / `app-sweep.js` / `os-sweep.js`. It uses real `:focus-visible` modality and synthetic hover, and checks closure, ancestor leaks and CSS state lines.
- **Elements story site:** 483 failing checks at [baseline](live-baseline.md), **0 / 354 targets** in the final run, repeated after the last compositor change.
- **OS showcase**, 72 parts at 1440: **0 / 548**.
- **Course:**
  - en at 1440: 0 / 28
  - en at 919: 0 / 26
  - en at 768: 0 / 27
  - en at 375: 0 / 13 after the fix
  - de at 375: 0 / 12
- **Other apps at 1440:**
  - demonstrator: 0 / 15
  - play: 0 / 50, and 0 / 7 at 375
  - CAD: 0 / 40
  - hub admin: 0 / 10
- **Vitest:** UI 725 passed. The 4 failures need Playwright Chromium. Product suites are in [verification-tests.md](verification-tests.md): course 1155, OS final 160, interpreter 196/197 (others' corpus change), demonstrator 29, presentation 197, teaching 279/280 (catalog content).
- **Rust** (before the last priority change):
  - border-presentation 13
  - draw-contract 23
  - wgpu paint 72
  - wgpu events 90
  - tui unit 100
  - tui laws 102

## November 20 continuation

The original ticket was reopened from the workspace. The MCP reopen operation resolved the ticket relative to the filesystem root and failed. The repository CLI reopened the existing record.

The foreground staffage profile used a 1440 × 900 content viewport and native animation frames. Before the follow-up fixes it delivered five frames in 18.45 s, with a 3,091 ms median frame interval. All five samples had pending border work. The inert all-assets preview mounted the entire archive behind the overview. It now mounts cards as their fixed-height cells intersect the preview scrollport. An opened archive still mounts the full filtered result.

Grid-cell style mutations fell from 1,622 to 35 following the archive fix. Moving ambient drawing to a worker reduced the median frame interval to 88 ms. Closed detail panes still mounted running editors and rewrote their playback controls. They now retain the asset preview and facts until opened. Closed categories defer their full load, and featured requests survive unrelated category updates.

The final visible 1440 × 900 sample retained 41 drawings and delivered 550 native frames in 15.05 s. The median interval was 27 ms and the 95th percentile was 46 ms. The compositor spent 114 ms committing across the sample. The pending marker cleared in 545 of 550 frames. The clock callback occupied 5 ms. [Foreground measurements](foreground-profile.md) · [Staffage implementation and tests](staffage-followup.md)

The compositor now coalesces writes to its pending marker, ignores completion of untracked CSS transitions, and preserves the structural cache when window geometry changes. Moving an unchanged-size frame updates its position without rewriting its SVG dimensions. Changing a React control's text reattaches its existing overlay on the next commit. [Foreground harness](profile.html) · [App and locale coverage](matrix-audit.md)

The full Staffage React suite passed 289 of 290 tests. The authored archive check reached its 120-second limit while another ticket was editing its geometry. The focused Staffage run passed all 61 tests, and the package type check passed. [Staffage verification](staffage-followup.md) · [Authored catalog findings](catalog-followup.md)

The final in-app browser matrix passed all 48 cells across Elements, OS, course, demonstrator, Play, CAD, Hub Admin and Staffage in English and German at widths 1440, 768 and 375. It examined 6,331 targets and recorded 6,286 focus-visible checks with zero findings. The final Staffage English tablet sweep passed 35 targets, and the final mobile OS sweep passed 454 targets across all 72 parts. [Live results](live-final.md)

The compositor now uses vertically overlapping window edges when snapping a chip's vertical side. All 86 tests in the four matching shared-border Vitest files passed, including a regression for snapping to a nearby edge that ends above the chip. The browser diagnostics showed that five mobile OS stories had closed Window Options outlines whose left stroke sat 1.97 px inward from the DOM edge. The probe now verifies the joins at the painted corners within the compositor's snap range. All three replay cases passed, including the missing-side and disconnected-side negatives.

The last OS retry used the existing showcase source and config with a native Vite process backed by an isolated dependency install. Reads through the repository's mounted caches were hanging. The ticket retained the recovery inputs and the host's original images and workspace links. [Launcher diagnosis](launcher-diagnosis.md)

The requested UI and product `bun:test` suites and Playwright Chromium tests were not run. A separate focused repository disk-admission test did run under Bun. No browser was installed.

All seven Rust suites passed, 472 tests in total. Border presentation passed 14, draw contract 23, wgpu paint 73, wgpu events 91, TUI unit 100, TUI companion 102 and Dock 69. Both priority laws passed. Two Dock tests now find their shared fixture from the repository root. A third uses the geometry oracle's 0.001 px tolerance. The 96 GB fixed Rust image was preserved. [Native verification and disk recovery](native-rerun.md)
