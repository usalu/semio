//! 🪚 Process plugin — subtractive/additive processing simulation in one hot-swappable WASM component.

pub mod app_3d {
    //! 🪚 Process 3D plugin — subtractive/additive processing simulation bundled as a hot-swappable WASM component.

    use base64::Engine;
    use kernel_3d_brepkit::{
        BrepkitKernel, ObjSolidExporter, ObjSolidImporter, SolidExporter, SolidImporter, StepSolidExporter, StepSolidImporter, StlSolidExporter,
        StlSolidImporter,
    };
    use kernel_3d_engine::{BrepKernel, GeometryHandle};
    use process_3d::{Pose, Process3dOp, ProcessMeasure, ProcessStep, ProcessStepPatch, SolidSpec, Stock, StepOrigin};
    use semio_framework_core::kernel::HostEffect;
    use semio_framework_plugin::{
        apply_world3d_sun_action, build_world_3d_scene, create_default_layout, mesh_from_indexed_with_face_groups, mesh_from_kind,
        ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, world3d_camera_json, world3d_mesh_id_from_url, world3d_scene,
        world3d_sun_measures, world3d_selection_json, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionEmit, ActionKind, App,
        DocumentApp, DocumentView, MeshData, MeshExporter, MeshImporter, OsMediaCapability, PanelGroup, ResourceKindSpec, SurfaceKind, UtilityCategory, UtilityDefinition, UiFieldNode,
        UiInputNode, UiInspectorFieldGroup, UiNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement,
        WindowEngagementControl, WindowEngagementInput, WindowEngagementStatus, WindowMeasure, WorldSunConfig,
        SET_ACTIVE_UTILITY_ACTION_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
        FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Mutex, OnceLock};
    use vcs::CollectionOp;

    //#region 🔖Constants
    const PROCESS_3D_PLAY_APP_ID: &str = "process3d-play";
    const PROCESS_3D_PLAY_CONTROLLER_ID: &str = "process3d-play";
    const PROCESS_3D_PLAY_SURFACE_MAIN: &str = "process.play";
    const PROCESS_3D_PLAY_BODY_MAIN: &str = "process.play.main";
    const PROCESS_3D_PLAY_BODY_DOCUMENT: &str = "process.play.document";
    const PROCESS_3D_PLAY_BODY_CATALOGUE: &str = "process.play.catalogue";
    const PROCESS_3D_PLAY_BODY_INSPECTION: &str = "process.play.inspection";
    const PROCESS_3D_PLAY_WINDOW_MAIN: &str = "process-workpiece";
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
        source: &'static str,
        window_main: &'static str,
        field_width: &'static str,
        field_depth: &'static str,
        field_height: &'static str,
        field_radius: &'static str,
        field_pos_x: &'static str,
        field_pos_y: &'static str,
        field_pos_z: &'static str,
        field_angle: &'static str,
        stock_kind_box: &'static str,
        stock_kind_cylinder: &'static str,
        stock_kind_sphere: &'static str,
        import_model: &'static str,
        step_control: &'static str,
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
        source: "Source",
        window_main: "Workpiece",
        field_width: "Width",
        field_depth: "Depth",
        field_height: "Height",
        field_radius: "Radius",
        field_pos_x: "X",
        field_pos_y: "Y",
        field_pos_z: "Z",
        field_angle: "Angle",
        stock_kind_box: "Box",
        stock_kind_cylinder: "Cylinder",
        stock_kind_sphere: "Sphere",
        import_model: "Import Model…",
        step_control: "Step",
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
        source: "Quelle",
        window_main: "Werkstueck",
        field_width: "Breite",
        field_depth: "Tiefe",
        field_height: "Hoehe",
        field_radius: "Radius",
        field_pos_x: "X",
        field_pos_y: "Y",
        field_pos_z: "Z",
        field_angle: "Winkel",
        stock_kind_box: "Quader",
        stock_kind_cylinder: "Zylinder",
        stock_kind_sphere: "Kugel",
        import_model: "Modell importieren…",
        step_control: "Schritt",
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

    /// 🧰 The utility active when the host has not yet set `view_state.active_utility_id`.
    const PROCESS3D_DEFAULT_UTILITY: &str = "select";

    /// 🧰 Resolves the host-owned active utility from session view state, falling back to the default.
    fn process3d_active_utility(view_state: &ViewState) -> &str {
        view_state.active_utility_id.as_deref().unwrap_or(PROCESS3D_DEFAULT_UTILITY)
    }

    /// 🎛️ Ephemeral view state (selection, camera, engagement scratch, sun) — lives in the app struct,
    /// not the document, so it never pollutes undo history. The active utility is host-owned
    /// (`view_state.active_utility_id`), never stored here.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase", default)]
    struct Process3dRuntime {
        selected_id: Option<String>,
        hovered_id: Option<String>,
        /// 🖱️ Id of the brep face currently picked in the viewport (drag-to-cut/attach target).
        selected_face_id: Option<u32>,
        selection_method: String,
        engagement_input: String,
        camera: Process3dCamera,
        sun: WorldSunConfig,
    }

    impl Default for Process3dRuntime {
        fn default() -> Self {
            Self {
                selected_id: None,
                hovered_id: None,
                selected_face_id: None,
                selection_method: default_selection_method(),
                engagement_input: String::new(),
                camera: Process3dCamera::default(),
                sun: WorldSunConfig::default(),
            }
        }
    }

    fn default_document() -> process_3d::Process3dDocument {
        serde_json::from_str(TIMBER_EXAMPLE_JSON).unwrap_or_default()
    }

    fn plate_document() -> process_3d::Process3dDocument {
        serde_json::from_str(PLATE_EXAMPLE_JSON).unwrap_or_else(|_| default_document())
    }

