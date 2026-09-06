# Explore: remodel plugin editor runtime dispatch + rendering audit

Read-only Sonnet explorer, 2026-09-06. Not compiled (cargo forbidden this session — host at
63/64GB swap). Every claim below is grep/read-verified against current source in this working
tree. Oracles read first: `BLOCK-PLUGIN-END-TO-END/📓️explore-block5d-editor.md` (healthy
pattern), `📓️w1-block2d-factory.md` (the fix shape), `DRAW-PLUGIN-END-TO-END/📓️explore-editor-dispatch.md`
(sibling audit, same day).

**Headline: remodel is more broken than either block2d or draw.** Both of those apps had real,
working command handlers but were dispatch-dead due to missing classification/factory wiring.
Remodel's `✏️editor/🦀️.rs` (1494 lines) and `👁️viewer/🦀️.rs` (114 lines) are written against a
**stale, fully-async generation of the `ArtifactEditor`/`ArtifactViewer` traits that no longer
exists in the framework** — virtually every trait method override in both files has the wrong
async-ness and, in one case, the wrong argument/return type. This reads as E0053-shaped
("method has an incompatible type for trait") and would very likely not compile at all. Layered
under that (would-be errors if §0 were fixed): only 2 of 39 declared commands are classified
`Migrated` (the framework panics at manifest-build time on ANY `Unclassified` action, so
`create_remodeling_app()` itself would abort, taking down editor AND viewer), and the one
retained-tool-factory's `controller:` literal does not match the runtime controller id derived
from the artifact's own (self-flagged-as-wrong) `REMODELING_DIALECT` constant.

---

## 0. Compile-breaking defect — editor/viewer overrides use a stale async trait shape

`ArtifactEditor` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26623-26970`) and
`ArtifactViewer` (`:26972-27130ish`) are the AUTHORING traits `RemodelingPlayApp`/`RemodelingViewer`
implement. Reading the trait declarations directly: `handle`, `command_id`, `command_from_action`,
`initial_snapshot`, `initial_config`, `render`, `window_measures`, `io`, `export_media`,
`import_media`, `app_schema` are all **plain, non-async `fn`** (only `command_from_intent` and
`media_ports` are `async fn`). Confirmed against a real, presumably-compiling peer —
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:6839-7047` —
which implements every one of these as plain `fn`, matching the trait exactly.

Remodel's editor (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`)
declares **every single one of these as `async fn`**: `app_schema` (:636), `initial_snapshot`
(:640), `io` (:644), `export_media` (:651), `import_media` (:678), `command_id` (:717),
`command_from_action` (:728), `handle` (:732), `render` (:743), `window_measures` (:764).

`command_from_action` has a second, independent mismatch: the trait requires
`fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault>`
(`DslValue` = `dsl::DslValue`, an enum — confirmed at `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:4,457` and
matched verbatim by every peer editor: puzzle 3d/5d/2d, procedural generation2d/3d all use
`Option<&dsl::DslValue>`). Remodel's override (`✏️editor/🦀️.rs:728`) is
`async fn command_from_action(action: &str, args: Option<&Value>) -> Result<RemodelingCommand, Fault>`
where `Value` = `serde_json::Value` (`use serde_json::Value;` at `:31`) — a completely different
concrete type, not just a naming difference. No `DslValue` import exists anywhere in this file
(confirmed by grep).

The viewer (`👁️viewer/🦀️.rs`) has the same drift: `ArtifactViewer::render` is
`fn render(...) -> UiAssemblyResult<ComponentTree>` in the trait (`:27100`), but remodel's viewer
declares `async fn render(...) -> UiNode` (`:64`) — wrong async-ness AND wrong return type.
`initial_snapshot` (`:52`) and `handle` (`:60`) are likewise `async fn` against a sync trait.

