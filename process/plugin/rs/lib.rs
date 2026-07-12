//! 🪚 Process plugin — subtractive/additive processing simulation in one hot-swappable WASM component.

pub mod app_3d {
    //! 🪚 Process 3D plugin — subtractive/additive processing simulation bundled as a hot-swappable WASM component.

    use kernel_3d_brepkit::BrepkitKernel;
    use kernel_3d_engine::GeometryHandle;
    use process_3d::{Pose, ProcessMeasure, ProcessStep, SolidSpec, Stock, StepOrigin};
    use semio_framework_plugin::{
        apply_world3d_sun_action, build_world_3d_scene, create_default_layout, mesh_from_indexed_with_face_groups, mesh_from_kind, tool_button,
        tool_toggle, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, world3d_camera_json, world3d_scene,
        world3d_sun_measures, world3d_selection_json, ActionDescriptor, App, MeshData, PanelGroup, PluginApp, SurfaceKind, ToolCategory, ToolNode,
        UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
        ViewState, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementOption, WindowMeasure, WorldSunConfig,
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
        &["addStep", "removeStep", "removeSelectedStep", "updateStep", "moveStep", "setStepEnabled", "patchInspector", "setStock", "worldFaceDragEnd"];
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
        select: &'static str,
        cut: &'static str,
        drill: &'static str,
        attach: &'static str,
        push_cut: &'static str,
        pull_attach: &'static str,
        enabled: &'static str,
        volume: &'static str,
        label_field: &'static str,
        no_selection: &'static str,
        remove: &'static str,
        provenance: &'static str,
        validation_warning: &'static str,
    }

    const PROCESS3D_LABELS_NATIVE_EN: Process3dLabels = Process3dLabels {
        stock: "Stock",
        steps: "Steps",
        select: "Select",
        cut: "Cut",
        drill: "Drill",
        attach: "Attach",
        push_cut: "Push Cut",
        pull_attach: "Pull Attach",
        enabled: "Enabled",
        volume: "Volume",
        label_field: "Label",
        no_selection: "No selection",
        remove: "Remove",
        provenance: "Made By",
        validation_warning: "Warning",
    };

    const PROCESS3D_LABELS_NATIVE_DE: Process3dLabels = Process3dLabels {
        stock: "Rohteil",
        steps: "Schritte",
        select: "Auswählen",
        cut: "Schnitt",
        drill: "Bohrung",
        attach: "Anbau",
        push_cut: "Schnitt (Drücken)",
        pull_attach: "Anbau (Ziehen)",
        enabled: "Aktiviert",
        volume: "Volumen",
        label_field: "Bezeichnung",
        no_selection: "Keine Auswahl",
        remove: "Entfernen",
        provenance: "Erstellt von",
        validation_warning: "Warnung",
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
        /// 🖱️ Id of the brep face currently picked in the viewport (drag-to-cut/attach target).
        #[serde(default)]
        selected_face_id: Option<u32>,
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
        #[serde(default)]
        sun: WorldSunConfig,
    }

