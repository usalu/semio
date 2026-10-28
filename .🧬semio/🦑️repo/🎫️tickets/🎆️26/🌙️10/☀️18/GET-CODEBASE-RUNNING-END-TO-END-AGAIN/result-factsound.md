# result-factsound (lane factsound, 10/27): why the graph memo's recheck is `stale` while the graph child serves the generation as verified

STATUS 08:32: DONE. Cause measured on the live record, fix staged + mirror-verified. NO graph soundness bug: the memo was the inconsistent part. Nothing applied live, no official run, mirror `fslice` dropped.

FOR coordinator: batch ready `C/factsound-tools/staged` (4 files, flat, house style). Apply `python3 -I C/factsound-tools/staged/stage.py apply` · revert `python3 -I C/factsound-tools/staged/stage.py revert` · state `… check` (all 4 `base` on live at 08:31) · laws `sh C/factsound-tools/staged/verify.sh` (exports the compiled watcher from `.cache/b/t/m/repo-watch/*/` for the REAL journal law).
FOR coordinator: verdict: `course:build` once (expect the owners whose command closure holds the 2 edited library files to be re-inferred, g27-like: a few projects, ~3-4 min), then hot ×4 with `gate-g-tools/product.py`: hot1 keeps a light record (new key: implementation digest moved; reason `none` prints nothing), hot2 runs the child once more and SEALS (sidecar ~0.8 s CPU instead of 3.2 s), hot3 and hot4 must print `graph: N projects served from the memo …` (`… 288 facts outside its sight held` order of magnitude, recheck ~0 s instead of 1.8 s). Caveat: any journal row in a relevant directory between hot2 and hot3 (peer edits under the slice's directories, or a mirror `create` before hotpath's `clonejournal` lands: clone-only FSEvents rows) still gives `cannot serve (changed: <path>)`, which is correct but not this batch's fault.
FOR coordinator: the out-of-slice owners whose facts ARE stale since the 10/25 21:28 node_modules relink (`@semio-tech/s-dev-composition`, `os-hub-ts`, `hub-compositions`, `hub-stdio-test-oracle`, draw-js, gis-gismap-js, space-plugin, …) are re-inferred by the first request whose closure contains them (by design of `refreshClosure`); no action needed.

