# 🌀️ Procedural 3d end to end — status

App under test: `generation3d` of plugin `✏️s/🔌️plugins/🌀️procedural`
(`bun run dev:procedural:3d` → `bun ./📜️script.ts dev procedural 3d` → nx `@semio-tech/framework-os-dev:dev generation3d`).
Ticket start commit: `7ad363fd1ec91cb0c83cf716bc66522be99a4785`.

## Static picture (established, with citations in the sibling notes)

Subset root: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`.

| Question | Answer |
|---|---|
| Modes / windows | `edit` = `procedural-main` (flow, `SurfaceKind::NodeGraph`, 68%) + `procedural-preview` (`SurfaceKind::World3d`, 32%); `generate` = `generation3d-generations` + `generation3d-generate-form` + `generation3d-generate-preview` |
| Document state | `Generation3dSnapshot { fixture: FlowFixture, generation: GenerationPlayRoot }` |
| Flow window non-empty needs | `fixture.widgets` + `fixture.synapses` → `fixture_to_workflow()` → `build_node_graph_scene()` |
| 3d preview non-empty needs | `Generation3dConfig.preview_eval_text`, written only by the `flowEvalTick` command |
| Examples | 8 ids in `🧬️schema/🦀️.rs:275`; default boot parses `hexagonal-mushroom-column` |
| `setActiveExample` | `Migrated` — reachable, arg is a `.select()` over all 8 (editor `🦀️.rs:636-647`) |
| `flowEvalTick` | `Migrated`, declared as an app-level **command** (`:550`), not a window action — present in the descriptor at `manifest/apps[2]/commands[0]` with `interactiveJob: "migrated"`. **Not a defect.** |
| flowEvalTick arming | `Generation3dPlayApp::pending_effects` (`:467`) emits `Effect::DispatchAction{action:"flowEvalTick"}`; host drains it via `response.requested_effects = …pending_effects()` (`🔌️plugin/🦀️.rs:30252`) and the react shell re-dispatches it (`ShellHost/🟦️.tsx:2614`, `:3055`). **Chain is wired.** |
| Snapshot-retirement fault (the puzzle3d blocker) | gen3d builds its store via `from_initialized_runtime_with_owners`, so the factory is installed — that fault should not apply here. Unverified at runtime. |

Both react surface hosts exist and have explicit empty states, which is what "empty window" would look like:
`NodeGraph/🟦️.tsx:1145` → `semio-node-graph-empty`; `World3dHost/🟦️.tsx:5054` → `semio-world-3d-empty`.

## Corrected claim

An exploration agent reported that the TS shell never calls `pending_effects`, which would have left the
3d preview permanently empty. That is wrong: the Rust host calls it once per `refreshUi` pass and ships
the result as `requestedEffects`, which `ShellHost` drains. Verified by reading both sides.

## The one real classification gap

Six editor actions are `BatchOnlyPendingRewrite` and therefore hard-rejected by
`validate_ui_dispatch_classification` before any handler runs (editor `🦀️.rs:599-627`):
`nodeGraphEdit` (600), `addGeneration` (610), `removeGeneration` (611), `renameGeneration` (612),
`updateGenerationValues` (613), `selectGeneration` (624). 23 of 29 are already `Migrated`, including
everything the *render* path and example switching need. See `📓️gap-six-blocked-actions.md`.

Consequence for the goal: the flow window and the 3d preview should RENDER non-empty without touching
these, and examples should load and switch. What is dead is graph *editing* and the generate-mode
generation lifecycle.

## Open at time of writing

Runtime truth. `bun run dev:procedural:3d` was started at ticket open; it sat behind another session's
cargo build-directory lock and is now compiling wasm. Nothing below is claimed until the app is
observed in a browser.

## Root cause (static, verified in framework source — 2026-09-03 16:40)

gen3d's tool proofs (editor `🦀️.rs:210-241`) use the bare
`factory: "BoundedFirstStepCommandJobFactory"` **without** `factory_type`. Consequences, all read from
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:

1. `bounded_first_step_tool_proofs!` (`:12541`) only calls `with_factory_type` when `factory_type:` is given,
   and gen3d never overrides `register_tool_job_factories` (default `:25931` registers nothing).
2. In `VcsArtifactApp` construction (`:19128-19165`) every tool that is in neither `app_tool_registrations`
   nor `framework_tool_registrations` lands in `bounded_tool_proofs` only.
3. `qualified_tool_proof` (`:18983-18995`) returns `Err(interactive-job.missing-owned-reducer)` for a verb
   that exists only in `bounded_tool_proofs`; `dispatch_typed` (`:22638`) → `admit_command_wire` goes through
   it, and `require_complete_tool_operation_pipeline` (`:22398`) would reject a `Bounded` proof anyway.

So **all 23 gen3d actions currently marked `Migrated` are dispatch-dead at runtime**, including
`flowEvalTick` (→ `preview_eval_text` is never written → 3d preview renders its empty state) and
`setActiveExample` (→ examples cannot be switched). This matches the goal's symptoms exactly. Only two apps
repo-wide use the bare factory (gen3d and `📋️forms`); 33 declare a `factory_type`. `generation2d` is the
in-plugin precedent (`Generation2dBoundedCommandJobFactory`, gen2d editor `🦀️.rs:115-203`).

Fix (in flight): give gen3d an app-owned `Generation3dBoundedCommandJobFactory` covering all 29 tools with
honest publication lanes, plus Artifact- and Config-lane `ArtifactStoreOneItemPreparationFactory` impls, and
flip the six `BatchOnlyPendingRewrite` labels once their wiring is real. Plan with literal code:
`📓️plan-migrate-six-actions.md` (§4-§9; note the plan scopes the factory to 6 tools — it must cover all 29).

## Boot attempts (`bun ./📜️script.ts dev procedural 3d`, react renderer, port 7371)

| # | Outcome | Log |
|---|---|---|
| 1 | died at the 20-min `SEMIO_BUILD_BUDGET_MS` default while queued on another session's cargo build-directory lock (`spawnSync bun ETIMEDOUT`) | `🗑️generated/dev-3d-boot-attempt1-etimedout.txt` |
| 2 | engine wasm (`framework_surface`) failed: `semio-framework-os-kernel` 3 × E0599 `set_token`/`mint_session` in `📇️directory/🪪️identity/🦀️.rs` — another session's in-flight `DirectoryClient` refactor (semio-25 / semio-2f / semio-ac all confirmed not theirs; identity was adapted to `DirectoryClient::authenticated` at 16:35 by its owner). On `wasm32-wasip2` the procedural plugin pulls os-kernel via `semio-framework` (`cargo tree -i`), so this blocks the plugin build too, not just engines. | `🗑️generated/dev-3d-boot-attempt2-directory-e0599.txt` |
| 3 | in flight, `SEMIO_BUILD_BUDGET_MS=3600000 SEMIO_PLUGIN_ONLY=procedural` | `🗑️generated/dev-3d-boot.txt` |

Prebuilt engine packages exist for surface/editor (`pkg/` 14:59 today) but not flow-core, so
`SKIP_ENGINE_BUILD=1` is not a shortcut. A Sep 1 procedural component sits in
`🧑️‍💻️dev/🔌️plugin-modules/procedural/` — stale relative to the tree, useful only as a boot smoke.
| 3 | `semio-framework-os-kernel` 1 × E0599: the 16:35 identity rewrite calls `LocalHubCredential::read_inherited`, which is `#[cfg(not(target_arch = "wasm32"))]` in the client — native passes, wasm fails. Fixed by gating `restore_inherited`/`now_ms` in `📇️directory/🪪️identity/🦀️.rs` the same way (zero callers repo-wide; the module documents itself as the native bootstrap). Peers semio-25/-2f/-ac notified. | `🗑️generated/dev-3d-boot-attempt3-read-inherited-e0599.txt` |
| 4 | in flight after the gate | `🗑️generated/dev-3d-boot.txt` |
| 4 | engines surface (13m41s) + editor (4m15s) built clean after the identity gate; flow-core failed on ONE stale error — `curve_ops::closest_point` at stdio `✳️brep/…/📏mass-properties/🦀️.rs:746`, a call site that had already been removed from the tree (file mtime 17:13 > build start; semio-ac's live BREP work). Killed and relaunched. | `🗑️generated/dev-3d-boot-attempt4-stale-closest-point.txt` |
| 5 | in flight | `🗑️generated/dev-3d-boot.txt` |

## Baseline measurement (implementer, 16:53) — the test target was already broken

`cargo test --package semio-s-plugin-procedural --lib generation3d::` does not compile: **606 errors in the
`(lib test)` target before any edit of this ticket** (`🗑️generated/gen3d-tests-baseline.txt`). E0277 ×454
dominate (trait bounds — consistent with the repo-wide serde → `ToValue`/`FromValue` migration), plus
unresolved `protocol::testkit::assert_mutation_diff_absorb_law` / `assert_mutation_inverse_law`,
`ui_wgpu::wgpu::kernel_3d_scene::Mesh3d`, and `Widget`/`FormGeneration` out of scope. By file: gen3d editor
52, `🧩️assembly` inferences 44 + its per-mutation test dirs, gen2d mutations 27 + editor 22, gen3d mutations
13. gen2d files were modified 15:08-15:16 today by another session (staged, not mine).

Also: `cargo tree -i` shows `semio-s-plugin-procedural` depends on `semio-s-plugin-stdio` directly, so every
procedural check/test/wasm build is gated on semio-ac's live BREP refactor in stdio (7 errors at 17:05).

Consequence for sequencing: the runtime goal needs only the lib target + a green stdio; the test target is a
separate repair (gen3d-scope part is ours — see `📓️gap-test-target-606-errors.md` when written).

## Review of the app-owned factory (coordinator, 17:35)

Editor `🦀️.rs` now defines `GENERATION3D_RETAINED_TOOL_IDS` (29, `:165`), `generation3d_bounded_contract/extent/retained_reduce` (`:209-247`),
`Generation3dBoundedCommandJobFactory` (`:248`), `Generation3dArtifactStorePreparationFactory` (`:385`),
`Generation3dConfigPreparationFactory` (`:523`), overrides for `build_*_one_item_preparation_factory`,
`register_tool_job_factories`, `build_tool_job`, `factory_type:` in the proofs block, the six flips, and a
`retained_route_dispositions_are_exact_and_exhaustive` law test. Lane table verified against every handler's
`Emit`: Artifact for `Emit::mutations` handlers (nodeGraphEdit, deleteSelection, removeWidget, moveMediaNode,
addWidget, patchFlowWidgets, reorganize, translate/rotate/scaleSelection), Artifact+Config for
setActiveExample and the four generation mutators, Config for the config-only setters incl. flowEvalTick and
selectGeneration, HostOnly for the two pointer-down effects. No mismatch found. Unverified by compile until
stdio is green.

## Implementer handed over (17:45) — compile verification pending on stdio

`📓️implementation-app-owned-factory.md` documents the change. Edited: gen3d editor `🦀️.rs` and five command
handlers (`🕸️node-graph-edit`, `🧩️delete-selection`, `🧭️translate/rotate/scale-selection`). Only `rustfmt
--check` (parse-clean) and scripted set-equality of tool ids / contracts / macro rows could be run: `cargo
check -p semio-s-plugin-procedural` stops in `semio-s-plugin-stdio` (7 BREP errors, semio-ac's live ticket
26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME). Pending, in order, once stdio is green: native check → wasm32
check → dev boot (attempt 5) → browser verification → `describe` to regenerate `🔣️.json` → registry check.

## Test-target repair (launched 17:52)

`📓️gap-test-target-606-errors.md` maps the gen3d-scope share (220 of 606) to six mechanical causes: test code
still using serde on `ToValue`/`FromValue` types (E0277 ×167), mutation constructors that became enum variants
(E0423 ×14), un-awaited futures in tests (E0599 ×6), `VcsArtifactApp` construction mismatches (E0308 ×16),
renamed `protocol::testkit` law helpers (E0425 ×6), and three scoping issues (`Widget`,
`Generation3dPreviewCamera`, `Mesh3d`). Its "lib target affected" verdict compares baseline line numbers
against the post-edit file (the testkit region moved ~560 lines), so it is unproven; the stdio gate probe
(`🗑️generated/stdio-check-gate.txt`, ends with `exit=<code>`) decides when a real `cargo check` can settle it.
A repair agent is fixing the gen3d-scope errors only; generation2d / 🧩️assembly share the same causes but are
another session's in-flight files and are left alone.
