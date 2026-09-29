# Audit A15-state — Session 15 State Refresh

Auditor A15-state, Sonnet 5, read-only, foreground, 2026-09-29 19:1x–19:4x (CEST). No builds, processes or edits beyond this file.
Successor of `📓️audit-s14-state.md` (2026-09-27 18:36; most rows superseded, see §8). Sources: `📓️session-14/15-preamble.md`,
`📓️fleet-14-agents.md` (coordinator log from 2026-09-29 13:00 on, earlier lines for context), `📓️fleet-15-agents.md`, `📓️t6-queue.md`,
`📓️landing.md` `# Session 14` + `# Session 15`, every `📓️wp-<slice>.md` Session 14 table + tail of the 28 slices, plus the captures
named below (`.🧬semio/🌐hub/…`). "measured" = a capture is named; "reported" = report text only; "unverified" = neither.

## 0. Headline

- **No open P0.** The session-13 P0 (read-audience agent could edit) is closed: refused-relay probe correct on 7800/p33 (G12 15:42/16:04,
  `s14-g12-logs/battery-p33-1/`) and `agent-ceiling-check` 16/16 en+de on a p24 clone (H13, 28 Sep 21:2x). Every open item below is P1 or lower.
- **The critical path is still the chain.** Coordinator-launched chain 57946 (t6 catalog, 18:14:37) sits in rebuild-all 4/11 components
  (measured 19:21). Until `final-publish.rc = 0` + "WINDOW 5 OPEN": guest freeze ON, 7800 stays on p33 (hub built 13:10, before the hub
  fixes listed in 2.2), and a tree-built hub cannot boot the p33 catalog, so every hub-side live re-check waits for the t6 publish.
- **36 T6 sets landed in three GREEN rounds (16:12, 17:33, 18:15)** with combined native 228 / wasm32 178 / renderer / tsc 0 / boot proofs, but
  owner laws after landing are almost unmeasured (LW1 recorded 2 rows). Nine prepared sets are not landed, and **5 of them are not in
  `📓️t6-queue.md` or `wp-l1/w3-trains.json`** (§1.3).
- **Newest MCP coverage is worse than the record says:** capture `battery-p33-4` (18:49) = **17/63**, not the 32/63 in the fleet log/G12 report
  (p33-3, 17:43). 41 of 63 opens took ≥ 10 s and 43 mutations failed: consistent with H13's WAL-writer cap (32 live document sockets per
  backend), not proven for this run. G12's agent died ~18:45, before the run ended.
- Collaboration on p33 is 7/15 (en) and 7/15 (de) in the React two-human run; the two P1 causes (same-point concurrent typing divergence, Space
  table never updating) are both prepared-not-landed sets (C12 hub-order, SH2 P2).

## 1. Cross-cutting state

### 1.1 Chain and hub (measured)

| item | state | evidence |
|---|---|---|
| chain t6 (`s14-w4-catalog-t6`, root `s14-w4-hub-7800-t6`, pid 57946) | hub-prewarm rc=0 18:23:24 (527 s); rebuild-all "4/11 components (descriptors)" started; at 19:21 the demonstrator build was at 580 s; not yet: generate 5/11, check 6/11, activate-s, verify-s, flow-core-bindings, preflight, publish-all, hub-build, mcp-build, 7800 restart | `s14-w4-logs/final-rebuild-all.txt` (l. 8555 + tail), `chain-final.txt` |
| forward release lane | stdio rc=0 18:48, gis rc=0 19:04, animate rc=0 19:15, architect warming | `s14-w4-logs/final-warm-fwd.txt` |
| durations to expect | components step 6 810 s (L1 T1-close 03:25), publish-all 817 s (c10 13:09), hub-build 30 s + mcp-build 209 s; estimate publish 21:30–22:30 (unverified) | `wp-l1.md`, `wp-w4.md:311-313` |
| chain fix by coordinator 19:1x | `CATALOG_MANIFEST_FIELDS` admits `hostedArtifactKinds` (8 stdio family describes had failed); module load verified only | `📓️landing.md` `# Session 15` |
| 7800 | p33 (33 packages = all but the 9 stdio families + demonstrator), first READY 13:14:16, resumed 15:36:43 (boot 6 s), gen `21109b7e…`, FRESH, open-plan 63/63 creatable kinds PASS, idle footprint 1 764 MB | `wp-w4.md:313-316`, fleet-14 15:3x |
| tree hub vs p33 catalog | tree os-hub refuses p33 at boot (`stdio.definition: decoded Stdio descriptor differs from native artifact semantics`, C13 17:33–17:58): private hubs of H13/H14/C13 cannot verify T6-era hub fixes before the t6 publish | `wp-c13.md` (17:33 entry) |
| lanes | native lane HELD by U6 orphan (`r6-base.sh`, LANE-EXIT 19:20:39 = ended); overlay lane HELD by LB2 orphan `lb2-p17-proof.sh p17-s1` (pid 70960); disk 61 GiB at 18:39 (chain growing) | `📓️session-15-preamble.md`, fleet-14 18:39, `s14-u6-logs/r6-base.txt` |

