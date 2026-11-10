//! 🪟️ Manifest and media of the norm app surface: the mode, window and panel definitions, the artifact kind spec and the media export and import. See [`super`].

use super::*;
use semio_plugin::{ActionArgDef, WindowLayoutAxisNode, WindowLayoutChild};

//#region 🔖️Manifest
/// ✏️ The `edit` mode definition — identical for all fifteen apps.
pub fn edit_mode_definition() -> ModeDefinition {
    ModeDefinition { id: MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A norm window kind — both windows of every app are plain `Canvas2d` surfaces with no measures,
/// engagement, actions or utilities, so only id/label/body/icon vary.
pub fn window_definition(id: &str, label: LocalizedLabel, body_key: &str, icon_id: &str) -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: id.into(),
        label,
        body_key: body_key.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: icon_id.into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}

/// 🆔️ Window kind id of one visual window of an artifact variant: `norm-<variant>-<role>`.
pub fn visual_window_id(variant: &str, role: &str) -> String {
    format!("norm-{variant}-{role}")
}

fn view_action(id: &str, en: &str, de: &str, icon: &str, args: Vec<ActionArgDef>) -> semio_plugin::ActionDefinition {
    semio_plugin::ActionDefinition { args, ..semio_plugin::ActionDefinition::new(id, LocalizedLabel::native(en, de), semio_plugin::ActionKind::View, icon) }
}

/// ⌨️ The window actions of the documented keyboard shortcuts, one per distinct shortcut action with the shortcut's English and German label; the shell dispatches a key the norm hook answered with an action to exactly these ids. The focus shortcuts are no app actions: the hook answers them with the framework `focusWindow` verb, which every window kind has through the palette.
pub fn shortcut_action_definitions() -> Vec<semio_plugin::ActionDefinition> {
    let mut seen = std::collections::HashSet::new();
    crate::accessibility::focus_order::shortcuts()
        .into_iter()
        .filter(|shortcut| !matches!(shortcut.action, crate::accessibility::focus_order::ShortcutAction::Focus(_)))
        .filter_map(|shortcut| {
            let id = crate::accessibility::focus_order::shortcut_action_id(&shortcut.action);
            seen.insert(id.clone()).then(|| view_action(&id, &shortcut.label.en, &shortcut.label.de, "keyboard", Vec::new()))
        })
        .collect()
}

/// 🧊️ The 3D viewport window kind: `World3d` surface bound to the `norm.<variant>` interaction domain, with the transform utility and the camera, layer, colour map, section, deformation (amplitude), load-case, active-field and playback actions.
pub fn viewport_window_definition(variant: &str, label: LocalizedLabel, body_key: &str, icon_id: &str) -> WindowKindDefinition {
    use crate::viewport3d::playback::{ACTION_SEEK, ACTION_SELECT_TRACK, ACTION_SET_ACTIVE_FIELD, ACTION_SET_SPEED, ACTION_STEP, ACTION_TOGGLE_PLAY};
    let text = |name: &str, en: &str, de: &str| ActionArgDef::text(name, LocalizedLabel::native(en, de)).required();
    let number = |name: &str, en: &str, de: &str| ActionArgDef::number(name, LocalizedLabel::native(en, de)).required();
    WindowKindDefinition {
        surface_kind: SurfaceKind::World3d,
        utilities: vec![semio_plugin::UtilityRef::new("transform")],
        interactions: vec![semio_plugin::InteractionRef::new(format!("norm.{variant}"))],
        actions: vec![
            view_action("viewportSetCamera", "Set camera", "Kamera setzen", "camera", vec![text("preset", "Preset", "Voreinstellung")]),
            view_action("viewportFrame", "Frame", "Einpassen", "maximize", vec![text("target", "Target (all, selection, worst)", "Ziel (alle, Auswahl, kritischster)")]),
            view_action("viewportToggleLayer", "Toggle layer", "Ebene umschalten", "layers", vec![text("layer", "Layer", "Ebene")]),
            view_action("viewportSetColourMap", "Set colour map", "Farbskala setzen", "palette", vec![text("ramp", "Ramp", "Verlauf")]),
            view_action("viewportSetSection", "Set section", "Schnitt setzen", "scissors", vec![number("offset", "Offset", "Versatz")]),
            view_action("viewportSetDeformationScale", "Set deformation scale", "Verformungsmaßstab setzen", "activity", vec![number("scale", "Scale", "Maßstab")]),
            view_action("viewportSetLoadCase", "Set load case", "Lastfall setzen", "list", vec![text("loadCase", "Load case", "Lastfall")]),
            view_action(ACTION_SET_ACTIVE_FIELD, "Set active field", "Aktives Feld setzen", "gauge", vec![text("field", "Field", "Feld")]),
            view_action(ACTION_SELECT_TRACK, "Select parameter track", "Parameterverlauf wählen", "clock", vec![ActionArgDef::text("track", LocalizedLabel::native("Track", "Verlauf"))]),
            view_action(ACTION_SEEK, "Seek parameter", "Parameter setzen", "sliders-horizontal", vec![number("value", "Value", "Wert")]),
            view_action(ACTION_TOGGLE_PLAY, "Play or pause", "Abspielen oder anhalten", "play", Vec::new()),
            view_action(ACTION_STEP, "Step parameter", "Parameter schrittweise ändern", "skip-forward", vec![number("direction", "Direction", "Richtung")]),
            view_action(ACTION_SET_SPEED, "Set playback speed", "Wiedergabegeschwindigkeit setzen", "gauge", vec![number("speed", "Speed", "Geschwindigkeit")]),
            view_action(crate::TOUCH_MENU_OPEN, "Open edit menu", "Bearbeiten-Menü öffnen", "menu", Vec::new()),
            view_action(crate::TOUCH_MENU_CLOSE, "Close edit menu", "Bearbeiten-Menü schließen", "x", Vec::new()),
            view_action(crate::TOUCH_FRAME_SELECTION, "Frame selection", "Auswahl einpassen", "maximize", Vec::new()),
        ]
        .into_iter()
        .chain(shortcut_action_definitions())
        .collect(),
        ..window_definition(&visual_window_id(variant, "viewport"), label, body_key, icon_id)
    }
}

/// 📈️ The 2D diagram window kind: a `Canvas2d` surface bound to the same interaction domain so a pick in 3D switches the diagrams.
pub fn diagram_window_definition(variant: &str, label: LocalizedLabel, body_key: &str, icon_id: &str) -> WindowKindDefinition {
    WindowKindDefinition {
        interactions: vec![semio_plugin::InteractionRef::new(format!("norm.{variant}"))],
        actions: vec![view_action("diagramSelect", "Select diagram", "Diagramm wählen", "chart-line", vec![ActionArgDef::text("diagram", LocalizedLabel::native("Diagram", "Diagramm")).required()]), view_action("diagramToggleTable", "Toggle table", "Tabelle umschalten", "table", Vec::new())],
        ..window_definition(&visual_window_id(variant, "diagrams"), label, body_key, icon_id)
    }
}

fn layout_window(variant: &str, role: &str) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode { kind: "window".into(), window_kind_id: visual_window_id(variant, role), title: None, instance_id: None, template_id: None, corner: None }
}

