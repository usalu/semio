---
slug: REMOVE-YJS-FROM-APP-STORES
prompt: IMPLEMENT the refactor plan to remove Yjs from all app stores and keep Yjs only in KitStore
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-19T22:05:57.199Z
iterations:
  - prompt: Implement the refactor plan to remove Yjs from all app stores
    model: claude-opus-4-5
    date:
      started: 2025-12-19T22:06:13.245Z
      ended: 2025-12-19T22:14:07.374Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 2ac49129091a7cee4b05f2fe08423415f179ea77
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
  - prompt: Make sure 100% of yjs disappears and no yMap are left in any app store but only in kit app. All other apps should use exclusively the state machine.
    model: claude-opus-4-5
    date:
      started: 2025-12-19T22:18:36.753Z
      ended: 2025-12-19T23:08:20.382Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 2ac49129091a7cee4b05f2fe08423415f179ea77
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
  - prompt: Remove Y.js from Design.tsx, Quality.tsx, Docs.tsx, and Tutorials.tsx by removing legacy store classes and switching all hooks to use XState
    model: claude-opus-4-5
    date:
      started: 2025-12-19T23:15:09.367Z
      ended: 2025-12-20T00:12:27.853Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 2ac49129091a7cee4b05f2fe08423415f179ea77
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
---


# Previously

# Plan

# Changes
