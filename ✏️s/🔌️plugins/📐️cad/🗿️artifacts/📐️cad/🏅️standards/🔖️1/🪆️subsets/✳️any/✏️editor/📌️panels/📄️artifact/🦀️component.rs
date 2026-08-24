//! 📄️ CAD play app panel — the document tree: every pane's objects (with their primitive children)
//! and reference overlays, plus the scene's nodes.

use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::CadObject;
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC};
use crate::artifacts::cad::{CadPaneId, CadReference, CadSnapshot};
use crate::editor::cad::terminology::{typology_label, CadLabels};
use crate::editor::cad::{cad_action, cad_tree_item, ui_node_list, ui_value_bool, ui_value_list, ui_value_map, ui_value_text, CadPlayRuntime, CadPlayView};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, BuiltNode, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const CAD_PLAY_BODY_DOCUMENT: &str = "cad.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(CAD_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub(crate) async fn object_tree_item(id_suffix: &str, object: &CadObject, labels: &CadLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let primitive_items = ui_node_list(object.primitives.iter().map(|primitive| {
            // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `UiTreeItemNode` no longer
            // carries `hoverAction`/`unhoverAction` — mesh hover is the framework-owned `"cad"`
            // domain now. This tree stays un-bound to `interaction_domain` (see
            // `document_tree_selected_ids`'s doc comment), so the click action below is a
            // pending-domain-binding placeholder — already a documented no-op path today, since
            // `build_document_tree` renders every pane's object section empty (UNIFIED-COMPOSABLE-
            // ARTIFACT-SYSTEM gap).
            let args = ui_value_map([("modelDefinitionId", ui_value_text(id_suffix)?)])?;
            cad_tree_item(
                format!("cad-primitive:{id_suffix}:{}:{}", object.id, primitive.primitive_id),
                Label::data(format!("{}: {}", primitive.slot, primitive.primitive_id)),
                Some("hexagon"),
                cad_action("focusModelDefinition", Some(args))?,
            )
        }))?;
    let args = ui_value_map([("modelDefinitionId", ui_value_text(id_suffix)?)])?;
    let mut item = cad_tree_item(format!("cad-object:{id_suffix}:{}", object.id), Label::data(object.label.clone()), Some("box"), cad_action("focusModelDefinition", Some(args))?)?;
    item.base.children = primitive_items;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        if !object.typology.is_empty() {
            props.description = Some(UiText::try_from_string(typology_label(&object.typology, labels).to_string()).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad object description admission failed"))?);
        }
        props.dimmed = Some(!object.visible);
        props.draggable = Some(!object.locked);
        props.default_open = Some(false);
        let mut row_actions = UiFixedList::default();
        let specs = [
            (if object.visible { "eye-off" } else { "eye" }, if object.visible { labels.hide } else { labels.show }, "patchObject", Some(ui_value_map([("objectId", ui_value_text(&object.id)?), ("field", ui_value_text("hidden")?), ("value", ui_value_bool(object.visible))])?), RowActionPlacement::Row),
            (if object.locked { "unlock" } else { "lock" }, if object.locked { labels.unlock } else { labels.lock }, "patchObject", Some(ui_value_map([("objectId", ui_value_text(&object.id)?), ("field", ui_value_text("locked")?), ("value", ui_value_bool(!object.locked))])?), RowActionPlacement::Row),
            ("copy", labels.duplicate, "duplicateObject", Some(ui_value_map([("objectId", ui_value_text(&object.id)?)])?), RowActionPlacement::Menu),
            ("trash-2", labels.delete, "deleteObject", Some(ui_value_map([("objectId", ui_value_text(&object.id)?)])?), RowActionPlacement::Menu),
        ];
        for (icon, label, action, args, placement) in specs {
            let (action, args) = cad_action(action, args)?;
            let row_action = RowAction {
                icon: UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad object row action icon admission failed"))?,
                label: Some(label.as_str().into()),
                action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
                placement,
            };
            row_actions.try_push(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad object row action admission failed"))?;
        }
        props.row_actions = row_actions;
    }
    Ok(item)
}

