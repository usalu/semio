# Course React UI Repair

## Scope And Contract

The current course schema owns `CourseView`, `PresencePresentation`, and cursor `place`/`home`. The React presence rooms still consumed an obsolete catalog field and omitted public presentations, cursor surfaces, nested home routes, and two watched pages. The repair follows the authored schema and existing language-neutral presence fixture instead of introducing aliases.

Presence now accepts the app's course identifier, publishes public presentations, publishes the cursor's current place and nested home route, clears pointers when their visible surface changes, watches learner/preferences pages, and joins their cursor rooms. Results retain the visible task. Peer projection rejects stale surfaces, away or absent roster learners, mixed quiz screens, and mismatched nested overview routes. Existing self-session and thinking-room behavior remains covered by the suite.

The React views now import their declared anchors, follow hook, peer overlay and current schema types. Place labels resolve the course translation keys. The introduction imports the shared introduction component lazily and removes duplicate obsolete imports. Pets import the layout hook and declare their already-produced shiny species input.

## Validation

- Before repair, the actual presence-client suite executed: **29 failed, 32 passed, 3 uncaught errors**. The uncaught errors identify the undefined presence anchor constant.
- After the presence repair: **61 passed** in jsdom, including shared JSON place/cursor/watch vectors, Lodash throttle and clamp oracles, socket state publication, reconnection, semantic cursor placement, and rendered accessible roster/profile/preferences checks.
- The first full configured React run completed: **23 passed / 15 failed files; 787 passed / 33 failed tests** (820 executed). Its diagnostics exposed stale fixture imports, missing moved fixture bindings, tests importing the deliberately split run component from the root entry, old unqualified/quiz-qualified protocol expectations, old introduction-screen expectations, and a stylesheet test reading only the root import list.
- Onboarding and learner profile suites pass (**16 tests**); the follow suite passes (**25 tests**) after importing the run component from its existing public subpath. The run/results remain split from the root entry.
- Corrected the moved sheet/live-region fixture paths, missing followed-layout and pet-context fixture imports, current state initializer imports, and current course fixture names. The keyboard stylesheet test now uses the public concatenated stylesheet source. The expanded affected-UI rerun is in progress.
- The root's mapped public-export TypeScript diagnostic reported no course React diagnostics after the source repairs. The shared layered pane's missing `LayeredRevealSource` import is repaired as well.
- The React Vitest config now uses the generic authored public workspace export resolver, with generated cache under this ticket when `SEMIO_TICKET_DIR` is set. This makes verification independent of materialized volume artifacts and removes stale quiz aliases.

These are source and jsdom checks. They do not establish native browser, container, deployment, or host grant behavior.

## Files

- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/👥️presence/🧭️rooms/🟦️.ts`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/👥️presence/👁️view/🟦️.ts`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/📇️profile/🟦️.tsx`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🎛️preferences/🟦️.tsx`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/👋️introduction/🟦️.tsx`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🐾️pets/🎪️stage/🟦️.tsx`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- This report.

Additional authored test repairs cover the follow, pet allowances, challenge clock/views/shared kit, home grid/shared kit, task icons, live regions, pet companions, and keyboard suites. They update the tests to existing public contracts rather than adding obsolete API exports.

The eight-suite expansion executed **212 tests: 119 passed, 93 failed** before the final fixture/selector repairs. Home-grid, follow, task-icons and live-regions passed; the remaining failures exposed old `.quiz-*` selectors in the clock/hint tests, missing shiny fixture/retry helper imports, the current German guess-field guidance, and an old assertion forbidding incomplete submission. These authored tests now use `.course-*`, the existing current helper exports and fixture files, current wording, and verify the incomplete-task confirmation list before submission.

The subsequent six-suite attempt stopped during genuine read-only workspace custody verification because the teaching installation's published workspace patterns drifted. The parent authorized the launch agent to publish the current source-only ownership projection. After that succeeded, nine UI suites executed **261 tests: 259 passed, 2 failed; 8 files passed**. The only remaining failures were two stale hint-test expectations: `data-quiz-item` instead of `data-course-item`, and the current nonpositive-number German guidance. Both are corrected, and the native-loader hint rerun passes **89/89 tests**. Presence, clock, pet allowances, pet companions, challenge views, keyboard, onboarding and profile all passed the nine-suite run. These results establish all nine owned UI suites individually passing; no counts from repeated tests are added together. The config awaits the real async resolver and selects the teaching installation that owns this framework dependency. No source ownership policy was weakened or synthesized to run the tests.

The final test changes also include `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🧵️pane/🟦️.tsx` (missing domain-neutral type import). No app module change was required: its existing presence input is now the input the rooms implement.

