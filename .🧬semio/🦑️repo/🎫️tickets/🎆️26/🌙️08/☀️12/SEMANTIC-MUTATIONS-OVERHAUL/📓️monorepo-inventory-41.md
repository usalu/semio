# Monorepo Mutation Inventory 41

## Controller

The ticket-only controller is `🧪️monorepo-inventory-41/📜️script.ts` and calls only the exported `inventoryMutationTaxonomy(process.cwd(), { progress })` from the workspace `📜️script.ts`. It does not call clean, plan, or apply.

Executed through the required runner:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️monorepo-inventory-41/📜️script.ts
```

The controller uses an owned worker process. The worker checks elapsed time from its progress callback; the controller kills the worker if progress stalls for 30 seconds. Its 115-second worker deadline leaves final log handling within the requested 120-second budget.

## Retained source identity

| Input | SHA-256 |
| --- | --- |
| workspace `📜️script.ts` | `ea4ce1967af1e7ec122a26491393d8ef79a5b7beac59e3b8da8db801e943efeb` |
| ticket controller `🧪️monorepo-inventory-41/📜️script.ts` | `b22d5e4c113a24cd4e8107059b4455581f18ceea85e376b1cf0ae62693268d61` |

## Final bounded observation

The final run is retained at `🧪️monorepo-inventory-41/🧫️run-zJVt1K`.

- Controller elapsed time: 116,244 ms.
- Worker deadline: 115,000 ms; no 30-second progress stall occurred.
- Worker exited with the exact error: `[inventory-41] deadline exceeded after 115002ms during consumer-graph.`
- Source indexing had reached a 54,634-file snapshot; consumer graph processing reached 28,313 / 54,634 files before the callback enforced the deadline.
- No complete result was returned, so `🔣️inventory.json`, a source-tree digest, records, and source roster do not exist for this run. The retained `stdout.log`, `stderr.log`, and `🔣️summary.json` are the authoritative failure capture.

An earlier `🧫️run-yK2VPk` was a controller-streaming defect: the parent consumed worker stdout only after exit and incorrectly declared a 30-second stall despite worker progress. `🧫️run-wwAnrU` proved streaming progress after that repair but had a 120-second worker threshold and finished controller bookkeeping slightly above the outer cap. The final run uses the tightened 115-second threshold and is the only final observation.

## Opaque-root and completeness limits

The imported inventory implementation's source walker excludes `compose` before accessing directory entries, as observed in its root-walk condition. This controller supplied no scope that could override that policy and did not enumerate or read `compose` itself.

Even a future complete result would inventory only recognized mutation facets and their mounted consumers. It cannot alone establish that inline mutation types outside those facets are absent; that requires a distinct structural audit. This timed-out result makes no stable monorepo census claim and proposes no migration or apply action.