Chain risks (not measured, from reports): (1) C13 row 16 part B (SDK guard `plugin-assembly.declaration-document-schema`) is proven only by
this describe; red = L1 reverts B (1 file). (2) First publish with hosting (9 families + demonstrator via p9/p15/3b); earlier publishes died on
profile-id > 256 B, demonstrator ownership collision, EAGAIN. (3) hub-build/mcp-build compile the tree right after publish. (4) LB2 p17 is NOT in
this chain: stdio kinds will not open on the t6 hub (guest pack-schema-hash vs catalog pin, LB2 18:39), so a further chain follows window 5.

### 1.2 Landing ledger (T6 landing.md + `wp-l1.md`)

- Round 1 GREEN 16:12 (22 sets): rows 1, 2, 6b, 7 (12 WG11 scripts), T7a, T7b, 9, 11, 13, 14, 15.
- Round 2 GREEN 17:33 (9 sets + typegen regen + pptx fixtures): rows 3, 3b, 4, 5, 5a, 5b, 5b2, 5c (+ 2 LB2 in-tree hotfixes).
- Round 3a GREEN 18:15 (5 sets): rows 16 (A+B), 17 (H14 Rust link), 6 (U6 row-target, 145 files), 8 (WG11 Marketplace).
- Proof per round: native 228 crates rc 0, wasm32-wasip2 178 rc 0, renderer wasm32-unknown-unknown + os-kernel rc 0, tsc os/ui-react/renderer-react/hub 0, boot Home ok
  (`s14-l1-logs/T6R{1,2,3a}-*.txt`). Owner laws after landing: only `wp-lw1.md` T6-1 (os-kernel nextest 1 270/0) + T6-2 (taxonomy valid).
- `📓️landing.md` `# Session 15`: 1 row (coordinator). L1/C12 have `## Session 15` sections; the other 11 wave-A reports had none at 19:3x.

### 1.3 Prepared, NOT landed (window 5, freeze)

| set | owner | size | proof state | in `t6-queue.md` / manifest |
|---|---|---|---|---|
| fault localization F1+F2+F3 (+ FH1–FH3 families) | S20 | 688 files clean, 1 merged, 8 new (dry run 13:02, BEFORE round 1: must re-base) | framework `check --lib --tests` green on overlay 12:02/12:51; family crate checks not recorded (FH1–3 waiting on "framework green") | row 12 (LAST) |
| P4 ephemeral-on-continuation (peer selection) | C13 | 2 files (SDK) | dry run clean 17:4x, write→revert byte-identical, **not compiled** | duplicate row number 17 |
| hub-order replicas (concurrent typing) | C12 | 85 ops / 21 files | overlay 18:44–18:59: check rc 0; store `os_store::` 489/493, plugin `document_backbone tool_run` 35/39 (4 reds); wasm32 not run (`s14-c12-captures/order-overlay-proof-2.txt`) | **no row** |
| Space index transient + checkpoint activity (P2) | SH2 | on top of the 80-file set | overlay `p2a`: space-home 123/123, kernel `os_directory` 62/62, plugin-space 85/1; space-space 92/19 with 15 reds needing a baseline run | **no row** |
| jack headless query | P9 | 19 hunks / 3 files | dry run clean; `overlay-t7-1` queued 18:26, result not recorded | T7c |
| pack-schema identity (stdio) | LB2 | 116 files | dry run clean 18:4x; overlay proof orphan holds the overlay lane | T7d |
| paged DOCX (E1) | U6 | — | overlay-proven 13:2x (docx 125/0) | **no row** |
| verb-arg law p2/p2b | LB2 | 39 files | dry run clean 28 Sep 17:3x | **no row, absent from `w3-trains.json`** |
| contributions 2b-2 / i18n 1b | S19 | 2b-2 65 files sized, not started; 1b unblocked (T14 h9l landed T1) | not started | none |

