//! 🏪️ The ONE `ArtifactStore` fixture the generation2d schema tests use, built exactly the way the
//! runtime builds it.
//!
//! A bare `ArtifactStore::new` carries NO owner catalog, so the first `Apply` fails closed with
//! `edit history insertion requires its exact mutation retirement factory`
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`). Production installs
//! the framework's bounded document owners through `ArtifactEditor::build_document_store_owners`,
//! and the owned document disposer walks the store to terminal-empty on the way out — this fixture
//! does both. 2d twin of `🧊️generation3d/🧪️tests/🔬️store-fixture/🦀️.rs`
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;

pub type Generation2dFixtureStore = store::ArtifactStore<Generation2dSnapshot, Generation2dMutation>;

/// 🏗️ A document store over `snapshot` carrying the artifact's own owner catalog.
pub async fn document_store(snapshot: Generation2dSnapshot) -> Generation2dFixtureStore {
    let mut store = Generation2dFixtureStore::new(store::create_document_envelope(crate::GENERATION_2D_SCHEMA, "generation2d", snapshot, None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(store::funded_bounded_artifact_store_owners::<Generation2dSnapshot, Generation2dMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    store
}

/// 🧹️ Walks the fixture store to its terminal-empty ownership witness through its exact bounded owner close loop.
pub fn close(mut store: Generation2dFixtureStore) {
    store.close_owned_unscheduled().expect("generation2d document store closes through its exact bounded owners");
}
