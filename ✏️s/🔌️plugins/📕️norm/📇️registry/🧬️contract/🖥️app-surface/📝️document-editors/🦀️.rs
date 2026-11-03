//! 📝️ Document editors of the norm app surface: the section ids and the summary. See [`super`]; the typed editors live in `🪟️visual-windows/🧾️properties`.

use super::*;

//#region 🔖️Render
/// 📜 Arrays and object maps at or above this length rely on host windowing (same path for both).
pub const NORM_LIST_VIRTUALIZE_THRESHOLD: usize = 64;

/// 🪪 Stable `TreeWindows` node key for a document path (`root` when empty).
pub fn inputs_section_id(path: &str) -> String {
    if path.is_empty() {
        "norm-inputs-root".into()
    } else {
        format!("norm-inputs-{path}")
    }
}

/// 🧾️ Localized summary headline from the report rollup.
pub fn render_summary<F: NormFamily>(host: &NormHost<F>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let report = host.report();
    let summary = &report.summary;
    let verdict = if summary.complies {
        chrome("complies", "erfüllt", locale)
    } else {
        chrome("does not comply", "nicht erfüllt", locale)
    };
    let parts = summary
        .parts
        .iter()
        .map(|part| {
            let mark = if part.complies { "✓" } else { "✗" };
            format!("{mark}{}", part.part)
        })
        .collect::<Vec<_>>()
        .join(", ");
    render_text(format!(
        "{} — {} {}: {verdict}: {} {}, {} {}, {} {} · {}={:.2} · {parts}",
        F::family_id().label(),
        summary.total,
        chrome("checks", "Nachweise", locale),
        summary.fail,
        chrome("failing", "nicht bestanden", locale),
        summary.warning,
        chrome("warnings", "Warnungen", locale),
        summary.pass,
        chrome("passing", "bestanden", locale),
        chrome("worst u", "max. Ausnutzung", locale),
        summary.worst_utilization
    ))
}
//#endregion 🔖️Render
