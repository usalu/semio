use super::*;
use crate::GENERATION_3D_SCHEMA;
use semio_framework_os_kernel::os_store::test_support;
use store::ArtifactDsl;

#[test]
fn dsl_round_trip_empty_projection() {
    test_support::assert_dsl_round_trip(&Generation3dSnapshot::default());
    test_support::assert_dsl_pack_equivalence(&Generation3dSnapshot::default());
}

#[test]
fn dsl_round_trip_every_bundled_example() {
    for text in [
        GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT,
        GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT,
        GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT,
        GENERATION3D_EXAMPLE_BOX_FILLET_TEXT,
        GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT,
        GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT,
        GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT,
        GENERATION3D_EXAMPLE_BOX_SHELL_TEXT,
    ] {
        let projection = Generation3dSnapshot::parse_dsl(text).expect("parse bundled example");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

    let mut store: ArtifactStore<Generation3dSnapshot, Generation3dMutation> = ArtifactStore::new(create_document_envelope(GENERATION_3D_SCHEMA, "generation3d", Generation3dSnapshot::default(), None)).await.expect("valid artifact store fixture");
    use crate::standards::v1::subsets::any::schema::mutations::create_widget::CreateWidget;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![Generation3dMutation::CreateWidget(CreateWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } })], description: None }).await.expect("apply");
    let edit: &Edit<Generation3dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    test_support::assert_command_envelope_round_trip::<Generation3dSnapshot, Generation3dMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
