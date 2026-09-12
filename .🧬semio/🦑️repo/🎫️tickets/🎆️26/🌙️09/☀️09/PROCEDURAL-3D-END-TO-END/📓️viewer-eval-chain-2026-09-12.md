# Viewer Eval Chain and Viewer Reachability — 2026-09-12

Closes `📓️audit-window-inventory-2026-09-12.md` §4 **P0 item 1** (viewer had no extension round-trip)
at the source level with native proof, and **P0 item 2** (viewer role unreachable) at the launch-entry
level. Part (2) of the reachability brief — an in-shell role control — is deliberately **not**
implemented; §6 documents why and what a clean design would be.

---

## 1. The defect, restated

`Generation3dViewCommandWork::step` built a fresh `FlowEvalSession::new()` per command and called
`session.tick(host)` synchronously to completion, with **zero** `ExtensionInvocation` sites anywhere in
`👁️viewer/`. The brep/math operators are CONTRIBUTED by the host at runtime (`setContributions`) and are
never linked into the guest, so that loop could only ever fault: no brep-bearing document could ever
tessellate in the viewer. The editor closed the same defect for generate-mode preview on 2026-09-11
16:46; the viewer was never touched by that lane.

## 2. What changed — the shared chain

A new **surface-neutral** module holds the one `flowEvalTick` law both surfaces now run:

- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` (new), mounted at the ARTIFACT level as
  `crate::preview_eval` (`🧊️generation3d/🦀️.rs`, `#[cfg(feature = "component-app-assembly")]`).
  It sits beside `✏️editor`/`👁️viewer` rather than inside either — the same reason
  `GENERATION3D_DIALECT` lives at the artifact level: `policyViewerPurityBreaches` forbids a viewer
  file reaching through `::editor::`.

Moved there verbatim (one implementation, no twins):

| kind | members |
|---|---|
| payloads | `FlowEvalTick`, `FlowEvalResolve`, `FlowTessellateResolve` |
| addressing | `rearm`, `window_args`, `rearm_attached_previews`, `preview_kind`, `attached_preview_windows`, `may_rearm` (peer lane) |
| geometry | `is_brep_geometry_handle`, `PreviewInlineGeometry`, `PreviewChannelItem`, `preview_channel_items_for_widget`, `widget_previews`, `preview_widget_ids`, `mesh_has_preview_geometry`, `point_marker_mesh`, `vector_marker_mesh`, `apply_show_mode_mesh`, `preview_tolerance`, `PREVIEW_TESSELLATE_STEP_BUDGET`, `GENERATION_3D_GEOMETRY_EXTENSION_ID`, `geometry_extension_address`, `decode_preview_mesh_pack`, `session_preview_mesh`, `mesh_data_for_preview_handle`, `pending_preview_tessellate_handles`, `preview_tessellate_invocations` |
| chain | `FlowEvalTickOutcome`, `evaluate_tick`, `resolve_eval`, `resolve_tessellate` |

`✏️editor/🦀️.rs` now `pub use`s those names so every editor call site (panels, modes, the io bridge)
keeps naming them unqualified, and keeps exactly one editor-owned wrapper —
`preview_tessellate_invocations(…, cfg: &Generation3dConfig)` reading the deflection off its own config
LOD. The three editor command modules (`⏱️flow-eval-tick`, `✅️flow-eval-resolve`,
`🔺️flow-tessellate-resolve`) are now thin bindings; the only editor-only logic left in the tick is the
generate-mode `generation_fixture_for` patch.

`👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` deleted its ~160-line block of "read-only twin"
geometry helpers and imports the shared ones.

## 3. What changed — the viewer surface