    impl Default for Process3dRuntime {
        fn default() -> Self {
            Self {
                selected_id: None,
                hovered_id: None,
                selected_face_id: None,
                selection_method: default_selection_method(),
                active_tool: default_active_tool(),
                engagement_input: String::new(),
                camera: Process3dCamera::default(),
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),
                preview_cache: None,
                sun: WorldSunConfig::default(),
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

    /// 🖱️ Extends the base object-selection JSON with face-picking/drag fields: `targets.face` lets the
    /// renderer hit-test individual triangles; `engagementSessionActive` gates the ground-click placement
    /// path used by the cut/drill/attach tools; `faceDragActive` gates the push/pull drag gesture, only
    /// while the select tool is active (so a click-to-place tool doesn't also start a face drag).
    fn process3d_selection_json(envelope: &Process3dEnvelope) -> String {
        let mut value: Value = serde_json::from_str(&world3d_selection_json(&envelope.runtime.selection_method, &selected_ids(envelope), envelope.runtime.hovered_id.as_deref()))
            .unwrap_or_else(|_| json!({}));
        if let Some(object) = value.as_object_mut() {
            object.insert("engagementSessionActive".into(), json!(envelope.runtime.active_tool != "select"));
            object.insert("selectionMode".into(), json!("face"));
            object.insert("targets".into(), json!({ "mesh": true, "face": true, "vertex": false, "edge": false }));
            object.insert("componentIds".into(), json!(envelope.runtime.selected_face_id.map(|id| vec![id]).unwrap_or_default()));
            object.insert("faceDragActive".into(), json!(envelope.runtime.active_tool == "select"));
        }
        value.to_string()
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

    /// ✂️➕ Inserts a new step at the cursor, advances the cursor past it, and selects it.
    fn insert_step_at_cursor(fixture: &mut process_3d::Process3dDocument, step: ProcessStep) {
        let cursor = fixture.resolved_up_to.unwrap_or(fixture.steps.len()).min(fixture.steps.len());
        fixture.steps.insert(cursor, step);
        fixture.resolved_up_to = Some((cursor + 1).min(fixture.steps.len()));
    }

    //#region 🔖Modules
    /// 🔧 A named numeric machine parameter (e.g. blade diameter) — sizes the tool geometry a
    /// modification kind builds and gates which modifications are legal against the current stock.
    struct Capability {
        id: &'static str,
        label: &'static str,
        value: f64,
    }

    /// 🪚 Which kernel-level geometry operation a modification kind produces — `ProcessMeasure`'s three
    /// existing shapes are the fixed, small vocabulary every machine ultimately maps onto.
    #[derive(Clone, Copy, PartialEq)]
    enum MeasureKind {
        Cut,
        Drill,
        Attach,
    }

    /// 📏 A stock dimension a validation rule checks against a capability value.
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum TargetQuantity {
        StockWidth,
        StockDepth,
        StockHeight,
    }

    /// ✅ "quantity must be at least/at most the named capability's value (± margin)" — a modification
    /// kind's rules are ANDed together, e.g. crosscut needs stock width AND height above the blade diameter.
    enum ValidationRule {
        MinAgainstCapability { quantity: TargetQuantity, capability: &'static str, margin: f64 },
        MaxAgainstCapability { quantity: TargetQuantity, capability: &'static str, margin: f64 },
    }

    /// 📐 The stock dimensions a validation rule is checked against.
    struct ValidationContext {
        stock_width: f64,
        stock_depth: f64,
        stock_height: f64,
    }

    /// 🚫 One failed validation rule, with the actual vs. required value for a human-readable reason.
    #[derive(Debug)]
    struct ValidationFailure {
        quantity: TargetQuantity,
        actual: f64,
        required: f64,
        is_min: bool,
    }

    /// 🪚 One thing a machine can do (e.g. "crosscut"), producing `measure_kind` geometry sized from
    /// the machine's capabilities, gated by `rules`.
    struct ModificationKind {
        id: &'static str,
        label: &'static str,
        icon_id: &'static str,
        measure_kind: MeasureKind,
        rules: &'static [ValidationRule],
    }

    /// 🛠️ A tool (e.g. a circular saw) with capabilities and the modification kinds it offers.
    struct Machine {
        id: &'static str,
        label: &'static str,
        icon_id: &'static str,
        capabilities: &'static [Capability],
        modification_kinds: &'static [ModificationKind],
    }

    /// 📦 A domain-specific bundle of machines (e.g. "wood", "concrete"); `geometry` is the generic default.
    struct Module {
        id: &'static str,
        label: &'static str,
        machines: &'static [Machine],
    }

    const GEOMETRY_SAW: Machine = Machine {
        id: "saw",
        label: "Generic Saw",
        icon_id: "scissors",
        capabilities: &[],
        modification_kinds: &[ModificationKind { id: "cut", label: "Cut", icon_id: "scissors", measure_kind: MeasureKind::Cut, rules: &[] }],
    };
    const GEOMETRY_DRILL: Machine = Machine {
        id: "drill",
        label: "Generic Drill",
        icon_id: "circle-dot",
        capabilities: &[],
        modification_kinds: &[ModificationKind { id: "drill", label: "Drill", icon_id: "circle-dot", measure_kind: MeasureKind::Drill, rules: &[] }],
    };
    const GEOMETRY_ATTACHER: Machine = Machine {
        id: "attacher",
        label: "Generic Attacher",
        icon_id: "plus",
        capabilities: &[],
        modification_kinds: &[ModificationKind { id: "attach", label: "Attach", icon_id: "plus", measure_kind: MeasureKind::Attach, rules: &[] }],
    };
    const GEOMETRY_MODULE: Module = Module { id: "geometry", label: "Geometry", machines: &[GEOMETRY_SAW, GEOMETRY_DRILL, GEOMETRY_ATTACHER] };

    const CROSSCUT_RULES: &[ValidationRule] = &[
        ValidationRule::MinAgainstCapability { quantity: TargetQuantity::StockWidth, capability: "diameter", margin: 0.0 },
        ValidationRule::MinAgainstCapability { quantity: TargetQuantity::StockHeight, capability: "diameter", margin: 0.0 },
    ];

    const WOOD_CIRCULAR_SAW: Machine = Machine {
        id: "circularSaw",
        label: "Circular Saw",
        icon_id: "scissors",
        capabilities: &[Capability { id: "diameter", label: "Diameter", value: 0.184 }],
        modification_kinds: &[ModificationKind { id: "crosscut", label: "Crosscut", icon_id: "scissors", measure_kind: MeasureKind::Cut, rules: CROSSCUT_RULES }],
    };
    const WOOD_TABLE_SAW: Machine = Machine {
        id: "tableSaw",
        label: "Table Saw",
        icon_id: "scissors",
        capabilities: &[Capability { id: "diameter", label: "Diameter", value: 0.315 }],
        modification_kinds: &[ModificationKind { id: "crosscut", label: "Crosscut", icon_id: "scissors", measure_kind: MeasureKind::Cut, rules: CROSSCUT_RULES }],
    };
    const WOOD_MODULE: Module = Module { id: "wood", label: "Wood", machines: &[WOOD_CIRCULAR_SAW, WOOD_TABLE_SAW] };

    const CONCRETE_DIAMOND_SAW: Machine = Machine {
        id: "diamondSaw",
        label: "Diamond Saw",
        icon_id: "scissors",
        capabilities: &[Capability { id: "diameter", label: "Diameter", value: 0.35 }],
        modification_kinds: &[ModificationKind { id: "crosscut", label: "Crosscut", icon_id: "scissors", measure_kind: MeasureKind::Cut, rules: CROSSCUT_RULES }],
    };
    const CONCRETE_MODULE: Module = Module { id: "concrete", label: "Concrete", machines: &[CONCRETE_DIAMOND_SAW] };

    const ALL_MODULES: &[Module] = &[GEOMETRY_MODULE, WOOD_MODULE, CONCRETE_MODULE];

    /// 🕳️ Kerf/thickness of a machine-built disc cut tool (crosscut etc.) — the tool's extent along its own normal.
    const CROSSCUT_KERF: f64 = 0.05;

    fn find_modification(module_id: &str, machine_id: &str, modification_kind_id: &str) -> Option<(&'static Module, &'static Machine, &'static ModificationKind)> {
        let module = ALL_MODULES.iter().find(|module| module.id == module_id)?;
        let machine = module.machines.iter().find(|machine| machine.id == machine_id)?;
        let kind = machine.modification_kinds.iter().find(|kind| kind.id == modification_kind_id)?;
        Some((module, machine, kind))
    }

    /// 🔎 Finds the geometry module's machine offering a given legacy `measure` kind ("cut"/"drill"/"attach")
    /// — the routing target for the toolbar, click/drag placement, and pre-module `addStep` callers.
    fn geometry_machine_for_measure(measure_kind: MeasureKind) -> (&'static Machine, &'static ModificationKind) {
        for machine in GEOMETRY_MODULE.machines {
            for kind in machine.modification_kinds {
                if kind.measure_kind == measure_kind {
                    return (machine, kind);
                }
            }
        }
        unreachable!("every MeasureKind has a generic geometry machine")
    }

    fn capability_value(machine: &Machine, capability_id: &str) -> Option<f64> {
        machine.capabilities.iter().find(|capability| capability.id == capability_id).map(|capability| capability.value)
    }

    fn validate_modification(machine: &Machine, kind: &ModificationKind, ctx: &ValidationContext) -> Vec<ValidationFailure> {
        kind.rules
            .iter()
            .filter_map(|rule| {
                let (quantity, capability, margin, is_min) = match rule {
                    ValidationRule::MinAgainstCapability { quantity, capability, margin } => (*quantity, *capability, *margin, true),
                    ValidationRule::MaxAgainstCapability { quantity, capability, margin } => (*quantity, *capability, *margin, false),
                };
                let actual = match quantity {
                    TargetQuantity::StockWidth => ctx.stock_width,
                    TargetQuantity::StockDepth => ctx.stock_depth,
                    TargetQuantity::StockHeight => ctx.stock_height,
                };
                let capability_value = capability_value(machine, capability)?;
                let required = if is_min { capability_value + margin } else { capability_value - margin };
                let ok = if is_min { actual >= required } else { actual <= required };
                if ok { None } else { Some(ValidationFailure { quantity, actual, required, is_min }) }
            })
            .collect()
    }

    fn validation_reason(failures: &[ValidationFailure]) -> String {
        failures
            .iter()
            .map(|failure| {
                let axis = match failure.quantity {
                    TargetQuantity::StockWidth => "width",
                    TargetQuantity::StockDepth => "depth",
                    TargetQuantity::StockHeight => "height",
                };
                let comparator = if failure.is_min { "≥" } else { "≤" };
                format!("needs stock {axis} {comparator} {:.0}mm (have {:.0}mm)", failure.required * 1000.0, failure.actual * 1000.0)
            })
            .collect::<Vec<_>>()
            .join("; ")
    }

    fn stock_extent(solid: &SolidSpec) -> [f64; 3] {
        match solid {
            SolidSpec::Box { width, depth, height } => [*width, *depth, *height],
            SolidSpec::Cylinder { radius, height } => [*radius * 2.0, *radius * 2.0, *height],
            SolidSpec::Sphere { radius } => [*radius * 2.0, *radius * 2.0, *radius * 2.0],
        }
    }

    fn validation_context_for_stock(stock: &Stock) -> ValidationContext {
        let [width, depth, height] = stock_extent(&stock.solid);
        ValidationContext { stock_width: width, stock_depth: depth, stock_height: height }
    }

    /// 🪚 Builds the `ProcessMeasure` a machine's modification kind produces — capability-parameterized
    /// where the machine has one (e.g. a saw's `diameter` capability sizes a disc cut tool), otherwise
    /// falling back to the generic geometry-module defaults.
    fn measure_for_modification(machine: &Machine, kind: &ModificationKind, position: Option<[f64; 3]>) -> ProcessMeasure {
        let mut measure = match kind.measure_kind {
            MeasureKind::Cut => match capability_value(machine, "diameter") {
                Some(diameter) => ProcessMeasure::Cut { tool: SolidSpec::Cylinder { radius: diameter / 2.0, height: CROSSCUT_KERF }, pose: Pose::default() },
                None => default_cut_measure(),
            },
            MeasureKind::Drill => default_drill_measure(),
            MeasureKind::Attach => default_attach_measure(),
        };
        if let Some(position) = position {
            let pose = match &mut measure {
                ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose,
            };
            pose.position = position;
        }
        measure
    }
    //#endregion 🔖Modules

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

    /// 🧭 Axis-angle rotation that maps world-up `[0,0,1]` onto an arbitrary unit `normal`, so a box
    /// primitive's local Z axis (its `height` dimension) ends up flush with a picked face's normal.
    fn axis_angle_from_up_to(normal: [f64; 3]) -> ([f64; 3], f64) {
        const UP: [f64; 3] = [0.0, 0.0, 1.0];
        let dot = (UP[0] * normal[0] + UP[1] * normal[1] + UP[2] * normal[2]).clamp(-1.0, 1.0);
        if dot > 1.0 - 1e-9 {
            return ([0.0, 0.0, 1.0], 0.0);
        }
        if dot < -1.0 + 1e-9 {
            return ([1.0, 0.0, 0.0], std::f64::consts::PI);
        }
        let cross = [UP[1] * normal[2] - UP[2] * normal[1], UP[2] * normal[0] - UP[0] * normal[2], UP[0] * normal[1] - UP[1] * normal[0]];
        let len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
        let axis = if len > 1e-9 { [cross[0] / len, cross[1] / len, cross[2] / len] } else { [0.0, 0.0, 1.0] };
        (axis, dot.acos())
    }

    /// 🖱️➡️ Builds a push/pull step from a face-drag gesture: dragging into the solid (negative `distance`
    /// along the face's outward `normal`) removes material (Cut); dragging outward (positive) adds material
    /// (Attach). The tool box's local origin corner lands at `point + normal * distance.min(0.0)` so it spans
    /// exactly the dragged region, flush with the picked face — `box_prim_sync` places a primitive's corner
    /// (not its center) at the local origin, confirmed by `box_primitive_spans_from_local_origin_corner` below.
    fn process3d_step_from_face_drag(normal: [f64; 3], point: [f64; 3], distance: f64, face_extent: Option<[f64; 2]>, labels: &Process3dLabels) -> Option<ProcessStep> {
        if distance.abs() < 1e-6 {
            return None;
        }
        let (width, depth) = face_extent.map(|[w, d]| (w.max(0.02), d.max(0.02))).unwrap_or((0.2, 0.2));
        let height = distance.abs();
        let (axis, angle) = axis_angle_from_up_to(normal);
        let offset = distance.min(0.0);
        let position = [point[0] + normal[0] * offset, point[1] + normal[1] * offset, point[2] + normal[2] * offset];
        let pose = Pose { position, axis, angle };
        let (measure, label, machine_id, modification_kind_id) = if distance < 0.0 {
            (ProcessMeasure::Cut { tool: SolidSpec::Box { width, depth, height }, pose }, labels.push_cut, GEOMETRY_SAW.id, "cut")
        } else {
            (ProcessMeasure::Attach { component: SolidSpec::Box { width, depth, height }, pose }, labels.pull_attach, GEOMETRY_ATTACHER.id, "attach")
        };
        let origin = StepOrigin { module_id: GEOMETRY_MODULE.id.to_string(), machine_id: machine_id.to_string(), modification_kind_id: modification_kind_id.to_string() };
        Some(ProcessStep { id: next_step_id(), label: label.to_string(), enabled: true, origin: Some(origin), measure })
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
        let face_groups: Vec<(u32, u32, u32)> = mesh.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
        Some(mesh_from_indexed_with_face_groups(&mesh.position, &mesh.normal, &mesh.index, &face_groups))
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

    /// 🏭 Builds one catalogue tree item per machine modification kind across all modules, disabling
    /// (non-clickable, with a reason) any kind the current stock doesn't satisfy.
    fn build_catalogue_tree(envelope: &Process3dEnvelope, labels: &Process3dLabels) -> UiNode {
        let ctx = validation_context_for_stock(&envelope.fixture.stock);
        let mut sections: Vec<UiTreeSectionNode> = ALL_MODULES
            .iter()
            .map(|module| {
                let items: Vec<UiTreeItemNode> = module
                    .machines
                    .iter()
                    .flat_map(|machine| {
                        machine.modification_kinds.iter().map(move |kind| {
                            let failures = validate_modification(machine, kind, &ctx);
                            let id = format!("process3d-catalogue.{}.{}.{}", module.id, machine.id, kind.id);
                            let label = format!("{} — {}", machine.label, kind.label);
                            if failures.is_empty() {
                                tree_item_with_action(
                                    id,
                                    label,
                                    Some(kind.icon_id),
                                    process3d_action("addStep", Some(json!({ "moduleId": module.id, "machineId": machine.id, "modificationKindId": kind.id }))),
                                )
                            } else {
                                UiTreeItemNode {
                                    id,
                                    label,
                                    description: Some(validation_reason(&failures)),
                                    icon_id: Some(kind.icon_id.into()),
                                    selected: None,
                                    default_open: None,
                                    action: None,
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
                        })
                    })
                    .collect();
                UiTreeSectionNode { id: format!("process3d-play-catalogue.{}", module.id), label: Some(module.label.into()), default_open: Some(module.id == "geometry"), items }
            })
            .collect();
        let stock_items = vec![
            tree_item_with_action("process3d-catalogue.stock-box", "Box", Some("box"), process3d_action("setStock", Some(json!({ "kind": "box" })))),
            tree_item_with_action("process3d-catalogue.stock-cylinder", "Cylinder", Some("cylinder"), process3d_action("setStock", Some(json!({ "kind": "cylinder" })))),
            tree_item_with_action("process3d-catalogue.stock-sphere", "Sphere", Some("circle"), process3d_action("setStock", Some(json!({ "kind": "sphere" })))),
        ];
        sections.push(UiTreeSectionNode { id: "process3d-play-catalogue.stock".into(), label: Some(labels.stock.into()), default_open: Some(false), items: stock_items });
        UiNode::Tree(UiTreeNode { sections, selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None })
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

    fn build_step_inspector(step: &ProcessStep, stock: &Stock, labels: &Process3dLabels) -> UiNode {
        let target = format!("step:{}", step.id);
        let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &step.label, &target, "label")];
        if let Some(origin) = &step.origin {
            if let Some((module, machine, kind)) = find_modification(&origin.module_id, &origin.machine_id, &origin.modification_kind_id) {
                fields.push(ui_inspector_readonly_field("process3d-inspector.origin", labels.provenance, format!("{} · {} · {}", module.label, machine.label, kind.label)));
                let failures = validate_modification(machine, kind, &validation_context_for_stock(stock));
                if !failures.is_empty() {
                    fields.push(ui_inspector_readonly_field("process3d-inspector.validation", labels.validation_warning, validation_reason(&failures)));
                }
            }
        }
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
            return build_step_inspector(step, &envelope.fixture.stock, labels);
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
                    action: Some(process3d_action("setActiveTool", Some(json!({ "tool": "select" })))),
                },
                WindowEngagementOption {
                    id: PROCESS3D_ENGAGEMENT_TOOL_CUT.into(),
                    label: Some("Cut".into()),
                    icon_id: Some("scissors".into()),
                    pressed: Some(envelope.runtime.active_tool == "cut"),
                    disabled: None,
                    action: Some(process3d_action("setActiveTool", Some(json!({ "tool": "cut" })))),
                },
                WindowEngagementOption {
                    id: PROCESS3D_ENGAGEMENT_TOOL_DRILL.into(),
                    label: Some("Drill".into()),
                    icon_id: Some("circle-dot".into()),
                    pressed: Some(envelope.runtime.active_tool == "drill"),
                    disabled: None,
                    action: Some(process3d_action("setActiveTool", Some(json!({ "tool": "drill" })))),
                },
                WindowEngagementOption {
                    id: PROCESS3D_ENGAGEMENT_TOOL_ATTACH.into(),
                    label: Some("Attach".into()),
                    icon_id: Some("plus".into()),
                    pressed: Some(envelope.runtime.active_tool == "attach"),
                    disabled: None,
                    action: Some(process3d_action("setActiveTool", Some(json!({ "tool": "attach" })))),
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
                "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                    apply_world3d_sun_action(&mut envelope.runtime.sun, action, args);
                    return vec![set_document_op(&envelope)];
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
                    let position = args.and_then(|value| value.get("position")).and_then(value_as_vec3);
                    let module_id_arg = args.and_then(|value| value.get("moduleId")).and_then(|value| value.as_str());
                    let machine_id_arg = args.and_then(|value| value.get("machineId")).and_then(|value| value.as_str());
                    let modification_kind_id_arg = args.and_then(|value| value.get("modificationKindId")).and_then(|value| value.as_str());
                    let resolved = if let (Some(module_id), Some(machine_id), Some(modification_kind_id)) = (module_id_arg, machine_id_arg, modification_kind_id_arg) {
                        find_modification(module_id, machine_id, modification_kind_id)
                    } else {
                        let legacy_kind = args.and_then(|value| value.get("measure")).and_then(|value| value.as_str()).unwrap_or("cut");
                        let measure_kind = match legacy_kind {
                            "drill" => MeasureKind::Drill,
                            "attach" => MeasureKind::Attach,
                            _ => MeasureKind::Cut,
                        };
                        let (machine, kind) = geometry_machine_for_measure(measure_kind);
                        Some((&GEOMETRY_MODULE, machine, kind))
                    };
                    let Some((module, machine, kind)) = resolved else {
                        return Vec::new();
                    };
                    let failures = validate_modification(machine, kind, &validation_context_for_stock(&envelope.fixture.stock));
                    if !failures.is_empty() {
                        return Vec::new();
                    }
                    let origin = StepOrigin { module_id: module.id.to_string(), machine_id: machine.id.to_string(), modification_kind_id: kind.id.to_string() };
                    let step = ProcessStep { id: next_step_id(), label: kind.label.to_string(), enabled: true, origin: Some(origin), measure: measure_for_modification(machine, kind, position) };
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
                "setActiveTool" => {
                    let tool = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()).unwrap_or("select");
                    envelope.runtime.active_tool = match tool {
                        "cut" => "cut",
                        "drill" => "drill",
                        "attach" => "attach",
                        _ => "select",
                    }
                    .into();
                    envelope.runtime.selected_face_id = None;
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
                    if let Some(point) = args.and_then(|value| value.get("position")).and_then(value_as_vec3) {
                        let measure_kind = match tool.as_str() {
                            "drill" => MeasureKind::Drill,
                            "attach" => MeasureKind::Attach,
                            _ => MeasureKind::Cut,
                        };
                        let (machine, kind) = geometry_machine_for_measure(measure_kind);
                        let origin = StepOrigin { module_id: GEOMETRY_MODULE.id.to_string(), machine_id: machine.id.to_string(), modification_kind_id: kind.id.to_string() };
                        let step = ProcessStep { id: next_step_id(), label: kind.label.to_string(), enabled: true, origin: Some(origin), measure: measure_for_modification(machine, kind, Some(point)) };
                        envelope.runtime.selected_id = Some(step.id.clone());
                        insert_step_at_cursor(&mut envelope.fixture, step);
                        envelope.runtime.active_tool = "select".into();
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "worldPick" => {
                    let granularity = args.and_then(|value| value.get("granularity")).and_then(|value| value.as_str()).unwrap_or("mesh");
                    if granularity == "face" {
                        envelope.runtime.selected_face_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).map(|id| id as u32);
                        return vec![set_document_op(&envelope)];
                    }
                }
                "worldFaceDragEnd" => {
                    if envelope.runtime.active_tool != "select" {
                        return Vec::new();
                    }
                    let normal = args.and_then(|value| value.get("normal")).and_then(value_as_vec3);
                    let point = args.and_then(|value| value.get("startPoint")).and_then(value_as_vec3);
                    let distance = args.and_then(|value| value.get("distance")).and_then(|value| value.as_f64());
                    let face_extent = args.and_then(|value| value.get("faceExtent")).and_then(|value| value.as_array()).and_then(|entries| {
                        Some([entries.first()?.as_f64()?, entries.get(1)?.as_f64()?])
                    });
                    if let (Some(normal), Some(point), Some(distance)) = (normal, point, distance) {
                        if let Some(step) = process3d_step_from_face_drag(normal, point, distance, face_extent, process3d_labels(_view_state)) {
                            envelope.runtime.selected_id = Some(step.id.clone());
                            envelope.runtime.selected_face_id = None;
                            insert_step_at_cursor(&mut envelope.fixture, step);
                            return vec![finalize_document_op(&mut envelope)];
                        }
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
                            process3d_selection_json(&envelope),
                            &envelope.runtime.sun,
                        ),
                    )
                }
                PROCESS_3D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
                PROCESS_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(&envelope, labels),
                PROCESS_3D_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn tools(&self, document_json: &str, view_state: &ViewState) -> Vec<ToolNode> {
            let envelope = parse_envelope(document_json);
            let labels = process3d_labels(view_state);
            let active_tool = envelope.runtime.active_tool.as_str();
            vec![
                tool_toggle("process3d.tool.select", "cursor", labels.select, active_tool == "select", process3d_action("setActiveTool", Some(json!({ "tool": "select" }))))
                    .with_category(ToolCategory::Selection),
                tool_toggle("process3d.tool.cut", "scissors", labels.cut, active_tool == "cut", process3d_action("setActiveTool", Some(json!({ "tool": "cut" }))))
                    .with_category(ToolCategory::Tools),
                tool_toggle("process3d.tool.drill", "circle-dot", labels.drill, active_tool == "drill", process3d_action("setActiveTool", Some(json!({ "tool": "drill" }))))
                    .with_category(ToolCategory::Tools),
                tool_toggle("process3d.tool.attach", "plus", labels.attach, active_tool == "attach", process3d_action("setActiveTool", Some(json!({ "tool": "attach" }))))
                    .with_category(ToolCategory::Tools),
                tool_button("process3d.tool.stepBack", "chevron-left", "Step Back", process3d_action("stepCursorBack", None)).with_category(ToolCategory::History),
                tool_button("process3d.tool.stepForward", "chevron-right", "Step Forward", process3d_action("stepCursorForward", None)).with_category(ToolCategory::History),
                tool_button("process3d.tool.applyAll", "fast-forward", "Apply All", process3d_action("setCursor", Some(json!({ "value": null })))).with_category(ToolCategory::History),
            ]
        }

        fn window_engagements(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
            let envelope = parse_envelope(document_json);
            HashMap::from([(PROCESS_3D_PLAY_WINDOW_MAIN.into(), process3d_engagement(&envelope))])
        }

        fn window_measures(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
            let envelope = parse_envelope(document_json);
            HashMap::from([(PROCESS_3D_PLAY_WINDOW_MAIN.into(), vec![world3d_sun_measures("process3d", &envelope.runtime.sun, process3d_action)])])
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
                origin: None,
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
                origin: None,
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
                origin: None,
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
                origin: None,
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
        fn toggle_sun_round_trips_through_runtime_and_defaults_off() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let envelope = parse_envelope(&document_json);
            assert!(!envelope.runtime.sun.enabled, "sun must be off by default");
            let measures = app.window_measures(&document_json, &ViewState::default());
            assert!(measures.contains_key(PROCESS_3D_PLAY_WINDOW_MAIN));
            let ops = app.handle_action_patch_ops("toggleSun", None, &document_json, &ViewState::default());
            let patched: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            assert_eq!(patched["document"]["runtime"]["sun"]["enabled"], json!(true));
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
        fn set_active_tool_updates_runtime_and_tools_pressed_state() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops("setActiveTool", Some(&json!({ "tool": "cut" })), &document_json, &view_state);
            let patched: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            assert_eq!(patched["document"]["runtime"]["activeTool"], json!("cut"));
            let updated_document_json = patched["document"].to_string();
            let tools = app.tools(&updated_document_json, &view_state);
            let cut_pressed = tools.iter().any(|tool| matches!(tool, ToolNode::Toggle { id, pressed: Some(true), .. } if id == "process3d.tool.cut"));
            assert!(cut_pressed, "expected the cut tool toggle to report pressed after setActiveTool");
            let select_pressed = tools.iter().any(|tool| matches!(tool, ToolNode::Toggle { id, pressed: Some(true), .. } if id == "process3d.tool.select"));
            assert!(!select_pressed, "select toggle must not report pressed while cut is active");
        }

        #[test]
        fn world_pointer_down_reads_position_key_not_point() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops("setActiveTool", Some(&json!({ "tool": "cut" })), &document_json, &view_state);
            let after_tool_json = serde_json::from_str::<Value>(&ops[0]).expect("patch op json")["document"].to_string();
            let ops = app.handle_action_patch_ops("worldPointerDown", Some(&json!({ "position": [1.0, 2.0, 3.0] })), &after_tool_json, &view_state);
            assert_eq!(ops.len(), 1, "worldPointerDown must read the `position` key the renderer actually sends");
            let patched: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let steps = patched["document"]["fixture"]["steps"].as_array().expect("steps array");
            let last = steps.last().expect("inserted step");
            assert_eq!(last["pose"]["position"], json!([1.0, 2.0, 3.0]));
        }

        #[test]
        fn repeated_world_pointer_down_places_steps_at_distinct_positions() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops("setActiveTool", Some(&json!({ "tool": "cut" })), &document_json, &view_state);
            let step1_input = serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string();
            let ops = app.handle_action_patch_ops("worldPointerDown", Some(&json!({ "position": [1.0, 0.0, 0.0] })), &step1_input, &view_state);
            let after_first_json = serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string();
            let ops = app.handle_action_patch_ops("setActiveTool", Some(&json!({ "tool": "cut" })), &after_first_json, &view_state);
            let step2_input = serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string();
            let ops = app.handle_action_patch_ops("worldPointerDown", Some(&json!({ "position": [2.0, 0.0, 0.0] })), &step2_input, &view_state);
            let after_second: Value = serde_json::from_str(&ops[0]).unwrap();
            let steps = after_second["document"]["fixture"]["steps"].as_array().unwrap();
            let last_two: Vec<&Value> = steps.iter().rev().take(2).collect();
            assert_ne!(last_two[0]["pose"]["position"], last_two[1]["pose"]["position"], "repeated clicks at different points must produce distinct step poses");
        }

        #[test]
        fn face_drag_negative_distance_yields_cut() {
            let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], -0.5, None, &PROCESS3D_LABELS_NATIVE_EN).expect("step");
            assert!(matches!(step.measure, ProcessMeasure::Cut { .. }));
            assert_eq!(step.label, "Push Cut");
        }

        #[test]
        fn face_drag_positive_distance_yields_attach() {
            let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.5, None, &PROCESS3D_LABELS_NATIVE_EN).expect("step");
            assert!(matches!(step.measure, ProcessMeasure::Attach { .. }));
            assert_eq!(step.label, "Pull Attach");
        }

        #[test]
        fn face_drag_zero_distance_is_noop() {
            assert!(process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.0, None, &PROCESS3D_LABELS_NATIVE_EN).is_none());
        }

