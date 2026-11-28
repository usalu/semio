//! 🤝️ FEM plugin — helpers shared by the 2D and 3D apps' `ui` crates (non-constitutional; see the
//! constitutional-split recipe's "shared code used by ≥2 apps" rule). Everything here is used by BOTH
//! `fem2d_ui` and `fem3d_ui` — hex/von-Mises color helpers, mode-shape
//! normalization, and the result-window display projection.

#[path = "🧬️schema/👁️result-mode/🦀️.rs"]
mod result_mode;
pub use result_mode::ResultMode;
#[path = "🧬️schema/🔣️results-animation/🦀️.rs"]
mod results_animation;
pub use results_animation::FemLoopMode;
pub use results_animation::FemResultsAnimation;
pub use results_animation::FemWaveform;

use semio_s_fem_model::Dof;
use semio_plugin::ActionArgDef;
use semio_plugin::ActionArgOption;
use semio_plugin::BuiltNode;
use semio_ui_contract::Buildable;
use semio_ui_contract::HasChildren;
use semio_ui_contract::HasStackLayout;
use semio_ui_locale::LocalizedLabel;
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
/// 🎬️ Encodes one Canvas2d scene into the semantic UI contract — its spine inside the fixed surface
/// doc and every declared payload lane beside it as a paged carrier (`scene_surface`).
pub fn canvas_2d_surface(id: impl Into<String>, scene: &semio_ui_scene::Canvas2dScene, name: &str) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    semio_plugin::scene_surface(&id.into(), semio_ui_contract::SurfaceKind::Canvas2d, scene, name, None)
}

/// 🌍️ Encodes one World3d scene into the semantic UI contract: the meshes/instances lanes ride as
/// paged carriers, so a meshed solid with per-vertex stress is bounded by carrier pages rather than
/// refused by the 32 KiB surface doc (`ui.scene.encode: World3d scene admission failed`).
pub fn world_3d_surface(id: impl Into<String>, scene: &semio_ui_scene::World3dScene, name: &str) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    semio_plugin::scene_surface(&id.into(), semio_ui_contract::SurfaceKind::World3d, scene, name, None)
}

/// 🏷️ The caption a read-only viewer carries above its model: the load case its loads belong to, `∅` when the document has none.
pub fn viewer_case_caption(load_case: Option<&str>, locale: semio_plugin::Locale) -> String {
    format!("{}: {}", semio_plugin::surface_name(locale, "Case", "Lastfall"), load_case.unwrap_or("∅"))
}

/// 🏷️ Places the viewer's data caption above its scene; both enter the column as builders so the column keys them positionally.
pub fn with_viewer_caption(scene: BuiltNode, load_case: Option<&str>, locale: semio_plugin::Locale) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let admission = |detail: &'static str| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", detail);
    let caption = semio_ui_contract::Label::try_from(viewer_case_caption(load_case, locale).as_str()).map_err(|_| admission("fem viewer caption label admission failed"))?;
    let scene_stack = semio_ui_contract::column().grow(true).try_child(scene).map_err(|_| admission("fem viewer caption scene child admission failed"))?;
    semio_ui_contract::column()
        .grow(true)
        .try_child(semio_ui_contract::text(caption))
        .map_err(|_| admission("fem viewer caption text admission failed"))?
        .try_child(scene_stack)
        .map_err(|_| admission("fem viewer caption scene stack admission failed"))?
        .try_build()
        .map_err(|_| admission("fem viewer caption node admission failed"))
}

/// 🖱️ The Canvas2d meta layer a read-only viewer carries: primary and secondary drag pan, as in the editor's Direct Select, and nothing else is armed.
pub fn canvas_2d_viewer_meta_layer() -> semio_framework_pack_json::Value {
    semio_framework_pack_json::json!({ "id": "meta:utility", "role": "meta", "utility": "selectDirect", "leftDragPans": true, "rightDragPans": true })
}

/// 🎨️ Projects the shared color-input parser into Canvas2d RGB, using black for refused input.
pub fn hex_to_rgb01(hex: &str) -> (f64, f64, f64) {
    let rgba = semio_ui_contract::parse_ui_color_hex(hex).unwrap_or([0.0; 4]);
    (rgba[0], rgba[1], rgba[2])
}

/// 📐️ Rescales a node-id-keyed displacement map in place so its largest translational magnitude
/// (`sqrt(tx²+ty²+tz²)`) becomes exactly 1.0 — mode shapes from `subspace_iteration` are mass/Kg-
/// orthonormalized (arbitrary physical magnitude), so this gives a deterministic, comparable-across-
/// modes amplitude before scaling by `MODE_SHAPE_AMPLITUDE_RATIO * model_extent`. A near-zero shape
/// (degenerate/rigid mode) is left untouched rather than divided by a near-zero magnitude.
pub fn normalize_mode_shape(disp_map: &mut HashMap<String, [f64; 6]>) {
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
pub fn von_mises_color(value: f64, min: f64, max: f64) -> &'static str {
    let span = (max - min).max(1e-9);
    let t = ((value - min) / span).clamp(0.0, 1.0);
    let index = ((t * (VON_MISES_BANDS.len() - 1) as f64).round() as usize).min(VON_MISES_BANDS.len() - 1);
    VON_MISES_BANDS[index]
}
//#endregion 🔖️Shared