## 2. Outcome 1 — working os `s` frontend, all plugins and artifacts

**Measured**
- Compile/boot: see §1.2 (R3a 18:15: 228/178/renderer/tsc 0/boot ok).
- p33 hub sweep, en (S18, 29 Sep 15:3x–17:36): **20/33 kinds PASS** create→open→rendered edit→undo→redo→reload `[0,1,0,1]`, 0 faults; picker shows
  63 kinds with real labels; sweep paused for C13's P1, de and Home/directory fresh user not run (`s14-s18-logs/sweep-s18-14c-p33-en.txt`).
- Local matrix (S18 28 Sep 18:59, de; en same): **127/145** (viewers 70/70, editors 57/75 = 12 norm + 6 stdio guest reds); csv/tsv PASS 19:4x. NOT re-run
  since T1 (p6/p1/p7, xml regen), T3 (norm), T6 (LB2 rows) landed.
- io matrix (S20): local en 32/75, archive round trip 41/75 (28 Sep 13:5x, before the initializer set landed T1); hub 7800/p24 en 10/12 pinned kinds
  (29 Sep 00:31, `s14-s20-io/s20c-hub7800-en-3/`); de not run.
- Command reachability 4/1166 unreachable (S20, 27 Sep 18:4x); SH2 (3) and AV2 (1) landed T3; not re-measured.
- Perf (F3): serve first HTTP 65→34.6 s under load 80–116; writer 3→0.04 shell renders per key; puzzle3d hover 21.8→17.2 ms; `verify boot` blocked under load.
- Renderer-wgpu: WG11 overlay build 8 lib 1 564/1 570 (12:50, pre-landing). Post-landing base run (U6 `r6-base`, 38 binaries, lane exit 19:20:39): a renderer-lib
  process hung ~37 min (launched 18:42, sampled 19:19); the suspected test passes alone in 0.09 s (`s14-u6-logs/r6-base-renderer-hang-sample.txt`, `r6-live-renderer-hang-alone.txt`).
- Dependency gate: literal-external oracle conflicts 6→2 (image, serde_json) (R10 07:12); launch.json 1 420 rows, laws 7/7, goal plan 71 checks (R10 04:52).

**Open P1**

| # | defect | owner | evidence |
|---|---|---|---|
| 1.1 | t6 publish + first hosted publish; stdio 9 families/demonstrator absent from 7800; stdio kinds still won't open after it (needs p17 + republish) | W4 / L1 (p17) | §1.1 |
| 1.2 | Row 12 fault localization not landed (1 287 code-like sites, strict global law, must re-base on the post-T6 tree) | S20 (+FH1–3, L1) | `wp-s20.md` row 7, t6-queue row 12 |
| 1.3 | Space directory stalls at ~28 docs (`foldDirectoryEvents` refused `publication-stalled`, 4 096 units) | SH2 P2 | S18 row 15 (reproducer space `01a0ed64-…`) |
| 1.4 | hub-sweep guest reds: computation.flow "ownership violation", 5 norm kinds `setSnapshot` no-op | S19 | S18 row 15 (d)(e), G12 §14c |
| 1.5 | hub-doc engagements/measures read from the LOCAL instance (draw "N layers" frozen on every hub doc) | S18 (host+worker, approved 17:36, not landed) | `wp-c13.md` 17:28, fleet-14 17:36 |
| 1.6 | stdio json/i-json/xml/xml-valid + 9 families not on hub; csv/tsv/txt/md/html open but have no hub write path | LB2 p17 (T7d) + W4 | G12 L7, LB2 18:39 |
| 1.7 | Local matrix, 1.8 census, io de/hub legs not re-measured on the post-T6 tree | S18 / S20 | above |
| 1.8 | wgpu shell answers `os.local-catalog.*` with a typed refusal only (route B not implemented there) | WG11 | `wp-wg11.md` row 4 |
| 1.9 | renderer-wgpu suite hang in U6's base run | U6 | above |
| 1.10 | 4 kinds `created=false` in the sweep = 120 s window vs cold component install (harness `createdLate` fix, re-run pending) | S18 | S18 row 15 (a) |

