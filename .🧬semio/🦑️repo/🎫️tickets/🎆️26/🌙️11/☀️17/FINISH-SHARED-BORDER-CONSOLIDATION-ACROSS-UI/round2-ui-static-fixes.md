# Round 2 UI static audit: fixes

Source: `round2-audit-ui-static.md` (16 findings). Paths relative to `🧰️framework/🔨️modules/🖱️ui/` unless noted. Mechanism: `SharedBorderRow`/cells, `data-shared-border-cells`/`-group`, compositor frames.

| # | Finding | Fix |
|---|---|---|
| F1 | Canvas app-mode nav | `🎨️Canvas`: nav frame is now `data-shared-border-cells`, `items-stretch`, no padding/gap; the Select trigger is a stretched cell whose bottom edge is the band edge. |
| F2 | Window controls | `🪟️Window`: wrapper `data-slot="window-controls"` is a gapless `data-shared-border-group` + cells (no `gap-standard`); overlay anchored `top-0 right-0` (was `top-1 right-1` + gap). |
| F3 | PresenceBar gaps | `👥️PresenceBar`: roster is a `SharedBorderRow` (framed, `w-fit`); each peer / overflow chip is a `SharedBorderRowCell` (no `rounded-full`, no border). Avatar `border-0`, identity ring is `box-shadow` (not a border); badges lost their border and sit inside the cell (box-shadow ring). Fixture `📻️TableAvatar/🧪️tests/🧫️fixtures/📐️spacing` `rosterGapClass` is now `null`. Rust twin (`🎯️targets/🧊️wgpu`) not touched. |
| F4 | UIDialog arg stack | `📨️UIDialog`: frame `p-0 gap-0`; title/body/notice in a padded text block; args are a vertical `SharedBorderRow` (top+bottom edges) of nested rows (label = `SharedBorderRowCell`, field a direct child cell); footer is a `SharedBorderRow edges=["top"]` with Cancel / choice groups (nested vertical rows, description cell) / Submit as cells, no `mb-*`/`gap-*`. |
| F5 | Dialog padding/gap | `💬️Dialog`: content frame `p-0 gap-0`; `DialogHeader` carries the text padding; `DialogFooter` is a `SharedBorderRow` (top edge) so its buttons are cells attached to the frame. Consumer `🌎️hub/…/🏛️SpacesPage` create dialog migrated to a vertical `SharedBorderRow` with `SharedBorderRowLabel` + Input/Select cells (hub-admin suite 23/23 green). |
| F6 | Popover padding | `🗨️Popover`: content is `p-0`, `data-shared-border-cells`, `data-orientation="vertical"`, `flex flex-col` (live probe caught a double right edge when it was still block layout). Interpreter conflict popover text got `p-single` (`🧰️framework/🛍️products/💻️os/…/🗣️Interpreter/✍️input-views`). `🔄️ShellSync` popover (product, own `p-3` + inset framed inputs) not touched. |
| F7 | Tree file card | `🌳️Tree` `TreeFiles`: no `my-medium`, `p-medium`, `rounded-lg`; padding on title and list wrappers; `data-slot="tree-files"` + CSS `tree-files + tree-files {border-top-width:0}` for one seam (live probe found the 1px double before this). |
| F8 | Value slot / actions | Verified: label-to-control gaps (`window-measures`, `control-tree`, tree grid) separate unframed text from one control: unchanged. Real violation: `tree-item-control` (value + `gap-double` + actions) and `tree-header-actions` (`gap-single` between bordered `Action`s). Both are now `data-shared-border-group` + cells without gaps; header actions are a cell of the control. `DisabledReasonHint` gained `bare` (clones handlers/ref onto the child instead of a wrapper span) so actions stay direct children while the element identity test (`Row action identity`) stays green. |
| F9 | Tabs | `📑️Tabs`: root `gap-0`; list `p-0 items-stretch`; trigger `h-[calc(100%-hairline)]` workaround removed so the active (framed) chip is flush with the list. Active-chip-only frame ownership (feature `boundary.feature`) kept. |
| F12 | Intro callout chips | `🎨️styling/🖌️ui/🎨️.css`: `.introduction-demo-callout-chips` `gap:0`; chips overlap by one hairline (`margin-block-start: -hairline`), i.e. one shared line. Deliberate exception to the cell mechanism: chips toggle `display:none` through data attributes, so `:last-child` cell dividers would be wrong. |
| F13 | Dock insert preview | `🎨️Canvas/🧲️drag-preview`: no margins, no `border-2`/rounding, `h-full`; `data-shared-border-cell` + `data-shared-border-state="active"` (accent seam via priority layer) on the preview in `⚓️dock`. |
| F14 | Menu swatch | `🖱️ContextMenu/🎨️item-render`: border box replaced by an inset hairline `box-shadow` (colour content, not chrome). |
| F15 | Avatar stack | `🕰️HistoryTable`: overlapping avatars `border-0 ring-2 ring-background` (no double arc). |
| F17 | Scroll-area hairline padding | Verified, no change: the hairline is exactly the compositor stroke clearance of the frame (scrollbars/content must not paint under it); it is not an inset gap. |
| F19 | Skeletons | `🦴️Skeletons`: `PanelTreeSkeleton` rows (which were bordered loading rows with `gap-single` + `p-single`) are a vertical `SharedBorderRow` of `SharedBorderRowCell` (loading seam state); `field`/`group`/`section` placeholders are vertical cell stacks. `loadingBorderClass` import removed. |
| F21 | ToggleGroup action `ms-single` | Verified, no change (spacing inside one framed item, action slot is unframed). |
| F10/F11 | Field / dashed drop zone | Checked in audit: no finding; no change. |

