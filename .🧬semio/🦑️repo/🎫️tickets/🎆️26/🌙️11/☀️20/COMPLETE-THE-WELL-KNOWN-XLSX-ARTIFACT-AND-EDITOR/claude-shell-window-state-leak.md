# Shell per-window state leak (`view context: utility capacity exceeded`)

## Root cause
Every per-window shell slice was append-only. `SET_ACTIVE_UTILITY` (and the fold, expand, staged-arg, title, engagement, measure and body-store writers) only ever added keys. A spawned program's windows live under `${programId}::${kind}` with a fresh program id per opened workbook, so each workbook left two permanent keys. The closing path only wrote `null` into `activeUtilityByWindowId` (an explicit clear, which keeps the key), and nothing reacted when a program left the roster or when the session owner was replaced. The view-context admission (`🛂️manifest/👁️view-state-and-public-invocation`) forwards the whole map and correctly refuses more than 64 entries, so the ~30th workbook broke spawning. The 64 cap is untouched.

Stale commands: `noteShellCommand` was memoised on the render-time `session` and `onAction`, and `publishOpenedProgram` called the render-time `onAction`. A long-lived window or async continuation holding an older closure therefore addressed the predecessor instance, which the funnel then refused as `instance-sealed`.

## Retire and close events found
- Window close: `🖼️canvas-footer/🔸️canvas`, which writes explicit `null` clears, drops the extra window instance and, for the last window of a spawned program, retires the program through `admitProgramRetirementRef` and removes it from the space panel roster.
- Program retire or replace: `useSwitchPluginAppState` (`publish`, `seedLayout`, `retire`, seal via `sealedInstancesRef`), `publishOpenedProgram`, plugin reload and uninstall (these change the spawned roster or `SET_SESSION`).
- Per-window shell state: `windowUiByWindowId`, `windowEngagementsByWindowId`, `windowMeasuresByWindowId`, `toolMeasuresByToolId`, `panelBodyStoreByKey`, `spawnedWindowActivityByWindowId`, `spawnedWindowEngagements`, `spawnedWindowMeasures`, `foldedByWindowId`, `expandedByWindowId`, `stagedArgsByKey`, `activeUtilityByWindowId`, `windowTitlesById`, `windowIconsById`, `extraWindowInstances`, plus the host-side utility register cells (`utilityGenerationByWindowRef`).

## Fix (event-driven)
- New event `WINDOWS_RETIRED { windowIds, programIds }`. Every slice reducer prunes the keys it owns in the same event (exact window ids, plus the whole `${programId}::` namespace; staged keys by `${windowId}:` prefix). It keeps the identity when nothing matches. `SET_ACTIVE_UTILITY(null)` still keeps its explicit clear, so the existing tests stand.
- Roster effect: when a program id leaves the live spawned roster (closed, replaced, plugin retired), the shell dispatches `WINDOWS_RETIRED` for its namespace and drops that program's register cells.
- Window close: the canvas dispatches `WINDOWS_RETIRED` for the closed window.
- Session owner change: the root `shellReducer` retires the replaced primary program's own windows (keys outside any `::` namespace) in the same reduction as the `SET_SESSION` that replaces it. Spawned programs' state is kept; the layout and extra instances are not touched (`seedLayout` owns them).
- No stale commands: new pure `liveNoteTargetV1` (surface-switch) resolves the note target from the live session at call time and refuses sealed or unloaded instances; `noteShellCommand` uses it with `core.sessionRef`, `core.sealedInstancesRef` and `core.onActionRef`. `publishOpenedProgram` now calls `core.onActionRef.current` for the pre-switch `commitCheckpoint`.

