# terra-sdk-wire — M1 (UiIntent dispatch) + M2 (presence) — report

Packet: `sdk-wire`. Scope: `🔌️plugin/**` (root `🦀️component.rs`, `⚛️reactor/**`) + `kernel::TurnResult`
wherever it lives (found at `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`, not the `💻️os/`-prefixed
path named in the brief — same file, path drift only).

## M1 — UiIntent dispatch, hop by hop

1. **Revision guard.** `PatchTracker::revision(&self, surface: &str) -> ui_contract::UiRevision`
   added at `⚛️reactor/🩹️patches/🦀️component.rs` (new method on the existing `impl PatchTracker`).
   Reads through `SurfaceReconciler::snapshot().revision` — the only accessor the landed
   `ui_runtime` API exposes for this (matching `ui_runtime`'s own `SurfaceSlot::current_revision`
   idiom at `🦀️transaction.rs:82-88`, which flags the same cheaper-accessor gap as a registrar
   item). A never-observed surface reads revision 0.
2. **Reactor batching.** `⚛️reactor/🦀️component.rs`: `Event::UiIntent` now decodes the intent,
   calls `PATCHES.with(|p| p.revision(&intent.surface.0))`, and drops it via
   `ui_runtime::is_stale_intent(intent.revision, current, DEFAULT_REVISION_TOLERANCE)` (imported,
   never reimplemented) instead of unconditionally marking dirty. Survivors batch per instance into
   a new `dirty_intents: HashMap<u32, Vec<ui_contract::UiIntent>>`, mirroring `app_commands`.
   Immediately after the `app_commands` dispatch loop, a new loop calls
   `plugin_runtime::plugin_dispatch_intents(instance, &intents)`, routes `output.frames` through the
   SAME `route_app_frame`, decodes `output.effects`/`output.events` identically to the
   `app_commands` loop, then pushes each batch's unique surfaces into `dirty_render` so the reply
   patch is produced in the same turn.
3. **SDK entry point.** `plugin_runtime::plugin_dispatch_intents(instance_id, intents) ->
   Result<PluginExchangeOutput, Fault>` (`🔌️plugin/🦀️component.rs`, next to `plugin_resume_task`).
   Per intent: `instance.app.handle_intent_frame(intent, &meta)` via `with_instances_mut`/
   `find_instance`/`resolve_ready`, `ActionMeta { actor: instance_actor(instance_id).await,
   instance_id }`, frames exactly like `plugin_resume_task` (`AppFrame::Emit`/`Error`,
   `in_reply_to: 0`). `plugin_take_presence(instance_id) -> Vec<ui_contract::PresenceUpdate>` added
   alongside it (M2, see below).
