//! 🗣️ BIM viewer terminology: the single `app_labels!` block of the viewer, English first and German second, native and reuse wording. The
//! viewer imports nothing from the editor, so it owns its labels; the macro compile-checks every locale cell.

//#region 🔖️Labels
semio_framework_ui_locale::app_labels! {
    pub struct BimViewerLabels {
        window_world: native_en "World", native_de "Welt", reuse_en "World", reuse_de "Welt";
        window_plan: native_en "Plan", native_de "Grundriss", reuse_en "Plan", reuse_de "Grundriss";
        storey: native_en "Storey", native_de "Geschoss", reuse_en "Level", reuse_de "Ebene";
        storeys: native_en "Storeys", native_de "Geschosse", reuse_en "Levels", reuse_de "Ebenen";
        no_storeys: native_en "No storeys", native_de "Keine Geschosse", reuse_en "No levels", reuse_de "Keine Ebenen";
        projection: native_en "Projection", native_de "Projektion", reuse_en "Projection", reuse_de "Projektion";
        set_camera: native_en "Set camera", native_de "Kamera setzen", reuse_en "Set camera", reuse_de "Kamera setzen";
        set_projection: native_en "Set projection", native_de "Projektion setzen", reuse_en "Set projection", reuse_de "Projektion setzen";
        set_projection_parameter: native_en "Set projection parameter", native_de "Projektionsparameter setzen", reuse_en "Set projection parameter", reuse_de "Projektionsparameter setzen";
        set_storey_visible: native_en "Show or hide storey", native_de "Geschoss ein- oder ausblenden", reuse_en "Show or hide level", reuse_de "Ebene ein- oder ausblenden";
        set_plan_storey: native_en "Show storey in plan", native_de "Geschoss im Grundriss zeigen", reuse_en "Show level in plan", reuse_de "Ebene im Grundriss zeigen";
        elements: native_en "Elements", native_de "Bauteile", reuse_en "Objects", reuse_de "Objekte";
        element: native_en "Element", native_de "Bauteil", reuse_en "Object", reuse_de "Objekt";
        mode_view: native_en "View", native_de "Ansicht", reuse_en "View", reuse_de "Ansicht";
        camera_pose: native_en "Camera pose", native_de "Kamerapose", reuse_en "Camera pose", reuse_de "Kamerapose";
        unknown_body: native_en "Unknown body", native_de "Unbekannter Bereich", reuse_en "Unknown body", reuse_de "Unbekannter Bereich";
    }
}

impl BimViewerLabels {
    /// 🌐️ The English and German text of one label field, read through borrowed constants (`NATIVE_EN.field` would copy the whole label set into the caller's frame at `opt-level = 0`).
    pub fn localized(pick: fn(&Self) -> semio_framework_ui_locale::LabelText) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(pick(&Self::NATIVE_EN).as_str(), pick(&Self::NATIVE_DE).as_str())
    }
}

/// 🗣️ The label set of the viewing user's locale and terminology.
pub fn bim_viewer_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static BimViewerLabels {
    semio_framework_plugin::resolve_labels::<BimViewerLabels>(view_state)
}
//#endregion 🔖️Labels

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
