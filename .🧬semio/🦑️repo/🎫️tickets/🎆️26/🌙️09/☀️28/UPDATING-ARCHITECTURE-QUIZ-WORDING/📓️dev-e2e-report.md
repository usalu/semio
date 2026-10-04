# 🧱️ Dev stack and end-to-end gate — report

Work package of the reopened ticket (2026-10-01/02): one dev command that starts backend and frontend, and an automated
end-to-end gate for the architecture quiz website.

Status: done. The gate was green three times in a row (section 4); other agents were still editing at that time, so
the last word is one more run of `bun nx run @teaching/architecture-quiz:test-e2e` after everything has landed. Two
taxonomy errors remain, both in directories other agents created (section 4).

## 1. What exists now

| Thing | Where | What |
|---|---|---|
| Local stack | `🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts` | `runDevStack` (proctor, then site, stopped together), `awaitReady`, `launchOwned`, `serveStaticSite` |
| Proctor launcher parts | `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts` | `buildProctor` (async, progress, cancellable), `launchProctor`, `proctorReady`, `proctorOrigin`; `runProctor` is built from them |
| End-to-end gate | `🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🟦️.ts` | boots the `dev` and `rehearsal` stacks, control endpoint, runs Playwright, tears down |
| Playwright projects | `…/🎭️e2e/🎚️config/🟦️.ts` | `boot` → `desktop` + `phone` (parallel) → `presence` → `shortage` |
| Learner driver | `…/🎭️e2e/🚶️learner/🟦️.ts` | device with console/request/CSP guard, journey steps, catalog solutions by item id |
| Specs | `…/🧪️tests/{🚀️site-boot,🪪️first-visit,🥞️layered-home,🎯️quiz-runs,🏆️live-leaderboard,🗣️both-languages,📱️phone,👥️shared-presence,🔌️connection-shortage}/🟦️.ts` | nine spec files, 19 tests per topology |
| Node test | `…/🧪️tests/🧱️local-stack/🟦️.ts` | readiness waits, the gate's command line and ports, owned process tree stop, static origin against Vite's preview server |

### Commands

| Command | Launch row (`.vscode/launch.json` and its seed) | `.claude/launch.json` |
|---|---|---|
| `bun nx run @teaching/architecture-quiz:dev` — proctor, then site | `🛠️dev🏛️architektur-und-technologie❓️quizze` (opens the site when it is ready) | `architektur-und-technologie-quizze` (6061) |
| `bun nx run @teaching/architecture-quiz:dev-site` — site alone | `🛠️dev🎓️teaching🏛️architecture❓️quiz🌐️site` | `architecture-quiz-site` |
| `bun nx run @teaching/proctor:dev` — proctor alone (unchanged) | `🛠️dev🎓️teaching🛂️proctor` | `teaching-proctor` |
| `bun nx run @teaching/architecture-quiz:test-e2e` — both topologies | `⚖️gate🎓️teaching🏛️architecture❓️quiz🎭️e2e` | |
| `bun nx run @teaching/architecture-quiz:test-e2e -- rehearsal` — release rehearsal only | (same row, argument) | |

The compound `🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor` is removed. `package.json` scripts: `dev:site`, `test:e2e`
in the site package; `dev:teaching:architecture-quiz:site`, `test:teaching:architecture-quiz:e2e` at the root.

## 2. Decisions

1. **`dev` is the stack, `dev-site` the frontend alone, both router commands.** The taxonomy's `root-script` grammar
   accepts only delegations in a `📜️script.ts`; a `dev site` subcommand needed `segments.slice(1)`, which it rejects
   (`fixed-source-disposition-unresolved`). Two commands, each one delegation, are accepted.
2. **The stack is plain library code** (`🧱️stack`), no shell and no new dependency: processes are started with
   `node:child_process`, stopped with the repo library's `terminateOwnedChildTree` (process tree on Windows), readiness is
   an HTTP probe (`GET /instance` must answer 200 and name `teaching-proctor`; the site must answer its document).
3. **Reuse, never steal.** A proctor (or site) that already answers on its port is reused and never stopped; only what
   the command started is stopped. A proctor that exits before it is ready fails the command with its status.
4. **The proctor launcher was split, not wrapped.** `buildProctor`/`launchProctor`/`proctorReady` are the parts of the
   existing `runProctor`; the build is asynchronous with a progress line and is cancelled by killing cargo's tree.
