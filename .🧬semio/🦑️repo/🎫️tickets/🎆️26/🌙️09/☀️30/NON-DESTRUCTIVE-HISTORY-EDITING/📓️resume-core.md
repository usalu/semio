# 📓️ Resume State — Core Runtime, Shells, Store, E2E

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, session 2 state reconstruction (read-only auditor, 2026-10-01 ~11:55).
Scope: W1-A, W1-B, W1-C, W1-E, W1-G (incl. design §15 / follow-up 5), W2-A, W2-B, W2-C, W2-D, W3-E2E.
No cargo, nx, bun, server or modifying git command was run. Everything below comes from the ticket files, committed and
working-tree sources, file mtimes and the compile/test logs under `🗑️generated/`.

Aliases: `T` = this ticket folder · `FW` = `🧰️framework/🔨️modules` · `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules` ·
`P` = `OS/🔌️plugin` · `K` = `FW/🎠️kernel` · `M` = `FW/🛂️manifest` · `R` = `FW/📡️replication` ·
`RE` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements` · `SH` = `RE/🐚️Shell` · `WGPU` = `SH/🎯️targets/🧊️wgpu/🦀️.rs` ·
`PZ` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any` · `ED` = `PZ/✏️editor`.

Evidence tags: **[RAN]** a report or log shows the command and its result · **[COMPILED]** a log shows `Finished` and no test ran ·
**[SRC]** present in source, never compiled or run since · **[HYP]** my inference, not proven.

---

## 0. Time anchors and how much the repo can tell us

