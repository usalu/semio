# 🧱 Finish Shared Border Consolidation Across UI

Continues `26/11/05/CONSOLIDATE-NEIGHBORING-UI-BORDERS`. Goal: every seam between neighbouring bordered things is one hairline with one owner — no doubled lines, no box-in-box, no gap-separated bordered siblings.

## Framework (🖱️ui)

- **Navbar / Footer** ([navbar-footer.md](navbar-footer.md)): band items span the full band height so control edges coincide with the band edge; the compositor suppresses a frame side lying on an undrawn parent edge (`flushWithUndrawnParentEdge`). Verified live: one bottom line, painted as disjoint segments by left group / band / language group.
- **Nested cells** ([nested-cells.md](nested-cells.md), [primitive-gaps.md](primitive-gaps.md)): `data-shared-border-cells` owner turns direct-child framed controls (Button, Input, Textarea, Select trigger, Toggle, ToggleGroup, ActionGroup) into cells; compositor skips their frames. New `SharedBorderRow` / `SharedBorderRowCell` / `SharedBorderRowLabel` / `SharedBorderRowField` (`as`, refs, `tone="destructive"`, native field cells), `Input`/`Textarea` `rootClassName`/`rootStyle`, `useSharedBorderFrameRef` exported. OverviewCard `flush`.
- **Element seams** ([ui-elements-fixes.md](ui-elements-fixes.md), [panel-tab-bar.md](panel-tab-bar.md)): mobile Layout panel frame, Canvas mode nav, HistoryTable rows, Table last row, dead CSS; PanelTabBar on a non-scrolling compositor frame declaring only its content-facing edge.
- **New elements** ([sites-fixes.md](sites-fixes.md)): `LanguageChoice` (ToggleGroup radio, no default language), hub `StatTiles`.

## Products

- **Course / Architektur und Technologie** ([course.md](course.md)): quiz page, home cards (leaderboard toggles, badge strip), run head + steps, every task type (classification pool/categories as one group, selects/inputs/buttons as trailing row cells), results, badges, identity, loading. Verified live with a DOM overlap probe on all quizzes.
- **Presentation, CAD** ([presentation-plugins-fixes.md](presentation-plugins-fixes.md), [os-rows-b.md](os-rows-b.md)): PDF pager and reset/enlarge as shared groups; REPL aside compositor frame; REPL Select triggers without triple border.
- **OS** ([os-rows-a.md](os-rows-a.md), [os-rows-b.md](os-rows-b.md), [primitive-gaps.md](primitive-gaps.md)): SpaceBrowser, AgentDelegations, ShellHost notice bands, hub rows (`HubFieldRow`, `HubRowList`…), HubSignIn, Paint2dHost panel, staged-args, NodeGraph, SpaceAdministration, ChromePanels, InkCanvasHost, BlockListHost, TextEditor, block palette.
- **Teaching, hub, semio-tech, mit-bestand** ([teaching-os-pets-fixes.md](teaching-os-pets-fixes.md), [sites-fixes.md](sites-fixes.md)).

## Audits

[audit-ui-elements.md](audit-ui-elements.md), [audit-products.md](audit-products.md), [audit-nested-controls.md](audit-nested-controls.md), [adhoc-rows-checklist.md](adhoc-rows-checklist.md), [audit-sites.md](audit-sites.md).

## Unrelated fixes

Stale tests repaired: OverviewCard lift assertions, legacy `owned-locale-detector-retirement` aggregator (8, now all green) ([legacy-suite-fixes.md](legacy-suite-fixes.md)), Tree focus retention ([tree-suite-fixes.md](tree-suite-fixes.md)); incomplete `ResizeObserver` test mocks without `unobserve` ([resize-observer-mocks.md](resize-observer-mocks.md)).

## Verification

- 🖱️ui regression set incl. legacy aggregator: 608 passed / 0 failed; course suite 1123/1123; presentation 140/140; os-rows-b 139/139; os SpaceBrowser/AgentDelegations/ShellHost suites green.
- Live (course dev server): navbar, home, quiz page, all task types.
- Not verified: browser-only fixtures (Playwright Chromium not installed on this machine); `bun:test` suites; visual check of OS/presentation/hub (no running hosts).

## Tooling notes

`ticket_open`/`ticket_reopen` bound to another session's ticket and `repo run` failed (`BUILD_MEMBERSHIP_CHANGED`, `s.busy`), so this ticket was created by hand and tests ran through ticket-local vitest configs (`vitest-*.config.ts`); the course dev server ran natively via `vite-preview.config.ts` (`SEMIO_BUILD_MODE=prototype`, `SEMIO_VITE_CACHE_DIRECTORY` under `.cache/b/p/m`).

## Conventions