**UNOWNED (no wave-A slice)**
- `catalogue.sourcing`: guest `plugin.internal validation failed: app-owned one-item preparation factory rejected its exact owner bundle` (S18 hub sweep; G12 coverage `mutate=failed`, p33-3/p33-4). No owner named anywhere.
- 63 example loaders silently fall back to an empty document (`PRIMARY_TEXT).unwrap_or_default()`; AGENTS.md forbids fallbacks) — LB2 14c: "not mine, 63 sites".
- Kind-picker labels ambiguous without the plugin ("2D", "3D", "2D Grid", "Flow", "Equation", S18 row 15); writer editor tab reads "Jack" (C12 row 28); nobody assigned (T14 kind-choices landed T3-late; wave B T14 idle).
- `[TRACE] typed-operation slots …` printed on every page since the T5 `[DEBUG]→[TRACE]` codemod (C12 row 28 "R10: gate off by default") — R10 is wave B.
- Accessibility (WCAG/keyboard) and progress+cancellation regression measurements (audit-s14 5.3/5.4): unmeasured since session 12/13; no slice.
- Whole-suite reds in `⚡️caching` (≥ 5 independent points, ST2 row F) and framework `ui_turn_patch` + describe-ABI reds (S19 17:02 "UNOWNED, track").

## 3. Outcome 2 — working hub backend (db, presence, auth, observability)

**Measured**
- Hub gates (28 Sep 14c → 29 Sep): `os-hub:test-all-features` EXIT 0 (lib 247, bin 176) 14:09 and again 18:18 (28 Sep); kernel-db vcs removal landed 29 Sep 08:40 (sqlite 726/1, the 1 = wall-ratio law, load-bound);
  `semio-hub check --all-features` EXIT 0 + auth laws 35/35 + hostile-input 8/8 for the stream limiter 17:53 (`s14-h13-logs/hubassets2-*.txt`); H14 hold 12 laws 3/3 18:13 (`s14-h14-logs/hold12-*.txt`).
- Live on p33 (H13, 29 Sep ~18:3x): component assets streamed lazily, per-user content-addressed LRU, 33/33 first opens, norm 53 MiB 1.3 s (was 78 s), 2nd gateway 0 component bytes (`s14-h13-logs/asset-probe-run{1,2}.txt`).
- Durability/security on p33: participant 19/19, security 6/6, untrusted 6/6, durability 23/23 (G12 15:39–16:04); hostile-input-check PASS 28 Sep 17:20.
- Drills (B3-era, 26–27 Sep): backup/restore 6/6, graceful shutdown SIGTERM→exit 1 025 ms with a creation interpreting 7/7, README/metrics parity 7/7 (H14 item 4/5); residency p24 128 MiB: 6 guests / 133.2 MB ≤ 134.2 MB, 68/72 creations (`s14-h14-logs/residency-8161-p24-128mib-2.txt`).
- Not re-measured on p33/t6: pg/neo4j live gates (rows 2.4/2.6; H13 item 5 "blocked until ALL" though p33 has been READY since 13:14), boot/creation latency after the T4 codec-origin set, Docker cold build, native Linux.

**Open P1**