**Consequence**: if this trace is correct, `impl ArtifactEditor for RemodelingPlayApp` and
`impl ArtifactViewer for RemodelingViewer` do not type-check against the current trait — the
crate does not compile, full stop. Everything below is analysis of what the code *would* do
once this is fixed (mechanical: drop every stray `async`, fix `command_from_action`'s arg type
to `Option<&dsl::DslValue>`, fix the viewer's `render` return type), following the same
"argued from framework admission code, not observed" caveat draw's report used for its own §0.
This is NOT independently confirmed by a real compiler run (forbidden this session) — flagged as
the single highest-priority thing for whoever picks this ticket up to verify with `cargo check`.

Module-path plumbing itself is clean (unlike draw's `draw`/`drawing` rename drift): the plugin
root `✏️s/🔌️plugins/📸️remodel/🦀️.rs` references `crate::editor::remodeling::RemodelingPlayApp` /
`crate::viewer::remodeling::RemodelingViewer`, and the mount file
`📦️packages/🦀️rust/🦀️.rs:883-1146` wires `pub mod editor::remodeling` / `pub mod viewer::remodeling`
to exactly those `#[path]` targets — every referenced path exists on disk (verified by `find`).

---

## 1. Every command and its interactive-job classification

`RemodelingCommand` (`✏️editor/🦀️.rs:188-245`, one `app_commands!` block) has exactly 39 rows, one
per `🎮️commands/*` handler (39 command directories confirmed on disk, no extras/gaps). All 39 are
declared as manifest actions via `.mutation(...)`/`.action_with(...)` in `create_remodeling_app()`
(`:774-1019`) — `.mutation()` is sugar for `.action_with(ActionDefinition::bounded_catalog(id, ..,
ActionKind::Mutation))` (confirmed in framework `🔌️plugin/🦀️.rs:5187-5189`), so every row becomes a
real `ActionDefinition` subject to classification.

Only **two** calls to `.action_interactive_job(..., InteractiveJobClassification::Migrated)`
exist in the whole file (`:1006-1007`): `importFrames`, `importVideo`. The other **37** commands
are never touched and stay at the default `InteractiveJobClassification::Unclassified`. The
manifest's own doc comment right above the two calls (`:1003-1005`) is self-aware of this: "Only
fixture-classified O(1) config/view and host-request reducers are admitted here. Every document
mutation, reconstruction step, payload decode, calibration traversal, QC encoding, and
cancellation route remains fail-closed until resumable work is mounted."

**This is worse than "UI-dispatch-dead"**: `create_remodeling_app()` ends with `.build_definition()`
(`:1019`), which is `EditorBuilder::build_definition` → `AppBuilder::build_definition`
(`🔌️plugin/🦀️.rs:5813-5814`, `self.try_build_definition().unwrap_or_else(|error| panic!("{error}"))`).
`AppBuilder::try_build_definition` (`:5303-5804`) calls
`semio_framework::validate_interactive_job_classification` (`:5722-5729`) over every declared
action and command and returns `Err(PluginAssemblyError::new("app-definition.interactive-job-classification", ...))`
if **any** is `Unclassified` (`🛂️manifest/🦀️.rs:952-966`). With 37 unclassified actions, this `Err`
is guaranteed, so `build_definition()` **panics** the instant `create_remodeling_app()` runs.

Since the plugin root evaluates `create_remodeling_app()` inline as an argument to `.editor(...)`
(`🦀️.rs:30`), this panic happens during `plugin()` construction — before `.viewer(...)` even runs.
**Both the editor and viewer surfaces of remodel fail to register**, not just the 37 unclassified
actions. Corroboration inside the file itself: `remodeling_app_manifest_for_testkit()`
(`✏️editor/🦀️.rs:1042-1044`) calls `create_remodeling_app()`, and at least two tests use it —
`app_with_registry()` (`:1052-1054`, used at `:1397`) and
`testkit::assert_declared_actions_bridge_to_commands::<EditorApp<RemodelingPlayApp>>(remodeling_app_manifest_for_testkit)`
(`:1330`) — both would panic if run (not run this session; cargo forbidden).

---

## 2. Tool-job factory — present, but its controller literal doesn't match the runtime id

Unlike block2d (no factory apparatus) and closer to draw's shape (real factory, real
`ArtifactOwnedToolJobFactory` impl), remodel has one real bounded factory,
`RemodelingCommandJobFactory` (`✏️editor/🦀️.rs:505-561`), covering exactly the two `Migrated` tools
(`REMODELING_BOUNDED_TOOL_IDS = ["importFrames", "importVideo"]`, `:477`), wired via
`bounded_first_step_tool_proofs!` with `factory_type: RemodelingCommandJobFactory` (`:581-592`) and
a non-empty `PUBLICATION_CONTRACTS` (`:557-560`, both `HostOnly` lane — plausible: both commands are
shell/dialog-triggered imports with no `Emit::mutations` at the manifest layer itself).

**But the `controller:` literal is wrong.** `bounded_first_step_tool_proofs!`'s `controller:
"s.remodeling.remodeling@1/*#editor"` (`:584`) is compared at runtime against
`row.controller_id == runtime_controller_id` inside `validate_tool_job_rows`
(`🔌️plugin/🦀️.rs:12265`), where `runtime_controller_id` is `surface_app_id(&E::DIALECT.into(),
E::ROLE)` = `format!("{}@{}/{}#{}", dialect.artifact_kind, standard, subset, role)`
(`🛂️manifest/🦀️.rs:3403`, `ArtifactDialect::to_coordinate` at `🚪️io/🧬️schema/🦀️.rs:82-85`).

`RemodelingPlayApp::DIALECT = crate::artifacts::remodeling::REMODELING_DIALECT` (`✏️editor/🦀️.rs:578`),
and `REMODELING_DIALECT.artifact_kind = "s.remodel.remodeling.remodeling"`
(`🗿️artifacts/📸️remodeling/🦀️.rs:180`) — so the real runtime controller id is
`"s.remodel.remodeling.remodeling@1/*#editor"`, NOT `"s.remodeling.remodeling@1/*#editor"` as
hardcoded in the proof. These do not match. Per `validate_tool_job_rows`'s `authoritative` check
(`:12265`), a controller mismatch makes the proof row non-authoritative, and the function returns
`Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.catalog-authority"), ...))`.
So even if §0 and §1 were both fixed, `importFrames`/`importVideo` would *still* fault at
factory-registration time with `interactive-job.catalog-authority`.

Worth noting: the artifact file's own doc comment on `REMODELING_DIALECT`
(`🗿️artifacts/📸️remodeling/🦀️.rs:172-179`) states the constant *should* be `"s.remodeling.remodeling"`
("matches this artifact's own `definition()` capability row... the schema-artifact descriptor")
— i.e. the doc comment agrees with the editor's `controller:` literal, and it is the
`REMODELING_DIALECT` **constant's own value** (`"s.remodel.remodeling.remodeling"`, with an extra
`remodel.` segment) that contradicts its own documentation. Whichever side is "correct" by
intent, the two must be made to literally match — right now they provably don't (verified by
direct string comparison of both literals in the source, not a runtime trace).

