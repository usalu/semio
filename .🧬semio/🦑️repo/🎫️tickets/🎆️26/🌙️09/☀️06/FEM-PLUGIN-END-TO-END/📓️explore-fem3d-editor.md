# Explore: fem3d artifact + editor (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/`)

Read-only exploration, 2026-09-06. All paths below are relative to the repo root
`/Users/ueli/Documents/semio` unless given absolute. No cargo/bun/nx was run; no fixes applied.

## 1. Mounting — editor/viewer/mode/window modules, and `live_visual`

`Fem3dPlayApp`/`Fem3dViewer` are mounted in the plugin root contract
`✏️s/🔌️plugins/🏗️fem/🦀️.rs:14-16` inside the `FemApps` closed enum:
```
Fem3dEditor(VcsArtifactApp<EditorApp<crate::editor::fem3d::Fem3dPlayApp>>)
Fem3dViewer(VcsArtifactApp<ViewerApp<crate::viewer::fem3d::Fem3dViewer>>)
```
and registered via `.editor_mutation_roster::<Fem3dPlayApp>()` / `.viewer_mutation_roster::<Fem3dViewer>()`
(`✏️s/🔌️plugins/🏗️fem/🦀️.rs:39-42`). `plugin()` also calls
`crate::artifacts::fem3d::live_visual::initialize()` (`🦀️.rs:36`) at plugin build time.

The crate entry `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs` mounts these via `#[path]`:

- Editor module tree — `pub mod fem3d` at line 1449, backed by
  `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs` (the `Fem3dPlayApp` impl).
  Children: `config` (1455, → `✏️editor/🎚️config/🦀️.rs` + `🎚️config/🧬️schema/🦀️.rs`), `presence`
  (1464, → `✏️editor/👥️presence/🦀️.rs` + schema), `wasm` (1473, → `✏️editor/🌉️wasm/🦀️.rs`), `commands`
  (1476-1519, 18 leaf modules, one per `🎮️commands/<slug>/🦀️.rs`), `modes::edit` (1521, →
  `✏️editor/🎭️modes/✏️edit/🦀️.rs`) with `windows::model` (→ `🪟️windows/🧱️model/🦀️.rs`) and
  `windows::results` (→ `🪟️windows/📊️results/🦀️.rs`).
  `live_visual` itself is mounted one level up, directly under `fem3d` (crate entry line 726-727):
  `#[path = "…/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs"] pub mod live_visual;` — i.e. it lives beside
  `standards`, not inside `editor`, even though its source file sits under `✏️editor/🧵️session/`.
- Viewer module tree — `pub mod fem3d` at crate-entry line 1566, backed by
  `👁️viewer/🦀️.rs` (the `Fem3dViewer` impl). Only child: `modes::view` (→
  `👁️viewer/🎭️modes/👁️view/🦀️.rs`) with `windows::model` (→ `🪟️windows/🧱️model/🦀️.rs`). **No
  `windows::results` on the viewer** — the viewer is strictly read-only Model-only, unlike the editor
  which has both Model and Results windows.

### Window content — none are placeholder/empty text

- `✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs` (62 lines): real `render()` (test-only, builds a
  `World3dScene` from `fem3d_scene_parts`) plus the runtime `render_with_progress()` used by
  `live_visual::with_live_visual`. Not a stub.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` (263 lines): real static/modal/buckling analysis
  rendering (`config_result_display`, `fem3d_model_extent`, mode-shape amplitude, von Mises coloring).
  Not a stub.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs` (63 lines): read-only mirror of the editor's Model
  window, hardcoded default camera (`Config = NoConfig`).

None of these window `render()` functions return bare placeholder text. The only
`built_text_node(Label::data(...))` fallback in either file is the `Unknown body: {body_key}` branch
in `👁️viewer/🦀️.rs:88` and the editor's equivalent — a genuine "unrecognized body key" guard, not a
default-path stub.

### `live_visual` — real progressive mesh/solve pipeline, but the base scene is literally `"[]"`

