# Coordination

Ticket created by hand: `ticket_open` over repo MCP returned a malformed result (structuredContent null) twice on 2026-10-18.

## Fleet Log

### 01:20 Repo CLI Boot Unblocked
- `bun repo --help` failed: `.cache/b/t/.publication` and `.cache/b/d/.publication` held pending intents from killed Codex runs (Tools/Dependencies, 00:39 and 00:58; no live process). Backed up to `stale-publication-intents/`, removed.
- Then it refused the `d` and `t` receipts (`image.json`), which predate the `mountDirectory` schema field. Rebuilt them from the covered directory inode (readdir `d_ino` of `m`; verified this method against l/o/g/p, which already have the field). Originals backed up as `*-image.json.orig`.
- `cleanup/🎛️command/🟦️.ts` "Exact canonical persistent Native publication required" now names the directory and the failing fields.
### 01:28 Dependencies image switched to Case-sensitive APFS; old HFS image moved to .cache/b-d-hfs-retired (delete after green sync)

### 01:45 Build CLI Fixes
- Lock-order bug: `🏗️build/📜️script.ts` pinned the dependencies partition shared around the whole command, and dependency sync/bootstrap needs nonwaiting exclusive access to it, so every command refused. Now `dependencies` and `ensureBuildEnvironmentV1` run before the shared dependencies pin.
- Zero-touch: the CLI import closure (379 files) had 35 bare `@semio-tech/*` imports that resolve only via `node_modules`, and `node_modules` lives inside the dependencies image. Rewritten to relative paths with `relativize-bare-imports.ts` (resolves through package `exports`). `bun repo --help` now boots without `node_modules`.
- Image filesystem: `native-build/💾️storage/🧬️schema/🔣️.json` ImageProtocol.filesystem changed from "Case-sensitive HFS+" to "Case-sensitive APFS". HFS+ returns ENOTSUP for `renameatx_np(RENAME_SWAP)`, which file publication and binding use; verified with a direct probe. The dependencies image was recreated as APFS (receipt OK). Other partitions are still HFS and get recreated when they first need swap.
- Dependency sync fences on workspace ownership; concurrent split edits trip it ("Bun current ownership changed during verification"). Retry when edits are quiet.
- Splits: C7 (package 7) and F (package 5) are DONE statically. c5b had already installed packages 5 and 6 (RESULT-CENSUS5-B is stale).
- 02:00 Deleted stale real node_modules directories outside dist (installer requires missing-or-symlink). Workspace fence now compares rederived membership instead of replaying every directory observation (🗂️workspaces/🟦️bun/🟨️.cjs).

### 02:10 Wave 2 (user: saturate usage)
- Census-5 splits DONE statically: F(5), G(6), C7, C8 (pai refused → `pai` executor), C9, D(10–12), E(13, 14-partial, proctor part B). Compile lists are in each F/RESULT-CENSUS5-*.md.
- Running: deps (critical path: sync green), quizdock, demo, play, pai, cg1 (process3d-host/job/brep-boolean), cg2 (flow-flow, bim-model), bd12, led2 (O-05/O-07/O-01/O-10), c5h (architect diff generator), wgpu; Haiku audits: split verification, Nx coupling, Bazel targets, census 6.

### 02:35 Wave 3
- quizdock DONE (static): `.cargo` kept out of the Docker context; builder config `🚀️deploy/Dockerfile.cargo.toml`; dev stack builds the proctor through `bun repo cargo`. Docker daemon is not running (socket missing).
- play DONE (static): describe `.sh` scripts removed, the route is now `<pkg>:component`; tool-neutral wording.
- Split audit: lossless, but 77 glob `pub use child::*` were added → `glob` executor removes them.
- Census 6: 295 files over 1,000 lines. Go wave launched: gocomp (46,930-line cli component), gocomptest (23,637-line tests), gocli, gogql, gotix, gotree, gocoord.
- The ticket `🗑️generated` folder was deleted externally (sweep); recreated. Cargo probe running (geometry-primitives).

### 02:50
- gotree DONE: tree/test-runner/hooks/languages split; build+vet+test identical; 3 tree tests were already failing (missing g1 fixture).
- Launched c6plug, c6ts, c6repo, nxneutral. Fleet at the cap of 20. QUEUED: gofix (pre-existing Go tree test failures, then every repo Go package test).

### 03:15
- gogql DONE (graphql and model split, tests identical; applyAutofixes and devcontainer drift handed to gofix). led2 DONE (O-05/O-07/O-01/O-10 closed).
- Cargo probe: queued 36 min behind the dependencies lease, then failed in the rustup archive acquisition (`🛠️tools/🦀️rustup/📥️installation/🟦️.ts:44`): "Original process admission closed". Likely the wait used up the budget. Retry after deps. The tools image `t` is still HFS+ and may hit ENOTSUP on swap.

