# Dry run queued behind the host scheduler

Root cause: `runRepoInvocationV1` (`🏗️build/🧭️invocation/🏃️run/🟦️.ts`) skipped the scheduler only for `watch`. `--dry-run` / `--graph` went through `runScheduledInvocationV1` (class `compile`, learned estimate), so a plan-only call queued for host capacity and ran as a job. CLI parsing is not at fault: `hoistBuildRunTaskV1` / `buildOperationArgumentsV1` keep `--dry-run` (a valueless flag) before or after the task.

Fix: new `planOnlyInvocationV1` (`--dry-run` true, or `--graph[=file]`, before `--`); such invocations run `execute` directly, no scheduler admission, no success recording.

Tests: `🧪️tests/🛰️plan/📨️task-arguments.json` (`planOnly` rows: flag after, before, between valued options, `--graph`, `=false`, after `--`) and test "a dry run plans but is never admitted..." in `🧪️tests/🛰️plan/🟦️.ts`, cross-checked with yargs-parser and a store proxy that throws on any scheduler access. test_file: 19 pass, 0 fail.

Live check: `repo run @semio-tech/framework-rs:generate --dry-run` no longer queued on the scheduler, but the process-level host-capacity gate (`withBuildCapacityV1`, script.ts:124) still reserved 2304 MB and waited 15 min when the service did not plan (plannedByService false; a stale dependency receipt then failed with "Actual declared requested Source scope required"). No cargo/rustc process appeared. That second gate is unchanged and open: a self-planned dry run reserves the fixed part by design.
