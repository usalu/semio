# 🧭 Wave 1127 Coordination

## 00:10 (11/27) start
Weekly usage 4 %. Fleet: ready 8/16 rows (m02 m06 m07 m10 m11 m12 m13 m15). Coordinator load 46, swap 6.7/8 GB, disk 125 GiB free.
| lane | model | agentId | scope |
|---|---|---|---|
| H1 | sonnet | a2e4c59a9c5f89ad0 | workspace stage failures m01 m03 m08 m14 |
| H2 | sonnet | ae1f360f5ce6e83bb | cache credentials m11 m12, g1-server read-only |
| H3 | sonnet | aae1789b1cc15a0a5 | unit cache skip reasons / hit rate |
| H4 | sonnet | a0667c3dc24724283 | remote execution offload reasons |
| H5 | sonnet | aae9d22a0bcdd3f73 | monitoring readers + dashboard metrics live |
| H6 | sonnet | a6522793c14a8c199 | low-resource carving (units over ceiling) |
| H7 | sonnet | af2dc8783770eb80c | Windows m05 |
| H8 | sonnet | a798c1e76e92b8c7e | Linux m04 inodes zero-touch |
| H9 | sonnet | ac156ae36d194ce89 | whole graph seal + repo-mcp rebuild |
| A1 | haiku | a5a48c55b6ad4b1f4 | open-items backlog audit |
| A2 | haiku | a56b4693e8b73f5ff | worker failure census |

## 00:20 (11/27)
- A2 DONE (audit-A2-census.md): all origins on head 9c8a98ec2349 / code 5af6df83766e; m05 node pin 24.14.1 vs 24.15.0 + guards 2.53.0 (→ H7 notified); m04 guards 2.55.0; stalled-SIGTERM stages on 13 origins; host-name collisions (uelis-mac-mini m08/m10/m13, uelis-mac-mini-4 m06/m12, coordinator hostLabel semio-g1-m08) remain USER (`repo fleet hostname apply` needs sudo).
- Launched H10 (sonnet, a5efa58ac45ce9d2f): stalled stages + guard/tool version drift.

## 00:25 (11/27)
- A1 DONE (audit-A1-backlog.md, ~90 items in 9 sections, 12 USER-only).
- Launched H11 (sonnet, a9d53dd880c2848e9): POST-SEAL queue Q2-Q5 + T5. H12 (sonnet, ab844ae3f1b2a4236): test reds T1,T3,T4,T6,T7,T9-T11,T14-T16,T20.
- Coordinator load 75 (1 min), memory free 55 %, swap 6.7/8 GB.
- 00:15 H6 round 1: 23 units over ceiling listed; only declared carving entries (1.1× peak) — sent back to really carve process3d_host/harness, actor_input. Brief addendum: no whole-checkout searches (our rg/ugrep added to load 62).
- 00:35 H1 interim: m01/m14 failure was E0425 OriginalCancelRootIdentity (peer re-exports since landed; exit 130 = Nx killing siblings); m03 stale bundle from mid-edit source (now consistent); m08/m14 wait on a 5918 MB non-shrinkable slice-read grant on 16 GB → launched H13 (slice read fits 16 GB).

