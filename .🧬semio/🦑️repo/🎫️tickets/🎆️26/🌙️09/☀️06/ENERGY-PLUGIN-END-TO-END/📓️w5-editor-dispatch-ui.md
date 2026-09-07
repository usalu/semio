# 📓️ W-E — editor/viewer surface, dispatch apparatus, windows, examples

Worker W-E. Subset root `A` = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.
Live logs: `<scratchpad>/w5-check-*.txt`. Everything below is read off the current framework source or
off a pasted command run — no claim here is inferred from the exploration reports alone.

---

## 1. Framework facts that changed the plan (all verified in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`)

### 1.1 The `EditorBuilder`/`ViewerBuilder` gap is already closed upstream
`📓️explore-editor-ui-and-boot.md` §1.1 Gap A reports that `EditorBuilder`'s only inherent impl exposes
four methods and that neither energy's nor puzzle's manifest builder can compile. That is no longer
true: `:27753-27827` now carries a `surface_builder_forward!` macro that generates a sync forwarding
wrapper on BOTH `ViewerBuilder` and `EditorBuilder` for ~45 `AppBuilder` methods — `icon_id`,
`mode_def`, `window_kind_def`, `default_mode_id`, `default_layout`, `window_kind_actions`,
`keybinding`, `action_interactive_job`, `interactive_jobs`, `interaction`, `panel_tab_def`,
`artifact_kind`, `terminology`, … Each forwards through `resolve_ready(self.inner.<method>(…))`.
**Nothing for energy (or this ticket) to do there.**

### 1.2 `Migrated` alone is NOT enough — every UI-dispatched verb needs an app-owned factory
`VcsArtifactApp::dispatch_action` (`:22399`) routes every non-framework-reserved action through
`admit_command_json` → `require_complete_tool_operation_pipeline` (`:23080`), which accepts ONLY
`QualifiedToolProof::AppOwned`; a `FrameworkOwned` or `Bounded` (generic) proof is refused with
`interactive-job.missing-owned-reducer`, and no proof at all with `interactive-job.missing-factory`.
So the editor report's §5 item 5 ("may be fine if `record_event`'s direct call is the intended
mechanism") resolves **negative**: the five simulation actions were dead on dispatch. There is no
"mounted-job bypass" — `record_event` is called from inside `handle`, and `handle` is only ever
reached through this pipeline.

### 1.3 The proof set must equal `TOOL_JOB_IDS ∩ migrated`
`AppActionRegistry::validate_tool_job_rows` (`:12233`) computes
`expected = <A::Command as protocol::OpBinary>::TOOL_JOB_IDS ∩ migrated_tool_ids()` and requires the
`bounded_first_step_tool_proofs!` rows to be exactly that set (`seen != expected` ⇒
`interactive-job.catalog-incomplete`). Energy's hand-written `impl protocol::OpBinary` never declared
`TOOL_JOB_IDS`, so it inherited the trait default `&["typed-command"]` — which the filter drops —
making `expected` empty. **Any** proof row would have been rejected with
`interactive-job.catalog-authority`. The `app_commands!` macro (`:10740`) declares this const for
generated command enums; a hand-written enum must declare it too.

### 1.4 The two framework-kit actions are part of that set
`window_kind_definition` (`:25729`) stamps `InteractiveJobClassification::Migrated` on every action a
kit supplies, unconditionally. `TreeWindowKit::editable_window_kind` supplies `set-node` (`:26147`)
and `TableWindowKit::editable_window_kind` supplies `set-cell` (`:25834`). Both therefore need a
proof and a factory registration exactly like the authored verbs.

### 1.5 Declaring the `Artifact` publication lane requires a preparation factory
`VcsArtifactApp`'s construction (`:19560-19579`) marks a registration `unsupported` — and
`qualified_tool_proof` then refuses it with `interactive-job.publication-authority-missing` — when a
tool declares `ArtifactToolPublicationLane::Artifact` while
`A::build_artifact_store_one_item_preparation_factory()` is `None`. `ArtifactToolFactoryRegistry::register`
(`:12894`) additionally rejects an empty or omitted `PUBLICATION_CONTRACTS` with
`interactive-job.publication-contract`, and a factory whose `classification()` is not `Migrated` with
`interactive-job.owner-classification`.

