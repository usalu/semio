# 📓️ S3-WIRES — reasoning/wires, design §20.15 conversion

Plugin root `W = ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires`, subset `A = W/🏅️standards/🔖️1/🪆️subsets/✳️any`. Crate
`semio-s-artifact-reasoning-wires` (`--manifest-path ✏️s/Cargo.toml`), hub `semio-hub-reasoning`. Session 3 (S3-WIRES, launched
10-03 12:03) never wrote a report or touched the tree.

## Session 4 — 2026-10-04

Executor S4-WIRES-MATH (coordinator `⚪487b04ad…`, fleet rules 1–38). Scratch `🗑️generated/s4-wires-math/`. Companion report for
mathematical: `📓️s3-math-report.md` § Session 4.

### Status log (newest first)

- 21:50 COMPOSITION GREEN reasoning (sent): hub `semio-hub-reasoning` + `semio-hub-mathematical` `--lib --target wasm32-wasip2` exit 0,
  0 errors; wires `--lib` 0 errors / 161 warnings (`check-wires-6`), wires + math `--lib --tests` 0 errors (`check-both-tests-1`).
  The Codex peer's test adaptation compiles against the composed API; my now-unused `context::{composed, board_node,
  board_counts}` helpers and an unused `EditorApp` import were deleted. stdio-semio went green after S4-PACKFIX (21:35).
- 21:2x CARGO OPEN resumed. `check-wires-2` SIGKILLed (exit 137, external); `check-wires-3` (21:14-21:16) exit 101 with 77 errors, ALL
  in dependency `semio-s-artifact-stdio-semio` (`PackError::Schema` removed by the pack-error API), routed to S4-PACKFIX. Source done
  meanwhile: wires PackError sites converted per `📓️s4-packfix-report.md` (canvas config, canvas transient, snapshot pack); F6
  `composed_child_history_law!` with an `addNode {"kind":"identity"}` seed; F21 `samples`/`cancelled` required (no `#[value(default)]`),
  `samples_or_last` deleted, the legacy decode law replaced by `wires_pointer_wire_requires_samples_and_cancelled`.
- 13:3x PARKED (coordinator usage limit). Exact state and next steps: §5. Wires lib last checked 12:23 (interrupted by rule 44 before
  the wires crate compiled; 0 errors in the deps reached). Every later wires edit (F9 code, F13 diff facet) is source-only, OWED.
- 13:00 F5 + F9 (core audit) GREEN: `cargo check -p semio-framework-tool-run -p semio-framework-plugin --lib --message-format=short`
  exit 0, 0 errors, plugin 256 pre-existing warnings, 4m05s (`🗑️generated/s4-wires-math/check-f5-1.txt`, rule 45 exception).
- 12:5x rule 44 (cargo freeze) obeyed: my gated `check-wires-1` was interrupted (cargo + orphan rustc killed).
- 04:13 child-target ToolRun (§1.1) source complete and GREEN: `cargo check -p semio-framework-tool-run -p semio-framework-plugin
  --lib` 0 errors (attempts 1–2 SIGKILLed/timed out under load ~100; `🗑️generated/s4-wires-math/check-plugin-3.txt`). Files:
  `🧰️framework/🔨️modules/⏯️tool-run/{🦀️.rs, 🧬️schema/🔣️.json, 🧪️tests/🔬️unit/🦀️.rs, 🧪️tests/🧩️conformance/🟦️.ts}`,
  `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` (`member?: string`, coordinator: confirm with framework generate),
  `OSM/🔌️plugin/⏯️tool-run/🦀️.rs` (region 🔖️Member + driver), `OSM/🔌️plugin/⏪️time-travel/🦀️.rs` (`render_children_or`),
  `PLG` (close step + 4 render seams), 15 `ToolRunDefinition` literals (`member: None`: layout-run, plugin tests ×2, remodel, wfc ×5,
  puzzle ×5, energy), puzzle 5d precompute 🖌️brush/🪣️fill (`children`/`member_ops` pass-through).
- 02:4x coordinator: generic child-target ToolRun is mine (design §1.1).
- 02:2x rule 34 repair-first: `git diff HEAD --stat` over `💡️reasoning` (108 files) and `➗️mathematical` (127 files) = earlier
  peer waves only (ValueError/`warning`/value-crate paths, Cargo deps 01:19); no unstaged change, no half-edit by S3-WIRES/S3-MATH
  (neither started in source). Nothing to repair.

