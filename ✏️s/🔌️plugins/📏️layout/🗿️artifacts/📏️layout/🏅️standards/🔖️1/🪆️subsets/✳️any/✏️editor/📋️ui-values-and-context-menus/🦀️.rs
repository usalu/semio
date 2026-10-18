//! 📋️ UI values and context menus of the layout play app: the boolean, number and list UI value builders, the layout element ids, the preflight issue lookup, the frame kinds, the authoring surface and the context menu items with their internal action. See [`super`].

use super::*;

//#region 🔖️Constants
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

//#region 🔖️ContextMenu
fn layout_element_ids(surface: Option<&ContextMenuSurfaceTarget>, fallback: &[String]) -> Vec<String> {
    let groups = surface.map_or(&[][..], |target| target.selection.as_slice());
    let mut ids: Vec<String> = groups.iter().filter(|group| group.domain == LAYOUT_GRANULARITY_ELEMENT).flat_map(|group| group.ids.iter().cloned()).collect();
    if ids.is_empty() {
        ids = fallback.to_vec();
    }
    ids
}

fn layout_preflight_issue_for_row(doc: &LayoutSnapshot, labels: &LayoutLabels, row_id: &str) -> Option<PreflightIssue> {
    run_layout_preflight(doc, labels).into_iter().find(|issue| {
        let object = issue.object_id.clone().unwrap_or_else(|| issue.message.clone());
        format!("layout-preflight.{}.{}", issue.code, object) == row_id
    })
}

fn layout_frame_kind<'a>(doc: &'a LayoutSnapshot, frame_id: &str) -> Option<&'a str> {
    doc.pages.iter().flat_map(|page| page.frames.iter()).find(|frame| frame.id() == frame_id).map(|frame| frame.kind_str())
}

fn layout_link_referencing_frame_ids(doc: &LayoutSnapshot, link_id: &str) -> Vec<String> {
    doc.pages
        .iter()
        .flat_map(|page| page.frames.iter())
        .filter(|frame| matches!(frame, Frame::Image { link_id: id, .. } if id == link_id))
        .map(|frame| frame.id().to_string())
        .collect()
}

fn layout_authoring_surface(surface: Option<&ContextMenuSurfaceTarget>) -> bool {
    surface.map_or(true, |target| target.surface_id != LAYOUT_PLAY_SURFACE_PREVIEW)
}

fn layout_context_menu_item(id: &str, label: &str, icon: &str, action: &str, args: Option<DslValue>, destructive: bool, disabled: bool) -> ContextMenuItemSpec {
    ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args,
        destructive: destructive.then_some(true),
        disabled: disabled.then_some(true),
        ..Default::default()
    }
}

fn layout_add_frame_items<'a>(menu: semio_framework_plugin::Menu<'a>, labels: &'a LayoutLabels, kind: &'a str) -> semio_framework_plugin::Menu<'a> {
    let (id, label, icon) = match kind {
        "text" => ("add-text-frame", labels.kind_text.as_str(), "type"),
        "image" => ("add-image-frame", labels.kind_image.as_str(), "image"),
        _ => ("add-rect-frame", labels.kind_rect.as_str(), "square"),
    };
    menu.item(layout_context_menu_item(id, label, icon, "addFrame", Some(semio_framework::dsl_value!({ "kind": kind })), false, false))
}