## 1. Cause (measured)
- `C/factsound-tools/owners.ts` (read-only) maps every stale recheck row of the sealed course record `549f20e0…` (generation 609, slice of 198 owners from the pin's `covered` bitmap) to the generation owners that read it (fact-index classes): **all 212 stale rows (187 typescriptRealpath, 24 typescriptText, 1 resolve) are read ONLY by owners OUTSIDE the slice** (`@semio-tech/s-dev-composition` 189, `os-hub-ts` 70, `hub-compositions` 7, `hub-stdio-test-oracle` 4, draw-js 2, gis-gismap-js 2, space-plugin 2, 6 more with 1). **0 of the 198 covered owners reads a stale row.**
- The planner is sound: `refreshClosure` verifies exactly the facts of the requested closure + `@env` + derived supports, `🕸️graph/🕰️generations/🟨️.mjs:438` (`selectFactClassesV1(index, scopeWithDerivedV1(index, only ∪ @env))`), by design (:645 "an edit elsewhere neither waits for a builder nor is refreshed for this caller"). Those facts are checked with the same engine: `typescriptRealpath`/`typescriptText` → `recordedFactHoldsV1` (`👁️observation/🟨️.mjs:416,431-432`), never vouched by stable directories (`:939` only lstat/exists/readdir kinds). Nothing treats node_modules/.cache as immutable, nothing carries facts forward unverified for a requested owner; the verified stamp (`🔭️coverage`) only answers waiters whose request is older than the stamp (`coverageOfV1`: `stamp.atMs<requestedAtMs` → none).
- The MEMO classified the WHOLE generation: `🧭️invocation/🕸️graph/🧠️memo/📤️relevance/🟦️.ts` (base) `writeGraphMemoSidecarV1`, `rows=function*(){for(let at=0;at<index.rows.length;at++)…}` → every stale fact of ANY unrelated owner refused every lookup forever (`stale`), although the slice never read it.

## 2. Fix (staged; the memo changes, the graph does not)
- `📤️relevance/🟦️.ts`: new `graphMemoSliceRowIdsV1(index, manifest, pin)` = the rows of the owners the served pin covers + `@env` + derived supports (the planner's own selection, same functions); new testable `writeGraphMemoSliceSidecarV1({root,generations,exclude,pin,stem,expected,client,insensitive,signal})`; `writeGraphMemoSidecarV1(root, pin, environment, signal, client)` resolves the store + journal state and delegates. No pin / malformed pin → unsealable `pin`; pin of another manifest → `other-generation` (fail closed).
- `🧭️invocation/🕸️graph/🟦️.ts`: `writeBuildGraphPlanV1` passes the pin it just wrote.
- `🧠️memo/🧬️schema/🔣️.json`: Sidecar `$comment` declares the slice scope (no shape change).
- Soundness: a hit now means "same key, same generation, journal clean for the slice's directories + frontier, and every slice fact outside the journal's sight holds" = exactly the planner's `clean` verdict for that slice (still conservative: whole frontier + directory granularity).
- No dependency-install-identity vouch needed: the realpath rows were never the slice's.

## 3. Measured on the live record 549f20 (read-only, `C/factsound-tools/scoped.ts`)
| | whole generation (base) | slice (fix) |
|---|---|---|
| rows classified | 543,801 | 218,171 |
| relevance directories | 277,034 | 157,360 |
| recheck rows / identities | 32,162 / 3 | 288 / 0 |
| stale by the engine | 212 | **0 → served** |
| sealing CPU (child, once) | 3.15 s | 0.76 s |
| recheck CPU per hit | 1.78 s | ~0 s |
Probe peak RSS 1.4 GB (index view), 8 s wall.

## 4. Laws (`🧠️memo/🧪️tests/🎯️slice/🟦️.ts`, new; mirror `fslice`)
1. Relink law on the synthetic generation world (`🕰️generations/🧪️tests/🏃️world`): owners `a`, `b`; `node_modules → store/one`, only `b`'s analysis records `typescriptRealpath(node_modules/vitest)`; generation built; sidecars of slice {a} and {b}; relink `node_modules → store/two`: slice a recheck holds, slice b refuses; full memo lifecycle (light → sealed → lookup): a `hit`, b `{hit:false,reason:"stale"}`; planner: `refreshClosure(["a"])` not built, `refreshClosure(["b"])` built with computed=1 (only the reading owner), new sidecar records the new realpath and holds. Oracles: the engine's `observationsHold`; the rows the compute port emitted per owner (sidecar `rows` = unique rows of covered owners + env, independent of the fact index). Negative control inside the law: the whole-generation recheck (base row choice) refuses slice a after the relink.
2. Missing/malformed pin → `pin`, foreign manifest → `other-generation`.
- RED on base: `RED_ON_BASE=1 sh verify.sh` restores base `📤️relevance/🟦️.ts` → law fails (base has no slice selection; semantically the in-law negative control = base behaviour refuses), re-applies.
- Results on mirror: slice law 2/2; memo suite 9/9 incl. the REAL FSEvents watcher law; invocation neighbours isolated-plan/plan-file/taxonomy 7 pass + 1 pre-existing skip; `tsc --strict` (boot-g config + the new law, `factsound-tools/tsconfig.fs.json`) 0 errors, negative control (injected type error) caught; docstring check clean; stage selftest: revert byte-exact (new dir removed) → apply → idempotent re-apply → drift exit 2; guard clean on every run.

## 5. Status (newest first)
- 08:32 done; mirror `fslice` dropped.
- 08:27 staged + mirror green.
- 08:22 cause found (owners.ts), slice measured (scoped.ts).
