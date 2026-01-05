---
slug: UI-BORDER-KINDS
prompt: 'The border mechanism of all UI elements should be more flexible: add semantic border kinds with different styles. Introduce Tailwind classes border-element (element borders use hover color) and border-window (window borders use current normal border).'
summary: Semantic border kinds (element/window)
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-17T15:50:11.288Z"
    finished: "2025-12-17T16:10:02.845Z"
commit: 7c4820638369104ae259b238d9240f08e429e67e
model: gpt-5.2-codex
iterations:
    - prompt: 'The border mechanism of all UI elements should be more flexible: add semantic border kinds with different styles. Introduce Tailwind classes border-element (element borders use hover color) and border-window (window borders use current normal border).'
      model: gpt-5.2-codex
      date:
        started: "2025-12-17T15:51:12.049Z"
        ended: "2025-12-17T16:09:50.461Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 7c4820638369104ae259b238d9240f08e429e67e
      bundles:
        '@semio':
            files:
                "":
                    sections: {}
      files:
        updated:
            - path: ""
            - path: ""
            - path: ""
            - path: ""
            - path: ""
            - path: ""
            - path: ""
            - path: ""
      lines:
        added: 76
        removed: 50
bundles:
    '@semio':
        files:
            "":
                sections: {}
files:
    updated:
        - path: ""
        - path: ""
        - path: ""
        - path: ""
        - path: ""
        - path: ""
        - path: ""
        - path: ""
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
