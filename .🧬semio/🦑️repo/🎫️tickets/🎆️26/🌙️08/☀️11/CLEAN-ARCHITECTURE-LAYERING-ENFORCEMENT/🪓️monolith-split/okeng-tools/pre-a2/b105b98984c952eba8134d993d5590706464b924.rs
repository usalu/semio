//! 🌱️ Selected concrete factories consume each original baseline into its actual Store or reset envelope.

/// 🏭️ The child owns original genesis encoding, native publication and installed catalog admission.
pub struct OriginalStoreGenesisFactory<P,M> {
 pub construct:fn(P)->Result<crate::os_store::ArtifactStore<P,M>,crate::os_vcs::VcsError>,
 pub envelope:fn(P)->Result<crate::os_store::ArtifactEnvelope<P,M>,crate::os_vcs::VcsError>,
}
