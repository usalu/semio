---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Implemented a shared local-only sketchpad chat panel and replaced all chat placeholders, including Kit. Added Playwright coverage, but validation remains blocked by pre-existing Quality.tsx syntax errors and the existing initDesign chat-toggle timeout path.
## Changes

- `compose/js/sketchpad/elements.tsx`: Added a shared `BasicChatPanel` component with seeded assistant messages, local draft state, Enter-to-send behavior, send/clear actions, and stable `data-testid` hooks for automation.
- `compose/js/sketchpad/Home.tsx`: Replaced the home chat placeholder with `BasicChatPanel`.
- `compose/js/sketchpad/Type.tsx`: Replaced the type chat placeholder with `BasicChatPanel`.
- `compose/js/sketchpad/Kit.tsx`: Replaced the kit chat placeholder with `BasicChatPanel`.
- `compose/js/sketchpad/Docs.tsx`: Replaced the docs chat placeholder with `BasicChatPanel`.
- `compose/js/sketchpad/Quality.tsx`: Replaced the quality chat placeholder with `BasicChatPanel`.
- `compose/js/sketchpad/Design.tsx`: Replaced both design chat placeholder registrations with `BasicChatPanel`.
- `compose/js/sketchpad.test.ts`: Added a dedicated `Chat Panel` Playwright test that exercises open, seed visibility, send, and clear behavior.

## Log

- Gathered sketchpad chat references and confirmed current chat tabs render placeholder-only content in app-specific files.
- Identified a shared implementation point in `compose/js/sketchpad/elements.tsx` to avoid duplicating chat behavior across apps.
- Implemented the shared local-only chat panel and switched all app chat tabs over to the common component.
- Corrected the remaining Kit chat tab registration after verifying the file still referenced the placeholder block.
- Ran `npx tsc -p compose/js/tsconfig.json --noEmit`; it failed on pre-existing syntax errors in `compose/js/sketchpad/Quality.tsx` around `QualityTree` (`TreeItem` / `TreeContent` mismatch starting near line 1776), not on the new chat edits.
- Ran `npx playwright test compose/js/sketchpad.test.ts --grep "Chat Panel" --reporter=line`; the new test still times out while trying to interact with the existing Design app navigation flow after `initDesign`, so the new assertions were added but not fully validated end-to-end.

## Todos

- [x] Add reusable basic chat panel component in `compose/js/sketchpad/elements.tsx`
- [x] Replace placeholder chat content in sketchpad app files with the shared component
- [x] Extend the existing sketchpad test file to cover the new chat mechanism
- [x] Run targeted validation and update ticket summary

## Plan

1. Inspect the shared sketchpad element primitives and existing chat tab wiring.
2. Implement a reusable chat panel in `elements.tsx` with local state for message history and draft input.
3. Replace each app-level chat placeholder with the shared panel so all chat tabs behave consistently.
4. Extend the existing sketchpad test file to assert seeded content and sending behavior.
5. Run the relevant test scope and close the ticket with the changed files.