---

## 3. Boot snapshot

`RemodelingPlayApp::initial_snapshot()` / `RemodelingViewer::initial_snapshot()` both return
`crate::artifacts::remodeling::default_remodeling_scene()`
(`🗿️artifacts/📸️remodeling/🦀️.rs:1643-1656`): empty `streams`/`assets`/`gcps`, default
`calibration`/`params`/`job`, but `results.mesh` is seeded with `placeholder_remodeling_mesh_handle()`
(`source: MeshSource::Placeholder`) specifically so, per its own doc comment, "the 3D
editor/preview always has something to render before any media has been imported/reconstructed."
Reasonable, non-crashing boot shape — similar in spirit to draw's non-empty default, though here
almost everything except the mesh is genuinely empty.

---

## 4. Examples — orphaned, unreachable from any command

Two example modules exist on disk: `📚️examples/🎬️demo` (id `"demo"`, DSL text via
`include_str!("🖼️assets/🗣️.dsl.semio")`, `📚️examples/🎬️demo/🦀️.rs:5-13`) and
`✏️editor/📚️examples/🎬️demo-session` (a `.cmd.semio` command-replay fixture). Both are mounted as
plain `pub mod` declarations in `📦️packages/🦀️rust/🦀️.rs` (`art_remodeling_demo`,
`app_remodeling_demo_session`) — `grep -rln` for either module name across the whole plugin
directory returns **only** that one mount file. Neither is referenced by any `examples()`
aggregator, any `ExampleSource` registry, or any command handler.

Unlike draw/block2d, remodel's 39-command `RemodelingCommand` enum has **no `setActiveExample`
variant at all** (confirmed: absent from the full command list in §1). There is therefore no
UI-reachable path to load either example — this is dead content, not merely a broken id mapping.

---

## 5. Windows / panels / modes

Three modes declared in the manifest (`create_remodeling_app`, `:783-786`): `capture`, `model`
(default), `analyze`. Window kinds registered: `model::windows::model` (main 3D view, default
layout `model::layout()`), `capture::windows::frames` (named layout), `analyze::windows::report`
(named layout) — `:787-813`. Seven panel tabs registered (`:814-820`): document/pipeline, media,
results, parameters, calibration, tracks, quality. `render()` (`✏️editor/🦀️.rs:743-760`) matches
all ten body keys plus a named fallback (`"Unknown body: {body_key}"`) — no panic path, matches
the framework's non-panic-safe convention seen in draw/block5d. Not independently verified this
pass whether each window/panel renders non-empty content from `default_remodeling_scene()`
specifically (time-boxed) — flagged as a follow-up read, same caveat draw's report gave its
catalogue/properties panels.

---

