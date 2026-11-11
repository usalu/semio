# Repo Cargo Error Surface

## Cause

`runBuildDoorMainV1` (`🏃️process/...` build door) hands every error to `reportRepoErrorV1`. That reporter only handled `RepoErrorV1`, so an `OwnedCommandFailure` from a non-zero child escaped as an uncaught Bun exception with a source excerpt and stack trace.

## Fix

- `🎛️owned-execution/📪️outcome/🟦️.ts`: `OwnedCommandFailure` keeps its `label`; new `ownedCommandExitCodeV1` (status, or 128 + signal number) and `isOwnedCommandFailureV1` (cross-instance guard).
- `📖️texts/🔣️.json`: `execution.child-failed` and `execution.child-killed` in en and de.
- `🏗️build/🗣️language/📣️progress/🟦️.ts`: `refuseChildFailureV1` builds a `failed` refusal with the child's evidence as `cause`; `reportRepoErrorV1` renders it at the CLI boundary and sets the child's exit code. Plain errors still answer `false`.
- `🎛️owned-execution/🟦️.ts`: re-exports the two new helpers.

## Stops (timeout, cancellation, termination, output limit)

`runOwnedCommand` now throws typed refusals at the stop sites instead of plain `Error`s (`stopRefusalV1`, same boundary):

| Stop | Code | Key | Exit |
|---|---|---|---|
| pre-cancelled, abort, SIGINT | `cancelled` | `execution.cancelled` | 130 |
| SIGTERM | `terminated` (new) | `execution.terminated` | 143 |
| time limit | `timeout` (new) | `execution.timed-out` | 124 (as read oracle) |
| output byte/line limit | `breach` | `execution.output-limit` | 1 |

Progress deadlines and the remaining internal stops (publication refused, input refused, consumer refused) still throw plain errors.

## Tests

- `🏗️build/🗣️language/📣️progress/🧪️tests/🔣️.json` and `🟦️.ts`: real failing children (exit 101, exit 1) and synthetic signal outcomes, each checked in en and de, with exit code and an Ajv check of the outcome against its schema. 13/13 pass under the managed Bun.
- Stop corpus rows (pre-cancel, cancel-after, timeout, output-limit) run real children, checked in en and de with exit codes.
- Suites pass: progress 17, error 10, texts 3, outcome 1, catalog 25. Owned-execution 6 of 7 pass; the remaining bundle-allowlist check fails on the unmodified validator's schema-closure and pack imports, which predates this change.

## Verification

`repo cargo --store=shared --configuration=prototype -- check` in `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990` now ends with `repository:command failed with exit code 101` and exit 101. No Bun banner or stack trace. Log: `🗑️generated/repo-cargo-check-verify.log`.

## Open

Remaining internal stop reasons still throw a plain `Error`. The cargo repro was run before the stop change; it only exercises child exit, so it is unaffected.