`✏️editor/🧵️session/🦀️.rs` is **4209 lines** — a mounted bounded-job reactor
(`FEM3D_MOUNTED_VISUAL_JOB_KIND = "semio.fem3d.mounted-live-visual"`, line 22) implementing real
coarse/refined meshing, LDLT/PCG/modal solves (`Tet4` elements, `AssemblyJob`, `LdltJob`, `PcgJob`,
`SubspaceIterationJob`) and a `Fem3dVisualState` progression (`Unmeshed → CoarseMesh → RefinedMesh →
Assembling → SolvingUnconverged → SolvingConverged`, line 76-83). This is **not** a stub — it is the
plugin's actual progressive 3D-mesh-preview engine for the wgpu/react `World3d` surface.

However, both the editor's `render_with_progress` (`🪟️windows/🧱️model/🦀️.rs:29-33`) and the viewer's
`render` (`👁️viewer/…/🧱️model/🦀️.rs:35-39`) construct the base scene as
`world3d_scene(camera, "[]".into(), "[]".into(), …)` — **meshes/instances are literally the empty JSON
array** — and only overwrite `scene.snapshot` with `visual.map(Fem3dPageVisualLease::snapshot)`.
`live_visual::with_live_visual` (`🧵️session/🦀️.rs:3601-3613`) returns `build(None)` whenever there is
no render context, no mounted shell for the app instance, or an identity/generation mismatch
(`base_revision`/`generation` must match exactly). So the 3D mesh preview is genuinely empty until the
reactor's background job has meshed the document **and** the render context's identity lines up; a
never-reconciled `reconcile()` effect (`🧵️session/🦀️.rs:3525`, wired via `Fem3dViewer::pending_effects`
at `👁️viewer/🦀️.rs:82-84`) or an app-instance/generation mismatch would silently produce an empty
`World3d` scene with no error — visually indistinguishable from a genuine stub. This is the single
sharpest edge in the whole surface and worth a runtime smoke test (open fem3d, confirm the Model
window actually paints geometry, not just `"[]"`/`"[]"`).

## 2. Mutation kinds — fem3d vs fem2d, per subset

Mutation kinds are **not** filed directly under `🧬️schema/🧬️mutations/<kind>/` at the `🌐️any` subset
(that directory holds only the aggregate `🦀️.rs` enum + `📝️text`/`💾️binary` codec mounts). The 25
individual mutation-kind triads live one level up, in the **domain-specific subsets**
(`🕸️mesh`, `🧱️material`, `🛡️boundary`, `🏋️load`, `📈️analysis`), and the crate entry re-mounts each
kind's `create_node`/`delete_node`/etc. submodule *under* `standards::v1::subsets::any::schema::mutations::<kind>` even though the source files physically live under `subsets::<domain>::…` (see e.g. crate
entry lines 800-822 mounting `🕸️mesh/…/⚪️create-node/` into the `any` mutations module).

| Subset | Kind (25 total, fem3d) | `🔺️diff` | `↩️inverse` | `🧪️tests` cases | `🧬️.schema.json` |
|---|---|---|---|---|---|
| 🏋️load | change-load-case-self-weight | y | y | 1 | y |
| 🏋️load | delete-combination | y | y | 1 | y |
| 🏋️load | add-load | y | y | 1 | y |
| 🏋️load | remove-load | y | y | 1 | y |
| 🏋️load | create-load-case | y | y | 1 | y |
| 🏋️load | create-combination | y | y | 1 | y |
| 🏋️load | delete-load-case | y | y | 1 | y |
| 📈️analysis | update-analysis-settings | y | y | 1 | y |
| 🕸️mesh | replace-element | y | y | 1 | y |
| 🕸️mesh | create-node | y | y | 1 | y |
| 🕸️mesh | delete-section | y | y | 1 | y |
| 🕸️mesh | replace-section | y | y | 1 | y |
| 🕸️mesh | create-section | y | y | 1 | y |
| 🕸️mesh | replace-solid | y | y | 1 | y |
| 🕸️mesh | delete-node | y | y | 1 | y |
| 🕸️mesh | delete-element | y | y | 1 | y |
| 🕸️mesh | delete-solid | y | y | 1 | y |
| 🕸️mesh | create-solid | y | y | 1 | y |
| 🕸️mesh | create-element | y | y | 1 | y |
| 🛡️boundary | replace-support | y | y | 1 | y |
| 🛡️boundary | delete-support | y | y | 1 | y |
| 🛡️boundary | create-support | y | y | 1 | y |
| 🧱️material | create-material | y | y | 1 | y |
| 🧱️material | replace-material | y | y | 1 | y |
| 🧱️material | delete-material | y | y | 1 | y |

