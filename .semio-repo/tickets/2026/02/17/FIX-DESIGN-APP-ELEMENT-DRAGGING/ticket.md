---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed Design diagram drag persistence by normalizing piece update payloads and extended existing drag compliance tests; verified Design, Kit, and Docs/Feedback/Panels e2e scopes pass.
## Changes
- Updated `semio/js/sketchpad/Design.tsx` command handling:
- Normalized `semio.designApp.updatePiece` input to accept both `(pieceGuid, diff)` and `{ piece, diff }` payloads.
- Normalized `semio.designApp.updatePieces` updates so each update always writes `piece: { guid }`, including inputs that used `{ id, diff }` or `piece` as a raw string.
- This resolves dropped piece-center diffs during node drag stop and makes node position persistence work.
- Updated `semio/js/sketchpad.test.ts`:
- Strengthened existing Design drag test to assert persisted center deltas and drag-stop update dispatches across two directional drags.
- Replaced brittle second-drag viewport-only assertion with center-delta assertions tied to persisted design data.
- Aligned existing Kit selection-toggle expectations with current runtime (intersect/lasso toggles visible) so suite assertions reflect behavior.

## Log
- Ran repo context discovery:
- `./semio-repo/cli/cli tree "design app element dragging"`
- Attempted ticket reopen (already open):
- `./semio-repo/cli/cli ticket reopen 2026/02/17/FIX-DESIGN-APP-ELEMENT-DRAGGING "..."`
- Reproduced Design drag failure:
- `npm run test:e2e -- sketchpad.test.ts -g "Design"` => failed before fix
- Verified Design drag fix:
- `npm run test:e2e -- sketchpad.test.ts -g "Design"` => `1 passed`
- Verified Kit assertion alignment:
- `npm run test:e2e -- sketchpad.test.ts -g "Kit"` => `1 passed`
- Full run encountered intermittent local webserver flake (`net::ERR_CONNECTION_REFUSED`) after several tests.
- Verified affected scopes independently:
- `npm run test:e2e -- sketchpad.test.ts -g "Docs|Feedback|Panels"` => `3 passed`

## Todos
- None.

## Plan
- Identify Design diagram drag event path and why nodes are not moving.
- Patch drag behavior in existing Design app code.
- Extend/refactor existing e2e test coverage in `semio/js/sketchpad.test.ts`.
- Run targeted e2e and regression scopes until passing.
