# 📓️ A7 — graph & list apps (flow, dag, vcs, note, sequence, writer, animate, imperative, remodel)

Wave-2 packet A7 of ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING. Nine plugins whose panels
rendered unbounded `ui_node_list`s and hard-failed with `ui.fixed-capacity` past the fixed child
capacity. Every list section is now a windowed section, every recursive level a windowed group item,
and the per-crate `ui_node_list`/`fixed_nodes` copies are gone.

Written against 📓️design-virtualised-tree.md §5/§8 and 📓️p3-sdk.md (signatures confirmed by reading
the landed SDK, region `🔖️PanelWindowing`, before any call site was written).

## 1. Panels migrated

| Plugin (crate) | Panel | What changed |
|---|---|---|
| 🌊️flow (`semio-s-artifact-flow-flow`) | `📌️panels/🗿️artifact/🦀️.rs` | `widgets` + `synapses` → `window_section_or_placeholder`; rows → `pick_item(.., "node"/"edge")` |
| | `📌️panels/🛍️catalogue/🦀️.rs` | every host catalogue section + `extensions.installed` + `extensions.actions` → `window_section`; rows keep `addWidget`/`toggleExtension`/`runExtensionAction` and their drag payload |
| | `📌️panels/🔍️inspection/🦀️.rs` | empty section → `window_section_or_placeholder` |
| 🕸️dag (`semio-s-artifact-dag-dag`) | `📌️panels/🗿️artifact/🦀️.rs` | `nodes` + `edges` → `window_section_or_placeholder`; rows → `pick_item(.., "node"/"edge")` |
| | `📌️panels/🛍️catalogue/🦀️.rs` | `node-kinds` → `window_section`; rows keep `addNode` |
| 🌿️vcs (`semio-s-artifact-vcs-vcs`) | `📌️panels/🗿️artifact/🦀️.rs` | `checkpoints` (`history.columns.iter().rev()`) + `alternatives` → windowed; rows keep `checkoutCheckpoint`/`switchAlternative` |
| 🗒️note (`semio-s-artifact-note-note`) | `📌️panels/🗿️artifact/🦀️.rs` | five `Add …` rows moved into their own fixed `note-play-blocks.add` section; blocks → `window_section_or_placeholder` on `note-play-blocks.blocks`; `Group` → `tree_window_item` at every level |
| 🎬️sequence (`semio-s-artifact-sequence-sequence`) | `📌️panels/🗿️artifact/🦀️.rs` | `steps` + `edges` → windowed; every control-flow **slot** row → `tree_window_item` at every depth |
| | `📌️panels/🛍️catalogue/🦀️.rs` | five built-in kinds keep their fixed section; per-slot `addStepToSlot` shortcuts → new windowed `sequence-play-catalogue.slots` |
| ✒️writer (`semio-s-artifact-writer-writer`) | `📌️panels/🗿️artifact/🦀️.rs` | `ast` → `window_section_or_placeholder`; every AST level → `tree_window_item` |
| | `📌️panels/🔍️inspection/🦀️.rs` | **`.take(8)` on lint diagnostics deleted** → `window_section` over the full diagnostic list |
| 🎞️animate (`semio-s-artifact-animate-presentation`) | `📌️panels/🗿️artifact/🦀️.rs` | `tiles` → `window_section_or_placeholder`; rows carry granularity `tile` |
| 📜️imperative (`semio-s-artifact-imperative-procedure`) | `📌️panels/🗿️artifact/🦀️.rs` | `steps` → `window_section_or_placeholder`; rows carry granularity `step` |
| | `📌️panels/🛍️catalogue/🦀️.rs` | `actions` → `window_section`; rows keep `addStep` |
| 📸️remodel (`semio-s-artifact-remodel-remodeling`) | all 7 panels | `⚙️parameters`, `🧵️results`, `🗿️artifact` (pipeline): single fixed row set → `window_section` over a `(id, text)` vector; `✅️quality`: metrics section + new windowed `remodeling-qc.warnings`; `🎯️calibration`: fixed `…summary` section + windowed `…cameras` + `…gcps`; `🏃️tracks`: fixed `…motion` status + windowed `…tracks`; `🗂️media`: fixed drop-zone/counts section + windowed `remodeling-media.imported`, panel-level `.drop_action` kept |

