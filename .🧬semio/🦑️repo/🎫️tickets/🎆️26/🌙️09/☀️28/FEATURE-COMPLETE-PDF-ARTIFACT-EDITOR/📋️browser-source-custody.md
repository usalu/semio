# Browser component source custody

The ordinary single-component original is `repo run semio-hub-stdio-pdf-component:component --configuration=prototype`. The selected PDF playground prepares both real actors, `stdio` and `stdio-pdf`. The ticket's validation driver invokes the same component closure owner through `repo capacity run` and an issued Original repository script. The two-component pass uses `🌎️hub/Cargo.toml` and its checked-in lock. The retry retains the compiler and product stores.

## Source and cache resolution

1. The component pass acquires the repository Cargo input and compiler store custody before running Cargo. `admitCargoSelectionV1` captures the exact Cargo metadata and unit graph from the selected worktree and compiler environment. For workspace units it reads the authored files selected by actual Rust cfg, walks declared module and include ownership, enforces the source boundaries, and records source digests. Its local admission memo reuses a roster only if the previously observed path and inode stamps still match. A racy worktree path also requires its content digest to match. This occurs before compile admission, with the shared worktree remaining writable.
2. Cargo starts the selected process behind the host governor. When Cargo dispatches a unit, the repository rustc wrapper computes its cache identity from its source and incoming dependency artifacts. Cargo may first accept an already-fresh unit in its lane. The wrapped unit otherwise checks the local content-addressed cache, then the fleet tier when configured. Local restoration verifies the stored dependency and environment digests. A fleet entry imports the validated result and then passes through the same restoration path. If neither cache qualifies, the wrapper tries an admitted remote Rust compile when its cost and policy allow it, or runs the local compiler. The remote execution port enrolls the specific source and dependency inputs from this Original. No remote provider chooses the live workspace paths or changes the Cargo graph.
3. The component driver observes actual compiler outputs and their dep-info, and seals provenance before final publication. It already supports a bounded re-admission when one warm selected component fails to compile and an actual dependency has moved. This branch requires a single selected component. The combined Stdio and PDF gate continued independent work after two compiler failures and could reuse those outputs in its next pass.

## Reproduced timing

In the baseline generation wPRwaUR, the actual graph closed at 17:14:20 UTC. The representation unit's cache lookup at 17:28:10 UTC was a miss. The rlib produced in that pass predates the live BREP representation export changes at 17:58:46 UTC. The new consumers can read their updated source while Cargo still supplies that earlier representation rlib. The resulting missing imports are reproducible from the actual provider and consumer chronology. This sequence can occur with a correct admission memo and a cold cache.

## Bounded prevention

Carry the captured source roster into the pass and dependency observation, including unit aliases for Cargo-fresh and fleet-restored artifacts. Before a consumer uses observed workspace providers, compare its own source stamps and its provider epochs with this admission. Rehash changed or racy paths and refuse a genuinely changed input before passing it to rustc. Reuse unchanged workspace digests, external package identities and the existing local and fleet stores. Apply the comparison equally to cached and compiled providers.

A changed path should return a structured stale result naming the provider unit, consumer unit and admitted and current digests. Replan under the same host governor and invalidate the transitive affected units on the existing graph. Permit a bounded retry in multi-component passes for the affected component closures. Preserve immutable unchanged outputs and publish a closure only when all of its provenance refers to one consistent admitted roster. This extends the existing drift-aware warm retry while preserving the writable shared worktree and ordinary fleet cache.

## Implementation references

- [Component pass and existing bounded retry](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts:819)
- [Original Cargo source admission](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts:49)
- [Repository source-admission memo](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts)
- [Local, fleet and worktree compiler dispatch](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts:126)

## Live compiler and host process ownership

The w8jCBpl broker timeline continued sampling through the slow IFC interval. Its count advanced from 571 to 572 compiled units, held 181 Cargo-fresh units, and moved from one to two active rustc workers. The broker reported `frozen=false`. The read-only process-tree sample of the governed compiler Original identified IFC2x3 entities shard 05 and XML mutations as actual rustc children, each accumulating more than two seconds of CPU time in a three-second sample. IFC shard 00 and JSON number were scheduler waits without compiler children. The recorded evidence shows productive compilation under the memory envelope.

The old capacity entry at PID 84814 belongs to the resident `repo service host run` process tree, with a live service worker. PID 98509 is an unrelated build of `@semio-tech/norm-din4108`, `@semio-tech/norm-din4108-rs` and `repo-mcp`. Both were read through the host process interface. No capacity owners were changed.


## Stdio-specific warm recovery

In w8jCBpl, the current BREP representation, mesh preview, solid transaction and curve sources all met initial source admission. Two of the files changed again while Cargo was working. The current curve nearest source is 4,091 bytes, SHA-256 `3cbb9efc2e7379b00207bd18fbcf6942e07a58648951a1f4aebdfcf9fb07dfa4`. The actual 18:44:04.059 UTC WASI curve rlib is matched to a compiler receipt containing that same SHA. The late source move therefore preceded this unit's compile. The Solid Transaction source changed to 31,878 bytes at 19:15 UTC and awaits its emitting receipt.

The captured BREP representation, curve and Solid Transaction units are in the ordinary Stdio link closure. The selected PDF component's Cargo closure has none of those units. If the PDF browser module finishes publication and only Stdio fails on BREP provenance, the ordinary recovery target is `repo run semio-hub-stdio-component:component --configuration=prototype`. Its authored manifest and implicit component target select Stdio alone. Recheck current BREP provider closure, exact source/receipt digests and the actual PDF browser product marker before invoking that recovery. Keep the existing PDF component and shared warm store in ordinary custody. This recovery has not been invoked.

The w8jCBpl pass later produced Stdio-side compile errors against the already-linked BREP representation library. Its representation rlib was emitted at 18:44:52.722 UTC. The live Annular Revolution added `new_two_steps` after that at 18:46:21 UTC. The live `BorePreviewSource` received the bore span and preview capability methods at 19:44:56 UTC. The newly dispatched Boolean and Mesh IO sources expected the changed API. Their compilation errors identify the mismatched provider. The current BREP WASI source reader accepts the actual sources, including Boolean (3,985 lines), Native Registry (3,971 lines) and Engine (3,989 lines). No BREP source fix was applied. The coordinated pass continues independent PDF and other output production.

The same pass exposed additional late provider changes used by Stdio's BREP Analysis and Mesh IO. Rust separately rejected the live Mesh IO match-arm token `7..=>`. The pinned nightly compiler reproduced the same parser diagnostic and accepted `7.. =>` in a focused fixture. By the apply-time reread, the shared file already contained the valid separation. The workspace source was left as authored. The retained `📜️brep-range-grammar.py` reproduces the parser check through repo tooling.
