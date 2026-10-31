# Final Completion Summary

Date: 2026-10-30. Ticket `26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY`. Workspace `/home/deck/semio`.

The architecture-course deploy gates below are green. Live services were probed again at `2026-10-30T23:43:22Z` and left running. Tool-generated outputs under `🗑️generated` were removed after this report was written. The markdown audits named below remain the evidence.

## Live pair

| Endpoint | Result |
| --- | --- |
| `http://127.0.0.1:6061/` | HTTP 200, `text/html`, 4360 bytes, title `Architecture and Technology · Architektur und Technologie` |
| `http://127.0.0.1:8791/instance` | HTTP 200, `application/json`, 1332 bytes, `id` `teaching-proctor`, `version` `0.1.0`, module `teaching.proctor` |

No service was restarted for this audit.

## Root-cause fixes

These are the defects that kept the release browser gate and the teaching unit suite red, and the repairs that the green runs below actually executed.

### Presence handshake oracle

`@teaching/architecture-course` presence tests timed out at 5s even though the product handshake had already returned `timed-out` or `aborted`. A bare `require("ws")` inside Vitest is rewritten to a client that stays `CONNECTING` and never applies `handshakeTimeout` or `AbortSignal`. `loadWsOracle` loads `index.js` beside the resolved `ws` package so the oracle matches Node. Production code was unchanged. After the fix the teaching suite passed 19 files and 826 tests, and the presence file passed 6/6. Report: `unit-verification-after-fixes-2026-10-30.md`.

### Learner pane after reload

Balanced-pool specs click `a.course-card-link` (`href="#learner"`) after an expert reload. The click completed and the URL stayed on `/?preview=1`. For 20s `[data-layered-pane="learner"]` stayed `inert` and `aria-hidden`, without `data-opened`. The same click had opened the learner page on the easy, medium, and hard reloads of the same test. The heading handler called `preventDefault` and `onOpen`. When that open did not change the step, the browser never followed the hash, so the address listener never opened the page.

Repair: an unmodified primary click on the card heading writes the link hash before `onOpen`. A hash change opens the page through the address binding even when the overview drops the open request. Modified clicks still follow the href. `@semio-tech/course-react` then passed the course-shell and navigation files, 2 files and 64 tests, including `writes the heading hash when the card open is ignored` and `opens the page of a card from its heading link when the browser does not follow the hash`. The desktop `bim-concepts` spec passed on re-verify, and all 10 balanced-pool specs passed inside the later 15/15 quiz-run. Reports: `quiz-runs-reload-and-timeout-fix-2026-10-30.md`, `dev-topology-e2e-full-run-2026-10-30.md`.

### Perfect-score timeout

`every quiz is playable to a perfect score, and the perfect runs earn every badge of the course` reached the 600s budget inside `openGrid` for `bim`, after excel-workbook had already finished. The failure was an after-hook bounding-box read on a closed page. `boxOf` waited again for the root overview to attach, then the alignment poll called it again. Once the active grid is visible, the helper reads the root and active overview rectangles in one `page.evaluate` and polls that delta. The 600s budget was not raised. The spec later passed in 3.5m on the live dev stack, and again inside the serial 15/15 quiz-run. Reports: `quiz-runs-reload-and-timeout-fix-2026-10-30.md`, `verify-four-e2e-fixes-2026-10-30.md`.

### German mockup region

`software mockups render and grade in de` called `getByRole("region")` inside the Grasshopper figure. The scroll surface (`div.course-mockup-scroll`) and the data tree (`section.course-tree-panel`, accessible name `Datenbaum`) are both regions, so `focus()` failed strict mode. The spec now takes `.course-mockup-scroll[role="region"]`, then still focuses it and checks `aria-labelledby`. The spec passed in 45.5s on the live dev stack and again inside the 15/15 quiz-run. Report: `mockup-and-pet-walk-fix-2026-10-30.md`.

### Pet-walk throw budget

