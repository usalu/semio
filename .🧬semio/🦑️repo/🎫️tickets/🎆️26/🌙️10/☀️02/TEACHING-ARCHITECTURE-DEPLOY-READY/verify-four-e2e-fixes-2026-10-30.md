# Verify Four E2E Fixes

Live dev stack `http://127.0.0.1:6061` answered HTTP 200 before the run. `repo://goals` was read. Ticket `26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY` was already open (`ticket_reopen` returned `ticket is already open`) under `🎯runningframework🎯runningproducts`.

Playwright was invoked the same way as `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts`: `repo tools exec node -- node_modules/playwright/cli.js test --config 🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts`.

Environment: `PLAYWRIGHT_BASE_URL=http://127.0.0.1:6061`, `TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=dev`, `PLAYWRIGHT_BROWSERS_PATH=/home/deck/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` (`chromium-1248`). One worker. `--output` under `🗑️generated/verify-e2e-fixes`.

## Desktop

`--project=desktop --grep "balanced pools rotate bim-concepts and preserve individual answers|software mockups render and grade in de"`

| Spec | Result | Duration |
| --- | --- | --- |
| balanced pools rotate bim-concepts and preserve individual answers | passed | 1.6m |
| software mockups render and grade in de | passed | 45.5s |

2 passed (2.4m). Exit 0. Started 2026-10-30T20:47:32+01:00.

## Pets

`--project=pets --grep "no two pets ever overlap during a lively minute"`

| Spec | Result | Duration |
| --- | --- | --- |
| no two pets ever overlap during a lively minute in which the learner drags them over each other and throws them | passed | 1.4m |

1 passed (1.4m). Exit 0. Started 2026-10-30T20:49:56+01:00. Finished 2026-10-30T20:51:20+01:00.

## Outcome

All three formerly failing specs passed against the live dev stack. No traces or screenshots were written (failure-only artifacts).

## Perfect score

`--project=desktop --grep "every quiz is playable to a perfect score"`

`--output` under `🗑️generated/verify-perfect-score`. Live stack still HTTP 200 at `http://127.0.0.1:6061` immediately before the run.

| Spec | Result | Duration |
| --- | --- | --- |
| every quiz is playable to a perfect score, and the perfect runs earn every badge of the course | passed | 3.5m |

1 passed (3.6m). Exit 0. Started 2026-10-30T21:00:36+01:00. Finished 2026-10-30T21:04:11+01:00. Wall clock 215s.
