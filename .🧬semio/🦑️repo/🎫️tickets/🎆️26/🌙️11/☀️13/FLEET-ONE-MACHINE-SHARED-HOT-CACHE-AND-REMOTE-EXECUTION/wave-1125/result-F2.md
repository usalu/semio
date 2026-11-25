# F2: ui-styling-tokens:generate failure and other failing Nx tasks

## Finding
- The worker logs lost the actual error (736 lines elided); only the Nx summary names `@semio-tech/ui-styling-tokens:generate:prototype` (only g1-m02; g1-m03 ran it green).
- Reproduced here: `repo run @semio-tech/ui-styling-tokens:generate:prototype --exclude-dependencies` is a one-second generator, but its `repository:command` child waits for `native-fixed-image:<repo>/.cache/b/g` (exclusive) behind other holders (on a worker: framework-rs:generate, 20 min). After ~6 min it dies with `The resource loan of repo ran out without observable progress: Original resource loan deadline exhausted`, exit 1.
- Root cause: the queued wait for a native image lease was not counted as admission wait (the design rule: a deadline starts when the work is admitted, see `beginAdmissionWaitV1`; capacity leases already do it, native image leases did not).

## Change
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts`: new private `waitingForAdmissionV1`, wraps the lease acquisition in `persistentOptionsGuardV1`. Landed under lock `landing.lock.native-build`, recorded in `T/landings.md`.
- Tests: receiver `🧪️tests/🟦️.ts` 3/0. Failing independent of the edit (verified by reading the assertions, they inspect other code): `📥️release` (`existingOnly is not defined` in extracted test source), `🌐️platform/📋️contract/🧪️tests/📥️persistent` (text assertion on `openPersistentNativeImageV1`).

## Not verified
- The exact m02 error text is unrecoverable; the deadline source of the child is inferred (credit via admission ledger). A post-fix rerun result: see end of file.
- No new regression test for the wait credit (private helper; the native image tests need a mounted image).

## Other failing Nx tasks in `S/origins/*/ready-log-*failed*.txt`
- `@semio-tech/framework-rs:generate:prototype` (m01..m13): F1 (crate-refused admission).
- `@semio-tech/framework-actor-rs:typegen:prototype` (m03, m06, m07, m11): error text elided; the logs show the same run failing `norm-din4108-rs:build` with `process.source.unresolved-module {"module":"dbg"}` in `🌱️value/🛬️decode/🧳️owned/🧬️protocol/📦️packages/🦀️rust/🦀️.rs`. The file has no `dbg` today, so it was a peer's transient edit that has since healed; nothing to fix. typegen itself unconfirmed.
- `@semio-tech/norm-din4108-rs:build:prototype` (m03, m06, m07, m11): the `dbg` unresolved module above (transient, healed).
- `repo-mcp:build:prototype`: not run (depends on the above).
- tools/native/dependencies/repo-mcp failed logs contain no Nx failed-task list (F3/F4 territory).

## Post-fix rerun
Same command after the edit: exit 0 in about 6 min under load 35-40 (task ran, 26 s critical path). No native-fixed-image queue wait occurred in this run, so it shows the task passes but does not by itself prove the wait credit under contention.