/// 🖱️ On-demand layout context menu — canvas, document tree, and preflight surfaces each carry
/// different hit/selection shapes; `organize_context_menu` runs automatically at the host funnel.
pub(super) fn layout_context_menu_items(
    registry: &AppActionRegistry,
    doc: &LayoutSnapshot,
    labels: &LayoutLabels,
    view_state: &semio_framework_plugin::ViewModel,
    surface: Option<&ContextMenuSurfaceTarget>,
    fallback_selected: &[String],
) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, Menu, SelectionKind};
    let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;

    let hits = surface.map_or(&[][..], |target| target.hits.as_slice());
    let authoring = layout_authoring_surface(surface);
    let preview = surface.is_some_and(|target| target.surface_id == LAYOUT_PLAY_SURFACE_PREVIEW);

    if let Some(hit) = hits.first().filter(|hit| hit.id.starts_with("layout-document.page.")) {
        if let Some(page_id) = hit.id.strip_prefix("layout-document.page.") {
            return Menu::of(registry, view_state).item(layout_context_menu_item("set-active-page", labels.active_page.as_str(), "file", "setActivePage", Some(semio_framework::dsl_value!({ "pageId": page_id })), false, false)).build();
        }
    }

    if let Some(hit) = hits.first().filter(|hit| hit.id.starts_with("layout-document.link.")) {
        if let Some(link_id) = hit.id.strip_prefix("layout-document.link.") {
            let ids = layout_link_referencing_frame_ids(doc, link_id);
            let disabled = ids.is_empty();
            return Menu::of(registry, view_state)
                .item(layout_context_menu_item(
                    "select-link-frames",
                    if is_de { "Verknüpfte Rahmen auswählen" } else { "Select linked frames" },
                    "image",
                    INTERACTION_SELECT_ACTION_ID,
                    Some(layout_select_action_args(&ids, "replace")),
                    false,
                    disabled,
                ))
                .build();
        }
    }

    if let Some(hit) = hits.first().filter(|hit| hit.id.starts_with("layout-preflight.")) {
        if let Some(issue) = layout_preflight_issue_for_row(doc, labels, &hit.id) {
            let issue_value = semio_framework::dsl_value!({
                "severity": issue.severity,
                "code": issue.code,
                "message": issue.message,
                "objectId": issue.object_id,
                "pageId": issue.page_id,
            });
            return Menu::of(registry, view_state).item(layout_context_menu_item("focus-preflight-issue", labels.preflight.as_str(), "alert-triangle", "focusPreflightIssue", Some(semio_framework::dsl_value!({ "issue": issue_value })), false, false)).build();
        }
    }

    let selected = layout_element_ids(surface, fallback_selected);
    let element_hit = hits.iter().find(|hit| hit.domain == LAYOUT_GRANULARITY_ELEMENT);

    if selected.is_empty() {
        if preview {
            return Menu::of(registry, view_state).action("selectAll").build();
        }
        let mut menu = Menu::of(registry, view_state);
        if let Some(hit) = element_hit {
            menu = menu.item(layout_context_menu_item(
                "select-hit",
                if is_de { "Auswählen" } else { "Select" },
                "mouse-pointer",
                INTERACTION_SELECT_ACTION_ID,
                Some(layout_select_action_args(std::slice::from_ref(&hit.id), "replace")),
                false,
                false,
            ));
        }
        menu = menu.group("create", |group| layout_add_frame_items(layout_add_frame_items(layout_add_frame_items(group, labels, "rect"), labels, "text"), labels, "image"));
        return menu.action("addPage").action("selectAll").action("paste").destructive("clearSelection").disabled_because(&semio_framework_plugin::nothing_selected()).build();
    }

    let phrase = selection_count_phrase(view_state.locale, &[(selected.len(), SelectionKind::Frame)]).unwrap_or_default();
    let mut menu = Menu::of(registry, view_state);
    if let Some(hit) = element_hit.filter(|hit| !selected.contains(&hit.id)) {
        menu = menu.item(layout_context_menu_item(
            "select-hit",
            if is_de { "Auswählen" } else { "Select" },
            "mouse-pointer",
            INTERACTION_SELECT_ACTION_ID,
            Some(layout_select_action_args(std::slice::from_ref(&hit.id), "replace")),
            false,
            false,
        ));
    }
    if selected.len() == 1 {
        if let Some(kind) = layout_frame_kind(doc, &selected[0]) {
            menu = layout_add_frame_items(menu.group("create", |group| group), labels, kind);
        }
    }
    menu = menu.action("copy").action("cut").action("paste");
    if authoring {
        menu = menu.item(layout_context_menu_item(
            "delete-selection",
            &format!("{} ({phrase})", if is_de { "Löschen" } else { "Delete" }),
            "trash",
            "deleteSelection",
            None,
            true,
            false,
        ));
    }
    menu.destructive("clearSelection").build()
}
//#endregion 🔖️ContextMenu

/// 🙈️ An internal (non-palette) action declaration — the pointer/inspector/DnD/engagement-bound
/// vocabulary dispatched by the canvas and panels, never surfaced as a standalone palette command.
pub(super) fn layout_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}
//#endregion 🔖️Constants
