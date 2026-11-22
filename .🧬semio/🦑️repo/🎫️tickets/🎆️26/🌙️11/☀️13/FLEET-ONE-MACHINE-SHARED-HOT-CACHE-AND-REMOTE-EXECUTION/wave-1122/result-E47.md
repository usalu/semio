# E47 harness-drift (halted by coordinator, state as of halt)

## New harness
`🧰️framework/🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts` now exports `loadProductFunctionsV1` (async) and `loadProductFunctionsSyncV1`: load named functions from the REAL product module with every module-level helper and import they reach transitively; only names in `ports` are replaced; a moved/missing helper or import throws at load (no silent pass-through stub). Re-exported via the public `@semio-tech/process/testing/extracted-function`. 3 new tests in its test file (7 pass).

## Before / after (pass/fail)
- dependencies/💾️storage/🧪️tests/🟦️.ts: 28/20 -> 47/0. Causes: stale ports (group staging API), busy now returns `{acquired:false}`, path limit 1024 (nested dirs), membership message, recipe v7/hoisted, missing-parent now creates `j` (corpus row updated), link stamp ctime 0 (oracle), duplicate scopes refused earlier; ticket diagnostic script `tooling-diagnostic/📜️script.ts` (old ticket 10/14) repaired: facts 8->7, `[DEBUG]`->`[DATA]` for two refusal prefixes.
- 🏪️store/🧪️tests/📦️child: 5/5 -> 9/0 (scope shape gained top/queue/cleanups; real module loads)
- 📐️layout: 2/1 -> 3/0
- 🧹️retirement/📄️generation: 5/1 -> 6/0 (alias identity is now rows+ancestors)
- 🏗️native-build/📦️owner: 1/11 -> 11/0 (directory-envelope ports, dispatch key port, scope.options, backing allocation = declared extent)
- cohort-controller: 0/1 -> 1/0 (renewable control port)

## Open / not done
- Not run after edits: neighbours reading `📥️install.json` corpus (reading/, 🏭️producer/) after the missing-parent row change; verify them.
- 🏪️store/🧪️tests/🟦️.ts (spawns nested bun test, child output empty): environment/Original cluster, untouched.
- Not touched: prepared-worker, anchor, 🗃️unready/♻️roles.ts, headroom, queued, reset, reservation re-check.
- Law test forbidding extracted function text NOT written. Plan: ratchet test with an allowlist of the ~95 files still using `new Function`/`evaluateExtractedFunctionV1` (101 listed at scratchpad E47/files.txt), list may only shrink.
- Remaining `new Function` uses remain in several converted files (other tests inside them).
- Full 101-file sweep was started and stopped by the halt; no results.
