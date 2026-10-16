//! 📣️ The selected actual native producer publishes its declared snapshot codec after original Store validation.

/// 🏭️ Publication preserves the original atomic registry and its typed native provenance.
pub trait ArtifactStorePublication<P> {
 fn publish_native_snapshot()->Result<(),crate::io::ArtifactAssemblyRegistryError>;
}

/// 🧰️ Selects the authored native registration of a generic snapshot owner.
pub struct ArtifactSnapshotPublication;
impl<P:crate::os_store::serialization::ArtifactStoreSnapshot> ArtifactStorePublication<P> for ArtifactSnapshotPublication {
 fn publish_native_snapshot()->Result<(),crate::io::ArtifactAssemblyRegistryError>{crate::io::publish_native_snapshot::<P>()}
}

impl<P:crate::os_store::serialization::ArtifactStoreSnapshot> ArtifactStorePublication<P> for crate::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer {
 fn publish_native_snapshot()->Result<(),crate::io::ArtifactAssemblyRegistryError>{crate::io::publish_native_snapshot::<P>()}
}
