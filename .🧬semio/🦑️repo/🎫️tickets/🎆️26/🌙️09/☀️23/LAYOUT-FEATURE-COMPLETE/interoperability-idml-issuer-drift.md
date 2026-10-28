# Current Generic Retained Issuer Drift

Re-read current source on 2026-10-21. File: C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🦀️.rs.

Stage at line29 declares only Inverse, Apply, Retire, Done. The actual retained implementation matches Birth(line51), Decide52, Source53, Plan54, Preflight55, Commit57, Join58 and Publish59; none is declared. MutationApplyEdit at line37 contains only stage, clone_bytes, displaced and custody. Its implementation reads/writes semantics(line52), source55, plan55, diff70, messages70, inverse79, closing48 plus join/refused/retirement/inverse_close/issuer, none declared in that struct. The native retained methods call SnapshotProjection and protocol helpers without the old struct bounds matching the current new semantic owner design. This is concrete source drift, not a native compiler result.

The IDML owners must implement genuine RetainedMutation and RetainedDiff first. Their desired store factory seam remains generic PagedOneItemPreparationFactory<IdmlSnapshot,IdmlMutation,MutationApplyEdit<...>> plus existing shared semantic preparation owner constructor. Do not route IDML retained publication through the old cold apply path or clone the original whole package as the staged post. Untouched fields of the live unpublished post must survive central commit.

This agent has not edited the generic framework source. Parent is coordinating repair separately; no build-refactor files or raw build commands were used.

## Scoped Runtime Repair Mounted

With explicit parent authorization, repaired the generic owner declaration rather than bypassing it. Stage now declares all ten retained frontiers. MutationApplyEdit now owns the exact semantic/diff/source/central-plan/inverse/message/join/issuer/typed-retirement fields that its existing implementation uses; Default creates empty headers, and missing fixed-header demand/progress/context helpers and direct protocol imports are supplied. Removed the inaccurate old snapshot-sized cold apply declaration. Actual independent web-tree-sitter AST verification agrees with the new portable ten-stage/nine-custody/three-flow fixture, syntax errors0. This does not establish native compilation or runtime correctness. Existing older generic integration tests themselves still have cold-only fixtures and outdated advance signatures; the new canonical IDML issuer law must use the actual retained source/control API. RetainedMutation and issuer registration remain unfinished.
