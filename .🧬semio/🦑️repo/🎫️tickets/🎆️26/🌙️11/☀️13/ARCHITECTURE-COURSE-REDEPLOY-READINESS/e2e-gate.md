# End-to-end gate: dev topology and release rehearsal

Run on macOS arm64, 16 GB, shared with other work; every command with `--skip-remote-cache`, one `repo` command at a time.

## Root causes and fixes

| # | Symptom | Root cause | Fix |
|---|---|---|---|
| 1 | `Complete repository command policy required` before Chromium installs | `ensureBrowser` passed the caller's output grant (`transport`: 128 MiB, **1 000 000 lines**) as the `output` of a repository command policy, whose schema admits at most 65 536 lines (and 256 MiB). The policy check in `runRepositoryCommand` (`owned-execution/🟦️.ts`) refused it. The grant is a ceiling of the caller; a policy may only be narrower. | `browserInstallOutputV1` in `🎭️e2e/🔒️sessions/🟦️.ts` narrows the grant to the policy maxima; unit test (draft-07 via Ajv and the owned validator) in `🧪️tests/🎭️sessions/🟦️.ts`. |
| 2 | Boot spec: `Playwright Test did not expect test() to be called here` | The gate ran the Playwright CLI of the **root** dependency scope (`node_modules/playwright` -> generation `86415d...`), while specs and config import `@playwright/test` through `🎓️teaching/node_modules` (generation `e599d4...`): two copies of `@playwright/test`. | `PLAYWRIGHT_CLI` resolves `playwright/package.json` from the gate module itself, i.e. the scope the specs resolve from. |
| 3 | `a en/de learner receives every localized assignment as a published PDF` (dev) | Spec asked for the link inside `[data-layered-pane=<topic>]`; the product renders the `Assignments` region at screen level (`HomeScreen` -> `AssignmentDownloads`). Spec wrong. | Spec reads the region by role/name (en `Assignments`, de `Übungen`). |
| 3b | same spec (rehearsal only) | The spec used in-page `fetch('/assignments/..pdf')`; the release document's CSP `connect-src` names only the proctor origin. The product's link is a plain `<a download>` (not governed by `connect-src`), so the spec's probe was the violation. Spec wrong. | Spec checks status and `content-type` with `page.request` (outside the page) and the real browser download (suggested name, `%PDF-` signature, size). |
| 4 | `each nested grid pans under glass ...`: expected veil `clear` on hover | Product changed on purpose (glass clears only while the explore control is pressed; hover keeps it whole; documented in `LayeredOverview`, unit-tested in `layeredClearedId`). Spec stale. | Spec asserts `whole` + visible on hover, presses (`clear` + hidden + pane still inert), moves off and releases (`whole` again; a release on the control would open the page). |
| 5 | pet specs `a click on a pet ...`, `no two pets ever overlap ...`, `a control under a pet still receives its click ...`: `no pet the hand can reach among ...` | **Product defect.** `COURSE_PET_CONTROLS` contained `[data-layered-card]`, the host of the whole grid *cell*; the visible card is only a small window inside it. Pressing the empty cell space around a card does nothing (verified in the browser: hash unchanged), yet pets standing there could never be picked up. On the overview almost every pet stands in a cell. | `COURSE_PET_CONTROLS` is now `[data-layered-card] [data-card]` (the card itself); e2e kit `CONTROLS` mirrors it; unit test `hand-topics` asserts the card is a control and its cell host is not. The three specs are unchanged and pass. |
| 6 | release artifact over budget: `the artifact weighs 6551861 bytes, over the budget of 5000000` | The four published assignment PDFs (1 849 985 B) count toward the 5 MB pin that predates them; code, fonts and assets alone are 4 706 712 B. Budget stale, not the build. | Schema-first: `budget.assignmentBytes` (2 500 000) in `🚀️deploy/🧬️schema` + `🔣️.json`; `siteArtifactProblems` charges `assignments/` against it and the rest against `totalBytes`; test and README updated. |
| 7 | rehearsal: `a pet on a ladder on the page of physics (last seen: [])` | Probabilistic pet behaviour: the climber never chose the ladder trip within one walk span of pets' time in about 3 of 14 runs (isolated runs pass). Not reproduced as a defect. | Same assertion with 3x patience (`LADDER_MS`). Residual flake risk remains (see below). |

## "Complete repository command policy required": where it comes from

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts` (`readRepositoryCommandPolicyV1`) validates the policy against `📥️invocation/🧬️schema/🔣️.json`. The browser install does not lack a policy; its policy was invalid (output ceiling above the schema maximum).

## `workspace:deps-browsers` and the stale dependency receipt

- `deps-browsers` runs the same `playwright install chromium` through the dependency sync owner (`🏗️native/🌐️system`), into the same `ms-playwright` directory; the gate's own install is equivalent and self-contained, so the gate does not need to rely on it. Not executed separately (unverified).
- `Store dependency generation receipt stale ... binding-changed` / `provider-changed (@semio-tech/course)` appears on every run. It is the store noticing that the dependency generation of scope `.` was re-derived after workspace package sources (here `@semio-tech/course`, edited concurrently on the shared tree and by these fixes) changed, then relinking `node_modules` to a new generation (new `.cache/b/d/m/j/<id>` directories). Every run that printed it continued and succeeded, so it is self-healing diagnostics, not a defect; I did not find a case where it was followed by a failure. Not root-caused further (unverified why `binding-changed` recurs each run).

## Rehearsal on macOS

The rehearsal runs on macOS: it builds the release site and a production-mode proctor and serves the static site on a loopback port. Only `docker-image-build` (refuses non-Linux-x64 hosts, `🚀️deploy/🐳️docker-image/🟦️.ts`) and what follows it (`docker-image-publish`, `deploy-check`, Docker daemon) are Linux-x64-only; the gate does not use them.

## Unrelated blocker seen

`repo run @teaching/architecture-course:test` currently fails in `@semio-tech/framework-rs:generate` (Rust test targets `kernel-event-receiving`, `original_kernel_retirement`: `harness::capture/receive/post_step` not found), caused by someone else's in-progress kernel edits. The vitest suite of the site could therefore not be re-run through `repo` after my edits (see report for what was run).
