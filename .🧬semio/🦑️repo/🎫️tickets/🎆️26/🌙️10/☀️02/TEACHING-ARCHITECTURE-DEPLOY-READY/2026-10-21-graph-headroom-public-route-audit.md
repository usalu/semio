# Graph Headroom Public Route Audit

Read-only Source audit after actual prefix0842 outer exit 1. No command, helper, test, cleanup, or retirement executed. The frozen refusal and physical postflight are in 2026-10-21-prefix0842-native-execution.md; this report identifies supported routes, not current candidate eligibility.

## Disposition

No declared public command for graph-image headroom preview or retirement was found in the actual native early-recovery allowlist, build cache dispatcher, or caching owner targets. Existing graph recovery already runs during ordinary graph admission. No guard change, broad cleanup, or reset follows from this finding. A later ordinary Native retry would still require Root authorization and actual changed eligibility; this audit does not prove that a retry can now succeed.

## Exact Supported Routes

| Native call | Admission and scope | Relevance |
| --- | --- | --- |
| repo run repo:cache-retire-inactive-preview | Fixed authored early recovery before graph; opens rust partition, compiler reservation/layout, compiler writer custody; preview only | Compiler store, not graph image |
| repo run repo:cache-retire-inactive | Same early recovery; fixed retire operation | Compiler store, not graph image |
| repo run repo:cache-reset-nx | Fixed authored early recovery before graph; qualified inactive Nx capsule only | Nx partition, not graph |
| repo cache status --area graph --json | Early build cache-ledger branch before graph; bounded ledger observation | Reports physical graph partition state; no retirement |
| repo cache gc --area graph --dry-run --json | Same early branch, managed graph partition has no collect policy and is skipped | Not a graph headroom retirement plan |
| repo cache clear | Tool task cache operation; Nx adapter points to rust/nx/artifacts | Not graph closure retirement |
| repo reset | Explicit graph authority invalidation retains charged storage, immutable pins and Source facts; then tool reset | Not a general graph-byte reclamation operation |

These are Source-established call spellings, not a recommendation or dispatch authorization. No arbitrary forwarding was inferred.

## Evidence

- Native Go frontend:  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go, PlanNativeEntrypoint lines182–194 routes run/cache/reset to the build owner.
- Build owner:  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts lines118 and151–154 dispatch fixed recovery and cache-ledger commands before loadBuildInvocationGraph.
- Fixed recovery: build/🚀️bootstrap/⌨️cli/🟦️.ts lines15–21 and30–34; adjacent schema Protocol permits only repo-mcp build/install, repo compiler preview/retire, and repo inactive Nx reset. No graph maintenance target is admitted.
- Caching owner: ⚡️caching/📋️project.json declares compiler preview/retire with fixed command/cwd/no forwarded args. Its current target names include no separate graph headroom preview/retire.
- Compiler implementation: ⚡️caching/🧹️pruning/🏪️budget/📜️script.ts opens partition rust, computes repositoryCompilerLayoutV1 and invokes retireInactiveCargoStoreV1 under compiler writer custody.
- Cache ledger: ⚡️caching/💾️storage/🧾️ledger/🔣️policy.json declares graph as native b/g, kind partition, managed image, without collect. build/🧹️cache-gc/🏃️runner/🟦️.ts lines215–222 skips a selected area without collect; only test-evidence has a delegated pruning port in the command adapter.
- build/🕸️graph/🟦️.ts lines138–152 explicitly invalidates authority while retaining charged storage and Source facts.

## Exact Optional Retirement Boundary

The admission caller ⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts lines70–75 already attempts selected-publication recovery, scoped optional recovery, nonwaiting coarse optional recovery, and then queued full graph-image recovery with retireBuildGraphOptionalHeadroomV1 as its callback.

That callback in build/📥️inputs/💾️storage/🟨️.mjs lines60–63 and84–92 inventories only image.root/inference. The adjacent storage schema OptionalQuota.optionalPath accepts exact recomputable np/ip/rr/mr, selected/context variants, record packs/indexes, and bounded observation blobs. It never enumerates top-level build-input-closures or build-lock-closures. Thus the independently observed ~808734720 allocated bytes in build-input-closures are outside this callback, regardless of their age. This is a concrete scope boundary, not proof they are safe to retire or a request to add them.

The full image headroom owner additionally authenticates snapshots/selected/canonical publication pairs under physical writer custody; retired0/0 does not establish that all image namespaces are empty, that no scope was busy, or that every byte has an eligible public retirement route. Root/Light physical inventory remains separate evidence.
