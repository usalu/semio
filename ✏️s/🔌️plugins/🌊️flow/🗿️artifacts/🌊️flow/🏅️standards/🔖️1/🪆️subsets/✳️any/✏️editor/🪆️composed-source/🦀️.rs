//! 🪆️ Retained original SemioFlow fields build the actual stable execution scene.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use flow::FlowJsonField as JsonField;
use semio_framework_pack_json::{JsonReadSource,JsonMemberPolicy};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original composed Flow source changed its funded frontier")}
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
#[path="📥️field/🦀️.rs"]mod field;
#[path="❓️unknown/🦀️.rs"]mod unknown;
#[path="🧩️node/🦀️.rs"]mod node;
#[path="🧵️assembly/🦀️.rs"]mod assembly;
pub use assembly::{FlowComposedSourceWork,FlowComposedRead,FlowComposedScene};

pub use node::{NodeWork as FlowNodeReceiveWork,NodeOutput as FlowNodeReceiveOutput};