## 01:00 (11/27)
- H4 DONE: offloadGateV1 offloads worthy units when host verdict is `wait` even with a free slot (gate 4/0); counters are cumulative since 11-14 (unmapped/infrastructure historic) → H5 should show windowed rates. Live delta unverified.
- H3 DONE: root cause = cache server down for hours (341k upload-deferred down, 36k 507); uploader pauses whole during outage, unshareable split from failed, spool 16384, job age 24 h, pin 16384. typecheck 0. Live counters owed. Divergences 1038 = non-deterministic .d/.rmeta → H15.
- H10 DONE: false stalls on "Planning the selected targets" (slice worker progress invisible) → readySliceMovedSignatureV1 resets stall clock. Landed WITHOUT lock/typecheck → coordinator ran typecheck (83 files, 0 errors) + dangling 0 for H3+H10 files. Guards drift = host git version (follow-up: pinned git per platform).
- H7 round 1: EPERM unlink → rename into .busy + deferred unlink (tree-removal 9/0). Round 2 sent: tools-stage hang at source.served + partition capacity.
- H8 round 1: need is real (copyfile backend), collectors exhausted, inode census diagnostic landed (no lock/landing entry). Round 2 sent: reduce the need (scoped closure, one generation, package-store sharing).
- H6 round 2 running (real carve). H1 round 2 running (source-transient half-edit classification).
- Launched H14 (sonnet, a609fa3f103516cac): cache server reliability (restarts, downtime, 507). H15 (sonnet, ac703d865f24ce669): deterministic .d/.rmeta outputs.
- H12 extra: profile test lto + disk test type errors, uploader facts-access + buildscript fleet-run reds.
- 01:10 H2 DONE: no live denial (all 16 registrations valid; m11/m12/g1-server put < 5 min); findings were stale cumulative counters → unit-cache-* findings fire only on growth within 900 s; agent re-publishes registration every 30 s; g1-server writes (admin, read-write). build-fleet-status 31/0. → H5 told to generalize windowed rates.
- Launched H16 (sonnet): fleet end-to-end proof probe (cold/warm/remote-down, output identity).
- 01:30 H8 round 2: scoped install (root + ✏️s ≈ 142k + 70k cache inodes) is the only big lever; hardlinks refused for the package store by protocol; need = sum of six scopes. DECISION: per-scope sync receipts + workers install only the scopes their stage needs, one atomic fleet-wide landing (one dependencies re-run per worker accepted); bun-cache as lease-aware last-priority collector.
- 01:40 H5 DONE: storage reader (8 serial diskutil) + fleet reader (re-hash 1032 files) blocked the event loop → async probes + persisted closure digest; fleet status 10.3 s/3 failed → 3-4 s/13 ok; `rates` section 15m/1h/24h from metric rings; warden finding windowed; metrics live 15/16 origins, rollups verified. DECISION round 2: per-reason metric ids, drop H2 baseline file, client-side cache.remote.* on every origin.

