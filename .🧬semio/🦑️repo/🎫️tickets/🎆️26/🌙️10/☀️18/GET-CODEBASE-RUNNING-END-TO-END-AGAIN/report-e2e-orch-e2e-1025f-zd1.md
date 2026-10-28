# 🧭 e2e-orch report e2e-1025f-zd1

Created 2026-10-26T07:34:00+01:00, written 2026-10-26T07:34:45+01:00, orchestrator pid 86384. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-zd1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 1s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 1s | 0 | 0 | 0 | 0 | - | 0 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

no waiting lines recorded.

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T07:34:00+01:00, end@2026-10-26T07:34:45+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-zd1`.

## Resources

Max load 21.7, min available memory 16.0 GB, min free disk 65.1 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1025f-zd1/course/e2e-1-test-e2e.log`

```
# bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev
# cwd /Users/ueli/Documents/semio
[run-e2e 07:34:40] root /Users/ueli/Documents/semio; ticket /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
  ok      /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium-1248
  ok      /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium_headless_shell-1248
  ok      /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/ffmpeg-1013
[run-e2e 07:34:40] test-e2e: repo run @teaching/architecture-course:test-e2e -- --topology dev --  (log /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-102
$ repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts repo "--tool=bazel" run "@teaching/architecture-course:test-e2e" -- --topology dev --
33 | export type HostDefinitionV1="Policy"|"Host"|"Request"|"Waiter"|"Grant"|"Decision"|"Evaluation"|"Plan"|"GrantLiveness"|"Handover"|"HandoverFacts";
34 | 
35 | /** 🛂️ Requires a value to match one closed definition of the host capacity schema. */
36 | export function requireHostCapacityV1<T>(definition:HostDefinitionV1,value:unknown):T{
37 |  const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
38 |  if(errors.length)throw hostCapacityErrorV1("definitionRequired",{definition,detail:errors[0]!});
                             ^
error: Closed host capacity Policy required: /estimates is required
      at requireHostCapacityV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts:38:25)
      at readHostCapacityPolicyV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts:85:15)
      at /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts:20:65
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
[run-e2e 07:34:41] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1      1s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025f-zd1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-zd1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
