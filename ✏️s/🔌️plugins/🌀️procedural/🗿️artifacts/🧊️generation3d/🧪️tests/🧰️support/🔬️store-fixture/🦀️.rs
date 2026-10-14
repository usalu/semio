//! 🏪️ The ONE `ArtifactStore` fixture the schema tests use, built exactly the way the runtime builds
//! it.
//!
//! A bare `ArtifactStore::new` carries NO owner catalog, so the first `Apply` fails closed with
//! `edit history insertion requires its exact mutation retirement factory`
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15216`). Production installs
//! the framework's bounded document owners through `ArtifactEditor::build_document_store_owners`, and
//! the owned document disposer walks the store to terminal-empty on the way out — this fixture does
//! both (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub type Generation3dFixtureStore = store::ArtifactStore<Generation3dSnapshot, Generation3dMutation>;

/// 🏗️ A document store over `snapshot` carrying the artifact's own owner catalog.
pub async fn document_store(snapshot: Generation3dSnapshot) -> Generation3dFixtureStore {
    let mut store = Generation3dFixtureStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(store::os_store::document_materialize::create_document_envelope(semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA, "generation3d", snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Generation3dSnapshot, Generation3dMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    store
}

/// 🧹️ Walks the fixture store to its terminal-empty ownership witness through its exact bounded owner close loop.
pub fn close(mut store: Generation3dFixtureStore) {
    store.close_owned_unscheduled().expect("generation3d document store closes through its exact bounded owners");
}
