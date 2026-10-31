# Quiz Runs Reload And Timeout Fix

Run `55895`, `dev-shared`, traces under `🗑️generated/nx/e2e/20261030T102602-1008322/report/dev-shared`.

## Balanced pools (learner link after reload)

`test.trace` call `dowb@12133` clicked `a.course-card-link` (`href="#learner"`, text `Pool Player gukpl0`) with `noWaitAfter: true`. The action snapshot marks that anchor `__playwright_target__`. Reload had already landed on `http://127.0.0.1:42983/?preview=1` with no hash. The click action and the following snapshot stay on that URL. `hashchange` never had a `#learner` to deliver. For the next 20s `[data-layered-pane="learner"]` stayed `inert` and `aria-hidden`, without `data-opened`.

Earlier in the same trace the same link did set `#learner` (easy and medium). The hard iteration did not. `CourseCard` ignores clicks on `a` and only the card surface calls `onOpen`. The heading therefore opened only when the browser completed hash navigation. The card host and the click wrapper are `display: contents` under a `pointer-events-none` overlay, so a hit on the anchor can run without the activation behavior that writes the hash. `useAddress` keeps an external hash until `hashchange`; it does not open a page the browser never navigated to.

Fix: an unmodified primary click on the heading calls `onOpen` and `preventDefault`. Modified clicks still follow the href. Covered by `opens the page of a card from its heading link when the browser does not follow the hash` (`@semio-tech/course-react:test`, 1 passed).

## Perfect score (600s)

The test body reached 600.0s inside `openGrid` for `bim`, after excel-workbook's results, badge check, and `goHome`. It was not inside a quiz task. `page.evaluate` setting `location.hash = "bim"` started at 594s. At 600s Playwright started after-hooks and closed the context. The reported `boundingBox` on `[data-course-grid="root"] > [data-layered-overview]` is that teardown: parent chain is After Hooks → fixture `device` → Bounding box, and the error is `Target page, context or browser has been closed`.

A snapshot at the end of the hash write already shows `#bim`, the bim grid `data-grid-active`, and the root overview mounted as the direct child of `[data-course-grid="root"]`. Earlier successful `openGrid` calls still spent about 4–15s in `boxOf` on that same selector (17 bounding-box calls, 103s). `boxOf` waits until the locator is attached before reading a rectangle, then the alignment poll calls it again.

Fix: once the active grid is visible, read the root and active overview rectangles in one `page.evaluate` and poll that delta. No second attachment wait on the root overview.

Time in this run is still dominated by slow CDP turns (opened-pane `toHaveCount` 133s, clicks 97s, evaluates 87s, visibility 78s), including expects whose DOM was already true in the before snapshot. This change removes the root-overview attachment waiter that the timeout reported. It does not raise the 600s budget.
