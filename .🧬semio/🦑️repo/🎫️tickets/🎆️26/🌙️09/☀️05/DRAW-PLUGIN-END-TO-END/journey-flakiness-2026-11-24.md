# Flaky Draw journeys (2026-11-24)

Cause: the served playground reloads the page when any agent edits a host TypeScript file that cannot be hot-replaced. A reload drops the folder attachment, the open popover and the view, so the journey fails at whichever step it is in.

| run | failed at | vite `page reload` | journey failure |
|---|---|---|---|
| r36w1 en | reload | 17:31:59 | 17:32:33 |
| r36w1 de | example | 17:33:22 | 17:33:29 |
| r36w3 de | export form in the rectangle setup | 17:40:20 | 17:40:36 |
| r36w4 en | second export | 17:41:48 | 17:42:04, Playwright log: "navigated to ..." during the click |

r36w2 en and de and r36w3 en had no reload and passed all ten steps under load 8 to 11. The earlier failures (from 16:05) fall in the same period as reloads at 16:04:37, 16:04:53, 16:08:17 and 16:08:48; they were not matched one by one. r36w1 de had already waited 23 s before its reload, so a slow example switch or the closed Actions popover may also have played a part.

Fix in the journey: the HMR socket of the served origin is answered by an inert route, so live edits cannot reload the test page; `documentLoads` (2) is asserted; `waitLayers` reopens the Actions popover when an example switch closes it.

Not changed: live edits still reload a person's own playground page. The progress back-off is not the cause of these four failures.
