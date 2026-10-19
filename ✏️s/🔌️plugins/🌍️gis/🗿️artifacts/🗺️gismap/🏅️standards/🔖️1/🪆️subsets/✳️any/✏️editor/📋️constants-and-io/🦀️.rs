//! 📋️ Constants and typed media I/O of the GIS 2D play app: the app id, the interaction domain and snapshot, the layer ids, the UI value helpers and the I/O ports. See [`super`].

use crate::tile_coordinates::gis_policy::GIS_MAP_LAYER_KEYS;
use crate::{GisMapSnapshot, MapFeature, GIS_MAP_SCHEMA};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::tree_item;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_ui_locale::Label;

//#region 🔖️Constants
pub const GIS2D_PLAY_APP_ID: &str = "gis2d-play";

/// 🕹️ The framework-owned interaction domain this app declares (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — the document tree picks layers in it and
/// `TiledMapHost` picks map features in it.
pub const GIS2D_INTERACTION_DOMAIN: &str = "features";

/// 🪜️ The map granularity of [`GIS2D_INTERACTION_DOMAIN`] — a position or route id.
pub const GIS2D_FEATURE_GRANULARITY: &str = "feature";

/// 🪜️ The document-tree granularity of [`GIS2D_INTERACTION_DOMAIN`] — a [`GIS_MAP_LAYER_IDS`] id.
pub const GIS2D_LAYER_GRANULARITY: &str = "layer";

/// 🕹️ Owned snapshot of the framework's `"features"` domain — `InteractionView::selection` plus the
/// `"pointer"` hover — read once per render by `ArtifactApp::render_with_request_context` and threaded
/// into the map scene (selected/hovered feature paint) and the inspection panel (the selected layer's
/// or feature's own fields). Decouples both from `semio_framework_plugin::app::InteractionView`, whose
/// fields are `pub(crate)` to that crate, so this crate's tests construct this plain struct instead
/// (mirrors `📐️cad`'s `CadInteractionSnapshot`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Gis2dInteractionSnapshot {
    pub granularity: String,
    pub ids: Vec<String>,
    pub hovered_ids: Vec<String>,
}

impl Gis2dInteractionSnapshot {
    /// 🐁️ The hover channel `TiledMapHost` publishes feature hover on.
    pub const POINTER_CHANNEL: &'static str = "pointer";

    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let selection = interaction.selection(GIS2D_INTERACTION_DOMAIN);
        Self {
            granularity: selection.granularity.clone(),
            ids: selection.ids.clone(),
            hovered_ids: interaction.hover(GIS2D_INTERACTION_DOMAIN, Self::POINTER_CHANNEL).ids.clone(),
        }
    }

    /// 🗂️ The selected layer id, `None` unless the live granularity is `"layer"` — the inspector's
    /// per-layer detail branch.
    pub fn selected_layer(&self) -> Option<&str> {
        (self.granularity == GIS2D_LAYER_GRANULARITY).then(|| self.ids.first().map(String::as_str)).flatten()
    }

    /// 🗺️ Splits the selected feature ids into the `{positions, routes}` shape `TiledMapHost`'s
    /// `parseFeatureSelection` reads, by membership in the document itself — an id the document does
    /// not carry belongs to neither bag and is dropped.
    pub fn feature_selection_json(&self, document: &GisMapSnapshot) -> String {
        let bag = |features: &[MapFeature]| -> Vec<&str> { self.ids.iter().map(String::as_str).filter(|id| features.iter().any(|feature| feature.id == *id)).collect() };
        serde_json::json!({ "positions": bag(&document.positions), "routes": bag(&document.routes) }).to_string()
    }

    /// 🐁️ The `{kind, id}` hover shape `TiledMapHost`'s `parseMapHoveredFeature` reads, `"null"` when
    /// nothing in this domain is hovered or the hovered id is not a document feature.
    pub fn feature_hover_json(&self, document: &GisMapSnapshot) -> String {
        let Some(id) = self.hovered_ids.first() else { return "null".to_string() };
        let kind = if document.routes.iter().any(|route| &route.id == id) {
            "route"
        } else if document.positions.iter().any(|position| &position.id == id) {
            "position"
        } else {
            return "null".to_string();
        };
        serde_json::json!({ "kind": kind, "id": id }).to_string()
    }
}

/// 📏️ The default extent, in degrees, of the geometry `addFeature` mints for a route or a region —
/// roughly a kilometre at mid latitude, so a freshly added feature is visible at the default camera
/// instead of collapsing to a point.
pub const GIS2D_DEFAULT_FEATURE_SPAN: f64 = 0.01;

