# 📓️ S4-RESUME — resume sheet for ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING` (session 4)

Author: S4-RESUME (read-only auditor, Sonnet). Written 2026-10-04 ~02:00 from the session-3 reports, `📓️status.md` L342–691, the three
audits (`📓️audit-s3-core.md`, `📓️audit-s3-tools.md`, `📓️s3-gap.md`) and a read-only look at the disk. Nothing was compiled or run by me;
every "verified" below is what a report says it ran green (command + count quoted), every "owed" is a run nobody has done since the last edit.
Paths: `T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/`; `OSM` = `🧰️framework/🛍️products/💻️os/🔨️modules`;
`FW` = `🧰️framework/🔨️modules`; `PLG` = `OSM/🔌️plugin/🦀️.rs`; `TT` = `OSM/🔌️plugin/⏪️time-travel/🦀️.rs`; `STORE` = `OSM/🏪️store/🦀️.rs`; `FWT` = `FW/⏪️time-travel/🦀️.rs`.

**Layout.** Tables first: §1 per-WP table, §2 owed runs by crate, §3 final regeneration wave, §4 open design items, §5 goal conformance
matrix, §6 audit findings routing. Details after: §7 = one section per WP (item 1 of the brief: report + last heading, owned paths, state, exact
owed commands, open items, coordinator actions, peer blockers). State vocabulary: **DONE-VERIFIED**, **SOURCE-COMPLETE-UNVERIFIED** (SCU),
**IN-PROGRESS**, **NOT-STARTED** (`partly` = split state named in the cell).

## 0. Facts a successor must know before touching anything

1. **Tree and index.** `HEAD` is still auto-commit `202c4b7b5b1` (2026-10-02 17:04). `git diff HEAD --stat` = 20 599 files changed (`git status` snapshot
   2026-10-04 01:50); the changes are STAGED (`M ` column), so a plain `git diff --stat -- <paths>` prints nothing. For fleet rule 28 use
   `git diff HEAD --stat -- <paths>` or `git diff --cached`. Do not trust mtimes: a Codex peer is active (3 242 files under `🧰️framework`, `✏️s/🔌️plugins`,
   `🌎️hub` have mtimes since 2026-10-04 00:00) and ~131 files in the dag/wires/mathematical/sequence trees carry mtimes 00:25–00:32 although their content equals the index
   (e.g. wires `🧬️mutations/🦀️.rs` differs from HEAD only by earlier peer waves: `dsl::DslValue` -> `semio_framework_value::DslValue`, `dsl::os_pack::json` -> `semio_framework_pack_json`, `warn` -> `warning`).
   So "TREE GREEN" of 10-03 11:38 must be re-established before any WP trusts it (`cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib --keep-going`, then `cargo check --manifest-path ✏️s/Cargo.toml --workspace --lib --keep-going`).
