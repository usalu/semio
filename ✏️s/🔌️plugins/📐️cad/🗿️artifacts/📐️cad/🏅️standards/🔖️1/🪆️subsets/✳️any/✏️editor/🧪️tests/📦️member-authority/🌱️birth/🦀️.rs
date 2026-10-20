#[test]
fn cad_imported_geometry_catalog_birth_matches_original_system_allocations() {
    use semio_framework_os_kernel::os_store::document_store_owners::MemberStoreProducer;
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    use semio_s_artifact_stdio_semio_base::SemioMemberStoreProducer;
    use {semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot, semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation, semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot, semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation};
    use semio_framework_io_sqlite_snapshot_testing_backing::measure;
    macro_rules! catalog {
        ($snapshot:ty,$mutation:ty,$kind:literal) => {{
            let producer=SemioMemberStoreProducer::<$snapshot,$mutation>::new();
            let (birth,query_heap)=measure(|| producer.member_store_owners_birth_demand().expect("pure catalog price"));
            assert_eq!((query_heap.bytes,query_heap.released_bytes),(0,0));
            let grant=RetainedCloneGrant { maximum_items:1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,maximum_copy_bytes:0,maximum_release_bytes:0 };
            let ((mut owners,progress),heap)=measure(|| producer.member_store_owners(grant).unwrap_or_else(|_| panic!("funded {} catalog",$kind)));
            assert_eq!((heap.bytes,heap.released_bytes),(birth.capacity_bytes,0),"pure {} catalog query must price original constructor",$kind);
            assert_eq!(progress.retained_capacity_bytes,heap.bytes);
            let mut born=heap.bytes;
            let mut physical=0;
            for _ in 0..1024 {
                if owners.uninstalled_owners_terminal_is_empty() { break; }
                let copy=owners.uninstalled_owners_demands(0).unwrap().copy_bytes;
                let demand=owners.uninstalled_owners_demands(copy).unwrap();
                let grant=RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth };
                let (step,heap)=measure(|| owners.close_uninstalled_owners_step(grant).unwrap());
                let progress=step.progress();
                assert!(progress.fits(grant));
                assert_eq!((heap.bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
                assert!(progress.copied_items>0 || matches!(step,RetainedCloneStep::Complete(_)),"an exact current owner grant must advance");
                born+=heap.bytes;
                physical+=heap.released_bytes;
            }
            assert!(owners.uninstalled_owners_terminal_is_empty());
            assert_eq!(physical,born);
        }};
    }
    catalog!(SemioModelSnapshot,SemioModelMutation,"model");
    catalog!(SemioBrepSnapshot,SemioBrepMutation,"brep");
}
