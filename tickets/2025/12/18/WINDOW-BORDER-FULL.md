---
slug: WINDOW-BORDER-FULL
prompt: Every window should have a full border around it. Currently the bottom and right border are missing.
summary: Fix missing bottom/right window borders
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-18T15:40:32.088Z"
    finished: "2025-12-18T16:01:54.858Z"
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
    - prompt: Every window should have a full border around it. Currently the bottom and right border are missing.
      model: gpt-5.2-codex
      date:
        started: "2025-12-18T15:40:50.516Z"
        ended: "2025-12-18T15:44:21.803Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
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
      lines:
        added: 111
        removed: 36
    - prompt: The windows have no border now.
      model: gpt-5.2-codex
      date:
        started: "2025-12-18T15:51:19.817Z"
        ended: "2025-12-18T15:52:19.552Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
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
      lines:
        added: 154
        removed: 40
    - prompt: The windows have no border now.
      model: gpt-5.2-codex
      date:
        started: "2025-12-18T15:53:28.711Z"
        ended: "2025-12-18T15:57:38.721Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
      bundles:
        '@semio':
            files:
                "":
                    sections: {}
      files:
        updated:
            - path: ""
            - path: ""
      lines:
        added: 135
        removed: 13
    - prompt: Fix GoldenLayout borders disappearing due to invalid CSS and record final ticket updates.
      model: gpt-5.2-codex
      date:
        started: "2025-12-18T15:58:55.376Z"
        ended: "2025-12-18T16:01:06.676Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
      bundles:
        '@semio':
            files:
                "":
                    sections: {}
      files:
        updated:
            - path: ""
            - path: ""
      lines:
        added: 159
        removed: 11
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
lines:
    added: 208
    removed: 42
---


# Previously

- GoldenLayout window stacks were framed using an outer border while the GoldenLayout root uses clipped layout containers.
- Bottom and right frame edges could disappear due to clipping/rounding at the container boundary.

# Plan

- Keep semantic window borders but ensure the frame is rendered inside the window surface so it cannot be clipped.
- Update GoldenLayout styling to use an inset 1px stroke for the stack container.
- Document the window border mechanism in root dev docs.

# Changes

- Updated `js/js/globals.css` so GoldenLayout stack windows render the frame as an inset stroke (via inset shadow) instead of an outer border.
- Adjusted `js/js/globals.css` to render the inset stroke via a `::after` overlay frame after the shadow-based frame was not visible.
- Fixed a broken `.lm_item.lm_stack` CSS block (missing closing brace) and consolidated the GoldenLayout stack frame to a single `::after` inset stroke implementation.
- Restored GoldenLayout chrome and content backgrounds to use the window/base background tokens (instead of `transparent`) while keeping the inset stack frame.
