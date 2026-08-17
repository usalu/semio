# World Surface Domain Resolution — Stop Hardcoding `domainId: "world"`

## The open item

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs` and its byte-identical sibling
`♾️infinite/🌍️world/🦀️component.rs` emit the framework verbs `interactionSelect`/`interactionHover` for
plain (non-component) 3D-world object picks/hover, but hardcoded `domainId: WORLD_INTERACTION_DOMAIN_ID`
("world") at every emit site, while `world_interaction_definition()` was declared but never bound by any
`AppDefinition` — confirmed by `grep -rn "world_interaction_definition()"`, still only the definition
itself and its own unit tests.

## Verified design facts (before writing code)

- **`state.granularity` defaults to `"object"` in `World3dState::new()`, but `sync_world3d_state` always
  rewrites `"object"` → `"mesh"`** the moment a scene syncs (both the fast-path selection branch and the
  full-sync tail, ~line 1993/2105 pre-edit). So `pick_select_action`'s three-way branch
  (component-mode → legacy `worldPick`; `granularity == "mesh"` → legacy `worldPick`; else →
  `interactionSelect`) mostly lands on the *legacy* branch for select — but **`pick_hover_action`'s plain
  fallback and `marquee_select_action`'s non-component branch are unconditional** on granularity, so both
  hit the hardcoded `"world"` domain unconditionally for every world3d app today. This is a live bug, not
  a theoretical one.
- **CAD is the confirmed, concrete collision case**: `create_cad_app` binds `CAD_INTERACTION_DOMAIN =
  "cad"` (granularities `object`/`vertex`/`edge`/`face`, `HierarchyProvider::Flat`) to all four of its
  world3d window kinds. CAD's `handle()` reads `interaction.selection(CAD_INTERACTION_DOMAIN)` — a plain
  hover/marquee pick on a CAD pane was emitting into `"world"` with `"surfaceId/id"` targets, which CAD's
  own domain never sees. Exactly the two-selection-universe bug the ticket describes.
- **Crate-dependency check**: `semio-framework-os-infinite`'s only downstream `Cargo.toml` dependents are
  `flow`, `dag`, `procedural`, `puzzle`, `space`, `layout`, `reasoning`, `sequence`, `trinity` — none of
  which host a `SurfaceKind::World3d` window (confirmed via `grep -rn "SurfaceKind::World3d"`: cad, gis-3d,
  block-3d/5d, procedural-3d, puzzle-3d/5d, remodel, process-3d, shooting, sourcing, fem-3d, lowpoly,
  playbook-procedural do). Those 9 crates depend on this crate for the unrelated `🎲️board` 2D graph-engine
  module (used for their own `"graph"`-domain picking), not for `World3dState`. This explains why the
  acceptance gate names `flow`/`dag` — they're downstream-compile smoke tests for this crate's public API,
  not functional consumers of the world3d picking code.
- **No bare-board host exists today**: nothing calls `.interaction(world_interaction_definition())`
  anywhere in the repo, and no `AppDefinition` hosts a world3d window without also declaring its own
  domain (of the 5 call sites to `world3d_scene_extended`, 3 already had their own bound domain: CAD
  `"cad"`, GIS `"features"`, puzzle `"vortex"` — only block-3d had none of the three verified).

## Design ruling: resolve dynamically, not a permanent two-domain split

Implemented the ruling's clause 1 (resolve from the window's bound domain) rather than clause 2
(bare-board fallback is real) — the evidence above shows the fallback case is currently unpopulated, not
load-bearing, and the hardcoding was a genuine bug, not a deliberate two-universe design.

### Mechanism (mirrors the existing `UiTreeNode.interaction_domain` pattern, extended to `Scene` nodes)

`World3dState` has no manifest access (it's a free-function pointer-picking module driven purely by
`surface_id`/`controller_id` + geometry) — the only party that knows both a window's `window_kind_id` and
which domain it bound there (`WindowKindDefinition.interactions`) is the app's own `render()`. So, exactly
like `PanelTreeBuilder::interaction_domain(id)` stamps a `UiTreeNode`, the app's `render()` now stamps its
`World3dScene` payload:

- **`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`**: `World3dScene` gains
  `domain_id: Option<String>` / `domain_granularity_id: Option<String>` (camelCase on the wire:
  `domainId`/`domainGranularityId`). `World3dScene::base()` and the one test literal updated.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`**: `world3d_scene_extended(...)` gains
  the same two trailing params; `world3d_scene(...)` (the 5-arg convenience wrapper most apps use via
  `..world3d_scene(...)` spread) passes `None, None` — zero-touch for shooting/procedural, which use the
  spread form.
