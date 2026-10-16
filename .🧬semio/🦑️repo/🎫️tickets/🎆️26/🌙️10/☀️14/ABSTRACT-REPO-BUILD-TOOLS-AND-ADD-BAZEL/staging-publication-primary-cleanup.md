# Staging Publication Primary Cleanup

The actual stage `try/finally` in tooling provisioning previously let `rmSync(staging)` replace the publication failure. Its final `return installed(destination)` also returned an unsettled promise before the `finally`, so asynchronous validation rejection escaped that protected lifetime.

The permanent Source-derived test reproduced this with an undefined asynchronous primary and a cleanup error: exit 1, expected undefined but received `cleanup-secondary`; the old returned promise also emitted an unhandled undefined rejection. This is an algorithm-level RED. No claim is made that the failed SDK64212 cleanup actually failed or that its stage was removed.

The concise owner correction awaits final installed validation inside the existing `try`, records a separate failed boolean (including `throw undefined`), and always attempts unchanged staging removal. Cleanup failure after a successful body still throws. After an existing body failure, a guarded diagnostic records the secondary error and preserves the exact arbitrary primary even if logging itself throws. No property is written onto a frozen error. Store, quote, policy, cancellation, limits and removal scope are unchanged.

Seven literal language-neutral vectors cover undefined/frozen/primitive body failure, logging failure during secondary cleanup, cleanup failure after success, full success and body failure with successful cleanup. Ajv checks the closed schema; independent SQLite CASE expressions supply the selected body/cleanup/value outcome. The test extracts the actual current stage declaration/catch/finally and actual terminal return-await boundary through TypeScript AST; explicit ports replace only the expensive publication body, rm and logger. It imports no provisioning runtime and performs no Native, SDK, store or real deletion.

Registration: `tools/🧪️tests/🟦️.ts` imports `🧹️cleanup/🟦️.ts`, so the new law reaches the existing permanent toolkit test entry. No executable command was added.

Final actual model: exit 0, 1 pass / 40 assertions, `staging-cleanup-final.log`. Focused strict TypeScript for the production tools owner and new law: exit 0, session 78246, `staging-cleanup-typescript-final.log`. The first strict attempt found only an optional AST-finally presence diagnostic; an actual presence check corrected the test before the final run. Low reviewed the final production delta clean narrowly.

The production tools-script byte change requires fresh SDK runtime/compiler Source capture before publication qualification. Generated `staging-cleanup-source-receipt.json` binds current authored Source/input bytes and retained RED/GREEN/strict logs. All paths remain within 256 UTF-16 units. Production Source is held and all owned runtimes/custody are closed. Actual SDK publication on these bytes remains Root-owned and pending.
