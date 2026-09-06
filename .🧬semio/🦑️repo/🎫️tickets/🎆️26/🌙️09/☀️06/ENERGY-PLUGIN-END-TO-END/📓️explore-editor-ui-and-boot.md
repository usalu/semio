# 📓️ Energy editor UI + boot path — current state and end-to-end gaps

Read-only exploration. Root: `✏️s/🔌️plugins/🔋️energy`, subset root
`🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. All paths below are relative to the repo root
unless given as absolute. Compared against `✏️s/🔌️plugins/🧩️puzzle` (`◻️2d` artifact) as the working
oracle. Fault taxonomy cross-checked against this ticket's `📓️explore-sibling-recipes.md` §5/§6
(interactive-job dispatch faults, descriptor staleness) — every predicted failure mode in that doc is
independently confirmed here by direct source inspection.

**Correction to the task's assumed layout**: there is no `✏️s/🔌️plugins/🔋️energy/✏️editor` — the editor
lives at the subset root, `…/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`. Also, the zones window
directory is named `📊️zones`, not `🗺️zones`.

---

## 1. Editor surface (`✏️editor/🦀️.rs`) vs puzzle — gap list

File: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`.

### 1.1 Manifest builder (`create_energy_model_editor`, :275-286)

```rust
pub fn create_energy_model_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .icon_id("battery")
        .mode_def(edit::definition())
        .default_mode_id(edit::ENERGY_MODEL_EDIT_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .default_layout(edit::layout())
        .build_definition()
}
```

**Gap A — `EditorBuilder` no longer has most of these methods (repo-wide, not energy-specific).**
`Editor::builder(dialect)` returns `EditorBuilder` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27685`),
whose *entire* inherent `impl EditorBuilder` block (:27728-27752, the only one in the repo — verified
`grep -rn "impl EditorBuilder"`) exposes exactly four sync methods: `document`, `terminology_document`,
`mutation`, `build_definition`. Every other builder method energy calls here — `icon_id` (:4978),
`mode_def` (:5086), `window_kind_def` (:5095), `default_mode_id` (:5033), `default_layout` (:5158),
plus (used by puzzle, not energy) `action_with`, `action_interactive_job` (:5223),
`action_args` (:5210), `interaction` (:5277), `window_kind_interactions` (:5145), `panel_tab_def` (:5102),
`keybinding` (:5181), `artifact_kind` (:4903), `terminology` (:4942) — is declared `pub async fn` **only
on the inner `AppBuilder`**, with no sync `EditorBuilder` wrapper. `EditorBuilder` has no `Deref`/`DerefMut`
to `AppBuilder` either (grep confirms none). Calling an async fn without `.await` from a non-async fn
does not type-check as a chained `Self` return. **This means neither energy's `create_energy_model_editor`
(sync `pub fn`, :275) nor puzzle's `create_puzzle2d_app` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs:2189`,
also sync `pub fn`) can currently compile as written against the current framework — this is very
recent, in-flight framework churn, not an energy-authored bug.** Provenance: `git log` on the framework
file shows the async conversion landed 2026-08-20 (commit `cb9bcce7a4`, "Asyncify framework kernel,
plugin runtime, store, manifest, …") and the file was touched again as recently as 2026-09-05
(`b0dfa0f09b`) — i.e. yesterday relative to this ticket. **Do not treat this as an energy fix; check
with whichever worker (W-A or a peer session) is landing the EditorBuilder sync-wrapper fix before
duplicating effort** (see `feedback-concurrent-cargo-workspace-churn` / `feedback-no-claude-peer-claims-it-is-not-no-owner`
memory notes on attributing repo-wide breakage correctly).

**Gap B — zero `.action_interactive_job(...)` / no factory apparatus in the editor builder at all.**
Unlike puzzle 2d (`✏️editor/🦀️.rs:2267-2304`, dozens of explicit
`.action_interactive_job("<id>", InteractiveJobClassification::Migrated | BatchOnlyPendingRewrite)`
calls) energy's manifest builder never calls `.action_interactive_job` or `.interactive_jobs(...)`.
This is **not by itself fatal** for energy's `structure`/`zones` windows, because those two use the
*generic* framework window kits (`TreeWindowKit`/`TableWindowKit`), whose shared
`window_kind_definition` helper (`🦀️.rs:25730-25749`) stamps `InteractiveJobClassification::Migrated`
on every kit-supplied action (`set-node`, `set-cell`, plus the framework's generic
undo/redo/commitCheckpoint/… history actions) **unconditionally, at the kit level** — confirmed live in
the committed descriptor (`✏️s/🔌️plugins/🔋️energy/🔣️.json`: every action under `windowKinds[0]` and
`windowKinds[1]` has `"execution": "migrated"`). So the structure/zones edit paths *are* classified.
The **simulation** window is different: it hand-declares its own five actions
(`start/cancel/retry/discard/adopt-energy-simulation`) via a local `action()` helper in
`…/🪟️windows/⚡️simulation/🦀️.rs:16-20` that explicitly sets
`action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;` — also correctly
classified, confirmed by that file's own test (`actions_are_localized_and_registered_as_interactive`,
same file, bottom). **So: no `Unclassified`-panic risk from the manifest side.**

