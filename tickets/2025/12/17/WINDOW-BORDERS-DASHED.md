---
slug: WINDOW-BORDERS-DASHED
prompt: The windows dont have a border on the bottom and on the right. Make the window border dashed.
summary: Sketchpad windows have continuous dashed borders
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-17T18:15:25.407Z"
    finished: "2025-12-17T18:20:11.814Z"
commit: 7c4820638369104ae259b238d9240f08e429e67e
model: gpt-5.2-codex
iterations:
    - prompt: The windows dont have a border on the bottom and on the right. Make the window border dashed.
      model: gpt-5.2-codex
      date:
        started: "2025-12-17T18:15:31.251Z"
        ended: "2025-12-17T18:20:04.557Z"
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
      lines:
        added: 146
        removed: 54
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
lines:
    added: 148
    removed: 54
---


# Previously

GoldenLayout window borders were clipped on the outer edges (bottom/right) and window borders were not visually distinct enough for the intended “window boundary” semantics.

# Plan

Enforce border-box sizing for GoldenLayout items, switch window borders to dashed style for both Canvas and GoldenLayout windows, and document the border + spacing mechanism.

# Changes

GoldenLayout items now use border-box sizing and stacks render a dashed border so borders remain visible on all sides. Canvas windows use a dashed `border-window` border while GoldenLayout windows use `kind=\"layout\"` to avoid nested borders. Developer documentation updated to reflect dashed window borders.
