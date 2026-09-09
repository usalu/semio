# Peer Config/Runtime Split — Puzzle 3D Impact Audit (2026-09-09)

Read-only audit of the GPT-fleet ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/` as it lands on our `puzzle/3d` editor. No source was edited, no build/test was run beyond one allowed `cargo check`.

## TL;DR

**Lane model** (proven by 2D's finished shape, `✏️s/…/◻️2d/…/✏️editor/🪟️window/🦀️.rs`):
- **Shared `Config`** (`ArtifactApp::Config`, one per document): only fields that drive the one shared document/precompute plan and must never disagree across split windows — 3D: `Puzzle3dConfig{fill_count, overlap_budget, object_kind_weights, vortex_kind_weights}` (`🎚️config/🦀️.rs:247-262`).
- **`WindowConfigOwner`** (persisted-local, keyed by concrete window **instance** id): per-window chrome that should survive reload but must isolate split panes — camera, LOD, grid, sun, vortex display, selectable kinds, voxel/gumball flags. 3D: `Puzzle3dWindowConfig` (`🪟️window/🦀️.rs:9-24`, registered against `main::WINDOW_KIND_ID` only, `🪟️window/🦀️.rs:130-139`).
- **`WindowTransientOwner`** (ephemeral, keyed by window instance): interaction scratch that must NOT survive reload — engagement input, suggestion-menu popup, brush candidate index. 3D: `Puzzle3dWindowTransient{suggestion_menu, engagement_input, brush_candidate_index}` (`🪟️window/🦀️.rs:78-82`).
- **OS `ViewModel`** (host-owned, never plugin config): locale/terminology, `active_tool_id` (mode-wide) and `active_utility_by_window_id`/`active_utility_id` (per window). Confirmed at `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4278-4299`. Puzzle3d's `host_configuration_mutation` correctly returns `Ok(None)` for both `setActiveTool`/`setActiveUtility` (`✏️editor/🦀️.rs:6617-6619`) — no plugin config duplicate.
- **`Puzzle3dRuntime` lands nowhere persisted.** It is a call-scoped **compatibility shim**, synthesized fresh on every `handle`/`render`/`window_measures` call by `window_ownership::runtime(shared, window, transient, view)` (`🪟️window/🦀️.rs:159-187`) purely so ~50 old call sites deep in the 419KB editor body don't all need touching in one pass — exactly 2D's pattern (`◻️2d/…/🪟️window/🦀️.rs:228-259`, `Puzzle2dPlayRuntime`).
- **Required plugin surface per window kind** (2D's finished shape): `WindowConfigOwner`/`WindowTransientOwner` impls + a `Snapshot{..}`-only `Mutation` per state + `json_store!`/`mutation_wire!` macro-generated `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` codecs, e.g. `◻️2d/…/🪟️window/🦀️.rs:38-66,107-140`.

**Migration status:** NOT converging cleanly — it is actively **regressing state**, not just failing to compile. `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` → **55 errors** (28×`E0609` unknown field, 25×`E0308` `Puzzle3dConfig`/`Puzzle3dRuntime` mismatch, 1×`E0433` undeclared `Puzzle3dScalarConfigWork`, 1×`E0599` missing `load_window` method), not the ~130 the 06:05 note implied — either they've made real progress since, or that count included test targets/other crates.

**Re-landing checklist after their split converges** (see §4 for full detail):
1. Re-verify `fill_checkpoint`/`fill_apply_generation`/`fill_applied_count` continuity — currently **silently dropped** on the `handle()` path (see §2), which will corrupt/discard in-flight fill-build jobs across a config round-trip until fixed.
2. Re-verify `handle()`'s reconstructed runtime carries the REAL captured window transient, not `Puzzle3dWindowTransient::default()` (`✏️editor/🦀️.rs:6749`) — engagement input/suggestion menu/brush candidate index are currently discarded on every action dispatch.
3. Re-run this ticket's own wave-G/G2/O/D fixtures (session registry, document-tree memo, outliner paging, store disposers) once the crate compiles — none of them exercise the split; they're presumed frozen underneath it, but `puzzle3d_config_store_mutation_bytes` (`✏️editor/🦀️.rs:6166`) and the retained-jobs dispatch table (`✏️editor/🦀️.rs:6653,6680`) directly reference the mutation/config shapes this ticket is changing.
4. Resolve `Puzzle3dScalarConfigWork` (referenced but never declared — `✏️editor/🦀️.rs:6680`; expected shape asserted by a static test at `🧪️tests/🔬️unit/🦀️.rs:847-880`) — and flag that test's asserted shape as likely WRONG (see §4.4): it expects window-scoped mutations (`SetWindowCamera`, `SetWindowSun`, …) on the **shared** `Puzzle3dConfigMutation`, contradicting the window/config split this same ticket is enforcing.

---

## 1. Ownership lanes and required plugin surface

### (a) Shared `Config` lane

3D's current `Puzzle3dConfig` (`✏️editor/🎚️config/🦀️.rs:245-262`):
```
pub struct Puzzle3dConfig { fill_count: u32, overlap_budget: f64, object_kind_weights: HashMap<String,f64>, vortex_kind_weights: HashMap<String,f64> }
```
`Puzzle3dConfigMutation` still carries the 5 variants the task described — `Snapshot`, `SetFillCount`, `SetOverlapBudget`, `SetObjectKindWeights`, `SetVortexKindWeights` (`✏️editor/🎚️config/🦀️.rs:409-415`). This matches 2D's shared `Puzzle2dConfig{node_kind_weights, handle_kind_weights}` (`◻️2d/…/🎚️config/🦀️.rs:242-247`), which also carries only fields that drive the one shared document/precompute plan.

### (b) `WindowConfigOwner` (persisted-local per window)

Framework trait: `🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🦀️.rs:13-23` — `WindowConfigOwner` requires `WINDOW_KIND_ID`, `SCHEMA`, `MAXIMUM_PUBLICATION_BYTES`, a `State` (`ArtifactPack`+`ConfigRecord` etc.), and a `Mutation`; the registry (`…/🎚️config/🦀️.rs:512-631`) partitions storage by `(window_kind_id, window_id)` via `WindowConfigOwnerRegistry`, each partition its own event-sourced `ConfigStore` (`…/🎚️config/🦀️.rs:354-365`) — i.e. genuinely independent CQRS/event-sourced publication authority per concrete window instance, not a shared map field.

2D's finished window config (`◻️2d/…/🪟️window/🦀️.rs:8-17,107-119`): `Puzzle2dWindowConfig{camera_x, camera_y, camera_zoom, lod_mode, fill_count, grid_snap_enabled, grid_factor, suggestion_offset}` + `Puzzle2dWindowConfigMutation::Snapshot{config}` (whole-record, no scalar-leaf variants) + the `owners!` macro (`◻️2d/…/🪟️window/🦀️.rs:107-130`) registering one `WindowConfigOwner`/`WindowTransientOwner` pair PER window kind (overview/detail/selection — 3 kinds, 3 registrations).

3D's window config (`🧊️3d/…/🪟️window/🦀️.rs:9-24,129-139`): `Puzzle3dWindowConfig{lod_automatic, lod_depth_variable, grid_visible, lod_manual, grid_snap_enabled, grid_spacing, selectable_kinds, proximity_radius, chunk_size, voxel_dims, transform_move, transform_rotate, vortex_show, vortex_direction, sun, camera}` — registered against exactly one window kind, `main::WINDOW_KIND_ID` (`🧊️3d/…/🪟️window/🦀️.rs:131,143`). This is consistent with 3D having one window kind today (`main`), unlike 2D's three.

### (c) `WindowTransientOwner` (ephemeral per window)

Framework trait: `🧰️framework/…/🔌️plugin/🪟️window/🫧️transient/🦀️.rs:12-20` — parallel shape, no `SCHEMA`/publication-byte-bound (ephemeral, not event-sourced/persisted — `store::TransientStore`, generation-only authority, `…/🫧️transient/🦀️.rs:146-155`).

3D: `Puzzle3dWindowTransient{suggestion_menu, engagement_input, brush_candidate_index}` (`🧊️3d/…/🪟️window/🦀️.rs:78-82`), registered `🧊️3d/…/🪟️window/🦀️.rs:141-149,155-157`.

### (d) OS `ViewModel` (host-owned)

`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4275-4299`: `active_mode_id`, `active_window_kind_id`, `active_utility_id` (per-call overlay for the addressed window), `active_utility_by_window_id: HashMap<window_id,String>` (full per-window map, sent every refresh), `active_tool_id: Option<String>` (mode-wide, mutually exclusive with utility). Locale/terminology are noted as "part of the host projection" in `window-context-ownership.md` (peer's own note, not independently verified here — framework locale fields weren't in scope of files read).

Puzzle3d correctly treats these as read-only: `host_configuration_mutation` returns `Ok(None)` for `setActiveTool`/`setActiveUtility` (`✏️editor/🦀️.rs:6614-6619`, comment: *"puzzle3d's active utility IS host view state"*). `window_ownership::runtime()` projects `view.active_tool_id` and `view.window_instances` READ-ONLY into the synthesized runtime for old call sites (`🧊️3d/…/🪟️window/🦀️.rs:184-185`) — this is NOT a duplicate config declaration since `Puzzle3dConfig`/`Puzzle3dWindowConfig` carry neither field; it's a per-call convenience projection only, consistent with `active-utility-ownership.md`'s rule.

### Where `Puzzle3dRuntime` lands

Nowhere persisted. It stays in `🎚️config/🦀️.rs:131-243` as a plain struct (still has `ArtifactPack`? — **no**, only `ToValue`/`FromValue` derives, no `store::ArtifactDsl`/`ArtifactPack` impl block for it in the current file, confirmed by absence in the 460-line file) — it is purely an in-memory call-scoped aggregate, assembled by `window_ownership::runtime()` (`🪟️window/🦀️.rs:159-187`) and torn back down by `window_ownership::shared()`/`transient()` (`🪟️window/🦀️.rs:189-195`). 2D proves this is the intended end-state, not a leftover: `Puzzle2dPlayRuntime` (`◻️2d/…/🎚️config/🦀️.rs:158-238`) has the identical no-persistence shape, and 2D's own test asserts the app's real persisted `Puzzle2dConfig` pack/SPR excludes every window/transient field name (`◻️2d/…/🪟️window/🦀️.rs:278-289`, test `app_pack_and_spr_exclude_window_and_transient_fields`). 3D has the matching test at `🪟️window/🦀️.rs:227-238`.

---

## 2. Call-site shape changes

| Call site | Current 3D signature | Target shape (per 2D) | Status |
|---|---|---|---|
| `handle`/`ArtifactEditor::handle` | `cfg: &ConfigView<'_, Puzzle3dConfig>` (already narrow) → internally does `window_ownership::config_from_view(cfg)` + `window_ownership::runtime(cfg.snapshot, &window, &Puzzle3dWindowTransient::default(), view_state)` (`✏️editor/🦀️.rs:6738-6751`) | Should also read the REAL `TransientView` for the window, not `::default()` | **Converted but wrong**: transient is defaulted, discarding live engagement/suggestion-menu/brush-index state on every dispatch (see §4.1) |
| `handle_action_impl` | Still `config: &Puzzle3dRuntime` (`✏️editor/🦀️.rs:2553-2560`) | Fine as-is — it's fed the synthesized shim from `handle()`, exactly 2D's pattern | OK by design |
| `render`/`render_with_request_context` | Already `cfg: &ConfigView<'_, Puzzle3dConfig>` (`✏️editor/🦀️.rs:6848,6856-6864`) | Same | Signature converted, but... |
| `render_body` (the one real impl both funnel into) | Passes `cfg.snapshot` (type `&Puzzle3dConfig`, 4 fields) directly into `with_puzzle3d_app_for(…, cfg.snapshot, …)` which expects `&Puzzle3dRuntime`, and reads `config.fill_checkpoint`/`config.window_ids`/`config.load_window(wid)`/`config.fill_applied_count` (`✏️editor/🦀️.rs:6893-6990`) | Must reconstruct via `window_ownership::runtime(cfg.snapshot, &window, &transient, Some(view_state))` first, like `handle()` does | **NOT converted** — this is the single largest source of the 55 compile errors (E0609/E0308 at lines 2850-2874, 5227-5228, 5568-5976, 6894-7018) |
| `window_measures`/`window_measures_with_request_context`/`window_measures_body` | Same `cfg.snapshot`-as-`Puzzle3dRuntime` bug (`✏️editor/🦀️.rs:7010-7021`) | Same fix needed | NOT converted |
| `window_engagements`, `tool_measures`, `context_menu`/`context_menu_body` | Same bug (`✏️editor/🦀️.rs:6893-6902, 6968-6972, 6894`) | Same fix needed | NOT converted |
| `with_puzzle3d_app_for(config)` — reads `fill_checkpoint` | `config.fill_checkpoint` (`✏️editor/🦀️.rs:2452-2453`) | **`fill_checkpoint`/`fill_apply_generation`/`fill_applied_count` have NO owner in the new split** — absent from `Puzzle3dConfig` (4 fields), `Puzzle3dWindowConfig` (16 fields, no fill fields), and `Puzzle3dWindowTransient` (3 fields) | **Orphaned.** `window_ownership::runtime()` never sets these three fields (`🪟️window/🦀️.rs:159-187` — no `fill_checkpoint`/`fill_apply_generation`/`fill_applied_count` line at all), so every synthesized runtime has them at `Default` (empty/0) regardless of what was persisted |
| `Puzzle3dScalarConfigWork`/`Puzzle3dConfigMutation::*` emitters (camera/sun/grid/lod/vortex/engagement/suggestion menu/brush candidate) | Referenced, undeclared (`✏️editor/🦀️.rs:6680`, E0433) | Per 2D's model, these must emit `WindowConfigMutation`/`WindowTransientMutation` (via `window_ownership::addressed_config`/`addressed_transient`, `🪟️window/🦀️.rs:202-210`), NOT `Puzzle3dConfigMutation` leaf variants | **Not implemented**, and the static test asserting its expected shape (`🧪️tests/🔬️unit/🦀️.rs:847-866`) asserts the WRONG target (see §4.4) |
| `puzzle3d_config_store_mutation_bytes` | `fn(mutation: &Puzzle3dConfigMutation) -> Option<usize>` (`✏️editor/🦀️.rs:6166`) | Stays as-is for the 5 shared-config variants; window-scoped mutations need an analogous byte-bound fn keyed off `WindowConfigOwner::MAXIMUM_PUBLICATION_BYTES` (framework already provides this via `BoundedWindowConfigPreparationFactory::preflight`, `🎚️config/🦀️.rs:44-54`) | Framework-side plumbing exists; plugin-side wiring pending |
| `host_configuration_mutation` (setActiveTool/setActiveUtility) | `Ok(None)` for both (`✏️editor/🦀️.rs:6617-6619`) | Correct, no change needed | Already correct |
| `Puzzle3dPresence` | `👥️presence/🦀️.rs:18`, `👥️presence/🧬️schema/🦀️.rs:7` — untouched by this audit's scope; not referenced by the window-ownership split fns | Out of scope for this split (framework interaction presence handles selection/hover per `trinity-command-ownership.md`'s "Duplicate Presence Removal") | Unaffected |
| Retained-jobs/publication-authority fixtures | `🧊️3d/…/🗄️retained-jobs/🔣️.json:719-733` encodes a "scalarConfigOldReducer" hostile-mutation-law case expecting `Puzzle3dScalarConfigWork::new(tool_id)` to exist and reject a `BoundedFirstStepCommandWork` replacement | Needs the type to exist before its own hostile-law fixture can even run | Fixture written ahead of implementation |