| # | defect | owner | evidence |
|---|---|---|---|
| 2.1 | **WAL writer capacity 32 per backend**: every document socket holds one slot, the 33rd live document gets `limit exceeded: WAL writer capacity` → client backoff 8 s → `artifact_open` returns after ~10 s unlinked → Ack timeouts; fix = writers per commit with idle release/LRU + declared capacity + FIFO typed wait, law 1 000 docs × 50 clients | H13 (S15 focus) | `wp-h13.md` 18:2x, `s14-h13-logs/link-probe-2.txt`, coverage p33-4 (below) |
| 2.2 | Live 7800 lacks the landed hub fixes: plan-socket authority (no `4401` on Check In, H14 17:47), foreign-revert admission refusal (C13 17:07), stream limiter (H13 17:53), typed 429 | W4 (t6 publish + hub-build) | landing rows; C13 17:33 |
| 2.3 | pg/neo4j live gates + backup/restore/shutdown drills on the current hub/tree | H13 nominal, absent from S15 focus | audit-s14 2.4/2.6/2.11 |
| 2.4 | sqlite storm wall-ratio law red on a loaded machine (0.64 > 0.5); H13 recommends a load-free bound — decision pending | H13 / coordinator | fleet-14 18:1x (28 Sep) |
| 2.5 | Creation latency = guest `codec.genesis` in the hub's own interpreter (stdio md/csv 26–72 s, puzzle 80–184 s cold); T4 codec-origin landed 06:32 but latency not re-measured | H14 (wave B) | `wp-f3.md`, `wp-h14.md` item 3 |

**UNOWNED in wave A:** Docker cold build + native Linux + devcontainer container proof (Z4, wave B, waiting for "Docker GO" after 7800 READY on ALL); Windows audit is closed at review level (Z4 row 3).

## 4. Outcome 3 — collaboration between users over the hub (React + wgpu shells)