pub async fn reference_tree_item(model_definition_id: &str, reference: &CadReference, labels: &CadLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let select_args = ui_value_map([
        ("modelDefinitionId", ui_value_text(model_definition_id)?),
        ("referenceId", ui_value_text(&reference.id)?),
    ])?;
    let mut item = cad_tree_item(
        format!("cad-reference:{model_definition_id}:{}", reference.id),
        Label::data(reference.id.clone()),
        Some("image"),
        cad_action("setReferenceSelection", Some(select_args))?,
    )?;
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `UiTreeItemNode` no longer carries
    // `hoverAction`/`unhoverAction` — no generic tree-hover mechanism replaces it for a non-
    // `interaction_domain`-bound tree; `referenceHover` stays reachable from the World3d surface only.
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.description = Some(UiText::try_from_str(&reference.source_url).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad reference description admission failed"))?);
        props.dimmed = Some(reference.hidden);
        let specs = [
            (if reference.hidden { "eye" } else { "eye-off" }, if reference.hidden { labels.show } else { labels.hide }, "hidden", !reference.hidden),
            (if reference.locked { "unlock" } else { "lock" }, if reference.locked { labels.unlock } else { labels.lock }, "locked", !reference.locked),
        ];
        let mut row_actions = UiFixedList::default();
        for (icon, label, field, value) in specs {
            let args = ui_value_map([
                ("modelDefinitionId", ui_value_text(model_definition_id)?),
                ("referenceId", ui_value_text(&reference.id)?),
                ("field", ui_value_text(field)?),
                ("value", ui_value_bool(value)),
            ])?;
            let (action, args) = cad_action("patchCadPlayReference", Some(args))?;
            let row_action = RowAction {
                icon: UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad reference row action icon admission failed"))?,
                label: Some(label.as_str().into()),
                action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
                placement: RowActionPlacement::Row,
            };
            row_actions.try_push(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad reference row action admission failed"))?;
        }
        props.row_actions = row_actions;
    }
    Ok(item)
}

/// 🗂️ The `document.references_by_model_definition_id` lookup repeated once per pane in `build_document_tree`.
pub async fn references_for<'a>(document: &'a CadSnapshot, model_definition_id: &str) -> &'a [CadReference] {
    document.references_by_model_definition_id.get(model_definition_id).map_or(&[][..], |rows| rows.as_slice())
}

/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): only reference-overlay selection is
/// resolved here now — mesh object/primitive selection is the framework-owned `"cad"` interaction
/// domain, unreachable at this `render` boundary (no `InteractionView` parameter; see
/// `edit::instance_is_component_hovered`'s doc comment). This tree stays un-bound to
/// `interaction_domain` for that reason (its item ids, `"cad-object:…"`/`"cad-reference:…"`/
/// `"cad-node:…"`, are UI-namespaced composites, not the domain's raw ids anyway).
pub async fn document_tree_selected_ids(_document: &CadSnapshot, runtime: &CadPlayRuntime) -> semio_framework_plugin::UiAssemblyResult<Option<UiFixedList<String>>> {
    if let (Some(model_definition_id), Some(reference_id)) = (runtime.selected_reference_model_definition_id.as_deref(), runtime.selected_reference_id.as_deref()) {
        let mut ids = UiFixedList::default();
        ids.try_push(format!("cad-reference:{model_definition_id}:{reference_id}"))
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad selected id admission failed"))?;
        return Ok(Some(ids));
    }
    Ok(None)
}

pub async fn document_tree_highlighted_ids(document: &CadSnapshot, runtime: &CadPlayRuntime) -> semio_framework_plugin::UiAssemblyResult<Option<UiFixedList<String>>> {
    let Some(hovered) = runtime.hovered_reference_id.as_deref() else {
        return Ok(None);
    };
    for pane in CadPaneId::all() {
        let model_definition_id = pane.model_definition_id();
        if document.references_by_model_definition_id.get(model_definition_id).is_some_and(|rows| rows.iter().any(|row| row.id == hovered)) {
            let mut ids = UiFixedList::default();
            ids.try_push(format!("cad-reference:{model_definition_id}:{hovered}"))
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad highlighted id admission failed"))?;
            return Ok(Some(ids));
        }
    }
    Ok(None)
}

