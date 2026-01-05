---
slug: APP-HOOKS-OVERFETCHING
prompt: All hooks for app state are massively overfetching. E.g. useDesignApp is fetching the entire design app state. But it only needs the selection, hover, camera, active tool, etc. Make sure to only fetch the necessary state. All app states should come directly from the state machine and use e.g. useSelector from xstate. Refactor all apps (home, kit, design, type, docs, feedback).
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-20T15:27:20.159Z"
iterations:
    - prompt: Refactor all app hooks to use granular XState selectors instead of fetching entire app state
      model: claude-opus-4-5
      date:
        started: "2025-12-20T15:27:39.808Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
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
        added: 0
        removed: 0
---


# Previously

# Plan

# Changes
