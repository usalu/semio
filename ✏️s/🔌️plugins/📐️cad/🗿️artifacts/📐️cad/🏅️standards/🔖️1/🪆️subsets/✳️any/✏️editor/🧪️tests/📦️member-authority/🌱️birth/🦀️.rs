#[test]
fn cad_imported_geometry_catalog_virtual_birth_matches_original_system_allocations() {
    use semio_framework_os_kernel::{MemberStoreOwner, SnapshotRetirementStep};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::{model::schema::{snapshot::SemioModelSnapshot,mutations::SemioModelMutation},brep::schema::{snapshot::SemioBrepSnapshot,mutations::SemioBrepMutation}};
    use crate::standards::v1::subsets::any::io::sqlite::snapshot::tests::backing_observer::measure;
    macro_rules! catalog {
        ($snapshot:ty,$mutation:ty,$kind:literal) => {{
            let (birth,query_heap)=measure(|| <$snapshot as MemberStoreOwner<$mutation>>::member_store_owners_birth_bytes());
            assert_eq!((query_heap.bytes,query_heap.released_bytes),(0,0));
            let (mut owners,heap)=measure(|| <$snapshot as MemberStoreOwner<$mutation>>::member_store_owners());
            let actual_birth=heap.bytes;
            assert_eq!(heap.released_bytes,0);
            let mut released=0;
            for _ in 0..128 {
                if owners.uninstalled_owners_terminal_is_empty() { break; }
                let bytes=owners.next_close_byte_demand();
                assert!(bytes<=1048576);
                let (denied,heap)=measure(|| owners.close_uninstalled_owners_step(0,bytes).unwrap());
                assert!(matches!(denied,SnapshotRetirementStep::Pending { released_items:0,released_bytes:0 }));
                assert_eq!((heap.bytes,heap.released_bytes),(0,0));
                if bytes>0 {
                    let (denied,heap)=measure(|| owners.close_uninstalled_owners_step(1,bytes-1).unwrap());
                    assert!(matches!(denied,SnapshotRetirementStep::Pending { released_items:0,released_bytes:0 }));
                    assert_eq!((heap.bytes,heap.released_bytes),(0,0));
                    assert_eq!(owners.next_close_byte_demand(),bytes);
                }
                let (step,heap)=measure(|| owners.close_uninstalled_owners_step(1,bytes).unwrap());
                let claimed=match step { SnapshotRetirementStep::Pending { released_bytes,.. }=>released_bytes,SnapshotRetirementStep::Complete=>0,other=>panic!("unexpected catalog close: {other:?}") };
                assert_eq!((heap.bytes,heap.released_bytes),(0,claimed));
                released+=claimed;
            }
            assert!(owners.uninstalled_owners_terminal_is_empty());
            assert_eq!(actual_birth,birth,"pure {} catalog query must price original constructor",$kind);
            assert_eq!(released,birth);
            println!("[DEBUG] {} member virtual catalog birth={birth} query heap0 original constructor={actual_birth} exact close={released}",$kind);
        }};
    }
    catalog!(SemioModelSnapshot,SemioModelMutation,"model");
    catalog!(SemioBrepSnapshot,SemioBrepMutation,"brep");
}
