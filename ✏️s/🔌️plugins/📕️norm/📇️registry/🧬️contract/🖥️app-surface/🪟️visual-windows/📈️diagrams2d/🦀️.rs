//! 📈️ Diagrams window of the norm app surface: `Diagram2d` lowered to Canvas2d layers with hit-testing and an accessible table twin.
//!
//! 🔗️ See `NORM/🎬️structural-scene/🧬️schema/📈️diagram2d` for the schema and `NORM/🎬️structural-scene/📈️diagram-kit` for the geometry builders.

use semio_framework_plugin::Canvas2dScene;
use semio_framework_ui_locale::Locale;
use semio_s_plugin_norm_structural_scene::{Axis, AxisRef, AxisScale, Diagram2d, Dash, EntityRef, Fill, Hatch, LocalizedCopy, Marker, MarkerShape, Series, SectionProfile, TickRule, Tone, Vec2};

#[path = "🎯️hit-test/🦀️.rs"]
mod hit_test;
pub use hit_test::*;

#[path = "🖼️canvas-lowering/🦀️.rs"]
mod canvas_lowering;
pub use canvas_lowering::*;

#[path = "📊️table-twin/🦀️.rs"]
mod table_twin_model;
pub use table_twin_model::*;

#[path = "🪧marker-grips/🦀️.rs"]
mod marker_grips;
pub use marker_grips::*;

#[path = "🪟️window/🦀️.rs"]
mod window;
pub use window::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🌐️ The text of bilingual copy in `locale`.
pub fn copy_text(copy: &LocalizedCopy, locale: Locale) -> &str {
    match locale {
        Locale::En => &copy.en,
        Locale::De => &copy.de,
    }
}

/// 🏷️ The legend and table label of a series.
pub fn series_label(series: &Series) -> &LocalizedCopy {
    match series {
        Series::Polyline { label, .. } | Series::Polygon { label, .. } | Series::Band { label, .. } | Series::Scatter { label, .. } | Series::Limit { label, .. } | Series::Section { label, .. } | Series::Ray { label, .. } | Series::Surface3 { label, .. } => label,
    }
}

/// 🔢️ Locale-aware number text: four significant digits, decimal comma in German, thin-space grouping from 10 000.
pub fn format_number(value: f64, locale: Locale) -> String {
    if !value.is_finite() {
        return "–".to_string();
    }
    let magnitude = value.abs();
    let decimals = if magnitude == 0.0 { 0 } else { (3 - magnitude.log10().floor() as i32).clamp(0, 6) as usize };
    let mut text = format!("{value:.decimals$}");
    if text.contains('.') {
        text = text.trim_end_matches('0').trim_end_matches('.').to_string();
    }
    if text == "-0" {
        text = "0".to_string();
    }
    let (sign, body) = text.strip_prefix('-').map_or(("", text.as_str()), |b| ("-", b));
    let (int, frac) = body.split_once('.').map_or((body, None), |(i, f)| (i, Some(f)));
    let grouped = if int.len() >= 5 {
        let n = int.len();
        int.chars().enumerate().fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && (n - i) % 3 == 0 {
                acc.push('\u{2009}');
            }
            acc.push(c);
            acc
        })
    } else {
        int.to_string()
    };
    let separator = if locale == Locale::De { ',' } else { '.' };
    match frac {
        Some(f) => format!("{sign}{grouped}{separator}{f}"),
        None => format!("{sign}{grouped}"),
    }
}

/// 💯️ Utilisation ratio as a percentage in the locale.
pub fn format_percent(ratio: f64, locale: Locale) -> String {
    format!("{}\u{202f}%", format_number(ratio * 100.0, locale).replace('\u{2009}', ""))
}

/// 🖼️ The Canvas2d scene of one diagram: layers in pixel space, camera at the origin with unit zoom.
pub fn diagram_scene(diagram: &Diagram2d, options: &LoweringOptions) -> (Canvas2dScene, Vec<PickTarget>) {
    let lowered = lower_diagram(diagram, options);
    (Canvas2dScene::base(0.0, 0.0, 1.0, lowered.layers_json), lowered.targets)
}
