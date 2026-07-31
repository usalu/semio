# Richer Introduction Click Previews

## Theme integration pass

Replaced broken shadcn-style `hsl(var(--primary|background|border|…))` with the real design system:

- Button fills: `--accent` (left), `--accent-tertiary` (middle), `--accent-secondary` (right)
- Outline/chips: `--foreground`, `--muted-foreground`, `--accent-foreground`, `--temporary`
- Spacing/stroke/type: `--spacing-*`, `--ui-spacing`, `--stroke-hairline`, `--size-medium`, `--text-2xs`, `--font-mono`
- Inline SVG mouse uses `currentColor` so light/dark appearance flips with chrome
- Modifier chips mirror hotkey `kbd` chrome (`border-accent-foreground` + temporary fill)

## Verify

- vitest: `introductionDemoResolveVisual|UIIntroduction demonstration` — 7 passed
- cargo: `introduction_gesture` — 2 passed
