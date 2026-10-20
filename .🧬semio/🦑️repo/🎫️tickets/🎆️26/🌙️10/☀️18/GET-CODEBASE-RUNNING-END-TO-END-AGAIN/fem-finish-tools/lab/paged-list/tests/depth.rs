use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value_paged_list::list::PagedList;
#[derive(Debug,PartialEq,Eq)]
struct Row{key:u64,body:[u8;88]}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:17,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64}}
#[test]
fn original_paged_list_capacity_depth_quote_is_the_funded_reservation_depth(){
 let g=grant();let mut owner=PagedList::<Row,4096>::empty();
 assert_eq!(owner.next_capacity_depth_demand(0).unwrap(),0);
 while owner.capacity()<200{
  let depth=owner.next_capacity_depth_demand(200).unwrap();assert!(depth>=3);
  let before=(owner.capacity(),owner.allocated_bytes());let mut denied=g;denied.maximum_depth=depth-1;
  assert_eq!(owner.reserve_capacity_one_funded(200,denied).unwrap(),RetainedCloneProgress::default());assert_eq!((owner.capacity(),owner.allocated_bytes()),before);
  let mut exact=g;exact.maximum_depth=depth;assert!(owner.reserve_capacity_one_funded(200,exact).unwrap().retained_capacity_bytes>0);
 }
 assert_eq!(owner.next_capacity_depth_demand(200).unwrap(),0);
 assert_eq!(owner.next_capacity_depth_demand(4097).unwrap_err().reason,"fixed list logical capacity exhausted");
}
