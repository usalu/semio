# Pet Task List Semantics Audit

Read-only diagnosis of actual fullReact73624 hand-topics failure. Product course-react/modules/quiz-page/TSX:223–231 still renders native ol role=list and one li per authored task. AccessGate is inside each li; the inner div at225 carries data-pet-prop and actual content. Thus received DIV marker hosts retain the same task IDs and course-tasks card while native ordered-list semantics remain in their ancestors.

The stale assertion is product/tests/pet-allowances/hand-topics/TSX:81–83: it requires the pet marker itself to be LI. This is not evidence of a missing list item or inaccessible task structure. Correct the test to assert current inner DIV hosts and exact IDs/card, then separately assert each host has a native closest LI directly under the expected OL/list, preserves authored order, text and list-item count. This strengthens the intended distinction between pet surface and semantic container.

Moving the marker out of AccessGate to satisfy a tag expectation would risk bypassing locked preview/inert ownership and is not justified. No product/test edit or execution was performed by this audit. Backend owns the schema/test correction and current focused/ordinary owner evidence; full BOTH remains separate.

## Final Test-Only Repair Review

Current hand-topics Source234432d2 now expects the exact DIV markerhost while independently asserting native LI direct children of OL role=list, exact task IDs/card/order/text and unmarked outer LI. Closed neutral fixture at product/course/fixtures/pet-contexts/task-topics covers open and first-task-locked cases. The latter inspects actual retained inert/aria-hidden preview; markers remain inside AccessGate rather than moving to operable outer nodes. Existing selector excludes drag ghosts only, so this retained-preview inspection is intentional. A second JSDOM parser reads actual rendered HTML and independently checks marker/LI/OL ancestry. Original overview/run/result marker assertions remain. No blocking static defect found; pure schema/parser success is not rendered test execution. Parent owns sole complete normal React rerun.
