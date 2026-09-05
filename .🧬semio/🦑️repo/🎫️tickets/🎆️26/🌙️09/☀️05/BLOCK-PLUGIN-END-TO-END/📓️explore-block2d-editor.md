# block2d editor — exploration notes

Subset root: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/`.
Crate entry: `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs`.

## 1. Modes and windows

**Editor** (`✏️editor/🦀️.rs:186-188`, mode def `✏️editor/🎭️modes/✏️edit/🦀️.rs:9-11`, window def
`✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs:14-30`):

| Mode | Window | `SurfaceKind` | Body key |
|---|---|---|---|
| `edit` (`BLOCK2D_PLAY_MODE_EDIT`, only mode, `.default_mode_id`) | `block2d-board` (`BLOCK2D_WINDOW_BOARD`, only window) | `SurfaceKind::Board2d` | `block2d.play.board` |

Board window render (`✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs:33-38`) is a two-line
`ui_stack_vertical`/`ui_text` summary: `"{summary}: {node_kind.label or '—'}"` and
`"{handle_kinds.len()} …, {handles.len()} …"`. It always prints *some* text (never a true empty
node) — the counts are the only thing that vary with document state, and they read straight off
`Block2dSnapshot.node_kind` / `.handle_kinds` / `.handles`. So "non-empty content" for this window
means `node_kind.label` non-blank and/or `handle_kinds`/`handles` non-empty.

Two panel tabs also render document state: the document tree
(`✏️editor/📌️panels/🗿️artifact/🦀️.rs:35-45`, needs `handle_kinds`/`handles` non-empty to show
anything other than the `no_handle_kinds`/`no_handles` placeholder rows) and the inspector
(`✏️editor/📌️panels/🔍️inspection/🦀️.rs:53-66`, always renders the `node_kind` identity fields —
name/label/variant/description — plus a read-only handle count; blank fields just show empty text
inputs, never a placeholder).

**Viewer** (`👁️viewer/🦀️.rs:79-88`, mode `👁️viewer/🎭️modes/👁️view/🦀️.rs`, window
`👁️viewer/🎭️modes/👁️view/🪟️windows/📋️board/🦀️.rs:14-30`): one mode `view`
(`BLOCK2D_VIEW_MODE_VIEW`), one window `block2d-view-board`, also `SurfaceKind::Board2d`, body key
`block2d.view.board`. Render (`…board/🦀️.rs:37-46`) lists node-kind label, then every handle kind
(id/color/label) and every handle (id/kind/angle/radius) as `ui_text` lines — needs
`handle_kinds`/`handles` non-empty to show more than the two header lines. The viewer never mutates
(`Block2dViewCommand` has one inert `Noop` variant, `👁️viewer/🦀️.rs:14-30`).

Both editor and viewer are single-mode, single-window apps — no board/mode alternatives to compare.

## 2. Document/snapshot state and default boot document

`Block2dSnapshot` (`🧬️schema/📸️snapshot/🦀️.rs:9-60`, `#[artifact_schema(id = "s.block.block2d")]`):
`schema: String`, `node_kind: BlockKindIdentity`, `presentation: Block2dPresentation`,
`handle_kinds: Vec<Block2dHandleKind>`, `handles: Vec<Block2dHandleTemplate>`,
`compatibility: Vec<BlockCompatibilityRule>`, `attributes: Vec<BlockAttribute>`,
`authors: Vec<BlockAuthor>`, `camera2d: BlockCamera2d`, `meta: BlockMeta` (plus trailing fields not
read past line 60).

Boot document: `ArtifactEditor::initial_snapshot()` (`✏️editor/🦀️.rs:67-69`) calls
`crate::artifacts::block2d::schema::empty_block2d_snapshot()`, which is literally
`Block2dSnapshot::default()` (`🧬️schema/🦀️.rs:251-254`, pinned by test
`empty_definition_matches_default` at `🧬️schema/🦀️.rs:275-277`). **No example is parsed by
default** — unlike `generation3d` (which boots `hexagonal-mushroom-column` via its own
`initial_snapshot`), block2d's default boot state is the all-`Default` empty snapshot: empty
`node_kind`, zero `handle_kinds`, zero `handles`. The viewer's `initial_snapshot`
(`👁️viewer/🦀️.rs:52-54`) is the same `empty_block2d_snapshot()`.