### 03:30
- DONE: gocli (cli/move split, 7 failures that predate the split → gofix), pai (8 inherent role impls; the crate test closure has 3,253 B of headroom), demo (335 launch routes → framework-os-dev dev-<variant>-<renderer>-prototype; ship→release; demonstrator prepare deps; runtime paths from activation API).
- Launched c6fin, c6fin2, c6db1 (PEER areas now owned by the coordinator).
- Remaining census-6 PEER queue: P27–P35 (os db/flow/infinite), P13–P16 (framework 3d/actor/job), P18–P22 (framework ui rust), P38–P42 (os store), P47/P48/P54/P55/P58 (fem/flow/puzzle/wfc), P60/P62–P65/P70 (TS actor/ui/store).

### 04:00
- cg1 DONE (job carved to 0.857; process3d-host/brep-boolean were already under; 179 Cargo.lock need an offline refresh). gofix DONE (10 Go tests fixed, applyAutofixes decomposed, devcontainer drift). gocomp DONE (46,930 lines → 134 member dirs; canonical).
- Go layout ruling: one component per registered member dir with a single 🐹️.go (taxonomy gate admits only 🐹️.go) → gocanon converts the ~250 sibling files. gofix2: dedupe the cli autofix/graphql-schema copies, buildSchema, 58 cli test failures that predate the split.
- Index has staged `zz-gocomp-stage/*.go` (AD) from gocomp; no git-modifying commands were run by the coordinator. User/auto-commit to resolve.

### 05:00
- DONE (static): glob (89 globs removed), c6db1, c6db2, c6flow, c6fw, c6plug (+ older defects → plugfix), c6ts (+ PDF schema as JSON import), nxneutral (root lookup neutral; AGENTS.md line 7 conflict reported to the user), wgpu (+ wgpu2 follow-up), gocomptest, compile-plan (784 crates / 34 workspaces / 119 batches).
- Build CLI: `cargo` now runs before the dependency bootstrap and pin (no node_modules needed).
- New critical path: `cc` (cargo launcher + compile plan) runs in parallel with `deps`. Also running: catfix (taxonomy catalog load error), hdr3 (crate headroom), gocanon, gofix2, c6* splits, audit of other products.

### 05:30
- CARGO LAUNCHER GREEN (cc): geometry-primitives checks clean; rustup acquire-cache ENOENT closed the invocation (fixed); legacy rust image removed; test-support manifest typo; 8 carved crates moved to the root workspace; framework lock refreshed online (offline lacks `zip`). Batches running.
- DONE: c6store (27 files → 146 children), c6plug2 (16 → 106; brep register → fnsplit), c6repo (P73–P78, library root barrel removed, 474 importers re-pointed), wgpu2, other-products audit (os-dev broken → osdev executor).
- Usage 69% at 1h33m before reset → throttling: no refills until the fleet is ≤12.

### 05:50
- DONE: osdev (refs resolve; most audit alarms were inferred targets), fnsplit (tjc/srs/brep register decomposed, trace-verified), c6ui (31 files → 202 dirs; semio-framework-ui all-features closure 56,932 lines vs a 12,000 gate — carve TODO), gofix2 (client duplicate graphql/autofix stack deleted; short suite 74 → 3 env failures; 7 Go defects fixed).
- INCIDENT: gofix2 non-short Go tests rewrote about 3,450 ticket and 204 goal JSON files in the real checkout; restored from HEAD content (no git state change). Uncommitted ticket edits by others could have been reset; my ticket and the Bazel ticket were verified intact. The offending tests now use a throwaway repo.

### 06:10
- Usage 82% at 53 min before reset: told cg2, c6fin, c6inf, c6tsfw, hdr3 to wrap up.
- c6tsfw DONE (16/18; ShardClient 1,593 and DocumentBrowserActorReservation 1,133 need a design; the taxonomy wgpu-frame-worker inputPatterns are missing the new children). plugfix DONE (norm app_surface imports in 14 crates; restored the layout args_bridge lost in a carve).
- AFTER-RESET QUEUE: resume cg2/c6fin/c6inf/hdr3 from their reports; class splits (ShardClient, DocumentBrowserActorReservation); semio-framework-ui all-features carve; wgpu-frame-worker taxonomy; vdi3805 half-carved root; run all TS suites once node_modules is installed.

