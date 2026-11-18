# Tree suite fixes

Result: `🧱️elements/🌳️Tree/🧪️tests` now 72 pass, 3 fail (was 5 fail). Aggregator `🧪️tests/🧪️owned-locale-detector-retirement`: 565/565 tests pass (the "37 files failed" lines are the aggregator's sub-modules reporting "No test suite found"; unchanged, not tests).

## Fixed (stale test, no product change)
`🧩️component` "Tree focus retention > returns to the surviving ancestor after action replacement" (en, de). Focus retention (`🎯️focus`) correctly lands on the surviving ancestor row, but the row is now the roving tab stop (`⌨️roving`, `tabindex=0`, commit 720) so `candidate()` returns the `role=treeitem` row, not the disclosure button inside it. Test now expects `getByRole("button", {name: parent}).closest("[role=treeitem]")`. That is the intended roving-tabindex behaviour.

## Environmental, left alone (3)
`🔤️text-flow` (3 tests) need Playwright's `chromium_headless_shell-1248`, which is not installed (`npx playwright install`). They fail at `browserType.launch`, not on assertions.

Edited: `🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx`.
