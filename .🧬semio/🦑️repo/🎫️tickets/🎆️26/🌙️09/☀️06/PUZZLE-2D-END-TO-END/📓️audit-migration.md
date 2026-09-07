# Read-only migration audit — puzzle2d editor (2026-09-06)

Scope: current source only (not the wave reports), read after B1/B2/B3 all landed and native
`cargo check` reportedly reached 0 errors (check #3). No file was edited to produce this audit; no
cargo/git command was run. All paths relative to `/Users/ueli/Documents/semio`. Editor file referred
to below as `EDITOR` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
(4,618 lines at audit time — line numbers will drift under concurrent edits, re-grep before acting).

---

## 1. Lane table — every `PUZZLE2D_RETAINED_TOOL_IDS` id, cross-checked source ↔ fixture

`PUZZLE2D_RETAINED_TOOL_IDS` (`EDITOR:1054-1092`, 39 ids) is fully covered by `build_tool_job`
(`EDITOR:3663-3681`): 5 explicit named arms (`setActiveExample`, `forceLayout`, `reorganize`,
`addNode`, `applyBoardEvents`) + `PUZZLE2D_GENERIC_TOOL_IDS` guard (23 ids, `EDITOR:1101-1123`) +
`PUZZLE2D_HOST_ONLY_TOOL_IDS` guard (2 ids, `EDITOR:1130`) + explicit `redrawHandles` arm + a
`set_fill_count::is_fill_session_action` guard (8 ids, `…set-fill-count/🦀️.rs:24`) = 5+23+2+1+8 = 39,
no gaps, `_ => Err("puzzle2d-command-tool-unmapped")` only below all of them.

`PUBLICATION_CONTRACTS` (`EDITOR:1193-1232`, retained factory) has exactly 39 entries, one per
retained id, plus a **separate** 40th contract (`EDITOR:2934`) on `Puzzle2dImportJobFactory` for the
reserved `import-media` route (lane `Artifact`). Fixture
`✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json`, `Puzzle2dPlayApp` row: 4 groups —
`HostOnly` (2), `Artifact` (5, includes `import-media`), `Config` (22), `Artifact+Config` (11) — sums
to 40, and every route name in every group matches the source contract byte-for-byte (diffed by hand,
see below). No emitted-but-undeclared lane and no declared-but-never-emitted lane was found:

| Lane group | Source (`PUBLICATION_CONTRACTS`) | Fixture group | Match |
|---|---|---|---|
| `HostOnly` | `lodScaleJson`, `selectSameKind` | same 2 | ✅ |
| `Artifact` | `addNode`, `forceLayout`, `redrawHandles`, `reorganize` + `import-media` (separate contract) | same 5 | ✅ |
| `Config` | 22 ids incl. `setLocale`/`setTerminology`/`brushFillSessionAdopt/Cancel/Discard/Clear` (tightened per status.md 06:30) | same 22 | ✅ |
| `Artifact, Config` | `applyBoardEvents`, `brushCommitSlot`, `brushFillSessionBegin/Retry/Step`, `deleteSelection`, `duplicateSelection`, `patchInspectorNodes`, `setActiveExample`, `setFillCount`, `setSelectionFlag` | same 11 | ✅ |

`brushCommitSlot` is declared `Artifact, Config` and genuinely can produce both: it calls
`ctx.host.borrow_mut().brush_commit_slot()` (`…✅️commit-slot/🦀️.rs`), which unconditionally calls the
board host's private `brush_commit_preview()` (`🧰️framework/…/♾️infinite/🎲️board/…/🦀️.rs:5379`, not
gated by `brush_alt_pressed` — that gate belongs only to the separate, never-called
`brush_finish_slot`, `:5396`), so a `brushPlace` host event is genuinely reachable and reaches
`apply_host_events` → an `Artifact` mutation; a real `Config` snapshot mutation is possible too
because `sync_host_runtime_state` on the freshly rebuilt host always starts with
`brush_candidates_emit_key: None`, so `brush_sync_preview_events` emits a `brushCandidates` clearing
event once. Lane declaration is honest.

`import-media` (reserved route, `EDITOR:2874-2934`, `Puzzle2dImportJobFactory`): declared `[Artifact]`
only, emits `connect-kind-compatibility` / `replace-kind-catalogs` mutations
(`ArtifactToolCompletion::complete(Ok(Emit { artifact_mutations, .. }))`) and no config mutation —
consistent. It correctly carries **no** `.action_interactive_job(...)` entry (39, not 40, at
`EDITOR:3929-3967`) — reserved routes are declared through `register_tool_job_factories` +
`build_reserved_tool_job`, not the action catalog, matching puzzle5d's pattern.

