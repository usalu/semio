# run-many "Closed scoped plan required"

- Violated rule (reproduced red before the fix): `ScopedPlan` arrays are `uniqueItems`; `scopedPlanOfReplyV1` pushed one edge per `dependsOn` entry and one task per reply row, so a repeated dependency or repeated task id made `/edges` or `/tasks` non-unique. Repeated named targets did the same to `/scope/targets`.
- Second possible rule, not observable without a real run: `/scope/targets` maxItems 4096 (roster roots fallback). The error now names it.
- Fix: dedupe tasks, edges and named targets; the refusal now lists the first four violated rules with schema path plus task/edge/project/scope counts, in en and de.
- "Planning 1 targets": a roster request sends `["*:*"]`; now reports `host-plan-roster`.
- Tests: `🧭️invocation/🧪️tests/🛰️plan/` fixture cases (multi-project run-many with missing targets and repeated deps), Python oracle now emits scope/dedupes, TS test for the diagnostic.
- Unrelated failing test: "a real client gets its plan..." fails at slice digest (not verified whether pre-existing).