**No mutation dir has zero fixture cases** — all 25 have exactly 1 `🧪️tests/<case>/🦀️.rs` case, a
`🔺️diff/`, an `↩️inverse/`, and a `🧬️.schema.json`. fem2d's 25 kinds (verified the same way) are
identically complete.

**Enum vs disk — no drift.** `Fem3dMutation` (`🌐️any/🧬️schema/🧬️mutations/🦀️.rs:29-53`) has exactly 25
variants, one per on-disk kind directory — perfect 1:1 match, no missing/extra variant either
direction. `Fem2dMutation` (same file, fem2d) likewise has 25 variants matching its 25 disk dirs.

**2D vs 3D — the only real domain difference is region vs solid:**
- fem3d has `CreateSolid`/`DeleteSolid`/`ReplaceSolid` (🕸️mesh subset) — **only in 3D**.
- fem2d has `CreateRegion`/`DeleteRegion`/`ReplaceRegion` (🕸️mesh subset) — **only in 2D**.
- Every other kind name (node/element/material/support/load-case/load/combination/
  analysis-settings) is identical between fem2d and fem3d. This is intentional domain parity
  (volume vs area), not an inconsistency.

## 3. Editor action dispatch — mostly unmigrated, one small retained factory

`✏️editor/🦀️.rs` declares 18 actions via `app_commands!` (line 39-58: `Fem3dCommand` enum) and a
matching `.action_interactive_job(...)` manifest entry per action (lines 964-981):

| Classification | Count | Actions |
|---|---|---|
| `Migrated` | **2** | `setCamera`, `setResultDisplay` (both `.view_action(...)`, config-only, never mutate the document) |
| `BatchOnlyPendingRewrite` | **16** | `addNode`, `addBar`, `addFrame`, `addMaterial`, `addSection`, `addSupport`, `addNodalLoad`, `addMemberUdl`, `addAreaLoad`, `addSolid`, `addLoadCase`, `addCombination`, `setSelfWeight`, `setAnalysisSettings`, `removeSelection`, `setActiveExample` |

A `bounded_first_step_tool_proofs!` block (line 679-686) exists and declares
`factory_type: Fem3dRetainedCommandJobFactory`, but its `tools:` list is only `["setCamera",
"setResultDisplay"]` — i.e. the retained-command factory covers exactly the 2 Migrated view actions,
none of the 16 real document mutations. `register_tool_job_factories` (line 687-690) registers this
one factory; `build_tool_job` (line 691+) gates on `FEM3D_RETAINED_TOOL_IDS = ["setCamera",
"setResultDisplay"]` (line 71).

**Comparison with block3d / puzzle3d** (both cited as "recently fixed" precedents):
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: 23 actions,
  **all 23 classified `Migrated`** (lines 999-1021), one `factory_type: Block3dRetainedCommandJobFactory`
  covering the full action set, and a test asserting
  `bounded_first_step_tool_proofs().len() == 23` (line 1157).
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: ~34 actions,
  all `Migrated` except one (`setFixtureJson` stays `BatchOnlyPendingRewrite`), one
  `factory_type: Puzzle3dRetainedCommandJobFactory`.

