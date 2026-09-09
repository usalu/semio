//! 🏪️ The ONE `ArtifactStore` fixture the generation2d schema tests use, built exactly the way the
//! runtime builds it.
//!
//! A bare `ArtifactStore::new` carries NO owner catalog, so the first `Apply` fails closed with
//! `edit history insertion requires its exact mutation retirement factory`
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`). Production installs
//! `generation2d_document_store_owners()` through `ArtifactEditor::build_document_store_owners`,
//! and the owned document disposer walks the store to terminal-empty on the way out — this fixture
//! does both. 2d twin of `🧊️generation3d/🧪️tests/🔬️store-fixture/🦀️.rs`
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_document_store_owners;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_plugin::ArtifactOwnedDisposer;

pub type Generation2dFixtureStore = store::ArtifactStore<Generation2dSnapshot, Generation2dMutation>;

/// 🏗️ A document store over `snapshot` carrying the artifact's own owner catalog.
pub async fn document_store(snapshot: Generation2dSnapshot) -> Generation2dFixtureStore {
    let mut store = Generation2dFixtureStore::new(store::create_document_envelope(crate::GENERATION_2D_SCHEMA, "generation2d", snapshot, None)).await.expect("valid artifact store fixture");
    store.install_member_store_owners_exact(generation2d_document_store_owners());
    store
}

/// 🧹️ Walks the fixture store to its terminal-empty ownership witness through the same
/// `ArtifactDocumentStoreDisposer` the editor declares.
pub fn close(mut store: Generation2dFixtureStore) {
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<Generation2dSnapshot, Generation2dMutation>::new();
    for _ in 0..1_000_000 {
        if matches!(disposer.close_step(&mut store, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("generation2d document store close step"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(std::thread::panicking() || disposer.terminal_is_empty(&store), "generation2d fixture store did not reach its terminal-empty ownership witness");
}