4. **`PluginApp::handle_intent_frame`** — new trait method (object-safe boundary,
   `#[dyn_enum]`-closed, so every fleet plugin's app-enum picks it up automatically) + its one
   implementation on `VcsArtifactApp<A, M>`: resolves `A::command_from_intent(intent)` then calls
   the EXISTING `dispatch_typed_command_inner` — the same body `dispatch_typed_command`/
   `dispatch_typed` already use, so kind discipline, the command log, undo grouping and `AsyncTask`
   spawning apply with no parallel path.
5. **`ArtifactApp::command_from_intent`** default (+ mirrored on `ArtifactEditor`, + `EditorApp<E>`
   delegation): rejects `intent.action.version != 1` with a `Fault` (`app.intent.version-mismatch`
   — the default bridge only ever resolves v1, matching `ActionFactory`'s sole `ActionId::v1`
   constructor), then calls `Self::command_from_action(&intent.action.name,
   merge_ui_values(intent.args, intent.input).as_ref())`. `ViewerApp<V>`/every hand-written
   `ArtifactApp` gets this automatically via the trait default (compatible — no existing plugin
   changes).
6. **The fold.** `ui_value_to_json`/`merge_ui_values` added beside `dsl_value_to_ui_value`
   (`🔖️ActionFactory` region, `🦀️component.rs`): `input` wins on key collision when both sides are
   JSON objects; a scalar `input` replaces `args` wholesale; `None` when neither is set.

## M2 — presence derivation, hop by hop

1. **Outbox.** `VcsArtifactApp<A, M>.pending_presence: Vec<ui_contract::PresenceUpdate>` field +
   `PluginApp::take_pending_presence()` trait method (object-safe drain, mirrors
   `take_last_emit_wire`).
2. **Derivation.** `stamp_and_cache_interaction_ui`/`_node` (previously `let _ = state;`) now walks
   every node under an `interaction_domain`-bound `Component::Tree` and, for a node with own
   selection/hover (`state.selection`/`self.interaction_hover`) or peer marks
   (`InteractionView::peers_selecting`/`peers_hovering`), pushes one `PresenceUpdate` onto the
   outbox. `render()` now threads `body_key` through (new 3rd param) since `VcsArtifactApp` never
   knows its own instance id.
3. **Drain + stamp.** `plugin_runtime::plugin_take_presence(instance_id)` drains the outbox and
   rewrites each update's bare `body_key` surface to `"<instance>:<body-key>"` — the SAME split
   `wit_event_to_kernel`/`parse_surface_instance` already use — so it keys identically to `PATCHES`.
4. **Reactor hub.** `PRESENCE: RefCell<ui_runtime::PresenceHub>` thread_local beside `PATCHES`
   (ungated — `PresenceHub` is standalone, needs no `EntityStore`/`UiRuntime`, exactly as the design
   promised; API consumed as-is from disk: `new()`, `record_own(surface, node_key, own, ttl_ms)`,
   `record_peer(surface, node_key, mark, ttl_ms, now_ms)`, `expire(now_ms)`, `flush() ->
   Vec<PresenceUpdate>` — matched the design doc exactly, no drift). After each dirty render,
   `plugin_take_presence` drains into the hub; once per `poll`, `hub.expire(now_ms)` then
   `result.presence = hub.flush()`. `now_ms` read once per turn via `crate::host::now_ms()`.
5. **`kernel::TurnResult`** gains `presence: Vec<ui_contract::PresenceUpdate>`
   (`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`, `#[serde(default, skip_serializing_if =
   "Vec::is_empty")]`). `kernel_turn_result_to_wit` marshals it via a new
   `kernel_presence_update_to_wit` using the SAME `pack_patch_field` helper every `patch-op` variant
   uses — reusing the CURRENT wit `presence-update.peer: pack` field verbatim (see WIT lease-request
   below: the field is already opaque `pack` bytes, so no structural schema change was needed to
   repoint what it carries, only its doc comment is stale).

**Invariant verified**: presence and patches share zero code path — `stamp_and_cache_interaction_ui`
writes only `pending_presence`; `PATCHES`/`PENDING_PATCHES` are untouched by it. Test
`a_presence_only_turn_emits_presence_and_zero_patches` asserts this directly (see below).

## WIT lease-request (verbatim — please apply)

File: `🔌️plugin/🧬️schema/📜️component.wit`, `interface reactor`, the `presence-update` record
(currently ~line 918-923). Doc-comment-only change — the field itself (`peer: pack`) is unchanged,
so no Rust follow-up is needed on my side once applied:

```wit
  /// 👥️ M2 (ticket 26/08/17 design-unified.md): one RENDER-PLANE presence update, as of this turn
  /// — pack-encoded `semio-framework-ui-contract::PresenceUpdate` (surface + node-key addressed,
  /// TTL-scoped), NOT a replication `PresencePeer` (supersedes the `wit-flip` note this replaces:
  /// the turn-result's consumer is the renderer, which needs node-key addressing + TTL; the
  /// collaboration roster already has its own channel via `PresencePeer`/`adopt-presence`).
  record presence-update {
    peer: pack,
  }
```

Optional nice-to-have, NOT required for correctness: rename `peer` to `update` for clarity now that
it no longer carries a replication peer. If you take it, my one call site
(`kernel_presence_update_to_wit` in `⚛️reactor/🦀️component.rs`) needs the matching one-word rename
— tell me and I'll make it in my next turn.

## M2 fallout fixed (sibling `scene-surface` relocation, reported live by sol)

`scene-surface` moved 15 product scene structs (incl. `TableScene`, `World3dScene`) + helpers
(`world3d_default_selection_json`) from `ui_wgpu::wgpu` into new crate `semio-framework-ui-scene`.
Fixed in my path scope:
- `🔌️plugin/📦️packages/🦀️rust/Cargo.toml`: added `semio_framework_ui_scene` path dependency
  (package `semio-framework-ui-scene`) — this crate's own Cargo.toml, not registrar-only.
- `🦀️component.rs:20180` (now shifted): split the `ui_wgpu::wgpu::{world3d_camera_json,
  world3d_default_selection_json, ..., World3dScene}` import — `World3dScene`/
  `world3d_default_selection_json` now come from `semio_framework_ui_scene`;
  `world3d_camera_json`/`ActionDescriptor`/`MeasureSelectItem`/`WindowMeasure` stayed in
  `ui_wgpu::wgpu`.
- Two `TableWindowKit::render` sites (`ui_wgpu::wgpu::TableScene::base(...).await` →
  `semio_framework_ui_scene::TableScene::base(...)`, no `.await` — `TableScene::base` is E6
  sync-by-decree in its new home; the OUTER `ui_wgpu::wgpu::build_table_scene(...)` stays async and
  keeps its `.await`).

Verified: `world3d_default_selection_json()`'s OWN call site (`default_world3d_selection`) already
had no stray `.await` — not a 4th error, matches sol's count of 3.

## Tests added

- `⚛️reactor/🩹️patches/🦀️component.rs`: `revision_reads_zero_for_a_never_observed_surface_and_tracks_diff_afterwards`.
- `⚛️reactor/🦀️component.rs`, new `test_support` hooks (`patches_revision`, `patches_diff`,
  `presence_record_own`, `presence_record_peer`, `presence_expire_and_flush`) + new
  `m1_m2_reactor_tests` module (native — `poll`/`wit_bridge` is wasm32-wasip2-only and cannot run
  under `cargo test`, per this file's own `test_support` doc; these hooks exercise the SAME
  `PATCHES`/`PRESENCE` thread-locals `poll` touches):
  - `revision_guard_never_rejects_an_intent_at_the_never_rendered_default`
  - `revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance` (the "trails by 2 ⇒ no
    patch, no command" acceptance criterion, reduced to the guard predicate `poll` evaluates)
  - `a_presence_only_turn_emits_presence_and_zero_patches`
  - `a_burst_of_same_key_presence_writes_between_polls_coalesces_to_one_update`
  - `ttl_expiry_drops_a_peer_mark_with_no_goodbye_message`
- `🦀️component.rs`, `merge_ui_values_tests` module (6 unit tests: every `UiValue` shape folds
  correctly; none/single-side/object-merge-input-wins/scalar-replace).
- `🦀️component.rs`, `plugin_builder_contract_tests` (extended `ui_tree_stamping_...` fixture +
  3 new tests):
  - `ui_tree_stamping_caches_interaction_topology_from_a_domain_bound_tree` — extended to assert the
    derived `PresenceUpdate` (own.selected, one peer mark, own color threaded, bare-body-key
    surface) alongside the pre-existing topology assertion.
  - `activate_intent_dispatches_through_the_typed_command_path_same_turn` — Activate intent →
    mutation applies + `result.mutations.len()==1` + `history_patch.is_some()` in one
    `handle_intent_frame` call.
  - `view_kind_intent_returning_operations_hard_faults` — kind discipline survives the intent path
    (added a `"badView"` arm to `TestApp::command_from_action`, reachable only via the intent
    bridge, to exercise this without touching any existing test).
  - `command_from_intent_rejects_a_non_v1_action_version` — the version-mismatch decision.

**Run status — HONEST, not green**: `cargo test -p semio-framework-plugin --lib` does **not**
compile — 383 errors at time of writing. This is the SAME documented, already-known blocker
`important.md`'s "sdk-final"/"sdk-tests" entries name ("`cargo test -p semio-framework-plugin
--lib` cannot compile — 1373 errors, all `#[cfg(test)]`... needs its own dedicated packet") —
reduced by other packets to the 380s in the interim, still entirely outside M1/M2:
every remaining error is in `⚛️reactor/💼️jobs/**`/`⚛️reactor/📮️requests/**` (job-registration
fn-pointer/missing-`.await` residue) or elsewhere pre-existing; **zero** touch
`⚛️reactor/🦀️component.rs`, `⚛️reactor/🩹️patches/🦀️component.rs`, or my new test regions
(verified by line-range and symbol-name grep against the full error list, pasted in
`terra-sdk-wire-census.txt`). I caught and fixed two mistakes of my own in this same pass (a
missing local `use semio_framework_ui_contract::{...}` in `plugin_builder_contract_tests`, which
names every import explicitly rather than glob-importing — my bare `UiIntent`/`ActionId`/etc.
weren't in scope there; and a private-field read on `command_log`, replaced with the public
`InvocationResult.{mutations,history_patch}`) — both confirmed fixed by a second compile pass
before this report was written. **My new tests are therefore compile-clean by elimination (no
error in their file:line range across two clean re-checks) but UNRUN — I cannot get any test in
this crate to execute until the pre-existing `⚛️reactor/💼️jobs`/`📮️requests` residue is fixed by
its own packet.** Treating this as anything but an honest UNRUN would be the false-green this
ticket's rules explicitly forbid.

## Compile gates (all four, `CARGO_TARGET_DIR` = scratchpad `target-sdkwire`)

```
cargo check -p semio-framework-plugin --lib                                                    → EXIT 0, 0 errors
cargo check -p semio-framework-plugin --lib --all-features                                     → EXIT 0, 0 errors
cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest   → EXIT 0, 0 errors
cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-extension-guest → EXIT 0, 0 errors
```

## Dropped-future census (R12/R13/R17)

`cargo clean -p semio-framework-plugin` then forced rebuild `cargo check -p semio-framework-plugin
--lib --message-format=short` → EXIT 0; `grep -c "unused implementer of"` → **0** (unchanged from
the ticket's own 15:02 baseline). `grep -n "let _ = "` over `⚛️reactor/🦀️component.rs` +
`🦀️component.rs` → every hit is pre-existing (already-awaited discards, pinned test fixtures that
deliberately never resolve, sync error-suppression) — none touch M1/M2 code; no new dropped future
introduced.

## Regression floor

- `cargo test -p semio-framework-os-kernel --lib` → **779 passed / 0 failed / 0 ignored**.
- `cargo check -p semio-framework-plugin-host --lib` → **EXIT 0, 0 errors** (sol's 3
  `E0063 missing field presence` sites were routed to `shard-lane`; confirmed clear on my own
  re-check, not touched by me).

## Blocked / not fixed here (out of scope, flagged)

- `cargo test -p semio-framework-plugin --lib` residue (383 errors, `⚛️reactor/💼️jobs`/`📮️requests`
  job-registration fn-pointer/`.await` shape) — pre-existing, needs its own packet (already named
  in `important.md`'s "sdk-tests"/"sdk-final" entries).
- `PanelTreeBuilder.selected_ids`/`.highlighted_ids` for APP-OWNED trees that do NOT declare an
  `interaction_domain` — genuinely cannot feed the M2 derivation this wave:
  `PanelTreeBuilder::build()` returns a plain `BuiltNode` with no presence field surviving into it,
  and the only place with `&mut self: VcsArtifactApp` access to `pending_presence`
  (`stamp_and_cache_interaction_ui`) only sees the built tree AFTER `.build()` has already erased
  the selection data. Only `interaction_domain`-bound trees are covered this wave (the framework-
  owned mechanism, matching the M2 acceptance test fixture). Flagged, not silently dropped — a
  future packet giving each plugin a real `Present`/`HandleIntent` impl (the same one M1's own
  design doc named as out of scope: "do NOT embed full `UiRuntime<S,D>` per actor") is the natural
  place to close this, since only that plugin-authored code still holds the selection ids at
  present time.
