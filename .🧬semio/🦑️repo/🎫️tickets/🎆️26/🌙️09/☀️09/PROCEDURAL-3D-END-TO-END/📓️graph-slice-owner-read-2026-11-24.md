# Graph slice owner read: why `semio-hub-procedural-component` took minutes (2026-11-24)

## What the log really says

- `native-component-procedural-16.txt`: "reading the owners (round 1) 92 of 3600" is NOT an owner count. The number is the elapsed seconds of the step against its display bound (`owners-round.boundMilliseconds` = 3 600 000 in `📋️slice-report/🧬️schema/🔣️.json`; same for "0 of 600" of the inventory step). Round 1 started at 218 s, so 92 at 317 s is 99 s of wall time, not 92 owners. The whole slice of run 16 took 346 s and then continued to the build (it was not hours).
- Timeline of run 16: 0-8 s host capacity, 36-217 s "checking the file inventory" (17 630 -> 68 032 distinct source observations, 1 completed owner check at 203 s), 219-321 s reading the owners (two passes, "taken"), 327-346 s final hold check. The flow hub run 15 needed 102 s for the same phases.
- With `--exclude-dependencies` only 2 owners are admitted (`semio-hub-procedural-component` and the catalog owner `@semio-tech/s-dev-composition`), but the first has 68 359 remembered rows (a 26 MB admission memo for the scope).

## Measurement

- Admission memo `.cache/b/g/m/selected/4cc59441...admission.json` (written by run 16 at 12:35): 68 505 rows. Held in isolation (`graph-slice-owner-hold-profile.ts`): the catalog owner (17 949 rows) holds in 15-60 ms, `semio-hub-procedural-component` (68 359 rows) is STALE at every read, 1.5-5 s to find out. Row by row (`stale-owner.ts`, scratchpad): exactly 3 stale rows, all `file` rows with a content sha256 of the build tool's OWN code, being edited by peers right then:
  `🏗️build/👁️observation/🟨️.mjs`, `🏗️build/👁️observation/🧩️field/🟨️.mjs`, `🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🕸️wasm-outputs/🟦️.ts` (mtimes 12:44, 12:44, 12:48, during my read).
- A stale owner is rediscovered whole: 68 k observations, about 130 s warm in the host (`dry-prof`: 106 s when nothing changes in between), and a standalone discovery (`graph-slice-owner-profile.ts`) of the same owner does not finish in 8 minutes (stage `command-sources`, event loop starved). A second dry run (`dry-warm-1.err`, same command, memo from run 16) ran 546 s without finishing: every settle round met another edit of an observation module (and a slice worker restart each time a loaded file such as `🏗️build/🔣️policy.json` changed: `dry-after-2.err`, 21 restarts).
- Memo composition: 922 `file` rows (content sha) of build-tool modules inside the owner, next to 4 `analysis` rows per module (script ports, command imports, command references, module imports) which already hold exactly as long as the import result is the same.

## Root cause

`discoverBuildOwnerAdmissionFactsV1` (stage `compile-closure`, `🔎️owner-discovery/🟨️.mjs`) recorded the CONTENT of every module of every command and compile closure, although the answer (`files`, `dependencies`, imports) depends only on the imports:

1. `observedSourceAnalysisPairV1(module,references,absolute,{recordSource:true})` added a `file` row (sha256 of all bytes) per walked module on top of its analysis rows.
2. For every import it resolved it called `readPhysicalSource(file)` (reads and discards the bytes through the observed fs: a `file` row with sha256 per imported file, json and d.ts included) only to check that the file is a real file.

Every `repo "<script>" ...` command closes over the build tool itself (the cargo driver imports observation, graph, inputs, caching, ...: 922 modules). All agents edit these modules all day, so each body edit staled every owner whose closure contains one (about every cargo owner) and forced a full rediscovery (minutes), repeatedly under churn (the read settles again each time a path changed). Same shape as the taxonomy contracts of 2026-11-23 (one consumed-too-wide row), now for module bytes.

## Fix

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs`, compile-closure loop only:
- `observedSourceAnalysisPairV1(module,references,absolute)` (no `recordSource`): the module is remembered by its analysis rows (import result).
- `requirePhysicalSourceV1(file)` instead of `readPhysicalSource(file)`: the real-file check (lstat of the ancestry and leaf, recorded as `lstat` rows) without reading or hashing the bytes.

An edit that leaves the imports as they were no longer moves any row of the owner; an edit that adds, removes or retargets an import still changes the analysis result and stales exactly that owner. The admission format hash changes once (this file is in `graphAdmissionFormatUrlsV1`), the first read after the change is cold.

## Tests

- New `🔎️owner-discovery/🧪️tests/🪶️body-edits/🟦️.ts` (2 tests, real discovery in a scratch workspace, real observation module): body edits of a command-role module (deep import of the command script) and of a compile-role source (`sourceInputObservation: compile`, glob input) keep every recorded row valid and the discovery answer equal to a fresh discovery (oracle), an added import makes the rows stale and the fresh answer contains the new file. Failed before the change (rows stale after a body edit), passes after.
- `🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts`: the fixture ports gained `requirePhysicalSourceV1` (the demanded-package test asserted the resolved files are touched; it still does through the physical check). 34/34.
- Green (each file alone, `repo tools exec --light bun -- test`): owner-discovery tests (contract-rows 3, shared-catalog 1, observation 4, owner-names 1, authority-round 1, source-matches 3, generated-trees 1, body-edits 2), admission tests (4+6+6+2+3+1+4+10+4), `🧪️tests/🕸️graph/🛂️admission` (lookahead 4, observed-catalog 2, catalog-producer 2, 🟦️ 20 with `SEMIO_TEST_ARTIFACT_DIR`), observed-facts 7, source-demand 34, repair-rounds, stale-facts. Running them together in one bun process collides on the shared test sandbox (unrelated ENOENT).

## Before / after

See the section "Measured" below.

## Not changed (measured, still open)

- A cold read of this owner is 100-130 s per attempt on the main thread (68 k observations, about 700 us each; the sample shows openat/lstat/stat/read at 46 % of main-thread samples); not parallelised. The owner farm of 2026-11-23 only helps when several owners are cold.
- A slice worker whose loaded files change (`🏗️build/🔣️policy.json`, observation modules) exits with 75 and restarts with an empty in-process memo (21 restarts in `dry-after-2.err`); on a tree where peers edit the tool this stays the dominant cost until the loaded set stops changing.

## Measured

`repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies --dry-run` (logs `🗑️generated/dry-*.err`, `dry-final-times.txt`), slice read = last "Reading the project graph slice (N s)" line:

| Run | State | Slice read | Note |
|---|---|---|---|
| procedural-16 (live run) | cold, before | 346 s | inventory 181 s, owners 100 s, 2 passes |
| dry-warm-1 | warm memo of run 16, peers editing the tool, before | 546 s, not finished (killed) | owner stale at every read |
| dry-prof | quiet tree, before | 106 s | no edit in flight |
| dry-after-2 | first read after the fix (format changed, cold) | ~520 s total, 21 worker restarts | peers edited `🏗️build/🔣️policy.json` in flight |
| dry-final-4 | after the fix | 30 s (95 s whole command) | |
| dry-final-5 | after the fix, repeated | 28 s (132 s whole command) | |

Content rows (`file`, sha256) of build-tool modules inside the owner `semio-hub-procedural-component` of the memo: 922 before, 392 after the first half of the fix (command role only), 1 after the full fix.
