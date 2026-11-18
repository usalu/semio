# 🧱 Shared Border Consolidation — Worker Brief

Goal: every visible seam between two adjacent bordered things is ONE hairline, never two (no double lines, no 2px seams, no "border + border of neighbour", no extra border on an edge that coincides with a parent's edge, e.g. navbar toggles drawing their own bottom border on top of / just above the navbar's bottom border).

## Mechanism (read these before changing anything)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` ~L6655-6740 and ~L7335-7380:
  - `[data-shared-border-cell]` / `.shared-border-cell`: one `border-inline-end` divider per cell, last cell none.
  - `[data-shared-border-frame]` / `[data-shared-border-group]` with `data-orientation="vertical"` → block-end dividers.
  - `@layer shared-border-priority`: seam colour follows hover/active/focus/waiting/loading/introduced/celebrated of either incident cell.
  - `@layer shared-border-ownership`: `[data-shared-border-attached~="top|right|bottom|left"]` makes that physical side transparent (another owner paints it).
- Compositor: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts` (`BorderCompositor`, `useSharedBorderFrameRef`, `useSharedWindowBorder`) — resolves coincident segments, one owner per segment, sets `data-shared-border-attached`. Tests: `shared-border.test.ts`, `compositor.cases.json`, `🔨️modules/📏️border-presentation/segments.test.ts`.
- Silhouette chips: `[data-window-silhouette-chip]` defer outline to the silhouette SVG (WindowChrome `🟦️.tsx`).
- Previous ticket (history, design): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️05/CONSOLIDATE-NEIGHBORING-UI-BORDERS/` (SUMMARY.md, border-core.md, control-integration.md).

Prefer the existing mechanism (mark cells/frames/groups, or let the compositor attach) over ad-hoc `-ml-px`/`border-l-0` hacks. Fix the root (the shared element/component) so every product benefits, not per-product overrides, unless the issue is product-specific CSS.

## Rules (repo CLAUDE.md, abbreviated)

- Shared live worktree replicated to many machines: re-read a file right before editing; others edit concurrently — ignore unrelated changes, never revert others' work.
- NEVER run git commands that modify the tree (commit, stash, checkout, reset, restore, clean...). Read-only git is fine.
- Concise code, no comments inside definitions; docstrings start with a unique emoji.
- Temporary files/scripts/logs go ONLY into the ticket folder `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION/` (generated output under `🗑️generated/`). Temporary logs prefixed `[DEBUG] `.
- Build/test through the `repo` binary or repo MCP (`repo test <project>` / `repo run <project>:<target>`). If the fleet/MCP mechanism hangs or misbehaves, fall back to running vitest/bun directly from the package folder. Never get stuck: time-box any command (use `timeout`).
- Keep tests green; add/extend a test (prefer language-agnostic JSON cases where one exists) for each fix.
- Write your result report as a markdown file in the ticket folder (name given in your task) and reply with only a short summary + the file path.

## ⚠️ Round 2 — the radical rule (supersedes anything softer above)

The design system is radical: **every border run is one shared line. No bordered element may be separated from another bordered element by a gap, and no bordered element may sit inset inside another bordered element.** This includes things previously treated as intentional: gaps between cards in stacks and grids, the page margin between the navbar and the first card, dashed drop zones inside sections, tokens/chips with gaps, buttons hanging under a card with space to the next card, cards centred in grid cells with padding. Stacked cards interlock: the next card's title-chip row starts exactly on the previous card's bottom line, and a footer chip of the previous card (e.g. "Submit quiz >") shares that band with the next card's title chip. Grids of cards tile edge to edge with shared borders. Content spacing goes INSIDE cells (padding of text), never between borders. Only content art (charts, mockups, diagrams, canvases, 3D views) is exempt — mark its root `data-border-probe-art` and list it in your report.

### Live probe (the acceptance gate)

`border-gap-probe.js` in this folder. In any Vite-served page, run via the browser javascript tool:
`const p = await import('/@fs' + encodeURI('<ABS PATH>/border-gap-probe.js') + '?t=' + Date.now()); const r = p.probe({mark:true}); JSON.stringify({count:r.count, v:r.violations.map(v=>v.distance+' '+v.a+' || '+v.b)})`
`count` must be 0 on every view/state at 1440x900 AND 375x812 (use `resize_window`). `mark:true` outlines offenders in magenta for screenshots. If you believe a reported pair is a probe false positive, explain it in your report (do not silently ignore).

### Browser

Use the built-in browser tools (`mcp__Claude_Browser__*`). Several agents share it: create your OWN tab with `tabs_create` and pass that `tabId` on every browser call; never touch other tabs.

### Dev servers natively (the repo runner is broken)

`repo run` fails (resident service errors). Start Vite directly: from the package dir that owns the vite config, run in background
`SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=/Users/ueli/Documents/semio/.cache/b/p/m/<unique-name> /Users/ueli/Documents/semio/node_modules/.bin/vite --config <config> --mode prototype --port <unique port> --strictPort --host 127.0.0.1 > <ticket>/🗑️generated/<name>.log 2>&1 &`
(mkdir the cache dir first; it must be under `.cache/b/p/m`). If a plugin needs a repo-runner transport and throws, write a ticket-local wrapper config that imports the real config factory and filters that plugin out (see `vite-preview.config.ts`). Course dev server already runs at http://127.0.0.1:5391 (`?preview=1`, quizzes by hash e.g. `#physics`, runs work offline). Kill only servers you started when done.
