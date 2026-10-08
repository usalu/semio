//! 🧰️ BIM editor kit: the UI admission helpers every taxonomy node shares, the id mint, and the window-config declaration macro.

use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::ActionId;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::DslValue;
use semio_framework_ui_locale::Label;

//#region 🔖️Constants
pub const BIM_EDITOR_CONTROLLER_ID: &str = "s.bim.model@1/*#editor";
//#endregion 🔖️Constants

//#region 🔖️Actions
/// 🎯️ An action descriptor addressed at this app: the one factory every panel, table and window chrome builds its actions with.
pub fn bim_action(action: &str, args: Option<UiValue>) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
    semio_framework_plugin::ActionFactory::new(BIM_EDITOR_CONTROLLER_ID).action(action, args)
}

/// 🎛️ Addresses host window chrome with its native action descriptor.
pub fn bim_window_action(action: &str, args: Option<semio_framework_plugin::DslValue>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: BIM_EDITOR_CONTROLLER_ID.into(), action: action.into(), args }
}
//#endregion 🔖️Actions

//#region 🔖️Admission
/// 🚧️ Reports fixed-capacity UI admission failure.
pub fn ui_capacity_error() -> PluginAssemblyError {
    PluginAssemblyError::new("bim.ui.capacity", "bim UI admission failed")
}

/// 🏷️ Admits localized text into the semantic UI contract.
pub fn ui_label(value: &str) -> UiAssemblyResult<semio_framework_ui_contract::Label> {
    value.try_into().map_err(|_| PluginAssemblyError::new("ui.label.capacity", "fixed UI label admission failed"))
}

/// 📝️ Admits a fixed UI string.
pub fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<semio_framework_ui_contract::UiText> {
    semio_framework_ui_contract::UiText::try_from_str(value.as_ref()).ok_or_else(ui_capacity_error)
}

/// 🧱️ Admits one fixed UI text action value.
pub fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 📚️ Admits one fixed UI list action value.
pub fn ui_value_list(values: impl IntoIterator<Item = UiValue>) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}

/// 🌳️ A tree row with an icon and a click action.
pub fn tree_item_with_icon(id: impl AsRef<str>, label: impl TryInto<Label>, icon_id: &str, action: UiAssemblyResult<(ActionId, Option<UiValue>)>) -> UiAssemblyResult<BuiltNode> {
    let label: Label = label.try_into().map_err(|_| PluginAssemblyError::new("ui.tree-item.label", "tree-item label conversion failed"))?;
    let mut node = tree_item_with_action(id, label.as_str(), None, action?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon_id).ok_or_else(|| PluginAssemblyError::new("ui.tree-item.icon", "fixed tree-item icon admission failed"))?);
    }
    Ok(node)
}
//#endregion 🔖️Admission

//#region 🔖️Faults
/// 🚫️ A refusal of this app with a stable code.
pub fn fault(code: &'static str, message: impl Into<String>) -> semio_framework_plugin::Fault {
    semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}

/// 🕹️ Requests the framework's own `interactionSelect` for `targets` (`(granularity, id)` pairs) in `domain`, merged as `merge`: selection is framework state, never a mutation.
pub fn select_effect(domain: &str, targets: &[(String, String)], merge: &str) -> semio_framework_plugin::Effect {
    let items = targets.iter().map(|(granularity, id)| DslValue::object([("granularity".to_string(), DslValue::String(granularity.clone())), ("id".to_string(), DslValue::String(id.clone()))])).collect();
    semio_framework_plugin::Effect::ReplayShellCommand {
        action_id: semio_framework::INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(DslValue::object([
            ("domainId".to_string(), DslValue::String(domain.to_string())),
            ("targets".to_string(), DslValue::String(semio_framework_pack_json::to_json_string(&DslValue::Array(items)))),
            ("merge".to_string(), DslValue::String(merge.to_string())),
            ("method".to_string(), DslValue::String("pick".to_string())),
        ])),
    }
}
//#endregion 🔖️Faults