## Tests

New/extended jsdom tests: `📨️UIDialog` (cells stack), `💬️Dialog/🔹️shared-border-padding`, `🗨️Popover/🔹️shared-border-cells`, `👥️PresenceBar` (updated), `🎨️Canvas/🧪️tests/app-mode-nav.test.tsx`, `📑️Tabs/boundary.tsx`, `🪟️Window` (controls), `🦴️Skeletons`, `🌳️Tree` ("Tree shared-border consolidation"), `🪢️SharedBorderRow/round2-static.test.ts` (CSS rules, dock preview class, swatch, avatar stack).

Regression run (`vitest-ui.config.ts`): Canvas, PresenceBar, UIDialog, Dialog, Popover, Tree, Tabs, Window, Skeletons, ContextMenu, HistoryTable, ChromeControlHint, Command, TableAvatar, SharedBorderRow, `🧪️tests/🧪️owned-locale-detector-retirement`: 773 passed, 4 failed (46 "No test suite found" children are the expected aggregator pattern).
- 3 failures: Tree `🔤️text-flow` need Playwright chromium (env, pre-existing).
- 1 failure: `🪟️uiintroduction-appearance-1` "one borderless dialog-level glass silhouette" (UIIntroduction/WindowChrome chip classes). First full run (before WindowChrome changes landed) was 3 failed only; WindowChrome files are being changed concurrently by another agent, so this is outside my edits (I touched neither WindowChrome nor UIIntroduction). Please re-run after that agent finishes.
- Hub admin suite (`vitest-hub-admin.config.ts`): 23/23 passed.

## Live probe

Course dev server (5391) does not render these elements, so a ticket-local showcase `ui-static-live.tsx` (mounts PresenceBar, Tabs, Canvas `App` nav, skeletons, Popover with Input+Button, Dialog+Footer, UIDialog with two framed Input args, two stacked `Tree.Files`) was imported in my own tab and `border-gap-probe.js` run per part: count **0 for every part at 1440x900 and 375x812** (after fixing two real hits the probe found: popover right-edge double, tree card seam). Not covered live (not reproducible in the showcase): Window controls, tree item value/actions row, intro chips, dock insert preview, ContextMenu swatch, HistoryTable avatars (jsdom/CSS tests only). The course page itself still has 39 pre-existing hits (course card stack, tables, navbar) owned by other agents.

No `data-border-probe-art` markers added.