`TreeWindows::for_body(view_state, BODY_KEY)` is built in each app's `ArtifactApp::render` (and in
flow's and sequence's `render_with_request_context`) and threaded into every panel; panels take
`&TreeWindows<'_>`.

## 2. Domain binding

`.interaction_domain(CONTROLLER_ID, DOMAIN)` (two-arg form) on every domain-bound tree:

| Plugin | controller | domain | granularity on pick rows |
|---|---|---|---|
| flow | `FLOW_PLAY_APP_ID` | `FLOW_INTERACTION_GRAPH` | `node` (widgets), `edge` (synapses) |
| dag | `DAG_PLAY_APP_ID` | `DAG_PLAY_INTERACTION_DOMAIN` | `node`, `edge` |
| vcs | `VCS_PLAY_APP_ID` | `VCS_INTERACTION_HISTORY` | — rows keep their own `checkoutCheckpoint`/`switchAlternative` action, so they are not pick rows |
| note | `NOTE_PLAY_CONTROLLER_ID` | `NOTE_INTERACTION_BLOCKS` | `block` (new `NOTE_INTERACTION_GRANULARITY`) |
| sequence | `SEQUENCE_PLAY_APP_ID` | `SEQUENCE_INTERACTION_STEPS` | `step` (new `SEQUENCE_INTERACTION_GRANULARITY`) |
| writer | `WRITER_PLAY_APP_ID` | `WRITER_INTERACTION_AST` (new const) | `node` (new `WRITER_INTERACTION_GRANULARITY`) |
| animate | `PRESENTATION_PLAY_APP_ID` | `PRESENTATION_INTERACTION_DOMAIN` | `PRESENTATION_INTERACTION_GRANULARITY` (`tile`) |
| imperative | `IMPERATIVE_PLAY_APP_ID` | `IMPERATIVE_INTERACTION_STEPS` | `step` (new `IMPERATIVE_INTERACTION_GRANULARITY`) |
| remodel | — declares no `InteractionDefinition`; its panels have no pick rows | | |

Each new granularity constant is also the one the app's own `InteractionDefinition` declares (the
literal in the manifest was replaced by the constant, so the two can no longer drift).

Row keys are the canonical domain target ids the app's `interaction_topology` already registers —
`flow_graph_node_target_id`/`flow_graph_edge_target_id`, the raw dag node/edge id, the raw sequence
step id, `block_tree_row_id` (`note-play-block:{id}`), `step_row_id`, the raw tile/AST id. No pick row
carries a binding or an argument map any more; the tree root carries exactly one `interactionSelect`.

Rows that are not pick targets keep their action: vcs `checkoutCheckpoint`/`switchAlternative`,
catalogue `addWidget`/`addNode`/`addStep`/`addStepToSlot`/`addBlock`, flow
`toggleExtension`/`runExtensionAction`. Flow's catalogue rows stay
`tree_item_with_action_draggable`; note's block rows stay `draggable`.

## 3. Symbols deleted

- Crate-local `ui_node_list` copies in ✒️writer, 🌊️flow, 🌿️vcs, 🎞️animate (panel-local), 🎬️sequence,
  📜️imperative (panel-local), 📸️remodel, 🕸️dag — each replaced by
  `pub use semio_framework_plugin::ui_node_list;` so existing call sites keep their path.
- 🗒️note's panel-local `fixed_nodes` helper.
- ✒️writer inspection's `.take(8)` diagnostic truncation.
- No `paged_panel_section` / `panel_continuation_row` / `panel_page_rows` / `PanelRowBudget` /
  `setPanelPage` / `SECTION_ROWS` / `IDS_ROWS` / `panel_pages` / `.more` / `+N` symbol exists anywhere
  in these nine plugins (verified by grep over `*.rs`, `*.json`, `*.ts`) — this set never had them
  (📓️audit-app-panels-b.md §0), which is exactly why it hard-failed instead of paging.

Added: a small `pick_item(id, label, description, granularity)` helper beside the deleted
`ui_node_list` in 🌊️flow's and 🕸️dag's editor modules (those two build their pick rows from
`tree_item_desc`, which returns a `BuiltNode` rather than a `TreeItemBuilder`; the other plugins set
`granularity` through `TreeItemBuilder::granularity` or directly on `TreeItemProps`).

