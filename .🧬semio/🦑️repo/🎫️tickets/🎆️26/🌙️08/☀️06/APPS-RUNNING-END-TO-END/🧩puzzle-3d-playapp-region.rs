//#region 🔖️PlayApp
/// 🧩️ Puzzle-3d play app. Owns the precompute engine and the gumball scratch session; the persisted
/// document (bare `Puzzle3dFixture` json) lives in the wrapping `VcsDocumentApp`'s operation store and
/// the view state in `Puzzle3dConfig`. Each action rehydrates the engine from the projection, mutates
/// a transient [`Puzzle3dScene`], then emits the granular operation delta.
///
/// 🧲️ Gumball drags use a scratch-commit session (`transform_drag_active` + `transform_base` /
/// `transform_scratch`): mid-drag ticks accumulate incremental deltas onto the scratch and emit no
/// operations; `transformEnd` commits the base→scratch fixture delta once.
pub #[derive(Default, Clone, Copy)]
struct Puzzle3dPlayApp;

impl Default for Puzzle3dPlayApp
}

impl Puzzle3dPlayApp
        let (_, instances, meshes) = cache.as_ref().expect("geometry cache populated");
        (instances.clone(), meshes.clone())
    }

    fn document_sections_cached(&self, fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> Vec<UiTreeSectionNode> {
        let fingerprint = main::fixture_geometry_fingerprint(fixture);
        let mut cache = (std::sync::Mutex::new(None)).lock().expect("document cache");
        if cache.as_ref().is_none_or(|(fp, _)| *fp != fingerprint) {
            *cache = Some((fingerprint, document::sections(fixture, labels)));
        }
        cache.as_ref().expect("document cache populated").1.clone()
    }

    /// 🎬️ Snapshots the live fixture as the gumball drag base and clears any prior scratch.
    fn begin_transform_session(&self, projection: &Value) {
        let fixture = serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture());
        *(std::cell::RefCell::new(false)).borrow_mut() = true;
        *(std::cell::RefCell::new(None)).borrow_mut() = Some(fixture);
        *(std::cell::RefCell::new(None)).borrow_mut() = None;
    }

    /// 🧹️ Drops an in-progress gumball scratch without committing.
    pub(crate) fn clear_transform_session(&self) {
        *(std::cell::RefCell::new(false)).borrow_mut() = false;
        *(std::cell::RefCell::new(None)).borrow_mut() = None;
        *(std::cell::RefCell::new(None)).borrow_mut() = None;
    }

    /// 🧲️ One mid-drag gumball tick: accumulates an incremental delta onto `transform_scratch`
    /// (seeded from the drag-start base) and emits zero operations (scratch-commit pattern b).
    pub(crate) fn transform_drag_tick(&self, action: &str, args: Option<&Value>, projection: &Value, config: &Puzzle3dConfig) -> Emit<Puzzle3dOperation, Puzzle3dConfigOperation> {
        if (std::cell::RefCell::new(None)).borrow().is_none() {
            self.begin_transform_session(projection);
        }
        let object_ids = mesh_selection_ids(args, &config.selection.object_ids);
        let volume_ids = config.selection.target_volume_ids.to_vec();
        let mut scratch = (std::cell::RefCell::new(None)).borrow().clone().or_else(|| (std::cell::RefCell::new(None)).borrow().clone()).unwrap_or_else(empty_fixture);
        let axis = |key: &str, fallback: f64| args.and_then(|value| value.get(key)).and_then(|value| value.as_f64()).unwrap_or(fallback);
        match action {
            "translateSelection" => puzzle3d_apply_translate(&mut scratch, &object_ids, &volume_ids, axis("dx", 0.0), axis("dy", 0.0), axis("dz", 0.0)),
            "rotateSelection" => puzzle3d_apply_rotate(&mut scratch, &object_ids, &volume_ids, axis("ax", 0.0), axis("ay", 0.0), axis("az", 0.0), axis("angle", 0.0)),
            "scaleSelection" => puzzle3d_apply_scale(&mut scratch, &object_ids, &volume_ids, axis("sx", 1.0), axis("sy", 1.0), axis("sz", 1.0)),
            _ => {}
        }
        *(std::cell::RefCell::new(None)).borrow_mut() = Some(scratch);
        {
            let next = (std::cell::RefCell::new(0u64)).borrow().wrapping_add(1);
            *(std::cell::RefCell::new(0u64)).borrow_mut() = next;
        }
        Emit { ui_scope: puzzle3d_transform_drag_scope(), ..Default::default() }
    }

    /// 📌️ Commits the whole gumball drag as ONE fixture delta (base → scratch), resolving attractions once.
    pub(crate) fn commit_transform(&self, projection: &Value, config: &Puzzle3dConfig) -> Emit<Puzzle3dOperation, Puzzle3dConfigOperation> {
        *(std::cell::RefCell::new(false)).borrow_mut() = false;
        let Some(mut scratch) = (std::cell::RefCell::new(None)).borrow_mut().take() else {
            *(std::cell::RefCell::new(None)).borrow_mut() = None;
            return Emit::default();
        };
        *(std::cell::RefCell::new(None)).borrow_mut() = None;
        let object_ids = config.selection.object_ids.to_vec();
        let incoming = resolve_puzzle3d_attractions(&mut scratch);
        puzzle3d_rederive_moved_attractions(&mut scratch, &object_ids, &incoming);
        resolve_puzzle3d_attractions(&mut scratch);
        let operations = puzzle3d_operations_from_fixture_change(projection, &scratch);
        if operations.is_empty() {
            Emit { ui_scope: puzzle3d_transform_drag_scope(), ..Default::default() }
        } else {
            Emit::commit(operations, "Transform selection")
        }
    }

    /// 🖼️ Fixture used for world render — live scratch while a gumball drag is in progress.
    fn render_fixture(&self, projection: &Value) -> Puzzle3dFixture {
        if let Some(scratch) = (std::cell::RefCell::new(None)).borrow().as_ref() {
            return scratch.clone();
        }
        serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture())
    }

    //#region 🔖️GesturePreview
    /// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": the live gumball
    /// drag's current fixture state, expressed as the same document-delta operations
    /// `commit_transform` would eventually emit for real — anchored to the drag-start snapshot
    /// (`transform_base`), never to the previous preview tick, so a preview built from this stays
    /// correct even when the lossy, uncredited preview lane drops every message but the latest.
    /// `None` outside an active drag; this reads `transform_base`/`transform_scratch` only, never
    /// emits or mutates a `Puzzle3dOperation`.
    ///
    /// 🚧️ Deliberately unwired beyond this accessor — `framework/sync::SyncSession::publish_preview`
    /// is host-only and unreachable from this WASI-P2 sandboxed plugin crate, and
    /// `store::BackboneMessage` has no preview-shaped variant to relay one through.
    /// `#[allow(dead_code)]`: exercised by `🧪️Tests` only until a host bridge exists.
    #[allow(dead_code)]
    pub(crate) fn gesture_preview(&self) -> Option<(&'static str, u64, Vec<u8>)> {
        let base_binding = (std::cell::RefCell::new(None)).borrow();
        let base = base_binding.as_ref()?;
        let scratch_binding = (std::cell::RefCell::new(None)).borrow();
        let scratch = scratch_binding.as_ref()?;
        let before = serde_json::to_value(base).ok()?;
        let operations = puzzle3d_operations_from_fixture_change(&before, scratch);
        let payload = json!({ "operations": operations });
        Some(("gesture:transform", *(std::cell::RefCell::new(0u64)).borrow(), serde_json::to_vec(&payload).ok()?))
    }
    //#endregion 🔖️GesturePreview

    /// 🧾️ Rebuilds the transient render bundle for one `(projection, config, window)` triple, with the
    /// window instance's own view-local options materialized onto the runtime.
    fn scene_for(&self, projection: &Value, config: &Puzzle3dConfig, window_id: &str) -> Puzzle3dScene {
        let active_utility = puzzle3d_scene_active_utility(config, Some(window_id));
        let mut runtime_for_window = config.clone();
        runtime_for_window.load_window(window_id);
        scene_from_projection(projection, runtime_for_window, &active_utility)
    }

    /// @emoji 🧩️ B1: the pure per-action core, dispatched into by `DocumentApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle3dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle3dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    fn handle_action_impl(&self, action: &str, args: Option<&Value>, window_id: Option<&str>, doc: &DocumentView<'_, Puzzle3dPlayProjection>, config: &Puzzle3dConfig) -> Emit<Puzzle3dOperation, Puzzle3dConfigOperation> {
        // 🗨️ Shell-only effect (no document interaction, hence no scene/before/after scaffolding
        // below): opens the declared "addObject" dialog over a glass veil.
        if action == "openAddObjectDialog" {
            return Emit::effect(HostEffect::OpenDialog { dialog_id: "addObject".into(), args: None });
        }
        if action == "transformBegin" {
            self.begin_transform_session(&doc.projection.0);
            return Emit::default();
        }
        if action == "transformEnd" {
            return self.commit_transform(&doc.projection.0, config);
        }
        if *(std::cell::RefCell::new(false)).borrow() && matches!(action, "translateSelection" | "rotateSelection" | "scaleSelection") {
            return self.transform_drag_tick(action, args, &doc.projection.0, config);
        }
        let document_action = puzzle3d_action_document_intent(action);
        let before = document_action.then(|| doc.projection.0.clone());
        let active_utility_initial = puzzle3d_scene_active_utility(config, window_id);
        // 🪟️ This action targets exactly one window instance — materialize ITS view-local options onto
        // the scene runtime before handling, and snapshot them back out (via `save_window`) so a
        // grid/LOD/selection/vortex/sun mutation never leaks into another window's options. Fill count
        // / distribution / overlap stay on the flat runtime and are shared.
        let wid = window_id.map_or_else(|| main::WINDOW_KIND_ID.into(), str::to_string);
        let mut runtime_for_window = config.clone();
        // 🪟️ B1: self-maintaining window registry — was host-pushed `view_state.window_instances`; now
        // the app itself remembers every window instance id it has ever been dispatched an action for,
        // so `window_engagements`/`window_measures` still see every live split pane.
        if !runtime_for_window.window_ids.iter().any(|id| id == &wid) {
            runtime_for_window.window_ids.push(wid.clone());
        }
        runtime_for_window.load_window(&wid);
        let mut scene = scene_from_projection(&doc.projection.0, runtime_for_window, &active_utility_initial);
        let mut ui_scope = UiDirtyScope::Full;
        let mut effects = Vec::new();
        let preserve_fill_plan = matches!(action, "setFillCount" | "fillBuildTick");
        let skip_precompute_sync = matches!(action, "worldPick" | "worldSelect" | "setSelection" | "clearSelection" | "selectAll");
        if !preserve_fill_plan && !skip_precompute_sync {
            sync_precompute_session(&mut (std::cell::RefCell::new(Puzzle3dPrecomputeSession::default())).borrow_mut(), &scene);
        }
        let mut ctx = Puzzle3dActionCtx { app: self, scene: &mut scene, window_id: &wid, config, ui_scope: &mut ui_scope, abort: false };
        dispatch_puzzle3d_action(&mut ctx, action, args);
        let aborted = ctx.abort;
        if aborted {
            return Emit::default();
        }
        ui_scope = match action {
            "setHover" | "worldHover" => puzzle3d_chrome_scope(),
            "setCamera" | "setProjection" | "setProjectionParam" | "focusSelection" => puzzle3d_viewport_scope(),
            "worldPick" | "worldSelect" | "setSelection" | "clearSelection" | "selectAll" | "worldVortexHover" | "worldVortexSelect" => puzzle3d_selection_scope(),
            _ => ui_scope,
        };
        if puzzle3d_chrome_action(action) {
            effects.push(puzzle3d_patch_chrome_effect(&scene));
        }
        let next_active_utility = scene.active_utility.clone();
        scene.runtime.save_window(&wid);
        let operations = if let Some(before) = before.as_ref() {
            puzzle3d_operations_from_fixture_change(before, &scene.fixture)
        } else {
            debug_assert!(!puzzle3d_action_document_intent(action));
            Vec::new()
        };
        let coalesce_key = match action {
            "translateSelection" => Some("gumball-translate".to_string()),
            "rotateSelection" => Some("gumball-rotate".to_string()),
            "scaleSelection" => Some("gumball-scale".to_string()),
            "setFillCount" => Some("fill-count".to_string()),
            _ => None,
        };
        // 🧰️🛠️ Programmatic utility/tool switches (engagement submit/abort, suggestions, fill) push the
        // active utility/tool back into the host session; `setActiveUtility`/`setActiveTool` themselves
        // never re-emit (the command IS the direct switch, so this arm self-excludes). Fill transitions
        // go through `SetActiveTool` exclusively — the window's real utility is untouched by entering or
        // leaving the fill tool; a genuine utility transition (not involving fill on either side) still
        // emits `SetActiveUtility` exactly as before.
        let is_direct_utility_switch = matches!(action, x if x == SET_ACTIVE_UTILITY_ACTION_ID || x == SET_ACTIVE_TOOL_ACTION_ID);
        let initial_is_fill_tool = active_utility_initial == fill_tool::TOOL_ID;
        let next_is_fill_tool = next_active_utility == fill_tool::TOOL_ID;
        if !is_direct_utility_switch && next_is_fill_tool != initial_is_fill_tool {
            effects.push(HostEffect::SetActiveTool { tool_id: if next_is_fill_tool { fill_tool::TOOL_ID.into() } else { String::new() } });
        }
        if !is_direct_utility_switch && !next_is_fill_tool && !initial_is_fill_tool && next_active_utility != active_utility_initial {
            effects.push(HostEffect::SetActiveUtility { window_id: wid, utility_id: next_active_utility });
        }
        // 🧮️ B1: only a REAL config change becomes a `Puzzle3dConfigOperation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action (e.g. a re-materialize/re-save of an
        // already-idle window's options) from creating a no-op undo entry.
        let config_operations = if &scene.runtime != config { vec![Puzzle3dConfigOperation::Snapshot { config: scene.runtime }] } else { Vec::new() };
        Emit { document_operations: operations, config_operations, coalesce_key, effects, ui_scope, ..Default::default() }
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️component.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle3d_action(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "setFixtureJson" => example::set_fixture_json(ctx, args),
        "setActiveExample" => example::set_active_example(ctx, args),
        "setSelection" => selection_commands::set_selection(ctx, args),
        "worldSelect" => selection_commands::world_select(ctx, args),
        "worldPick" => selection_commands::world_pick(ctx, args),
        "worldVortexSelect" => selection_commands::world_vortex_select(ctx, args),
        "selectAll" => selection_commands::select_all(ctx),
        "clearSelection" => selection_commands::clear_selection(ctx),
        "selectSameKindSelection" => selection_commands::select_same_kind(ctx),
        "contextMenuAt" => selection_commands::context_menu_at(ctx, args),
        "setSelectionMethod" => selection_commands::set_selection_method(ctx, args),
        "setSelectionModeDefault" => selection_commands::set_selection_mode_default(ctx, args),
        "setSelectableKind" => selection_commands::set_selectable_kind(ctx, args),
        "worldHover" => hover::world_hover(ctx, args),
        "setHover" => hover::set_hover(ctx, args),
        "worldVortexHover" => hover::world_vortex_hover(ctx, args),
        "setKindHover" => hover::set_kind_hover(ctx, args),
        "addObjectKind" => object::add_object_kind(ctx, args),
        "deleteSelection" => object::delete_selection(ctx),
        "duplicateSelection" => object::duplicate_selection(ctx),
        "setSelectionFlag" => object::set_selection_flag(ctx, args),
        "patchInspector" => object::patch_inspector(ctx, args),
        "createAttraction" => attraction::create_attraction(ctx, args),
        "deleteAttraction" => attraction::delete_attraction(ctx, args),
        "addTargetVolume" => volume::add_target_volume(ctx, args),
        "deleteTargetVolume" => volume::delete_target_volume(ctx, args),
        "setTargetVolumeFlag" => volume::set_target_volume_flag(ctx, args),
        "relocateTargetVolume" => volume::relocate_target_volume(ctx, args),
        "setCamera" => camera::set_camera(ctx, args),
        "setProjection" | "setProjectionParam" => camera::set_projection(ctx, action, args),
        "focusSelection" => camera::focus_selection(ctx),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => sun::apply(ctx, action, args),
        "setLodAutomatic" => lod::set_automatic(ctx, args),
        "setLodDepthVariable" => lod::set_depth_variable(ctx, args),
        "setLodManual" => lod::set_manual(ctx, args),
        "setGridVisible" => grid::set_visible(ctx, args),
        "setGridSnapEnabled" => grid::set_snap_enabled(ctx, args),
        "setGridSpacing" => grid::set_spacing(ctx, args),
        "setProximityRadius" => settings::set_proximity_radius(ctx, args),
        "setChunkSize" => settings::set_chunk_size(ctx, args),
        "setBrushPlacementOverlapBudget" => settings::set_brush_placement_overlap_budget(ctx, args),
        "setVoxelDims" => settings::set_voxel_dims(ctx, args),
        "setTransformGumballFlag" => settings::set_transform_gumball_flag(ctx, args),
        "setVortexShow" => settings::set_vortex_show(ctx, args),
        "setVortexDirection" => settings::set_vortex_direction(ctx, args),
        "translateSelection" => transform::translate_selection(ctx, args),
        "rotateSelection" => transform::rotate_selection(ctx, args),
        "scaleSelection" => transform::scale_selection(ctx, args),
        "worldRelocate" => transform::world_relocate(ctx, args),
        "addBrushObject" => brush::add_brush_object(ctx, args),
        "cycleBrushCandidate" | "cycleBrushCandidateBack" => brush::cycle_candidate(ctx, action, args),
        "openVortexSuggestions" => brush::open_vortex_suggestions(ctx, args),
        "closeVortexSuggestions" => brush::close_vortex_suggestions(ctx),
        "hoverSuggestion" => brush::hover_suggestion(ctx, args),
        "acceptSuggestion" => brush::accept_suggestion(ctx, args),
        "suggestionsTick" => brush::suggestions_tick(ctx),
        "registerBrushMesh" => brush::register_brush_mesh(ctx, args),
        "engagementControlSelect" => brush::engagement_control_select(ctx, args),
        "setFillCount" => fill::set_fill_count(ctx, args),
        "fillBuildTick" => fill::fill_build_tick(ctx),
        "setObjectKindWeight" | "setVortexKindWeight" => fill::set_kind_weight(ctx, action, args),
        "engagementInput" => engagement::engagement_input(ctx, args),
        "engagementSubmit" => engagement::engagement_submit(ctx, args),
        "engagementRepeatLast" => engagement::engagement_repeat_last(ctx),
        "engagementAbort" => engagement::engagement_abort(ctx),
        "setLocale" => locale::set_locale(ctx, args),
        "setTerminology" => locale::set_terminology(ctx, args),
        SET_ACTIVE_UTILITY_ACTION_ID | SET_ACTIVE_TOOL_ACTION_ID => utility::set_active(ctx, action, args),
        "worldPointerDown" => {}
        _ => {}
    }
}