**Gap C — total absence of the bounded-factory apparatus, for the simulation actions.**
`grep -rn` across the whole `🔋️energy` tree for `bounded_first_step_tool_proofs!`, `factory_type`,
`PUBLICATION_CONTRACTS`, `register_tool_job_factories`, `ToolJobFactory`, `ArtifactOwnedToolJobFactory`
returns **nothing**. Puzzle 2d has all of it: `PUZZLE2D_RETAINED_TOOL_IDS` (:1072),
`impl ToolJobFactory for Puzzle2dRetainedCommandJobFactory` (:1085),
`impl ArtifactOwnedToolJobFactory` with `const PUBLICATION_CONTRACTS` (:1131-1135), the
`bounded_first_step_tool_proofs! { … factory_type: Puzzle2dRetainedCommandJobFactory, … }` macro
invocation (:2012-2018), `register_tool_job_factories` (:2023), `build_tool_job` (:2028). Per this
ticket's own `📓️explore-sibling-recipes.md` §5.2-§5.3, an action classified `Migrated` **without** a
matching factory/proof either faults `interactive-job.missing-owned-reducer` (bare/absent proof) at
dispatch, or (if there is genuinely no `AppActionRegistry` entry path for it) fails closed with
`interactive-job.catalog-authority`. Energy's five simulation actions are marked `Migrated` but have
**no owning factory at all** — they are very likely dead on dispatch even once the crate compiles,
unless the mounted-job path (`energy_simulation_session::record_event`, called directly from
`ArtifactEditor::handle`, see §1.2) is itself the intended dispatch mechanism and bypasses the
tool-job-factory system entirely (plausible, since `record_event` is invoked synchronously inline in
`handle`, not via a `ToolJobFactory::create_job`) — **this needs runtime confirmation once the crate
compiles; the pattern doesn't match puzzle's retained-command factory shape at all**, so at minimum the
convention is inconsistent with the sibling recipe, and no test in the simulation-session module
exercises the actual dispatch path end-to-end (its tests are pure unit tests on `reconcile`/projection
logic, see §2.3).

**Gap D — `Config`/`Presence`/`DESCRIPTORS` surface: correctly minimal, not a gap.**
`type Config = NoConfig`, `ConfigMutation = NoConfigMutation`, `Presence = NoPresence`,
`PresenceMutation = NoPresenceMutation`, `Transient = NoTransient`, `TransientMutation =
NoTransientMutation` (:122-129) — matches the all-`📌️.empty.md` `🎚️config`/`👥️presence`/`🫧️transient`
directories throughout the editor tree (verified by directory listing: every `🎚️config`, `👥️presence`,
`🫧️transient` folder under `✏️editor/` contains only `📌️.empty.md`). This is consistent, not a gap —
energy genuinely has no per-editor config/presence/transient state yet.