So fem3d is the outlier: only 2/18 (11%) migrated vs. block3d's 23/23 (100%) and puzzle3d's ~33/34
(97%). **No `interactive-job.missing-owned-reducer`/`missing-factory` risk was found** — the factory
that does exist is correctly wired (registered, gated, tested via
`retained_command_fixture_matches_exact_routes_and_value_codec_boundaries`, line 1046-1060, which
asserts the fixture's `Migrated` route ids equal `FEM3D_RETAINED_TOOL_IDS` exactly). The risk here is
scope, not wiring: 16 real mutating actions have no owned-tool-job path at all and go through the
generic (presumably slower/queued) Batch dispatch.

**Audit cross-check (a discrepancy worth flagging):** the repo-wide per-plugin table
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/📓️explore-per-plugin-blockers.md:49` reports
fem/3d as `Mig E/V = 4/0`, `Batch E/V = 0/16` ("16 viewer dead"). Direct `grep -c` on the actual files
gives editor: 3 raw `InteractiveJobClassification::Migrated` string occurrences (2 real
`.action_interactive_job` calls + 1 inside `Fem3dRetainedCommandJobFactory::classification()`), 16
`BatchOnlyPendingRewrite`; viewer: **0 of either** (`Fem3dViewer` has exactly one `Noop` command and
declares no actions at all, `👁️viewer/🦀️.rs:19-21`). The audit's E/V split for fem3d appears
transposed/stale — the 16 unmigrated actions are in the **editor**, not the viewer, and the viewer has
no actions to migrate. Treat that specific "16 viewer dead" phrase as unreliable for fem3d; the
concrete, source-verified fact is "16/18 editor actions unmigrated."

## 4. Boot snapshot & examples

- **Editor boots empty**: `Fem3dPlayApp::initial_snapshot()` (`✏️editor/🦀️.rs:731-733`) returns
  `crate::artifacts::fem3d::schema::empty_fem3d_snapshot()` unconditionally — a brand-new fem3d editor
  document starts with zero nodes/elements/etc. The user (or `setActiveExample`) must populate it.
- **Viewer boots with the bundled example**: `Fem3dViewer::initial_snapshot()`
  (`👁️viewer/🦀️.rs:70-73`) parses `FEM3D_EXAMPLE_TEXT` via `dsl::parse_dsl`, falling back to
  `empty_fem3d_snapshot()` only if that parse fails. Test `initial_snapshot_is_the_bundled_example_not_empty`
  (line 130-133) asserts `!snapshot.nodes.is_empty()`.
- **Examples inventory**: fem3d ships exactly **one** example, mounted twice in the crate entry under
  `pub mod examples`:
  - `art_3d_demo` → `🗿️artifacts/🧊️3d/…/🌐️any/📚️examples/🎬️demo/🦀️.rs` (artifact-level example, has
    `🖼️assets/🗣️.dsl.semio` + its own `🧪️tests/`).
  - `app_3d_demo_session` → `✏️editor/📚️examples/🎬️demo-session/🦀️.rs` (app/session-level example, has
    `🖼️assets/🎮️.cmd.semio` + its own `🧪️tests/`).
- **Example-switch handler is reachable and tested**: `set_active_example::handle`
  (`✏️editor/🎮️commands/📚️set-active-example/🦀️.rs:23-25`) — `example_id == "default"` parses
  `FEM3D_EXAMPLE_TEXT` and emits `Effect::LoadDocument` (whole-document replace is banned from the
  `Mutation` enum by policy, so this is a load effect, not a mutation); any other id resets to an empty
  document. 4 tests cover it directly, including that it is declared `ActionKind::Mutation` (not a
  View/Shell action) at manifest level (line 62-67). But it is classified
  `BatchOnlyPendingRewrite` (§3 above) — so switching examples goes through the slow/batch path, not
  a retained tool job.
- **Catalog-audit cross-check**: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/📓️explore-per-plugin-blockers.md:49`
  attributes fem/3d's "catalog audit" note to `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-plugin-catalog-completion-audit.md:68`,
  which reads `cad/fem: missing mutation module`. That audit is explicitly self-disclaimed
  (`…terra-plugin-catalog-completion-audit.md:64`): *"recorded Cargo diagnostic files under untracked
  `target/wasm32-wasip2/debug/.fingerprint`… useful source leads but must be rerun in a clean target
  directory… neither success nor failure was established."* Given §2 above shows fem3d's/fem2d's
  mutation module is structurally complete and 1:1 with its enum on disk today, this "missing
  mutation module" note is very likely **stale** — but see §5, which found a real, current,
  independently-reproducible compile hazard in the same module family (broken `#[cfg(test)]` path
  mounts), which is a much more plausible present-day root cause for a Cargo diagnostic that gets
  paraphrased as "missing mutation module" than an actual absent module.
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/📓️opus-fault-discriminators.md` was checked
  and contains **no fem-specific content** — it documents a generic runtime-cleanup fault taxonomy
  (why a spawned window can render `null`/empty on an ABI-mismatch/ceiling-overrun/dead-clock fault)
  that is relevant only in the generic sense that it explains one *other* way a fem3d window could
  legitimately render empty (a plugin-runtime fault), independent of the `live_visual` empty-until-reconciled
  behavior in §1.

## 5. `#[path]`/`include_str!` resolution audit (crate entry → `🧊️3d`)

Method: parsed every `#[path = "…"]` and `include_str!("…")` in
`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs` whose target string contains `🧊️3d`, resolved each
relative to the crate-entry file's directory, and checked existence on disk.

- **`#[path]` into `🧊️3d`: 166 total → 144 resolved, 22 unresolved.**
- **`include_str!` into `🧊️3d`: 0 total** (the crate entry never `include_str!`s anything under
  `🧊️3d` directly — grammar files are `include_str!`'d from *inside* the target `.rs` files
  themselves, e.g. `🧬️mutations/🦀️.rs:15` includes `📖️.grammar.semio` relative to its own directory,
  which does resolve).
- **All 22 unresolved paths are `#[cfg(test)] mod tests_*` mounts for mutation-kind fixture tests** —
  one per mutation kind, e.g.:
  ```
  ../../🗿️artifacts/🧊️3d/…/🕸️mesh/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-the-column-head-node-under-a-live-frame/🦀️.rs
  ```
  On disk, that test-case directory is actually named
  `🚫️removes-the-column-head-056295` — **truncated to a short slug + a 6-hex-digit suffix**, not the
  long descriptive name the crate entry still mounts. This same truncated-name pattern was found for
  22 of fem3d's 25 kinds (only `create-node`, `create-element`, `create-solid` happen to have disk
  names short enough to have survived unchanged) — **and the identical pattern exists in fem2d too**
  (21 of 25 kinds unresolved there, verified separately; see below).
