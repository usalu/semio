
#[test]
fn paged_list_exact_final_extent_preserves_original_8194_and_parent_authority(){
    use crate::{NativeEncodeControl,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/@EXACT_TAIL_FIXTURE@"))).unwrap();
    assert_eq!(fixture["ownerWords"],6);assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["sourceCount"],5);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(size_of::<PagedList<u8,{isize::MAX as usize}>>(),6*size_of::<usize>());
    let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut owners:[PagedList<u8,{isize::MAX as usize}>;5]=std::array::from_fn(|_|PagedList::empty());let mut pointers=[None;5];let mut total=0;
    for(ordinal,owner)in owners.iter_mut().enumerate(){
        for index in 0..8194{
            while !owner.has_reserved_slot(){
                let required=owner.next_exact_capacity_allocation_bytes(8194).unwrap().unwrap();assert!(required<=4096);let retained=owner.allocated_bytes();let pointer=owner.backing_ptr(0);assert!(!owner.reserve_exact_capacity_one(8194,required.saturating_sub(1)).unwrap().progressed);assert_eq!(owner.allocated_bytes(),retained);assert_eq!(owner.backing_ptr(0),pointer);control.charge(required).unwrap();let step=owner.reserve_exact_capacity_one(8194,4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);
            }
            assert!(owner.push_reserved(if index==0{b'1'}else if index==1{b'7'}else{b' '}).is_ok());if index==0{pointers[ordinal]=owner.backing_ptr(0);}assert_eq!(owner.backing_ptr(0),pointers[ordinal]);
        }
        assert_eq!(owner.len(),8194);assert_eq!(owner.capacity(),8194);assert_eq!(owner.leaf(0).unwrap().capacity(),4096);assert_eq!(owner.leaf(4096).unwrap().capacity(),4096);assert_eq!(owner.leaf(8192).unwrap().capacity(),2);for(index,byte)in owner.iter().enumerate(){assert_eq!(*byte,if index==0{b'1'}else if index==1{b'7'}else{b' '});}
        let retained=owner.allocated_bytes();assert!(owner.next_exact_capacity_allocation_bytes(8195).is_err());assert!(owner.reserve_exact_capacity_one(8195,4096).is_err());assert_eq!(owner.backing_ptr(0),pointers[ordinal]);assert_eq!(owner.allocated_bytes(),retained);assert_eq!(owner.len(),8194);total+=retained;
    }
    assert_eq!(control.owned_bytes(),total);assert!(total<=65536);assert!(total>=40970);let mut parent=ParentAllocationReturn::<512>::try_new(4096,total).unwrap();let mut returned=0;
    for owner in &mut owners{
        while owner.pop().is_some(){}let before=owner.allocated_bytes();assert!(!owner.return_empty_page(&mut parent,0).unwrap().progressed);assert_eq!(owner.allocated_bytes(),before);
        while !owner.terminal_is_empty(){let before=owner.allocated_bytes();let step=owner.return_empty_page(&mut parent,1).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;}
    }
    assert_eq!(returned,total);assert_eq!(parent.retained_bytes(),total);let mut disposed=0;let mut observed_tail=false;
    for _ in 0..512+128{if parent.terminal_is_empty(){break}let required=parent.next_close_byte_demand();if required==2{observed_tail=true;}assert_eq!(parent.close_step(1,required.saturating_sub(1)),AllocationReturnStep::Pending{released_items:0,released_bytes:0});match parent.close_step(1,4096){AllocationReturnStep::Pending{released_items,released_bytes}=>{assert_eq!(released_items,1);assert_eq!(released_bytes,required);assert!(released_bytes<=4096);disposed+=released_bytes;},AllocationReturnStep::Complete=>panic!("genuine parent token disappeared before its physical allocation")}}
    assert!(observed_tail);assert_eq!(disposed,total);assert!(parent.terminal_is_empty());assert!(owners.iter().all(PagedList::terminal_is_empty));
    let mut initial=PagedList::<u8,{isize::MAX as usize}>::empty();let mut allow=|_|true;let mut limited=NativeEncodeControl::new(4096,&mut allow);let metadata=initial.next_exact_capacity_allocation_bytes(8194).unwrap().unwrap();limited.charge(metadata).unwrap();assert!(initial.reserve_exact_capacity_one(8194,4096).unwrap().progressed);let payload=initial.next_exact_capacity_allocation_bytes(8194).unwrap().unwrap();assert_eq!(payload,4096);assert!(limited.charge(payload).is_err());assert_eq!(initial.len(),0);assert_eq!(initial.allocated_bytes(),metadata);while !initial.terminal_is_empty(){assert!(initial.release_empty_page(4096).unwrap().progressed)}
    eprintln!("[DEBUG] five original8194 literal sources retain exact4096/4096/2 payload pages and actual metadata under unchanged cumulative64k; original six-word wrapper and pointers survive short grants/forbidden extension; initial4096 still refuses actual first payload; real parent alone physically deallocates every full token1/4096");
}
