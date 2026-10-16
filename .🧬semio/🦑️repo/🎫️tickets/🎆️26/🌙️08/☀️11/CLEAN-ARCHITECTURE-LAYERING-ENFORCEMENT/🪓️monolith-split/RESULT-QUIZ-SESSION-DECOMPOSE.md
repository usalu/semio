# Quiz Session Decompose (slice QUIZ-SESSION-DECOMPOSE, prefix `qsd`)

Executor: Sonnet, 2026-10-16. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `S` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🧭️session`. `Q` = `🧰️framework/🛍️products/❓️quiz`. Tools: `F/qsd-tools/` (`vitest.sh`, `summ.mjs`, `importgraph.mjs`, `taxcheck.ts`, `orig/session.ts` = the pre-edit copy of the god class). Logs: `F/🗑️generated/qsd-*`. No sub-agent, no ticket tool, no modifying git command.

## 1. Verdict

`QuizSession` (857-line class, 40 private methods, 979-line file) is now a **254-line thin composer plus 12 collaborators**, each in `S/<emoji>️<kebab>/🟦️.ts`, each owning its fields and an explicit narrow API. The largest file in the folder is the composer (254 lines); the largest new collaborator is `📖️proctor-reads` (160). The public API of `QuizSession` is unchanged (same member names, signatures and semantics), so no consumer, stub or test had to change.

- quiz-react suite: **926 tests, 926 passed** in the final run. The title multiset equals the baseline's 923 titles plus exactly the 3 titles of the new playthrough test (`diff` of the sorted rows: only those 3 added lines). Two of three full runs had the one known timing flake of the baseline (section 5), which passes when rerun alone.
- `tsc --noEmit` on the whole quiz-react project: the identical single pre-existing `TS7016` (`jsdom`) error, before and after.
- No new import cycle (section 5).
- Nothing outside the session folder changed except three registrations (the new test in the vitest include list, in the package `tsconfig.json` include list and in the taxonomy's `members-of-tests`).

## 2. Map of the old class

### 2.1 Field ownership (old field to new owner)

| Old field | New owner | Notes |
| --- | --- | --- |
| `proctor` (public) | `QuizSession.proctor`, `ProctorLink.proctor` | same object; the composer keeps it public as before |
| `outbox` (public) | `QuizSession.outbox` | built by the composer; its two callbacks are bound to `Settlement` |
| `store` | composer (`store.watch`), `HeldState`, `OutboxOverlay`, `Adoption` | each gets it as an explicit constructor argument |
| `deputy` | `Deputising` (+ `OutboxOverlay` for the catalog stand-in) | the rest ask `deputising.present` |
| `patienceMs`, `timing`, `clock` | `ProctorLink` | `timing` and the raw clock are also handed to the `Outbox` |
| `state` | `HeldState` | only `dispatch` and `reset` write it |
| `snapshot`, `listeners` | `HeldState` | `getSnapshot`/`subscribe` are its arrow members, re-exposed by the composer |
| `online` | `Lifecycle` | |
| `lifetime`, `teardown` | `Lifecycle` | |
| `signUpsFrom` | `Registration` | |
| `unconfirmed` | `Settlement` | |
| `edits`, `edited`, `reads`, `adopted` | `ProctorReads` | answer stamps and read tickets live together, so they can only be compared in one place |
| `refreshCatalog`, `refreshLearner`, `refreshLeaderboard` | `ProctorReads` | the composer re-exposes the same `latestWins` function objects |

### 2.2 Method placement (all 77 old members)

| New owner | Old members |
| --- | --- |
| `HeldState` (68) | `dispatch`, `notify`, `freeze`, `getSnapshot`, `subscribe`; new: `read`, `learnerId`, `reset` |
| `Lifecycle` (69) | `connectivity`, the window listeners of `start`, `lifetime` abort/renew of `start`/`stop`/`reconnect` |
| `ProctorLink` (59) | `now`, `projected`, `patient`; new: `retrying` (the `retryTransient` calls) |
| `OutboxOverlay` (45) | `withPending`, `retire`, `restored` (now `restore`) |
| `Deputising` (87) | `held`, `fold`, `deputise`, `rehint`, `rehintAll`, `ownStanding`; new: `present`, `handle` |
| `DecisionRouting` (113) | `ahead`, `behind`, `direct`, `decidable`, `decided`, `holding`, `saved` |
| `ProctorReads` (160) | `refreshCatalog`, `refreshLearner`, `refreshCrowd`, `refreshLeaderboard`, `loadRun`, `adoptLearnerView`, `followClosure`, `voided`, `background`; new: `noteEdit` (the two lines of `answer` that stamp an edit) |
| `Registration` (80) | `identify` (its two inner closures became `register` and `adopt`) |
| `RunPlay` (152) | `startRun`, `resumeRun`, `openTask`, `answer`, `submit`, `submitting` |
| `Settlement` (131) | `deliver`, `enrolled`, `recalled`, `settled`, `decisionSettled`, `answerSettled` |
| `Adoption` (92) | `remote`, `adoptLearner`, `adoptRun`, `adoptStore`, `overlayPending` |
| `Attention` (75) | `open`, `back`, `forward`, `up`, `retrace`, `refresh`, `chooseBoard`, `askCrowd`, `unaskCrowd` |
| `QuizSession` (254) | constructor wiring, `now`, `start`, `stop`, `reconnect`, `readIntroduction`, `dismissNotice`, `forgetLearner` and one-line delegations for every other public member |

The brief's examples (progression, scoring, pet reactions, timers) do not exist in this class: it holds no scoring and no pets. The real responsibilities are the twelve collaborators of 3.2; the timers (learner patience, projection grace, the leaderboard patience timer) sit in `ProctorLink` and `ProctorReads`.

### 2.3 Method call graph of the old class (caller to callees; `~` marks a recursion inside one new component)

```
refreshCatalog     -> background, dispatch
refreshLearner     -> ahead, projected, adoptLearnerView, dispatch, outbox.forget
refreshCrowd       -> dispatch
refreshLeaderboard -> ownStanding, dispatch, now
askCrowd/unaskCrowd/chooseBoard -> dispatch, refreshCrowd / refreshLeaderboard
constructor        -> Outbox(deliver, settled), restored, rehintAll, freeze
start/stop/reconnect -> overlayPending, notify, remote, connectivity, refreshCatalog, refreshLearner
open/back/forward/up -> dispatch, retrace -> refresh -> refreshLearner, refreshLeaderboard, loadRun, refreshCrowd
identify           -> decided, holding, fold, dispatch, refreshLearner
startRun           -> decided, adoptLearnerView, projected, resumeRun, outbox.discard, dispatch
resumeRun          -> loadRun, dispatch, refreshLearner
openTask           -> loadRun, decided, voided, dispatch
answer             -> dispatch, outbox.enqueue, rehint
submit/submitting  -> loadRun, direct, saved, decidable, decided, deputise, voided, dispatch, refreshLearner, refreshCrowd
loadRun            -> behind, background, projected, withPending, dispatch, rehint, followClosure
withPending        -> outbox            overlayPending -> withPending, dispatch, rehint
followClosure      -> retire, dispatch, refreshLearner (~ loadRun, adoptLearnerView)
adoptLearnerView   -> dispatch, followClosure
remote             -> adoptStore, adoptRun, adoptLearner, dispatch, adoptLearnerView
adoptLearner       -> dispatch, withPending, rehint, refreshLearner
adoptRun           -> withPending, dispatch, rehint, followClosure
adoptStore         -> restored, rehintAll, notify
rehint             -> held, dispatch    rehintAll -> rehint
restored           -> retire, withPending            retire -> outbox
ahead/behind/direct -> outbox, proctor, state        (direct -> ahead)
decided            -> decidable, direct, patient, deputise
deputise           -> held, fold, outbox.enqueue     fold -> dispatch
holding            -> direct, patient                saved -> direct, outbox
ownStanding        -> held, dispatch
deliver            -> enrolled -> recalled, projected;   recalled -> outbox, dispatch
settled            -> answerSettled | decisionSettled
decisionSettled    -> voided, ahead, refreshLearner, loadRun, dispatch
answerSettled      -> loadRun, voided, behind, dispatch
voided             -> outbox, dispatch, refreshLearner
projected          -> retryTransient       background -> projected, dispatch
connectivity       -> outbox.wake, notify
dispatch           -> evolveQuizState, persistQuizState, notify     notify -> freeze
```

The cycles in this graph are the reason a base class would have needed abstract callbacks. Three kinds exist:
1. `loadRun` / `followClosure` / `refreshLearner` / `adoptLearnerView` (read, follow the closure, read the learner again). They stay together as the recursion inside `ProctorReads`.
2. Everything else that calls "up" (`decisionSettled` -> `loadRun`, `answer` -> `rehint`, ...) is a call to a collaborator that is lower in the new order, so it became a constructor argument.
3. `Outbox` -> `deliver`/`settled` (the queue calls the session back). The composer binds the two callbacks to `Settlement` by closure, exactly as the old constructor bound them to `this.deliver`/`this.settled`; this is the only back-reference that remains.

## 3. The decomposition

### 3.1 Dependency order (a collaborator only knows those above it in this list; there is no base class and no interface)

```
HeldState   Lifecycle   ProctorLink                      (no collaborator below them)
OutboxOverlay   Deputising(HeldState, Lifecycle, ProctorLink)
DecisionRouting(HeldState, ProctorLink, Lifecycle, Deputising)
ProctorReads(HeldState, ProctorLink, Lifecycle, OutboxOverlay, Deputising, DecisionRouting)
Registration  RunPlay  Settlement  Adoption  Attention    (ProctorReads and below)
QuizSession                                                 (builds all of them, delegates)
```

### 3.2 Collaborators and their narrow API (files under `S/`)

| Folder (lines) | Class | API | Owns |
| --- | --- | --- | --- |
| `🗃️held-state` (68) | `HeldState` | `snapshot`, `subscribe`, `read`, `learnerId`, `dispatch(event, persist?)`, `reset(state)`, `notify` | state, snapshot, listeners |
| `🔌️lifecycle` (69) | `Lifecycle` | `signal`, `isOnline`, `begin`, `own`, `watchBrowser`, `abort`, `release`, `restart` | lifetime, teardown, online |
| `📡️proctor-link` (59) | `ProctorLink` | `proctor`, `timing`, `patienceMs`, `now`, `retrying`, `projected`, `patient` | the clock and the timing options |
| `🧺️outbox-overlay` (45) | `OutboxOverlay` | `withPending`, `retire`, `restore` | nothing (stateless over outbox, store, deputy) |
| `🫡️deputising` (87) | `Deputising` | `present`, `handle`, `held`, `fold`, `deputise`, `rehint`, `rehintAll`, `ownStanding` | the deputy |
| `🏛️decision-routing` (113) | `DecisionRouting` | `ahead`, `behind`, `direct`, `decidable`, `decided`, `holding`, `saved` | nothing (derives from outbox, link, deputising) |
| `📖️proctor-reads` (160) | `ProctorReads` | `refreshCatalog`, `refreshLearner`, `refreshLeaderboard`, `refreshCrowd`, `loadRun`, `adoptLearnerView`, `followClosure`, `voided`, `noteEdit` | read tickets and answer stamps |
| `🙋️registration` (80) | `Registration` | `identify` | `signUpsFrom` |
| `🎯️run-play` (152) | `RunPlay` | `startRun`, `resumeRun`, `openTask`, `answer`, `submit` | nothing |
| `📬️settlement` (131) | `Settlement` | `deliver`, `settled` | `unconfirmed` |
| `🪟️adoption` (92) | `Adoption` | `remote`, `overlayPending` | nothing |
| `🔭️attention` (75) | `Attention` | `open`, `back`, `forward`, `up`, `chooseBoard`, `askCrowd`, `unaskCrowd` | nothing |
| `🟦️.ts` (254) | `QuizSession` | the old public surface | the collaborators |

Existing children are untouched: `🧮️state` (392), `💾️restore` (56), `📋️session-model` (89). Every file is far below 500 lines. Each new file starts with a docstring whose first character is the folder emoji, every docstring in a file has a unique emoji (the ones that collided with the file emoji were renamed), there is no comment inside a definition, and every file keeps a `//#region` pair like its siblings.

