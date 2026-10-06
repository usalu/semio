
#[test]
fn child_paged_operation_carrier_borrows_exact8194_and_returns_actual_pages_without_physical_credit(){
    use super::operation_bytes::{OperationSourceCollection,OperationByteReturnStep,OperationByteOutput};
    use crate::value::list::PagedList;
    use crate::value::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🏠️child-carrier.json")).unwrap();
    let max=f["maximumBytes"].as_u64().unwrap()as usize;
    let mut operation=OwnedOperationBytes::try_new(f["maximumPayloadBytes"].as_u64().unwrap()as usize,f["maximumAllocationBytes"].as_u64().unwrap()as usize).unwrap();
    let mut allow=|_|true;let mut encoding=crate::value::NativeEncodeControl::new(f["maximumAllocationBytes"].as_u64().unwrap()as usize,&mut allow);
    operation.write_bytes(f["wire"].as_str().unwrap().as_bytes(),&mut encoding).unwrap();
    for _ in 0..f["paddingBytes"].as_u64().unwrap(){operation.write_bytes(&[f["paddingByte"].as_u64().unwrap()as u8],&mut encoding).unwrap();}
    let retained=operation.allocated_bytes();assert_eq!(retained,encoding.owned_bytes());assert_eq!(operation.len(),f["wireBytes"].as_u64().unwrap()as usize);
    let mut collection=PagedList::<OwnedOperationBytes,8194>::empty();
    while !collection.has_reserved_slot(){let step=collection.reserve_one(max).unwrap();assert!(step.progressed&&step.allocated_bytes<=max);}
    assert!(collection.push_reserved(operation).is_ok());
    let collection_bytes=collection.allocated_bytes();
    let ((),allocated,released)=crate::test_allocation::observe_backing(||{
        let sources:&dyn OperationSourceCollection=&collection;assert_eq!(sources.len(),1);assert!(!sources.is_empty());assert!(sources.source_at(1).is_none());
        let span=sources.source_at(0).unwrap();assert_eq!(span.len(),8194);assert_eq!(span.get(0),Some(&49));assert_eq!(span.get(1),Some(&55));
        for index in 2..8194{assert_eq!(span.get(index),Some(&32));assert!(std::ptr::eq(span.get(index).unwrap(),collection.get(0).unwrap().byte_ref(index).unwrap()));}
    });assert_eq!((allocated,released),(0,0));
    let mut parent=ParentAllocationReturn::<1>::try_new(max,f["maximumAllocationBytes"].as_u64().unwrap()as usize).unwrap();
    let owner=collection.get_mut(0).unwrap();
    for (items,bytes)in[(0,max),(1,0)]{let(step,a,r)=crate::test_allocation::observe_backing(||owner.return_one(&mut parent,items,bytes));assert_eq!(step.unwrap(),OperationByteReturnStep::Pending{returned_items:0,returned_bytes:0});assert_eq!((a,r),(0,0));assert_eq!(owner.len(),8194);assert_eq!(owner.allocated_bytes(),retained);}
    let mut physical=0;let mut handed=0;
    for _ in 0..f["maximumTurns"].as_u64().unwrap(){
        if !parent.terminal_is_empty(){
            let demand=parent.next_close_byte_demand();
            let(step,a,r)=crate::test_allocation::observe_backing(||parent.close_step(1,demand-1));assert_eq!(step,AllocationReturnStep::Pending{released_items:0,released_bytes:0});assert_eq!((a,r),(0,0));
            let(step,a,r)=crate::test_allocation::observe_backing(||parent.close_step(1,max));assert_eq!(a,0);assert_eq!(step,AllocationReturnStep::Pending{released_items:1,released_bytes:demand});assert_eq!(r,demand);assert!(r<=max);physical+=r;
        }else{
            let(step,a,r)=crate::test_allocation::observe_backing(||owner.return_one(&mut parent,1,max));assert_eq!((a,r),(0,0));
            match step.unwrap(){OperationByteReturnStep::Pending{returned_items,returned_bytes}=>{assert!(returned_items<=1);assert_eq!(parent.retained_bytes(),returned_bytes);handed+=returned_bytes;if returned_bytes!=0&&owner.allocated_bytes()!=0{let kept=owner.allocated_bytes();let(step,a,r)=crate::test_allocation::observe_backing(||owner.return_one(&mut parent,1,max));assert_eq!(step.unwrap(),OperationByteReturnStep::Pending{returned_items:0,returned_bytes:0});assert_eq!((a,r),(0,0));assert_eq!(owner.allocated_bytes(),kept);assert_eq!(parent.retained_bytes(),returned_bytes);}},OperationByteReturnStep::Complete=>break}
        }
    }
    assert!(owner.terminal_is_empty()&&parent.terminal_is_empty());assert_eq!(handed,retained);assert_eq!(physical,retained);assert_eq!(owner.allocated_bytes(),0);
    let((),a,r)=crate::test_allocation::observe_backing(||{assert!(collection.pop().unwrap().terminal_is_empty());});assert_eq!((a,r),(0,0));
    let mut outer=0;
    while !collection.terminal_is_empty(){let(step,a,r)=crate::test_allocation::observe_backing(||collection.return_empty_page(&mut parent,1));let step=step.unwrap();assert!(step.progressed);assert_eq!((a,r),(0,0));assert_eq!(step.returned_allocation_bytes,parent.retained_bytes());let(drain,a,r)=crate::test_allocation::observe_backing(||parent.close_step(1,max));assert_eq!(a,0);assert!(r<=max);assert_eq!(drain,AllocationReturnStep::Pending{released_items:1,released_bytes:r});outer+=r;}
    assert!(parent.terminal_is_empty());assert_eq!(outer,collection_bytes);
    eprintln!("[DEBUG] actual paged Child operation capability borrows exact original8194 refs without allocation; physical allocator proves every source and outer collection handoff releases0 child bytes, parent full4096 deallocation only");
}