The lively-minute pets spec expected at least three thrown pets and observed two (`housy`, `double-curved-surface`). Each continuous carry cost about 14–17s, and the perch wait after `mouseup` then held the loop until the same pet was perched. A parachute glide kept that pet in the air while the `LIVELY_MS - 15s` guard expired, so the last 15s only collected the overlap watch. The loop now runs for the whole minute, prefers a pet not yet thrown, and after `mouseup` waits only until that pet's footing is no longer `hand`. A glide frees the hand immediately. The pets project passed the spec in 1.4m on the live dev stack. Report: `mockup-and-pet-walk-fix-2026-10-30.md`.

Earlier course repairs that these green waves already include, and that this closing pass did not reopen: feedback rows matched by item id, Records content folding, phone host alignment, locked-topic keyboard titles, presence card scroll into view, and the assignment PDF lossless publication. Their audits stay in this ticket.

## Unit tests

Direct Vitest 4.1.12 through `repo tools exec vitest -- run`, each package config, `--configLoader native`.

| Package | Result |
| --- | --- |
| `@semio-tech/course-react` | 43 files, 1082 tests, passed in 38.86s |
| `@teaching/architecture-course` | 19 files, 826 tests, passed in 11.21s |
| `@semio-tech/course` | 19 files, 595 tests, passed in 6.21s |
| `@semio-tech/pets` | 34 files, 1164 tests, passed in 6.42s |
| `@semio-tech/pets-react` | 9 files, 218 tests, passed in 4.44s |
| Architecture course, six non-browser files | 6 files, 125 tests, passed in 16.60s |
| Course shell and navigation after the heading-hash repair | 2 files, 64 tests, passed in 16.24s |

The six non-browser files are assignment PDFs (14), local stack (43), deploy (41), dashboard (4), sessions (12), and continuous (11). Live dev PDFs at that check were HTTP 200 `application/pdf`: English 169673 bytes, German 169861 bytes. Reports: `unit-verification-after-fixes-2026-10-30.md`, `course-and-pets-typecheck-and-unit-audit-2026-10-30.md`, `course-non-browser-suites-verification-2026-10-30.md`, `dev-topology-e2e-full-run-2026-10-30.md`.

## Typecheck

`repo tools exec tsc -- --noEmit --pretty false -p <tsconfig>`. Each exited 0 with no diagnostics.

| Package | Project |
| --- | --- |
| `@semio-tech/course-react` | `framework/products/course/targets/react/packages/typescript/tsconfig.json` |
| `@semio-tech/pets` | `framework/products/pets/packages/typescript/tsconfig.json` |
| `@semio-tech/pets-react` | `framework/products/pets/targets/react/packages/typescript/tsconfig.json` |
| `@teaching/architecture-course` | `teaching/architecture/course/packages/typescript/tsconfig.json` |

Earlier full Nx typechecks on the same trees also exited 0 with zero diagnostics: course-react owner `13408`, architecture-course owner `28639`, generic UI React owner `62827`. The release `publish` after the heading-link change ran 14 tasks, cache 0/14, and exited 0. Report: `course-and-pets-typecheck-and-unit-audit-2026-10-30.md`.

## Proctor Rust

From `🎓️teaching`, `repo cargo --store=shared --configuration=release --parallelism=1 -- test --workspace -- --test-threads=1`. Exit 0. Wall time 10m 22s. Release compile 9m 14s. The teaching workspace is the three proctor crates.

| Target | Result |
| --- | --- |
| `teaching-proctor` lib | 25 passed, 1.55s |
| `teaching-proctor` bin | 0 tests |
| `conformance` | 15 passed, 0.28s |
| `end_to_end` | 21 passed, 5.95s |
| `teaching-proctor-base` lib | 41 passed, 1.50s |
| `teaching-proctor-domain` lib | 28 passed, 0.20s |
| doc-tests | 0 tests |

130 passed, 0 failed, 0 ignored. One unused-import warning in `projections` unit tests did not fail the run.

`repo run @teaching/proctor:check` with the absolute architecture course file exited 0. Course `architecture`, 10 quizzes, 13 badges, fingerprint `95b3baa21f3b49f1f1d42813bc4534f0b3650358b4acd2e1c509f186db04c0ef`. A repo-relative course path joined onto the package directory and exited 1; that path is not the acceptance command.

`repo run @teaching/proctor:health` exited 0. It opened the already-live listener on `127.0.0.1:8791` and required `GET /instance` status 200. The capacity gate was not part of this verification. Report: `proctor-backend-verification-2026-10-30.md`.

