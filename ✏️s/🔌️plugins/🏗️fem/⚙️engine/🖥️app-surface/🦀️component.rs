//! 🤝️ FEM plugin — helpers shared by the 2D and 3D apps' `ui` crates (non-constitutional; see the
//! constitutional-split recipe's "shared code used by ≥2 apps" rule). Everything here is used by BOTH
//! `fem2d_ui` and `fem3d_ui` — `next_id`/id-collision retry, hex/von-Mises color helpers, mode-shape
//! normalization, and the `setResultDisplay` ephemeral view-state plumbing.

use crate::model::Dof;
use semio_framework_plugin::{ActionArgDef, ActionArgOption, LocalizedLabel};
use serde_json::Value;
use std::collections::HashMap;

//#region 🔖️Constants
/// 🎨️ Blue→green→yellow→red banded ramp for von Mises stress contour fill colors, low to high.
pub const VON_MISES_BANDS: [&str; 8] = ["#1d4ed8", "#2563eb", "#0ea5e9", "#22c55e", "#eab308", "#f97316", "#ef4444", "#b91c1c"];

/// 📐️ Mode shapes are normalized to unit peak displacement (see `normalize_mode_shape`), so a single
/// ratio of the model's own extent gives a visually consistent, deterministic amplitude for both
/// 2D and 3D modal/buckling overlays regardless of the eigen-solver's arbitrary shape normalization.
pub const MODE_SHAPE_AMPLITUDE_RATIO: f64 = 0.1;
//#endregion 🔖️Constants

//#region 🔖️Shared
/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub async fn next_id(existing: impl Iterator<Item = String>, prefix: &str) -> String {
    let ids: std::collections::HashSet<String> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.contains(&candidate) {
            return candidate;
        }
        i += 1;
    }
}

/// 🎨️ Parses a `"#rrggbb"` hex color into 0..1 float components for a Canvas2d `fill.color` array.
pub async fn hex_to_rgb01(hex: &str) -> (f64, f64, f64) {
    let h = hex.trim_start_matches('#');
    let component = |slice: &str| u8::from_str_radix(slice, 16).unwrap_or(0) as f64 / 255.0;
    (component(&h[0..2]), component(&h[2..4]), component(&h[4..6]))
}

/// 📐️ Rescales a node-id-keyed displacement map in place so its largest translational magnitude
/// (`sqrt(tx²+ty²+tz²)`) becomes exactly 1.0 — mode shapes from `subspace_iteration` are mass/Kg-
/// orthonormalized (arbitrary physical magnitude), so this gives a deterministic, comparable-across-
/// modes amplitude before scaling by `MODE_SHAPE_AMPLITUDE_RATIO * model_extent`. A near-zero shape
/// (degenerate/rigid mode) is left untouched rather than divided by a near-zero magnitude.
pub async fn normalize_mode_shape(disp_map: &mut HashMap<String, [f64; 6]>) {
    let peak = disp_map.values().map(|d| (d[Dof::Tx.index()].powi(2) + d[Dof::Ty.index()].powi(2) + d[Dof::Tz.index()].powi(2)).sqrt()).fold(0.0_f64, f64::max);
    if peak < 1e-12 {
        return;
    }
    for values in disp_map.values_mut() {
        for v in values.iter_mut() {
            *v /= peak;
        }
    }
}

/// 🌡️ Maps `value` within `[min, max]` onto one of `VON_MISES_BANDS`' 8 hex colors, low to high.
pub async fn von_mises_color(value: f64, min: f64, max: f64) -> &'static str {
    let span = (max - min).max(1e-9);
    let t = ((value - min) / span).clamp(0.0, 1.0);
    let index = ((t * (VON_MISES_BANDS.len() - 1) as f64).round() as usize).min(VON_MISES_BANDS.len() - 1);
    VON_MISES_BANDS[index]
}
//#endregion 🔖️Shared