        #[test]
        fn face_drag_orients_box_along_normal() {
            let (axis, angle) = axis_angle_from_up_to([0.0, 1.0, 0.0]);
            assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
            assert!((axis[0] - (-1.0)).abs() < 1e-9 && axis[1].abs() < 1e-9 && axis[2].abs() < 1e-9);
        }

        #[test]
        fn face_drag_degenerate_antiparallel_normal_does_not_panic() {
            let (_, angle) = axis_angle_from_up_to([0.0, 0.0, -1.0]);
            assert!((angle - std::f64::consts::PI).abs() < 1e-9);
        }

        #[test]
        fn box_primitive_spans_from_local_origin_corner() {
            let mut kernel = BrepkitKernel::new();
            let handle = kernel.box_prim_sync(2.0, 3.0, 4.0).expect("box prim");
            let mesh = kernel.tessellate_sync(&handle, 0.1).expect("tessellate");
            let axis_bounds = |offset: usize| -> (f32, f32) {
                let values: Vec<f32> = mesh.position.iter().skip(offset).step_by(3).copied().collect();
                (values.iter().cloned().fold(f32::INFINITY, f32::min), values.iter().cloned().fold(f32::NEG_INFINITY, f32::max))
            };
            let (min_x, max_x) = axis_bounds(0);
            let (min_y, max_y) = axis_bounds(1);
            let (min_z, max_z) = axis_bounds(2);
            assert!(min_x.abs() < 1e-4 && (max_x - 2.0).abs() < 1e-4, "box x should span [0, width] from the local origin corner, got [{min_x}, {max_x}]");
            assert!(min_y.abs() < 1e-4 && (max_y - 3.0).abs() < 1e-4, "box y should span [0, depth], got [{min_y}, {max_y}]");
            assert!(min_z.abs() < 1e-4 && (max_z - 4.0).abs() < 1e-4, "box z should span [0, height], got [{min_z}, {max_z}]");
        }

