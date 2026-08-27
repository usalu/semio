# VCS, Wires, and Imperative Retained Route Cohort

## Scope

This source-only fleet slice owns exactly the VCS, Wires, and Imperative editor command routes. It does not modify the framework runtime, Store, child-owner implementation, or any other app.

## Census

| App | Total routes | Migrated | Explicit fail-closed | Publication |
| --- | ---: | --- | --- | --- |
| VCS | 10 | all ten existing routes | none | 4 Artifact, 1 Config, 5 HostOnly |
| Wires | 10 | `canvasPointerUp`, `setLocale` | 8 `BatchOnlyPendingRewrite` | Config only |
| Imperative | 11 | `setLocale` | 10 `BatchOnlyPendingRewrite` | Config only |

The VCS jobs already had route-specific bounded/resumable work, checkpoint/replay for text edits, retained completion, freshness, cancellation, retry, and incremental close. Their missing exact publication contracts and app-owned Artifact/Config one-item Store preparation factories are now supplied per route.

The three newly migrated Wires/Imperative routes are one-mutation, byte-admitted `ArtifactRetainedCommandJob` operations owned by the concrete `EditorApp`. Their app-owned Config preparations bind exact operation, generation, base revision, actor and lane authority, create semantic inverses, advance in two granted items, support cancellation and incremental close, and reject retained bases above the byte ceiling.

## Fail-closed child-seam finding

Wires graph commands currently enter child content through `wires_working_scene`/`wires_working_board`, which clone the complete node/edge scene before route work begins. This blocks honest item-bounded admission for fresh-id scans, hit lookup, selection deletion, layout, drag movement, and whole-document example replacement.

Imperative program commands currently enter composed child content through `imperative_working_scene`, which clones the complete flow path and seed. Fresh-id recursion, nested owner/slot lookup, step edits, and program execution therefore cannot start from an item-addressable resumable Store/child cursor.

`setContributions` also remains fail-closed: `ImperativeConfigMutation::SetContributions::diff` synchronizes a process-global imperative module registry before Store publication. That violates app-owned retained ownership even though its JSON payload is byte-bounded.

Those routes remain unreachable from the interactive UI through explicit `BatchOnlyPendingRewrite` classifications. Migrating them without a child-owned item cursor would only disguise scan-then-monolith work as a job.

## Schema-first and independent coverage

- VCS, Wires, and Imperative each carry a draft-2020-12 JSON Schema plus a language-neutral route fixture covering every command, execution/disposition, publication lane, bound, and blocker reason.
- Rust source tests decode those fixtures with `serde_json` and compare their migrated census to the concrete factory/publication constants.
- Store-preflight adversaries reject Interaction history lanes, oversized Wires locale values, and Imperative contribution-registry mutations while admitting the exact Document-lane mutations.
- An independent Bun/Ajv 2020 check validates all three fixtures strictly.
- Bun source adversaries prove every fixture row has the expected manifest classification, every migrated row has its exact publication lane, and no `OnceLock`, `static mut`, or `thread_local!` owner entered the retained regions.
- A separate VCS source census proves all ten exact publication contracts.

Evidence is recorded in `🧪️sol-vcs-wires-imperative-source-checks-2026-08-27.txt`.

## Compiler lease

No cargo, nx, rustfmt, compiler, or task runner was invoked. The root fleet owner retains the only compiler lease and must run the focused Rust tests after integrating concurrent source changes.
