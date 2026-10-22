# result-lock-sync (executor lock-sync, wave 1020e, ROUND 5)

READY (updated after conc-ts verification: `renderer` group REMOVED; conc-deps group 3 live): batch staged and rehearsed, NOT applied, lock/sync NOT run (waiting for ROUND5 GO after chain16 green). `C` = this ticket folder, tools `C/lock-sync-tools/`, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

Nothing was written outside `C/` (and `/private/tmp/lock-sync`, deleted again). No `bun repo`, cargo or bazel was run. One try of bare `bun install --lockfile-only` in a copy (teaching root, dead registry, empty cache) failed with "failed to resolve" for every package: an offline lock cannot reproduce the result without the image's warm `bun-cache`, so the real `lock --all` needs that cache (the run script checks it).

## 1. Starting point (measured, read only)
- Oracle `lockdelta.py` (pure python, compares every root `package.json` + member manifest with the `workspaces` section of the root `bun.lock`): all six locks equal their manifests today (members 135/310/306/314/136/308, 0 added, 0 removed, 0 changed). Sensitivity proven: on the overlay with the batch it reports every edited row.
- Last manifest edit of any root/member is older than the newest generations (22:59:53), so the six live generations (`j/e7c8cff97e60`, `8fe96cbeac42`, `271e4186d341`, `4c55952662cb`, `2815d9cfde5b`, `2d8eb7081eb5`) reflect all manifests applied so far. Six older generations (19:43-19:45) are unreferenced.
- `sync-incr` (recipe v6, clonefile) and `conc-deps` groups 1 and 2 are live (their `apply.py check` refuse because the anchors are gone; markers `admissionRefused`, `🚦️concurrent`, `🧾️incremental` present). `nlink.py`: 2815 tracked manifests, 0 with nlink != 1.
- Dependencies image `.cache/b/d/m`: 8.0 GiB, 1.02 GiB free (87 % used), `pointer/` directory does not exist yet (the live publication protocol has not run once).
- Running now: chain16 `show projects` (41121), a peer `bun repo cargo ... hub` (45216). Load 34-36. `.nx/installation` dangles (target replaced); chain16's `tools setup nx` repoints it.

## 2. Inventory of manifest changes and disposition
| source | change | disposition |
|---|---|---|
| bzl-default `manifest` | tools recipe `package.json` drops `dependencies."@semio-tech/framework-filesystem"` (closed pinned-recipe schema; devDependencies row stays). Supersedes the frozen part of zt-e2e-fix | in batch (`manifests`) |
| bzl-default `manifest` | hand edit of the root `bun.lock` row | DROPPED: the lock is regenerated from the manifests; a hand edit only adds a second writer |
| bzl-default `manifest2` | root `package.json`: member row for the new package `🏗️build/🚀️bootstrap/📦️parsers` (`@semio-tech/build-parsers`: typescript 5.9.3 + @iarna/toml 2.2.5), 4 Nx rows (`nx`, `@nx/js`, `@nx/devkit`, `@nxlv/python`) out of devDependencies; new `📦️parsers/package.json` | in batch (`manifests`) |
| **gap found** | manifest2 only added the member row to the ROOT. All six roots list `🧰️framework/**` members (`../` prefix for the five product roots), so the planner's invariant "a member path exists in every root that includes its area" (puzzle-5d-ts defect of manifest-batch2) would break | FIXED: same row in the five other roots (anchored after the tools row, sorted position asserted) |
| bzl-default `closure` | closure-scoped JS install (13 files, 37 edits, 3 new files; inert until `javascriptInstall: closure`) | in batch; **rebased**: `script-inputs/🟨️.mjs` closure-2 now matches twice (a second `createRequire(...)("typescript")` appeared), counts `[2]`, both are the same expression; the `createRequire` import has no other use (checked) |
| bzl-default `nx` | Nx resolved from `.nx/installation` (9 files, 14 edits). Must go with the root Nx rows, else code still imports a root `nx` that the sync removes | in batch; refuses while `.nx/installation` dangles |
| conc-ts open item (TS2742 x20) | REMOVED: conc-ts verified that the generated exports live since 00:10 clear all sites; no dependency edit needed, group `renderer` deleted from plan, apply, run and lockdiff |
| conc-ts exports (generated entries of framework-manifest) | `🧰️framework/🔨️modules/🛂️manifest/package.json` exports | already live (00:10), nothing to do here; exports are install inputs (see 3), so they are part of the same cycle |
| testfix-real | `🧰️framework/📦️packages/🟦️typescript/package.json` export keys removed | already live, locks unaffected (oracle 0), already part of the 22:59 generations |
| testfix-real, iface-wrap `lock-sync.py`, gran-value, cargo-fix, lockref | Cargo.lock / Cargo.toml only | out of scope (`dependencies lock` is JavaScript only: `SyncScript` -> `prepareJavascriptDependencies`); cargo window |
| bzl-default `flip`, `setup`, `prep`, `commands`, `probe` | code/docs | not lock-relevant (`flip` is gated, `setup` has drift for its owner) |

