#[semio_framework_async_macros::async_test]
async fn cad_imported_geometry_members_install_exact_typed_publication_authority() {
    use semio_framework_os_kernel::{ArtifactStore, MemberStoreOwner, MemberStoreOwnedBatch, MemberStoreOwnedBatchRequest, SpaceMember};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::{model::schema::{snapshot::SemioModelSnapshot, mutations::{SemioModelMutation,insert_element::InsertElement}},brep::schema::{snapshot::SemioBrepSnapshot,mutations::{SemioBrepMutation,create_vertex::CreateVertex}}};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let text = include_str!("../../../../../../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧫️fixtures/📦️set-object-applied/⬅️before.obj");
    let imported = crate::standards::v1::subsets::any::io::import_obj_object(text).unwrap();
    macro_rules! member {
        ($snapshot:ty, $mutation:ty, $operation:expr, $index:expr) => {{
            let expected = &corpus["members"][$index];
            let operation: $mutation = $operation;
            let semantic = protocol::SemanticMutation::semantics(&operation);
            assert_eq!((semantic.entity,semantic.kind),(expected["entity"].as_str().unwrap(),expected["kind"].as_str().unwrap()));
            let original = vec![operation];
            let pointer = original.as_ptr();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 1048576, maximum_release_bytes: 1048576, maximum_depth: 64 };
            let (mutations,_) = MemberStoreOwnedBatch::try_new(original,grant).unwrap_or_else(|_| panic!("exact original imported mutation owner"));
            let envelope = semio_framework_os_kernel::create_document_envelope("stdio.semio","neutral-member",<$snapshot>::default(),None);
            let mut store = ArtifactStore::<$snapshot,$mutation>::new(envelope,protocol::ActorId("fixture-member".into())).await.unwrap();
            store.install_document_store_owners_exact(<$snapshot as MemberStoreOwner<$mutation>>::member_store_owners());
            let (generation,revision) = store.one_item_publication_identity();
            let mut request = MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(9131),expected_generation:generation,expected_revision:revision,actor:"fixture-member".into(),group_id:Some("neutral-imported-group".into()),transaction:None,mutations };
            let admission = store.owned_publication_birth_bytes(&request);
            assert_eq!(request.mutations.mutations::<$mutation>().unwrap().as_ptr(),pointer);
            for _ in 0..100000 {
                if request.mutations.terminal_is_empty() { break; }
                let (birth,release) = request.mutations.next_demands().unwrap();
                let step = request.mutations.close_granted(RetainedCloneGrant { maximum_capacity_bytes:birth,maximum_release_bytes:release,..grant }).unwrap();
                assert!(step.progress().fits(RetainedCloneGrant { maximum_capacity_bytes:birth,maximum_release_bytes:release,..grant }));
            }
            assert!(request.mutations.terminal_is_empty());
            for _ in 0..100000 {
                if store.close_owned_terminal_is_empty() { break; }
                let demand = store.close_owned_byte_demand();
                assert!(demand <= 1048576);
                store.close_owned_step(1,demand).unwrap();
            }
            assert!(store.close_owned_terminal_is_empty());
            assert!(admission.is_ok(), "actual installed {} member requires its exact typed preparation authority: {admission:?}",expected["subset"]);
            println!("[DEBUG] actual installed imported {} member typed birth={} original vector pointer retained, complete bounded owner closure",expected["subset"],admission.unwrap());
        }};
    }
    member!(SemioModelSnapshot,SemioModelMutation,SemioModelMutation::InsertElement(InsertElement { element:imported.element, at: None }),0);
    member!(SemioBrepSnapshot,SemioBrepMutation,SemioBrepMutation::CreateVertex(CreateVertex { id:imported.geometry.vertices[0].id.clone(), point:imported.geometry.vertices[0].point, tol:imported.geometry.vertices[0].tol, at:None }),1);
}

include!("🌱️birth/🦀️.rs");
