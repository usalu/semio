//! 🧪️ Exact borrowed-key collisions and all retained currencies share the neutral metadata corpus.
use crate::*;
fn query(index:&mut MetadataIndex,source:&impl MetadataSource,key:MetadataKey,domain:u8,scope:usize,insert:Option<usize>,grant:RetainedCloneGrant)->Result<MetadataLookup,semio_framework_value::ValueError>{
    effect_turn(grant,|effect|{
        let result=index.query(source,key,domain,scope,insert,effect);
        (result,index.last_receipt())
    })
}
struct Source{keys:Vec<String>}
impl MetadataSource for Source{fn identity(&self)->usize{self as*const Self as usize}fn text(&self,key:MetadataKey)->Result<&str,&'static str>{self.keys.get(key.a).map(String::as_str).ok_or("metadata fixture key")}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn close(index:&mut MetadataIndex){
    index.cancel();
    for _ in 0..10000{
        if index.terminal_is_empty(){return;}
        let demand=index.next_close_demand().unwrap();
        let exact=grant(demand);
        let mut step=|grant|effect_turn(grant,|effect|{let receipt=index.close_one(effect).unwrap();(receipt,receipt)});
        for denied in [RetainedCloneGrant{maximum_items:0,..exact},
            RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),maximum_items:usize::from(demand.copy_bytes!=0),..exact},
            RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),maximum_items:usize::from(demand.release_bytes!=0),..exact},
            RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..exact}]{
            assert_eq!(step(denied),Default::default());
        }
        let receipt=step(exact);
        assert_eq!((receipt.copied_items,receipt.copied_bytes,receipt.released_bytes),(1,demand.copy_bytes,demand.release_bytes));
    }
    panic!("metadata close stalled");
}
#[test]fn neutral_borrowed_metadata_index_matches_pinned_lookup_oracle(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let source=Source{keys:fixture["keys"].as_array().unwrap().iter().map(|s|s.as_str().unwrap().to_owned()).collect()};for _ in [1,7,4096]{let mut index=MetadataIndex::new();for op in fixture["operations"].as_array().unwrap(){let key=MetadataKey{kind:0,a:op["key"].as_u64().unwrap()as usize,b:0};let domain=op["domain"].as_u64().unwrap()as u8;let scope=op["scope"].as_u64().unwrap()as usize;let insert=op["insert"].as_u64().map(|n|n as usize);let mut done=false;for _ in 0..100000{let d=index.next_demand().unwrap();let g=grant(d);if d.capacity_bytes>0{assert!(d.depth>=3);assert_eq!(query(&mut index,&source,key,domain,scope,insert,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g}).unwrap(),MetadataLookup::Pending);assert_eq!(index.last_receipt().retained_capacity_bytes,0);}if d.copy_bytes>0{assert_eq!(query(&mut index,&source,key,domain,scope,insert,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g}).unwrap(),MetadataLookup::Pending);assert_eq!(index.last_receipt().copied_bytes,0);}if d.depth>1{assert_eq!(query(&mut index,&source,key,domain,scope,insert,RetainedCloneGrant{maximum_depth:d.depth-1,..g}).unwrap(),MetadataLookup::Pending);assert_eq!(index.last_receipt().retained_capacity_bytes,0);assert_eq!(index.last_receipt().copied_bytes,0);}let result=query(&mut index,&source,key,domain,scope,insert,g).unwrap();let p=index.last_receipt();assert!(p.retained_capacity_bytes<=d.capacity_bytes);assert!(p.copied_bytes<=d.copy_bytes);if let MetadataLookup::Ready(value)=result{assert_eq!(value,op["expected"].as_u64().map(|n|n as usize));done=true;break;}}assert!(done);}close(&mut index);}}
#[test]fn metadata_refuses_source_and_active_query_changes(){let source=Source{keys:vec!["original".into()]};let changed=Source{keys:vec!["original".into()]};let mut index=MetadataIndex::new();let g=grant(RetirementDemand{copy_bytes:4096,capacity_bytes:65536,release_bytes:0,depth:8});let key=MetadataKey{kind:0,a:0,b:0};assert_eq!(query(&mut index,&source,key,1,0,Some(7),g).unwrap(),MetadataLookup::Pending);assert!(query(&mut index,&changed,key,1,0,Some(7),g).is_err());assert!(query(&mut index,&source,key,1,1,Some(7),g).is_err());close(&mut index);}