### 1. Design

#### 1.1 Child-target ToolRun (coordinator decision 02:4x; owner S4-WIRES-MATH; consumers wires Reorganize, dag Reorganize)

One generic run mechanism, target = document | owned member; parent-target runs are unchanged.

1. **Schema-first declaration.** `ToolRunDefinition.member: Option<String>` (crate `semio-framework-tool-run`, Rust + JSON schema +
   TS twin): the composed-child SLOT whose owned member store a mutating run edits (`"content"`); absent = the document's own store.
   A read-only run never names a member (definition validation).
2. **Ledger.** A member run's `ToolRunEntry` resolves `(slot, child_id, dialect)` from the live children at start and holds a
   `ToolRunMemberRun`: provisional child ops as `OpBinary` bytes, the member store generation it is based on, typed owners
   (`base`/`overlay` `Arc<P>` + displaced aliases, created and folded through `store::MemberStoreVisitor`, the time-travel member
   pattern) and the composed read `ChildContentView` with the overlay leased in the member's place
   (`snapshot_read_erased_of` + `with_member_read`). The parent `provisional` stays empty.
3. **Compose on read (§20.15).** `ToolRunLedger::children_or(live)`; `TimeTravel::render_children_or(tool_runs, live)` mirrors
   `render_snapshot_or`, so every render seam shows the run's provisional child state; the parent snapshot is untouched.
