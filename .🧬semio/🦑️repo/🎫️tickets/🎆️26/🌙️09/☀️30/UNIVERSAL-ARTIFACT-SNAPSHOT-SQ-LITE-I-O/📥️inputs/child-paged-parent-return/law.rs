
#[test]
fn paged_child_source_returns_genuine8194_backing_to_parent_without_physical_credit(){
    use crate::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏠️parent-return.json")).unwrap();
    let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;
    let exact=fixture["wireBytes"].as_u64().unwrap()as usize;
    let mut source=PagedList::<u8,8194>::empty();
    for byte in fixture["wire"].as_str().unwrap().bytes().chain(std::iter::repeat_n(fixture["paddingByte"].as_u64().unwrap()as u8,fixture["paddingBytes"].as_u64().unwrap()as usize)){
        while !source.has_reserved_slot(){let progress=source.reserve_one(maximum).unwrap();assert!(progress.progressed&&progress.allocated_bytes<=maximum);}
        source.push_reserved(byte).unwrap();
    }
    assert_eq!(source.len(),exact);
    let pointer=source.backing_ptr(0).unwrap();
    let initial=source.allocated_bytes();
    let mut parent=ParentAllocationReturn::<1>::try_new(maximum,fixture["parentTotalBytes"].as_u64().unwrap()as usize).unwrap();
    assert!(!source.return_empty_page(&mut parent,0).unwrap().progressed);
    assert_eq!(source.backing_ptr(0),Some(pointer));assert_eq!(source.len(),exact);assert!(parent.terminal_is_empty());
    assert_eq!(source.return_empty_page(&mut parent,1).unwrap_err().kind,crate::ValueRefusalKind::InvariantViolated);
    assert_eq!(source.backing_ptr(0),Some(pointer));assert_eq!(source.len(),exact);assert!(parent.terminal_is_empty());
    while source.pop().is_some(){}
    let expected=source.next_release_allocation_bytes().unwrap();
    let mut blocker=Vec::with_capacity(1);blocker.push(9u8);assert!(parent.return_bytes(&mut blocker,1).unwrap());
    let before=source.allocated_bytes();
    assert!(!source.return_empty_page(&mut parent,1).unwrap().progressed);
    assert_eq!(source.allocated_bytes(),before);assert_eq!(parent.retained_bytes(),1);
    assert_eq!(parent.close_step(1,maximum),AllocationReturnStep::Pending{released_items:1,released_bytes:1});assert!(parent.terminal_is_empty());
    let mut insufficient=ParentAllocationReturn::<1>::try_new(expected-1,expected-1).unwrap();
    assert_eq!(source.return_empty_page(&mut insufficient,1).unwrap_err().kind,crate::ValueRefusalKind::OwnershipLimit);
    assert_eq!(source.allocated_bytes(),before);assert!(insufficient.terminal_is_empty());
    let mut returned=0;let mut released=0;
    for _ in 0..fixture["lawTurns"].as_u64().unwrap(){
        if source.terminal_is_empty()&&parent.terminal_is_empty(){break;}
        if !parent.terminal_is_empty(){
            let exact=parent.next_close_byte_demand();
            assert_eq!(parent.close_step(1,exact-1),AllocationReturnStep::Pending{released_items:0,released_bytes:0});
            assert!(!parent.terminal_is_empty());
            match parent.close_step(1,maximum){AllocationReturnStep::Pending{released_items,released_bytes}=>{assert_eq!(released_items,1);assert_eq!(released_bytes,exact);assert!(released_bytes<=maximum);released+=released_bytes;},AllocationReturnStep::Complete=>panic!("actual parent owner disappeared")}
        }else{
            let before=source.allocated_bytes();let progress=source.return_empty_page(&mut parent,1).unwrap();
            assert!(progress.progressed);assert_eq!(before-source.allocated_bytes(),progress.returned_allocation_bytes);assert_eq!(parent.retained_bytes(),progress.returned_allocation_bytes);assert!(!parent.terminal_is_empty());returned+=progress.returned_allocation_bytes;
        }
    }
    assert!(source.terminal_is_empty()&&parent.terminal_is_empty());assert_eq!(returned,initial);assert_eq!(released,initial);
    eprintln!("[DEBUG] original8194 paged child backing transfers actual payload/metadata allocations under preadmitted one-slot parent; zero/live/full/small refusal unchanged, child zero physical credit, parent actual1/4096 deallocation");
}

