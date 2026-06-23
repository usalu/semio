---
goal: SKETCHPAD
---

# Ticket

## Summary

Added the Type app toolbar model import action and extended the existing Type Playwright test to cover it. Playwright still reports no tests found for this repo layout, and the current tsc baseline still fails in sketchpad.test.ts for unrelated pre-existing errors.
## Changes

- Inspect Type app toolbar registration and existing file import commands.
- Add a toolbar import action in the Type app create controls.
- Extend the existing Playwright Type test to cover toolbar-driven model import.
- Keep the new import path on top of the existing kit file and type model commands.

## Log

- Opened ticket for the Type app toolbar import change.
- Verified the Type app toolbar currently exposes filter, selection, and create groups, but no file import action.
- Added a hidden multi-file input plus toolbar action in the Type app create controls.
- Extended the existing Type Playwright test with a model import assertion using `placeholder.glb`.
- `playwright test` could not discover `compose/js/sketchpad.test.ts` in this repo layout and returned `No tests found`.
- `npx tsc --noEmit --pretty false` failed on pre-existing `sketchpad.test.ts` errors at lines 3387, 3400, 5226, 5228, and 5280.

## Todos

- Implement the toolbar import action in `compose/js/sketchpad/Type.tsx`. Done.
- Extend `compose/js/sketchpad.test.ts` to validate the new import control. Done.
- Run the targeted Type Playwright test. Blocked by Playwright discovery in the current repo layout.

## Plan

1. Reuse the existing kit file command path instead of introducing a separate import mechanism.
2. Add the import action inside the Type app create toolbar settings so the new control stays with model/connector creation.
3. Validate the flow by importing a fixture model through the existing Playwright Type test.
