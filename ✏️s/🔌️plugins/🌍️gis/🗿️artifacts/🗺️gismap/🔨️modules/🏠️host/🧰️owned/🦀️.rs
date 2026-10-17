//! 🏠️ Artifact document-store and publication authorities.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::{GisMapSnapshot, MapFeature};
use protocol::{Mutation, OpBinary};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::RetirementDemand;

#[path = "♻️retirement-and-clone/🦀️.rs"]
mod retirement_and_clone;
pub use retirement_and_clone::*;

#[path = "🏁️initialization-authority/🦀️.rs"]
mod initialization_authority;
use initialization_authority::*;

#[path = "🔚️initialization-close/🦀️.rs"]
mod initialization_close;
pub use initialization_close::*;

#[path = "🔡️owned-field-decode/🦀️.rs"]
mod owned_field_decode;
pub use owned_field_decode::*;

#[path = "📇️owned-field-catalog/🦀️.rs"]
mod owned_field_catalog;
pub use owned_field_catalog::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
