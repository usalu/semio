//! 📊️ Table twin: the accessible tabular alternative of a diagram plus its screen-reader description. See [`super`].

use super::*;

/// 📋️ One formatted table-twin row set: column headers with units and string cells per row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableTwin {
    pub caption: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub marker_headers: Vec<String>,
    pub marker_rows: Vec<Vec<String>>,
}

fn with_unit(text: &str, unit: &str) -> String {
    if unit.is_empty() { text.to_string() } else { format!("{text} [{unit}]") }
}

/// 📋️ The diagram's data table in the locale; short columns are padded with empty cells and markers form a second table.
pub fn table_twin(diagram: &Diagram2d, locale: Locale) -> TableTwin {
    let table = &diagram.table_twin;
    let rows = table.columns.iter().map(|c| c.values.len()).max().unwrap_or(0);
    let marker_headers = match locale {
        Locale::En => ["Marker", "x", "y", "Utilisation", "Check"],
        Locale::De => ["Markierung", "x", "y", "Ausnutzung", "Nachweis"],
    };
    TableTwin {
        caption: copy_text(&diagram.title, locale).to_string(),
        headers: table.columns.iter().map(|c| with_unit(copy_text(&c.header, locale), &c.unit)).collect(),
        rows: (0..rows).map(|r| table.columns.iter().map(|c| c.values.get(r).map_or_else(String::new, |v| format_number(*v, locale))).collect()).collect(),
        marker_headers: marker_headers.iter().map(|h| h.to_string()).collect(),
        marker_rows: diagram
            .markers
            .iter()
            .map(|m| {
                vec![
                    copy_text(&m.label, locale).to_string(),
                    format_number(m.at.x, locale),
                    format_number(m.at.y, locale),
                    m.utilisation.map_or_else(String::new, |u| format_percent(u, locale)),
                    m.bound_check.clone().unwrap_or_default(),
                ]
            })
            .collect(),
    }
}

/// ♿️ Screen-reader description: title, description, axes with units and ranges, series and markers with their utilisation.
pub fn accessible_description(diagram: &Diagram2d, locale: Locale) -> String {
    let t = |en: &str, de: &str| if locale == Locale::En { en.to_string() } else { de.to_string() };
    let mut parts = vec![copy_text(&diagram.title, locale).to_string(), diagram.basis().map_or_else(String::new, |basis| copy_text(basis, locale).to_string()), copy_text(&diagram.description, locale).to_string()];
    for (i, axis) in diagram.axes.iter().enumerate() {
        let (lo, hi) = resolve_domain(diagram, i);
        let name = if i == 0 { t("Horizontal axis", "Horizontale Achse") } else { t("Vertical axis", "Vertikale Achse") };
        parts.push(format!("{name}: {} [{}], {} {} {} {}", copy_text(&axis.label, locale), axis.unit, t("from", "von"), format_number(lo, locale), t("to", "bis"), format_number(hi, locale)));
    }
    parts.push(format!("{}: {}", t("Series", "Datenreihen"), diagram.series.iter().map(|s| copy_text(series_label(s), locale)).collect::<Vec<_>>().join(", ")));
    for m in &diagram.markers {
        let shape = marker_shape_name(m.shape, locale);
        let u = m.utilisation.map_or_else(String::new, |u| format!(", {} {}", t("utilisation", "Ausnutzung"), format_percent(u, locale)));
        parts.push(format!("{} ({shape}) {} {}; {}{u}", copy_text(&m.label, locale), t("at", "bei"), format_number(m.at.x, locale), format_number(m.at.y, locale)));
    }
    parts.retain(|p| !p.is_empty());
    parts.join(". ")
}