**No lane defect found.** (The one previously-flagged issue — `brushFillSessionAdopt/Cancel/Discard/Clear`
over-declaring `Artifact` when they only ever emit `Config` — was already fixed in both
`PUBLICATION_CONTRACTS` and the fixture per status.md's 06:30 entry; confirmed still fixed in current
source.)

---

## 2. Silent no-ops — generic reduce traced arm-by-arm

`puzzle2d_generic_reduce` (`EDITOR:1734-1746`) and `puzzle2d_board_events_reduce` (`EDITOR:1688-1706`)
both call the single `puzzle2d_dispatch_emit` (`EDITOR:1619-1682`), whose `match action { … }` was
enumerated against all 23 `PUZZLE2D_GENERIC_TOOL_IDS` plus the 5 non-generic explicit ids. **Every one
of the 39 retained ids has an explicit match arm that calls a real `🎮️commands/*` function**
(`EDITOR:1638-1666`); the trailing `_ => {}` (`EDITOR:1666`) is unreachable for any id in
`PUZZLE2D_RETAINED_TOOL_IDS` — it exists only because `Puzzle2dCommand::action_id()` is a fixed enum
covering ids outside this artifact's retained set (view-only/framework actions dispatched through the
same `handle()`/pipeline, e.g. `setActiveUtility`, which does not go through this match at all — it
has its own `SET_ACTIVE_UTILITY_ACTION_ID` arm at `EDITOR:1649`). **No silent no-op found** in the
generic path.

Specifically checked, all real:
- `setLocale` / `setTerminology` (`…🗣️set-locale/🦀️.rs`, `…📖️set-terminology/🦀️.rs`): write
  `ctx.scene.runtime.locale` / `.terminology`; picked up by `puzzle2d_dispatch_emit`'s
  `if &scene.runtime != config` config-mutation guard (`EDITOR:1679`) — real, non-cosmetic.
- `reorganize`: routed in `build_tool_job` to `Puzzle2dForceLayoutWork::new("reorganize")`
  (`EDITOR:3673`), the *same* work as `forceLayout` — not a no-op, not a stub.
- `brushCommitSlot` / `brushOpenSlot` / `brushCancelSlot` (`…✅️commit-slot`, `…🔓️open-slot`,
  `…🚫️cancel-slot`): each calls a real `BoardHost` method (`brush_commit_slot`/`brush_open_slot`/
  `brush_cancel_slot`, `🧰️framework/…/♾️infinite/🎲️board/…/🦀️.rs:7473-7492`). `brushOpenSlot` no-ops
  only when the caller passes an unknown `handleId` — a correct guard, not a structural dead end.
- `ctx.operation` (the field `puzzle2d_dispatch_emit` threads through as `None` for every retained
  path): grepped every file under `…/✏️editor/🎮️commands/` for `ctx.operation` — **zero matches**, so
  no command handler ever depends on it being `Some`; the "retained work never sees an `ArtifactView`"
  limitation the fill family had to design around does not silently break any of the 39 ids.

**HostOnly ids are honest no-ops by declaration, not by accident**: `lodScaleJson` / `selectSameKind`
route to `crate::retained_command::NoopPuzzleCommandWork`, whose `extent()` is unconditionally
`Some(1)` (`🎮️commands/🧵️retained/🦀️.rs:115-117`) and whose `step()` completes once with
`Emit::default()` (never dead-by-extent). This mirrors what their legacy handlers already did (both
were empty by construction; not a regression).

**Cosmetic dead code, not a no-op bug**: `puzzle2d_dispatch_emit`'s match still carries
`"redrawHandles" => redraw_handles::redraw_handles(ctx)` (`EDITOR:1641`) — the old unbounded
whole-fixture legacy path. It is unreachable from any retained job (`redrawHandles` has its own
`Puzzle2dRedrawHandlesWork` arm in `build_tool_job`, `EDITOR:3675`, and the id is `Migrated` so
`handle()` should never receive it in production dispatch). It **is** reachable if anything calls
`ArtifactEditor::handle` directly with `action_id() == "redrawHandles"` (e.g. a test helper bypassing
classification-based routing) — worth deleting per wave B2's own §6.4 note, still present unfixed.

---

## 3. Extent budgets — recomputed independently from source, not copied from the wave reports

Recomputed by hand from the current formulas and cross-checked against B2's report numbers (all
matched exactly, giving confidence the formulas are stable across the reports → current-source gap):