//#region 🔖️Identity
/// 🪪️ Mints element ids from the authoring seed of the admitted command, so two writers (or two sessions of one) never mint the same id at one base;
/// an ordinal keeps the ids of one command apart from each other and from every id already in the document.
pub struct IdMint {
    seed: String,
    minted: Vec<String>,
}

impl IdMint {
    pub fn new(operation: Option<&semio_framework_plugin::AppOperationContext>) -> Self {
        Self::seeded(operation.map_or("", |operation| operation.authoring_seed.as_str()))
    }

    pub fn seeded(authoring_seed: &str) -> Self {
        Self { seed: authoring_seed.chars().filter(char::is_ascii_alphanumeric).take(8).collect(), minted: Vec::new() }
    }

    pub fn mint(&mut self, prefix: &str, taken: impl Fn(&str) -> bool) -> String {
        let mut ordinal = self.minted.len();
        loop {
            let id = if self.seed.is_empty() { format!("{prefix}-{ordinal}") } else { format!("{prefix}-{}-{ordinal}", self.seed) };
            if !taken(&id) && !self.minted.contains(&id) {
                self.minted.push(id.clone());
                return id;
            }
            ordinal += 1;
        }
    }
}
//#endregion 🔖️Identity

//#region 🔖️CanvasRecords
/// 🎨️ A linear RGBA colour, components in 0..1.
pub type Rgba = [f64; 4];

/// 🖌️ How one canvas record is painted: an optional fill and an optional stroke of `(colour, width in metres, dash)`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Paint {
    pub fill: Option<Rgba>,
    pub stroke: Option<(Rgba, f64)>,
    pub dash: Option<[f64; 2]>,
}

fn float(value: f64) -> DslValue {
    DslValue::float(value)
}

fn colour(value: Rgba) -> DslValue {
    DslValue::Array(value.iter().map(|component| float(*component)).collect())
}

fn text(value: &str) -> DslValue {
    DslValue::String(value.to_string())
}

fn segment(kind: &str, at: Option<(f64, f64)>) -> DslValue {
    let mut entries = vec![("kind".to_string(), text(kind))];
    if let Some((x, y)) = at {
        entries.push(("to".to_string(), DslValue::Array(vec![float(x), float(y)])));
    }
    DslValue::object(entries)
}

/// 📍️ A path of the points `ring`: move to the first, line to the rest, closed when `closed`.
pub fn path(points: &[[f64; 2]], closed: bool) -> Vec<DslValue> {
    let mut segments: Vec<DslValue> = points.iter().enumerate().map(|(index, [x, y])| segment(if index == 0 { "move" } else { "line" }, Some((*x, *y)))).collect();
    if closed && !points.is_empty() {
        segments.push(segment("close", None));
    }
    segments
}

/// 🖼️ One canvas path record in paint order; coordinates are already in the window's flipped logical space.
pub fn path_record(id: &str, role: &str, segments: Vec<DslValue>, paint: Paint) -> DslValue {
    let stroke = paint.stroke.map_or(DslValue::Null, |(rgba, width)| {
        let mut entries = vec![("color".to_string(), colour(rgba)), ("width".to_string(), float(width)), ("cap".to_string(), text("round")), ("join".to_string(), text("round"))];
        if let Some([on, off]) = paint.dash {
            entries.push(("dash".to_string(), DslValue::Array(vec![float(on), float(off)])));
        }
        DslValue::object(entries)
    });
    DslValue::object([
        ("id".to_string(), text(id)),
        ("role".to_string(), text(role)),
        ("transform".to_string(), DslValue::Array([1.0, 0.0, 0.0, 1.0, 0.0, 0.0].iter().map(|component| float(*component)).collect())),
        ("segments".to_string(), DslValue::Array(segments)),
        ("fill".to_string(), paint.fill.map_or(DslValue::Null, |rgba| DslValue::object([("kind".to_string(), text("solid")), ("color".to_string(), colour(rgba))]))),
        ("stroke".to_string(), stroke),
        ("opacity".to_string(), float(1.0)),
        ("blendMode".to_string(), text("normal")),
        ("visible".to_string(), DslValue::Bool(true)),
        ("fillRule".to_string(), text("evenodd")),
    ])
}

