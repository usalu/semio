use super::*;
use crate::editor::wires::unit_tests::context::{dispatch, metabolism_app, new_app};
use crate::editor::wires::WiresCommand;
use crate::schema::fixture_nodes;

/// 🧬️ Whole-document replace is not an in-history mutation (a whole-snapshot variant is banned
/// outright), so `setActiveExample` now surfaces as a `Effect::LoadDocument` carrying the
/// example document's pack bytes rather than an `artifact_mutations` entry — `dispatch`'s
/// in-process harness never applies `effects` to its own store, so this asserts on the emitted
/// effect directly (mirrors `🎮️commands/📚️example`-style facets elsewhere in this ticket).
#[semio_framework_async_macros::async_test]
async fn set_active_example_metabolism_loads_seven_nodes() {
    use semio_framework_plugin::Effect;
    let mut app = new_app().await;
    let result = dispatch(&mut app, WiresCommand::SetActiveExample(SetActiveExample { example_id: WIRES_PLAY_EXAMPLE_METABOLISM_ID.into() })).await;
    assert!(result.mutations.is_empty(), "setActiveExample replaces the whole document via an effect, not in-history mutations");
    let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let document = <crate::WiresSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert_eq!(fixture_nodes(&crate::wires_working_board(&document)).len(), 7);
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_unknown_id_loads_empty_document() {
    use semio_framework_plugin::Effect;
    let mut app = metabolism_app().await;
    let result = dispatch(&mut app, WiresCommand::SetActiveExample(SetActiveExample { example_id: "nope".into() })).await;
    let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let document = <crate::WiresSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert!(fixture_nodes(&crate::wires_working_board(&document)).is_empty());
}
