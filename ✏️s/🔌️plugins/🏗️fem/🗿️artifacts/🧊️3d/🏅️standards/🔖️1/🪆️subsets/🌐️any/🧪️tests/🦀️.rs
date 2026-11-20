//! 🧪️ Fem3d persistent snapshot laws, kept in a dedicated test crate so the model crate stays inside its source boundary.
#![allow(clippy::result_large_err)]
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;

#[cfg(test)]
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::*;
#[cfg(test)]
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::*;
#[cfg(test)]
use semio_framework_dsl_record::receiving::{NativeSnapshotDecodeOwner, work};
#[cfg(test)]
use semio_framework_dsl_record::{FieldValue, RecordValue};
#[cfg(test)]
use semio_framework_value::retirement::RetireOwned;
#[cfg(test)]
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
#[cfg(test)]
use semio_framework_value::retained_clone::retained_clone_grant_admits;
#[cfg(test)]
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
#[cfg(test)]
use semio_s_fem_material::volumetric::FemMaterialValue;
#[cfg(test)]
use semio_s_fem_node::volumetric::FemNodeValue;

#[cfg(test)]
use semio_framework_test_original_snapshot_owners as original_owners;
#[cfg(test)]
struct Lend(semio_framework_value_testing_admission::Authorities, RetainedCloneGrant);
#[cfg(test)]
impl Lend {
    fn on<R>(&mut self, native: &mut NativeDecodeControl<'_>, operation: impl FnOnce(&mut NativeDecodeControl<'_>) -> Result<R, ValueError>) -> Result<R, ValueError> {
        self.0.lend(native, self.1, operation)
    }
}
#[cfg(test)]
fn with_lend<R>(grant: RetainedCloneGrant, operation: impl FnOnce(&mut Lend) -> R) -> R {
    operation(&mut Lend(semio_framework_value_testing_admission::Authorities::new(), grant))
}

#[cfg(test)]
#[path = "native-tests.rs"]
mod native_tests;
#[cfg(test)]
#[path = "identity-index-tests.rs"]
mod identity_index_tests;
#[cfg(test)]
#[path = "write-pack-tests.rs"]
mod write_pack_tests;
#[cfg(test)]
#[path = "loads/receiver-tests.rs"]
mod load_tests;
#[cfg(test)]
#[path = "examples-tests.rs"]
mod examples_tests;
#[cfg(test)]
#[path = "diff-tests.rs"]
mod diff_tests;
#[cfg(test)]
#[path = "roundtrip-tests.rs"]
mod roundtrip_tests;