- **fem2d, same check: 167 `#[path]` into `◻️2d` → 146 resolved, 21 unresolved**, all likewise
  `#[cfg(test)] mod tests_*` fixture-case mounts with the identical truncated-name-on-disk pattern
  (e.g. mount expects `…/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-cascading-to-its-support/🦀️.rs`,
  disk has `🚫️removes-node-n3-without-6eab3f`).

**This is a real, present-day, reproducible compile hazard**: a `cargo test`/`cargo check --tests`
build of the fem crate will fail with an unresolved `#[path]` error inside `#[cfg(test)] mod
tests_<slug>` for 43 total mount sites (22 in fem3d + 21 in fem2d) across every mutation-kind module
except three. This matches the memory-noted "Codex Rename-Plan Codemod Incident" pattern (a
repo-wide automated renamer that truncates/hashes long emoji/descriptive directory names) and is a
much better-fitting, currently-verifiable explanation for the "missing mutation module" Cargo
diagnostic referenced in §4 than an actually-absent module.

## 6. Framework-API drift indicators in `🧊️3d/**`

Checked (grep, whole `🧊️3d/…/🪆️subsets` tree) against the drift classes documented in
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️w6-de-async.md` (block's 2125-async-fn
codemod fallout) and `📓️w7b-block3d-compile.md` (block3d's 7 post-de-async compile-error classes):

| Indicator | Count in fem3d | Verdict |
|---|---|---|
| `async fn diff` | 0 | Clean — never had the async codemod (unlike block's 214) |
| `async fn inverse` | 0 | Clean (block had 214) |
| `async fn handle` | 0 | Clean (block had 47) |
| `async fn print_dsl` / `parse_dsl` | 0 | Clean |
| `UiNode` | 2 | Both are `semio_framework_plugin::UiNode` type imports, not a bare-`UiNode`-return drift (fem3d's `render()` correctly returns `UiAssemblyResult<ComponentTree>`, matching w7b class 4's fixed shape) |
| `ui_stack_vertical` | 0 | Not used (not applicable to this surface) |
| `with_ports` | 0 | Not used — no w7b-class-5 `impl Future<Output = AppIo>` pattern present |
| `command_from_action` custom impl | 0 non-test hits | fem3d relies entirely on the `app_commands!` macro's generated dispatch; no hand-rolled `command_from_action`, so no w7b-class-2 `Value` vs `dsl::DslValue` signature risk to check |
| `DESCRIPTORS` on Config/Presence | 2 hits total (framework-generated via `#[derive(dsl::Mutations)]`/`dsl::DslEnum` on `Fem3dMutation`, `Fem3dConfigMutation`, etc.) | No hand-written `Mutation`/`descriptor` impl found missing `DESCRIPTORS` (w7b class 3) — everything derives it |
| bare `fn render(...) -> ComponentTree` (no `Result`) | 1 hit, `✏️editor/🦀️.rs:1032` | This is the **testkit helper** `pub fn render(app: &mut Fem3dApp, body_key: &str) -> String` (returns a JSON string for assertions), not the `ArtifactEditor`/`ArtifactViewer` trait's own `render()` — both real `render()` impls (editor `🦀️.rs`, viewer `🦀️.rs`) correctly return `UiAssemblyResult<ComponentTree>` |

