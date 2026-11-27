# BUILD_SOURCE_CONFLICT on the closure-graph root of the plugin store provider

## Where it is raised
`conflictingInferenceErrorV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs`. `settleObservedComputationV1` calls it when two rows of one identity (`kind\0path`) have different `observationMeaningV1` inside one settle unit. The row kind is `global`, the path is `["closure-graph", workspaceRoot, {roots:[provider/🟦️.ts], ...}]`.

## Existing policy (already correct in principle)
A conflict between different units is tolerated as churn. A conflict inside one unit repeats the whole round (`conflictRounds` = 5, pauses 25 ms times 4^i, about 8.5 s in total). If it persists, it fails as genuine. The pause budget is tiny against rounds of minutes, so on a live-replicated worktree any closure that is edited all day exhausts it.

## Root cause
The meaning of a `global` row was the whole fact `{digest, answer}`. For a closure-graph fact the digest covers the bytes of every file in the module closure. The provider closure (`provider/🟦️.ts` -> `packaging/🟦️.ts` -> the plugin store) is edited constantly by concurrent agents. Any edit moved the digest during the 79 min inference and counted as a contradiction. But `closureGraphFactHoldsV1` already decides that such a fact holds when the **answer** (what consumers read: roster, compiled modules, generated outputs, ports) is equal. So detection was stricter than validity: a body-only edit that leaves the answer unchanged was reported as a conflict. `provider/🟦️.ts` itself is unchanged (473 bytes, 2026-11-24) and consistent: it imports only the extension-package port type and `runExtensionComponentPackage` from `../🟦️.ts`.

It is a concurrent-edit race (case b), made fatal by an over-strict meaning. Not verified against the original failing run, whose log is not retained, so a real answer change (an import edit in the closure) would still fail after the 5 rounds.

## Fix
- `registerGlobalFactV1(name, holds, prepare, meaning)`: new optional `meaning` (observation `🟨️.mjs`, `🟨️.d.mts`); `globalFactMeaningV1` feeds `observationMeaningV1` for `global` rows. Without it the whole fact is compared as before.
- Closure-graph registers `fact => fact.answer` (`🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`).

## Test
`🧪️tests/🪞️global-meaning/🟦️.ts` (new; Python-checked answer extraction): equal answer with moved digest settles, different answer conflicts, no registered meaning compares the whole fact. Run with `repo test-file` together with `🛎️global-prepare`: all passed.

## Not done
The rebuild of `semio-os-mcp` was not rerun (memory, 1 h+). Possible follow-up: retry only the conflicting unit instead of the whole round.