fn layout_stack(variant: &str, roles: &[&str], size: Option<f64>) -> WindowLayoutStackNode {
    WindowLayoutStackNode { kind: "stack".into(), size, active_window_kind_id: None, children: roles.iter().map(|role| layout_window(variant, role)).collect() }
}

/// 🧩️ The visual window layout of one device class at the default preferences, see [`visual_layout_with`].
pub fn visual_layout(variant: &str, device: crate::viewport3d::config::DeviceClass) -> WindowLayout {
    visual_layout_with(variant, device, &crate::device_profiles::LayoutPrefs::default())
}

/// 🧩️ The viewer's layout: the model, checks and diagrams of one device class under the user's preferences, see [`visual_layout_with`].
pub fn view_layout_with(variant: &str, device: crate::viewport3d::config::DeviceClass, prefs: &crate::device_profiles::LayoutPrefs) -> WindowLayout {
    visual_layout_roles(variant, device, prefs, &["viewport", "checks", "diagrams"])
}

/// 🧩️ The editor's layout: all five windows of one device class under the user's preferences, see [`visual_layout_roles`].
pub fn visual_layout_with(variant: &str, device: crate::viewport3d::config::DeviceClass, prefs: &crate::device_profiles::LayoutPrefs) -> WindowLayout {
    visual_layout_roles(variant, device, prefs, &["outliner", "viewport", "properties", "checks", "diagrams"])
}

/// 🆔️ Id of the named layout of a device class; a shell that learns the device class from the window size switches to this layout.
pub fn visual_layout_id(device: crate::viewport3d::config::DeviceClass) -> String {
    use crate::viewport3d::config::DeviceClass;
    format!("norm-visual-{}", match device { DeviceClass::Desktop => "desktop", DeviceClass::Mobile => "mobile", DeviceClass::Tablet => "tablet" })
}