The existing registered React `📜️script.ts` test command now forwards `--configLoader native`, avoiding bundled config artifacts outside the ticket. Its typecheck owner label now names course-react. No executable command was added, so its existing launch entry remains the entry point.

A bounded no-resolve TypeScript binding audit of seven repaired UI suites reports zero undefined local bindings after restoring the missing `CourseLocale` type import. This audit is narrower than a full typecheck and is not a runtime pass.

## Current Full Course TypeScript Diagnostic

Executed the current teaching course package's authored `tsconfig.json` with TypeScript's real parser and program, `noEmit: true`, and `incremental: false`. The in-memory paths map came from the current authenticated teaching workspace public exports, with a 180-second cancellation signal. Explicit `.mjs` runtime exports select an already-existing `.d.mts` sibling where present; no declaration was generated or synthesized. All diagnostics are recorded in `🗑️generated/current-source-typecheck.log` until ticket output cleanup.

The result is **8 diagnostics**, all in the existing generated UI-axes prerequisite closure:

| File | Line | Code | Classification |
| --- | ---: | ---: | --- |
| `🧰️framework/🔨️modules/🎠️kernel/🏷️notice-labels/🟦️.ts` | 6 | 2307 | Missing generated `framework/manifest/generated/ui-axes` module |
| Same file | 144 | 7053 | Dependent unresolved `ShellLocale` index type |
| `🧰️framework/🔨️modules/🎠️kernel/🕰️history/🟦️.ts` | 6 | 2307 | Missing generated UI-axes module |
| Same file | 186 | 7053 | Dependent unresolved `ShellLocale` index type |
| Same file | 190 | 7053 | Dependent unresolved `ShellLocale` index type |
| `🧰️framework/🔨️modules/🎠️kernel/🕸️plugin-graph/🟦️.ts` | 6 | 2307 | Missing generated UI-axes module |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts` | 14 | 2307 | Missing generated UI-axes module |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` | 8 | 2307 | Missing generated UI-axes module |

There were no course React, presence, exact-Cargo, layered-pane type-import or inventory-tooling declaration diagnostics in that program. The typecheck did not pass. The original classification as a generated-source prerequisite was incorrect: the actual generated UI-axes source exists; the package public export targeted a missing package wrapper. See manifest-export-repair.md for the authored wrapper repair and independent resolution/runtime proof. No fake generated file, native compiler admission, or host grant was introduced.

The root reran the actual current-source TypeScript program after that repair. All eight diagnostics above disappeared. Four new diagnostics in concurrently changed Windows fixed-storage FFI generic typing were then repaired by preserving the exact security-library opener's inferred signature. The final root rerun exited zero with **zero diagnostics and 3,174 source reads**, recorded in `🗑️generated/current-source-typecheck-complete.log`. It uses the actual authored course tsconfig and current controlled public exports, with only already-existing declarations selected for runtime `.mjs` exports. It emits no files and is not claimed as the canonical managed typecheck gate. See native-system-typing.md and readiness-status.md.

## Final Handoff

The UI product repair and complete configured diagnostic verification are complete: the root-owned React run exited zero with **38 files and 1,032 tests passed**, recorded in `🗑️generated/ui-full-react-root.log`. The earlier agent-owned full run stopped before completion and its process identifier could not be transferred across agent sessions; it is not used as passing evidence. The root restarted and observed the complete run itself. This is a Node/jsdom diagnostic, not native release-browser proof. The parent owns ticket closure and generated-output cleanup once the full deployment objective is achieved.

Additional changed files (relative to `🧰️framework/🛍️products/📚️course`):

- `🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧪️tests/👀️follow/🟦️.tsx`
- `🧪️tests/🎛️pet-allowances/🟦️.tsx`
- `🧪️tests/⏱️challenge-clock-views/🟦️.tsx`
- `🧪️tests/🪜️challenge-views/🟦️.tsx`
- `🧪️tests/💬️challenge-hint-views/🟦️.tsx`
- `🧪️tests/🪞️challenge-views-kit/🟦️.ts`
- `🧪️tests/🏠️home-grid/🟦️.tsx`
- `🧪️tests/🛠️home-grid-kit/🟦️.tsx`
- `🧪️tests/🖼️task-icons/🟦️.tsx`
- `🧪️tests/📢️live-regions/🟦️.tsx`
- `🧪️tests/🐾️pet-companions/🟦️.tsx`
- `🧪️tests/⌨️task-keyboard/🟦️.tsx`

The remaining full-run session/deputy/playthrough/journey failures have been handed to the runtime agent for current qualified protocol and onboarding fixture reconciliation. The full run's generated log remains under the ticket's generated folder until the parent cleans ticket outputs.
