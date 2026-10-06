
#[test]
fn paged_actual_height_preserves_maximum_authority_pointer_and_parent_handoff(){
    use crate::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/@ADAPTIVE_FIXTURE@"))).unwrap();assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["ownerWords"],6);assert_eq!(size_of::<PagedList<u64,{isize::MAX as usize}>>(),6*size_of::<usize>());
    let mut scalar=PagedList::<u64,{isize::MAX as usize}>::empty();let mut allocations=0;while !scalar.has_reserved_slot(){let required=scalar.next_allocation_bytes().unwrap();assert!(required<=4096);let step=scalar.reserve_one(4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);allocations+=1;}assert!(scalar.push_reserved(fixture["scalar"].as_u64().unwrap()).is_ok());assert!(allocations<=2,"one scalar preallocated maximum-height scaffolding");assert!(scalar.allocated_bytes()<=8192);assert_eq!(scalar[0],7);scalar.pop();while !scalar.terminal_is_empty(){assert!(scalar.release_empty_page(4096).unwrap().progressed)}
    for bytes in fixture["growthPayloadBytes"].as_array().unwrap(){
        let length=bytes.as_u64().unwrap()as usize;let mut owner=PagedList::<u8,{isize::MAX as usize}>::empty();let mut first=None;
        for index in 0..length{
            while !owner.has_reserved_slot(){let required=owner.next_allocation_bytes().unwrap();assert!(required<=4096);let retained=owner.allocated_bytes();let pointer=owner.backing_ptr(0);assert!(!owner.reserve_one(required.saturating_sub(1)).unwrap().progressed);assert_eq!(owner.allocated_bytes(),retained);assert_eq!(owner.backing_ptr(0),pointer);let step=owner.reserve_one(4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);}
            assert!(owner.push_reserved((index%251)as u8).is_ok());if index==0{first=owner.backing_ptr(0);}assert_eq!(owner.backing_ptr(0),first);
        }
        assert_eq!(owner.len(),length);for(index,byte)in owner.iter().enumerate(){assert_eq!(*byte,(index%251)as u8)}let allocated=owner.allocated_bytes();while owner.pop().is_some(){}let mut parent=ParentAllocationReturn::<512>::try_new(4096,allocated).unwrap();let mut returned=0;
        while !owner.terminal_is_empty(){let before=owner.allocated_bytes();let step=owner.return_empty_page(&mut parent,1).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;}assert_eq!(returned,allocated);assert_eq!(owner.allocated_bytes(),0);assert_eq!(parent.retained_bytes(),allocated);let mut physical=0;for _ in 0..512+128{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=4096);physical+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(physical,allocated);
    }
    println!("[DEBUG] actual-height paged owners preserve maximum logical authority and existing six-word layout, singleton only leaf+payload allocations,8194/65537 exact source and original pointer, denied growth unchanged and real parent handoff with zero child physical disposal1/4096");
}
