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
