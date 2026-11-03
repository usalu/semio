# Badge And ActionGroup Spacing

## Changes

All source paths below are relative to `🧰️framework/🔨️modules/🖱️ui/`.

- `🎯️targets/⚛️react/🚦️status-surface/🟦️.tsx`: Badge uses existing `px-double gap-single`, retains `py-0.5`, and constrains SVG descendants to its tiny icon wrapper.
- `🧱️elements/⚡️ActionGroup/🟦️.tsx`: medium-height shell; full-inner-height items with `min-h-0`; identical classless-SVG selector overrides the inherited small size through `cn` utility grouping. Generic SVG sizing remains tiny for explicitly classed icons. No clipping introduced.
- `🧱️elements/⚡️ActionGroup/🧪️tests/🔣️.json`: language-neutral utility and geometry regression vectors for both components.
- `🧱️elements/⚡️ActionGroup/🧪️tests/spacing.test.ts`: Babel parses actual JSX/class composition; repository `cn` resolves inherited geometry; theme resolver supplies current metrics; existing third-party CSS calculator independently checks fixture arithmetic.

No shared chrome helper change was necessary. No class-name-composition, avatar, Label, Tree/Catalogue, global scale, generator, Storybook configuration, or server changes were made.

## Validation

Test-first: both spacing contracts failed against original source (missing Badge padding and medium ActionGroup shell). The icon-wrapper assertion also failed before adding SVG containment.

Final command (run through the required repo launcher):

```sh
PATH=$HOME/.local/share/semio/launcher/bin:$PATH repo tools exec --light bun -- test './🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🧪️tests/spacing.test.ts' './🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/📐️geometry/🟦️.ts'
```

Result: 5 passed, 0 failed, 993 assertions. Scoped source diff inspected; scoped `git diff --check` passed. No generated output files were created. MCP lifecycle unavailable, as supplied by lead; existing ticket retained.

## Browser Acceptance For Lead

- `[data-slot="badge"]`: horizontal padding 6.4px each side, gap 3.2px; unchanged vertical padding 2px and expected 22px overall height. Every tone shares this geometry. Icon wrapper and contained SVG should measure 9.6px square.
- `[data-slot="action-group"]`: 22.4px exterior height; 1px border leaves 20.4px inner height.
- `[data-slot="action-group-item"]`: 20.4px height, wholly inside shell borders, 3.2px padding, minimum height zero; no hidden overflow. Icon-only items remain square.
- Item SVGs: 9.6px square for raw/classless and catalog icons. In a bordered sibling item, expected icon-edge clearance is 5.4px vertically and 4.9px horizontally, both exceeding 3.2px padding. Pixel rounding tolerance is appropriate.
- Verify focus, disabled presentation, button keyboard activation, and dropdown opening as before; event/accessibility code was not changed.

## Remaining Risks

Tests validate authored geometry and composition, not browser layout or CSS generation. Lead still owns rebuild/browser measurement and catalog/raw icon comparisons. Badge normalizes SVG icons; arbitrary non-SVG React nodes remain caller-controlled. The separately owned shared `cn` arbitrary-color defect is not repaired here. StatusChip's pre-existing `gap-half` is deliberately unchanged because this assignment covers Badge only.
