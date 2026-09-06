# Wave B1 — editor migration core (2026-09-06)

Scope delivered: store-preparation factories, one shared dispatch pipeline + generic retained
reduce/extent, registration of all remaining verbs, `setLocale`/`setTerminology` declarations,
removal of the dead legacy example chain, publication-authority fixture rewrite, real puzzle2d
branch in the TS oracle.

No cargo command was run (host contention); the main session owns compile verification. Everything
below that is asserted as *measured* was measured with grep/bun/rustfmt, not inferred.

---

## 1. Final classification counts (read directly from source at the end of the wave)

Measured on `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

| Measurement | Command | Result |
|---|---|---|
| `Migrated` declarations | `grep -c 'action_interactive_job(.*Migrated)'` | **39** |
| `BatchOnlyPendingRewrite` declarations | `grep -c 'InteractiveJobClassification::BatchOnlyPendingRewrite'` | **0** |
| Unclassified command ids | `puzzle2d_command_variants!` (40) minus 39 classified | **1** — `setActiveUtility`, framework-injected `Migrated` via `resumable_framework_catalog`; it never had (and must not have) an `.action_interactive_job` call |
| `PUZZLE2D_RETAINED_TOOL_IDS` | parsed | **39** |
| retained-factory `PUBLICATION_CONTRACTS` | parsed | **39** (a 40th `ArtifactToolPublicationContract` exists at `:2952` — that is B3's separate `Puzzle2dImportJobFactory` for the reserved `import-media` route, not the retained factory) |
| proofs `tools: [...]` | parsed | **39** |
| fixture `Puzzle2dPlayApp` routes | parsed | **39**, all `Migrated`, 0 blockers |

Start of wave: 4 Migrated / 34 BatchOnly / 2 Unclassified (`setLocale`, `setTerminology`) + a dead
`setActiveExampleStep`. End of wave: **39 / 0 / 0**, `setActiveExampleStep` deleted outright.

Ground truth is now a single 39-element set replicated across four lists; `bun ./📜️script.ts
publication-authority-audit` proves set-equality across all of them plus the fixture.

## 2. Lane table (what each verb actually emits, re-verified against its command file)

| Lanes | Routes | Why |
|---|---|---|
| `Artifact` | `addNode`, `forceLayout`, `redrawHandles` (B2), `reorganize` | isolated reducer / `Puzzle2dForceLayoutWork`: neither runs the scene pipeline, so no config snapshot is reachable |
| `Config` | `brushCancelSlot`, `brushCycleCandidate`, `brushOpenSlot`, `brushSetCandidateIndex`, `engagementAbort`, `engagementControlSelect`, `engagementInput`, `engagementSubmit`, `focusSelection`, `setBrushKindWeights`, `setBrushNodeSize`, `setCamera`, `setGridFactor`, `setGridSnapEnabled`, `setLocale`, `setLodModeForPane`, `setSuggestionOffset`, `setTerminology` | write `scene.runtime` and/or queue `brushPreview`/`brushCandidates` host events; none of them can write `scene.fixture` (verified: `brush_clear_slot`/`brush_rebuild_preview` emit candidates only; `brush_commit_preview` — the only `brushPlace` emitter — is reachable only from `brushCommitSlot`, since `brush_finish_slot` commits only under `brush_alt_pressed`, always `false` on the rebuilt host) |
| `Artifact, Config` | `applyBoardEvents`, `brushCommitSlot`, `deleteSelection`, `duplicateSelection`, `patchInspectorNodes`, `setActiveExample`, `setSelectionFlag`, + B2's `setFillCount`, `brushFillSession{Adopt,Begin,Cancel,Clear,Discard,Retry,Step}` | write `scene.fixture` *and* can produce a config snapshot: `sync_host_runtime_state` runs on a fresh `BoardHost` whose `brush_candidates_emit_key` is `None`, so the first `brush_sync_preview_events` always emits a `brushCandidates` event, which clears `config.brush_candidates`/`brush_candidate_index`/`brush_candidate_source_handle_id` whenever they were non-empty |
| `HostOnly` (solo) | `lodScaleJson`, `selectSameKind` | both `🎮️commands/*` bodies are empty by construction; they complete through `NoopPuzzleCommandWork` (`Emit::default()`), so no lane is ever touched |

**Two latent runtime faults fixed in passing** (both would have hard-failed the framework's
publication gate at `🧰️framework/…/🔌️plugin/🦀️.rs:22904-22948` the first time they ran):
- `setActiveExample` was declared `[Artifact]`, but `Puzzle2dActiveExampleWork`'s terminal step emits
  `config_mutations: vec![Puzzle2dConfigMutation::Snapshot { .. }]`. Now `[Artifact, Config]`.
- `applyBoardEvents` was declared `[Artifact]`, but `puzzle2d_board_events_reduce` emits a config
  snapshot for the `camera` and `brushCandidates` board events. Now `[Artifact, Config]`.

## 3. Extent budgets (verified against `PUZZLE_COMMAND_WORK_ITEMS = 4_096`, Nakagin = 180 nodes / 179 edges / 358 handles / 14 compat rows)

| Tool(s) | `extent()` | Nakagin value | Verdict |
|---|---|---|---|
| every `PUZZLE2D_GENERIC_TOOL_IDS` verb except the four selection-acting ones | constant `1` | 1 | safe, document-size-independent |
| `patchInspectorNodes`, `setSelectionFlag`, `deleteSelection`, `duplicateSelection` | `max(args.ids.len() [patchInspectorNodes only], selection.ids.len())`, `None` above `PUZZLE2D_SELECTION_BATCH_LIMIT = 1_024` | whole-board selection = 180+179+358 = **717** → admitted, 717 ≤ 1 024 ≤ 4 096 | safe with 4.2× headroom over Nakagin; an oversized selection is **refused at preflight**, never truncated |
| `lodScaleJson`, `selectSameKind` | `NoopPuzzleCommandWork` → `Some(1)` | 1 | safe |
| `setActiveExample` | unchanged (`src.N+src.E+src.C + tgt.N+tgt.E+tgt.C + 2`, capped at 4 096) | empty→Nakagin **375**; Nakagin→Nakagin **748** | safe |
| `applyBoardEvents` | unchanged (event count, ≤ 256) | ≤ 256 | safe |
| `reorganize` | shares `Puzzle2dForceLayoutWork::extent` (B2 owns its node/edge/handle gate) | whatever B2's gate admits | B2 owns |

`PUZZLE2D_SELECTION_BATCH_LIMIT = 1_024` was chosen (not 4 096) so the refusal happens in the
artifact's own vocabulary well below the shared budget, matching the way
`PUZZLE2D_BOARD_EVENT_BATCH_LIMIT` and `PUZZLE2D_FORCE_MAX_*` are written.

## 4. Every edit site

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
Line numbers are the file as it stands **after** this wave; B2 and B3 were editing concurrently, so
they will drift.

| Lines | Change |
|---|---|
| `:857` | deleted the `SetActiveExampleStep = "setActiveExampleStep"` command variant |
| `:1112-1151` | `PUZZLE2D_RETAINED_TOOL_IDS` 4 → 39 ids |
| `:1153-1185` | new `PUZZLE2D_GENERIC_TOOL_IDS` (23) and `PUZZLE2D_HOST_ONLY_TOOL_IDS` (2) route sets — the single source both `build_tool_job` and the generic reduce/extent gate on |
| `:1161` | `impl ToolJobFactory` → `impl semio_framework::ToolJobFactory` (the TS oracle's structural anchor) |
| `:1208` | `type Owner = EditorApp<…>` → `semio_framework_plugin::EditorApp<…>` (same reason) |
| `:1210-1251` | `PUBLICATION_CONTRACTS` 4 → 39 entries, grouped Artifact / Config / Artifact+Config / HostOnly |
| `:1253-1615` | **new** `//#region 📬️StorePreparation`: `PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES`, `Puzzle2dConfigStorePreparation(Factory)`, `puzzle2d_config_store_bounded_bytes`, `puzzle2d_config_store_mutation_bytes`, `puzzle2d_config_store_edit`, `Puzzle2dArtifactStorePreparation(Factory)`, `puzzle2d_artifact_store_edit` — mirrors `Puzzle3dConfig/ArtifactStorePreparationFactory` (3d `:6312-6698`) with 2d's types |
| `:1648-1740` | **new** `puzzle2d_dispatch_emit` — the ONE dispatch pipeline (scene+host rebuild → `🎮️commands/*` arm → host-event drain → document delta → config snapshot), extracted verbatim out of `handle` |
| `:1742-1757` | `puzzle2d_board_events_reduce` reduced to a guard + `puzzle2d_dispatch_emit` call (was a second copy of the pipeline) |
| `:1759-1786` | **new** `PUZZLE2D_SELECTION_BATCH_LIMIT`, `puzzle2d_generic_extent`, `puzzle2d_generic_reduce` |
| `:1788-1806` | **new** `PUZZLE2D_EXAMPLE_STAGE_STEPS` + `puzzle2d_active_example_emit` — drives `Puzzle2dActiveExampleWork` to its terminal emit for the batch path, so the example load has exactly one implementation |
| `:2637-2660` | **new** `ArtifactEditor` overrides: `build_document_store_owners`, `build_config_store_owners`, `build_config_store_one_item_preparation_factory`, `build_artifact_store_one_item_preparation_factory`, `build_document_store_disposer`, `build_config_store_disposer` (2d had none; 3d has all six) |
| `:3605-3626` | `handle` body: the legacy `setActiveExample`/`setActiveExampleStep` branches replaced by one `puzzle2d_active_example_emit` call; the ~80-line pipeline tail replaced by `Ok(puzzle2d_dispatch_emit(command, before, config, interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), doc.operation_optional().cloned()))`. `let before = doc.snapshot.0.clone();` is deliberately kept as the literal anchor `mounted_fill_dispatch_contract` searches for |
| `:3648-3688` | proofs `tools: [...]` 4 → 39 ids |
| `:3704-3712` | `build_tool_job`: new `"reorganize"`, `generic if PUZZLE2D_GENERIC_TOOL_IDS.contains(&generic)`, `host_only if PUZZLE2D_HOST_ONLY_TOOL_IDS.contains(&host_only)` arms, all **above** the `_ =>` fallthrough |
| `:3905-3909` | **new** `.view_action("setLocale", …)` / `.view_action("setTerminology", …)` with EN+DE inline `LocalizedLabel::native` (mirrors 3d `:7217-7218`); removed the `set_active_example::STEP_ACTION_ID` `.action_with(...)` declaration |
| `:3951-3989` | `.action_interactive_job` block rewritten: 39 ids, all `Migrated`, alphabetical; `setActiveExampleStep` gone |
| `:4118-4125` | testkit: `finish_example_load` deleted (nothing left to pump); `load_example` is now one dispatch and asserts no continuation effect is requested |
| `:4243-4245` | `mounted_fill_dispatch_contract`: anchors `"async fn handle("` → `"    fn handle("` and `"async fn pending_effects"` → `"fn pending_effects"`. **This test was already red at HEAD** — the repo-wide async→sync sweep desugared `ArtifactEditor`'s methods and left the string anchors behind; `production.find("async fn handle(")` returned `None`, so the assertion could never pass |
| `:4306-4326` | the two example tests rewritten: `set_active_example_loads_concrete_forest_via_operations` now asserts one dispatch commits operations and requests no effect; `newer_example_load_supersedes_a_stale_continuation` (a test of the deleted generation-chain) replaced by `a_newer_example_load_replaces_the_previous_document` |

### `…/✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs`
Cut from 167 to 33 lines. Removed `STEP_ACTION_ID`, `MAX_MUTATIONS_PER_STEP`, the eight `STAGE_*`
consts, `queue`, `step_emit`, `begin_active_example`, `step_active_example` and every import they
needed. Kept `warm_examples`, `canonical_example_id`, `target` and the three `LazyLock` example
snapshots — the only things `Puzzle2dActiveExampleWork` reads.

### `✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json`
`Puzzle2dPlayApp` owner rewritten: 1 stale `BatchOnlyPendingRewrite` group (3 routes) → 4 `Migrated`
groups, 39 routes, `HostOnly` solo, no blockers.

### `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`
- `ownerOracle`: deleted the puzzle2d "must have NO proofs macro / NO retained factory" branch
  (`:85-89`), which had been false since before this ticket opened.
- `const factory` is now derived uniformly (`` `${owner.owner.slice(0, 8)}RetainedCommandJobFactory` ``),
  so all three owners run the same structural block.
- the old "2d/3d must NOT contain `build_artifact_store_one_item_preparation_factory`" clause became
  a positive requirement that **every** owner declares both
  `fn build_artifact_store_one_item_preparation_factory()` and
  `fn build_config_store_one_item_preparation_factory()`.
- new `Puzzle2dPlayApp` structural branch, modelled on the 3d one: both preparation factory types,
  both `ArtifactStoreOneItemPreparationFactory`/`ArtifactStoreOneItemPreparation` impls, both
  `Some(std::sync::Arc::new(…))` wirings, the fixed `PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES` envelope,
  the stale-authority triple (`operation`/`generation`/`base_revision`), `Progress` before
  `Prepared`, both checkpoint cursors, `cancel`/`begin_close`/`return_to_registry`/`terminal_is_empty`,
  **plus** three anchors specific to this wave's design: `fn puzzle2d_dispatch_emit(` exists and both
  the retained reduce and `handle` call it (so a second divergent copy of the pipeline is refused),
  and the `PUZZLE2D_SELECTION_BATCH_LIMIT` ceiling with its `None`-above-bound expression.
- the now-dead `if (owner.owner !== "Puzzle5dPlayApp" && production.includes("build_config_store_one_item_preparation_factory")) return false;` line removed.
- `hostileFixtures[2]` hardened: it forced `status: "Migrated"` while carrying over
  `owner.groups[0].blocker`, which is `undefined` once the first group is itself `Migrated` — the
  "hostile" fixture was then identical to the real one and would have made the audit throw. It now
  injects a blocker unconditionally, so it stays a real violation of "a `Migrated` group carries no
  blocker" for any owner shape.

### `…/✳️any/🗄️retained-jobs/🔣️.json`
`"toolIds"` was `[]`; set to the 39 ids **in `PUZZLE2D_RETAINED_TOOL_IDS` order** — the shared oracle
test asserts `assert_eq!(actual.tool_ids, expected.tool_ids)` against that const (`🎮️commands/🧵️retained/🦀️.rs:996-1001`),
and a `Vec` comparison is order-sensitive. No vectors added (see §7).

---

## 5. Verification actually run

- `bun ./📜️script.ts publication-authority-audit Puzzle2dPlayApp` → **exit 0**, 39 admitted routes.
- `bun ./📜️script.ts publication-authority-audit` (all three owners) → **exit 0**; 3d and 5d still pass
  unchanged under the refactored `ownerOracle`.
- Both runs execute the hostile self-checks; exit 0 means every hostile mutation was still rejected.
  Independently confirmed that the `missingContract` hostile regex now *does* match puzzle2d source
  (it strips `ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[…Config] },`) — so
  puzzle2d is genuinely covered by it, closing the gap `📓️explore-editor-actions.md` §5.2 flagged.
- `rustfmt --config-path rustfmt.toml --edition 2021 --check` on both edited Rust files: the
  set-active-example command file is clean; the editor file's only remaining diffs are at `:30`,
  `:246`, `:261` (pre-existing at HEAD), `:1853`/`:1942` (inside `Puzzle2dActiveExampleWork`,
  pre-existing) and `:3731` (B3's `io()`). **Every hunk this wave wrote is rustfmt-clean.**
- rustfmt parsing both files successfully is also the only syntax check available without cargo.

## 6. What B2 and B3 must know

**B2** — I registered your nine ids in all four lists and flipped them to `Migrated`, but I wrote
**no `build_tool_job` arms** for them:

| id | lanes I declared | your arm must go **below** my `generic`/`host_only` guard arms and above `_ =>` |
|---|---|---|
| `setFillCount` | `Artifact, Config` | yes |
| `brushFillSessionAdopt/Begin/Cancel/Clear/Discard/Retry/Step` | `Artifact, Config` | yes |
| `redrawHandles` | `Artifact` | yes |

Until those arms exist, dispatching any of the nine reaches `Err("puzzle2d-command-tool-unmapped")` —
they are in `PUZZLE2D_RETAINED_TOOL_IDS`, so `build_tool_job` no longer returns `Ok(None)` for them.
This is the one place where B1 landing without B2 is a live regression; it is the reason the fixture
now carries them as `Migrated`.

Two more things for B2:
- I added `"reorganize" => Box::new(Puzzle2dForceLayoutWork::new("reorganize"))`. Your `tool_id`
  field + `new(tool_id)` constructor (which you had already landed) is what makes this legal — the
  retained job's decode phase compares the wire action id against `PuzzleCommandWork::tool_id`, so
  please keep `tool_id` a field and keep `extent()` comparing against `self.tool_id` rather than the
  literal `"forceLayout"`.
- If `redrawHandles` gets a bespoke chunked `Work`, its lane stays `Artifact` only (it rewrites
  `scene.fixture` through `apply_edge_handle_snap_to_fixture_v1_json` and touches no runtime field) —
  but if you route it through `puzzle2d_generic_reduce` instead, add it to `PUZZLE2D_GENERIC_TOOL_IDS`
  **and** widen its fixture/contract lanes to `Artifact, Config` for the brush-candidate-clearing
  reason in §2.

**B3** — I did not touch `🗣️terminology/🦀️.rs`. `setLocale`/`setTerminology` now have real
`ActionDefinition`s via `.view_action(...)` with inline `LocalizedLabel::native(en, de)`, exactly as
every other action in this file does, so nothing here depends on your new `puzzle2d_localized_phrase`
API. If you want them to read from `Puzzle2dLabels` instead, those two lines (`:3908-3909`) are the
only place to change. Your `Puzzle2dImportJobFactory`'s `import-media` contract at `:2952` is
untouched and is correctly *not* in the fixture (reserved routes carry no `.action_interactive_job`).

## 7. What the main session must compile-verify / decide first

1. **Compile the new store-preparation region.** It is a near-verbatim port of 3d's; the two places
   it is not verbatim are `puzzle2d_config_store_mutation_bytes` (2d's `Puzzle2dConfigMutation` has
   only `Snapshot`/`Fill`, so both are admitted, each bounded by its encoded length) and
   `PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES = 65_536` (2d's config carries `brush_candidates:
   Vec<dsl::DslValue>`, unbounded in the type; 3d's 32 KiB would be tighter than that field warrants).
2. **The six new `ArtifactEditor` store overrides** rely on
   `Puzzle2dPlaySnapshot`/`Puzzle2dConfig`/`Puzzle2dMutation`/`Puzzle2dConfigMutation` satisfying
   `Clone + protocol::ToValue + protocol::FromValue + ArtifactPack (+ store::Mutation)`. All four
   derive `Clone, ToValue, FromValue` (checked), and the `ArtifactApp` bounds already require
   `ArtifactPack` — but this is the single most likely trait-bound failure in the wave. If
   `bounded_document_store_owners`/`bounded_config_store_owners` or either disposer fails to
   type-check, **delete just those four overrides** (`build_document_store_owners`,
   `build_config_store_owners`, `build_document_store_disposer`, `build_config_store_disposer`);
   the two *preparation factory* overrides are the ones the lane gate actually needs and they have
   weaker bounds.
3. **Two pre-existing red tests this wave did not create** (both discovered while working):
   - `mounted_fill_dispatch_whole_artifact_and_config_mutations_are_rejected` was already failing at
     HEAD because it searched for `"async fn handle("` / `"async fn pending_effects"` after the
     async→sync sweep. **Fixed here.**
   - `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`
     (`🎮️commands/🧵️retained/🦀️.rs:996`) asserts each dimension's fixture `toolIds` equals its
     `PUZZLE*_RETAINED_TOOL_IDS`. 2d's is now correct (39, in order). **3d's is still wrong** —
     `🧊️3d/…/🗄️retained-jobs/🔣️.json` has 4 `toolIds` against a 62-id const, so this test stays red
     until someone syncs the 3d fixture. Not routed to any B-wave; needs an owner.
4. **`describe` output is stale and still advertises `setActiveExampleStep`**
   (`✏️s/🔌️plugins/🧩️puzzle/🔣️.json:937`, `:4414`, `:7888`). Regenerating it is wave D/A2's step; the
   action no longer exists in source.
5. **Dead config field left in place on purpose**: `Puzzle2dConfig::example_load_id`
   (`…/✏️editor/🎚️config/🦀️.rs:253`, mirrored in `🎚️config/🧬️schema/🦀️.rs:84`, `🧬️schema/🟦️.ts`,
   `🧬️schema/🔣️.json`) was written only by the deleted `begin_active_example` and read only by the
   deleted `step_active_example`. It is now write-never/read-never. Removing it is a 4-file
   schema-surface change in `🎚️config`, which B2 is actively editing (fill runtime lives in the same
   struct), so I did not touch it. Route it after B2 lands. `example_load_generation` is still live
   (the Work's coalesce key).
6. **`selectSameKind` / `lodScaleJson` decision, with the evidence.** I did **not** delete them.
   - `selectSameKind` is referenced by production UI: `puzzle2d_context_menu_items` emits a
     `selectSameKind` menu item (editor `:1029`) inside the `"selection"` group, and it is a
     palette-visible `ActionKind::View` action. Deleting it would remove a user-facing context-menu
     entry. Its handler is empty only because selection is framework-owned and `handle` has no
     channel to write it back — the *same* limitation puzzle3d documents.
   - `lodScaleJson` has no dispatcher anywhere: a repo-wide grep for `lodScaleJson` outside `🧊️3d`/`🖐️5d`
     found only unrelated engine/session methods (`dag_lod_scale_json`, the `BoardSession`/`DagSession`
     stubs, tiled-map, flow bindings) and never a puzzle2d action dispatch; the chrome reads the LOD
     table at render time through `🎭️modes/✏️edit/☑️options/🔭️lod`. It is genuinely dead as an action.
     I still kept it because deleting it means editing the **shared, contended** crate root
     (`📦️packages/🦀️rust/🦀️.rs:2292-2293` `#[path]` mount) plus `🔣️taxonomy.json` member names plus a
     directory removal — precisely the class of change the memory note *Artifact Name Registry Gate*
     says to do as its own measured step, and A1 already has a routed fix in that same file.
   - Both are therefore migrated as **`Migrated` / `HostOnly`** through
     `crate::retained_command::NoopPuzzleCommandWork`, which is faithful: both legacy handlers emit
     nothing, so the retained completion (`Emit::default()`) reproduces the legacy path exactly.
     This is strictly better than the status quo, where `BatchOnlyPendingRewrite` made the context
     menu's `selectSameKind` fail `validate_ui_dispatch_classification` outright.
     One cosmetic delta: `lodScaleJson`'s legacy path set `UiDirtyScope::None` explicitly and
     `selectSameKind`'s left it `Full`; `Emit::default()` now decides the scope for both. Neither
     emits a mutation, so no client can desync.
   - **Recommended follow-up ticket**: delete `lodScaleJson` (action + `LodScaleJson` variant +
     `📊️lod-scale-json` module + crate-root mount + taxonomy entry) once A1's crate-root edit lands.
7. **The one behavioural change worth a runtime look in wave D**: `setActiveExample` on the batch
   path now commits its whole mutation set in a single `Emit` instead of a `MAX_MUTATIONS_PER_STEP =
   4` chain of `Effect::DispatchAction`. The retained (production/UI) path is unchanged — it always
   went through `Puzzle2dActiveExampleWork` and was already chunked one mutation per `step()`. For
   Nakagin the batch path now produces ~750 mutations in one emit; if that trips a batch-path
   ceiling anywhere, the fix is to bound `puzzle2d_active_example_emit`, not to resurrect the
   `Effect::DispatchAction` ladder.
8. **Retained-job fixture vectors**: `evidenceToolIds` still lists only `addNode`, `forceLayout`,
   `setActiveExample`, and `semanticCursors` covers the same three. Nothing forced an addition (the
   oracle only requires every vector's `toolId`/`toolIds` to be inside `evidenceToolIds`, and no new
   vector was added), but the 36 newly-retained verbs now have zero fixture evidence. Whoever owns
   fixture coverage (A1) may want a `boundedDecode → dispatch → publish → closeOwner` cursor and a
   vector for one generic verb and one `HostOnly` verb. Separately, `forceLayoutMax`/`forceLayoutMaxPlusOne`
   in that fixture still assert a 512/513-node boundary while `PUZZLE2D_FORCE_MAX_NODES` is 64 —
   that is B2's gate to reconcile.
