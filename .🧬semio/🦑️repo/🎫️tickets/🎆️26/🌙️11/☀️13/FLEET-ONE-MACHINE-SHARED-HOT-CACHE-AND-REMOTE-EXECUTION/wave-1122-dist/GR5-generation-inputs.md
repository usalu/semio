# GR5: derived artifacts are not generation inputs (2026-11-23 15:20 to 15:55 Mac clock)

L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## 1. Why `units/sealed/…` was an input
- It was a real, git-tracked directory `units/` in the checkout root (349 files, 13 TS files under `sealed/<hash>/`). Cause: a lane probe (`P1-seal.ts` / `N1-live-probe.ts` style, `ensureUnitCacheShimV1(<relative root>)`) was called with the relative cache root `units`, so `sealModulesV1` (`L⚡️caching/🦀️cargo/🗃️unit-cache/🏛️seal/🟦️.ts:111`, `join(root,"sealed")`) sealed the wrapper sources into the workspace and the wrapper script `units/bin/rustc-wrapper-<hash>` even embedded a relative path to them. Birth 11-22 15:51, resealed 11-23 06:49.
- Why it counted: the one roster walk (`L🏗️build/🕸️graph/🧭️roster/🟨️.mjs`, `descends`) skips directories by NAME from `⚡️caching/🔣️policy.json` `generatedDirectories` (plus a fixed list). `bin` was listed, `sealed` was not, so `units/sealed/**` became unowned workspace members, `plan` (`🕰️generations/🧭️plan/🟨️.mjs`, `else{envStale=true;…}`) marked the environment stale for every file added or resealed.
## Changes
- `L⚡️caching/🔣️policy.json`: `generatedDirectories` + `sealed` (the unit cache's declared `seal.directory`). One source for the walk, the owner discovery and the frontier.
- `L⚡️caching/🦀️cargo/🗃️unit-cache/🏛️seal/🟦️.ts` and `🚪️launch/🟦️.ts`: a relative cache root is resolved to absolute, so a store can no longer land relative to the cwd.
- Deleted the stray derived tree `units/` (nothing referenced it: no process, no tracked consumer).
- Law test (new) `L🏗️build/🕸️graph/🧭️roster/🧪️tests/🚫️non-inputs/🟦️.ts` + `🐍️.py`: 2/0. (a) the unit cache's `seal.directory` is in `generatedDirectories` and `ROSTER_RULES_V1.descends` refuses every declared name; (b) a real frontier walk of a scratch tree with `units/sealed/…`, `units/bin`, `.cache`, `node_modules` returns no member below them and equals an independent Python walker (reads the policy JSON and the unit-cache schema).
## 2. A removed input settles once
- Mechanism already correct at the frontier: the same test removes a root member (`launch-kit`), `advanceFrontierV1` + `diffFrontiersV1` report exactly one `removed` change, and a second advance from the new frontier is `{unchanged:true}` (asserted). In `plan` an unowned removed file sets `envStale` once; it repeated 112 times only because no read has sealed a generation since 11-20 (every read restarts from the same published manifest), not because removal is not handled. The three removed `*-launch-kit` files and the 349 deleted `units/` files are therefore noticed by the next completed generation, once.
## 3. `closure-probe`, real cause
In the live read (below, GR3 code is current) all 7 `[graph-stale]` events are `source take, kind closure`, why `closure-entry-stale (recipe <hash>, …)`: 1 names a moved row (`analysis …/🏃️process/📦️artifacts/🐹️go-build/🟦️.ts`), 6 `no moved row found` within the 400 ms budget; none is a read-parent refusal and none is another exception. So cause (a) of GR3: a take met a stored command-closure entry whose digest changed (recorded b5bea41b6c62, now 9067b12990aa for animate-plugin) because the tool sources under `🏃️process/…` are edited all day. Effect is bounded: the owner is inferred here instead of taken (3 to 4 of each 60 owners), it does not restart the read. A real cure is GR3's open (b), per-query read sets for the `digest` query; not done (record semantics).
## 4. One read on the coordinator
Not started by me: `repo fleet status` already showed run 34600 (`default show owners`, background, code current, started 15:23, runner 34597, reason `refresh`, generation 261, `refreshing 5097 projects`, 21,689 stale facts). I observed it (log `.cache/f/graph-server/read.log`, record `read-record.json`) every 2 min:

| clock | wave-1 owners done (present / inferred here) | whole-graph owners | restarts |
|---|---|---|---|
| 15:37 | 58-60 of 128 | 63/5222 | 0 |
| 15:40 | 66 (62/4) | | 0 |
| 15:44 | 75 (75/0) | | 0 |
| 15:48 | 83 (83/0) | | 0 |
| 15:52 | 88 (85/3) | 102/5222 | 0 |
- Restarts on stale: 0 in 29 min (`refreshing 5097` printed once, status null, still beating). Wave 1 advances about 3 owners per minute, more than any read since 11-20 reported at this stage. It still carries the `units/sealed` and launch-kit stale lines from its start (16 lines, printed once at the start, they did not restart it); it started before this lane's edits, so whether the policy change makes its last step restart is the open question the next lines of the log answer. At 15:52 the whole read is at 102/5222; at ~3 owners/min wave 1 ends about 16:05, the 5,094 remaining owners need the fleet's workers (GR2/GR4) and were not reached in this window.
## Tests
roster `🚫️non-inputs` 2/0; unit-cache `🏛️seal/🧪️tests` 10/0 (674 s under load 40, no `units/` recreated afterwards). Landing in `T/landings.md`. Not run: other graph members (policy change affects the graph identity: the next generation is cold-identity once, expected).
## Open
- Graph identity: the policy.json edit changes the digest of an input of the generation, so the first generation after it will re-verify; if the running read restarts at its end, that is this edit.
- `no moved row found` (6 of 7): the 400 ms budget or a moved layer; no change made.
