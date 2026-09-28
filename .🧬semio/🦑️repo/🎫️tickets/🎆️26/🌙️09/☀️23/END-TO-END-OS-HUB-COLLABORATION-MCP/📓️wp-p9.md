# WP-P9 — SDK Agent-Lane Preview For Plugin Tool-Command Jobs, Jack Node Targets Over MCP (Session 14)

Slice P9 · session 14 · 2026-09-27. Successor of P8 (`📓️wp-p8.md`, agent lane ≡ shell lane) for outcome 4 (the semio MCP
agent mutates every kind; G11 battery b3-2/b3-3: 71/71 created, 41/71 mutated). Owns the SDK/plugin-side causes no other
slice owns: `interactive-job.preview-unsupported` (puzzle 2d/3d/5d, writer) and trinity jack's `patchNodes` selection
precondition. G12 owns the MCP host + battery, S19 norm, LB2 stdio snapshot schema, SH2 space-home. Ports: hubs 8180–8189,
serves 6680–6689. Scripts + patches `wp-p9/`; captures `wp-p9/generated/` (expendable); binaries
`CARGO_TARGET_DIR=wp-p9/target`; overlay + durable logs `.🧬semio/🌐hub/s14-p9-*`.

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | Agent-lane preview for plugin-owned tool-command jobs (puzzle 2d/3d/5d, writer): design + SDK implementation, laws + oracle | **prepared** (`wp-p9/patches/p9-agent-lane.py`, 44 hunks / 24 files, dry run clean on the live tree 19:4x); AJV twin 9/9 + red-checked; Rust laws written, overlay build pending |
| 2 | Trinity jack (and rewriting) `patchNodes`: node targets declared as a typed entity-id list, typed precondition fault, human rail unchanged; law | **prepared** in the same set (entity-id arg + build check + `app.command.targets-required` + laws); overlay build pending |
| 3 | Guest-linked → prepared idempotent patches (dry-run clean), overlay proofs during the freeze; land compile-atomic in window 3 (native + wasm32), landing rows; G12 re-runs the coverage battery | pending |

### Session 14b

Successor P9 agent, 2026-09-28 from 12:1x (predecessor cut ~20:45 by the usage limit; its overlay run `overlay-test-1` was
still queued behind 10 holds and never started). Guest freeze ON since 12:02 (chain pid 45604).