## Release site

`SEMIO_BUILD_MODE=release NX_DAEMON=false SEMIO_LANGUAGE=en repo run @teaching/architecture-course:publish --configuration=release --parallel=1 --output=stream`, started `2026-10-30T22:35:30Z`. Exit 0. Duration 26m 11s. Nx reported `Successfully ran target publish`. The publisher staged the site for `https://architektur-und-technologie.de` against `https://semio.iek.uni-hannover.de` and printed no rehearsal warning.

`verify-staged-site.ts` ran `siteArtifactProblems` on `dist/release/pages/course`, including `_headers`.

| Check | Result | Budget |
| --- | --- | --- |
| Problems | 0 | — |
| Files | 86 | — |
| On-disk bytes | 4,991,648 | 5,000,000 |
| Topology | `publish` | — |
| Document scripts, gzip | 222,244 | 250,000 |
| Document stylesheets, gzip | 41,479 | 60,000 |
| Baked proctor | `https://semio.iek.uni-hannover.de` | — |
| `CNAME` | `architektur-und-technologie.de` | — |
| `index.html` equals `404.html` | yes | — |
| `_headers` | present | — |

Margin under 5,000,000 bytes: 8,352. File-list sha256 `ffdaf4c31edf64232a0c7df046ce1ed20364c29ce1f099209df077f1be1971c1`. Report: `release-site-publish-verification-2026-10-30.md`.

## End-to-end

### Rehearsal, 85/85

Release both-topology gate, run directory `nx/e2e/20261030T102602-1008322`, log `course-e2e-both-current-semantics.log`. Command: `repo run @teaching/architecture-course:test-e2e --configuration=release --parallel=1 -- --topology both --serial`.

| Wave | Result |
| --- | --- |
| Boot | 1 passed (12.2s) |
| Shared (desktop, phone, layout, pets) | 72 passed (23.0m) |
| Exclusive (presence, shortage, away, faults) | 12 passed (5.9m) |

1 + 72 + 12 = 85/85. That same process then continued into dev and failed four dev-shared specs. Those four are the learner-pane, perfect-score, German mockup, and pet-walk defects above. Rehearsal itself had already passed. Report: `course-end-to-end-audit-2026-10-30.md`.

### Dev topology

Full dev serial release run, directory `nx/e2e/20261030T204911-1220686`:

| Wave | Result | Duration |
| --- | --- | --- |
| Boot | 1 passed | 23.5s |
| Shared | 71 passed, 1 failed | 46.6m |
| Exclusive | 12 passed | 10.5m |

The single shared failure was `balanced pools rotate bim-concepts and preserve individual answers`, the learner-pane defect. Exclusive still ran and passed 12/12. After the heading-hash repair, the same spec on desktop passed with boot (1 + 1, exit 0, 139s).

Serial quiz-runs on the quiet live stack (`http://127.0.0.1:6061`, one worker, desktop project) then passed 15/15 in 27.5m, Playwright exit 0, from `2026-10-31T00:13:46+01:00` to `2026-10-31T00:41:15+01:00`:

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

A two-worker quiz-run immediately before that serial run was 14/15. The perfect-score spec failed on a 20s primary-button click while the run page was already open. The serial rerun on a quiet host passed that spec. Report: `dev-topology-e2e-full-run-2026-10-30.md`. The three focused live-stack repairs (bim-concepts, German mockups, pet-walk) and the separate perfect-score spec had already passed before the full dev topology run; report: `verify-four-e2e-fixes-2026-10-30.md`.

## Final state

Local architecture-course acceptance for this ticket is complete: unit suites, the four package typechecks, proctor release tests plus check and health, the production publish tree (86 files, 4,991,648 bytes, 0 problems), rehearsal 85/85, dev exclusive 12/12, and dev quiz-runs 15/15. The managed pair on ports 6061 and 8791 was healthy at the probe above.

Historical ledger rows for cold-caller laws, Energy post laws, Darwin directory census, native coalescing, and a public university `/instance` probe were not re-executed in this closing pass. They stay in `completion-gates-2026-10-28.json` as their last recorded outcomes. This summary does not convert those rows into new green results.
