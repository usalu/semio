# Result E51: scope-keyed dependency receipts (HALTED, nothing landed)

## State
No product file was edited. No lock was taken or held, nothing was written to `T/landings.md`, and no background process is left. The `bootstrap` area lock was held by E47 (14:19) from my start until 14:44, so I waited and never edited. I was halted just after it was released.

## Design (verified by reading code and live receipts)
- Live receipts (`.cache/b/d/m/generation`): every scope receipt holds all 324 `membership.texts` AND all six `inputs` groups. Even an inert-aware `sources` list is therefore global, so both must be scoped, not only `texts`. Execution rows are already per required scope. Frontier rows are global (100k).
- Ownership rule = `closurePlan.ownerOf` in `🗂️workspaces/🟦️bun/🟨️.cjs`: deepest declared scope directory that contains the manifest's directory.
- Plan:
  1. Bun walk: `scopesPlan` stores `owner.scopeDirectories`; a shared helper `deepestScopeDirectory` (also used by `closurePlan`); `observedOwnership` returns a new `textOwners` (path to scope). `texts` is unchanged, because `🗜️packed` pins `textsSha`.
  2. Receipt schema: `Text.owner` required, `walkStamp` /3 to /4 (one re-key, no migration).
  3. Capture: keep only texts and `inputs` groups whose scope is in `required`, and drop frontier `s` rows of foreign manifests.
  4. Verify: a text owner or input scope outside `required` gives `receipt-invalid`.
  5. Glue (`🧾️generation/💾️storage`): `manifestTextOwnerV1` reads `textOwners`, throws if missing.
- Expected effect (required closures: `.`=[.], sa=[.,sa], sb=[.,sa,sb]): a framework or root manifest edit stales all scopes; an `sa` edit stales sa and sb; an `sb` edit stales only sb; inert edits stale nothing.

## Prepared, not applied (scratchpad `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-kfaIfm/ccee8eb5-3363-4a84-af9e-53e38d236627/scratchpad/E51/`)
- `patch.py`: anchored, count-asserting edits. Run `bun`, `gen`, `oracle` and `gentest` in this order under locks `bootstrap`, `caching-bootstrap`, `workspaces`. `gen` changes the schema, core, glue and the existing fixture test together.
- `t/scoping.ts`: new test, to be copied to `🧾️generation/🧪️tests/🧭️scoping/🟦️.ts`. It covers the owners law against Python, scoped capture, a five-edit matrix (TS equals Python oracle), foreign stat drift, and receipt-invalid.
- The relative import depths in `scoping.ts` (the toolchain path needs ten `../`) are unchecked; the test has never been run.

## Not done / unverified
Everything: no code landed, no test run, no live probe, no walk time measured. Frontier `d` rows stay global, so a structural change (added or removed package directory) in any scope still stales all receipts. Other lanes: `Text` gets a required `owner` and the bun observed ownership gets `textOwners`; no other file consumes them today.
