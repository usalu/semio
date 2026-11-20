//! 🍰️ Native diff laws run against Layout's public mutation and retirement entry points.
use semio_os_kernel as protocol;
use semio_os_kernel::{RetainedDone as Done,RetainedReply as Reply,RetainedDiffPreflightStep,retained_conclude,retained_settle};
use semio_s_artifact_layout_layout_diff::*;
use semio_s_artifact_layout_layout_model::*;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_value::{NativeDecodeControl,OriginalRetirementFailure,RetirementDemand,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[path = "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📽️projection/👁️post/🧪️testing/🦀️.rs"]
mod post_fixture;
#[path = "../🔬️unit/🦀️.rs"]
mod unit;
#[path = "../⚖️portable/🦀️.rs"]
mod portable;
#[path = "../../🧳️retained/🪡️apply/🧪️tests/🦀️.rs"]
mod apply;
#[path = "../../🧳️retained/🪡️apply/👁️final-graph/🧪️tests.rs"]
mod final_graph;
#[path = "../../🧳️retained/📦️collection/🧪️tests/🦀️.rs"]
mod collection;