/// 🔤️ One canvas text record anchored at `(x, y)`, `size` metres tall.
pub fn text_record(id: &str, at: (f64, f64), content: &str, size: f64, rgba: Rgba) -> DslValue {
    DslValue::object([
        ("id".to_string(), text(id)),
        ("role".to_string(), text("overlay")),
        ("transform".to_string(), DslValue::Array([1.0, 0.0, 0.0, 1.0, at.0, at.1].iter().map(|component| float(*component)).collect())),
        ("segments".to_string(), DslValue::Array(Vec::new())),
        ("fill".to_string(), DslValue::object([("kind".to_string(), text("solid")), ("color".to_string(), colour(rgba))])),
        ("opacity".to_string(), float(1.0)),
        ("blendMode".to_string(), text("normal")),
        ("visible".to_string(), DslValue::Bool(true)),
        ("text".to_string(), DslValue::object([("content".to_string(), text(content)), ("size".to_string(), float(size))])),
    ])
}

/// 🧰️ The leading meta record that tells the host which utility is armed.
pub fn meta_record(utility: &str) -> DslValue {
    DslValue::object([("id".to_string(), text("meta:utility")), ("role".to_string(), text("meta")), ("utility".to_string(), text(utility))])
}

/// 🖼️ Publishes the records as the window's canvas surface, framed on `bounds` until the author has navigated.
pub fn canvas_surface(surface_id: &str, viewport: store::Viewport2d, framing: Option<semio_framework_plugin::Canvas2dFraming>, records: &[DslValue]) -> UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::scene_surface(
        surface_id,
        semio_framework_ui_contract::SurfaceKind::Canvas2d,
        &semio_framework_plugin::Canvas2dScene { framing, camera_x: viewport.x, camera_y: viewport.y, zoom: viewport.zoom, layers_json: semio_framework_pack_json::to_json_string(&records.to_vec()), snapshot: None, tool_run_trace: None, lanes: Vec::new() },
    )
}
//#endregion 🔖️CanvasRecords

//#region 🔖️WindowConfig
/// 🎚️ Declares the persisted-local state of one window kind from its field table: the record with its default, then the shared `crate::bim_window_config!` for the codecs, the
/// whole-record `Snapshot` mutation and the `WindowConfigOwner` with `current`, `from_snapshot` and `addressed`, and the owner's `register`.
macro_rules! window_config {
    (
        window: $window:expr,
        schema: $schema:literal,
        envelope: $envelope:literal,
        extension: $extension:literal,
        owner_path: $owner_path:literal,
        display: $display:literal,
        type $Config:ident, $Mutation:ident, $Owner:ident;
        $( $(#[$attr:meta])* $field:ident : $ty:ty = $default:expr; )+
    ) => {
        #[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
        #[value(rename_all = "camelCase", deny_unknown_fields)]
        #[dsl(layout = "lines")]
        #[artifact(id = $envelope, extension = $extension)]
        pub struct $Config {
            $( $(#[$attr])* pub $field: $ty, )+
        }

        impl Default for $Config {
            fn default() -> Self {
                Self { $( $field: $default, )+ }
            }
        }

        crate::bim_window_config! { config: $Config, mutation: $Mutation, owner: $Owner, window: $window, schema: $schema, owner_path: $owner_path, display: $display, bytes: 4_096 }

        pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
            registry.register::<$Owner>()
        }
    };
}

pub(crate) use window_config;
//#endregion 🔖️WindowConfig