fn named_device_layout(device: &str, layout: impl Fn(crate::viewport3d::config::DeviceClass) -> WindowLayout) -> semio_plugin::NamedLayout {
    use crate::viewport3d::config::DeviceClass;
    let (class, en) = match device { "mobile" => (DeviceClass::Mobile, "Mobile"), "tablet" => (DeviceClass::Tablet, "Tablet"), _ => (DeviceClass::Desktop, "Desktop") };
    semio_plugin::create_named_layout(visual_layout_id(class), en, layout(class), "builtin", None, None)
}

/// 📱️ The editor layout of `device` (`desktop`, `mobile` or `tablet`) at the default preferences, declared as a named layout so a shell can switch by device class.
pub fn edit_named_layout(variant: &str, device: &str) -> semio_plugin::NamedLayout {
    named_device_layout(device, |class| visual_layout(variant, class))
}

/// 📱️ The viewer layout of `device` at the default preferences, see [`edit_named_layout`].
pub fn view_named_layout(variant: &str, device: &str) -> semio_plugin::NamedLayout {
    named_device_layout(device, |class| view_layout_with(variant, class, &crate::device_profiles::LayoutPrefs::default()))
}

/// 🧩️ The visual window layout of the `available` windows of one device class under the user's layout preferences, desktop first: a dock of outliner and properties left, viewport centre, checks and diagrams right, hidden windows dropped and shares renormalised; mobile is one tab bar in the user's order with the model active; tablet pairs the viewport with a tabbed side pane. Preferences are sanitised first, so any stored value yields a valid tree.
pub fn visual_layout_roles(variant: &str, device: crate::viewport3d::config::DeviceClass, prefs: &crate::device_profiles::LayoutPrefs, available: &[&str]) -> WindowLayout {
    use crate::viewport3d::config::DeviceClass;
    let prefs = crate::device_profiles::sanitised(prefs);
    let axis = |kind: &str, size: Option<f64>, children: Vec<WindowLayoutChild>| WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: kind.into(), size, children });
    let stack = |roles: &[&str], size: Option<f64>| WindowLayoutChild::Stack(layout_stack(variant, roles, size));
    let root = match device {
        DeviceClass::Desktop => {
            let dock = &prefs.dock;
            let shown = |role: &str| role == "viewport" || (available.contains(&role) && !dock.hidden.iter().any(|h| h == role));
            let column = |pair: [&str; 2], split: f64, size: f64| {
                let parts: Vec<(&str, f64)> = [(pair[0], split), (pair[1], 1.0 - split)].into_iter().filter(|(role, _)| shown(*role)).collect();
                let total: f64 = parts.iter().map(|(_, weight)| weight).sum();
                (!parts.is_empty()).then(|| (size, axis("column", Some(size), parts.iter().map(|(role, weight)| stack(&[*role], Some(weight / total))).collect())))
            };
            let columns: Vec<(f64, WindowLayoutChild)> = [column(["outliner", "properties"], dock.left_split, dock.left), Some((dock.centre, stack(&["viewport"], Some(dock.centre)))), column(["checks", "diagrams"], dock.right_split, dock.right)].into_iter().flatten().collect();
            let total: f64 = columns.iter().map(|(size, _)| size).sum();
            let resize = |share: f64, child: WindowLayoutChild| match child {
                WindowLayoutChild::Axis(node) => WindowLayoutChild::Axis(WindowLayoutAxisNode { size: Some(share), ..node }),
                WindowLayoutChild::Stack(node) => WindowLayoutChild::Stack(WindowLayoutStackNode { size: Some(share), ..node }),
            };
            WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: columns.into_iter().map(|(size, child)| resize(size / total, child)).collect() })
        }
        DeviceClass::Mobile => {
            let tabs: Vec<&str> = prefs.mobile.tabs.iter().map(String::as_str).filter(|tab| available.contains(tab)).collect();
            let active = if tabs.contains(&prefs.mobile.active.as_str()) { prefs.mobile.active.as_str() } else { "viewport" };
            WindowLayoutRoot::Stack(WindowLayoutStackNode { active_window_kind_id: Some(visual_window_id(variant, active)), ..layout_stack(variant, &tabs, None) })
        }
        DeviceClass::Tablet => {
            let tabs: Vec<&str> = prefs.tablet.side_tabs.iter().map(String::as_str).filter(|tab| available.contains(tab)).collect();
            let active = if tabs.contains(&prefs.tablet.side_active.as_str()) { prefs.tablet.side_active.as_str() } else { tabs.first().copied().unwrap_or("checks") };
            let side = WindowLayoutStackNode { active_window_kind_id: Some(visual_window_id(variant, active)), ..layout_stack(variant, &tabs, Some(1.0 - prefs.tablet.viewport)) };
            WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![stack(&["viewport"], Some(prefs.tablet.viewport)), WindowLayoutChild::Stack(side)] })
        }
    };
    WindowLayout { root }
}

