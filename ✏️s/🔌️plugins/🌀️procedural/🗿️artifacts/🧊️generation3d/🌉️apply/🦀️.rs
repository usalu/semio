//! 🌉️ Central apply entry points: the only code of this artifact that turns a mutation's diff into the next snapshot with `protocol::apply_diff`; the schema and mutation leaves only build diffs and inverses.
#![allow(unused_imports)]

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::{widget_id, Generation3dSnapshot};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{ArtifactEnvelope, ArtifactStore};

/// 🎬️ Fallible in-place `vcs::apply_mutation` boundary. A diff builder that REJECTED the mutation
/// answers `MutationOutcome::{error,fatal}`, whose diff side is forced to `Default` (LAW 1,
/// `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1061-1069`) — applying that empty delta
/// would return the unchanged base as implicit success, exactly what [`protocol::MutationDiff`]'s
/// own contract forbids. The rejection is raised here instead, so a caller's `Ok` is a real witness
/// that the mutation landed. The refusal travels as the outcome's own messages, codes and levels unchanged; an apply-time
/// rejection joins them as the `Fatal` `mutation.apply.*` message `MutationOutcome::apply_to` would persist — a vocabulary
/// code is never re-typed as an apply error.
pub fn apply_generation3d_mutation(projection: &mut Generation3dSnapshot, mutation: &Generation3dMutation) -> Result<(), Vec<protocol::MutationMessage>> {
    let (delta, messages) = protocol::Mutation::diff(mutation, &*projection).into_parts();
    if messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        delta.retire_cold();
        return Err(messages);
    }
    let applied = protocol::apply_diff(&delta, &*projection);
    delta.retire_cold();
    match applied {
        Ok(next) => {
            std::mem::replace(projection, next).retire_cold();
            Ok(())
        }
        Err(error) => Err(messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect()),
    }
}

/// 🎚️ The slider fields after `target` lands on it (the range widened exactly like the canvas knob), computed on a probe copy; `None` when no range can hold the value.
pub(crate) fn generation3d_slider_landing(value: f64, min: f64, max: f64, step: f64, target: f64) -> Option<(f64, f64, f64, f64)> {
    let mut landing = semio_framework_artifact_flow_flow::Widget::InputSlider { id: String::new(), label: String::new(), value, min, max, step };
    if !semio_framework_artifact_flow_flow::set_widget_slider_value(&mut landing, target) {
        return None;
    }
    match landing {
        semio_framework_artifact_flow_flow::Widget::InputSlider { value, min, max, step, .. } => Some((value, min, max, step)),
        _ => None,
    }
}