        #[test]
        fn world_face_drag_end_cut_reduces_volume_end_to_end() {
            let mut app = Process3dPlayApp::default();
            let mut fixture = process_3d::Process3dDocument::default();
            fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
            let stock_volume = processed_volume(&fixture).expect("stock volume");
            let envelope = Process3dEnvelope { fixture, runtime: Process3dRuntime::default() };
            let document_json = serde_json::to_string(&envelope).expect("envelope json");
            let ops = app.handle_action_patch_ops(
                "worldFaceDragEnd",
                Some(&json!({ "normal": [0.0, 0.0, 1.0], "startPoint": [0.5, 0.5, 1.0], "distance": -0.5, "faceExtent": [1.0, 1.0] })),
                &document_json,
                &ViewState::default(),
            );
            assert_eq!(ops.len(), 1);
            let patched: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let new_fixture: process_3d::Process3dDocument = serde_json::from_value(patched["document"]["fixture"].clone()).expect("fixture");
            assert_eq!(new_fixture.steps.len(), 1);
            assert!(matches!(new_fixture.steps[0].measure, ProcessMeasure::Cut { .. }));
            let new_volume = processed_volume(&new_fixture).expect("volume after cut");
            assert!(new_volume < stock_volume, "face-drag cut should reduce volume below stock ({new_volume} vs {stock_volume})");
        }