### 11:40 (new usage window)
- node_modules RESOLVES (generation 441fd20…, published 07:16). deps decisions: installed-path cap 1024 OK, parent identity OK, resume support YES.
- cc: framework batches 1–2 and root batch 1 green; semio-framework-ui all-features refused by the gate, blocking 257/549 root crates → uicarve (TOP).
- Build CLI: ensureBuildEnvironmentV1 read-only inspect now uses a SHARED dependencies store (graph commands no longer refuse while cargo holds the image); removed my [DEBUG] in store/🟦️.ts.
- Launched: uicarve, c6inf, c6fin, cg3 (bim unmirror), cls (ShardClient/Reservation), lex (layout-export), tst1 (course + library TS suites), tst2 (plugin/os/ui TS suites); bzl resumed (runtime phase).

### 13:00
- MANIFEST FREEZE: only deps may edit package.json and bun.lock until one green sync (any manifest change forces a ~20 min reinstall in every bun repo command).
- DONE: gocanon (332 Go files → member dirs, canonical overlay for script.ts/test host), lex (layout export tests ported, gate 0.996 → 0.676), c6fin (11 → 62), c6inf (13 → 151), cls (ShardClient/Reservation redesigned; actor-transport 179/179), vtax (validateTaxonomy → 20 validators; 5,803-variant differential), bzl partial (tools setup bazel OK, readiness 18/18, closure cap 512 → 2048, 3 graph-load import fixes; dry-runs blocked by reinstall).
- Graph load >1.6 h → gperf (critical).

### 14:05
- uicarve DONE: semio-framework-ui → 2 files + 32 tui/wgpu crates (≤ 0.892); cc resumed. cc so far: framework 34–35, root 63–67/69/71/73/75, teaching 147–148 green (~60 fixes); tco on test-companion privacy (batches 68/76).
- tst1 DONE: quiz 254/254 vitest, quiz tsc 0; repo-lib tsc 180 → 5; checkQuizCatalog via the launcher. tst2 DONE: bun suites 1,064 → 1,261/1,465 green; 151 stale source-text laws → law1/law2.
- `bun repo run` still blocked by nonwaiting maintenance (nx cache pool) → lq; graph load perf → gperf; deps: single manifest batch + one green sync.

### 15:00
- lq DONE: nonwaiting maintenance on ordinary commands → queued (30 min bound, cancel, progress). own DONE: attach `-owners on` (nested attach works); Nx pool still failed as an unfunded nested child → DECISION: Nx cache becomes a top-level partition (nxpart).
- gperf DONE: inference records brotli plus shared observations; cold 4h+ (extrapolated) → 7–9 min; warm 70 s / 1–9 s; tail blocked by concurrent source edits (stale Store implementation) plus storage errors.
- cg3 DONE (bim unmirror applied; generator 18 GB blowup fixed). lfix/lfix2 DONE (fixture API port for 35 plugins plus dev; puzzle BrowserCaller). law1 DONE (34 suites green; framework→repo exemptions reverted per ruling → inv executor).
- NEW compile blocker: close_step API (213 plugin files grant-based, 75 Refused) → cstep.
- Running: deps, cc, tco, law2, nxpart, inv, cstep.

### 15:30
- law2 DONE (40/63 green; repo scope 408/664 → repot executor). cstep DONE (159 files; 73 crates queued for cc; the wfc headless driver still uses the removed worker-session API = open design). inv DONE (framework→product imports 196 → 0 via a provider loader and ports; 2 laws need manifest edits → deps batch).
- FOLLOW-UP: build-graph inputs of the 15 cache:true framework targets no longer include product provider code (stale-cache risk) → declare provider modules as inputs.
- Weekly usage 54%: fleet held at ~7.

### 16:45 (user: saturate)
- nxpart DONE: Nx cache is the top-level partition `.cache/b/n`; namespaces are funded children; failed births detach and clean up.
- My sync probe was terminated by deps starting `dependencies lock --all` (manifest batch in progress).
- Launched 12: layering audit, census-7 audit, hdr4 (remaining near-gate designs), wfcj (wfc headless + job retained work), law3, law4, vit (vitest configs), pin (provider inputs in the build graph), tsc + tscp (typecheck sweeps), gofin (Go leftovers + exhaustive), sweep (external cache deletion root cause).

### 18:30
- DEPS GREEN at 18:21 (generation 4151d2262cb4a4e7a25a; 309 links; 124 undeclared deps added; warm sync ~16 min). Stale again at 19:01 (law4 exports edits) → STRICT FREEZE sent to all executors (package.json/bun.lock plus storage/revision modules). Coordinator runs lock → sync → show projects → tools setup nx (scratchpad seq.sh).
- sweep DONE: clean protects native images and open-ticket 🗑️generated; the 02:08 image deletion cause is unproven (needs root fs_usage).
- CLOSE-OUT TODO: delete `orig/` source snapshots and other copied sources from ticket tool folders (494 paths > Windows limit; AGENTS forbids source copies in tickets) after compile verification; delete all 🗑️generated folders; remove scratch node_modules copies.