| Work | Formula (current source) | Nakagin (N=180,E=179,H=358) | Redraw ceiling (N=4096,E=4096,H=8192) | Verdict |
|---|---|---|---|---|
| `Puzzle2dForceLayoutWork` (`EDITOR:2404-2422`) | `ceil((N+H+E)/256) + 3·ceil(N/512) + 8 + iter·(ceil(pairs/8192)+ceil(E/8192)+2·ceil(N/512)+4)`, `iter=clamp(2_000_000/(pairs+E),24,420)` | pairs=16,110 → iter=122 → **1,112** (hand-recomputed, matches B2 exactly) | refused (N=4096 > `PUZZLE2D_FORCE_MAX_NODES=512`) | safe; `MAX_HANDLES=4_096`/`MAX_EDGES=4_096` are preflight-checked (`EDITOR:2407`), so an over-wide document is refused, not truncated |
| `Puzzle2dRedrawHandlesWork` (`EDITOR:2776-2791`) | `ceil((N+H)/256) + ceil(E/256) + ceil(H/256) + 4` | **10** (hand-recomputed, matches) | **100** (hand-recomputed, matches) | safe, both ≪ 4,096 |
| `Puzzle2dFillSessionWork` search verbs (`…set-fill-count/🦀️.rs:1463-1477`) | fixed `256+512+256+4 = 1,028` (document-size-independent — a budget ceiling, not a size-derived estimate) | 1,028 | 1,028 | safe; a document too large to fully **capture** or **apply** within its chunk ceiling faults explicitly (`puzzle2d-fill-capture-budget`/`-apply-budget`), a **search** exhaustion instead publishes `CheckpointReady` and resumes — by design, not silent |
| `Puzzle2dFillSessionWork` control verbs (adopt/cancel/discard/clear) | fixed `4` | 4 | 4 | safe |
| generic verbs (23 ids) except 4 selection-acting ones | constant `1` (`EDITOR:1725`) | 1 | 1 | safe |
| `patchInspectorNodes`/`setSelectionFlag`/`deleteSelection`/`duplicateSelection` | `min(selection.len() [or explicit `ids` for patchInspectorNodes], refused above `PUZZLE2D_SELECTION_BATCH_LIMIT=1_024`)` | whole-board selection 717 ≤ 1,024 | — | safe, refused-not-truncated above the batch limit |
| `HostOnly` (2 ids) | `NoopPuzzleCommandWork::extent` = constant `1` | 1 | — | safe |
| `setActiveExample` | unchanged legacy formula, capped at 4,096 | empty→Nakagin 375, Nakagin→Nakagin 748 | — | safe |
| `applyBoardEvents` | event count ≤ `PUZZLE2D_BOARD_EVENT_BATCH_LIMIT` (256) | ≤ 256 | — | safe |
| `import-media` (reserved) | admission caps `PUZZLE2D_IMPORT_SEMANTIC_ITEMS=64`/collection, `PUZZLE2D_IMPORT_DECODED_ITEMS=4_096`, `PUZZLE2D_IMPORT_WORK_UNITS=4_096` (`EDITOR:2870-2874`) | n/a (not document-sized) | — | safe, refused before decode |

