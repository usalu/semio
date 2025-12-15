---
date:
  created: '2025-12-14T22:53:36.715Z'
  updated: '2025-12-14T22:57:06.255Z'
slug: YJS-KIT-ONLY
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: XState owns Sketchpad UI state
model: gpt-5.2-codex
prompts:
  - >-
    Yjs should only be used to synchronize the kit data. All app state should be
    stored in the state machine. E.g. when updating sketchpad settings I get:
    [Machine] Y_UPDATE → {\navigation\:\design\}
commit: 7765b633fe739bc29cd811ac7ec884e782e2e945
affectedFiles:
  - AGENTS.md
  - README.md
  - js/js/sketchpad/Sketchpad.tsx
  - log/2025/12/14/YJS-KIT-ONLY.md
lines:
  added: 353
  removed: 323
---
# Previously

# Context

- Sketchpad had a hybrid setup where `sketchpadMachine` observed a Y.js `ySketchpad` map via a `yjsSync` callback actor and emitted `Y_UPDATE` events.
- Sketchpad settings/navigation were stored in Y.js and any updates produced `[Machine] Y_UPDATE → ...` logs.

# Plan

- Remove Y.js observation for Sketchpad UI state (`Y_UPDATE`) and make `sketchpadMachine` own `SketchpadState`.
- Keep Y.js for Kit data only (kit-level `KitStore` documents).
- Persist Sketchpad UI state outside Y.js (local `localStorage`) and seed the machine from existing persisted values.
- Update dev docs to reflect the current state ownership.

# Changes

- Refactored `sketchpadMachine` to store `SketchpadState` in XState context (`context.sketchpad`) and removed the Y.js `yjsSync` actor and `Y_UPDATE` events.
- Updated `SketchpadStore` to proxy Sketchpad state through the XState actor, and to apply `SketchpadDiff` via `actor.send({ type: "CHANGE", diff })` instead of mutating `ySketchpad`.
- Added local persistence for Sketchpad UI state via `localStorage` key `semio.sketchpad.state.<id>` and merged it into the actor initialization.
- Updated `README.md` and `AGENTS.md` to document the current Sketchpad state ownership and persistence mechanisms.