### 3.3 Decisions

- **Public API unchanged.** 26 `session.*` members are used by 20+ components and test files, many of which stub the session as `{ open, startRun, ... } as unknown as QuizSession`. Changing the surface would only have produced stub churn. The composer's 15 delegations are one-liners and return the collaborator's promise as is, so microtask ordering is the same as before (no `async` wrapper added).
- **No base class, no interfaces, no extension traits.** Collaborators take concrete lower collaborators in the constructor. The deputy-absent behaviour is a `present` check inside `Deputising`/`DecisionRouting`, as in the old code.
- **`Deputising.present` replaces `this.deputy === undefined`** in every other collaborator; the `!` assertions on the deputy stay where the old code had them (only reached when a deputy exists).
- **`HeldState` freezes its first snapshot in its constructor**, before the composer's `rehintAll`; the old code froze after it. The snapshot at the end of construction is the same value (a changed `rehint` notifies and freezes again); only the identity of the first object can differ, which nobody observes before `getSnapshot` is called.
- **Connection summary** (`QuizConnection`) is built by a closure in the composer constructor from outbox status, proctor reachability, `Lifecycle.isOnline()` and the deputy flag, and handed to `HeldState` to freeze with the state.
- **`stop()` keeps the old order**: abort the lifetime, stop the outbox, then run the releases.

