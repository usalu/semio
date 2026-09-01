# Source Index Inter-call Write Order 76

## Current Effect

An exact root Git observation reported that three existing ticket-run JSON files are untracked and nonignored: `raw.json`, `receipt.json`, and `critical-before.json`. This is an input observation, not a completed pair result. The current pair controller also creates, after its first SourceIndex/snapshot boundary and before its second invocation, three before JSONL artifacts and `before-manifest.json`. Under unchanged admission of nonignored untracked files, those four writes can alter the second call's admitted source membership.

The new [neutral vector](../🧪️source-index-intercall-write-order-76/🔣️.json) models only that supplied finite union. Ajv validates its closed shape. Lodash independently projects the added paths: current ordering adds all four paths to the second capture; deferred materialization adds none to the second capture and materializes the same four only after both captures. It neither executes Git nor claims a real pair outcome.

## Desired Actual Source Boundary

The ticket [source test](../🧪️source-index-intercall-write-order-76/📜️script.ts) parses the actual pair controller with the TypeScript AST. It requires two direct `firstIndex`/`secondIndex` `invoke` statements before every snapshot or direct write statement; two snapshots; three nested `writeJsonLines` calls; and one literal `before-manifest.json` write. The old ordering is retained only as the supplied neutral current-effect comparison, not as a success predicate.

Once ordering is satisfied, the same controller extracts `projectSourceIndex` and evaluates it in a bounded `node:vm` call. The exact projection must be a distinct object with only `admission`, `roots`, `files`, `sourceRoster`, `sourceTreeDigest`, `taxonomySchema`, `mutationDescriptorSchema`, and `ledger`; it must omit `bytes`, `contents`, and `directories`, while retaining both schema byte buffers. The controller captures the actual source, vector, and schema with lexical Compose rejection, no-follow root/ancestor/file checks, descriptor identity checks, and before/after source identity/hash comparison. It invokes no SourceIndex, Git, collector, or filesystem walk.

## Smallest Controller-only Remedy Proposal

Add `projectSourceIndex(index)` with the exact eight metadata fields above. Make `invoke` wrap the actual API result in that projection, capture `firstIndex` and `secondIndex` before either unchanged snapshot/writer executes, then pass those projected values to the existing snapshots. This is a lexical reachability boundary for the controller's full SourceIndex values, not a forced-GC or allocator claim. The existing writers then materialize their unchanged rows and manifests only after the second projected invocation.

The parent-owned run directory and `critical-before.json` remain pre-first-call evidence, so this proposal intentionally removes only inter-call observer writes. The fixed cancellation file is written only at the existing cooperative deadline; if it is reached before the first capture completes, no second call is attempted. Existing no-follow input capture, fixed120s/15s child ownership, concurrent pipe draining, and terminal/log retention stay unchanged.

## Boundary

The source test passes only after both the desired ordering and exact metadata-projection boundary are present. Its current RED proves the desired ordering is absent plus a language-neutral finite untracked-union consequence; it does not prove a completed real pair changed membership, establish source bytes, or attribute the historical timeout.

## Executed Source-only RED

The bounded command was:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-index-intercall-write-order-76/📜️script.ts" check
```

The earlier [run-tulQM8](../🧪️source-index-intercall-write-order-76/🧫️run-tulQM8) remains an exploratory old-order observation. It is not an unchanged regression endpoint because it unconditionally failed when it detected the old order.

The superseding desired-predicate run is [run-VyxbAs](../🧪️source-index-intercall-write-order-76/🧫️run-VyxbAs). [result JSON](../🧪️source-index-intercall-write-order-76/🧫️run-VyxbAs/result.json) retains actual controller SHA-256 `d9cd27afb325fa28abf5d4d0f14897fbf1892855435459757e1e610eb9fcef46`, two total invokes, zero direct projected invoke statements, two snapshots, three snapshot JSONL writes, one before-manifest write, and `bothInvokesBeforeMaterialization=false`. It therefore fails before attempting the legitimately absent helper. The independently projected finite case still adds all four paths to the second capture under current ordering, adds none under deferred materialization, and materializes the same four after both captures. [failure JSON](../🧪️source-index-intercall-write-order-76/🧫️run-VyxbAs/failure.json) records this intended RED and stable controller/vector/schema captures.

Earlier packet76 execution attempts exposed ticket-runner path and assertion defects before this source-law endpoint; they are preserved as controller-authoring evidence and are not reported as actual pair-controller findings. No pair, SourceIndex, Git, or admission call occurred.
