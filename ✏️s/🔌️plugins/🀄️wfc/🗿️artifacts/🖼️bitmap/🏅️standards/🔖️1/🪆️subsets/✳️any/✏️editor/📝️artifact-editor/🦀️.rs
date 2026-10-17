//! 📝️ Artifact editor of the WFC bitmap editor: the editor type, its `ArtifactEditor` implementation and its helpers. See [`super`].

use super::*;

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct BitmapEditor;

impl ArtifactEditor for BitmapEditor {
    /// 📢️ The localized notices of this editor's user-reachable refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        use semio_framework_ui_locale::LocalizedLabel;
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 13]> = std::sync::LazyLock::new(|| {
            [
            ("wfc.bitmap.stroke.phase-invalid", LocalizedLabel::native("This stroke step is not known.", "Dieser Strichschritt ist unbekannt.")),
            ("wfc.bitmap.stroke.points-invalid", LocalizedLabel::native("The stroke's cells are not valid.", "Die Zellen des Strichs sind ungültig.")),
            ("wfc.bitmap.stroke.points-required", LocalizedLabel::native("A stroke needs at least one cell.", "Ein Strich braucht mindestens eine Zelle.")),
            ("wfc.bitmap.stroke.window-required", LocalizedLabel::native("A streamed stroke needs an open input window.", "Ein gestreamter Strich braucht ein geöffnetes Eingabefenster.")),
            ("wfc.bitmap.palette.unknown-color", LocalizedLabel::native("This colour is not in the palette.", "Diese Farbe ist nicht in der Palette.")),
            ("wfc.bitmap.window.view-required", LocalizedLabel::native("This action needs an open window.", "Diese Aktion braucht ein geöffnetes Fenster.")),
            ("wfc.bitmap.window.stale", LocalizedLabel::native("The window of this action is no longer open.", "Das Fenster dieser Aktion ist nicht mehr geöffnet.")),
            ("wfc.bitmap.input.window-required", LocalizedLabel::native("Focus the input window for this action.", "Für diese Aktion das Eingabefenster fokussieren.")),
            ("wfc.bitmap.input.window-kind-required", LocalizedLabel::native("This action belongs to the input window.", "Diese Aktion gehört zum Eingabefenster.")),
            ("wfc.bitmap.output.window-required", LocalizedLabel::native("Focus the output window for this action.", "Für diese Aktion das Ausgabefenster fokussieren.")),
            ("wfc.bitmap.output.window-kind-required", LocalizedLabel::native("This action belongs to the output window.", "Diese Aktion gehört zum Ausgabefenster.")),
            ("wfc.bitmap.solution.contradiction", LocalizedLabel::native("A contradicting solve cannot be pinned.", "Eine widersprüchliche Lösung lässt sich nicht fixieren.")),
            ("wfc.bitmap.solution.pixels-invalid", LocalizedLabel::native("The solved pixels cannot be read.", "Die gelösten Pixel lassen sich nicht lesen.")),
            ]
        });
        &*NOTICES
    }

    type Snapshot = BitmapSnapshot;
    type Mutation = BitmapMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = BitmapTransient;
    type TransientMutation = BitmapTransientMutation;
    type Command = BitmapEditorCommand;

    const DIALECT: Dialect = WFC_BITMAP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_BITMAP_DOCUMENT_SCHEMA;

    /// 👥️🫧️ The close ladder retires the presence and transient ROOTS through installed factories and
    /// refuses to teardown without them; this editor carries no presence at all, so the `No*` factories
    /// are its exact terminal.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    /// 🗃️ The bounded retirement catalog every store lane is released THROUGH: a store built without
    /// owners answers `artifact store has no owner-supplied bounded disposer` the moment the close
    /// ladder reaches it, so the owners and the disposers below are one declaration in two halves.



    /// ♻️ The instance close ladder walks one owned store lane per stage and faults the whole close
    /// with `interactive-job.close-owned-disposer-missing` the moment a lane answers `None`, so an
    /// editor must declare how EVERY store it types is released, not only the ones it edits.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn initial_snapshot() -> BitmapSnapshot {
        crate::examples::rooms_16::snapshot()
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<input::config::BitmapInputWindowConfigOwner>()?;
        registry.register::<output::config::BitmapOutputWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        BitmapCommandJobFactory::register(registry)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<BitmapEditor>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.wfc.bitmap@1/*#editor",
        artifact_schema: "s.wfc.bitmap",
        factory: "BitmapCommandJobFactory",
        factory_type: BitmapCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "change-seed",
            "resize-input",
            "set-input-pixels",
            "add-palette-color",
            "change-palette-color",
            "remove-palette-color",
            "resize-output",
            "change-model",
            "pin-pixel",
            "unpin-pixel",
            "set-active-color",
            "paint-stroke",
            "solve",
            "commit-fill-solve",
            "setActiveExample",
            "pin-solution",
            "canvasPointerDown",
            "canvasPointerMove",
            "canvasPointerUp",
            "canvasDoubleClick",
            "setCamera"
        ]
    }

    /// 📬️ Publication authority for the document lane. Without it every `Artifact`-lane verb is
    /// refused closed with `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(BitmapOneItemPreparationFactory { maximum_bytes: BITMAP_ARTIFACT_MUTATION_MAXIMUM_BYTES }))
    }

    /// 🫧️ Publication + retirement authority for the solve cache the output window renders.
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !BITMAP_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let tool_id = bitmap_command_id(&request.command);
        if tool_id != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "WFC bitmap command does not match its exact registered tool"));
        }
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            bitmap_command_id,
            BITMAP_RETAINED_RAW_BYTES,
            BITMAP_RETAINED_WORK_ITEMS,
            Box::new(BitmapCommandWork { tool_id, consumed: false }),
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        bitmap_command_id(command)
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    /// ✏️ Dispatches straight onto the real schema-tree mutation builders — one `BitmapMutation` per
    /// document command, no whole-document rewrite anywhere.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        Self::dispatch(command, doc, cfg, view_state)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_bodies(body_key, doc.snapshot, cfg, &BitmapTransient::default(), None)
    }

    /// 🧮️ The real render path: the same bodies, but reading the solve cache the `Solve` command published on the
    /// transient lane, and the input window's brush stroke in flight from its own window transient.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        Self::render_bodies(body_key, doc.snapshot, cfg, transient.snapshot, doc.tool_run())

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        if request.tool_id != fill_tool::TOOL_ID || request.purpose != semio_framework_plugin::ToolRunJobPurpose::Run {
            return Ok(None);
        }
        Ok(Some(Box::new(fill_tool::BitmapFillRunJob::from_request(request)?)))
    }

    /// 🫧️ Retained through the tool factory instead: `ArtifactApp::ephemeral` is never called by the
    /// live dispatch path, so the solve is published from [`BitmapCommandWork::step`].
    fn ephemeral(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _presence: &PresenceView<'_, Self::Presence>,
        _transient: &TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        (Vec::new(), Vec::new())
    }
}

impl BitmapEditor {
    /// 🎯️ The pure dispatch every route shares — the ordinary `handle` and the retained work step
    /// both enter here, so a verb can never behave differently depending on which one ran it.
    /// 🖥️ `ui_scope` is the whole point of this arm: `Solve` publishes ONLY on the transient
    /// lane, and a transient publication carries no document revision for the shell to notice,
    /// so with the default scope the collapse landed in the store and the output window kept
    /// rendering the buffer it already had (measured: a black square after a successful solve).
    pub fn dispatch(command: &BitmapEditorCommand, doc: &ArtifactView<'_, BitmapSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: Option<&semio_framework_plugin::ViewModel>) -> Result<Emit<BitmapMutation>, Fault> {
        let Some(mutation) = Self::command_mutation(command) else {
            return match command {
                BitmapEditorCommand::SetActiveColor { index } => Self::set_active_color(doc, cfg, view_state, *index),
                BitmapEditorCommand::PaintStroke { .. } => Self::paint_stroke(command, doc, cfg, &semio_framework_plugin::app::GestureSlot::detached(), false),
                BitmapEditorCommand::SetActiveExample { example_id } => set_active_example::handle(&set_active_example::SetActiveExample { example_id: example_id.clone() }, doc),
                BitmapEditorCommand::PinSolution { pixels, contradiction } => pin_solution::handle(&pin_solution::PinSolution { pixels: pixels.clone(), contradiction: *contradiction }, doc),
                BitmapEditorCommand::Solve => Ok(Emit { effects: vec![fill_tool::start_fill_effect()], ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }),
                BitmapEditorCommand::CommitFillSolve { .. } => Ok(Emit { ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }),
                BitmapEditorCommand::CanvasPointerDown { .. } | BitmapEditorCommand::CanvasPointerMove { .. } | BitmapEditorCommand::CanvasPointerUp { .. } | BitmapEditorCommand::CanvasDoubleClick | BitmapEditorCommand::SyncCamera => Ok(Emit::default()),
                _ => Err(Fault::from("wfc-bitmap-command-unmapped")),
            };
        };
        Ok(Emit::mutations(vec![mutation]))
    }

    /// 🫧️ Runs the artifact's own headless inference adapter and packages the answer as the one
    /// transient write the output window reads.
    ///
    /// An UNSATISFIABLE problem answers a real transient with no pixels — that is the document's own
    /// answer and the pane says so. An inference that could not RUN is a FAULT, not a contradiction:
    /// folding the two together is what made a silently refused solve look like a black square with no
    /// diagnostic anywhere in the shell.
    pub fn solve_transient(snapshot: &BitmapSnapshot) -> Result<BitmapTransientMutation, Fault> {
        let (pixels, contradiction) = match crate::inferences::solve_with_job(snapshot) {
            Ok(commit) if !commit.contradiction && !commit.pixels.is_empty() => (Some(crate::standards::v1::subsets::any::io::text::snapshot::encode_base64(&commit.pixels)), false),
            Ok(_) => (None, true),
            Err(error) => return Err(Fault::from(format!("wfc-bitmap-solve-failed:{error}"))),
        };
        Ok(BitmapTransientMutation::SetSolve(SetSolve { output_pixels: pixels, contradiction, output_width: snapshot.output.width, output_height: snapshot.output.height }))
    }

    /// 🗺️ The command→mutation map, lifted out of `dispatch` so it is a pure, testable function: a
    /// command that answers `None` is one of the non-document verbs this file's own doc comment
    /// accounts for, and nothing else.
    pub fn command_mutation(command: &BitmapEditorCommand) -> Option<BitmapMutation> {
        Some(match command {
            BitmapEditorCommand::ChangeSeed { seed } => change_seed(*seed),
            BitmapEditorCommand::ResizeInput { width, height } => resize_input(*width, *height),
            BitmapEditorCommand::SetInputPixels { x, y, width, height, pixels } => set_input_pixels(*x, *y, *width, *height, pixels.clone()),
            BitmapEditorCommand::AddPaletteColor { index, r, g, b, a } => add_palette_color(*index, BitmapColor { r: *r, g: *g, b: *b, a: *a }),
            BitmapEditorCommand::ChangePaletteColor { index, r, g, b, a } => change_palette_color(*index, BitmapColor { r: *r, g: *g, b: *b, a: *a }),
            BitmapEditorCommand::RemovePaletteColor { index } => remove_palette_color(*index),
            BitmapEditorCommand::ResizeOutput { width, height, periodic } => resize_output(*width, *height, *periodic),
            BitmapEditorCommand::ChangeModel { pattern_size, symmetry, periodic_input, ground } => change_model(*pattern_size, *symmetry, *periodic_input, *ground),
            BitmapEditorCommand::PinPixel { x, y, color } => pin_pixel(*x, *y, *color),
            BitmapEditorCommand::UnpinPixel { x, y } => unpin_pixel(*x, *y),
            BitmapEditorCommand::SetActiveColor { .. }
            | BitmapEditorCommand::PaintStroke { .. }
            | BitmapEditorCommand::Solve
            | BitmapEditorCommand::CommitFillSolve { .. }
            | BitmapEditorCommand::SetActiveExample { .. }
            | BitmapEditorCommand::PinSolution { .. }
            | BitmapEditorCommand::CanvasPointerDown { .. }
            | BitmapEditorCommand::CanvasPointerMove { .. }
            | BitmapEditorCommand::CanvasPointerUp { .. }
            | BitmapEditorCommand::CanvasDoubleClick
            | BitmapEditorCommand::SyncCamera => return None,
        })
    }

    fn render_bodies(body_key: &str, snapshot: &BitmapSnapshot, cfg: &ConfigView<'_, NoConfig>, transient: &BitmapTransient, tool_run: Option<&semio_framework_plugin::ToolRunView>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            input::BODY_KEY => input::render(snapshot, &input::config::current(cfg)).map(semio_framework_plugin::built_to_component_tree),
            output::BODY_KEY => output::render(snapshot, transient, &output::config::current(cfg), tool_run).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🖌️ One `paint-stroke` dispatch through the dispatching window's gesture `slot`: the stroke's colour is the
    /// payload's, else the window's armed colour, refused when the palette has no such entry; the committed stroke is ONE
    /// edit stamped with its `TransactionRef` (plain without an admission seed — a render or test view), a stream tick or
    /// a cancel publishes nothing on the document. `interrupt` first drops a stroke a lost release left open
    /// (`captureLost`, zero trace). A stroke left open outside a window is refused: nothing would hold it.
    pub fn paint_stroke(command: &BitmapEditorCommand, doc: &ArtifactView<'_, BitmapSnapshot>, cfg: &ConfigView<'_, NoConfig>, slot: &semio_framework_plugin::app::GestureSlot<BitmapMutation>, interrupt: bool) -> Result<Emit<BitmapMutation>, Fault> {
        let BitmapEditorCommand::PaintStroke { xs, ys, color, phase, reason } = command else { return Err(Fault::from("wfc-bitmap-command-unmapped")) };
        let phase = GesturePhase::parse(phase.as_deref(), reason.as_deref()).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.stroke.phase-invalid"), "wfc.bitmap.stroke.phase-invalid"))?;
        if xs.len() != ys.len() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.stroke.points-invalid"), "wfc.bitmap.stroke.points-invalid"));
        }
        let color = color.unwrap_or_else(|| input::config::current(cfg).active_color);
        let open = !interrupt && slot.open().is_some();
        let opens = matches!(phase, GesturePhase::Once) || (!open && matches!(phase, GesturePhase::Stream | GesturePhase::Commit));
        if opens && color as usize >= doc.snapshot.input.palette.len() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.palette.unknown-color"), "wfc.bitmap.palette.unknown-color"));
        }
        if matches!(phase, GesturePhase::Once) && xs.is_empty() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.stroke.points-required"), "wfc.bitmap.stroke.points-required"));
        }
        let points = xs.iter().zip(ys).map(|(x, y)| BitmapStrokePoint { x: *x, y: *y }).collect();
        let seed = doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default();
        if interrupt {
            slot.drive::<BitmapBrush>(None, BITMAP_PAINT_STROKE_VERB, GesturePhase::Abort(ToolAbortReason::CaptureLost), None, seed)?;
        }
        let committed = slot.drive::<BitmapBrush>(None, BITMAP_PAINT_STROKE_VERB, phase, Some(BrushToolRequest::on(doc.snapshot, points, color)), seed)?;
        if slot.window().is_empty() && slot.open().is_some() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.stroke.window-required"), "wfc.bitmap.stroke.window-required"));
        }
        Ok(semio_framework_plugin::app::gesture_emit(committed, seed))
    }

    /// 🖱️ The `paint-stroke` a canvas pointer event on the input surface means for a window whose gesture slot holds an
    /// `open` stroke or not, in the window's armed colour, and whether a stroke a lost release left open is dropped first
    /// because a fresh press opens a new one. `None` when the event means nothing to the brush: zero trace, no write.
    pub fn pointer_stroke(pointer: &BitmapBrushPointer, snapshot: &BitmapSnapshot, open: bool) -> Option<(BitmapEditorCommand, bool)> {
        let stroke = bitmap_brush_pointer_stroke(pointer, snapshot.input.width, snapshot.input.height, open)?;
        let (xs, ys) = stroke.points.iter().map(|point| (point.x, point.y)).unzip();
        let (phase, reason) = match stroke.phase {
            GesturePhase::Once => (None, None),
            GesturePhase::Stream => (Some("stream"), None),
            GesturePhase::Commit => (Some("commit"), None),
            GesturePhase::Abort(reason) => (Some("abort"), Some(reason.as_str())),
        };
        Some((BitmapEditorCommand::PaintStroke { xs, ys, color: None, phase: phase.map(str::to_string), reason: reason.map(str::to_string) }, stroke.interrupt))
    }

    /// 🎨️ The active brush colour is per-window-instance state: refused outright when the index is
    /// not a real palette entry, so a pane can never arm a colour the document does not have.
    fn set_active_color(doc: &ArtifactView<'_, BitmapSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: Option<&semio_framework_plugin::ViewModel>, index: u32) -> Result<Emit<BitmapMutation>, Fault> {
        if index as usize >= doc.snapshot.input.palette.len() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.palette.unknown-color"), "wfc.bitmap.palette.unknown-color"));
        }
        let view = view_state.ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.window.view-required"), "wfc.bitmap.window.view-required"))?;
        let mutation: WindowConfigMutation = input::config::addressed(view, input::config::BitmapInputWindowConfigMutation::SetActiveColor { color: index })?;
        Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
    }
}
//#endregion 🔖️Editor
