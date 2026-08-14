# W3c — Surface/Render Layer Off Push-Setters, Onto Framework Interaction State

## What changed (framework layer only, as scoped)

### `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️component.rs`
- Deleted `GraphHoverRecord`, `NodeGraphScenePayload.selection`/`.hover` (and their `from_json`/
  `payload_signature`/`sync_from_payload` wiring).
- Deleted `GraphHost::set_hover`/`set_hover_channel` (push-setters) and the wasm `setHover`/
  `setHoverChannel` bindings.
- Added `GraphHost::sync_interaction(selection: Option<&DomainSelection>, hover: Option<&DomainHover>)`
  — reads the framework's resolved selection/hover for this domain and applies it to the DAG paint
  backend (`self.dag.set_selection`/`set_hover`, which stay — they are the internal paint-state API,
  not the deleted push-setters). Added wasm `syncInteraction(selectedIdsJson, hoveredId)`.
- Marquee/lasso: `pointer_up_screen` now captures the DAG engine's own geometric hit-test result (plain
  pick vs. rectangle/lasso, detected via `preselect_widget_ids()`/`selection_preview_method()`) into a
  new `SelectionGather{target_ids, method}`, readable once via `GraphHost::take_selection_gather()`
  (wasm `takeSelectionGatherJson`). No merge/mode algebra added here — the caller pairs the gather with
  the modifier→merge policy and dispatches ONE `interactionSelect`; `next_selection` (os-kernel) is the
  only place merge algebra runs. Routed the wasm `pointerUpScreen` binding through the `GraphHost`
  wrapper (it previously called `host.dag.pointer_up_screen` directly, bypassing gather capture).
- Extended in-file tests: `graph_host_pointer_up_after_plain_click_gathers_one_pick_target`,
  `graph_host_sync_interaction_sets_hover_node_only`/`_clears_hover_when_absent`,
  `graph_host_syncs_selection_from_framework_interaction_state`, plus updated every test that used to
  set `payload.selection`/`.hover` to call `sync_interaction` instead.

### `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️component.rs`
- Deleted `RasterHost::set_hovered_id`/`set_selection_ids_json` and their wasm bindings
  (`setHoveredIdSilent`, `setSelectionIdsJson`).