### 1.6 A latent runtime bug in the old `handle`: `render_operation()` is `None` during dispatch
`ArtifactView::render_operation` (`:8238`) returns the field set only by `with_render_context`
(`:8223`), which the framework builds on the RENDER path (`:25353`). The dispatch path builds the view
with `with_dispatch_context`, which sets `operation` and leaves `render_operation: None`. The previous
`handle` did `doc.render_operation().ok_or_else(…)` before calling `record_event`, so **every**
simulation action would have faulted `energy simulation command lacks document operation context`
even after the crate compiled and the factory existed.
The identity is reconstructible exactly, because the framework derives it the same way at render time
(`:25351-25357`): `base_revision` = first eight big-endian bytes of the canonical revision,
`generation` = the store generation the operation was admitted against. That is now
`energy_simulation_session::render_identity_of(&AppOperationContext)`.

---

## 2. What was implemented

### 2.1 `A/🧵️simulation-session/🦀️.rs` — run settings as event-sourced ephemeral local-only state
- `EnergySimulationEventKind::Configure { config }` added; `record_event` validates it through the
  same `EnergySimulationConfigProjection::validate` the `Start` event uses.
- `Registry.settings: [EnergySimulationConfigProjection; ACTIVE_SLOTS]`, reset with the slot on close.
- `EnergySimulationConfigProjection::DEFAULT` — a `const` twin of `SimulationConfig::default()`'s three
  session fields (60 / 60 / 7), needed because the fixed arena cannot call a non-const `Default`.
  `Default::default()` now delegates to it, so the two cannot drift.
- `session_settings(render)` — the addressed instance's live settings, read by the window and by
  `start-energy-simulation`.
- `render_identity_of(operation)` — §1.6.

The run period and the schedules deliberately stay OUT of this: W-D0 moved them into `Model`, so they
are document data edited through a mutation (`set-run-period` → `update-run-period`), not session state.

### 2.2 `A/✏️editor/🦀️.rs` — the full bounded-factory apparatus
- `ENERGY_MODEL_RETAINED_TOOL_IDS` — 18 verbs (2 kit + 10 authored document + 6 session).
- `ENERGY_MODEL_DOCUMENT_TOOL_IDS` — the 12 that publish into the document store.
- `impl protocol::OpBinary for EnergyModelEditorCommand { const TOOL_JOB_IDS = ENERGY_MODEL_RETAINED_TOOL_IDS; … }`.
- `EnergyModelCommandJobFactory` implementing `semio_framework::ToolJobFactory`
  (payload `ArtifactRetainedCommandPayload<EditorApp<EnergyModelEditor>>`, job
  `ArtifactRetainedCommandJob<…>`, classification `Migrated`, contract
  `bounded_first_step(65_536, 65_536, 1, 262_144, 7_500)`) **and**
  `semio_framework_plugin::ArtifactOwnedToolJobFactory` (`Owner = EditorApp<EnergyModelEditor>`,
  `TOOL_IDS`, `DOCUMENT_SCHEMA = "energy.model"`, and 18 non-empty `PUBLICATION_CONTRACTS`: `Artifact`
  for the twelve document verbs, `HostOnly` for the six session verbs).
- `bounded_first_step_tool_proofs! { … factory_type: EnergyModelCommandJobFactory, tools: [ …18… ] }`
  — a real `factory_type`, never a bare proof.
- `register_tool_job_factories` and `build_tool_job` (tool/command match check, extent guard, payload
  built with `try_new_with_context` so the retained work carries the same operation authority).
- `build_artifact_store_one_item_preparation_factory` → `EnergyModelStorePreparationFactory`, the
  document lane's one-item retained preparation (§1.5), with its `advance`/`checkpoint`/`take_prepared`/
  `cancel`/`close_step`/`terminal_is_empty` state machine and `energy_model_retained_edit`.
