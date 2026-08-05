//! 🗣️ Process 3d play app — the single `app_labels!` block plus the locale resolver every taxonomy
//! node reaches for. Deliberately ONE block for the whole app (never split per window/panel).

//#region 🔖️Labels
/// 🗣️ Complete UI label set for the 3D app; one field per label makes every locale combination compile-checked.
semio_framework_plugin::app_labels! {
    pub struct Process3dLabels {
        stock: native_en "Stock", native_de "Rohteil", reuse_en "Stock", reuse_de "Rohteil";
        steps: native_en "Steps", native_de "Schritte", reuse_en "Steps", reuse_de "Schritte";
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        cut: native_en "Cut", native_de "Schnitt", reuse_en "Cut", reuse_de "Schnitt";
        drill: native_en "Drill", native_de "Bohrung", reuse_en "Drill", reuse_de "Bohrung";
        attach: native_en "Attach", native_de "Anbau", reuse_en "Attach", reuse_de "Anbau";
        push_cut: native_en "Push Cut", native_de "Schnitt (Drücken)", reuse_en "Push Cut", reuse_de "Schnitt (Drücken)";
        pull_attach: native_en "Pull Attach", native_de "Anbau (Ziehen)", reuse_en "Pull Attach", reuse_de "Anbau (Ziehen)";
        enabled: native_en "Enabled", native_de "Aktiviert", reuse_en "Enabled", reuse_de "Aktiviert";
        volume: native_en "Volume", native_de "Volumen", reuse_en "Volume", reuse_de "Volumen";
        label_field: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
        remove: native_en "Remove", native_de "Entfernen", reuse_en "Remove", reuse_de "Entfernen";
        provenance: native_en "Made By", native_de "Erstellt von", reuse_en "Made By", reuse_de "Erstellt von";
        validation_warning: native_en "Warning", native_de "Warnung", reuse_en "Warning", reuse_de "Warnung";
        source: native_en "Source", native_de "Quelle", reuse_en "Source", reuse_de "Quelle";
        window_main: native_en "Workpiece", native_de "Werkstück", reuse_en "Workpiece", reuse_de "Werkstück";
        field_width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        field_depth: native_en "Depth", native_de "Tiefe", reuse_en "Depth", reuse_de "Tiefe";
        field_height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        field_radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        field_pos_x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        field_pos_y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        field_pos_z: native_en "Z", native_de "Z", reuse_en "Z", reuse_de "Z";
        field_angle: native_en "Angle", native_de "Winkel", reuse_en "Angle", reuse_de "Winkel";
        stock_kind_box: native_en "Box", native_de "Quader", reuse_en "Box", reuse_de "Quader";
        stock_kind_cylinder: native_en "Cylinder", native_de "Zylinder", reuse_en "Cylinder", reuse_de "Zylinder";
        stock_kind_sphere: native_en "Sphere", native_de "Kugel", reuse_en "Sphere", reuse_de "Kugel";
        import_model: native_en "Import Model…", native_de "Modell importieren…", reuse_en "Import Model…", reuse_de "Modell importieren…";
        step_control: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        workshop: native_en "Workshop", native_de "Werkstatt", reuse_en "Workshop", reuse_de "Werkstatt";
        machines: native_en "Machines", native_de "Maschinen", reuse_en "Machines", reuse_de "Maschinen";
        remove_machine: native_en "Remove Machine", native_de "Maschine entfernen", reuse_en "Remove Machine", reuse_de "Maschine entfernen";
        installed: native_en "Installed", native_de "Installiert", reuse_en "Installed", reuse_de "Installiert";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn process3d_labels(cfg: &crate::apps::process3d::config::Process3dConfig) -> &'static Process3dLabels {
    semio_framework_plugin::resolve_labels_for_locale::<Process3dLabels>(&cfg.locale)
}

/// 🎨️ The icon a process measure renders with, shared by the document panel's step tree and the
/// catalogue's capability listing.
pub fn process3d_measure_icon(measure: &crate::artifacts::process3d::ProcessMeasure) -> &'static str {
    match measure {
        crate::artifacts::process3d::ProcessMeasure::Cut { .. } => "scissors",
        crate::artifacts::process3d::ProcessMeasure::Drill { .. } => "circle-dot",
        crate::artifacts::process3d::ProcessMeasure::Attach { .. } => "plus",
    }
}

/// 🗣️ The localized label a process measure's kind renders with, used by the inspector's step group title.
pub fn process3d_measure_label(measure: &crate::artifacts::process3d::ProcessMeasure, labels: &Process3dLabels) -> semio_framework_plugin::LabelText {
    match measure {
        crate::artifacts::process3d::ProcessMeasure::Cut { .. } => labels.cut,
        crate::artifacts::process3d::ProcessMeasure::Drill { .. } => labels.drill,
        crate::artifacts::process3d::ProcessMeasure::Attach { .. } => labels.attach,
    }
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::process3d::config::Process3dConfig;

    #[test]
    fn labels_resolve_native_by_default_and_in_german() {
        let mut config = Process3dConfig::default();
        assert_eq!(process3d_labels(&config).stock.as_str(), "Stock");
        config.locale = "de".into();
        assert_eq!(process3d_labels(&config).stock.as_str(), "Rohteil");
    }
}
//#endregion 🧪️Tests
