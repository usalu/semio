# Web UI handoff: audit2-web lanes A, B, I, J-1

Paths relative to `🧰️framework/🔨️modules/🖱️ui`.

## Verified false or skipped

- A-5, B-5, B-6, B-7 and the `z-[2]` claim in B-1: false. Compiling the module stylesheet (`@source "../../"`, `@tailwindcss/node` compile) emits `.z-\[1\]`, `.z-\[2\]`, `.z-\[32\]`, `.z-\[40\]`. The WindowChrome comment is stale. Not changed.
- B-2, B-4: low, needs a visual check, `PanelTabBar` is the drag agent's file. Not touched.
- A-4 border: classes cannot carry markers, so the ribbon class lost its drawn border and radius and now documents `data-glass-chip` for its host.
- J-1: false. `createSharedBorderSnap` (WindowChrome/shared-border.ts:500) and the junction resolver and `sharedBorderPaintPoints` (WindowChrome/shared-border.ts:624, 657) are live production code, and `🎯️targets/⚛️react/🧪️tests/🎚️config` includes `snap.test.ts`. Nothing deleted.
- I-10 (`♻️mit-bestand/.../footer.tsx`): product file, other agent. Not touched.
- Kept `rounded-*` only on content shapes: TableAvatar, Skeletons, Table skeleton, host-ports (`rounded-[50%]` ellipse is a real radius), introduction-demo ripple, class-name-composition merge groups.

## Changed

- B-1 `🧱️elements/📑️Tabs/🟦️.tsx`: tab list gets `data-glass-chip=""`.
- B-3 `🧱️elements/🖼️Panel/🟦️.tsx`: all tab rows (root and deeper) sit in the one top chip; the body-stack tab row is gone; folded chip-only keeps `maxRows={1}`. `PanelTabBar` `startDepth` has no consumer left (drag agent's file, left in place).
- A-1/A-2 `🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts`: deleted `borderNormalFrameClass`, `borderEmphasizedFrameClass`, `borderEmphasizedBottomClass`, `borderEmphasizedTopClass`, `shellChromeBorderClass` and their re-exports (`🎯️targets/⚛️react/🟦️.tsx`, `📦️packages/.../📥️source/🟦️.ts`). Menu and ribbon surface classes lost `rounded-md border shadow-sm`.
- A-3/A-11 `📋️canvas-pick`: menu is a compositor frame (`useSharedBorderFrameRef`, `data-shared-border-frame`, `data-level="menu"`).
- A-6 `🔨️modules/🎨️chrome-class-names`: `GLASS_OVERLAY_BOX_CLASS` without `rounded-lg shadow-lg`. Introduction `bodyClassName` without `shadow-lg`.
- A-7 `💡️ChromeControlHint`: tooltip is a borderless glass chip (`data-glass-chip`), no `border shadow-lg`.
- A-8 `📣️display-components`: `Aside` is a `SharedBorderRow` with one cell, no margin and no drawn border (destructive tone for danger).
- A-9/A-10 `🎥️tutorial-ui`: dead radius and shadow removed.
- I-1..I-9, sweep: dead `rounded*` removed in MenuItem, ContextMenu item-render and trigger, PresenceBar, Slider, HistoryTable, Icons, Table skeleton cell kept as is except radius, Dialog close, VirtualFileSystem, fullscreen buttons, status-surface dot, window-chrome measure classes. I-5 host-ports badge gets `data-glass-chip`. I-6 `Badge` is a glass chip without border. I-7 `StatCard` is a `SharedBorderRow` cell.

## Tests

- New `🧱️elements/🪢️SharedBorderRow/frame-law-static.test.ts`: sweeps the ui module for `rounded-*`/`shadow-*` class tokens (allowlist above) and asserts the chrome-class exports draw no border.
- Updated: `📑️Tabs/.../boundary.tsx` (glass chip), `🎨️shell-components-1` (Panel: both rows in the chip, body holds only the tree), `📦️control-chrome-1` and `🧰️harness` (removed `shellChromeBorderClass`), `🎯️tree-helpers-3` (slider thumb has no radius).
- Runner: `web-ui-vitest.sh "<comma list of ui-relative tests>"` (config `web-ui-vitest.config.ts`), through `lockf`.
- Full UI suite (stock config via `compositor-vitest.sh`): 1688 passed, 10 failed, same 10 as the pre-existing baseline in `uitests-handoff.md`: Level Context (2, Navbar/Footer are now `data-level="panel"` chips, other agent), Layout (5), Tree text-flow (3, needs Playwright browser/other), Icons theme alias (1, SVG whitespace). None touch my files.
- `tsc -p tsconfig.json --noEmit` for the ui package: only the 2 existing `Layout/🧪️tests` null errors.
- `ActionGroup/🧪️tests/spacing.test.ts` is a `bun:test` file (no bun here); the Badge class contract tokens it checks are unchanged.

## Open / for others

- OS consumers of `floatingMenuSurfaceClass` (`catalogue-spotlight`, `TiledMapHost`) and r3f `floatingRibbonSurfaceClass` no longer get a drawn border; they need a frame marker or `data-glass-chip` on their host.
- Navbar/Footer stale Level Context tests belong to the owner of the chip-glass change.
