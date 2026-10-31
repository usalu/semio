# Build Owner Typing Review

Read-only diagnosis of the declared Layout TypeScript target refusal. No jobs, tests, native execution, cache operations or source edits were performed.

## Exact Collision

The current 📋️build-owners.json has 3,810 owner rows. owners[1019], starting at line 6429, is named repo-cache-retirement. owners[1826], starting at line 11483, is named repo-cache-retire. Both declare the identical root:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget`

Both declare the same root/📋️project.json manifest and empty packages arrays. The actual manifest at that path names its project repo-cache-retirement and owns the existing cache-retire-inactive-preview and cache-retire-inactive targets.

The registry implementation at repository library/build/graph/admission/🟦️.ts lines 27–30 keeps one root set containing owner roots and nativeRoots. At row 1826 the predicate roots.has(owner.root) is true because row 1019 already added it. It throws Canonical build owner identity collision before target execution. These two distinct names do not permit two canonical owners of the same physical root.

A scoped structural inspection matching name/root/nativeRoots/package uniqueness and the special workspace-root manifest rule found only this duplicate physical root; no typing/fold/original-job row was implicated. This inspection is not an execution of the TypeScript registry or its schema validator.

## Principled Fix

Remove only the stale repo-cache-retire owner row from the canonical registry. Preserve repo-cache-retirement, the actual project manifest, both existing targets, every peer owner and all source files. The physical project already supplies the canonical name; no alias or relaxed collision check is needed. A later authorized target invocation must demonstrate admission succeeds; this review does not claim it has.

## Actual Refusal Evidence

Source agent supplied the exact retained log: 🗑️generated/typescript-declared-recheck.log in this ticket. The command was `repo --service=off run @semio-tech/layout-js:test -- --level=medium`; it exited 1 before tests. The stack points to the actual admission source at line 30:238. No native test proof or TypeScript suite pass follows from this failed invocation. Root's native lane remains independent and untouched.

## Resolution Recheck

Root removed only the stale repo-cache-retire registry row. Fresh read confirms exactly one owner remains for this physical root, named repo-cache-retirement; no repo-cache-retire alias row remains. The concrete identity collision is source-resolved. This metadata correction is not a cache operation. No job or target rerun was performed by this review, so declared-target execution remains unverified here.
