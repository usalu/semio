# Cache Cleanup Consumer Contract — 2026-10-28

Only the command-source test and its language-neutral fixture changed. Runtime files, cleanup behavior, cache owner count, schema, declared input metadata and backend packaging repair remain unchanged.

## Baseline

The exact existing coordinate check was run before editing through registered repo tools exec --light Bun. It exited1 in0.97seconds: zero passed, one failed, ten filtered, three assertions. It failed at line218 because the cleanup source no longer references the caching CLI router. Policy and bootstrap coordinate assertions passed first. The baseline log is generated/command-source-baseline.log.

Current production cleanup correctly imports CachePruneScript from pruning/orchestration and passes cachingRoot, this.repoRoot and the original this.invocation directly. Restoring router re-entry would make the stale test dictate an incorrect owner graph, so no runtime routing change was made.

## Coherent Contract Repair

Added workspace-cleanup/command to fixture.consumers with the actual pruning/orchestration owner. It remains an external consumer, not a fourteenth cache owner. The existing closed schema accepts this row and all13 owner/declaration checks remain intact.

The coordinate test was renamed to describe policy/bootstrap coordinates and direct owner invocation. TypeScript AST assertions resolve the actual non-type CachePruneScript named import (including a legal local alias) to its owner and refuse a cleanup import of the CLI router. They bind the actual awaited constructed owner to cachingRoot, this.repoRoot and this.invocation; verify cachingRoot is joined from this.root and the canonical caching directory; and require the method's boolean dry parameter to forward exactly dry ? ["--dry-run"] : []. Both process-table refusal return guards must precede the delegate. The cache-prune log remains supplementary evidence.

The existing exact-consumer check now consumes the added edge. The named input check additionally binds the cleanup Source's already-declared workspace input. Original policy/bootstrap checks, strict owner diagnostics, no command back edge, DFS cycle detection, owner count and all previously declared fixture/schema/input checks are preserved.

## Focused Result

The registered lightweight owning-file run selected four relevant tests: fixture schema, moved bodies/exact consumers, coordinates/direct owner and Bun/Nx input closure. It exited0 in0.96seconds; **4 passed,0failed,7filtered,130assertions**. Bun reported527ms inside the runner. Log: generated/command-source-fixed.log. The language-neutral fixture is independently validated by Ajv. Cleanup was never executed; the AST reads its actual Source.

The expensive owner semantic-program test was unchanged and not rerun under current tight capacity; its prior backend focused pass remains separate evidence. This focused run did not create a large capacity waiter or overlap the qualified product build with another compiler. No live handles remain owned here.

Changed files:
- caching/🧪️tests/🧱️command-source/🟦️.ts
- caching/🧪️tests/🧫️fixtures/🧱️command-source/🔣️.json

## Independent Read-Only Review

Reviewed actual changed test/fixture and actual 🗑️generated/command-source-fixed.log, not just its report. Confirmed four named PASS lines,4pass/0fail/7filtered/130expect calls,527ms runner duration. The log lives under 🗑️generated; earlier shorthand generated/ paths refer to that directory. No gate was launched for this review.

Correction is substantive: current AST assertion resolves a single non-type CachePruneScript import to the exact pruning/orchestration owner (:218–225), rejects a router import (:226), binds the constructed callee to that import and checks three constructor arguments cachingRoot/this.repoRoot/this.invocation plus await (:235–243), canonical cachingRoot coordinate (:244–250), typed dry parameter and exact dry-run argument branches (:251–260). It also requires both structural process-table return guards precede delegation (:261–267). The stale router expectation was replaced with actual owner/custody checks, not simply deleted.

Fixture adds cleanup solely as external consumer with that owner, preserving13owners. Existing exact-consumer check95–104 consumes the new edge; policy/bootstrap coordinates remain215–216. Existing strict owner diagnostics, no command back edge and DFS cycle checks49–93 remain present; they were FILTERED in this4test execution, so this review does not promote the focused result to a fresh semantic/acyclic test pass. Their source contract remains intact. Structural AST checks validate invocation wiring and guard placement, not execution of destructive cleanup or every possible runtime guard meaning. No concrete defect found in the requested correction; runtime Source and declared input identities were untouched.
