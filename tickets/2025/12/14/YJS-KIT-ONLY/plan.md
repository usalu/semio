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