- Shared seams: use `SharedBorderRow` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪢️SharedBorderRow`) or `data-shared-border-cells` owners; controls placed as direct children become cells. Never wrap a cell control in an extra element (it falls back to a floating frame).
- A row that scrolls must hold its scroller inside it, never be the scroller itself (compositor overlay would scroll).
- Test `ResizeObserver` mocks must implement `unobserve` (the compositor calls it).

## Round 2 — the radical rule

User feedback: cards, nested elements and drop zones were still individual boxes with gaps. New rule: no bordered element may be separated from another by a gap or sit inset inside another; every border run is one shared line. Enforced live with [border-gap-probe.js](border-gap-probe.js) (parallel near-but-not-coincident edges, coincident double CSS borders, overlaps; clips to scroll ancestors; skips behind-glass pages, inert content and cross-modal pairs; reports `pending` while the compositor has not committed). Gate: count 0 at 1440x900 and 375x812.

- **Framework** ([round2-framework-stacks.md](round2-framework-stacks.md), [round2-framework-followup.md](round2-framework-followup.md)): `CardStack`, `CardGrid`, `CardGridCell`, `useCardInterlock` — stacked cards interlock (next title chip on the previous bottom line, footer chip shares the band), grids tile edge to edge, first card on the navbar line; swipe hints as full bars; no hover lift in tiled grids; compositor snap within 0.75px; `navbarTrailingItems` shell prop.
- **Glass and pixels** ([round2-glass-backdrop.md](round2-glass-backdrop.md), [round2-snapped-heights.md](round2-snapped-heights.md), [round2-visual-375.md](round2-visual-375.md)): frosted glass kept, backdrop pages paint no borders; bar heights snapped to device pixels with one source of truth (`--navbar-band-height`, `--footer-band-height`); stale disposed compositor commit fixed; navbar captions icon-only below 768px so the brand never overlaps controls.
- **Elements** ([round2-ui-static-fixes.md](round2-ui-static-fixes.md)): Canvas nav, PresenceBar, UIDialog, Dialog, Popover, Window controls, Tree, Tabs, dock preview, intro chips, HistoryTable, ContextMenu, Skeletons; `Field` on `SharedBorderRow`.
- **Apps**: course ([round2-course.md](round2-course.md), [round2-course-final.md](round2-course-final.md)) — categories as one group, every card body flush, selects/inputs/buttons as cells, run actions as one footer chip group; staffage ([round2-staffage.md](round2-staffage.md)); mit-bestand demonstrator ([round2-demonstrator.md](round2-demonstrator.md)); presentation, hub admin, semio-tech play ([round2-presentation-hub-site.md](round2-presentation-hub-site.md)); s/OS ([round2-os.md](round2-os.md), [round2-os-chrome.md](round2-os-chrome.md), [round2-os-final.md](round2-os-final.md)) incl. one-pane-at-a-time on narrow viewports; verification fixes ([round2-verification-fixes.md](round2-verification-fixes.md)).
- **Native** ([round2-native.md](round2-native.md), [round2-native-tests.md](round2-native-tests.md)): Rust Field metrics as cells (fixture re-measured in the browser), PresenceBar/Tabs without gaps, `bands.rs` consuming the shared band cases; wgpu test crates adapted to the new job API.
- **Tests/types** ([round2-tests-types.md](round2-tests-types.md)): Interpreter suites collected (195/195), staffage fixture path fixed, type errors from new elements fixed.
- **Verification** ([round2-final-verification.md](round2-final-verification.md), [round2-final-verification-2.md](round2-final-verification-2.md)): independent re-probe of all apps; defects found there fixed and re-probed.

## Rounds 3 and 4: root causes, native parity, final gate

Round 3 status and audits: [round3-status.md](round3-status.md), [round3-audit-framework.md](round3-audit-framework.md), [round3-audit-plugins.md](round3-audit-plugins.md), [round3-audit-sites.md](round3-audit-sites.md), [round3-audit-os-products.md](round3-audit-os-products.md), [round3-audit-native.md](round3-audit-native.md); round 4 regression audit [round4-audit-regressions.md](round4-audit-regressions.md) with verdicts in [round4-audit-fixes.md](round4-audit-fixes.md).

- **Framework** ([round3-framework-fixes.md](round3-framework-fixes.md), [round3-elements.md](round3-elements.md)): `data-shared-border-edges` drives native border sides; Popover/Select/Toggle dock flush; `--chip-row-height` token for title/footer chip rows and panel headers (chip rows pinned to the navbar/footer band); scroll bodies without scrollbar gutter; `SharedBorderRow`/`ToggleGroup` wrap; `--shared-border-flow` moves a vertical group's divider to the inline end (replaces hand-rolled course dividers); overlay svgs placed on the snapped corner.
- **Card interlock** ([round4-course-live.md](round4-course-live.md), [round4-final-reprobe.md](round4-final-reprobe.md)): lift is inset-aware (`resolveCardBands(cards, inset)` / Rust `resolve_card_bands(cards, inset)`, one function), card heights snap to device pixels in batched phases, the lift absorbs clipping drift and seams are re-read until every overlap equals the band. Single source of truth `🃏️CardStack/bands.cases.json`.
- **Chips** ([round4-last-defects.md](round4-last-defects.md), [round4-chips-focus.md](round4-chips-focus.md), [round4-last-gate.md](round4-last-gate.md)): a chip within two insets of a card side reaches it (`snapWindowSilhouetteChips`, `chip-span.cases.json`); chips of one row (also across a window's anchored panes) closer than `CARD_BAND_MIN_GAP` (40 px) abut (`chip-abut.ts`, `bands.cases.json` `gaps`, Rust + TUI mirrors).
- **Focus** ([round4-chips-focus.md](round4-chips-focus.md), [round4-focus-close.md](round4-focus-close.md)): keyboard focus on shared-border cells recolours the seam (`@layer shared-border-priority`, `chip-focus.ts`) instead of rings/outlines; accent-painted cells add a label underline, active-window chips a 2 px line (layer order, no `!important`); forced-colours `Highlight` variants; contrast 3.22–5.86:1. Guard `focus-seam.cases.json`.
- **Apps**: course ([round3-course.md](round3-course.md), [round4-course-live.md](round4-course-live.md)), staffage ([round3-staffage-fixes.md](round3-staffage-fixes.md)), OS ([round3-os-fixes.md](round3-os-fixes.md); shell footer credits fold into one popover cell on phone/tablet), plugins CAD/GIS ([round3-plugins-fixes.md](round3-plugins-fixes.md)), hub admin, demonstrator intro docking, play, presentation ([round3-apps-open-defects.md](round3-apps-open-defects.md)).
- **Native** ([round3-native-fixes.md](round3-native-fixes.md), [round4-native.md](round4-native.md), [round4-card-roles-conformance.md](round4-card-roles-conformance.md)): kit Field/Section/stacks gapless, `nativeSeams` in Rust, TUI `├┤` junctions, native CardStack layout/paint wired into retained `CardStack`/`OverviewCard` nodes (container roles `cardStack`/`card`, schema-first, React Interpreter mapping, conformance case `🗂️card-stack`, accessibility rows); border resolver split into its own crate so `semio-ui-draw-contract` is under its line boundary.

### Unrelated fundamental fixes
Course proctor revive loop starved the event loop during outages (paced revive, [round4-course-journey-hang.md](round4-course-journey-hang.md)); paged draw list reservation (`whole_page_capacity`); TUI focus target within the window; Diagram suite harness order ([round3-diagram-force.md](round3-diagram-force.md)); stale ContextMenu/TUI assertions and an unbounded test sort loop ([round4-ui-tests.md](round4-ui-tests.md)); Rust arena fixtures (see below).

### Verification
- Live gate (in-app browser, pane fronted, only `pending:false`): border-gap probe count 0 at 1440x900, 768x1024 and 375x812 for elements (126 stories), OS harness (72 parts), CAD harness, staffage, demonstrator, play, presentation, hub admin, course en/de incl. all quizzes and levels; false-positive classes A–H documented in [round4-final-reprobe.md](round4-final-reprobe.md).
- Tests: course 1135/1135, OS 158/158, interpreter 196/196, CardStack/WindowChrome/Window/Tabs/OverviewCard 127/127, Rust light/paint-laws/hosting/tui-laws/tui-unit-laws/draw-contract/layout/mounted-layout/widgets green.
- Not verified: Playwright browser tests (Chromium not installed; install needs approval), forced-colours rendering (not emulatable in the browser tools).
- Rust arena ([round4-rust-arena.md](round4-rust-arena.md)): central `UiFixture` (`semio_ui_wgpu_engine::testkit`) owns a `Ui` with its arena scope and pays the window close on drop; plain document/patch laws open `UiArenaFixture` per test. `semio-ui-retained-tree-tree` 21/21, hosting 44/44, wgpu-laws 36/45, contract-conformance 38/43; `original_clipboard_ingress` ported to `WorkerJobSessionAdmission` (compiles, 0/2 at runtime).
- Open, outside this ticket: stale size-derived conformance fixtures (5), 9 wgpu-laws paint/layout laws, clipboard admission laws, and crates that no longer compile after API removals (`semio-ui-wgpu-engine-laws`, `engine-scene-laws`, `layout-laws`, `raster`, `semio-ui-locale`, `text-splice-native`); about 55 UI crates not swept.