## 6. Long-running compute — `run-reconstruction`/`run-stage`/`retry-stage`

The plugin root's own doc comment (`✏️s/🔌️plugins/📸️remodel/🦀️.rs:34-38`) frames this as "this
packet's genuine 'SfM' long-running-compute finding" and says `Effect::SpawnJob` conversion is
"blocked upstream, not by anything in this crate" — implying a single synchronous blocking loop.
Reading `🎮️commands/🏗️run-reconstruction/🦀️.rs` (1524 lines) directly, the actual implementation is
**not** a monolithic blocking call: it is a tick-based, self-rescheduling continuation —
`handle_advance` (`:1029`) processes one bounded chunk of engine work per invocation, then emits
`Effect::DispatchAction { action: ADVANCE_RECONSTRUCTION_ACTION_ID, .. }` (`:546`) to reschedule
the next tick; tests bound this to `MAX_RECONSTRUCTION_TICKS` iterations (`:1085`). Every tick
checks `scene.job.cancel_requested` (`:904`) and calls `cancel_session`/`cancel_current_reconstruction`
(`:377,416-434`) when set; `progress_0_1` and `stage_cursor` are updated and surfaced to the
document every tick (`preview_job`, `:579-585`; `terminal_progress`, `:696`). This is a real
progress+cancellation mechanism at the application layer — CLAUDE.md's bar is met structurally,
even though the deeper `Effect::SpawnJob`-backed resumable-job architecture isn't available yet
(consistent with the plugin root's framing, just not "blocking" in the naive sense).

**None of this matters at runtime today**: `runReconstruction`, `advanceReconstruction`,
`retryStage`, `runStage`, `cancelReconstruction` are all among the 37 `Unclassified` commands from
§1 — under the current manifest they either prevent the app from constructing at all (§1) or, if
that gate were bypassed, would be rejected by `validate_interactive_job_classification`'s sibling
runtime check (UI dispatch rejects anything that isn't exactly `Migrated`).

---

## 7. Viewer — `RemodelingViewer`

Single command `RemodelingViewCommand::Noop` (`👁️viewer/🦀️.rs:19-22`), `handle` always returns
`Ok(ViewEmit::default())` (`:60-62`, explicitly kept as a real dispatch rather than
`unreachable!()`, same pattern as draw's viewer). Single mode `view`, single window `model`
(`create_remodeling_viewer`, `:74-83`). Boots on the same `default_remodeling_scene()` as the
editor (§3). No purity breach found: no import from `crate::editor::remodeling::*`. Same §0
async/return-type drift as the editor (`initial_snapshot`, `handle` wrongly `async fn`; `render`
wrongly `async fn` AND wrongly returns `UiNode` instead of `UiAssemblyResult<ComponentTree>`).

---

## Priority gap list

1. **(§0, highest)** Fix every stray `async fn` in `✏️editor/🦀️.rs` and `👁️viewer/🦀️.rs` back to
   the trait's plain `fn` shape; fix `command_from_action`'s arg type to `Option<&dsl::DslValue>`;
   fix the viewer's `render` return type to `UiAssemblyResult<ComponentTree>`. Verify with a real
   `cargo check` — this session could not run one, so treat §0 as high-confidence but unconfirmed
   by compilation.
2. **(§1)** Classify or explicitly triage all 37 currently-`Unclassified` commands
   (`.action_interactive_job(id, InteractiveJobClassification::Migrated)` + a matching retained
   factory row, following the block2d/block5d precedent), or the app cannot even construct —
   `create_remodeling_app()` panics inside `.build_definition()`.
3. **(§2)** Reconcile `REMODELING_DIALECT.artifact_kind` (currently
   `"s.remodel.remodeling.remodeling"`) against the `bounded_first_step_tool_proofs!` `controller:`
   literal (currently `"s.remodeling.remodeling@1/*#editor"`) — pick one spelling and make both
   sides match, using the dialect's own doc comment (which wants `"s.remodeling.remodeling"`) as
   the likely tie-breaker.
4. **(§4)** Either wire `📚️examples/🎬️demo` / `✏️editor/📚️examples/🎬️demo-session` into a real
   `setActiveExample` command + example registry, or remove the orphaned modules — they are dead
   weight either way.
5. **(§5)** Follow-up: confirm each window/panel renders non-empty content off
   `default_remodeling_scene()` (not done exhaustively this pass).

All line numbers above are current-source references, not compiled offsets. Unverified items are
called out inline; nothing here was confirmed by running `cargo check`/`cargo test`
(forbidden this session per host swap pressure).
