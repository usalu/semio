//! 🏷️ Ids, window layout and labels of the norm app surface. See [`super`].

use super::*;

//#region 🔖️Ids
/// 🆔️ The single mode every norm app's editor declares.
pub const MODE_EDIT: &str = "edit";

/// 🆔️ The single mode every norm app's viewer declares (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET).
pub const MODE_VIEW: &str = "view";

/// 🆔️ The checks tree every norm results window renders, its summary section and its one windowed check section.
pub const NORM_CHECKS_TREE_ID: &str = "norm-checks";

pub const NORM_CHECKS_SUMMARY_ID: &str = "norm-checks.summary";

pub const NORM_CHECKS_SECTION_ID: &str = "norm-checks.list";
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

/// 📊️ `TableWindowKit` column headers for a norm `CheckReport` — the accessible table twin of the checks window.
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
                check.clause.text(&locale),
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
pub fn chrome(en: &str, de: &str, locale: Locale) -> String {
    LocalizedLabel::native(en, de).resolve(Terminology::Native, locale).to_string()
}

/// 🏷️ `MediaPortSpec.label` is a plain `String` — pack both native terms from a [`LocalizedLabel`].
pub fn media_port_label(label: LocalizedLabel) -> String {
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
        CheckStatus::UnverifiedBasis => chrome("Unverified basis", "Ungeprüfte Grundlage", locale),
        CheckStatus::Info => chrome("Info", "Hinweis", locale),
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
        QuantityKind::Velocity => format!("{:.3} m/s", quantity.value),
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

//#region 🔖️FaultNotices
/// 📢️ The localized notice of every refusal and warning code the norm editors raise (`mutation.*`, `norm.*`), the table each norm editor and viewer declares as `fault_notices`; one fixed sentence per code, never the English developer detail.
pub fn norm_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> = std::sync::LazyLock::new(|| {
        [
            ("mutation.no-op", "Nothing changed: the value is already set.", "Keine Änderung: Der Wert ist bereits gesetzt."),
            ("mutation.clamped", "The position was past the end of the list; the item was added at the end.", "Die Position lag hinter dem Listenende; das Element wurde am Ende eingefügt."),
            ("mutation.invariant", "The value breaks a rule of this standard and was not applied.", "Der Wert verletzt eine Regel dieser Norm und wurde nicht übernommen."),
            ("mutation.duplicate-id", "Another row already uses this id.", "Eine andere Zeile verwendet diese Id bereits."),
            ("norm.unhandled-action", "The norm editor does not handle this action.", "Der Norm-Editor verarbeitet diese Aktion nicht."),
            ("norm.value-path", "The field path is not valid.", "Der Feldpfad ist ungültig."),
            ("norm.set-field-value", "The value does not fit this field.", "Der Wert passt nicht zu diesem Feld."),
            ("norm.set-active-example-invalid", "This example is not available.", "Dieses Beispiel ist nicht verfügbar."),
            ("norm.insert-item-value", "The row could not be inserted.", "Die Zeile konnte nicht eingefügt werden."),
            ("norm.set-fields-args", "The live edit arguments are incomplete or malformed.", "Die Argumente der Live-Änderung sind unvollständig oder fehlerhaft."),
            ("norm.insert-placement", "The item could not be placed at this position.", "Das Element konnte an dieser Position nicht platziert werden."),
            ("norm.edit-unresolved", "The edit could not be resolved against the document.", "Die Änderung konnte im Dokument nicht aufgelöst werden."),
            ("norm.apply-remedy-option-type", "The chosen remedy option has the wrong type.", "Die gewählte Abhilfe-Variante hat den falschen Typ."),
            ("norm.apply-remedy-uint", "The remedy value must be a non-negative whole number.", "Der Abhilfewert muss eine nicht negative ganze Zahl sein."),
            ("norm.apply-remedy-int", "The remedy value must be a whole number.", "Der Abhilfewert muss eine ganze Zahl sein."),
            ("norm.apply-remedy-not-applicable", "This remedy cannot be applied to the check.", "Diese Abhilfe kann auf den Nachweis nicht angewendet werden."),
            ("norm.apply-remedy-missing-remedy", "The check offers no such remedy.", "Der Nachweis bietet diese Abhilfe nicht an."),
            ("norm.apply-remedy-missing-option", "The remedy offers no such option.", "Die Abhilfe bietet diese Variante nicht an."),
            ("norm.apply-remedy-missing-check", "The check no longer exists.", "Der Nachweis existiert nicht mehr."),
            ("norm.apply-remedy-empty-path", "The remedy names no field to change.", "Die Abhilfe nennt kein zu änderndes Feld."),
            ("norm.apply-remedy-remove-path", "A removing remedy must target a list row.", "Eine entfernende Abhilfe muss auf eine Listenzeile zielen."),
            ("norm.entity-delete-empty", "Select at least one row to delete.", "Mindestens eine Zeile zum Löschen auswählen."),
            ("norm.entity-delete-unknown", "A selected row no longer exists, so nothing was deleted.", "Eine ausgewählte Zeile existiert nicht mehr, daher wurde nichts gelöscht."),
            ("norm.remedy-removes-row", "This remedy removes a row and writes no value.", "Diese Abhilfe entfernt eine Zeile und schreibt keinen Wert."),
        ]
        .into_iter()
        .map(|(code, en, de)| (code, LocalizedLabel::native(en, de)))
        .collect()
    });
    NOTICES.as_slice()
}
//#endregion 🔖️FaultNotices

/// 🖐️ The window actions that open and close the touch Edit menu (the shell answers a long press with the first); both become `viewportTouchMenu` of `viewportAct`.
pub const TOUCH_MENU_OPEN: &str = "norm.touch.menu-open";
pub const TOUCH_MENU_CLOSE: &str = "norm.touch.menu-close";

/// 🖐️ The window action a double tap raises: it toggles the camera between the whole scene and the selected entities (`viewportTouchFrame` of `viewportAct`).
pub const TOUCH_FRAME_SELECTION: &str = "norm.touch.frame-selection";