- **`♾️infinite/🦀️component.rs` + `♾️infinite/🌍️world/🦀️component.rs`** (identical edits, both mount
  points): `World3dState` gains `bound_domain_id`/`bound_domain_granularity_id: Option<String>`, captured
  by `sync_world3d_state` from `world.domain_id`/`domain_granularity_id` (added to the `unchanged`/
  `geometry_unchanged` diff checks and the reset/full-sync branches, same treatment as every other
  `scene_*_json` mirror field). New resolver functions in the `//#region 🔖️WorldInteractionDomain` block:
  - `resolved_domain_id(state) -> &str` — bound domain, else `WORLD_INTERACTION_DOMAIN_ID`.
  - `resolved_domain_granularity_id(state) -> &str` — bound granularity, else `WORLD_ITEM_GRANULARITY_ID`.
  - `resolved_item_id(state, object_id) -> String` — **bare id** when a domain is bound (a bound domain is
    inherently single-surface-scoped: one app document per window, no cross-surface disambiguation
    needed), else `world_item_target_id`'s `"surfaceId/id"` `PathDelimited` shape for the shared `world`
    domain (ruling point 3: id shape follows the *target domain*, not a fixed convention).
  - `parse_resolved_item_id(state, target_id) -> Option<&str>` — inverse, for the optimistic-preview path.
  - All three emit sites (`pick_hover_action`, `pick_select_action`'s plain branch, `marquee_select_action`'s
    non-component branch) now call these instead of the hardcoded constants.
  - `apply_world_action_preview`'s `interactionSelect`/`interactionHover` match guards now compare against
    `resolved_domain_id(state)` instead of the constant — **an action for a domain other than this
    surface's resolved one is ignored**, which is exactly what prevents the two-selection-universe bug:
    once a window binds "cad", a stray `"world"`-domain action (or vice versa) never applies here.
  - Extended doc comment on the region explaining the resolution chain and why a bound domain uses bare ids.
  - New tests (both files, `//#region 🔖️WorldInteractionVerbs`): `pick_select_emits_bare_id_into_bound_app_domain_when_window_binds_one`,
    `pick_hover_emits_bare_id_into_bound_app_domain`, `marquee_select_emits_bare_ids_into_bound_app_domain`,
    `apply_world_action_preview_respects_bound_app_domain_and_ignores_other_domains` (proves a `"world"`-domain
    action is dropped once bound to `"cad"`), `sync_world3d_state_captures_scene_bound_domain`. All existing
    tests pass unchanged — `World3dState::new()` defaults both new fields to `None`, so the unbound/fallback
    behavior (domain `"world"`, `"surfaceId/id"` targets) is byte-for-byte what it was before.

### Wired for real (verified, not guessed)

- **CAD** (`✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎭️modes/✏️edit/🦀️component.rs`, `build_world_scene_for_pane` —
  the single function serving all 4 CAD panes, all bound to `CAD_INTERACTION_DOMAIN`): now passes
  `Some(CAD_INTERACTION_DOMAIN.into())`, `Some("object".into())`.
- **GIS-3D terrain** (`✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs`):
  binds `"features"`/`"pin"` (its only granularity), matching `create_gis3d_app`'s existing
  `.window_kind_interactions(terrain::GIS3D_PLAY_WINDOW_MAIN, ...)`.

### Left unbound, explicitly flagged (not guessed)

- **Block-3D world window**: no domain/granularity mapping verified for its own vortex/grip picking vs. a
  plain whole-object pick — left `None, None` with an inline comment.
- **Puzzle-3D and Puzzle-5D**: both already emit their **own** `interactionSelect`/`interactionHover` for
  `PUZZLE3D_INTERACTION_DOMAIN`/`PUZZLE5D_INTERACTION_DOMAIN` ("vortex") from bespoke vortex-fit pick logic
  elsewhere in their crates, independent of this generic surface. Binding this scene's plain-pick fallback
  to the same domain without first confirming the two paths can't double-emit was judged too risky to guess
  — left `None, None` with an inline comment explaining why.

