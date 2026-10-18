//! 🏠️ Artifact document-store and publication authorities.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_mutations::{standards::v1::subsets::any::io::binary::mutations::process3d_protocol_error, standards::v1::subsets::any::io::binary::mutations::PROCESS3D_MUTATION_BINARY_FORMAT};
use semio_s_artifact_process_process3d_mutations::standards::v1::subsets::any::io::binary::mutations::{PROCESS3D_MAXIMUM_DOMAIN_ITEMS, PROCESS3D_OWNER_BYTES};
use store::{os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority as _, os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority as _};
use semio_framework_plugin::ArtifactOwnedDisposer as _;
use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::CapabilityParameter, snapshot::CapabilityRule, snapshot::MeasureRecipe, snapshot::Pose, snapshot::Process3dSnapshot, snapshot::ProcessMeasure, snapshot::ProcessStep, snapshot::StepOrigin, snapshot::Stock, snapshot::StockQuantity, snapshot::WorkingSolid, snapshot::WorkshopMachine};

#[path = "📬️publication-authority/🦀️.rs"]
mod publication_authority;
pub use publication_authority::*;

#[path = "🚀️store-initialization/🦀️.rs"]
mod store_initialization;
pub use store_initialization::*;

#[path = "🧮️census-copy-timeline/🦀️.rs"]
mod census_copy_timeline;
pub use census_copy_timeline::*;

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

const PROCESS3D_MAXIMUM_DOMAIN_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

pub const PROCESS3D_MOUNTED_OUTPUT_CHANNELS: usize = 4;

pub const PROCESS3D_MOUNTED_CONTROL_CREDITS: usize = 1;

const PROCESS3D_PUBLICATION_SLOTS: usize = 4;

const PROCESS3D_RETAINED_STACK_CAPACITY: usize = 64;

const PROCESS3D_MAXIMUM_OUTPUT_PAGES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;

const PROCESS3D_MUTATION_VARIANT_COUNT: usize = 15;

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-laws/🦀️.rs"]
mod retained_laws;
