# Drawing Explicit Null Paint Admission — 2026-11-04

Final validation: the normal Draw admission owner passed after the null-paint and terminal source-custody fixes. The inner Bun run reports 1 pass, 0 fail; the root separately reports Layout behavior 77 pass, 0 fail plus successful strict checking.

The canonical Drawing schema already admits explicit null for both attributes.fill and attributes.stroke, and the model carries optional nullable fields. Null means paint is cleared. The TypeScript admission cursor incorrectly checks only undefined before dereferencing those fields; the Rust implementation already uses Option admission.

Read all applicable AGENTS instructions; there is no additional AGENTS.md in the current Draw plugin subtree or target ancestry.

## Test-First Vectors

Extended the existing language-neutral admission corpus with null fill, null stroke, both-null acceptance, null fill plus malformed nonnull stroke refusal, and null stroke plus malformed nonnull fill refusal. These exact vectors also feed the existing Rust corpus consumer. Updated the existing admission contract's paint invariant without changing the canonical nullable schema.

The existing TypeScript admission law was export-only with no consumer; added a tiny test entry and an admission selector to the existing Draw test owner. Its default Draw test run also executes the admission law. Registered the actual dynamic entry in runtimeSourceEntries. The existing Ajv projection and independent Kahn graph oracle remain the reference.

## Validation

The test-first red command was submitted while the original paint guards remained unchanged: `repo --tool=bazel run @semio-tech/draw-js:test -- admission` (session 79801). It admitted normally, then stopped before the runner with exit 1: `Authorizer source closure capacity exceeded: files=1789/2048, bytes=16781078/16777216`. This is an infrastructure refusal, not a test failure or a passing result. No cap, dependency store, receipt, or custody control was changed. The root coordinates that existing authorizer blocker.

The root's preceding executed full Layout baseline had already passed all 77 behavioral tests and failed strict checking at the original Drawing null dereferences. After the attempted red, changed only the two production paint guards from undefined-only checks to `!= null`, preserving null clear semantics and fully validating every present nonnull paint. The dedicated admission command passes the caller cancellation signal to its owned child.

The next normal Draw admission or full Layout owner run must execute the new cases; no green runtime result is claimed yet.

## Files

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️tests.mjs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🟦️.ts`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📋️project.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/fleet-null-paint-2026-11-04.md`

Final source inspection: the shared corpus now contains 56 cases (51 prior plus 5 regressions); both production guards exclude only null/undefined, preserving the original nonnull validation bodies. The new explicit test entry's absolute path is 145 characters, below the 256-character limit. No files were removed.

## Green Retry After Source Admission Repair

Retried the exact normal owner once on the parent's request: `repo --tool=bazel run @semio-tech/draw-js:test -- admission` (session 5169). Full output was captured under ticket `🗑️generated/fleet-draw-admission-green-2026-11-04.log`. The command admitted host capacity but terminated with exit 1 before the test runner: `Authorizer source closure capacity exceeded: files=1822/2048, bytes=16783971/16777216`.

The diagnostic points at repository build `🏛️sealed/🔣️.json`, through sealed execution, build script, and runtime command imports. This retry did not execute any Drawing tests; no green result is claimed. The five neutral vectors, independent oracle, and production guard remain unchanged. No capacity or custody bypass was attempted.

## Terminal Source Custody Regression

The root's subsequent actual Draw admission run (session 21006) exposed an existing lifecycle defect: after completion, a new value at the same issuer/projection was accepted by a terminal no-op before the captured-value identity check. The existing third assert.throws remained unchanged and is the observed red result.

Inspected both DrawingAdmissionCursor and EmbeddedChildAdmission: issuer/projection checks already precede terminal and funding decisions, while a nonterminal value mismatch intentionally retains paid progress. Added a terminal-only captured-value check before the no-op, preserving the original nonterminal funding and refusal receipts. No shared interface changed.

Extended the existing neutral corpus with four terminal source sequences: the original source remains accepted and done; an equivalent newly owned value, changed issuer, and changed projection are refused. The TypeScript admission law drives these after both successful and refused admission, before final close, and retains all three previous replacement assertions, original source immutability, and paired birth/retirement receipts. The same document cases continue to compare against the existing independent Ajv/Kahn projection oracle.

The root reports full Layout JavaScript behavior and strict checking already passed (77 behavioral tests, zero failures, layout ts ok); no full Layout rerun is required for this cursor-local change. A normal declared Draw admission rerun follows.

Lifecycle repair additionally updates `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️tests.ts`; the existing fixture, admission contract, production cursor, and this report are also updated. The normal rerun is session 6181 and its full output is ticket `🗑️generated/fleet-draw-admission-lifecycle-2026-11-04.log`.

## Final Green Result

Session 6181 completed with exit 0 for `repo --tool=bazel run @semio-tech/draw-js:test -- admission`. Bazel executed 1 of 1 test targets, passed in 3.3 seconds (not a cached test result). Its actual owned child log confirms:

- `Drawing source admission preserves explicit null paint and nonnull refusals` passed in 412.99 ms.
- Bun: 1 pass, 0 fail, 1 test across 1 file, 489.00 ms.

That test executes the shared 56 document vectors, including all five null-paint regressions; the four neutral terminal-source replacements; the unchanged issuer/projection/value refusal assertions; cancellation, long-key, deep-ancestry, original-source immutability, and paired-retirement laws. The existing independent Ajv projection and Kahn graph witness remain active. No further tests or shared-interface edits were needed.
