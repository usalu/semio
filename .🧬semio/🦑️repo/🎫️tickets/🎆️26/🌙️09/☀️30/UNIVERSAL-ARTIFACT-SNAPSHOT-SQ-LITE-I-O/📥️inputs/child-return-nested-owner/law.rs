
#[test]
fn ready_child_parent_return_keeps_nested_owner_backing_until_paid_parent_release(){
    use semio_framework_value::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    use crate::app::{ChildEmit,PluginCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🧫️fixtures/📦️nested-parent-return.json")).unwrap();
    let mut child=ChildEmit::open(fixture["slot"].as_str().unwrap(),fixture["childId"].as_str().unwrap(),0);child.owner=fixture["owner"].as_str().unwrap().to_owned();
    let owner_pointer=child.owner.as_ptr();let expected=child.owner.capacity()+child.slot.capacity()+child.child_id.capacity()+child.op_schema.0.capacity();
    let mut emit:Emit<TestMutation,NoConfigMutation,NoDraftMutation>=Emit::default();emit.child_emits.push(child);let expected=expected+emit.child_emits.capacity()*std::mem::size_of::<ChildEmit>();
    let items=fixture["maximumItems"].as_u64().unwrap()as usize;let child_bytes=fixture["maximumChildBytes"].as_u64().unwrap()as usize;let parent_bytes=fixture["maximumParentBytes"].as_u64().unwrap()as usize;
    let mut parent=ParentAllocationReturn::<16>::try_new(parent_bytes,parent_bytes*fixture["parentSlots"].as_u64().unwrap()as usize).unwrap();
    assert_eq!(emit.return_child_one(&mut parent,0,child_bytes).unwrap(),Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));assert_eq!(emit.child_emits[0].owner.as_ptr(),owner_pointer);assert_eq!(parent.retained_bytes(),0);
    let mut complete=false;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){match emit.return_child_one(&mut parent,items,child_bytes).unwrap(){None=>{complete=true;break;},Some(PluginCloseStep::Pending{released_items,released_bytes})=>{assert!(released_items<=items);assert_eq!(released_bytes,0);},other=>panic!("child handoff issues logical pending or empty recipient lane: {other:?}")}}
    assert!(complete);assert!(emit.child_emits.is_empty());assert_eq!(emit.child_emits.capacity(),0);assert_eq!(parent.retained_bytes(),expected,"every original child field and backing allocation remains physically owned by the actual parent");assert!(!parent.terminal_is_empty());let before=parent.retained_bytes();assert_eq!(parent.close_step(items,child_bytes),AllocationReturnStep::Pending{released_items:0,released_bytes:0});assert_eq!(parent.retained_bytes(),before);
    let mut released=0;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){match parent.close_step(items,parent_bytes){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=parent_bytes);released+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(parent.retained_bytes(),0);assert_eq!(released,expected);
    eprintln!("[DEBUG] actual nested owner UTF8 allocation joins every child field and Vec backing in persistent parent; no physical child4-byte release, exact paid parent4096 terminal");
}