4. **Jobs.** `ToolRunJobRequest` gains `children: ChildContentView` (the composed read the job runs on) and `member_ops: &[Vec<u8>]`
   (a member run's provisional child ops, for resume and revalidate). Ticks keep their wire (`append_ops` = child op bytes).
5. **Stepping, rebase, cancel.** Ticks fold O(k) onto the member overlay; retract and rebase (member store generation moved =
   `BaseChanged`, same policies) refold from the member head within the turn wall budget; abort/fault/close = zero trace (ops
   dropped, typed aliases retired against the member store, the children view through `child_content_retirements`).
6. **Finalize = one child edit.** A typed visitor builds `ChildEmit::of::<P, Mu>(slot, child_id, ops)` (labels from the child
   leaves) and the runtime publishes it as ONE composite group through `dispatch_emit_group(…, Some(tool_run_transaction(…)))`
   (§12: the run's `TransactionRef` stamped on the member edit); the history row keeps the tool label, its mutation rows are the
   child leaves, so time travel edits them on the member store.
7. **Laws** (plugin crate, `🧪️tests/🧪️tool-run-member`): stepped progress + cancel, finalize = one child row with the run's
   transaction, abort = zero trace, a published child mutation is editable in history.

#### 1.2 Wires §20.15

1. **The content child is the single truth for the board.** `s.stdio.semio@v1/graph` (`SemioGraphSnapshot`): board node =
   native `id`, `kind` (`nodeKind`), `label` (`text`), `position` (`x`,`y`), `width`/`height`, plus one keyed property per other
   board field (`shape`, `radius`, `root`, `handles`, …; `SemioValue`, keys ascending); board edge = native `id`, `source`,
   `target`, `kind` (`edgeKind`) plus keyed properties — the wires relationship of an edge is its `relationship` property (map
   `kind`, `sourceIdentityId`, `targetIdentityId`, `relationshipId`), captured from the parent identities when the edge is authored
   (self-describing, §20.9). The old `wires.node` JSON blob and the edge-label JSON are deleted.
2. **Parent.** `WiresSnapshot { wires_fixture: {schema, identities}, content, meta }`: identities stay parent state (no leaf edits
   them); the persisted `board` mirror and `relationships` leave the parent. `WiresMutation` is the uninhabited parent vocabulary
   (sequence template); the 12 parent leaves, their fixtures, the `📡️mutate-wires-1` case/oracle and the parent mutation codecs are
   deleted (rule 32).
3. **Readers compose on read.** `wires_content(snapshot, children)` (exact dialect, typed read), `wires_board(snapshot, content)` and
   `wires_composed_fixture(snapshot, content)` (relationships from edge properties, identities whose node is gone pruned) feed every
   render, panel, command and the Reorganize layout; `WiresWorkingScene`, `local_owner` and `materialize_wires_content` are deleted.
4. **Edits are child leaves through the tool machines.** addNode → graph `create-node`; addRelationship → `create-edge`;
   deleteSelection → `delete-edge` (selected edges) then `delete-node` (cascades incident edges); canvas drag → graph `drag-nodes`
   via `Emit::node_drag_child` (one child transaction row); Reorganize → member ToolRun (§1.1) of graph `move-node`s. Node
   field edits use GRAPHS' shared `set-node-property` / `resize-node` / `change-node-label` / `change-node-kind` (no wires-local
   equivalents).
5. **Genesis.** The empty board's child id is content-addressed (`content_id("wires-content", pack)`); the demo board's child
   id is the named `WIRES_DEMO_CONTENT_ID` (`metabolism-content`, so the committed parent asset needs no hash).
   `genesis_wires_child_pack` resolves the bundled contents (`wires_bundled_contents()`: empty graph, the demo's committed child
   asset `🖼️assets/🎬️demo/🕸️content/🗣️.dsl.semio`). A saved document reloads from its recursive archive (members), never from
   genesis.
6. **Known seam (S4-AGNOSTIC W-a).** Parent-only readers (topology inference, json/txt serializers) see no board until the
   `ArchiveChildren` reader seam lands; topology inference answers the empty topology meanwhile.

### 2. Changes

**Child-target ToolRun (§1.1, shared crates)** — `🧰️framework/🔨️modules/⏯️tool-run/{🦀️.rs (ToolRunDefinition.member,
EmptyMember/ReadOnlyMember, TOOL_RUN_MEMBER_OPS_MAX = 4096), 🟦️.ts, 🧬️schema/🔣️.json (member, limits.memberOpsMax),
🧫️fixtures/⚖️lifecycle-law.json, 🧪️tests/🔬️unit/🦀️.rs, 🧪️tests/🧩️conformance/🟦️.ts}`; `OSM/🔌️plugin/⏯️tool-run/🦀️.rs` (region
🔖️Member, driver: member target/refresh/fold/stepped emit decode/publish/retire, member cap on tick append, finalize restart on a member
move while Publishing, F9 `retire_tool_run_views` hand-back); `OSM/🔌️plugin/⏪️time-travel/🦀️.rs` (`render_children_or`); `PLG`
(`ChildEmit::open`/`push`, `of` built on them; close step + 4 render seams); 15 `ToolRunDefinition` literals (`member: None`); puzzle 5d
precompute pass-through. `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` hand edit `member?: string` = audit F17
(coordinator: regenerate).

**F4 law (source, run OWED)** — new `OSM/🔌️plugin/🧪️tests/🧪️tool-run-member/🦀️.rs` (mounted as `member` child of `🔬️tool-run`): own
roster `ToolRunMembers` (`s.test.child` TestSnapshot/TestMutation), `ToyMemberJob`; five laws: compose-on-read with both stores
untouched (fresh-fold cross-check), pause+step one unit, abort zero trace (both stores, command log, history), finalize = ONE member edit
with the run's `TransactionRef` + group id, fresh fold, one editable history row labelled by the tool in en/de, one member undo removes
it; member ceiling + cap step. `🔬️tool-run/🦀️.rs`: member tool in `toy_manifest`, member branch in `build_tool_run_job`, `mod member`.
`🧫️fixtures/⏯️tool-run/🔣️.json`: `member` section. D6 law stays with S4-RUNTIME.

**Wires §20.15** (`W`, `A` as above) — parent vocabulary uninhabited (`WiresMutation {}`); content child = graph; drag =
`Emit::node_drag_child` (`drag-nodes`); Reorganize = member ToolRun; readers compose on read (`wires_composed*`); demo asset split
(parent identities + `🕸️content` graph child); notices `wires_fault_notices()` (en/de). Deleted (rule 32, zero references): 12 parent
leaf dirs, `A/🧬️schema/🧬️mutations/📖️.grammar.semio`, `🧪️tests/🧪️node-drag-history`, `A/🧫️fixtures/🧬️mutations` (60),
`A/🚪️io/🧬️mutations` (14), `A/🧪️tests/📡️mutate-wires-1`, `W/🧫️fixtures/🧫️child-owner-isolation`, and (audit-tools F13)
`A/🚪️io/🔺️diff/**` (15 files: text/binary codec + tests). `A/🧬️schema/🔺️diff/` is REDUCED, not deleted: `WiresDiff {}` (identity
apply) + empty json/ts/proto/graphql twins, because `semio_framework_schema_registry::ArtifactSchemaDescriptor` requires four
facets for every artifact (removing it = `diff: Option<FacetLeaves>` across every plugin descriptor → coordinator decision).
Audit-tools F9: `A/✏️editor/🦀️.rs:624` → `FaultCode::new("app.command.tool-mismatch")`.

### 3. Verification

| Command | Result |
| --- | --- |
| `cargo check -p semio-framework-tool-run -p semio-framework-plugin --lib` (04:13, after §1.1) | exit 0, 0 errors |
| `cargo check -p semio-framework-tool-run -p semio-framework-plugin --lib` (13:00, after F5/F9) | exit 0, 0 errors, 256 warnings |
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-reasoning-wires --lib` (12:23) | interrupted by rule 44, OWED |
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-reasoning-wires --lib` (21:35-21:37, `check-wires-6`) | exit 0, 0 errors, 161 warnings |
| same `-p …-reasoning-wires -p …-mathematical-equation --lib --tests --keep-going` (21:50, `check-both-tests-1`) | exit 0, 0 errors (wires lib-test 260 warnings) |
| `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-reasoning -p semio-hub-mathematical --lib --target wasm32-wasip2` (21:48-21:49) | exit 0, 0 errors |
| `cargo test --lib` wires (incl. F6 `child_history_edits_end_to_end`, F21 pointer-wire law) | OWED (rule 43, TESTS RESUMED) |
| F4 `cargo test -p semio-framework-plugin --lib tool_run_tests::member` | OWED (rule 43) |

### 4. Coordinator actions

- After CARGO OPEN: wires lib check first (rule 39 spirit), then `--tests`, then hub wasip2; `test inventory --artifact
  s.reasoning.wires`; `parity exhaustive --case mutate-wires-1` is DELETED with the parent vocabulary (drop it from plans).
- `framework generate` for the tool-run manifest type (F17) and `schema generate` (the wires diff/mutation catalogs changed:
  `🔣️schema-catalog.json` still lists the old `WiresDiff` fields and wires leaves); taxonomy regeneration for the new
  `🖼️assets/🎬️demo/🕸️content`, `🧪️tests/🧪️tool-run-member` dirs; describe reasoning; re-activation.
- Decide F13 framework scope (optional diff facet) for wires/dag/sequence/flow/imperative.

### 5. State after the CARGO OPEN wave (21:50)

All items of the parked list below are done except the OWED test runs: wires lib/tests/hub green, F6 and F21 in, wires tests compile
(Codex peer adapted them). Remaining: `cargo test --lib` for wires (TESTS RESUMED), F4 member laws run, F13 framework decision
(S4-INFRA, design §21.8; the empty `WiresDiff` stays meanwhile).

### 5a. Parked state and next steps (13:3x, historical)

Done and consistent on disk: §1.1 + F5 + F9 (green), F4 law source, wires production conversion, F9 code, F13 (reduced as above).
Not started / in progress, in order:
1. Wires lib check (OWED) and fix any error.
2. Wires tests adaptation (Codex peer edited them at 12:01-12:55; last wires test edit 12:55 → quiet window reached 13:25): editor unit
   tests, reorganize tests (rewrite as member-run laws), transient tests, pointer-down, set-active-example, delete-selection,
   declared-verbs, commands tests — composed reads (`wires_composed`, `context::board`).
3. Audit-tools F6: `composed_child_history_law!("wires", WiresEditor, manifest, [("addNode", {...})])` next to the reload law.
4. Audit-tools F21 (NOT started, no file touched): drop `#[value(default)]` on `CanvasPointerMove.samples` /
   `CanvasPointerUp.cancelled` (both hosts always send them: wgpu `sampled()`/`release()`, `Canvas2dHost` `samples`/`cancelled`),
   remove `samples_or_last` and the `[x, y]` fallback docs, delete the legacy laws at
   `🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:417-436`.
