---
slug: UI-BORDER-KINDS
summary: Semantic border kinds (element/window)
prompt: >-
  The border mechanism of all UI elements should be more flexible: add semantic
  border kinds with different styles. Introduce Tailwind classes border-element
  (element borders use hover color) and border-window (window borders use
  current normal border).
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-17T15:50:11.288Z'
  finished: '2025-12-17T16:10:02.845Z'
commit: 7c4820638369104ae259b238d9240f08e429e67e
model: gpt-5.2-codex
iterations:
  - prompt: >-
      The border mechanism of all UI elements should be more flexible: add
      semantic border kinds with different styles. Introduce Tailwind classes
      border-element (element borders use hover color) and border-window (window
      borders use current normal border).
    date:
      started: '2025-12-17T15:51:12.049Z'
      ended: '2025-12-17T16:09:50.461Z'
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 7c4820638369104ae259b238d9240f08e429e67e
    files:
      updated:
        - js/js/globals.css:
            lines:
              added: 6
              removed: 2
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 34
              removed: 33
        - js/js/sketchpad/Sketchpad.tsx:
            lines:
              added: 8
              removed: 5
        - js/js/sketchpad/Design.tsx:
            lines:
              added: 4
              removed: 4
        - js/js/sketchpad/Docs.tsx:
            lines:
              added: 1
              removed: 1
        - README.md:
            lines:
              added: 11
              removed: 0
        - AGENTS.md:
            lines:
              added: 12
              removed: 5
        - log/tickets/2025/12/17/UI-BORDER-KINDS.md:
            lines:
              added: 0
              removed: 0
      created: []
      removed: []
    lines:
      added: 76
      removed: 50
files:
  updated:
    - AGENTS.md:
        lines:
          added: 12
          removed: 5
    - README.md:
        lines:
          added: 11
          removed: 0
    - js/js/globals.css:
        lines:
          added: 6
          removed: 2
    - js/js/sketchpad/Design.tsx:
        lines:
          added: 4
          removed: 4
    - js/js/sketchpad/Docs.tsx:
        lines:
          added: 1
          removed: 1
    - js/js/sketchpad/Sketchpad.tsx:
        lines:
          added: 8
          removed: 5
    - js/js/sketchpad/elements.tsx:
        lines:
          added: 34
          removed: 33
    - log/tickets/2025/12/17/UI-BORDER-KINDS.md:
        lines:
          added: 0
          removed: 0
  created: []
  removed: []
lines:
  added: 76
  removed: 50
---
# Previously

- UI borders used a single border color token for both interactive elements and window frames.
- Tailwind usage relied on `border-border` / `divide-[color:var(--border-color)]`, making it hard to distinguish semantic border intent.

# Plan

- Add semantic border kinds: `element` and `window`.
- Map border kinds to design tokens so styling can change centrally.
- Update UI components to use `border-element` / `divide-element` for interactive UI elements.
- Keep the existing window border appearance via `border-window` / window border tokens.
- Document the border kinds in `README.md` and `AGENTS.md`.

# Changes

- Added `--border-element-color` and `--border-window-color` tokens and mapped them to Tailwind color utilities (`border-element`, `border-window`) via `@theme inline`.
- Updated Sketchpad UI components to use `border-element` / `divide-element` instead of the previous single-border token usages.
- Updated developer documentation to define the semantic border kinds.
