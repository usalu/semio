//! 🚪️ vcs -> zip — foreign `Serializer<VcsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Pre-migration behavior
//! preserved verbatim (a plain structural `DslValue` coercion): `VcsSnapshot`'s fields have no
//! counterpart in `ZipSnapshot`'s `{schema,entries,comment}` shape, so only `schema` survives,
//! hence `IoFidelity::Lossy`.

use crate::artifacts::vcs::VcsSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::zip::ZipSnapshot;

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct VcsIntoZip;

impl Serializer<VcsSnapshot> for VcsIntoZip {
    const INTO: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &VcsSnapshot) -> IoResult<IoPayload> {
        let zip = ZipSnapshot::from_value(from.to_value()).map_err(|error| IoError { message: format!("VcsIntoZip: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<ZipSnapshot as store::ArtifactPack>::encode_pack(&zip))))
    }
}
