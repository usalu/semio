//! 📡️ CAD artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `CadMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! cad host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

use crate::op::CadMutation;
use crate::CadSnapshot;
use protocol::OpBinary;


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `CadMutation` to its binary command form.
pub fn encode_op(operation: &CadMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `CadMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<CadMutation, protocol::ProtocolError> {
    CadMutation::decode_op(bytes)
}

//#region 🔖️Store














//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
pub use crate::mutations::{CadMutation, CadNodePatch, CadReferencePatch};

impl protocol::OpBinary for CadMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

/// 🫳️ Borrows the original concrete Cad leaf and its canonical protocol tag before publication.
pub fn prepared_operation_wire_source(operation: &CadMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
    fn source<T: semio_framework_dsl_record::DslField + semio_framework_dsl_record::BorrowedDslRecord>(value: &T) -> Option<store::ArtifactPreparedOperationSource<'_>> {
        let spec=T::RECORD;
        let keyword=spec.keyword?;
        let tag=dsl::protocol_record::records(COMPONENT_PROTOCOL_SEMIO).find(|(kind,_)|*kind==keyword)?.1;
        Some(store::ArtifactPreparedOperationSource::Pack {tag,body:value,spec})
    }
    match operation {
        CadMutation::CreateShapeModel(value)=>source(value),
        CadMutation::DeleteShapeModel(value)=>source(value),
        CadMutation::CreateBuildingModel(value)=>source(value),
        CadMutation::DeleteBuildingModel(value)=>source(value),
        CadMutation::CreateEnergyModel(value)=>source(value),
        CadMutation::DeleteEnergyModel(value)=>source(value),
        CadMutation::CreateStructureClassicModel(value)=>source(value),
        CadMutation::DeleteStructureClassicModel(value)=>source(value),
        CadMutation::CreateDrawing(value)=>source(value),
        CadMutation::DeleteDrawing(value)=>source(value),
        CadMutation::CreateNode(value)=>source(value),
        CadMutation::DeleteNode(value)=>source(value),
        CadMutation::RenameNode(value)=>source(value),
        CadMutation::ChangeReferenceHidden(value)=>source(value),
        CadMutation::ChangeReferenceLocked(value)=>source(value),
        CadMutation::ChangeReferenceWidth(value)=>source(value),
        CadMutation::MoveReference(value)=>source(value),
        CadMutation::ReplaceReferenceMedia(value)=>source(value),
        CadMutation::ReplaceReferences(value)=>source(value),
        CadMutation::CreateBrep(value)=>source(value),
        CadMutation::DeleteBrep(value)=>source(value),
    }
}
