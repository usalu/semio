# Menu Attachment and Placement Handoff

## Result

Implemented only the ContextMenu subtree, Popover element and their tests. No CSS, WindowChrome, ShellScope, staging or commits.

- ContextMenu now positions each wing synchronously before measuring the next column, eliminating stale nested-wing overlap.
- One layout measurement updates fusion offsets, bounds and viewport clamping together. Opening/collapsing columns, live items, ResizeObserver, captured scroll and viewport resize all remeasure. Re-clamping starts from the original pointer anchor, so closed columns release their extra space.
- ContextMenuChrome no longer clips its silhouette with outer overflow. Viewport-limited scrolling belongs to its fusion content instead; title-chip geometry remains unchanged.
- Popover preserves explicit offsets and standalone four-pixel spacing; omitted offsets inside window silhouettes are flush. It stamps `data-shared-border-frame` for the approved compositor. It never guesses suppressed sides: the compositor replaces only actual coincident segments and retains detached/flipped outlines.
- Both components retain shell-owned portals, existing menu/dialog roles, focus restoration, keyboard navigation and pointer dismissal.

## Changed Files

Relative to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`:

- `🖱️ContextMenu/🟦️.tsx`
- `🖱️ContextMenu/🥣️chrome/🟦️.tsx`
- `🖱️ContextMenu/🧪️tests/🧩️component/placement.tsx`
- `🖱️ContextMenu/🧪️tests/🧩️component/placement.feature`
- `🗨️Popover/🟦️.tsx`
- `🗨️Popover/🧪️tests/🧩️component/placement.tsx`
- `🗨️Popover/🧪️tests/🧩️component/placement.feature`

Ticket-local reusable inputs: `menus-vitest.config.ts`, `menus-setup.ts`, this report.

## Validation

Tests authored before implementation. The initial test run could not import missing generated theme-token and icon-catalog modules. The ticket-local runner therefore mocks those two external presentation interfaces, not menu placement, portal ownership, WindowChrome, fusion geometry, accessible interactions or the compositor.

Command, using the restored launcher:

```text
/home/clstb/.local/share/semio/launcher/bin/repo tools exec vitest -- run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/menus-vitest.config.ts
```

Final run: **4 suites, 27 tests passed**. Includes both existing suites plus new geometry, resize, collapse, explicit-offset, collision-flip and shell-portal checks. New neutral Gherkin cases document both feature contracts. Popover viewport results are also compared with an independent lodash clamp reference over 16 placements.

Scoped `git diff --check` passed; final scoped diff inspected. Ticket-local menu Vite cache removed. Other agents' generated directories were not touched.

## Remaining Integration Verification

- Real-browser clipping, stacking, glass and forced-colors verification remains unavailable: the checkout lacks generated theme tokens/icon assets. The runner's mocked assets cannot prove rendered paint.
- The supplied compositor contract is adopted without imports. Lead should validate actual coincident-segment rendering after the silhouette agent integrates it.
- Oversized fused menus intentionally scroll. WindowChrome currently measures natural wing rectangles rather than their clipped scroll intersection; the silhouette agent should check outline bounds for a fusion wider/taller than the viewport when performing browser integration.
- Ticket registration/closure remains with lead/setup because native repo MCP is unavailable.
