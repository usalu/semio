//! 🚪️ vcs -> xlsx — foreign `Serializer<VcsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Pre-migration behavior
//! preserved verbatim (a plain `serde_json` struct coercion): `VcsSnapshot`'s fields have no
//! counterpart in `XlsxSnapshot`'s `{schema,opc,workbook}` shape, so only `schema` survives, hence
//! `IoFidelity::Lossy`.

use crate::artifacts::vcs::VcsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::xlsx::XlsxSnapshot;

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct VcsIntoXlsx;

impl Serializer<VcsSnapshot> for VcsIntoXlsx {
    const INTO: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &VcsSnapshot) -> IoResult<IoPayload> {
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("VcsIntoXlsx: {error}"), diagnostics: Vec::new() })?;
        let xlsx: XlsxSnapshot = serde_json::from_value(value).map_err(|error| IoError { message: format!("VcsIntoXlsx: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<XlsxSnapshot as store::ArtifactPack>::encode_pack(&xlsx))))
    }
}