Result: `C/lock-sync-tools/staged/plan.json` = 29 files with 60 anchored edits + 4 new files = 33 files (manifests 9 edits + 1 new, closure 37 + 3 new, nx 14); live dry run clean. Longest absolute path 158 UTF-16 units (before the removal) (limit 256). Backups and a ledger go to `C/🗑️generated/lock-sync/`.

## 3. Predicted lock delta (`staged/predicted-lock-delta.json`, from `lockdelta.py --overlay` on the overlay with the batch applied)
| lock (root) | workspaces added | workspace rows changed | packages |
|---|---|---|---|
| root `bun.lock` | +1 `@semio-tech/build-parsers` | 5: devDependencies `nx`, `@nx/js`, `@nx/devkit`, `@nxlv/python` removed; tools recipe loses `framework-filesystem` | +1 key `@semio-tech/build-parsers` |
| `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech` | +1 each | 1 each: tools recipe row | +1 key each |

No third-party package is added, removed or re-versioned: all six locks already hold `typescript@5.9.3` and `@iarna/toml@2.2.5`, and `nx`, `@nx/js`, `@nx/devkit`, `@nxlv/python` stay because the tools recipe and R still depend on them (2-3 workspace rows per lock). So `lock --all` needs no new registry resolution (but see the cache requirement above). `lockdiff.py` compares the six locks before/after the real run against exactly this prediction and fails on anything else (negative control: a changed `typescript` entry is reported).

Predicted sync delta: all six scopes change generation key (every root lists the framework members, and the key covers canonical manifest bytes except `scripts` and `semio`, see `📥️install.json`): six new generations, then `node_modules` of every root repoints; the root gains `node_modules/@semio-tech/build-parsers`. Incrementality does not help for this batch: any framework `package.json` edit, even `exports` only, rebuilds all six. Bundle every pending manifest edit into this one cycle.

## 4. Run script `lock-sync-tools/run.py`
```
cd /Users/ueli/Documents/semio
python3 C/lock-sync-tools/run.py                       # preflight only (read only), safe any time
nohup nice -n 10 python3 C/lock-sync-tools/run.py --yes > C/🗑️generated/lock-sync-run.log 2>&1 & disown   # after ROUND5 GO, from the main session
python3 C/lock-sync-tools/run.py --yes --from lock     # resume at preflight|apply|lock|sync1|sync2|verify; --only PHASE; --without nx|closure; --accept-lock-delta
```
Phases (log, state and probe output in `C/🗑️generated/lock-sync/run-<stamp>/`, heartbeat every 60 s, SIGINT/SIGTERM stops the child cleanly and the run resumes with `--from`):
1. preflight: no `bun repo` process at all (a build-module or manifest edit aborts them), load <= 30, warm bun cache present, `.nx/installation` resolves, `nlink.py` 0, `lockdelta.py` = 0 before, batch dry run without drift.
2. apply: snapshot of the six locks, `apply.py --write` (all-or-nothing, temp + rename, rollback), `lockdelta.py` prints the predicted delta per root.
3. lock: `bun repo dependencies lock --all` (timed), then `lockdelta.py` must be 0 (locks equal manifests), then `lockdiff.py` before/after vs prediction (stops before the sync on surprises; `--accept-lock-delta` after review).
4. sync1: `bun repo dependencies sync --all` with `probe-readers.py` beside it; sync2: again, must print `Store dependency sync current`, probe again.
5. verify: nlink, locks = manifests, `bun repo --help` x3 and `dependencies ensure` x3 timings (rc 0), all six `node_modules` switched to existing generations, `build-parsers` linked in the root, pointer epochs, image free space, probe summaries. `state.json` holds the numbers; `ALL GREEN` on success.
Admission: conc-deps group 3 is live (256 MiB per scope, budget check passes on this host, stale-scratch sweep), which removes most of risk 1 below; the probe still logs free bytes.
Undo before the lock: `python3 C/lock-sync-tools/apply.py --undo --write`.

Rehearsal: `lock-sync-tools/selftest.py` runs the whole `run.py --yes` on a throw-away git copy (36 files + all member manifests) with a stub `bun` (lock regenerates the six locks from the manifests, sync switches `node_modules`, second sync prints current): ALL GREEN, lockdiff passes, nlink 0, all six switched, ensure rc 0. Also: apply idempotent (65 already), undo byte-identical (33 files), syntax of all changed `.ts/.mjs/.json` files (`bun build --no-bundle`, JSON parse) ok in the overlay.

