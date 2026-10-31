# Dev Topology End-to-End Full Run

`repo://goals` was read. Ticket `26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY` was already open under `🎯runningframework🎯runningproducts`.

## Full suite

```
SEMIO_LANGUAGE=en SEMIO_TICKET_DIR=<this ticket> NX_DAEMON=false repo run @teaching/architecture-course:test-e2e --configuration=release --parallel=1 -- --topology dev --serial
```

Run directory `🗑️generated/nx/e2e/20261030T204911-1220686`. One dev stack, shared then exclusive. Exit 1. Gate failed after 3460 s: specs failed.

| Wave | Result | Duration |
| --- | --- | --- |
| Boot | 1 passed | 23.5s |
| Shared (desktop, phone, layout, pets) | 71 passed, 1 failed | 46.6m |
| Exclusive (presence, shortage, away, faults) | 12 passed | 10.5m |

85 tests, 9 projects, 16 specs. Exclusive ran after the shared failure.

## Failure

`balanced pools rotate bim-concepts and preserve individual answers` (`🎯️quiz-runs/🟦️.ts:76`).

After the expert reload, the click on `a.course-card-link` (`href="#learner"`, `noWaitAfter`) completed and the URL stayed `http://127.0.0.1:35151/?preview=1`. For 20s `[data-layered-pane="learner"]` stayed `inert` and `aria-hidden`, without `data-opened`. The same click had opened `#learner` on the easy, medium, and hard reloads of that test. The overview was interactive and resting on the learner card. Screenshot and `error-context.md` are under `report/dev-shared/🎯️quiz-runs-🟦️.ts-balanc-91d78-preserve-individual-answers-desktop/`.

The heading click calls `preventDefault` and `onOpen`. When that open does not change the step, the browser never follows the hash, so the address listener never opens the page.

## Fix

An unmodified primary click on a card heading now writes the link's hash before `onOpen`. A hash change opens the page through the address binding even when the overview drops the open request. Modified clicks still follow the href.

`@semio-tech/course-react` vitest: course shell and navigation, 2 files, 64 passed, including `writes the heading hash when the card open is ignored` and `opens the page of a card from its heading link when the browser does not follow the hash`.

## Re-verify

```
repo run @teaching/architecture-course:test-e2e --configuration=release --parallel=1 -- --topology dev --serial -- --project=desktop --grep=balanced.pools.rotate.bim-concepts
```

| Wave | Result | Duration |
| --- | --- | --- |
| Boot | 1 passed | 24.1s |
| Shared desktop, that spec only | 1 passed | 1.7m |

`passed in 139 s: dev`. Exit 0.

A first re-verify attempt failed before Playwright because Nx split the quoted grep into separate end-to-end options. The command above keeps `--project` and `--grep` after the script's own `--`.

## Live-stack re-verify

`repo://goals` was read (27706 bytes). Ticket `26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY` was already open (`ticket_reopen`: ticket is already open) under `🎯runningframework🎯runningproducts`. Live stack before the browser run: `http://127.0.0.1:6061/` HTTP 200, `http://127.0.0.1:8791/instance` HTTP 200.

### Vitest

```
repo tools exec vitest -- run --config 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts --configLoader native --reporter=verbose 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🧪️tests/📚️course-shell/🟦️.tsx 🧰️framework/🛍️products/📚️course/🧪️tests/🚏️navigation/🟦️.tsx
```

Vitest 4.1.12. Exit 0. 2 files, 64 passed, 16.24s. Includes `opens the page of a card from its heading link when the browser does not follow the hash`.

### Quiz runs

```
PLAYWRIGHT_BASE_URL=http://127.0.0.1:6061 TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=dev repo tools exec node -- node_modules/playwright/cli.js test --config 🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts --workers 2 --project=desktop 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎯️quiz-runs/🟦️.ts
```

Running 15 tests using 2 workers. Started 2026-10-30T23:36:21+01:00, ended 2026-10-31T00:05:38+01:00. Playwright exit 1. **14 passed, 1 failed** (28.3m).

