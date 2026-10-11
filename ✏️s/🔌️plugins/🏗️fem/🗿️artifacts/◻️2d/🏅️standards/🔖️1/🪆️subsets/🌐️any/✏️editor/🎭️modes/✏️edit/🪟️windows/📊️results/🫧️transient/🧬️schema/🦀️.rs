//! 🧬️ FEM results window-transient schema — the running playback clock.

/// ⏱️ Where the running playback sits this frame: the phase in `0..=1` and, for `pingPong`, the
/// direction it is travelling. Present only while the window plays.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FemPlaybackClock {
    pub phase: f64,
    pub reverse: bool,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(layout = "lines")]
#[artifact(id = "fem.resultswindowtransient")]
pub struct FemResultsWindowTransient {
    #[dsl(block)]
    pub clock: Option<FemPlaybackClock>,
}

/// 🔺️ Owned-field diff of [`FemResultsWindowTransient`]: a present change carries the clock the window now holds, `None` meaning it stopped.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct FemResultsWindowTransientDiff {
    pub clock: Option<FemPlaybackClockChange>,
}

/// 🔺️ One clock publication inside a [`FemResultsWindowTransientDiff`]; the inner `None` clears the clock.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FemPlaybackClockChange {
    pub clock: Option<FemPlaybackClock>,
}