| Time | Fact |
|---|---|
| 2026-09-30 23:43 | commit `fa3fdf76eb7` |
| 2026-10-01 ~05:50 | session-limit cut #1 of the night (reset 07:20); peers keep editing (06:17–06:28 puzzle graph-manifest rework) |
| 2026-10-01 07:21–07:55 | fleet resumed; last fleet source edits 07:43–07:52 (W2-A, W1-G, W3-T*) |
| 2026-10-01 08:05:38 | activation #6 retry died (exit 130); `🗑️generated/e2e/activate-react-6.log` |
| 2026-10-01 08:00–08:01 | last compile evidence of the fleet: `w2-c-build.txt` (wgpu renderer test build, Finished), `w2d-test-all.txt` (puzzle-2d lib test build + run, aborted), activation #6 `puzzle-plugin:wasm` + `component-dev` ✔ |
| 2026-10-01 08:05–08:16 | fleet dead (last log writes 08:16:44) |
| 2026-10-01 11:16 | commit `4e36b2b5012` (auto-commit snapshot, includes the fleet's last edits) |
| now | load average ~40–50, 4 `rustc` running, free disk 54 GiB (guard PID 94966 alive); serve supervisor PID 81031 alive on :6012 |

Consequence: `git diff` between `fa3fdf76eb7` and HEAD mixes the fleet with peers, so I used mtimes + the compile/test logs to
separate them. **The working tree for every core WP is clean against HEAD except** `OS/🌿️vcs/🦀️.rs` (another session, 11:44,
see H6) and `FW/🖱️ui/🎯️targets/🧊️wgpu/{🧩️component,🔀️reconcile,🎬️action}/🦀️.rs` + `🧪️tests/🔬️targets-wgpu-action-unit/🦀️.rs`
(a peer, 11:32–11:35, intrinsic-bytes action values).

I checked brace/paren/bracket balance (string- and comment-aware scanner, negative-controlled) on every core Rust file the fleet
touched after its WP's report: `P/🦀️.rs`, `P/⏪️time-travel/🦀️.rs`, `P/🛠️tool-machine/🦀️.rs`, `OS/🏪️store/🦀️.rs`, `OS/🌿️vcs/🦀️.rs`,
`OS/📡️spr/🦀️.rs`, `OS/📡️spr/📜️history/🦀️.rs`, `R/🔗️causal/🦀️.rs`, `R/🎮️mutation/🦀️.rs`, `R/📡️wire/🦀️.rs`, `K/🦀️.rs`, `M/🦀️.rs`,
`FW/⏪️time-travel/🦀️.rs`, `FW/🛠️tool-machine/🦀️.rs`, `WGPU`, `SH/…/📎️local-folders/🦀️.rs`, `SH/…/⏪️time-travel/🦀️.rs`, both W2-C test
files, `ED/🦀️.rs`, the select tool, the board `➕️normal/🦀️.rs`, both W2-A test files. **All balanced; no TODO/FIXME/`todo!`/`[DEBUG]` in the
W2-A files.** There is no half-written file; the open risk is "written, never compiled as a test target", not "cut in the middle of an edit".

---

## 1. Status table

| WP | Report last written | Last own source edit | Last compile evidence | Last test evidence | State |
|---|---|---|---|---|---|
| W1-A replication | 09-30 07:53 (+ follow-up 2 ~08:20) | 09-30 ~08:20 | libs OK 07:55 | repl 307/307, os-kernel sync 1303/1303 [RAN]; 313/1 at 03:09 after W2-A's `verb` wire change | done; later `verb` additions by W2-A unverified |
| W1-B time-travel | 09-30 05:52 | 09-30 05:49 | 05:5x | cargo 12/12, bun 18/18 [RAN] | done |
| W1-C tool-machine | 09-30 05:54 | 09-30 05:5x (module since extended by W3-T2, 10-01 03:31) | wasip2 + unknown OK | cargo 13/13, bun 18/18 [RAN] at 05:5x | done; counts stale |
| W1-E UI contract | 09-30 18:30 | 09-30 ~18:45 (tree rows) | wgpu renderer test build Finished 08:00 | ui 761 ✔, contract 215 ✔, wgpu 17/29 [RAN] 18:45 | done |
| W1-G store | 10-01 03:09 | **10-01 07:48–07:53 (follow-up 5, §15, unreported)** | kernel lib `Finished` 07:53 (`w1-g/f5-kernel-check.log`) | kernel 883/0, replication 313/0 [RAN] 02:46 | follow-up 5 half done |
| W2-A plugin | **09-30 17:43 (stale)** | **10-01 07:44:55 / 07:52:14** | plugin lib `Finished` 07:55 (`w3-codes/check-os-run.txt`); plugin test target `Finished` 07:27 | plugin lib 918/13 (baseline) [RAN] 09-30 ~22:40 | code ahead of its report |
| W2-B React | 10-01 03:21 | 10-01 02:52 | none after 03:21 | os-config 193/193, folder-archive-restore 4/4, React suites green [RAN] 03:2x | done; ShellHost churned by a peer 11:12 |
| W2-C wgpu | 10-01 03:04 | **10-01 03:15–03:38 (follow-up 5, unreported)** | renderer test build `Finished` 08:00 | wgpu tt+dialog 40/40, vitest 86/86 [RAN] 03:05 (before follow-up 5) | follow-up 5 source-complete, unrun |
| W2-D puzzle 2d | 10-01 02:34 | 10-01 03:40 (+ peer edits in its files 06:22–06:26) | wasm2d `Finished` 07:30; test build `Finished` 08:01 | **1071 listed: 1056 ok, 9 FAILED, run SIGABRTed** [RAN] 08:01 | regressions to triage |
| W3-E2E | 09-30 21:44 | probe 10-01 03:05 | n/a | React Run 2: 85 PASS / 3 FAIL (en, de); Run 3 steps 1–3: 26/26 | blocked on activation |

---

## 2. Hazards that affect every core WP (read first)

- **H1. The live serve on :6012 is dead for puzzle 2d.** `curl :6012` answers 200 (vite root) but `🗑️generated/serve-6012-supervised.txt`
  repeats every 20 s `Pre-transform error: Failed to load url …/✏️s/🔌️plugins/🧩️puzzle/🧑‍💻dev/🚀️entry/🟦️.ts … Does the file exist?`.
  The puzzle plugin's dev entry moved (committed in `fa3fdf76eb7`) to `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🚀️entry/🟦️.ts`; the old dir is empty. The
  puzzle plugin crate also moved to `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust` (04:27). Supervisor PID 81031 serves the activation-#4
  build and must be replaced after a fresh activation. :6112 answers 000 (never served).
- **H2. Activation #6 failure causes (both still true):**
  1. `@semio-tech/plugin-registry:generate`: `Invalid registry descriptor: /executionProtocol/appChannelVersion must equal its const`. The const is
     `20` (`OS/🔌️plugin/📇️registry/🧬️schema/🔣️.json:664`); **22 of 34** `🌎️hub/🧩️compositions/*/🔣️.json` descriptors still say `19`
     (including `🧩️puzzle`, written 04:27). A peer is regenerating them one by one (12 at `20`, vcs/gis/animate/architect written 08:14–08:29).
  2. `puzzle-plugin:materialize-dev`: `Unowned artifact directory: …/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle`. Its
     `.nx-artifact.json` owner is `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:browser:dev` (activation #4); the manifest moved to `🌎️hub/…`, so the
     owner differs and the stager throws instead of replacing (it only deletes unmarked dirs). Needs a deliberate delete of that one staging dir.
  The puzzle wasm itself built fine in the same run (`✔ puzzle-plugin:wasm`, `✔ component-dev`, 08:00), i.e. the fleet's last source state compiles for the puzzle plugin.
- **H3. Puzzle 2d graph manifests were decentralised by a peer (06:17–06:28).** `PZ/🤖️generated/📇️registry/🦀️.rs` (untracked, generated 06:20) now
  lists only `puzzle2d-default`; `manifest_by_id("nakagin")` is `None` in the puzzle crate (the nakagin source lives in
  `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/` under a mangled name `🛂️manifest.jsonnakagin.manifest.json`). Shipped example documents carry only
  `meta.manifestId` (no `meta.kindCatalogs`), so the engine kind catalogs of the Nakagin example (the e2e document) resolve to nothing. **[HYP]** root cause
  of most of W2-D's 08:01 failures (see W2-D) and a risk for handle/proximity/fill in the e2e.
- **H4. Peers changed channel-version and cargo layout after the fleet died:** `OS/🧑‍💻dev/🔖️channel-version/**` (08:02–09:25, census of
  `appChannelVersion`/`CHANNEL_VERSION` literals), and 210 `Cargo.toml` files got `workspace = "../../../../.."` at 09:11:54 (incl. both new modules).
  Re-run `cargo metadata`/a gated `cargo check` before trusting any cargo result; never assume the old build-dir still applies.
- **H5. ShellHost is being refactored by a peer** (`RE/🏛️ShellHost/🟦️.tsx`, 11:12: inference → installed-service rename, `backboneWorkerFactory`,
  `documentServices`, import of `💡️inference/🔌️service`). W2-B's React suites and the renderer-react typecheck must be re-run after it settles.
- **H6. Two sibling tickets of this ticket (other sessions) collide with W1-G/W2-A:**
  `PAGED-ARTIFACT-HISTORY-LEDGER` (⚪fcf22be5, editing `OS/🌿️vcs/🦀️.rs` now, +370 lines of `HistoryPageStack`, replaces `ARTIFACT_HISTORY_LEDGER_CAPACITY`
  semantics, will touch `reserve_edit_history_slot`, `vacancies`, `extract_if`, `CursorRevisionAccumulator`) and `PER-VIEWER-ALTERNATIVE-HEAD`
  (⚪bfafbe85, opened 11:23: makes `Branch`/`Checkout` per-viewer, rewrites `fold_history`, `reproject`, hydration — i.e. the trunk/`active_line_id` model of W1-G follow-up 2 and
  W2-A's alternatives section). Neither is ours; successors must re-read before every edit and expect API churn in `vcs`/`store`.
- **H7. Peer sqlite-snapshot rollout** (`ArtifactSqliteSnapshot`, untracked `🪶️sqlite` dirs in wfc/stdio): `semio-s-artifact-stdio-las` fails `E0599`
  (`🎩️header/🧬️schema/📸️snapshot/🦀️.rs:206`, seen 07:48 and 07:55). It aborts wide `cargo check` runs that include stdio crates; check puzzle/plugin crates by `-p`.
- **H8. Machine load ~40–50**: timing laws (8 ms fill step, `tool_run_overlay_append_per_tick`) fail at random; every fleet run must keep the gate
  `until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 8 ]` and private `CARGO_TARGET_DIR=…/target-nde-<wp>`.
- **H9. Taxonomy validity is unknown now.** It was invalid at 07:40 (peer edits to `plugin-registry`/`wgpu-frame-worker` inputPatterns, `📽️nested-cargo-package-projection`
  catalog restored by W3-TAX). Activation fails fast when it is invalid; check `bun nx show projects` first.
- **H10. Shared scratchpad:** the four reconstruction auditors share `…/scratchpad/`; do not rely on any scratch file you did not just write.

---

## 3. Dependencies between the core WPs

```
W1-A types ─► W1-B/W1-C (types only) ─► W2-A (ledger + tool runner) ─► W2-B/W2-C (wire + band) ─► W3-E2E
W1-G store ─► W2-A (state_before, begin_report_replay, commit_finished_replay, Edit.verb, active_line_id)
W1-E contract ─► W2-A panel recipes ─► W2-B/W2-C staged controls
W1-C ToolMachineRunner ─► W2-D select tool ─► W2-A Emit::commit_transaction
W2-A HostEvent ─► W2-D `ArtifactEditor::host_event` ─► W2-B/W2-C forwarders
W1-G follow-up 5 (store) ─► W2-A runtime route (not started) ─► W3-T2-STROKES remodel conversion (waits)
W3-E2E needs: H2 fixed + activation of W2-A/B/C/D final state
```
Open cross-WP edges: (a) W2-A ↔ W2-D hostEvent roster laws (both stale, see W2-A/W2-D); (b) W2-B ↔ W2-C shared corpus
`RE/../🧫️fixtures/📎️local-folder-bindings/🔣️.json` is consumed by wgpu only and has no schema; (c) W1-G §15 store ↔ W2-A runtime shape rule; (d) W2-A refusal
codes ↔ W2-B/W2-C notice tables.

---

## 4. Work packages

### 4.1 W1-A — replication (`Supersede`, `TransactionRef`, `ReplayReport`)

1. **Last assignment.** Follow-up 2 (transition id excludes the actor string), done 08:20 on 09-30. Later `R/**` edits belong to others: W2-A added `verb` to
   `MutationEnvelope` (binary flags varint: bit0 transaction, bit1 verb; Rust `R/🔗️causal/🦀️.rs` 906–973, 1160; TS `R/🟦️.ts` 07:33, `R/👕️peer-overlay/🟦️.ts`; corpus
   `🔗️causal/🧫️fixtures/🧮️document-backbone-batch-v1`), W2-A also bit 13 `historyEdit` presence, W1-G the trunk id / shapes / retract, W3-CODES the outcome-code table.
2. **Done [RAN].** Replication lib 307/307 → 313/0 (W1-G follow-up 3); replication TS 17 → 21 passing; Python corpus generator reproduces byte for byte; os-kernel sync 1303/1303;
   os TS (backbone + worker) 382/382. Types/laws per design §2 are all present (`Supersede` tag 6, `InputReplacement`, `EffectiveSupersession`, `HistoryFold.supersessions`,
   `TransactionRef::mint`, `MutationReplayOutcome`, `ReplayReport::blocks_finalize`, trunk id, `HistoryShape`).
   **Claimed but unverified since:** the `verb` wire flag (W2-A, 03:30–03:35 Rust, 07:33 TS). W1-G measured `313 passed / 1 failed`
   (`causal::tests::envelope_transaction_round_trips_through_binary_and_value`) at 03:09 while that change was in flight; W2-A edited
   `R/🔗️causal/🧪️tests/🔬️unit/🦀️.rs` and `R/🧪️tests/🧪️{document-backbone-envelope-batch,transaction-ref}/🟦️.ts` at 03:32; **no replication test log exists after that**.
3. **Remaining (checklist).**
   - [ ] `cargo test -p semio-framework-replication --lib` (expect ≥ 313, 0 failed) and `cd R/📦️packages/🟦️typescript && bun ./📜️script.ts test` (≥ 21) and `python3 R/🧪️tests/🧪️history-transition/🐍️.py`.
   - [ ] Confirm the `verb` envelope additions are covered by the three encoders: add `verb` cases to the Python generator for `🧮️document-backbone-batch-v1` if the corpus is hand-written (12 `verb` mentions in the fixture, 2 in the schema — check that `🐍️.py`-style independent generation exists, else add the independent encoder).
   - [ ] Sweep hex literals / `.bin` frames for the flags byte again (the W1-A sweep predates `bit1 verb`): `R/🧫️fixtures/📡️wire/*/💾️.bin`, `OS/🧪️tests/🧪️backbone-envelope-io/🟦️.ts` (modified 08:51 by a peer), `🏪️store/👷️worker`.
   - [ ] Record `verb` (design §2 addendum) and bit 13 `historyEdit` in `📋️design.md` (coordinator) — W1-A's report does not know them.
   - [ ] Pre-existing debt flagged earlier, unchanged: unregistered taxonomy kinds (`🔗️causal/🔀️transition`, `🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata`), `db_sync` 50k WAL-close failure.
4. **Interrupted-edit suspicion.** None in W1-A's own code. `R/🟦️.ts` and `R/👕️peer-overlay/🟦️.ts` (07:33:27–29, W2-A) have no later test evidence (TS balance not scanned).
5. **Blockers.** None. Depends on W2-A for the `verb` additions.
6. **Brief.** W1-A is finished and green as of 09-30 08:20. Do not extend it. Your only job is to re-prove the replication crate and TS twin after W2-A's `verb`/`historyEdit` wire additions (commands above), fix any hex/`.bin` fixture that predates the flags varint, and fold the additions into the corpus + generator so the three-encoder rule still holds. Touch `R/` only for fixture/test repairs.

### 4.2 W1-B — `⏪️time-travel` module

1. **Last assignment.** Audit follow-up (M-3 `review()`/`Rerun`, N-2 validators, N-5, N-7), done 09-30 05:5x.
2. **Done [RAN].** Rust 12/12, bun (ajv + xstate + fast-check) 18/18 at `fundamental` and `quick`, wasip2 + wasm32-unknown-unknown checks, clippy clean, taxonomy clean. 35 legal matrix rows, 22 contexts, 45 cases, 10 scenarios. Consumed by W2-A (`P/⏪️time-travel`), W2-B (labels parity test), W2-C (`WGPU` dependency). Nothing unverified.
3. **Remaining (all optional audit items, unrequested):** N-1 (no `CloseFinalizePrompt` effect; hosts infer from stage), N-3 (no `TargetsLost`; W2-A exits on vanished targets in its base watch), N-4 (Finalizing exit contract note), N-6 (fast-check compares effect kinds, not `showPreview`/`commit*` payloads), N-8 (choice/frozen labels duplicated in `M` and `P`). Only do N-6/N-8 if a successor wants them; none blocks the e2e.
   - [ ] Re-run once after H4: `cargo test -p semio-framework-time-travel`, `bun test <absolute path>/🧪️tests/🧪️conformance/🟦️.ts` (absolute path! relative paths without `./` make bun scan the repo and segfault).
4. **Interrupted-edit suspicion.** None (last file edit 09-30 05:49; Cargo.toml touched 09:11 by the peer sweep, content = one `workspace =` line).
5. **Blockers.** None.
6. **Brief.** Treat as frozen. Re-run the two suites once after the cargo-layout sweep; if W2-A needs a new event or label (e.g. a `TargetsLost`), change the Rust owner, the TS twin, the schema, the lifecycle-law fixture and its Python generator together (`T/🧪️w1-b-generate-lifecycle-law.py`).

### 4.3 W1-C — `🛠️tool-machine` module

1. **Last assignment.** Audit M-1/M-2 (host `abort(reason)`/`reset()`, rest law `Unclosed`) + persistence API (`resume`/`into_parts`) for W2-D, done 09-30 05:54.
2. **Done [RAN].** Rust 13/13, bun 18/18 (ajv, xstate, fast-check, Map oracle), wasip2/wasm32 checks, negative controls. **Counts are stale:** the module was extended by W3-T2 on 10-01 02:5x–03:31
   (`ScrubMachine`/`ScrubPhase` region 457+, typing law, `🧫️node-drag-law`, `🧫️scrub-law`, `🧫️typing-law` fixtures; unit test file now has ~80 functions incl. scrub/typing/node-drag laws; schema 71 KB). Those additions are W3-T2/W3-T-FLOWCAD's and were last run by them.
3. **Remaining.** Optional audit N-10 (clock on every event), N-11 (no `InertHost`), N-12 (schema `minLength` unchecked in code), N-13 (TS `entries()` returns the live array), N-13b (probe charts not in fixture).
   - [ ] Re-baseline both suites (`CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w1c cargo test -p semio-framework-tool-machine`; `bun test ./FW/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts`) so W1-C's report counts match reality; if W3-T2 left them red that is W3-T2's, report it.
4. **Interrupted-edit suspicion.** None for W1-C. The 03:30–03:31 edits are W3-T2's and balanced.
5. **Blockers.** `ToolStep::Aborted(..)` pattern in the puzzle select tool depends on the reason enum; unchanged.
6. **Brief.** Do not redesign. Re-baseline and, only if asked, close N-10..N-13b. The runner API (`start/send/abort/reset/resume/into_parts`, `Unclosed`/`Closed` refusals, `ToolAbortReason`) is what W2-D and W2-A depend on.

### 4.4 W1-E — UI contract

1. **Last assignment.** Follow-up 2: wgpu tree rows render recipe content (`UiTreeItemNode.content_lines`, corpus case `🌲️tree-row-recipes`), done 09-30 18:45 (report mtime 18:30 + §7).
2. **Done [RAN]** (18:45): `semio-framework-ui-contract --lib` 215 ✔; `semio-framework-ui --features testkit --lib` 761 ✔; TS corpus 71 ✔; renderer-react Interpreter 168 ✔, typecheck 0; wgpu `time_travel` 17 / `tree` 29 ✔; `exports_typescript_bindings` ✔; number-controls 126, color-input 31 TS checks (decimal.js, color-string oracles).
   **Not re-run:** renderer `staged` filter (peer WIP at that moment), full `shell::` module (global-state contention: ~100 order-dependent failures, pre-existing).
3. **Remaining.**
   - [ ] Re-run after the peer's 11:32 edits to `FW/🖱️ui/🎯️targets/🧊️wgpu/{🧩️component,🔀️reconcile,🎬️action}`: `cargo test -p semio-framework-ui-contract --tests -- --test-threads=1`, `cargo test -p semio-framework-ui --features testkit --lib`, `cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel dialog_choices staged tree`.
   - [ ] wgpu chrome dialog stages reference ids as plain strings (`WGPU:24739 stage_references`), while the staged-arg row already honours `id_type` (`WGPU:6570`); mirror it (W2-C finding 4; only matters for dialog reference args, not the history editor).
   - [ ] `decimal.js@10.6.0` and `color-string@1.9.1` root devDependencies must be recorded in the `verify dependencies` baseline (coordinator, W3-G).
4. **Interrupted-edit suspicion.** None in W1-E files since 18:45. The peer's 11:32 wgpu `ui` edits (bounded-action bytes) are adjacent, uncommitted and unrelated to W1-E.
5. **Blockers.** A peer is editing the same wgpu UI target right now (§0: 11:32–11:35, uncommitted).
6. **Brief.** W1-E is complete. Re-run the three UI suites once the peer's `bytes` edit settles, fix only regressions in the slider/stepper/precision/choices/recipes/tree-row code, and mirror typed reference ids in the wgpu dialog if time permits. Do not touch the dialog region unless a W2-C law demands it.

### 4.5 W1-G — store core (incl. design §15 transaction-scoped amend)

1. **Last assignment.** Follow-up 5 = design §15: **transaction-scoped amend** (Owner: store + runtime shape rule, Rust + TS twin, laws; W3-T2-STROKES converts remodel on top). Follow-up 4 (hub no forced rebuild, `history.transition-refused` text, browser-actor retract) finished and reported 10-01 03:09.
2. **Done [RAN] (reports).** Design §3 items 1–9, audit fixes F-C1/F-M1..M6, trunk as first-class alternative, config stores refuse non-undo/redo at dispatch (`VcsError::HistoryShape`), `BackboneMessage::Retract` (Rust + wasm + TS actors, parity scenario), all-or-nothing command decode (`decode_command::<P>`), two-replica + hub concurrent-supersede law, exact authored HLC relay (TS `encodeClientCommandsFrameExact`). Kernel per-test runner 883/0 (`os_store:: os_vcs:: os_spr::`), replication 313/0, replication TS 21, os TS worker 14, hub bin 173 incl. concurrent-supersede law.
   **Follow-up 5, as found on disk [SRC + COMPILED]:**
   - `OS/🌿️vcs/🦀️.rs` ~987–1019: `VcsError::TransactionOpen{transaction_id}`, `VcsError::UnknownTransaction(String)` (+ Display).
   - `OS/🏪️store/🦀️.rs`: `ArtifactCommand::{AppendTransaction{mutations, transaction}, CommitTransaction{transaction_id}, AbortTransaction{transaction_id}}` (3086–3105, ordinals 19/20/21), `OpenToolTransaction{transaction, edit_id}` (3106), projection-cause mapping (3138–3165), text codec (`CommandHeaderLine` 13476–13488, print 13979–13990, parse 14071–14079), binary codec (14360–14380, 14490–14495), `retire_command` (14210–14229), store field `open_transaction` (16674; ctor 16821/16972) + accessor (17155), dispatch gate (19688–19699: while open only its own appends, commit/abort, `IngestRemote`, `SetMergePolicy`; everything else `TransactionOpen` and the command retires), `append_transaction`/`open_transaction_edit`/`commit_transaction`/`abort_transaction`/`keep_open_edit_at_tail` (19776–19867), `event_log()` excludes the open edit (20517).
   - Kernel lib `Finished` 07:53:32 (`w1-g/f5-kernel-check.log`) after two transient mid-edit errors at 07:51–07:52 (`E0560 open_transaction`, `E0004 AppendTransaction` in `f4-hub-build.log` and `w3-t-flowcad/plugin-check-3.txt`); plugin + plugin-host lib `Finished` 07:55. **No test, no fixture, no report entry exists for follow-up 5.**
3. **Remaining (checklist).**
   - [ ] **Laws (Rust, `OS/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs` or a new `🧪️tool-transaction`)**: N appends + commit equals one `Apply` of the same ops (state, one edit, ordering, revision, messages, every op stamped with the `TransactionRef`); abort leaves zero trace (projection, `applied_edit_ids`, ledger count, `edit_messages`, revision equal to before the first append, no announcement on the backbone); every non-admitted command while open is refused `TransactionOpen` **and retires its operations**; `IngestRemote` while open then append/commit keeps the open edit at the local tail (`keep_open_edit_at_tail` re-stamps clocks); `SetMergePolicy` while open; commit/abort with a wrong id → `UnknownTransaction`; `AppendTransaction` with an empty op list → `EmptyApply`; undo/redo/supersede/checkout while open refused; ledger-full during an append (64 slots, H6) leaves the open edit consistent.
   - [ ] **Codecs:** `assert_command_text_binary_equivalence::<P, Op>` for the three new variants; `a_refused_command_decode_retires_every_operation_it_decoded` extended to ordinal 19 (garbled third op, trailing bytes, hostile count).
   - [ ] **Language-agnostic fixture + schema:** `OS/🏪️store/🧫️fixtures/🧫️tool-transaction/🔣️.json` + `🧬️schema/🔣️tool-transaction/🔣️.json`, checked by Rust and a TS/Python replay (fast-json-patch oracle, like `🧫️supersede-replay`).
   - [ ] **Persistence decision (not made):** `print_document_text/pack`, `.spr`/`.ops` writers and `materialize` do not look at `open_transaction`; an open edit (`finished_at: None`) would be written and, after reload, folded as a normal edit and announced on attach. Either exclude it from every persisted form (zero-trace abort across a crash) or document that reload commits it; add a law either way. `finished_at: None` is also used by coalesced amends, so reload cannot tell them apart today.
   - [ ] **Runtime shape rule (W2-A file but §15 names W1-G):** `P/🦀️.rs:24053 tool_transaction_shape_fault` still refuses any transaction with a coalesce key and nothing emits `AppendTransaction`; add the streamed route: stream tick → `AppendTransaction` (same ref), commit → `CommitTransaction`, abort/blur/captureLost/frozen/baseMoved/retired → `AbortTransaction`; make `historyEditBegin` busy while `store.open_transaction().is_some()`; make every runtime path that would dispatch a refused command surface `TransactionOpen` as a typed fault. Then notify W3-T2-STROKES.
   - [ ] **TS twin:** check whether any TS codec of `ArtifactCommand` exists (grep found none; the guest→store command path is Rust-only). If none, state "no TS twin needed" in the report.
   - [ ] Re-run the full per-test kernel runner (`T/🧪️w1-g/run-each-test.sh`, `RUST_MIN_STACK=268435456`) after H6 settles; baseline = 883 ok / 0 fail.
   - [ ] **Report:** add Follow-up 5 (and re-confirm Follow-up 4 numbers) to `📓️w1-g-report.md`.
4. **Interrupted-edit suspicion.** `OS/🏪️store/🦀️.rs` (balanced, +500 lines vs 23:43, the §15 region plus peers' sqlite/member-factory hunks at ~7067, 10550–11215, 23913–25827) and `OS/🌿️vcs/🦀️.rs` (07:48:29 W1-G error variants; **now modified in the working tree by the PAGED-LEDGER session at 11:44, uncommitted**). The store file was last written 11:13:44, i.e. after the fleet died (08:16), so by a peer or another session, not by W1-G (HEAD includes it; I did not identify the hunk). The §15 region reads complete; what is missing is verification, not code. One thing to double-check by test: `abort_transaction` removes the edit with `extract_if` and relies on `reproject()` to drop the id from `applied_edit_ids` and to restore the tail-undo cache (`replace_tail_undo_cache_retained` was set in `open_transaction_edit`).
5. **Blockers / dependencies.** H6 (paged ledger edits the same `vcs` file and the ledger API that `abort_transaction`/`retract` use; per-viewer-head will rewrite fold/reproject/hydration), H7 (sqlite rollout touches `ArtifactCodec`), the broken hub/plugin test targets until peers finish. Downstream: W2-A (runtime route), W3-T2-STROKES (remodel).
6. **Brief.** Finish §15 and prove it. The store half is written and its lib compiles (07:53); you owe the laws, the fixture/oracle, the persistence decision and the report. Start with `cargo check -p semio-framework-os-kernel --tests --features sync` (gated, shared target) to learn whether the test targets still compile, then write the laws first (abort zero-trace and N-appends-equal-one-apply are the two that matter), then coordinate the runtime route with W2-A. Re-read `vcs/🦀️.rs` and `store/🦀️.rs` before every edit: two other tickets are rewriting them.

### 4.6 W2-A — plugin runtime

1. **Last assignment (reconstructed from `📓️status.md`; the report stops at 09-30 17:43).** After follow-up 2 (window transient, ui_scope, generation, interaction rows, op-lines) the coordinator sent: (3) host events, (4) alternatives section + switch, R2-1 section order, R2-5 `hostEvent` declared, trunk "Main line" via `active_line_id`, then the **reload-label gap** ("locale-neutral verb on the edit", needs `Edit.verb` in the store and on the wire) and, on resume 07:21, "complete interrupted edits compile-atomically".
2. **Done.**
   - [RAN, report] items 1–8 of the plan, history wire (§10), 12 verbs, ledger, presence `historyEdit`, undo/redo of finalize, input addressing (unions, nullable, snaps, colour), `ui_history_panel`, finalize dialog. Plugin lib 904/13 → 912/14 → 918/13 (all failures peer/baseline: `merge_ui_values` ×4, `tool_run` ×4, `window_kits` ×2, `command_ingress_terminal` ×1, `activated_tool_factory_keys…` (`hostEvent`), `neutral_checked_diff_boundaries…`); `time_travel_tests`+`supersede_ledger_tests` 20→23 passed; kernel TS 77; puzzle 2d 1061/5 at 18:xx.
   - [SRC + law written, status.md says "laws pass" at 22:43] R2-1: `ui_history_panel` (`P/🦀️.rs:12198–12331`) puts band/editor first (`session.chain(alternatives).chain([actions, commands])`), editor inputs unwindowed up to 64, React `mergeTreeSectionOrder` fix (`FW/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, wgpu twin law in `🧪️targets-wgpu-reconcile-unit`); law `the_session_leads_the_history_body_and_the_editor_inputs_are_all_materialised`. (4) alternatives (`history_panel_alternative_row`, trunk read "Main line/Hauptlinie", Switch row action + `SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID`), law `the_alternatives_section_lists_each_alternative_and_switching_reprojects_its_history_edits` + `🧪️history-alternatives` Rust/TS oracle. (3) `HostEvent` (`WindowBlurred`, `PointerCaptureLost`, `UtilityChanged`/`Retiring`, `TimeTravelFrozen`, `BaseMoved`), `HOST_EVENT_ACTION_ID` framework-declared and injected into every window kind (`P/🦀️.rs:23962, 23974, 26561, 30053`), law `opening_a_history_edit_and_a_remote_edit_deliver_host_events_to_every_window`; React forwarder `RE/🏛️ShellHost/🟦️.tsx:1066`, wgpu forwarder `WGPU:14952`, `RE/🧪️tests/🪟️window-host-events`.
   - [SRC + compiled 07:27 test target / 07:55 lib] reload label: `Edit.verb` (store `authoring_verb` + `set_authoring_verb` stamped at `P/🦀️.rs:28000, 28055, 28317`; wire flags bit1 in `R`), backfill `backfilled_edit_label(edit, verb_label)` (`P/🦀️.rs:11893`, 27205, 27233, 27432), `retire_displaced_document_rows`; new law `P/🧪️tests/🧪️history-label-reload/🦀️.rs` (mounted `P/🦀️.rs:7287`) + fixture/schema `P/🧫️fixtures/🧫️history-label-reload/` + TS oracle registered in `P/📦️packages/🦀️rust/📜️script.ts:33,55`. **The test target was last compiled 07:27, before the 07:42–07:44 verb edits; this law has never run.**
3. **Remaining (checklist).**
   - [ ] Gated `cargo check -p semio-framework-plugin --tests` then `cargo test -p semio-framework-plugin --lib -- time_travel_tests supersede_ledger_tests history_label_reload history_alternatives ui_history_panel rendering_the_history_body` with `CARGO_INCREMENTAL=0 RUST_MIN_STACK=268435456`; then the full lib per test; baseline = 918 ok / 13 peer fails; any new red is yours. Also `cd P/📦️packages/🦀️rust && bun ./📜️script.ts test` (TS oracles: time-travel 8 cases, supersede-ledger 7, history-alternatives, history-label-reload) and kernel TS 77.
   - [ ] **Fix the `hostEvent` fallout** (determined, not hypothetical): `P/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:2571 activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations` — its two expected sets (`framework_registered_without_declaration`, `framework_directly_routed_migrated`) contain no `hostEvent` (W1-G saw it red at 21:27; the test was edited 03:33 but still has no `hostEvent`); and the sibling roster law in puzzle 2d (see W2-D).
   - [ ] **§15 runtime route** (see W1-G): refine `tool_transaction_shape_fault` (`P/🦀️.rs:24053`, callers 27913 and 31550) so a streamed transaction is admitted, route stream/commit/abort to the new store commands, abort on every `ToolAbortReason`.
   - [ ] **Refusal notices for the plugin-level codes.** `timeTravel.busy|unknown-mutation|not-editable|unknown-input|invalid-input|no-selection|name-required|schema-unavailable` (`P/⏪️time-travel/🦀️.rs:41–49`) have no entry in React `HISTORY_REFUSAL_LABEL_KEYS` (10 codes: 3 hub + frozen/illegal/stale/blocked/empty/cancelled/name-invalid) nor in wgpu `time_travel::history_refusal_notice`. Verify which of them are shown elsewhere (editor "refused" row for `invalid-input`/`schema-unavailable`); give the rest en/de notices in both shells (coordinate with W2-B/W2-C; "no default language").
   - [ ] **Report:** `📓️w2-a-report.md` §7.8 still says (3) and (4) are "not started"; add Follow-up 3 (R2-1, R2-5/host events, alternatives + trunk, `Edit.verb` + label reload, tests) and correct §7.8.
   - [ ] **Descriptors** are a coordinator chore (12 verbs, finalize dialog, `hostEvent`, `noteShellCommand.label` shape): `describe` for the puzzle plugin is the one that gates activation (H2).
   - [ ] Watch H6 (per-viewer head changes what `active_line_id`, `Branch`/`Checkout` rows and the alternatives section mean).
4. **Interrupted-edit suspicion.** `P/🦀️.rs` (07:44:55), `P/⏪️time-travel/🦀️.rs` (07:52:14), `P/📦️packages/🦀️rust/📜️script.ts` + new law/fixture (07:42–07:43): all balanced, no stubs, referenced harness symbols exist (`settle_registered_typed_operation`, `new_registered_app`, `document_text/pack`, `load_document_text/pack`, `history_patch`, `refresh_cache`), every `#[path]` mount resolves. Lib compiled at 07:55 (after the last edit); the **test target was last compiled 07:27**. The 07:52:14 write to `⏪️time-travel/🦀️.rs` may be W3-T-FLOWCAD's `HistoryMutationEntry.store` work (design §12) rather than W2-A's; `P/🛠️tool-machine/🦀️.rs` (07:52:19, scrub/typing glue) is W3-T2's.
5. **Blockers / dependencies.** Needs W1-G for §15 and for any store API change; shares `P/🦀️.rs` with W3-T2-CONTROLS/TEXT/STROKES, W3-T-FLOWCAD (child stores, §12), W3-CODES; H6; peers' `describe` wave; the `🧪️tests/🧬️mutation-fixtures-*` sqlite files (H7).
6. **Brief.** The code is ahead of the report. First prove the last edits compile and pass as a test target (command above), fix the `hostEvent` roster law and the missing refusal notices, add the §15 runtime route with W1-G, then rewrite the report's stale tail. Do not redesign the ledger, the panel or the wire; they are what React, wgpu and the probe now depend on. Keep `P/🦀️.rs` edits compile-atomic: at least four other executors write into it.

### 4.7 W2-B — React shell

1. **Last assignment.** Follow-up 4 (event-sourced `os.config.local-folders`, `LocalFolderReconnectBand`), done 10-01 03:21; follow-up 3 (typed rejections, folder re-attach R2-2/R2-4, R2-6 check-in, dev reload, hub-less serve) done 02:xx.
2. **Done [RAN].** Interpreted history body, band + window indicator + chords + `ui.timeTravel.*` i18n, refusal notices (10 codes), staged arg controls, rerun/review model, progress folding (`AppChannelClient` routes patch-only frames), `noteShellCommand` both languages, peers' history-edit presence, typed `CommandAckOutcome.rejected` (10 codes, schema + fixture, Rust + TS; sync 87/87), folder archive restore fix chain (identity via `readAppDocumentIdentity`, own-write echo, hydration through `restoreDocumentArchiveV1`, `retire_displaced_document_rows` law), `dispatchCheckpoint` frozen during time travel, page-wide `ShellScopeContext`, hub-less serve env fix, local-folder facet (Rust + TS twins, quintets, host case 12/12, os-config 193/193, `folder-archive-restore` 4/4, React band 3/3). Typechecks at 03:21: renderer-react 4 peer errors, ui-react 2 peer errors, framework-os no error in touched files.
   **Claimed but unverified live:** every R2-2/R2-4/R2-6 fix and the reconnect band in a real browser (activation never succeeded after them).
3. **Remaining.**
   - [ ] Re-run after H5: renderer-react + ui-react typecheck; `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts` on `⏪️time-travel/🧪️tests/🧩️component`, `🛠️ShellHelpers/🧪️tests/{🧪️staged-arg-controls,⚔️hub-command-rejection,🧪️command-rejection,🌐️chrome-history-locale}`, `🏛️ShellHost/📎️local-folders/🧪️tests/🧩️component`, `🔬️engine-contract` (3 baseline fails: text-editor paste/compose, `Worker is not defined`), framework-os `🧪️folder-archive-restore` + `🧪️backbone-envelope-io`.
   - [ ] **Adopt the shared local-folder corpus in React:** `OS/📺️renderer/🧑‍🎨engine/🧫️fixtures/📎️local-folder-bindings/🔣️.json` (14.7 KB, written by W2-C at 03:16) is read only by `SH/🧪️tests/🧪️wgpu-local-folders`; it has **no JSON Schema** (every sibling corpus has one). Add `🧬️schema/🔣️local-folder-bindings/🔣️.json`, validate with Ajv in a React test (like `🧫️time-travel-peers`).
   - [ ] Refusal notices for the plugin-level `timeTravel.*` codes (with W2-A), en/de keys in `UI/🧱️elements/📚️I18n/🟦️.tsx` + both bundles.
   - [ ] **Live re-verification after activation** (probe steps 5, 7, 9): `document-rows-survive-the-reload`, `overwrite-row-survives-the-reload`, no `local.backbone-scope-mismatch`, no `commitCheckpoint refused`, reconnect band flow.
   - [ ] Taxonomy of the new dirs is **unverified** (`verify taxonomy report --scope RE/🛠️ShellHelpers` crashed on an unrelated digest in 09-30).
4. **Interrupted-edit suspicion.** None in W2-B files (last write 02:52). `RE/🏛️ShellHost/🟦️.tsx` was rewritten in parts by a peer at 11:12 (H5): read the diff before editing; imports `../../../../💡️inference/🔌️service/🟦️.ts`.
5. **Blockers.** H5; activation (H1/H2); W2-A for the notice codes.
6. **Brief.** Nothing is half-done. Prove the React side still typechecks and passes after the peer's ShellHost refactor, add the schema + React consumer for the local-folder corpus, add the plugin-level refusal notices, and then support W3-E2E's re-probe (steps 5/7/9) with fixes if the live run shows any. Never edit ShellHost without re-reading it first.

### 4.8 W2-C — wgpu shell

1. **Last assignment.** Follow-up 5: native/wgpu **folder reattach parity** with W2-B's `os.config.local-folders` binding (started 03:05; cut ~05:50; resumed 07:21). **No report entry.**
2. **Done [RAN] (03:05).** Band + indicator + chords + finalize prompt + refusal notices by code (10) + staged editors (Actions/palette/dialog) + peers' presence + shared corpora (`🧫️time-travel-band`, `🧫️time-travel-peers`, `🧫️command-rejection`) + P2 `shell.notice` ARIA status node + P1 `dumpBoard2d`. `cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel dialog_choices introspection_tests` 40/40; vitest `♿️wgpu-accessibility-interaction` + `📨️browser-frame-transport` 86/86; `🧩️package-integration` 28/28; wasm32-unknown-unknown and native `cargo check` ✔ (03:05).
   **Follow-up 5 [SRC + COMPILED 08:00, never run]:** `SH/🎯️targets/🧊️wgpu/📎️local-folders/🦀️.rs` (414 lines, mounted `WGPU:1766`, wired at `WGPU:4079, 7188, 11388–11400, 11686–11694, 26373`: remember on attach, reattach natively on boot, browser "Reconnect folder"/"Forget folder" band with polite status node, `FOLDER_RECONNECT_ACTION`/`FOLDER_FORGET_ACTION` on controller `framework.sync`) and `SH/🧪️tests/🧪️wgpu-local-folders/🦀️.rs` (227 lines, includes the shared corpus). `w2-c-build.txt` (08:00) is `--no-run` only; `w2-c-test-tt.txt` (07:40) was cut during compilation.
3. **Remaining.**
   - [ ] `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2-c cargo test -p semio-framework-os-renderer-wgpu --lib -- time_travel dialog_choices introspection_tests local_folders` (expect 40 + the new local-folder laws); `cargo test -p semio-framework-ui --features testkit --lib -- conformance_corpus presence_bar`; `bun ./📜️script.ts test-browser` and `test-preview-generated` in the wgpu TS package (86 + 28); `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` (the browser build is what 6112 serves; last verified 03:05).
   - [ ] Add the missing **schema** for `📎️local-folder-bindings` and make W2-B's React test read the corpus (see W2-B).
   - [ ] Plugin-level `timeTravel.*` notices (see W2-A) in `time_travel::history_refusal_notice`.
   - [ ] Re-run the same wgpu laws after the peer's `🖱️ui` wgpu edits (§0) and the cargo-layout sweep (H4).
   - [ ] Browser-only gaps listed in the report and never closed: live sync notice path, painted note pixels, remote-undo route (documented as unreachable: six prerequisites in `📓️w2-c-report.md` F3; not needed for the e2e).
4. **Interrupted-edit suspicion.** None found: `WGPU` last write 03:38:36, balanced; the follow-up-5 module and its test are complete files (test ends with `//#endregion 🌐️GestureBand`, module ends with `mod tests;`). They have never been executed.
5. **Blockers.** H4/H5-style churn in `🖱️ui` wgpu; activation for any browser verification.
6. **Brief.** Source for follow-up 5 is complete; prove it. Run the wgpu laws (list above) in a private target dir, fix what the never-run local-folder laws expose, write the corpus schema, then write follow-up 5 into the report. When the serve exists, be on call for W3-E2E's wgpu `--explore` calibration (mirror keys).

### 4.9 W2-D — puzzle 2d tool + engine

1. **Last assignment.** Follow-up after the first report: brush/fill root-cause fixes, f32 shortest-decimal offsets (engine + React), example-loader no-op, host events through `ArtifactEditor::host_event`; resumed 07:21, "compile-clean" 07:31.
2. **Done [RAN].** Gesture records (drag/rotate/scale, one `gesture` row per gesture, `gestureId` on the `select` row), both coalescers over one 18-case corpus (TS 19, Rust corpus replay), `select_tool` `ToolMachine` + window-transient persistence + host aborts, one `ToolTransaction` per gesture via `Emit::commit_transaction`, row labels en/de via `mutation_label`, engine suite 52–53/53, wgpu `board2d` filter **6/6 (07:49, closes the "wgpu f32 replay unverified" item)**, React `board-event-coalescing` 19, `float32-decimal` 10/10, engine-contract puzzle filter 39, publication-authority audit ✔, wasm32-wasip2 plugin check ✔ 07:30.
3. **State at death — the 08:01 full lib run (`w2d-test-all.txt`):** 1071 tests listed; 1056 ok; **9 FAILED**: `editor::puzzle2d::component::unit_tests::every_declared_action_resolves_to_a_command`, `…::shipped_node_kinds_are_the_two_examples_own_catalog_rows`, `engine::brush::tests::board_host_fill_base_core_rectangular_excludes_cylindric_tambour`, `…::manifest_only_documents_resolve_engine_kind_catalogs`, `precompute::fill::tests::{fill_revalidate_job_retracts_conflicting_placements_and_reappends_survivors, fill_run_job_matches_the_language_neutral_fill_run_fixture, fill_run_job_resume_raise_continues_the_sequence_and_lower_retracts_the_tail, fill_run_job_places_only_inside_visible_target_regions, fill_run_job_step_with_one_unit_of_fuel_reaches_exactly_one_candidate_verdict}`; and the binary **SIGABRTed** in `fill_run_start_complete_finalize_is_one_undo_entry` (`fill/🧪️tests/🔬️unit/🦀️.rs:614` `left: Some((0, Some(8)))` vs `Some((8, Some(8)))`, then the `BoardFillJob::drop` assertion "must reach exact terminal-empty before Drop" in `♾️infinite/…/➕️normal/🦀️.rs:7477` → panic in destructor → abort). Five tests never reported.
   Triage notes: (a) `every_declared_action_resolves_to_a_command` (`ED/🧪️tests/🔬️unit/🦀️.rs:819`, last touched 09-30 18:36) has no `hostEvent` in its `reserved` list while W2-A injects `hostEvent` into every window kind — **determined**, one-line fix (`semio_framework::HOST_EVENT_ACTION_ID`). (b) the rest: **[HYP]** H3 — fill places nothing without kind catalogs (0 of 8), `manifest_only_documents_resolve_engine_kind_catalogs` uses `manifestId: "nakagin"`, and the generated registry now knows only `puzzle2d-default`. The peer also changed `ED/🦀️.rs` (`crate::graph_manifest::manifest_by_id`, `verb: None` literals, `host_event` builds `Puzzle2dCommand::TranslateSelection` directly), `ED/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs` and the board `➕️normal/🦀️.rs` (`validate_against_manifest(&GraphManifest)` replaces `…_by_id`). Verify by running the 9 tests singly after restoring the manifests (or giving the examples `meta.kindCatalogs`).
4. **Remaining.**
   - [ ] Fix (a); decide with the manifest peer how the Nakagin/Concrete-Forest manifests reach the puzzle crate (add them to `PZ/../◻️2d/🛂️manifest/📇️outputs.json` `manifests`, or inline `kindCatalogs` in the shipped examples); rerun the 9 tests; the 2 timing laws (`board_fill_job_large_host_has_no_step_at_or_above_eight_ms`, `fill_run_job_drive_step_stays_below_the_interactive_ceiling_for_nakagin`) need low load (H8).
   - [ ] Make the destructor assertion survivable in tests: a failing fill test must not abort the whole binary (e.g. retire/complete the job in the test's guard, or turn the Drop assertion into a debug-only report), otherwise one red hides 1000 tests.
   - [ ] Run `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib` per failing test, then whole lib; baseline before the peer's manifest change was 1061/5.
   - [ ] Gesture-level e2e items from the probe: `hostEvent` no longer dropped (verify in step 9), `Duplicate Selection` row label after reload (W2-A verb), sole-free-handle node cannot be dragged (UX note from probe finding 6, board `➕️normal/🦀️.rs:9344`).
   - [ ] Puzzle 5d editor changed at 07:40 by W3-T-PUZZLE (not W2-D); W2-D laws for 5d (`a_board_gesture_drag…`, 4 tests) should be re-run when that lands.
   - [ ] `📓️w2-d-report.md` does not mention the 08:01 run or H3.
5. **Interrupted-edit suspicion.** None of W2-D's own files (last 03:40). Peer edits at 06:22–06:28 in `ED/🦀️.rs`, board-host tests, board `➕️normal/🦀️.rs` and `🕸️dag/🦀️.rs` are the manifest rework (balanced, compile at 07:30/08:01).
6. **Blockers.** H3 (peer manifest registry), H8 (load), W2-A for the roster law fix.
7. **Brief.** Your engine/tool work is complete and proven. At death the lib run showed nine failures plus an abort that is probably a peer's manifest-registry change starving the engine of kind catalogs. Fix the determined roster law, resolve the manifest question with whoever owns it (the Nakagin example must resolve its kind catalogs again or the e2e's handle/proximity/fill paths silently lose them), make the Drop assertion non-fatal for the test binary, rerun the lib, then update the report.

### 4.10 W3-E2E — browser probe battery

1. **Last assignment.** Prepare Run 3 on both renderers: React re-probe after activation; `--renderer=wgpu` mode for 6112 (written 21:47, P1/P2 prerequisites delivered by W2-C at 03:05); step 5 adopted the reconnect flow (03:05).
2. **Done [RAN].** Probe `T/🔍️time-travel-probe.ts` (2198 lines, complete, ends with the `🔖️Main` region). Run 1 (en+de): 117 PASS / 27 FAIL (editor not rendered, etc.). **Run 2 (en and de, real controls only, no bypass): PASS 85 / FAIL 3, 0 uncaught, 0 hard faults each** (`probe-2026-09-30T16-49-01`, `T16-51-11`); the three FAILs are `document-rows-survive-the-reload`, `overwrite-row-survives-the-reload` (step 5) and `trunk-listed-after-new-alternative` (step 7). Run 3 partial (21:43, en, steps 1–3 after the wgpu refactor): **26/26**. wgpu mode: strict tsc clean, **never executed** (6112 never served).
3. **Remaining.**
   - [ ] Only after activation: `bun T/🔍️time-travel-probe.ts --port=6012 --only=1,2,3 --locales=en` (smoke, expect 26/26), then `--port=6012 --locales=en,de` (~10 min), expecting the three Run-2 FAILs to turn green (see §5) and step 9 to lose `hostEvent` / `commitCheckpoint refused` / `scope-mismatch` lines; then `--renderer=wgpu --port=6112 --explore`, calibrate mirror keys from `probe-wgpu-<stamp>-<locale>-s1-explore.json`, then `--renderer=wgpu --locales=en,de`.
   - [ ] New verdicts never run live: step 5 `folder-reconnect-offered` / reconnect / detach flow (React `#s-folder-reconnect`, wgpu `s-folder-reconnect` via `wgpuPress`), trunk ↔ alternative switching in step 7, `band-reads-choosing…` as a note on wgpu.
   - [ ] Update `📓️w3-e2e-report.md` with Run 3 (React) and the first wgpu run; delete any leftover probe bypass (none left); keep screenshots en+de; confirm console is `[DEBUG]`-free.
   - [ ] The positive two-peer case (⏪ badge, "is editing" notes) is not browser-testable without a hub-backed space; it stays covered by W2-C `the_roster_badges_and_announces_an_editing_peer_and_notes_its_rows` + the shared `🧫️time-travel-peers` corpus.
4. **Interrupted-edit suspicion.** None; the only mid-edit marker would be a half-written probe and it is complete. Evidence directories under `🗑️generated/e2e/` are large (≈630 MB) — leave them for the coordinator's closing sweep.
5. **Blockers.** H1/H2 (no serve), H5/H9 environment, peer Vite reloads (R2-3: peers' edit bursts reload the dev page mid-step; the probe records `page-reloaded-under-the-step` — discard such runs).
6. **Brief.** You cannot start servers; the main session must activate and serve (nohup + supervisor script `T/🔁️serve-supervisor.sh`, never from an agent). When told "up", run the smoke first, then the full battery per renderer, and report each verdict by step with the root cause for every FAIL and which WP owns it. Treat any `page-reloaded-under-the-step` note as an invalid run.

---

## 5. Critical path to a green puzzle 2d e2e — React (6012) and wgpu (6112)

### 5.1 What Run 2 found and where each finding stands in code

| Finding | Meaning | Fixed in code? | Evidence | Verified live? |
|---|---|---|---|---|
| R2-1 | band + editor below Actions/Commands, inputs windowed out | **yes** (W2-A) | `P/🦀️.rs:12317–12326` session-first order; `mergeTreeSectionOrder` fix; law `the_session_leads_the_history_body…` | no |
| R2-2 | history rows collapse to the newest edit after folder reload | **root causes fixed** (W2-B F3.2/F3.3, W2-A `Edit.verb`) | `restoreDocumentArchiveV1`, `retire_displaced_document_rows`, `Edit.verb` + `backfilled_edit_label`, laws `a_document_archive_round_trip_lists_every_history_row_of_its_source`, `history_labels_survive_a_text_and_a_pack_reload_in_every_locale` | **no** (needs a re-probe; the history-label-reload law has never run) |
| R2-3 | peers' Vite full reloads interrupt steps | environment | — | n/a |
| R2-4 | worker refuses every local batch `local.backbone-scope-mismatch` | **yes** (W2-B F3.2: document addressed by `readAppDocumentIdentity`) | `🏪️store/👷️worker/🟦️.ts:6887`, `folder-archive-restore` 4/4 | no |
| R2-5 | `hostEvent` undeclared in `2d-overview` | **yes** (W2-A framework-declared, W2-D `host_event`) | `P/🦀️.rs:23962+`, `ED/🦀️.rs:5027`; **but two rosters/laws were not updated** (see W2-A, W2-D) | no |
| R2-6 | `commitCheckpoint` fired during Finalize | **yes** (W2-B F3.4) | check-in effect keyed on program identity, `dispatchCheckpoint` frozen in time travel | no |
| trunk not listed | after finalize-as-new-alternative the original trunk could not be switched back | **yes** (W1-G follow-up 2 + W2-A rows) | `trunk_alternative_id`, `active_line_id`, law `a_new_alternative_preserves_the_trunk_it_branched_from` | no (needs the process restarted onto the new build; probe expects it listed) |
| wgpu P1 `dumpBoard2d` | no board introspection | **yes** (W2-C) | `RE/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` region `🔬️Board2dStats`, 3 Rust laws + vitest | no browser run |
| wgpu P2 notices in ARIA mirror | refusal code unobservable | **yes** (W2-C) | `shell.notice` status node, fixture `🧯️wgpu-transient-notice` | no browser run |
| wgpu P3 second peer | positive presence case | **not feasible** in a browser without a hub | one-tab negative in both shells | n/a |

### 5.2 What must land before activation + the probe battery can pass (ordered)

1. **Make the build environment loadable (peers; coordinator decision):** taxonomy valid and nx plugin load OK (H9); cargo workspace loads after the 09:11 `workspace =` sweep (H4); `bun nx show projects` works.
2. **Descriptor regeneration** (`describe`), minimum for the puzzle plugin (`bun nx run @semio-tech/puzzle-plugin:describe`) **and** whatever `plugin-registry:generate` scans: today 22/34 `🌎️hub/🧩️compositions/*/🔣️.json` are at `appChannelVersion 19` against const `20` (H2.1). Re-run after every W2-A wire/manifest change (12 verbs, `hostEvent`, finalize dialog).
3. **Clear the stale staging dir** `OS/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle` (owner moved with the manifest path, H2.2). The old serve is already broken (H1), so nothing is lost.
4. **Resolve H3 for the puzzle crate** (nakagin/concrete-forest manifests) or accept that brush/fill/proximity are dead in the e2e; fix W2-D's roster law and W2-A's bijection law so the lib suites are meaningful.
5. **Compile gates on the final fleet state** (each gated, private target dir, foreground): `cargo check -p semio-framework-os-kernel --tests --features sync`, `-p semio-framework-plugin --tests`, `-p semio-framework-os-renderer-wgpu --tests`, `-p semio-s-artifact-puzzle-2d --features component-app-assembly --tests`, `--target wasm32-wasip2 -p semio-s-plugin-puzzle`, `-p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown`. Known compiled at fleet death: kernel/plugin libs 07:55, puzzle plugin wasm 08:00, wgpu test build 08:00, puzzle-2d test build 08:01. Not compiled since the last edits: plugin test target (07:27), kernel test targets, replication tests.
6. **Activate (one activation for both renderers):** React `activate-puzzle2d-react-dev` (6012), wgpu `activate-puzzle2d-wgpu-dev` (6112); then serve both from the **main session** (`nohup`, supervisor loop; replace PID 81031). W3-E2E cannot do this. Expect ~45 min (activation #4 took 48 min).
7. **Probe order:** React `--only=1,2,3` → React full en+de → wgpu `--explore` → wgpu full en+de.

### 5.3 Expected probe outcome once 5.2 is done, and what could still fail

- React: Run 2's three FAILs should turn green (rows survive reload, trunk listed, trunk ↔ alternative switch); step 9 should lose the `hostEvent`, `commitCheckpoint refused` and `scope-mismatch` console lines; new reconnect-band verdicts in step 5 are untested.
- Residual risks to watch: (a) H3 → empty engine kind catalogs for the Nakagin example (drag/selection tests unaffected, proximity/`connect-handles`, fill and clone-then-drag could change); (b) the history-label-reload law has never run, so labels after reload (`Duplicate Selection`, `Set Active Example`) may still read as op text; (c) the 07:21–07:52 edits were never executed as tests (W2-A, W2-C follow-up 5, W1-G §15); (d) ShellHost peer refactor (H5) can break the React boot independent of this ticket; (e) Vite reloads from peer edit bursts (R2-3) invalidate steps; (f) first-ever wgpu run needs mirror-key calibration (`--explore`).
- wgpu: plan W.1–W.4 holds; P1/P2 delivered; first run expected to need calibration only (`mirrorFind` matches exact keys then a `/`- or `.`-delimited suffix). Verdicts that differ by design on wgpu: `dialog-offers-destructive-overwrite` (label/description, no tone) and `band-reads-choosing-while-the-dialog-is-open` (note; modal hides the band).

---

## 6. Evidence index (all under `T/🗑️generated/` unless noted)

`w1-g/f5-kernel-check.log` (kernel `Finished` 07:53) · `w1-g/f4-hub-build.log` (transient §15 mid-edit errors 07:51) · `w3-t-flowcad/plugin-check-{2,3}.txt` (plugin OK 07:47, kernel mid-edit 07:52) · `w3-codes/check-os-run.txt` (plugin + host `Finished` 07:55) ·
`w2-a/check-verb-plugin.log` (plugin **test** target `Finished` 07:27) · `w2-a/check-verb-puzzle-wasm.log` (07:55, aborted by peer `stdio-las` E0599) · `w2-c-build.txt` (wgpu test build 08:00) · `w2d-test-all.txt` (08:01, 9 FAILED + SIGABRT) · `w2d-test-wgpu.txt` (07:49, board2d 6/6) · `w2d-check-wasm2d.txt` (07:30) ·
`e2e/activate-react-6.log` (08:05 failure) · `e2e/probe-2026-09-30T16-49-01.md`, `…T16-51-11.md` (Run 2), `…T19-43-21.md` (Run 3 steps 1–3) · `serve-6012-supervised.txt` (vite pre-transform errors, live) · `coord/disk-guard.txt`.
Reports to update by the successors: `📓️w1-g-report.md` (follow-up 5), `📓️w2-a-report.md` (follow-up 3 + stale §7.8), `📓️w2-c-report.md` (follow-up 5), `📓️w2-d-report.md` (08:01 run, H3), `📓️w3-e2e-report.md` (Run 3 + wgpu).
