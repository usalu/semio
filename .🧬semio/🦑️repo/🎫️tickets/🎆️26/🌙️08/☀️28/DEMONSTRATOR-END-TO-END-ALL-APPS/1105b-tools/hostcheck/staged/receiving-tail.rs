
#[path="🔢️binary/🦀️.rs"]
pub mod binary;
pub use binary::OriginalBinaryReceiving;
impl<M:semio_framework_replication::OpBinaryReceiving+Send+Sync+'static> ArtifactOperationWire<M> where M::Receiving:Send{
 /// 🔢️ Registration binds the vocabulary's own binary receiving cursor for persisted operations; the installer names the encoding, the receiver never sniffs bytes.
 pub fn with_original_binary_receiving(mut self)->Self{self.original_receiving=Some(OriginalOperationReceivingAdmission::binary(crate::lifted_member_owned_batch::BatchRetirementIssuer::typed()));self}
}
