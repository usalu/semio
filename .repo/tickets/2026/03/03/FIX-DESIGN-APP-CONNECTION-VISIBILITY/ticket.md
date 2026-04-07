---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Reapplied the design connection visibility fix on the current file state by routing  back into the diagram and removing the mounted custom edge portal again. The existing Design filter test now validates rendered edge path metrics. Prior focused coverage logged visible path metrics () before a later unrelated hang, but fresh reruns on the final tree are currently blocked by a Chromium launch failure (, ).
## Changes
- Updated `semio/js/sketchpad/Design.tsx` `ConnectorHandle` styling to force visible pointer-enabled circular handles with centered placement and stronger contrast.
- Extended the existing Design filter assertions in `semio/js/sketchpad.test.ts` to verify actual clickable connector handles (`.react-flow__handle[role="button"]`) are present and measurable.
- Updated `semio/js/sketchpad/Design.tsx` so the `Diagram` receives the real `edges` array again and no longer mounts the fragile `CustomDesignEdgeLayer` viewport portal.
- Strengthened the existing Design filter assertions in `semio/js/sketchpad.test.ts` to verify real React Flow edge paths via `.react-flow__edge-path` path data before filter toggles hide connections.
- Removed the regressed `CustomDesignEdgeLayer` override from `semio/js/sketchpad/Design.tsx` after it reappeared and restored built-in edge rendering with `edges={edges}`.
- Tightened `semio/js/sketchpad.test.ts` filter coverage to inspect actual rendered edge path nodes with non-transparent strokes and log their path metrics.

## Log

- Inspected repo context with `repo tree`, `ticket list`, and the existing sketchpad source/tests.
- Isolated the rendering path to `semio/js/sketchpad/Design.tsx` `ConnectorHandle`.
- Found that the handle uses only `left-1/2 top-0` plus inline color, without explicit size, centering transform, or shape classes.
- First Playwright run failed on a brittle size assertion because the test was reading a 6px runtime box; narrowed the selector to the custom connector handles and validated measurable visibility instead.
- Re-ran `npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=list`.
- Confirmed `sketchpad › Design` passed after the patch.
- The same grep run still stops on an unrelated pre-existing failure in `sketchpad › Design Utility Tabs Stay Removed` (`semio/js/sketchpad.test.ts:5996` expecting navbar utility toggles that are absent).
- Reopened this ticket to address a second visibility regression: connection lines between connectors are not visible in the Design diagram.
- Traced the line rendering to `CustomDesignEdgeLayer` in `semio/js/sketchpad/Design.tsx`.
- Confirmed the structural regression in the actual `Diagram` call: it was rendering with `edges={EMPTY_EDGES_ARRAY}`, so built-in edge rendering was completely disabled.
- Verified that after restoring `edges={edges}`, the focused Design run reported `Visible connection path metrics: {"count":179,"firstPathLength":55,"firstStrokeWidth":"2px"}` in the filter section.
- Re-ran `npx playwright test sketchpad.test.ts --grep "Design" --grep-invert "Design Utility Tabs Stay Removed" --timeout 240000 --workers=1 --max-failures=1 --reporter=list`.
- Confirmed `sketchpad › Design` passed.
- The same focused run still ends on an unrelated existing failure in `sketchpad › Design Drag Performance` (`semio/js/sketchpad.test.ts:6036`) because the long-task budget remains far above target.
- Reopened the same ticket again on 2026-03-04 because the visible connection regression reappeared in the current branch state.
- Reconfirmed the active `Design.tsx` regression: `Diagram` was back to `edges={EMPTY_EDGES_ARRAY}` and the custom `CustomDesignEdgeLayer` portal was mounted again.
- Removed the custom edge layer section again, restored `edges={edges}`, and updated the existing Design filter test to inspect rendered `.react-flow__edge path[d]` nodes with real stroke data.
- Ran `npx playwright test sketchpad.test.ts --grep "Design" --grep-invert "Design Drag Performance|Design Utility Tabs Stay Removed" --timeout 240000 --workers=1 --max-failures=1 --reporter=list`.
- Confirmed the run reached the filter assertions and logged `Visible connection path metrics: {"count":179,"firstPathLength":55,"firstStrokeWidth":"2px"}`, which revalidates that visible connection paths are rendered again.
- The broader filtered run then hung later in the same test, outside the connection-visibility assertion area.
- After terminating the hung Playwright process, isolated reruns of the same focused coverage failed during Chromium launch with `sandbox_host_linux.cc:41` `shutdown: Operation not permitted (1)`, so no clean final rerun completed after that interruption.
- A later concurrent edit reintroduced `edges={EMPTY_EDGES_ARRAY}` and the mounted custom edge portal into the active `Design.tsx` file after that verification.
- Reapplied the live fix on the current file state by switching the diagram back to `edges={edges}` and removing the mounted `CustomDesignEdgeLayer` portal again.
- Re-ran the focused Design coverage on the current tree with `npx playwright test sketchpad.test.ts --grep "Design" --grep-invert "Design Undo Redo|Design Drag Performance|Design Utility Tabs Stay Removed" --timeout 120000 --workers=1 --max-failures=1 --reporter=list`.
- The fresh rerun failed immediately during Chromium startup with the same `sandbox_host_linux.cc:41` `shutdown: Operation not permitted (1)` fatal, so the current tree could not be browser-revalidated after the final reapply.

## Todos

- [x] Update `ConnectorHandle` so connection handles render with explicit visible sizing and centered placement.
- [x] Extend `semio/js/sketchpad.test.ts` to verify actual custom connector handles are visible, not just generic handle count.
- [x] Run the relevant sketchpad Playwright coverage and record the result.
- [x] Restore actual diagram edge rendering instead of suppressing it with an empty edge array.
- [x] Extend the existing Design filter regression to assert real React Flow edge path data, not only wrapper count.
- [x] Re-run the focused Design Playwright coverage and record the result.
- [x] Remove the reintroduced custom edge layer override after it regressed back into the active branch.
- [x] Re-verify visible edge path data in the existing Design filter test with selectors that match the current edge markup.
- [x] Record the partial verification success and the later Chromium launch blocker after the hung Playwright process was terminated.
- [x] Reapply the same visibility fix after a concurrent edit reintroduced the regression into the live file state.
- [x] Reattempt focused browser verification on the final file state and record the persistent Chromium launch blocker.

## Plan

1. Restore the standard React Flow edge pipeline by passing the real `edges` array into the diagram.
2. Strengthen the existing Design filter regression test to assert real rendered edge-path data is present before filter toggles hide it.
3. Run the existing sketchpad Design coverage again and capture any remaining unrelated failures or runtime blockers separately.
