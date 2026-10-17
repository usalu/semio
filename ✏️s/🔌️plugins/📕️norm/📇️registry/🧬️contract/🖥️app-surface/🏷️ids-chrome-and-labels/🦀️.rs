//! 🏷️ Ids, window layout and labels of the norm app surface. See [`super`].

use super::*;

//#region 🔖️Ids
/// 🆔️ The single mode every norm app's editor declares.
pub const MODE_EDIT: &str = "edit";

/// 🆔️ The single mode every norm app's viewer declares (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET).
pub const MODE_VIEW: &str = "view";

/// 🆔️ The report tree every norm results window renders, and its one windowed check section.
pub const NORM_REPORT_TREE_ID: &str = "norm-report";

pub const NORM_REPORT_SECTION_ID: &str = "norm-report.checks";
//#endregion 🔖️Ids

/// 📎️ Norm applications own no config or presence facets; Results-window config is registered separately.
pub fn app_schema_descriptor() -> semio_framework_schema_registry::AppSchemaDescriptor {
    semio_framework_schema_registry::AppSchemaDescriptor {
        id: "s.norm.norm",
        config: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
        presence: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
    }
}

//#region 🔖️ViewerManifest
/// ✏️ The `view` mode definition — identical for all fifteen viewers, the read-only counterpart of
/// `edit_mode_definition`.
pub fn view_mode_definition() -> ModeDefinition {
    ModeDefinition { id: MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane window layout — every norm viewer has exactly one window (the compliance
/// report table), so there is no quadrant/split layout to allocate.
pub fn single_window_layout(window_kind_id: &str, title: &str) -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}

/// 📊️ `TableWindowKit` column headers for a norm `CheckReport` — shared by all fifteen viewers.
pub fn report_table_columns(locale: Locale) -> Vec<String> {
    let terminology = Terminology::Native;
    vec![
        LocalizedLabel::native("Part", "Teil").resolve(terminology, locale).into(),
        LocalizedLabel::native("Clause", "Klausel").resolve(terminology, locale).into(),
        LocalizedLabel::native("Subject", "Gegenstand").resolve(terminology, locale).into(),
        LocalizedLabel::native("Status", "Status").resolve(terminology, locale).into(),
        LocalizedLabel::native("Utilization", "Ausnutzung").resolve(terminology, locale).into(),
        LocalizedLabel::native("Title", "Titel").resolve(terminology, locale).into(),
        LocalizedLabel::native("Remedy", "Abhilfe").resolve(terminology, locale).into(),
    ]
}

/// 📊️ `TableWindowKit` rows for a norm `CheckReport` — columns matching [`report_table_columns`].
pub fn report_table_rows(report: &CheckReport, locale: Locale) -> Vec<Vec<String>> {
    let protocol_locale = protocol_locale(locale);
    report
        .checks
        .iter()
        .map(|check| {
            let remedy = check
                .remedies
                .first()
                .map(|remedy| remedy.action.resolve(&protocol_locale).to_string())
                .unwrap_or_default();
            vec![
                check.part.clone(),
                check.clause.to_string(),
                check.subject.label.resolve(&protocol_locale).to_string(),
                status_label(check.status, locale),
                format!("{:.2}", check.utilization),
                check.title.resolve(&protocol_locale).to_string(),
                remedy,
            ]
        })
        .collect()
}
//#endregion 🔖️ViewerManifest

//#region 🔖️LocaleChrome
/// 🗣️ Maps the shell [`Locale`] onto the protocol axis [`LocalizedCopy`] resolves against.
pub fn protocol_locale(locale: Locale) -> Locale {
    match locale {
        Locale::En => Locale::En,
        Locale::De => Locale::De,
    }
}

/// 🏷️ Localized chrome string from a fixed en/de pair.
pub(super) fn chrome(en: &str, de: &str, locale: Locale) -> String {
    LocalizedLabel::native(en, de).resolve(Terminology::Native, locale).to_string()
}

/// 🏷️ `MediaPortSpec.label` is a plain `String` — pack both native terms from a [`LocalizedLabel`].
pub(super) fn media_port_label(label: LocalizedLabel) -> String {
    format!(
        "{} / {}",
        label.resolve(Terminology::Native, Locale::En),
        label.resolve(Terminology::Native, Locale::De),
    )
}

/// ✅️ Localized status chip text.
pub fn status_label(status: CheckStatus, locale: Locale) -> String {
    match status {
        CheckStatus::Pass => chrome("Pass", "Bestanden", locale),
        CheckStatus::Warning => chrome("Warning", "Warnung", locale),
        CheckStatus::Fail => chrome("Fail", "Nicht bestanden", locale),
        CheckStatus::NotApplicable => chrome("Not applicable", "Nicht anwendbar", locale),
    }
}

/// 📐️ Formats a SI [`Quantity`] with a natural display unit for the kind.
pub fn format_quantity(quantity: &Quantity) -> String {
    match quantity.kind {
        QuantityKind::Force => format!("{:.3} kN", quantity.value / 1_000.0),
        QuantityKind::Stress | QuantityKind::Pressure => format!("{:.3} MPa", quantity.value / 1_000_000.0),
        QuantityKind::Length => {
            if quantity.value.abs() > 0.0 && quantity.value.abs() < 1.0 {
                format!("{:.1} mm", quantity.value * 1_000.0)
            } else {
                format!("{:.3} m", quantity.value)
            }
        }
        QuantityKind::Area => format!("{:.3} m²", quantity.value),
        QuantityKind::Volume => format!("{:.3} m³", quantity.value),
        QuantityKind::HeatTransferCoefficient => format!("{:.3} W/(m²K)", quantity.value),
        QuantityKind::ThermalResistance => format!("{:.3} m²K/W", quantity.value),
        QuantityKind::ThermalConductivity => format!("{:.3} W/(m·K)", quantity.value),
        QuantityKind::Energy => format!("{:.3} kWh/(m²a)", quantity.value / 3_600_000.0),
        QuantityKind::Power => format!("{:.3} W", quantity.value),
        QuantityKind::Moment => format!("{:.3} kN·m", quantity.value / 1_000.0),
        QuantityKind::Mass => format!("{:.3} kg", quantity.value),
        QuantityKind::Time => format!("{:.3} s", quantity.value),
        QuantityKind::Temperature => format!("{:.2} K", quantity.value),
        QuantityKind::Acceleration => format!("{:.3} m/s²", quantity.value),
        QuantityKind::AirPermeability => format!("{:.3} m³/(m²h)", quantity.value),
        QuantityKind::VentilationRate => format!("{:.3} 1/h", quantity.value),
        QuantityKind::Dimensionless => format!("{:.3}", quantity.value),
    }
}

/// 🆔️ Editor controller id for a norm variant (`s.norm.<variant>@1/*#editor`).
pub fn editor_controller_id(variant: &str) -> String {
    format!("s.norm.{variant}@1/*#editor")
}
//#endregion 🔖️LocaleChrome