2. **Processes at 01:54.** No `rustc`/`cargo` running (load 18 from peers). 6012 and 6112 answer 000 (no serve, no activation since 09-30 #4/#7 builds). Detached guards from this morning are alive: orphan-cargo guard (PID 37166), fingerprint guard (37172), deadlock-breaker (Python, 37180), disk guard (42848); `🔁️activate-retry.sh` is NOT running.
3. **Disk 27–30 GiB free (97 %)** at 01:54–02:05 (rule 36 of the coordinator's addendum says ≈ 14 GiB at 02:00 — it fluctuates), below INFRA's external-sweep threshold (~42 GiB; S3-INFRA S3-13). The disk guard prunes stale cargo units and nx entries (first sweep 10-02 freed 16 GiB); keep > 100 GiB free or the tooling-sweep incident (`project-external-sweep-breaks-nx-tooling`) can recur.
4. **Session-4 fleet already running** (`📓️fleet-4-agents.md`, 02:05): S4-INFRA (activation closure, puzzle 3d first; reports into `📓️s2-infra-report.md` § Session 4) and S4-BUMP (channel-bump frame wave A, then store `.description` wave B; report `📓️s4-bump-report.md`). §3.3 of this sheet is therefore NOT to be started by anyone else — read S4-BUMP's report first; W2A/W1G/LOAD successors must not touch the frame/`LoadDocument`/`.description` code while it runs.
5. **Agent ids from session 3 do not resolve**: every WP below needs a fresh successor; give it rule 28 (read last section, diff with `git diff HEAD`, repair half-edits compile-atomically, append `## Session 4 — 2026-10-04` to the SAME report). S3-WIRES has no report file (create `📓️s3-wires-report.md`); S3-MATH's report is design-only (sections 2–5 literally "(in progress)").
6. **Repo MCP / bookkeeping**: in session 3 `ticket_reopen` returned a malformed result (`structuredContent` not a record), so the session was appended to `🎫️ticket.json` by hand; the `repo` MCP tools (`mcp__repo__ticket_*`) are listed in this audit's session, the product MCP `semio` failed to connect (`CONNECT_TIMEOUT`) and is not needed for the ticket. Retry `ticket_reopen` with an explicit path before falling back; `ticket_close` needs an explicit path, a files array starting with one ASCII path, and `📌️important/📝️.md` emptied last. Stop the four guards and any serve at close; delete `🗑️generated/` but keep input scripts and `.md` reports.

## 0.1 Brief template for every successor (paste, then add the WP's §7 block)

1. You are a SESSION-4 SUCCESSOR; session 3's agents do not resolve. Read `📌️important/📝️.md` (rules 1–33 plus the coordinator's Session 4 addendum, rules 34–38 added 02:00: successor/report rule, VERIFY -> FIX -> CLOSE, disk-critical build-dir rule, unchanged build gate, activation/describe/schema generate/channel bump = coordinator only), `📋️design.md`, the LAST section of your report, and your §7 block of this sheet.
2. Rule 28: `git diff HEAD --stat -- <your paths>` (changes are staged; plain `git diff` is empty), repair half-edits compile-atomically, append `## Session 4 — 2026-10-04` to the SAME report, keep it current at every milestone (usage cuts come without warning). No sub-agents, no tickets/goals, no `run_in_background`/Monitor, no describe/activate/serve/channel-bump/schema-generate (coordinator-owned, §3).
3. First re-check your crates (`cargo check -p <crate> --lib`) — a peer is mid-refactor (value/DSL/pack/ValueError); name the peer file + error and continue source work when red is not yours.
4. Run your owed list in §2 order (one gated cargo at a time, private target `target-nde-s4-<wp>`, `RUST_MIN_STACK`/`component-app-assembly` notes in §2); never claim green without a count.
5. Report: final message = report section, exact commands with pass/fail counts, coordinator actions (rule 24).

## Top blockers (also in the chat reply)

1. Nothing core was run since 06:29 10-03 except the 11:2x–12:06 kernel/replication/tool-machine/TS runs: **almost every Rust change of session 3 is SCU** (plugin, ✏️s plugin crates, wgpu renderer native test target). Acceptance (audit X-1) = the owed lists in §2.
2. **Stdio peer migration** (ValueError/sqlite-snapshot, 11 stdio crates red: epw, step, gif, avi, jpg, mp4, wav, zip, mp3, svg, gltf) blocks every plugin crate that links stdio (puzzle, flow, cad, gen2d/3d, dag, energy, forms, raster, …). INFRA sweeps only after the peer is quiet ≥ 30 min.
3. **No live proof exists for the current build**: all live evidence is Run 2 (React, 09-30). Activation attempts 2–24 failed on peer breaks; wgpu never ran live; steps 10–17 of the probe (G6 dial/log, keep-editing, warning flow, tablet, long history, two peers) never ran on any renderer.
4. **Final channel-bump wave not done** (AppFrame tag 15 + `TransactionProposal.description`, `AppCommand::LoadDocument`, `AppChannelClient.loadDocument`, store `.description` fields); `verify history-closure` stays red (`coalesce-key 14`, `bracket-verb 11`) until it lands with descriptors.
5. **§20.15 composed-content conversion**: 92 `parentLeafReadsChild` findings; only sequence is converted; flow partly (`FlowMutation {}` on disk), wires NOT started, mathematical design-only, dag/procedure/playbook/cad/jack/din18599 not started; AGNOSTIC W-a/W-b serializer + inference seams not started.
6. **O(document) publication regression** (puzzle 3d 22 vs 1590 units per mutation) — fix by W2A written, still unmeasured (needs puzzle 3d to build).
7. **Runtime audit majors open**: W2A-1 (O(history) `HistoryView`), W2A-2 (L4 config rows are history rows; energy L4 law red), W2A-5, W2A-6 (sync `load_document_*` still in `PluginApp`: 96 sites/56 files by the 12:10 census, 118/60 by the audit's count), W1G-2 (8 duplicated quadratic `ValidateEditPair` initializers).
8. **Central regenerations still owed** (76 uncatalogued/malformed leaf rows, graph catalog `contractId`/`policy` x8, ~31 plugin `describe`s + dev copies, launch.json, `check-generated`) — see §3.
9. **Raster `editPixels`/`editMask` still on disk** (decided delete), stdio patch leaves lack third-party oracle rows (47), stdio >1 MiB inverse missing, wgpu live gumball/paint unproven.
10. **Disk 27 GiB + active Codex peer** (value/DSL/pack extraction) — expect red trees; the coordinator owns the "TREE GREEN" signal.

## 1. Per-WP table (28 session-3 WPs + 4 audits; details in §7)

Report files are in `T`. "Last section" = the final heading of the report (what a successor reads first). Counts are the last green numbers quoted by the report.

| # | WP | Report → last section | State | Owed verification (headline; exact commands in §7 and §2) | Open items to implement (headline) | Coordinator actions requested | Last peer blockers |
|---|---|---|---|---|---|---|---|
| 1 | S3-INFRA | `📓️s2-infra-report.md` → `### S3-15. 10:42–11:20 …` | partly DONE-VERIFIED / SCU | `cargo test -p semio-framework-os-kernel --lib -- hygienic_`; process3d `-- example`; `✏️s --workspace --lib --keep-going` | stdio ValueError sweep (15 crates); `import()`/`createRequire` discovery; wgpu ownership law red (grammar peer) | none (tree-green signal) | stdio sqlite peer; store peer; process3d `✏️editor/🦀️.rs:312` E0061 |
| 2 | S3-PUZZLE | `📓️w3-t-puzzle-report.md` → `### S3.7 Open items and coordinator actions` | SCU (3d every-example 4/4; 3d suite 899/9 on 10-02) | puzzle 3d/5d/2d `--features component-app-assembly --lib`; 5d wasip2 check; `os-infinite -- world_gumball` | O(document) publication red (-> W1G); `an_id_only_announcement…` isolation; 4 wall-clock laws | describe + re-activate puzzle; central `schema generate` | `✏️s` stdio crates |
| 3 | S3-W2A | `📓️w2-a-report.md` → `### 9.9 F7 context menus, archive load lifts head-only` | SCU (partly verified: `-- history_patch history_notices history_edit` 10/10, time-travel 14/14, plugin check native+wasip2) | plugin `--lib --tests` + the §9.6 law list; puzzle 2d `history_edit_runtime_tests`; 2-instance cold-pair law | W2A-1,2,3(driver),5,6,7–14; child-member row paging | bump wave (LoadDocument); activation | os-kernel red 11:20 (store; fixed by W1G) |
| 4 | S3-W1G | `📓️w1-g-report.md` → `### Audit majors, 10-03` | partly DONE-VERIFIED (kernel 92/92 N17+§15+per-viewer; lib 1254/2) | O(document) measurement (puzzle 3d); plugin suite; FU4 hub bin; replication | W1G-2 (8 `ValidateEditPair` copies), W1G-4..10, ring-stride prefix fold, `.description` store fields | none | `✏️s` not building (stdio) |
| 5 | S3-W1E | `📓️w1-e-report.md` → `### S3.12 x-semio-ui.optionSource` | SC, partly verified (UI lib 769/2 peer) | `-p semio-framework --lib -- mutation_inputs number_facets history_edit_actions dialog_choices`; plugin `-- time_travel`; puzzle 2d `select_tool_history` | W1E-1/2/3 a11y (visible reason, selected state, N17 live announcement), W1E-5 layering | puzzle 2d re-activation | peer UI wgpu tests (2); stdio |
| 6 | S3-W2B | `📓️w2-b-report.md` → `### S3.14 HistoryPatch.editCount` | **DONE-VERIFIED** (128/128, PluginRuntime 135/135, related 1201/14 peer) | none | `requestMediaFrames` host cancel; live probe verdicts | `schema generate` (folder leaf, band corpus), React re-activation | `World3dHost :2225` tsc (peer) |
| 7 | S3-W2C | `📓️w2-c-report.md` → `### S3.16 After TREE GREEN (11:38)` | SCU native (wasm32 check green 10:45) | `renderer-wgpu --lib --no-run` + 52+9 law filters; wasm32 check; vitest browser 92/92 | shared `DocumentArchiveLoadHost` adoption unchecked; reprojection announcement; marketplace reasons | one activation + serve of 6112 | `os-infinite` `DagExpandedPaths` (peer, 12:04) |
| 8 | S3-W2D | `📓️w2-d-report.md` → `### S3.6 … (wgpu rotate ring)` | core VERIFIED (55/0, 9/0, 41), puzzle crates SCU | puzzle 2d full lib + 2 fill laws; 5d filters; wasip2 check | none new | describe+activate puzzle; wgpu re-activation; `schema generate` scope `editor/select-tool-history` | stdio gltf/zip/svg |
| 9 | S3-AGNOSTIC | `📓️s2-agnostic-report.md` → `### S3.12 W-a plan …` | G7 DONE-VERIFIED; G8 reader/gates verified; G12 + reload law SCU; W-a NOT-STARTED | G12 batches `🧪️s3-agnostic-run-acceptance.sh`; strict gates | **W-a serializer/inference seams**, W-b producers (-> LOAD), 92 `parentLeafReadsChild` backlog | `schema generate`; "✏️S GREEN" | ✏️s red |
| 10 | S3-DRAW | `📓️w3-t-draw-note-report.md` → `### S3.8 Open items and coordinator actions` | SCU (leaf oracle tests 5/0, TS 280/2 pre-existing) | draw + note `--lib`; renderer `--tests`; hub wasm32 | none | re-activate draw/note | stdio zip/svg |
| 11 | S3-SPATIAL | `📓️w3-t-spatial-report.md` → `### S3.6 Audit fixes` | core VERIFIED (os-infinite 42/1 peer), plugin crates SCU | fem 2d/3d, lowpoly, shooting `--lib`; `-- use_selection selection_value reference_chips`; wasip2 | S3 twin fem playback clocks | `schema generate` + describes + re-activation | stdio (all fem/lowpoly/shooting) |
| 12 | S3-FLOWCAD | `📓️w3-t-flow-cad-report.md` → `### S3.8 §20.15 …` (ends at a plan, no verify line) | SC (§12/N6/F1–F6); **§20.15 flow IN-PROGRESS** (`pub enum FlowMutation {}` on disk), cad NOT-STARTED | the 6-item list S3.6 (cad/flow `--lib --tests`, plugin, os-infinite, wasip2) | finish flow §20.15 deletions; cad §20.15 (8 findings) | surface + flow-core wasm; describe flow/cad; taxonomy dirs | stdio zip/step/svg/gltf |
| 13 | S3-LAYOUT | `📓️w3-t-layout-report.md` → `### S3.10 After TREE GREEN (core)` | SCU (React 107, Python corpora, standalone paint-proof 6/0) | layout `--lib`; renderer `-- canvas2d`; hub wasm32 | TAX: register `🧭️gumball`, move overlay | `schema generate`, describe, launch rows, re-activate 6079/6179 | os-kernel IoError (since settled) |
| 14 | S3-CONTROLS | `📓️w3-t2-controls-report.md` → `### S3.12 Playbook on the child lane (design)` | glue VERIFIED (tool-machine 33/33, TS 34/34, plugin `tool_machine::` 39/4); plugin crates SCU; **playbook NOT-STARTED** | energy fixture writer + lib; forms `field_transactions`; gis/playbook/norm contract; wasip2 | playbook §20.15 conversion; CLOSURE-5 `TransactionRef` uniqueness | `schema generate` (13 energy leaves), describes | stdio epw/zip |
| 15 | S3-TEXT | `📓️w3-t2-text-report.md` → `### S3.5 Coordinator actions and peer breaks` | SC; TS/Python verified; Rust owed; **jack §20.15 NOT-STARTED** | writer `--lib`; trinity rewriting `--features component-app-assembly --lib`; stdio md/html/txt/binary/deflate; wgpu `text_editor` | jack §20.15 (8); `restore-working-edges`; trinity `wordOnlyFloat` x6 | `schema generate`, describes, frame-worker regen | typed `workingGraph` peer migration |
| 16 | S3-STROKES | `📓️w3-t2-strokes-report.md` → `### S3.10 Gates to 0 …` | SC; TS verified, raster/remodel cargo NEVER run | raster `--lib` + `--ignored emit_committed_fixtures`; remodel; wgpu `paint2d`; ui-scene | **delete `editPixels`/`editMask`** (still on disk); K3 derive | `schema generate` (4 raster leaves), describe raster/remodel | stdio gif/jpg/png/svg/tiff/bmp |
| 17 | S3-PROCEDURAL | `📓️w3-t2-procedural-report.md` → `### S3.8 Coordinator actions` | SC; engine 67/0 verified; rest SCU | gen2d/gen3d lib + DEV `generation3d-app-laws`; os-flow lib; wasip2 | example normalization (§20.9 `[DEBUG]` printers), live gumball re-anchor check | `schema generate` (1 leaf), describe procedural | stdio zip/gltf/svg |
| 18 | S3-GRAPHS | `📓️w3-t2-graphs-report.md` → `### S3.4 Coordinator actions` | sequence §20.15 SC; shared helpers verified; dag never compiled | dag/wires/math/seq/space cargo + wasip2; root `tool-machine --lib` | dag/procedure §20.15 + graph `drag-nodes`; **sequence drag no transaction row**; sqlite native limits | describe 5; `test inventory`; graph catalog migration | dag depends on red stdio svg/zip |
| 19 | S3-WIRES | **no report** (launched 12:03, `🗑️generated/s3-wires/` empty) | **NOT-STARTED** (disk: `WiresMutation` still has 12 parent variants) | all | whole §20.15 conversion for wires | `schema generate` (2 wires leaves) | — |
| 20 | S3-MATH | `📓️s3-math-report.md` → `### 1. Design note …` (sections 2–5 "in progress") | **DESIGN-ONLY** (model (a) approved) | all | whole §20.15 conversion for mathematical | describe + inventory | — |
| 21 | S3-E2E | `📓️w3-e2e-report.md` → `### S3.7 Run 4 — results` ("Pending: no serve") | Phase A DONE (written, type-clean); **Phase B NEVER RUN** | `verify time-travel` batches A–H on 6012 then 6112 | everything live | activation + serve (main session), launch.json regen | no serve |
| 22 | S3-CLOSURE | `📓️s3-closure-report.md` → `### TREE GREEN core run (10-03 11:39–12:03)` | SC; core VERIFIED (kernel 1254/2); plugin layer SCU; audit says "not done" | plugin lib tests, 79+ plugin wasm32 checks, L3/L4, `verify history-closure` | bump wave; store `.description` wave; CLOSURE-4/5 | **bump wave (§3.3)**, describe 4 bracket descriptors | plugin test target (cleared 12:05) |
| 23 | S3-CODES-TAX | `📓️s3-codes-tax-report.md` → `### Resume 3 (10-03 11:50)` | DONE-VERIFIED (TS/library/replication 260/0); ✏️s rename compile owed | `✏️s --workspace --lib --keep-going`; S2.7 reruns | assign 2 named owners; retire 2 stale fixtures | rebuild of all components (API change) | stdio sqlite |
| 24 | S3-NORM | `📓️s3-norm-report.md` → `## 8. Files` | SC; oracle 15/15 1197/1197, wire-twins 1901/1901; cargo NEVER run | norm contract + 15 crates `--lib`; wasip2; parity exhaustive x15; inventory | diff-schema repair (separate WP); din18599 §20.15 | describe norm, `schema generate`, 2 launch rows | contract crate (fixed 11:14) |
| 25 | S3-STDIO | `📓️s3-stdio-report.md` → `### S3.6 Coordinator actions` | SC; contract 89/0, patch TS 34/0, gates 0 | 11 red crates + dependents `--lib` (`component-app-assembly`); inherited S2 case verification (wasm32 check, 10 crates `--lib`, oracle crate, `🧪️w3-stdio-run-cases.sh`) | **oracle+feature row per new patch leaf (47)**; >1 MiB inverse; STDIO-B/WP-3 leftovers | `schema generate` (49 leaves), describe all stdio | stdio sqlite peer |
| 26 | S3-GATES | `📓️s3-gates-report.md` → `### 7. Coordinator actions` (§5, §6, §7 "Pending") | **IN-PROGRESS** (§1–4 done) | `verify taxonomy report` on new dirs; Rust warnings; layering re-run | docstring-emoji scope decision; 17 peer deps | `schema generate` (76 rows) | none |
| 27 | S3-LOAD | `📓️s3-load-report.md` → `### 8.2 Host router keeps a composed document's children` | kernel driver + TS twin DONE-VERIFIED; MCP + `🏃️run` SCU | `os-run`/`os-mcp` check + tests | W-b producers (guest dependency-key validation refuses `child:<slot>/<id>`); T2/T3 test sweep onto `artifact_app_laws::load_document`; delete LoadDocument | bump wave | `🏃️run`/MCP dependency reds |
| 28 | S3-NOTICES | `📓️s3-notices-report.md` → `### Coordinator actions` | mechanism DONE-VERIFIED native; wasm32 JS path SCU | wgpu wasm32 check + wider suites | per-plugin notice debt (1778 anonymous faults…) | describe procedural; framework table decision | `PropertyBag` graph peer (cleared) |
| A1 | S3-GAP (audit) | `📓️s3-gap.md` → `## (d) What a "done" ticket still needs` | done | — | N1–N18 (§6) | — | — |
| A2 | S3-CLOSURE-CENSUS (audit) | `📓️s3-closure-census.md` | done (input to CLOSURE) | — | — | — | — |
| A3 | S3-AUDIT-CORE | `📓️audit-s3-core.md` → `## 6. Order of work` | done | — | W2A-1..14, W1G-1..10, W1E-1..7, CLOSURE-1..5 (§6) | — | — |
| A4 | S3-AUDIT-TOOLS | `📓️audit-s3-tools.md` → `## Fix order` | done | — | X1–X5, F/C/S/K/T/P/L/D/G/Z (§6) | — | — |

Not launched (listed in session 3 plans only): audit wave B, W3-G beyond S3-GATES, W3-R audits. `S3-EVIDENCE` (queued name, status L360/L375/L380) was split into S3-NORM + S3-STDIO and never existed as its own WP. Extra note on disk: `📓️w2a-sync-load-paths.md` (12:10, W2A/LOAD split of the synchronous-load census; folded into §7 S3-W2A and S3-LOAD).

## 2. Consolidated owed runs, crate by crate (one integrator, one gated cargo at a time)

Rules that apply to every row (`📌️important/📝️.md` 11–15, 30, 33): gate `until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 14 ]; do sleep 30; done`; one command, `--message-format=short`, `CARGO_INCREMENTAL=0` for `test`,
private `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s4-<wp>`; plugin crates live in `✏️s/Cargo.toml` (`--manifest-path ✏️s/Cargo.toml`), hub crates in `🌎️hub/Cargo.toml`; wrap runs > 10 min as `cmd > T/🗑️generated/s4-<wp>/<name>.txt 2>&1; echo exit=$?`;
`RUST_MIN_STACK=67108864` for renderer-wgpu/puzzle 2d binaries, `268435456` for kernel `retained_clone_tests`; editor modules compile only with `--features component-app-assembly` (stdio, puzzle, trinity, procedural). Always re-run `cargo check` of the crate first (tree is churned by a peer).

### 2.1 Order of work (dependency order)

0. Tree gate: `cargo check -p semio-framework-os-kernel -p semio-framework-schema -p semio-framework-plugin --lib --keep-going` (root), then `cargo check --manifest-path ✏️s/Cargo.toml --workspace --lib --keep-going` (stdio sweep decides).
1. Core framework (root workspace) — kernel, replication, plugin, framework manifest, time-travel, tool-machine, UI, renderer (table 2.2).
2. Plugin crates (✏️s) — in the order puzzle 2d/3d/5d -> draw/note/layout -> flow/cad -> dag/wires/mathematical/sequence/space/procedure -> gen2d/gen3d -> forms/gis/energy/playbook -> raster/remodel/wfc/process3d -> fem/lowpoly/shooting -> writer/trinity/stdio -> norm (table 2.3).
3. Hub wasm32 checks (table 2.4), TS/bun/vitest (2.5), Python (2.6), strict gates (2.7).
4. Only then: the final regeneration wave (§3) and activation/live probe.

### 2.2 Root workspace (cwd repo root)

| Crate / package | Commands (verbatim from the reports) | Depending WPs | Last known |
|---|---|---|---|
| `semio-framework-os-kernel` | `cargo check -p semio-framework-os-kernel --lib`; `cargo test -p semio-framework-os-kernel --lib` (all) ; filters `-- tool_transaction_tests deferred_reprojection_tests supersede_replay_tests dispatch_group_stamps_one_tool_transaction os_spr::history::tests`; per-test `os_store:: os_vcs:: os_spr::` (`RUST_MIN_STACK=268435456`); `-- os_spr::channel::`; `-- the_document_archive_load_host`; `-- persisted_messages_admit_exactly_the_outcome_vocabulary`; `-- fault_params`; `-- hygienic_ carrier_record` | W1G, CLOSURE, LOAD, CODES-TAX, NOTICES, INFRA, W2A | lib 1254 ok/2 FAIL (1 CLOSURE fixed to `>= 14` + PASS, 1 peer space-history sqlite); 92/92 N17/§15/per-viewer; per-test 820/16 peer `durable_group`; channel 93/0 |
| `semio-framework-plugin` | `cargo check -p semio-framework-plugin --lib --tests` (+ `--features artifact-app-testing`); `… --lib --target wasm32-wasip2`; `cargo test -p semio-framework-plugin --lib` full, filters: `time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body activated_tool_factory document_archive composed_child_history scrub history_ bounded_reload` (W2A), `-- time_travel` (W1E), `-- tool_machine:: tool_run_tests::` (CONTROLS), `-- typing tool_machine` (TEXT), `-- composed_child_history node_graph_delete_row time_travel scrub` (FLOWCAD), `-- use_selection selection_value reference_chips` (SPATIAL), `-- command_rejection_tests time_travel history_code` (CODES-TAX), `-- the_draft_editor_keys` | W2A, W1G, W1E, W2B, CLOSURE, CONTROLS, TEXT, FLOWCAD, SPATIAL, CODES-TAX, AGNOSTIC | lib check green 11:05/11:38; `tool_machine::` 39 ok/4 baseline FAIL; time_travel 51/1 (10-01); 943/16 per-test (10-02 12:30) |
| `semio-framework` | `cargo test -p semio-framework --lib -- history_patch history_notices history_edit` (10/10); `-- number_facets history_edit_actions dialog_choices mutation_inputs`; `-- every_corpus_case` (2/0); `-- fault_notice` (4/4); `-- gumball_verb_audience` (1/1); `--features typegen exports_typescript_bindings` (`SEMIO_TYPEGEN_OUT`; 1/1) | W2A, W1E, AGNOSTIC, NOTICES, SPATIAL, CLOSURE | as listed |
| `semio-framework-time-travel` | `cargo test -p semio-framework-time-travel` | W2A, FLOWCAD | 14/14 (06:32) |
| `semio-framework-tool-machine` | `cargo test -p semio-framework-tool-machine --lib` (root workspace only: tests need `serde_json`/`blake3`) | CONTROLS, TEXT, GRAPHS | 33/33 (11:20) |
| `semio-framework-replication` | `cargo test -p semio-framework-replication --lib` | W1G, CODES-TAX | 260/0 (11:25) |
| `semio-framework-ui` | `cargo test -p semio-framework-ui --features testkit --lib`; `-- conformance_corpus presence_bar peer_notes`; `-- ui_node_wire_format`; `--features wgpu --lib -- paint2d scene_records`; `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features wgpu-engine` | W1E, W2C, STROKES | 769 ok/2 peer; 16/16 |
| `semio-framework-ui-scene` | `cargo test -p semio-framework-ui-scene --lib -- text_splice` (10/0); `--lib` (paint-2d lane contract; blocked by peer test `scenes-value-round-trip/🦀️.rs:41`) | TEXT, STROKES | — |
| `semio-framework-os-renderer-wgpu` | `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=… cargo test -p semio-framework-os-renderer-wgpu --lib --no-run`; run with filters `time_travel local_folders introspection_tests dialog_choices` (52 + 9 new), `chrome_overlays_tour_tests board2d_engine_tests board_presence_tests agent_overlays_tests shortcuts_palette window_actions_search_panes panel_anchor_model hub_projection_workspace canvas_presence`, `context_menu window_lifecycle navbar_footer shell_input ui_command_wiring scenes::`, `-- canvas2d`, `-- text_editor`, `-- paint2d`, `-- board2d`, `-- fault_notice history_lane_refusal dispatch_faults_are_classified history_edit_refusal a_failed_dispatch uncorrelated_answer`; `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` and `--lib --target wasm32-unknown-unknown` | W2C, W2D, DRAW, LAYOUT, TEXT, STROKES, NOTICES, W1E | 52/52 + 177/178 (10-02), board2d 9/0, notices 6/6; wasm32 exit 0 (10:45) |
| `semio-framework-os-infinite` | `-- gumball paint cancel` (42/1 peer); `-- directed_normal` (55/0); `-- node_graph_edit_rows wire_edit`; `-- world_gumball`; `-- gumball paint world` (30 peer scene-bridge failures) | SPATIAL, W2D, FLOWCAD, PUZZLE | as listed |
| `semio-framework-os-flow` / `-os-flow-core` / `semio-framework-surface` | `cargo test -p semio-framework-os-flow --lib` (+ `-- flow_vcs`); `cargo check -p semio-framework-os-infinite -p semio-framework-surface -p semio-framework-os-flow -p semio-framework-time-travel -p semio-framework-plugin -p semio-framework-os-renderer-wgpu --lib` | PROCEDURAL, FLOWCAD, CLOSURE | check 0 errors (06:00) |
| `semio-framework-os-config`, `-plugin-host`, `-os-run`, `-os-mcp` | `cargo test -p semio-framework-os-config --lib -- local_folder` (20/20) and `-- the_folder_is_edited`; `cargo test -p semio-framework-plugin-host --lib -- artifact_inference_router` (3/0); `cargo check -p semio-framework-os-run --lib --bins --tests` then `cargo test -p semio-framework-os-run --lib`; `cargo check -p semio-framework-os-mcp --lib --tests` then `… -- long::a_long_history_document_loads_through_polls quick::an_abandoned` + test target `binding-cancellation-law` | W2B, LOAD | run/mcp UNRUN |
| `semio-framework-dsl`, `-diagnostic`, `-pixels`, neural engine, infinite dag, workflow | `cargo test -p semio-framework-dsl --test carrier-record-lists` (2/0); `-p semio-framework-diagnostic --lib` (11/11); `-p semio-framework-pixels -p semio-framework-ui --lib` (58/58); `-p semio-framework-os-kernel-neural-engine --lib` (67/0); `-p semio-framework-artifact-infinite-dag --lib` (`bundled_demo_fixture_is_canonical`); `-p semio-framework-artifact-workflow-workflow --tests` | INFRA, NOTICES, STROKES, PROCEDURAL, GRAPHS | as listed |

### 2.3 Plugin crates (`--manifest-path ✏️s/Cargo.toml`, private target per WP)

| Crate(s) | Commands | WPs |
|---|---|---|
| `semio-s-artifact-puzzle-2d/-3d/-5d` | `cargo test … -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib` (3d first; 5d `-- a_board_gesture_drag board_node_delete apply_board_events language_neutral_fixtures`; 2d `-- every_registered_example dsl_asset_parses`, `select_tool`, `select_tool_history`, `history_edit_runtime_tests`, fill laws `board_fill_job_large_host_has_no_step_at_or_above_eight_ms`, `fill_run_job_drive_step_stays_below_the_interactive_ceiling_for_nakagin`); `cargo check … -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --lib`; `cargo check … --target wasm32-wasip2 -p semio-s-plugin-puzzle` | PUZZLE, W2D, W1E, W2A |
| `…draw-drawing`, `…note-note` | `cargo test … -p semio-s-artifact-draw-drawing --lib`; `… -p semio-s-artifact-note-note --lib` | DRAW |
| `…layout-layout`, `…fem-2d/-3d`, `…lowpoly-lowpoly`, `…shooting-shooting` | `cargo test -j 2 … --no-fail-fast --lib -p semio-s-artifact-layout-layout -p semio-s-artifact-lowpoly-lowpoly`; fem/lowpoly/shooting `--features component-app-assembly --lib --no-fail-fast`; fem 2d `-- gumball` | LAYOUT, SPATIAL, CODES-TAX |
| `…cad-cad`, `…flow-flow` | `cargo check … -p semio-s-artifact-cad-cad -p semio-s-artifact-flow-flow --lib --tests --keep-going`; `cargo test … -p …cad-cad --lib` (baseline 459/2); `… -p …flow-flow --lib` | FLOWCAD |
| `…dag-dag`, `…mathematical-equation`, `…sequence-sequence`, `…reasoning-wires`, `…imperative-procedure`, hub space | `cargo check … -p semio-s-artifact-{dag-dag,mathematical-equation,sequence-sequence} --lib --tests --keep-going` (+wires, procedure); then `cargo test --lib` each; `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-space --lib --tests` | GRAPHS (WIRES, MATH) |
| `…procedural-generation2d/3d`, `semio-s-composition-laws`, DEV | `cargo check -j 4 … -p …generation2d -p …generation3d --features <both>/component-app-assembly --lib --tests --keep-going`; `cargo test --lib` each; DEV `--test generation3d-app-laws`, `--test generation2d-example-export` | PROCEDURAL |
| `…forms`, gis terrain/map, `…energy-model`, `…playbook` | `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test … -p semio-s-artifact-energy-model --lib -- writes_the_committed_vector_when_requested`; `cargo test … -p …energy-model --lib`; forms `-- field_transactions change_block_field block_field try_value`; gis/playbook crate tests | CONTROLS |
| `…raster-raster`, `…remodel-remodeling`, wfc 2d/3d/bitmap, process3d | `cargo test … -p semio-s-artifact-raster-raster --lib`; `… --lib -- --ignored emit_committed_fixtures`; remodel `cargo check … -p semio-s-artifact-remodel-remodeling --lib --tests` + lib; `cargo test … -p semio-s-artifact-wfc-bitmap`; process3d `-- example` (blocked `✏️editor/🦀️.rs:312` E0061) | STROKES, CODES-TAX, INFRA |
| `…writer-writer`, `…trinity-rewriting`, stdio md/html/txt/binary/deflate | `cargo test … -p semio-s-artifact-writer-writer --lib`; `… -p semio-s-artifact-trinity-rewriting --features component-app-assembly --lib`; `… -p semio-s-artifact-stdio-md -p …-html -p …-binary -p …-deflate -p …-txt --features <each>/component-app-assembly --lib` | TEXT |
| stdio non-text (11 red + dependents) | `cargo test … --lib` for epw, step, gif, avi, jpg, mp4, wav, zip, mp3, svg, gltf + xlsx, docx, pptx, ifc, bcf, semio, png, dwg, dxf, obj, ply, stl, tiff, xml, json, pdf, csv, las, tsv, bmp; media `-p semio-s-artifact-stdio-{png,jpg,wav,tiff,mp4,gif,mp3,avi,bmp,pptx}`; `-p semio-s-artifact-stdio-gltf -p …-zip -p …-wfc-bitmap` | STDIO, CODES-TAX |
| norm | `cargo test … -p semio-s-artifact-norm-contract --lib --test config_mutation`; `--lib` for all 15 artifact crates; `cargo check … -p …norm-contract -p …en1995 -p …en1998 --lib` | NORM |
| whole workspace | `cargo check --manifest-path ✏️s/Cargo.toml --workspace --lib --keep-going`; every plugin `cargo check … --target wasm32-wasip2 --lib` (79 crates + `semio-hub-space` per CLOSURE) | INFRA, CODES-TAX, CLOSURE |

### 2.4 Hub workspace (`🌎️hub/Cargo.toml`)

`cargo test --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml -p semio-hub --bin os-hub` and `-p semio-hub --lib` (FU4; 247/8 at S2, none W1G); wasm32-wasip2 checks: `semio-hub-{draw,note,layout,space,dag,mathematical,sequence,reasoning,raster,remodel,wfc,process}` (DRAW, LAYOUT, GRAPHS, STROKES).

### 2.5 TypeScript / bun / vitest

| Package (cwd) | Command | WPs | Last |
|---|---|---|---|
| os TS (`OSM/…`) | `bun ./📜️script.ts test-store-oracles` (7/7); `bun ./📜️script.ts test-channel-oracles` (3/226 expects); `bun ./📜️script.ts test 🏪️store/👷️worker` (18/0); `bun ./📜️script.ts test quick` (476) | W1G, LOAD | green |
| replication TS | `bun ./📜️script.ts test --reporter=verbose` with `SEMIO_VITEST_POLICY=repositoryVitestPolicyV1(cwd)`, `SEMIO_TEST_BUDGET_MS=400000` | W1G | 20/21 (R-1) |
| kernel TS / FWT | kernel vitest (75); FWT TS conformance (20): `bun test …/⏪️time-travel/🧪️tests/🧪️conformance/🟦️.ts` | W2A | green |
| tool-machine / text-splice | `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` (33/34); `bun test ./🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🟦️.test.ts` (69) | CONTROLS, TEXT | green |
| manifest | `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🟦️.ts` (92); `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts` (5) | W1E, AGNOSTIC, NOTICES | green |
| renderer-react | `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts <patterns>`: W2B `⏪️time-travel/🧪️tests/🧩️component 🧪️staged-arg-controls 📎️local-folders/🧪️tests 🧪️command-rejection 🪟️spawned-program-session` (128); `Canvas2dHost` (107); `🖌️Paint2dHost` (63); `engine-contract` pattern lists (PUZZLE 5, W2D 41); `🧪️fault-notices` | W2B, LAYOUT, STROKES, PUZZLE, W2D, NOTICES | green |
| wgpu TS | `bun ./📜️script.ts test-browser 🧪️tests/♿️wgpu-accessibility-interaction/🟦️.ts 🧪️tests/📨️browser-frame-transport/🟦️.ts 🧪️tests/🧪️wgpu-backbone-folder-door/🟦️.ts` (92); `bun ./📜️script.ts test-preview-generated` (27/29); `bun ./📜️script.ts check-browser-worker` | W2C | re-run after INFRA profile fix |
| root / library | `bun test ./🧪️tests/🧪️outcome-law-gate/🟦️.ts` (26); `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️owned-script-routes/🟦️.ts` (4) and `🧪️script-async-runners` (2); library laws (use absolute or `./`-prefixed paths) | CODES-TAX, INFRA | green |
| stdio contract / norm / forms | `bun test ./✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🧪️tests/🟦️.test.ts` (34); `bun test ./✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts` (1901); `bun T/🧪️s2-norm-ts-twins.ts --check` | STDIO, NORM | green |
| e2e | `bun ./📜️script.ts verify time-travel …` batches A–H (see §7 S3-E2E) | E2E | never run |

### 2.6 Python / ticket oracles (all `--check`-able, idempotent)

`python3 T/🧪️s3-load-archive-host.py` (12/0); `.venv/bin/python T/🧪️s3-layout-path-paint-corpus.py --check`; `… T/🧪️s2-layout-gumball-dispatch-corpus.py --check`; `… T/🧪️w3-t-layout-author-vectors.py`; `python3 T/🧪️s3-graphs-dag-oracle-vectors.py` (34/34 + 24/24); `python3 T/🧪️s3-puzzle-brace-record-lists.py --check <7 example DSLs>`; puzzle `select-tool-history/🐍️.py` (4 scenarios) and `history-edit-runtime/🐍️.py` (5 scenarios); `T/🧪️s3-norm-*.py --check` (0 pending), `T/🧪️s2-norm-wire-order.py`; `bun T/🧪️s3-gates-source-rules.ts`.

### 2.7 Strict gates and repo verifies (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` unless root)

`bun ./📜️script.ts schema mutation-labels|mutation-editability|mutation-payloads|mutation-inputs --json` (AFTER 10-03: labels 0; editability 92 `parentLeafReadsChild` (0 before the §20.15 rule); payloads 32 at GATES' read, 4 after STROKES (raster unwitnessed); inputs 251 at GATES' read, ~78 central-generate rows after STROKES: leafUncatalogued 73 + malformed 4 + trinity refUnresolved 1); `bun ./📜️script.ts schema fault-notices [--census]`; root `bun ./📜️script.ts verify mutation-outcome-law` (passed), `verify history-closure --json` (red: coalesce-key 14, bracket-verb 11), `verify taxonomy report --scope <dir>` (per new directory; GATES §5 never ran), `verify dependencies [literal-external|parity js]`, `verify layering`, `verify debug-tags`, `verify docstrings emoji-first`, `bun ./📜️script.ts test outcome-law-gate`, `bun ./📜️script.ts test fault-notices-gate`.

## 3. Final regeneration wave (coordinator-owned; do NOT start before §2 is green — fleet rules 23 and 27)

Order matters: constants + frame code (3.3) -> central schema catalog (3.1) -> graph catalogs (3.2) -> wasm/frame-worker artifacts (3.4) -> `describe` per plugin (3.5) -> registry/launch.json + `check-generated` (3.6) -> inventories (3.7) -> activation + serve + live probe (3.8). Reason: descriptors embed payload schemas and the channel version (CLOSURE step 8, W2A), the launch rows come from nx targets, the live probe needs everything.

### 3.1 Central `schema generate` (command: `bun ./📜️script.ts schema generate`; regenerates `📚️library/🔣️schema-catalog.json` + `📓️schema-catalog.md`; after it every `schema mutation-inputs` finding of class `leafUncatalogued`/`malformed`/`refUnresolved` must be 0)

Contents the reports name (~78 rows + hash-only staleness):
- **New leaves (uncatalogued):** stdio 49 `…/mutation/patch-snapshot/schema.json` (+ 40 patch leaves of other formats noted by TEXT: docx, dwg, dxf, epw, gif, ifc, mp3, pptx, semio/v1/*, step, stl, tsv, xlsx); energy 13 (`change-site-{latitude,longitude,elevation,time-zone,north-axis}`, `change-ground-temperature-{building-surface,shallow,deep}`, `change-run-period-{start-month,start-day,end-month,end-day,year}`); trinity rewriting 4 (`delete-working-nodes`, `connect-working-ports`, `disconnect-working-edges`, `add-working-node`); raster 4 (`fill-region`, `apply-filter`, `transform-image`, `fill-selection`); reasoning wires 2 (`move-nodes`, `set-node-positions`); gen3d 1 (`change-widget-input`); fem 2d 1 (`set-playback-clock`); puzzle `🔣️selection-time-travel` schemas + changed 3d leaf descriptions; os.config `attach-local-folder` payload schema + time-travel band corpus schema; layout `CH/🧬️schema/🔣️path-paint`, `🔣️gumball-meta`, `🔣️gumball-dispatch`; scope `editor/select-tool-history` (`ED/🧬️schema/🔣️select-tool-history/🔣️.json`).
- **Malformed (rows of deleted leaves):** energy `update-site`, `update-ground-temperature`, `update-run-period`; shooting `set-camera-draft-label`; deleted sequence leaves/subsets (8 leaves + 2 subsets) and 4 stale case names in central `📚️library/🔣️taxonomy.json` (coordinator regenerate); deleted flow parent leaves when FLOWCAD finishes §20.15.
- **Unresolved ref:** trinity `change-parameter-binding /newValue` -> `framework/graph/manifest/property-value.json` must be published into the catalog documents (new uncommitted `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🌱️value/🧬️schema/🔣️.json`).
- **Hash-only staleness:** norm (en1995 `change-member-role`/`-support`; din18599, vdi3805, en1997 snapshot schemas; en1997 artifact schema; the 20 renamed leaf paths hand-rewritten by NORM), remodel schemas, 62 leaf schemas that gained `x-semio-inverse-rows`.
- **Verify after:** `bun ./📜️script.ts schema mutation-inputs --json` -> 0 `leafUncatalogued`; `schema mutation-payloads` -> 0 `unwitnessed` once raster quintets are emitted (`--ignored emit_committed_fixtures`).

### 3.2 Graph catalog migration (command per plugin: nx target `graph-generate` -> `bun ./📜️script.ts graph-generate` in the plugin package dir)

The 8 `🛂️manifest/📇️outputs.json` of draw, puzzle 2d/3d/5d, writer, wires, trinity rewriting/jack lack `contractId`/`policy`, so `graph generate` refuses ("missing or unknown fields" — GRAPHS S3.4); migrate the 8 rows by hand/script, then regenerate (the wires output already equals the current emitter). Also: the dag/wires/mathematical `test inventory` (3.7).

### 3.3 Channel-bump wave (design §20.7; CLOSURE `#### Final channel-bump wave` L188–209; LOAD §5/§6; W2A §9.6)

One wave, one `APP_CHANNEL_VERSION` bump (currently 20), TS twin, regenerated frame-worker, rebuild + describe of EVERY plugin component:

1. `OSM/📡️spr/🧵️channel/🦀️.rs`: delete `coalesce_key` from `AppFrame::TransactionProposal` (frame tag 15) — enum field, encoder arm destructure + `write_str(coalesce_key)`, decoder `coalesce_key: read_str(..)`; ALSO delete `AppFrame::TransactionProposal.description` (frame sends `""` today; `TransactionProposalDraft.description` already gone); bump `CHANNEL_VERSION` (`:26`).
2. Channel unit tests (`🧪️tests/🔬️unit/🦀️.rs` ≈471, ≈869) drop `coalesce_key`/`description` from both frame literals; the pin `channel_version_matches_the_shared_cross_language_pin` reads `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/🔖️channel-version.json` (`channelVersion: 20`) — bump the JSON in the same edit (the other fixtures in that dir, `📨️app-frame-transaction.json`, `🧾️app-command-transaction.json`, `🔀️app-command-merge.json`, `🚪️app-command-opening.json`, contain no `coalesce`/`LoadDocument` text as of 2026-10-04; re-check the golden byte lengths after the field deletions).
3. `OSM/🔌️plugin/🦀️.rs` `plugin_exchange` (≈42595): drop `coalesce_key: String::new()` (+ `description`) from the `AppFrame::TransactionProposal` literal.
4. TS `🧰️framework/🛍️products/💻️os/🟦️.ts`: type field (≈2705), `writeStr(out, frame.transactionProposal.coalesce_key)` (≈3414), decoder (≈3584–3586), `APP_CHANNEL_VERSION` (`:3724`).
5. `🧪️backbone-envelope-io/🟦️.ts` ≈524, ≈603, ≈804: drop `coalesce_key` from the three frame literals (+ byte-length expectations: one zero-length varint per frame).
6. **`AppCommand::LoadDocument`** (W2A `📓️api-stepped-document-load.md` §5; no sender remains — LOAD §5): variant `📡️spr/🧵️channel/🦀️.rs` (≈2528) + field-page arms (≈1978/1981/2007/2010/2154/2347) + encoder (≈3267/3403) + decoder tag 6 (≈3655/3791; tag 6 stays unassigned, no renumbering) + docs; channel tests (≈98, ≈632, ≈810, golden `060101010102`); guest arm `🔌️plugin/🦀️.rs` (≈43798/43966); exhaustive seq arms `🏃️run/🦀️.rs:2260`, `🌉️mcp/🏠️workspace/🦀️.rs:2179`; TS union (≈2617), tag (≈2896), encoder (≈2959–2963), decoder (≈3149–3153), `sendCommand` document cache (≈4055), **`AppChannelClient.loadDocument`** (≈4192–4194); `🧪️backbone-envelope-io` callers (≈1336, ≈1448, ≈1454, ≈1486, 467/665/686); wgpu `🐚️plugin-bridge/🟦️.ts:1921` (W2C already moved to the archive load).
7. **Store wave** (with W1G): delete `Edit.description` (store, `.spr` `F_EDIT_DESCRIPTION`, `.ops`, canonical digest), `ArtifactCommand::Apply.description`, `GroupMeta.description`, `begin_*apply_batch(description)`, the agent `transaction_commit` label (only writer left).
8. Registry/descriptor constant: `OSM/🔌️plugin/📇️registry/🧬️schema/🔣️.json` `appChannelVersion` const (≈L664); every `🌎️hub/🧩️compositions/*/🔣️.json` descriptor (written by `describe`).
9. Optional (needs the bump, W2A residual): a per-lifetime `ColdPairIngressStatus` list on `TurnResult` (pooled-actor answer channel).
10. After the wave: `bun ./📜️script.ts verify history-closure --json` must show `coalesce-key 0`; `bracket-verb 0` after 3.5. Persistence changes already made (no channel impact): `.spr` `HistoryEdit` presence bit 2 refused, `.ops` edit field `key` gone, `ArtifactCommand` binary ordinals 8/12 unassigned.

### 3.4 Generated artifacts

- Frame worker + browser boot (wgpu): `nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker` and `…:generate-browser-boot --excludeTaskDependencies` (INFRA: exit 0 at 11:03; needed again for the bump and for TEXT's `host-page-hidden` message / `semioWgpuHostPageHidden` door). Source of truth for imports: wgpu `sourceModulePaths` + `frameWorkerSources` (INFRA S3-7/S3-15).
- `framework-surface` wasm bindings (new `GraphSession.takeGraphEditsJson`): `nx run @semio-tech/framework-surface-rs:wasm` (`bun ./📜️script.ts wasm` in `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust`); flow-core wasm (`pointerUpScreen` / `alignSelection` answer rows only): `nx run semio-framework-os-flow-core:wasm`; puzzle `BoardSession.setHighlightedIdsJson` rides the puzzle plugin activation.
- Manifest typegen (hand-synced `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts`): re-check with `cargo test -p semio-framework --lib --features typegen exports_typescript_bindings` (`SEMIO_TYPEGEN_OUT`); `bun nx run @semio-tech/dsl-derive-rs:generate` for `✨️derive/🔣️mutation-authority.json` (STDIO regenerated 06:45; re-check fresh).
- Every plugin component rebuild (CODES-TAX: `OutcomeCode`/`warning` API reaches every component on the next activation chain; CLOSURE: derived footprints).

### 3.5 `describe` per plugin (command `bun nx run @semio-tech/<plugin>-plugin:describe`, e.g. `bun nx run @semio-tech/puzzle-plugin:describe`; also the dev copies `OSM/🧑‍💻dev/🔌️plugin-modules/*` and `🌎️hub/🧩️compositions/*/🔣️.json` + `🛂️.descriptor.semio`)

puzzle (stale `transformBegin/End`, `paintStrokeBegin/End`; new selection leaf kinds), draw, note, layout, flow (`addWidget` `label/action/format`; `nodeGraphEdit` rows), cad, fem, lowpoly, shooting (`setCameraDraftLabel` gone), demonstrator, energy, playbook, forms (`change-block-field`), procedural (gen3d `change-widget-input`, `channel` optionSource, 12 `faultNotices`, gen2d rows), writer, trinity (rewriting + jack), stdio (all formats; "Load example" verb label), dag, mathematical (`addNode`), sequence, space, reasoning (wires 2 leaves), raster (`setFillTolerance`, `applyFilter`, `transformImage`, `fillSelection`, `paintStroke{phase,reason,gesture}`, `paintBucket`), remodel, wfc x5, process3d, norm (`🌎️hub/🧩️compositions/📕️norm/🛂️.descriptor.semio` still names old en1991 kind + 20 pre-§3.6 leaf dirs), plus the four with stale bracket verbs (demonstrator, fem, lowpoly, puzzle). No describe is needed for W1E/W2B/W2C/W2D beyond the puzzle ones listed.

### 3.6 Registry, `.vscode/launch.json`, `check-generated`

- `bun nx run @semio-tech/plugin-registry:generate` (regenerates catalog + `.vscode/launch.json` from `.vscode/🧩️launch.seed.jsonc` + playground registry + every nx target; never hand-edit launch.json) then `bun nx run @semio-tech/plugin-registry:check-generated` (also `preview-generated`).
- Rows that must exist afterwards: `⚖️gate⏪️time-travel⚛️react` (411.2781, :6012) and `⚖️gate⏪️time-travel🧊️wgpu` (411.2782, :6112) [seed rows by E2E]; `⚖️test-channel-oracles💻️os🟦️` (900.03555, `4_gate`) [LOAD]; `test-schema-fault-notices`, `test-schema-fault-notices-census`, `test-fault-notices-gate` (900.04775 / 900.04901 / 900.04902) [NOTICES hand-added to launch.json — add to the seed or let the generator derive them from the nx targets, else they vanish]; `test-outcome-law-gate` (CODES-TAX, in seed); `verify layout-frame-selection` target (missing in LA `📋️project.json`) + the two layout Python corpus scripts [LAYOUT]; `bun nx run @semio-tech/norm-js:test-oracle-source` and `…:test-wire-twins` [NORM]; keep `test-store-oracles` [W1G]; remove the dead sequence generator rows (`@semio-tech/fixture-generator-sequence-1-any`, 2 launch options) [GRAPHS].

### 3.7 Inventories and parity

`bun ./📜️script.ts test inventory --artifact s.dag.dag --standard 1 --subset any`; `… --artifact s.mathematical.equation … graph`; `… --artifact s.reasoning.wires … any` (clears `manifest-only-mutation`); norm `bun ./📜️script.ts inventory --artifact s.norm.<a> --standard 1` per artifact (builds `🏭️bridge`), `bun ./📜️script.ts parity exhaustive --case <c>` x15 (cwd `🧪️test`) and `bun ./📜️script.ts test parity exhaustive --case mutate-dag-1` / `mutate-wires-1`; `bun ./📜️script.ts test contract exhaustive --case mutate-dag-1`.

### 3.8 Activation, serve, live probe

`nx run-many -t activate-puzzle2d-react-dev activate-puzzle2d-wgpu-dev -p @semio-tech/framework-os-dev` (or `🔁️activate-retry.sh`; precheck = `cargo metadata --locked` in `✏️s` + `🌎️hub` + kernel/schema check, gate rustc < 30); serve from the MAIN session with `🔁️serve-supervisor.sh` (`S_OS_PORT=6012 S_OS_RENDERER=react` and `6112` + `wgpu`; `setopt no_bg_nice; nohup … & disown`; `curl` -> 200); also fem/lowpoly/shooting/draw/note/layout (6079/6179, 6064/6164)/flow/procedural lanes for the live consumer checks. Then S3-E2E Run 4 (§7).

### 3.9 Also owed from the registry / checks in the wave

`bun ./📜️script.ts verify history-closure --json`; `bun ./📜️script.ts verify dependencies` (ratchet: 17 peer additions flagged for their owners); `schema fault-notices` (decide the framework table for guest-raised `app.command.*` / `mutation.target-*`); wire `policyHistoryClosureBreaches` is already in `runGate` (red until 3.3 + 3.5).

## 4. Open design items not yet implemented (owner = the successor WP that must take it)

| # | Item | Owner WP | Evidence line |
|---|---|---|---|
| D1 | **§20.15 composed content only on the child lane — conversions per plugin.** sequence: DONE in source (`SequenceMutation` uninhabited, drag via `Emit::node_drag_child`). flow: IN-PROGRESS (parent vocabulary `pub enum FlowMutation {}` already on disk; the 10 parent leaves, `🌊️mutate-flow-1`, parent one-item preparation, `flow_content_edit`/`apply_flow_mutation`, subset oracle catalog still to delete + law). cad (8 `cad_pane_local_scene` x5, `cad_selection_inverse_objects` x3; 22 refs on disk 2026-10-04), dag (17), procedure/imperative (4), wires (12), mathematical (16, model (a) approved: parent-owned `graph`/`geometry`, derived content-addressed children, `move-points`), playbook (8, flow child as single source of truth, delete `document` slot), jack (8), din18599 (1) — all NOT-STARTED; needs a graph `drag-nodes {targets,dx,dy}` child leaf first (dag/wires/procedure). One `composed_reload_law!` per plugin. | FLOWCAD (flow, cad), GRAPHS (dag, procedure, graph `drag-nodes`), WIRES, MATH, CONTROLS (playbook), TEXT (jack), NORM (din18599), AGNOSTIC (law) | gate `schema mutation-editability` = 92 `parentLeafReadsChild` (`📓️s2-agnostic-report.md` S3.10 L462); `📓️w3-t-flow-cad-report.md` S3.8; `📓️w3-t2-graphs-report.md` S3.1b; `📓️s3-math-report.md` §1; `📓️w3-t2-controls-report.md` S3.12; `📓️status.md` L673/L678 |
| D2 | **W-a serializer/inference seams**: `Serializer<S>::serialize(from, &ArchiveChildren)` over 76 io impls + 140 registrations + 142 call sites; merge `ArtifactSerializer`(41)+`serializer_entry_of`(67) into it; delete `ArtifactInferrer` (112 marker impls; gltf `🧊️gltf/🦀️.rs:214` -> `protocol::Inference::infer`). **W-b producers**: guest `validate_wire_request_resources` (`PLG` ≈2123) runs `ArtifactIdentity::parse` on every dependency key, so `child:<slot>/<childId>` requests are REFUSED; no requester holds child head packs (propose `PluginApp::child_head_packs()` + host `ReadChildHeads`); MCP `infer_real` sends `Vec::new()`; the guest default `export_media("artifact:out")` must emit the composed carrier `encode_document_archive_bytes({parent HEAD pack, empty spr, members})`. | AGNOSTIC (W-a, guest validation), LOAD (W-b) | `📓️s2-agnostic-report.md` S3.11–S3.12; `📓️s3-load-report.md` §8.1 |
| D3 | **Stdio > 1 MiB removal/replace has no exact inverse** (`inverse_snapshot_patch` refuses -> empty inverse; `SNAPSHOT_PATCH_MAX_BYTES` 1 MiB vs 16 MiB sources): chunked or blob-referenced inverse; undo must never be lossy | STDIO | `📓️s3-stdio-report.md` S3.5 item 2; `📓️status.md` L599 |
| D4 | **Third-party oracle arm + `🥒️.feature` Examples row for each of the 47 new `patch-snapshot` leaves** (AGENTS: one language-agnostic test + one third-party library output per feature) | STDIO | `📓️s3-stdio-report.md` S3.5 item 1 |
| D5 | **Delete raster `editPixels`/`editMask`** (no host dispatches them any more; renumber binary command variants, re-seal fixtures, describe): `…/🖨️raster/…/✏️editor/🎮️commands/{🎨️edit-pixels,🖌️edit-mask}` + rows + tests + `🖼️assets/🔄️replacement` test + `🧫️fixtures/🔏️publication-authority/🔣️.json` still present 2026-10-04 | STROKES | `📓️status.md` L682; `📓️w3-t2-strokes-report.md` S3.10 end |
| D6 | **Sequence node drag writes no transaction row** (`a_node_drag_record_is_one_child_transaction`: step moves, 0 rows; hypotheses: empty authoring seed in the retained step, empty `artifact_mutations` at `drag_transaction`, composite publication dropping the parent's `transaction`) | GRAPHS | `📓️w3-t2-graphs-report.md` S3.3 last bullet |
| D7 | **O(document) publication gate**: puzzle 3d `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` 22 vs 1590 units; W2A's `PUBLICATION_RETURNED_ROOT_ALLOWANCE = 1` is unmeasured and only amortises (audit W2A-5: streamed ticks still pay the drain; use size-bounded retirement per publication) | W1G (measure) + W2A (gate) + PUZZLE | `📓️w1-g-report.md` "O(document) publication regression"; `📓️audit-s3-core.md` W2A-5; `📓️w3-t-puzzle-report.md` S3.3 |
| D8 | **Cold-pair per-instance slots**: serialization + supersession + answer precedence landed in source (W2A 9.9); turn-level two-instance law still owed (needs a cold-pair header builder over a real pack in the native lifecycle harness); per-lifetime status list on `TurnResult` for pooled actors needs the bump wave; host cold-pair settle has no wall deadline/backoff (W1G-10) | W2A, W1G | `📓️w2-a-report.md` §9.9 last bullet; `📓️audit-s3-core.md` W2A-4, W1G-10 |
| D9 | **Runtime audit majors**: W2A-1 incremental `HistoryView` + per-edit slice reads; W2A-2 L4 predicate (`history_row_is_recorded` drops config-only edits; invert 3 laws; energy L4 law counts ALL rows); W2A-3 wall-deadline driver for `step_reprojection(deadline_us)`; W2A-6 remove `load_document_*` from `PluginApp` (96 sites/56 files at 12:10 -> `artifact_app_laws::load_document`; live sync path P5 `consume_media` default `🔌️plugin/🦀️.rs` ≈14718 first), delete `hydrate_document_lane`/`resolve_ready` panic path; W2A-7..14 minors | W2A | `📓️audit-s3-core.md` §1; `📓️w2a-sync-load-paths.md` |
| D10 | **Store audit**: W1G-2 one framework `ValidateEdit` helper replaces 8 plugin copies of `ValidateEditPair` (writer, gen2d, gen3d, gismap, process3d, jack, drawing, raster) + coverage-gate phase names (`🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts` ≈713/782/879/939/1409); W1G-4 `FoldSupersessions`/`CloneInitial` O(history) steps; W1G-5 three §15 law scenarios; W1G-6 N17 language-agnostic corpus + third-party oracle + TS twin; W1G-7 `adopt_pending` drops waiting remote transitions; W1G-8 one-op identity per route; W1G-9 `.spr` `REC_VIEWER` round trip; ring-stride prefix fold (`📓️api-deferred-history-replays.md` §4) | W1G | `📓️audit-s3-core.md` §2 |
| D11 | **Closure remainder**: bump wave + store `.description` wave (§3.3); CLOSURE-4 property test per `perTarget` leaf at cap+1 + localized `mutation.too-large` en/de (raw English `plugin_sdk_fault` today; 87 hand aggregates default to 1 inverse row); CLOSURE-5 `TransactionRef` unique per press (per-instance monotonic `logical`) | CLOSURE, CONTROLS | `📓️audit-s3-core.md` §4 |
| D12 | **Press transactionality**: a release refused downstream of settlement (store admission) surfaces as a Fault with both lanes unpublished — a two-phase release through the publication ladder is not built | CONTROLS | `📓️w3-t2-controls-report.md` S3.10 note |
| D13 | **Child-member history rows beyond a member's projection are not paged**; severity `worst` computed over projected rows only (cap 32 + <=32 flagged) | W2A | `📓️audit-s3-core.md` W2A-11; `📓️w2-a-report.md` §9.2 N1 open item |
| D14 | **Twin fem playback clocks** -> one `FemPlaybackClock` + `FemResultsWindowTransient` + `set-playback-clock` in the fem 2d crate + `FemPlaybackTransport` trait; `FemGumballTransient` vs lowpoly paint duplicate runner bookkeeping | SPATIAL | `📓️w3-t-spatial-report.md` S3.6 (S3) |
| D15 | **Trinity**: `restore-working-edges` leaf for exact relative undo of `disconnect-working-edges`/`delete-working-nodes`; `wordOnlyFloat` x6 (jack CameraJson; rewriting snapshot schemas) after the typed-`workingGraph` peer migration is quiet; confirm the peer carried `connect/disconnect/add-working` leaves + inverses | TEXT | `📓️w3-t2-text-report.md` S3.4/S3.5; `📓️s2-agnostic-report.md` S3.8 |
| D16 | **Norm**: diff schemas are not the Rust diff wire for 10 of 15 artifacts (0 % match; Option diff members declared non-nullable) — own WP + witness-test extension; hand-written choice labels in EN 1995/1992/1999 `🏷️field-meta` (second truth); en1999 `support` free-text enum | NORM | `📓️s3-norm-report.md` §6 |
| D17 | **Fault-notice debt**: 1778 anonymous `Fault::from(text)`, 258 codes without notices, 115 malformed codes, 35 framework-namespace guest codes (decide a framework table beside `HISTORY_NOTICE_LABELS`); ticket scope = history-editing/tool refusals (gate scoped mode); repo-wide rest = separate task chip | NOTICES + plugin owners | `📓️s3-notices-report.md` gate counts; `📓️status.md` L689 |
| D18 | **A11y/UX gaps**: W1E-1 disabled row-action reason visible (title/tooltip) on both renderers; W1E-2 `selected` row semantics for long option rows; W1E-3 live announcement of `HistoryPatch.reprojection` in both shells (React `🛠️ShellHelpers/⏪️time-travel`, wgpu `history_lane_notice`); W2A-8/9/13 status lies (paused flag, raw fault code fallback, Edit `Busy`) | W1E, W2A, W2B, W2C | `📓️audit-s3-core.md` W1E-1..3, W2A-8/9/13 |
| D19 | **Live proof**: Run 4 on both renderers/en+de/desktop+phone+tablet; positive presence (⏪ badge, "is editing") needs a hub-backed space serve or a coordinator waiver; the probe predates N1/N17 (W1E-6: derive from `window.total`; add `reprojection` kinds, `history.replaying`, `document.loading`) | E2E | `📓️w3-e2e-report.md` S3.6/S3.7; `📓️s3-gap.md` (c), (d) |
| D20 | **G12 cross-plugin law has not run in session 3** (wired into 118 editor modules / 96 crates / 34 plugins; last real: 9 PASS in S2); extend harness to child-lane (member-store) edits when child vocabularies land | AGNOSTIC | `📓️s2-agnostic-report.md` S3.12 table |
| D21 | **K3**: derive/macro for window-transient partitions (~220 lines hand-written per plugin: flow, fem, remodel, cad, lowpoly); `requestMediaFrames` host cancel (neither shell); `FemGumballTransient` etc. | W2A (K3), W2B/W2C (media) | `📓️audit-s3-tools.md` K3; `📓️w2-b-report.md` S3.10 |
| D22 | **Undo/redo/checkout/supersede still fold the log once per step** (not O(change)); `reproject` after a batched publication folds once | W1G | `📓️w1-g-report.md` W1G-3 "Not O(change) yet" |
| D23 | **Housekeeping decisions**: docstring-emoji uniqueness scope (2532 repeats, no gate); 17 unapproved peer dependency additions; `js:xstate` oracle conflict (ui-react production); `design.md` §20 has two items numbered `4.` (the second is §19.4 — audit X5); live-only P5: React must not re-anchor the gumball on shape answers | GATES, coordinator | `📓️s3-gates-report.md` §1/§3; `📓️audit-s3-tools.md` X5, P5 |
| D24 | **Per-plugin leftovers**: gen3d §20.9 example normalization (28 operators, 2 gen2d; `[DEBUG]` printers still in tree until run); flow DEFAULT document not self-describing (`🌊️flow/…/📸️snapshot/🦀️.rs:276`); layout TAX move of `🟦️GumballOverlay.tsx` to `🧭️gumball` (needs taxonomy registration); sequence dead `🏭️generator`; raster stale placeholder `📌️.empty.md` in the composite window transient dir; remodel `Binary64Transport` ref spelling | PROCEDURAL, FLOWCAD, LAYOUT+TAX, GRAPHS, STROKES | respective reports (§7) |

## 5. Goal conformance matrix

Evidence levels: **LIVE-VERIFIED** (an e2e run proved it; the only live run on disk is **Run 2**, React, en + de, 09-30 build "activation #4": 85 PASS / 3 FAIL each, 0 uncaught, 0 hard faults; dirs `T/🗑️generated/e2e/probe-2026-09-30T16-49-01` (en) and `…16-51-11` (de); report `📓️w3-e2e-report.md` L247–335 — everything landed since is law-level only, so "LIVE" below always means "stale build"); **UNIT-VERIFIED** (named law/test + count a report saw pass); **WRITTEN-ONLY** (source/law written, never run or never run green); **MISSING**. wgpu has NEVER run live (no serve on 6112; probe step 10–17 never ran on any renderer).
Symbols (line numbers drift, checked 2026-10-04 01:5x): `STORE` `effective_forwards`@23238, `state_before`@19103, `begin_report_replay`@18997, `ReplayMode`@23484, `EditReplay`@23535, `ArtifactCommand::Supersede`@3088, `CreateAlternativeWithSupersede`@3095, `defer_local_replays`@20857; `TT` `begin_refusal`@489, `history_outcome_introduced`@975, `time_travel_selection_value`@1408, `commit_time_travel`@2381, `step_time_travel_replay`@2673; `FWT` stage machine @113; manifest `HISTORY_EDIT_ACTION_IDS`@2676, `mutation_input_defs`@1248, `number_facets`@1128, dialog `finalizeHistoryEdit`@2714; `blocks_finalize` `FW/📡️replication/⚔️conflict/🦀️.rs`@453; `TransactionRef` `FW/📡️replication/🎮️mutation/🦀️.rs`@1533, `OutcomeCode`@1025; `InputReplacement`/`TransitionSupersede` `FW/📡️replication/🔗️causal/🔀️transition/🦀️.rs`@58/73; tool machine `ToolTransaction`@167, `ToolMachineRunner`@287, `ScrubMachine`@552, `TypingMachine`@1173; `PLG` `commit_transaction`@13154, `tool_intent_kinds`@14084, `fault_notices`@14091.

| # | Goal sentence | Mechanism (file / symbol) | React | wgpu | Gap to close (owner) |
|---|---|---|---|---|---|
| 1 | Non-destructive edit of ANY history mutation | `HistoryTransition::Supersede` (tag 6) + `effective_forwards` at every fold site; `ArtifactCommand::Supersede`; `editable = !viewer && !foreign && input_schema().is_some()` (`TT` `history_mutation_entry`); gate `schema mutation-editability` | LIVE (Run 2 step 4–5: overwrite finalize, head +120, 0 drift) + UNIT (kernel `supersede_replay_tests` incl. `finished_replays_commit_atomically_and_refuse_stale_or_blocking_ones`, `scoped_supersessions_follow_their_alternative_and_unscoped_ones_every_alternative`: 92/92 on 10-02 19:00; TS store oracles 7/7; replication 260/0) | UNIT (same store/fold code, renderer-agnostic); LIVE MISSING | "any": gate editability 0 at 10-03 07:10, but 92 parent-lane composed-child leaves (§20.15) cannot be edited on reloaded/decoded documents; reload law written, unrun (D1, AGNOSTIC/GRAPHS/…) |
| 2 | Time-travel mode on edit | `TimeTravelSession` reducer (`FW/⏪️time-travel/🦀️.rs`), runtime freeze + `render_snapshot_or` seam (`TT`), reserved verbs (12), React band `RE/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`, wgpu `RE/🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🦀️.rs` | LIVE (Run 2 step 3: en+de 11/11, band `role=status` polite, stage `editing`, indicator on all 3 windows) + UNIT (`cargo test -p semio-framework-time-travel` 14/14 incl. `begin_is_refused_exactly_where_the_reducer_refuses_it`; FWT TS 20; React time-travel component suite 128/128) | UNIT (wgpu `time_travel …` 52/52, 10-02 11:22); 9 later laws WRITTEN-ONLY; LIVE MISSING | live wgpu (E2E); re-run plugin `time_travel` laws (W2A) |
| 3 | Edited mutation shown with downstream NOT applied | preview = `state_before` + draft (`STORE` `state_before` + prefix ring; `TT` `shown()`); rows flagged `pending` ("Not applied while editing") | LIVE (Run 2 step 3, board position checked, downstream unapplied) + UNIT (store `state_before` ring law 12/12 at 10-01; kernel 92/92 since) | UNIT (laws 52/52; `dumpBoard2d` incl. `highlighted`); LIVE MISSING | W1G-3 changed the accessors: re-run store + plugin laws; live non-board windows |
| 4 | Accept / discard | verbs `historyEditAccept` / `historyEditDiscard` (`HISTORY_EDIT_ACTION_IDS`), lifecycle fixture `🧫️lifecycle-law`, React controls, wgpu `…/⏪️time-travel/🦀️.rs` | Accept LIVE (Run 2 step 4: chord/button -> `ready` ~100 ms; Exit 0 drift, steps 4/8); Discard UNIT (lifecycle fixture + xstate oracle) / live WRITTEN-ONLY (probe step 11) | UNIT (wgpu keyboard-only laws 52/52); LIVE MISSING | probe step 11 (E2E) |
| 5 | Input UI metadata: slider / stepper / min / max / snaps | `x-semio-ui` + `mutation_input_defs` (Rust + TS twin), `ActionArgDef::number_facets` / TS `actionArgNumberFacets`, `snapSource`, `optionSource`, editor arms in `TT` (Slider/Dial/Stepper/Number/Toggle/Select/Vector/Color/Reference/Text), React `StagedNumberField` (`🛠️ShellHelpers`) + §18 `uiNumberFieldKey`, wgpu `staged_arg_row` | LIVE only as a stepper (Run 2 steps 3–4: `step="1"` = grid snap, typed 120, Arrow +-1); UNIT: UI contract 215+12+1, ui-react 78, number-keyboard-law (W2B 128/128), `🧫️number-facets` corpus bun 13/13, mutation-inputs bun 92 + Python 216/42; dial/log/hard-min/vector LIVE = WRITTEN-ONLY (probe steps 8, 10) | UNIT 52/52 (10-02); new `every_staged_number_row_carries_the_shared_corpus_facets` + `…the_guest_editor_offers_list_and_chip_edits_within_their_bounds` WRITTEN-ONLY; LIVE MISSING | strict `schema mutation-inputs`: ~78 central-generate rows + `wordOnlyFloat` x6 trinity (§3.1); live dial/log/vector both shells |
| 6 | Accept replays all downstream | `begin_report_replay`, `ReplayMode`/`EditReplay` (Report mode), `step_time_travel_replay` (4 ms turn slices), N17 deferred local/remote steps (`defer_local_replays`, `ReplayTurnBudget`), `HistoryPatch.reprojection {kind: remote\|step\|load}` | LIVE (Run 2 step 4: head +120, downstream re-applied) + UNIT (kernel deferred-remote 4 + N17 local-step 3 x 240 mutations: 92/92 10-02 19:00; replication 260/0) | UNIT mirror laws WRITTEN-ONLY (`a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror`); LIVE MISSING | long history >= 200 live (probe step 16, WRITTEN); W2A-3 wall-deadline driver; plugin laws owed |
| 7 | Success / warning / error outcomes per downstream mutation | `ReplayReport` severities, 9-code `OutcomeCode` (`refuse(code: OutcomeCode …)`), `MutationOutcome::warning`, durable outcomes (`applied_edit_outcomes`), `history_mutation_entry` | LIVE (Run 2 step 8: Error row "Fehler: Ziel fehlt", Finalize disabled) + UNIT (`verify mutation-outcome-law` passed 0 breaches; outcome-law-gate 26/0; kernel `persisted_messages_admit_exactly_the_outcome_vocabulary` 1/0; replication `the_typed_codes_are_the_vocabulary`, `refuse_picks_the_vocabulary_level`) | UNIT (`history_refusals_…` 18-code gate within 52/52); LIVE MISSING | success/info distinct rows live; plugin crates' `refuse` callers compile (`✏️s --workspace`) |
| 8 | Warnings visible in history (and after finalize/reload) | `history_outcome_introduced` (`introduced`), durable outcomes in `.spr`/store, law `a_warning_an_edit_introduces_stays_visible_after_finalize_and_reload` (`PLG/🧪️tests/🧪️time-travel/🦀️.rs`, passed 10-01), puzzle 2d corpus scenario `warning-from-an-upstream-edit` | LIVE only an Error row + a pre-existing Warning after reload (Run 2 finding 8); introduced-warning flow (probe step 13 + `reload`) WRITTEN-ONLY; the 3 Run 2 FAILs (`document-rows-survive-the-reload`, `overwrite-row-survives-the-reload`, `trunk-listed-after-new-alternative`) fixed in source, never re-probed; UNIT 10-01 pass, plugin re-run owed | UNIT (corpus); LIVE MISSING | probe 13/reload both renderers; W2A-2 (config rows are history rows today) |
| 9 | Fatal must be edited first; repeat until clean | `blocks_finalize` (Error+Fatal), `Next problem`, Withdraw, "Use selection" (`time_travel_selection_value`), reference chips + preview highlight (`entity_label`, `InteractionView::draft_references`), puzzle corpus `fatal-loop-by-editing-targets` | LIVE via Withdraw only (Run 2 step 8: 15/15, blocked -> next problem -> withdraw -> ready); edit-targets path WRITTEN-ONLY live (probe step 12); UNIT: engine `draft_referenced_ids_paint_highlighted_without_publishing` (os-infinite `directed_normal` 55/0), React engine-contract 41 incl. highlight forwarding, Python oracle 5 scenarios/20 head nodes; puzzle 2d `history_edit_runtime_tests` WRITTEN, unrun | UNIT: `a_draft_reference_highlight_reaches_the_wgpu_board_engine` (board2d 9/0); Use-selection corpus click WRITTEN; LIVE MISSING | run puzzle 2d lib (`history_edit_runtime_tests`, `select_tool_history`); live step 12 both renderers |
| 10 | Then the final result; finalize or keep editing other mutations | `Begin` legal from `Reviewing`, `TimeTravelPanel.begin_refusal`, `several_drafts_from_a_review_finalize_as_one_overwrite_supersede` | LIVE only from a BLOCKED review (Run 2 step 8: "Accepted changes: 2"); clean-`ready` -> edit another -> finalize once = probe step 11 never run; UNIT: time-travel 14/14, corpus scenario "several drafts -> ONE overwrite + undo/redo" WRITTEN | UNIT; LIVE MISSING | probe step 11; plugin law run |
| 11 | Finalize prompts: new alternative vs overwrite | injected dialog `finalizeHistoryEdit` (choices Overwrite [destructive] / New alternative + name), `commit_time_travel`, `CreateAlternativeWithSupersede` (Branch + scoped Supersede, atomic), per-viewer alternative head | LIVE (Run 2 steps 5 & 7 en+de: Overwrite `data-destructive=true`, default name "Edited history"/"Bearbeiteter Verlauf", alternatives listed + switchable); FAIL `trunk-listed-after-new-alternative` (since fixed by PER-VIEWER, kernel laws pass; never re-probed); R2-6 `commitCheckpoint refused: timeTravel.frozen` fixed in source; UNIT kernel finalize laws (92/92), FLOWCAD §12 alternative-finalize replication fix | UNIT (wgpu keyboard-only `🧪️wgpu-time-travel` law); LIVE MISSING | live re-probe (E2E); FLOWCAD §12 laws need compile |
| 12 | Puzzle 2d drag: selection AND drag offset are editable | leaves `✋️drag-selection` / `rotate-selection` / `scale-selection` (`PZ/🧬️schema/🧬️mutations/…`), select tool = `ToolMachineRunner` (`PZ/…/🖱️select/🦀️.rs`), board gesture record in ONE dispatch (React `RE/🖥️Board2dHost/🟦️.tsx`; wgpu `…/🧊️wgpu/🦀️.rs` `puzzle_board_direct_pointer_into`), chips via `entity_label` | LIVE (Run 2 steps 2–4: exactly one row "Drag 2 items by (80, 40)", dx/dy steppers, targets chips + "Use selection"); rotate/scale/Use-selection live WRITTEN-ONLY (steps 10, 12); UNIT: Python oracle select-tool-history 4 scenarios/12 head nodes, engine 55/0, React coalescing 29 + engine-contract 41; puzzle 2d lib UNRUN in S3 (3d suite 899/9 on 10-02) | UNIT: board2d 9/0 incl. `the_wgpu_rotate_ring_publishes_one_rotate_record` + shared coalescer corpus (core crates only); LIVE MISSING | puzzle 2d/5d/3d lib runs (§2.3); live steps 10, 12 both renderers |
| 13 | Tools are state machines yielding mutations in a transaction | `ToolTransaction`, `ToolMachineRunner`, `ScrubMachine`, `TypingMachine`, runtime `PressLeaf` ledger, `Emit::commit_transaction`, `TransactionRef` stamped on every op, §15 transaction-scoped amend in the store | LIVE only for puzzle 2d (Run 2 step 2: row grouped by `TransactionRef`); UNIT: tool-machine Rust 33/33 + TS 34/34 (Ajv/xstate/fast-check), store §15 8/8 + TS oracles 7/7 + plugin 2/2 (10-02), plugin `tool_machine::` 39 ok / 4 baseline FAIL, typing 33/31; per-plugin conversions SCU (§1) | UNIT: shared coalescer corpus; LIVE MISSING | per-plugin crate runs (§2.3); live gestures per renderer |
| 14 | Tools are NOT history-editable, their mutations are | tool context in window transient (never history); `Emit::amend`/`AmendLast*`/`Edit.coalesce_key`/`UtilityPreviewContract` deleted; gate `verify history-closure` | n/a (same code) UNIT: gate self-test 31/31; closure census `amend-emit 0 · amend-last 0 · preview-contract 0 · edit-literal 0 · footprint-hand 0 · release-plain-commit 0 · host-snapshot-bracket 0`; kernel 1254/2; derived payload laws 10 ok; LIVE MISSING (Run 2 predates CLOSURE) | same; LIVE MISSING | `coalesce-key 14` + `bracket-verb 11` need the bump wave + describes; L4 (config edits never history rows) WRITTEN, energy law red until W2A-2 |
| 15 | Artifact-agnostic | framework-only mechanisms (`FW/⏪️time-travel`, `FW/🛠️tool-machine`, `STORE`, `TT`); labels from `SemanticMutation::label` (`Emit.description`/`Emit::commit` deleted); G7/G8/G12 gates; generic `snapshot_edit_patch`; one Rust-produced `framework.body.history` rendered by both shells | UNIT: G7 `schema mutation-labels` 0 / 3096; G8 reader laws (bun 92, Python 216/42, Rust `every_corpus_case` 2/0); G12 harness: S2 9 PASS (vcs, puzzle 2d, norm en1990, block 2d, note, layout, shooting, generation2d, remodel), **S3: none run** (wired in 118 editor modules / 96 crates / 34 plugins); LIVE only puzzle 2d | same code; LIVE MISSING | G12 batches (`🧪️s3-agnostic-run-acceptance.sh`), §20.15 backlog (D1), W-a/W-b (D2) |

## 6. Audit findings: routed, closed, open (status = end of session 3 per the reports; "src" = source written, not run)

### 6.1 `📓️audit-s3-core.md` (verdicts: W2A, W1G, W1E accept-with-changes; CLOSURE "reject as done"; 0 critical)

| Finding | Routed to | Status at 12:07 10-03 | Remainder |
|---|---|---|---|
| W2A-1 O(history) `HistoryView` per generation | W2A + W1G-3 | store half DONE-VERIFIED (kernel 1254/2; `🧪️tests/⚡️hot-path`: 1000 Applies 0 folds); runtime half OPEN | incremental view + `applied_edit_mutations/outcomes/position` readers (D9) |
| W2A-2 / CLOSURE-2 L4 config rows | W2A predicate + CLOSURE law | decided §20.13; law rewritten (counts ALL rows) = RED by design; predicate OPEN | D9 |
| W2A-3 / W1G-3 wall budget | W1G stepper + W2A driver | stepper src (`ReplayTurnBudget`, `step_reprojection(deadline_us)`); driver OPEN | D9 |
| W2A-4 cold-pair slot | W2A | src (serialization, supersession, precedence); 2-instance law owed | D8 |
| W2A-5 publication gate unmeasured | W1G + W2A | OPEN | D7 |
| W2A-6 sync loads | W2A + LOAD | senders gone (LOAD), archive lifts head-only (W2A 9.9); `PluginApp::load_document_*` + `LoadDocument` OPEN | D9, §3.3 |
| W2A-7..14 (minors) | W2A | OPEN | D9/D18 |
| W1G-1 `[DEBUG]` probes | W1G | **CLOSED** (6 sites deleted; `grep SEMIO_DEBUG_STORE_UNITS` 0) | — |
| W1G-2 8 `ValidateEditPair` copies | W1G + plugin owners | OPEN (not started) | D10 |
| W1G-3 O(E^2) accessors | W1G | DONE-VERIFIED in kernel (accessors, `fold_frontier`, `StackMirrorDirt`, prefix ring, hot-path law) | D22 (undo/redo still folds) |
| W1G-4..10 | W1G | OPEN | D10 |
| W1E-1..7 | W1E (+W2C, W2A) | OPEN minors (W1E-4 wgpu menu reason landed by W2C 06:5x src; W1E-7 rides W2A-7) | D18 |
| CLOSURE-1 not finished | CLOSURE + coordinator | PARTIAL: `Emit.description`/`Emit::commit` deleted, gate in `runGate`; bump wave + descriptors OPEN | §3.3, §3.5 |
| CLOSURE-3 config press loses final value | CONTROLS/CLOSURE | **DONE** src + verified (tool-machine 33/33, TS 34/34; plugin glue law written, `tool_machine::` 39/4) | — |
| CLOSURE-4 footprint hazard; localized `mutation.too-large` | CLOSURE | OPEN | D11 |
| CLOSURE-5 `TransactionRef` uniqueness | CONTROLS | OPEN | D11 |
| X-1 acceptance needs green runs | coordinator | OPEN (§2) | — |

### 6.2 `📓️audit-s3-tools.md` (9 accept-with-changes, 2 accept, 0 reject)

| Finding | Owner | Status | Remainder |
|---|---|---|---|
| X1 almost no session-3 Rust ran | all | OPEN | §2 |
| X2 drag-emit wrappers x12 + clock one-liner x20 | GRAPHS | **CLOSED src** (shared `NodeDragEmit`, `authoring_clock`, `Emit::node_drag_child`; tool-machine + plugin lib 0 errors) | crate runs |
| X3 `[TRACE]` logs | FLOWCAD | **CLOSED src** (dag/wgpu traces deleted) | — |
| X4 core `[DEBUG]` probes | W1G | **CLOSED** | — |
| X5 design.md numbering | coordinator | OPEN (two `4.` under §20) | D23 |
| F1 dag journal drops >64 rows | FLOWCAD | **CLOSED src** (`DAG_GRAPH_EDIT_CAPACITY = 256`, whole-gesture refusal, law `a_two_hundred_node_gesture_journals_whole_and_an_oversized_one_is_refused_whole`) | run |
| F2 second wgpu encoder | FLOWCAD + W2C | **CLOSED src** (`write_dag_graph_edit_rows` one encoder, sinks) | run |
| F3 `Leave`=release | FLOWCAD | **CLOSED src** (`Leave` -> `pointer_cancel_screen`) | run |
| F4 `insertPort.side` hole | FLOWCAD | **CLOSED src** | run |
| F5 wire+drag label | FLOWCAD | **CLOSED src** (`tool_intent_kinds`) | run |
| F6 §12 finalize order/label | W2A/FLOWCAD | **CLOSED src + TT 14/14** (`TimeTravelLabel::MemberEdited`) | — |
| F7 `is_de` menu + empty selection row | W2A | **CLOSED src** (`🖱️context-menu`, ICU oracle 27/0) | Rust run |
| F8 schema typo "bounded to256" | GRAPHS | OPEN (minor; re-seal fixture) | — |
| C1 130 `{}` energy witnesses | CONTROLS | **CLOSED** (payloads 600/600, 301/301 witnessed, Python 26/26) | energy lib run |
| C2 forms frozen-press law / `[DEBUG]` | CONTROLS | **CLOSED src** (harness takes UI-progress frames; `[DEBUG]` gone) | forms run |
| C3 `settle_press_config` Abort arm | CONTROLS | **CLOSED** (superseded by the unified `PressLeaf` ledger) | — |
| C4 `Emit.description` in glue | CLOSURE | **CLOSED** | — |
| S1 wgpu live gumball consumer | SPATIAL | **CLOSED + verified** (`-- gumball paint cancel` 42/1 peer) | live wgpu (E2E) |
| S2 corpus in every consumer | SPATIAL/PROCEDURAL | **CLOSED** (React 13/13, wgpu law, fem 3d, gen3d law src) | gen3d DEV run |
| S3 twin fem clocks | SPATIAL | OPEN | D14 |
| S4 gen3d refusals localized | PROCEDURAL + NOTICES | **CLOSED src** (12 named codes + `gumball_fault_notices`) | run |
| K1 remodel re-decode per tick | STROKES | **CLOSED src** (`RemodelingImport.rolling_scores`, shared blur gate) | remodel run |
| K2 tick 0 over live import | STROKES | **CLOSED src** (`remodeling.import.open`) | run |
| K3 dead scaffolding / window-transient boilerplate | STROKES / W2A | scaffolding **CLOSED**; derive OPEN | D21 |
| K4 wgpu bucket + gesture replay law | STROKES + W2C | **CLOSED src** (laws named in §7); `fill-region` quintets unemitted | raster run + emit |
| K5 §17.2 streamed preview | STROKES | **CLOSED src** (S3.9) | run |
| T1 trinity lost add-node | TEXT | **CLOSED** (`add-working-node`, TS+law) | Rust run |
| T2 whole-graph inverses; length bounds | TEXT | bounds **CLOSED**; `restore-working-edges` OPEN | D15 |
| T3 group-branch revalidation | TEXT/W2A | judged no defect (S3.3); law added | run |
| T4 replace-intent fallbacks law | TEXT/STDIO | **CLOSED src** (laws) | run |
| P1 §19.4 | PROCEDURAL | **CLOSED src** | DEV run |
| P2 phantom channels | PROCEDURAL | **CLOSED src** (§20.9 pure folds, `target-missing`) | run |
| P3 bounds/label truncation | PROCEDURAL | **CLOSED src** | run |
| P4 mesh edits as transactions | PROCEDURAL | **CLOSED src** (one-shot `Scrub` at rest, intent `create-widget`) | run |
| P5 live-only gumball re-anchor | PROCEDURAL | OPEN (live) | D23 |
| L1 wgpu path approximations | LAYOUT | OPEN minor (pin as `approximate:true` corpus cases; `gl-matrix` devDependency; launch target) | §3.6 |
| L2 overlay home | LAYOUT + TAX | OPEN | D24 |
| D1 draw/note runs | DRAW | OPEN (runs) | §2.3 |
| G1 gesture replay laws for drag leaves | GRAPHS | **CLOSED src** (shared replay law: wires, math, trinity, sequence; space blocked) | run |
| G2 wrapper dup | GRAPHS | **CLOSED** (see X2) | — |
| G3 sequence `work_items: 1` | GRAPHS/CLOSURE | moot (sequence parent vocabulary empty) | D6 |
| G4 NODE_GRAPH_EDIT_ROOT_FIELDS opaque `gesture/commit/abort` | GRAPHS | unknown (not reported) | verify |
| Z1 O(document) publication | W1G + PUZZLE | OPEN | D7 |
| Z2 5d/2d runs never executed | PUZZLE | OPEN | §2.3 |
| Z3 `every_registered_example_builds_its_document` x3 copies | INFRA/AGNOSTIC | OPEN (one generic law via G12 harness + nx feature) | — |
| Z4 puzzle local-one-shot corpus case | SPATIAL | OPEN | — |

### 6.3 `📓️s3-gap.md` (18 new gaps N1–N18 + G1–G14 + 14 acceptance items)

| Gap | Routed to | Status at end of S3 |
|---|---|---|
| N1 8-row cap (mutations 9+ unreachable) | W2A | **src** (tree-window paging over ALL mutations; `HISTORY_PANEL_MUTATION_ROWS` deleted); React/wgpu laws written; live probe stale (W1E-6) |
| N2 editor limits (input cap, chips, array add/remove, option cap) | W1E | **src** (`historyEditInput{path, value?, edit?: insert\|remove, generation?}`; windowed rows) |
| N3 generic chip default | W2A + plugins | **src** (generic default `resolve_time_travel_reference_labels`; puzzle 3d/5d, draw, note, layout, spatial hook) |
| N4 row label = first leaf | W2A + PROCEDURAL | **src** (§19.1 `tool_intent_kinds`; flow F5) |
| N5 CLOSURE not started | CLOSURE | **src** (gates 0 except bump items) — bump wave pending |
| N6 React NodeGraph whole-snapshot edits | FLOWCAD + GRAPHS | **src** (`nodeGraphEdit{operations}`, shared decoder `node_graph_edit_rows`, `setHostSnapshot`/`deleteSelection` deleted) |
| N7 writer typing laws red | TEXT | **src** (root cause: `dispatch_emit_inner` ordering); writer lib run owed |
| N8 wgpu typing blur/hidden | TEXT + W2C | **src** (`host-page-hidden`, `end_every_text_editor_typing`); wgpu check green 10:54 |
| N9 wgpu parity (path segments, gumball stream, paint blur) | LAYOUT, SPATIAL, W2C | **src** (all three); live MISSING |
| N10 trinity/procedural whole-graph/operator leaves | TEXT, PROCEDURAL | **src** (`change-widget-input`, `add-working-node` …) |
| N11 G12 harness holes (dag cannot seed, …) | AGNOSTIC | wired into 118 modules; dag seeding gap = §20.15; NO run |
| N12 strict input findings | W2B (os.config), SPATIAL (lowpoly), NORM, STDIO | labels/options fixed (inputs 21 -> remodel/stdio 0; ~78 central-generate rows) |
| N13 DSL brace carriers | INFRA | **CLOSED** (305 carriers, native law 2/0; 12 residual bare plugin assets fixed) |
| N14 probe holes | E2E | **WRITTEN** (never run) |
| N15 Edit enabled in every stage | W2A | **src + law** (`begin_refusal`, `disabled_because`) |
| N16 permanent e2e target | E2E / INFRA | **DONE** (`verify time-travel`, nx target, seed rows 411.2781/2); launch.json regeneration owed |
| N17 sync `dry_run`/`reprojection_replay` | W1G + W2A | store half VERIFIED (92/92), runtime adoption src (`HistoryPatch.reprojection {kind}`), stepped load src |
| N18 roster gaps | coordinator | WPs created in S3; GATES partial; audit wave B, W3-R not launched |
| G1 live evidence on current tree | INFRA, PUZZLE | OPEN (activation attempts 2–24 failed; blockers (a)/(b) fixed, then peer waves) |
| G2 wgpu live | W2C, E2E | OPEN |
| G3 fatal loop by editing | W2A, W2D | VERIFIED-LAWS (core), live OPEN |
| G4 warning flow end to end | W2D, E2E | VERIFIED-LAWS (10-01), live OPEN |
| G5 reload persistence | W2B, W2C, W2A | laws; live strict reload OPEN |
| G6 input metadata rendering | W1E, W2C | React laws verified, wgpu src, live OPEN |
| G7 generic labels | AGNOSTIC | **DONE-VERIFIED** (0 / 3096) |
| G8 editable-everything gate | AGNOSTIC + owners | editability 92 (D1), payloads/inputs per §3.1 |
| G9 long histories | W1G, E2E | store VERIFIED; live OPEN |
| G10 collaboration live | W1G, E2E | laws (PER-VIEWER closed, laws pass); hub crates not re-run; two-peer live = waiver or hub serve |
| G11 tool conversions + central gates | per plugin, CLOSURE | PARTIAL (see §1) |
| G12 cross-plugin harness | AGNOSTIC | wired; no S3 run |
| G13 shell parity / a11y / devices | W2B/W2C/E2E | laws; live OPEN; tablet in scope (decision) |
| G14 blocking rule | — | **VERIFIED** (design §16.1; `blocks_finalize`) |
| (d) done-ticket list | coordinator | live proof, strict gates 0, regenerations (§3), CLOSURE, conversions verified, residual runtime (N1–N4, N8, N15, N17), hygiene (guards, `🗑️generated`, ticket close) |


## 7. Per-WP sections (details for briefing successors; one section per session-3 WP)

Each section: report + last heading, owned paths, state, exact owed verification, open items, coordinator actions, last peer blockers. The follow-ups for S3-WIRES and S3-MATH come right after the S3-GRAPHS block.

### S3-INFRA — build/routing/browser-profile/DSL-hygiene infrastructure

| Field | Value |
|---|---|
| Report | `📓️s2-infra-report.md` (409 lines); last section `### S3-15. 10:42–11:20: workflow green, browser profiles, non-stdio residue sweep, N13 closed` (L378–409) |
| Owns | root `📜️script.ts` routing (`metadata.semio.workspaceCommand` rows), `⚡️caching` nx inputs (`frameWorkerSources`), taxonomy wgpu `sourceModulePaths`, rebuild chain `📇️registry/🔁️rebuild` (+ schema), `🗣️dsl` derive hygiene + native carrier law, `🔁️workflow` plugin root + sqlite file, ValueError-sweep residue crates (wfc engine, cad aec x2, norm contract, space sqlite, block-2d sqlite, block io), `📚️library/🧪️tests/🧱️owned-script-routes` + `🧪️script-async-runners` |
| State | SOURCE-COMPLETE, PARTLY VERIFIED. Verified: owned-routes law 4/0 (S3-14, 278 expects), script-async law 2/0 (S3-11), rebuild schema test 5/0 (S3-6), `cargo test -p semio-framework-dsl --test carrier-record-lists` 2/0 (11:16, 305 carriers), workflow + 6 non-stdio residue crates `cargo check --manifest-path ✏️s/Cargo.toml … --lib` exit 0 (S3-15). Written, unproven: derive-hygiene law test run (died in the 10-02 reboot). |
| Last peer blockers | kernel red 11:19 from store peer (`🏪️store/🦀️.rs:22062` E0061/E0308); stdio sqlite crates (ply/dxf/pdf/gif/epw/bcf/wav/avi/jpg/mp4/mp3/xlsx/docx/pptx/svg) are an ACTIVE peer's migration (wait >= 30 min quiet before sweeping); process3d `✏️editor/🦀️.rs:312` E0061 (peer API change) |

**Owed verification (verbatim from report)**
1. Hygiene law (S3-5/S3-10, never ran to a result): `cargo test -p semio-framework-os-kernel --lib -- hygienic_` (law lives at `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧪️hygienic-bindings/🦀️.rs` + fixture `🧫️fixtures/🧫️hygienic-bindings/🔣️.json`; private target `target-nde-s3-infra`).
2. process3d examples (S3-15): `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-process-process3d --lib -- example` (fails to compile on `✏️editor/🦀️.rs:312` E0061 — peer API change; fix or re-check).
3. Stdio ValueError sweep (S3-13 direction): `cargo check --manifest-path ✏️s/Cargo.toml --workspace --lib --keep-going` (last 10-03 05:50: 16 crates/1148 errors; 11:03: 21 crates/1080 left — non-stdio now 0, stdio = gif, epw, bcf, wav, avi, jpg, mp4, mp3, xlsx, pdf, docx, pptx, ply, dxf, svg).
4. Activation precheck after tree green: `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib --keep-going` then `cargo check -p semio-framework-plugin -p semio-framework --lib --tests`.

**Open items to implement**
- Stdio sqlite ValueError sweep (S3-13 pattern: helpers return `Result<_, ValueError>`, terminals `.map_err(ValueError::into_message)`, no `From<ValueError> for String`) — only when peer files >= 30 min quiet (status 11:03).
- Registry input discovery misses `import()`/`createRequire` (status L674, from CODES-TAX) -> INFRA.
- Peer TS errors watch: `📇️directory/🧪️testkit/📡️client-probe/🟦️.ts:144`, `🔌️plugin/🏗️build/📥️installation/🟦️.ts:51` (status L415); os-infinite `world::tests` 30 failures from the multi-UV/tangent `Mesh3dField` 9->14 peer (S3-13 / status L562).
- `testWgpuGeneratorOwnership` (`⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts`) red: `classifyPackageSource` classifies `binary-adapter` as `implementation` (S3-7) — owner: package-glue grammar peer (not ours), keeps `generate-wgpu` red.
- Grammar docs: `.grammar.semio` files (stdio binary diff `📖️.grammar.semio`) no longer describe braced printer output (S3-4) — grammar peer.
- Optional: dag `slider_field` + `#[dsl(key="field")]` workaround can revert to `field` (S3-5, S3-GRAPHS' call).

**Coordinator actions**: none of its own besides tree-green signalling; browser-profile fix (`generate-browser-boot`, `generate-frame-worker`) already exit 0; process3d fixtures already regenerated and installed (`🖼️assets/🎬️demo`, `🖼️assets/🌲️concrete-forest`, inline plate). Keep disk > 100 GiB free (S3-13 risk note).

---

### S3-PUZZLE — puzzle 2d/3d/5d plugin (data fix, N3 chips + highlight, G7)

| Field | Value |
|---|---|
| Report | `📓️w3-t-puzzle-report.md` (402 lines); last section `### S3.7 Open items and coordinator actions (current)` (L390–402), preceded by `### S3.6 Resume — TREE GREEN (10-03 05:50)` |
| Owns | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}` (leaves, example DSLs, `🧪️every-example` laws, N3 `entity_label` + highlight), React `World3dHost/🟦️.tsx` highlight/owner-pose helpers, `🔬️engine-contract` laws |
| State | SOURCE-COMPLETE-UNVERIFIED (3d every-example law 4/4 verified 10-02 11:53; 3d whole suite 899/9 on 10-02, fixes since unrun; 5d and 2d suites NEVER run; React engine-contract 5/5 passed 19:45) |
| Last peer blockers | `✏️s` kernel red (io/store peers); stdio zip/gltf crates; 10-03 05:48 stdio avi Cargo.toml half-written (fixed 05:50) |

**Owed verification (verbatim, run in this order; gated; `--manifest-path ✏️s/Cargo.toml`; tests with `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s3-puzzle`)**
1. `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib`
2. `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib` (never run; audit Z2)
3. `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -- every_registered_example dsl_asset_parses` (never run; Z2)
4. `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --lib`
5. `cargo test -p semio-framework-os-infinite --lib -- world_gumball` (root workspace)
6. Isolated re-runs of the 3d wall-clock laws and `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`.
Also: `python3 T/🧪️s3-puzzle-brace-record-lists.py --check <7 example DSLs>` (0 bare lists, passed 10:45) and React `cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --run "--testNamePattern=guest-stamped highlight|resolves mesh style by priority|multi-object turn and scale of markers|gumball rotate handles commit rotateSelection|gumball transform preview"` (5 passed 19:45).

**Open items to implement**
- `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` red: 22 vs 1590 publication units (O(document) per mutation) -> S3-W1G (publication gate `publish_mounted_typed_operation_unit`, commit 6f33e313da9).
- `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`: process-global session registry try_lock + byte cap drops standing re-upload set under 900-test parallel load; re-run isolated, else fix (S3.3/S3.5).
- 4 wall-clock laws (8 ms/2 ms ceilings) flaky under peer load (not a code change).
- GATES may drop obsolete `#![allow(unexpected_cfgs)]` in shooting `🎮️commands/🧭️gumball/🦀️.rs:10` and `🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/🦀️.rs:13` once the wasm check is clean.
- Closure items M (Edit literals, 5d `coalesce_key` retire) and F (hand footprints) delegated to CLOSURE (done there per status L658/L650).

**Coordinator actions**: re-describe + re-activate puzzle (descriptor lists deleted `transformBegin`/`transformEnd`, lacks 7 selection leaf kinds); central `schema generate` (`🔣️selection-time-travel` schemas, changed 3d leaf descriptions).

---

### S3-W2A — plugin runtime (time-travel host, history wire, stepped load, F7 menus)

| Field | Value |
|---|---|
| Report | `📓️w2-a-report.md` (1151 lines); last section `### 9.9 F7 context menus, archive load lifts head-only (2026-10-03 11:00–11:40) — SOURCE WRITTEN; ICU oracle green, Rust pending TREE GREEN` (L1093–1151); earlier S3 sections 9.1–9.8 |
| Owns | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (P), `🔌️plugin/⏪️time-travel/🦀️.rs` (TT), new `🔌️plugin/🖱️context-menu/`, `⚛️reactor` (`📥️cold-pair`, `🔄️turn`), `🎠️kernel` HistoryPatch wire (`K/🦀️.rs`, TS twin, schema), `📓️api-stepped-document-load.md`, puzzle 2d `✏️editor/🧫️fixtures/🧫️history-edit-runtime` corpus + `🧪️tests/🧪️history-edit-runtime` |
| State | SOURCE-COMPLETE-UNVERIFIED for everything after 06:29 10-03. Verified: `cargo test -p semio-framework --lib -- history_patch history_notices history_edit` 10/10 (06:04); `cargo check -p semio-framework-plugin --lib` (+ `--target wasm32-wasip2`) Finished 10:50/10:55; `cargo test -p semio-framework-time-travel` 14/14 (06:32); FWT TS conformance 20; kernel vitest 75; context-menu ICU oracle `bun test` 27/0; plugin `--lib --tests` check 0 errors at 11:38 (INFRA) |
| Last peer blockers | os-kernel red 11:20 (`🏪️store/🦀️.rs:17806/17823` `shared_prefix_len`, `:22122` 5->6-arg call — W1G store work, later fixed 11:3x per status L671/L670); plugin TEST target sqlite ValueError (INFRA, fixed 11:38) |

**Owed verification (verbatim; §9.6 order + §9.9)**
1. `cargo check -p semio-framework-plugin --lib --tests` and `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --features component-app-assembly --tests`
2. Plugin laws: `time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body activated_tool_factory document_archive composed_child_history scrub history_ bounded_reload` (incl. new: `a_transaction_row_reads_its_declared_intent_leaf_before_and_after_reload`, `every_mutation_of_a_long_transaction_is_reachable_and_edit_follows_the_begin_law`, `reference_chips_fall_back_to_the_document_name_then_the_kind_and_a_short_id`, `the_catalogue_route_reads_the_framework_label_never_a_hand_written_one`, `an_interior_undo_over_a_long_history_replays_over_turns_and_cancel_leaves_zero_trace`, `switching_to_a_long_alternative_replays_over_turns_and_cancel_leaves_zero_trace`, `a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse`, `a_newer_transfer_of_the_same_lifetime_retires_the_finished_pair_in_bounded_steps`, the 4 `🖱️context-menu` Rust laws)
3. `cargo test -p semio-framework --lib -- history_patch history_notices history_edit`
4. Puzzle 2d laws `history_edit_runtime_tests` (5 scenarios + progress + remote replay)
5. `cargo check --target wasm32-wasip2` of the plugin and the puzzle plugin
6. Full plugin lib suite against the baseline
7. Turn-level two-instance cold-pair law (needs a cold-pair header builder over a real instance pack in the native lifecycle harness) — owed (§9.9 W2A-4 residual)

**Open items to implement (audit `📓️audit-s3-core.md` §1 W2A-1..14 + report)**
- W2A-1 [major] O(history) `HistoryView` rebuild per store generation (`refresh_cache`, `build_history_view`, `history_edit_mutation_views`, `history_mutation_pages`): incremental view + per-edit slice accessor (store half W1G-3 landed 11:22) — runtime half OPEN.
- W2A-2 [major] L4 (§20.13): `history_row_is_recorded` must drop config-only edits (3 existing laws invert: `config_lane_row_reports_itself_applied_so_the_host_can_count_it`, `an_op_less_view_action_is_logged_with_edit_id_none_and_count_one`, runtime-contract) — OPEN (energy L4 law red until this lands, status L660).
- W2A-3 [major] driver: pass wall deadline to `step_reprojection(deadline_us)` (W1G signature landed 12:06); law with a slow counted op — OPEN.
- W2A-4 cold-pair slot: serialization + supersession + answer precedence written (9.9); turn-level two-instance law owed; per-lifetime status list on `TurnResult` needs the bump wave.
- W2A-5 [major] publication gate `PUBLICATION_RETURNED_ROOT_ALLOWANCE = 1` UNMEASURED (needs W1G `b54_measures…`, streamed ticks).
- W2A-6 [major] remaining sync loads: `load_document_pack/text` off `PluginApp` (118 sites/60 files -> `artifact_app_laws::load_document`), `hydrate_document_lane`/`resolve_ready` panic path, `parse_document_pack` folds before initializer; delete `AppCommand::LoadDocument` in the bump wave (17 refs).
- W2A-7..14 minors: memoised previewed value per render; reprojection pause/fault stale + raw-code fallback; Edit `Busy` refusal in `TimeTravelPanel.begin_refusal`; `history_row_window_rows` slicer mirror (placeholder row on miss); N1 caps skew `worst` + member rows not paged; `tool_intent_kinds` tool-id parsing (pass tool id part); `document_loading_refusal` should exempt `Interaction` verbs, emit history event on adoption; RFC 6901 index aliases (`01`).
- Child-member history rows beyond a member's projection are not paged (parent ops only).
- **Synchronous load census (`📓️w2a-sync-load-paths.md`, 12:10, W2A + LOAD split)**: P5 `consume_media` default loads a `Document{schema}` media artifact through `load_document_pack` (`🔌️plugin/🦀️.rs` ≈14718) = a LIVE sync path -> move onto a guest-driven archive load (same reactor queue as checkpoint restore); P4 `PluginApp::load_document_pack/text` + `plugin_load_document_pack/text` (≈14610/14616, 34740/34755, 41610/41730) leave the host surface in the bump wave once tests moved; T1 plugin crate tests (`🧪️time-travel` 6, `🧪️history-label-reload` 2, `🧪️history-edit-acceptance` 2, builder contract 2, `declared_verb_fixture_app` ≈8368) -> helper `artifact_app_laws::load_document(app, &ArtifactPackFiles)` + `load_document_text(app, &ArtifactTextFiles)` (stamp identity, admit as archive load with `members: []`, poll to terminal, acknowledge, answer the archive's fault) — W2A writes the helper; T2 ~45 `✏️s` test files (stdio x17, raster x6, wires x5, fem x4, note x3, cad x3, puzzle 2d `history-edit-runtime` x2 = W2A) and T3 `✏️s/🧑‍💻dev/🧩️composition` tests x2 -> S3-LOAD sweeps (not yet done). Counts: 96 sites in 56 files (tracked-file `git grep`), `AppCommand::LoadDocument` 17 refs.

**Coordinator actions**: activation for live proof; bump wave must carry `AppCommand::LoadDocument` deletion (+ `AppFrame` tag 15 + `AppChannelClient.loadDocument`); no descriptor regeneration needed for the verb set (unchanged) — but plugin describes are already needed for other reasons.

---

### S3-W1G — store / vcs / spr / worker (supersede replay, §15 transactions, N17 deferred replays, O(change))

| Field | Value |
|---|---|
| Report | `📓️w1-g-report.md` (1234 lines); last section `### Audit majors, 10-03 (from 📓️audit-s3-core.md §2)` (L1165–1234; W1G-1 and W1G-3 closed) preceded by `### Session 3 summary (10-03 06:55)` (L1087) |
| Owns | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` (regions `🔖️ToolTransactions`, `🔖️DeferredReprojection`), `🌿️vcs/🦀️.rs`, `📡️spr/📜️history/🦀️.rs`, `🏪️store/👷️worker/🟦️.ts` (cold-pair `settleColdPairLoading`/`retryColdPairBackpressure`), plugin `BoundedStoreInitializationAuthority` region (`ValidateEdit`, `edit_index`, `progress()`), store tests `🧪️tool-transaction`, `🧪️deferred-reprojection`, `🧪️hot-path`, `🧪️supersede-replay` |
| State | PARTLY DONE-VERIFIED. Verified (ran, saw pass): §15 8/8 store + TS oracles 7/7 + plugin 2/2 (10-02); N17 laws 3 x 240 mutations + PER-VIEWER supersede-replay laws: kernel 92/92 (19:00); kernel per-test 820/16 (all peer `durable_group`); TS worker 18/0 (10:50); tsc worker+law 0 errors; kernel lib (W1G-3) 1254 ok / 2 peer FAIL (11:2x–11:4x). Written-unverified: linear `ValidateEdit` initializer + `progress()`, `🧪️bounded-reload` law (moved by W2A), W1G-3 plugin-side reads |
| Last peer blockers | `✏️s` workspace red from stdio peer (zip/gltf ValueError) so puzzle 3d cannot build for the O(document) measurement; two kernel-lib reds that are peers': `outbound_announcement…exactly_once` (asserts `steps >= 22`, fixture cut to 14 by CLOSURE — CLOSURE fixed at 12:04 per status L687), `sqlite_snapshot…native_input_retained_materialization…` (`📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:62`) |

**Owed verification (verbatim / from report)**
1. O(document) confirmation (blocked by `✏️s` stdio): puzzle 3d `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`, `b54_measures…` and the 816-op fill memory law (`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- one_mutation_publishes b54_measures` + fill filter; use streamed ticks for W2A-5). Probes are already deleted, so a measurement needs a fresh temporary `[DEBUG]` probe or the census units only.
2. Plugin suite: `cargo test -p semio-framework-plugin --lib` (reload law `🧪️bounded-reload` progress ends `(240,240)` + steps < 16*240, archive laws, initializer). Prior: `--no-run` 27 min (12:29), per-test 943 ok/16 FAIL at 12:30 10-02.
3. Kernel: `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-w1g cargo test -p semio-framework-os-kernel --lib -- tool_transaction_tests deferred_reprojection_tests supersede_replay_tests dispatch_group_stamps_one_tool_transaction os_spr::history::tests` (92/92 at 19:00) re-run after W1G-3 (hot-path law `🧪️tests/⚡️hot-path` windows: 1000 Applies 0 folds; 400 ticks+commit 0/0/1/401; 3 ticks+commit+200 Applies 0/0/201/204) — note `retained_clone_tests` need `RUST_MIN_STACK=256MiB`.
4. FU4 hub bin: `cargo test --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml -p semio-hub --bin os-hub` and `cargo test -p semio-hub --lib` (247/8 at S2; none W1G).
5. Replication: `cargo test -p semio-framework-replication --lib` (316/0 in S2; test imports moved by CODES-TAX 260/0) and replication TS `bun ./📜️script.ts test --reporter=verbose` (needs `SEMIO_VITEST_POLICY=repositoryVitestPolicyV1(cwd)`, `SEMIO_TEST_BUDGET_MS=400000`; 20/21, R-1 peer fixture).
6. os TS: `bun ./📜️script.ts test-store-oracles` (7/7) and `bun ./📜️script.ts test 🏪️store/👷️worker` (18/0).

**Open items to implement (audit W1G-2, 4–10; report open list)**
- W1G-2 [major] `ValidateEditPair` (N^2/2 steps) survives as hand-written copies in 8 plugin initializers (writer, generation2d, generation3d, gismap, process3d, jack, drawing, raster; `git grep ValidateEditPair ✏️s`) + coverage gate pins old phase names (`🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts` ≈713/782/879/939/1409) -> one framework `ValidateEdit` helper, delete 8 copies, update gate. NOT STARTED per tail.
- W1G-4 `FoldSupersessions` O(edits*ops) single step; `CloneInitial` sum; `edit_index` drop — slice by edit index with fuel.
- W1G-5 §15 law gaps: abort compare redo/cursor/tail cache; remote ingest under open transaction then abort; transaction streaming while a local step waits (drag can starve the step).
- W1G-6 N17 language-agnostic corpus + third-party oracle (Rust-only laws today; no TS twin of `deferRemoteReplays|discardLocalStep`).
- W1G-7 `adopt_pending` drops waiting remote transitions on projection error -> route through `refuse_local_step`; set `last_projection_cause = Replay` on sync adoption.
- W1G-8 one-op edit identity differs by route (batched close never calls `stamp_primary_operation_identity`).
- W1G-9 per-viewer: add `.spr` `REC_VIEWER` pack round trip law.
- W1G-10 host cold-pair settle has no wall deadline/backoff (`1<<20` turns).
- O(document) publication regression: measure + confirm W2A's `PUBLICATION_RETURNED_ROOT_ALLOWANCE = 1` fix (22 vs 1590 units); if >1 returned root persists, size-bounded retirement per publication.
- Ring-stride prefix fold at replay start (<= ceil(N/16) edits for N>256, `📓️api-deferred-history-replays.md` §4).
- Delete Edit/Apply/GroupMeta `.description` store fields with the store wave (status L662) — only agent `transaction_commit` label still writes it.
- Routed peers: R-1 replication fixture `trailing-flags-invalid` flags `04`->`08` (`🧫️document-backbone-batch-v1`), R-2 `durable_group` x16 `max_total_alloc = 162 000` (pack materialization accounting, `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2240,2344`).

**Coordinator actions**: none for W1G itself (no descriptor/launch/schema regeneration, no new verbs); keep `test-store-oracles` launch row; store-wave deletion of `.description` fields rides the final regeneration wave.

---

### S3-W1E — UI contract / manifest input descriptors / tree + number controls

| Field | Value |
|---|---|
| Report | `📓️w1-e-report.md` (1136 lines); last section `### S3.12 x-semio-ui.optionSource (design §20.11) — TS verified, Rust written (tree red)` (L1079–1136); owed list `### S3.9` (L1012) and `### S3.10` (L1040) |
| Owns | `🧰️framework/🔨️modules/🖱️ui` (contract, tree/disclosure, `🧩️component` ContextMenuItemSpec, wgpu a11y mirror), `🛂️manifest` (`number_facets`, `OptionSource`, `mutation_input_defs`, corpora `🧫️number-facets`, `🧫️mutation-inputs`), React `ui-react` (ContextMenu, Tree), `🗣️Interpreter` row semantics, plugin `⏪️time-travel` N2/optionSource regions (`time_travel_pointer_insert/remove`, `time_travel_default_value`, `time_travel_option_target`, `resolve_time_travel_options`) |
| State | SOURCE-COMPLETE, PARTLY VERIFIED. Verified: UI lib `--features testkit` 769 ok / 2 peer FAIL (05:49; every W1E law green incl. row-semantics, disclosure, a11y corpus, G6 + 55 shape cases); UI contract 215+12+1; TS corpus 77; bun facets 13; history-edit vocab 3; ui-react 78 (+ContextMenu 11/11); mutation-inputs bun 88 -> 90 ok, Python jsonschema 207 verdicts/40 cases; plugin check native+wasip2; UI wasm32 x2 check. Rust of optionSource/N2/F7 NOT compiled/tested on the green tree |
| Last peer blockers | UI 2 FAIL are peer (`wgpu::draw::tests::world_mesh_instance_packs_policy_and_standard_material_into_one_fixed_stride` 128!=112; `wgpu::gpu::prepared_present_tests::every_world_color_cursor_uses_the_encoded_composite_attachment` 17!=15); kernel test `🎠️kernel/🧪️tests/💡️service-operation/🦀️.rs:9` `dsl::json` (fixed by W2A 06:04); stdio crates for puzzle 2d `select_tool_history` |

**Owed verification (verbatim)**
1. `cargo test -p semio-framework-ui --features testkit --lib` (re-run to confirm 769/2 unchanged)
2. `cargo check -p semio-framework --lib --tests`, then `cargo test -p semio-framework --lib -- mutation_inputs number_facets history_edit_actions dialog_choices`
3. `cargo test -p semio-framework-plugin --lib -- time_travel` (N2 list law, chips/session-leads/pointer laws, `sourced_options_are_the_keys_of_the_previewed_document`, W2A's editor-keys law)
4. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --lib -- select_tool_history`
5. `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2` and `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features wgpu-engine`
6. UI wire-format test `-- ui_node_wire_format`; projection test `exports_typescript` (needs CLOSURE's regenerated `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` — CLOSURE did regenerate at 11:25 per status L659, re-check)
7. TS: `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🟦️.ts` (90), `python3 🛂️manifest/🧪️tests/🧪️mutation-inputs/🐍️.py` (207)

**Open items to implement (audit W1E-1..7 + report)**
- W1E-1 a11y: disabled tree-row-action reason is screen-reader-only (no visible title/tooltip) in `TreeRowActionButton` (`UI/🧱️elements/🌳️Tree/🟦️.tsx` ≈694-718) and wgpu mirror; extend `💬️row-semantics`.
- W1E-2 long-option rows signal chosen state by icon only — add contract `selected` on the row (both renderers).
- W1E-3 N17 has no live announcement: shell status region for `patch.reprojection` (React `🛠️ShellHelpers/⏪️time-travel`, wgpu `history_lane_notice`), en/de (with W2C).
- W1E-4 wgpu hand-painted context menu must paint/describe `ContextMenuItemSpec.reason` (routed to S3-W2C); assert `reason => disabled`.
- W1E-5 layering: `number_facets` in neutral manifest returns ui-contract types (`MF/🦀️.rs` ≈1063-1076) — optional.
- W1E-6 live e2e probe `g9-every-mutation-of-the-long-transaction-is-reachable` predates N1 (S3-E2E).
- W1E-7 optionSource: linear template scans, rides W2A-7 memoisation.
- N2 (d) option search deferred (all options reachable via windowed rows). Array indexes with leading zeros accepted (W2A-14).
- wgpu mirror law for row descriptions/disabled actions pending tree green.

**Coordinator actions**: puzzle 2d re-activation for the live N2/N1 probe (history-edit verbs stay 12; no describe/launch/schema regeneration for W1E); central `schema generate` only if the `OptionSource`/meta-schema changes are projected (`🧬️schema/📽️projection` ArgSchema v3 + OptionSource v1 mirrored by hand in generated manifest TS).

---

### S3-W2B — React host (shell, history body, number controls, folder, load/notices)

| Field | Value |
|---|---|
| Report | `📓️w2-b-report.md` (943 lines); last section `### S3.14 HistoryPatch.editCount replaces the digits in a fault message` (L925–943); open/coordinator in `### S3.10`/`### S3.11` (L835/L852), load in `### S3.13` (L875) |
| Owns | `RE` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/…`: `🛠️ShellHelpers`, `🏛️ShellHost`, `🔌️PluginRuntime`, `🗣️Interpreter` (number-keyboard-law), `⏪️time-travel/🧪️tests/🧩️component`, `🧪️staged-arg-controls`; `🧰️framework/🛍️products/💻️os/🟦️.ts` (`AppChannelClient.loadDocumentArchive`); `🎚️config/…/📎️attach-local-folder` schema |
| State | **DONE-VERIFIED** (S3.13/S3.14 final). 128/128 vitest (time-travel component, staged-arg-controls, local-folders, command-rejection, spawned-program-session); PluginRuntime 135/135; framework-os `loadDocumentArchive` 3/3; related set 1201 ok / 14 peer-or-pre-existing FAIL; os-config `local_folder` 20/20 + `the_folder_is_edited…` 1/1; plugin `the_draft_editor_keys…` 1/1; taxonomy clean; renderer-react tsc 1 peer error (`🌐️World3dHost/🟦️.tsx:2225`) |
| Last peer blockers | `World3dHost :2225` tsc; 14 red React laws (peers/pre-existing, list in S3.8); package-integration 26/29 (`Current WGPU package artifact authority drift` x2, `deps-javascript` id) |

**Owed verification**: none of its own. Resume only if W2A/W2C change the wire again. (Plugin `-- time_travel` run to the end remained undone, owned by W2A/W1E.)

**Open items (routed)**
- W1-E `🌳️Tree`: property-layout rows dropped `description` (later fixed by W1E S3.8: row descriptions always exposed + `aria-describedby`); disabled row actions focusable `aria-disabled` (done by W1E, React verified).
- W2A: Add/Remove item disabled reason (wire has only `disabled`) — later routed to W1E/W2A `RowAction.reason` (done in source); structured `history.full` count (done: `editCount`).
- `requestMediaFrames` has no host cancel in either shell (no import abort possible) — OPEN, unowned.
- Live probe verdicts to add (S3.11): reveal on session start, focus to the editor, refused Edit named and inert, Add/Remove item named, staged dialog number controls with display units; `axe-core` still not installed (dev consent) — a11y oracle stays `aria-query` + `dom-accessibility-api`.
- `AppChannelClient.loadDocument` + `LoadDocument` codec tag stay until the bump wave (wgpu `🐚️plugin-bridge/🟦️.ts:1921` and `🧪️backbone-envelope-io` still call them; W2C adopted `DocumentArchiveLoadHost`).
- Peer: ui-react vitest config now needs `SEMIO_VITEST_POLICY` (derive with `repositoryVitestPolicyV1(<package dir>)`; 15 s default budget is too short).

**Coordinator actions**: central `schema generate` (`attach-local-folder` payload schema + band corpus schema changed); React re-activation 6012 for live probe; route W1-E Tree items; bump wave deletes `AppChannelClient.loadDocument` + codec tag.

---

### S3-W2C — wgpu host/shell (renderer-wgpu: parity N1/N2/N15, reprojection, notices, stepped load, blur/close cancel, import task, F7 menu)

| Field | Value |
|---|---|
| Report | `📓️w2-c-report.md` (1280 lines); last section `### S3.16 After TREE GREEN (11:38) — 11:39–11:46` (L1252–1280); owed list `### S3.15` (L1217) |
| Owns | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu` (`🐚️Shell/…/🧊️wgpu/🦀️.rs`, `🧊️renderer/🦀️.rs`, `🐚️plugin-bridge/🟦️.ts`, `🌐️browser-host`, `🚚️browser-frame-transport`, `🎞️frame-worker`), `🧪️wgpu-time-travel` laws; WorkerCell/UI-engine locale refactor completion (`UiEngineCell`, refusal `ui.engine.locale-unresolved`) |
| State | SOURCE-COMPLETE-UNVERIFIED for native Rust (native test target never built since 11:22 10-02). Verified: 10-02 test build Finished (11:22, 0 errors), 52/52 laws (`time_travel local_folders introspection_tests dialog_choices`), wider 177/178 (1 peer: dag demo DSL, since fixed by GRAPHS), vitest browser 90/90 -> 92/92 (10-03), `test-preview-generated` 27/29 (2 peer: not schema-owned `🌱️value/⚠️refusal/🟦️.ts`), UI crate 16/16 (`conformance_corpus presence_bar peer_notes`), `tsc` wgpu host 0 errors, wasm32 `--lib` Finished exit 0 (10:45) |
| Last peer blockers | graph peer: `🕸️graph/🛂️manifest/🪆️binding/🦀️.rs` RecordSpecProducer (cleared 12:00); os-infinite `🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:770,2243,5408,6732` `DagExpandedPaths` missing (peer in-flight 12:04) — renderer unreached since 11:40, so 11:4x edits (ProgramFault tails, `DocumentArchiveLoadHost`) are unchecked |

**Owed verification (verbatim, S3.15)**
1. Build: `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s3-w2c cargo test -p semio-framework-os-renderer-wgpu --lib --no-run`; run binary from the renderer crate dir with `RUST_MIN_STACK=67108864` and filters:
   - `time_travel local_folders introspection_tests dialog_choices` (52 + new: `every_staged_number_row_carries_the_shared_corpus_facets`, `the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason`, `the_guest_editor_offers_list_and_chip_edits_within_their_bounds`, `a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror`, `every_history_lane_refusal_is_a_notice_carrying_its_code`, `a_host_window_blur_blurs_the_active_pane_for_its_program_once`, `a_picked_import_is_a_cancellable_task_that_frees_a_started_import`, `a_disabled_context_menu_row_is_reachable_and_tells_its_reason`, `a_plain_document_load_is_the_stepped_archive_load_without_members`)
   - `chrome_overlays_tour_tests board2d_engine_tests board_presence_tests agent_overlays_tests shortcuts_palette window_actions_search_panes panel_anchor_model hub_projection_workspace canvas_presence`
   - `context_menu`, `window_lifecycle`, `navbar_footer`, `shell_input`, `ui_command_wiring`, `scenes::` (WorkerCell/UI-engine locale wave)
2. `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` (native) and `--lib --target wasm32-unknown-unknown`.
3. TS: wgpu package `bun ./📜️script.ts test-browser 🧪️tests/♿️wgpu-accessibility-interaction/🟦️.ts 🧪️tests/📨️browser-frame-transport/🟦️.ts 🧪️tests/🧪️wgpu-backbone-folder-door/🟦️.ts` (92/92) and `bun ./📜️script.ts test-preview-generated`; `bun ./📜️script.ts check-browser-worker` (blocked by `🖱️ui/🌐️locale/🟦️.ts` not schema-owned — INFRA fixed in browser profile 11:03, re-run).
4. `cargo test -p semio-framework-ui --features testkit --lib -- conformance_corpus presence_bar peer_notes` (16/16).

**Open items to implement**
- wgpu bridge adopts shared `DocumentArchiveLoadHost` (written 11:4x, unchecked); `AppChannelClient.loadDocument` callers deleted at the bump wave.
- WorkerCell/no-default-locale wave: compile + run (`UI_ENGINE` = `WorkerCell<Option<Ui>>`, `Ui::new(locale)` at ShellState::new + setLocale + preference resolution, cfg(test)-only fixture locale).
- Cross-shell (not done): announce `HistoryPatch.reprojection` as polite status in both shells' bands (with W2B/W1E W1E-3); marketplace disabled row actions localized reasons (needs `UiTreeItemAction.reason` — out-of-scope chip spawned); feed `ActionArgDef::number_facets` into ChromeDialog staged slider (`ChromeDialogFieldKind::Slider`).
- wgpu window-focus signal for S3-SPATIAL N9 (world3d streaming/paint blur) — delivered as `arm_host_window_blur` + `cancel(Blur)`; CaptureLost on close delivered via `world3d_close_cancel_settled` sightings.
- ProgramBridge wasm32 `ProgramFault` tails fixed (11:4x) — recheck compile.
- wgpu live gumball visual proof belongs to S3-SPATIAL/E2E (`--renderer=wgpu`, port 6112).

**Coordinator actions**: one activation + serve of 6112 for the `--renderer=wgpu` probe; wgpu Vite config/host-io/bridge/transport/shell changes take effect on next activation; no descriptor or schema regeneration for W2C.

---

### S3-W2D — puzzle 2d editor apps (board engine, wgpu board, React Board2dHost, select-tool laws)

| Field | Value |
|---|---|
| Report | `📓️w2-d-report.md` (516 lines); last sections `### S3.4 Owed verification` (L449), `### S3.5 Coordinator actions (exact)` (L458), `### S3.6 Open item analysed: session-1 §3.6 (wgpu rotate ring)` (L478–516, "VERIFIED" at 11:09) |
| Owns | puzzle 2d editor `ED` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor` (select-tool corpus laws, `🌉️wasm` `BoardSession.setHighlightedIdsJson`), board engine `P/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal` (`pointer_lane_is_direct`, `plan_pointer`), wgpu `R/⚙️EngineCanvas/🎯️targets/🧊️wgpu` board routing + highlight sync, React `🖥️Board2dHost` (`applyBoard2dHighlightedIds`) |
| State | SOURCE-COMPLETE; core crates VERIFIED, puzzle crates UNVERIFIED. Verified: python oracle select-tool-history 4 scenarios/12 head nodes PASS; `cargo test -p semio-framework-os-infinite --lib -- directed_normal` 55/0 (11:09); `cargo test -p semio-framework-os-renderer-wgpu --lib -- board2d` 9/0; React board-event-coalescing + float32-decimal 29, engine-contract puzzle-2d filter 41; board+puzzle wgpu 57 ok / 2 FAIL (not W2D: `the_puzzle3d_app_carries_the_introduction_the_tour_arms_on` reads moved descriptor `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` now under `🌎️hub/🧩️compositions/…`, owner W2C/shell; `focused_text_editor_clipboard_composition_and_accessibility_share_the_accepted_host` text-editor owner) |
| Last peer blockers | kernel red 11:19 (store peer `🏪️store/🦀️.rs:17111,17263,22056`); stdio crates (gltf/zip/svg ValueError) block the puzzle 2d lib build; root `📜️script.ts:21073` `continue` load error blocked taxonomy report (fixed by INFRA 11:10) |

**Owed verification (verbatim, S3.4)**
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib` (full lib, `RUST_MIN_STACK=67108864`), then filters `select_tool` (machine + transactions + corpus/G3 laws incl. Use-selection click) and the two 8 ms fill laws re-run alone: `board_fill_job_large_host_has_no_step_at_or_above_eight_ms`, `fill_run_job_drive_step_stays_below_the_interactive_ceiling_for_nakagin`
2. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- a_board_gesture_drag board_node_delete apply_board_events language_neutral_fixtures`
3. `cargo check --manifest-path ✏️s/Cargo.toml --target wasm32-wasip2 -p semio-s-plugin-puzzle`
(Also from W2D's S2: `bun ./📜️script.ts verify taxonomy report --scope …/select-tool-history` on the 3 dirs.)

**Open items**: none implemented-blocking; rotate-ring / region drag wgpu direct lanes now FIXED in source and core-verified (S3.6). The wgpu direct-lane live proof (rotate ring, region body drag publishing one `select` + `gesture` batch) still needs the live wgpu e2e (S3-E2E).

**Coordinator actions (exact, S3.5)**
1. Puzzle plugin re-describe + re-activation (React 6012): new wasm binding `BoardSession.setHighlightedIdsJson` (`ED/🌉️wasm/🦀️.rs`) and the Nakagin manifest registration (`◻️2d/🛂️manifest/📇️outputs.json`, example `🏗️nakagin-capsule-tower/🖼️assets/🛂️manifest.json`). No channel bump.
2. wgpu shell re-activation (6112): `board_local_pointer` mapping + board highlight sync (`R/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`), `dumpBoard2d.highlighted` (`R/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`), direct-lane routing.
3. Central `schema generate` for new scope `editor/select-tool-history` (`ED/🧬️schema/🔣️select-tool-history/🔣️.json`).

---

### S3-AGNOSTIC — artifact-agnostic gates (G7 labels, G8 editability/inputs/payloads, G12 harness, §20.15 reload law + reader seam)

| Field | Value |
|---|---|
| Report | `📓️s2-agnostic-report.md` (544 lines); last section `### S3.12 W-a plan, serializer-trait merge evaluation, verdicts and open items (10-03 12:05)` (L506–544), preceded by S3.11 reader seam (L484), S3.10 §20.15 gate+law (L447), S3.8 `wordOnlyFloat` (L403) |
| Owns | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts` (gates), `🧪️test/🧪️tests/🧪️mutation-history-gates/🟦️.ts`, `🛂️manifest` reader (`WordOnlyFloat`, numeric transport), `🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` (G12 harness + `composed_reload_law!`), plugin region `🔖️InferenceChildren`, `🚪️io` region `🔖️ArchiveChildren`; wiring of G12 into 118 editor modules / 96 crates / 34 plugins; `🧪️s3-agnostic-run-acceptance.sh` |
| State | G7 gate DONE-VERIFIED (strict repo-wide 10-03 07:10: 0 findings / 3096 labels). G8 reader/gates VERIFIED (bun mutation-inputs 92/0; Python 216 verdicts/42 cases; `cargo test -p semio-framework --lib -- every_corpus_case` 2/0 at 11:28; mutation-history-gates 39/0). G12 UNVERIFIED (wired, **no session-3 run reached a test binary**; last real results session 2: 9 PASS — vcs, puzzle 2d, norm en1990, block 2d, note, layout, shooting, generation2d, remodel). §20.15 reload law WRITTEN, compiles (`cargo check -p semio-framework-plugin --lib --features artifact-app-testing` exit 0 11:45/11:52), runs owed. W-a NOT STARTED |
| Last peer blockers | ✏️s workspace red until stdio ValueError sweep lands; peer schema split / DSL extraction (earlier); trinity jack/rewriting `wordOnlyFloat` in the migrating peer's files |

**Owed verification (verbatim)**
1. Strict gates repo-wide (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): `bun ./📜️script.ts schema mutation-labels`, `bun ./📜️script.ts schema mutation-editability --json`, `bun ./📜️script.ts schema mutation-inputs --json`, `bun ./📜️script.ts schema mutation-payloads` — last: labels 0; editability 92 `parentLeafReadsChild`; inputs 312 (6 `wordOnlyFloat` trinity, 178 in-flight remodel (since 0 per STROKES), 72 `leafUncatalogued` -> schema generate); payloads 32 (remodel, raster `fill-region` unwitnessed).
2. G12 per plugin in batches: `🧪️s3-agnostic-run-acceptance.sh <batch> <crates…>` (test `history_edits_end_to_end` + `documents_reload_identically`), all 96 crates / 34 plugins, when ✏️s compiles.
3. Derive runtime laws `semio_payload_law_*`, `history_edit_inputs_resolve` (kernel last green 10-01 31/0).
4. `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-history-gates/🟦️.ts` (39), `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🟦️.ts` (92), `python3 🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🐍️.py` (216/42).

**Open items to implement**
- **W-a (one gated wave once ✏️s compiles)**: `Serializer<S>::serialize(from: &S, children: &ArchiveChildren)`; `serializer_entry`/`serializer_entry_text` recognise the composed carrier (`encode_document_archive_bytes(&DocumentArchivePack{parent_pack: HEAD pack, parent_spr: empty, members})`, first byte 0x01 vs pack magic 0x89); 76 io impls + 140 registrations + 142 `::serialize(&…)` call sites; migrate 41 `ArtifactSerializer` impls onto `Serializer<S>`, delete `ArtifactSerializer`/`serializer_entry_of` (67 sites), `ArtifactDeserializer` likewise; delete `ArtifactInferrer` + 112 marker impls (gltf `🧊️gltf/🦀️.rs:214` -> `protocol::Inference::infer`); audit every `ComposerEntry` consumer resolves through `io_route`/`io_run`.
- **W-b** producers (host `run_io` `🔌️plugin/🖥️host/🦀️.rs:6527`, TS shell export, MCP gateway `🌉️mcp/🏠️workspace/🦀️.rs:1670-1690`) -> S3-LOAD (resumed).
- §20.15 backlog **92 `parentLeafReadsChild`**: dag 17, mathematical 16, wires 12, flow 10, cad 8, jack 8, playbook 8, sequence 8 (converted by GRAPHS), imperative 4, din18599 1 -> owners GRAPHS/FLOWCAD/TEXT/CONTROLS/NORM.
- Extend G12 harness to seed child-lane (member-store) edits when child vocabularies land (sequence representative: stdio-semio `drag-nodes` on `content`).
- Trinity `wordOnlyFloat` x6 (jack CameraJson x/y/zoom; rewriting snapshot schemas) -> S3-TEXT after the migrating peer is quiet >= 30 min.

**Coordinator actions**: central `schema generate` (72 `leafUncatalogued`, trinity `property-value.json` catalog document); re-describe after composed-plugin conversions; send "✏️S GREEN" so G12 batches + W-a can run.

---

### S3-DRAW — draw + note plugins (gestures, labels, chips, taxonomy)

| Field | Value |
|---|---|
| Report | `📓️w3-t-draw-note-report.md` (399 lines); last section `### S3.8 Open items and coordinator actions (as of 10-03 10:45)` (L383–399) |
| Owns | `✏️s/🔌️plugins/🖍️draw` (DA aggregate `🧬️schema/🧬️mutations`, DT tool leaves, `🎮️commands`, `✏️editor`), `✏️s/🔌️plugins/🗒️note` (NB/NO), hub `🌎️hub/🧩️compositions/🖍️draw` + note Cargo manifests |
| State | SOURCE-COMPLETE-UNVERIFIED for Rust. Verified: 5 moved leaf oracle tests bun 5/0 (56 expects); note TS 3/0; draw TS 280 ok / 2 pre-existing `sharp` fill-sampling FAIL; `schema mutation-inputs\|mutation-payloads` draw 0/0 (52 inputs, 22 leaves witnessed), note 0/0 (60 inputs, 35 payloads, 34 leaves); G7 `mutation-labels` draw 22/22, note 28+6 forwarding: 0 findings; taxonomy 5 x clean; path-budget 0 dangling |
| Last peer blockers | `✏️s` kernel/plugin earlier; stdio zip (52 errors) / svg (6) ValueError; workflow sqlite (fixed 10:56 by INFRA); renderer-wgpu locale wave (W2C) |

**Owed verification (verbatim, S3.7/S3.8)**: gated one at a time, `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-draw`:
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-draw-drawing --lib` (incl. `retained_blend_mutations`, both `⏪️TimeTravel` laws, `layer_references_read_their_name_and_take_the_canvas_selection`, the 9 `Emit::mutations` handlers)
2. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-note-note --lib` (ink laws, three `⏪️TimeTravel` laws, label law, `document_setting_labels_read_the_value_not_a_debug_option`, `block_references_read_their_name_and_take_the_block_selection`)
3. `cargo check -p semio-framework-os-renderer-wgpu --tests`
4. `cargo check -p semio-hub-draw -p semio-hub-note --target wasm32-wasip2`

**Open items**: none implemented-blocking. Left to CLOSURE (done at 19:16 wave 5a: coalesce refs gone). Pre-existing/not this WP: draw TS `sharp` fill-sampling 2 reds; taxonomy findings of 4 older draw leaf `🔬️unit` dirs + `DA/…/🧪️tests/🔬️kinds-catalog`; note label "Some(…)" bug class fixed in 9 leaves (law written).

**Coordinator actions**: re-activate draw (React 6064 / wgpu 6164) and note after green (labels, `Emit::mutations`, Cargo feature changes `semio-framework-2d features = ["booleans","trace"]`); no descriptor/launch/schema regeneration needed by DRAW (but see global describe list); GATES may adjust `#![allow(unexpected_cfgs)]` in hub draw `🦀️.rs:13` once wasm check is clean.

---

### S3-SPATIAL — fem 2d/3d, lowpoly, shooting, World3d gumball/paint protocol (React + wgpu consumer)

| Field | Value |
|---|---|
| Report | `📓️w3-t-spatial-report.md` (361 lines); last section `### S3.6 Audit fixes (📓️audit-s3-tools.md S3-SPATIAL, 2026-10-03)` (L307–361), owed list at its end; open `### S3.4` (L274), coordinator `### S3.5` (L297) |
| Owns | `✏️s/🔌️plugins/{🏗️fem (2d,3d), 💠️lowpoly, 🎥️shooting}`, `OS/♾️infinite/🌍️world/🦀️.rs` (World3d region: `WorldGumballGesture`, `WorldGumballPhase`, `WorldInteractionPhase::Cancel`, `world3d_cancel_owed`), React `🌐️World3dHost` gumball/paint step (`worldGumballStep`, `worldPaintStep`), shared corpus `🌐️World3dHost/🧬️schema/🔣️gumball-live-protocol` + fixture, plugin hook `selection_reference_id`/`draft_time_travel_selection` |
| State | SOURCE-COMPLETE; core VERIFIED, plugin crates UNVERIFIED. Verified: `cargo test -p semio-framework-os-infinite --lib -- gumball paint cancel` 42 pass / 1 peer FAIL (`authored_inline_surface_…_cancellation`; the 30 scene-bridge/pick failures in `world::tests` are peer: texture-sampler / multi-UV `Mesh3dField` 9->14); `cargo test -p semio-framework --lib -- gumball_verb_audience` 1/1; `cargo check -p semio-framework-plugin --lib` PASS 10:50; React engine-contract `-t "worldGumballStep\|worldPaintStep\|gumball"` 13/13; lowpoly TS 2+6; payloads fem 62/62, lowpoly 21/21, shooting 38/38 witnessed, 0 findings; mutation-outcome-law PASS 19:18; inputs lowpoly 60/60 (N12 closed) |
| Last peer blockers | plugin TEST target red (peer sqlite/IoError migration, since fixed by INFRA 11:38); every fem/lowpoly/shooting crate depends on red stdio crates (ply, dxf, svg, pdf, jpg, gif, zip, gltf) + workflow (fixed 10:56) |

**Owed verification (verbatim; private target `target-nde-s3-spatial`; one gated cargo each)**
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-shooting-shooting --features …component-app-assembly --lib --no-fail-fast` (report abbreviates the feature list with an ellipsis; use `--features component-app-assembly` per crate)
   — new laws: `the_world3d_gumball_live_protocol_lands_as_its_guest_edits`, `playback_presses_are_one_config_edit_and_never_a_history_row` (both fem crates), ported fem 2d tick + clock laws, `use_selection_retargets_a_vertex_move_onto_the_selected_vertices`, `mesh_rows_name_the_references_of_their_granularity`, `save_camera_stores_the_submitted_label`, manifest `gumball_verb_audience`.
2. `cargo test -p semio-framework-plugin --lib -- use_selection selection_value reference_chips`
3. `cargo check --manifest-path ✏️s/Cargo.toml -p <each of the 4> --lib --target wasm32-wasip2`

**Open items to implement**
- **S3 (audit, minor)**: twin fem playback clocks (fem 2d + fem 3d results windows) -> one shared `FemPlaybackClock` / `FemResultsWindowTransient{Mutation}` / `set-playback-clock` in fem 2d + trait `FemPlaybackTransport {phase, reverse, speed, playback_loop, rested_at}`; fem 3d keeps `Fem3dResultsWindowTransientOwner` + wrappers; bridge `🏗️fem/🏭️bridge/🦀️.rs` and feature corpus `🧪️tests/🫧️mutate-fem-3d-1-any-editor-edit-results-transient` move to the shared leaf. Waits for compile.
- N9 wgpu parity: DONE in source for gumball (S1) and window blur/CaptureLost cancel (with W2C). wgpu live visual proof still needed (E2E).
- Scrub glue (CONTROLS) must hold config/window-config ticks as provisional overlay (done by CONTROLS 12:53/11:17); fem PLAYBACK_COALESCE_KEY deleted (fem 2d window-transient clock).
- `FemGumballTransient` and lowpoly paint drive duplicate per-window runner bookkeeping (follow-up for `🛠️tool-machine`).
- Peer-owned warnings: `🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs:15`, lowpoly `🌐️model/🦀️.rs:10`, `🖼️uv/🦀️.rs:6,9`.

**Coordinator actions (S3.5)**: central `schema generate` (drops deleted shooting `set-camera-draft-label`, catalogues fem 2d `set-playback-clock`; the last 2 `schema mutation-inputs` findings in scope); `describe` for hub compositions `🎥️shooting` (verb `setCameraDraftLabel` gone), `🏗️fem` (playback lanes, `resultAnimationTick{windowId}`), `💠️lowpoly`, `🎪️demonstrator`, `🧩️puzzle` (stale `transformBegin/End`, `paintStrokeBegin/End`) and `OS/🧑‍💻dev/🔌️plugin-modules/*` copies; re-activate fem, lowpoly, shooting for live gumball stream/commit/abort, playback press, camera-label field, lowpoly colour picker.

---

### S3-FLOWCAD — flow + CAD + shared node-graph contract (`nodeGraphEdit`), composed child (§12 / §20.15)

| Field | Value |
|---|---|
| Report | `📓️w3-t-flow-cad-report.md` (681 lines); last section `### S3.8 §20.15 — composed content only on the child lane (in progress, 11:45 10-03)` (L670–681; ends mid-plan at the deletion list, NO verification line). Owed list `### S3.6` (L598–616), audit fixes `### S3.7` (L618–668) |
| Owns | `✏️s/🔌️plugins/🌊️flow` (guest, `🧩️extensions/*`), `✏️s/🔌️plugins/📐️cad`, dag journal encoder in `OS/♾️infinite/…/🕸️dag` (`write_dag_graph_edit_rows`, `DagGraphEditRowSink`, `DagHost::journal_moves`), `framework-surface` `GraphHost`/`GraphSession.takeGraphEditsJson`, `os-flow` host, plugin `🧪️node-graph-delete-row` + `node_graph_delete_selection_spec`, React `dispatchGraphEdits`, node-graph row contract `RE/🧬️schema/🔣️node-graph-edit-rows` + fixture + tests, time-travel `Finalized{authored}` + `TimeTravelLabel::MemberEdited` |
| State | SOURCE-COMPLETE for §12/N6/F1–F6/X3; §20.15 flow IN PROGRESS (disk shows `pub enum FlowMutation {}` uninhabited at `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:13` — flow parent-vocabulary deletion partly landed, completeness unreported); §20.15 cad NOT STARTED (22 refs `cad_pane_local_scene` still in parent leaves: delete-object, move-objects, scale-objects, create-object, rotate-objects diff/inverse). Verified: TS node-graph vitest 26/26; flow TS source-contract suite exit 0; `cad-window-transient-contract rows=6` PASS; `composed-child-history-oracle cases=4` PASS; cargo check (10-03 06:00) 0 errors for os-infinite, surface, os-flow, time-travel, plugin; `cargo test -p semio-framework-time-travel` 14/0; TS conformance 20 |
| Last peer blockers | `✏️s` stdio crates (zip 52, step 125, gltf 13, svg 6 errors, ValueError class — peer; INFRA sweeps non-stdio only); `renderer-wgpu` `🧊️renderer/🦀️.rs:18985–18998` ValueError (since fixed by peer 10:22) |

**Owed verification (verbatim, S3.6; gated, one cargo at a time, `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-flowcad`)**
1. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-flow-flow --lib --tests --keep-going`
2. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad --lib` (baseline 459/2; S2.7 laws + transient laws)
3. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow --lib` (§12 laws x6 incl. the 2 new, F6 x2, intent rows x4; `optional_field_rows_keep_their_pre_migration_bytes` may need the AddWidget vector re-sealed — greenfield: re-seal or delete with reason, no compat pins)
4. `cargo test -p semio-framework-plugin --lib -- composed_child_history node_graph_delete_row time_travel scrub`
5. `cargo test -p semio-framework-os-infinite --lib -- node_graph_edit_rows wire_edit` (dag laws `node_graph_edit_rows_tests` x4 incl. 200-node + census laws; `wire_edit_tests`)
6. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-flow-flow --target wasm32-wasip2 --lib`
7. flow F5 label law `a_release_that_wires_and_drags_is_labelled_by_the_drag`, `renderer-wgpu` check; TS: `bun …/✏️editor/🧪️tests/🔬️source-contract/🟦️.ts`; vitest node-graph suites.

**Open items to implement**
- §20.15 flow: finish — retained remove/disconnect/delete-selection build child leaves from scanned ids; delete the 10 parent leaves (+ text/binary codecs, grammar, fixtures `🧫️fixtures/🧬️mutations/*`), `🌊️mutate-flow-1` (case, fixture, python second implementation), parent one-item preparation (`🧵️retained/🗿️artifact/**`, `🧵️retained/🧾️canonical/**`, dead generic `FlowStoreOneItemPreparationFactory`), `flow_content_edit`/`apply_flow_mutation`, subset oracle catalog (all via rule-32 zero-reference proof). Gate target: flow 10 `parentLeafReadsChild` (`flow_working_scene` x9, `precondition` x1).
- §20.15 cad: convert 8 `parentLeafReadsChild` (`cad_pane_local_scene` x5, `cad_selection_inverse_objects` x3) — NOT STARTED.
- F7 (MenuBuilder locale) landed under W2A §9.9 (done there).
- Dag journal / wgpu encoder F1/F2 verification (`write_dag_graph_edit_rows` single encoder; 256-row limit = decoder limit).
- 4 `Edit { coalesce_key: None }` literals died with the field in CLOSURE (done).

**Coordinator actions (S3.6)**: rebuild `framework-surface` wasm bindings (new `GraphSession.takeGraphEditsJson`) and flow-core wasm (`pointerUpScreen`/`alignSelection` answer rows only) before any React node-graph probe; `describe` flow (`addWidget` gains `label`/`action`/`format`; `nodeGraphEdit` row vocabulary) and cad (S2.5 config/lanes); `bun ./📜️script.ts verify taxonomy report --scope` for the new open-pattern dirs (`RE/🧬️schema/🔣️node-graph-edit-rows`, `RE/🧫️fixtures/🧫️node-graph-edit-rows`, `RE/🧪️tests/🧪️node-graph-edit-rows`, dag `🧪️tests/🧪️node-graph-edit-rows`, plugin `🧪️tests/🧪️node-graph-delete-row`); drop other guests' own `setHostSnapshot`/`deleteSelection` decoders (done by GRAPHS/PROCEDURAL/TEXT).

---

### S3-LAYOUT — layout plugin + wgpu Canvas2d path paint + gumball overlay + frame chips

| Field | Value |
|---|---|
| Report | `📓️w3-t-layout-report.md` (411 lines); last section `### S3.10 After "TREE GREEN (core)" 05:47 on 10-03` (L399–411); hand-over/owed list `### S3.9` (L362) |
| Owns | `✏️s/🔌️plugins/📏️layout` (tool machine, `ED` editor `🪧️EntityLabels`), React `Canvas2dHost` (`CH`: `🎨️paint`, `🧪️path-paint`, `🧫️fixtures/🧫️path-paint`, `🧬️schema/🔣️path-paint`), wgpu `🎞️Scenes` region `Canvas2d` + `CH/🎨️paint/🦀️.rs` (Rust twin, mounted `canvas2d_paint` in `🧊️renderer/🦀️.rs`), `🧪️wgpu-canvas2d` laws |
| State | SOURCE-COMPLETE-UNVERIFIED for Rust. Verified: Python corpora current (path-paint 16 cases/68 probes; gumball dispatch 23 cases; author vectors 17, 0 pending; layout oracle 58/0); `schema mutation-inputs\|payloads --under ✏️s/🔌️plugins/📏️layout` 0 findings (146/146 inputs, 48 leaves); layout verify commands exit 0; React `Canvas2dHost` 8 files / 107 passed; standalone harness `G3/paint-proof` 6/0 of the REAL `CH/🎨️paint/🦀️.rs`; React typecheck 0 errors in my files |
| Last peer blockers | os-kernel/IoError/ValueError in-flight (06:54: 12 errors), workflow 222 errors (fixed 10:56), stdio crates; wgpu native lib-test target |

**Owed verification (verbatim, S3.9; one gated command each, private `target-nde-s3-layout`)**
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-layout-layout --lib` (S2 laws + warning cleanup + new `reference_chips_name_frames_by_kind_content_and_page`, `a_drag_edited_onto_a_missing_frame_blocks_finalize_until_its_targets_are_fixed`; last green 595/0 on 10-01)
2. `cargo test -p semio-framework-os-renderer-wgpu --lib -- canvas2d` (`canvas2d_paint` corpus/laws, new seam law `layout_path_records_paint_their_geometry_in_record_order`, gumball seam/corpus laws)
3. `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-layout`
4. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d --features component-app-assembly --lib -- gumball` (blocked in S2 by fem example DSL parse)
5. TS: React pkg `bun ./📜️script.ts test long Canvas2dHost`; `.venv/bin/python T/🧪️s3-layout-path-paint-corpus.py --check`; `.venv/bin/python T/🧪️s2-layout-gumball-dispatch-corpus.py --check`; `.venv/bin/python T/🧪️w3-t-layout-author-vectors.py`; LA pkg `bun ./📜️script.ts verify layout-frame-selection` / `layout-window-ownership` / `layout-document-contract`.

**Open items**
- TAX action: register `🧭️gumball` as an element member (members-of-members-of-elements), then move `CH/🟦️GumballOverlay.tsx` -> `CH/🧭️gumball/🟦️.tsx` and wgpu twin `CH/🎯️targets/🧊️wgpu/🦀️.rs` -> `CH/🧭️gumball/🎯️targets/🧊️wgpu/🦀️.rs` with 2 importers (`CH/🟦️.tsx:19`, `CH/🧪️tests/🧪️gumball-dispatch/🟦️.ts:26`); overlay keeps 7 pre-existing taxonomy findings until then.
- wgpu approximations (documented in `render_canvas_scene_node`): group opacity per piece; rotated images/text keep axis-aligned box; >= 1 px strokes; raster quads composite in separate pass.
- Dependency-truth follow-up: React suites import `gl-matrix` (gumball test, S2) without declared devDependency in the React package (GATES territory).

**Coordinator actions**: central `schema generate` (new `CH/🧬️schema/🔣️path-paint` + S2's `🔣️gumball-meta`/`🔣️gumball-dispatch`); layout `describe`; launch rows for the new `verify layout-frame-selection` segment (target still missing in LA `📋️project.json`) and the two Python corpus scripts; re-activate layout React 6079 + wgpu 6179 (and fem 2d), then a visual wgpu probe (Blueprint frames, guides, dashed inherited frames, text runs must paint — they were invisible boxes at the origin).

---

### S3-CONTROLS — forms, gis, energy, playbook, norm-registry controls; press/scrub glue (unified press ledger)

| Field | Value |
|---|---|
| Report | `📓️w3-t2-controls-report.md` (553 lines); last section `### S3.12 Playbook on the child lane (design §20.15; coordinator 11:50) — design` (L520–553, DESIGN ONLY); owed `### S3.11` (L511) + `### S3.9` (L456); coordinator actions `### S3.6` (L409) |
| Owns | `✏️s/🔌️plugins/{📋️forms,🗺️gis (terrain/map),🔋️energy,📖️playbook}`, `OS/🔌️plugin/🛠️tool-machine/🦀️.rs` (`ToolMachineRuntime.presses: ScrubLedger<PressLeaf<M,CM>>`, `PressLeaf::{Member,Child,Config,WindowConfig}`, `settle_press`), `🔌️plugin/🪟️window/🎚️config/🦀️.rs` (`WindowConfigMutation: Clone`), `🧰️framework/🔨️modules/🛠️tool-machine` (`ScrubLedger::send` transactional, Rust + TS twin), plugin tests `🧪️scrub`, `🧪️typing`; energy 13 per-field leaves + 130 witness quintets |
| State | SOURCE-COMPLETE-UNVERIFIED for plugin crates. Verified: `cargo test -p semio-framework-tool-machine --lib` 33/33 (11:20); tool-machine TS `bun test` 34/34 (11:12); plugin `cargo check --lib --tests --features artifact-app-testing` + wasm32-wasip2 pass (11:38); plugin `cargo test … -- tool_machine:: tool_run_tests::` 39 pass / 4 baseline FAIL (11:39–11:45; the 4: `tool_run_panel_of_a_running_run_is_the_shell_fixture`, `tool_run_reconfigure_resume…`, `tool_run_settings_changed…`, `tool_run_window_settings_reads…` — baseline per `📓️w2-a-report.md` §6.4 wp-c13); `schema mutation-payloads --under ✏️s/🔌️plugins/🔋️energy` 0 findings, 600/600, 301/301 witnessed, 138 negatives rejected; Python second implementation 26/26 vectors agree. Playbook §20.15 conversion NOT STARTED (design only) |
| Last peer blockers | stdio `epw` (139 errors) / `zip` ValueError sqlite — energy + forms crate tests blocked; plugin test-target fixtures (fixed by INFRA 11:38) |

**Owed verification (verbatim, S3.5/S3.9; gated, one cargo each)**
1. Energy fixture writer then full lib: `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-energy-model --lib -- writes_the_committed_vector_when_requested`, then `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-energy-model --lib` incl. 26 vectors (Rust writer must reproduce authored quintets byte-for-byte modulo key order), `a_history_edit_of_an_earlier_site_press_keeps_a_later_press_of_another_field`, `a_config_press_is_one_config_edit_a_cancel_is_none_and_neither_is_a_history_row` (L4 law counting ALL rows — red until W2A drops config rows), scrub + F-4 laws
2. `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- tool_machine:: tool_run_tests::` (done 39/4; re-run for `every_lane_of_a_press_rides_one_ledger_step` after changes)
3. Forms: `field_transactions change_block_field block_field try_value` (C2 frozen-press proof; `[DEBUG]` removed; harness fix `settle_registered_typed_operation` takes UI-progress frames)
4. gis terrain/map, playbook, norm registry contract crate tests
5. `cargo check` wasm32-wasip2 of the five plugin crates
6. `bun ./📜️script.ts schema mutation-payloads --under ✏️s/🔌️plugins/🔋️energy`

**Open items to implement**
- **Playbook §20.15 child-lane conversion** (8 `parentLeafReadsChild`: add-step, remove-step, move-step, update-step, add-block, remove-block, move-block, replace-block): `flow` child (`s.stdio.semio@v1/flow`) = single source of truth (step = node kind `step`, params `blocksJson`, order = chain of `sequence` edges `seq-<a>-<b>`); delete the `document` child slot; writers emit `ChildEmit::of::<SemioFlowSnapshot>("flow", id, leaves)`; delete the 8 parent leaves + registries/fixtures/oracles; `change-title` stays the only parent leaf; readers `playbook_composed_spec(snapshot, children)`; delete `PlaybookWorkingScene`, `attach_playbook_steps`, `seed_playbook_scene_json`; stable child ids `playbook-flow`/`playbook-demo-flow`; laws: child-leaf vectors per verb + `composed_reload_law!("playbook", …)`. Interim gaps: txt/json export/import + inference see no steps until AGNOSTIC W-a/W-b land.
- CLOSURE-5 (audit): `TransactionRef` ids not unique per press (scrub path mints `authoring_clock(0)`; `mint` hashes `(actor, hlc, tool)`) -> feed per-instance monotonic counter as `logical` (owner CONTROLS).
- Two-phase release through the publication ladder (a release refused DOWNSTREAM of settlement surfaces as Fault with both lanes unpublished) — open design item.
- Coordinator-routed: S3-NORM hand labels in norm app-surface `commit_snapshot(_fields)` (`📇️registry/🧬️contract/🖥️app-surface/🦀️.rs:1582,1590`) — done by NORM per its report; fem playback key drop — done by SPATIAL.
- 7 truncated energy leaf dirs join taxonomy `projection-member-unresolved` class (153 in energy, REPO-PATH-BUDGET hand-off).

**Coordinator actions (S3.6)**: central `schema generate` (catalogue 13 new energy leaf scopes `s.energy.model.mutation.change-site-{latitude,longitude,elevation,time-zone,north-axis}`, `change-ground-temperature-{building-surface,shallow,deep}`, `change-run-period-{start-month,start-day,end-month,end-day,year}`; drop 3 deleted `update-site`, `update-ground-temperature`, `update-run-period`; clears 16 `schema mutation-inputs` findings); `describe` energy (`set-site` args no longer required), playbook (old `updatePlaybook` describe text), forms (`change-block-field` roster); activation before any browser probe of press changes (colour, ring, NodeGraph sliders, config presses).

---

### S3-TEXT — writer, trinity (rewriting + jack), vcs, stdio text editors (typing runs N7/N8, patch leaves, node-graph rows)

| Field | Value |
|---|---|
| Report | `📓️w3-t2-text-report.md` (417 lines); last sections `### S3.4 Open items` (L361) and `### S3.5 Coordinator actions and peer breaks` (L384–417); changes `### S3.3` (L239) |
| Owns | `✏️s/🔌️plugins/{📝️writer, 🔱️trinity (♻️rewriting, jack), vcs}`, stdio text editors md/html/txt/binary/deflate (net leaves on `snapshot_edit_patch`), `OS/🔨️modules/🛠️tool-machine` typing machine, `🖱️ui/🎬️scene/✂️text-splice`, wgpu text editor N8 (`end_every_text_editor_typing`, `blur_text_editor`, `host-page-hidden` message + `semioWgpuHostPageHidden` door), plugin `dispatch_emit_inner` ordering (N7 root cause) |
| State | SOURCE-COMPLETE, TS/schema/lint/Python VERIFIED, Rust runs OWED. Verified: typing machine TS twin 33/0 (20 340 expects); Rust tool-machine 31/0; text-splice bun 69/0 + Rust `-- text_splice` 10/0; presence 9/0 + 41 neutral Rust/TS vectors; React vitest `browser-frame-transport text-carets echo-pack` 3 files/71; stdio md/html/txt/binary net-leaves 51/0 (oracles markdown-it, parse5, jsdiff); `schema mutation-payloads` 0 findings across 9 scopes (rewriting 17/17 leaves witnessed); `schema mutation-inputs`: writer/vcs/jack 0, rewriting 20/21 described + 1 `refUnresolved` + 4 `leafUncatalogued` (central generate); `binary64-transport` law 2/0 (79 expects); `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` GREEN 10:54 (854 warnings, N8 compiles) |
| Last peer blockers | kernel/plugin earlier; stdio crates (`--features <each>/component-app-assembly` needed to compile editor modules: without it 0 editor tests); **peer migrating rewriting document off `beforeFixtureJson` to typed `workingGraph`** (since 10:36; boundary test `📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts:58` red; 6 TS2353 `newBeforeFixtureJson` in `↩️inverse/🟦️.ts` mirrors of ✂️delete-working, ✋️drag-working, ➕️add-working, 🔌️connect-working, 🩹️patch-working, 🪚️disconnect-working) |

**Owed verification (verbatim, S3.4; one gated cargo each, private `target-nde-s3-text`)**
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-writer-writer --lib` (N7: six `typing_runs_tests` + `text_edit`)
2. `cargo test -p semio-framework-plugin --lib -- typing tool_machine` (`dispatch_emit_inner` reorder)
3. `cargo test -p semio-framework-os-renderer-wgpu --lib -- text_editor` (N8 law) + `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown`
4. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-trinity-rewriting --features component-app-assembly --lib` (3 new leaves' quintets, structural correspondence, retirement, node-graph laws, add-node law, P1 `a_history_edit_of_a_binary64_offset_validates_and_replays`, harness)
5. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-md -p semio-s-artifact-stdio-html -p semio-s-artifact-stdio-binary -p semio-s-artifact-stdio-deflate -p semio-s-artifact-stdio-txt --features <each>/component-app-assembly --lib`
6. After the typed-`workingGraph` peer migration: re-run P1 law, payload/input lints and `tsc -p T/🧪️w3-t2-text-typecheck.tsconfig.json`; confirm TEXT working-graph leaves (`connect/disconnect/add-working`, relative inverses, add-node verb, node-graph row map) were carried.
7. TS: `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` (33), `bun test ./🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🟦️.test.ts` (69), `bun ./📜️script.ts schema mutation-payloads|mutation-inputs --under <scope>`.

**Open items to implement**
- **§20.15 jack** (8 `parentLeafReadsChild`: `jack_working_scene` x3, `nodes` x3, `base_property_value` x2, routed 11:43) — NOT STARTED.
- **`wordOnlyFloat` x6 trinity** (jack CameraJson x/y/zoom via `replace-query-result` `/result/graphFixture/camera/{x,y,zoom}`; rewriting `edit-before-fixture` `/newWorkingGraph/camera/{x,y,zoom}` + snapshot schemas) -> `Binary64Transport` once the migrating peer is quiet >= 30 min; also catalog float-as-`bits` findings (27 trinity, resolved by P1 for the 5 layout/drag leaves).
- trinity rewriting `disconnect-working-edges`/`delete-working-nodes` undo with the whole base graph — exact relative undo needs a `restore-working-edges` leaf (T2 rest).
- `snapshot_edit_patch`/`snapshot_edit_set_snapshot` stamp literal `"Edit document details"` (G7) — contract owner (S3-STDIO) — closed by CLOSURE (`Emit.description` deleted).
- Writer `setSnapshot`/`setSnapshotJson`/`setFixtureJson`/`openDocument` whole-document load intents: genuine intents, kept (named).
- Remodel `change-stream-sync` refs its own `Binary64Transport` (STROKES tree).

**Coordinator actions (S3.5)**: central `schema generate` (`delete-working-nodes`, `connect-working-ports`, `disconnect-working-edges`, `add-working-node`; publish `framework/graph/manifest/property-value.json` into catalog documents so `change-parameter-binding /newValue` resolves; 40 stdio `patch-snapshot` leaves of other formats are STDIO's); `describe` writer, trinity rewriting (new leaves + node-graph rows), stdio md/html/binary/deflate; wgpu `generate-frame-worker` (new `host-page-hidden` message); re-activate wgpu lane to observe N8 live (blur by pressing outside, hidden by switching tabs).

---

### S3-STROKES — raster, remodel, wfc, process3d (streamed imports §15, parametric raster commands §17.2, gates)

| Field | Value |
|---|---|
| Report | `📓️w3-t2-strokes-report.md` (579 lines); last section `### S3.10 Gates to 0 (remodel + raster), geometry and mask fill as parametric commands — 10-03 11:00 → 11:48` (L502–579); streamed preview `### S3.9` (L442); audit K1–K4 `### S3.8` (L351) |
| Owns | `✏️s/🔌️plugins/🖨️raster` (leaves `fill-region`, `apply-filter`, `transform-image`, `fill-selection`; commands `🎮️commands/{🌈️apply-filter,🔄️transform-image,🫗️fill-selection,🌊️set-fill-tolerance}`; composite window transient `🖼️composite/🫧️transient`; options `☑️options/🪣️bucket`), `✏️s/🔌️plugins/📸️remodel` (window-owned import transient, blur gate in `⚙️engine/🏭️reconstruction`), `wfc` (5 variants), `process3d` (one config edit per seek), React `🖌️Paint2dHost`, wgpu `🗺️surface/🎨️paint` (`PaintEditCommand`, `RasterHost`), `EngineCanvas write_paint2d_edit/flush_paint2d_edit`, framework `Paint2dScene.fillTolerance` |
| State | SOURCE-COMPLETE; TS/schema/lint VERIFIED, raster/remodel cargo NEVER RUN this session (stdio gif/jpg/png/svg/tiff/bmp peer). Verified: React Paint2dHost 63/63 (twice); Paint2dHost editing twin bun 62/62; surface-idle-frames 3/3; engine-contract 699/699; raster config twin 19/19; `cargo test -p semio-framework-pixels -p semio-framework-ui --lib` pixels 58/58; `scene_records_serialize_to_golden_json` 1/1; wgpu renderer wasm32 check exit 0 (172 warnings); gates: remodel payloads 136/136 (36/36 witnessed, 4 invariants) 0 findings, inputs 56/56 0; wfc 85/85 0 / 153/153 0; process 15/15 0 / 25/25 0; raster payloads 23/23 clean but **4 unwitnessed**, inputs 49/49 + **4 `leafUncatalogued`**; `schema mutation-labels` 0 findings (3098 labels); twins == schema probes 0 failures; taxonomy new dirs clean (command dirs `directory-kind-unresolved` = same class as all raster command dirs) |
| Last peer blockers | stdio peers (gif/jpg/png/svg/tiff/bmp, ValueError/sqlite migration) block every raster+remodel `cargo`; workflow (fixed 10:56); `🖱️ui/🎬️scene/🧪️tests/🔬️scenes-value-round-trip/🦀️.rs:41` `ValueError::new` one arg (peer) blocks `-p semio-framework-ui-scene --lib` |

**Owed verification (verbatim; one gated cargo each)**
1. raster `--lib`: `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-raster-raster --lib` (package id per report "raster `--lib`"; new laws in `🎮️commands/{🔄️transform-image,🫗️fill-selection}/🧪️tests`, editor unit counts 31/32/31, `🧪️fill-tool-transactions::a_bucket_click_is_one_transaction_whose_seed_tolerance_and_colour_time_travel_edits_replay_exactly`, `🧪️stroke-stream-transactions`, `fill_tolerance_shared_vectors_round_trip_and_restore`, S3.7–S3.9 lists)
2. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-raster-raster --lib -- --ignored emit_committed_fixtures` (4 leaves' quintets), then the 4 TS twin tests `bun test ./…/{🫗️fill-selection,🔄️transform-image,🌈️apply-filter,🪣️fill-region}/🧪️tests/🟦️.ts` and `schema mutation-payloads --under ✏️s/🔌️plugins/🖨️raster` (expect 4 unwitnessed -> 0)
3. remodel: `cargo check -p semio-s-artifact-remodel-remodeling --lib --tests` and its lib tests (`the_blur_gate_rolls_in_the_import_state_and_refuses_a_blurred_frame`, `a_new_import_over_a_live_one_in_the_same_window_is_refused`, §15 import laws)
4. wgpu: `cargo test -p semio-framework-os-renderer-wgpu --lib -- paint2d` (`native_bucket_click_is_one_fill_region_intent_in_layer_pixels`, `paint2d_bucket_press_publishes_one_fill_region_click_in_layer_pixels`, `native_long_stroke_streams_batches_and_commits_the_rest_under_one_press`, `native_dropped_streamed_stroke_leaves_the_abort_of_its_press`, `paint2d_long_stroke_streams_ticks_then_commits_under_one_press`)
5. `cargo test -p semio-framework-ui-scene --lib` (paint-2d lane contract)
6. TS: React `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🖌️Paint2dHost`; `bun test ./🧰️framework/…/🖌️Paint2dHost/✍️editing/🧪️tests/🟦️.ts`

**Open items to implement**
- **Delete the dead `editPixels` / `editMask` commands** (decision status L682; no host dispatches them any more): `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/{🎨️edit-pixels,🖌️edit-mask}`, their app_commands rows (renumbering the binary command variants is fine — greenfield; re-seal fixtures; describe), tests (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`), `🖼️assets/🔄️replacement` test, `🧫️fixtures/🔏️publication-authority/🔣️.json` — still PRESENT on disk (confirmed 2026-10-04); do after a repo-wide `/usr/bin/grep -rn` zero-reference proof (rule 32).
- Delete stale placeholder `✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🫧️transient/📌️.empty.md` (directory now holds `🦀️.rs`).
- K3 (routed to S3-W2A): a derive for window-transient partitions (remodel transient hand-writes Mutation/Diff/Retire/Dsl/pack).
- K5/§17.2: streamed stroke preview DONE in source (S3.9); wgpu bucket + gesture replay law DONE in source (K4).
- Remodel `change-stream-sync` refs its own `remodeling/artifact.json#/$defs/Binary64Transport` (not the canonical framework def) — note only.
- `requestMediaFrames` has no host cancel in either shell (shared with W2B).

**Coordinator actions**: central `schema generate` (raster leaf catalog: fill-region, apply-filter, transform-image, fill-selection; remodel schema hashes); describe + re-activate raster (new verbs `setFillTolerance`, `applyFilter`, `transformImage`, `fillSelection`, `paintStroke{phase,reason,gesture}`, utility `paintBucket`, Composite window transient owner) and remodel (schemas); decide the editPixels/editMask deletion (already decided: delete).

---

### S3-PROCEDURAL — generation2d / generation3d (World3dHost consumer, §19 change-widget-input, §20.9/20.10/20.11, P4, S4)

| Field | Value |
|---|---|
| Report | `📓️w3-t2-procedural-report.md` (471 lines); last sections `### S3.7 Open items` (L434) and `### S3.8 Coordinator actions` (L465–471); decisions `### S3.9` (L308), `### S3.10` (L348) |
| Owns | `✏️s/🔌️plugins/🌀️procedural` (gen2d, gen3d: leaf `🎛️change-widget-input` (tag 19, 10 variants), `🧭️transforms` (`GumballRefusal` 12 codes, `gumball_fault_notices`, `gumball_splice`), `edit_mesh_selection::mesh_edit_emit`, `tool_intent_kinds`, `default_neuron_params_from_info` in flow snapshot, `unwired_params` in `🧠️neural/⚙️engine/🦀️.rs:2639`), DEV `OS/🧑‍💻dev/…generation3d` laws, generators `🧪️s3-procedural-change-widget-input.py` (dry-run default, sha256 stamps in `🧪️s3-procedural-generator-stamps.json`), `🧪️s3-procedural-normalize-examples.py` |
| State | SOURCE-COMPLETE, mostly UNVERIFIED in Rust. Verified: engine `cargo test -p semio-framework-os-kernel-neural-engine --lib` 67/0 incl. `a_wire_shadows_the_literal_its_neuron_records_for_that_port` (06:40, §20.10); `verify semantic-wire` checks=83 (gen3d pkg, Ajv); `canonical-architecture` pass (gen2d 18+5, gen3d 334/2921/5/47); payloads 55/55, 53/53 witnessed, 0 findings; inputs 80/80, 1 finding (`change-widget-input` `leafUncatalogued`); strict tsc 0 errors on new leaf twins; taxonomy clean; generator idempotence proof (`--bogus` exit 1, `--write` 1 file, rerun 0). Rust runs since 10-02 11:55 WRITTEN, NOT VERIFIED (check-8/9: stdio zip/gltf/svg errors) |
| Last peer blockers | stdio zip (2), gltf (13), svg (6) ValueError (peer migration, since 11:03 INFRA: only non-stdio swept); os-kernel IoError earlier |

**Owed verification (verbatim, S3.7 item 1; one gated cargo at a time; private target `target-nde-s3-procedural`)**
1. `cargo check -j 4 --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-procedural-generation2d -p semio-s-artifact-procedural-generation3d --features <both>/component-app-assembly --lib --tests --keep-going --message-format=short` and `-p semio-s-composition-laws --tests`
2. `cargo test -p semio-framework-os-flow --lib` (incl. `an_inserted_operator_wired_into_a_defaulted_input_evaluates_its_wire`, `an_inserted_operator_records_every_declared_default_input`, `host_with_two_node_chain` laws)
3. `cargo test --lib` for gen2d and gen3d (`--manifest-path ✏️s/Cargo.toml`, features `component-app-assembly`)
4. DEV `--test generation3d-app-laws` (streamed-gumball law + §19.1 first-grab label; §19.4 `a_first_component_grab_inserts_defaults_then_one_input_per_chosen_channel`; corpus law `the_world3d_gumball_live_protocol_lands_as_its_guest_edits`; preview law; mesh-edit/knife P4 row tool + label `Insert “Extrude Mesh Faces” (id)`; `every_bundled_example_records_every_declared_default_of_its_operators`; `every_gumball_refusal_code_has_a_localized_notice`, `every_gumball_refusal_is_a_named_fault_code`)
5. DEV `--test generation2d-example-export` (`the_bundled_example_records_every_declared_default_of_its_operators`)
6. `cargo check --target wasm32-wasip2` for both plugin crates
7. `bun ./📜️script.ts verify semantic-wire` (gen3d `📦️packages/🦀️rust`), `bun ./📜️script.ts canonical-architecture` (gen2d/gen3d packages), `bun ./📜️script.ts schema mutation-payloads|mutation-inputs --under ✏️s/🔌️plugins/🌀️procedural`

**Open items to implement**
- **Example normalization (§20.9)** after item 1 compiles: run the two temporary `[DEBUG]` printers `debug_print_normalized_examples` and `debug_print_normalized_example` with `--nocapture` into `🗑️generated/s3-procedural/`; `T/🧪️s3-procedural-normalize-examples.py <capture>...` (dry run prints diff, refuses any asset whose authored lines would change) then `--write`; delete both `[DEBUG]` printers; re-run the two example laws. (28 gen3d operators `params=[ ]` across 9 gen3d examples + 2 gen2d.)
- Live-only e2e: with `gumballLiveDispatch` on shapes, React host must not re-anchor the gumball to the moving answer mid-drag (shapes have no `gumballTarget`; fem 3d sets one) — verify on a 6018-class serve.
- Flow DEFAULT document (`🌊️flow/…/📸️snapshot/🦀️.rs:276`) is not self-describing (`math.add` `params` empty): `change-widget-input` on `add@b` -> `mutation.target-missing`; normalizing is FLOWCAD's call (gen3d `default_generation3d_snapshot()` is normalized).
- Fault-notice gate residue (S3-NOTICES): `schema fault-notices --under ✏️s/🔌️plugins/🌀️procedural --census`: 12/19 guest codes labelled; 126 pre-existing findings (3 missing notices `generation3d.io.export`, `generation3d.io.import-accept`, `generation3d.widget.add`; 2 two-segment codes `generation2d.child-projection`, `generation3d.child-projection`; 2 framework-namespace codes `mutation.target-mismatch`/`-missing`; 118 anonymous `Fault::from(text)`; 1 `faultNoticeDescriptor` needs describe).
- W2A-12 (audit): `tool_intent_kinds` parsing `<appId>#<toolId>` via `GENERATION3D_EDITOR_APP_ID` strip prefix (`✏️editor/🦀️.rs` ≈1959) — switch to the tool id part once W2A passes it.
- gen3d CONFIG leaf camera `uiInvalid` (12 findings) — fixed by AGNOSTIC transport reader (gone).

**Coordinator actions (S3.8)**: central `schema generate` (`change-widget-input` `leafUncatalogued`); `describe` the procedural composition (`🌎️hub/🧩️compositions/🌀️procedural/🔣️.json`): gen3d new leaf, `channel` option source, `setWidgetInput`/mesh-edit semantics, mesh edits as tool transactions, context-menu delete row, 12 named gumball codes + editor `faultNotices` (12 rows); gen2d `nodeGraphEdit` rows; activate procedural lanes for the live consumer check.

---

### S3-GRAPHS — dag, sequence, imperative/procedure, space, reasoning (wires/math split off), shared node-drag helpers, §20.15 child-lane

| Field | Value |
|---|---|
| Report | `📓️w3-t2-graphs-report.md` (458 lines); last sections `### S3.3 Open items` (L409) and `### S3.4 Coordinator actions` (L443–458); §20.15 in `### S3.1b` (L333) |
| Owns | `✏️s/🔌️plugins/{🕸️dag, 🎬️sequence, 📜️imperative, 🪐️space}`, `OS/…/♾️infinite/🗿️artifacts/🕸️dag` (peer-co-owned journal), shared helpers in `🛠️tool-machine` (`node_graph_edit_rows`, `NodeDragEmit`, `authoring_clock`) + plugin `Emit::node_drag_child`, plugin law folder `🧪️node-drag-history`, per-aggregate drag history laws; sqlite codecs of dag/mathematical/sequence moved to `ValueError`; mathematical publication-authority gate fixes |
| State | SOURCE-COMPLETE for sequence §20.15 (SequenceMutation uninhabited: 8 leaves + 2 subsets deleted; drag = `Emit::node_drag_child` on the `content` member); dag/wires/mathematical/imperative §20.15 split: GRAPHS keeps graph `drag-nodes` child leaf -> procedure -> dag; S3-WIRES and S3-MATH launched 12:03/12:07 (see their sections). VERIFIED: tool-machine `cargo test -p semio-framework-tool-machine --lib` 31/31 (12:55, incl. 3 `node_graph_edit_rows_tests`); dag Python oracle 34/34 scenarios (17 kinds), wires 24/24 (12 kinds); wires TS twins + Ajv 23/23; `schema mutation-inputs\|payloads` 0 findings in 11/12 runs (dag 43/43 inputs, 57/57 payloads, 20/20 witnessed; mathematical 37/37, 19/19, 18/18; sequence 17/17, 40/40, 8/8; space 9/9, 8/8, 6/6; workflow 41/41, 25/25; reasoning payloads 39/39 + inputs 27/27 with 2 `leafUncatalogued`); mathematical publication-authority green (routes=8 Ajv strict, 3 hostile mutations rejected, 2/2 example bun tests); `cargo check` (ROOT workspace) tool-machine + plugin lib 0 errors (10:46–10:50). Triage: `test-12` mathematical 405/6, wires 207/13, sequence 218/6 |
| Last peer blockers | **dag never compiles** (depends on stdio-svg/zip, red from ValueError drift: zip 46, svg 6; STDIO's trees); plugin lib-test 129 errors in peer fixture ValueError (fixed 11:38); sqlite native row/byte limits not enforced (wires x2, mathematical x2, sequence x2: `decode_sqlite_snapshot_native` under `max_rows: 30` returns Ok — shared store/sqlite-snapshot cause) |

**Owed verification (verbatim, S3.2/S3.3; one gated command at a time)**
1. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-{dag-dag,mathematical-equation,sequence-sequence} --lib --tests --keep-going` plus wires (`semio-s-artifact-reasoning-wires`, verified crate name) and procedure (`semio-s-artifact-imperative-procedure`); hub space `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-space --lib --tests`; OS workflow `-p semio-framework-artifact-workflow-workflow --tests`; infinite dag `-p semio-framework-artifact-infinite-dag --lib` (for `bundled_demo_fixture_is_canonical`)
2. `cargo test --lib` (private `target-nde-s3-graphs`) of dag, mathematical, sequence, wires, hub space, workflow
3. `wasm32-wasip2` checks of `semio-hub-{dag,mathematical,sequence,space,reasoning}`
4. both schema lints `--under` each tree; `bun ./📜️script.ts test parity exhaustive --case mutate-dag-1` / `mutate-wires-1` (generated hosts); `bun ./📜️script.ts test contract exhaustive --case mutate-dag-1` (dag catalog 0 breaches; 3 `manifest-only-mutation` = stale runtime inventory cache)
5. ROOT-workspace `cargo test -p semio-framework-tool-machine --lib` + plugin crate `node_drag_history` users; the X2-touched crates (wfc 2d/3d, flow, gen2d/gen3d, trinity rewriting, raster, process3d, cad, wfc bitmap, note, shooting, layout, puzzle 2d/3d, fem 2d/3d, drawing, lowpoly) `--lib --tests`
6. `python3 T/🧪️s3-graphs-dag-oracle-vectors.py` (34/34 + 24/24), `bun -e verifyMathematicalPublicationAuthority(...)`

**Open items to implement**
- **§20.15 dag** (17 `parentLeafReadsChild`, `dag_working_scene`), **imperative/procedure** (4: `resolve_steps` x3, `procedure_working_scene` x1): dag/wires compose `s.stdio.semio@v1/graph`, which has absolute `move-node` but no relative drag -> a graph `drag-nodes {targets, dx, dy}` leaf (flow twin) is needed (assigned to GRAPHS), dag's bridge must make native `position`/`label` authoritative on decode (today `dag.node` JSON property also carries x/y); procedure composes a flow child (path) + text child; sequence order = edge chain.
- **Sequence node drag writes no transaction row** (`a_node_drag_record_is_one_child_transaction`: step moves, 0 transaction rows); `[DEBUG]` probes removed at 10:5x; decide among: empty authoring seed in the retained step's `operation`, empty `artifact_mutations` at `drag_transaction`, or composite publication dropping the parent row's `transaction` — needs the next sequence test run.
- Sequence census F: `work_items: 1` with multi-row delete inverse (`DeleteStep` inverse `sequence_delete_inverse` = 2*steps + edges rows) — parent preparation dead (delete) or derived footprint (CLOSURE; sequence parent vocabulary now empty so this dies).
- Dead sequence `🏭️generator` (+2 standalone engines, nx project `@semio-tech/fixture-generator-sequence-1-any`, 2 launch.json options) — delete; central `🔣️taxonomy.json` still lists 4 deleted case names (coordinator regenerate); `csv-rfc4180-reader` / json-carried oracles lose mutate capabilities.
- Readers still on `local_owner`: `topology` inference, json/md/csv serializers (`try_to_host_snapshot`) — AGNOSTIC W-a/W-b.
- 12 bundled `.dsl.semio` assets outside GRAPHS' trees in bare-record form (N13; INFRA closed: carriers 0).
- Stale doc comments mentioning deleted `to_dsl_value` bridge in mathematical snapshot rework (peer-owned, left).
- Wires: `history_edit_inputs_resolve` `$ref` `https://json.schemas.assets.semio-tech.com/framework/value/schema.json` unresolved (create-node, connect-nodes) — peer schema split; test-12 mathematical `history_edits_end_to_end` (change-coefficient), `math_document_text_round_trips_through_store`, wires `reorganize_start_complete_finalize_is_one_undo_entry` not triaged.
- Peer: sqlite native controls not enforced (see blockers).

**Coordinator actions (S3.4)**: re-describe dag, mathematical (new `addNode` verb), sequence, hub space, reasoning (wires gained 2 leaves); central `schema generate` for the two wires leaves; `bun ./📜️script.ts test inventory --artifact s.dag.dag --standard 1 --subset any` (and `s.mathematical.equation … graph`, `s.reasoning.wires … any`) so the inventory cache lists the new leaves; re-activate dag/mathematical/sequence/space/reasoning; **graph catalog contract migration**: the 8 plugin `🛂️manifest/📇️outputs.json` (draw, puzzle 2d/3d/5d, writer, wires, trinity rewriting/jack) lack `contractId`/`policy` -> graph generate refuses; migrate then regenerate; sqlite-trait `ValueError` sweep wrappers still owed in procedural gen2d/gen3d, flow codecs (`(||…)().map_err(ValueError::into_message)`), stdio zip/svg.

---

### S3-WIRES — reasoning/wires §20.15 conversion (launched 12:03; NO REPORT FILE)

| Field | Value |
|---|---|
| Report | none on disk (`📓️s3-wires-report.md` absent; `🗑️generated/s3-wires/` empty, created 12:03). Only trace: status L686 ("launched S3-WIRES a628087015b0f186b (wires)") |
| Owns (target) | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires` (parent leaves reading `local_owner`: `find_board_node` x8, `find_board_edge` x2, `wires_working_board` x2 = 12 `parentLeafReadsChild`) |
| State | **NOT-STARTED** (verified 2026-10-04: `pub enum WiresMutation` at `🧬️schema/🧬️mutations/🦀️.rs:79` still carries the 12 parent variants; no new graph-child leaves). NOTE: files under wires + mathematical show mtime Oct 4 00:32 — a peer/auto sweep, not the WP; do not use mtimes as a progress signal, use `git diff`/content |
| Prior wires facts (from GRAPHS) | wires gained `move-nodes` and `set-node-positions` (relative drag + absolute inverse) — 2 `leafUncatalogued` pending central `schema generate`; Python oracle 24/24 (12 kinds); wires lib 207/13 triage: composed-child materialization gap (`wires-drag-child-not-materialized` at `W/✏️editor/🦀️.rs:360,407` `local_owner::<WiresWorkingScene>()` — dies with §20.15), 2 sqlite controls |
| Owed / to do | (1) convert wires to child-lane leaves in the `s.stdio.semio@v1/graph` child (graph `drag-nodes {targets,dx,dy}` leaf from GRAPHS), empty/slim parent vocabulary, readers compose on read; (2) delete parent leaves + fixtures + oracles after zero-reference proof; (3) `composed_reload_law!("wires", …)`; (4) verification list as GRAPHS items 1–4 for wires; (5) catalog: `📇️outputs.json` lacks `contractId`/`policy` |

---

### S3-MATH — mathematical §20.15 conversion (launched 12:07)

| Field | Value |
|---|---|
| Report | `📓️s3-math-report.md` (68 lines); last section heading `### 1. Design note — where is the equation graph's single source of truth?` (sections 2–5 literally "(in progress)"); only status line: "12:10 design note written, choice (a) sent to `main` (no framework contract change); implementation started." |
| Owns | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation` (`MA`; subsets `SUB`): 16 `parentLeafReadsChild` (`equation_graph` x12, `equation_geometry` x4) |
| State | **DESIGN-ONLY / NOT-STARTED in source** (verified 2026-10-04: `✏️s/…/➗️equation/…/✳️any/🧬️schema/🦀️.rs` mtime Oct 2 12:24, no `graph: EquationGraph` field, `EquationWorkingScene` still present) |
| Approved design (status L691) | model (a): `EquationSnapshot` gains `#[state(artifact)] graph: EquationGraph` + `geometry: EquationGeometry` (schema-first json/ts/graphql/proto twins, DSL grammar, pack, sqlite, fixtures); notation/results/computed = content-addressed DERIVED child handles `child_id = store::content_id(<slot>, canonical derived pack)`, re-minted per diff, opened via `follow_derivable_children` -> `genesis_child_pack`, never edited; delete `EquationWorkingScene`, `equation_scene*`, `require_equation_scene`; relative `move-points {indices, dx, dy}` replaces absolute `move-point {index,x,y}`, inverse ONE absolute row `set-point-positions {positions:[{index,x,y}]}`; delete `move-point` + fixtures/oracle vectors/feature rows (rule 32); `composed_reload_law!` |
| Owed | everything: implementation (sections 2–5 of its report), cargo check/test of `semio-s-artifact-mathematical-equation`, gate `schema mutation-editability` (expect 16 -> 0), payload/input lints, `🥒️.feature` rows, Python oracle, taxonomy report, wasm32 check of `semio-hub-mathematical`; coordinator: describe mathematical + `test inventory --artifact s.mathematical.equation … graph` |

---

### S3-E2E — live time-travel probe (puzzle 2d, React 6012 + wgpu 6112)

| Field | Value |
|---|---|
| Report | `📓️w3-e2e-report.md` (689 lines); last section `### S3.7 Run 4 — results` = "Pending: no serve was up when Phase A closed." (L687–689); plan `### S3.6 Run 4 plan` (L637) |
| Owns | permanent probe `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts` (3.9 k lines; `runTimeTravelCli`), router `🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts` (`verify time-travel …`), nx target `@semio-tech/framework-os-dev:time-travel` (`🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`), 2 seed rows in `.vscode/🧩️launch.seed.jsonc` (`⚖️gate⏪️time-travel⚛️react` order 411.2781 / `⚖️gate⏪️time-travel🧊️wgpu` 411.2782) |
| State | Phase A DONE (written, type-clean, taxonomy clean, router loads); **Phase B = Run 4 NEVER RUN** (no serve). Live evidence on disk is still Run 2 (React, en+de, 85 PASS / 3 FAIL each, 0 uncaught, 0 guest faults, build of 09-30 activation #4; dirs `🗑️generated/e2e/probe-2026-09-30T16-49-01` and `…16-51-11`, report `📓️w3-e2e-report.md` L247–300) |
| Last peer blockers | none own; needs activation + serve (activation attempts 2–24 all failed on peer breaks; see INFRA/W2C) |

**Owed verification (verbatim, S3.6 batches; one foreground Bash call <= 9 min; before each batch `curl` the port)**
```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript && \
bun ./📜️script.ts verify time-travel --serve <url> --renderer <r> --locales <l> --chords en,de --only <batch> \
  --out <T>/🗑️generated/s3-e2e/run4 > <T>/🗑️generated/s3-e2e/run4-<r>-<l>-<batch>.txt 2>&1; echo exit=$?
```
Batches: A `1,2,3,4,5,6,7,9`; B `1,2,3,4,5,reload,9`; C `1,8,10,9`; D `1,2,11,12,9`; E `1,13,reload,9`; F `1,14,15,9`; G `1,16,9`; H `1,17,9`. Order: React :6012 A–H in `en` then `de`; wgpu :6112 `--explore --locales en` (calibrate mirror keys) then A–H `en`/`de`. Coordinator single detached alternative (~1 h/renderer): `bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6012/ --renderer react --locales en,de --chords en,de --out <T>/🗑️generated/s3-e2e/run4-full`. Typecheck: `bunx tsc --noEmit -p T/🗑️generated/s3-e2e/probe-tsconfig.json` (0 errors); `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel` (clean).

**Open items**
- **Everything live**: Run 4 on current build, both renderers, en+de, desktop + 375 px phone (step 14) + 768x1024 tablet (step 15). Predictions to compare (S3.6 table): `g9-every-mutation-of-the-long-transaction-is-reachable` (probe reads N1 as cap 8 — stale after N1; W1E-6 requires deriving from `window.total`), `g9-edit-during-replay-is-refused` PASS via `disabled`, `g10-*` base-move-while-editing UNKNOWN (never tested), step 9 wgpu `rejection-notices-carry-their-code` PASS when no unannounced refusal.
- Positive presence case (`g10-presence-travels-only-through-a-hub`, ⏪ badge, "is editing" notes) needs a hub-backed space serve (`🚀️local-hub` + serves joined with `?space=`) — only the coordinator can start; folder route is last-writer-wins whole-archive and carries no presence.
- Probe peer type errors reached via router: `📇️directory/🧪️testkit/📡️client-probe/🟦️.ts:144` TS2741 `line`, `🔌️plugin/🏗️build/📥️installation/🟦️.ts:51` TS2345.
- axe-core not installed: a11y oracle = `aria-query` 5.3.0 + `dom-accessibility-api` 0.5.16 (coordinator decision); N14 hole closure verdicts (en chords, stepper hard-min, rotate/scale edit finalize, tablet, long history >=200 with progress/Cancel/Replay again, second peer over one shared folder) are WRITTEN, never run.
- Peer gap: probe must be updated for N17 (`HistoryPatch.reprojection {kind}`), N1/N2 windows, stepped load, `history.replaying`, `document.loading` notices.

**Coordinator actions**: activate `activate-puzzle2d-react-dev activate-puzzle2d-wgpu-dev` (run-many, `-p @semio-tech/framework-os-dev`); serve from the MAIN session: `🔁️serve-supervisor.sh` with `S_OS_PORT=6012 S_OS_RENDERER=react` and `6112` + `wgpu` (`setopt no_bg_nice;` `nohup … & disown`), `curl` -> 200; regenerate `.vscode/launch.json` from the seed (2 new rows); then launch the E2E successor.

---

### S3-CLOSURE — delete amend/coalesce/bracket paths, derived fold footprints, closure gate `verify history-closure`

| Field | Value |
|---|---|
| Report | `📓️s3-closure-report.md` (311 lines); last section `### TREE GREEN core run (10-03 11:39–12:03)` (L293–311); coordinator wave steps `#### Final channel-bump wave` (L188–209), Step 8 acceptance + coordinator actions (L226–259), follow-ups (L261) |
| Owns | closure gate (`policyHistoryClosureBreaches`, `historyClosureFindings`, wired into `VerifyScript.runGate`), `OS/📡️spr/🧵️channel`, store `next_edit` helper (no description param), derived footprint (`x-semio-inverse-rows` on 62 leaf schemas, `Mutation::inverse_rows`, `ArtifactStoreOneItemFootprint::for_leaf`, `for_ephemeral_item`), `Emit.description` / `Emit::commit` / `Emit::amend` / `Emit::amend_config` / `Emit::commit_config` deletions, `ArtifactCommand::AmendLast*` + `Edit.coalesce_key` (wire/digest/`.spr`/`.ops`/channel/manifest), manifest typegen (`TutorialDocumentEventKind` lost `coalesceKey`), history-label-reload fixture re-seal (oracle 5 cases), outbound-announcement law (now `steps >= 14`) |
| State | SOURCE-COMPLETE; CORE VERIFIED, PLUGIN LAYER UNVERIFIED. Gate census (10-03 06:35/11:25): `amend-emit 0 · amend-last 0 · coalesce-key 14 (bump wave) · preview-contract 0 · bracket-verb 11 (stale generated descriptors) · host-snapshot-bracket 0 · edit-literal 0 · release-plain-commit 0 · footprint-hand 0`; gate self-test 31/31; `Emit.description` + `Emit::commit` DELETED (label gate repo-wide 0); strict-Ajv vocabulary fixed (draft-07 dependencies; strict 9/9, engine-contract 12). Verified runs: `cargo test -p semio-framework-os-kernel --lib` **1254 passed / 2 failed** (1 CLOSURE's `outbound_announcement_tests::every_locally_authored_operation_is_announced_exactly_once` fixed to `>= 14` and re-run PASS 11:45; 1 peer `sqlite_snapshot_space_history_baseline::…_uses_same_caller_ledger`), 10 derived payload laws `semio_payload_law_*` ok (L3), plugin `--lib` check PASS 11:05 (147 warnings); TS history-label-reload oracle PASS (5 cases); digest re-seal oracle PASS |
| Last peer blockers | plugin lib tests `--no-run` 6 errors at 12:03 = `store::ReplayTurnBudget` / `ArtifactStoreInitializationEditAdmission::{Oversized,Duplicate}` landing (W1G/W2A callee-first; W1G confirmed green 12:05); `semio-framework-os-infinite` `DagExpandedPaths` missing (`🎲️board/…/🕸️dag/🦀️.rs:2243`, peer) blocks flow-vcs; stdio-epw red blocks the energy L4 law; stdio sqlite peer blocks all 34+45 plugin wasm32 checks |

**Owed verification (verbatim, report L255–259 + L293–311)**
1. Plugin batches `--target wasm32-wasip2 --lib` for the 45 step-4 crates + the 34 step-3 crates + `semio-hub-space` (hub: `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-space`).
2. Kernel lib tests (store unit, tool-transaction, outbound-announcement, canonical-edit, spr history/CLI/protocol-laws, flow-vcs): `cargo test -p semio-framework-os-kernel --lib`; `cargo test -p semio-framework-os-flow --lib -- flow_vcs` (`a_streamed_layout_drag_produces_one_edit`).
3. Plugin lib tests (runtime contract streamed laws, transaction fixtures, full-operation, time-travel, gen3d fold-contract): `cargo test -p semio-framework-plugin --lib` (private target).
4. L3 in plugins: derived payload laws (`semio_payload_law_*`/`mutation_inverse_rows_failures`) of the touched crates.
5. L4: energy `a_config_press_is_one_config_edit_a_cancel_is_none_and_neither_is_a_history_row` (counts ALL rows; RED until W2A drops config-lane rows).
6. Gate: `bun ./📜️script.ts verify history-closure --json` (`coalesce-key` must reach 0 after bump wave; `bracket-verb` 0 after describe); `exports_typescript` freshness: `cargo test --features typegen exports_typescript_bindings` with `SEMIO_TYPEGEN_OUT`.

**Open items to implement**
- **Final channel-bump wave (coordinator-run) — exact list, report L188–209** (see wave list in section 3 below): AppFrame tag 15 `coalesce_key`, `AppFrame::TransactionProposal.description`, `AppCommand::LoadDocument` (+ field-page arms, encoder, decoder tag 6 stays unassigned), `AppChannelClient.loadDocument`, TS twin, backbone-envelope-io tests, frame-worker regeneration, rebuild + describe of every plugin.
- Store wave (with W1G): delete `Edit.description` (store, `.spr` `F_EDIT_DESCRIPTION`, `.ops`, canonical digest), `ArtifactCommand::Apply.description`, `GroupMeta.description`, `begin_*apply_batch(description)`, agent `transaction_commit` label.
- Audit CLOSURE-4 (minor): property test per `perTarget` leaf at cap+1; localized refusal code `mutation.too-large` en/de for the new bounded ceilings (1025/768/258/4096) — currently raw English `plugin_sdk_fault` (`store_publication_fault`, `P` ≈24216); 87 hand-written aggregates default to 1 inverse row (silent).
- Audit CLOSURE-5: `TransactionRef` id uniqueness — CONTROLS owns.
- Wire `policyHistoryClosureBreaches` red until bump wave + descriptor regeneration (by design).
- Persistence/codec changes already made (detectable): `.spr` `HistoryEdit` presence bit 2 refused; `.ops` edit field `key` gone; `ArtifactCommand` binary ordinals 8/12 unassigned.

**Coordinator actions**: (1) final channel-bump wave then `coalesce-key` = 0; (2) `describe` demonstrator, fem, lowpoly, puzzle composition descriptors (lists stale `transformBegin/End`, `paintStrokeBegin/End`) -> `bracket-verb` = 0, plus any descriptor embedding payload schemas (62 leaf schemas gained `x-semio-inverse-rows`); (3) wire already done; (4) peer `📋️project.json` blocking verify CLI routes (fixed by INFRA).

---

### S3-CODES-TAX — outcome-law gate proofs, typed refusals (`OutcomeCode`), warn→warning rename, sealed evidence ledger

| Field | Value |
|---|---|
| Report | `📓️s3-codes-tax-report.md` (270 lines); last section `### Resume 3 (10-03 11:50) — engine root re-sealed through the ledger — DONE` (L252–270); owed list `### Owed` (L174), coordinator actions `### Coordinator actions` (L185) |
| Owns | root `📜️script.ts` rule-2 outcome-law region + `test outcome-law-gate` route; `🧪️tests/🧪️outcome-law-gate` (+ fixture/schema, repo root); `📡️replication/🎮️mutation/🦀️.rs` (`OutcomeCode` 9 variants, typed `refuse`, builders `warning`) + `🧪️tests/🧪️outcome-code`; 848-file `warn`->`warning` rename (873 sites; file list `📓️s3-codes-tax-warn-rename-files.md`; idempotent codemod `🧪️s3-codes-tax-warning-builders.py`); `📚️library` frozen seal ledger (`🧫️frozen-seal-ledger`, `catalogs`), `🔍️discovery/🟦️.ts` prerequisites, taxonomy seals (`🔣️taxonomy.json` one seal each), liveBindings split (`📐️cad-draw-path-projection`) |
| State | DONE-VERIFIED for TS/library/replication; ✏️s plugin compile of the rename + typed refuse callers UNVERIFIED. Verified: `bun test ./🧪️tests/🧪️outcome-law-gate/🟦️.ts` / `bun ./📜️script.ts test outcome-law-gate` / `NX_DAEMON=false bun nx run workspace:test-outcome-law-gate --skip-nx-cache` 25/0 each (26/0 after later additions); `bun ./📜️script.ts verify mutation-outcome-law` 0 breaches, 7 rules; `cargo check -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin --lib` exit 0 (147 plugin warnings); `cargo test -p semio-framework-replication --lib` 260/0; `cargo test -p semio-framework-os-kernel --lib -- persisted_messages_admit_exactly_the_outcome_vocabulary` 1/0; library tests: `🕰️historical-json-source-encoding` 25/0, `🏺️historical-package-owner-identity` 26/0, `☂️frozen-coordinate-wildcard-coverage` 5/0, `❄️frozen-markdown-coordinates` 35/1–36/0 (load timing), `🔤️taxonomy-leading-grapheme` 10/0, `🧪️mutation-leaf-identity` 10/0, `🧪️mutation-wire-witness` 11/0, `💥️nested-cargo-collision-authority` 26/0 (after 2 re-seals), `🖼️runtime-taxonomy-asset-paths` 1/0; generators layout/puzzle 3d/5d/selection dry-run 0 pending; taxonomy 7 new dirs clean; `.vscode/launch.json` render probe ok |
| Last peer blockers | plugin lib-test target (129 fixture errors — fixed by INFRA 11:38); stdio jpg/gif/ply/dxf/svg/pdf red from sqlite-snapshot ValueError migration; `verify dependencies literal-external` red unrelated (270 literal-external; oracle conflicts `js:xstate`, `rust:image`, `rust:serde_json`) |

**Owed verification (verbatim, `### Owed`)**
1. `cargo check --manifest-path ✏️s/Cargo.toml --workspace --lib --keep-going` — proves the 848-file builder rename and typed `refuse` callers (raster x5 + `patch-layer`, gif 87a/89a, bmp, glTF, stdio contract).
2. S2.7 reruns: `cargo test -j 2 --manifest-path ✏️s/Cargo.toml --no-fail-fast --lib -p semio-s-artifact-layout-layout -p semio-s-artifact-lowpoly-lowpoly`; `… -p semio-s-artifact-stdio-gltf -p semio-s-artifact-stdio-zip -p semio-s-artifact-wfc-bitmap`; media `-p semio-s-artifact-stdio-{png,jpg,wav,tiff,mp4,gif,mp3,avi,bmp,pptx}`; `-p semio-s-artifact-energy-model`; raster + puzzle 2d (C-2/C-5); remodel `bun ./📜️script.ts test parity exhaustive --case 📸️mutate-remodeling-1` (report: "remodel `parity exhaustive --case 📸️mutate-remodeling-1`")
3. `cargo test -p semio-framework-plugin --lib -- command_rejection_tests time_travel history_code` once plugin test fixtures migrated.

**Open items**
- Assign the two named owners: wgpu nested-cargo catalog vs `members-of-wgpu-target` (renderer catalog 10 of 32 wgpu destination rows moved individually after projection — S3-GATES final pass); plugin-registry generator input discovery for dynamic `import()` / `createRequire` (`🔬️workspace-contract` Draw producer x6) -> S3-INFRA.
- Notes for owners: S3-STDIO (10 dead set-snapshot `🦠️mutation` apply facets); S3-W2C (`replay_local_folder_events` skips refusals); hub (`hub.unavailable` built as a Warning `MutationMessage`).
- Candidate retirements (owner): `📚️library/🧫️fixtures/🦀️nested-cargo-package-authority` and `💎️nested-cargo-package-purity` still spell old `🧑️‍🎨️engine` (no reader).
- Unread fixtures with the old spelling (2) -> S3-GATES final pass (status L680).

**Coordinator actions**: none for activation/describe/schema generate from C-1/T-1/T-2/C-4; the Rust API change (`OutcomeCode`, `warning` builders) reaches every plugin component on the next activation chain run (so rebuild is required in the final wave).

---

### S3-NORM — norm plugin trees (EN/DIN/VDI standards): gates, labels, rule-1 breaches, set-snapshot base, §14 renames

| Field | Value |
|---|---|
| Report | `📓️s3-norm-report.md` (205 lines); last section `## 8. Files` (L190–205) preceded by `## 7. Coordinator actions` (L179), `## 6. Open items` (L162), `## 5. Owed` (L148) |
| Owns | `✏️s/🔌️plugins/📕️norm/**` (15 artifact crates en1990…en1999, din4108, din16798, din18599, vdi3805, iso16757 + norm contract/registry/app-surface `📇️registry/🧬️contract/🖥️app-surface/🦀️.rs` regions `🔖️ValuePath`, `🔖️Commands`), norm tests `🧪️wire-twins`, `🔮️oracle-source-ownership`; ticket inputs `🧪️s3-norm-*.py/ts` |
| State | SOURCE-COMPLETE; Rust UNVERIFIED (no cargo run: kernel red 10-02 then norm contract red 10-03; INFRA fixed norm contract 11:14). Gates under `--under ✏️s/🔌️plugins/📕️norm`: inputs 6->0, payloads 8->0, labels 2->0, editability 0, outcome-law 0 (47 rule-1 breaches fixed). Verified: oracle `exhaustive` 15/15 cases 1197/1197 (en1990 72, din18599 43, en1997 44, din16798 89, en1991 161, en1992 59, vdi3805 39, iso16757 59, en1993 115, en1994 51, din4108 103, en1996 121, en1995 135, en1999 38, en1998 68); `bun test ./✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts` 1901/1901 (10 704 assertions) after EN 1997 `annex` schema fix; TS twin census `bun T/🧪️s2-norm-ts-twins.ts --check` 15 subsets/690 twins/0 stale; strict tsc 0 errors (971 files); sqlite companions en1997 7/7, en1995 28/28, din18599 35/35, vdi3805 23/23; `oracle exhaustive --case 🪵️mutate-en1995-1` 135/135, `🌍️mutate-en1997-1` 44/44; `mutation-leaf-taxonomy-check` 554 payloads; `verify mutation-outcome-law` passed; oracle-source 5 pass / 1 fail (left: launch row) |
| Last peer blockers | norm contract crate (peer DSL/locale fallout 22 errors) — INFRA fixed 11:14 (diagnostic/schema-registry deps, `diagnostic::Limits`, pack_json x7, `ui_locale::Locale`); trinity `test-snapshot-sqlite` project.json (fixed) |

**Owed verification (verbatim, §5; private `CARGO_TARGET_DIR=…/target-nde-s3-norm`, `CARGO_INCREMENTAL=0`, one gated command)**
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-norm-contract --lib --test config_mutation` and `--lib` for all 15 artifact crates — proves §3.1 (en1995 vectors decode under camelCase `FromValue`), §3.3/§3.4 (15 rewritten set-snapshot tests + 6 new laws), §3.5, §3.6 (62 rewritten files), §3.7, incl. `semio_payload_law_*` and `committed_vectors_are_this_implementations_answer`.
2. `cargo check --target wasm32-wasip2` for the norm component crates touched (all 15 + contract).
3. `bun ./📜️script.ts parity exhaustive --case <c>` (cwd `🧪️test`) for the 15 cases (en1995, en1997, en1991, din4108, en1998 changed).
4. `bun ./📜️script.ts inventory --artifact s.norm.<a> --standard 1` per artifact (builds `🏭️bridge`), then the contract phase (binary-protocol-drift en1992/en1999/en1996/din18599 pending in S2 closure table). Also `cargo check -p …norm-contract -p …en1995 -p …en1998 --lib`.

**Open items to implement**
1. Diff schemas are not the Rust diff wire (`bun T/🧪️s3-norm-diff-schema-census.ts`): committed `🔺️diff` fixtures meet their diff schema in din18599 18/18, en1997 20/20, en1993 49/49, en1994 25/25, en1998 29/29 but **0 %** in en1990 (0/36), din16798 0/43, en1991 0/80, en1992 0/28, vdi3805 0/19 (missing `limits`), iso16757 0/29, din4108 0/53, en1996 0/62, en1995 0/66, en1999 0/18 (Option diff members declared non-nullable) — proposed as its own WP with a witness-test extension.
2. `T/🧪️w2-w-norm-1-vectors.py` (session-1 generator) still names 17 pre-§3.6 en1991 leaf directories — mark superseded or update before reuse.
3. EN 1995 `✏️editor/🏷️field-meta/🦀️.rs` (and en1992/en1999) hand-write choice labels for snapshot enums (second truth) — candidate for schema-driven inspector.
4. en1999 snapshot `support` and similar free-text enums are `type: string` + text widget; make them `enum` + options (needs Rust vocabulary change).
5. S3-CONTROLS routing: norm app-surface `commit_snapshot(_fields)` hand labels (`…app-surface/🦀️.rs:1582,1590`) — S3-NORM §20.4 done in source.
6. §20.15: norm `din18599` 1 `parentLeafReadsChild` (`din18599_climate`) routed 11:43 — NOT STARTED (report predates routing).
7. Repo-wide over-budget path census outside norm (energy, stdio, architect) is out of ticket scope (design §14).

**Coordinator actions (§7)**: `describe` norm (committed descriptor `🌎️hub/🧩️compositions/📕️norm/🛂️.descriptor.semio` still names the pre-session-2 en1991 kind and the 20 pre-§3.6 leaf dirs) + re-activation after cargo passes; central `schema generate` (hash-only staleness: en1995 `change-member-role`/`-support`, din18599/vdi3805/en1997 snapshot schemas, en1997 artifact schema, 20 renamed leaf paths — catalog paths hand-rewritten on exact names); launch rows `bun nx run @semio-tech/norm-js:test-oracle-source`, `bun nx run @semio-tech/norm-js:test-wire-twins`; "TREE GREEN" for the norm contract crate (done by INFRA).

---

### S3-STDIO — non-text stdio artifacts (path-scoped `patch-snapshot` leaves §20.3, labels §20.6, findings)

| Field | Value |
|---|---|
| Report | `📓️s3-stdio-report.md` (190 lines); last section `### S3.6 Coordinator actions` (L184–190); open items `### S3.5` (L167); status `### S3.0` (L9) |
| Owns | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*` (all except md/html/txt = TEXT) and contract crate `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract` (`✏️editing/🩹️patch`: `SnapshotPatch` one RFC 6901 op `set\|insert\|remove\|move\|rename`, `snapshot_patch_leaf!` macro, `snapshot_schema_location`, `snapshot_patch_input_schema`), derive `#[mutation_leaf(input_schema = path)]` in `🗣️dsl/✨️derive/🦀️.rs`, taxonomy `mutationDomainOwners` `🩹️patch` = `patch-snapshot` + regenerated `🔣️mutation-authority.json`; 49 new `patch-snapshot` leaves (56 stdio aggregates), 83 editors on `snapshot_edit_patch`; ticket inputs `🧪️s3-stdio-patch-leaves.py`, `🧪️s3-stdio-convert-editors.py`, `🧪️s3-stdio-codec-tools/` |
| State | SOURCE-COMPLETE; contract lib 89/0 + a handful of crates ran; 11 red crates + dependents UNVERIFIED. Verified: patch TS `bun test ./✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🧪️tests/🟦️.test.ts` 34/0 (303 expects; fast-json-patch + ajv oracles); strict tsc 0 errors; `cargo check -p semio-s-artifact-stdio-contract` exit 0; contract lib 89/0; dwg 94/1, dxf 46/3, json 107/2, obj 57/2, pdf 557/3, ply 59/3, stl 53/2, tiff 117/1, xml 93/1 (own failures fixed; rest = peer `🪶️sqlite` controlled-admission tests + json snapshot-DSL `grammar_conformance_law`); `bun …/🧪️s3-stdio-audit-patch-leaves.ts` 56 leaves / 0 findings. Gates `--under ✏️s/🔌️plugins/🗄️stdio`: labels 178 -> 0 (3 html/md/txt closed by STDIO 06:36), payloads 10 -> 0 (2047/2047 fixtures, 1038/1038 leaves witnessed), editability 0 (1038/1038 of 87 aggregates), inputs 48 `leafUncatalogued` (central generate; 1661/1661 labelled) |
| Last peer blockers | peer ValueError/`dsl::json` sqlite migration in 11 stdio crates (epw, step, gif, avi, jpg, mp4, wav, zip, mp3, svg, gltf; dependents xlsx, docx, pptx, ifc, bcf, semio/png tests); kernel `IoError` refactor (06:44; later settled) |

**Owed verification (verbatim, S3.6/S3.5 item 6)**
1. After S3-INFRA/peer sweep green: `cargo test --lib` (private `target-nde-s3-stdio`) for the 11 red crates + dependents + semio/png tests; **editor modules compile only with `--features <each>/component-app-assembly`** (without it 0 editor tests); rerun dwg/dxf/obj/ply/xml (blocked 06:37). Payload laws `semio_payload_law_*` run with the lib tests.
2. `cargo check` of 20 stdio crates `--keep-going -j 2` (xlsx/docx/pptx/ifc/bcf blocked behind zip/step).
3. Four gates under `--under ✏️s/🔌️plugins/🗄️stdio` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): `bun ./📜️script.ts schema mutation-labels|mutation-inputs|mutation-payloads|mutation-editability`.
4. gltf+bmp gated `cargo check` (authority fix applied 06:45; `bun 🧪️s3-stdio-leaf-authority.ts` proof green).

**Open items to implement (S3.5)**
1. Third-party oracle + `🥒️.feature` row per new `patch-snapshot` kind (47 new leaves): catalogs list the kind and each has a wire witness, but no Examples row / oracle `"patch-snapshot"` arm exists yet (existing json/csv/png/wav/mp4/jpg/tiff patch kinds have both) — needs a per-oracle generic JSON-pointer apply over each oracle's wire projection. (Design §11 evidence rule + AGENTS "one third-party oracle per feature".)
2. **> 1 MiB removal has no exact inverse** (`inverse_snapshot_patch` refuses -> empty inverse); same for JSON `ReplaceSource` > 1 MiB (`SNAPSHOT_PATCH_MAX_BYTES` = 1 MiB vs `SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES` 16 MiB): needs streamed patch value or chunked/blob-referenced inverse — undo must never be lossy (status L599).
3. Snapshot-schema labels: deep pointers fall back to the structural payload schema where the snapshot sub-schema lacks `x-semio-ui` labels (many stdio schemas unlabelled).
4. pdf / xml base / svg base TS unions have no `PatchSnapshot` member (no leaf `🟦️.ts`).
5. pptx: `set-snapshot`/diff leaf `prologPosition` still `integer` while the snapshot is `string` (peer conversion 10-02 11:18); GraphQL `Int!` for `prologPosition` in xml/pptx `🔗️.graphql` not regenerated.
6. jpg/tiff baseline `SetPixelRegion` -> `SetSnapshot` (raster exceeds patch bound; no pixels leaf); `agg_inverse` -> `SetSnapshot(base)` for ifc 2x3 / docx / xlsx non-patch kinds (pre-existing).
7. Peer notes from CODES-TAX: 10 dead set-snapshot `🦠️mutation` apply facets.
8. **Inherited owed case verification (brief item "owed case verification", status L381/L426; NOT mentioned in the S3-STDIO report — treat as open)** from `📓️w3-stdio-cases-2-report.md` §S2.4/§S2.6 and `📓️w2-r-stdio-a-report.md`: (a) `cargo check -p semio-s-artifact-stdio-{contract,html,docx,pptx,xml,semio,csv,json} --target wasm32-wasip2 --message-format=short`; (b) `cargo test -p …-{pdf,docx,xlsx,pptx,zip,semio,json,csv,html,xml} --lib` (private `target-nde-s4-stdio`, incl. `semio_payload_law_*`); (c) oracle crate `cargo test --manifest-path ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/Cargo.toml --features oracles --lib`; (d) `zsh T/🧪️w3-stdio-run-cases.sh` (output to `🗑️generated/s4-stdio/`) for `📰️mutate-xml-1-0`, `🧾️mutate-pdf-1-7-vt`, `🗄️mutate-pdf-1-7-a`, `🖨️mutate-pdf-1-7-x`, `📐️mutate-pdf-1-7-e`, docx base/strict/transitional, semio drawing, `🔀️mutate-json-rfc8259(-i-json)`, `📊️mutate-csv-rfc4180`, `🎨️mutate-svg-1-1`; (e) `contract exhaustive --owner 🗄️stdio`.
9. **STDIO-B / WP-3 follow-ups (S2, still open)**: 22 case generator/probe scripts still need the one-line `process.exitCode` fix or the framework-level capture fix (`CAPTURE_WRAPPER_SOURCE` in `📚️library/🟦️.ts` hands the child file descriptors to remove the 64 KiB bun pipe truncation); bcf `💬️bcf/…/🖊️markup/🧬️mutations/🦀️.rs` and gltf `🧊️gltf/…/🔺️diff/🦀️.rs` still carry their own `deserialize_double_option` (swap to `semio_s_artifact_stdio_contract::deserialize_double_option`); gif/avi/dxf `no-mutation-no-op` recipes; semio `no-mutation-baseline-*` scenarios in 10 cases (identity probes, to convert per the text lane's recipe step 2).

**Coordinator actions (S3.6)**: central `schema generate` (49 new `…/mutation/patch-snapshot/schema.json` leaves `leafUncatalogued`, 47 of 49 input findings; plus 40 stdio patch leaves from other formats noted by TEXT); `describe`/`materialize` for every stdio plugin (new variants, catalog branches, "Load example" verb label, removed descriptions) then re-activate stdio dev lanes.

---

### S3-GATES — repo gates over the ticket's changes (dependencies, layering, docstrings/[DEBUG], strict schema gates, taxonomy, warnings)

| Field | Value |
|---|---|
| Report | `📓️s3-gates-report.md` (206 lines); last sections `### 5. verify taxonomy report over the ticket's new directories` = "Pending.", `### 6. Rust warnings (after TREE GREEN)` = "Pending.", `### 7. Coordinator actions` = "Pending (collected at the end)." (L196–206); content in §1 (dependencies, L18), §2 (layering, L118), §3 (docstrings, L140), §4 (strict gates, L174) |
| Owns | repo gate runs + fixes of ticket-owned findings; oracle registry contributions `🔮️oracles/🔣️.json` in 9 owners (ui, time-travel, tool-machine, replication, store, dev, engine, manifest, repo-test) via idempotent `🧪️s3-gates-oracle-claims.py`; owner-package devDependency declarations (`@semio-tech/framework` +`@types/d3-scale`,`color-string`,`d3-scale`,`decimal.js`,`fast-check`,`xstate ^5.32.5`; `framework-replication`+fast-check; `framework-os`+dom-accessibility-api,fast-check,xstate; `framework-os-dev`+aria-query,dom-accessibility-api; `repo-test`+tree-sitter-wasms 0.1.13, web-tree-sitter 0.20.8; `forms-js`+fast-check); 6 lockfiles (lockfile-only); `🔒️dependencies.json` baseline 248 -> 256; ticket scripts `🧪️s3-gates-ticket-files.py`, `🧪️s3-gates-source-rules.ts`; outcome-law fix `mutation.target-in-use` -> `mutation.target-referenced` (semio mesh delete-texture diff + TS twin + sqlite law) |
| State | PARTIALLY DONE. DONE-VERIFIED: dependencies (`bun install --frozen-lockfile --dry-run --ignore-scripts` exit 0 in all 6 roots; `verify dependencies` ratchet = exactly the 17 peers' additions flagged, ours approved; `literal-external` current=270, oracle-conflicts=3 [js:xstate ui-react production, rust image, serde_json = peers]; parity js lock mismatches 0, ticket packages undeclared imports 33 -> 7 peer files); strict gates AFTER (10-03 10:45–10:55): `mutation-labels` 0, `mutation-editability` 0, `verify mutation-outcome-law` passed (was 1297 -> 1 -> 0), `mutation-payloads` 32 (remodel 30 + raster 2 -> STROKES, since remodel 0), `mutation-inputs` 251 (remodel 175 -> STROKES since 0; `leafUncatalogued` 71 + `malformed` 4 + trinity `refUnresolved` 1 = 76 -> central `schema generate`); layering: exit 1 with ticket-owned findings 0 (listed peers: 3 TS edges incl. Canvas2dHost->draw fixture; 9 Rust violations from active multi-UV mesh/DSL peers; cargo-direction metadata timeout 30000 ms -> re-run once); `[DEBUG]` in probe removed (0 tag lines in time-travel probe); docstring fixes in plugin `NodeGraphDeleteDispatch` + `I18n` port. NOT DONE: §5 taxonomy report over ticket's new dirs, §6 Rust warnings, §7 coordinator actions |
| Last peer blockers | none own (Rust warnings need TREE GREEN) |

**Owed verification (verbatim)**
1. `bun ./📜️script.ts verify dependencies literal-external --format json` (6:40 min) and `bun ./📜️script.ts verify dependencies` (ratchet), `bun ./📜️script.ts verify dependencies parity js` (2:18 min), `bun ./📜️script.ts verify dependencies write-baseline` (only coordinator-approved).
2. `bun ./📜️script.ts verify layering` (nx `@semio-tech/repo-lib` lint-dependency-direction, lint-rust-source-direction, lint-cargo-dependency-direction, `--skip-nx-cache`; ~13 min) — re-run once for the cargo-direction verdict.
3. `bun ./📜️script.ts verify debug-tags` (repo-wide 341 lines/154 files, growth = peers), `bun ./📜️script.ts verify docstrings emoji-first` (4475 repo-wide pre-existing class), ticket sweep `bun 🧪️s3-gates-source-rules.ts` (590 files: docstring no-emoji 39 [37 in DSL peer crate], `[DEBUG]` 14 [all peers/not ticket], indented line comments 389, dup emoji 2532).
4. Strict gates (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): `bun ./📜️script.ts schema mutation-labels --json`, `schema mutation-editability --json`, `schema mutation-payloads --json`, `schema mutation-inputs --json`, `bun ./📜️script.ts verify mutation-outcome-law` (root).
5. **§5** `bun ./📜️script.ts verify taxonomy report --scope <each new dir>` for every directory the ticket added (pending — GATES never ran it; per-WP reports show their own dirs clean) and **§6** Rust warnings (`cargo check … --message-format=short` warnings in touched crates; e.g. plugin 147 warnings baseline) after TREE GREEN.
6. `bun ./📜️script.ts verify dependencies literal-external` after any further dependency additions.

**Open items**
- Decision for dev/coordinator: per-file docstring-emoji uniqueness has no repo gate scope (2532 repeats in window; convention `/// ⚖️ LAW:`, `🔖️`) — define gate scope or accept convention.
- 17 unapproved PEER dependency additions flagged (js: `@csstools/css-calc`, `@csstools/css-tokenizer`, `@testing-library/user-event`, `@types/lodash`, `@types/opentype.js`, `lightningcss`, `opentype.js`, `@types/d3-color`, `@types/d3-ease`, `d3-ease`, `polygon-clipping`, `commander`, `graphlib`, `yaml`; rust: `parking_lot`, `percent-encoding`, `tungstenite`) — for their owners/dev.
- `js:xstate` oracle conflict (ui-react production use re-exporting `assign, createActor, fromCallback, setup` at `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:8706`): interface-owner registration or removal — out-of-scope chip.
- Canvas2dHost test importing a draw fixture (`📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx:609` -> `🕹️nudge-selection/🧫️fixtures`): peer keyboard-binding wave; listed only.
- `fast-check` was in no lockfile before this session (now added to all 6); `aria-query`/`dom-accessibility-api` etc. declared in owner packages.
- CODES-TAX routed: renderer catalog 10 of 32 wgpu destination rows moved individually after the projection + 2 unread fixtures with old spelling (retire) -> GATES final pass.
- Remodel/raster gate findings -> STROKES (remodel closed there; raster 4 unwitnessed/uncatalogued wait on cargo).

**Coordinator actions**: central `schema generate` (76 inputs findings); decide docstring-emoji uniqueness scope; approve/route 17 peer deps; (§7 never collected).

---

### S3-LOAD — whole-document load senders (MCP, `🏃️run`) onto the stepped archive load; TS twin; W-b producers

| Field | Value |
|---|---|
| Report | `📓️s3-load-report.md` (220 lines); last section `### 8.2 Host router keeps a composed document's children (DONE, verified)` (L215–220) under `## 8. W-b of design §20.15 — producers supply composed children` (L193); owed list `## 4` (L95), bump list `## 5`/`## 6` (L115/L130), TS twin `## 7` (L141) |
| Owns | kernel driver `DocumentArchiveLoadHost` (`📡️spr/🧵️channel/🦀️.rs` region `🔖️DocumentArchiveLoadHost`; corpus + schema `🧫️fixtures/🧫️document-archive-load-host`, `🧬️schema/🔣️document-archive-load-host`), TS twin `💻️os/🟦️.ts` region `🗃️DocumentArchiveLoadHost` + `AppChannelClient.loadDocumentArchive`, law `💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts`, MCP `🌉️mcp/🏠️workspace/🦀️.rs` (`load_session_document`, `ExportMedia`, phase `loading-document`), `🏃️run` `SpaceRunner::compute_node` (`drive_document_load`, progress observer, CLI reporter), host router `🔌️plugin/🖥️host/🦀️.rs` (`routed_inference_dependencies`), launch row `⚖️test-channel-oracles💻️os🟦️` (order 900.03555, group `4_gate`), nx target `test-channel-oracles` |
| State | kernel driver + TS twin DONE-VERIFIED; MCP + `🏃️run` WRITTEN-UNVERIFIED; W-b host router DONE-VERIFIED, other W-b producers OPEN. Verified: `python3 T/🧪️s3-load-archive-host.py` 12 cases/0 failures (`--negative` exits 1); `cargo check -p semio-framework-os-kernel --lib` Finished (10:46); `cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::` 93/0 (11:23); `cargo test -p semio-framework-os-kernel --lib -- the_document_archive_load_host` 1/0; `bun test /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts` 3 pass/226 expects (also via `bun ./📜️script.ts test-channel-oracles` and `bun nx run @semio-tech/framework-os:test-channel-oracles`); negative control 2 fail; OS vitest `bun ./📜️script.ts test quick` 476 passed; React archive subset 7 passed; `cargo check -p semio-framework-plugin-host --lib --tests` Finished 12:02; `cargo test -p semio-framework-plugin-host --lib -- artifact_inference_router` 3/0 (12:06) |
| Last peer blockers | `🏃️run` dependency `🔁️workflow/🗿️artifacts/🏃️run` 154 errors (peer locale + ValueError; last peer edit 09:05; INFRA fixed workflow 10:56 — re-check); MCP io region 42 errors (`🏠️workspace/🦀️.rs` 402, 3119–3155 + `🪶️sqlite` x21) |

**Owed verification (verbatim, §4)**
1. `cargo check -p semio-framework-os-run --lib --bins --tests` then `cargo test -p semio-framework-os-run --lib` (private target `target-nde-s3-load`) — expect the three new laws + updated ordering/upstream laws green.
2. `cargo check -p semio-framework-os-mcp --lib --tests`, then `cargo test -p semio-framework-os-mcp --lib -- long::a_long_history_document_loads_through_polls quick::an_abandoned` and the `binding-cancellation-law` test target (8-phase table).
3. Re-run after INFRA green: workflow/run deps.

**Open items to implement**
- **Delete `AppCommand::LoadDocument` + every reference (bump wave, §5 list)**: variant + codec `📡️spr/🧵️channel/🦀️.rs` (enum, paged field take/put 1978/1981/2007/2010, paged decoder 2154, fields 2347, encoder 3403, decoder tag 6 at 3791; tests 98, 632, 810), guest receiver `🔌️plugin/🦀️.rs:43798`, exhaustive seq arms `🏃️run/🦀️.rs:2260`, `🌉️mcp/🏠️workspace/🦀️.rs:2179`, TS `💻️os/🟦️.ts` (type 2617, tag 2896, encode 2959–2963, decode 3149–3153, `sendCommand` document cache 4055, `AppChannelClient.loadDocument` 4192–4194), `🧪️backbone-envelope-io` tests (467, 665, 686 hex `060101010102`, 1336–1339, 1448, 1454, 1486); docs-only mentions (energy model `:603`, plugin 8315/43449, checkpoint 8/74, host 7423/7451, ShellHost 8726/8811, builder-contract test 4733). Senders: **none** left.
- wgpu `ProgramBridge` `load_app_document_archive` adopts the shared driver — written by W2C 11:4x (unchecked).
- **W-b (§20.15) producers**: (a) guest validation `validate_wire_request_resources` (`🔌️plugin/🦀️.rs` ≈2123) runs `ArtifactIdentity::parse` on every dependency key; `child:<slot>/<childId>` is not identity-shaped, so every composed inference request is REFUSED (`artifact-inference.dependencies`) — sent to AGNOSTIC (owner of the key); (b) no requester holds child head packs (proposed guest `PluginApp::child_head_packs()` + host read `ReadChildHeads -> ChildHeads`); (c) serialize producers: no host/shell producer hands a native payload to io in production — framework's one native-head-pack producer is guest default `export_media("artifact:out")` (base64 head pack, read by `🏃️run` MediaOut and MCP `ExportMedia`): needs the composed carrier `encode_document_archive_bytes({parent HEAD pack, empty spr, members})`; (d) MCP gateway `infer_real` sends `dependencies: Vec::new()` — must inject `child:<slot>/<childId>`; TS host builds no inference request.
- Archive Ready must lift head-only mode — DONE by W2A 9.9.
- **T2/T3 sweep (from `📓️w2a-sync-load-paths.md`)**: move ~45 `✏️s` plugin test files (stdio x17, raster x6, wires x5, fem x4, note x3, cad x3; puzzle 2d = W2A) and 2 `✏️s/🧑‍💻dev/🧩️composition` tests from `load_document_pack/text` onto the stepped helper `artifact_app_laws::load_document` once W2A has written it — NOT started.

**Coordinator actions (§6)**: bump wave deletes the variant + every §5 item; re-run owed commands once peers green; no describe/activation needed (host-side only; guest unchanged).

---

### S3-NOTICES — app-declared, localized fault notices (design §20.12), `Fault.params`, gate `schema fault-notices`

| Field | Value |
|---|---|
| Report | `📓️s3-notices-report.md` (179 lines); last section `### Coordinator actions` (L169–179); verification `### Verification (12:40)` (L84), routed per-plugin debt table (L108) |
| Owns | `⚠️diagnostic/🦀️.rs` (`FaultParams`, `Fault.params` boxed, inline size stays 112 B) + controlled/retirement codec + schemas/fixtures; manifest region `🔖️FaultNotices` (`FaultNoticeDefinition`, `validate_fault_notices`, `fill_fault_notice`; `AppDefinition.faultNotices` + schema `$defs.FaultNoticeCode/Text/Locales/Definition/Table/FaultNoticeCorpus`), TS twin; `🎠️kernel` `fault_notice(fault, notices, terminology, locale)` + TS `faultNotice`; plugin `ArtifactApp::fault_notices()` + `declarations::stamp_fault_notices::<A>` + builder stamping; React `appFaultNoticeV1` (`🛠️ShellHelpers`, `🏛️ShellHost` three refusal sites); wgpu `ProgramFault`, `RefusedGuestFault`, `classify_dispatch_fault_notice` (branch 3 replaced), `🧪️wgpu-fault-notices` law; `🌿️vcs` `VcsError::fault_params` (`history.full` carries `n = capacity`); gate `schema fault-notices` (region `📢️FaultNotices` in `🧪️test/🧬️schema/📋️orchestration/🟦️.ts`, targets `test-schema-fault-notices`, `test-schema-fault-notices-census`, `test-fault-notices-gate`, 3 hand-added `.vscode/launch.json` rows orders 900.04775 / 900.04901 / 900.04902) |
| State | MECHANISM DONE-VERIFIED (native); wasm32 JS-path unverified; repo-wide notices are DEBT. Verified: `cargo test -p semio-framework-diagnostic --lib` 11/11; `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts` 5/5; React `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🧪️fault-notices 🧪️command-rejection` 6/6; `cargo check -p semio-framework-plugin --lib` green; `cargo test -p semio-framework --lib fault_notice` 4/4; `cargo test -p semio-framework --lib --features typegen exports_typescript_bindings` 1/1; `cargo test -p semio-framework-os-kernel --lib fault_params` 1/1; `cargo check -p semio-framework-os-renderer-wgpu --lib` native green; wgpu `-- fault_notice history_lane_refusal dispatch_faults_are_classified history_edit_refusal a_failed_dispatch uncorrelated_answer` 6/6; `bun test …/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` 2/2; mutation-history-gates 39/39; schema-invariants 119/122 (3 peer failures). Gate census 12:40: **12 labelled / 420 guest fault codes, 12 declared notices, 1778 anonymous faults, 2187 `schema-fault-notice` findings** (faultAnonymous 1778, faultNoticeMissing 258, faultNoticeSyntax 115, faultNoticeFramework 35, faultNoticeDescriptor 1) |
| Last peer blockers | wgpu wasm32: `🔌️plugin/⏯️tool-run/🦀️.rs:1895`, `🔌️plugin/⏪️time-travel/🦀️.rs:2072` E0308 `record_command` (peer/W2A, since fixed per W2A compile 10:55); wider wgpu suites blocked by graph peer `PropertyBag` (cleared 12:00) |

**Owed verification**
1. `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` (JS path `js_program_fault`) and the wider wgpu suites (time-travel/overlays/bridge): `cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel agent_overlays wgpu_fault_notices`.
2. `bun ./📜️script.ts schema fault-notices` (cwd `🧪️test`; exit 1 expected until debt is paid; with `--census` for table) and `bun ./📜️script.ts test fault-notices-gate`.
3. `bun test ./🧰️framework/🔨️modules/🗣️dsl/🧪️tests/🧱️ownership/🟦️.ts` (5/6; peer dsl crate dependency `semio-framework-async`).

**Open items**
- Per-plugin debt (table L108): 1778 anonymous `Fault::from(text)` (stdio 288, puzzle 313, fem 182, draw 111, procedural 118, flow 102, raster 87, sequence 85, wfc 61, forms 50, … ) -> named codes + notices; 258 codes without notices; 115 codes with < 3 segments / non-kebab (`*.child-projection`, `artifact-store.initializer-close`, …); 35 framework-namespace codes raised by guests (`app.command.*`, `mutation.target-*`) — decide a framework table beside `HISTORY_NOTICE_LABELS`. Design decision (status L689): ticket scope = refusals on history-editing/tool flows named + localized (gate scoped mode); repo-wide rest filed as a separate task chip.
- `faultNoticeDescriptor`: procedural describe owed (12 generation3d notices unpublished).
- Register `i18next` (test oracle of `🛂️manifest/🧪️tests/🧪️fault-notices`) with the dependency gate (GATES).
- Regenerate `.vscode/launch.json` (3 hand-added rows) on next generator run.

**Coordinator actions**: `describe` procedural (+ every plugin declaring notices); decide the home of notices for guest-raised framework codes (35 findings); route per-plugin debt; regenerate `.vscode/launch.json`; i18next dependency registration; re-run wgpu wasm32 check.

---