| piece | file | note |
|---|---|---|
| retained session | `👁️viewer/🦀️.rs` `Generation3dViewInstanceOperationOwner` + `build_instance_operation_owner` | one `FlowEvalSession` per app instance, with the full maintenance/close ladder — mirrors the editor's |
| window transient | `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🫧️transient/🦀️.rs` (new) | `Generation3dViewPreviewWindowTransientOwner` keyed by `procedural-view-preview`, STATE reusing this surface's own `Generation3dViewTransient`/`…Mutation` (one evaluated-output shape, one set of format leaves), plus `addressed()` and a real byte-weighed `preflight` |
| retirement | `👁️viewer/🫧️transient/🦀️.rs`, `…/🧬️schema/🧬️mutations/🦀️.rs` | `artifact_retire_struct!` + `RetireOwned` so the window owner's `OwnedValueRetirementFactory` can drain a published evaluation under a grant |
| commands | `👁️viewer/🎮️commands/{⏱️flow-eval-tick,✅️flow-eval-resolve,🔺️flow-tessellate-resolve}/🦀️.rs` (new) | `view_commands!` rows; their `ViewEmit` `handle` is the session-free fallback, the served routes do the work |
| routes | `👁️viewer/🦀️.rs` `Generation3dViewFlowEvalWindowWork`, `Generation3dViewFlowResolveWork`, `Generation3dViewFlowEvalJobFactory` (+ proofs) | `flowEvalTick` → `WindowTransient` lane only; both resolves → `HostOnly`. Still read-only: no row names `Artifact`/`Draft` |
| addressing | `retained_window_transient_target`, `GENERATION3D_VIEW_PREVIEW_KINDS`, `generation3d_view_preview_windows` | the tick's window comes from the PAYLOAD, validated against the trusted `ViewModel` roster |
| arming | `pending_effects` (new on the viewer) | attached preview roster + `may_rearm` gate + scratch-session `sync` |
| re-arm | `Generation3dViewCommandWork::step`, `set_contributions::apply` | every view command and the contributions install re-arm every attached preview |
| render | `render_with_request_context` | reads the preview window's OWN transient first (app-level lane as fallback) and lends the retained session into `preview::render` → `preview_payload` |
| meshes | `preview_payload` / `build_preview_mesh_table` | mesh source is now `preview_eval::session_preview_mesh` (the chain's `flowTessellateResolve` pack); the in-process kernel is only the session-free fallback, and `preview_mesh_signature` now also hashes each handle's resolved pack LENGTH so an arriving mesh invalidates the retained table |
| manifest | `create_generation3d_viewer` | the three chain commands declared as hidden `runtime` commands (`in_palette: false`, `Migrated`) — an undeclared command is dropped by `ShellHost`'s `declaredAction` gate before `plugin.handleAction` |
| test-only | `preview::evaluate_fixture` now `#[cfg(test)]` | the compile-time half of "render never evaluates" |

**No synchronous `FlowEvalSession::tick` remains anywhere under `👁️viewer/`** (grep verified; the only
`FlowEvalSession::new()` sites are the retained owner and the `pending_effects` scratch session, both of
which walk the explicit close ladder). `grep -rn '::editor::|\.mutation\(|Emit::mutations|artifact_mutations' 👁️viewer/`
→ clean, so `policyViewerPurityBreaches` still holds.

## 4. Tests

### 4.1 Language-agnostic fixture, now covering both surfaces

`✏️editor/🧫️fixtures/🪟️tick-addressing.json` **moved** to `🧫️fixtures/🪟️tick-addressing.json` (subset
level — it is no longer an editor fixture). Every row gained a `surface` discriminator
(`"editor"` | `"viewer"`), `windowKinds` gained `viewPreview: "procedural-view-preview"`, and five
viewer rows were added: `viewer-no-surface-mounted`, `viewer-preview-attached` (arming);
`viewer-unaddressed`, `viewer-preview-addressed`, `viewer-detached-window-addressed` (dispatch).

Third-party twin `✏️editor/🧪️tests/🔬️tick-addressing/contract.ts` rewritten to assert both surfaces'
rows through `JSON.parse`.

### 4.2 Rust laws

New `👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs` (mounted from `👁️viewer/🦀️.rs`):

- `every_armed_tick_names_a_viewer_preview_window_that_is_actually_attached` — fixture-driven
- `only_a_viewer_preview_addressed_tick_passes_the_retained_preflight` — fixture-driven, through the
  real `handle_command` → wire decode → retained preflight
- `a_viewer_tick_emits_extension_work_or_re_arms_but_never_settles_silently`
- `a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted` — the
  requested equivalent of the editor law, on the UNLINKED served shape, asserting **meshes ≥ 1**

New `🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` — three laws on the shared addressing/LOD surface.

New `👁️viewer/…/👁️preview/🫧️transient/🧪️tests/🔬️unit/🦀️.rs` — three laws on the window-transient owner.

`UnlinkedFlowExtensions` and `staged_flow_extension_contributions_json` moved out of the editor's private
test module into the surface-neutral `🧪️tests/🔬️flow-operators/🦀️.rs` (`crate::flow_operators`) so both
surfaces' late-install laws run the identical guard; the editor testkit re-exports the latter.
Viewer testkit gained `view_shell_view`, `dispatch_effect_command`, `dispatch_with_view`,
`armed_ticks`/`armed_window_ids`, `drain_armed_flow_eval_ticks[_from]`, `render_with_view`,
`preview_mesh_count`.

### 4.3 Commands run, and their results

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- \
  viewer::generation3d::component::eval_chain_tests:: --test-threads=1 --nocapture
```
→ **4 passed, 0 failed.** Key evidence:

```
[DEBUG] extension runner received extension=flow-extension-math capability=evaluate ok=true
[DEBUG] extension runner received extension=flow-extension-brep capability=evaluate ok=true
[DEBUG] extension runner received extension=flow-extension-brep capability=tessellate ok=true   (×3)
[STATS] viewer late install re-armed 5 ticks and painted meshes=3
[DEBUG] viewer preview tick: rearmed=[] answered=2
[DEBUG] viewer tick arming viewer-preview-attached: attached=["view-preview"] armed=["view-preview"]
[DEBUG] viewer tick dispatch viewer-preview-addressed: payloadWindowId="view-preview" refusal=None
[DEBUG] viewer tick dispatch viewer-unaddressed: refusal=Some(… retained command work refused …)
[DEBUG] viewer tick dispatch viewer-detached-window-addressed: refusal=Some(… requires an attached window instance)
```

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- viewer:: preview_eval:: \
  editor::generation3d::component::tick_addressing:: --test-threads=1
```
→ **51 passed, 0 failed** (whole viewer surface + the shared module + the editor's addressing laws).

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- a_late_contributions_install \
  re_pushing_an_unchanged_closure --test-threads=1
```
→ **3 passed, 0 failed** (both surfaces' late-install laws plus the editor's re-push law — proves the
`UnlinkedFlowExtensions` rehome is sound).

```
bun ✏️s/…/✏️editor/🧪️tests/🔬️tick-addressing/contract.ts
```
→ `generation3d tick-addressing generatePreview=generation3d-generate-preview viewPreview=procedural-view-preview editorRows=5/5 viewerRows=2/3`

```
bunx tsc --noEmit --strict … 🔒️preferences/🟦️.ts 🔬️tick-addressing/contract.ts
```
→ exit 0.

```
cargo check -p semio-s-artifact-procedural-generation3d --lib            # default features
RUST_MIN_STACK=33554432 cargo check … --features component-app-assembly --lib --profile test
```
→ both clean (no new warnings from this lane).

Raw output: `🗑️generated/viewer-lane/`.

### 4.4 Full-suite failures that are NOT this lane

`cargo test … --lib -- --test-threads=1` → **354 passed, 10 failed**. Every failure was triaged; none is
caused by this lane, and all were reproduced in isolation:

| failing test | attribution |
|---|---|
| `…component::tests::refresh_pending_effects_arms_flow_eval_tick_chain` | **wall-clock, pre-existing.** Fails at exactly 32 s with "typed operation did not retire within 30 seconds" on the FIRST tick. Measured independently: `cargo test --test example-geometry -- sphere_cut_with_torus_evaluates_to_the_difference_volume` takes **29.84 s** in a debug build — this test drives that same boolean cut through a route with a hard 30 s retire deadline. Deterministic across three runs; nothing in the chain body changed (`git diff` of the tick shows only the move + the peer `may_rearm` gate). |
| `…tests::generation_preview_is_one_app_transient_shared_by_two_generation_windows` | same deadline class ("preview operation did not finish") |
| `…commands::flow_eval_tick::tests::an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain` | **peer lane, in flight.** A concurrent session added `may_rearm` + `🧫️fixtures/🚧️contribution-gated-arming.json` at 01:24–01:25 today; this is their own new test |
| `add_generation_records_an_undoable_generation_operation`, `undo_redo_round_trips_flow_graph_edits`, `two_instances_converge_disjoint_widget_moves`, `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | **peer lane.** All four assert inside `🧰️framework/…/🔌️plugin/🦀️.rs:6866` ("undo did not revert to the expected snapshot"); that framework file is uncommitted-modified by a peer |
| `fold_contract::interaction_select_publishes_through_the_retained_typed_path`, `work_capacity::interaction_select_passes_the_reserved_preflight` | **peer lane.** The selection-command lane (`{translate,rotate,scale,delete}-selection` + new test dirs) is uncommitted-modified by a peer |

The one genuine finding of mine in that run — `every_viewer_tool_id_is_declared_in_all_four_tables`
missing the new chain tool ids — was fixed (the law now chains all three tool-id lists and all three
factories' publication contracts) and passes.

---

## 5. Task B (1) — launch rows for the viewer role

### 5.1 The missing projection

The shells read `VITE_SEMIO_APP_ROLE` (`🐚️Shell/🟦️.tsx` `resolveBootAppRole`, `🧑‍💻dev/🟦️.ts` `appRole`),
but `frameworkOsLockedPrefsEnv` — the declared process-env → `VITE_*` projection table that BOTH
`serve` and `dev` apply — had **no** row for it. A launch entry therefore could not reach the role at
all; only hand-exporting the `VITE_`-prefixed name before starting the dev server worked. Added,
schema-first:

- `🎮️playground/🔒️preferences/🔣️.json` — `"SEMIO_APP_ROLE": "VITE_SEMIO_APP_ROLE"`
- `…/🧬️schema/🔣️.json` — `VITE_SEMIO_APP_ROLE` property
- `…/🧫️fixtures/🔒️playground-preferences/🔣️.json` — `" viewer "` → `"viewer"` (also proves trimming)
- `…/🟦️.ts` — `SEMIO_APP_ROLE_ENV` const + docstring

`testPlaygroundPreferences` (lodash oracle + jsonschema) → PASS.

### 5.2 Seed rows

`.vscode/🧩️launch.seed.jsonc`, group `3_dev`, immediately after the two existing
`🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column…` rows (orders 200 / 200.1):

| name | order | port | env | opens |
|---|---|---|---|---|
| `🛠️dev🔧️procedural🏙️3d👁️viewer⚛️react` | 200.2 | 6018 | `SEMIO_RENDERER=react`, `SEMIO_APP_ROLE=viewer` | `…:6018/?plugin=generation3d` |
| `🛠️dev🔧️procedural🏙️3d👁️viewer🧊️wgpu🌐️wasm` | 200.3 | 6118 | `SEMIO_RENDERER=wgpu`, `SEMIO_APP_ROLE=viewer` | `…:6118/?plugin=generation3d&role=viewer` |

Both keep the neighbours' `command` (`bun nx run workspace:dev -- procedural 3d`), `cwd`, group and
`serverReadyAction` shape; naming follows the seed's `👁️` role convention placed where the existing rows
put their `🎛️`/renderer segments. The wgpu row additionally pins `&role=viewer` on the opened URL because
the wgpu browser boot reads the role from the QUERY STRING, not the env
(`🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:50`) — so that row is belt-and-braces and the
react row is env-only.

### 5.3 Regeneration

```
bun nx run @semio-tech/plugin-registry:generate
```
→ `plugin registry catalog refreshed (59 plugin crates, 61 playgrounds, 47 framework packages)` /
`.vscode/launch.json regenerated`. `.vscode/launch.json` was **never hand-edited**.

Verified: `Bun.JSONC.parse` → **2484 configurations, 2484 unique names**; the only `"name"` additions in
`git diff .vscode/launch.json` are the two rows above (the other hunks — `🛠️dev🔺️trinity🃏️jack🦀️shell`,
`⚖️gate📐️cad🧠️brep-invoke🦀️check`, `🏛️bestest🔋️energy♻️regenerate` — are peers' seed edits propagating
through regeneration and were preserved, not reverted). Ran `generate` a second time: `launch.json`
sha256 unchanged → **regeneration-stable**.

### 5.4 Gates that fail for reasons outside this lane

- `bun nx run @semio-tech/plugin-registry:check` → fails on `duplicate playground port 6019 (assembly
  react and shooting react)` / `6119 (assembly wgpu / shooting wgpu)`. A pre-existing peer registration
  collision; my rows use 6018/6118 and are not named. `generate` itself succeeds.
- `bun ./📜️script.ts verify interactivity apps` → 782 failures, headed by
  `.vscode/launch.json: 2484 configurations exceed fixed capacity 512` and four missing
  `⚖️gate⚡️interactivity*` registrations. Zero of the 782 mention `generation3d` or `procedural`
  (`grep -c` → 0). Pre-existing repo-wide state.

## 6. Task B (2) — why there is no in-shell role control (and what a clean one would be)

**There is no clean existing notion of switching the active app inside a playground plugin session, so
nothing was hacked in.** Evidence:

- `ShellHost/🟦️.tsx:4841 switchToManagedApp(appId, viewState?)` IS the clean mechanism — it
  `handle.createApp(app.id)`s a second instance on the SAME plugin handle, re-seeds the layout through
  `applyFrameworkLayoutSeed`, and `SET_SESSION`s. But its first line is
  `const sPlugin = hostConfig ? loadedPlugins.find(e => e.handle.pluginId === hostConfig.pluginId) : undefined`
  and it returns `null` when that misses. It is scoped to the HOST plugin's landing/host apps only.
- `hostConfig = pluginFilter ? resolvePluginHostConfig(PLUGIN_CATALOG, pluginFilter) : undefined`
  (`:1859`) resolves against `catalog.hosts`. `generation3d` is not a host, so `hostMode === false` and
  `switchToManagedApp` is dead in this playground.
- `establishPrimarySession` (`:3320`) picks the app exactly once at boot — `appId` pin, else the
  playground default app id, else the first app matching `appRole`, else `apps[0]` — and nothing ever
  re-picks it for a non-host session.
- `applyShellUri` (`:5422`) can route by `(dialect, role)` via `findDialectApp`, but only inside
  `hostMode`.

A control that switched apps today would therefore have to reach around all of that, and the blast
radius is real: `session.app.role` gates VCS check-in (`:8127`, `:8316`), mutation-command filtering
(`:8500`, `:8585`), the viewer mutation-action refusal (`:6145`), and `canonicalSurfaceId(...)` scope
validation (`:2590`).

**Clean design, for a later lane:**

1. Generalise `switchToManagedApp` into `switchToPluginApp(pluginId, appId, viewState?)` that resolves
   the plugin from `loadedPlugins` rather than from `hostConfig`, and keep the host-only bookkeeping
   (`openSpaceIdRef` / `openInstanceIdRef` reset, `landingAppId` branch) behind an explicit
   `hostMode` guard inside it. `reloadRetainsActiveApp` already assumes the active app is just "one of
   `manifest.apps`", so the invariant is already app-id-shaped, not host-shaped.
2. Retire the previous instance through the existing close ladder before `SET_SESSION` — the current
   host path leaks one `createApp` per switch, which is tolerable for two host apps and is not for a
   user-facing toggle.
3. Surface it as a navbar `ButtonGroup id="playground.navbar.roles"` with
   `playground.navbar.roles.editor` / `playground.navbar.roles.viewer`, sitting beside
   `playground.navbar.modes` (`ShellHost/🟦️.tsx:8423-8429`), rendered only when the loaded plugin
   declares more than one app for the OPEN document's dialect (`findDialectApp` for both roles).
   Localised en/de from the `AppDefinition`'s own labels, and — unlike the existing mode buttons, which
   §3.3 of the inventory audit flags as mouse-only — registered as a real app keybinding so mode and
   role switching both get a keyboard path (CLAUDE.md accessibility).
4. Make the React dev entry read `?role=` as well, so the two renderers agree: the wgpu boot already
   does (`🚀️browser-boot/🟦️.ts:50`), the React entry does not. That alone would make
   `?plugin=generation3d&role=viewer` work on 6018 without any launch row — a strictly smaller change
   than (1)–(3) and a reasonable first step.

Until then the viewer is reachable by the two launch rows in §5.2.

---

## 7. Open items

1. **Runtime (browser) proof is still missing** for the viewer chain, exactly as
   `📓️audit-window-inventory-2026-09-12.md` §4 P0 item 3 says for the editor: this lane restaged
   nothing and started no dev server (the coordinator owns restaging). The native evidence is
   `meshes=3` over a real `evaluate`+`tessellate` extension round trip on the unlinked served shape,
   which is the strongest signal available without a wasm rebuild.
2. `refresh_pending_effects_arms_flow_eval_tick_chain` and
   `generation_preview_is_one_app_transient_shared_by_two_generation_windows` fail against the 30 s
   retained-operation deadline because `sphere-cut-with-torus` alone costs 29.84 s in a debug build.
   Either budget them explicitly or move them to a cheaper example — not fixed here (editor lane).
3. The peer `may_rearm` lane's own test and the selection/undo lanes' tests were failing when this lane
   ran; left untouched per the concurrency rule.
4. `@semio-tech/plugin-registry:check` stays red until the `assembly`/`shooting` 6019/6119 port
   collision is resolved by whoever registered `shooting`.
5. Generate-mode preview still has no gumball trio (inventory audit P1 item 4) — unchanged.

## 8. Files created / changed

**Created**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`
- `…/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs`
- `…/✳️any/🧫️fixtures/🪟️tick-addressing.json` (moved from `✏️editor/🧫️fixtures/`)
- `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🫧️transient/🦀️.rs`
- `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🫧️transient/🧪️tests/🔬️unit/🦀️.rs`
- `…/✳️any/👁️viewer/🎮️commands/⏱️flow-eval-tick/🦀️.rs`
- `…/✳️any/👁️viewer/🎮️commands/✅️flow-eval-resolve/🦀️.rs`
- `…/✳️any/👁️viewer/🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs`
- `…/✳️any/👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs`

**Changed**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs` — mounts `preview_eval`, the three viewer commands, and the preview window's `transient` submodule
- `…/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs` — `staged_flow_extension_contributions_json`, `UnlinkedFlowExtensions`
- `…/✳️any/✏️editor/🦀️.rs` — re-exports the shared members, keeps the config-LOD `preview_tessellate_invocations` wrapper
- `…/✳️any/✏️editor/🎮️commands/{⏱️flow-eval-tick,✅️flow-eval-resolve,🔺️flow-tessellate-resolve}/🦀️.rs`
- `…/✳️any/✏️editor/🧪️tests/{🔬️testkit,🔬️unit,🔬️tick-addressing/🦀️.rs,🔬️tick-addressing/contract.ts}`
- `…/✳️any/👁️viewer/🦀️.rs`
- `…/✳️any/👁️viewer/🎮️commands/🧩️set-contributions/🦀️.rs`
- `…/✳️any/👁️viewer/🫧️transient/🦀️.rs`, `…/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs`
- `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs`
- `…/✳️any/👁️viewer/🎭️modes/👁️view/🧪️tests/👁️preview/🔬️unit/🦀️.rs`
- `…/✳️any/👁️viewer/🧪️tests/{🔬️testkit,🔬️unit}/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json,🧫️fixtures/🔒️playground-preferences/🔣️.json}`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json` (regenerated only)

**Deleted**
- `…/✳️any/✏️editor/🧫️fixtures/🪟️tick-addressing.json` (moved to the subset level)
- `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🫧️transient/📌️.empty.md` (the directory is real now)
