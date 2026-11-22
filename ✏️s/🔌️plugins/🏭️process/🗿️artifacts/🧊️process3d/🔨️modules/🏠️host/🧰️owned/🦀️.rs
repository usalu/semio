//! 🏠️ Artifact document-store and publication authorities.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_mutations::standards::v1::subsets::any::io::binary::mutations::{PROCESS3D_MAXIMUM_DOMAIN_ITEMS, PROCESS3D_OWNER_BYTES};
use store::{os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority as _, os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority as _};
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

#[path = "🔓️snapshot-decode/🦀️.rs"]
mod snapshot_decode;
use snapshot_decode::*;

#[path = "🧬️mutation-decode/🦀️.rs"]
mod mutation_decode;
use mutation_decode::*;

#[path = "🪪️decode-owner-catalog/🦀️.rs"]
mod decode_owner_catalog;
pub use decode_owner_catalog::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-laws/🦀️.rs"]
mod retained_laws;