### 1.2 `ArtifactEditor` impl — async/sync mismatch (already flagged by W-A, independently reconfirmed)

`impl ArtifactEditor for EnergyModelEditor` (:119-271) declares `async fn initial_snapshot` (:135),
`async fn command_id` (:155), `async fn handle` (:172), `async fn pending_effects` (:259),
`async fn render` (:263). Per `📓️w1-compile.md` §2 (this ticket, W-A worker), the current
`ArtifactEditor` trait (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26623-26965`) declares
these as **sync** except `command_from_intent` — confirmed against puzzle 2d
(`✏️editor/🦀️.rs:2089`, `fn render(...) -> semio_framework_plugin::UiAssemblyResult<ComponentTree>`,
no `async`). This is a real, already-identified compile blocker; not re-litigated here beyond citing it.

### 1.3 `render()` — three separate `BuiltNode`/`Result` type mismatches

`WindowKit::render` (`🦀️.rs:25756`, trait method) returns `UiAssemblyResult<BuiltNode>` =
`Result<BuiltNode, PluginAssemblyError>` (`:370`), **not** a bare `BuiltNode`. `built_text_node`
(`:357`) returns `Result<BuiltNode, ui_wgpu::wgpu::Label>`, also not bare. `built_to_component_tree`
(`:346`) takes a bare `BuiltNode` argument.

- `structure::render` (`…/🪟️windows/🌳️structure/🦀️.rs:27-77`) is declared
  `pub fn render(document: &EnergyModelSnapshot) -> BuiltNode` but its body's tail expression is
  `TreeWindowKit::render(&TreeView { roots: vec![root] })` (:76) — unwrapped, so this returns
  `UiAssemblyResult<BuiltNode>` where `BuiltNode` is declared. Type mismatch (E0308) if compiled as-is.
- `zones::render` (`…/🪟️windows/📊️zones/🦀️.rs:24-29`) has the identical bug:
  `TableWindowKit::render(&TableView { columns, rows })` (:29) unwrapped, same declared return type
  `BuiltNode`.
- `simulation::render` (`…/🪟️windows/⚡️simulation/🦀️.rs`, near the bottom) does this correctly —
  `TreeWindowKit::render(&TreeView { roots }).unwrap_or_else(|_| built_text_node(...).expect(...))` —
  properly unwraps both `Result`s. This asymmetry (simulation correct, structure/zones not) suggests
  structure.rs/zones.rs predate a `WindowKit::render` signature change to `Result` and were never
  updated, while simulation.rs was authored or fixed after.
- Independently of the above, `EnergyModelEditor::render` itself (editor `🦀️.rs:263-270`) has a fourth
  bug: its `match` arms feed `structure::render(...)`/`zones::render(...)` (whatever type those end up
  being) alongside `simulation_session::with_projection(...)` (delegates to `simulation::render`, which
  is a real `BuiltNode`) alongside the wildcard arm `built_text_node(Label::data(...))` (:268) —
  **unwrapped**, still `Result<BuiltNode, Label>` — into one `match` whose value is passed straight to
  `built_to_component_tree` (:264), which wants a bare `BuiltNode`. Even if structure.rs/zones.rs are
  fixed, this wildcard arm alone breaks the match-arm type unification.

Net: `render()` cannot compile in its current form through at least 3 independent call sites (structure,
zones, and the editor's own wildcard arm), on top of the async/sync issue in §1.2.

### 1.4 Everything else checked and found consistent with the puzzle/remodel shape

`OpText`/`OpBinary` hand-rolled impls for `EnergyModelEditorCommand` (:62-111) mirror the documented
"P6: derive emits `DslVariants` only" convention exactly (matches the doc comment's own citation of
norm/trinity). `mounted_job_maintenance_step`/`mounted_job_close_step`/`mounted_jobs_terminal_is_empty`/
`mounted_job_prepare_snapshot_read` (:139-153) are all thin sync delegations into
`energy_simulation_session`, which is the expected shape for a mounted-job-owning editor.

---

## 2. Windows (`🎭️modes/✏️edit/🪟️windows/{🌳️structure,📊️zones,⚡️simulation}`)

| Window | Kit | Renders | Dispatches | File |
|---|---|---|---|---|
| `structure` | `TreeWindowKit` (generic) | Whole `Model`: `name`/`version`/`site` as edit-target-shaped leaves, plus one read-only count leaf per collection (zones, spaces, surfaces, fenestrations, materials, … 34 collections total) | Generic kit action `set-node` → routed through `SetStructureField` (only `name`/`version` recognized; anything else is a documented no-op, `EnergyModelEditorCommand` doc comment :24-28) | `…/🪟️windows/🌳️structure/🦀️.rs` |
| `zones` | `TableWindowKit` (generic) | One row per `Model::Zone`: columns `id,name,volumeM3,multiplier,conditioned,partOfTotalFloorArea` | Generic kit action `set-cell` → `SetZoneCell` (every column but `id` is a real edit target) | `…/🪟️windows/📊️zones/🦀️.rs` |
| `simulation` | Hand-rolled (own `WindowKindDefinition`, not a kit) | Four-tier progress tree (steady-state/design-day/coarse-timestep/final), `aria-live` status region, operation/generation/checkpoint/fault/final-ready fields, localized (en/de) | Its own 5 actions (`start/cancel/retry/discard/adopt-energy-simulation`) → `EnergySimulationEventKind::{Start,Cancel,Retry,Discard,Adopt}` via `energy_simulation_session::record_event` | `…/🪟️windows/⚡️simulation/🦀️.rs` |

**Simulation ↔ mounted job wiring** (`✳️any/🧵️simulation-session/🦀️.rs`, 2832 lines — note: lives at the
*subset root*, not under `✏️editor/🧵️simulation-session/` which contains only descriptor JSON, no Rust):
`ENERGY_SIMULATION_JOB_KIND = "semio.energy.mounted-simulation.v1"` (:17). Command flow:
`EnergyModelEditor::handle` matches `StartSimulation`/`CancelSimulation`/etc, builds an
`EnergySimulationEventKind`, and calls `simulation_session::record_event(render, event)` (editor `🦀️.rs:216`)
directly — **not** through a `ToolJobFactory::create_job` — before falling through to the normal
document-mutation path for the two field/cell commands. Key entry points, all present:
`record_event` (:1825), `prepare_snapshot_read` (:1928), `reconcile` (:1956, called from
`ArtifactEditor::pending_effects`), `with_projection` (:2154, called from `render`),
`maintenance_step`/`close_step`/`terminal_is_empty` (:2206/:2269/:2311, called from the four
`mounted_job_*` trait methods in editor `🦀️.rs`). **Results are displayed**: `simulation::render` reads
the live `EnergySimulationProjection` (status/tiers/operation/generation/checkpoint/fault/final flags)
via `with_projection`, so once a job actually runs, the window has real UI to show its state — this part
is complete and not stubbed. What's unverified is whether `record_event` alone is sufficient to actually
admit/run/step the mounted job without the bounded-factory dispatch path (§1.1 Gap C) — that's a runtime
question, not resolvable by static reading.

---

## 3. Viewer, examples, TS package

**Viewer** (`✳️any/👁️viewer/🦀️.rs`) exists and mirrors the editor 1:1: same three windows
(`structure`/`zones`/`simulation`, viewer versions under `…/🎭️modes/👁️view/🪟️windows/`), `EnergyModelViewCommand`
has a single inert `Noop` variant (correctly documented as intentional — viewers never mutate), `render()`
has the **same three bugs** as the editor's (`structure::render`/`zones::render` return-type mismatch,
unwrapped wildcard `built_text_node`), and `create_energy_model_viewer` (`🦀️.rs`, bottom) chains
`Viewer::builder(...).icon_id(...).mode_def(...).window_kind_def(...).default_mode_id(...).default_layout(...)`
— same `ViewerBuilder`-missing-methods problem as §1.1 Gap A (its own inherent impl block,
`🦦️.rs:27696-27717`, likewise only has `document`/`terminology_document`/`build_definition`, no
`mutation` by design per its own doc comment, but also no `icon_id`/`mode_def`/`window_kind_def`/etc.).

**Examples**: `📚️examples/🎬️demo/` has `🦀️.rs` (id/label/icon consts, id `"demo"`), `🟦️.ts` (mirrors the
same consts for TS), `🖼️assets/🗣️.dsl.semio` (5-line real DSL asset: `schema=`/`structure=`/`zones=`/
`referencedModel=` hex-encoded pointers), and `🧪️tests/` (both `🦀️.rs` and `🟦️.ts`). The asset is
`include_str!`'d into `SEMIO_ENERGY_MODEL_EXAMPLE_TEXT` (`🧬️schema/📸️snapshot/📝️text/🦀️.rs:12`) and the
module is re-exported through the crate root (`📦️packages/🦀️rust/🦀️.rs:496-499,592-593`). **But neither
`create_energy_model_editor` nor `create_energy_model_viewer` ever references this example module** —
there is no `.example(...)`/examples-list call in either manifest builder, and the committed descriptor's
`manifest.examples` is `[]` (confirmed by parsing `✏️s/🔌️plugins/🔋️energy/🔣️.json`). Per this ticket's
`📓️explore-sibling-recipes.md` §6, the react shell's example picker reads `PluginManifest.examples`
**from the descriptor**, not from a live registry scan — so even once wired, this needs a `describe`
re-run to reach the shell (see §4 below; this is the exact pattern RASTER/REMODEL already hit).

**TS package** (`📦️packages/🟦️typescript/package.json`): already fixed by W-A this session (see
`📓️w1-compile.md` §3) — was a verbatim cad copy, now rewritten to the remodel shape: energy-specific
description, `"dependencies": {}` (verified zero non-relative/non-`node:`/non-`bun:test` imports in
`🟦️.ts` under the plugin), single `test` script targeting `@semio-tech/energy-js:test` (its only
`nx` target). No further TS package gap found.

---

## 4. Boot path

**`bun ./📜️script.ts dev energy`**: resolves through the generic dev script
(`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`) via
`SEMIO_PLUGIN=energy` / `SEMIO_APP=s.energy.model@1/*#editor` env vars — **not** a per-plugin hardcoded
path. Confirmed three matching `.vscode/launch.json` entries already exist for energy:
`🛠️dev🧩️energy⚛️react` (:9717), `🛠️dev🧩️energy🧊️wgpu🌐️wasm` (:9739), `🛠️dev🧩️energy🧊️wgpu🖥️native` (:9761)
— all pointing at `bun ./📜️script.ts dev energy` (react/wasm) or the wgpu-native runner
(`🧰️framework/…/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native energy`), each
with `SEMIO_PLUGIN=energy` and `SEMIO_APP=s.energy.model@1/*#editor` set. **Boot wiring itself is
already in place — nothing to add here.**

**Registry**: the taxonomy file actually used is
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (the path in the task's phrasing,
under `🔌️plugin/📇️registry/`, does not exist on disk — likely stale phrasing, not a real gap).
`🔋️energy` is present in the generic `deployed-module-members` list (`ownerKindIds: ["dev-plugin-outputs",
"dev-extension-install-root"]`, line ~5439) alongside every other plugin (puzzle, remodel, cad, block,
…), and again in the artifact-kind list near line 8157. No energy-specific registry omission found —
this is a flat, generic membership list, not a curated allowlist.

**Descriptors**: `✏️s/🔌️plugins/🔋️energy/🔣️.json` and `🛂️.descriptor.semio` exist at the plugin root.
Directly parsing the JSON descriptor shows it is **stale against current source**:

- `manifest.apps[0].iconId` is `"circle"`, but source (`✏️editor/🦀️.rs:278`) sets `.icon_id("battery")`.
- `manifest.apps[0].windowKinds` has only **two** entries (`framework.window.tree`,
  `framework.window.table`) — the `energy.simulation` window kind, and its five simulation actions, are
  **entirely absent** from the descriptor.
- `manifest.examples` is `[]` — the `demo` example (§3) is not reflected.

Git history dates this precisely: the descriptor JSON's last commit is 2026-09-02 17:38
(`96aa4f8c12`), which **postdates** both the simulation-window source commit (2026-09-02 12:19,
`21fbcd3538`) and the icon-to-battery source commit (2026-09-02 13:31, `e5465a2c1c`) — so the descriptor
was touched *after* both source changes existed but still doesn't reflect them, meaning `describe` was
either not re-run at that commit or the regeneration didn't pick up the changes. Either way: **the
committed descriptor cannot be trusted and must be regenerated via `describe` before any registry/catalog
check is meaningful** — exactly the pattern `📓️explore-sibling-recipes.md` §6 predicted for every sibling
plugin (DRAW, RASTER, REMODEL all had the identical "present but stale" failure mode, which is a **hard
error** under `validateCatalogDescriptorPair`, not just a warning).

**`verify catalog` smoke path**: `VerifyScript.run` (`📜️script.ts`, ~:3013-3029) handles the `catalog`
segment by calling `runCatalogSmokeVerify(studioUrl, {...})`, which boots the studio shell, renders every
registered program, and reports `report.totals.pass`/`fail` plus `report.failedPlugins`. This is the
authoritative end-to-end boot confirmation once the crate compiles and the descriptor is regenerated —
not run in this exploration (read-only, no builds).

---

## 5. Prioritized gap list

**Dead-on-arrival (blocks any boot at all):**
1. Repo-wide `EditorBuilder`/`ViewerBuilder` missing sync wrappers for `icon_id`/`mode_def`/
   `window_kind_def`/`default_mode_id`/`default_layout`/`action_*`/`interaction`/`panel_tab_def`/
   `keybinding`/`artifact_kind`/`terminology` (§1.1 Gap A) — blocks `create_energy_model_editor` **and**
   `create_energy_model_viewer` from compiling. **Not energy-specific**; also blocks puzzle 2d as
   currently written. Check whether another session/worker is already landing the fix before
   duplicating it.
2. `ArtifactEditor`/`ArtifactViewer` async-vs-sync trait mismatch (§1.2) — already tracked by W-A
   (`📓️w1-compile.md` §2), independently reconfirmed here for both editor and viewer.
3. `render()` `BuiltNode`/`Result<BuiltNode,_>` mismatches in `structure::render`, `zones::render`, and
   the editor's/viewer's own wildcard match arm (§1.3) — three independent compile errors once §1/§2 are
   fixed.
4. Committed descriptor (`✏️s/🔌️plugins/🔋️energy/🔣️.json`/`🛂️.descriptor.semio`) stale vs source: missing
   `energy.simulation` window kind entirely, stale icon, empty `examples` (§4) — blocks the react example
   picker and any classification-drift gate until `describe` is re-run.

**Real but not dead-on-arrival (needs runtime confirmation, not just static reading):**
5. No `ToolJobFactory`/`ArtifactOwnedToolJobFactory`/`bounded_first_step_tool_proofs!` apparatus for the
   simulation window's five `Migrated`-classified actions (§1.1 Gap C) — may be fine if
   `energy_simulation_session::record_event`'s direct-call dispatch (bypassing the tool-job-factory
   system) is the intended mechanism for mounted-job actions, but this diverges from every sibling
   recipe pattern and has no end-to-end dispatch test (only unit tests on `reconcile`/projection logic).
   Needs a live dispatch trace once the crate compiles.
6. Examples never wired into either manifest builder (§3) — the `demo` example module/asset exist and
   are `include_str!`'d for the schema/snapshot text surface, but no `.example(...)`-shaped call
   registers it on `create_energy_model_editor`/`create_energy_model_viewer`, so even a correct
   `describe` run has nothing to pick up yet.

**Cosmetic / already fixed:**
7. TS `package.json` cad-copy-paste — already fixed by W-A this session (§3); no action needed.
8. Task-description path drift (`✏️editor` at plugin root vs subset root; `🗺️zones` vs `📊️zones`;
   registry path) — documentation/prompt drift only, not a code gap.