- `create_energy_model_editor` classifies every id in the roster `Migrated` in one loop, so a new verb
  cannot be added to the roster and left `Unclassified`.

**Decision on the mounted-job path** (the brief asks for it explicitly): the five simulation actions
go through the SAME bounded-factory apparatus as everything else, and `record_event` stays where it
is — called from the shared `reduce`, now with a reconstructed render identity. Rationale: §1.2 proves
there is no second dispatch route to choose between, and the sibling recipe (`remodel`, `puzzle`) puts
host-only session verbs on the same factory with a `HostOnly` publication lane, which is exactly what
these are.

### 2.3 Editor commands for the model
`EnergyModelEditorCommand` grew from 7 to 18 variants; every DSL wire key is IDENTICAL to its manifest
action id, so `command_id`, the roster, the classification loop and the proof rows cannot drift by a
rename. `command_from_action` is implemented through an `args_bridge` module (the remodel shape:
numbers also accepted as numeric strings, so a select-sourced argument works).

| Action id | Payload | Ledger mutation kind | Landed? |
|---|---|---|---|
| `set-node` | `field,value` | `rename-model` / `change-model-version` | ✅ |
| `set-cell` | `row,column,value` | `rename-zone`/`change-zone-{volume,multiplier,conditioned,floor-area-participation}` | ✅ |
| `create-zone` | `name,volumeM3,multiplier,conditioned` | `create-zone` | ⏳ |
| `rename-zone` | `zone,newName` | `rename-zone` | ✅ |
| `delete-zone` | `zone` | `delete-zone` | ⏳ |
| `create-surface` | `name,zone,construction,class` | `create-surface` | ⏳ |
| `delete-surface` | `surface` | `delete-surface` | ⏳ |
| `assign-surface-construction` | `surface,construction` | `change-surface-construction` | ⏳ |
| `set-material-property` | `material,property,value` | `change-material-*` (7 scalars) | ⏳ |
| `set-thermostat-setpoints` | `thermostat,heating/coolingSchedule,throttle ranges` | `change-thermostat-*` | ⏳ |
| `set-site` | 5 site scalars | `update-site` | ✅ |
| `set-run-period` | `startMonth/Day,endMonth/Day` | `update-run-period` | ✅ |
| `start/cancel/retry/discard/adopt-energy-simulation` | session identity | — (session) | n/a |
| `configure-energy-simulation` | `locale,zone/systemTimestepMinutes,warmupDays` | — (session) | n/a |

⏳ = the command is wired against the ledger name but the semantic leaf has not landed yet, so W-D0's
`model_edit` seam refuses it LOUDLY with `mutation.kind-unavailable` naming the missing kind. This is
W-D0's own design (no whole-document replace to fall back on, `📓️derivation-rules.md` rule 6) and is
recorded as a test (`verbs_awaiting_their_semantic_kind_refuse_loudly_and_name_it`) that flips to a
round-trip assertion the moment the owning group lands its leaves. **Group workers: the kinds this
editor is waiting on are exactly the ⏳ rows above.**

