# Nested controls become cells (shared-border owner mechanism)

## API

Import (both settled): `import { SharedBorderRow, SharedBorderRowCell } from "@semio-tech/ui-react"` (types `SharedBorderRowProps`, `SharedBorderRowOrientation`, `SharedBorderRowEdge` too, also in the `ui-react/source` barrel), or from the ui package `@semio-tech/ui/elements/SharedBorderRow`. Element file: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪢️SharedBorderRow/🟦️.tsx`.

```tsx
<SharedBorderRow orientation="horizontal|vertical" framed={true} edges={["top","right","bottom","left"]} className=...>
  <Button .../> <Input .../> <Select><SelectTrigger/>...</Select> <Textarea/> <Toggle/> <ToggleGroup/> <ActionGroup/>
  <SharedBorderRowCell>plain text / non-control content</SharedBorderRowCell>
</SharedBorderRow>
```

- Replaces `rounded-sm border px-single py-1` rows/cards. Remove the native `border`, `rounded*`, `p-*`/`px-*`/`gap-*` of the old container: the row paints the edges (compositor), no padding.
- `SharedBorderRow` renders `div[data-slot="shared-border-row"][data-shared-border-cells][data-shared-border-frame][data-shared-border-edges][data-orientation]` (flex, `items-stretch`, `flex-col` when vertical). `framed={false}` renders `data-shared-border-group` instead of the frame (an enclosing owner paints the outer edges). `edges` restricts painted sides (default all four). Extra props (`data-slot`, `role`, `aria-*`, `ref`, `className`) pass through; your `data-slot` overrides.
- Rule: a framed control (`ButtonGroup`/`Button`, `Input`, `Textarea`, `SelectTrigger`, `Toggle`, `ToggleGroup`, `ActionGroup`; anything with `data-shared-border-frame`) that is a DIRECT child of an element carrying `data-shared-border-cells` is a cell: no own frame/overlay, no native border or radius (inner `<input>`/`<textarea>` border too), `height:auto` stretched to the row (`align-self: stretch`; width in vertical rows), one divider (`border-inline-end`, `border-block-end` when vertical) to the next cell, none after the last. Hover/active/focus/loading/etc. seam priority works as for any cell.
- Wrapping a control in another element (Tooltip span, `div`, Field) breaks "direct child": it falls back to a floating frame. Put the wrapper's role on the cell instead (give the wrapper `data-shared-border-cell` and its own padding) or use `SharedBorderRowCell`.
- Plain (non-control) content must be a direct child `SharedBorderRowCell` (`data-shared-border-cell`, `flex items-center px-single py-1`, extra `className` merges) so it participates in the dividers. Un-marked siblings get no divider.
- Rows nest: a `SharedBorderRow` that is a direct child of another owner is itself a cell (no second overlay), its own children are cells of it.
- Fill: give controls `className="flex-1 min-w-0"` (applied to the control root: ButtonGroup/Input root/Select trigger) when they should share the row's width.
- Any existing element can be an owner without the primitive: add `data-shared-border-cells` to a `data-shared-border-group`/`-frame` element (OverviewCard `flush` content group already has it; use `data-orientation="vertical"` for columns).
- Compositor helper: `isSharedBorderCellFrame(element)` in `🗂️WindowChrome/shared-border.ts`.

## Mechanism

- Owner attribute `data-shared-border-cells` (domain-neutral). CSS (`🎨️styling/🖌️ui/🎨️.css`, before "Shared cells use one real divider"): `[cells] > [data-shared-border-frame]` gets `border-width:0; border-radius:0; align-self:stretch; height:auto; min-height:0; max-height:none` (vertical: `width:auto; max-width:none`), inner `input`/`textarea` `border-width:0`. The cell "unit" in all structural divider rules and in the whole `@layer shared-border-priority` (incl. forced-colors) is now `:is(.shared-border-cell, [data-shared-border-cell], [data-shared-border-cells] > [data-shared-border-frame])`, so dividers, last-cell, vertical block-end dividers and hover/active/focus/loading/introduced/celebrated seam priority apply to frame-cells unchanged.
- Compositor (`🗂️WindowChrome/shared-border.ts`): `isSharedBorderCellFrame(element)` (parent has `data-shared-border-cells`); such frames are skipped in `commit` (and restored/cleaned if they were registered earlier), so one owner (the row) paints each outer segment and no second overlay exists. `data-shared-border-cells` added to the MutationObserver attribute filter so toggling re-evaluates.
- No React context (portals/popovers would leak it): purely DOM direct-child based, so `Button`, `Input`, `Textarea`, `SelectTrigger`, `Toggle`, `ToggleGroup`, `ActionGroup` need no code change.

## Migrated sites

- New `🧱️elements/🪢️SharedBorderRow/🟦️.tsx` (+ package.json export, `📚️component-exports`, `ui-react/source` barrel explicit re-exports).
- `🎴️IconSelector`: outer `rounded-md border p-2 gap-2` removed; vertical `SharedBorderRow` with Select trigger, Textarea, preview as `SharedBorderRowCell`, and an inner `SharedBorderRow` holding the two Buttons (`flex-1`). No other 🖱️ui element had a control inside an ad-hoc bordered box (grep of `rounded* border` in elements).
- `🃏️OverviewCard` `flush` content group now also an owner: framed controls placed directly in it are cells.

## Tests

- New `🪢️SharedBorderRow/shared-border.test.tsx` (4 pass): attributes, one overlay for a row with Button/Input/Textarea/nested row/cell, exact four outer row paths, frame cells never `attached`, falls back to a frame when wrapped, CSS assertions.
- Language-agnostic: `WindowChrome/compositor.cases.json` + schema `cellFrames` (row size, cells, expected overlays, row paths, seam-priority fixtures); `shared-border.test.ts` "framed controls that are direct children of a cells owner..." (pass); `📏️border-presentation/segments.test.ts` browser fixtures for frame-cell seams (needs Playwright chromium, absent here like the existing browser case; unrun) and updated literal-selector expectations; legacy aggregator expectation in `owned-locale-detector-retirement/🌳️.../🟦️.tsx` updated for the new unit.
- Regression run: SharedBorderRow, WindowChrome, PanelTabBar, Canvas, OverviewCard tests, owned-locale-detector-retirement aggregator, react-export-surface: 633 passed, 0 failed ("No test suite found" children expected). Only failure left: the Playwright browser case (env).
- Real-browser probe (built-in pane, raw CSS): row of 3 frame cells -> dividers on first two only, last none, radius 0, height stretched to row 40px, vertical divider block-end. Probe file deleted.
- Not verified: course product tests (`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` fails to load in this environment: `require is not defined` in `🦑️repo/.../workspaces/bun`), so `🧪️tests/🧱️shared-borders` was not run. Reasoning: course flush cards only gain an attribute and course children are `data-shared-border-cell` (not frames), so computed output is unchanged.