## 5. Reader impact (design read from conc-deps, to be measured by the run)
- Readers that already hold the bindings gate + pin keep the old generation; the pointer replacement is the only exclusive moment (milliseconds); builds run under the `build` lease with the image shared. Expected: `bun repo --help` (no installation needed) unaffected; `node_modules` of each root stays valid until the switch (probe checks `readlink` + `typescript/package.json` + `@semio-tech` every 5 s and counts breaks).
- NOT non-blocking: every command that calls `ensureBuildEnvironmentV1` (all `run/exec/test/...`) sees the root scope unready from the moment the manifests change until the publish, and coalesces behind the build (progress line every 10 s, EN/DE). So the window between apply and the first publish (lock + first build) is a real freeze for new commands. Keep it short: nothing else may start between phase 2 and sync1, and the sync uses a 15-min publish drain for readers that hold the gate.
- Probe (`probe-readers.py`, also runnable alone): per tick `bun repo --help` wall time, continuity of all six `node_modules`, image free bytes; every 60 s `dependencies ensure` wall time (this is the measured wait of a new consumer). Summary p50/p95/max/failures per probe in `state.json.verify.sync1Probe|sync2Probe`.
- Open design point: a lock-then-sync that rebuilds six scopes at once makes the freeze as long as the longest scope. A phased variant (root scope first: `lock --scope .`, `sync --scope .`, then the rest) would release `ensure` early because its predicate checks only the bootstrap scope; not scripted because `--scope` owner names were not verified. Ask if wanted.

## 6. Estimates (not measured here, no lease/idle machine)
Lock: 30-40 min (manifest-batch2: 33 min at load 45-66, 13.4 M ownership operations; deps5/walkperf cut the walks). Sync1 (six generations, clonefile): 30-40 min (deps5 estimate at comparable load; incremental tiers help only the second pass). Sync2: gate, seconds to a minute. The run measures all three.

## 7. Risks and what the coordinator must know
1. Image capacity: 1.02 GiB free of 8.0 GiB at first measurement; with group 3 admission is 256 MiB per scope (was expected size x 1.15, default 1 GiB); the six unreferenced 19:4x generations are what the pressure rule frees first. If admission refuses (EN/DE message) nothing is touched; the next run resumes. The probe logs free bytes.
2. First live use of the side-by-side protocol: `pointer/` does not exist yet and shared-custody writes into the dependencies image are unproven (conc-deps open item 1). If the first build fails at open there is no fallback by design: stop and report to the conc-deps owner.
3. Freeze: any other `bun repo` process aborts or makes preflight refuse (currently the peer cargo `metadata` on hub and `show projects`). Edits to the closure/nx files touch loaded build modules.
4. Order against round 4: conc-ts exports, agents-fix, cli-zt2, xplat-linux-fix overlap my files only through anchored lines (`📜️script.ts`, `🔣️policy.json`, `script-inputs/🟨️.mjs`, dependencies `💾️storage/🟦️.ts`, `nx.json`); apply refuses on drift, then `python3 C/bzl-default-tools/plan-round2.py`/`build-plan.py` re-derive. Best order: round 4, then this batch.
5. (removed) the `renderer` group and its low-confidence caveat are gone: conc-ts verified the exports alone clear TS2742.
6. Cargo locks are untouched; testfix-real's hand-inserted `Cargo.lock` entries and gran-value's removed dependencies still wait for the compile window / lockref.
7. The `nx` group moves Nx resolution to `.nx/installation`; with Nx removed from the root manifest every Nx caller outside the repo product scope must go through the pinned installation. The root `patchedDependencies` entry stays (patches are projected for every scope).

## Files
`C/lock-sync-tools/`: `apply.py` (batch apply/undo), `build-plan.py` (regenerates `staged/plan.json` from the bzl-default plan + new edits, live read only), `staged/plan.json`, `staged/new/**` (4 files), `staged/predicted-lock-delta.json`, `lockdelta.py` (oracle), `lockdiff.py`, `lockio.py`, `probe-readers.py`, `run.py`, `selftest.py`. Generated outputs of my rehearsals were deleted.

## Round 6 re-stage by integrate2 (script.ts split)
closure-14 (`createRequire(join(root,"package.json")),parsers=` -> `join(base,...)`) dropped: the parser witness of 🚀️bootstrap/🟦️.ts no longer has that loader (it reads installed package directories of the root scope only), so the scoped-root intent is moot. round6-tools/staged/make-lock-sync.py.