Domain guards that run BEFORE the seam, so they hold whatever the vocabulary state:
`mutation.target-missing` (unknown zone/surface/construction/material/thermostat/schedule id),
`mutation.target-in-use` (a zone still referenced by a space, surface or thermostat is refused rather
than cascaded — `📓️explore-mutation-vocabulary.md` §4.2's `RESTRICT` recommendation),
`mutation.invalid-payload` (non-positive volume/thickness/conductivity/density/specific heat,
out-of-[0,1] absorptance, latitude/longitude outside their SI range, a run period addressing a
non-calendar month or day, an unknown surface class or material property).

Keybindings (puzzle/remodel pattern, `.keybinding(keys, action)`): `mod+shift+n` create-zone,
`mod+shift+s` create-surface, `mod+shift+g` set-site, `mod+enter` start, `mod+period` cancel,
`mod+shift+enter` adopt.

i18n: every authored action and every argument carries `LocalizedLabel::native(en, de)` with no
default; three tests assert `native() != secondary()` for every label and every argument label, in the
editor root, the structure window and the zones window.

### 2.4 Windows
- `🌳️structure` and `📊️zones` now OWN their authored actions (`actions()`), and their `definition()`
  APPENDS them to the kit's own action list. This is load-bearing: `AppBuilder::window_kind_actions`
  (`:5117`) **replaces** a window's action vector, so composing in the manifest builder would have
  silently dropped `set-node`/`set-cell`. Both windows assert the kit action survived.
- Both `render` functions now return `UiAssemblyResult<BuiltNode>` (editor report §1.3) — in the
  editor AND the viewer.
- `⚡️simulation` rewritten: six actions (the five session verbs + `configure-energy-simulation`,
  each with typed `ActionArgDef`s), a localized `stage_text` for all 22 `EnergyJobStage` values,
  per-tier progress with a completed percentage, the run-settings block (rendered even with no live
  run, so a user configures before starting), a headline result node picking the highest tier that
  actually published, and a rendered keyboard contract. Cancellation is the `cancel-energy-simulation`
  verb, which now genuinely dispatches (§1.2/§1.6).
- Editor and viewer three-column layouts corrected from `size: Some(0.5)` ×3 to `Some(1.0/3.0)`.

### 2.5 Viewer
`render` is sync and returns `UiAssemblyResult<ComponentTree>`; `initial_snapshot`/`handle` de-asynced;
the viewer simulation window shows the adopted projection's per-tier meters plus the model's own run
period. A viewer declares NO dispatchable action (its two kit windows use the read-only
`window_kind()` variants) and therefore needs no factory and no proof — asserted by
`viewer_declares_no_dispatchable_action`.

### 2.6 Examples
`crate::editor::model::examples() -> Vec<ExampleSource>`, table-driven (one line per example), and the
plugin root now registers
`.editor_with_examples::<EnergyModelEditor>(create_energy_model_editor(), examples())`.
`PluginBuilder::editor` (`🏗️builder/🦀️.rs:421`) hardcodes `examples: Vec::new()`, so the bare form
could never have filled `PluginManifest.examples` — this is the only call that does, and the react
shell's picker reads `activePluginManifest.examples`. Viewers are correctly excluded
(`project_artifact_declarations` collects examples only for `AppRole::Editor`).

### 2.7 TypeScript twins
`✏️editor/🟦️.ts` (retained/document tool rosters, the corrected `EnergySimulationEvent` union with
`configure`, the example-id list), and the three window `🟦️.ts` files (their action tables, the
`EnergyModelMaterialProperty` union, `EnergySimulationRunSettings`).

---

## 3. Tests added

In `A/✏️editor/🦀️.rs`:
- `retained_roster_is_exact_and_exhaustive` — set equality across the roster, `TOOL_JOB_IDS`, the
  factory's `TOOL_IDS`, its `PUBLICATION_CONTRACTS` and the proof rows, plus roster ⊆ migrated.
- `every_declared_action_is_classified_and_resolves_to_a_command` — no `Unclassified`, and every
  roster id bridges through `command_from_action` back to itself.
- `document_verbs_publish_to_the_artifact_lane_and_session_verbs_do_not`.
- `every_declared_verb_dispatches_without_an_interactive_job_fault` — the end-to-end dispatch law,
  through `testkit::new_app_with_registry` + `bind_instance_id` + the real `PluginApp::handle_action`
  route, asserting no `interactive-job.*` code for any of the 18 verbs.
- `renaming_the_model_dispatches_cleanly_through_the_real_action_route` — the positive half.
- `landed_semantic_kinds_produce_granular_mutations`,
  `verbs_awaiting_their_semantic_kind_refuse_loudly_and_name_it`,
  `a_zone_still_referenced_by_a_surface_cannot_be_deleted`,
  `out_of_range_payloads_are_refused_before_they_reach_the_vocabulary`.
- `examples_are_registered_for_the_shell_picker`, `a_command_round_trips_through_its_own_text_and_binary_codec`.

In the windows: kit-action survival + localization for structure and zones; six localized actions,
idle-render-still-shows-settings, both-languages-differ and all-22-stages-translated for simulation;
run-period rendering for the viewer's result window.

---

## 4. Command output

(appended below as runs complete)

---

# 📓️ W-E, second shift (relaunch 20:50) — verification and two real fixes

## 5. What the disk actually held (the predecessor's report was BEHIND its own work)

Verified by direct read, not by trusting §1–§4 above:

- The roster is **19** ids, not 18: `setActiveExample` is fully landed (command variant, `action_id`,
  `args_bridge` row, proof row, publication contract, and an `ActionArgDef::select("exampleId", …)`
  picker fed by `example_options()`).
- All fifteen examples (`🎬️demo` + the fourteen `🏛️bestest-*`) are mounted in
  `📦️packages/🦀️rust/🦀️.rs` (`#[path]` leaves + the `crate::examples` alias block), each leaf
  exporting `ID`/`LABEL_EN`/`CASE`/`model()`.
- The plugin root registers
  `.editor_with_examples::<EnergyModelEditor>(create_energy_model_editor(), crate::editor::model::examples())`
  (`📦️packages/🦀️rust/🦀️.rs:47`), so `PluginManifest.examples` is genuinely filled.
- `🧵️simulation-session/🦀️.rs` carries `render_identity_of`, `session_settings`, `with_projection`,
  `with_adopted_projection`, the `Configure` event and `EnergySimulationConfigProjection::DEFAULT`.
- **Nothing had been compiled.** §4 "Command output" was empty and there was no evidence of a run.

## 6. Two defects found by reading the framework, both fixed

### 6.1 `set-run-period` was proved but declared on NO window — fatal at app construction

`ENERGY_MODEL_RETAINED_TOOL_IDS`, `OpBinary::TOOL_JOB_IDS` and the `bounded_first_step_tool_proofs!`
rows all carried `set-run-period`, but no `WindowKindDefinition` and no builder-level action ever
declared it. That is not a cosmetic gap:

- `AppBuilder::action_interactive_job` (`🔌️plugin/🦀️.rs:5223`) iterates
  `self.actions ∪ window_kinds[].actions` and silently does nothing for an id it cannot find, so the
  roster loop in `create_energy_model_editor` never classified it.
- `AppActionRegistry::from_definition` (`:12094`) builds its `actions` map ONLY from
  `definition.window_kinds[].actions`, so `migrated_tool_ids()` (`:12212`) could never contain it.
- `validate_tool_job_rows` (`:12233`) then computes `expected = TOOL_JOB_IDS ∩ migrated` WITHOUT
  `set-run-period` while `seen` (the proof rows) HAS it ⇒ `seen != expected` ⇒
  `interactive-job.catalog-incomplete` — **the whole editor app fails to construct**, and with it
  every one of the nineteen verbs, not just this one.

Fixed by declaring it where it is actually rendered and edited: `⚡️simulation`'s `definition()` now
carries `run_period_action()` — an `ActionKind::Mutation` (it IS document data; W-D0 moved the run
period onto `Model`) with four `slider` args, stamped `Migrated` like its six siblings. The window's
own test now asserts seven actions and that `set-run-period` is among them.

Corollary worth recording for the other lanes: a builder-level action (`.mutation(...)` /
`.action_with(...)`) is NOT lost — `try_build_definition` (`:5463-5487`) pushes every builder action
that no window explicitly owns onto EVERY window kind. That is why `setActiveExample`, declared on
the builder, is legitimately reachable while `set-run-period`, declared nowhere, was not.

### 6.2 `setActiveExample` went through the mutation seam, which cannot express a whole-model swap

The verb read the example's `Model` and handed it to `model_edit(...)`, whose exhaustive `probe`
comparison refuses anything the landed vocabulary cannot name. Loading ANY example therefore faulted
`mutation.kind-unavailable` — and it always would have, because whole-document replace has no
mutation representative in this artifact's vocabulary on purpose (`📓️derivation-rules.md` rule 6,
and `🧬️schema/🧬️mutations/🦀️.rs:10` routes file-open / import / load-example through
`store::ArtifactStore::reset`).