/// 🗂️ The app-wide map layer stack: `(id, native English name, icon id)?`, one row per GIS policy toggle
/// key ([`GIS_MAP_LAYER_KEYS`]). Every chrome node (document and catalogue trees, the layers/weights
/// window options, the inspector summary) enumerates it.
pub const GIS_MAP_LAYER_IDS: &[(&str, &str, &str)] = &[
    (GIS_MAP_LAYER_KEYS[0], "Raster", "layers"),
    (GIS_MAP_LAYER_KEYS[1], "Water", "cloud"),
    (GIS_MAP_LAYER_KEYS[2], "Land", "trees"),
    (GIS_MAP_LAYER_KEYS[3], "Roads", "compass"),
    (GIS_MAP_LAYER_KEYS[4], "Buildings", "building"),
    (GIS_MAP_LAYER_KEYS[5], "Borders", "square-dashed"),
    (GIS_MAP_LAYER_KEYS[6], "Labels", "type"),
    (GIS_MAP_LAYER_KEYS[7], "Positions", "crosshair"),
    (GIS_MAP_LAYER_KEYS[8], "Position Labels", "tags"),
    (GIS_MAP_LAYER_KEYS[9], "Routes", "git-branch"),
    (GIS_MAP_LAYER_KEYS[10], "Regions", "layers"),
];

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`) builds its `on_change`/item actions with.
pub fn gis2d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(GIS2D_PLAY_APP_ID).action(action, args)
}

/// 🪟️ Bridges semantic app actions into the retained window-measure transport.
pub fn gis2d_window_action(action: &str, args: Option<semio_framework::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: GIS2D_PLAY_APP_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits resolved app text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "GIS UI label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ A layer tree item — `tree_item_with_action`/`tree_item` plus the icon that identifies each map
/// layer, since the SDK's `PanelKit` family has no icon-carrying constructor. Shared by the document
/// panel (`action: None`, `granularity: Some(GIS2D_LAYER_GRANULARITY)` — the tree is
/// `interaction_domain`-bound, so the row declares its pick and the tree's single `interactionSelect`
/// carries it) and the catalogue panel (`action: Some(..)`, `granularity: None` — a real,
/// non-selection click that toggles layer visibility in an unbound tree).
pub fn gis2d_layer_tree_item(
    id: impl AsRef<str>,
    label: semio_framework_ui_contract::Label,
    description: Option<String>,
    icon_id: &str,
    action: Option<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)>,
    granularity: Option<&str>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut node = match action {
        Some(action) => tree_item_with_action(id, label, description.clone(), action)?,
        None => tree_item(id, label)?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        if props.description.is_none() {
            props.description = match description {
                Some(value) => Some(semio_framework_plugin::UiText::try_from_string(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "gis layer description admission failed"))?),
                None => None,
            };
        }
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "gis layer icon admission failed"))?);
        props.granularity = match granularity {
            Some(value) => Some(semio_framework_plugin::UiText::try_from_str(value).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "gis layer granularity admission failed"))?),
            None => None,
        };
    }
    Ok(node)
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` `crate::standards::v1::subsets::any::artifact_declaration::artifact_kind()`
/// declares (schema/media type/export+import formats/presentation fields copied verbatim), plus the
/// two app-specific workflow ports (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE
/// Wave 2 port recipe): `features:in` (any TwoD×Vector producer feeds new/patched
/// positions/routes/regions) and `map:out` (this document's own feature layers, the `2d.map`
/// interchange kind gis3d's `map:in` consumes).
pub fn gis2d_io() -> AppIo {
    AppIo {
        artifact_schema: GIS_MAP_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![gis2d_features_in_port(), gis2d_map_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: crate::GISMAP_DIALECT.artifact_kind.into(), name: "2D Map".into(), dimension: "2d".into(), component_kind: "gismap".into() },
    }
}

/// 🔌️ `features:in` — accepts any TwoD×Vector producer (draw's `vector:out`, another gis2d's
/// `map:out`, …); no `kind_id` pin since it's a generic vector-features sink, not one specific kind.
/// `Many`/optional: several producers may fan into one map, and a map with no upstream edge is valid.
pub fn gis2d_features_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "features:in".into(),
        label: "Features".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: None,
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🔌️ `map:out` — this document's positions/routes/regions as the `2d.map` interchange kind (gis3d's
/// `map:in` consumes it). `Many`/optional: several downstream consumers may fan out from one map, and a
/// map with no downstream edge is valid.
pub fn gis2d_map_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "map:out".into(),
        label: "Map".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: Some(crate::GISMAP_DIALECT.artifact_kind.into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🎞️ Map transport carries complete first-party feature values with exact tags and order.
pub fn gis2d_map_media(document: &GisMapSnapshot) -> Media {
    let value=semio_framework_value::DslValue::Object([("positions",&document.positions),("routes",&document.routes),("regions",&document.regions)].into_iter().map(|(name,features)|(name.into(),semio_framework_value::DslValue::Array(features.iter().map(|feature|feature.data.clone()).collect()))).collect());
    Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Intrinsic { schema: "2d.map".into(), value } }
}
//#endregion 🔖️Io

//#region 🧩️ArtifactItems
pub mod artifact_items {
//#region 🔹Constants

pub const GIS_MAP_SCHEMA: &str = "gis.map";

/// 🪪️ One canonical map identity shared by definition, composer and both app roles.
pub const GISMAP_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.gis.gismap", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };
//#endregion 🔹Constants
}
//#endregion 🧩️ArtifactItems