## 4. Tests

Window laws (a)–(d) added per the brief, each against a 200-entry document, driving the panel
`render` directly with `TreeWindows::unhosted()` or
`TreeWindows::for_body(&ViewModel { tree_windows: vec![TreeWindowRequest{..}], ..Default::default() }, BODY_KEY)`:

| Plugin | test file | laws |
|---|---|---|
| flow | `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 200 `InputNote` widgets — (a)(b)(c)(d) |
| dag | same path | 200 `DagNodeSpec`s — (a)(b)(c)(d) |
| vcs | same path | 200 `HistoryColumn`s — (a)(b)(c) + the vcs variant of (d) (one tree-level `interactionSelect`, every row keeps `checkoutCheckpoint`) |
| note | same path | 200 top-level `Group`s each nesting 200 more — (a) asserts the nested group stamps its own `total`, plus (b)(c)(d) and a law that the five quick-add rows live in their own fixed section |
| sequence | same path | 3 nested slot levels (root steps → `control.if.then` → `control.while.body`), 200 entries each — (a) asserts all three levels stamp `total` and materialise exactly their requested rows; plus (b)(c)(d) and a law that a control step's children stay `[collapse toggle, then-slot, else-slot]` |
| writer | same path | a synthetic 200-leaf `match` AST level — (a)(b)(c) on `jack_ast_to_tree_item`, (d) through `render` |
| animate | same path | 200 `FigureTileDraft`s — (a)(b)(c)(d) |
| imperative | same path | 200-step `Path` — (a)(b)(c)(d) |
| remodel | `📌️panels/🎯️calibration/🧪️tests/🔬️unit/🦀️.rs` | 200 `GroundControlPoint`s — (a)(b)(c) + a law that the two count lines live in their own fixed section. Law (d) is N/A: remodel declares no interaction domain |

Every law also walks the whole built tree asserting no key ends with `.more` and no `TreeItem` label
starts with `+`.

Call sites updated for the new `&TreeWindows` parameter: remodel's `🗿️artifact` unit test and the
`🔍️inspection/🧪️tests/🔬️semantic-contract` tests of sequence, imperative and animate.

### Commands (foreground, shared target dir, no `CARGO_TARGET_DIR`)

Driver: `🐚️a7-verify.sh check|test|wasm` in this ticket folder; logs under `🗑️generated/a7/`.
None of the nine crates declares `component-app-assembly` (checked each `Cargo.toml`), so no feature
flags are needed.

| Lane | Result |
|---|---|
| `cargo check -p <crate> --tests` ×9 | **all 9 rc=0** (`check-summary.txt`) |
| `cargo check -p <crate> --target wasm32-wasip2` ×9 | **all 9 rc=0** (`wasm-summary.txt`) |
| `cargo test -p <crate>` ×9 | compiles everywhere; **all 38 window-law tests pass in all 9 crates** |

Per-crate `cargo test` totals (`test-semio-s-artifact-*.txt`):

| Crate | result |
|---|---|
| `semio-s-artifact-dag-dag` | 177 passed, 27 failed |
| `semio-s-artifact-flow-flow` | 128 passed, 95 failed |
| `semio-s-artifact-vcs-vcs` | 97 passed, 18 failed |
| `semio-s-artifact-note-note` | 335 passed, 57 failed |
| `semio-s-artifact-sequence-sequence` | 73 passed, 130 failed |
| `semio-s-artifact-imperative-procedure` | 82 passed, 59 failed |
| `semio-s-artifact-remodel-remodeling` | 1262 passed, 37 failed |
| `semio-s-artifact-animate-presentation` | binary aborts (SIGABRT) partway |
| `semio-s-artifact-writer-writer` | binary aborts (SIGABRT) partway |

## 5. Pre-existing / peer failures, NOT caused by this packet

The failures above are a framework-wide regression that landed while this packet ran. None mentions a
panel, tree, window, granularity or `ui.fixed-capacity`:

1. **`interactive-job.catalog-authority` / `interactive-job.missing-factory`** — the dominant cause in
   every crate (`tool proof catalog must exactly join migrated generated declarations to live
   concrete factories`, e.g. flow's `removeWidget` with `schema_eq=false`, sequence's `addStep`,
   vcs's `incrementCounter`, remodel's `importFramePayload`). Every app-fixture test that dispatches
   a typed command or renders through the app fixture fails on it.
2. **Retirement/ownership witnesses** — `ordered-map root must be explicitly retired before drop`
   (flow, 20×), `final Dictionary ownership must be explicitly retired or owned by a cold boundary`
   (sequence, 88×), `artifact store reached Drop …` (dag, imperative).
3. **`io::mutations::binary` codec tests abort the binary** in ✒️writer
   (`writer_edit_history_decoder_uses_begin_mutation_and_faults_malformed_input`) and 🎞️animate
   (`retained_presentation_envelope_materializes_populated_history_in_order`) — "panic in a
   destructor during cleanup" → SIGABRT, so those two binaries never print a `test result:` line.
   Both are outside `📌️panels`; every panel test in both crates ran and passed before the abort.
4. **`semio-framework-plugin` briefly did not compile** during one run (`close_witness_report` in a
   peer's `[DBGCLOSE]` line at `plugin/🦀️.rs:7018`); it compiled again on the next attempt.
5. **Cargo was SIGKILLed repeatedly** — a peer session runs `pkill -9 cargo rustc`; the verify script
   retries on any exit code other than 0/101. One run also hit `No space left on device` on the shared
   build dir, which cleared on its own (84 GiB free afterwards; no stale incremental sessions to prune).

## 6. Breakages fixed on the way (in this packet's crates, but not of this packet's making)

These blocked compilation of crates I had to verify, so they are fixed here and called out:

- `🕸️dag/…/📌️panels/🔍️inspection/🦀️.rs` imported `semio_framework_ui_contract` while the crate had no
  such dependency → added `semio-framework-ui-contract.workspace = true` to dag's `[dependencies]`.
- The same file passed already-unwrapped `BuiltNode`s to `ui_node_list` (three call sites) → they now
  pass `UiAssemblyResult<BuiltNode>` (`.map(Ok)` / drop the `?`).
- `🎬️sequence/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:676` referenced an undefined `snapshot` → `fixture`.
- `✒️writer/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` called an unimported, un-annotated
  `artifact_app_laws::new_app()` at five sites → the crate's own registry-backed
  `context::new_app()`.
- `🎞️animate/…/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs` lost `BuiltNode` when a peer
  rewrote that panel onto `PanelTreeBuilder` → explicit `use semio_framework_plugin::BuiltNode;`.
  The same peer edit referenced `crate::editor::animate::ui_node_list`, which did not exist; the SDK
  re-export this packet adds satisfies it.

## 7. Decisions worth knowing

- **Sequence's collapse toggle stays a child of the step row, not of the slot row.** A control step's
  children are a closed set — one `setStepCollapsed` toggle plus the one or two slots
  `control_slots` declares — so that level can never overflow and needs no window. The unbounded axis
  is the *slot's* child-step list, and that is what became `tree_window_item`, at every depth. Putting
  the toggle under each slot row instead would have duplicated one step's collapse control across its
  slots; the law
  `a_control_step_keeps_its_collapse_toggle_beside_its_slot_rows` pins the chosen shape.
- **Note's content section was renamed `note-play-blocks` → `note-play-blocks.blocks`.** The section id
  used to equal the tree root's namespace, so root and section shared one key — ambiguous for the
  host's `data-tree-window-key` and for `TreeWindows::slice`. The quick-add rows now sit in
  `note-play-blocks.add`.
- **Genuinely fixed row sets are still windowed** where they are one flat list (remodel's parameters,
  results and pipeline readouts), so every remodeling panel reports a `total` and the host paints one
  uniform scrollbar. Fixed *chrome* beside a content list (note's add rows, remodel's count lines and
  drop zone, sequence's five built-in kinds) stays in its own plain `.section`, per the brief.
- **Rendering `TreeWindows` per body key inside the `match`**, rather than once before it, keeps the
  first-paint budget scoped to the body actually being rendered.

## 8. Not done

- Wave-3 browser verification is not part of this packet.
- The framework-level failures in §5 are left for whoever owns the tool-proof/retirement work; this
  packet did not touch action factories, retirement or codecs.
