# result-slice-play2 (wave 1022a, executor slice-play2)

STATUS 12:05: play activation NOT reached. Eleven `activate-prototype --dry-run` attempts (logs `🗑️generated/slice-play2/dry1..11.*.log`, retry wrapper `slice-play2-tools/retry.sh`) each stopped on a different fault of the round-6 live transfer / lock burst; the last real blocker is capacity, not code: the graph generation was invalidated by the transfer and is being re-inferred at ~4 projects/min.

## Live edits by slice-play2 (all unverified except where stated)
1. `build/🚀️bootstrap/📦️parsers/🟨️.mjs`: now only `parserTypescriptV1 = () => createRequire(import.meta.url)("typescript")` (literal, derivable by the closure capture; node resolution walks parsers/node_modules then the root install). Removed `parserRequireV1` (computed `own(name)` on a `package.json` base was refused by the captured loader: "Uncaptured implementation require refused: typescript (required by 📦️parsers/package.json)").
2. Consumers switched to `parserTypescriptV1()`: `⚡️caching/📥️inference/🟨️.mjs`, `graph/📥️inference/{🧶️bun-lock,🔎️owner-discovery,🧩️component-targets,🧱️project-assembly,🧭️script-inputs}/🟨️.mjs`; bootstrap closure test `🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts` rewritten for the accessor (not run).
3. `⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs`: literal `descriptors` map (6 `require("./<manager>/🔣️.json")`) instead of `require("./"+folder+"/🔣️.json")` (computed require refused by the capture). `node -e` check: allManagersV1() = 6.
4. `graph/📥️inference/{♻️revision,🕸️dependency-edges,🧭️script-inputs,📦️project-nodes}/🟨️.mjs`: `javascriptManagerV1.primaryLockName()` -> `.primaryLockName` at 9 sites (the cjs accessor returns a string; `TypeError: primaryLockName is not a function` stopped graph refresh). Accessor test in managers/🧪️tests (property form) already pins the string form.
5. Ran `bun repo dependencies lock --scope .` (pid 81358, rc 0, bun.lock 09:41) once the 09:23 package.json burst had landed; peer integrate3 then ran `lock --all` + sync1 + sync2 (rc 0).
(A schema row I added to ComputedImportSites for the managers .cjs was reverted again; the table is not the mechanism for relative literal requires.)