        #[test]
        fn world_face_drag_end_attach_increases_volume_end_to_end() {
            let mut app = Process3dPlayApp::default();
            let mut fixture = process_3d::Process3dDocument::default();
            fixture.stock.solid = SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 };
            let stock_volume = processed_volume(&fixture).expect("stock volume");
            let envelope = Process3dEnvelope { fixture, runtime: Process3dRuntime::default() };
            let document_json = serde_json::to_string(&envelope).expect("envelope json");
            let ops = app.handle_action_patch_ops(
                "worldFaceDragEnd",
                Some(&json!({ "normal": [0.0, 0.0, 1.0], "startPoint": [0.5, 0.5, 1.0], "distance": 0.5, "faceExtent": [0.2, 0.2] })),
                &document_json,
                &ViewState::default(),
            );
            assert_eq!(ops.len(), 1);
            let patched: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let new_fixture: process_3d::Process3dDocument = serde_json::from_value(patched["document"]["fixture"].clone()).expect("fixture");
            assert_eq!(new_fixture.steps.len(), 1);
            assert!(matches!(new_fixture.steps[0].measure, ProcessMeasure::Attach { .. }));
            let new_volume = processed_volume(&new_fixture).expect("volume after attach");
            assert!(new_volume > stock_volume, "face-drag attach should increase volume above stock ({new_volume} vs {stock_volume})");
        }