### 19:30
- wfcj DONE (HeadlessCaller, max_step_micros budget restored, retained-work gate; default work_demands is now minimal). tscp DONE (11 fixes; batch-2 packages → deps). launch audit: 462 routes name projects not in manifests (likely graph-inferred -rs), 49 undeclared targets, seed and launch drifted → QUEUED: reconcile against the real graph after show projects.
- deps resumed: batch 2 manifests → lock (lease-wait fix) → sync → show projects → tools setup nx.

### 20:30
- c7w DONE (census 7: 37 Rust + 4 TS split; jcoprobe bundles marked @generated) → FILE-LEVEL UNTANGLING COMPLETE (all hand-authored files < 1,000 lines except generated).
- hdr4 DONE (7/8 crates ≤ 0.90; instance-app 0.950 accepted as ceiling pending a redesign); hdr4b removed its compat re-exports. tsc DONE (core configs 8 → 13/23 at zero errors). repot DONE (repo tests 2,284 → 2,488 pass; child-envelope defect fixed). wfcj2 DONE. lay0 DONE (violations 11,025 → 9,696).
- deps: batch 2 synced, final sync running. cc: framework/root/teaching/ui-carve/cstep-framework green; batch 331 running.
- Running: deps, cc, tco, law3, vit, gofin, repot2, tjcov, lay1, lay6, lay12.

### 21:30
- vit DONE (3 → 7/27 plain green, 11 with harness; Play maxRows 128 → 512 fixed). gofin DONE (+ real fix mutation, bundle node parity; Go short suites all green). lay6 DONE (OS cycles 129 → 2; 85 exports spec → deps batch 3). tco DONE (companions; 2 crates to confirm → cc). law3 DONE (21/25; Locale Default ruling → loc).
- RULING: no Default for Locale/Terminology (no default language). Playwright chromium 1248 vs 1234 → deps checks whether tools setup owns the install.
- NOTE for user: a peer staged deletions of the publication-owner and receipt-conflict laws (unknown origin); `zz-gocomp-stage` files are staged in the index.
- Weekly 71%: stop launching at ~92%.

### 22:00 (user: fully saturate, may hit the limit)
- repot2 DONE (17 more suites green; router classifier fix; isolated-process laws). NOTE for user: 37 generated files and ~370 stale ticket copies are tracked in git (need `git rm --cached`, forbidden for agents).
- Launched 12: lw1, lw2, lw4, lw5, lw7, lw8, lw10a, lw10b (layering WPs; exports specs → deps batch), spr, pfx, law5, osplay. Fleet ≈ 20.
- 22:45 Relativized 13 bare imports that re-entered the CLI closure (leases retirement, graph inference, build-input ledger) after the lw10a/inference work.
- 23:10 Root 📜️script.ts had 21 static imports of self-test modules moved by lay7 (breaks the repo MCP server, clean, etc.); re-pointed and verified with bun build (0 unresolved). LESSON: movers must grep the root 📜️script.ts too.
- seed DONE: seed authored, launch.json generated plus a drift check; 1,122 dead entries removed (fold-log). Rule added to brief-executor.md.

### 23:30
- LAYERING TOTAL 11,025 → 5,469 (lay1: plugin cycles 92 → 0, no-core-path 0; lw1: plugins→framework 1,140 → 32; lw10b: repo 2,428 → 1,101; lay9: renderer cycles 203 → 0; lay6: os cycles 129 → 2; lw4 hub 713 → 563; lw5; lw7; lw8). The remainder is mostly exports → repoapi consolidated spec → deps batch 3.
- osplay DONE (os dev 29 → 5 failing; Play green except http-cache-semantics; catalog audit ~4× faster).
- Running: deps, cc, tjcov, lay12, lw2, repoapi, tsc2, gofin5, lw1b, scc.

### 00:30 (10/19)
- DONE: docs, memory notes refreshed, tclean (6.2 GB freed; long paths 416 → 0), tsc2, gofin4/5 (Go fully hermetic, all modules green), seed (launch.json generated), lw2 (333 → 61), scc (repo cycles 6 → 0), lay1.
- Running: deps, cc, tjcov, lay12, repoapi, lw1b, qe2e, lw3, census-bun, gen, rmod, vcfg, wpath, gofin6, lw2b.