## Files changed (under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`)
- `🧱️elements/🐚️Shell/🎛️shell-actions/🟦️.ts` (action type)
- `🧱️elements/🐚️Shell/🗃️state-shapes/🟦️.ts` (retirement helpers)
- `🧱️elements/🐚️Shell/♻️slice-reducers/🟦️.ts` (four reducers)
- `🧱️elements/🐚️Shell/🕹️shell-store/🟦️.ts` (owner change retirement)
- `🧱️elements/🏛️ShellHost/🎮️focus-and-input/🎯️focused-program-and-introduction/🟦️.tsx` (roster effect)
- `🧱️elements/🏛️ShellHost/🖼️canvas-footer/🔸️canvas/🟦️.tsx` (window close)
- `🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts` (`liveNoteTargetV1`)
- `🧱️elements/🏛️ShellHost/🖱️shell-actions/🟦️.ts` (`noteShellCommand`)
- `🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx` (`publishOpenedProgram`)
- `🧪️tests/🗃️shell-store-reducer/🟦️.ts` (four new tests)
- Ticket: `claude-shell-leak-vitest.sh` (runner), this report.

## Tests
New: 100 programs opened and closed leave every per-window slice empty and never exceed 64 active-utility entries; exact-window and program-namespace retirement leave other programs and identities alone; a session owner change retires primary windows and keeps spawned ones; `liveNoteTargetV1` never targets a sealed instance after replacement.