An example is only loaded if `setActiveExample` is dispatched (see §5) — and per §3/§4 below, that
dispatch is currently unreachable from the UI.

## 3. Every editor action/command

All nine rows come from one `app_commands!` block (`✏️editor/🦀️.rs:140-153` region, i.e. lines
20-37 of the earlier read) and are declared as bare app-level actions via `.mutation(id, label)`
(`✏️editor/🦀️.rs:208-216`) each immediately followed by `.action_interactive_job(id, …)`
(`:217-225`):

| id | `InteractiveJobClassification` | Handler exists |
|---|---|---|
| `patchNodeKind` | `BatchOnlyPendingRewrite` (`:217`) | yes — `✏️editor/🎮️commands/🏷️patch-node-kind/🦀️.rs:16` |
| `addHandleKind` | `BatchOnlyPendingRewrite` (`:218`) | yes — `✏️editor/🎮️commands/🔘️add-handle-kind/🦀️.rs:13` |
| `removeHandleKind` | `BatchOnlyPendingRewrite` (`:219`) | yes — `✏️editor/🎮️commands/🗑️remove-handle-kind/🦀️.rs:15` |
| `addHandle` | `BatchOnlyPendingRewrite` (`:220`) | yes — `✏️editor/🎮️commands/🌱️add-handle/🦀️.rs:13` |
| `removeHandle` | `BatchOnlyPendingRewrite` (`:221`) | yes — `✏️editor/🎮️commands/➖️remove-handle/🦀️.rs:15` |
| `addCompatibilityRule` | `BatchOnlyPendingRewrite` (`:222`) | yes — `✏️editor/🎮️commands/🔗️add-compatibility-rule/🦀️.rs:17` |
| `removeCompatibilityRule` | `BatchOnlyPendingRewrite` (`:223`) | yes — `✏️editor/🎮️commands/🚫️remove-compatibility-rule/🦀️.rs:15` |
| `setActiveExample` | `BatchOnlyPendingRewrite` (`:224`) | yes — `✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` (`handle` at end of file) |
| `edit` | `BatchOnlyPendingRewrite` (`:225`) | yes — `✏️editor/🎮️commands/🎨️edit/🦀️.rs:156` |

None of the nine is `Migrated`; **every single block2d action is `BatchOnlyPendingRewrite`** — a
strictly worse ratio than the sibling `generation3d` ticket's "6 of 29" gap. `grep -n
"unimplemented!\|todo!" ` across all nine command files returns nothing — every handler body is a
real implementation, not a stub.

`command_from_action` (`✏️editor/🦀️.rs:82-99`) bridges all nine action ids to `Block2dCommand`
variants; `handle` (`✏️editor/🦀️.rs:101-110`) dispatches through `command.dispatch(doc, cfg)`
(framework-generated by `app_commands!`). The wiring from action-id to handler is intact; only the
UI-dispatch admission gate in front of it rejects every one of them (see §4).

## 4. Tool proofs / factory wiring — missing entirely

`grep -n "bounded_first_step_tool_proofs\|factory_type\|BoundedCommandJobFactory\|register_tool_job_factories\|build_tool_job\|build_artifact_store_one_item_preparation_factory\|RETAINED_TOOL_IDS" ✏️editor/🦀️.rs` on block2d returns **nothing**. There is:

- no `bounded_first_step_tool_proofs!` macro invocation at all (not even the bare, no-`factory_type`
  form the `PROCEDURAL-3D` root-cause note flags as broken),
- no app-owned `*BoundedCommandJobFactory`,
- no `register_tool_job_factories` / `build_tool_job` / `build_artifact_store_one_item_preparation_factory`
  override,
- no `BLOCK2D_RETAINED_TOOL_IDS` constant.

Compare to **block5d** (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`),
which has the complete pattern:
- `BLOCK5D_RETAINED_TOOL_IDS: &[&str] = &["patchPartKind", "addGripKind", "removeGripKind",
  "addGrip", "removeGrip", "setActiveExample", "edit"]` (`:148`),
