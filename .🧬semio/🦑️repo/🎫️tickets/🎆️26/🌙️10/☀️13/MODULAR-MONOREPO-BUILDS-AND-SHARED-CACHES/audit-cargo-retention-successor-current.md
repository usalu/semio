# Cargo Retention Successor Independent Source Audit

Read-only rereview, 2026-10-16. No actionable Source defect found in the two successor repairs. This does not qualify Native execution or final publication. Only this Markdown was authored; no test, compiler, Native, provider, network, cache, lease, issuer, or modifying Git operation was run.

## Rechecked Findings

The former P1 is repaired in actual custody: `native-build/🟦️.ts:378–380` retains every created `CurrentPhysicalOwnerV1` before returning its invocation port, with admission bounded by the existing source inventory maximumEntries. Owners retain their original controls and cumulative counters; none is reconstructed for structural rechecks. Ordinary custody recheck reaches all owners. The private relocated-output branch at `native-build:429–430` additionally calls `shared.structures()` after original-file and retained-copy verification. Session close and completed-snapshot callbacks (`435`, `771–795`) therefore retain directory/absence checks after settlement. `current/🟦️.ts:50–58,68–77` preserves cumulative work/byte admission, original cancellation/deadline callbacks, and joined bounded structural enumeration; no new clock or renewable structural budget was introduced.

The former P2 is repaired locally: `component-pass/🟦️.ts:511–522` wraps each complete input-admission/graph-execution/logging operation in the actual graph owner. `component-pass/📥️planning/🟦️.ts:7–13` bounds accepted pending operations, retains a distinct failure flag for `undefined`, memoizes refusal, stops births and joins actual promises. Canonical `invocation/🏃️process/🟦️.ts:47–53,96` schedules the operation body in a later microtask and enrolls it first; the local owner observes that returned promise before effects. The pre-spawn and post-exec checks at `component-pass:514,518` prevent delayed input admission or completed children from producing late graph effects after stop. `508` stops births, terminates known direct children, joins graph operations, drains tracked work, then discards the session and closes resources. `694–712,728` uses explicit failure flags and preserves direct execution failure while attempting cleanup; Error and undefined remain distinguishable. This is settlement ordering, not proof of arbitrary descendant death.

Existing output/current classification remains intact: `native-build:420–422` installs current claims before considering output transfer and deletes overlapping relocations; transfer requires the actual shared claim, original identity, bytes and retained copy. `429–430` verifies retained output copies while leaving other original files current, and now additionally checks original structures. The unchanged inventory, snapshot, copy and reader roster rows were inspected with these boundaries; DATA decoding alone remains insufficient authority. No structural owner is redirected into retained output custody.

## Evidence Qualification

The preserved RED logs were read: `cargo-retention-structure-graph-red.log` reports the actual directory-before-close mutation accepted (13 pass / 1 fail), and `red2.log` reports that omission plus missing graph owner (13 pass / 2 fail). This graph RED is a missing-owner/topology failure, not a reproduced Native execution race. Final prototype2 and release2 logs each show 15 pass / 0 fail / 140 assertions. The two strict logs are empty; their exit status is reported by the successor record and was not independently recovered or rerun here.

Actual tests at `retention/🧪️tests/🟦️.ts:78–85` exercise normal temporary filesystem changes before/after session close, actual shared custody, and isolated received-Original graph owners. Both isolated graph rows retain a blocked sibling, assert close is unfinished, release it into a distinct later failure, preserve first Error/undefined, and assert late callbacks have zero effects. Ajv validates outcome DATA and SQLite checks failure → input settlement → cleanup ordering. `88` checks actual driver wiring text. The post-close structural test invokes shared custody directly; it does not fabricate or execute a completed Native token. These are genuine Source/Original laws, with Source topology covering integration. They do not establish actual Cargo/backend behavior, compiler prerequisites, throughput, product consumption, or OS descendant termination.

## Observed Hash Scope

All **29** Source rows in `cargo-retention-custody-successor-current.md` matched their listed byte lengths and SHA-256 at both initial inspection and final report checkpoint. No moved row was observed. This includes the actual new graph owner (`194cad48a5295784e0ebe8cb26d1572e5eef1d5df290e41089c8e28c9788da13`) and neutral test schema (`13cf81ed7c9627f89fbdf1f375391776911663b4b356574495e9885f17de1c57`). The successor table is the exact 29-row roster, not an inferred repository-wide freeze.

Additional inspected dependencies:

`🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts` — 14404 bytes, SHA-256 `b3fa92065908f3730654ae321cea611caeff3e95275da82a3856294d22141470`.

`🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` — 24986 bytes, SHA-256 `a16fbc76b0836bde8e7e1ec4634648a0ce2c810f87396e5d7d8b737f0a5cf07e`.

Ticket and goal remain active. Pending Native/production/concurrency and coordinated current-input reader gates remain outside this audit.