//#region 🔖️ResultDisplay
/// 👁️ Ephemeral (non-document) view state selecting what the results window shows — which
/// `fem2d_solve_all`/`fem3d_solve_all` case-or-combination id (`source_id`) and which `DisplayMode`.
/// Mutated by the `setResultDisplay` VIEW action (`ActionEmit::default()`, no operations — never recorded in
/// history) and lives directly on the app struct, per `ArtifactApp::handle_action`'s `&mut self`.
#[derive(Clone, Debug, Default)]
pub struct ResultDisplay {
    pub source_id: Option<String>,
    pub mode: DisplayMode,
}

/// 👁️ Which analysis result the results window renders: the static solve, or the `n`-th modal/buckling
/// mode shape (0-indexed).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DisplayMode {
    #[default]
    Static,
    Modal(usize),
    Buckling(usize),
}

/// 👁️ Parses `setResultDisplay`'s `{"sourceId"?, "mode": "static"|"modal"|"buckling", "modeIndex"?}`
/// args into a `ResultDisplay` — unknown/missing `mode` falls back to `Static`.
pub async fn parse_result_display(args: Option<&Value>) -> ResultDisplay {
    let source_id = args.and_then(|v| v.get("sourceId")).and_then(Value::as_str).map(str::to_string);
    let mode_index = args.and_then(|v| v.get("modeIndex")).and_then(Value::as_u64).unwrap_or(0) as usize;
    let mode = match args.and_then(|v| v.get("mode")).and_then(Value::as_str) {
        Some("modal") => DisplayMode::Modal(mode_index),
        Some("buckling") => DisplayMode::Buckling(mode_index),
        _ => DisplayMode::Static,
    };
    ResultDisplay { source_id, mode }
}

/// 📝️ Shared `setResultDisplay` arg declarations for both apps' builders — `sourceId` (a case/
/// combination id), `mode` (static/modal/buckling), and `modeIndex` (0-based, only meaningful for
/// modal/buckling).
pub async fn result_display_action_args() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::text("sourceId", LocalizedLabel::data("Source")),
        ActionArgDef::select(
            "mode",
            LocalizedLabel::data("Mode"),
            vec![ActionArgOption::new("static", LocalizedLabel::data("Static")), ActionArgOption::new("modal", LocalizedLabel::data("Modal")), ActionArgOption::new("buckling", LocalizedLabel::data("Buckling"))],
        ),
        ActionArgDef::number("modeIndex", LocalizedLabel::data("Mode Index")),
    ]
}
//#endregion 🔖️ResultDisplay

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn next_id_retries_past_collisions() {
        let existing = vec!["n0".to_string(), "n2".to_string()];
        assert_eq!(next_id(existing.into_iter(), "n"), "n3");
    }

    #[semio_framework_async_macros::async_test]
    async fn hex_to_rgb01_parses_pure_colors() {
        assert_eq!(hex_to_rgb01("#ffffff"), (1.0, 1.0, 1.0));
        assert_eq!(hex_to_rgb01("#000000"), (0.0, 0.0, 0.0));
        assert_eq!(hex_to_rgb01("#ff0000"), (1.0, 0.0, 0.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn von_mises_color_maps_extremes_midpoint_and_clamps() {
        assert_eq!(von_mises_color(0.0, 0.0, 100.0), VON_MISES_BANDS[0]);
        assert_eq!(von_mises_color(100.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() - 1]);
        assert_eq!(von_mises_color(50.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() / 2]);
        assert_eq!(von_mises_color(-10.0, 0.0, 100.0), VON_MISES_BANDS[0]);
        assert_eq!(von_mises_color(200.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() - 1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_result_display_unknown_mode_falls_back_to_static() {
        assert_eq!(parse_result_display(Some(&serde_json::json!({ "mode": "bogus" }))).mode, DisplayMode::Static);
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_result_display_missing_args_defaults_to_static_with_no_source() {
        let display = parse_result_display(None);
        assert_eq!(display.mode, DisplayMode::Static);
        assert!(display.source_id.is_none());
    }
}
//#endregion 🧪️Tests
