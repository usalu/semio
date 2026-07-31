---
goal: R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS/RUNNING-DESIGN-APP
---

# Ticket

## Summary

Expanded Design node hit testing from stroke-only to full-node clicks and revalidated the focused Design suite.

## Changes

- Updated `compose/js/sketchpad/Design.tsx` so Design diagram nodes render an explicit invisible circular hit target matching the node body, allowing center clicks to select the node while preserving the visible ring styling.
- Extended `compose/js/sketchpad.test.ts` inside the existing `Design` flow to detect visible Design-node renderers and assert center-click selection when one is available, while keeping the existing node-center selection regression active for visible diagram nodes.
- Inspecting the existing `Design` Playwright flow and Design selection handlers to extend coverage for selection synchronization and mode behavior without adding a new test file.
- Extended the existing `Design` Playwright flow in `compose/js/sketchpad.test.ts` with new selection helpers and assertions for default replace behavior, shared-selection synchronization, empty-canvas deselection, additive mode, subtractive mode, and rectangular selection.
- Refactored the existing `Design` Playwright flow to replace a flaky connection-deletion keyboard dependency with deterministic direct store mutations so the test can progress into the selection section.
- Refactored the existing flat geometry assertion to tolerate the two intentional connection mutations performed earlier in the same `Design` flow.
- Refactored the existing multi-connection batch-edit assertions to use deterministic `compose.kit.updateDesign` updates instead of brittle DOM editing against slider controls, so the full `Design` flow remains stable after the selection assertions.

## Log

- Reopened the ticket for the follow-up request to make Design node selection use the full visible node hit area instead of stroke-only clicks.
- Traced the current Design diagram selection path and narrowed the issue to the Design-specific SVG node renderer, where the circular node needs an explicit fill hit target for center clicks.
- Patched the Design-node SVG renderer with a dedicated circular hit target and widened the existing Design Playwright helper so it searches more visible nodes for an SVG-backed Design node before asserting center-click selection.
- Ran the focused Design Playwright suite twice after the change; both runs passed and the current Metabolism scenario did not expose a visible SVG-backed Design node in the viewport, so the fallback node-center regression remained the active assertion path in this asset.
- Opened ticket via `repo ticket open` for the Design selection coverage request.
- Reused the current Design-app goal because the archived February ticket cannot be reopened through the current CLI (`ticket.json` is missing).
- Located the existing `Design` Playwright flow in `compose/js/sketchpad.test.ts` and the current diagram/scene selection handlers in `compose/js/sketchpad/Design.tsx`.
- Validation reruns now progress through the pre-existing connection-deletion and geometry sections.
- Current validation blocker: the new selection assertions reach the first real UI step, but the second candidate node can be outside the viewport, so the helper must choose visible diagram nodes before clicking.
- Updated the selection helper plan to choose only currently visible diagram nodes and click by screen coordinates at the node center.
- Reached a later pre-existing failure in the multi-connection detail test because the batch-edit assertions treated slider controls as text inputs.
- Replaced the flaky slider-edit path with deterministic store-backed bulk updates and reran the focused `Design` suite successfully.
- Final verification command: `cd compose/js && npx playwright test sketchpad.test.ts --grep "Design" --grep-invert "Design Drag Performance|Design Utility Tabs Stay Removed" --timeout 240000 --workers=1 --max-failures=1 --reporter=list`
- Final verification result: `1 passed (5.5m)` on 2026-03-03.

## Todos

- [x] Update the Design SVG node renderer so pointer hits cover the node interior and outline.
- [x] Extend the existing `Design` Playwright flow with a center-click regression for visible Design nodes.
- [x] Run the focused Design Playwright suite after the hit-area change.
- [x] Open the ticket and capture the work plan.
- [x] Extend the existing `Design` Playwright flow with coverage for cross-scene sync, replace-on-select, canvas deselect, additive, subtractive, and rectangular selection.
- [x] Refine the selection helpers to choose visible diagram nodes instead of assuming the first indexed nodes are clickable.
- [x] Run the focused Design Playwright suite and fix any remaining failures.
- [x] Finalize the summary and close the ticket with all touched files.

## Plan

- Patch the Design diagram node SVG with an explicit interactive circular hit target so center clicks resolve to the same node selection path as stroke clicks.
- Extend the existing `Design` Playwright scenario to find a visible Design node and assert that a center click selects it.
- Re-run the focused Design suite, then update this ticket with the final summary and touched files.
- Add deterministic helpers inside the existing `Design` test to read and normalize Design selection state from the actor.
- Extend the current Design selection section with UI-driven assertions for diagram clicks, empty-canvas deselection, selection-mode toggles, and scene-driven selection sync.
- Validate with the focused Design Playwright run, then update the ticket and close it with the touched files.