No `extent()` returns `None` on either real example; no extent exceeds `PUZZLE_COMMAND_WORK_ITEMS =
4_096` (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:12`) on either example. Traced
`Puzzle2dForceLayoutWork::step()`'s stage machine by hand
(`Nodes/Handles/Edges` scan → `Seed`/`Center` once → `iteration` loop of `Reset→Repel→Springs→Integrate`
until `self.iteration >= iterations` (`EDITOR:2358-2364`, `integrate_one`) → `Emit`): the **declared**
extent is a conservative over-estimate of the **actual** step-call count in every stage (the `+4`/`+8`
margin constants absorb boundary rounding), never an under-estimate — the safe direction, since:

**Systemic finding, not introduced by this wave**: the shared retained-job driver's `Work` phase
(`🎮️commands/🧵️retained/🦀️.rs:553-571`) increments `work_cursor` on every `Progress` step but **never
compares it against `work_extent`** — only the one-time `Preflight` check
(`🧵️retained/🦀️.rs:545`) gates on the declared `extent()`. If any Work's `step()` ever ran *more*
chunks than its own `extent()` promised, nothing in the framework would fault; it would simply run
longer than declared. This audit's hand-trace found every 2d Work's declared extent to be
**greater-or-equal** to its actual step count (safe direction), but this is enforced by hand-checked
arithmetic in each `extent()`/`step()` pair, not by any runtime assertion — a latent correctness
dependency worth a dedicated runtime test (`cargo test` + a real `dev 2d` boot with a step counter),
not something this static read can close out.

---

## 4. Consistency — five independent lists, grep-measured

| Source | Count | Notes |
|---|---|---|
| `PUZZLE2D_RETAINED_TOOL_IDS` (`EDITOR:1054`) | **39** | ground truth |
| `.action_interactive_job(_, Migrated)` (`EDITOR:3929-3967`) | **39**, alphabetical, all `Migrated` | set-equal to the const (diffed, 0 delta) |
| `InteractiveJobClassification::BatchOnlyPendingRewrite` | **0** | `grep -c` |
| proofs `tools: [...]` (`EDITOR:3614-3649`) | **39**, same order as the const | set-and-order-equal |
| `PUBLICATION_CONTRACTS` retained factory (`EDITOR:1193`) | **39** | set-equal to the const |
| `PUBLICATION_CONTRACTS` reserved factory (`EDITOR:2934`) | **1** (`import-media`) | correctly excluded from the 39 |
| `🔏️publication-authority/🔣️.json`, `Puzzle2dPlayApp` | **40** routes across 4 groups, all `Migrated`, 0 blockers | 39 retained + `import-media`; set-equal by group to source |
| `🗄️retained-jobs/🔣️.json` `toolIds` | **39**, same order | order-and-set-equal to the const (Python-verified) |
| `🗄️retained-jobs/🔣️.json` `evidenceToolIds` | **13** | 36 of the 39 retained ids (all but `addNode`/`forceLayout`/`setActiveExample`) have **zero** fixture-vector evidence — a coverage gap, not a classification defect (already flagged in wave B1 §8, still open) |

**Unclassified/BatchOnly count: 0/0.** No id present in one list and missing from another was found.
`setActiveUtility` is correctly outside all five lists — it is a framework-injected action via
`resumable_framework_catalog`, never carries `.action_interactive_job`, and is not itself a retained
tool id (confirmed by wave B1; re-verified: `SET_ACTIVE_UTILITY_ACTION_ID` has its own dispatch arm at
`EDITOR:1649` reached only from `puzzle2d_dispatch_emit`, never from `build_tool_job`).

---

## 5. Runtime-fault risks visible statically

| Risk | Where | Assessment |
|---|---|---|
| `Puzzle2dFillSessionWork::close_one` `Drop`-assert ordering | `…set-fill-count/🦀️.rs:960-1002` | Closes innermost-first: `outcome` → `apply` cursor → `checkpoint` (hand back to `search` via `adopt_checkpoint`, or spin up `closing_search` on failure) → `closing_search` → `search` → `ingress` → scalar fields. On a failed hand-back, the **original** `self.search` (if `Some`) is left untouched this call but is still reached and closed by the `self.search.is_some()` branch on a later call (after `closing_search` drains) — no leak by static trace, but this is exactly the class of bug (`Drop`-assert abort, not a compile error) B2's own §7.4 flagged as uncatchable without `cargo test -p semio-s-plugin-puzzle` + a real fill dispatch. **Not independently runtime-verified by this audit.** |
| `unwrap()`/`expect()` on wire/document-decoded input | grepped `EDITOR` + `…set-fill-count/🦀️.rs` | Every `.expect(...)` in both files sits inside `#[test]` functions or on `Puzzle2dConfig::default()` (`EDITOR:3851`, guarded by a passing unit test that the default locale/terminology are always `Some`). **No unwrap/expect found on live wire-decoded input in either file's production code**, including the new `import-media` region (`EDITOR:2874-3531`, zero `unwrap`/`expect`). |
| `RefCell<BoardHost>` borrow pattern | `puzzle2d_dispatch_emit`, `EDITOR:1619-1682` | The scoped `host_mut` borrow (`EDITOR:1627-1633`) is released via block-scope before the `match`; every command fn borrows/releases `ctx.host` independently and sequentially (never holds two overlapping `borrow_mut()`s) — e.g. `commit_slot` calls `ctx.host.borrow_mut().brush_commit_slot();` then a **separate** statement `apply_host_events(&mut ctx.host.borrow_mut(), ctx.scene)`; the first borrow's temporary is dropped at the `;` before the second is taken. No double-borrow panic found by trace. |
| `Fault` paths / `build_document_store_owners` / `build_config_store_owners` | `EDITOR:3530-3551` | All six overrides present (`build_document_store_owners`, `build_config_store_owners`, `build_config_store_one_item_preparation_factory`, `build_artifact_store_one_item_preparation_factory`, `build_document_store_disposer`, `build_config_store_disposer`) and structurally identical in shape to Puzzle3dPlayApp's six (`🧊️3d/…/✏️editor/🦀️.rs:6716-6738`), using the same `bounded_*` framework helpers rather than hand-rolled bodies — lowest-risk of the six, per B1's own risk ranking. `mounted_job_prepare_snapshot_read`/`pending_effects` overrides are confirmed **absent** (only referenced inside a hostile-mutation source-contract test, `EDITOR:4184-4206`), matching B2's design (fill sessions no longer need a store lease). |
| Reserved `import-media` job (`Puzzle2dImportJob`) | `EDITOR:2874-3531` | `build_reserved_tool_job` rejects non-empty `raw_wire` and non-`Media` input before touching the payload (`EDITOR:3729-3734` region); admission (root-key/schema/collection-size/item-total checks) runs strictly before any mapping. No unwrap/expect. Not compiled or runtime-tested by this audit. |

---

## Findings, ranked by severity

| # | Severity | Finding | Evidence | Fix |
|---|---|---|---|---|
| 1 | **Unverified runtime risk (not a found bug)** | `Puzzle2dFillSessionWork::close_one`'s engine-owner release order and the `Drop`-assert correctness of `BoardFillCheckpoint`/`BoardFillPlacement`/`FillPlacementApplyCursor`/`BoardFillJob` cannot be proven by static reading — a wrong release order aborts the wasm component rather than failing a test | `…set-fill-count/🦀️.rs:960-1002` | Run `cargo test -p semio-s-plugin-puzzle` plus a real fill dispatch (`setFillCount`/`brushFillSessionBegin` → `Step` → `Adopt`/`Cancel`) under `dev 2d`, watching for an abort rather than a clean fault |
| 2 | **Unverified runtime risk (not a found bug)** | The shared retained-job driver never checks `work_cursor ≤ work_extent` during the `Work` phase (`🎮️commands/🧵️retained/🦀️.rs:553-571`) — every 2d Work's extent formula is *hand-proven*, not framework-enforced, to be ≥ its actual step count | `🎮️commands/🧵️retained/🦀️.rs:12,338-340,545` (preflight-only check) | No source fix needed for 2d today (all formulas verified conservative by hand-trace); flag for whoever owns the shared retained-job driver to add a `work_cursor ≤ work_extent` assertion so a future Work's mismatch faults loudly instead of silently over-running |
| 3 | **Cosmetic dead code** | `puzzle2d_dispatch_emit`'s match still carries a `"redrawHandles" => redraw_handles::redraw_handles(ctx)` arm running the old *unbounded whole-fixture* legacy handler; unreachable from any retained job but reachable if `ArtifactEditor::handle` is ever invoked directly with that action id (e.g. a test bypassing classification-based routing) | `EDITOR:1641` (arm), `EDITOR:3675` (retained arm that supersedes it), `…🔄️redraw-handles/🦀️.rs` (legacy leaf) | Delete the match arm, the `🔄️redraw-handles` leaf module and its crate-root `mod` mount, per wave B2 §6.4 (already routed there, still open) |
| 4 | **Cosmetic / test-coverage gap** | 36 of the 39 retained ids have zero `evidenceToolIds`/vector coverage in the retained-jobs fixture — only `addNode`, `forceLayout`, `setActiveExample` are represented | `🗄️retained-jobs/🔣️.json`: `evidenceToolIds` (13 total, Python-diffed against the 39 `toolIds`) | Add at least one vector for a generic verb and one for a `HostOnly` verb, per wave B1 §8 (already routed to A1, still open) |
| 5 | **No defect found** | Lane declarations (§1), silent-no-op risk (§2), extent budgets (§3), and the five-list consistency (§4) all check out exactly — every emitted lane is declared, every declared lane is reachable, every retained id has a real handler, no extent is `None` or `>4,096` on either example, and all five classification lists are set-and-order-equal | See §1-§4 above | none needed |

No emitted-but-undeclared lane (would runtime-fault) and no silent no-op reachable from a real
dispatch were found anywhere in the 39 retained ids or the reserved `import-media` route. The two
"unverified runtime risk" rows are exactly the class of thing this static read cannot close — they
need `cargo test -p semio-s-plugin-puzzle` and a real `dev 2d` boot, which is the ticket's own next
step per `📓️status.md`'s last log line.