## 00:35 (11/27, real clock; entries headed 01:00-01:40 above were written between 00:20 and 00:34)
- H14 DONE: no crashes; 37 restarts = code-landing hand-overs (~every 30 min) → cache role quiet 15 min, ≤ 1 hand-over / 3 h; hours "down" 11-26 04-08Z = 96 % volume → 1.13 M 507s (fixed by F79/F80/F82); 507/503 now logged bilingually. Risk: coordinator data volume 92 GiB free vs 74 GiB reserve.
- H7 round 2: provisioning deadlines 3600 s + bounded PS downloads (26/0, PS untested); death note cross-platform (NTSTATUS, .dmp). Leases partition overflow on m05 suspected leak. Round 3 sent: per-partition usage self-reporting, leases retirement, supervisor heartbeat-only stall.
- Coordinator host: swap 10.7/12 GB, load 51, data volume 92 GiB free.
- 00:50 Launched H17 (sonnet, a5fca739c0a6fc6ed): coordinator disk headroom (92 GiB free vs 74 GiB reserve), regrowth, reserve policy.
- 00:55 H13 round 1: slice read minimum = 40 % of measured booking (floor 2048 MB), interactive rank, lend admission vs ramping reservations; m14 now runs a 2230 MB slice worker; m08 held by host pressure outside the gate (fseventsd 3.2 GB, swap 9.9 GB). Round 2 sent: gate/main/progress reds + our fseventsd churn on workers.
- 00:40 Goal check-in: lanes progressing but several blocked on load<40 monitors (load 51-53 from external processes). Brief addendum: load gate replaced by memory gate (≥20 % free, swap ≥1 GB, max 10 min wait, nice -n 10).
- 00:45 H1 round 2: m01 READY (cargo check job-context green, 14 min); source-transient patterns extended (ReferenceError, rustc cannot find, No matching export, bundle-changed) 4/0; Nx authorizer bundle re-captures sources before rename (concurrent-edit refusal). m03 retrying on lagged bundle; m08/m14 wait on swap-crowded hosts.
- 00:55 H15 round 1: .d canonicalised (per-run proc-macro observation paths dropped; 10/11 real pairs identical). Open: .rmeta/.rlib reorders, stderr-only (43 of 64 recent), build_script_build (78). Round 2 sent. Ready 9/14 (m01 added). H1 round 3: m03 stale bundle key.
- 01:00 H7 round 3: CPU-only liveness > 30 min now ends a stage as stall (persisted idle clock); every worker publishes partitions.json (files, logical/allocated bytes, top holders) every 10 min; capacity-exceeded marked as full store → lease store retires idle state + retries in place. 64 MiB "leases" claim retracted (test constant). Waiting for m05 partitions.json. H1: m03 past the bundle error, now capacity wait.
- 01:10 H15 round 2 DONE: spool-full test fixed (37/37); stream digests now compared rows on the server so stderr-only divergences keep both blobs; real proc-macro + dependent in two roots → identical .rmeta/.rlib (observation paths not embedded); build_script_build/dylib identical with -oso_prefix/-install_name (wrapper has them since 11-25 17:12). Open: 11-26 .rmeta rows (job_clock, manifest_plugin) → audit after worker restart.
- 01:15 Weekly 10 %. Launched H18 (a47be71cb59b09014) Nx cache hit rate + namespace + endpoint resolver; H19 (a1e41e3ee8b2a5c46) wrapper wedges, broker watchdog, Windows pipe, grant growth; H20 (a3ce10f7f12137f35) scheduler registration identity, drain, Windows restart; H21 (a49a567de83c9dcee) slice inventory resumable/incremental; A3 haiku (a2edf8daa6825e663) verify closed items + stall counts. H17 + R11 wasmtime cwasm cap.
- 01:20 A3 DONE (audit-A3-verify.md): closed L13 L19 E7 C1 C3-C6; M4 open (75 stalls / 541 runs, last 11-26 22:30 — before H10/H7 stall fixes); F74 still unlanded (H11 owns); m05 "still working after 9372 s" (H7's 30-min CPU-only stall reaches m05 on next start).
- 01:30 H1 DONE (m03/m08/m14 running, none failed again). H13 round 2 DONE: gate 34/0, main 29/0, progress 6/0 (test/oracle fixes only, product verdict proven unchanged); image volumes already silenced; hypothesis: warden-stopped FSEvents stream holders make fseventsd queue → launched H22 (sonnet) to test + fix + move churn trees into silenced images.
- 01:35 H8 round 3 DONE (landed 00:57-01:07): per-scope receipts, scoped `dependencies sync --scope`, ready plan derives scopes (m04 need 428,796 → ~142k), bun-cache lease-aware collector. m04 now RUNNING dependencies. ALERT: m04 free disk 129 GB → 14.4 GB. Launched H23 (sonnet): m04 disk runaway, on-demand scope install (no manual sync), ops/help-offline/ready timing reds.
- 01:45 H21 round 1: journal-vouched memo check (20.5k rows 5231 → 818 ms here, 6/0, typecheck 211/0); pre-inventory labels live (L13). Round 2 sent: L16 pass-result facts + journal-vouched catalog/membership hash.
- 02:00 H22 DONE: hypothesis refuted (SIGSTOPped FSEvents clients don't grow fseventsd; 240k-event churn, RSS 4-8 MB); churn per 10 min: checkout ~90k (replica), $TMPDIR 12.6k, .cache/f 7k, scratch 460 → no move landed. Coordinator fseventsd fell 9.6 GB → 6 MB on its own (transient). m08 needs sudo sampling → USER (steps in result-H22.md). H17 round 1: no garbage, CAS 53 → 98 GiB is legit cache growth; stale usage hint defect → round 2 (bound tracks free space, fresh measure, one physical copy per blob).
- 02:10 H5 round 2 DONE: 26 per-reason counters + cache.remote.refused (14/15 origins publish), client-side cache.remote.* on all origins, H2 baseline deleted (one rate source), fleet status 15m/60m/24h rows, dashboard skips/declines groups. Dashboard Rust headless test not compiled: `Cargo workspace has no current source-bound members: …/📈️metrics/📦️packages/🦀️rust/Cargo.toml` → launched H24 (urgent).
- 02:15 H20 interim: registrations carry startedMs, younger same-origin process replaces older (worker-replaced); leave/replace/stop no longer costs an attempt; queue wait = 10× measured p90 clamped 60-300 s; Windows supervise script one statement per line (probable restart-loop defect). Topology tests pending.
- 02:20 H19 DONE: wrappers return fleet claim after 3 s admission wait, ≤4 long-polls per host, bounded slot waits (🪢️wedge 10/0); broker drops units with 20-min silent scheduler (🐕️watchdog 5/0); Windows CancelSynchronousIo (type-checked only); grant growth on driver memory growth. Big suites unfinished → launched H25 (sonnet): re-check fleet cache after long wait + run H19/H20 suites to completion.
- 01:40 Goal check-in: Ready 9 → 5 (m02 m11 m13 re-running workspace after our landings, m12 backoff stage-failed, m03 backoff). Coordinator load 66, swap 12/13.3 GB. Brief addendum: landing slots :00-:10 hourly. H6 round 2: baseline cold check 163/805 units in 43 min (capacity gate under swap) — no carve yet; process3d_host cut = move 461-line test module into its own test crate.