- Added `RasterHost::sync_interaction(selected_ids: &[String], hovered_id: Option<&str>)` + wasm
  `syncInteraction(selectedIdsJson, hoveredId)`. `hover_stroke`/`selection_stroke` chrome
  (`build_vector_scene`'s selection outline) untouched — only the state source changed.
- Extended tests: replaced the two setter-specific tests with
  `sync_interaction_updates_hovered_and_selected_state`; every test that called the old setters to seed
  a selection now calls `sync_interaction`.

### `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs`
- Deleted `MapHost::set_selection_json`/`set_hover_json` and wasm `setSelectionJson`/`setHoverJson`.
- Added `MapHost::sync_interaction(granularity: &str, selected_ids: &[String], hovered_id: Option<&str>)`
  + wasm `syncInteraction(granularity, selectedIdsJson, hoveredId)`. The framework's `DomainSelection`/
  `DomainHover` carry one flat id list scoped to a single active granularity (unlike the old JSON shape,
  which could carry `positions`+`routes` simultaneously) — `granularity` (`"position"`|`"route"`, the
  domain's `active_granularity`) picks which of `selected_positions`/`selected_routes` the ids populate,
  clearing the other. `hovered_kind`/`hovered_id` fields kept, now sourced the same way.
- Extended/renamed tests accordingly (`sync_interaction_updates_host_state_scoped_to_active_granularity`,
  `sync_interaction_replaces_previous_selection_within_active_granularity`,
  `sync_interaction_none_clears_hover_and_some_sets_kind_and_id`).

### `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs`
- Added `world_interaction_definition()` — the OS `world` `InteractionDefinition`: granularities
  `surface`/`item`, `HierarchyProvider::PathDelimited{delimiter:"/"}` over `"{surfaceId}/{id}"`,
  `SelectionMethod::{Pick,Rectangle}`, `MergeMode::{Replace,Additive,Invertive}`. Not wired onto any
  `AppDefinition` yet — that's per-app (wave 4); this is the one framework-owned declaration every world3d
  app reuses, per the task's "declare the OS world domain" instruction.
- Replaced the ad-hoc `worldSelect`/`worldHover` command strings with the framework verbs
  `interactionSelect{domainId,targets,merge,method}` / `interactionHover{domainId,channel,targets}` at
  all three emission sites: `pick_hover_action`'s fallback, `pick_select_action`'s fallback, and
  `marquee_select_action`'s non-component branch. `apply_world_action_preview` (optimistic local preview)
  now parses the new shape for `domainId == "world"`, stripping this state's own `surfaceId/` prefix
  back off item target ids. `worldPick`/`setSelection`/`setHover` (component-level vertex/edge/face
  picking — a separate mechanism) are untouched, per the task's named scope.
- Merge vocabulary: `pick_select_action`/`marquee_select_action` now compute the canonical `MergeMode`
  wire labels (`"replace"|"additive"|"invertive"`) directly instead of the old ad-hoc `"add"|"toggle"`;
  `merge_string_ids`/`merge_u32_ids` (shared with the untouched `worldPick` path) accept both — `"add"`/
  `"additive"` are now synonyms, so `worldPick`'s own emissions (still `"add"`/`"toggle"`) keep working
  unchanged.
- Marquee stays geometric: `screen_select_instances` (the surface's own hit-test) is unchanged;
  `marquee_select_action` just batches its raw hits into ONE `interactionSelect` with `method:"rectangle"`
  — no selection algebra added.
- Extended tests (new `WorldInteractionVerbs` region): `world_interaction_definition_declares_...`,
  `pick_select_emits_batched_interaction_select_for_plain_object_pick`,
  `marquee_select_emits_batched_interaction_select_with_rectangle_method`,
  `pick_hover_emits_interaction_hover_and_clears_when_nothing_hit`,
  `apply_world_action_preview_applies_interaction_select_and_hover_for_world_domain`.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- **No functional change** — investigated and confirmed not needed for this wave's DO items:
  - Zero direct calls to any of the deleted push-setters (`set_hovered_id`/`set_selection_ids_json`/
    `set_hover`/`set_hover_channel`/`set_selection_json`/`set_hover_json`) anywhere in this file — the
    actual `GraphHost`/`RasterHost`/`MapHost` wiring lives in `EngineCanvas`/`Scenes`/`Interpreter`
    (none of which are in this task's file list).
  - `handle_world3d_input` (Shell's own world3d pointer routing) forwards whatever `ActionDescriptor`
    `♾️infinite`'s handlers return straight to `self.dispatch_action(action).await?` — a generic,
    name-agnostic dispatch. Renaming the emitted action from `worldSelect`/`worldHover` to
    `interactionSelect`/`interactionHover` needed zero change here.
  - The one plausible Shell-side task — wiring the active domain's `DomainSelection` into
    `open_context_menu`'s `selection` field (flagged by W3a's own comment at the call site, "a
    follow-up wires the active domain's DomainSelection through here") — was left alone: `ShellState`
    has no `InteractionState`/session-config plumbing to read from yet, W3a explicitly deferred it as a
    separate follow-up, and it is not one of this task's five DO items. Attempting it without that
    plumbing would be speculative, not a bounded fix.

## Known breaks outside this task's file scope (for the next wave, not fixed here)

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs`** — a byte-identical
   duplicate of `♾️infinite/🦀️component.rs` (both mounted via `#[path=...]` glob-`pub use`d into the
   `semio-framework-os-infinite` crate root, per `📦️glue.rs`; both landed together in commit
   `1cf6018596`, unrelated to this ticket). It still has the OLD `worldSelect`/`worldHover` strings and
   is now divergent from the file I edited. Not in this task's file list — flagging rather than editing
   a file outside the named scope, and because "two files with identical content, one glob-`pub use`d
   over the other" looks like a pre-existing repo inconsistency worth the coordinator's attention on its
   own, independent of this ticket.
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs:1150,1153`**
   — calls `host.set_selection_json(&scene.selection_json)` / `host.set_hover_json(&scene.hover_json)`
   directly on a `framework_surface_tiled_map::MapHost`, both now-deleted methods. Not in this task's
   file list. Could not be verified with `cargo check -p semio-framework-os-renderer-wgpu` either
   before or after this change, because that crate hard-depends on `semio-s-plugin-puzzle`
   (pre-existing wave-1 breakage, `interactions: vec![]` missing-field errors — see W3a's own
   "renderer-wgpu informational check"), which aborts the build before EngineCanvas's own files are
   ever reached. The break is real regardless of whether `cargo check` can currently observe it — the
   fix is `EngineCanvas::sync_tiled_map_from_scene`'s `sync_field` block calling
   `host.sync_interaction(granularity, &ids, hovered_id)` instead, sourcing `granularity`/`ids`/
   `hovered_id` from the framework's `InteractionState` for the map's domain (not from
   `scene.selection_json`/`scene.hover_json`, which are the same kind of app-pushed opaque JSON this
   ticket exists to remove).
3. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx:4102,4519`**
   — two `dispatch("worldSelect", {...})` calls (marquee-drag preview commit), a separate TS-side path
   independent of the Rust `ActionDescriptor`s this wave converted. Not in this task's file list. Given
   wave 1 already deleted all per-app selection/hover View actions from the manifest (master doc: "All
   per-app selection/hover View actions ... are deleted"), `worldSelect` may already be an unroutable
   action name post-wave-1, in which case this is pre-existing breakage this ticket surfaced rather than
   caused — worth the next wave confirming either way.

## Acceptance (real output, saved alongside this file)

- `w3c-cargo-test-semio-framework.txt` — `cargo test -p semio-framework`: **105 passed, 0 failed**
  (unchanged from W3a's baseline; this task made zero edits inside `semio-framework` itself).
- `w3c-cargo-check-semio-framework-os-kernel.txt` — `cargo check -p semio-framework-os-kernel`:
  **0 errors**.
- `w3c-cargo-test-semio-framework-surface.txt` — `cargo test -p semio-framework-surface`: node-graph
  16 passed / 24 failed, paint 57 passed / 0 failed, tiled-map 97 passed / 1 ignored / 0 failed. The
  24 node-graph failures are ALL the identical pre-existing panic
  `bundled dag demo DSL is valid DagSnapshot text: TextError{...}` from `DagFixture::default()`
  (`♾️infinite/…/🕸️dag/🦀️component.rs:1900`, `include_str!`-ing a plugin demo fixture under
  `✏️s/🔌️plugins/🕸️dag/…` whose DSL no longer parses) — pre-existing, unrelated to this wave's edits
  (confirmed: `set_canvas_theme_dark_applies_board_palette`, a test I never touched, fails with the
  exact same panic; every node-graph test that does NOT call `GraphHost::default()` passes). Not one of
  this task's required-green crates.
- `w3c-cargo-check-semio-framework-os-infinite.txt` — `cargo check -p semio-framework-os-infinite`
  (lib target): **0 errors**. `cargo test --lib` for this crate currently fails to even compile, but
  for two causes fully independent of this wave, both proven pre-existing by reproducing them in the
  byte-identical, untouched `🌍️world/🦀️component.rs` duplicate: (a) a missing bundled asset
  (`🧊️capsule_J.glb`), and (b) `DslValue` no longer implementing the indexing operator used by dozens of
  this file's PRE-EXISTING tests (e.g. `pick_select_emits_numeric_world_pick_id`, untouched by this
  wave) — see `w3c-infinite-preexisting-dslvalue-index-break.txt`. This is the concurrent
  `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` ticket's in-progress refactor
  (its scratch/target files were present in `git status` at the start of this task), not anything this
  wave introduced. My five new `WorldInteractionVerbs` tests follow the exact same (currently-broken)
  `args["key"]` idiom as the neighboring pre-existing tests, so they are logic-complete but cannot be
  confirmed to pass by running them until that unrelated regression resolves.
- `cargo check -p semio-framework-os-renderer-wgpu` (informational, not required): **54 errors, every
  one inside `✏️s/🔌️plugins/🧩️puzzle/**` or `🌊️flow/📖️playbook/component.rs`** — identical shape/location
  to W3a's own "renderer-wgpu informational check" baseline; zero errors traceable to any file this
  task touched (grepped every `--> path` in the output).

## Files touched
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️component.rs`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️component.rs`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` (investigated, no change needed)