| # | Item | Status |
|---|---|---|
| 0 | Tree carries no half-applied P9 edit | **verified 12:1x**: 7/7 created files absent, 37/37 replace anchors match 1× (old text present, new absent), `drive_agent_lane_preview`/`preview_typed_command_job`/`probe_agent_lane`/`fn entity_ids`/`reducer_fault_of_detail` 0× in `🧰️framework` + `✏️s` (`targets-required`/`preview-budget` only in G12's host mapping `🌉️mcp/🔀️dispatch/🦀️.rs`) |
| 1 | `p9-agent-lane.py` re-diffed on the live tree, builder law, overlay law run | **overlay-proven 14:21, ready for window 3**: re-based + dry-run clean (44 hunks / 24 files); builder law = hunk 20 (`an_entity_id_argument_names_a_granularity_of_a_declared_interaction`); AJV twin 9/9; overlay `cargo test` (7 crates) **all 14 P9 laws pass**; SDK lib 295/297 with 2 pre-existing peer reds (not P9) |
| 2 | Delete trinity jack's LSP shim, migrate callers | **prepared, ON HOLD** (coordinator 12:4x: deleting the module's bundle `AGENTS.md` needs the human's explicit OK) — `wp-p9/patches/p9-jack-lsp-removal.py`, dry-run clean (19 hunks / 11 files + 1 dir); kept OUT of the window-3 agent-lane landing and out of the overlay run |
| 3 | S20 SDK export/import overlap | **agreed** (relay 12:2x): no span overlap; P9 lands first, S20 re-runs its dry run |
| 4 | G12 re-runs the MCP coverage battery after window 3 + publish | pending (window 3) |

Log 14b:

- 12:1x dry run of the 19:58 set on the live tree: **2 problems**, both in `🔌️plugin/🦀️.rs` (hunks 12 + 14, the
  `preview_retained_command` head/tail): the overnight peer (04:45) ran rustfmt over the file (2 428 diff lines vs my base,
  plus a new `⏳️operation-progress` module) and re-wrapped `let operation_id = …` and the `spec.payload.into_inner` chain.
  `🛂️manifest/🦀️.rs` drifted too (04:14) but every anchor still held. No P9 text in the tree (item 0).
- 12:2x `wp-p9/p9-rebase.py` (new): base := live file, stage := live + P9 hunks, the two wrapped anchors re-anchored on the
  live span with asserted equivalence (only the wrap differs). Stage backup `.🧬semio/🌐hub/s14-p9-stage-backup-0928`.
  `p9-regen.sh` → **44 hunks / 24 files, dry run clean on the live tree**.
- 12:2x rustfmt (repo `rustfmt.toml`, `skip_children`) over every P9-added line: 4 lines were not rustfmt-stable (`lost_step`,
  `lease`, `dispatch` in the SDK, `named` in the jack law) → stage re-wrapped as rustfmt emits them; re-check: **0 unstable
  added lines**; regenerated, dry run clean again.
- 12:2x overlay refresh (`p9-overlay.py`: 78 464 listed, 2 144 re-cloned, 66 generated) → patch applied **into the overlay only**
  (24 files; every replace-file byte-equal to the stage; tree untouched: `drive_agent_lane_preview` 0× in the tree, 3× overlay).
- 12:3x AJV twin in the overlay: `bun 🧪️tests/🤖️agent-lane-preview/🟦️.ts` → `agent-lane-preview-verdict-oracle cases=9`.
- 12:32 overlay law run QUEUED (`p9-overlay-cargo.sh overlay-test-2`, pid 68270, capture
  `.🧬semio/🌐hub/s14-p9-logs/overlay-test-2.txt`): the 7 crates as before, filters `agent_lane entity_id_argument
  reducer_fault_detail patch_nodes an_agent_names app_builder_tests plugin_builder_contract_tests` (the whole builder module +
  the 196-test plugin contract module, since the definition build gained the entity-kind check and the preview path changed).
  Overlay lane: sh2 holding, 6 more ahead (st2 av2 s19 lb2 t14 en2).
- 12:2x S20 relay: S20's window-3 SDK spans = framework action chain `.chain(document_transfer_action_definitions())`
  (~l.5953) + new `🔖️DocumentTransfer` region after `pub use operation_progress::{…}` (~l.7236); mine = definition build
  (~l.5582), artifact_app_laws probe, `drive_agent_lane_preview`, `preview_addressed_action`/`preview_typed_command_job`,
  manifest `entity_kinds`/`entity_ids`, action-bus/retained-command deletions → **disjoint**; P9 lands first, S20 re-runs
  its dry run. Removed APIs (`ToolPayload::into_inner`, `preview_emit`, `require_proof_operation_authority`,
  `preview_retained_command`) have no other caller in the live tree (only G12's `AGENT_LANE_PREVIEW_UNSUPPORTED_FAULT_CODE`
  host constant, which G12 deletes when this lands).
- 12:3x item 2 analysis: the crate `semio-s-plugin-trinity-jack-lsp` only re-exports `dsl_lsp::lsp::{handle_json_rpc,
  LanguageSession}`; the Jack language runs in the kernel (`dsl_lsp` + Jack's `LanguageSpec`, writer drives it in-process via
  `dsl::lsp::LanguageSession::open`). Its one code caller, the TS worker `@semio-tech/trinity-jack-lsp-worker`, imports a
  `JackLspSession` the shim no longer exports (stale gitignored `pkg/` of 2026-08-06) and is imported by nothing (git grep
  over every tracked + untracked file). So "migrate callers" = remove the dead module and its registrations:
  Cargo.toml member + Cargo.lock block, root package.json workspaces + bun.lock (3 entries), `.vscode/launch.json` (5
  pickString option lists; configurations 1247 → 1247), `🔒️dependencies.json` (2 users rows), vite wasm stub
  `JackLspSession`, fixtures rust-warnings (native requiredPackages), nx-contract (wasm row), vitest-configuration-ownership
  (owner row; its test 46 → 45). Left alone: the purity HISTORY fixture (sha-pinned in taxonomy, records a past census),
  taxonomy `🧠️lsp` (a member name the kernel dsl also uses), `.cursor/plans`, the root census note in `📜️script.ts`.
- 12:3x `wp-p9/patches/p9-jack-lsp-removal.py` (engine gained `delete_tree`, counted outside `pkg/` + `node_modules/`):
  dry run clean on the live tree. Overlay trial: JSON of every edited file parses (launch.json as JSONC), vitest fixture 45
  owners, `cargo metadata --locked --offline` **OK** (the hand-edited Cargo.lock is exactly cargo's). Relayed to R10
  (generator equality + dependency gate after landing).
- 12:4x **HOLD** (coordinator): AGENTS.md forbids editing AGENTS.md files; deleting the module's bundle `AGENTS.md` needs the
  human's explicit OK (requested by the coordinator). The removal was reverted in the overlay before `overlay-test-2` started
  (23 files re-cloned, byte-equal to the tree), so the overlay run proves the agent-lane set alone, as it lands.
- 12:5x S20 correction relayed: its ids + `document_transfer_action_definitions()` go in `🛂️manifest/🦀️.rs` after
  `start_introduction_action_definition()` (~l.1609) — disjoint from mine (~l.209–226, ~l.405). Answered: the declared-verb
  probe already skips every `is_framework_reserved_action_id` id (🔌️plugin ~l.22990) → S20 adds its two ids there; keep off
  my hunk-5 anchor (`for (action, windows) in &order {`).
- 13:38 `overlay-test-2` got the lane and **died after 65 s** (EXIT 101, capture renamed
  `overlay-test-2-missing-tokens.txt`): `semio-framework-ui-styling` `mod generated` → the gitignored generated
  `🎨️styling/🔤️tokens/🦀️.rs` was not in the overlay (p9-overlay.py clones tracked files + `🤖️generated/` dirs only).
  New `wp-p9/p9-overlay-ignored.py` clones every live-tree file the overlay lacks (pruning target/node_modules/dist/pkg/.git/
  `.🧬semio`/`🗑️generated`/`♻️mit-bestand`): 3 255 files. Overlay refreshed again (348 re-cloned; the SDK file drifted at
  12:49 — T14's rule-22 test doors `test_from_media_export` / `test_segmented_closure_contains`) → `p9-rebase.py` +
  regen → still **44 hunks / 24 files, dry run clean**; patch re-applied into the overlay (plugin file == stage).
- 13:45 `overlay-test-3` queued with my original FIFO stamp 20260928123222 (coordinator told), 2nd behind t14; same crates
  and filters as test-2.
- 13:48–14:09 `overlay-test-3` ran 21 min, compiled every crate and test binary except writer's: **peer test red**
  `semio-s-artifact-writer-writer` (lib test) 2× E0609 `DocxSnapshot.document` (docx import/export tests vs the peer's OPC
  `xml_parts` rewrite; capture `overlay-test-3-writer-docx-red.txt`); no test ran (cargo test aborts the build). A peer fixed
  both sites in the tree at 14:06:42 (`project_document()`). Overlay refreshed (61 files), patch re-applied (== stage), dry
  run on the tree clean; `overlay-test-4` queued 14:1x with the original stamp (coordinator told).
- 14:17–14:21 **`overlay-test-4` (capture `.🧬semio/🌐hub/s14-p9-logs/overlay-test-4.txt`, EXIT 101 only from 2 SDK peer
  reds)**. P9 laws, all `ok`: SDK `agent_lane_preview_verdicts_match_the_language_agnostic_fixture`,
  `a_reducer_fault_detail_reads_back_as_the_reducers_own_code_and_message`,
  `an_entity_id_argument_names_a_granularity_of_a_declared_interaction`; jack `an_agent_names_the_nodes_patch_nodes_renames_and_is_refused_by_name_without_them`
  + 3 patch_nodes laws; rewriting 2 patch_nodes laws; `the_agent_lane_runs_the_puzzle{2d,3d,5d}_editors_own_tool_command_job`,
  `the_agent_lane_runs_the_writer_editors_own_tool_command_job` (every battery verb reachable on the agent lane, the write
  verb previews its document write, 0 agent-lane divergences). SDK lib (filters incl. the whole `app_builder_tests` +
  `plugin_builder_contract_tests` modules): **295 passed, 2 failed — both pre-existing peer reds, unrelated to P9**:
  `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations` (left `{cancelTypedOperation,
  setActiveUtility}` vs right `{setActiveUtility}`: the overnight peer's `operation_progress::cancellation_action_definition()`
  declares a migrated verb with no activated factory) and `a_scene_that_declares_no_lanes_still_publishes_one_childless_surface`
  (`lanes.is_empty()` for a `TableScene`). Reported to the coordinator (T14 owns SDK lib reds, rule 22).
- 14:2x rule 23: overlay build-dir `s14-p9-build` + `s14-p9-target` deleted (disk 82 → 94 GiB free). Overlay source clone kept.
- 14:3x window-3 runbook written: `wp-p9/p9-land.sh write` (dry run → byte backup of the 24 files in
  `.🧬semio/🌐hub/s14-p9-land-backup` → write) / `p9-land.sh revert` (restore, created files + empty dirs removed). Then, compile-
  atomic before any hand-off: (a) native `zsh wp-p9/p9-native-cargo.sh land-native-1 test <same 7 crates + features as
  overlay-test-4> --lib --no-fail-fast -- <same filters>` (expect the 2 known peer reds only, unless T14 fixed them);
  (b) wasm32 `zsh 📜️fleet-mutex.sh wasm p9 -- zsh wp-w4/w4-wasm-check.sh .🧬semio/🌐hub/s14-p9-logs/land-wasm-1.txt
  semio-framework-plugin semio-s-artifact-trinity-jack semio-s-artifact-trinity-rewriting semio-s-artifact-puzzle-2d
  semio-s-artifact-puzzle-3d semio-s-artifact-puzzle-5d semio-s-artifact-writer-writer`; (c) AJV twin `bun
  🔌️plugin/🧪️tests/🤖️agent-lane-preview/🟦️.ts`; red → `p9-land.sh revert`. Green → landing row, tell S20 (re-run its dry
  run) and G12 (delete `AGENT_LANE_PREVIEW_UNSUPPORTED_FAULT_CODE` arm); after the next publish ask G12 (via main) to re-run
  the coverage battery and record mutated counts per kind here (item 4).
- 14:3x waiting for "WINDOW 3 OPEN" (chain still in rebuild-all components at 14:22). Ended the turn instead of polling;
  the coordinator resumes P9 at window 3.

### Log

- 18:5x started; read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" line yet), audit row 4.8 +
  §8 items 6–7, `📓️wp-g11.md` (routing table, battery b3-2/b3-3 rows), `📓️wp-p8.md`. Coverage rows read from
  `.🧬semio/🌐hub/s13-g11-logs/battery-b3-3/coverage/coverage-rows.jsonl`: puzzle 2d `puzzle.2d.fixture.tool-command.v1`,
  3d `puzzle.3d.fixture.tool-command.v1`, 5d `puzzle.5d.tool-command.v1`, writer `writer.writer.tool-command.v1` — every
  attempted verb refused `preview-unsupported`; jack `patchNodes` refused `patchNodes needs node ids or a node selection in
  the graph`, `loadExampleQuery` refused `targeted window transient capture requires an attached window instance`.
- 19:0x design (see § Design): the preview runs the app's OWN job through the SAME admission + factory entry the shell
  uses, driven on the calling turn by a caller-driven `BatchJobSession` (budgets + keyed cancellation), instead of
  downcasting to the SDK's retained payload. One path for every job shape; `preview_emit`/`into_inner`/
  `require_proof_operation_authority`/`preview-unsupported` removed as dead.
- 19:1x–19:4x staged edits (gitignored stage `.🧬semio/🌐hub/s14-p9-stage/{base,stage}`, scripts `wp-p9/p9-stage.py`,
  `wp-p9/p9-hunks.py` base→stage anchored hunks, engine `wp-p9/patches/p9_patch.py`). Patch set
  `wp-p9/patches/p9-agent-lane.py` generated: 44 hunks / 24 files; `--dry-run` on the live tree **clean** 19:4x (a peer's
  new `reduce_editor_command` in `🔌️plugin/🦀️.rs` since staging does not touch any anchor).
- 19:3x AJV twin (`bun 🧪️tests/🤖️agent-lane-preview/🟦️.ts` from the stage) → `agent-lane-preview-verdict-oracle cases=9`;
  red-check: mutating a steps count, the reducer-code case and the framed-code case each fails with its case name.
- 19:43 overlay clone started (`wp-p9/p9-overlay.py` → `.🧬semio/🌐hub/s14-p9-overlay`, APFS clonefile; load 118–134).
- 19:55 overlay ready (11.5 min under load ~130); patch set applied INTO THE OVERLAY ONLY (`P9_ROOT=<overlay>
  p9-agent-lane.py --write`: 24 files; tree untouched — `drive_agent_lane_preview` 3× in the overlay SDK, 0× in the tree).
- 19:59–20:05 private build-dir seeded by APFS clone of `build-fleet-b/debug` (`cp -cR`, pid 87739, log
  `.🧬semio/🌐hub/s14-p9-logs/seed-build-dir.txt`, EXIT 0) → registry deps reuse; the shared build-dir is never used by the overlay.
- 20:06 overlay law run QUEUED in the overlay lane (`wp-p9/p9-overlay-cargo.sh overlay-test-1 test -p semio-framework-plugin
  -p …-trinity-jack -p …-trinity-rewriting -p …-puzzle-2d/3d/5d -p …-writer-writer --features … --lib --no-fail-fast --
  agent_lane entity_id_argument reducer_fault_detail patch_nodes an_agent_names`, pid 97327, capture
  `.🧬semio/🌐hub/s14-p9-logs/overlay-test-1.txt`). **Queue: 10 overlay holds ahead of mine** (t14 ×2, lb2 ×3, h14, sh2,
  s19, en2, av2; t14 holding since 19:26 and still compiling registry crates) → hours of wait.
- 20:0x G12 told (SendMessage): `preview-unsupported` disappears with this set; new `interactive-job.preview-budget`;
  `app.command.targets-required`; jack `nodeIds` entityId schema. G12 answered: host mapping done (budget →
  PLUGIN_UNAVAILABLE + en/de remedy, targets-required → INPUT_INVALID, law extended); deletes the preview-unsupported
  arm when my set lands; battery will fill entityId arrays from `artifact_snapshot` and re-run after the landing.

### Design

- **Where the refusal came from.** `preview_retained_command` built the verb's job (`A::build_tool_job`) and downcast the
  payload to `ArtifactRetainedCommandPayload<A>`; puzzle (`RetainedPuzzleCommandJob`), writer (`WriterCommandToolJob`)
  and every other plugin-owned job answered `interactive-job.preview-unsupported`.
- **Agent-lane preview (SDK).** `preview_addressed_action` now mirrors `dispatch_action` up to the applying tail:
  `command_from_action` → `admit_command_wire` → `require_complete_tool_operation_pipeline` → `preview_typed_command_job`
  (same roots capture, same `build_tool_job`, same `dispatch_wire_retained_with_spec`) → `drive_agent_lane_preview`
  (caller-driven `BatchJobSession`: per-step fuel/deadline from the job's contract, 2 s wall + 65 536-turn budget,
  keyed cancellation lease; every progress/checkpoint payload released; closed terminal-empty or handed to the
  retirement pump) → the completion value the shell would publish. Nothing is published; the admission is consumed
  and retired in the same call. Download completions and the existing uncarried lanes → `agent-lane-uncarried`;
  budget → `preview-budget`; job fault → the shell's fault-page code, or the reducer's own code for an SDK retained
  refusal. `jobSteps` joins the preview output. Schema-first: the verdict is a language-agnostic fixture read by the
  Rust law and the AJV twin; the op counts the MCP `PreparedActionReport` shows are unchanged; destructive verbs still
  go through the MCP approval policy (prepare never commits).
- **Jack/rewriting targets.** `nodeIds` = `ActionArgDef::entity_ids(id, label, interaction, granularity)` → MCP input
  schema `{type: array, items: {type: string, x-semio-format: entityId, x-semio-entity-kind: "ast/node"}}`; the
  definition build refuses an entity kind that names no declared interaction granularity; empty targets →
  `app.command.targets-required` (agent sees it by name through the preview's reducer-code recovery). Rail unchanged
  (Array → text control; `ids()` still parses text; empty = selection).
