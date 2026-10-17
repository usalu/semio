# Namespace Current Payload Diagnostic

The existing physical planner correctly refused a nonempty namespace during its fresh empty-image phase. This change adds only failure-path observation DATA so the real payload can be identified before its typed owner is implemented. It makes no ownership, format, funding, retirement, publication or emptiness claim.

The helper uses the already captured bounded Core payload and current journal observation. It performs no additional file, header, directory, mount or journal reads. It emits a single closed `[DEBUG] Current Graph namespace payload` record with exact directory/root, the root and entry seven-field facts, relative paths/kinds/full captured SHA, and observed journal presence/row metadata. Absent journal DATA contains no synthetic digest, byte count, state or allocation. The record explicitly carries `authority:false` and `publicationRetirementDispatches:0`; no opaque parent, writer, token, port or reservation is exposed. Existing Core 64-entry and 32-read-work limits remain unchanged. A new bounded diagnostic byte limit applies only to this output record.

The reporter is private and called only inside the existing nonempty `retire-empty-image` refusal, after the actual Core capture. Its output errors are contained, including undefined and frozen values. The same refusal still throws. The existing Root0 failure abort, accepted cleanup and real custody closure remain unchanged.

Ten portable vectors cover absent/present-zero/current-row observations, extra input, over-capacity, wrong root, wrong seven-field stamp, empty payload, and undefined/frozen delivery. Exact helper AST tests compare output with independent Ajv and SQLite, check full entry preservation, seven fields, absence separation, no privileged reader calls, and the actual planner refusal branch preserving its primary. No Native ports or filesystem captures are simulated as actual protected acceptance.

The first RED attempt exposed a missing test import and is retained as a harness failure. The semantic RED log `namespace-payload-diagnostic-semantic-red.log` contains the actual missing-reporter assertion; a second test also exposed a duplicate Ajv ID, repaired before qualification. Subsequent test-only AST property and typing errors are retained in `namespace-payload-diagnostic-{source,strict}.log`.

Final gates both exited 0:

- `graph-purpose/📜️script.ts --maintenance-source`: 17 pass, 1 intentional default-policy skip, 1216 assertions.
- `graph-purpose/📜️script.ts --maintenance-strict`: zero diagnostics, then the same 17/1/1216 Source suite.
- Scoped readonly `git diff --check`: 0.

Final logs are `🗑️generated/bazel-planner/namespace-payload-diagnostic-final-{source,strict}.log`. The stable nofollow FD/name/seven-field stamp/full-SHA six-input receipt is `namespace-payload-diagnostic-final-hashes.json`; four diagnostic files changed, with the unchanged actual Core payload contract/schema also bound. No Native/protected operation or physical namespace observation was performed by this agent. Root's fresh authenticated census/plan is the next separate physical checkpoint.

First-party Graph Source identifies the dedicated native namespace and closed watcher role, but a concrete SDK payload filename/header contract has not been assumed. Attempts to open public upstream Nx23.2.0 native Source URLs returned unavailable/404; no claim is derived from those unavailable pages. Actual bounded namespace DATA is needed before selecting a typed payload command.