The sanctioned mechanism is the one `📐️cad` (`✏️editor/🦀️.rs:566`) and `🔱️trinity`
(`✏️editor/🦀️.rs:68`) already use: `kernel::Effect::LoadDocument { pack, spr }`, which the host
applies through `load_document_pack` → `ArtifactStore::reset` (`🔌️plugin/🦀️.rs:25298`), outside
undo history. `reduce` now returns that effect and no mutation at all, via a new
`load_document_effect(&Model)` built from `ArtifactPack::encode_pack` + `create_document_envelope` +
`resolve_ready(print_document_spr(…))`.

Consequences carried through:
- `setActiveExample`'s publication lane moved `Artifact` → `HostOnly`, and it left
  `ENERGY_MODEL_DOCUMENT_TOOL_IDS` (now twelve verbs). The framework's lane check
  (`🔌️plugin/🦀️.rs:22944`) only refuses a lane you PUBLISH to without declaring; an effect-only
  emit is `HostOnly` by definition.
- New test `loading_an_example_swaps_the_document_through_an_effect_and_never_a_mutation`: every one
  of the fifteen rows emits exactly one `LoadDocument`, no mutation, and the emitted pack decodes
  back to that leaf's own `model()`; an unknown id is `mutation.target-missing`.
- TS twins updated (`✏️editor/🟦️.ts` document-id slice 13 → 12, `⚡️simulation/🟦️.ts` gains the
  `set-run-period` row).

