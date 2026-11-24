# Prepare Session Staleness (2026-11-23)

## Root cause
- `prepare` (`withPlaygroundSessionV1`, session `reading`) re-renders the session Source with the current `renderPlaygroundSessionTypeScript` and requires byte equality with the copied `🎮️playground-session/🟦️.ts`.
- The copy is written by `stagePlaygroundSession` (registry `session`), driven by `serveRootPlaygroundV1` or `registry session <variant>`. This is not a cached nx stage. It is guarded by `refreshPlaygroundSessionPublicationV1`, which republishes only when the retained catalog projection differs from the current one.
- The reader passed to that guard was `withPlaygroundSessionV1`. It compares the Source against the current renderer and throws on mismatch, so a changed renderer (the 17:10 `activationEvents` edit) either threw or left the old copy in place. The staleness check never saw the render code.

## Fix
- `🔌️plugin/📇️registry/🎮️playground/🧭️session/👓️reading/🟦️.ts`: shared core `withSessionV1` with a tolerate-stale flag, plus a new `retainedPlaygroundSessionProjectionV1` that returns `undefined` when the retained Source differs from what the current renderer emits. `withPlaygroundSessionV1` keeps its strict behaviour.
- `🦑️repo/.../🛠️dev/🎮️playground/🟦️.ts` (`serveRootPlaygroundV1`): the retained reader now uses `retainedPlaygroundSessionProjectionV1`, so a render-code change forces a restage.
- Not added: an automated test for the stale-renderer case (the existing session test is a long native-invocation test). Follow-up.

## One-time restage
Staging ran once via `registry session draw` (stage `session-p1`) because the stale copy was already published and the serve path was not used. This is not a cache deletion.

## Stages
- session-p1: ok
- prepare-p1: ok (1 of 1 components, 17 fonts)
- activate-p1: ok (after one torn-artifact retry, completed unchanged)

## Journeys (p1)
Both en and de: steps boot, blank, example, attach passed. They failed at `rectangle` with `TimeoutError: waitForFunction: Timeout 30000ms exceeded.`
Errors: `original-turn.native-return-refused: semantic: InvariantViolated: original work loan callback replaced its initiating context or refilled its work` (guest worker fault on `canvasPointerMove`, input #20, gesture window drawing-composite).
Steps undo, redo, persisted, export and reload were not reached. Export is unverified.
Reports: `🗑️generated/journey-p1-en/report.json` and `journey-p1-de/report.json`.
