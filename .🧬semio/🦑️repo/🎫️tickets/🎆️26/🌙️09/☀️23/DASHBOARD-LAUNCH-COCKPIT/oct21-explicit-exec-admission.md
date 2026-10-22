# Explicit Exec Owner Admission

## Observed Route Before This Change

`buildInvocationGraphSelectionV1` in build/graph/admission returned all for every command except run. `repo exec --projects=workspace -- git show ...` therefore requested the complete graph before the external read. The planner later reads project selectors and exclusions only before the literal `--` boundary, matches project identity/root/tag patterns, and generates each selected project's uncached repo-exec command target. Nx uses its owned exec operation and Bazel uses the same portable selection planner. This is avoidable discovery work for finite exact identities, not evidence of the live lease queue cause.

The prior closed selection schema had only all/tasks. Inventing a task named repo-exec during admission would be wrong: the task is generated later and is not an authored declaration. The owner registry already has canonical names and roots but not tags.

## Authored Owner-Only Contract

The implementation adds a schema-first owner-only selection variant (`kind:owners`, finite explicit project selectors and exclusions) for commands with a finite explicit project set. Resolve exact project names, name-prefixed exact identities and exact directory identities against the already captured registry. Preserve complete admission for absent selectors, wildcards, tag selectors, leading-negation/roster-dependent selectors and unsupported selection forms. Never inspect forwarded literal arguments for selectors. Unknown exact identities retain the existing planner's empty-selection behavior; repeated/comma selectors and exclusions must retain existing semantics.

Owner-only admission starts the same existing source/dependency closure without a fabricated target. The existing selected-graph path is extended consistently from tasks-only to every non-all selection: canonicalization, schema, scope digest, owner closure, source discovery inputs, selected receipt/publication checks and reuse. Keep actual per-owner physical observations, observed-fact retry, Original/dependency bootstrap and all final guards. The registry/selection already enter cache scope identity; the new variant must be explicitly admitted by every validator rather than adapted to tasks.

The authored tests include a closed language-neutral invocation/registry corpus and independent SQLite exact-name/root selection oracle; actual complete admission port counters must prove unrelated owners are unobserved while required source owners remain. Actual planner parity compares resulting project/task/cwd/literal argv against the existing complete graph for supported finite selectors. Wildcard/tag/no-selector/negative-only cases must still observe all; forwarded --projects after the literal boundary must not narrow. A genuine repo exec request remains required for performance/runtime acceptance.

The initial investigation was read-only. Root subsequently authorized schema/tests-first implementation. The selection variant, fourteen neutral vectors and production changes are now authored. The shared pure option/identity owner is explicitly included in graph implementation freshness; the serialized graph selection schema has an exact parity assertion. Actual Source bodies and runtime/performance acceptance remain pending. Source42 registrations and currently reviewed noise/export/admission diagnostic inputs remain frozen. Live corpus21940 retains ordinary guards; no queue cause, bypass or speedup is claimed.

## Independent Source Review

Low accepted the final owner-only slice and screened the bounded actual consumers without finding an unhandled owner-selection branch. The five selected graph branches, generated-producer validation, serialized schema and helper implementation freshness are included. TypeScript/MJS parses as DATA without syntax diagnostics. No actual test body, typecheck, measured speedup or successful exec runtime is inferred. The slice is frozen while the existing owned recovery and resolver continue with ordinary physical-source guards.