**Conclusion: fem3d shows none of the block-style async-codemod or block3d-style post-de-async
compile-error drift.** It was evidently written against (or already migrated to) the current
framework API conventions. The real defects found here (§3's unmigrated actions, §5's broken test
path mounts) are of a different, unrelated kind.

## Gaps to fix (prioritized)

1. **Broken `#[cfg(test)]` path mounts (§5)** — 43 unresolved `#[path]` sites (22 fem3d + 21 fem2d)
   in the crate entry `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs` point at test-fixture directory
   names that were truncated/hashed on disk but never updated in the mount. This almost certainly
   breaks `cargo test`/`cargo check --tests` for the whole fem crate today. Fix: either update the 43
   `#[path]` strings to match the current on-disk (truncated) directory names, or rename the
   directories back to their descriptive names and update nothing else — pick one direction and make
   them consistent. This is the most concrete, independently-verifiable, highest-leverage fix in this
   report.
2. **16/18 fem3d editor actions are `BatchOnlyPendingRewrite` with no owned tool-job factory (§3)** —
   only `setCamera`/`setResultDisplay` (both non-mutating view actions) are `Migrated`. Every real
   document mutation (`addNode`, `addBar`, `addFrame`, `addMaterial`, `addSection`, `addSupport`,
   `addNodalLoad`, `addMemberUdl`, `addAreaLoad`, `addSolid`, `addLoadCase`, `addCombination`,
   `setSelfWeight`, `setAnalysisSettings`, `removeSelection`, `setActiveExample`) still goes through
   the generic Batch path. block3d (23/23 `Migrated`) and puzzle3d (~33/34) are the reference
   patterns to copy: extend `Fem3dRetainedCommandJobFactory`'s `tools:` list (or add a second
   factory) and flip each `.action_interactive_job(..., BatchOnlyPendingRewrite)` to `Migrated` once
   wired.
3. **`live_visual`'s empty-until-reconciled base scene (§1)** — both editor and viewer Model windows
   build `meshes_json`/`instances_json` as literal `"[]"` and depend entirely on
   `with_live_visual`/`Fem3dPageVisualLease` for actual geometry. A missed `reconcile()` effect
   dispatch, or an app-instance/generation mismatch in `MOUNTED` bookkeeping, silently degrades to a
   visually-empty (but not erroring) 3D preview. Recommend a runtime smoke test (boot fem3d editor and
   viewer, load the `default` example, confirm the Model window actually paints non-`"[]"` mesh/instance
   JSON) since this can't be verified by static reading alone.
4. **Repo-wide per-plugin audit table has an apparent Editor/Viewer transposition for fem/3d (§3)** —
   `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/📓️explore-per-plugin-blockers.md:49`'s "16
   viewer dead" should read "16 editor actions unmigrated; viewer has 0 actions." Low priority
   (documentation-only) but worth correcting so a future reader doesn't chase a nonexistent viewer
   defect.
5. **Stale catalog-audit note** — `…COMPLETE-SEMIO-END-TO-END/📓️terra-plugin-catalog-completion-audit.md:68`'s
   "cad/fem: missing mutation module" is self-disclaimed as an unverified, possibly-stale
   `target/wasm32-wasip2` fingerprint diagnostic. Given §2's clean 1:1 enum/disk match, this note
   should either be re-verified with a fresh `cargo check` or retired/re-attributed to the real
   defect found in §5.
