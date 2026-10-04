//! ⏯️ Shared deformation playback transport of every FEM results window (2D and 3D).

/// 🔁️ How the deformation playback clock wraps when the phase leaves `0..=1`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum FemLoopMode {
    #[default]
    #[dsl(key = "loop")]
    Loop,
    #[dsl(key = "pingPong")]
    PingPong,
    #[dsl(key = "once")]
    Once,
}

impl TryFrom<&str> for FemLoopMode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "loop" => Ok(Self::Loop),
            "pingPong" => Ok(Self::PingPong),
            "once" => Ok(Self::Once),
            _ => Err("unknown FEM loop mode"),
        }
    }
}

/// 〰️ The curve the phase is read through before it scales the deformation: `Ramp` grows straight
/// from nothing to the full displacement, `Sine` swings the structure through both signs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum FemWaveform {
    #[default]
    #[dsl(key = "ramp")]
    Ramp,
    #[dsl(key = "sine")]
    Sine,
}

impl TryFrom<&str> for FemWaveform {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ramp" => Ok(Self::Ramp),
            "sine" => Ok(Self::Sine),
            _ => Err("unknown FEM waveform"),
        }
    }
}

/// 🎛️ Deformation playback state of ONE results window — view state, never a document field: the
/// solved displacement field is what the document owns, and this only says how much of it is drawn
/// this frame. `reverse` is the `PingPong` direction (`speed` stays positive in every loop mode).
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FemResultsAnimation {
    pub phase: f64,
    pub playing: bool,
    pub speed: f64,
    pub loop_mode: FemLoopMode,
    pub waveform: FemWaveform,
    pub reverse: bool,
}