/// 📌️ A norm panel tab — every one is a framework-predefined leaf id bound to this app's body key.
/// Byte-identical to the `AppBuilder::panel_tab(id, label, group, body_key)` scalar call it replaces
/// (`PanelTabSpec::leaf` builds exactly this shape).
pub fn panel_definition(id: &str, label: LocalizedLabel, group: PanelGroup, body_key: &str) -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(id.into()), label, group, body_key: Some(body_key.into()), children: Vec::new() }
}

/// 🗿️ A norm artifact kind — Data × Value document per owner-table (IO coverage lattice). `artifact_schema` is the
/// family's DOCUMENT_SCHEMA — the ONE schema identity the hub's codec rows, open targets and genesis key on (the same
/// value [`norm_io`] declares); the former `norm.<variant>.document` media string stays declared as `source_format`.
pub fn artifact_kind_spec(variant: &str, label: &str, artifact_schema: &str) -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: artifact_kind_id(variant),
        label: LocalizedLabel::data(label),
        source_format: format!("norm.{variant}.document"),
        component_kind: "norm".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: artifact_schema.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}

/// 🆔️ `computation.norm.<variant>` — the artifact kind id `report:out` pins itself to.
pub fn artifact_kind_id(variant: &str) -> String {
    format!("computation.norm.{variant}")
}

/// 🔌️ Every norm family's typed media I/O surface — the implicit `document:in`/`document:out` pair
/// (auto-injected by `AppIo::all_ports`) plus the two extra workflow ports every norm app gets:
/// `model:in` (a generic upstream-model input — an honest pass-through, no family `Document` shape has
/// a generic "raw model" field to receive one into yet) and `report:out` (the computed `CheckReport`,
/// pinned to this family's own already-declared `computation.norm.{variant}` artifact kind via
/// `kind_id`). One function serves both the builder's `.io(...)` declaration and each app's
/// `ArtifactApp::io` override, so the two never drift apart.
pub fn norm_io(variant: &str, artifact_schema: &str) -> AppIo {
    let artifact_kind_id = artifact_kind_id(variant);
    AppIo {
        artifact_schema: artifact_schema.into(),
        artifact_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![
            MediaPortSpec {
                id: "model:in".into(),
                label: LocalizedLabel::native("Model", "Modell"),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                kind_id: None,
                required: false,
                multiplicity: PortMultiplicity::One,
            },
            MediaPortSpec {
                id: "report:out".into(),
                label: LocalizedLabel::native("Report", "Bericht"),
                direction: MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
                kind_id: Some(artifact_kind_id.clone()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: ArtifactPresentation { id: artifact_kind_id, name: variant.into(), dimension: "data".into(), component_kind: "norm".into() },
    }
}
//#endregion 🔖️Manifest

//#region 🔖️MediaPorts
/// 🎞️ `"report:out"` dumps the currently computed `CheckReport`, pinned to this family's declared
/// artifact kind; `"artifact:out"` replicates the SDK default (whole-document pack) since overriding
/// `export_media` shadows it entirely. Any other port is `NotImplemented`.
pub fn export_media<F>(port: &str, variant: &str, artifact_schema: &str, document: &F::Document) -> Result<Media, MediaError>
where
    F: NormFamily,
    F::Document: pack::snapshot::ArtifactPack + ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot,
{
    if port == "report:out" {
        let report = F::live_report(document);
        let json = semio_framework_pack_json::to_json_string(&*report);
        return Ok(Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: artifact_kind_id(variant), json } });
    }
    if port != "artifact:out" {
        return Err(MediaError::NotImplemented);
    }
    let bytes = pack::snapshot::ArtifactPack::encode_pack(document);
    Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: artifact_schema.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
}

/// 🎞️ `"model:in"` is an honest generic pass-through: no family `Document` has a generic "raw model" field to receive one, so the
/// payload is accepted but inert. A whole document never arrives as a mutation: `"artifact:in"` is refused because importing a
/// whole document is the shell's document load path (genesis/load, not a history row).
pub fn import_media<M>(port: &str, _media: &Media) -> Result<Emit<M, NoConfigMutation>, MediaError> {
    match port {
        "model:in" => Ok(Emit::default()),
        "artifact:in" => Err(MediaError::Payload(port.to_string(), "a whole document is loaded by the shell's document load path, never imported as mutations".into())),
        _ => Err(MediaError::NotImplemented),
    }
}
//#endregion 🔖️MediaPorts