---

## 3. Migration progress

**`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 2` (this run, 2026-09-09):**
```
error: could not compile `semio-s-artifact-puzzle-3d` (lib) due to 55 previous errors; 1 warning emitted
```
By code: **28×E0609** (unknown field — `window_ids`, `fill_checkpoint`, `fill_applied_count`, `suggestion_menu`, `brush_candidate_index`, `camera`, `proximity_radius`, `window_options`, `grid_spacing`, `voxel_dims` all "no field on `Puzzle3dConfig`"), **25×E0308** (mismatched types, almost all `expected &Puzzle3dRuntime, found &Puzzle3dConfig`), **1×E0433** (`Puzzle3dScalarConfigWork` undeclared), **1×E0599** (`load_window` not found on `Puzzle3dConfig` — it's a `Puzzle3dRuntime` inherent method, `🎚️config/🦀️.rs:392-395`). 1 unrelated unused-import warning (`✏️editor/🦀️.rs:39`, stale `SET_ACTIVE_UTILITY_ACTION_ID`/`world3d_projection_*` imports).

All 55 errors trace to exactly the pattern in §2's table: functions whose SIGNATURE was updated to take `cfg: &ConfigView<'_, Puzzle3dConfig>` but whose BODY still treats `cfg.snapshot` as the old full `Puzzle3dRuntime` — `render_body`, `window_measures_body`, `window_engagements`, `tool_measures`, `context_menu_body`, plus the standalone `handle_action_impl`'s precompute-restore block (lines 2868-2874) and the `scene_for` micro-helper. `handle()` itself (line 6738) is the ONLY converted call site — it correctly reconstructs a shim runtime first.

**Who's touching what (mtimes today, 2026-09-09):** `🎚️config/🦀️.rs` (06:27, most recent), `🪟️window/🦀️.rs` (06:24), `🎮️commands/🪣️fill-build-tick/🦀️.rs` and `🧮️set-fill-count/🦀️.rs` (06:28, the two most recently touched files in the whole crate), `📌️panels/🛍️catalogue/🦀️.rs` (06:26), `⏳️precompute/🦀️.rs` (05:59), `👥️presence/🦀️.rs` (05:59), `🗣️terminology/🦀️.rs` (05:04). **They are converting `⏳️precompute`, `📌️panels`, and `🎮️commands` themselves**, not leaving those lanes to others — contradicts assuming this is config-file-only work; the fill-build-tick/set-fill-count command files being the LATEST touched (past `🎚️config`/`🪟️window`) suggests they're mid-way through wiring `Puzzle3dScalarConfigWork`/the retained-jobs dispatch table right now.

---

## 4. Risks and re-landing checklist

### 4.1 `fill_checkpoint` continuity — CRITICAL

`with_puzzle3d_app_for` restores persisted fill state only `if !config.fill_checkpoint.is_empty()` (`✏️editor/🦀️.rs:2452-2453`). Once `handle()` feeds it a shim built by `window_ownership::runtime()` — which never populates `fill_checkpoint`/`fill_apply_generation`/`fill_applied_count` (absent from `runtime()`'s field list, `🪟️window/🦀️.rs:159-187`) — every dispatched action sees an always-empty checkpoint, i.e. **the precompute fill-build session can never resume** via this path, silently. This directly collides with this ticket's own `cancelFillBuild`/fill-build-tick landed work (wave F/D): `fill_checkpoint`/`fill_apply_generation`/`fill_applied_count` need an explicit owner (most likely a NEW `WindowTransientOwner`-adjacent or job-scoped lane, since 2D's finished shape shows the analogous `fill_job_*` fields are ALSO absent from `Puzzle2dConfig`/`Puzzle2dWindowConfig`/`Puzzle2dWindowTransient` and from `Puzzle2dPlayRuntime`'s reconstruction in `runtime()`/`split()` — 2D's `runtime()` uses `..Default::default()` for them, `◻️2d/…/🪟️window/🦀️.rs:249`, meaning 2D has the IDENTICAL orphaning today and it is unclear from the read files whether 2D's fill-job continuity is intentionally handled elsewhere (a job-registry/`app_instance_id`-keyed session state, per `puzzle3d_session_registry`) or is itself broken). **Action: before closing out this wave, get an explicit confirmation from the peer ticket on where fill-job continuation state lives now, and add a regression test that round-trips a non-empty `fill_checkpoint` through `handle()`.**

### 4.2 Window transient defaulted in `handle()`

`handle()` builds its runtime shim with `&window_ownership::Puzzle3dWindowTransient::default()` (`✏️editor/🦀️.rs:6749`), not a captured `TransientView`. `Puzzle3dEditor::handle`'s trait signature does receive `interaction: &InteractionView` but no `TransientView` argument at all in the current 3D impl (compare framework `WindowTransientOwnerRegistry::capture` at `🫧️transient/🦀️.rs:271-281`, which exists precisely to serve this). Every action dispatch therefore sees `engagement_input=""`, `suggestion_menu=None`, `brush_candidate_index=0` regardless of actual state, then (per `handle_action_impl`'s `transient_before = window_ownership::transient(config)` line 2591) computes a bogus "before" snapshot and — depending on how the emitted `WindowTransientMutation` diff is built downstream — may **stamp real transient state back to empty** on every action. This needs its own targeted trace once the crate compiles; flag now because it's invisible in the current error list (it's a logic bug, not a type error).

### 4.3 Wave collisions with this ticket's other landed work

- **Session registry keyed by `app_instance_id`** (`✏️editor/🦀️.rs:2408-2439`, `puzzle3d_session_check_out`/`_check_in`): orthogonal to the config split — keyed by document instance, not window instance — should be unaffected, but `with_puzzle3d_app_for`'s signature (taking `&Puzzle3dRuntime`) is the exact point where session adoption and the fill-checkpoint restore both happen (line 2449-2458), so §4.1's fix must not disturb the session lease logic.
- **`Emit.interaction_writes`**: `Emit` already carries `window_config_mutations`/`window_transient` fields alongside `interaction_writes` (`✏️editor/🦀️.rs:2649`), i.e. the Emit plumbing for the split is already wired at the call site that matters — good sign, low risk here.
- **Document-tree memo / outliner paging** (`document_tree_cache`, `✏️editor/🦀️.rs:2412,2425,2493,2521`; outliner paging in `📌️panels/🛍️catalogue/🦀️.rs` and `📌️panels/🗿️artifact/🦀️.rs`): keyed off fixture content, not config shape — should be unaffected by the split, not touched by any of the 55 errors.
- **Store disposers** (`build_config_store_disposer`/`build_transient_store_disposer`, `✏️editor/🦀️.rs:6600-6604`): framework's `WindowConfigOwnerRegistry`/`WindowTransientOwnerRegistry` close-step machinery (`🎚️config/🦀️.rs:490-509`, `🫧️transient/🦀️.rs:226-241`) is independent per window-kind partition; 3D only registers one partition (`main`) so this is low-risk, but confirm `register_config`/`register_transient` (`🪟️window/🦀️.rs:151-157`) are actually wired into whatever central registry composes all app plugins (not verified in this audit — out of the files read).
- **`puzzle3d_config_store_mutation_bytes`**: only validated against the 5 shared-config variants today (`✏️editor/🦀️.rs:6166`); once window-scoped mutations exist, any code path that calls this fn on a window mutation will need a parallel window-scoped bound check (framework already exposes `BoundedWindowConfigPreparationFactory::preflight`, `🧰️framework/…/🪟️window/🎚️config/🦀️.rs:44-54`, so this is additive, not a rewrite).

### 4.4 Self-contradicting test — flag to the peer ticket, don't silently "fix"

`🧪️tests/🔬️unit/🦀️.rs:847-866` (`scalar_config_routes_are_direct`) is a static source-text assertion requiring `Puzzle3dConfigMutation::SetWindowCamera`, `SetWindowSun`, `SetWindowGridSpacing`, `SetOverlapBudget`, `SetWindowVoxelDims`, `SetSuggestionMenu`, `SetBrushCandidateIndex`, `SetWindowEngagementInput` to exist on the **shared** `Puzzle3dConfigMutation` enum. This contradicts the window/config split this same ticket enforces elsewhere: camera/sun/grid-spacing/voxel-dims/suggestion-menu/brush-candidate-index/engagement-input are exactly the fields that live in `Puzzle3dWindowConfig`/`Puzzle3dWindowTransient`, not `Puzzle3dConfig` — 2D's finished shape proves the target is a single `Snapshot{config}`/`Snapshot{transient}` mutation per window-owned type (`◻️2d/…/🪟️window/🦀️.rs:26-28,78-80`), never scalar per-field leaves on the shared config. This test (and the matching `🗄️retained-jobs/🔣️.json:721-733` fixture) look like they were authored against an EARLIER design draft and never updated after the window-ownership split was decided (`window-context-ownership.md`, `📋️plan.md`'s "Remaining Workstreams" item 1). Recommend surfacing this to the peer ticket rather than assuming either the code or the test is "right" without their input — it affects whether `Puzzle3dScalarConfigWork` should emit `Puzzle3dConfigMutation` leaves (test's expectation) or `WindowConfigMutation`/`WindowTransientMutation` (architecture's expectation, matching `Puzzle3dHostConfigurationProofs`/2D's actual implementation).

---

## Files read (evidence sources)

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/📋️plan.md`, `active-utility-ownership.md`, `window-context-ownership.md`, `ownership-verification.md`, `browser-actor-context-ownership.md`, `trinity-command-ownership.md`, `presentation-metadata-ownership.md` (skimmed, off-topic for puzzle3d)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs`, `…/🫧️transient/🦀️.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (`ViewModel` struct)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs`, `…/🎚️config/🦀️.rs` (2D reference)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`, `…/🪟️window/🦀️.rs`, `…/🦀️.rs` (419KB, grep+targeted reads only), `…/🧪️tests/🔬️unit/🦀️.rs`, `…/🗄️retained-jobs/🔣️.json`
- `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 2 --message-format=short` (ran once, per task's explicit allowance; output in scratchpad, not retained in the ticket)