5. **Two topologies, one set of specs.** `dev`: Vite dev server + development proctor, one origin through the proxy.
   `rehearsal`: `📜️script.ts build` with `PROCTOR_URL` baked, static files from our own static origin (checked against
   Vite's `preview` in multi-page mode as oracle), proctor in `PROCTOR_MODE=production` with
   `PROCTOR_ALLOWED_ORIGINS=<site origin>` — a real cross-origin, CSP-sealed release. Selectable by argument.
6. **Throw-away everything.** Ports 6161/8891 and 6162/8892, proctor data, catalog copy, Vite dependency cache
   (`<run>/dev/node_modules/.vite`) and the release artifact live in `.🧬semio/🎓️teaching/architecture-quiz-e2e/<run>/`
   (git-ignored); deleted on success, kept with logs, traces and screenshots on failure or `--keep`. Inherited
   `PROCTOR_*` variables are stripped so a developer's environment cannot leak in.
7. **The dev server of the gate does not watch** (`TEACHING_ARCHITECTURE_QUIZ_WATCH=off` → `hmr: false, watch: null`):
   several agents edit sources while it runs and a full reload under a test is not a product failure.
8. **A guard in the device fixture, not in the specs.** Every page of every spec fails on a console error, a page error,
   a failed request that is not an abort, an answer ≥ 400, a WebSocket error and any Content-Security-Policy violation
   (`securitypolicyviolation`), unless the spec announced the failure (`expectingFailures`, used only while the proctor is
   stopped on purpose). Policy violations can never be announced.
9. **Answers come from the catalog sources by item id**, never from the screen: `perfect` gives every item its true
   answer, `flawed` gets exactly one thing wrong per task (classification: one item dragged with the mouse into another
   bin; sorting: two neighbours of different value swapped; matching: the cards of the smallest and largest exchanged).
10. **Order and parallelism.** `boot` warms the cold dev server; `desktop` and `phone` run fully parallel; `presence`
    and `shortage` run alone afterwards because they count who is online and stop the proctor. Four workers for one
    topology, two each when both run (measured: more is slower on this 16-thread host, a driven page keeps a core busy).
    A step fails after 20 s without what it expects; a whole test may take 600 s.
11. **Fresh checkout.** `dev`, `dev-site` and `test-e2e` depend on `@semio-tech/assets:build` and
    `@semio-tech/ui-styling-tokens:generate` like `build` does; the rehearsal's in-process build is covered by the
    dependency of `test-e2e`. Proven by moving both generated files aside and running the rehearsal boot spec.
12. **Chromium** comes from `PLAYWRIGHT_BROWSERS_PATH` if set, else the repo tool cache; it is downloaded once when missing.

## 3. Verified behaviour of the dev stack on this host (Windows 11)

Checked with `dev_stack_ctrl_c.ps1` (this ticket folder), which starts the command in its own console, sends the stop
and then lists what is left of the command's process tree and who listens on the ports.

| Command | Stop | Result |
|---|---|---|
| `bun ./📜️script.ts dev` (ports 6171/8871) | Ctrl+C | `PASS: nothing left`; stack settled in 3.3 s; the proctor logged `interrupt received; stopping` and `proctor stopped` |
| same | Ctrl+Break | `PASS: nothing left` |
| same | console window closed | `PASS: nothing left` |
| same | process tree killed | `PASS: nothing left` |
| same, with a proctor already running on the port | Ctrl+C | `reusing the proctor …`; the foreign proctor still answered 200 afterwards |
| `bun nx run @teaching/architecture-quiz:dev` (6061/8791) | process tree killed | `PASS: nothing left` |
| same | Ctrl+C | both ports free, every stack process gone; only the outer `bun nx` wrapper waits at cmd's `Terminate batch job (Y/N)?` when the console is not interactive — identical for the pre-existing `@teaching/proctor:dev`, see Open 1 |

Failure paths (node test `🧱️local-stack` and by hand): a proctor that exits early → `the proctor … exited with status N
before it became ready`; never ready → `… did not become ready within N s`; cancelled → `waiting for … was cancelled`.

## 4. Commands and results

All on this host (Windows 11, 16 threads), 2026-10-02 local time, while other agents built and load-tested on it.

### The required three consecutive green runs

`bun nx run @teaching/architecture-quiz:test-e2e` — both topologies, the proctor built from the working tree each time,
production defaults (rate limits included) in the rehearsal, nothing skipped, no retries:

| Run | Time | Result |
|---|---|---|
| 1 | 03:33:51 – 03:46:39 | `[rehearsal] 19 passed (9.0m)`, `[dev] 19 passed (10.5m)`, `[e2e] passed in 660 s: dev, rehearsal` |
| 2 | 03:46:39 – 03:58:51 | `[rehearsal] 19 passed (9.3m)`, `[dev] 19 passed (10.8m)`, `[e2e] passed in 667 s: dev, rehearsal` |
| 3 | 03:58:51 – 04:09:41 | `[rehearsal] 19 passed (7.5m)`, `[dev] 19 passed (9.6m)`, `[e2e] passed in 592 s: dev, rehearsal` |

The run directly before them (03:19 – 03:32, same specs, test limit still 300 s) was green too: `19 passed` twice,
`passed in 769 s`. No request was throttled: the guard fails on any answer ≥ 400, and none occurred.

**What these runs cover.** Other agents were still editing during them: the last product change they include is
`🎓️teaching/🛂️proctor/🔨️modules/🎚️config/🦀️.rs` of 03:56 (run 3 built it). Anything changed after 03:58 is not covered;
rerun the command above once everything has landed.

### Runs that were not green, and why

| Time | Result | Cause |
|---|---|---|
| 02:19 | rehearsal `19 passed`; dev `1 failed` (`one mistake per task in physics …`: item still in its old bin after the drag), `3 did not run` | the spec's drag aimed at a stale point (section 5); another agent's gate run at 02:27 failed the same way in the rehearsal |
| 02:41, 02:45 | `cargo build of teaching-proctor failed (101)` after 5 s and 7 s | `SortingAnswer: Eq` missing in `🧰️framework/🛍️products/❓️quiz/🧬️schema/🦀️.rs`, mid-edit by another agent; compiled again minutes later |
| 02:36 | `the dev stack needs port 6161 and 8891, which something else listens on (another run of this gate?)` | another agent's gate run held the ports |
| 02:47 | rehearsal `19 passed`; dev `2 failed`, `3 did not run` | `net::ERR_NO_BUFFER_SPACE` on a request and a WebSocket: the host ran out of socket buffers while other agents load-tested (4 454 connections in `TIME_WAIT` measured on the host during a later run; the gate itself holds at most about 950). The guard reported it, as it must. |

### Everything else

| Command | Result |
|---|---|
| `bun nx run-many -t test -p @teaching/architecture-quiz @teaching/proctor @semio-tech/quiz-react @semio-tech/quiz --skip-nx-cache` (04:10) | `Successfully ran target test for 4 projects` |
| `bun ./📜️script.ts test` in the site package | `Test Files 4 passed (4)`, `Tests 40 passed (40)` |
| `bun ./📜️script.ts test` in the quiz React package, after bug 2 | `Test Files 16 passed (16)`, `Tests 365 passed (365)`; before the fix the new test failed with `expected '' to be 'cold-wet'` |
| `bun ./📜️script.ts test-e2e --grep "site boots\|one mistake\|perfect score\|leaderboard\|phone"` after the drag rewrite | `[rehearsal] 7 passed`, `[dev] 7 passed`, `passed in 262 s` |
| `bun nx run @teaching/architecture-quiz:test-e2e -- rehearsal --proctor <last good binary>` (earlier) | `19 passed`, 232 s |
| fresh checkout: both generated files moved aside, then `bun nx run @teaching/architecture-quiz:test-e2e -- rehearsal --project boot --proctor …` | both regenerated by the prerequisites, boot spec passed |
| `bun <ticket>/site_infra_launch.ts` (dry run of the launch generator) | `rows 1485 -> 1485; identical=true` |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` | `clean=false errors=1`: `directory-kind-unresolved 🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity` — another agent's new directory; nothing of this work package |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | `clean=false errors=1`: `directory-kind-unresolved 🧰️framework/🛍️products/❓️quiz/🧪️tests/🖼️task-icons` — another agent's new directory; nothing of this work package |
| release artifact of the rehearsal | `connect-src http://127.0.0.1:8892 ws://127.0.0.1:8892`; no policy violation in any spec |
| idle processor use of one page (`dev_e2e_idle_cpu.ts`) | introduction 0.3 %, overview 0.4 %, run 0.0 % of one core |

## 5. Bugs found

| # | Where | Bug | Fix |
|---|---|---|---|
| 1 | `🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts` | On Windows `@semio-tech/assets:build` failed with `asset output escapes owned roots`: containment was tested with `` `${root}/` `` against paths that use `\`. A fresh checkout therefore could not build or serve the site. | `below(path, root)` compares with `path.sep`; `@semio-tech/assets:build` and `check-generated` pass; the regenerated shortcodes are complete (the committed file was stale). |
| 2 | `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🤏️drag/🟦️.ts` | While an item of a classification is dragged, its ghost showed "Not assigned" instead of the category the item holds: `cloneNode` does not carry over what a `select` shows. Seen in the trace of a failed gate run. | `ghostOf` gives every select of the copy the value of the one it copies. Test first: `🧪️tests/⌨️task-keyboard` "drags a ghost that shows the category the item holds, and Escape abandons the drag" (failed with `''`, passes now); the gate asserts it in a real browser on every drag. |

No defect of the proctor, the domain or the site's shell was found by the specs. Everything else that failed during
development was the spec's own (listed here because each cost a run):

- The others' answers appear and disappear under every item while a learner works (`CrowdChoices`), which moves the
  page under a pointer that is mid-drag. The first drag helper aimed at a point measured before the drag and missed the
  bin by a few pixels when a peer left the task. `dragInto` now drags like a hand: takes the grip where it is, follows
  the bin until it lights up (`quiz-drop-active`), then lets go. See Open 3.
- A dependency cache outside a `node_modules` directory made `@vitejs/plugin-react` transform optimized dependencies
  (the app did not mount) → the throw-away cache is `<run>/dev/node_modules/.vite`.
- Six plus six fully parallel workers starved each other (180 s timeouts) → four, or two each.

## 6. Open

1. **`bun nx …` and Ctrl+C in a non-interactive Windows console.** The stack stops completely; the `cmd /c` wrapper nx
   uses then asks `Terminate batch job (Y/N)?` when nobody can answer. In an interactive terminal (VS Code's launch
   rows) Ctrl+C ends it as usual. Pre-existing and the same for every `nx run-commands` target; not caused by the stack.
2. **A hard kill of only the supervising process** (Task Manager "End task" on the one `bun` process, not its tree)
   cannot be intercepted on Windows; its children keep running until the next `dev` reuses or the developer stops them.
   Ctrl+C, Ctrl+Break, closing the terminal and killing the tree are all handled.
3. **Layout moves while the crowd comes and goes.** A line with the others' answers under each item appears when the
   first peer joins a task and disappears when the last leaves, shifting everything below by the height of the lines
   (and the scroll position with it). A hand follows it; reserving the line's space would keep the page still. A design
   decision of the client, not changed here.
4. **Two gate runs at once collide on the fixed ports**; the second fails at once with `the dev stack needs port 6161
   and 8891, which something else listens on (another run of this gate?)`. Happened once when another agent's readiness
   gate ran at the same time.
5. **Duration.** Both topologies together took 397 s on this host when it was otherwise quiet and 592–769 s beside the
   other agents' builds. The longest tests walk every screen in both languages and wait for the leaderboard's own
   polling (three to five minutes under load). The limit of a whole test is therefore 600 s; every single step still
   fails after 20 s.
6. **The host's sockets are shared.** A flood or capacity test of another agent can exhaust them
   (`net::ERR_NO_BUFFER_SPACE`), which the gate reports as the failure it is. Run the gate when the host is not
   load-tested.
7. **Kept by someone else:** `.🧬semio/🎓️teaching/architecture-quiz-e2e/20261002T002714-39256/` is the run directory
   of another agent's failed gate run (02:27, the stale drag); left in place.
8. **Type errors outside this work package** seen when type-checking the site sources ad hoc:
   `🏗️builder/🌐️vite/🟦️.ts(110)` (`OwnedBuildPlugin` vs Vite's `PluginOption`) and `🚀️deploy/🟦️.ts(444)` (identity
   claim literal). The site has no typecheck target; both files run.

## 7. Files

Created

- `🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🎚️config/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🚶️learner/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🚀️site-boot/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🪪️first-visit/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🥞️layered-home/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🎯️quiz-runs/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🏆️live-leaderboard/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🗣️both-languages/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/📱️phone/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/👥️shared-presence/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🔌️connection-shortage/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🧱️local-stack/🟦️.ts`
- ticket folder: `dev_stack_ctrl_c.ps1`, `dev_e2e_register_taxonomy.ts`, `dev_e2e_script_grammar.ts`,
  `dev_e2e_idle_cpu.ts`, `📓️dev-e2e-report.md`

Updated

- `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts` — launcher split into build, launch, readiness
- `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📜️script.ts` — `dev`, `dev-site`, `test-e2e`
- `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📋️project.json` — targets, prerequisites, inputs
- `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/package.json`, root `package.json` — scripts
- `🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts` — `cacheDir` and the frozen dev server of the gate
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🎚️config/🟦️.ts` — `🧱️local-stack` in the node tests
- `🎓️teaching/🏛️architecture/❓️quiz/README.md`, `🎓️teaching/🛂️proctor/README.md` — Develop sections
- `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` (regenerated), `.claude/launch.json` — rows
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — kinds `teaching-quiz-stack`,
  `teaching-quiz-e2e`, `teaching-quiz-e2e-learner`, nine new members of the site's tests
- `🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts` — bug 1
- `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🤏️drag/🟦️.ts`,
  `🧰️framework/🛍️products/❓️quiz/🧪️tests/⌨️task-keyboard/🟦️.tsx` — bug 2
- generated by `@semio-tech/assets:build` after bug 1: the assets' shortcode files (complete emoji map)

Removed

- launch compound `🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor` (seed and generated file)
- `<ticket>/🗑️generated/dev-e2e/` (logs, backups, probe) and the kept run directories under
  `.🧬semio/🎓️teaching/architecture-quiz-e2e/`