/// 🌳️ One pane's object section: namespaced by `id_suffix`, always expanded.
pub(crate) async fn document_pane_section(
    label: impl Into<Label>,
    id_suffix: &str,
    objects: &[CadObject],
    labels: &CadLabels,
) -> semio_framework_plugin::UiAssemblyResult<(String, Option<Label>, bool, UiFixedList<BuiltNode>)> {
    Ok((format!("cad-play-document.{id_suffix}"), Some(label.into()), true, ui_node_list(objects.iter().map(|object| object_tree_item(id_suffix, object, labels)))?))
}

/// 🌳️ One pane's references section: collapsed by default, "(none)"-placeholder when empty.
pub async fn artifact_references_section(
    document: &CadSnapshot,
    model_definition_id: &str,
    labels: &CadLabels,
) -> semio_framework_plugin::UiAssemblyResult<(String, Option<Label>, bool, UiFixedList<BuiltNode>)> {
    Ok((
        format!("cad-play-document.references.{model_definition_id}"),
        Some(labels.references.into()),
        false,
        ui_node_list(references_for(document, model_definition_id).iter().map(|reference| reference_tree_item(model_definition_id, reference, labels)))?,
    ))
}

pub async fn build_document_tree(envelope: &CadPlayView, labels: &CadLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let node_items = ui_node_list(envelope.document.nodes.iter().map(|node| {
        let node_ids = ui_value_list([ui_value_text(&node.id)?])?;
        let args = ui_value_map([("nodeIds", node_ids)])?;
        cad_tree_item(format!("cad-node:{}", node.id), Label::data(node.label.clone()), Some("git-branch"), cad_action("setNodeSelection", Some(args))?)
    }))?;

    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `CadSnapshot`'s inline
    // per-pane object lists are gone (composed `s.stdio.semio.model` CHILD documents now, unresolved
    // at this render boundary — see `🔖️Composition` in `🏪️store/🦀️component.rs`). Each pane
    // section renders empty until a resolved-child-content render path exists; documented gap.
    let (shape_id, shape_label, shape_open, shape_items) = document_pane_section(labels.pane_shape, "shape", &[], labels)?;
    let (shape_refs_id, shape_refs_label, shape_refs_open, shape_refs_items) = artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_SHAPE, labels)?;
    let (building_id, building_label, building_open, building_items) = document_pane_section(labels.pane_building, "building", &[], labels)?;
    let (building_refs_id, building_refs_label, building_refs_open, building_refs_items) = artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_BUILDING, labels)?;
    let (energy_id, energy_label, energy_open, energy_items) = document_pane_section(labels.pane_energy, "energy", &[], labels)?;
    let (energy_refs_id, energy_refs_label, energy_refs_open, energy_refs_items) = artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_ENERGY, labels)?;
    let (structure_id, structure_label, structure_open, structure_items) = document_pane_section(labels.pane_structure_classic, "structure-classic", &[], labels)?;
    let (structure_refs_id, structure_refs_label, structure_refs_open, structure_refs_items) = artifact_references_section(&envelope.document, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, labels)?;

    let mut builder = PanelTreeBuilder::new("cad-play-document")?
        .section(shape_id, shape_label, shape_open, shape_items)?
        .section_or_placeholder(shape_refs_id, shape_refs_label, shape_refs_open, shape_refs_items, labels.none_placeholder)?
        .section(building_id, building_label, building_open, building_items)?
        .section_or_placeholder(building_refs_id, building_refs_label, building_refs_open, building_refs_items, labels.none_placeholder)?
        .section(energy_id, energy_label, energy_open, energy_items)?
        .section_or_placeholder(energy_refs_id, energy_refs_label, energy_refs_open, energy_refs_items, labels.none_placeholder)?
        .section(structure_id, structure_label, structure_open, structure_items)?
        .section_or_placeholder(structure_refs_id, structure_refs_label, structure_refs_open, structure_refs_items, labels.none_placeholder)?
        .section("cad-play-document.nodes", Some(labels.nodes.into()), true, node_items)?;
    if let Some(ids) = document_tree_selected_ids(&envelope.document, &envelope.runtime)? {
        builder = builder.selected(ids)?;
    }
    if let Some(ids) = document_tree_highlighted_ids(&envelope.document, &envelope.runtime)? {
        builder = builder.highlighted(ids)?;
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::CadPrimitiveSlot;
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{default_document, forest_play_scene, CAD_MODEL_DEFINITION_SHAPE};
    use crate::artifacts::cad::CadPaneId;
    use crate::editor::cad::config::CadConfig;
    use crate::editor::cad::terminology::cad_labels;
    use crate::editor::cad::testkit::*;
    use crate::editor::cad::{make_object_for_typology, CadPlayApp, CadPlayRuntime};
    use semio_framework_plugin::{ArtifactView, PluginApp, UiNode, ViewModel};

    #[semio_framework_async_macros::async_test]
    async fn document_lists_nodes() {
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: pane object sections
        // render empty at this boundary now (documented gap, see `build_document_tree`'s own doc
        // comment) — `object_tree_item_shows_name_with_kind_as_secondary_label`/
        // `object_tree_item_includes_primitive_children` below cover the real (still-working)?
        // tree-item builder directly instead.
        let mut app = new_app();
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-node:"));
    }

    #[semio_framework_async_macros::async_test]
    async fn object_tree_item_shows_name_with_kind_as_secondary_label() {
        let mut object = make_object_for_typology("building.building.beam", 0, CadPaneId::Shape);
        object.label = "U2".into();
        let labels = cad_labels(&CadConfig::default());
        let item = object_tree_item("shape", &object, labels)?;
        assert_eq!(item.label.as_str(), "U2");
        assert_eq!(item.description.as_deref(), Some("Beam"));

        let de_config = CadConfig { locale: "de".into(), ..CadConfig::default() };
        let de_labels = cad_labels(&de_config);
        let de_item = object_tree_item("shape", &object, de_labels)?;
        assert_eq!(de_item.description.as_deref(), Some("Träger"));
    }

    #[semio_framework_async_macros::async_test]
    async fn object_tree_item_includes_primitive_children() {
        let mut object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
        object.primitives = vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }];
        let labels = cad_labels(&CadConfig::default());
        let item = object_tree_item("shape", &object, labels)?;
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("cad-primitive:"));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_tree_selected_and_highlighted_ids_are_none_without_a_reference_selection() {
        // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh object selection/hover is
        // framework-owned now, unreachable at this render boundary — only reference-overlay
        // selection/hover still resolves here (see `document_tree_selected_ids`'s doc comment).
        let scene = default_document();
        let runtime = CadPlayRuntime::default();
        assert_eq!(document_tree_selected_ids(&scene, &runtime), None);
        assert_eq!(document_tree_highlighted_ids(&scene, &runtime), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn document_tree_selected_ids_resolves_reference_selection() {
        let scene = forest_play_scene();
        let runtime = CadPlayRuntime { selected_reference_model_definition_id: Some(CAD_MODEL_DEFINITION_SHAPE.into()), selected_reference_id: Some("ref-concrete-forest".into()), ..CadPlayRuntime::default() };
        let selected = document_tree_selected_ids(&scene, &runtime).expect("selected");
        assert!(selected.iter().any(|id| id == "cad-reference:spatial.shape:ref-concrete-forest"));
    }

    #[semio_framework_async_macros::async_test]
    async fn cad_labels_translate_document_tree_panes_in_german() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
        let config = CadConfig { locale: "de".into(), ..CadConfig::default() };
        let node = render_direct(&app, CAD_PLAY_BODY_DOCUMENT, &doc, &config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"Form\""));
        assert!(json.contains("Gebäude"));
        assert!(json.contains("Energie"));
        assert!(json.contains("Tragwerk Klassisch"));
        assert!(json.contains("Referenzen"));
        assert!(json.contains("\"Knoten\""));
        assert!(!json.contains("\"Shape\""));
        assert!(!json.contains("Struktur Klassisch"));
    }
}
//#endregion 🧪️Tests