    fn process3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: PROCESS_3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
    }

    /// 🧰 Host effect that programmatically switches the workpiece window's active utility — the active
    /// utility is host-owned session state (`view_state.active_utility_id`), never a document op.
    fn set_active_utility_effect(utility: &str) -> HostEffect {
        HostEffect::SetActiveUtility { window_kind_id: PROCESS_3D_PLAY_WINDOW_MAIN.into(), utility_id: utility.into() }
    }

    /// 📇 A non-palette action declaration (dispatched by UI wiring/keybindings, never surfaced in the
    /// command palette) with the given execution kind.
    fn internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
        ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, kind) }
    }

    fn value_as_vec3(value: &Value) -> Option<[f64; 3]> {
        let array = value.as_array()?;
        Some([array.first()?.as_f64()?, array.get(1)?.as_f64()?, array.get(2)?.as_f64()?])
    }

    fn selected_ids(runtime: &Process3dRuntime) -> Vec<String> {
        runtime.selected_id.clone().into_iter().collect()
    }

    /// 🖱️ Extends the base object-selection JSON with face-picking/drag fields: `targets.face` lets the
    /// renderer hit-test individual triangles; `engagementSessionActive` gates the ground-click placement
    /// path used by the cut/drill/attach utilities; `faceDragActive` gates the push/pull drag gesture, only
    /// while the select utility is active (so a click-to-place utility doesn't also start a face drag).
    fn process3d_selection_json(runtime: &Process3dRuntime, active_utility: &str) -> String {
        let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, &selected_ids(runtime), runtime.hovered_id.as_deref()))
            .unwrap_or_else(|_| json!({}));
        if let Some(object) = value.as_object_mut() {
            object.insert("engagementSessionActive".into(), json!(active_utility != "select"));
            object.insert("selectionMode".into(), json!("face"));
            object.insert("targets".into(), json!({ "mesh": true, "face": true, "vertex": false, "edge": false }));
            object.insert("componentIds".into(), json!(runtime.selected_face_id.map(|id| vec![id]).unwrap_or_default()));
            object.insert("faceDragActive".into(), json!(active_utility == "select"));
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

    /// ✂️➕🗑️ Read-only op builders for the two structural collection edits every mutating action needs:
    /// inserting a step at the resolved-up-to cursor (and advancing it), and removing a step by id (and
    /// pulling the cursor back if it sat past the removed step). Building `Process3dOp`s from an immutable
    /// `&Process3dDocument` keeps `handle_action` free of manual mutation — the VCS store applies them.
    fn insert_step_ops(fixture: &process_3d::Process3dDocument, step: ProcessStep) -> Vec<Process3dOp> {
        let cursor = fixture.resolved_up_to.unwrap_or(fixture.steps.len()).min(fixture.steps.len());
        vec![
            Process3dOp::Steps { collection: CollectionOp::Add { index: cursor, item: step } },
            Process3dOp::SetCursor { resolved_up_to: Some(cursor + 1) },
        ]
    }

    fn remove_step_ops(fixture: &process_3d::Process3dDocument, id: &str) -> Option<Vec<Process3dOp>> {
        let index = fixture.steps.iter().position(|step| step.id == id)?;
        let mut ops = vec![Process3dOp::Steps { collection: CollectionOp::Remove { id: id.to_string() } }];
        if let Some(cursor) = fixture.resolved_up_to {
            if cursor > index {
                ops.push(Process3dOp::SetCursor { resolved_up_to: Some(cursor - 1) });
            }
        }
        Some(ops)
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
    #[derive(Clone, Copy)]
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

    /// 🔎 Finds the geometry module's machine offering a given `measure` kind ("cut"/"drill"/"attach")
    /// — the routing target for the toolbar, click/drag placement, and module-less `addStep` callers.
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
                let value = capability_value(machine, capability)?;
                let required = if is_min { value + margin } else { value - margin };
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

    /// 📐 Imported specs carry no persisted bounding box, so validation falls back to a 1m³ approximation
    /// until the kernel is consulted (matches `cad`'s extent-less fallback for handle-only objects).
    fn stock_extent(solid: &SolidSpec) -> [f64; 3] {
        match solid {
            SolidSpec::Box { width, depth, height } => [*width, *depth, *height],
            SolidSpec::Cylinder { radius, height } => [*radius * 2.0, *radius * 2.0, *height],
            SolidSpec::Sphere { radius } => [*radius * 2.0, *radius * 2.0, *radius * 2.0],
            SolidSpec::ImportedMesh { .. } | SolidSpec::ImportedSolid { .. } => [1.0, 1.0, 1.0],
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
            SolidSpec::ImportedMesh { .. } | SolidSpec::ImportedSolid { .. } => return false,
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

    /// 🩹 Builds the `Process3dOp` for one inspector field edit — clones the target (stock or step),
    /// mutates the clone via `apply_stock_patch`/`apply_step_patch`, then wraps it back into a
    /// `SetStock`/`Steps::Patch` op so the store computes the true pre-state inverse.
    fn process3d_inspector_patch_op(fixture: &process_3d::Process3dDocument, target: &str, field: &str, value: Option<&Value>) -> Option<Process3dOp> {
        if target == fixture.stock.id {
            let mut stock = fixture.stock.clone();
            return if apply_stock_patch(&mut stock, field, value) { Some(Process3dOp::SetStock { stock }) } else { None };
        }
        let step_id = target.strip_prefix("step:")?;
        let step = fixture.steps.iter().find(|step| step.id == step_id)?;
        let mut updated = step.clone();
        if !apply_step_patch(&mut updated, field, value) {
            return None;
        }
        let patch = ProcessStepPatch { label: Some(updated.label), enabled: None, measure: Some(updated.measure), origin: None };
        Some(Process3dOp::Steps { collection: CollectionOp::Patch { id: step_id.to_string(), patch } })
    }
    //#endregion 🔖InspectorPatch
    //#endregion 🔖Document

    //#region 🔖KernelReplay
    /// 🧠 Kernel + prefix memo: `hash(stock, enabled steps[0..i])` → solid handle, so cursor scrubbing and
    /// step edits only recompute the suffix that actually changed.
    /// 🧊 Concrete (not boxed-trait) so `SolidExporter`/`SolidImporter` (STEP/OBJ/STL/GLB import+export)
    /// can borrow `&BrepkitKernel`/`&mut BrepkitKernel` directly; `&mut BrepkitKernel` still coerces to
    /// `&mut dyn BrepKernel` at every existing call site below, so the CSG replay path is unaffected.
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
    fn solid_for_spec(kernel: &mut dyn BrepKernel, spec: &SolidSpec, pose: &Pose) -> Option<GeometryHandle> {
        let base = match spec {
            SolidSpec::Box { width, depth, height } => kernel_3d_engine::block_on(kernel.box_prim(*width, *depth, *height)).ok()?,
            SolidSpec::Cylinder { radius, height } => kernel_3d_engine::block_on(kernel.cylinder_prim(*radius, *height)).ok()?,
            SolidSpec::Sphere { radius } => kernel_3d_engine::block_on(kernel.sphere_prim(*radius)).ok()?,
            SolidSpec::ImportedSolid { solid_handle } => {
                let handle = GeometryHandle(solid_handle.clone());
                kernel_3d_engine::block_on(kernel.kind(&handle)).ok()?;
                handle
            }
            // 🖼️ A GLB-imported reference mesh has no real B-Rep topology in the kernel, so it cannot
            // serve as a CSG operand (stock or tool); the stock-level fallback handles display instead.
            SolidSpec::ImportedMesh { .. } => return None,
        };
        let rotated = if pose.angle != 0.0 { kernel_3d_engine::block_on(kernel.rotate(&base, pose.axis, pose.angle)).ok()? } else { base };
        if pose.position != [0.0, 0.0, 0.0] {
            kernel_3d_engine::block_on(kernel.translate(&rotated, pose.position)).ok()
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

    fn tool_solid_for_measure(kernel: &mut dyn BrepKernel, measure: &ProcessMeasure) -> Option<GeometryHandle> {
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
                ProcessMeasure::Attach { .. } => kernel_3d_engine::block_on(session.kernel.fuse(&handle, &tool)).ok()?,
                _ => kernel_3d_engine::block_on(session.kernel.cut(&handle, &tool)).ok()?,
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
        let mesh = kernel_3d_engine::block_on(session.kernel.tessellate(&handle, PROCESS3D_TESSELLATION_TOLERANCE)).ok()?;
        let face_groups: Vec<(u32, u32, u32)> = mesh.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
        Some(mesh_from_indexed_with_face_groups(&mesh.position, &mesh.normal, &mesh.index, &face_groups))
    }

    fn processed_volume(doc: &process_3d::Process3dDocument) -> Option<f64> {
        let mut session = process_kernel_session().lock().ok()?;
        let handle = replay_process(&mut session, doc)?;
        kernel_3d_engine::block_on(session.kernel.volume(&handle)).ok()
    }

    /// 🖼️ A GLB-imported reference mesh (`SolidSpec::ImportedMesh`) has no kernel-side geometry to
    /// tessellate; it renders by pointing the world3d scene straight at `mesh_url`, mirroring `cad`'s
    /// `resolve_object_mesh_url` → `world3d_mesh_id_from_url` bridge.
    fn evaluated_preview_payload(fixture: &process_3d::Process3dDocument) -> (String, String) {
        if let SolidSpec::ImportedMesh { mesh_url } = &fixture.stock.solid {
            let mesh_id = world3d_mesh_id_from_url(mesh_url);
            let meshes = json!([{ "id": mesh_id, "url": mesh_url }]);
            let instances = json!([{
                "id": "processed",
                "meshId": mesh_id,
                "position": [0.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": fixture.stock.label,
                "selected": false,
                "hovered": false,
            }]);
            return (meshes.to_string(), instances.to_string());
        }
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

    /// 🧊 In-memory memo of the last evaluated preview payload, keyed by document signature — `render`
    /// only sees `&self`, so this lives in a process-wide `Mutex` (mirrors `PROCESS_BREP_KERNEL` above)
    /// rather than the app struct.
    struct Process3dPreviewCache {
        signature: u64,
        meshes_json: String,
        instances_json: String,
    }

    static PROCESS3D_PREVIEW_CACHE: OnceLock<Mutex<Option<Process3dPreviewCache>>> = OnceLock::new();

    fn process3d_preview_cache() -> &'static Mutex<Option<Process3dPreviewCache>> {
        PROCESS3D_PREVIEW_CACHE.get_or_init(|| Mutex::new(None))
    }

    fn preview_payload_cached(fixture: &process_3d::Process3dDocument) -> (String, String) {
        let signature = fixture_signature(fixture);
        if let Ok(cache) = process3d_preview_cache().lock() {
            if let Some(entry) = cache.as_ref() {
                if entry.signature == signature {
                    return (entry.meshes_json.clone(), entry.instances_json.clone());
                }
            }
        }
        let (meshes_json, instances_json) = evaluated_preview_payload(fixture);
        if let Ok(mut cache) = process3d_preview_cache().lock() {
            *cache = Some(Process3dPreviewCache { signature, meshes_json: meshes_json.clone(), instances_json: instances_json.clone() });
        }
        (meshes_json, instances_json)
    }
    //#endregion 🔖KernelReplay

    //#region 🔖MediaImportExport
    /// 📤 A pending native-geometry export ready to become a `HostEffect::DownloadMediaExport`.
    struct Process3dModelExport {
        filename: String,
        data: Value,
        mime_type: String,
        encoding: Option<String>,
    }

    /// 📤 Encodes the replayed stock through `format`'s codec. STEP/OBJ/STL go through the
    /// `SolidExporter` trait objects (real B-Rep, exact where the format allows it); GLB goes through
    /// the mesh tessellation bridge (`processed_mesh` → `GlbExporter`), matching how it is already
    /// rendered/exported elsewhere in this app.
    fn export_process3d_model(fixture: &process_3d::Process3dDocument, format: &str) -> Option<Process3dModelExport> {
        if format == "glb" {
            let mesh = processed_mesh(fixture)?;
            let bytes = semio_framework_plugin::GlbExporter.export(&mesh).ok()?;
            let media_format = semio_framework_plugin::OsMediaFormat::Glb;
            return Some(Process3dModelExport {
                filename: format!("process3d.{}", media_format.as_str()),
                data: Value::String(base64::engine::general_purpose::STANDARD.encode(bytes)),
                mime_type: media_format.mime_type().into(),
                encoding: Some("base64".into()),
            });
        }
        let exporter: Box<dyn SolidExporter> = match format {
            "obj" => Box::new(ObjSolidExporter),
            "stl" => Box::new(StlSolidExporter),
            _ => Box::new(StepSolidExporter),
        };
        let mut session = process_kernel_session().lock().ok()?;
        let handle = replay_process(&mut session, fixture)?;
        let bytes = exporter.export(&session.kernel, &[handle], PROCESS3D_TESSELLATION_TOLERANCE).ok()?;
        let media_format = exporter.format();
        let binary = media_format.is_binary();
        let data = if binary {
            Value::String(base64::engine::general_purpose::STANDARD.encode(&bytes))
        } else {
            Value::String(String::from_utf8(bytes).ok()?)
        };
        Some(Process3dModelExport {
            filename: format!("process3d.{}", media_format.as_str()),
            data,
            mime_type: media_format.mime_type().into(),
            encoding: if binary { Some("base64".into()) } else { None },
        })
    }

    /// 📦 Decodes a `requestFileOpen(readAs: "dataUrl")` payload into raw bytes.
    fn process3d_bytes_from_data_url(data_url: &str) -> Option<Vec<u8>> {
        if let Some((header, encoded)) = data_url.split_once(',') {
            if header.starts_with("data:") {
                return base64::engine::general_purpose::STANDARD.decode(encoded).ok();
            }
        }
        Some(data_url.as_bytes().to_vec())
    }

    /// 📥 Imports a picked file into a brand-new stock-only fixture (steps cleared): STEP/OBJ/STL go
    /// through the `SolidImporter` trait objects and land as `SolidSpec::ImportedSolid` (real B-Rep,
    /// reusable as a Cut/Drill/Attach operand); GLB is decoded once (via the mesh tessellation bridge,
    /// `GlbImporter`) purely to validate it, then kept as `SolidSpec::ImportedMesh` referencing the
    /// original data url directly — it carries no exact B-Rep, so it is never re-imported into the kernel.
    fn import_process3d_model(name: &str, data_url: &str) -> Option<process_3d::Process3dDocument> {
        let bytes = process3d_bytes_from_data_url(data_url)?;
        let mut fixture = process_3d::Process3dDocument::default();
        if name.ends_with(".glb") {
            semio_framework_plugin::GlbImporter.import(&bytes).ok()?;
            fixture.stock = Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: data_url.into() }, pose: Pose::default() };
            return Some(fixture);
        }
        let (importer, label): (Box<dyn SolidImporter>, &str) = if name.ends_with(".stp") || name.ends_with(".step") {
            (Box::new(StepSolidImporter), "Imported STEP")
        } else if name.ends_with(".obj") {
            (Box::new(ObjSolidImporter), "Imported OBJ")
        } else if name.ends_with(".stl") {
            (Box::new(StlSolidImporter), "Imported STL")
        } else {
            return None;
        };
        let mut session = process_kernel_session().lock().ok()?;
        let handle = importer.import(&mut session.kernel, &bytes, PROCESS3D_TESSELLATION_TOLERANCE).ok()?.into_iter().next()?;
        session.memo.clear();
        session.stock_signature = 0;
        fixture.stock = Stock { id: "stock".into(), label: label.into(), solid: SolidSpec::ImportedSolid { solid_handle: handle.0 }, pose: Pose::default() };
        Some(fixture)
    }
    //#endregion 🔖MediaImportExport

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
            loading: None,
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

    fn build_document_tree(fixture: &process_3d::Process3dDocument, runtime: &Process3dRuntime, labels: &Process3dLabels) -> UiNode {
        let stock = &fixture.stock;
        let stock_item = UiTreeItemNode {
            id: stock.id.clone(),
            label: stock.label.clone(),
            description: None,
            icon_id: Some("box".into()),
            selected: Some(runtime.selected_id.as_deref() == Some(stock.id.as_str())),
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
            loading: None,
        };
        let cursor = fixture.resolved_up_to.unwrap_or(fixture.steps.len());
        let step_items: Vec<UiTreeItemNode> = fixture
            .steps
            .iter()
            .enumerate()
            .map(|(index, step)| UiTreeItemNode {
                id: step.id.clone(),
                label: step.label.clone(),
                description: if index >= cursor { Some("pending".into()) } else { None },
                icon_id: Some(process3d_measure_icon(&step.measure).into()),
                selected: Some(runtime.selected_id.as_deref() == Some(step.id.as_str())),
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
                loading: None,
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            sections: vec![
                UiTreeSectionNode { id: "process3d-play-document.stock".into(), label: Some(labels.stock.into()), default_open: Some(true), loading: None, items: vec![stock_item] },
                UiTreeSectionNode { id: "process3d-play-document.steps".into(), label: Some(labels.steps.into()), default_open: Some(true), loading: None, items: step_items },
            ],
            loading: None,
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }

    /// 🏭 Builds one catalogue tree item per machine modification kind across all modules, disabling
    /// (non-clickable, with a reason) any kind the current stock doesn't satisfy.
    fn build_catalogue_tree(fixture: &process_3d::Process3dDocument, labels: &Process3dLabels) -> UiNode {
        let ctx = validation_context_for_stock(&fixture.stock);
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
                                    loading: None,
                                }
                            }
                        })
                    })
                    .collect();
                UiTreeSectionNode { id: format!("process3d-play-catalogue.{}", module.id), label: Some(module.label.into()), default_open: Some(module.id == "geometry"), loading: None, items }
            })
            .collect();
        let stock_items = vec![
            tree_item_with_action("process3d-catalogue.stock-box", labels.stock_kind_box, Some("box"), process3d_action("setStock", Some(json!({ "kind": "box" })))),
            tree_item_with_action("process3d-catalogue.stock-cylinder", labels.stock_kind_cylinder, Some("cylinder"), process3d_action("setStock", Some(json!({ "kind": "cylinder" })))),
            tree_item_with_action("process3d-catalogue.stock-sphere", labels.stock_kind_sphere, Some("circle"), process3d_action("setStock", Some(json!({ "kind": "sphere" })))),
            tree_item_with_action("process3d-catalogue.stock-import", labels.import_model, Some("folder-open"), process3d_action("loadModelRequest", None)),
        ];
        sections.push(UiTreeSectionNode { id: "process3d-play-catalogue.stock".into(), label: Some(labels.stock.into()), default_open: Some(false), loading: None, items: stock_items });
        UiNode::Tree(UiTreeNode { sections, loading: None, selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None })
    }

    fn build_stock_inspector(stock: &Stock, fixture: &process_3d::Process3dDocument, labels: &Process3dLabels) -> UiNode {
        let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &stock.label, &stock.id, "label")];
        match &stock.solid {
            SolidSpec::Box { width, depth, height } => {
                fields.push(number_field("process3d-inspector.width", labels.field_width, *width, &stock.id, "width"));
                fields.push(number_field("process3d-inspector.depth", labels.field_depth, *depth, &stock.id, "depth"));
                fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &stock.id, "height"));
            }
            SolidSpec::Cylinder { radius, height } => {
                fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &stock.id, "radius"));
                fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &stock.id, "height"));
            }
            SolidSpec::Sphere { radius } => {
                fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &stock.id, "radius"));
            }
            SolidSpec::ImportedMesh { mesh_url } => {
                fields.push(ui_inspector_readonly_field("process3d-inspector.source", labels.source, mesh_url.clone()));
            }
            SolidSpec::ImportedSolid { solid_handle } => {
                fields.push(ui_inspector_readonly_field("process3d-inspector.source", labels.source, format!("solid #{solid_handle}")));
            }
        }
        fields.push(number_field("process3d-inspector.posX", labels.field_pos_x, stock.pose.position[0], &stock.id, "posX"));
        fields.push(number_field("process3d-inspector.posY", labels.field_pos_y, stock.pose.position[1], &stock.id, "posY"));
        fields.push(number_field("process3d-inspector.posZ", labels.field_pos_z, stock.pose.position[2], &stock.id, "posZ"));
        fields.push(number_field("process3d-inspector.angle", labels.field_angle, stock.pose.angle, &stock.id, "angle"));
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
                    fields.push(number_field("process3d-inspector.toolWidth", labels.field_width, *width, &target, "toolWidth"));
                    fields.push(number_field("process3d-inspector.toolDepth", labels.field_depth, *depth, &target, "toolDepth"));
                    fields.push(number_field("process3d-inspector.toolHeight", labels.field_height, *height, &target, "toolHeight"));
                }
                pose
            }
            ProcessMeasure::Drill { radius, depth, pose } => {
                fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &target, "radius"));
                fields.push(number_field("process3d-inspector.depth", labels.field_depth, *depth, &target, "depth"));
                pose
            }
            ProcessMeasure::Attach { component, pose } => {
                if let SolidSpec::Cylinder { radius, height } = component {
                    fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &target, "radius"));
                    fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &target, "height"));
                }
                pose
            }
        };
        fields.push(number_field("process3d-inspector.posX", labels.field_pos_x, pose.position[0], &target, "posX"));
        fields.push(number_field("process3d-inspector.posY", labels.field_pos_y, pose.position[1], &target, "posY"));
        fields.push(number_field("process3d-inspector.posZ", labels.field_pos_z, pose.position[2], &target, "posZ"));
        fields.push(number_field("process3d-inspector.angle", labels.field_angle, pose.angle, &target, "angle"));
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
            id: "process3d-inspector.step".into(),
            label: process3d_measure_label(&step.measure, labels).into(),
            default_open: Some(true),
            fields,
        }])
    }

    fn build_inspector_tree(fixture: &process_3d::Process3dDocument, runtime: &Process3dRuntime, labels: &Process3dLabels) -> UiNode {
        let Some(selected_id) = runtime.selected_id.as_deref() else {
            return ui_text(labels.no_selection);
        };
        if selected_id == fixture.stock.id {
            return build_stock_inspector(&fixture.stock, fixture, labels);
        }
        if let Some(step) = fixture.steps.iter().find(|step| step.id == selected_id) {
            return build_step_inspector(step, &fixture.stock, labels);
        }
        ui_text(labels.no_selection)
    }
    //#endregion 🔖Panels

    //#region 🔖Engagement
    fn process3d_engagement(fixture: &process_3d::Process3dDocument, runtime: &Process3dRuntime, active_utility: &str, labels: &Process3dLabels) -> WindowEngagement {
        let len = fixture.steps.len();
        let cursor = fixture.resolved_up_to.unwrap_or(len);
        let volume = processed_volume(fixture).unwrap_or(0.0);
        WindowEngagement {
            session_active: Some(active_utility != "select"),
            // 🧰 The select/cut/drill/attach switcher now lives in the framework toolbar (declared via `.utility` +
            // `.window_kind_utilities`), so the engagement no longer duplicates it as toggle options.
            options: None,
            input: Some(WindowEngagementInput {
                id: Some("process3d-engagement".into()),
                value: Some(runtime.engagement_input.clone()),
                placeholder: Some("cut, drill, attach, back, forward, all".into()),
                disabled: None,
                on_change: Some(process3d_action("engagementInput", None)),
                on_submit: Some(process3d_action("engagementSubmit", None)),
                on_repeat_last: None,
                on_abort: Some(process3d_action("engagementAbort", None)),
            }),
            control: Some(WindowEngagementControl::Stepper {
                id: Some("process3d-cursor".into()),
                label: Some(labels.step_control.into()),
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
    pub struct Process3dPlayApp {
        runtime: Process3dRuntime,
    }

    impl DocumentApp for Process3dPlayApp {
        type Projection = process_3d::Process3dDocument;
        type Op = Process3dOp;

        fn app_id(&self) -> &str {
            PROCESS_3D_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            process_3d::PROCESS_3D_SCHEMA
        }

        fn initial_projection(&self) -> process_3d::Process3dDocument {
            default_document()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, process_3d::Process3dDocument>,
            view_state: &ViewState,
        ) -> ActionEmit<Process3dOp> {
            match action {
                "setDocument" => {
                    if let Some(document_value) = args.and_then(|value| value.get("document")) {
                        if let Ok(document) = serde_json::from_value::<process_3d::Process3dDocument>(document_value.clone()) {
                            self.runtime.selected_id = None;
                            return ActionEmit::ops(vec![Process3dOp::SetDocument { document }]);
                        }
                    }
                    ActionEmit::default()
                }
                "setActiveExample" => {
                    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                    let document = match example_id {
                        PROCESS3D_EXAMPLE_PLATE | "plate" => plate_document(),
                        "" => process_3d::Process3dDocument::default(),
                        _ => default_document(),
                    };
                    self.runtime.selected_id = None;
                    ActionEmit::ops(vec![Process3dOp::SetDocument { document }])
                }
                "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                    apply_world3d_sun_action(&mut self.runtime.sun, action, args);
                    ActionEmit::default()
                }
                "setSelection" => {
                    self.runtime.selected_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                    ActionEmit::default()
                }
                "setHover" => {
                    self.runtime.hovered_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                    ActionEmit::default()
                }
                "setCamera" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                            self.runtime.camera = parsed;
                        }
                    }
                    ActionEmit::default()
                }
                "addStep" => {
                    let position = args.and_then(|value| value.get("position")).and_then(value_as_vec3);
                    let module_id_arg = args.and_then(|value| value.get("moduleId")).and_then(|value| value.as_str());
                    let machine_id_arg = args.and_then(|value| value.get("machineId")).and_then(|value| value.as_str());
                    let modification_kind_id_arg = args.and_then(|value| value.get("modificationKindId")).and_then(|value| value.as_str());
                    let resolved = if let (Some(module_id), Some(machine_id), Some(modification_kind_id)) = (module_id_arg, machine_id_arg, modification_kind_id_arg) {
                        find_modification(module_id, machine_id, modification_kind_id)
                    } else {
                        let measure_arg = args.and_then(|value| value.get("measure")).and_then(|value| value.as_str()).unwrap_or("cut");
                        let measure_kind = match measure_arg {
                            "drill" => MeasureKind::Drill,
                            "attach" => MeasureKind::Attach,
                            _ => MeasureKind::Cut,
                        };
                        let (machine, kind) = geometry_machine_for_measure(measure_kind);
                        Some((&GEOMETRY_MODULE, machine, kind))
                    };
                    let Some((module, machine, kind)) = resolved else {
                        return ActionEmit::default();
                    };
                    let failures = validate_modification(machine, kind, &validation_context_for_stock(&doc.projection.stock));
                    if !failures.is_empty() {
                        return ActionEmit::default();
                    }
                    let origin = StepOrigin { module_id: module.id.to_string(), machine_id: machine.id.to_string(), modification_kind_id: kind.id.to_string() };
                    let step = ProcessStep { id: next_step_id(), label: kind.label.to_string(), enabled: true, origin: Some(origin), measure: measure_for_modification(machine, kind, position) };
                    self.runtime.selected_id = Some(step.id.clone());
                    ActionEmit::ops(insert_step_ops(doc.projection, step))
                }
                "removeStep" => {
                    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                        if let Some(ops) = remove_step_ops(doc.projection, id) {
                            if self.runtime.selected_id.as_deref() == Some(id) {
                                self.runtime.selected_id = None;
                            }
                            return ActionEmit::ops(ops);
                        }
                    }
                    ActionEmit::default()
                }
                "removeSelectedStep" => {
                    if let Some(id) = self.runtime.selected_id.clone() {
                        if let Some(ops) = remove_step_ops(doc.projection, &id) {
                            self.runtime.selected_id = None;
                            return ActionEmit::ops(ops);
                        }
                    }
                    ActionEmit::default()
                }
                "moveStep" => {
                    if let (Some(id), Some(index)) =
                        (args.and_then(|value| value.get("id")).and_then(|value| value.as_str()), args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()))
                    {
                        if doc.projection.steps.iter().any(|step| step.id == id) {
                            return ActionEmit::ops(vec![Process3dOp::Steps { collection: CollectionOp::Move { id: id.to_string(), to_index: index as usize } }]);
                        }
                    }
                    ActionEmit::default()
                }
                "updateStep" => {
                    if let Some(step_value) = args.and_then(|value| value.get("step")) {
                        if let Ok(step) = serde_json::from_value::<ProcessStep>(step_value.clone()) {
                            if doc.projection.steps.iter().any(|existing| existing.id == step.id) {
                                let patch = ProcessStepPatch { label: Some(step.label.clone()), enabled: Some(step.enabled), measure: Some(step.measure.clone()), origin: Some(step.origin.clone()) };
                                return ActionEmit::ops(vec![Process3dOp::Steps { collection: CollectionOp::Patch { id: step.id, patch } }]);
                            }
                        }
                    }
                    ActionEmit::default()
                }
                "setStepEnabled" => {
                    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                        let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(true);
                        if doc.projection.steps.iter().any(|step| step.id == id) {
                            let patch = ProcessStepPatch { enabled: Some(enabled), ..Default::default() };
                            return ActionEmit::ops(vec![Process3dOp::Steps { collection: CollectionOp::Patch { id: id.to_string(), patch } }]);
                        }
                    }
                    ActionEmit::default()
                }
                "setStock" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("box");
                    let solid = match kind {
                        "cylinder" => SolidSpec::Cylinder { radius: 0.3, height: 1.0 },
                        "sphere" => SolidSpec::Sphere { radius: 0.5 },
                        _ => SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 },
                    };
                    let stock = Stock { id: doc.projection.stock.id.clone(), label: process3d_labels(view_state).stock.into(), solid, pose: Pose::default() };
                    let document = process_3d::Process3dDocument { stock, steps: Vec::new(), resolved_up_to: None };
                    self.runtime.selected_id = None;
                    ActionEmit::ops(vec![Process3dOp::SetDocument { document }])
                }
                "patchInspector" => {
                    let target = args.and_then(|value| value.get("target")).and_then(|value| value.as_str()).unwrap_or("");
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                    let value = args.and_then(|value| value.get("value"));
                    match process3d_inspector_patch_op(doc.projection, target, field, value) {
                        Some(op) => ActionEmit::ops(vec![op]),
                        None => ActionEmit::default(),
                    }
                }
                "setCursor" => {
                    let resolved = match args.and_then(|value| value.get("value")) {
                        None | Some(Value::Null) => None,
                        Some(value) => value.as_u64().map(|n| n as usize),
                    };
                    ActionEmit::ops(vec![Process3dOp::SetCursor { resolved_up_to: resolved.map(|n| n.min(doc.projection.steps.len())) }])
                }
                "stepCursor" | "stepCursorBack" | "stepCursorForward" => {
                    let delta = match action {
                        "stepCursorBack" => -1,
                        "stepCursorForward" => 1,
                        _ => args.and_then(|value| value.get("delta")).and_then(|value| value.as_i64()).unwrap_or(0),
                    };
                    let len = doc.projection.steps.len();
                    let current = doc.projection.resolved_up_to.unwrap_or(len) as i64;
                    ActionEmit::ops(vec![Process3dOp::SetCursor { resolved_up_to: Some((current + delta).clamp(0, len as i64) as usize) }])
                }
                SET_ACTIVE_UTILITY_ACTION_ID => {
                    self.runtime.selected_face_id = None;
                    ActionEmit::default()
                }
                "engagementInput" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        self.runtime.engagement_input = value.into();
                    }
                    ActionEmit::default()
                }
                "engagementAbort" => {
                    self.runtime.engagement_input = String::new();
                    ActionEmit::effect(set_active_utility_effect("select"))
                }
                "engagementSubmit" => {
                    let command = self.runtime.engagement_input.trim().to_lowercase();
                    self.runtime.engagement_input = String::new();
                    let len = doc.projection.steps.len();
                    let current = doc.projection.resolved_up_to.unwrap_or(len);
                    match command.split_whitespace().next() {
                        Some("cut") => ActionEmit::effect(set_active_utility_effect("cut")),
                        Some("drill") => ActionEmit::effect(set_active_utility_effect("drill")),
                        Some("attach") => ActionEmit::effect(set_active_utility_effect("attach")),
                        Some("back") => ActionEmit::ops(vec![Process3dOp::SetCursor { resolved_up_to: Some(current.saturating_sub(1)) }]),
                        Some("forward") => ActionEmit::ops(vec![Process3dOp::SetCursor { resolved_up_to: Some((current + 1).min(len)) }]),
                        Some("all") => ActionEmit::ops(vec![Process3dOp::SetCursor { resolved_up_to: None }]),
                        _ => ActionEmit::default(),
                    }
                }
                "worldPointerDown" => {
                    let utility = process3d_active_utility(view_state);
                    if utility == "select" {
                        return ActionEmit::default();
                    }
                    if let Some(point) = args.and_then(|value| value.get("position")).and_then(value_as_vec3) {
                        let measure_kind = match utility {
                            "drill" => MeasureKind::Drill,
                            "attach" => MeasureKind::Attach,
                            _ => MeasureKind::Cut,
                        };
                        let (machine, kind) = geometry_machine_for_measure(measure_kind);
                        let origin = StepOrigin { module_id: GEOMETRY_MODULE.id.to_string(), machine_id: machine.id.to_string(), modification_kind_id: kind.id.to_string() };
                        let step = ProcessStep { id: next_step_id(), label: kind.label.to_string(), enabled: true, origin: Some(origin), measure: measure_for_modification(machine, kind, Some(point)) };
                        self.runtime.selected_id = Some(step.id.clone());
                        let mut emit = ActionEmit::ops(insert_step_ops(doc.projection, step));
                        emit.effects.push(set_active_utility_effect("select"));
                        return emit;
                    }
                    ActionEmit::default()
                }
                "worldPick" => {
                    let granularity = args.and_then(|value| value.get("granularity")).and_then(|value| value.as_str()).unwrap_or("mesh");
                    if granularity == "face" {
                        self.runtime.selected_face_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).map(|id| id as u32);
                    }
                    ActionEmit::default()
                }
                "worldFaceDragEnd" => {
                    if process3d_active_utility(view_state) != "select" {
                        return ActionEmit::default();
                    }
                    let normal = args.and_then(|value| value.get("normal")).and_then(value_as_vec3);
                    let point = args.and_then(|value| value.get("startPoint")).and_then(value_as_vec3);
                    let distance = args.and_then(|value| value.get("distance")).and_then(|value| value.as_f64());
                    let face_extent = args.and_then(|value| value.get("faceExtent")).and_then(|value| value.as_array()).and_then(|entries| {
                        Some([entries.first()?.as_f64()?, entries.get(1)?.as_f64()?])
                    });
                    if let (Some(normal), Some(point), Some(distance)) = (normal, point, distance) {
                        if let Some(step) = process3d_step_from_face_drag(normal, point, distance, face_extent, process3d_labels(view_state)) {
                            self.runtime.selected_id = Some(step.id.clone());
                            self.runtime.selected_face_id = None;
                            return ActionEmit::ops(insert_step_ops(doc.projection, step));
                        }
                    }
                    ActionEmit::default()
                }
                "exportModel" => {
                    let format = args.and_then(|value| value.get("format")).and_then(|value| value.as_str()).unwrap_or("step");
                    match export_process3d_model(doc.projection, format) {
                        Some(export) => ActionEmit::effect(HostEffect::DownloadMediaExport {
                            filename: export.filename,
                            mime_type: export.mime_type,
                            data: match export.data {
                                Value::String(text) => text,
                                other => serde_json::to_string(&other).unwrap_or_default(),
                            },
                            encoding: export.encoding,
                        }),
                        None => ActionEmit::default(),
                    }
                }
                "loadModelRequest" => ActionEmit::effect(HostEffect::RequestFileOpen {
                    accept: ".stp,.step,.obj,.stl,.glb".into(),
                    read_as: Some("dataUrl".into()),
                    import_action: "importModelFile".into(),
                }),
                "importModelFile" => {
                    let name = args.and_then(|value| value.get("name")).and_then(|value| value.as_str()).unwrap_or("").to_ascii_lowercase();
                    let payload = args.and_then(|value| value.get("payload")).cloned().or_else(|| args.cloned());
                    let Some(data_url) = payload.as_ref().and_then(Value::as_str) else {
                        return ActionEmit::default();
                    };
                    match import_process3d_model(&name, data_url) {
                        Some(document) => {
                            self.runtime.selected_id = None;
                            ActionEmit::ops(vec![Process3dOp::SetDocument { document }])
                        }
                        None => ActionEmit::default(),
                    }
                }
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, process_3d::Process3dDocument>, view_state: &ViewState) -> UiNode {
            let labels = process3d_labels(view_state);
            match body_key {
                PROCESS_3D_PLAY_BODY_MAIN => {
                    let (meshes_json, instances_json) = preview_payload_cached(doc.projection);
                    build_world_3d_scene(
                        PROCESS_3D_PLAY_SURFACE_MAIN,
                        PROCESS_3D_PLAY_APP_ID,
                        world3d_scene(
                            world3d_camera_json(self.runtime.camera.position, self.runtime.camera.target, self.runtime.camera.fov),
                            meshes_json,
                            instances_json,
                            process3d_selection_json(&self.runtime, process3d_active_utility(view_state)),
                            &self.runtime.sun,
                        ),
                    )
                }
                PROCESS_3D_PLAY_BODY_DOCUMENT => build_document_tree(doc.projection, &self.runtime, labels),
                PROCESS_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(doc.projection, labels),
                PROCESS_3D_PLAY_BODY_INSPECTION => build_inspector_tree(doc.projection, &self.runtime, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_engagements(&self, doc: &DocumentView<'_, process_3d::Process3dDocument>, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
            HashMap::from([(
                PROCESS_3D_PLAY_WINDOW_MAIN.into(),
                process3d_engagement(doc.projection, &self.runtime, process3d_active_utility(view_state), process3d_labels(view_state)),
            )])
        }

        fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
            let labels = process3d_labels(view_state);
            let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
            semio_framework_plugin::AppLabelsOverlay {
                window_kind_labels: std::collections::HashMap::from([(PROCESS_3D_PLAY_WINDOW_MAIN.to_string(), labels.window_main.to_string())]),
                panel_tab_labels: std::collections::HashMap::new(),
                mode_labels: std::collections::HashMap::new(),
                action_labels: process3d_action_labels(is_de),
                utility_labels: process3d_utility_labels(is_de),
                example_labels: HashMap::new(),
                action_arg_labels: HashMap::new(),
                dialog_labels: HashMap::new(),
                introduction_labels: HashMap::new(),
            }
        }

        fn window_measures(&self, _doc: &DocumentView<'_, process_3d::Process3dDocument>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
            HashMap::from([(PROCESS_3D_PLAY_WINDOW_MAIN.into(), vec![world3d_sun_measures("process3d", &self.runtime.sun, process3d_action)])])
        }
    }
    //#endregion 🔖Process3dPlayApp

    //#region 🔖CommandLabels
    /// 🗣️ (action id) -> localized label for every operation/view-action/shell-action declared in
    /// `create_process3d_app`'s static manifest — the manifest itself has no `view_state`/locale
    /// parameter, so this overlay is how the command palette and Actions rail get a translated label
    /// without threading locale through the whole builder chain.
    fn process3d_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
        const ENTRIES: &[(&str, &str, &str)] = &[
            ("addStep", "Add Step", "Schritt hinzufuegen"),
            ("setStock", "Set Stock", "Rohteil festlegen"),
            ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
            ("removeSelectedStep", "Remove Selected Step", "Ausgewaehlten Schritt entfernen"),
            ("exportModel", "Export Model", "Modell exportieren"),
            ("loadModelRequest", "Load Model…", "Modell laden…"),
            ("setDocument", "Set Document", "Dokument festlegen"),
            ("importModelFile", "Import Model File", "Modelldatei importieren"),
            ("removeStep", "Remove Step", "Schritt entfernen"),
            ("moveStep", "Move Step", "Schritt verschieben"),
            ("updateStep", "Update Step", "Schritt aktualisieren"),
            ("setStepEnabled", "Set Step Enabled", "Schrittaktivierung festlegen"),
            ("patchInspector", "Patch Inspector", "Inspektor aktualisieren"),
            ("worldPointerDown", "World Pointer Down", "Welt-Zeiger gedrueckt"),
            ("worldFaceDragEnd", "World Face Drag End", "Welt-Flaechenzug beendet"),
            ("setCursor", "Set Cursor", "Cursor festlegen"),
            ("stepCursor", "Step Cursor", "Cursor schrittweise bewegen"),
            ("stepCursorBack", "Step Cursor Back", "Cursor zurueck"),
            ("stepCursorForward", "Step Cursor Forward", "Cursor vorwaerts"),
            ("engagementSubmit", "Engagement Submit", "Eingabe bestaetigen"),
            ("engagementInput", "Engagement Input", "Eingabe"),
            ("engagementAbort", "Engagement Abort", "Eingabe abbrechen"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("setHover", "Set Hover", "Hover festlegen"),
            ("setCamera", "Set Camera", "Kamera festlegen"),
            ("worldPick", "World Pick", "Welt-Auswahl (Pick)"),
            ("toggleSun", "Toggle Sun", "Sonne umschalten"),
            ("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen"),
            ("setSunElevation", "Set Sun Elevation", "Sonnenhoehe festlegen"),
            ("setSunIntensity", "Set Sun Intensity", "Sonnenintensitaet festlegen"),
        ];
        ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
    }

    /// 🗣️ (utility id) -> localized toolbar-button label, for every `.utility(...)` declared in `create_process3d_app`.
    fn process3d_utility_labels(is_de: bool) -> std::collections::HashMap<String, String> {
        const ENTRIES: &[(&str, &str, &str)] = &[
            ("select", "Select", "Auswaehlen"),
            ("cut", "Cut", "Schneiden"),
            ("drill", "Drill", "Bohren"),
            ("attach", "Attach", "Anbauen"),
        ];
        ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
    }
    //#endregion 🔖CommandLabels

    //#region 🔖Manifest
    pub fn create_process3d_app() -> App {
        App::from_builder(
            App::builder(PROCESS_3D_PLAY_APP_ID, "Process 3D")
                .document(["semio", "process", "3d"])
                .resource_kind(ResourceKindSpec {
                    id: "3d.process".into(),
                    name: "3D Process".into(),
                    source_format: "process.3d".into(),
                    component_kind: "process3d".into(),
                    dimension: "3d".into(),
                    media_capability: OsMediaCapability::Brep,
                })
                .icon_id("hammer")
                .mode("edit", "Edit")
                .default_mode_id("edit")
                .window_kind_with_engagement(
                    PROCESS_3D_PLAY_WINDOW_MAIN,
                    "Workpiece",
                    PROCESS_3D_PLAY_BODY_MAIN,
                    SurfaceKind::World3d,
                    process3d_engagement(&default_document(), &Process3dRuntime::default(), PROCESS3D_DEFAULT_UTILITY, &PROCESS3D_LABELS_NATIVE_EN),
                )
                .default_layout(create_default_layout(&[PROCESS_3D_PLAY_WINDOW_MAIN.into()], "row", None, Some(&["Workpiece".into()])))
                .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PROCESS_3D_PLAY_BODY_DOCUMENT)
                .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PROCESS_3D_PLAY_BODY_CATALOGUE)
                .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PROCESS_3D_PLAY_BODY_INSPECTION)
                // 🔧 Palette-visible create/mutate actions (staged arg forms attached below).
                .operation("addStep", "Add Step")
                .operation("setStock", "Set Stock")
                .operation("setActiveExample", "Set Active Example")
                .operation("removeSelectedStep", "Remove Selected Step")
                // 🐚 Palette-visible host round-trips.
                .shell_action("exportModel", "Export Model")
                .shell_action("loadModelRequest", "Load Model…")
                // 🔧 Internal document mutations dispatched by panel/viewport wiring (not palette-worthy).
                .action_with(internal_action("setDocument", "Set Document", ActionKind::Operation))
                .action_with(internal_action("importModelFile", "Import Model File", ActionKind::Operation))
                .action_with(internal_action("removeStep", "Remove Step", ActionKind::Operation))
                .action_with(internal_action("moveStep", "Move Step", ActionKind::Operation))
                .action_with(internal_action("updateStep", "Update Step", ActionKind::Operation))
                .action_with(internal_action("setStepEnabled", "Set Step Enabled", ActionKind::Operation))
                .action_with(internal_action("patchInspector", "Patch Inspector", ActionKind::Operation))
                .action_with(internal_action("worldPointerDown", "World Pointer Down", ActionKind::Operation))
                .action_with(internal_action("worldFaceDragEnd", "World Face Drag End", ActionKind::Operation))
                // ⏱️ Document-cursor navigation ops (NOT framework History — they move the replay cursor).
                .action_with(internal_action("setCursor", "Set Cursor", ActionKind::Operation))
                .action_with(internal_action("stepCursor", "Step Cursor", ActionKind::Operation))
                .action_with(internal_action("stepCursorBack", "Step Cursor Back", ActionKind::Operation))
                .action_with(internal_action("stepCursorForward", "Step Cursor Forward", ActionKind::Operation))
                // 🎛️ Engagement session command line (a separate system from utility selection).
                .action_with(internal_action("engagementSubmit", "Engagement Submit", ActionKind::Operation))
                .action_with(internal_action("engagementInput", "Engagement Input", ActionKind::View))
                .action_with(internal_action("engagementAbort", "Engagement Abort", ActionKind::View))
                // 👁️ Ephemeral view state — selection, hover, camera, face picking, sun.
                .action_with(internal_action("setSelection", "Set Selection", ActionKind::View))
                .action_with(internal_action("setHover", "Set Hover", ActionKind::View))
                .action_with(internal_action("setCamera", "Set Camera", ActionKind::View))
                .action_with(internal_action("worldPick", "World Pick", ActionKind::View))
                .action_with(internal_action("toggleSun", "Toggle Sun", ActionKind::View))
                .action_with(internal_action("setSunAzimuth", "Set Sun Azimuth", ActionKind::View))
                .action_with(internal_action("setSunElevation", "Set Sun Elevation", ActionKind::View))
                .action_with(internal_action("setSunIntensity", "Set Sun Intensity", ActionKind::View))
                // 📝 Staged argument forms for the palette-visible create/export actions.
                .action_args("addStep", vec![
                    ActionArgDef::select("measure", "Measure", vec![
                        ActionArgOption::new("cut", "Cut"),
                        ActionArgOption::new("drill", "Drill"),
                        ActionArgOption::new("attach", "Attach"),
                    ]).default_value("cut"),
                ])
                .action_args("setStock", vec![
                    ActionArgDef::select("kind", "Kind", vec![
                        ActionArgOption::new("box", "Box"),
                        ActionArgOption::new("cylinder", "Cylinder"),
                        ActionArgOption::new("sphere", "Sphere"),
                    ]).default_value("box"),
                ])
                .action_args("setActiveExample", vec![
                    ActionArgDef::select("exampleId", "Example", vec![
                        ActionArgOption::new(PROCESS3D_EXAMPLE_TIMBER, "Timber Beam Joinery"),
                        ActionArgOption::new(PROCESS3D_EXAMPLE_PLATE, "Drilled Plate"),
                    ]).required().default_value(PROCESS3D_EXAMPLE_TIMBER),
                ])
                .action_args("exportModel", vec![
                    ActionArgDef::select("format", "Format", vec![
                        ActionArgOption::new("step", "STEP"),
                        ActionArgOption::new("obj", "OBJ"),
                        ActionArgOption::new("stl", "STL"),
                        ActionArgOption::new("glb", "GLB"),
                    ]).required().default_value("step"),
                ])
                // 🧰 Flat top-level exclusive toolbar scoped to the workpiece window (active utility is
                // host-owned). These four are the window's entire utility set — not a sub-collection — so
                // each carries `group: None` and renders as its own flat toolbar icon.
                .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", "Select", "cursor") })
                .utility(UtilityDefinition { category: Some(UtilityCategory::Tools), ..UtilityDefinition::new("cut", "Cut", "scissors") })
                .utility(UtilityDefinition { category: Some(UtilityCategory::Tools), ..UtilityDefinition::new("drill", "Drill", "circle-dot") })
                .utility(UtilityDefinition { category: Some(UtilityCategory::Tools), ..UtilityDefinition::new("attach", "Attach", "plus") })
                .window_kind_utilities(PROCESS_3D_PLAY_WINDOW_MAIN, vec!["select".into(), "cut".into(), "drill".into(), "attach".into()])
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
        let document: process_3d::Process3dDocument = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
        processed_mesh(&document).ok_or_else(|| "process3d: kernel replay failed".to_string())
    }

    fn process3d_document_from_mesh(_mesh: &MeshData) -> Result<Value, String> {
        Err("process3d: mesh import not supported".into())
    }

    pub fn register_process3d_exports() {
        semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
        semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
        semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
        semio_framework_os::register_mesh_dwg_export_handler("3d.process", "process", process3d_mesh_from_document);
        semio_framework_os::register_mesh_dwg_import_handler("3d.process", process3d_document_from_mesh);
    }
    //#endregion 🔖Manifest

    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};

        fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        fn new_app() -> VcsDocumentApp<Process3dPlayApp> {
            VcsDocumentApp::new(Process3dPlayApp::default())
        }

        /// 🧰 A session view state with a specific host-owned active utility (mirrors how the shell threads
        /// `active_utility_id` after a toolbar switch).
        fn view_with_utility(utility: &str) -> ViewState {
            ViewState { active_utility_id: Some(utility.into()), ..ViewState::default() }
        }

        #[test]
        fn default_document_parses_timber_example() {
            let document = default_document();
            assert_eq!(document.steps.len(), 4);
            assert!(document.resolved_up_to.is_none());
        }

        #[test]
        fn utility_registry_declares_four_flat_utilities_scoped_to_workpiece_window() {
            let definition = create_process3d_app().definition;
            let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
            assert_eq!(utility_ids, ["select", "cut", "drill", "attach"], "utilities declared in registry order");
            assert!(
                definition.utilities.iter().all(|utility| utility.group.is_none()),
                "process's select/cut/drill/attach are the window's entire top-level utility set, so none carry a visual group (a shared group would fold them into one collection button)",
            );
            let window = definition
                .window_kinds
                .iter()
                .find(|window| window.id == PROCESS_3D_PLAY_WINDOW_MAIN)
                .expect("workpiece window");
            let scoped: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
            assert_eq!(scoped, ["select", "cut", "drill", "attach"], "all four utilities scoped to the workpiece window kind");
        }

        #[test]
        fn plate_document_parses_and_opens_mid_timeline() {
            let document = plate_document();
            assert_eq!(document.steps.len(), 3);
            assert_eq!(document.resolved_up_to, Some(2));
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
            let mut app = new_app();
            let measures = app.window_measures(&ViewState::default());
            let sun_group = |measures: &HashMap<String, Vec<WindowMeasure>>| {
                measures[PROCESS_3D_PLAY_WINDOW_MAIN]
                    .iter()
                    .find_map(|measure| match measure {
                        WindowMeasure::Group { id, children, .. } if id == "process3d-measure-sun" => Some(children.clone()),
                        _ => None,
                    })
                    .expect("sun measure group")
            };
            let children = sun_group(&measures);
            assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if !*pressed)));
            app.handle_action("toggleSun", None, &ViewState::default(), &meta("local")).expect("toggle");
            let measures = app.window_measures(&ViewState::default());
            let children = sun_group(&measures);
            assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if *pressed)));
        }

        #[test]
        fn add_step_action_inserts_and_selects() {
            let mut app = new_app();
            app.handle_action("addStep", Some(&json!({ "measure": "drill" })), &ViewState::default(), &meta("local")).expect("add step");
            let document = app.projection().expect("projection");
            assert_eq!(document.steps.len(), 5);
            let node = app.render(PROCESS_3D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
            let node_json = serde_json::to_string(&node).unwrap();
            assert!(!node_json.contains("No selection"), "expected the newly added step to be selected: {node_json}");
        }

        #[test]
        fn undo_after_add_step_restores_previous_step_count() {
            let mut app = new_app();
            app.handle_action("addStep", Some(&json!({ "measure": "cut" })), &ViewState::default(), &meta("local")).expect("add step");
            assert_eq!(app.projection().expect("projection").steps.len(), 5);
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert_eq!(app.projection().expect("projection").steps.len(), 4);
        }

        #[test]
        fn set_active_utility_emits_no_operations() {
            let mut app = new_app();
            let result = app
                .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "cut" })), &view_with_utility("cut"), &meta("local"))
                .expect("set utility");
            assert!(result.operations.is_empty(), "utility selection is host-owned view state and must never emit document ops or history");
        }

        #[test]
        fn engagement_exposes_no_utility_switch_options() {
            let app = Process3dPlayApp::default();
            let doc = process_3d::Process3dDocument::default();
            let engagement = process3d_engagement(&doc, &app.runtime, "cut", &PROCESS3D_LABELS_NATIVE_EN);
            assert!(
                engagement.options.is_none(),
                "select/cut/drill/attach switching lives only on the framework toolbar; the engagement must not duplicate it as options",
            );
        }

        #[test]
        fn arg_form_set_stock_emits_ops_reading_kind_arg() {
            let mut app = new_app();
            let result = app
                .handle_action("setStock", Some(&json!({ "kind": "cylinder" })), &ViewState::default(), &meta("local"))
                .expect("set stock");
            assert!(!result.operations.is_empty(), "the setStock arg form must materialize into document ops");
            let document = app.projection().expect("projection");
            assert!(matches!(document.stock.solid, SolidSpec::Cylinder { .. }), "setStock kind=cylinder must swap the stock solid");
            assert!(document.steps.is_empty(), "swapping stock resets the step timeline");
        }

        fn step_pose(step: &ProcessStep) -> [f64; 3] {
            match &step.measure {
                ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose.position,
            }
        }

        #[test]
        fn world_pointer_down_reads_position_key_not_point() {
            let mut app = new_app();
            let result = app.handle_action("worldPointerDown", Some(&json!({ "position": [1.0, 2.0, 3.0] })), &view_with_utility("cut"), &meta("local")).expect("pointer down");
            assert!(!result.operations.is_empty(), "worldPointerDown must read the `position` key the renderer actually sends");
            let document = app.projection().expect("projection");
            let last = document.steps.last().expect("inserted step");
            assert_eq!(step_pose(last), [1.0, 2.0, 3.0]);
        }

        #[test]
        fn world_pointer_down_resets_active_utility_to_select() {
            let mut app = new_app();
            let result = app.handle_action("worldPointerDown", Some(&json!({ "position": [1.0, 2.0, 3.0] })), &view_with_utility("cut"), &meta("local")).expect("pointer down");
            assert!(
                result.requested_effects.iter().any(|effect| matches!(effect, HostEffect::SetActiveUtility { utility_id, .. } if utility_id == "select")),
                "placing a step must hand the host a SetActiveUtility(select) effect so the click-to-place utility disengages",
            );
        }

        #[test]
        fn repeated_world_pointer_down_places_steps_at_distinct_positions() {
            let mut app = new_app();
            app.handle_action("worldPointerDown", Some(&json!({ "position": [1.0, 0.0, 0.0] })), &view_with_utility("cut"), &meta("local")).expect("pointer 1");
            app.handle_action("worldPointerDown", Some(&json!({ "position": [2.0, 0.0, 0.0] })), &view_with_utility("cut"), &meta("local")).expect("pointer 2");
            let document = app.projection().expect("projection");
            let last_two: Vec<&ProcessStep> = document.steps.iter().rev().take(2).collect();
            assert_ne!(step_pose(last_two[0]), step_pose(last_two[1]), "repeated clicks at different points must produce distinct step poses");
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
            let handle = kernel_3d_engine::block_on(kernel.box_prim(2.0, 3.0, 4.0)).expect("box prim");
            let mesh = kernel_3d_engine::block_on(kernel.tessellate(&handle, 0.1)).expect("tessellate");
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
            let mut app = new_app();
            app.handle_action("setStock", Some(&json!({ "kind": "box" })), &ViewState::default(), &meta("local")).expect("set stock");
            let stock_volume = processed_volume(&app.projection().expect("projection")).expect("stock volume");
            let result = app.handle_action(
                "worldFaceDragEnd",
                Some(&json!({ "normal": [0.0, 0.0, 1.0], "startPoint": [0.5, 0.5, 1.0], "distance": -0.5, "faceExtent": [1.0, 1.0] })),
                &ViewState::default(),
                &meta("local"),
            ).expect("face drag");
            assert!(!result.operations.is_empty());
            let document = app.projection().expect("projection");
            assert_eq!(document.steps.len(), 1);
            assert!(matches!(document.steps[0].measure, ProcessMeasure::Cut { .. }));
            let new_volume = processed_volume(&document).expect("volume after cut");
            assert!(new_volume < stock_volume, "face-drag cut should reduce volume below stock ({new_volume} vs {stock_volume})");
        }

        #[test]
        fn world_face_drag_end_attach_increases_volume_end_to_end() {
            let mut app = new_app();
            app.handle_action("setStock", Some(&json!({ "kind": "box" })), &ViewState::default(), &meta("local")).expect("set stock");
            let stock_volume = processed_volume(&app.projection().expect("projection")).expect("stock volume");
            let result = app.handle_action(
                "worldFaceDragEnd",
                Some(&json!({ "normal": [0.0, 0.0, 1.0], "startPoint": [0.5, 0.5, 1.0], "distance": 0.5, "faceExtent": [0.2, 0.2] })),
                &ViewState::default(),
                &meta("local"),
            ).expect("face drag");
            assert!(!result.operations.is_empty());
            let document = app.projection().expect("projection");
            assert_eq!(document.steps.len(), 1);
            assert!(matches!(document.steps[0].measure, ProcessMeasure::Attach { .. }));
            let new_volume = processed_volume(&document).expect("volume after attach");
            assert!(new_volume > stock_volume, "face-drag attach should increase volume above stock ({new_volume} vs {stock_volume})");
        }

        #[test]
        fn world_face_drag_end_ignored_while_a_placement_utility_is_active() {
            let mut app = new_app();
            let result = app.handle_action(
                "worldFaceDragEnd",
                Some(&json!({ "normal": [0.0, 0.0, 1.0], "startPoint": [0.5, 0.5, 1.0], "distance": -0.5 })),
                &view_with_utility("cut"),
                &meta("local"),
            ).expect("face drag");
            assert!(result.operations.is_empty(), "worldFaceDragEnd should be a no-op while a placement utility is active, not the select utility");
        }

        #[test]
        fn render_world_scene_contains_processed_mesh() {
            let mut app = new_app();
            let node = app.render(PROCESS_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
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

        /// 🪵 The default timber beam (0.24m tall) fits the circular saw's 0.184m diameter but not the
        /// table saw's 0.315m or the diamond saw's 0.35m — a real mix of valid and disabled items.
        #[test]
        fn catalogue_lists_wood_and_concrete_with_mixed_validity_on_default_stock() {
            let mut app = new_app();
            let node = app.render(PROCESS_3D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
            let node_json = serde_json::to_string(&node).expect("catalogue json");
            assert!(node_json.contains("Circular Saw"), "expected wood's circular saw in the catalogue: {node_json}");
            assert!(node_json.contains("Table Saw"), "expected wood's table saw in the catalogue: {node_json}");
            assert!(node_json.contains("Diamond Saw"), "expected concrete's diamond saw in the catalogue: {node_json}");
            assert!(node_json.contains("needs stock"), "expected at least one disabled-item validation reason: {node_json}");
        }

        #[test]
        fn add_step_via_catalogue_sets_origin_and_builds_capability_sized_tool() {
            let mut app = new_app();
            let result = app.handle_action(
                "addStep",
                Some(&json!({ "moduleId": "wood", "machineId": "circularSaw", "modificationKindId": "crosscut" })),
                &ViewState::default(),
                &meta("local"),
            ).expect("add step");
            assert!(!result.operations.is_empty(), "circular saw crosscut should be valid against the default timber beam stock");
            let document = app.projection().expect("projection");
            let last = document.steps.last().expect("inserted step");
            let origin = last.origin.as_ref().expect("origin");
            assert_eq!(origin.module_id, "wood");
            assert_eq!(origin.machine_id, "circularSaw");
            assert_eq!(origin.modification_kind_id, "crosscut");
            let ProcessMeasure::Cut { tool: SolidSpec::Cylinder { radius, .. }, .. } = &last.measure else {
                panic!("expected a cylinder cut tool, got {:?}", last.measure);
            };
            assert!((radius - 0.092).abs() < 1e-9, "circular saw diameter 0.184 should size the tool to radius 0.092, got {radius}");
        }

        /// 🪵 Table saw needs >= 0.315m stock height; the default timber beam is only 0.24m tall.
        #[test]
        fn add_step_via_catalogue_rejected_when_validation_fails() {
            let mut app = new_app();
            let result = app.handle_action(
                "addStep",
                Some(&json!({ "moduleId": "wood", "machineId": "tableSaw", "modificationKindId": "crosscut" })),
                &ViewState::default(),
                &meta("local"),
            ).expect("add step");
            assert!(result.operations.is_empty(), "table saw crosscut should be rejected server-side against undersized stock");
        }

        #[test]
        fn measure_arg_routes_to_geometry_module() {
            let mut app = new_app();
            app.handle_action("addStep", Some(&json!({ "measure": "cut" })), &ViewState::default(), &meta("local")).expect("add step");
            let document = app.projection().expect("projection");
            let last = document.steps.last().expect("inserted step");
            let origin = last.origin.as_ref().expect("origin");
            assert_eq!(origin.module_id, "geometry");
            assert_eq!(origin.machine_id, "saw");
            assert_eq!(origin.modification_kind_id, "cut");
            assert!(matches!(last.measure, ProcessMeasure::Cut { .. }));
        }

        #[test]
        fn inspector_shows_validation_warning_after_stock_shrinks_below_step_requirement() {
            let mut app = new_app();
            let add_result = app.handle_action(
                "addStep",
                Some(&json!({ "moduleId": "wood", "machineId": "circularSaw", "modificationKindId": "crosscut" })),
                &ViewState::default(),
                &meta("local"),
            ).expect("add step");
            assert!(!add_result.operations.is_empty());
            app.handle_action("patchInspector", Some(&json!({ "target": "beam", "field": "height", "value": 0.05 })), &ViewState::default(), &meta("local")).expect("shrink stock");
            let step_id = app.projection().expect("projection").steps.last().expect("step").id.clone();
            app.handle_action("setSelection", Some(&json!({ "id": step_id })), &ViewState::default(), &meta("local")).expect("select");
            let node = app.render(PROCESS_3D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
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
