---
slug: SKETCHPAD-PANEL-BG
prompt: The panel base needs to have panel background color
summary: Ensure panels use panel background and panel level context
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-18T16:20:06.035Z
  finished: 2025-12-18T16:22:19.556Z
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: Ensure panel base containers render with panel background (bg-panel) and panel level context so they don't inherit base background.
    model: gpt-5.2-codex
    date:
      started: 2025-12-18T16:20:54.781Z
      ended: 2025-12-18T16:22:08.587Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
---


# Previously

- Panel containers were rendered without enforcing panel level context and background.
- With a global base level provider, panels could inherit base background instead of `bg-panel`.

# Plan

- Ensure the shared `Panel` component runs under `LevelProvider level="panel"`.
- Ensure panel base containers paint `bg-panel` (for default `showBackground=true`).
- Document the invariant in root docs.

# Changes

- Wrapped `Panel` in `LevelProvider level="panel"` and applied `bg-panel` to the panel container.
- Documented panel level enforcement in `README.md` and `AGENTS.md`.
