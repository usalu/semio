---
goal: R26-02/RUNNING-SKETCHPAD
---

# Ticket

## Summary

Refactored the JS Storybook launcher to reuse an existing listener on port 6006 instead of failing with a silent exact-port exit, and added launcher coverage in the existing JS unit tests.
## Changes
- Refactored [dev.ts](/workspaces/semio/compose/js/dev.ts) into a real launcher with explicit `storybook` and workspace modes instead of top-level side effects.
- Added Storybook reuse detection that checks port 6006, probes `http://localhost:6006/index.json`, and keeps the wrapper alive when an existing Storybook instance is already serving on that port.
- Routed [package.json](/workspaces/semio/compose/js/package.json) `dev` and `dev:storybook` through the new launcher while preserving the actual Storybook CLI command as `dev:storybook:inner`.
- Extended [compose.test.ts](/workspaces/semio/compose/js/compose.test.ts) to cover the new Storybook launch classification logic and wrapper argument parsing.

## Log
- Reproduced `npm run dev:storybook` exiting immediately with code 255 and only the Storybook banner.
- Intercepted Storybook’s hidden `process.exit(-1)` path and traced it to `getServerPort` inside `storybook/dist/core-server/index.js`, which exits when `--exact-port` sees port 6006 already in use.
- Confirmed port 6006 was already listening and found active `storybook` and `nx dev:storybook compose/js` processes in the workspace, which explained the silent exit.
- Verified `timeout 3s npm run dev:storybook` now exits cleanly after printing `http://localhost:6006/` and `Storybook already running at http://localhost:6006/` when port 6006 is already occupied.
- Verified `npm run test --workspace @semio-tech/compose-js -- compose.test.ts` passed with 14 tests after the launcher refactor.

## Todos
- [x] Reproduce the current `dev:storybook` failure and identify the real exit path.
- [x] Refactor the JS launcher so duplicate Storybook launches reuse the existing server instead of failing.
- [x] Run focused verification for the updated launcher and tests.

## Plan
- Verify the new launcher behavior for both the occupied-port reuse path and the direct start path.
- Run the existing JS unit test file with the added launcher coverage.
- Close the ticket with the updated JS launcher files after verification.
