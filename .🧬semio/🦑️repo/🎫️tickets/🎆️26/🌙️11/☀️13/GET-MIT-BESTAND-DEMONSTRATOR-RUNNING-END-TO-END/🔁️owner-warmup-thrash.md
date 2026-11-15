# Owner warm-up thrash (activate-29)

Paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`.

## Findings
- Cause: fix 2 of `🐘️plan-heap-growth.md`. Under pressure (`governor.pressure()` is true while the log says "waiting for memory headroom") `releaseOwnerStateV1` (`🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`) dropped `reachState`, `walkNodes`, `declared`, `paths`, `files` after EVERY owner, so each next owner rebuilt the whole module graph and reach sets inside its own scope. `🧠️memory/🟨️.mjs` `growthOf` (`measure()-startBytes` when one owner runs, i.e. process-wide `heapUsed+external`) charged that shared warm-up to the owner: cargo-workspaces 3822, component-deployment-contract 3311, framework-os-mcp 3553 MB.
- Plan stop (5803 MB): the `recent` ring of the plan stop detail shows a sawtooth, heap 1.4 -> 2.9 GB and external 1 -> 2 GB, dropping at every release and climbing again during the next owner (heapUsed 1363 at 570652 ms, 4032 + external 3149 at the stop; 9.3M strings, 5.8M Objects). The plan baseline is the low point, so each rebuild peak counts as plan growth: the same thrash, not retained leakage by owner stops.

## Fix
- `🧠️memory/🎯️owner-scope/🟨️.mjs`: scopes carry `credit(bytes)`/`credited`; new `creditOwnerSharedV1`.
- `🧠️memory/🟨️.mjs` `growthOf` and `taken` (learned peak and recorded cost) subtract the owner's credit.
- `🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`: `chargedToSharedV1` wraps the transient `reachClosureV1` walk and credits what a build of the shared graph kept to `cache.sharedBytes` and the owner scope; `releaseOwnerStateV1` under pressure drops only per-owner memos (`files`, `closures`, `commandDependencies`) every owner and the shared walk state (`reachState`, `walkNodes`, `declared`, `paths`, `toolGroups`, half the closure entries) every `OWNERS_PER_SHRINK_RELEASE_V1` (8) owners, every 32 without pressure.
- Tests: `🧠️memory/🧪️tests/🛑️overrun/🟦️.ts` (credit never stops an owner, own growth still does), `🕸️graph/📥️inference/🧪️tests/🎯️target-fanout/🟦️.ts` (shrink keeps reach state for 8 owners, credit by `chargedToSharedV1`).

## Not verified
No activate was run (another was running); watch whether the plan still trips 4915 MB: the shared state itself is a few GB, so if it does, shrink the reach sets rather than release them.
