# audit-claims3-1026g: claims of the lanes finished since 18:45

Auditor aud-claims3-g, 2026-10-26 about 20:00 (local). Read-only: no source edit, no batch apply, no `bun repo` run, no git-modifying command, no sub-agent. Ticket not closed.

Evidence used: logs and JSON records under `🗑️generated/` and `🗑️generated/rebase-g/`, staged manifests and `bases.json`/`bases.tsv`/`base-hashes.json`, sha256 of live files against the recorded bases, `wc -c`, `git ls-tree -l HEAD` (read-only), file birth and mtime, and in-memory replays (gram2 index-chunks edits applied to the live file, rev3fix anchors replayed in order). Scratch scripts sit in the session scratchpad, outside the ticket.

Verdict key: CONFIRMED = disk evidence matches. UNSUPPORTED = no evidence on disk (not proof of falsehood). CONTRADICTED = disk evidence says otherwise, or the current file supersedes the claim.

Totals: 70 claims, 50 CONFIRMED, 13 UNSUPPORTED, 7 CONTRADICTED.

## Claims

| ID | Lane | Claim (short) | Evidence checked | Verdict |
|---|---|---|---|---|
| R1 | rev3fix-g | 51 anchored edits, 9 new law files | edits.json: 51 edits, 9 newFiles, 9 files in staged/files | CONFIRMED |
| R2 | rev3fix-g | edits touch "15 files" | edits.json: 18 distinct files; pre-rebase copy also 18 | CONTRADICTED |
| R3 | rev3fix-g | staged bases = live, nothing live; anchors unique | bases.json 18/18 sha256 = live; 51/51 anchors unique on in-order replay | CONFIRMED |
| R4 | rev3fix-g | patch-drift 2/0, fact-index 4/0, generation-protocol 47/0, prefetch 2/0, boot-anchor 3/0 and others green | rebase-g/rev3fix-verify-last.txt (19:07) shows the same counts | CONFIRMED |
| R5 | rev3fix-g | planner laws excluded-directory 9/0 and plan-reuse 4/0, "run from the repo root" | same log, line "planner skip (repo root cwd)": 0 pass 9 fail, 0 pass 4 fail (log predates the 19:16 rebase; no later run on disk) | CONTRADICTED |
| R6 | rev3fix-g | host-capacity gate 23/0 on reruns | only run on disk: gate 22 pass 1 fail (19:07); reruns not on disk | UNSUPPORTED |
| R7 | rev3fix-g | the fixed laws were red before the fix (patch-drift at generation 64; 5 of 9 excluded-directory red; 4 of 5 dead-holder) | no pre-fix run output on disk | UNSUPPORTED |
| N1 | nxlock-g | 12 staged files; policy and order test built on core bytes | manifest 12 entries; live policy.json and order test = core base (core not applied) | CONFIRMED |
| N2 | nxlock-g | order tests 73/0, component 21/0, startup 18/0, put-slots 14/0 | no log on disk | UNSUPPORTED |
| S1 | setup-g | 23 hand-written toolchain scenarios | runtime toolchain 🔣️.json: 23 scenarios | CONFIRMED |
| S2 | setup-g | staged "12 edited library files + 4 new" | index-edited.json 13 keys, staged base 13 files, overlay 17; the 13th (zero-touch closure) was added by rebase-g at 19:52 | CONTRADICTED (stale figure) |
| S3 | setup-g | all staged bases equal live (nothing live) | 13/13 live sha256 = recorded base | CONFIRMED |
| S4 | setup-g | zero-touch 36/0, preflight 3, toolchain 4, matrix 4, remedies 4, tool-setup 5 | rebase-g/setup-verify.txt lines | CONFIRMED |
| S5 | setup-g | runtime toolchain 5/5, prepare 4/4 pass; verify "ready, exit 0" | 5 and 4 test definitions live; no pass log for these two suites; verify not re-run (it would execute the build helper) | UNSUPPORTED |
| O1 | oracle2-g | laws 24/24 green | oracle2-g/laws-all.log: 24 pass, 0 fail | CONFIRMED |
| O2 | oracle2-g | 14 staged files; staged base equals live | 14 staged files; staged base 14/14 = live | CONFIRMED |
| O3 | oracle2-g | plan sizes 12,246 / 18,965 / 26,058 declared files, 59 / 15 / 6 profiles | no plan output on disk | UNSUPPORTED |
| Q1 | quietfc-g | staged, not applied (new profile module, SEMIO_HOST_CAPACITY_CORES absent live) | profile/🟦️.ts absent live; no CORES or hostProfileV1 reference live | CONFIRMED |
| Q2 | quietfc-g | 220 decision worlds, 70 forecasts, 8 envelope and 12 declaration cases; follow-up test counts | oracle source has range(220) only; no run output on disk | UNSUPPORTED |
| X1 | xplat3-g | 19 staged files; bases.tsv equals live | 19 files; 19/19 sha256 = live | CONFIRMED |
| X2 | xplat3-g | witness 6, busy 8, install 7, dedupe 9, sweep 10, admission 17, native 11 plus 1 skip, addon 15, runtime 32/2 | rebase-g/xplat3-verify2.txt: same counts | CONFIRMED |
| B1 | obs-g | 26 staged files: 12 new, 14 changed | manifest.json: 26 entries, 12 base null, 14 with a base | CONFIRMED |
| B2 | obs-g | nothing live: fleet module absent, changed files at base | fleet-status directory absent live; 14/14 live = base; 12 new absent | CONFIRMED |
| B3 | obs-g | fleet 10/10, guard 4/4, door 7/7, operations 14/15 | no log on disk | UNSUPPORTED |
| C1 | conv-g | 66 staged files (65 edited, 1 new) | staged/files: 66; base-hashes: 1 absent entry | CONFIRMED |
| C2 | conv-g | no live leak: 58 at base, 9 read-oracle at the oracle2 prerequisite, 1 new | base-hashes.json has 68 keys: 58 live = base, 9 live = oracle2 staged base (not conv's), 1 absent. rebase-g says 66 with 7 prerequisites | CONFIRMED (count note: 68 keys vs 66) |
| C3 | conv-g | static verify: 24 laws | no static-mode output on disk (conv-verify*.txt holds mirror and official runs) | UNSUPPORTED |
| D1 | dashlock-g | 11 files: 6 modified, 5 new | manifest: 6 entries match live base, 5 absent live | CONFIRMED |
| D2 | dashlock-g | digest law 4 tests; installation law 1 test | staged f06 has 4 test definitions, f09 has 1 | CONFIRMED |
| D3 | dashlock-g | 41 expects passing; "before" red | no run output on disk | UNSUPPORTED |
| T1 | toolsetup-g | planner skip never hits: "cannot be reused later" printed, zero "plan reused" lines | 5 bench2-g logs contain "cannot be reused later"; grep for "plan reused" over generated/: none | CONFIRMED |
| T2 | toolsetup-g | hot no-op walls about 60 / 77 / 83 s; Bazel 0.4 to 0.5 s | bench2-g course noop s1-s3 JSON wallMs 59.9 / 76.8 / 83.4 s; Bazel "Elapsed 0.520s" in s1 log | CONFIRMED (walls; phase columns not re-derived) |
| T3 | toolsetup-g | probes batch: 4 files, live equals base | probes/bases.json 4/4 = live | CONFIRMED |
| T4 | toolsetup-g | probes-after-conv drifts in two witness files until conv-g lands | live witness .mjs and .d.mts equal conv-g's recorded base, not the twin's base | CONFIRMED |
| T5 | toolsetup-g | witness 8/8, plan-reuse oracle 4/4 | no log on disk | UNSUPPORTED |
| T6 | toolsetup-g | LEAK: plan-reuse oracle leaves change-journal services running (8 live) | 8 semio-repo-watch processes alive; their bun parents have ppid 1 (orphans); command lines point at reuse-* checkouts and temp .cache paths | CONFIRMED |
| J1 | jobsrv-g | 9 new and 5 edited files, all under unit-cache | staged/files: 14; base.sha256: 5 rows = live; unit-cache jobserver directory absent live | CONFIRMED |
| J2 | jobsrv-g | unit-cache suite 26/26; pool laws 4/4 green on staged tree | main-after.log: 26 pass 0 fail; pool-after2.log: 4 pass 0 fail (pool-after.log, earlier, was red) | CONFIRMED |
| J3 | jobsrv-g | base red; Cargo pipe 16 jobs at once vs grant 4; pipe 6 and 9 before, 3 after | pool-before.log: "16 jobs at once, plain Cargo 4"; matrix1.txt: pipe 6 and pipe-high 9 before, 3 after | CONFIRMED |
| P1 | xplat4-g | 220 suites: 147 Windows-capable, 73 POSIX-only | the 10 staged route-table JSON files: 220 suites with platforms, 147 win32, 73 without | CONFIRMED |
| P2 | xplat4-g | platforms law 8 tests and workflow law 6 tests, all green | 8 and 6 test definitions exist; no run log on disk | UNSUPPORTED (green) |
| P3 | xplat4-g | advisory removed from all rows (staged workflow) | staged build-system.yml: 0 "advisory"; live file still has 22 mentions (not applied) | CONFIRMED |
| P4 | xplat4-g | freespace and corpus stacked on xplat3-g, not live | 3 base mismatches; live copies differ from xplat3 staged copies, so no xplat3 leak | CONFIRMED |
| M1 | mirror-g | shared tool exists: mirror.py, README, selftest.py | files present (mirror.py 29561 B) | CONFIRMED |
| M2 | mirror-g | selftest 20/20; mirror runs obs-g 14/0 and quietfc 9/0 | no output on disk | UNSUPPORTED |
| RB1 | rebase-g | rev3fix rebased: bases 18/18, anchors 51/51 unique | bases 18/18 = live; in-order anchor replay 51/51 unique | CONFIRMED |
| RB2 | rebase-g | generations/🟨️.mjs is 62464 B live; gram2 index-chunks alone makes it 65887 B (cap 65536) | wc -c 62464; in-memory apply of the gram2 edits to the live file: 65887 | CONFIRMED |
| RB3 | rebase-g | route-unrouted re-anchored; caching project.json has no test-lease-bootstrap | 0 hits for test-lease-bootstrap, 1 hit for test-lease-queue | CONFIRMED |
| RB4 | rebase-g | route2 manifest now 54 keys (56 before); apply refuses unstaged keys | staged-base.json 54; pre-rebase copy 56 | CONFIRMED |
| RB5 | rebase-g | zero-touch 36/0 after the node-native-core closure entry | setup-verify.txt: 36 pass 0 fail | CONFIRMED |
| BE1 | bench2-g | walls 368.7 / 76.8 / 59.9 / 83.4 s; dashboard prime exit 1 after 909 s | course and dashboard JSON records (wallMs, exit) | CONFIRMED |
| BE2 | bench2-g | hot footprint 0.67 to 0.73 GiB; RSS 2.10 to 2.25 GiB | peakFootprintBytes and peakRssBytes, s1 to s3 | CONFIRMED |
| BE3 | bench2-g | every hot course run asks for 7168 MB ("sized at 200%") | s1 log: "sized at 200% of the default from 13 earlier run(s)"; s2 and s3 logs not checked | CONFIRMED |
| BE4 | bench2-g | dashboard build refused by LeaseOrderErrorV1 on dashboard-cli/prototype/60a7 | dashboard prime log: 6 matches incl. "Refusing to wait ... 60a7" | CONFIRMED |
| BE5 | bench2-g | host-total patch "STAGED, NOT applied" (16:25 section) | the 3 live files carry SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES; backup-host-total exists; the lane's own 18:40 section says applied at 18:02 | CONTRADICTED (see live-leak section) |
| CE1 | census2-g | 10 edited and 12 new files; base equal to live at staging | batch.json: 10 entries, 10/10 live = base; new-files.json: 12 | CONFIRMED |
| CE2 | census2-g | ceiling suite 7/7; controls 6 pass 1 fail each | no log on disk; ceiling module not live (correct) | UNSUPPORTED |
| CE3 | census2-g | census two finished, rows listed | result has INTERIM_ROWS, INTERIM_TABLE and INTERIM_RETENTION placeholders; last write 19:02 | CONTRADICTED |
| SW1 | audit-sweep | monolith-split in HEAD: 8,024 files, 43,460,490 B, none over 5 MiB | git ls-tree -l HEAD: 8024 files, 43460490 B, 0 over 5242880 B | CONFIRMED |
| SW2 | audit-sweep | earliest survivor born 17:45:22; nx 119 MB, last write 18:12 | e2e-orch birth 17:45:22; nx 119288 KB; newest file mtime 18:12:12 | CONFIRMED |
| SW3 | audit-sweep | chain14 and round2-apply absent | neither in generated/ | CONFIRMED |
| SW4 | audit-sweep | CLEAN ticket still open; ticket.json untouched since Oct 18 | ticket.json mtime Oct 18 04:16, "status": "open" | CONFIRMED |
| SW5 | audit-sweep | Go PurgeOversizedTicketArtifacts walks the whole ticket folder and removes subtrees above 10 MiB | finish-reopen/🐹️.go rewritten 19:29; no such function now; purge is scoped to the generated folder via a selection enum. Audit-time code not recoverable | CONTRADICTED (current tree) |
| SW6 | audit-sweep | Rust purge_oversized_artifacts (line 705) walks via collect_real_tree | lifecycle/🦀️.rs rewritten 19:27; neither name exists; current functions are purge_ticket_generated_output and purge_closed_ticket_generated_output | CONTRADICTED (current tree) |
| SW7 | audit-sweep | repo clean: cleanTicketSizeRemovals at candidate-discovery line 179 | line 179 exports cleanTicketSizeRemovals | CONFIRMED |
| A1 | audit-staged | rev3fix: 3 base hashes drifted (18:46) | pre-rebase bases: the same 3 files mismatch live now; rebase-g fixed it (18/18 now) | CONFIRMED (resolved since) |
| A2 | audit-staged | route2 manifest 56 keys with 2 CHANGED entries | pre-rebase staged-base: 56 keys; the CHANGED state was not re-derived | CONFIRMED (count only) |
| A3 | audit-staged | laws-g route-unrouted stale (anchor gone) | live caching project.json: 0 test-lease-bootstrap | CONFIRMED |
| A4 | audit-staged | nxfix core pending: 11 base, 2 new absent | core manifest: 11 live = base, 2 absent | CONFIRMED |
| A5 | audit-staged | verify scripts missing for obsrows, setup, xplat3, conv | all four created 19:23 to 19:33, after the 19:02 audit (now superseded) | CONFIRMED (at audit time) |

## Live-leak check (staged batch says nothing live)

Checked by recorded base versus live sha256, or by presence of new symbols or files. No batch shows its own content live.

- rev3fix-g: 18/18 at base. None live.
- nxlock-g: none of its new content live; policy.json and the order test sit at core base (core not applied). None live.
- setup-g: 13/13 at base; 4 new files absent. None live.
- oracle2-g: 14/14 at base. None live.
- conv-g: 58 at base; 9 read-oracle files at the oracle2 prerequisite base (not conv content); 1 new absent. None live.
- xplat3-g and xplat4-g (platforms, freespace, corpus): xplat3 19/19 at base; xplat4 platforms 10 at base and 6 new absent; the workflow is not live. None live.
- obs-g: 14 at base, 12 absent. Fleet module absent. None live.
- dashlock-g: 6 at base, 5 absent. None live.
- toolsetup-g probes: 4/4 at base. None live.
- jobsrv-g: 5/5 at base; unit-cache jobserver directory absent. None live.
- census2-g: 10/10 at base. None live.
- quietfc-g: new profile module absent; no CORES reference. None live.
- LIVE, NOT BATCHED: bench2-g applied the host-total patch directly to three host-capacity files (schema, sampling module, sampling test) at about 18:02, outside any gate-g batch. quietfc-g's staged batch depends on it. Its own 16:25 section still says "staged, not applied" (CONTRADICTED, BE5). Gate-g should know this edit exists before it lands quietfc-g.

## Other findings

- R5 is the most load-bearing: the only on-disk run of the planner laws from the repo root fails (0/9, 0/4). The "9/0 and 4/0" claim rests on an overlay run that has no log here. Re-run on a mirror before gate-g relies on it.
- RB2 is confirmed by recomputation: gram2 index-chunks pushes generations/🟨️.mjs over the 65536 B graph-consumer cap. The rebase-g blocker stands.
- SW5 and SW6: the sweep audit's proposed code was partly implemented after 19:27 (Go and Rust purge code rewritten). The audit's present-tense description of the code is no longer accurate.
- S2 and C2: the lanes' own counts were revised after their result files were written; the files were not updated.
- CE3: census2-g's result file is an interim write. Treat its census-two numbers as absent.
- Not checked: the bench2-g dashboard "lease error" incident and the obs-g guide incident beyond the log lines above; the mirror-g "v2g" incident.