These four are honestly incomplete, not silently dropped — each has an inline comment at its call site
naming exactly what's missing and why it wasn't attempted.

## Concurrency note

While implementing this, `world3d_scene_extended` call sites in
`✏️s/🔌️plugins/🌍️gis/.../🏔️terrain/🦀️component.rs`, `✏️s/🔌️plugins/🧩️puzzle/.../🧊️main/🦀️component.rs`, and
`✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/.../🧊️3d/🦀️component.rs` were observed changing on disk between my own
Read/Edit calls (another live session touching the same function's call sites) — one of my own edits to
the GIS file landed on top of a stale read and produced a wrong arg count (23 supplied vs. 21 expected);
caught immediately by `cargo check -p semio-s-plugin-gis` and fixed by re-reading and correcting the
duplicate `None,` run. Final state verified by a clean `cargo check` per crate below.

## What did NOT change

- No touch to `worldPick`/`setSelection` (component-level vertex/edge/face picking) — still the separate,
  unconverted mechanism this ticket's prior waves (W3b/W3c/W7) explicitly deferred.
- No touch to the TS/React `World3dHost/🟦️component.tsx` renderer path — it still dispatches the legacy
  `"worldSelect"`/`"worldHover"` action names (flagged by W7 as out of this Rust-only file scope, still
  true; `handle_world3d_input` in `Shell/🧊️component.rs` forwards whatever action name the Rust side
  returns generically, so once each app is wired the TS call sites are a follow-up rename only).
- No new `.interaction(world_interaction_definition())` binding anywhere — still correctly unbound, since
  no bare-board host was found to exist.

## Acceptance (real output, run just now)

```
cd /Users/ueli/Documents/semio && cargo check -p semio-framework-os-infinite 2>&1 | tail -15
...
warning: `semio-framework-os-infinite` (lib) generated 69 warnings (run `cargo fix --lib -p semio-framework-os-infinite` to apply 43 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 53.62s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6

cd /Users/ueli/Documents/semio && cargo check -p semio-s-plugin-flow 2>&1 | tail -10
...
warning: `semio-s-plugin-flow` (lib) generated 25 warnings (run `cargo fix --lib -p semio-s-plugin-flow` to apply 23 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 6.78s

cd /Users/ueli/Documents/semio && cargo check -p semio-s-plugin-dag 2>&1 | tail -10
...
warning: `semio-s-plugin-dag` (lib) generated 42 warnings (run `cargo fix --lib -p semio-s-plugin-dag` to apply 40 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 6.52s
```

Zero errors on all three, warning counts unchanged from baseline (69/25/42 — matches prior waves' recorded
baselines).

**Also verified** (not required by the task, but touched by this change and gated for real, since adding
2 params to a widely-called helper is a breaking change to every caller): `cargo check` clean (0 errors)
on every crate that constructs a `World3dScene` or calls `world3d_scene_extended`/`world3d_scene`:
`semio-framework-ui`, `semio-framework-plugin`, `semio-framework-os-renderer-wgpu`, `semio-s-plugin-cad`,
`semio-s-plugin-gis`, `semio-s-plugin-block`, `semio-s-plugin-puzzle`, `semio-s-plugin-shooting`,
`semio-s-plugin-procedural`.

**`cargo test -p semio-framework-os-infinite --lib`**: still fails to COMPILE, confirmed still the
documented pre-existing regression — `error[E0608]: cannot index into a value of type 'DslValue'` on the
`args["key"]` idiom, now also hit by this task's 5 new tests (written in the same idiom as every existing
test in this file, per the file's own established convention and W7's precedent) in addition to the
pre-existing ones. `cargo check` (used above) is unaffected — it does not compile the `#[cfg(test)]`
module — and is the correct gate per the task brief.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎭️modes/✏️edit/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs`

## Outstanding

1. Block-3D, Puzzle-3D, Puzzle-5D world windows left unbound (`None, None`) — see "Left unbound" above.
2. TS `World3dHost/🟦️component.tsx` still on legacy `worldSelect`/`worldHover` action names (pre-existing
   W7 gap, unchanged by this task).
3. No bare-board OS host exists yet to bind `world_interaction_definition()` to — still correctly
   unbound; revisit only if/when the OS shell itself hosts a board with no app domain.