Runner: vitest 4 under the repo-managed bun, config `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (`claude-shell-leak-vitest.sh`, `SEMIO_VITEST_POLICY` as in `xlsx-routing-fixes.md`). `repo test-file` fails for these suites (no jsdom).
- Suites run: shell-store-reducer, window-action-panel, window-ids-and-layout-seed, surface-switch, spawned-program-session, registry-utilities, runtime-admission, refresh-coalescing: 8 files, 269 tests, all pass (reducer suite 41/41).
- `repo typecheck` on the 10 touched files: one error at `shell-store-reducer` line 426 (`id` on `WindowLayoutWindowNode`), which is not in the new code (new tests start near line 650).
- Not run: the `saved-state` (canvas), `tutorial-bridge`, `view-model-surfaces` and `window-lifecycle-template-drag` suites are not in the react vitest include list; no browser journey was repeated.

## Follow-ups (coordinator round 2)
1. Typecheck: the error at `shell-store-reducer` line 426 was a stale literal (`{ kind: "window", id }`); the layout node field is `windowKindId`. Fixed. `repo typecheck --no-importers` on all 13 touched files (including the new ones below): passed, 0 errors.
2. Suites outside the react vitest include list (`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`):
   - Added: `💾️saved-state` (5 files via a new `engineFile` helper: `🟦️.tsx`, `🖼️canvas.tsx`, `🚪️lifecycle.tsx`, `🔑️runtime-key.ts`, `🧮️derivation.ts`) and `🎥️tutorial-bridge` (it was only in the wgpu config). Both pass.
   - `🖥️view-model-surfaces` is not a standalone suite: it exports `registerViewModelSurfacesTests`, which `🔌️plugin-runtime-suite` already calls and the config already includes. It does run (plugin-runtime-suite: 150 tests pass). Not added again.
   - `🪟️window-lifecycle-template-drag` is a `bun:test` oracle (uses `import.meta.dir`), not vitest, so it cannot join the vitest list; it runs through `repo test-file` (plain runner). It was failing: two source-reading tests read the old `ShellHost/🟦️.tsx` monolith for `handleTemplateDrop`, which moved to `📐️layout-hotkeys/🟠️handle-template-drop`. Both tests now read that owner file; 9/9 pass.
   - Nothing needed more than a small fix.
3. Same program id, new instance: `PROGRAM_INSTANCE_REPLACED { programId, replacedInstanceId, liveWindowIds }`. `refreshSpawnedUiPass` (`🛫️spawned-session/🔶️document-services`) remembers the instance each program id last refreshed; when a pass for a different instance finishes, after all its own dispatches, it emits the event. Reducers then drop (a) refresh-owned state under `${programId}::` for windows the new instance did not declare (spawned activity, engagements, measures, titles, icons) and (b) all user-owned state under the namespace, live windows included (active utility, fold, expansion, staged args), because it was chosen against the old instance. Other programs are untouched, and a repeat is a no-op. Panel bodies and tool measures need nothing: `withProgramEntriesV1` already replaces them wholesale. Covered by a new reducer test.

## Round 3: sealed-instance refusals in the browser journey (145 en, 169 de)
Evidence: `🗑️generated/claude-full-s/journey-en-leakfix1.log` lists 117 distinct refusals: 110 `noteShellCommand` and 7 `commitCheckpoint`, all `instance-sealed`, all `window=stdio-office-N::s.stdio.window.snapshot-details`, instances 3..9 about 7 each, 10..43 two each.

Call site (the shell funnel, not the window's guest UI): the details window sends nothing itself. The funnel (`resolveDispatchTarget`) maps the input's window id to a program through the live roster and the render-time `panel`, then addresses that program's instance. Shell chrome built inputs against a window id that outlived its program:
1. `noteShellCommand` callers: `handleActiveWindowChange` (`windowActivate`), the 350 ms layout-settle timer (`windowResize` / `windowMove`, `LAYOUT_CHANGE_SETTLE_MS`), `windowClose`, panel toggles. The input carries the active window id, which still named `stdio-office-N::...`; the funnel resolved it to the owner program's instance, which a replacement had already sealed. My round-1 fix used the live host session, but the funnel still redirected by window id, so the route was untouched.
2. `commitCheckpoint`: `publishOpenedProgram`'s pre-switch check-in and the auto check-in (`AutoCheckinScheduler`, `useCheckpointOnCloseV1`) use the controller of the focused or primary program and the same active window id.
3. Nothing cleared `activeWindowId` when its program left the roster, so every later chrome input kept resolving to the dead window.

Why about 6 per instance (inferred from the code and the log; I could not run the journey): the emitters are not one periodic task but several timer- and event-driven ones that all fire during the same replacement. Each open/edit/save/reopen step re-seeds and re-renders the layout, and each settled layout change fires the 350 ms debounce note; window activations on the pointer path add `windowActivate`; `windowClose` is noted before retirement; and the auto check-in scheduler re-arms itself via `defer()` while a dispatch returns false. Instances 3..9 saw the whole edit, save, reopen cycle (5 to 6 notes plus one check-in); later instances were only opened and left (2 notes).

Fix (root: no input addresses a sealed instance or a window of a gone program):
- `liveChromeTargetV1` (`🖱️shell-actions/🧭️input-funnel/🎯️dispatch-target-and-view-state`) resolves the target with the same `resolveDispatchTarget` the funnel uses, but from live state, and returns `null` when the window names a program that left the roster (`windowProgramGoneV1`) or the resolved instance is sealed.
- `noteShellCommand` uses it, so nothing is emitted for a window whose program is gone or sealed; `publishOpenedProgram`'s checkpoint does the same; `dispatchCheckpoint` returns false for a sealed focused program (the scheduler re-arms and the retry reaches the live program, or is cancelled on the session switch).
- The roster effect also resets `activeWindowId` to `null` when the active window belonged to a retired program, so later chrome stops resolving to the dead window.
- The funnel's own `instance-sealed` refusal stays as the backstop for external callers; the chrome paths no longer reach it.
- Test: new suite `🧪️tests/🪦️sealed-chrome-target` (added to the react vitest include) reproduces the exact route: host session, spawned `stdio-office-3` (instance 3), active window `stdio-office-3::s.stdio.window.snapshot-details`. A note resolves to instance 3 while unsealed, resolves to `null` for six repeated notes once instance 3 is sealed, resolves to `null` once the program leaves the roster, and still notes the host for a host window.
- Not verified: the browser journey was not re-run; whether the 145/169 refusals reach zero needs that run. Any remaining refusals would come from a chrome caller outside `noteShellCommand`, `dispatchCheckpoint` and `publishOpenedProgram`; the log lines will name it by action and window.

## Final test results
- vitest (repo-managed bun, react config), after round 3: sealed-chrome-target, shell-store-reducer, saved-state (5 files), tutorial-bridge, plugin-runtime-suite, window-action-panel, window-ids-and-layout-seed, surface-switch, spawned-program-session, registry-utilities, runtime-admission, refresh-coalescing, document-opening: 17 files, 474 tests, all pass; `repo typecheck` clean on every touched file.
- `repo test-file` window-lifecycle-template-drag: 9 pass, 0 fail.
- `repo typecheck`: clean.

## Limits
- No browser journey re-run; the 30-workbook reproduction is covered by the reducer tests only.
- The instance-replacement event is wired in the spawned refresh pass only; the reducer is covered by a test, the pass wiring by typecheck and the existing suites, not by an integration test.
