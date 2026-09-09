# Wave K — scene payload references, a total view session key, and the stale retained fixtures

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-K, 2026-09-09. Written incrementally while the wave ran.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached at `9b605a4550`, working tree carries many peers' uncommitted
  edits. Peers active in the same crate throughout (W-J on the bounded fill job:
  `✏️editor/⏳️precompute/🦀️.rs`, `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs`,
  `✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs`, the reactor job wire — none touched by this wave).
- Crate under test `semio-s-artifact-puzzle-3d`, feature `component-app-assembly`.
- Seeded private target dir (49 GB):
  `/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d`.
- Command envelope for every Rust run below:
  `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 CARGO_TARGET_DIR=<target-p3d> cargo … -j 4`.
- Volume at 74 % with ~4 GiB free; no new target dir created.
- **Measured baseline before any edit of this wave** (whole suite, `--test-threads=1`, 130.0 s):
  `581 passed; 30 failed` — exactly the handover figure.

## 1 Defects + evidence

### 1.1 Built-in mesh geometry rode inline in every world-3d scene refresh

`world3d_mesh_kind_entry` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`) emitted
`{ id, data }` — the FULL tessellation of a procedural primitive — for every built-in kind named in a
scene's `meshes_json`. The puzzle 3d main window names two (`box` and `vortex-marker`,
`✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:177`), and `vortex-marker` is
`mesh_ico_sphere(0.12, 1)` — 80 triangles pushed non-indexed, i.e. 240 vertices × (3 positions + 3
normals) + 240 indices = 1 680 JSON floats. W-D3 measured the resulting scene payload directly:

```
meshes=27673 instances=270 selection=212 vortices=2639 attractions=2 volumes=2
refs=327 preview=281 interaction=1094 lod=125 chunk=40 env=92        → 33289 packed
```

`meshes_json` was 27 673 of a 32 KiB `UiFixedBytes` surface payload — 83 % — so any window that also
carried a resolved suggestion popup failed admission outright:

```
render: Fault { origin: Plugin, code: FaultCode("plugin.internal"),
  message: "ui.fixed-capacity: fixed UI admission failed at scene-surface.encode:
            surface payload exceeds fixed capacity with 33527 bytes" }
```

This is a pure re-send: the geometry is procedural and every renderer already owns a generator for it
(Rust `mesh_from_kind`, the wgpu viewport's own `WorldPlaceholderKind`/`ensure_primitive_mesh`, and the
R3F engine's primitives). A GLB mesh already rode as `{ id, url }`; the procedural case was the asymmetry.

### 1.2 `puzzle3d_view_session_key` was `None` on four host entry points

`puzzle3d_view_session_key` (`✏️editor/🦀️.rs:2662`) derives the per-document-instance session key from
`ArtifactView::operation_optional()` or `render_operation()`. `VcsArtifactApp` stamped an
`AppRenderOperationContext` into the view ONLY in `render` and `pending_effects`; `window_measures`,
`window_engagements`, `tool_measures` and `context_menu` built their view with
`ArtifactView::with_children(...)`, which carries no identity at all. So all window/tool chrome ran
`with_puzzle3d_app_for(None, …)` — no session lease, a cold `Puzzle3dPlayApp`, and
`session.brush_candidates(…)` empty (`[DEBUG] set_scene_config CHANGED … prior=None`, four times in one
test — W-D3 §5.2). The identity derivation was also duplicated verbatim in the two lanes that DID have it.

### 1.3 Two stale language-neutral retained-command fixtures

- `toolIds` in `🧫️fixtures/🗄️retained-jobs/🔣️.json:25` still listed `setFillCountStep`, an action that no
  longer exists anywhere in the puzzle 3d editor (`PUZZLE3D_RETAINED_TOOL_IDS`, `✏️editor/🦀️.rs:3100`).
- `checkpointBytes` was `112`/`113` in ALL THREE puzzle artifacts' fixtures (2d, 3d, 5d) while
  `PUZZLE_COMMAND_CHECKPOINT_BYTES` is `120` (`🎮️commands/🧵️retained/🦀️.rs:15`). The encoder writes an
  8-byte header plus 14 `u64` fields = 120 (`:177 encode`); the fixtures were authored when there were 13
  fields. This was hidden behind the `toolIds` assertion, which fails first.

### 1.4 The puzzle 5d fixture's `toolIds` was stale in the same way (found by W-K2 while landing 1.3)

Fixing 1.3's `checkpointBytes` in all three artifacts unmasked the sibling assertion in
`🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:182`
(`assert_eq!(actual.tool_ids, expected.tool_ids)`): the 5d fixture listed 9 of the 14 ids in
`PUZZLE5D_RETAINED_TOOL_IDS` (`🖐️5d/…/✏️editor/🦀️.rs:4280`), missing `cycleBrushCandidate`,
`engagementAbort`, `engagementControlSelect`, `engagementInput`, `engagementSubmit`. Same defect
class as 1.3, same fixture family, so it is fixed here rather than deferred.

### 1.5 `window_transient_is_exact_instance_local_and_resets_on_reload` held live reads across close

The wave's own `drain_close` diagnostic (§2) named the cause the bare `Ok(false)` had been hiding:
the test kept four `window_transient_snapshot` reads alive while asking the app to close, and a
partition close is blocked by design while a read lease is outstanding
(`close drained non-terminal while blocked on transient read remains live`). The test, not the
machine, was wrong.

## 2 Changes (file:line)

All paths relative to the repo root. Line numbers are post-change.

### 2.1 Scene payload carries mesh REFERENCES only (1.1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32982` — `world3d_mesh_kind_entry` emits
  `{ id, kind }` instead of `{ id, data }`. Both `world3d_meshes_json_from_kinds` (`:32941`) and
  `world3d_meshes_json_from_kinds_and_urls` (`:32987`) route through it, so every app that names a
  built-in kind is fixed at once. The GLB entry (`{ id, url }`) is untouched.
- Same file, `:32484` — the module's `use semio_framework::mesh_from_kind;` is gone: the plugin host
  no longer tessellates anything at all. W-K's interim `world3d_mesh_kind_aabb` wrapper and its
  re-export (`:33491`) were removed by W-K2 as dead API — the contract lives in the fixture and is
  asserted directly against `mesh_from_kind(...).aabb()` in the mesh engine's own suite.
- `🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🥽️scene-mesh-kinds/🔣️.json` (new) — the
  language-neutral contract: `sceneEntry.requiredKeys` `["id","kind"]`, `forbiddenKeys` `["data"]`,
  and the local-space bounding box of all 11 built-in kinds plus the fallback. The two generators
  tessellate differently on purpose, so the triangle list is deliberately NOT in the contract.
- `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:157` — `WorldMeshRecord.kind`;
  `:1074` `worldMeshKindGeometry` (the R3F engine's own primitives); `:1103` `meshDataFromKind`,
  memoized per kind, computing normals and synthesizing the sequential index run a non-indexed
  primitive owes its downstream readers; `:1123` `parseMeshes` resolves a `{ id, kind }` record into
  the `data` every existing reader already expects. A `{ id, url }` record passes through unchanged.
- No wgpu change was needed: `step_world3d_snapshot`
  (`🧰️framework/…/♾️infinite/🌍️world/🦀️.rs:8866`) already ingests a scene mesh by KEY only and calls
  `begin_world_placeholder_mesh` / `ensure_primitive_mesh` (`:10431`) — it never read inline `data`.

### 2.2 One total render-identity derivation (1.2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23112` — `VcsArtifactApp::live_render_operation`,
  the ONE place the `(app_instance_id, base_revision, generation, canonical_base_revision)` identity is
  derived. `render` (`:24821`) and `pending_effects` (`:24975`) previously each open-coded it; both now
  call it.
- Same file — every remaining host entry point that hands the app an `ArtifactView` now stamps it:
  `window_engagements:24841`, `window_measures:24875`, `tool_measures:24903`, `context_menu:24950`,
  and (added by W-K2 to make the derivation genuinely total rather than merely wider)
  `resolve_domain_topology:20659`, `export_media:24915`, `media_fingerprint:25020`.
- Same file — `ArtifactView::with_children` no longer has a single host caller; the identity-less
  constructor survives only as the GUEST-side composing view a retained worker builds from
  `ArtifactOwnedToolJobContext::children` (`🌊️flow`'s add-widget and host-effect workers are its two
  call sites, `🌊️flow/…/✏️editor/🦀️.rs:1446,1626`). W-K2 first deleted it outright and had to restore
  it — the deletion compiled cleanly against `semio-framework-plugin` and `semio-s-artifact-puzzle-3d`
  because flow is in neither dependency graph; `cargo check -p semio-s-artifact-flow-flow` is what
  catches it. `ArtifactView::render_operation()` is therefore total on every HOST lane, and puzzle 3d's
  `puzzle3d_view_session_key` (`🧊️3d/…/✏️editor/🦀️.rs:2682`) never returns `None` on one.

### 2.3 Fixtures to the real tool set and the real checkpoint width (1.3, 1.4)

- `🧊️3d/…/🧫️fixtures/🗄️retained-jobs/🔣️.json` — `setFillCountStep` removed from `toolIds`;
  `checkpointBytes` `112 → 120` in `capacities` and in the `checkpointMax` / `checkpointCorrupt` /
  `checkpointInterruptedClose` vectors, `113 → 121` in `checkpointMaxPlusOne`.
- `◻️2d/…/🧫️fixtures/🗄️retained-jobs/🔣️.json` — same four `checkpointBytes` edits.
- `🖐️5d/…/🧫️fixtures/🗄️retained-jobs/🔣️.json` — same four `checkpointBytes` edits, plus the five
  missing `toolIds` of 1.4.

### 2.4 Test-side machinery

- `🧊️3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs:499` — `world_surface_payload_bytes` reads the packed
  `Surface.doc.bytes` length back off the BUILT node (never a re-encode) together with
  `UI_FIXED_BYTES`, i.e. exactly the two numbers `scene-surface.encode` compares.
- Same file `:571` — `scene_meshes_of`, the `meshesJson` array of a rendered world body.
- Same file `:233` — `drain_close` now distinguishes a `Blocked { reason }` close from a merely
  non-terminal one and returns the reason as a fault instead of a bare `false`. This is what
  diagnosed 1.5.
- `🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1674` — the four live transient reads are dropped before the
  close witness (1.5).

## 3 Laws

Each defect has one language-agnostic law, and the mesh-reference contract is asserted from BOTH
implementations against the same fixture file.

| # | Law | Where |
| - | --- | ----- |
| 1.1 | `the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity` — opens the vortex suggestion popup, asserts `suggestionMenu.open`, then asserts the rendered world surface payload is within `UI_FIXED_BYTES`, that NO scene mesh carries `data`, that every mesh resolves by `kind` or `url`, that `vortex-marker` (the kind that used to blow the payload) is still declared, and that the whole mesh declaration block stays under `PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET` = 4 KiB. Measured on the live popup-open scene AND on the Nakagin catalog, the widest mesh set the editor can name. | `🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1529`, budget at `:1554` |
| 1.1 | `every_built_in_scene_mesh_kind_matches_the_language_neutral_bounding_box_fixture` — the Rust generator against the fixture's boxes and fallback. | `🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🔬️unit/🦀️.rs:230` |
| 1.1 | `world-3d scene mesh kind references` (3 cases: fixture bounding boxes + non-empty indices + normals-match-positions; unknown kind falls back to the fixture's `fallbackKind`; per-kind memoization) — the TypeScript twin, same fixture file. | `🧰️framework/…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:8583` |
| 1.2 | Covered by the *existing* per-instance session laws, which are what a `None` key broke: they only pass while `render_operation()` is total. There is no *host* call site of the identity-less constructor left. | `🧰️framework/…/🔌️plugin/🦀️.rs:7293` (`with_render_context`) |
| 1.3 | `retained_publication_contracts_are_an_exact_nonempty_tool_bijection` — fixture `toolIds` must equal `PUZZLE3D_RETAINED_TOOL_IDS` exactly and in order. | `🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:222` |
| 1.3/1.4 | `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle` — a serde_json THIRD-PARTY oracle re-reads each fixture and its `checkpointBytes` vectors are compared against `PUZZLE_COMMAND_CHECKPOINT_BYTES`. Runs once per artifact (2d, 3d, 5d). | `🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:225` (assertions at `:176`) |
| 1.5 | `window_transient_is_exact_instance_local_and_resets_on_reload` | `🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1649` |

## 4 Commands + tails

## 5 Not verified

## 4 Commands + tails

Envelope for every Rust run: `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 CARGO_TARGET_DIR=<target-p3d>`,
`-j 4`, foreground, from the repo root. All ran 2026-09-09 (W-K2 session).

### 4.1 Compile gates

```
cargo check -p semio-framework-plugin
    Finished `dev` profile [unoptimized] target(s) in 12.81s        (0 errors, 0 warnings)

cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
    Checking semio-s-artifact-puzzle-3d v0.1.0 (…/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 29.53s        (0 errors)

cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle
    Checking semio-s-plugin-puzzle v0.1.0 (…/🧩️puzzle/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 54.15s        (0 errors; 5 pre-existing
    `unused-qualifications` warnings in puzzle 2d/5d window-transient charging, not this wave's)
```

### 4.2 The laws, filtered

```
cargo test -p semio-framework-mesh-engine -- every_built_in_scene_mesh_kind mesh_from_kind
test tests::mesh_from_kind_maps_known_kinds_and_falls_back_to_box ... ok
test tests::every_built_in_scene_mesh_kind_matches_the_language_neutral_bounding_box_fixture ... ok
test result: ok. 2 passed; 0 failed; 34 filtered out; finished in 0.00s

cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -- --test-threads=1 \
  retained_publication_contracts_are_an_exact_nonempty_tool_bijection \
  language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle \
  the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity
test editor::puzzle3d::component::tests::retained_publication_contracts_are_an_exact_nonempty_tool_bijection ... ok
test editor::puzzle3d::component::tests::the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity ... ok
test retained_command::tests::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle ... ok
test result: ok. 3 passed; 0 failed; 609 filtered out; finished in 0.34s

cargo test -p semio-s-artifact-puzzle-2d --features …/component-app-assembly -- --test-threads=1 retained
test retained_command::tests::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle ... ok
  (9 of the 10 `retained`-matching 2d tests pass; the tenth,
   editor::puzzle2d::engine::brush::tests::board_fill_candidate_acceptance_exposes_every_retained_field_stage,
   is a 2d brush-engine failure in code this wave never touched)

cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly -- --test-threads=1 retained
test editor::puzzle5d::component::tests::retained_publication_contracts_are_an_exact_nonempty_tool_bijection ... ok
test retained_command::tests::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle ... ok
test result: FAILED. 20 passed; 8 failed; 298 filtered out; finished in 3.08s
  (BEFORE the 1.4 fix the oracle law failed with
   left:  ["canvasPointerDown", "worldPointerDown", …]                    ← 9 fixture ids
   right: ["canvasPointerDown", "cycleBrushCandidate", "worldPointerDown", …]  ← 14 production ids
   The 8 remaining failures are 5d import-media / retained-retirement laws that panic in
   `A::register_tool_job_factories` (`🔌️plugin/🦀️.rs:18324`) — 5d-owned, untouched here.)

cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -- --test-threads=1 \
  window_transient_is_exact_instance_local_and_resets_on_reload
test … window_transient_is_exact_instance_local_and_resets_on_reload ... ok
test result: ok. 1 passed; 0 failed; 612 filtered out; finished in 0.47s
```

### 4.3 Whole puzzle 3d suite

Run 1, before the 1.5 fix:

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -- --test-threads=1
test result: FAILED. 589 passed; 23 failed; 0 ignored; 0 measured; 0 filtered out; finished in 57.04s
```

Run 2, after it (the tree grew one more test between the two runs — peers are live in this crate):

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -- --test-threads=1
test result: FAILED. 591 passed; 22 failed; 0 ignored; 0 measured; 0 filtered out; finished in 93.48s
```

vs W-S's handover baseline `581 passed; 30 failed` (611 tests) → **591 passed / 22 failed of 613**.

Every one of the 22 remaining failures is fill/precompute or unrelated-lane work owned by live peer
waves, none in the surfaces this wave touched:

- 18 × `editor::puzzle3d::precompute::component::tests::fill_worker_*` — `PoisonError` on the fill
  envelope test guard and a `259 != 259` credit assertion (wave W-J's bounded fill job, in flight).
- 4 × `fill_build_tick_*` / `fill_count_is_shared_across_split_panes_…` /
  `fill_render_reveals_the_full_available_plan_…` /
  `set_fill_count_clamps_to_available_…` — same lane (`expected a Partial ui_scope for fillBuildTick,
  got None`).
- `gumball_translate_drag_coalesces_into_one_edit` — transform undo coalescing, `[3,0,0]` vs `[0,0,0]`.
- `two_instances_converge_disjoint_object_edits_via_backbone` — `module.vcs` "remote snapshot merge is
  fail-closed until the app-owned streaming envelope decoder … are terminal-authorized".

### 4.4 TypeScript

```
cd 🧰️framework/…/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react
SEMIO_TEST_LEVEL=standard bun x vitest run "🔬️engine-contract" -t "scene mesh kind"
 Test Files  1 passed (1)
      Tests  3 passed | 454 skipped (457)

SEMIO_TEST_LEVEL=standard bun x vitest run "🔬️engine-contract"
 Test Files  1 passed (1)
      Tests  457 passed (457)
   Duration  14.97s
```

`SEMIO_TEST_LEVEL` matters: the default `fundamental` level narrows `include` to the `⚡️quick` suite
and pins `testNamePattern`, so a bare `vitest run 🔬️engine-contract` reports "No test files found".

```
bun x tsc --noEmit -p tsconfig.json        (same directory)
1017 diagnostics, 0 of them in any file this wave touched
(grep -c "World3dHost\|ShellHelpers\|engine-contract" → 0; the 1017 are pre-existing, dominated by
 🧪️tests/🧪️docklayoutstore's `Type 'Command' is not generic` family)
```

## 5 Not verified

- **The two full-suite runs are of a moving tree.** Between them the crate gained a test (612 → 613
  cases) from a peer wave, so the 589/23 → 591/22 delta is not purely this wave's; the per-law runs in
  §4.2 are the attributable evidence.
- **No runtime browser confirmation.** The 1.1 payload claim is proven by the Rust law (the packed
  `Surface.doc.bytes` of a real popup-open render is inside `UI_FIXED_BYTES`) and the two generators
  are proven to agree on the fixture's bounding boxes — but nobody has yet opened the React shell and
  LOOKED at a `vortex-marker` resolved from `{ id, kind }`. The R3F path is asserted only through
  `meshDataFromKind`'s buffers, not through a rendered frame.
- **The wgpu viewport was reasoned about, not run.** `step_world3d_snapshot` demonstrably never read a
  mesh record's `data`, so dropping it cannot regress that renderer; no wgpu render was executed.
  Note in passing that its snapshot path admits every scene mesh as `WorldPlaceholderKind::Box`
  (`♾️infinite/🌍️world/🦀️.rs:8952`) while `ensure_primitive_mesh` (`:10431`) resolves the key properly
  — a pre-existing inconsistency this wave did not touch and did not introduce.
- **No leftover `[DEBUG]` census was found to remove.** W-K's stall message implied one was still in
  the tree; a sweep of every file the wave touched found none. The two `[DEBUG]` prints that DO live in
  the puzzle 3d unit suite (`local interaction read …` and `puzzle3d … section payload is … bytes`) are
  already in `HEAD` and belong to waves I and M.
- **1.4 and 1.5 are outside W-K's original charter.** Both were unmasked by this wave's own changes and
  are the same defect class, so they were fixed rather than deferred; neither was in the handover list.
- **Peer churn.** The plugin host (`🔌️plugin/🦀️.rs`) and `🌐️World3dHost/🟦️.tsx` carry uncommitted hunks
  from waves W-L and W-M2 throughout; only the hunks named in §2 are this wave's, and the measured
  numbers above are of the combined tree, not of this wave in isolation.
- **A guest-side identity gap remains, deliberately unfixed.** `🌊️flow`'s two retained workers still
  build their composing view with the identity-less `ArtifactView::with_children`. They run inside an
  already-admitted operation, so the correct identity for them is the OPERATION, not the render
  context — plumbing it into flow's worker payloads is a change in a peer plugin this wave has no
  mandate for and could not validate. Flow leases no per-instance session off the view, so nothing is
  broken today.
- **`cargo check -p semio-framework-plugin` does not cover every consumer of the plugin SDK surface.**
  Removing a `pub` constructor passed both of this wave's declared check gates and still broke the flow
  artifact, which is in neither graph. Any future edit to `ArtifactView`'s public surface needs at
  least one non-puzzle artifact crate in the check set.

### 4.5 Final re-verification against the tree as it stood at wave end

```
cargo check -p semio-framework-plugin -p semio-s-artifact-flow-flow
    Checking semio-s-artifact-flow-flow v0.1.0 (…/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 25.05s

cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -- --test-threads=1 \
  the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity \
  retained_publication_contracts_are_an_exact_nonempty_tool_bijection \
  language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle \
  window_transient_is_exact_instance_local_and_resets_on_reload
test result: ok. 4 passed; 0 failed; 609 filtered out; finished in 0.44s
```
