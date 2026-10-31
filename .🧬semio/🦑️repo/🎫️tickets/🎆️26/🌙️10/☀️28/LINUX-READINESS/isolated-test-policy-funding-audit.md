# Isolated Test Policy and Funding Audit

Read-only audit of current Source on 2026-10-30. No Source edits, new test jobs, compiler runs, or graph runs were made. Watcher owns the production isolation schema, implementation, registrations, and acceptance jobs.

## Contract and caller bounds

The production contract is `🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🪟️isolation/🧬️schema/🔣️.json`; its TypeScript implementation is the sibling `🟦️.ts`. Policies are closed objects with a body deadline and an explicit body/file scope. There is no numeric compatibility overload. The neutral corpora consume this production schema rather than defining fixture authority.

The receiver at `process/🧪️testing/📥️invocation/🟦️.ts:108` passes the authored body milliseconds to both the selected child Bun test and its CLI timeout. For body scope, the transport allocation remains the same body bound, clamped to the actual parent Original remainder. For file scope, transport is additionally clamped to the authored file maximum minus monotonic process age. The parent Bun observer's grace does not fund a larger transport Original.

The point-in-time census saved in `🗑️generated/isolated-caller-policy-census.json` found 72 syntactic registrations, not 72 runtime cases: loops and strings are not expanded. It found 68 body scopes and four file scopes: two new 2 s body/20 s file controls, Cargo failed-build 10 s body/300 s file, and Cargo empty-nextest 60 s body/300 s file. Existing Vite mutation remains 30 s body. Read-only Git comparison shows the existing numeric registration bounds retained as object body deadlines; registrations introduced or moved earlier in this session cannot be attributed to a historical migration merely from the current diff.

## Actual file funding and custody

The existing `process/🧪️testing/🗂️suites/🔣️.json` cargo-driver row declares 300000 ms. The genuine runner `process/🧪️testing/🗂️suites/🏃️run/🟦️.ts:43` holds its actual invocation, narrows its control, and invokes `runOwnedBunFilesV1` with the existing suite timeout. That implementation captures each file under the issued phase and forwards the inherited envelope. Receiver `receiveBuildTestBirthV1` uses the inherited Original when present and checks actual declared capability agreement; it does not replace it with a new 300 s bare-run birth. Thus the file policy exposes already-funded preparation time, while the 10 s/60 s body deadlines remain separate and unchanged. A 300 s authored maximum is not evidence that 300 s remains available in a particular run.

The Cargo admission fixture at `process/🧪️testing/🦀️cargo/🧪️tests/🗄️admission/🟦️.ts` retains its real private repository invocation, actual admitted compiler store, second callback control, complete environment, and captured async context through `retain`. Its callback consumers execute while that same resource scope remains held. The receiver tracks pending users, closes fixtures, joins settlement and cleanup, then finishes its Original. These paths do not substitute an environment locator for actual store custody.

## Findings delivered to the owner

1. The previous receiver silently shortened primary failure reasons to 8192 characters. An anchored matcher could accept the shortened prefix while rejecting the complete reason. Watcher has now added production `isolatedBuildTestFailureReasonV1`: overlong or unprintable reasons mark overflow, and receiver overflow prevents expected-primary acceptance.
2. Cargo preparation previously used `Date.now()` subtraction. The private repository invoke API issues exactly its requested budget and does not itself clamp to this file's remaining time, so that caller needed a monotonic deadline and a final parent-remainder clamp. Current admission Source uses `performance.now()`, takes the minimum with actual parent remaining immediately before issuance, and preserves parent cancellation. Watcher owns this correction and its tests.

Expected-negative acceptance checks the exact selected test name, exactly one structured primary record, a nonzero unsignalled process result, the settled checkpoint, no overflow, and matching reasons across the complete reported failure list. Later cleanup failures refuse acceptance. Object-identical propagation is deduplicated rather than manufactured as another independent error.

## Verification scope

Watcher reported shared causal red 366310 to green 367330 (3 tests, 18 assertions), then genuine isolation 377062 (4 tests, 34 assertions) and receiver 379207 green. The reason-overflow correction has causal red 383987; full correction gate 385084 was still live when reported. Existing full Cargo driver 378583 remains separately owned; this audit does not certify its outcome or restart it. This is Source/custody review plus the owner's stated receipts, not new independent runtime acceptance of all registrations.
