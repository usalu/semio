# Session Staleness Regression Test (2026-11-24)

## Files
- Test: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🧪️tests/🟦️.ts` (appended: `retained session reader <id>` per fixture row, and one source-wiring test).
- Fixture (language-agnostic): `.../🔎️discovery/🧪️tests/🧫️fixtures/🔣️.json`, new key `retainedSession` (rows: `current renderer output`, `renderer changed since staging`; fields `sourceSuffix`, `retained`, `strictAccepts`).
- Production code unchanged.

## What it checks
- Extracts the live `textAt`, `withPlaygroundSessionV1`, `retainedPlaygroundSessionProjectionV1`, `withSessionV1` from the reading module (TypeScript AST), runs them with the real session files written by `playgroundSessionPublicationV1` into a temp dir. Only the artifact-store and lease ports are stubbed.
- The stale row appends text to the copied session Source (an older renderer). Expected: the retained reader returns `undefined`, `refreshPlaygroundSessionPublicationV1` republishes once, the strict reader still throws `Session Source differs from its exact copied projection`. The current row: retained projection returned, no republish.
- Pre-fix behaviour is modelled in-test: the same refresh with the strict `withPlaygroundSessionV1` as reader rejects on the stale row.
- Source guard: the refresh reader in `serveRootPlaygroundV1` must use `retainedPlaygroundSessionProjectionV1` and not `withPlaygroundSessionV1`.

## Run
`repo test-file -- "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🧪️tests/🟦️.ts"`
Result: `13 pass, 0 fail` (2.5 s), including the 3 new tests. (Equivalent suite target: `test-component-owners`, not run.)

## Fails without the fix
Scratch copy `/private/tmp/claude-501/-Users-ueli-Documents-semio/3d55bb15-005d-4498-88da-1487da3fa74c/scratchpad/old-logic.test.ts`: the new tests with the reader replaced by the old logic (strict reader returning the projection) and the command source with the old reader name. Run with the pinned bun directly, since `repo test-file` refuses files outside the checkout. Result: `1 pass, 2 fail`: stale row throws `Session Source differs from its exact copied projection`, wiring guard fails. The live source was never modified.

## Not verified
- Real artifact store and native invocation path (stubbed); full real prepare/serve flow not run.
- Whole-project `test` target not run (depends on generate).
- No bug found in production behaviour.
