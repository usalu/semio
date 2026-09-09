//! 🏪️ The ONE `ArtifactStore` fixture the schema tests use, built exactly the way the runtime builds
//! it.
//!
//! A bare `ArtifactStore::new` carries NO owner catalog, so the first `Apply` fails closed with
//! `edit history insertion requires its exact mutation retirement factory`
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15216`). Production installs
//! `generation3d_document_store_owners()` through `ArtifactEditor::build_document_store_owners`
//! (`✏️editor/🦀️.rs:988`), and the owned document disposer walks the store to terminal-empty on the
//! way out — this fixture does both (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_document_store_owners;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactOwnedDisposer;

pub type Generation3dFixtureStore = store::ArtifactStore<Generation3dSnapshot, Generation3dMutation>;

/// 🏗️ A document store over `snapshot` carrying the artifact's own owner catalog.
pub async fn document_store(snapshot: Generation3dSnapshot) -> Generation3dFixtureStore {
    let mut store = Generation3dFixtureStore::new(store::create_document_envelope(crate::GENERATION_3D_SCHEMA, "generation3d", snapshot, None)).await.expect("valid artifact store fixture");
    store.install_member_store_owners_exact(generation3d_document_store_owners());
    store
}

/// 🧹️ Walks the fixture store to its terminal-empty ownership witness through the same
/// `ArtifactDocumentStoreDisposer` the editor declares.
pub fn close(mut store: Generation3dFixtureStore) {
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<Generation3dSnapshot, Generation3dMutation>::new();
    for _ in 0..1_000_000 {
        if matches!(disposer.close_step(&mut store, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("generation3d document store close step"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(std::thread::panicking() || disposer.terminal_is_empty(&store), "generation3d fixture store did not reach its terminal-empty ownership witness");
}
