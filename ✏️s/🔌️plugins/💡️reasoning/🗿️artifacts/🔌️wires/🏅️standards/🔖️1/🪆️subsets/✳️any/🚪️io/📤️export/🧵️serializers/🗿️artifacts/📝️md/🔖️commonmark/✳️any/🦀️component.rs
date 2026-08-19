//! 🚪️ wires -> md — foreign `Serializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling
//! `Deserializer`: wraps the full `.wires` DSL text losslessly inside `md`'s own binary pack, so
//! `IoFidelity::Canonical`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::MdSnapshot;

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct WiresIntoMd;

impl Serializer<WiresSnapshot> for WiresIntoMd {
    const INTO: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn serialize(from: &WiresSnapshot) -> IoResult<IoPayload> {
        let md = MdSnapshot::from_text(&<WiresSnapshot as store::ArtifactDsl>::print_dsl(from));
        Ok(IoOutcome::clean(IoPayload::Binary(<MdSnapshot as store::ArtifactPack>::encode_pack(&md))))
    }
}
