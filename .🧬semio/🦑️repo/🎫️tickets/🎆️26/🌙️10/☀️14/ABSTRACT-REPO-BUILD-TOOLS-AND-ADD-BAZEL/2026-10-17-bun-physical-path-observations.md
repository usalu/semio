# Bun Physical Path Observations

Root explicitly selected a new observation unit after the complete Source census exceeded the old operation/path storage limit. The new schema admits at most 131072 distinct physical paths and separately at most 262144 original facts, with at most two facts per path. This is a new contract, not preservation of the former 131072-fact capacity. The actual 64133 and Source 7163 failures remain historical evidence in `2026-10-17-bun-all-observation-capacity.md`.

## Actual Runtime Change

The private owner now stores one closed path record containing its original state and optional complete original listing. The record preserves the original device, inode, mode and kind state; a parent listing does not replace a child's own identity. A bounded global array stores frozen references to each record's first admitted fact. It does not duplicate fact payloads.

The complete first-fact admission order is preserved across paths, including when a listing arrives after another path's state. Every discovery, publication, read-only validation, and terminal fence traverses that same reference order. Repeated facts must still match their original complete values. State and topology drift retain the existing detailed refusal. Listing admission requires an existing directory state. Both capacity checks precede registration, freezing or accounting changes, so a refused new path/fact leaves the original registry intact.

No Source recipe, opaque-directory rule, collection taxonomy, physical filesystem operation, policy/text fence, aggregate directory-entry limit, manifest byte/document limit, or pending-recheck limit was removed or relaxed. Full physical discovery remains six current scopes and 304 packages in the preceding noncustody Source census; no generation eligibility follows from that DATA.

## Portable Counterlaws

Twelve language-neutral cases are defined in the existing Bun fixture and schema. They cover repeated facts, two facts per path, interleaved global order, separate path/fact refusal, adding a fact to an existing path at path capacity, rollback on refusal, missing-state appearance, state identity drift, topology kind drift, and listing without a directory state.

Tests select complete actual production declarations through the TypeScript AST. Explicit small Source capacity ports exercise the real private registrar without claiming Native authority. Ajv checks closed vectors/records; independent SQLite derives the first refusal, admitted distinct paths, distinct facts, and original ordered fact set. A distinct actual `currentFence` test proves interleaved replay order and exact genuine AbortSignal cancellation. Existing actual filesystem read-only, growth, aggregate-byte, topology, publication, cancellation, accepted-work and closure laws run alongside these tests.

## Gates And Scope

| Gate | Actual outcome | Generated log |
| --- | --- | --- |
| 25811 | Semantic RED: defining registrar declaration absent | `bun-observation-api-red.log` |
| 42071 | Natural 0: 22 tests, 347 assertions | `bun-observation-first-source.log` |
| 18747 | Strict failed: one test matcher type | `bun-observation-first-strict.log` |
| 58328 | Strict completed; terminal-fence test failed because harness omitted `limits` dependency | `bun-observation-final-helper.log` |
| 52337 | Strict failed: new helper supplied Resource progress to Script progress API | `bun-observation-final-helper-closed.log` |
| 72497 | Natural 0: four explicit strict roots, zero diagnostics; 23 tests, 354 assertions, 2.45-second test run | `bun-observation-final-gate.log` |
| 16335 | Full current Source run refused at its genuine 60-second deadline after 263936 operations; no accepted report | `bun-observation-full-current-source.log` |

The matcher and harness failures were fixed. The helper now uses the actual first-party Script progress API. Its repeatable finite Source command is `stdio-phase/📜️script.ts --bun-observations-strict`, within the existing unchanged 60-second Original. The existing generic EN/DE ticket-helper launch supports the mode arguments.

Root authorized a distinct fresh `--bun-observations-current` mode for the larger actual Source custody qualification. Before birth it rejects the schema-owned fresh-environment overrides and reads the canonical first-party default build policy. It uses workspace-only Source capabilities, proves actual Root0/no deadline, runs actual ALL read-only custody, awaits a callback recheck and final accepted-work drain, publishes bounded operation/count progress every 64 visits, and retains cancellation. It does not resume or enlarge the failed finite Original. This mode must run only after independent input hold; its result is recorded separately. It performs no Native or protected-cache operation.

After the clean independent thirteen-input hold (`2026-10-17-bun-observation-independent-review.md`), actual standalone Root0 run 28305 returned natural 0. It entered and settled six current scopes and 304 unique packages, with the actual Bun control counter reporting 560280 completed operations in 94560.93 milliseconds. That counter includes Source selection/continuation operations and excludes the separately synchronous terminal-fence syscall replays; it is not a total syscall count. The terminal fence remains synchronous, as before this change. The callback recheck and final validation/drain completed; the initial wire logged maximum elapsed 0 and deadline null. The log is `🗑️generated/bun-observation-ordinary-current-source.log`. A subsequent nofollow FD/name/seven-field stamp and SHA recheck matched all thirteen held inputs exactly; receipt `🗑️generated/bun-observation-ordinary-current-source-receipt.json`. This is actual Source custody qualification under the new unit. The old finite deadline failure remains a failed gate, and this successful new ordinary Original does not qualify a protected Dependencies plan or removal.

The fixed inactive-generation removal command is still incomplete. This checkpoint does not authorize retirement, assert that `0b06bf268e0b61667f62` is inactive, qualify NTFS behavior, or claim a completed protected Dependencies plan.
