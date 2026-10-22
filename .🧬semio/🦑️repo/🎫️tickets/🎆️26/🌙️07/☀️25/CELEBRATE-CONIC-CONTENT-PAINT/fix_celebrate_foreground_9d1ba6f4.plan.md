---
name: Fix Celebrate Foreground
overview: Stop celebrate from painting as a background fill (especially on tree rows), and extend the spinning conic to all tree foreground chrome—guides, elbow/stem, chevron, label, icon, and drag handle.
todos:
  - id: fix-icon-blend-hosts
    content: Limit destination-in celebrate recipe to SVG-owning [data-icon] elements; remove tree-icon/drag-handle wrappers
    status: completed
  - id: tree-stroke-chrome
    content: Paint celebrated tree elbow/stem/guide-line and ancestor IndentationLines with celebrate-conic
    status: completed
  - id: tests-foreground-only
    content: Extend celebrate CSS vitest for no-fill + tree chrome coverage
    status: completed
  - id: ticket-reopen-close
    content: Reopen CELEBRATE-CONIC-CONTENT-PAINT, close with summary and files
    status: completed
isProject: false
---

# Fix Celebrate To Foreground Only

## Root cause

In [`ui/styling/js/ui.css`](ui/styling/js/ui.css) `CelebrateContent`, the icon recipe targets wrappers that do **not** own the SVG:

```css
:is([data-icon], [data-icon-kind], [data-slot="tree-icon"], [data-slot="drag-handle"])::before
```

DOM is `tree-icon` / `drag-handle` → `span[data-icon]` → `svg`. The wrapper gets a large `::before` conic (`inset: -100%`) but `> svg { mix-blend-mode: destination-in }` never matches, so the conic shows as a **filled rectangle** behind the row chrome.

Tree indentation lines (`[data-tree-guide-line]`, `[data-slot="tree-branch-elbow"]`, `[data-slot="tree-branch-stem"]`) and gutter chevrons are also missing from celebrate paint—they stay solid muted/emphasized.

## Rules

- Celebrate paint is **only** foreground ink: border ring (`::after` mask), text (`background-clip: text`), icon/handle strokes (`destination-in` on the SVG owner), and 1px guide/elbow/stem strokes (conic as the line’s own background).
- Never put `--celebrate-conic` as a fill on row shells (`tree-item-row`, `tree-row-content`, etc.).

## Changes in [`ui/styling/js/ui.css`](ui/styling/js/ui.css)

### 1. Icon/handle recipe — SVG owners only

Apply isolation / `::before` / `destination-in` only to:

- `[data-icon]`
- `[data-icon-kind="catalog"]` / `[data-icon-kind="svg"]`

Remove `[data-slot="tree-icon"]` and `[data-slot="drag-handle"]` from that recipe. Nested icons under those slots (and under `[data-slot="tree-gutter-slot"]` chevrons) still match via `[data-icon]`.

Keep the celebrated drag-handle hover override so grips do not snap back to solid `--border-emphasized-color`.

### 2. Tree stroke chrome under a celebrated row

For celebrated tree row slots, paint descendant lines with the shared conic (these elements *are* the stroke, so this is foreground, not a row fill):

- `[data-tree-guide-line]`
- `[data-slot="tree-branch-elbow"]`
- `[data-slot="tree-branch-stem"]`

Set `background-color: transparent !important` and `background-image: var(--celebrate-conic)` (override hover-path solid emphasized rules while celebrating).

### 3. Ancestor indentation guides (`IndentationLines`)

Guides live on branch content (`[data-slot="tree-guide"]` inside `TreeBranchContent`), not inside the row. When a branch content `:has()` a celebrated tree row, paint its `> [data-slot="tree-guide"] [data-tree-guide-line]` with the same conic formula and local spin so ancestor guides match the celebrated row. Define `--celebrate-conic` / angle on that branch content for the `:has()` case (custom props do not inherit upward from the stamped row).

### 4. Tests in [`ui/js/react/index.tsx`](ui/js/react/index.tsx)

Extend the existing celebrate CSS contract test:

- Icon `::before` recipe must not list `tree-icon` / `drag-handle` as blend hosts.
- Celebrated tree rows include guide/elbow/stem conic stroke rules.
- Branch `:has([data-celebrated])` guide paint is present.
- No rule sets celebrate-conic as background on `tree-row-content` / tree row shells.

## Ticket

Reopen [`CELEBRATE-CONIC-CONTENT-PAINT`](.repo/🎫️/26/07/25/CELEBRATE-CONIC-CONTENT-PAINT/), keep notes in the ticket folder, close with summary + files. Goal remains `🎯️r2602/🎯️runningsketchpad`.