        #[test]
        fn world_face_drag_end_ignored_while_a_placement_tool_is_active() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops("setActiveTool", Some(&json!({ "tool": "cut" })), &document_json, &view_state);
            let after_tool_json = serde_json::from_str::<Value>(&ops[0]).expect("patch op json")["document"].to_string();
            let ops = app.handle_action_patch_ops(
                "worldFaceDragEnd",
                Some(&json!({ "normal": [0.0, 0.0, 1.0], "startPoint": [0.5, 0.5, 1.0], "distance": -0.5 })),
                &after_tool_json,
                &view_state,
            );
            assert!(ops.is_empty(), "worldFaceDragEnd should be a no-op while a placement tool is active, not the select tool");
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
                origin: None,
                measure: ProcessMeasure::Drill { radius: 0.1, depth: 1.0, pose: Pose::default() },
            });
            fixture.resolved_up_to = Some(1);
            processed_volume(&fixture).expect("volume at cursor 1");
            let session = process_kernel_session().lock().expect("kernel session lock");
            assert!(session.memo.len() >= 2, "expected stock + drilled prefixes memoized, got {}", session.memo.len());
        }

        #[test]
        fn catalogue_lists_wood_and_concrete_with_mixed_validity_on_default_stock() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let node = app.render(PROCESS_3D_PLAY_BODY_CATALOGUE, &document_json, &view_state);
            let node_json = serde_json::to_string(&node).expect("catalogue json");
            assert!(node_json.contains("Circular Saw"), "expected wood's circular saw in the catalogue: {node_json}");
            assert!(node_json.contains("Table Saw"), "expected wood's table saw in the catalogue: {node_json}");
            assert!(node_json.contains("Diamond Saw"), "expected concrete's diamond saw in the catalogue: {node_json}");
            // The default timber beam (0.24m tall) fits the circular saw's 0.184m diameter but not the
            // table saw's 0.315m or the diamond saw's 0.35m — a real mix of valid and disabled items.
            assert!(node_json.contains("needs stock"), "expected at least one disabled-item validation reason: {node_json}");
        }

        #[test]
        fn add_step_via_catalogue_sets_origin_and_builds_capability_sized_tool() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops(
                "addStep",
                Some(&json!({ "moduleId": "wood", "machineId": "circularSaw", "modificationKindId": "crosscut" })),
                &document_json,
                &view_state,
            );
            assert_eq!(ops.len(), 1, "circular saw crosscut should be valid against the default timber beam stock");
            let patched: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let steps = patched["document"]["fixture"]["steps"].as_array().expect("steps array");
            let last = steps.last().expect("inserted step");
            assert_eq!(last["origin"]["moduleId"], "wood");
            assert_eq!(last["origin"]["machineId"], "circularSaw");
            assert_eq!(last["origin"]["modificationKindId"], "crosscut");
            assert_eq!(last["measure"], "cut");
            let radius = last["tool"]["radius"].as_f64().expect("tool radius");
            assert!((radius - 0.092).abs() < 1e-9, "circular saw diameter 0.184 should size the tool to radius 0.092, got {radius}");
        }

        #[test]
        fn add_step_via_catalogue_rejected_when_validation_fails() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            // Table saw needs >= 0.315m stock height; the default timber beam is only 0.24m tall.
            let ops = app.handle_action_patch_ops(
                "addStep",
                Some(&json!({ "moduleId": "wood", "machineId": "tableSaw", "modificationKindId": "crosscut" })),
                &document_json,
                &view_state,
            );
            assert!(ops.is_empty(), "table saw crosscut should be rejected server-side against undersized stock");
        }

        #[test]
        fn legacy_measure_arg_routes_to_geometry_module() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops("addStep", Some(&json!({ "measure": "cut" })), &document_json, &view_state);
            assert_eq!(ops.len(), 1);
            let patched: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let steps = patched["document"]["fixture"]["steps"].as_array().expect("steps array");
            let last = steps.last().expect("inserted step");
            assert_eq!(last["origin"]["moduleId"], "geometry");
            assert_eq!(last["origin"]["machineId"], "saw");
            assert_eq!(last["origin"]["modificationKindId"], "cut");
            assert_eq!(last["measure"], "cut");
        }

        #[test]
        fn inspector_shows_validation_warning_after_stock_shrinks_below_step_requirement() {
            let mut app = Process3dPlayApp::default();
            let document_json = app.initial_document_json();
            let view_state = ViewState::default();
            let ops = app.handle_action_patch_ops(
                "addStep",
                Some(&json!({ "moduleId": "wood", "machineId": "circularSaw", "modificationKindId": "crosscut" })),
                &document_json,
                &view_state,
            );
            let after_add: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let step_id = after_add["document"]["runtime"]["selectedId"].as_str().expect("selected id").to_string();
            let after_add_json = after_add["document"].to_string();

            let ops = app.handle_action_patch_ops("patchInspector", Some(&json!({ "target": "beam", "field": "height", "value": 0.05 })), &after_add_json, &view_state);
            let after_shrink: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let shrunk_json = after_shrink["document"].to_string();

            let ops = app.handle_action_patch_ops("setSelection", Some(&json!({ "id": step_id })), &shrunk_json, &view_state);
            let after_select: Value = serde_json::from_str(&ops[0]).expect("patch op json");
            let selected_json = after_select["document"].to_string();

            let node = app.render(PROCESS_3D_PLAY_BODY_INSPECTION, &selected_json, &view_state);
            let node_json = serde_json::to_string(&node).expect("inspector json");
            assert!(node_json.contains("needs stock"), "expected a validation warning after shrinking stock below the step's requirement: {node_json}");
        }
    }
}

//#region 🔖Bundle
semio_framework_plugin::semio_plugin! {
    id: "process", label: "Process", version: "0.1.0",
    setup: app_3d::register_process3d_exports,
    apps: [ app_3d::create_process3d_app => app_3d::Process3dPlayApp ],
}
//#endregion 🔖Bundle
