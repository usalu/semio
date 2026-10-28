# Handover 2026-10-27 (coordinator session 1027a, second half) → next session

Ticket: `2026/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN` (C = this folder). Do NOT close it. The repo MCP failed to connect all day (no ticket tool used). Rules: `brief-wave-1026g.md` + `brief-wave-1027b.md` (lane brief incl. the 11:50 file-mode addendum). Gate log with every verdict and revert command: `coordination.md` (10/27 entries). Product logs: `gate-g-tools/run/g23…g37-*.log|json` (product.py: wall rounded up to 5 s steps, peak tree RSS, exit).

## Targets (brief-wave-1026g) — measured on this 32 GB / 10-core Mac with product.py
| Target | 10/26 | Now (run) | Status |
|---|---|---|---|
| Course hot no-op < 10 s, < 1.5 GB | 60–83 s | 10.0–10.1 s, 0.9–1.3 GB (g34–g36: graph served from the memo + Bazel plan reused) | at target (product.py rounds to 5 s steps) |
| Course one-line edit < 60 s | 506 s | 25.2 s edit, 35.2 s revert, both `plan reused` (g34c) | MET |
| Rust (print) one-line edit < 2 min | 2,637 s | 105–106 s (g33p, g36q), revert 20–30 s | MET (first edit after a landing 161 s) |
| print-rs:test hot no-op | 420–560 s | 20–25 s (plan reused; memo reseal landing in g37) | improving |
| Dashboard build no-op | 140–240 s | 140–206 s | NOT met (lane dashhot) |
| Peer edits: 0 re-executions | never measured | library landings still re-infer 7–93 owners (peeredit, peeredit2) | NOT met; harnesses being readied (harness3) |
| Graph after a typical peer edit < 2 min, builder < 2 GB | 307 s | 1 project refresh ≈ 30–40 s; builder peak 13–16 GB on 90-owner refreshes | NOT met (lanes buildmem, peeredit2) |
| 3 agents on 16 GB | never measured | not run yet (harness3 readying agents3-g) | open |
| Disk budget + lock-aware GC | — | lane diskbud; backlog C1 queued | open |

## Landed today (all product-verified; revert commands in coordination.md)
G23 graph-memo · memokey · memokey2 · memonotice · factsound · audit1 verified · carry · reseal (G37, verifying) — envleak · actenv v3 (LEAKS: none) — dashplan · leaseord 1-3 · cargofix leaseorder-c · cargo2 testorder — universe · ts-closure · graphlat reuse-once + plan-once2 · keys4 · rustfast · testmemo · cargofix synchronize · cargo2 incremental — clonejournal · oncecapture · rowid · jdir · memolist · xplat5 journal/witnessenv/winframe — refloop · planfp 1-3 · gitrow — quietfc round 2 + 3 + audit2 quietfc-mutual · rustcode-writer — docs4 (9 batches) — backlog group A1 (18 ids) — walk-views — course readiness fixture (preflight policy) — fix-modes (213 files 0600 → index mode).

