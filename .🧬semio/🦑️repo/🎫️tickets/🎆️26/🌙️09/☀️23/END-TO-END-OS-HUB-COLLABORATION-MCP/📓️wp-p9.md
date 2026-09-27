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
