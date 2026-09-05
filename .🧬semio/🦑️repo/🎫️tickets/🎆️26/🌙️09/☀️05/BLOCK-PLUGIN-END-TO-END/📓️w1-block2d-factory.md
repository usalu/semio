# W1 — block2d dispatch-live + non-empty boot + governance

Packet: make the `block2d` app of `✏️s/🔌️plugins/🧱️block` dispatch-live, boot on a real document, and
extend the plugin's publication-authority law to it. Precedent mirrored: the block5d editor
(`🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`).

## 1. Files and symbols changed

### `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

New `//#region 🧵️RetainedCommands`:

| symbol | note |
|---|---|
| `BLOCK2D_RETAINED_TOOL_IDS` | all nine ids (`patchNodeKind`, `addHandleKind`, `removeHandleKind`, `addHandle`, `removeHandle`, `addCompatibilityRule`, `removeCompatibilityRule`, `setActiveExample`, `edit`) |
| `BLOCK2D_RETAINED_PAYLOAD_SCHEMA` | `"block.2d.tool-command.v1"` |
| `BLOCK2D_RETAINED_RAW_BYTES` / `BLOCK2D_RETAINED_WORK_ITEMS` | `65_536` / `4_096` (block5d's envelope) |
| `BLOCK2D_PUBLICATION_CONTRACTS` | nine `ArtifactToolPublicationContract` rows, all `Artifact` lane |
| `block2d_retained_contract()` | `ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500)` |
| `block2d_retained_extent(..)` | rejects non-retained ids; bounds `handle_kinds + handles + compatibility + attributes + authors + 1` against `BLOCK2D_RETAINED_WORK_ITEMS` |
| `block2d_retained_reduce(..)` | dispatches through `ArtifactView::with_operation` |
| `Block2dRetainedCommandJobFactory` | `ToolJobFactory` (classification `Migrated`) + `ArtifactOwnedToolJobFactory` (owner `EditorApp<Block2dPlayApp>`, schema `BLOCK_2D_SCHEMA`) |

New `//#region 📬️StorePreparation`: `Block2dStorePreparationFactory` / `Block2dStorePreparation`
(one-item `ArtifactStoreOneItemPreparationFactory` / `ArtifactStoreOneItemPreparation` pair — freshness
guards on operation/generation/base_revision, `prepare_one_item`, cancel/begin_close/close_step/
terminal_is_empty), mirroring block5d row for row against block2d's own collections.

New trait items inside `impl ArtifactEditor for Block2dPlayApp`:

- `build_artifact_store_one_item_preparation_factory()` → `Block2dStorePreparationFactory`
- `bounded_first_step_tool_proofs! { … factory_type: Block2dRetainedCommandJobFactory, … }` —
  controller `"s.block.block2d@1/*#editor"`, `document_schema: "block.2d"`, all nine tools.
  `factory_type:` is what turns a bare (`missing-owned-reducer`) proof into an exact-owner proof.
- `register_tool_job_factories(..)` → registers `Block2dRetainedCommandJobFactory::new(controller_id)`
- `build_tool_job(..)` → builds `BoundedArtifactCommandWork` + `ArtifactRetainedCommandPayload` +
  `ToolOperationSpec`, faulting `block2d-retained-command-tool-mismatch` on an id/extent mismatch
- `initial_snapshot()` now returns `crate::artifacts::block2d::schema::default_block2d_snapshot()`

Manifest: all nine `.action_interactive_job(id, InteractiveJobClassification::BatchOnlyPendingRewrite)`
flipped to `::Migrated`.

### `…/◻️2d/…/✳️any/🧬️schema/🦀️.rs`

Added `default_block2d_snapshot()` next to the untouched `empty_block2d_snapshot()`:
parses `BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT` via `snapshot::text::parse_dsl`, falling back to
`empty_block2d_snapshot()` on a parse error. `empty_block2d_snapshot()` and its
`empty_definition_matches_default` test are unchanged. This mirrors generation3d's
`schema::default_snapshot()` (`…/🧊️generation3d/…/🧬️schema/🦀️.rs:286`), the cleanest of the precedents —
one artifact-side boot document shared by both surfaces rather than duplicated in editor and viewer.

### `…/◻️2d/…/✳️any/👁️viewer/🦀️.rs`

`Block2dViewer::initial_snapshot()` → `schema::default_block2d_snapshot()` (no editor import; the
viewer-purity rule is preserved because the boot document lives on the artifact side).

### `✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json` + `🧬️.schema.json`

Generalised from one `Block5dPlayApp` owner to an `apps[]` list. Each entry carries `owner`,
`toolIdsConstant` (`^BLOCK[0-9]D_RETAINED_TOOL_IDS$`), `source`, `routes[{id,lane}]`, `laws`, `ui`.
`Block2dPlayApp` (9 routes) and `Block5dPlayApp` (7 routes) are declared; the `owner` enum already
admits `Block3dPlayApp`, so adding block3d is a fixture-only edit — the oracle iterates `apps[]` and
parameterises both the tool-id-constant regex and the hostile mutations off each entry's own data.

### `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts`

- **Bug fixed**: the script resolved `🔣️publication-authority.json` / `🔣️publication-authority.schema.json`
  at the plugin root — neither exists on disk. It now resolves
  `🧪️publication-authority/🔣️.json` and `🧪️publication-authority/🧬️.schema.json`. (Before this fix the
  `test` target could only ever have thrown on the missing file.)
- `appOracle(app, source)` per entry; `oracle(fixture, sources)` iterates `apps[]`.
- The publication-contract regex now captures the **lane** as well
  (`ArtifactToolPublicationLane::(\w+)`) and compares `id:lane` pairs, so a future Config/HostOnly
  route must be declared honestly in the fixture instead of being silently accepted as `Artifact`.
- Anchors extended with `factory_type:`, `register_tool_job_factories`, `build_tool_job` — the three
  things whose absence made block2d dispatch-dead in the first place.
- Hostile checks kept and generalised, now three source mutations **per app** (drop the last route's
  publication contract, drop the `base_revision` freshness guard, drop one `Migrated` classification)
  plus one hostile fixture per app (drop that app's first route). Each hostile source is also asserted
  to have actually changed the text, so a stale/mis-typed hostile string cannot pass vacuously.

## 2. Lane table — `Emit` evidence per handler

`grep -rn "Emit::" ✏️editor/🎮️commands` (block2d), one row per action:

| action | handler | `Emit` evidence | lane |
|---|---|---|---|
| `patchNodeKind` | `🏷️patch-node-kind/🦀️.rs:26,28` | `Emit::default()` / `Emit::mutations(vec![mutation])` | `Artifact` |
| `addHandleKind` | `🔘️add-handle-kind/🦀️.rs:16` | `Emit::mutations(vec![… create_handle_kind …])` | `Artifact` |
| `removeHandleKind` | `🗑️remove-handle-kind/🦀️.rs:16` | `Emit::mutations(vec![… delete_handle_kind …])` | `Artifact` |
| `addHandle` | `🌱️add-handle/🦀️.rs:15,19` | `Emit::default()` / `Emit::mutations(vec![… create_handle …])` | `Artifact` |
| `removeHandle` | `➖️remove-handle/🦀️.rs:16` | `Emit::mutations(vec![… delete_handle …])` | `Artifact` |
| `addCompatibilityRule` | `🔗️add-compatibility-rule/🦀️.rs:19,23` | `Emit::default()` / `Emit::mutations(vec![… add_compatibility_rule …])` | `Artifact` |
| `removeCompatibilityRule` | `🚫️remove-compatibility-rule/🦀️.rs:16` | `Emit::mutations(vec![… remove_compatibility_rule …])` | `Artifact` |
| `setActiveExample` | `🎬️set-active-example/🦀️.rs:163,164` | `Emit::mutations(replace_document_operations(..))` / `Emit::default()` | `Artifact` |
| `edit` | `🎨️edit/🦀️.rs:158,159` | `Emit::mutations(replace_document_operations(..))` / `Emit::default()` | `Artifact` |

No block2d handler constructs a `Block2dConfigMutation`, a draft, presence or transient mutation —
zero `Emit::config*`/`Emit::draft*` call sites exist under `✏️editor/🎮️commands`. So, unlike
generation2d (whose `nodeGraphViewport`/`setShowMode`/`generate` are honest `Config` routes and whose
canvas pointer verbs are `HostOnly`), every block2d route is `Artifact` — the same disposition
block5d has, arrived at from block2d's own evidence rather than copied.

## 3. Law test

`retained_route_dispositions_are_exact_and_exhaustive` (block2d editor, `//#region 🔖️CommandSurface`),
patterned on generation2d's test of the same name:

- `BLOCK2D_RETAINED_TOOL_IDS.len() == 9`
- `<Block2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len() == 9`
- `Block2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len() == 9`, every route present and
  `lanes == [Artifact]`
- contract `shape == BoundedFirstStep`, `cancellation == PerOperation`
- the retained id set equals the `Block2dCommand` row set (`every_command()`)
- every retained id's manifest action carries `InteractiveJobClassification::Migrated`

Also added `boots_on_the_forest_left_example_document` (node kind id, 11 handles, non-empty handle
kinds, `!= empty_block2d_snapshot()`, and the board body render containing the node-kind label).

Three pre-existing tests were rewritten from absolute counts to deltas against the booted document
(`add_handle_kind_then_add_handle_then_remove_round_trips`, `undo_redo_round_trips_through_the_wrapper`,
`interaction_topology_nests_handles_under_their_handle_kind`) — they asserted `len() == 1`/`== 0` on
what used to be an empty boot snapshot.

## 4. Verification

See `🗑️generated/w1-*.txt` for the raw logs.

### `cargo check -p semio-s-plugin-block --lib`

```
PLACEHOLDER_CARGO_CHECK
```

### `cargo test -p semio-s-plugin-block --lib block2d`

```
PLACEHOLDER_CARGO_TEST
```

### `bun ./📜️script.ts test` (block TS bundle)

```
validated Block publication authority; apps=Block2dPlayApp:9,Block5dPlayApp:7; schema=Ajv; oracle=owned; hostile=8
```

## 5. Unverified / out of scope

- Not run in this packet: the `wasm32-wasip2` build, the descriptor `describe` regeneration, and the
  react playground boot (`bun run dev:block:2d`). Runtime dispatch-liveness in the browser is
  therefore argued from the framework's own admission code, not observed.
- block3d is untouched (another agent's packet). Its fixture entry is deliberately absent; the schema's
  `owner` enum and the oracle's `apps[]` loop already accept it.
- The inspector's `patchNodeKind` `on_change` payload carries only a `"field"` key and no `"value"`
  (`📌️panels/🔍️inspection/🦀️.rs:38`), flagged in `📓️explore-block2d-editor.md` §6. Unchanged here —
  it is a host input-commit question, not a tool-proof one.