## Lessons (also in memory)
- Staged whole-file copies built on a mirror can silently DROP live changes made since (winframe would have removed audit1's cursorAtMs): before applying any whole-file entry, diff base→new and check the removals are intended; rebase with `gate-g-tools/rebase_entry.py`.
- Verify scripts that re-check "applied" state fail after a later batch touches the same file: run their law lines directly.
- `SEMIO_TEST_ARTIFACT_DIR`: some suites need it under the ticket 🗑️generated (not /tmp, not repo .tmp), others fail WITH it (envelope tests); compare failure sets with the same environment.
- zsh does not word-split `$VAR` lists: use bash arrays in loop scripts.

## Open / next (in order)
1. g37 verdict (reseal) → print hot from the memo.
2. Land as they report: hotpath2 follow-up (memo vs git-ignored .🧾️store), dashhot, buildmem, peeredit2, capgrow, audit3 findings, cargo2 follow-up, diskbud, backlog2 queue (B1, C1, D1, D2, D3, E1, F1), sealflip (sealed mode last).
3. Run the acceptance scenarios from harness3 (peer edits, 3 agents on 16 GB), course e2e (`e2e-f-tools/final.sh`), `repo-tickets-rs:test`, `repo-graphql-go:test`.
4. Reboot proof of the journal/tools-image fix (needs a reboot — user's call).

## Open user decisions (unchanged)
Delete `🧰️framework/🤖️generated/process` (193 MB); delete stray module-dir `.cache` roots (16.2 GiB, `result-disk-g.md`); fixture tickets written by Go tests; three tracked ticket files exceed the 256-char path limit (`xplat5-tools/long-paths.py`).

## UPDATE 16:20 (after G37–G44)
Targets now (product.py, quiet-ish host unless noted): course hot 10.0 s / 0.6–1.3 GB; print-rs:test hot 10.0 s served from the memo (was 420–560 s); dashboard no-op 10.0 s / 1.15 GB (was 140–357 s); course one-line edit 25–50 s (memo serves via `derived`, g43); print one-line edit 50.5 s (g43q, memo-served, under load 35) — all four MET. Builder peak on a 90-owner refresh 9.6–10.3 GB (childheap), still > 2 GB target; farm workers still refused ('identity differs: records' → farmid).
Landed G37–G44: reseal, capgrow + grownow, B1 (+re-cut red-leasesentry), rev3fix, walk-footprint, seal2, sealflip batch (SEMIO_REPO_SEAL; fixes dashboard 'SEMIO_REPO_IMPLEMENTATION=rust' exit 2), diskbud, dashhot fresh + nestenv, childheap, C1, gatelock, agedlight, memoedit, keyparts, D1, releasefirst, profilefresh, derived, memoout, selnear, witdelta, planmemo, nxmeta, fix-modes ×3, audit6 ×3, farmid captured, peeredit2 providers + tools (forward fix of string-input consumers pending in lane peeredit2), linuxdc xdgroot/tmpdirep, D3, F1, docs4 round 2.
Not landed / pending: the sealed FLIP itself (ready; sequence in result-sealflip.md 14:35), E1 (Nx), G45 queue (scopedcut, td-ucache, td-misc ×5, td-leases ×2, td-capacity ×5, agents16 ×3, td-products ×3, audit7 ×2, compliance ×8 last), peeredit2 consumers, farmid records fix, jloss, gofix, runner findings.
Tools: `gate-g-tools/rebase_entry.py` (3-way rebase of a staged whole-file entry), `gate-g-tools/scan.ts` (runtime-equivalent parse check), `gate-g-tools/verdict.py` (gatefast recipe; not yet used), `prodmap-tools/group.py`.
Caveat: farmid's staged dir was reset to pre-rebase copies by a failed revert script (coordination 15:4x) — rebuild before reverting farmid.

## UPDATE 18:40 (G45 verifying)
- G45 LIVE: td-ucache/misc/leases/capacity/products/graph/observe/bazel test-debt batches, audit7 contracts+reclaim, agents16 farmfree, audit8 copies (windows, psgate, consumers), gofix1 (MCP server had derived no tools since F1), scopedcut, gatefast orchslice (18:3x: FIXES the dashboard build, broken on live since G44 `providers` — nested compile needed 31 projects outside the pinned outer slice).
- g45: course 904 s ok; course:test cut by its 300 s wall budget at load 40–70 (fix staged: cbudget `clock`); dash1 RED (pin, fixed by orchslice → g45b-dash proves); print1 2441 s ok.
- G46 queue (all pre-audited by audit9/audit10 unless noted): farmid authority, td-graph demand, agents16 allrun→farmfree(law)→behind, builderlock multiwriter (with audit9 fix), quiet16 hold→lastrun (re-cut, width gate), jloss (USE C/audit9-tools/staged/jloss; drop peerrun watchpipe – same fix) + journal restart, edit30 freshmtime, playdemo vitestext/wasmlock, goyaml (own group), docs5, cbudget clock (+shard later), rusttests envelope, pinpub nxsparse + pinned (after demand), optcache store, dashhot split, progress umlaut/lang/status/rows, windows winlaunch/fleetpy/wininstall, cicover ci1 + ciwf (.github: user commits/pushes), td-misc leftovers. G47: compliance ×8 (rebuild_all.sh first) → nolegacy lease/reapi/nobaseline/lang → schemafirst → sealed flip (result-sealflip.md 14:35) → E1 Nx.
- Acceptance: peer-edit chain ran (result-peerrun.md): all 12 runs exit 0, edits restored, 0 re-executions in 3 of 4 peer stages; stage 02 had 7 (foreign churn, rerun 0). 3-agent real run still owed (after agents16+quiet16 land).
- USER DECISIONS: commit/push the `.github` changes (ciwf fix in the index + cicover ci1); the fixture tickets in the index (td-products); delete 🤖️generated/process (193 MB), stray .cache roots (16.2 GiB), the 100 GiB `.🧬semio/🌐hub` scratch (diskbud); rust image budget (diskbud).

## UPDATE 21:10 (G46 verdict, G47 landed, verifying)
- g46: course 1992 s ok; course:test RED from multiwriter (reverted) → g46b-ctest 125 s ok; dash RED (same) → dash2 354 s ok; hot1 908 s / hot2 38.6 s ok; repo-mcp:test 152 s ok; course edit 50.7 s / revert 43.4 s; print edit 283 s / revert 129 s; print exit 143 = product.py's 45-min running-time kill (128 owners re-inferring). Root cause of the long re-inference: graph inference cache full since 10/26 18:56 (two sparse test files, 878 MB) → cachegc relief applied 20:49; permanent fix cachegc quota landed in G47.
- G47 LIVE (21:01): cachegc quota, e2e claimsctl, mem2 factview, audit12 sigexit/farmexports/typedecl/saytest/privparent, obsopen nestedfix/lostbirth, farmid declined, e2e petsrace/layoutfresh/runfix. Verdict chain: `🗑️generated/coord/g47-chain.sh` → `gate-g-tools/run/p47.out` (course, course:test, dash, print, hot ×2, repo-mcp, course + print edit probes, dash2).
- Held for G48: nxdeadline release (store entry rebased cleanly, all base; apply after the g47 verdict), quiet16 hold (rebased, all base) → lastrun (fixture conflict, lane lrrebase), builderlock multiwriter re-cut (2 claims/generations .mjs conflicts vs claimsctl, lane mwrebase), then audit14's order for compliance/nolegacy/schemafirst/edit30/langpick/benchci/cbudget shard/goyaml/isolate, incfacts LAST.
- Model rule from the user (10/27): coordinator = main chat; Sonnet High lanes execute; Haiku High audits/explores; never Opus workers; no spawn bursts.

## UPDATE 10/28 12:50 (G47–G50d)
- G47–G50c landed and GREEN (verdicts in coordination.md). g50c: course hot 7.2 s / 0.9 GB, dashboard no-op 6.9–9.3 s with 0 actions (dashnoop), course edit 40.5 s, print Rust edit 108.7 s — T1/T2/T3 met.
- USER DECISION 10/28: `repo run repo-mcp:install` owns ~/.local/bin/repo with the native binary. Done: the 10-26 symlink to the launcher-only binary (the reason `.mcp.json` `repo mcp` failed to connect since 10-26) was replaced atomically, then installed via repo-mcp:install twice (before and after the langpick launcher batch); receipt ~/.local/share/semio/repo/installed.json, `repo mcp` = 55 tools. Root fix (bootstrap installer must never create/advise a second PATH repo; chain into the native install) → lane installown.
- Gate fix 10/28: bootstrap ⌨️cli schema maximumOwnerBytes 262144 → 65536 (above the revision text ceiling every recovery route died at entry). ⚡️caching reset/maintenance routes still refused (owner manifest 105 KB) → lane ownerbytes.
- G50d part 1 LIVE: langpick core·copies·imports·dashboard·routing·launcher·launcher2; productparse (product.py). Verify + chain g50d running.
- Next: g50d verdict → lintglob globproj → bazelglob → buildscript (rebuild.py first) → sealed flip (C/sealprep-tools/flip.sh, then self-g post-flip, promote.sh rule) → G51 incfacts; land installown, ownerbytes, handshake, isov, osreds, hubreds, mem3 as they report.
