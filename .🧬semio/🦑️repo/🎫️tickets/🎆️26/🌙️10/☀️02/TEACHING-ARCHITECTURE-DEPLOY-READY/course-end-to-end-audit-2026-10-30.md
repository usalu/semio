# Course and Proctor End-to-End Audit

Checked 2026-10-30 about 18:54 Europe/Berlin. No `repo run` was started. Quiz validation used the existing release `proctor` binary. The site budget check reimplemented `siteArtifactProblems` against the retained rehearsal artifact and was cross-checked with the end-to-end log, which only serves after the real function returns no problems.

## Packages

`@teaching/architecture-course` is `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`. `dev` is continuous, ready on port 6061 (`TEACHING_ARCHITECTURE_COURSE_PORT`), and depends on assets, styling tokens, `@teaching/proctor:build`, and both assignment PDF builds. `dev-site` is the site alone. `publish` stages `{projectRoot}/dist/release/pages/course`. `typecheck`, `test`, and `test-e2e` are declared. `test-e2e` defaults to topology `both` in release and depends on the proctor build plus both PDFs. The task router is `📜️script.ts` (`dev`, `dev-site`, `build`, `test`, `test-e2e`, `typecheck`, `check`, `publish`, Docker verbs, `deploy-check`, assignment builds).

`@teaching/proctor` is `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`. `dev` and `serve-release` are ready on port 8791 (`PROCTOR_PORT`). `build` writes `dist/build/prototype` and `dist/build/release`. `check` loads the course through `load_course`. The proctor tree has no uncommitted changes.

`@semio-tech/course` exposes `test` only. `@semio-tech/course-react` exposes `test` and `typecheck`. `@semio-tech/pets` exposes `test`. `@semio-tech/pets-react` exposes `test` and `typecheck`.

## Live stack

| Endpoint | Result |
|---|---|
| `http://127.0.0.1:6061/` | HTTP 200, 4344 bytes, `text/html`, SHA-256 `2658f42d76d9cfd98e9791bb8dc2ef9c2e16ec99ea46925d1745e3201f18c7ef`. Title `Architecture and Technology · Architektur und Technologie`. Vite prototype mode with `/@vite/client` and React refresh. Listener: bun pid 66936, cwd the course TypeScript package. |
| `http://127.0.0.1:8791/instance` | HTTP 200, 1332 bytes, `application/json`, SHA-256 `b5f88949beee4c1643d55054094ea3a3c028e21705d247874dfd01d2180630fb`. `id` `teaching-proctor`, `version` `0.1.0`. Listener: pid 497520. |

Those two hashes match the earlier local readiness note in `website-completion-current-2026-10-30.md`. The process executable path `.🧬semio/🎓️teaching/proctor-bin/proctor-66127-bfc958fe-8f3b-4260-b214-f9f29e368602` is unlinked. `/proc/497520/exe` hashes to `69a3da49e0742dac1d67bf49cf053d7aae549f17a69b13f73eff55644c40da5c` (124,530,256 bytes), the prototype receipt from 2026-10-29T13:42:09Z. The release product is a different binary, `e116019ced585803aff80b59524152777402cca84fb7d5be36ba6ade87af5fed` (10,264,048 bytes, 2026-10-29T16:02:03Z).

Two receipt sources are newer than both binaries (about 14.5 hours after the release binary): `value` derive `controlled-output` and `expand-entries`. The course quiz files and the proctor tree are unchanged since those builds.

## Quizzes

`🎓️teaching/🏛️architecture/📚️course/🔣️.json` lists 10 quizzes. All ten files are present and clean in git. The release `proctor check` exited 0:

- course `architecture`, 10 quizzes, 13 badges, fingerprint `95b3baa21f3b49f1f1d42813bc4534f0b3650358b4acd2e1c509f186db04c0ef`
- `physics` 3 tasks, `ventilation` 3, `heating` 2, `cooling` 1, `demand` 2, `parametric-design` 5, `grasshopper-data` 4, `excel-workbook` 3, `bim-concepts` 3, `optimization-decisions` 2

That acceptance is the 2026-10-29 release binary. A rebuild was not run, so the two newer derive sources are outside this check.

## Release site and `siteArtifactProblems`

`🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/release/pages/course` is absent. Nothing there can pass `siteArtifactProblems`. `publish` is what stages that directory; the live server is the Vite dev site, which carries `/@vite/client` and React refresh and is the development topology.

The retained rehearsal product is `.cache/b/p/m/vite/release/ff5a42b966bfae70/out`. `.semio-artifact.json` owner is `architecture-course:rehearsal` (file mtime 2026-10-30T10:26:19Z). The same mirror of `siteArtifactProblems` used by `publish` reports:

- baked origin `http://127.0.0.1:35729`: 85 files, 4,991,099 bytes, zero problems (budget 5,000,000; margin 8,901 bytes)
- production origin `https://semio.iek.uni-hannover.de`: refused, because this tree bakes the rehearsal proctor (`connect-src` and the loopback address)