**Measured**
- React two-human collab-e2e on p33 (C12 29 Sep 15:3x–16:26; #11/#13 not yet landed at that time): **en 7/15** (1,2,3,4,5,8,12; 9/10 SKIP external hub), **de 7/15** (1–5,7,12) (`s14-c12-logs/c12collab-p33-{en,de2}`).
  Reds: STEP 6 (Space table Updated never moves), STEP 11/13 (concurrent typing loses/duplicates runs), STEP 8 de (key after reopen vanished), STEP 14 (puzzle3d `Identity`; row 16 landed R3a), STEP 15 (15 s cut shows "fresh authoritative restore").
- C13 on p33 (29 Sep 16:33–16:55): viewer en 3/3 + de 3/3 kinds (10/10 checks, no Undo/Redo offered), cross-undo en 2/2 (12/12 per kind; crafted foreign revert refused by replicas) (`s14-c13-acceptance/c13p33-{viewer-en,viewer-de,undo-en,undo-en-2}.json`). cross-undo de not run.
- wgpu shells (WG11, p24, 28 Sep ~22:00): wasm32 en 24/24 + de 24/24, agent-pixels en 12/12 (0 px diff), wasm32↔native 11/11, native↔React 8/8, cursors 4/4, native two-shell 12/12, wasm32↔React 13/14, wasm32 cursors 5/7. All WG11 T3/T6 sets landed since; NOT re-run on p33/t6 (agent-pixels de not run).
- Hub-side: foreign undo/redo refused at the socket (law 2/2 18:14, `s14-c13-runs/native-hub-law-1.txt`), live re-check waits for the t6 publish.

**Open P1**

| # | defect | owner | evidence |
|---|---|---|---|
| 3.1 | Two React writers typing at the SAME point diverge permanently (store folds remote ops by HLC, not hub order); hub-order set prepared but overlay proof red (store 489/493, plugin 35/39) and unregistered in the queue | C12 | `wp-c12.md` rows 28–30, `order-overlay-proof-2.txt` |
| 3.2 | Space table `Updated` never moves on check-in; Space index keeps directory+presence in the undoable CONFIG lane; host resends full history per event | SH2 P2 (unregistered) | `wp-sh2.md` 16:36 decision, fleet-14 16:36 |
| 3.3 | Peer selection never painted on note/draw/puzzle (outgoing presence carries `interaction: null`); draw additionally has no peer-selection painter (`Canvas2dHost`, guest scene contract) | C13 (P4 uncompiled; draw guest set not started) | `wp-c13.md` 17:28 + row 6 |
| 3.4 | STEP 15 long cut and STEP 8 reopen keystroke loss: fixes landed (SeedHistory T3, catch-up gate, row 17 Rust link) but no live re-run since | C12 | `wp-c12.md` rows 15, 28 |
| 3.5 | Live re-checks pending the t6 publish: puzzle3d opens (row 16), 4401 fix, foreign-revert refusal, writer collab en/de with #11/#13 | C12 / C13 / H14 | landing R1–R3a |
| 3.6 | wgpu live suite (24/24 class) not re-run post-T6; wgpu route-B (see 1.8) | WG11 | above |

**UNOWNED in wave A:** none new; cross-undo de and agent-pixels de are nominally C13/WG11 but appear in no S15 focus line.

## 5. Outcome 4 — AI integration over the semio MCP (`semio-framework-os-mcp`)

**Measured** (G12, 29 Sep, captures `.🧬semio/🌐hub/s14-g12-logs/`)
- 7800/p33 battery p33-1 15:39–15:56: participant 19/19, security 6/6, untrusted 6/6, durability 23/23, refused-relay correct (read → `PERMISSION_DENIED viewer.read-only`, head 0), quartet 18/19.
- Quartet 19/19 on p33-2 (16:56), p33-3 (17:24), p33-4 (18:15). User path on a fresh hub 8031: en 9/9, de 9/9 (16:02/16:03, scoped offers live; `battery-p33-8031-1/`).
- Coverage (63 hub-creatable kinds, all created + opened): p33-1 2/63 → p33-3 **32/63** (17:43; 28 `mutate=failed`, 22 opens ≥ 10 s) → **p33-4 17/63** (18:49; 43 `mutate=failed`, 41 opens ≥ 10 s, gateway `semio-os-mcp-codecpage`).
  p33-4 non-pass besides the 3 export refusals (3d.cad, 2d.layout, 2d.shooting; their mutate step passed): 43 `mutate=failed` rows, most after a ~10 s open (consistent with H13's link wait in 2.1, not proven for this run), incl. stdio ×5, s.vcs.vcs (60 s open), s.gis.gismap (18 s), graph.trinity (42 s), all 15 norm rows.
- Observation (unverified cause): this audit session's harness reported `semio` MCP `CONNECT_TIMEOUT` (30 s) at startup.

**Open P1**

| # | defect | owner | evidence |
|---|---|---|---|
| 4.1 | Coverage 17/63: WAL cap (2.1) is the dominant cause; re-run needed after H13's fix + t6 publish | H13, then G12 | `battery-p33-4/coverage-hub.txt` |
| 4.2 | stdio csv/html/md/tsv/txt: no hub write path (`codec.pack-schema-hash`: no structural record specification) | LB2 p17 (T7d) | G12 L7, LB2 18:39 |
| 4.3 | norm 12/14 (untyped `path`/`setField` args, "unexpected byte 67"), flow/sequence "ownership violation", forms args don't decode, procedural `addWidget` guest trap (wasm fn 30081) | S19 | G12 §14c "Failures by owner" |
| 4.4 | Exports refused: layout `layout:out` (no exact bounded snapshot), shooting `photos:out` (SVG raster needs native capability), cad `brep:out` | S20 (layout/shooting raster); cad = CD1 (wave B) | p33-3/p33-4 rows |
| 4.5 | trinity `loadExampleQuery` needs an attached window (T7c prepared); config/draft-lane agent ops declared n/a (decision 18:20) | P9 (wave B), S19 | `wp-p9.md` 18:2x, fleet-14 18:20 |
| 4.6 | Coverage battery must supply `revision` for stdio set-cell/SDK set-node (g12-revision-binding landed T1) | G12 | fleet-14 14:4x |

**UNOWNED in wave A:** `catalogue.sourcing` mutate failure (§2); the MCP connect timeout above (no diagnosis owner); trinity/puzzle preview follow-ups sit with wave-B P9.

## 6. Consolidated UNOWNED / unregistered (wave A)

1. `catalogue.sourcing` guest refusal (S18 sweep, G12 coverage).
2. 63 silent example-loader fallbacks.
3. Five prepared sets absent from `📓️t6-queue.md` + `w3-trains.json`: C12 hub-order, SH2 P2, U6 E1, LB2 p2/p2b, S19 2b-2; plus C13 P4 shares row number 17 with H14's landed row.
4. Chain #3 (after window 5: p17, P4, hub-order, P2, T7c, faults, describe regen, stdio republish) has no owner or schedule; W4's S15 focus ends at the t6 probes.
5. Owner-law runs for the 36 landed T6 sets (LW1 is wave B).
6. `verify interactivity commands` re-measure (expect 0/1166), matrix re-run, io de/hub legs: S18/S20 focus lines omit them.
7. pg/neo4j gates and drills on the t6 hub (H13 focus omits).
8. Kind-picker labels, writer tab "Jack", `[TRACE]` noise; `⚡️caching` suite reds; `ui_turn_patch`/describe-ABI reds.
9. Accessibility + progress/cancellation regression measurements.
10. Ticket housekeeping: `🎫️ticket.json` `sessions` ends at session 14; `ticket_reopen` answered "invalid tool params" ×4 (session-15 preamble).

## 7. Wave B candidates (one line each)

- **P9**: T7c `p9-jack-headless-query` overlay proof (`overlay-t7-1`, result unrecorded) + jack-LSP removal HELD (needs the human's OK to delete the module's bundle `AGENTS.md`) + G12 battery re-run after publish.
- **CD1**: rows 14/15 landed R1; open: wall `From2PointsAndHeight` depth 0 (typology model proposal pending), mass-properties accuracy follow-up (1.3e-4 origin-dependent), cad semio vitest suite after flow-core wasm rebuild, cad `brep:out` export failure.
- **R10**: post-chain taxonomy pass (146 pre-existing unresolved dirs per R10 21:5x 28 Sep, `🐳️containers/🔁️lifecycle`, T6-era dirs), schema-catalog regen (~1 900 lines stale), `bun.lock` note (TS1), gate `[TRACE]` off by default, per-outcome goal-gate verdict, verify every relayed harness target is registered (S18, G12, S20, H13, C13, F3, Z4, AV2).
- **H14**: all P1 landed; open = live re-measure on t6 (boot, residency at 33+ packages, creation latency after codec-origin, wasmtime codec laws on fresh components), hub-only refresh superseded by the chain.
- **LW1**: only T6-1/T6-2 recorded; owner laws for R1 (6b, 7, 9, 11, 13, 14, 15), R2 (3–5c, 3b), R3a (16, 17, 6, 8) not run; H14 wasmtime codec laws after chain.
- **F3**: F3-3 permanent perf-budget verb + acceptance record + R10 spec open; F2-3 idle-memory check open; hub-open + edit round-trip latency need t6; `verify boot` blocked under load.
- **Z4**: waits for Docker GO after 7800 READY on ALL; open = image build/run + two-client smoke, devcontainer container proof, B4 Linux cross type-check (needs a cross C toolchain).
- **T14**: done/idle (F9, H9-L, 5b, kind-choices landed); open only "verify window-2 landings" (LC F1, T13 F4, wfc clock, blocked 28 Sep) and 5b renderer `serde_json` remainder.
- **ST2**: T2 landed; open = confirm the 79 dropped stdio playground launch rows returned (T2 `st2-r10` re-rendered launch.json to 1 419 rows; not checked), `editor_catalog` 90/90 re-check after LB2 rows + U6 6b, `⚡️caching` suite reds (row F).
- **AV2**: `av2` landed T3, LW1 green (video_render 5/5, raster 3/3, TS 5/5 ffmpeg oracle 54 frames); open = live export from a running `s` (local + hub doc) with ffmpeg check, `video-render-export(-native)` targets via R10.
- **EN2**: `en2` landed T3, LW1 green (epJSON 17/17); open = F10b remainder (glTF ♾️any GLB, bcf ×3, docx pairs: 88/96 → 96/96, owner decision on ≈ 2 MB GLB fixtures), epJSON 32/32 live after describe regen.
- **TS1**: done (20 tsconfigs 0 errors 08:00, CAD suites load, `@types/bun`); follow-up: `import.meta.dir` passed at 116 sites / 69 files (22 suites), some guest-linked.
- **FH1 / FH2 / FH3**: idle; source-side census 0 violations (A+H, B+D, C+E+F+G) inside S20's row-12 overlay; crate checks (11 H plugins, stdio, fem, draw, 20 F/G/E/C plugins) not recorded — resume after S20 rebases row 12.

## 8. Stale or contradictory records

1. **`📓️t6-queue.md`** has no landing-status column. Landing per `📓️landing.md` `# Session 14` / `wp-l1.md`: R1 rows 1, 2, 6b, 7, T7a, T7b, 9, 11, 13, 14, 15; R2 rows 3, 3b, 4, 5, 5a, 5b, 5b2, 5c; R3a rows 16, 17 (H14), 6, 8; row 10 = merged into 12; **not landed**: 12, 17 (C13 P4), T7c, T7d.
   - T7a/T7b sit under "T7 (after T6)" with "AFTER row 7" but landed in round 1 (landing rows `wg11-text-advances (T7)`, `wg11-text-kerning (T7)`, 15:48).
   - Two rows numbered 17 (C13 P4 uncompiled vs H14 Rust link landed R3a); fleet-14 says C13 renumbers to 18, the file was not updated.
   - Rows 3–5c carry "PROVEN", not "landed"; row 9 still says "overlay-t6-4 confirms" (landed R1); row 6 mixes "dry-run clean on live 10:3x" with the 17:33 FAULT ×44 and the 16:4x/17:3x generated-hunk drop.
   - Header says "after the all-34 chain"; the chain naming drifted: `s14-w4-catalog-all` (failed) → p34 (c7–c9 failed) → p33 (published 13:09) → t6 (running).
   - No rows for C12 hub-order, SH2 P2, U6 E1, LB2 p2/p2b.
2. **`wp-l1.md`**: the "T6 (queued, post-chain)" row still says LB2 p9/p11/p12 "ON HOLD (scratch s9 red)" although all landed in R2; "T6 round 3 (superseded by 3a)" row kept.
3. **`📓️fleet-14-agents.md`**: the clock note says entries between ~03:5x and 07:4x carry wrong times and the log is not chronological (05:3x after 08:xx); MCP coverage "32/63" (18:20) is superseded by capture p33-4 = 17/63 (not analysed, G12 died ~18:45).
4. **`wp-w4.md`** ends 13:17: table rows 6/8 still "RUNNING", nothing on the coordinator-launched t6 chain (18:14) or the 19:1x manifest-field fix.
5. **`wp-h13.md`** table: item 5 (pg/neo4j "blocked until ALL") is stale versus p33 READY since 13:14; the tail (WAL writer cap) has no table row.
6. **`wp-s19.md`** `## Session 14` table (rows 1–6 "open", 27 Sep) versus the 14c table (PROVEN/PREPARED) and landed T1/T3/R2 sets; **`wp-lb2.md`** `## Session 14` lists p1/p2/p2b as "prepared" (p1 landed T1; p2/p2b never entered a train).
7. **`wp-sh2.md`**: last entry 17:55 has no row for "P2" although L1's round-4 list and the coordinator log use that name; an "evening, predecessor" section duplicates earlier content. **`wp-wg11.md`** table not updated after 12:54 (rows 7/8, T7a/b landed since).
8. **`📓️audit-s14-state.md`** (27 Sep): rows 1.2/2.5 "STILL RED" (p33 published 13:09 on 29 Sep), 3.4/3.11/4.9/4.10 "unowned/NO HARNESS" (C13 viewer + cross-undo measured, H14 parity 7/7, WG11 agent-pixels 12/12), "H11 has zero landing rows" (H13 reconciled) — all superseded.
9. **`🎫️ticket.json`**: `sessions` stops at session 14; session 15 exists only in `📓️session-15-preamble.md`/`📓️fleet-15-agents.md` (repo MCP `ticket_reopen` failed).
10. **`📓️session-15-preamble.md`** says the chain is "rebuild-all 4/11 (describe + materialize-dev)"; measured 19:21: still 4/11 components (descriptors), demonstrator building; forward lane at architect.

## 9. Decisions needed from the coordinator

- Confirm H13's WAL-writer fix (writer per commit, declared capacity, FIFO typed wait) is approved; it gates coverage, collab scale and MCP coverage.
- Add rows to `📓️t6-queue.md` + `w3-trains.json` for C12 hub-order, SH2 P2, U6 E1, LB2 p2/p2b; renumber C13 P4; give S19 2b-2 a decision (land or defer).
- Schedule chain #3 (p17 + describe regen + stdio republish) and name its owner before window 5 closes.
- Human question pending since 28 Sep: may P9 delete jack's LSP module bundle `AGENTS.md`?
- Give `catalogue.sourcing`, the 63 loader fallbacks and the accessibility/cancellation measurements an owner (wave B or a new slice).
- Spawn wave B only after the publish; LW1 first (owner laws for 36 landed sets), then H14/F3 live re-measures.
