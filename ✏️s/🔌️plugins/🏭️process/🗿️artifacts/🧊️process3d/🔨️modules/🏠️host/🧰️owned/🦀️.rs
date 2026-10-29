//! 🏠️ Artifact document-store and publication authorities.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_mutations::{standards::v1::subsets::any::io::binary::mutations::process3d_protocol_error, standards::v1::subsets::any::io::binary::mutations::PROCESS3D_MUTATION_BINARY_FORMAT};
use semio_s_artifact_process_process3d_mutations::standards::v1::subsets::any::io::binary::mutations::{PROCESS3D_MAXIMUM_DOMAIN_ITEMS, PROCESS3D_OWNER_BYTES};
use store::{os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority as _, os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority as _};
use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::Process3dSnapshot, snapshot::ProcessMeasure, snapshot::ProcessStep, snapshot::StepOrigin, snapshot::WorkshopMachine};

use semio_s_artifact_process_process3d_host_publication_authority as publication_authority;
pub use publication_authority::*;

use semio_s_artifact_process_process3d_host_retained_cursors as retained_cursors;
use retained_cursors::*;

#[path = "🔓️snapshot-decode/🦀️.rs"]
mod snapshot_decode;
use snapshot_decode::*;

#[path = "🧬️mutation-decode/🦀️.rs"]
mod mutation_decode;
use mutation_decode::*;

#[path = "🪪️decode-owner-catalog/🦀️.rs"]
mod decode_owner_catalog;
pub use decode_owner_catalog::*;

const PROCESS3D_MUTATION_VARIANT_COUNT: usize = 15;

/// 🎟️ Funds one synchronous fixture turn from exactly the demand its own owner quoted.
#[cfg(any(test, feature = "testkit"))]
pub fn process3d_demand_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-laws/🦀️.rs"]
mod retained_laws;
