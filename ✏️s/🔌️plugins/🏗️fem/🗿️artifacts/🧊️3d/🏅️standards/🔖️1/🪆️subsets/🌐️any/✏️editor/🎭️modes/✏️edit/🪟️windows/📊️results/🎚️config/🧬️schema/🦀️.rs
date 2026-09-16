//! 🧬️ FEM 3D results window-config schema.

/// 🔁️ How the deformation playback clock wraps when the phase leaves `0..=1`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, dsl::DslScalar, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Fem3dLoopMode {
    #[default]
    #[dsl(key = "loop")]
    Loop,
    #[dsl(key = "pingPong")]
    PingPong,
    #[dsl(key = "once")]
    Once,
}

impl TryFrom<&str> for Fem3dLoopMode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "loop" => Ok(Self::Loop),
            "pingPong" => Ok(Self::PingPong),
            "once" => Ok(Self::Once),
            _ => Err("unknown FEM 3D loop mode"),
        }
    }
}

/// 〰️ The curve the phase is read through before it scales the deformation: `Ramp` grows straight
/// from nothing to the full displacement, `Sine` swings the structure through both signs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, dsl::DslScalar, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Fem3dWaveform {
    #[default]
    #[dsl(key = "ramp")]
    Ramp,
    #[dsl(key = "sine")]
    Sine,
}

impl TryFrom<&str> for Fem3dWaveform {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ramp" => Ok(Self::Ramp),
            "sine" => Ok(Self::Sine),
            _ => Err("unknown FEM 3D waveform"),
        }
    }
}

/// ⏯️ Deformation playback state of ONE results window — view state, never a document field: the
/// solved displacement field is what the document owns, and this only says how much of it is drawn
/// this frame. `reverse` is the `PingPong` direction (`speed` stays positive in every loop mode).
#[derive(Clone, Copy, Debug, PartialEq, dsl::DslRecord, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fem3dResultsAnimation {
    pub phase: f64,
    pub playing: bool,
    pub speed: f64,
    pub loop_mode: Fem3dLoopMode,
    pub waveform: Fem3dWaveform,
    pub reverse: bool,
}

#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "fem.3d.resultswindowconfig", layout = "lines")]
pub struct Fem3dResultsWindowConfig {
    #[dsl(block)]
    pub camera: crate::Viewport3dOrbit,
    pub result_source_id: Option<String>,
    pub result_mode: crate::app_surface::ResultMode,
    pub result_mode_index: u32,
    #[dsl(block)]
    pub animation: Fem3dResultsAnimation,
}