| Result | Test | Duration |
| --- | --- | --- |
| passed | one mistake per task in physics earns partial credit and no badge | 1.1m |
| failed | every quiz is playable to a perfect score, and the perfect runs earn every badge of the course | 5.7m |
| passed | one mistake per task in demand earns partial credit and no badge | 59.9s |
| passed | balanced pools rotate physics and preserve individual answers | 5.0m |
| passed | balanced pools rotate ventilation and preserve individual answers | 5.0m |
| passed | balanced pools rotate heating and preserve individual answers | 4.8m |
| passed | balanced pools rotate cooling and preserve individual answers | 3.9m |
| passed | balanced pools rotate demand and preserve individual answers | 4.0m |
| passed | balanced pools rotate parametric-design and preserve individual answers | 3.6m |
| passed | balanced pools rotate grasshopper-data and preserve individual answers | 4.7m |
| passed | balanced pools rotate excel-workbook and preserve individual answers | 5.2m |
| passed | balanced pools rotate bim-concepts and preserve individual answers | 3.7m |
| passed | balanced pools rotate optimization-decisions and preserve individual answers | 4.1m |
| passed | software mockups render and grade in en | 2.2m |
| passed | software mockups render and grade in de | 1.8m |

All 10 balanced-pool specs passed, including `bim-concepts`. Each of those specs calls `layoutQuiet` and then clicks the learner card link after reload. Both EN and DE software mockups passed. Both partial-credit specs passed.

The perfect-score spec failed in `playQuiz` while opening `parametric-design`: `locator.click` on `[data-layered-card="parametric-design"] [data-overview-card-action="primary"]` exceeded 20s. The locator had resolved to a visible, enabled, stable primary button, scrolling finished, and the click action itself did not complete. The failure snapshot is already the Parametric Design run (heading, medium challenge, 0 of 5 tasks complete, Submit quiz). Navbar: Back Parametrics, Up Parametric Design, connection restored. Evidence: `🗑️generated/quiz-runs-reverify/🎯️quiz-runs-🟦️.ts-every--a0483-n-every-badge-of-the-course-desktop/`.

## Serial quiz-runs on the quiet live stack

Live stack before the run: `http://127.0.0.1:6061/` HTTP 200, `http://127.0.0.1:8791/instance` HTTP 200. Load average 0.93, 5.54, 8.90. No other Playwright process.

```
PLAYWRIGHT_BASE_URL=http://127.0.0.1:6061 TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=dev PLAYWRIGHT_BROWSERS_PATH=/home/deck/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright repo tools exec node -- node_modules/playwright/cli.js test --config 🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts --output .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/quiz-runs-15 --workers 1 --project=desktop 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎯️quiz-runs/🟦️.ts
```

Running 15 tests using 1 worker. Started 2026-10-31T00:13:46+01:00, ended 2026-10-31T00:41:15+01:00. Playwright exit 0. **15 passed, 0 failed** (27.5m).

| Result | Test | Duration |
| --- | --- | --- |
| passed | every quiz is playable to a perfect score, and the perfect runs earn every badge of the course | 3.5m |
| passed | one mistake per task in physics earns partial credit and no badge | 42.5s |
| passed | one mistake per task in demand earns partial credit and no badge | 34.6s |
| passed | balanced pools rotate physics and preserve individual answers | 2.4m |
| passed | balanced pools rotate ventilation and preserve individual answers | 2.2m |
| passed | balanced pools rotate heating and preserve individual answers | 2.2m |
| passed | balanced pools rotate cooling and preserve individual answers | 1.9m |
| passed | balanced pools rotate demand and preserve individual answers | 1.9m |
| passed | balanced pools rotate parametric-design and preserve individual answers | 2.1m |
| passed | balanced pools rotate grasshopper-data and preserve individual answers | 2.2m |
| passed | balanced pools rotate excel-workbook and preserve individual answers | 2.2m |
| passed | balanced pools rotate bim-concepts and preserve individual answers | 1.9m |
| passed | balanced pools rotate optimization-decisions and preserve individual answers | 1.8m |
| passed | software mockups render and grade in en | 52.9s |
| passed | software mockups render and grade in de | 52.7s |

The perfect-score spec that failed in the two-worker re-verify passed. All 10 balanced-pool specs passed, including `bim-concepts`. Both partial-credit specs passed. Both EN and DE software mockups passed. Log: `🗑️generated/quiz-runs-15.log`.
