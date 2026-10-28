# result-mwrebase: builderlock `multiwriter` rebased onto claimsctl + factview (10-27 ~21:10)

FOR coordinator:
- check: `python3 -I C/builderlock-tools/staged/multiwriter/stage.py check` -> 25/25 `base` on live at 21:10
- apply: `python3 -I C/builderlock-tools/staged/multiwriter/stage.py apply`
- verify (mirror-proven; on live pass the live root): `sh C/builderlock-tools/staged/multiwriter/verify.sh <root>`
- revert: `python3 -I C/builderlock-tools/staged/multiwriter/stage.py revert`
- product proof: course:test (the g46 failure 'Original lease signal identity required' came from custody waitAny)

## What conflicted
Only the two whole-file entries; the other 23 were `base`.
1. `generations/🟨️.mjs`: live (factview) extended the `build` docstring and added `if(usable)indexCache.delete("view:"+usable.factIndex);` before `compute`; multiwriter changed the same docstring line and the `build({... coverage})` signature (`claim`) right beside it.
2. `generations/🧩️claims/🟨️.mjs`: live (claimsctl) rewrote the tail of `createQueuedClaimLeasePortV1` (`follow`, `waiting`, coalesced `wait`); multiwriter rewrote the same `wait` (returns true/false with `timeoutMs`).

## How merged
- generations: multiwriter's file + factview's longer docstring (kept, line replaced) + factview's `indexCache.delete(...)` line inserted before `compute`. The view release does not touch the merge path (it reads the peer head through `readIndex`). Size 64620 B < 65536 consumer cap.
- claims: both fixes are the same on `request` (`signal:control?control.signal:signal??...`; identical text, merged cleanly). Port tail combined:
  - kept claimsctl's `follow` (the caller's signal only abandons the wait, the kernel only ever sees `control.signal`) and the coalesced untimed `wait` ticket; `follow` now disposes with `value?.release?.()` because timed waits resolve to a boolean, not a lease.
  - kept multiwriter's `wait` contract: `{signal,timeoutMs}` answers true / false on timeout; new local `shared()` does request + release + timeout-to-false.
  - paths: no control -> `shared(resource, signal, timeoutMs)` (caller signal goes straight to the file/kernel port, as before); control + timeoutMs -> `follow(shared(resource, undefined, timeoutMs), signal)` (ends by its own timeout, so waitAny's raced losers never leak a ticket and never pass a derived signal); control without timeout -> claimsctl's coalesced ticket.
- `waitAnyLeaseV1` (multiwriter) passes the caller's own signal and a timeout only, never a derived controller, so the kernel's original-signal rule holds on every path.

## Removed lines (vs live) and why
- the old `wait` one-liner and the claimsctl inline `ticket=request(...).then(lease=>{lease.release();})`: replaced by `shared()` which also returns true/false (needed by `waitAnyLeaseV1`).
- `follow`'s `lease?.release()`: now `value?.release?.()` (a boolean value has no `release`).
- nothing else of claimsctl/factview was dropped.

## Law results (mirror `mwrebase` cloned from live at 21:03 with the batch applied; guard clean: live unchanged)
- `verify.sh <mirror>`: VERIFY OK. stage check 25 applied; `generations/🟨️.mjs` 64620 B < 65536; scan.ts 13 files bad 0.
- bun test group 1 (custody, merge, multiwriter, 🔑️kernel, claims, claims/processes): 156 pass / 0 fail (kernel law 4/4 incl.)
- group 2 (generations, load, idle-builder, early-release, claims idle, plan-once, upgrade-before-custody, generation-protocol): 179 pass / 0 fail
- storage 'normal Graph state uses the same full resource': 1/0; withheld-dependency 'the full graph': 2/0
- claimsctl law `claims/🧪️tests/🎛️control`: 4/0 on the merged file; factview laws (view-release, index-chunks, fact-index): 11/0 (plan-once is inside group 1/2).
- Mirror dropped after the run. Backups: `files|base/<f>.pre-rebase` hold the pre-merge copies; `manifest.json.pre-rebase`.
