# Runtime Integration

## Current Findings

The course task router imported Course APIs but invoked nonexistent Quiz APIs. Its TypeScript and Vitest infrastructure imports still inserted an obsolete package subtree. Its development server and compiler owner labels still used architecture-quiz. The release-site helper lacked its Vite import and admission control; CI browser preparation lacked the path join import.

The E2E runner mixed an unfinished scoped-stack refactor with calls to nonexistent bootStack and stacks. It imported ScriptInvocation twice, declared Run.invocation twice, referenced an undefined QuizTopology, omitted resolve and ticketOutput imports, never ran rehearsal specs, called buildProctor without original invocation authority, and never retained the resulting artifact for cleanup.

## Implemented

The router now uses the current Course APIs and existing public framework owners. Publication and readiness build through the original release helper. Readiness passes direct build, typecheck and endToEnd callbacks retaining the original invocation instead of reentering an unrelated shell context.

E2E now uses scoped topology sessions: serial sessions close before opening the next; concurrent sessions open rehearsal before development and retain every resource until every consumer settles, including a late consumer after another fails. Release rehearsal uses the bounded Vite build owner and a matching immutable reader; the static listener closes before reader release. Both topologies run the same browser consumers. The proctor build retains and closes its artifact after the sessions drain. Original cancellation flows into the run and child launcher environment.

The development proctor watcher now imports its current source-directory resolver and follows PROCTOR_COURSE; the stale PROCTOR_CATALOG read and undefined course binding would have stopped normal watch-enabled development after the build. E2E attempts every owned cleanup and refuses a successful gate if any cleanup fails.

Language-neutral lifecycle cases and a draft-07 schema cover serial, concurrent, consumer failure and partial startup failure. The neutral validator and Ajv agree on valid and invalid fixtures.

## Validation

- Exact existing launch command `bun repo run @teaching/architecture-course:test`: executed, failed before tests at the canonical build router's duplicate openRepositoryCachePartitionV1/control/leases/failed declarations (lines 88 and 91). This is an observed infrastructure failure, not a claimed missing-module TDD failure.
- Direct installed Node Vitest diagnostic with the current course configuration, selecting `🧪️tests/🎭️sessions/🟦️.ts`: executed twice. Final result: one file passed, six tests passed. The delayed development consumer verifies that rejection does not release resources prematurely.
- No native compilation, fixed-volume allocation, deployment, or publication has been claimed or performed by this agent.
- Direct installed Node Vitest local-stack diagnostic initially failed in the then-malformed proctor bootstrap at line 259. After restoring the local-stack test imports and fixtures, the next attempt encountered a missing `node_modules/vitest/vitest.mjs`; the installed Vitest junction had concurrently changed from version 4.1.10 to 4.1.12. No dependency installation or compiler-storage changes were made for this diagnostic.

## Outstanding Integration Validation

The full canonical launch and actual browser journeys remain to be rerun after shared bootstrap parsing and deployment-module repairs. This diagnostic unit-test result does not establish an end-to-end release pass.

## Course Domain Follow-Up

The course domain validator declared both quiz and course schema identities as COURSE_SCHEMA and exported two different implementations as courseIssues. The legitimate quiz validator is restored as quizIssues with QUIZ_SCHEMA; the course validator retains its course identity and signature. This preserves the current quiz components of a course and introduces no compatibility API.

Observed test-driven validation: the existing course-content suite first failed transformation on the duplicate constant and duplicate function/export. After the correction, the exact same selected suite passed: one file, 89 tests, including the authored course and every quiz against both the owned core and Ajv's draft-07 validation.

In-memory, no-emit TypeScript diagnostics on the router, E2E, stack, sessions test and local-stack test now show only the unresolved ui-styling status-serve public package in the local-stack test. Runtime configuration export resolution is being repaired by the launch integration owner. The complete imported graph still has unrelated diagnostics and is not claimed clean.

The Rust validator twin had the same duplicate schema constant; quiz validation now compares against its distinct public QUIZ_SCHEMA, while course validation keeps COURSE_SCHEMA. Native Rust tests have not been run under the refused host preflight.

## Current Local-Stack Result

After current workspace-public-export alias resolution landed, the local-stack diagnostic suite passed: one file, 42 tests. These actual tests cover real Node servers and released port probes, isolated Vite listeners, the static rehearsal origin against Vite's preview oracle, owned process-tree shutdown, current source closures, development SQLite handling, forwarded socket cleanup and the unanswered capacity wire against fetch's timeout behavior. No native compiler invocation or store allocation was needed.

Capacity test imports now resolve their existing runtime bindings: the shared optional progress logger lives in the measurements owner and is imported by the gate and hall workers, the abusive wire imports its default deadline, and the process helper imports its retained artifact type. The full capacity load gate has not been run.

## Files Changed By This Runtime Agent

- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📜️script.ts`
- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🔒️sessions/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🔒️sessions/🧬️schema/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🔒️sessions/🧫️fixtures/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🧱️stack/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧱️local-stack/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎭️sessions/🟦️.ts`
- `🧰️framework/🛍️products/📚️course/🔨️modules/✅️validation/🟦️.ts`
- `🧰️framework/🛍️products/📚️course/🔨️modules/✅️validation/🦀️.rs`
- `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🟦️.ts`
- `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/📊️settings-and-measurements/🟦️.ts`
- `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🎒️hall-learner/🟦️.ts`
- `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🎭️abusive-script/🟦️.ts`
- `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🖥️proctor-process/🟦️.ts`
- This retained ticket report. Release-tag follow-up files are listed in `release-tags.md`.