## 7. Build

Cold `cargo check -p semio-s-plugin-energy --lib --tests` in the shared `target-energy-e2e`.

## 8. The mutation groups landed — the seam is now a real round trip, not a refusal

At 22:00 `🧬️schema/🧬️mutations/` held **278** kind directories (it held 17 at the start of this
shift). Every kind the editor was blocked on now exists, so `model_edit` no longer refuses:

| Editor verb | Semantic steps it now emits |
|---|---|
| `create-zone` / `delete-zone` | `create-zone` / `delete-zone` |
| `create-surface` | `create-surface` (boundary tag + optional interzone partner, the parallel-field shape `OutsideBoundaryKind` forces) |
| `delete-surface` | `delete-fenestration`* → `disconnect-surfaces`* → `delete-surface`, in that order |
| `assign-surface-construction` | `change-surface-construction` |
| `set-material-property` | one of the seven `change-material-*` |
| `set-thermostat-setpoints` | up to four `change-thermostat-*` |

`model_edit` grew four collection differs — `diff_zones`, `diff_surfaces`, `diff_materials`,
`diff_thermostats` — and the exhaustive `probe` grew the five collections they own, so anything
they do NOT name is still refused loudly. Two identity guards stay deliberately closed:
`create-material`/`delete-material` and `create-thermostat`/`delete-thermostat` — the editor has no
verb for either, so an identity change there is a bug, not a gap, and must not be masked by the probe.

`diff_surfaces` covers ALL nine surface fields (name, zone, class, vertices, construction, boundary,
sun/wind exposure, multiplier), not just the one `assign-surface-construction` produces, so the probe
over `surfaces` is honest rather than a rubber stamp.

The old dual-mode test `verbs_awaiting_their_semantic_kind_refuse_loudly_and_name_it` is REPLACED by
`every_document_verb_round_trips_through_the_granular_vocabulary`, which applies the emitted steps and
asserts the resulting `Model`. A `mutation.kind-unavailable` there is now a regression, not a pending
group. A shared `populated_model()` fixture (one zone, two constructions, one material, two constant
schedules, one surface, one thermostat) gives every authored verb a real target.