//#region ⏯️Playback
/// ⏱️ The playback frame delta, in milliseconds. A wasm guest has no monotonic clock it may read
/// inside a command, so the tick advances the phase by a FIXED delta and the host's
/// `Effect::DispatchAction { delay_ms }` is what keeps that delta honest (~30 fps).
pub const ANIMATION_TICK_MS: u64 = 33;

/// ⏲️ [`ANIMATION_TICK_MS`] in seconds — `speed` is stated in cycles per second.
pub const ANIMATION_TICK_SECONDS: f64 = ANIMATION_TICK_MS as f64 / 1_000.0;

/// 🐢️ Slowest playback the transport admits, in cycles per second.
pub const ANIMATION_SPEED_MINIMUM: f64 = 0.05;

/// 🐇️ Fastest playback the transport admits, in cycles per second.
pub const ANIMATION_SPEED_MAXIMUM: f64 = 4.0;

/// ⏭️ One transport step — a twenty-fourth of a cycle, the classic film frame.
pub const ANIMATION_PHASE_STEP: f64 = 1.0 / 24.0;

impl Default for FemResultsAnimation {
    /// 🎞️ A still results window sits at phase 1 — the FULL deformed shape, exactly what the window
    /// drew before playback existed. Phase 0 would open every results window on an undeformed
    /// structure, so the resting pose is the end of the ramp, not its start.
    fn default() -> Self {
        Self { phase: 1.0, playing: false, speed: 0.5, loop_mode: FemLoopMode::Loop, waveform: FemWaveform::Ramp, reverse: false }
    }
}

impl FemResultsAnimation {
    /// 〰️ The signed factor the solved displacement field is scaled by this frame. `Sine` returns
    /// negative values on purpose: the structure swings through both signs instead of only growing.
    pub fn amplitude(&self) -> f64 {
        match self.waveform {
            FemWaveform::Ramp => self.phase,
            FemWaveform::Sine => (std::f64::consts::TAU * self.phase).sin(),
        }
    }

    /// ▶️ Arms playback: a `Once` run that already sits at its end rewinds, so the play button is
    /// never a control that visibly does nothing.
    pub fn start(&mut self) {
        if self.loop_mode == FemLoopMode::Once && self.phase >= 1.0 {
            self.phase = 0.0;
        }
        self.playing = true;
    }
}

impl FemLoopMode {
    /// 🔑️ The wire key a transport select control carries.
    pub fn key(self) -> &'static str {
        match self {
            Self::Loop => "loop",
            Self::PingPong => "pingPong",
            Self::Once => "once",
        }
    }
}

impl FemWaveform {
    /// 🗝️ The wire key a transport select control carries.
    pub fn key(self) -> &'static str {
        match self {
            Self::Ramp => "ramp",
            Self::Sine => "sine",
        }
    }
}
//#endregion ⏯️Playback

//#region 🔖️ResultDisplay
/// 👁️ Render projection of the persisted configuration of one exact results window.
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

impl DisplayMode {
    /// 🎵️ Both eigenvalue analyses display normalized mode shapes.
    pub fn uses_mode_shape(self) -> bool {
        matches!(self, Self::Modal(_) | Self::Buckling(_))
    }

    /// 📏️ Modal eigenvalues become hertz; buckling eigenvalues remain load multipliers.
    pub fn display_eigenvalue(self, eigenvalue: f64) -> Option<f64> {
        match self {
            Self::Static => None,
            Self::Modal(_) => Some(eigenvalue.max(0.0).sqrt() / std::f64::consts::TAU),
            Self::Buckling(_) => Some(eigenvalue),
        }
    }
}

impl ResultMode {
    pub fn display(self, index: u32) -> DisplayMode {
        match self {
            Self::Static => DisplayMode::Static,
            Self::Modal => DisplayMode::Modal(index as usize),
            Self::Buckling => DisplayMode::Buckling(index as usize),
        }
    }
}

/// 🫙️ The wire id of the empty document, the Artifact panel combobox's `No example` row and the palette's first option.
pub const EMPTY_EXAMPLE_ID: &str = "";

/// 🏷️ The label of the empty example row, English first then German.
pub fn no_example_label() -> LocalizedLabel {
    LocalizedLabel::native("No example", "Kein Beispiel")
}

/// 📚️ The options every FEM surface offers for the same bundled examples: the empty example, then each registered example in registration order with its own label.
pub fn example_options(examples: &[semio_plugin::ExampleSource]) -> Vec<ActionArgOption> {
    std::iter::once(ActionArgOption::new(EMPTY_EXAMPLE_ID, no_example_label())).chain(examples.iter().map(|example| ActionArgOption::new(example.id(), example.label().clone()))).collect()
}

/// 📚️ The `exampleId` select of `setActiveExample`, derived from the registered examples so the palette and the Artifact panel combobox cannot name one example two ways.
pub fn set_active_example_arg(examples: &[semio_plugin::ExampleSource], default_id: &str) -> ActionArgDef {
    ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), example_options(examples)).default_value(&default_id)
}

/// 📝️ Shared `setResultDisplay` arg declarations for both apps' builders — `sourceId` (a case/
/// combination id), `mode` (static/modal/buckling), and `modeIndex` (0-based, only meaningful for
/// modal/buckling).
pub fn result_display_action_args() -> Vec<ActionArgDef> {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/👁️result-projection/🦀️.rs"]
mod result_projection_tests;
//#endregion 🧪️Tests