- `Block5dRetainedCommandJobFactory` (used as `factory_type:` inside
  `bounded_first_step_tool_proofs!`, `:386-396`),
- `register_tool_job_factories` (`:397`), `build_tool_job` (`:402-403`),
  `build_artifact_store_one_item_preparation_factory` override (`:382`),
- every one of its seven actions classified `Migrated` (`:597-603`).

The plugin-level publication-authority law/test only covers block5d, not block2d:
`✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json` has `"owner": "Block5dPlayApp"`,
`"source": "🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs"`, and lists exactly block5d's seven routes. The
Bun test that checks it, `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts:11-15`,
regexes `BLOCK5D_RETAINED_TOOL_IDS` (not a block2d equivalent) out of that one file. **There is no
governance test at all for block2d's tool-proof/factory wiring** — its total absence would not be
caught by this suite.

Tracing what this absence means at runtime (framework source,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`):
- `validate_ui_dispatch_classification` (`:12033-12038`) accepts only `Migrated`; every other
  classification returns `Fault("interactive-job.not-ui-safe")`. It is called at the very top of
  `dispatch_action` (`:22360`, the framework's single non-reserved UI entry point) and of
  `dispatch_command` (`:22407`). Since block2d's nine actions are all `BatchOnlyPendingRewrite`,
  **every one is rejected before `admit_command_json`/`qualified_tool_proof` is even reached** —
  100% of block2d's actions are dispatch-dead through the standard UI action path.
- Independently, even if all nine were reclassified `Migrated`, dispatch would still fail one step
  later: `qualified_tool_proof` (`:19357-19366`) checks `app_tool_registrations`, then
  `framework_tool_registrations`, then `bounded_tool_proofs`, in that order, returning
  `Err("interactive-job.missing-owned-reducer")` only for verbs present in `bounded_tool_proofs`
  (populated by a bare, no-`factory_type` `bounded_first_step_tool_proofs!` call — the
  `PROCEDURAL-3D` bug). Block2d verbs are in **none** of the three maps (no macro call registers
  them anywhere), so the fallback at `:19365-19366` fires instead:
  `Err("interactive-job.missing-factory", "typed command '{verb}' has no exact
  controller/owner/factory/tool/schema proof")`. Block2d's fault is strictly more total than the
  `generation3d` bare-factory bug: gen3d's actions at least reach the tool-proof table and get a
  more specific "missing-owned-reducer"; block2d's never reach any table at all.

Net: fixing block2d requires **both** (a) reclassifying the nine actions to `Migrated` and (b)
building the entire tool-proof/factory apparatus from scratch (an app-owned
`Block2dBoundedCommandJobFactory` or equivalent, `BLOCK2D_RETAINED_TOOL_IDS`,
`register_tool_job_factories`, `build_tool_job`, an `ArtifactStoreOneItemPreparationFactory`
override, and `factory_type:` inside a new `bounded_first_step_tool_proofs!` block) — mirroring
block5d's precedent (`🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs:148-403`) or gen2d's
(`generation2d` editor `🦀️.rs:115-203`, per the sibling ticket's status note).

## 5. Examples

Two example ids, both declared via `ExampleSource` in the subset root
(`🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:24`):
`crate::examples::art_2d_hexagonal_cut_concrete_forest_left::source()` and
`…_right::source()`. Each example module (`📚️examples/🎬️hexagonal-cut-concrete-forest-left/🦀️.rs`,
`📚️examples/➡️hexagonal-cut-concrete-forest-right/🦀️.rs`) defines `ID`
(`"hexagonal-cut-concrete-forest-left"` / `"-right"`), a `LocalizedLabel`, an icon, and
`PRIMARY_TEXT = include_str!("🖼️assets/…/🗣️.dsl.semio")` — **the example DSL text is embedded into
the binary at compile time via `include_str!`**, not read from disk at runtime. The
`🖼️assets/` subfolders also carry `.pack.semio`, `.spr.semio`, `.op.semio` sibling artifacts, but
`set_active_example`'s handler only reaches for the DSL text (see below) — those other asset files
are not loaded outside of the example module's own `🧪️tests` (fixture-style tests reading the same
directory).

Wiring: `set_active_example::handle` (`✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs`, end of
file) matches `payload.id` against `BLOCK2D_EXAMPLE_LEFT`/`BLOCK2D_EXAMPLE_RIGHT` constants (top of
same file), calls `crate::artifacts::block2d::dsl::parse_dsl(…_EXAMPLE_TEXT)` (a separate DSL
constant, not the example module's own `PRIMARY_TEXT` — both ultimately derive from the same
`.dsl.semio` source but through different plumbing), and on success diffs the parsed document
against the current one via `replace_document_operations` (same file, the whole first half — a
hand-rolled per-field/per-row VCS-operation diff, not a raw snapshot replace) to produce an
`Emit::mutations(...)`. This is a real, non-trivial implementation. Per §3/§4, though,
`setActiveExample` is one of the nine `BatchOnlyPendingRewrite` + proof-less actions, so it cannot
currently be triggered from the UI — examples cannot be switched at runtime even though the
plumbing to do so is fully implemented.

`command_from_action` accepts either `"exampleId"` or `"id"` as the JSON arg key
(`✏️editor/🦀️.rs:92`).

## 6. Other defects / notes

- **All nine block2d actions are dispatch-dead** (§3/§4) — this is the headline defect, matching
  and exceeding the `PROCEDURAL-3D-END-TO-END` reference fault pattern (there: 6 of 29 actions
  blocked by wrong classification, with the bare-factory bug affecting the other 23; here: 9 of 9
  blocked by wrong classification, AND zero tool-proof wiring exists for any of them).
- **No governance test catches this for block2d.** The one publication-authority law/oracle the
  block plugin owns (`🧪️publication-authority/🔣️.json` + `📦️packages/🟦️typescript/📜️script.ts`)
  is scoped exclusively to `Block5dPlayApp` — it does not assert anything about block2d's (or
  block3d's) tool-proof wiring, so block2d's total absence of that wiring was never flagged by CI.
- The inspector's `patchNodeKind` field inputs
  (`✏️editor/📌️panels/🔍️inspection/🦀️.rs:38`) build their `on_change` action payload with only a
  `"field"` key (`ui_value_map([("field", …)])`), never a `"value"` key, even though
  `command_from_action`'s `patchNodeKind` arm reads both `str_field("field")` and
  `str_field("value")` (`✏️editor/🦀️.rs:85`). This may be normal framework convention (the host
  merging the live input value in at commit time) rather than a bug — not verified against the
  framework's input-commit wiring in this pass; flagged for anyone chasing why `patchNodeKind`
  produces empty `value`s if/when it becomes dispatchable.
- `#[path]` mounts: verified all 604 unique `#[path = "…"]` targets referenced anywhere in the
  crate entry (`📦️packages/🦀️rust/🦀️.rs`, covering 2d/3d/5d) resolve to real files/directories on
  disk — **zero dangling mounts**, including every block2d-scoped one (editor, viewer, schema, io,
  examples, tests). Confirmed by literal existence-check of the extracted path list.
- Board-window and viewer-board-window renders never truly go empty (§1) — they always emit at
  least header text — so "empty window" in this app manifests as the counts staying at zero and
  the document/inspector panels showing their placeholder rows, not as a blank surface.
- The manifest comment at `✏️editor/🦀️.rs:227-234` explicitly documents an SDK gap: `EditorBuilder`
  has no `.example(...)`/`.workflow(...)` builder method, so old app-level example registrations
  were dropped in favor of the subset-level `📚️examples/*` facet used today — a known, self-reported
  limitation, not a silent regression.
