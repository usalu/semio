//! 📊️ Table twin: the accessible tabular alternative of a diagram plus its screen-reader description. See [`super`].

use {super::*, crate::chrome};

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

/// 📝️ Basis, description, axes with ranges and series of a diagram: the part of every description that does not grow with the marker count.
fn description_head(diagram: &Diagram2d, locale: Locale) -> Vec<String> {
    let mut parts = vec![diagram.basis().map_or_else(String::new, |basis| copy_text(basis, locale).to_string()), copy_text(&diagram.description, locale).to_string()];
    for (i, axis) in diagram.axes.iter().enumerate() {
        let (lo, hi) = resolve_domain(diagram, i);
        let name = if i == 0 { chrome("Horizontal axis", "Horizontale Achse", locale) } else { chrome("Vertical axis", "Vertikale Achse", locale) };
        parts.push(format!("{name}: {} [{}], {} {} {} {}", copy_text(&axis.label, locale), axis.unit, chrome("from", "von", locale), format_number(lo, locale), chrome("to", "bis", locale), format_number(hi, locale)));
    }
    parts.push(format!("{}: {}", chrome("Series", "Datenreihen", locale), diagram.series.iter().map(|s| copy_text(series_label(s), locale)).collect::<Vec<_>>().join(", ")));
    parts
}

/// 📝️ Bounded canvas description: [`description_head`] and a marker count with the worst utilisation. The surface's accessible description is a fixed-capacity text, so it summarises instead of enumerating every marker; [`accessible_description`] and the table twin keep the full list.
pub fn surface_description(diagram: &Diagram2d, locale: Locale) -> String {
    let mut parts = description_head(diagram, locale);
    if let Some(worst) = diagram.markers.iter().filter_map(|m| m.utilisation).reduce(f64::max) {
        parts.push(format!("{}: {}, {} {}", chrome("Markers", "Markierungen", locale), diagram.markers.len(), chrome("worst utilisation", "höchste Ausnutzung", locale), format_percent(worst, locale)));
    }
    parts.retain(|p| !p.is_empty());
    parts.join(". ")
}

/// ♿️ Screen-reader description: title, description, axes with units and ranges, series and markers with their utilisation.
pub fn accessible_description(diagram: &Diagram2d, locale: Locale) -> String {
    let mut parts = vec![copy_text(&diagram.title, locale).to_string()];
    parts.extend(description_head(diagram, locale));
    for m in &diagram.markers {
        let shape = marker_shape_name(m.shape, locale);
        let u = m.utilisation.map_or_else(String::new, |u| format!(", {} {}", chrome("utilisation", "Ausnutzung", locale), format_percent(u, locale)));
        parts.push(format!("{} ({shape}) {} {}; {}{u}", copy_text(&m.label, locale), chrome("at", "bei", locale), format_number(m.at.x, locale), format_number(m.at.y, locale)));
    }
    parts.retain(|p| !p.is_empty());
    parts.join(". ")
}
