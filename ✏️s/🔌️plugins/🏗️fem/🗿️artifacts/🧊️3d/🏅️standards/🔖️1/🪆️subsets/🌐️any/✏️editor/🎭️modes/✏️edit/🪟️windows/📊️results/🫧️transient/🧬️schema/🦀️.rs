//! 🧬️ FEM 3D results window-transient schema — the running playback clock.

/// ⏱️ Where the running playback sits this frame: the phase in `0..=1` and, for `pingPong`, the
/// direction it is travelling. Present only while the window plays.
#[derive(Clone, Copy, Debug, PartialEq, dsl::DslRecord, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Fem3dPlaybackClock {
    pub phase: f64,
    pub reverse: bool,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(id = "fem.3d.resultswindowtransient", layout = "lines")]
pub struct Fem3dResultsWindowTransient {
    #[dsl(block)]
    pub clock: Option<Fem3dPlaybackClock>,
}