## Timeline of blockers (dry1..dry11)
- dry1/2: `Stale loaded Source revision reader`, `Authorizer DATA directory changed` (peers editing under me).
- dry3/4: `Uncaptured implementation module refused: /private/tmp/i6/ovlaws/...` = Bun 1.4.2 transpiler cache leaking integrate3's overlay clone paths (fixed live by course-build2: `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` in launcher registry).
- dry5/6: `lockfile had changes, but lockfile is frozen` (package.json 09:23 vs bun.lock) -> fixed by lock/sync above.
- dry7: backends computed import (peer declared at 10:18), then managers .cjs computed require (edit 3).
- dry9: parsers require (edit 1/2). dry10: reached graph refresh ("refreshing 2842 projects", 68k stale inputs), died on primaryLockName (edit 4).
- dry11: passed all faults, queued on `repository-graph-facts:...:shard:7` (exclusive) behind `show projects --json` 78521/82797 (integrate-r7's `sh -c nice -n 10 bun repo show projects --json`, started 11:18). I KILLED my own queued dry11 (59227 wrapper, 59229, 63208) at 12:05 so it does not hold an idle 3.5 GB capacity grant.

## Capacity finding (needs the coordinator)
- The round-6 transfer changed inferrer stamps, so EVERY per-project record is invalid: the builder prints `Inferring projects: N of 3743` (N=447 at 12:01, was 376 at 11:41: ~4 projects/min -> ~13 h for the rest). The earlier cold generation ran 27-53/min with the farm.
- Why slow: the builder 82797 got `SEMIO_HOST_CAPACITY_PLAN={"jobs":1,"cpu":2,"memoryMegabytes":0,"fixedMegabytes":5120,"floorMegabytes":6553}` ("admitted run with 1 of 8 job(s): the host cannot carry more right now"), so the graph farm has one job, and its parent heap (3.7 GB retained, 6-8 GB RSS) is above the 3,072 MB governor budget -> `graph: waiting for memory (3751 of 3072 MB in use, 1 projects running)` = serial inference. 32 GB host, load 15.
- Two leaked fleet shells burn ~5.5 cores and are dead work: pid 39538 (+39537 `wc -l`, wrapper 39535/39536) = `find -L s0/*/a -type f | wc -l` over `.cache/b/p/m/bazel/workspaces` since 01:43 (483 % CPU, 10 h); pid 78489 (wrapper 78486) = python `glob.glob("♻️mit-bestand/**/📋️project.json", recursive=True)` since 02:31 (95 % CPU). Not mine, not killed.
- Records persist per project, so restarting the builder loses little. Suggested: (1) kill 39538/39537/78489; (2) quiet the fleet (fewer 8 GB repo commands) so the builder's capacity grant grows past 1 job; (3) optionally restart the builder with `SEMIO_GRAPH_MEMORY_MB=8192` (explicit value is honoured, the default is capped at 3,072) so the parent's retained heap does not force serial mode; (4) then relaunch ONE `show targets`/dry-run so every queued command reuses the sealed generation.

## Next steps once the generation is sealed (nothing of this was reached)
1. `sh slice-play2-tools/retry.sh 5 <log-prefix> nice -n 10 bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype --dry-run` via `python3 chain-tools/detach.py /dev/null ...`; check slice size/RSS.
2. Real `activate-prototype` (35 lanes, wasm mutex notes in memory), plugin crate errors in A-L are apps-a2's (`result-apps-a2.md`).
3. `bun repo run @semio-tech/semio-tech-play:serve --exclude-dependencies` from the root (detached, coordinator-owned), headless boot check (chromium-1248 present at `node_modules/.cache/ms-playwright`), then `sh e2e-orch-tools/run.sh play --go --run-id e2e-1022-play1`.

No process of mine is running now.


## Raw log (chronological, earlier notes)
- 09:21 dry1 launched pid 67837 log 🗑️generated/slice-play2/dry1.log
- 09:23 dry1 died: Stale loaded Source revision reader (peer edit); dry2 pid 71682
- 09:30 dry2 died 'Authorizer DATA directory changed' (peer edit). dry3 via slice-play2-tools/retry.sh pid 72044 (log 🗑️generated/slice-play2/dry3.N.log)
- 09:27 dry3 died: 'Uncaptured implementation module refused: /private/tmp/i6/ovlaws/…/continuation/🟦️.ts' (peer clone leak?). dry4 pid 74699
- 09:32 ovlaws error = integrate3's live transfer window (transient). dry5 pid 77958
- 09:42 dry5 died: frozen lockfile vs package.json 09:23 (integrate3 burst, also blocks course-rest/bzl-tail). No peer was running lock; started 'bun repo dependencies lock --scope .' pid 81358 log lock1.log
- 09:45 lock done (bun.lock 09:41). dry6 pid 83579
- 09:56 peer (integrate3) ran lock --all; dry7 pid 6183
- 10:24 dry7 attempts 1-2 died on 'backends computed import' (peer declared it in ComputedImportSites 10:18); attempt 3 running (waits on dependency build held by integrate3 lock-sync sync1 pid 8720)
- 10:30 dry7.3 died: computed require in dependencies/🔌️managers/🟨️.cjs (pm-seam). LIVE EDIT: authorizers/🧬️schema/🔣️.json ComputedImportSites += owner managers/🟨️.cjs count 1, roots = 6 manager 🔣️.json (sorted between 📜️script.ts and 🔌️nx-plugin). Law not run.
- 10:34 correction: table row was not the mechanism (reverted). LIVE EDIT: dependencies/🔌️managers/🟨️.cjs now has a literal descriptors map (6 literal requires) instead of require('./'+folder+...). Node check: allManagersV1()=6.
- 10:58 dry9 died: 'Uncaptured implementation require refused: typescript (required by 📦️parsers/package.json)' — parserRequireV1 used createRequire(package.json) + computed own(name): not derivable. LIVE EDITS: build/🚀️bootstrap/📦️parsers/🟨️.mjs now exports literal parserTypescriptV1() (createRequire(import.meta.url)('typescript'); no more fallback, node resolution walks up to root install); 6 consumers (caching/📥️inference, graph/📥️inference/{bun-lock,owner-discovery,component-targets,project-assembly,script-inputs}) + bootstrap closure test updated. Not run.
- 11:03 dry10 got into graph refresh ('refreshing 2842 projects'), died: javascriptManagerV1.primaryLockName is not a function (cjs accessor gives a string; 11 .mjs sites called it). LIVE EDIT: dropped '()' at 11 sites in graph/📥️inference/{♻️revision,🕸️dependency-edges,🧭️script-inputs,📦️project-nodes}/🟨️.mjs.
- 11:40 dry11 (retry.sh pid 59227, logs dry11.N.log) waits at position 2 on repository-graph-facts shard 7 behind show projects 78521 (82797, 8 GB, refreshing 2842 stale projects after the round-6 transfer)
