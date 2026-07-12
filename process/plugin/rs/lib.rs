//! 🪚 Process plugin — subtractive/additive processing simulation in one hot-swappable WASM component.

pub mod app_3d {
    //! 🪚 Process 3D plugin — subtractive/additive processing simulation bundled as a hot-swappable WASM component.

    use kernel_3d_brepkit::BrepkitKernel;
    use kernel_3d_engine::GeometryHandle;
    use process_3d::{Pose, ProcessMeasure, ProcessStep, SolidSpec, Stock};
    use semio_framework_plugin::{
        build_world_3d_scene, create_default_layout, mesh_from_indexed, mesh_from_kind, tool_button, tool_separator,
        ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, world3d_camera_json, world3d_scene,
        world3d_selection_json, ActionDescriptor, App, MeshData, PanelGroup, PluginApp, SurfaceKind, ToolNode, UiFieldNode,
        UiInputNode, UiInspectorFieldGroup, UiNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
        ViewState, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementOption,
        FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
        FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
        FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    };
    use semio_framework_plugin::layout::WindowEngagementStatus;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Mutex, OnceLock};

    //#region 🔖Constants
    const PROCESS_3D_PLAY_APP_ID: &str = "process3d-play";
    const PROCESS_3D_PLAY_CONTROLLER_ID: &str = "process3d-play";
    const PROCESS_3D_PLAY_SURFACE_MAIN: &str = "process.play";
    const PROCESS_3D_PLAY_BODY_MAIN: &str = "process.play.main";
    const PROCESS_3D_PLAY_BODY_DOCUMENT: &str = "process.play.document";
    const PROCESS_3D_PLAY_BODY_CATALOGUE: &str = "process.play.catalogue";
    const PROCESS_3D_PLAY_BODY_INSPECTION: &str = "process.play.inspection";
    const PROCESS_3D_PLAY_WINDOW_MAIN: &str = "process-workpiece";
    const PROCESS3D_ENGAGEMENT_TOOL_SELECT: &str = "process3d.tool.select";
    const PROCESS3D_ENGAGEMENT_TOOL_CUT: &str = "process3d.tool.cut";
    const PROCESS3D_ENGAGEMENT_TOOL_DRILL: &str = "process3d.tool.drill";
    const PROCESS3D_ENGAGEMENT_TOOL_ATTACH: &str = "process3d.tool.attach";
    /// ⏪ Actions that mutate the shared `fixture` in place and should be undoable — excludes `setDocument`/
    /// `setActiveExample` (wholesale envelope swaps) and view-only state (selection/hover/camera/tool/cursor).
    const PROCESS3D_UNDOABLE_ACTIONS: &[&str] =
        &["addStep", "removeStep", "removeSelectedStep", "updateStep", "moveStep", "setStepEnabled", "patchInspector", "setStock"];
    const PROCESS3D_UNDO_STACK_MAX: usize = 50;
    const PROCESS3D_TESSELLATION_TOLERANCE: f64 = 0.05;
    const PROCESS3D_FALLBACK_MESH_KIND: &str = "box";
    const PROCESS3D_KERNEL_MEMO_CAP: usize = 128;
    const PROCESS3D_EXAMPLE_TIMBER: &str = "timber-beam-joinery";
    const PROCESS3D_EXAMPLE_PLATE: &str = "drilled-plate";
    const TIMBER_EXAMPLE_JSON: &str = include_str!("../../3d/example/timber-beam-joinery.process.json");
    const PLATE_EXAMPLE_JSON: &str = include_str!("../../3d/example/drilled-plate.process.json");

    static PROCESS3D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn next_step_id() -> String {
        format!("step-{}", PROCESS3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    //#endregion 🔖Constants

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the 3D app; one field per label makes every locale combination compile-checked.
    struct Process3dLabels {
        stock: &'static str,
        steps: &'static str,
        cut: &'static str,
        drill: &'static str,
        attach: &'static str,
        enabled: &'static str,
        volume: &'static str,
        label_field: &'static str,
        no_selection: &'static str,
        remove: &'static str,
    }

    const PROCESS3D_LABELS_NATIVE_EN: Process3dLabels = Process3dLabels {
        stock: "Stock",
        steps: "Steps",
        cut: "Cut",
        drill: "Drill",
        attach: "Attach",
        enabled: "Enabled",
        volume: "Volume",
        label_field: "Label",
        no_selection: "No selection",
        remove: "Remove",
    };

    const PROCESS3D_LABELS_NATIVE_DE: Process3dLabels = Process3dLabels {
        stock: "Rohteil",
        steps: "Schritte",
        cut: "Schnitt",
        drill: "Bohrung",
        attach: "Anbau",
        enabled: "Aktiviert",
        volume: "Volumen",
        label_field: "Bezeichnung",
        no_selection: "Keine Auswahl",
        remove: "Entfernen",
    };

    /// 🗣️ Resolves the active label set from the shell-provided locale; falls back to native English.
    fn process3d_labels(view_state: &ViewState) -> &'static Process3dLabels {
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        if is_de { &PROCESS3D_LABELS_NATIVE_DE } else { &PROCESS3D_LABELS_NATIVE_EN }
    }

    fn process3d_measure_icon(measure: &ProcessMeasure) -> &'static str {
        match measure {
            ProcessMeasure::Cut { .. } => "scissors",
            ProcessMeasure::Drill { .. } => "circle-dot",
            ProcessMeasure::Attach { .. } => "plus",
        }
    }

    fn process3d_measure_label<'a>(measure: &ProcessMeasure, labels: &'a Process3dLabels) -> &'a str {
        match measure {
            ProcessMeasure::Cut { .. } => labels.cut,
            ProcessMeasure::Drill { .. } => labels.drill,
            ProcessMeasure::Attach { .. } => labels.attach,
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖Document
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Process3dCamera {
        #[serde(default = "default_cam_position")]
        position: [f64; 3],
        #[serde(default)]
        target: [f64; 3],
        #[serde(default = "default_cam_fov")]
        fov: f64,
    }

    impl Default for Process3dCamera {
        fn default() -> Self {
            Self { position: default_cam_position(), target: [0.0, 0.0, 0.0], fov: default_cam_fov() }
        }
    }

    fn default_cam_position() -> [f64; 3] {
        [3.0, -3.0, 2.0]
    }

    fn default_cam_fov() -> f64 {
        45.0
    }

    fn default_selection_method() -> String {
        "rectangle".into()
    }

    fn default_active_tool() -> String {
        "select".into()
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Process3dPreviewCache {
        signature: u64,
        meshes_json: String,
        instances_json: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Process3dRuntime {
        #[serde(default)]
        selected_id: Option<String>,
        #[serde(default)]
        hovered_id: Option<String>,
        #[serde(default = "default_selection_method")]
        selection_method: String,
        #[serde(default = "default_active_tool")]
        active_tool: String,
        #[serde(default)]
        engagement_input: String,
        #[serde(default)]
        camera: Process3dCamera,
        /// ⏮️ Fixture snapshots for undo, pushed before structural edits.
        #[serde(default)]
        undo_stack: Vec<process_3d::Process3dDocument>,
        /// ⏭️ Fixture snapshots for redo, cleared whenever a new edit is snapshotted.
        #[serde(default)]
        redo_stack: Vec<process_3d::Process3dDocument>,
        #[serde(default)]
        preview_cache: Option<Process3dPreviewCache>,
    }

    impl Default for Process3dRuntime {
        fn default() -> Self {
            Self {
                selected_id: None,
                hovered_id: None,
                selection_method: default_selection_method(),
                active_tool: default_active_tool(),
                engagement_input: String::new(),
                camera: Process3dCamera::default(),
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),
                preview_cache: None,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Process3dEnvelope {
        fixture: process_3d::Process3dDocument,
        #[serde(default)]
        runtime: Process3dRuntime,
    }

    fn envelope_from_fixture_json(json_text: &str) -> Option<Process3dEnvelope> {
        serde_json::from_str::<process_3d::Process3dDocument>(json_text).ok().map(|fixture| {
            let mut envelope = Process3dEnvelope { fixture, runtime: Process3dRuntime::default() };
            refresh_preview_cache(&mut envelope.runtime, &envelope.fixture);
            envelope
        })
    }

    fn default_envelope() -> Process3dEnvelope {
        envelope_from_fixture_json(TIMBER_EXAMPLE_JSON)
            .unwrap_or_else(|| Process3dEnvelope { fixture: process_3d::Process3dDocument::default(), runtime: Process3dRuntime::default() })
    }

    fn plate_envelope() -> Process3dEnvelope {
        envelope_from_fixture_json(PLATE_EXAMPLE_JSON).unwrap_or_else(default_envelope)
    }

    fn empty_envelope() -> Process3dEnvelope {
        Process3dEnvelope { fixture: process_3d::Process3dDocument::default(), runtime: Process3dRuntime::default() }
    }

    fn parse_envelope(document_json: &str) -> Process3dEnvelope {
        serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
    }

    fn set_document_op(envelope: &Process3dEnvelope) -> String {
        json!({ "op": "setDocument", "document": envelope }).to_string()
    }

    fn process3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: PROCESS_3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
    }

    fn value_as_vec3(value: &Value) -> Option<[f64; 3]> {
        let array = value.as_array()?;
        Some([array.first()?.as_f64()?, array.get(1)?.as_f64()?, array.get(2)?.as_f64()?])
    }

    fn selected_ids(envelope: &Process3dEnvelope) -> Vec<String> {
        envelope.runtime.selected_id.clone().into_iter().collect()
    }

    fn hash_value<T: Serialize>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        if let Ok(json) = serde_json::to_string(value) {
            json.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn fixture_signature(fixture: &process_3d::Process3dDocument) -> u64 {
        hash_value(fixture)
    }

    fn remove_step(fixture: &mut process_3d::Process3dDocument, id: &str) -> bool {
        let Some(index) = fixture.steps.iter().position(|step| step.id == id) else {
            return false;
        };
        fixture.steps.retain(|step| step.id != id);
        if let Some(cursor) = fixture.resolved_up_to {
            if cursor > index {
                fixture.resolved_up_to = Some(cursor - 1);
            }
        }
        true
    }

    fn default_cut_measure() -> ProcessMeasure {
        ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.05, depth: 0.5, height: 0.5 }, pose: Pose::default() }
    }

    fn default_drill_measure() -> ProcessMeasure {
        ProcessMeasure::Drill { radius: 0.05, depth: 0.3, pose: Pose::default() }
    }

    fn default_attach_measure() -> ProcessMeasure {
        ProcessMeasure::Attach { component: SolidSpec::Cylinder { radius: 0.03, height: 0.2 }, pose: Pose::default() }
    }

    fn measure_for_kind(kind: &str, position: Option<[f64; 3]>) -> ProcessMeasure {
        let mut measure = match kind {
            "drill" => default_drill_measure(),
            "attach" => default_attach_measure(),
            _ => default_cut_measure(),
        };
        if let Some(position) = position {
            let pose = match &mut measure {
                ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose,
            };
            pose.position = position;
        }
        measure
    }

    fn label_for_kind(kind: &str, labels: &Process3dLabels) -> String {
        match kind {
            "drill" => labels.drill,
            "attach" => labels.attach,
            _ => labels.cut,
        }
        .to_string()
    }

    /// ✂️➕ Inserts a new step at the cursor, advances the cursor past it, and selects it.
    fn insert_step_at_cursor(fixture: &mut process_3d::Process3dDocument, step: ProcessStep) {
        let cursor = fixture.resolved_up_to.unwrap_or(fixture.steps.len()).min(fixture.steps.len());
        fixture.steps.insert(cursor, step);
        fixture.resolved_up_to = Some((cursor + 1).min(fixture.steps.len()));
    }

    //#region 🔖InspectorPatch
    fn apply_pose_patch(pose: &mut Pose, field: &str, value: f64) -> bool {
        match field {
            "posX" => pose.position[0] = value,
            "posY" => pose.position[1] = value,
            "posZ" => pose.position[2] = value,
            "angle" => pose.angle = value,
            _ => return false,
        }
        true
    }

    fn apply_solid_patch(solid: &mut SolidSpec, field: &str, value: f64) -> bool {
        let clamped = value.max(0.001);
        match solid {
            SolidSpec::Box { width, depth, height } => match field {
                "width" => *width = clamped,
                "depth" => *depth = clamped,
                "height" => *height = clamped,
                _ => return false,
            },
            SolidSpec::Cylinder { radius, height } => match field {
                "radius" => *radius = clamped,
                "height" => *height = clamped,
                _ => return false,
            },
            SolidSpec::Sphere { radius } => match field {
                "radius" => *radius = clamped,
                _ => return false,
            },
        }
        true
    }

    fn apply_stock_patch(stock: &mut Stock, field: &str, value: Option<&Value>) -> bool {
        if field == "label" {
            return match value.and_then(Value::as_str) {
                Some(label) => {
                    stock.label = label.into();
                    true
                }
                None => false,
            };
        }
        let Some(number) = value.and_then(Value::as_f64) else { return false };
        apply_pose_patch(&mut stock.pose, field, number) || apply_solid_patch(&mut stock.solid, field, number)
    }

    /// 🔎 Generic inspector edit dispatcher for a step's measure — dimension fields are scoped to the
    /// measure's own solid ("radius"/"depth" for drill, "toolWidth..." for cut, "radius"/"height" for attach)
    /// so field names never collide across measure kinds.
    fn apply_step_patch(step: &mut ProcessStep, field: &str, value: Option<&Value>) -> bool {
        if field == "label" {
            return match value.and_then(Value::as_str) {
                Some(label) => {
                    step.label = label.into();
                    true
                }
                None => false,
            };
        }
        let Some(number) = value.and_then(Value::as_f64) else { return false };
        let clamped = number.max(0.001);
        match &mut step.measure {
            ProcessMeasure::Cut { tool, pose } => {
                if apply_pose_patch(pose, field, number) {
                    return true;
                }
                let SolidSpec::Box { width, depth, height } = tool else { return false };
                match field {
                    "toolWidth" => *width = clamped,
                    "toolDepth" => *depth = clamped,
                    "toolHeight" => *height = clamped,
                    _ => return false,
                }
                true
            }
            ProcessMeasure::Drill { radius, depth, pose } => {
                if apply_pose_patch(pose, field, number) {
                    return true;
                }
                match field {
                    "radius" => *radius = clamped,
                    "depth" => *depth = clamped,
                    _ => return false,
                }
                true
            }
            ProcessMeasure::Attach { component, pose } => {
                if apply_pose_patch(pose, field, number) {
                    return true;
                }
                let SolidSpec::Cylinder { radius, height } = component else { return false };
                match field {
                    "radius" => *radius = clamped,
                    "height" => *height = clamped,
                    _ => return false,
                }
                true
            }
        }
    }

    fn apply_process3d_inspector_patch(fixture: &mut process_3d::Process3dDocument, target: &str, field: &str, value: Option<&Value>) -> bool {
        if target == fixture.stock.id {
            return apply_stock_patch(&mut fixture.stock, field, value);
        }
        if let Some(step_id) = target.strip_prefix("step:") {
            if let Some(step) = fixture.steps.iter_mut().find(|step| step.id == step_id) {
                return apply_step_patch(step, field, value);
            }
        }
        false
    }
    //#endregion 🔖InspectorPatch
    //#endregion 🔖Document

    //#region 🔖KernelReplay
    /// 🧠 Kernel + prefix memo: `hash(stock, enabled steps[0..i])` → solid handle, so cursor scrubbing and
    /// step edits only recompute the suffix that actually changed.
    struct ProcessKernelSession {
        kernel: BrepkitKernel,
        memo: HashMap<u64, GeometryHandle>,
        stock_signature: u64,
    }

    impl ProcessKernelSession {
        fn new() -> Self {
            Self { kernel: BrepkitKernel::new(), memo: HashMap::new(), stock_signature: 0 }
        }
    }

    static PROCESS_BREP_KERNEL: OnceLock<Mutex<ProcessKernelSession>> = OnceLock::new();

    fn process_kernel_session() -> &'static Mutex<ProcessKernelSession> {
        PROCESS_BREP_KERNEL.get_or_init(|| Mutex::new(ProcessKernelSession::new()))
    }

    fn prefix_signature(stock_signature: u64, steps: &[&ProcessStep]) -> u64 {
        let mut hasher = DefaultHasher::new();
        stock_signature.hash(&mut hasher);
        if let Ok(json) = serde_json::to_string(steps) {
            json.hash(&mut hasher);
        }
        hasher.finish()
    }

    /// 📦 Builds a posed kernel solid for a spec via `*_prim_sync` → `rotate_sync` → `translate_sync`.
    fn solid_for_spec(kernel: &mut BrepkitKernel, spec: &SolidSpec, pose: &Pose) -> Option<GeometryHandle> {
        let base = match spec {
            SolidSpec::Box { width, depth, height } => kernel.box_prim_sync(*width, *depth, *height).ok()?,
            SolidSpec::Cylinder { radius, height } => kernel.cylinder_prim_sync(*radius, *height).ok()?,
            SolidSpec::Sphere { radius } => kernel.sphere_prim_sync(*radius).ok()?,
        };
        let rotated = if pose.angle != 0.0 { kernel.rotate_sync(&base, pose.axis, pose.angle).ok()? } else { base };
        if pose.position != [0.0, 0.0, 0.0] {
            kernel.translate_sync(&rotated, pose.position).ok()
        } else {
            Some(rotated)
        }
    }

    fn tool_solid_for_measure(kernel: &mut BrepkitKernel, measure: &ProcessMeasure) -> Option<GeometryHandle> {
        match measure {
            ProcessMeasure::Cut { tool, pose } => solid_for_spec(kernel, tool, pose),
            ProcessMeasure::Drill { radius, depth, pose } => solid_for_spec(kernel, &SolidSpec::Cylinder { radius: *radius, height: *depth }, pose),
            ProcessMeasure::Attach { component, pose } => solid_for_spec(kernel, component, pose),
        }
    }

    /// 🧠 Replays enabled steps up to the cursor, reusing the longest memoized prefix.
    fn replay_process(session: &mut ProcessKernelSession, doc: &process_3d::Process3dDocument) -> Option<GeometryHandle> {
        let stock_signature = hash_value(&doc.stock);
        if stock_signature != session.stock_signature {
            session.memo.clear();
            session.stock_signature = stock_signature;
        }
        let limit = doc.resolved_up_to.unwrap_or(doc.steps.len()).min(doc.steps.len());
        let enabled_steps: Vec<&ProcessStep> = doc.steps[..limit].iter().filter(|step| step.enabled).collect();

        let mut start = enabled_steps.len();
        let mut current: Option<GeometryHandle> = loop {
            let signature = prefix_signature(stock_signature, &enabled_steps[..start]);
            if let Some(handle) = session.memo.get(&signature) {
                break Some(handle.clone());
            }
            if start == 0 {
                break None;
            }
            start -= 1;
        };
        if current.is_none() {
            current = solid_for_spec(&mut session.kernel, &doc.stock.solid, &doc.stock.pose);
            if let Some(handle) = &current {
                session.memo.insert(prefix_signature(stock_signature, &[]), handle.clone());
            }
        }
        let mut handle = current?;
        for (index, step) in enabled_steps.iter().enumerate().skip(start) {
            let tool = tool_solid_for_measure(&mut session.kernel, &step.measure)?;
            handle = match step.measure {
                ProcessMeasure::Attach { .. } => session.kernel.fuse_sync(&handle, &tool).ok()?,
                _ => session.kernel.cut_sync(&handle, &tool).ok()?,
            };
            session.memo.insert(prefix_signature(stock_signature, &enabled_steps[..=index]), handle.clone());
        }
        if session.memo.len() > PROCESS3D_KERNEL_MEMO_CAP {
            if let Some(key) = session.memo.keys().next().copied() {
                session.memo.remove(&key);
            }
        }
        Some(handle)
    }

    fn processed_mesh(doc: &process_3d::Process3dDocument) -> Option<MeshData> {
        let mut session = process_kernel_session().lock().ok()?;
        let handle = replay_process(&mut session, doc)?;
        let mesh = session.kernel.tessellate_sync(&handle, PROCESS3D_TESSELLATION_TOLERANCE).ok()?;
        Some(mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index))
    }

    fn processed_volume(doc: &process_3d::Process3dDocument) -> Option<f64> {
        let mut session = process_kernel_session().lock().ok()?;
        let handle = replay_process(&mut session, doc)?;
        session.kernel.volume_sync(&handle).ok()
    }

    fn evaluated_preview_payload(fixture: &process_3d::Process3dDocument) -> (String, String) {
        let mesh = processed_mesh(fixture).unwrap_or_else(|| mesh_from_kind(PROCESS3D_FALLBACK_MESH_KIND));
        let meshes = json!([{ "id": "processed", "data": mesh }]);
        let instances = json!([{
            "id": "processed",
            "meshId": "processed",
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": fixture.stock.label,
            "selected": false,
            "hovered": false,
        }]);
        (meshes.to_string(), instances.to_string())
    }

    fn refresh_preview_cache(runtime: &mut Process3dRuntime, fixture: &process_3d::Process3dDocument) {
        let signature = fixture_signature(fixture);
        if runtime.preview_cache.as_ref().is_some_and(|entry| entry.signature == signature) {
            return;
        }
        let (meshes_json, instances_json) = evaluated_preview_payload(fixture);
        runtime.preview_cache = Some(Process3dPreviewCache { signature, meshes_json, instances_json });
    }

    fn preview_payload_cached(runtime: &Process3dRuntime, fixture: &process_3d::Process3dDocument) -> (String, String) {
        let signature = fixture_signature(fixture);
        if let Some(cache) = &runtime.preview_cache {
            if cache.signature == signature {
                return (cache.meshes_json.clone(), cache.instances_json.clone());
            }
        }
        evaluated_preview_payload(fixture)
    }

    fn finalize_document_op(envelope: &mut Process3dEnvelope) -> String {
        refresh_preview_cache(&mut envelope.runtime, &envelope.fixture);
        set_document_op(envelope)
    }
    //#endregion 🔖KernelReplay

    //#region 🔖Panels
    fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
        UiTreeItemNode {
            id: id.into(),
            label: label.into(),
            description: None,
            icon_id: icon_id.map(str::to_string),
            selected: None,
            default_open: None,
            action: Some(action),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }
    }

    fn number_field(id: impl Into<String>, label: impl Into<String>, value: f64, target: &str, field: &str) -> UiNode {
        let id = id.into();
        UiNode::Field(UiFieldNode {
            id: id.clone(),
            label: label.into(),
            description: None,
            required: None,
            error: None,
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("{id}.input"),
                input_kind: "number".into(),
                value: value.to_string(),
                placeholder: None,
                commit: None,
                on_change: process3d_action("patchInspector", Some(json!({ "target": target, "field": field }))),
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
        })
    }

    fn text_field(id: impl Into<String>, label: impl Into<String>, value: &str, target: &str, field: &str) -> UiNode {
        let id = id.into();
        UiNode::Field(UiFieldNode {
            id: id.clone(),
            label: label.into(),
            description: None,
            required: None,
            error: None,
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("{id}.input"),
                input_kind: "text".into(),
                value: value.into(),
                placeholder: None,
                commit: None,
                on_change: process3d_action("patchInspector", Some(json!({ "target": target, "field": field }))),
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
        })
    }

    fn build_document_tree(envelope: &Process3dEnvelope, labels: &Process3dLabels) -> UiNode {
        let stock = &envelope.fixture.stock;
        let stock_item = UiTreeItemNode {
            id: stock.id.clone(),
            label: stock.label.clone(),
            description: None,
            icon_id: Some("box".into()),
            selected: Some(envelope.runtime.selected_id.as_deref() == Some(stock.id.as_str())),
            default_open: None,
            action: Some(process3d_action("setSelection", Some(json!({ "id": stock.id })))),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        };
        let cursor = envelope.fixture.resolved_up_to.unwrap_or(envelope.fixture.steps.len());
        let step_items: Vec<UiTreeItemNode> = envelope
            .fixture
            .steps
            .iter()
            .enumerate()
            .map(|(index, step)| UiTreeItemNode {
                id: step.id.clone(),
                label: step.label.clone(),
                description: if index >= cursor { Some("pending".into()) } else { None },
                icon_id: Some(process3d_measure_icon(&step.measure).into()),
                selected: Some(envelope.runtime.selected_id.as_deref() == Some(step.id.as_str())),
                default_open: None,
                action: Some(process3d_action("setSelection", Some(json!({ "id": step.id })))),
                hover_action: Some(process3d_action("setHover", Some(json!({ "id": step.id })))),
                unhover_action: Some(process3d_action("setHover", None)),
                actions: Some(vec![
                    UiTreeItemAction {
                        icon_id: if step.enabled { "eye".into() } else { "eye-off".into() },
                        label: Some(labels.enabled.into()),
                        action: process3d_action("setStepEnabled", Some(json!({ "id": step.id, "enabled": !step.enabled }))),
                        reveal_on_hover: Some(true),
                    },
                    UiTreeItemAction {
                        icon_id: "trash".into(),
                        label: Some(labels.remove.into()),
                        action: process3d_action("removeStep", Some(json!({ "id": step.id }))),
                        reveal_on_hover: Some(true),
                    },
                ]),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: Some(!step.enabled),
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            sections: vec![
                UiTreeSectionNode { id: "process3d-play-document.stock".into(), label: Some(labels.stock.into()), default_open: Some(true), items: vec![stock_item] },
                UiTreeSectionNode { id: "process3d-play-document.steps".into(), label: Some(labels.steps.into()), default_open: Some(true), items: step_items },
            ],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }

    fn build_catalogue_tree(labels: &Process3dLabels) -> UiNode {
        let step_items = vec![
            tree_item_with_action("process3d-catalogue.cut", labels.cut, Some("scissors"), process3d_action("addStep", Some(json!({ "measure": "cut" })))),
            tree_item_with_action("process3d-catalogue.drill", labels.drill, Some("circle-dot"), process3d_action("addStep", Some(json!({ "measure": "drill" })))),
            tree_item_with_action("process3d-catalogue.attach", labels.attach, Some("plus"), process3d_action("addStep", Some(json!({ "measure": "attach" })))),
        ];
        let stock_items = vec![
            tree_item_with_action("process3d-catalogue.stock-box", "Box", Some("box"), process3d_action("setStock", Some(json!({ "kind": "box" })))),
            tree_item_with_action("process3d-catalogue.stock-cylinder", "Cylinder", Some("cylinder"), process3d_action("setStock", Some(json!({ "kind": "cylinder" })))),
            tree_item_with_action("process3d-catalogue.stock-sphere", "Sphere", Some("circle"), process3d_action("setStock", Some(json!({ "kind": "sphere" })))),
        ];
        UiNode::Tree(UiTreeNode {
            sections: vec![
                UiTreeSectionNode { id: "process3d-play-catalogue.steps".into(), label: Some(labels.steps.into()), default_open: Some(true), items: step_items },
                UiTreeSectionNode { id: "process3d-play-catalogue.stock".into(), label: Some(labels.stock.into()), default_open: Some(false), items: stock_items },
            ],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }

    fn build_stock_inspector(stock: &Stock, fixture: &process_3d::Process3dDocument, labels: &Process3dLabels) -> UiNode {
        let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &stock.label, &stock.id, "label")];
        match &stock.solid {
            SolidSpec::Box { width, depth, height } => {
                fields.push(number_field("process3d-inspector.width", "Width", *width, &stock.id, "width"));
                fields.push(number_field("process3d-inspector.depth", "Depth", *depth, &stock.id, "depth"));
                fields.push(number_field("process3d-inspector.height", "Height", *height, &stock.id, "height"));
            }
            SolidSpec::Cylinder { radius, height } => {
                fields.push(number_field("process3d-inspector.radius", "Radius", *radius, &stock.id, "radius"));
                fields.push(number_field("process3d-inspector.height", "Height", *height, &stock.id, "height"));
            }
            SolidSpec::Sphere { radius } => {
                fields.push(number_field("process3d-inspector.radius", "Radius", *radius, &stock.id, "radius"));
            }
        }
        fields.push(number_field("process3d-inspector.posX", "X", stock.pose.position[0], &stock.id, "posX"));
        fields.push(number_field("process3d-inspector.posY", "Y", stock.pose.position[1], &stock.id, "posY"));
        fields.push(number_field("process3d-inspector.posZ", "Z", stock.pose.position[2], &stock.id, "posZ"));
        fields.push(number_field("process3d-inspector.angle", "Angle", stock.pose.angle, &stock.id, "angle"));
        if let Some(volume) = processed_volume(fixture) {
            fields.push(ui_inspector_readonly_field("process3d-inspector.volume", labels.volume, format!("{volume:.4} m³")));
        }
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "process3d-inspector.stock".into(), label: labels.stock.into(), default_open: Some(true), fields }])
    }

    fn build_step_inspector(step: &ProcessStep, labels: &Process3dLabels) -> UiNode {
        let target = format!("step:{}", step.id);
        let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &step.label, &target, "label")];
        let pose = match &step.measure {
            ProcessMeasure::Cut { tool, pose } => {
                if let SolidSpec::Box { width, depth, height } = tool {
                    fields.push(number_field("process3d-inspector.toolWidth", "Width", *width, &target, "toolWidth"));
                    fields.push(number_field("process3d-inspector.toolDepth", "Depth", *depth, &target, "toolDepth"));
                    fields.push(number_field("process3d-inspector.toolHeight", "Height", *height, &target, "toolHeight"));
                }
                pose
            }
            ProcessMeasure::Drill { radius, depth, pose } => {
                fields.push(number_field("process3d-inspector.radius", "Radius", *radius, &target, "radius"));
                fields.push(number_field("process3d-inspector.depth", "Depth", *depth, &target, "depth"));
                pose
            }
            ProcessMeasure::Attach { component, pose } => {
                if let SolidSpec::Cylinder { radius, height } = component {
                    fields.push(number_field("process3d-inspector.radius", "Radius", *radius, &target, "radius"));
                    fields.push(number_field("process3d-inspector.height", "Height", *height, &target, "height"));
                }
                pose
            }
        };
        fields.push(number_field("process3d-inspector.posX", "X", pose.position[0], &target, "posX"));
        fields.push(number_field("process3d-inspector.posY", "Y", pose.position[1], &target, "posY"));
        fields.push(number_field("process3d-inspector.posZ", "Z", pose.position[2], &target, "posZ"));
        fields.push(number_field("process3d-inspector.angle", "Angle", pose.angle, &target, "angle"));
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
            id: "process3d-inspector.step".into(),
            label: process3d_measure_label(&step.measure, labels).into(),
            default_open: Some(true),
            fields,
        }])
    }

    fn build_inspector_tree(envelope: &Process3dEnvelope, labels: &Process3dLabels) -> UiNode {
        let Some(selected_id) = envelope.runtime.selected_id.as_deref() else {
            return ui_text(labels.no_selection);
        };
        if selected_id == envelope.fixture.stock.id {
            return build_stock_inspector(&envelope.fixture.stock, &envelope.fixture, labels);
        }
        if let Some(step) = envelope.fixture.steps.iter().find(|step| step.id == selected_id) {
            return build_step_inspector(step, labels);
        }
        ui_text(labels.no_selection)
    }
    //#endregion 🔖Panels

    //#region 🔖Engagement
    fn process3d_engagement(envelope: &Process3dEnvelope) -> WindowEngagement {
        let len = envelope.fixture.steps.len();
        let cursor = envelope.fixture.resolved_up_to.unwrap_or(len);
        let volume = processed_volume(&envelope.fixture).unwrap_or(0.0);
        WindowEngagement {
            session_active: Some(envelope.runtime.active_tool != "select"),
            options: Some(vec![
                WindowEngagementOption {
                    id: PROCESS3D_ENGAGEMENT_TOOL_SELECT.into(),
                    label: Some("Select".into()),
                    icon_id: Some("cursor".into()),
                    pressed: Some(envelope.runtime.active_tool == "select"),
                    disabled: None,
                    action: Some(process3d_action("engagementPossibleSelect", Some(json!({ "possibleId": PROCESS3D_ENGAGEMENT_TOOL_SELECT })))),
                },
                WindowEngagementOption {
                    id: PROCESS3D_ENGAGEMENT_TOOL_CUT.into(),
                    label: Some("Cut".into()),
                    icon_id: Some("scissors".into()),
                    pressed: Some(envelope.runtime.active_tool == "cut"),
                    disabled: None,
                    action: Some(process3d_action("engagementPossibleSelect", Some(json!({ "possibleId": PROCESS3D_ENGAGEMENT_TOOL_CUT })))),
                },
                WindowEngagementOption {
                    id: PROCESS3D_ENGAGEMENT_TOOL_DRILL.into(),
                    label: Some("Drill".into()),
                    icon_id: Some("circle-dot".into()),
                    pressed: Some(envelope.runtime.active_tool == "drill"),
                    disabled: None,
                    action: Some(process3d_action("engagementPossibleSelect", Some(json!({ "possibleId": PROCESS3D_ENGAGEMENT_TOOL_DRILL })))),
                },
                WindowEngagementOption {
                    id: PROCESS3D_ENGAGEMENT_TOOL_ATTACH.into(),
                    label: Some("Attach".into()),
                    icon_id: Some("plus".into()),
                    pressed: Some(envelope.runtime.active_tool == "attach"),
                    disabled: None,
                    action: Some(process3d_action("engagementPossibleSelect", Some(json!({ "possibleId": PROCESS3D_ENGAGEMENT_TOOL_ATTACH })))),
                },
            ]),
            input: Some(WindowEngagementInput {
                id: Some("process3d-engagement".into()),
                value: Some(envelope.runtime.engagement_input.clone()),
                placeholder: Some("cut, drill, attach, back, forward, all".into()),
                disabled: None,
                on_change: Some(process3d_action("engagementInput", None)),
                on_submit: Some(process3d_action("engagementSubmit", None)),
                on_repeat_last: None,
                on_abort: Some(process3d_action("engagementAbort", None)),
            }),
            control: Some(WindowEngagementControl::Stepper {
                id: Some("process3d-cursor".into()),
                label: Some("Step".into()),
                value: cursor as f64,
                min: Some(0.0),
                max: Some(len as f64),
                step: Some(1.0),
                unit: None,
                disabled: None,
                on_change: Some(process3d_action("setCursor", None)),
                on_commit: None,
            }),
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "process3d-status".into(), text: format!("{cursor}/{len} steps · {volume:.4} m³") }]),
            possible_engagements: None,
        }
    }
    //#endregion 🔖Engagement

    //#region 🔖Process3dPlayApp
    #[derive(Default)]
    pub struct Process3dPlayApp;

    impl PluginApp for Process3dPlayApp {
        fn app_id(&self) -> &str {
            PROCESS_3D_PLAY_APP_ID
        }

        fn initial_document_json(&self) -> String {
            serde_json::to_string(&default_envelope()).expect("process3d envelope json")
        }

        fn handle_action_patch_ops(&mut self, action: &str, args: Option<&Value>, document_json: &str, _view_state: &ViewState) -> Vec<String> {
            let mut envelope = parse_envelope(document_json);
            if PROCESS3D_UNDOABLE_ACTIONS.contains(&action) {
                envelope.runtime.undo_stack.push(envelope.fixture.clone());
                if envelope.runtime.undo_stack.len() > PROCESS3D_UNDO_STACK_MAX {
                    envelope.runtime.undo_stack.remove(0);
                }
                envelope.runtime.redo_stack.clear();
            }
            match action {
                "setDocument" => {
                    if let Some(document) = args.and_then(|value| value.get("document")) {
                        if let Ok(parsed) = serde_json::from_value::<Process3dEnvelope>(document.clone()) {
                            return vec![set_document_op(&parsed)];
                        }
                    }
                }
                "setActiveExample" => {
                    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                    envelope = match example_id {
                        PROCESS3D_EXAMPLE_PLATE | "plate" => plate_envelope(),
                        "empty" => empty_envelope(),
                        _ => default_envelope(),
                    };
                    return vec![finalize_document_op(&mut envelope)];
                }
                "setSelection" => {
                    envelope.runtime.selected_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                    return vec![set_document_op(&envelope)];
                }
                "setHover" => {
                    envelope.runtime.hovered_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                    return vec![set_document_op(&envelope)];
                }
                "setCamera" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                            envelope.runtime.camera = parsed;
                            return vec![set_document_op(&envelope)];
                        }
                    }
                }
                "addStep" => {
                    let kind = args.and_then(|value| value.get("measure")).and_then(|value| value.as_str()).unwrap_or("cut");
                    let position = args.and_then(|value| value.get("position")).and_then(value_as_vec3);
                    let step = ProcessStep { id: next_step_id(), label: label_for_kind(kind, process3d_labels(_view_state)), enabled: true, measure: measure_for_kind(kind, position) };
                    envelope.runtime.selected_id = Some(step.id.clone());
                    insert_step_at_cursor(&mut envelope.fixture, step);
                    return vec![finalize_document_op(&mut envelope)];
                }
                "removeStep" => {
                    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                        if remove_step(&mut envelope.fixture, id) {
                            if envelope.runtime.selected_id.as_deref() == Some(id) {
                                envelope.runtime.selected_id = None;
                            }
                            return vec![finalize_document_op(&mut envelope)];
                        }
                    }
                }
                "removeSelectedStep" => {
                    if let Some(id) = envelope.runtime.selected_id.clone() {
                        if remove_step(&mut envelope.fixture, &id) {
                            envelope.runtime.selected_id = None;
                            return vec![finalize_document_op(&mut envelope)];
                        }
                    }
                }
                "moveStep" => {
                    if let (Some(id), Some(index)) =
                        (args.and_then(|value| value.get("id")).and_then(|value| value.as_str()), args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()))
                    {
                        if let Some(from) = envelope.fixture.steps.iter().position(|step| step.id == id) {
                            let step = envelope.fixture.steps.remove(from);
                            let at = (index as usize).min(envelope.fixture.steps.len());
                            envelope.fixture.steps.insert(at, step);
                            return vec![finalize_document_op(&mut envelope)];
                        }
                    }
                }
                "updateStep" => {
                    if let Some(step_value) = args.and_then(|value| value.get("step")) {
                        if let Ok(step) = serde_json::from_value::<ProcessStep>(step_value.clone()) {
                            if let Some(existing) = envelope.fixture.steps.iter_mut().find(|entry| entry.id == step.id) {
                                *existing = step;
                                return vec![finalize_document_op(&mut envelope)];
                            }
                        }
                    }
                }
                "setStepEnabled" => {
                    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                        let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(true);
                        if let Some(step) = envelope.fixture.steps.iter_mut().find(|step| step.id == id) {
                            step.enabled = enabled;
                            return vec![finalize_document_op(&mut envelope)];
                        }
                    }
                }
                "setStock" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("box");
                    let solid = match kind {
                        "cylinder" => SolidSpec::Cylinder { radius: 0.3, height: 1.0 },
                        "sphere" => SolidSpec::Sphere { radius: 0.5 },
                        _ => SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 },
                    };
                    envelope.fixture.stock = Stock { id: envelope.fixture.stock.id.clone(), label: process3d_labels(_view_state).stock.into(), solid, pose: Pose::default() };
                    envelope.fixture.steps.clear();
                    envelope.fixture.resolved_up_to = None;
                    envelope.runtime.selected_id = None;
                    return vec![finalize_document_op(&mut envelope)];
                }
                "patchInspector" => {
                    let target = args.and_then(|value| value.get("target")).and_then(|value| value.as_str()).unwrap_or("");
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                    let value = args.and_then(|value| value.get("value"));
                    if apply_process3d_inspector_patch(&mut envelope.fixture, target, field, value) {
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "setCursor" => {
                    let resolved = match args.and_then(|value| value.get("value")) {
                        None | Some(Value::Null) => None,
                        Some(value) => value.as_u64().map(|n| n as usize),
                    };
                    envelope.fixture.resolved_up_to = resolved.map(|n| n.min(envelope.fixture.steps.len()));
                    return vec![finalize_document_op(&mut envelope)];
                }
                "stepCursor" | "stepCursorBack" | "stepCursorForward" => {
                    let delta = match action {
                        "stepCursorBack" => -1,
                        "stepCursorForward" => 1,
                        _ => args.and_then(|value| value.get("delta")).and_then(|value| value.as_i64()).unwrap_or(0),
                    };
                    let len = envelope.fixture.steps.len();
                    let current = envelope.fixture.resolved_up_to.unwrap_or(len) as i64;
                    envelope.fixture.resolved_up_to = Some((current + delta).clamp(0, len as i64) as usize);
                    return vec![finalize_document_op(&mut envelope)];
                }
                "engagementPossibleSelect" => {
                    let possible_id = args.and_then(|value| value.get("possibleId")).and_then(|value| value.as_str()).unwrap_or("");
                    envelope.runtime.active_tool = match possible_id {
                        PROCESS3D_ENGAGEMENT_TOOL_CUT => "cut",
                        PROCESS3D_ENGAGEMENT_TOOL_DRILL => "drill",
                        PROCESS3D_ENGAGEMENT_TOOL_ATTACH => "attach",
                        _ => "select",
                    }
                    .into();
                    return vec![set_document_op(&envelope)];
                }
                "engagementInput" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        envelope.runtime.engagement_input = value.into();
                        return vec![set_document_op(&envelope)];
                    }
                }
                "engagementAbort" => {
                    envelope.runtime.engagement_input = String::new();
                    envelope.runtime.active_tool = "select".into();
                    return vec![set_document_op(&envelope)];
                }
                "engagementSubmit" => {
                    let command = envelope.runtime.engagement_input.trim().to_lowercase();
                    envelope.runtime.engagement_input = String::new();
                    let len = envelope.fixture.steps.len();
                    let current = envelope.fixture.resolved_up_to.unwrap_or(len);
                    match command.split_whitespace().next() {
                        Some("cut") => envelope.runtime.active_tool = "cut".into(),
                        Some("drill") => envelope.runtime.active_tool = "drill".into(),
                        Some("attach") => envelope.runtime.active_tool = "attach".into(),
                        Some("back") => envelope.fixture.resolved_up_to = Some(current.saturating_sub(1)),
                        Some("forward") => envelope.fixture.resolved_up_to = Some((current + 1).min(len)),
                        Some("all") => envelope.fixture.resolved_up_to = None,
                        _ => {}
                    }
                    return vec![finalize_document_op(&mut envelope)];
                }
                "worldPointerDown" => {
                    let tool = envelope.runtime.active_tool.clone();
                    if tool == "select" {
                        return Vec::new();
                    }
                    if let Some(point) = args.and_then(|value| value.get("point")).and_then(value_as_vec3) {
                        let step = ProcessStep { id: next_step_id(), label: label_for_kind(&tool, process3d_labels(_view_state)), enabled: true, measure: measure_for_kind(&tool, Some(point)) };
                        envelope.runtime.selected_id = Some(step.id.clone());
                        insert_step_at_cursor(&mut envelope.fixture, step);
                        envelope.runtime.active_tool = "select".into();
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "undo" => {
                    if let Some(previous) = envelope.runtime.undo_stack.pop() {
                        envelope.runtime.redo_stack.push(envelope.fixture.clone());
                        envelope.fixture = previous;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "redo" => {
                    if let Some(next) = envelope.runtime.redo_stack.pop() {
                        envelope.runtime.undo_stack.push(envelope.fixture.clone());
                        envelope.fixture = next;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                _ => {}
            }
            Vec::new()
        }

        fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
            let envelope = parse_envelope(document_json);
            let labels = process3d_labels(view_state);
            match body_key {
                PROCESS_3D_PLAY_BODY_MAIN => {
                    let (meshes_json, instances_json) = preview_payload_cached(&envelope.runtime, &envelope.fixture);
                    build_world_3d_scene(
                        PROCESS_3D_PLAY_SURFACE_MAIN,
                        PROCESS_3D_PLAY_APP_ID,
                        world3d_scene(
                            world3d_camera_json(envelope.runtime.camera.position, envelope.runtime.camera.target, envelope.runtime.camera.fov),
                            meshes_json,
                            instances_json,
                            world3d_selection_json(&envelope.runtime.selection_method, &selected_ids(&envelope), envelope.runtime.hovered_id.as_deref()),
                        ),
                    )
                }
                PROCESS_3D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
                PROCESS_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
                PROCESS_3D_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn tools(&self, _document_json: &str, view_state: &ViewState) -> Vec<ToolNode> {
            let labels = process3d_labels(view_state);
            vec![
                tool_button("process3d.tool.stepBack", "chevron-left", "Step Back", process3d_action("stepCursorBack", None)),
                tool_button("process3d.tool.stepForward", "chevron-right", "Step Forward", process3d_action("stepCursorForward", None)),
                tool_button("process3d.tool.applyAll", "fast-forward", "Apply All", process3d_action("setCursor", Some(json!({ "value": null })))),
                tool_separator("process3d.tool.sep1"),
                tool_button("process3d.tool.addCut", "scissors", labels.cut, process3d_action("addStep", Some(json!({ "measure": "cut" })))),
                tool_button("process3d.tool.addDrill", "circle-dot", labels.drill, process3d_action("addStep", Some(json!({ "measure": "drill" })))),
                tool_button("process3d.tool.addAttach", "plus", labels.attach, process3d_action("addStep", Some(json!({ "measure": "attach" })))),
            ]
        }

        fn window_engagements(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
            let envelope = parse_envelope(document_json);
            HashMap::from([(PROCESS_3D_PLAY_WINDOW_MAIN.into(), process3d_engagement(&envelope))])
        }
    }
    //#endregion 🔖Process3dPlayApp

    //#region 🔖Manifest
    pub fn create_process3d_app() -> App {
        App::from_builder(
            App::builder(PROCESS_3D_PLAY_APP_ID, "Process 3D")
                .document(["semio", "process", "3d"])
                .icon_id("hammer")
                .mode("edit", "Edit")
                .default_mode_id("edit")
                .window_kind_with_engagement(PROCESS_3D_PLAY_WINDOW_MAIN, "Workpiece", PROCESS_3D_PLAY_BODY_MAIN, SurfaceKind::World3d, process3d_engagement(&default_envelope()))
                .default_layout(create_default_layout(&[PROCESS_3D_PLAY_WINDOW_MAIN.into()], "row", None, Some(&["Workpiece".into()])))
                .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PROCESS_3D_PLAY_BODY_DOCUMENT)
                .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PROCESS_3D_PLAY_BODY_CATALOGUE)
                .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PROCESS_3D_PLAY_BODY_INSPECTION)
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo")
                .keybinding("bracketleft", "stepCursorBack")
                .keybinding("bracketright", "stepCursorForward")
                .keybinding("escape", "engagementAbort")
                .keybinding("delete", "removeSelectedStep")
                .keybinding("backspace", "removeSelectedStep"),
        )
        .example(PROCESS3D_EXAMPLE_TIMBER, "Timber Beam Joinery", TIMBER_EXAMPLE_JSON)
        .example(PROCESS3D_EXAMPLE_PLATE, "Drilled Plate", PLATE_EXAMPLE_JSON)
        .program("process3d", "Process 3D", "brep")
    }

    fn process3d_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
        let envelope: Process3dEnvelope = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
        processed_mesh(&envelope.fixture).ok_or_else(|| "process3d: kernel replay failed".to_string())
    }

    fn process3d_document_from_mesh(_mesh: &MeshData) -> Result<Value, String> {
        Err("process3d: mesh import not supported".into())
    }

    pub fn register_process3d_exports() {
        semio_framework_os::register_mesh_export_handlers("3d.process", "process", process3d_mesh_from_document);
        semio_framework_os::register_mesh_dwg_import_handler("3d.process", process3d_document_from_mesh);
    }
    //#endregion 🔖Manifest

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_envelope_parses_timber_example() {
            let envelope = default_envelope();
            assert_eq!(envelope.fixture.steps.len(), 4);
            assert!(envelope.fixture.resolved_up_to.is_none());
        }

        #[test]
        fn plate_envelope_parses_and_opens_mid_timeline() {
            let envelope = plate_envelope();
            assert_eq!(envelope.fixture.steps.len(), 3);
            assert_eq!(envelope.fixture.resolved_up_to, Some(2));
        }

        #[test]
        fn drill_reduces_volume_below_stock() {
            let mut fixture = process_3d::Process3dDocument::default();
            fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
            let stock_volume = processed_volume(&fixture).expect("stock volume");
            fixture.steps.push(ProcessStep {
                id: "drill-1".into(),
                label: "Drill".into(),
                enabled: true,
                measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose { position: [0.0, 0.0, 0.5], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            });
            let drilled_volume = processed_volume(&fixture).expect("drilled volume");
            assert!(drilled_volume < stock_volume, "drilled volume {drilled_volume} should be less than stock volume {stock_volume}");
        }

        #[test]
        fn attach_increases_volume_above_stock() {
            let mut fixture = process_3d::Process3dDocument::default();
            fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
            let stock_volume = processed_volume(&fixture).expect("stock volume");
            fixture.steps.push(ProcessStep {
                id: "attach-1".into(),
                label: "Attach".into(),
                enabled: true,
                measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.3 }, pose: Pose { position: [1.0, 0.0, 0.5], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            });
            let attached_volume = processed_volume(&fixture).expect("attached volume");
            assert!(attached_volume > stock_volume, "attached volume {attached_volume} should exceed stock volume {stock_volume}");
        }

        #[test]
        fn disabled_step_is_skipped_on_replay() {
            let mut fixture = process_3d::Process3dDocument::default();
            fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
            let stock_volume = processed_volume(&fixture).expect("stock volume");
            fixture.steps.push(ProcessStep {
                id: "drill-1".into(),
                label: "Drill".into(),
                enabled: false,
                measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose::default() },
            });
            let volume_with_disabled_step = processed_volume(&fixture).expect("volume");
            assert!((volume_with_disabled_step - stock_volume).abs() < 1e-6);
        }

        #[test]
        fn cursor_zero_yields_stock_volume() {
            let mut fixture = process_3d::Process3dDocument::default();
            fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
            let stock_volume = processed_volume(&fixture).expect("stock volume");
            fixture.steps.push(ProcessStep {
                id: "drill-1".into(),
                label: "Drill".into(),
                enabled: true,
                measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose::default() },
            });
            fixture.resolved_up_to = Some(0);
            let volume_at_cursor_zero = processed_volume(&fixture).expect("volume");
            assert!((volume_at_cursor_zero - stock_volume).abs() < 1e-6);
        }

        #[test]
        fn labels_resolve_native_by_default_and_in_german() {
            let mut view_state = ViewState::default();
            assert_eq!(process3d_labels(&view_state).stock, "Stock");
            view_state.locale = Some("de".into());
            assert_eq!(process3d_labels(&view_state).stock, "Rohteil");
        }

        #[test]
        fn add_step_action_inserts_and_selects() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops("addStep", Some(&json!({ "measure": "drill" })), &document_json, &view_state);
            assert_eq!(ops.len(), 1);
            let patched: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let document = &patched["document"];
            let steps = document["fixture"]["steps"].as_array().expect("steps array");
            assert_eq!(steps.len(), 5);
            assert_eq!(document["runtime"]["selectedId"], steps.last().expect("last step")["id"]);
        }

        #[test]
        fn undo_after_add_step_restores_previous_step_count() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops("addStep", Some(&json!({ "measure": "cut" })), &document_json, &view_state);
            let after_add: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let after_add_json = after_add["document"].to_string();
            let ops = app.handle_action_patch_ops("undo", None, &after_add_json, &view_state);
            let after_undo: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let steps = after_undo["document"]["fixture"]["steps"].as_array().expect("steps array");
            assert_eq!(steps.len(), 4);
        }

        #[test]
        fn render_world_scene_contains_processed_mesh() {
            let app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let node = app.render(PROCESS_3D_PLAY_BODY_MAIN, &document_json, &view_state);
            let node_json = serde_json::to_string(&node).expect("scene json");
            assert!(node_json.contains("processed"), "expected the processed mesh id in scene json: {node_json}");
        }

        #[test]
        fn kernel_replay_memoizes_prefixes_across_cursor_scrub() {
            let mut fixture = process_3d::Process3dDocument::default();
            fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
            fixture.steps.push(ProcessStep {
                id: "drill-1".into(),
                label: "Drill".into(),
                enabled: true,
                measure: ProcessMeasure::Drill { radius: 0.1, depth: 1.0, pose: Pose::default() },
            });
            fixture.resolved_up_to = Some(1);
            processed_volume(&fixture).expect("volume at cursor 1");
            let session = process_kernel_session().lock().expect("kernel session lock");
            assert!(session.memo.len() >= 2, "expected stock + drilled prefixes memoized, got {}", session.memo.len());
        }
    }
}

use semio_framework_plugin::PluginBundle;

//#region 🔖Bundle
fn bundle() -> PluginBundle {
    app_3d::register_process3d_exports();
    PluginBundle::new("process", "Process", "0.1.0")
        .register_app(app_3d::create_process3d_app(), || Box::new(app_3d::Process3dPlayApp::default()))
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Bundle