impl DocumentApp for Puzzle3dPlayApp

    /// 🏷️ Maps each `Puzzle3dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle3dCommand) -> &'static str {
        command.action_id()
    }

    /// @emoji 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle3dCommand`.
    fn handle(command: &Puzzle3dCommand, doc: &DocumentView<'_, Puzzle3dPlayProjection>, cfg: &ConfigView<'_, Puzzle3dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Puzzle3dOperation, Puzzle3dConfigOperation, Self::DraftOperation>, Fault> {
        Ok(self.handle_action_impl(command.action_id(), command.args(), command.window_id(), doc, cfg.projection))
    }

    /// 🔌️ Declares puzzle3d's typed media I/O surface — the implicit document ports plus the flagship
    /// `kit:in` seam: an input port accepting `Kit×Type` media tagged `kit.catalog`, fanning IN from
    /// potentially many producers (`multiplicity: Many`).
    fn io() -> Option<AppIo> {
        Some(
            AppIo::from_document(
                "puzzle.3d",
                MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
                semio_framework_plugin::ArtifactPresentation { id: "3d.puzzle".into(), name: "3D Puzzle".into(), dimension: "3d".into(), component_kind: "puzzle3d".into() },
            )
            .with_ports(vec![MediaPortSpec {
                id: "kit:in".into(),
                label: "Kit Catalog".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                kind_id: Some("kit.catalog".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            }]),
        )
    }

    /// 🎞️ `kit:in` seam: normalizes an incoming `kit.catalog` fragment (`objectKinds`/`vortexKinds`/
    /// `cableKinds`/`attractionKinds`/`kindCompatibility`) into puzzle3d's own `meta.kind_catalogs`
    /// vocabulary (`objects`/`vortices`/`cables`/`attractions`) and upserts it (keyed by row `id`,
    /// deterministic/order-independent — safe for `multiplicity: Many` fan-in) via the same
    /// `puzzle3d_operations_from_fixture_change` delta bridge every other fixture-mutating action
    /// already uses, so this never mutates anything directly — only real, undoable operations.
    fn import_media(port: &str, media: &Media, doc: &DocumentView<'_, Puzzle3dPlayProjection>) -> Result<Emit<Puzzle3dOperation, Puzzle3dConfigOperation, Self::DraftOperation>, MediaError> {
        if port != "kit:in" {
            return Err(MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "kit:in only accepts a Structured (JSON) payload".into()));
        };
        let fragment: Value = serde_json::from_str(json.as_str()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let mut fixture: Puzzle3dFixture = serde_json::from_value(doc.projection.0.clone()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;

        let mut catalogs = fixture.meta.kind_catalogs.clone().unwrap_or_else(|| json!({ "objects": [], "vortices": [], "cables": [], "attractions": [] }));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "objects", fragment.get("objectKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "vortices", fragment.get("vortexKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "cables", fragment.get("cableKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "attractions", fragment.get("attractionKinds"));
        fixture.meta.kind_catalogs = Some(catalogs);

        if let Some(incoming_compat) = fragment.get("kindCompatibility").and_then(Value::as_array) {
            let mut compat: Vec<Value> = fixture.meta.kind_compatibility.as_ref().and_then(Value::as_array).cloned().unwrap_or_default();
            for row in incoming_compat {
                let source = row.get("source").and_then(Value::as_str).unwrap_or_default();
                let target = row.get("target").and_then(Value::as_str).unwrap_or_default();
                match compat.iter().position(|entry| entry.get("source").and_then(Value::as_str) == Some(source) && entry.get("target").and_then(Value::as_str) == Some(target)) {
                    Some(index) => compat[index] = row.clone(),
                    None => compat.push(row.clone()),
                }
            }
            fixture.meta.kind_compatibility = Some(Value::Array(compat));
        }

        let operations = puzzle3d_operations_from_fixture_change(&doc.projection.0, &fixture);
        Ok(Emit::operations(operations))
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Puzzle3dPlayProjection>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> UiNode {
        let (base_body_key, window_id_from_key) = body_key.split_once(':').map_or((body_key, None), |(b, w)| (b, Some(w)));
        let config = cfg.projection;
        let wid = window_id_from_key.or_else(|| config.window_ids.first().map(String::as_str)).unwrap_or(main::WINDOW_KIND_ID);
        let active_utility = puzzle3d_scene_active_utility(config, Some(wid));
        let mut runtime_for_window = config.clone();
        if !runtime_for_window.window_ids.iter().any(|id| id == wid) {
            runtime_for_window.window_ids.push(wid.to_string());
        }
        runtime_for_window.load_window(wid);
        // 🪣️ Additive-only: appends just the not-yet-committed fill-plan tail onto the live fixture —
        // safe even during a live gumball scratch drag, since it never touches/replaces any
        // already-present object (the dragged one included).
        let fill_available = (std::cell::RefCell::new(Puzzle3dPrecomputeSession::default())).borrow().fill_available_count();
        let fixture = puzzle3d_fixture_with_fill_display_memo(self.render_fixture(&doc.projection.0), &(std::cell::RefCell::new(Puzzle3dPrecomputeSession::default())).borrow(), runtime_for_window.fill_count, fill_available, &(std::sync::Mutex::new(None)));
        let envelope = Puzzle3dScene { fixture, runtime: runtime_for_window, active_utility };
        let labels = puzzle3d_labels(config);
        match base_body_key {
            main::BODY_KEY => {
                let (instances_json, meshes_json) = self.geometry_jsons(&envelope.fixture);
                main::render(&envelope, &(std::cell::RefCell::new(Puzzle3dPrecomputeSession::default())).borrow(), instances_json, meshes_json)
            }
            document::BODY_KEY => document::render((std::sync::Mutex::new(None))d(&envelope.fixture, labels), &envelope.runtime.selection),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels),
            inspection::BODY_KEY => inspection::render(&envelope, labels),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(doc: &DocumentView<'_, Puzzle3dPlayProjection>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.projection;
        let labels = puzzle3d_labels(config);
        // 🪟️ One entry per live window INSTANCE (split top/perspective panes are two instances of the
        // same kind) — each built from ITS OWN materialized options, never the shared kind entry.
        window_instance_ids(config, main::WINDOW_KIND_ID)
            .into_iter()
            .map(|wid| {
                let envelope = self.scene_for(&doc.projection.0, config, &wid);
                (wid, main::engagement(&envelope, labels))
            })
            .collect()
    }

    fn window_measures(doc: &DocumentView<'_, Puzzle3dPlayProjection>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let labels = puzzle3d_labels(config);
        window_instance_ids(config, main::WINDOW_KIND_ID)
            .into_iter()
            .map(|wid| {
                let envelope = self.scene_for(&doc.projection.0, config, &wid);
                (wid, main::window_measures(&envelope, &(std::cell::RefCell::new(Puzzle3dPrecomputeSession::default())).borrow(), labels))
            })
            .collect()
    }

    fn tool_measures(doc: &DocumentView<'_, Puzzle3dPlayProjection>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let wid = config.window_ids.first().map_or(main::WINDOW_KIND_ID, String::as_str);
        let labels = puzzle3d_labels(config);
        let envelope = self.scene_for(&doc.projection.0, config, wid);
        HashMap::from([(fill_tool::TOOL_ID.to_string(), fill_tool::measures(&envelope, &(std::cell::RefCell::new(Puzzle3dPrecomputeSession::default())).borrow(), labels))])
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &DocumentView<'_, Puzzle3dPlayProjection>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = cfg.projection;
        let labels = puzzle3d_labels(config);
        let wid = config.window_ids.first().map_or(main::WINDOW_KIND_ID, String::as_str);
        let active_utility = puzzle3d_scene_active_utility(config, Some(wid));
        let mut envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
        if let Some(surface) = request.surface.as_ref() {
            let object_ids: Vec<String> = surface.selection.iter().filter(|g| g.domain == "object" || g.domain == "node").flat_map(|g| g.ids.iter().cloned()).collect();
            if !object_ids.is_empty() {
                envelope.runtime.selection.object_ids = object_ids.into();
            }
        }
        puzzle3d_context_menu_items(&envelope, labels, registry)
    }
}