## 4. New test

`Q/🧪️tests/🎬️session-playthrough/🟦️.tsx` (170 lines, 3 tests, no screen): a whole learner session through the public API of `QuizSession` only, written **before** the refactor and run against the original class (3 consecutive runs, 3/3 passed each: `qsd-play-orig1..3.json`), then kept unchanged:
1. the household quiz from the introduction to the results: snapshot identity, introduction, registration under a pseudonym, board and weekly board, run start, answers, crowd ask/unask, submission phases, awards, the proctor's command sequence, `back`/`forward`/`up` along the trail, the step history, refreshes, listener silence after unsubscribe, a reload that resumes and `forgetLearner`;
2. an expert run on the session clock: every opening once (and `already-opened` as success), a submission with an already aborted signal (rejects, run stays open, no `submit-run` sent), then the submission through;
3. the deputy: the proctor is away, registration, start, answers and submission are decided on the device (connection `deputy`, 6 commands pending, nothing sent), then the proctor returns and the 6 commands arrive in order and the learner view reconciles.

Registered in `Q/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (vitest include), `Q/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json` (include) and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`members-of-tests.memberNames`, one line after `🪛️pet-companions-kit`). No launch entry: `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` only hold the whole-package `quiz-react:test` and `:typecheck`, which already run it.

Negative controls on the decomposed code (each mutation restored byte for byte, verified with `diff`): `Deputising.deputise` no longer enqueues, 1 playthrough test fails; `HeldState.dispatch` never persists, 1 playthrough test fails; `DecisionRouting.ahead` always false, the existing `🧩️deputy-and-contract` suite fails 2 tests (the playthrough does not notice that one); dropping the results step in `ProctorReads.followClosure`, the playthrough passes (its submission path opens the results through the `run-submitted` event, not through this branch); I did not run the rest of the suite on that mutation, so I make no claim about it. Logs `qsd-mut1/2/3/4.json`.

## 5. Verification (every command was run; results are the real ones)

| Check | Command | Result |
| --- | --- | --- |
| Baseline suite | `F/qsd-tools/vitest.sh base-all` (the `quiz-react` vitest config, 104 suites) | 923 tests, 922 passed, 1 failed: `pet companions the client carries the choice, fetches the site's menagerie and the real layer, and lets go of both when pets are switched off` (`expected null not to be null`, the flake that `RESULT-PACK-QUIZ-FILE-SPLITS.md` §3.4 describes). `vitest.sh base-pets 🎪️pets-on-stage` alone: 13/13 |
| Baseline types | `tsc --noEmit -p tsconfig.json` in `Q/🎯️targets/⚛️react/📦️packages/🟦️typescript` | 1 error, `TS7016` for `jsdom` in `🚏️navigation` (`qsd-tsc-base.log`) |
| Suite after the refactor, run 1 | `vitest.sh after-all` | 926 tests, 925 passed, 1 failed: the same pets test as in the baseline (the load was 26 to 41) |
| Rerun of that file alone | `vitest.sh after-pets`, `after-pets2`, `after-pets3` | first rerun 12/13 (the same test again), then 13/13 and 13/13 |
| Suite after the refactor, final | `vitest.sh final-all` (after all mutation controls) | **926 tests, 926 passed, 0 failed** |
| Title multiset | `diff` of the status-normalised, sorted `qsd-base-all.rows` against `qsd-final-all.rows` | only the 3 added lines of the playthrough test |
| Types after | `tsc --noEmit` (twice: `qsd-tsc-after1.log`, `qsd-tsc-final.log`) | the identical single `TS7016`, nothing else; the new files are in the project include (`🔨️modules/**/*.ts`) |
| Import cycles | `node F/qsd-tools/importgraph.mjs <🔨️modules>` (Tarjan strongly connected components over the relative imports) over the whole `🔨️modules` tree (47 files, 238 edges) | one cycle, `🐾️pets/🟦️.tsx` <-> `🐾️pets/🎪️stage/🟦️.tsx`, which is not in the session folder and was not touched. The session folder's graph is acyclic: no child imports the composer, and the 12 collaborators import only collaborators above them in 3.1 plus `🧮️state`, `💾️restore`, `📋️session-model` |
| Taxonomy | `bun F/qsd-tools/taxcheck.ts` (the repo's `matchDirectoryKind` over the whole ancestor chain) on the 12 new folders, the 3 old ones and the test folder, at 17:35 | 16 of 16 resolved: 15 as `crate-module-children`, the test folder as `members-of-tests`; 0 unresolved, 0 ambiguous. Not repeatable at the end: from about 17:45 `loadTaxonomy` throws on a peer's half-finished `🧱️base` -> `🧱️full` subset rename in the same `taxonomy.json` (not mine). After my `memberNames` line, the stand-alone matcher `pqs-tools/ts/taxmatch.mjs` resolves `🎬️session-playthrough` as `members-of-tests` |
| Source-text readers | `git grep` for the session path, its type names and `?raw` reads | none: no fs or `?raw` reader reads the session source |
| Prettier | not run | `node_modules/.bin/prettier` does not exist in this checkout; I kept the repo style by hand (`printWidth` 250, multi-parameter-property constructors expanded). Four lines are over 250: three file docstring headers and one statement of `Settlement.decisionSettled` copied verbatim from the old class |

The flake is a whole-app render polled with the default 1 s `waitFor`; it failed in the baseline run before any edit and passes alone, so it is not caused by this change. Different tests of the same kind failed in earlier slices (`RESULT-PACK-QUIZ-FILE-SPLITS.md` §3.4); I changed no timeout.

## 6. Files

Created (12 collaborators): `S/{🗃️held-state,🔌️lifecycle,📡️proctor-link,🧺️outbox-overlay,🫡️deputising,🏛️decision-routing,📖️proctor-reads,🙋️registration,🎯️run-play,📬️settlement,🪟️adoption,🔭️attention}/🟦️.ts`; test `Q/🧪️tests/🎬️session-playthrough/🟦️.tsx`.
Rewritten: `S/🟦️.ts` (979 to 254 lines).
Edited (one line each): the quiz-react vitest config, the package `tsconfig.json`, `taxonomy.json`.
Unchanged: every consumer (`Q/🎯️targets/⚛️react/🟦️.tsx` still `export { QuizSession }` and `import { QuizSession }`; 8 component modules import the type), `🧮️state`, `💾️restore`, `📋️session-model`.
Tools kept: `F/qsd-tools/`. Logs: `F/🗑️generated/qsd-*` (including `qsd-debug.log` from the exploratory run of the playthrough test; no `[DEBUG]` line is left in any source file).

## 7. Residual risks and hand-offs

- The playthrough is a behaviour pin, not a proof: the old class and the new composer were compared by the 923 existing tests plus the new 3, with mutation controls. The paths that only another browser tab exercises (`Adoption.remote` for `learner`/`catalog`/`learner-view`/`cleared`) are covered by `🚶️journey-and-tabs` and `🧷️run-truth`, as before.
- Construction order matters in `QuizSession`'s constructor: the `Outbox` is built before `Settlement` and calls it only after `start()` (the same property the old constructor relied on with `this.deliver`). Moving `outbox.start()` earlier would break it.
- `ProctorReads` still holds a 4-method mutual recursion by design; splitting `followClosure` out would need a `refreshLearner` port and gains nothing.
- The pre-existing `🐾️pets` import cycle and the `jsdom` type error are not part of this slice.
- No Rust, no `launch.json` command and no migration were needed.