The end-to-end log built this key (`Bundling architecture-course:rehearsal`, then `artifact-capture` of that `out` directory) and then printed `rehearsal: static site at http://127.0.0.1:33841`. `runCourseEndToEnd` throws before serving when `siteArtifactProblems` is non-empty, so that serve line is the in-process pass for origin `http://127.0.0.1:35729`. This is a rehearsal artifact. It is the wrong tree to upload to `architektur-und-technologie.de`.

## Latest full both gate

`🗑️generated/course-e2e-both-current-semantics.log` (mtime 2026-10-30T11:48:04Z) is `repo run @teaching/architecture-course:test-e2e --configuration=release --parallel=1 -- --topology both --serial`. It failed after 4909 s: `end-to-end gate failed after 4909 s: specs failed`. Nx reported `@teaching/architecture-course:test-e2e:release` failed. Run directory `🗑️generated/nx/e2e/20261030T102602-1008322`.

| Wave | Playwright `.last-run.json` | Log |
|---|---|---|
| rehearsal boot | passed | 1 passed (12.2s) |
| rehearsal shared | passed | 72 passed (23.0m) |
| rehearsal exclusive | passed | 12 passed (5.9m) |
| dev boot | passed | 1 passed (21.5s) |
| dev shared | failed | 4 failed, 68 passed (42.0m) |
| dev exclusive | passed | 12 passed (9.6m) |

Dev-shared failures, all on the throwaway dev site `http://127.0.0.1:42983`, not on the persistent 6061/8791 pair:

1. `quiz-runs`: every quiz playable to a perfect score and every badge. Timeout 600000 ms. Last locator was `[data-course-grid="root"] > [data-layered-overview]` after the page or browser had closed.
2. `quiz-runs`: balanced pools rotate `bim-concepts` and preserve individual answers. `[data-layered-pane="learner"]` stayed `inert` and `aria-hidden`, attribute value `null` where the spec expected `""` (20s).
3. `quiz-runs`: software mockups render and grade in `de`. `getByRole('region')` inside the Grasshopper mockup matched two elements (scroll region and `Datenbaum` section).
4. `pet-walk`: lively minute throw. Expected at least 3 thrown pets, received 2 (`housy`, `double-curved-surface`).

A lock-order warning appeared while bundling (`semio.compiler.host-capacity` requested while `javascript-dependency-bindings` was held). The rehearsal bundle still completed.

## Uncommitted work and typecheck or unit risk

HEAD on these trees is still the June 2026 history (`3ef757da83e` and earlier). The recent work is the working tree.

In scope of course, pets, and proctor (staffage separated below):

- `@teaching/proctor`: clean.
- The 10 quiz JSON files: clean.
- `@teaching/architecture-course`: staged edits in `project.json` (mtime 2026-10-30T02:13:37Z), `script.ts`, deploy site, builder, e2e, and tests (pet-walk, presence, layered home, phone index, fixtures). One unstaged line in `tests/phone`: grid centering is read from the cell (`alignItems`) instead of the card parent. Mtime 2026-10-30T08:01:59Z.
- `@semio-tech/course` and `@semio-tech/course-react`: 24 staged paths. React app, home, chrome scrollport, record-table CSS, `tsconfig.json`, and tests through 2026-10-30T06:59:49Z. One unstaged test file, `tests/shared-vectors`, mtime 2026-10-30T07:57:05Z, adds a physical schema-oracle import at the top of the file. `@semio-tech/course` has no `typecheck` target, and `course-react` inputs list `tests/*/*.tsx`, so this `.ts` file sits outside that typecheck closure.
- `@semio-tech/pets`: 6 staged schema paths, +1758/−6. The TypeScript schema file is +1307 lines (mtime 2026-10-30T00:29:04Z) plus Rust frame, stage, and schema edits and a parser fixture. No newer `@semio-tech/pets` or `@semio-tech/pets-react` test or typecheck receipt was in the ticket logs searched. `@semio-tech/course-react` depends on the pets TypeScript schema, so a pets schema drift would show up there.

Receipts that do exist, and that are newer than those course and pets source mtimes:

- `🗑️generated/course-react-typecheck-decoder-current.log`, 2026-10-30T09:09:21Z: `@semio-tech/course-react` typecheck succeeded with its 6 dependencies.
- `🗑️generated/course-typecheck-profile-current.log`, 2026-10-30T09:34:37Z: `@teaching/architecture-course` typecheck succeeded with its 6 dependencies.

Those receipts predate the 10:26Z both-gate, which then failed in dev shared. They do not cover a fresh `@semio-tech/course:test`, `@semio-tech/pets:test`, `@semio-tech/pets-react` typecheck or test, or a proctor rebuild after the two newer derive files.

`🎓️teaching` also has 292 uncommitted staffage paths (catalog splits, thermal and transport modules, site and tests). They are outside the course named inputs checked here and were not executed.

## Deploy readiness

The local dev pair answers and the 10 quizzes pass the existing release checker. The rehearsal artifact is under the 5,000,000-byte guard for its own loopback origin. The publish directory is missing, the production origin is not baked into the retained artifact, and the latest full both gate failed on four dev-shared specs after rehearsal and dev-exclusive had passed. That gate is the open course-website blocker.
