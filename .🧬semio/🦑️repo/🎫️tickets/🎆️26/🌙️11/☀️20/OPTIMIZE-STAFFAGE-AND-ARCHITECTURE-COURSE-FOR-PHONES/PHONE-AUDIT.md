# Phone Learner Journey Audit

- The phone Playwright project uses Chromium touch emulation at 375 × 812. Its existing journeys cover German home swipes, quiz completion and results, and English hard-run answer fields. The narrow navigation test uses 320 × 640 and requires 44 px navigation controls.
- The phone spec currently expects no iframe and no PDF request after opening an assignment, while AssignmentPage conditionally renders an iframe when opened. Check these together after ongoing changes settle. Mobile Safari PDF embedding requires device or WebKit verification; Chromium emulation cannot establish that behavior.
- Exercise real touch dragging across the edge of the task scrollport, cancellation, and subsequent scroll/tap interactions; input typing alone does not verify these gestures.
- Repeat the primary journey at a short landscape height and enlarged text, in both languages, checking reachable controls and horizontal overflow. Existing main journey only checks one portrait size.
- The gate documents forwarding Playwright options: repo run @teaching/architecture-course:test-e2e -- --topology dev -- --project phone --workers 1. The gate boots its own site and proctor.
