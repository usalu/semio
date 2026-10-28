from pathlib import Path
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_enum_input_tests.rs');s=f.read_text(encoding='utf8')
a='''            for turn in 0..100000 {
                if let ArtifactCommandWorkStep::Complete(emit) ='''
b='''            for turn in 0..100000 {
                if work.constituent_roster.receiving_primitives() || work.pending.is_some() && !work.domain_validated {
                    let demand=work.work_demands(input,4096).unwrap();
                    for axis in 0..6 {let mut denied=grant();let mut fuel=1;match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes=demand.copy_bytes-1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes=demand.capacity_bytes-1,3 if demand.release_bytes>0=>denied.maximum_release_bytes=demand.release_bytes-1,4=>denied.maximum_depth=0,5=>fuel=0,_=>continue};let before=work.constituent_roster.metadata();let turns=work.domain.turns();caller.work_turn(denied,fuel,|cx|{let receipt=cx.retained_progress();assert!(!matches!(work.step(input,cx)?,ArtifactCommandWorkStep::Complete(_)));assert_eq!(receipt,cx.retained_progress());Ok(())}).unwrap();assert_eq!(before,work.constituent_roster.metadata());assert_eq!(turns,work.domain.turns());}
                }
                if let ArtifactCommandWorkStep::Complete(emit) ='''
assert a in s;s=s.replace(a,b)
s+='''
#[test]
fn original_enum_repair_refuses_nonmembers_inherited_simple_and_invalid_preserved_aggregate_before_publication(){
    let law=enum_contract();
    for mode in 0..4{
        let row=&law["cases"][if mode==2{3}else{2}];let mut snapshot=enum_source(&law,row["source"].as_str().unwrap());
        if mode==2{Arc::get_mut(&mut snapshot).unwrap().entities[0].complex.clear();}
        if mode==3{Arc::get_mut(&mut snapshot).unwrap().entities[0].args[0]=IfcValue::Aggregate(Vec::new());}
        let mut request=enum_request(row,true);
        if mode<2{let value=request.args.get_mut("value").unwrap().get_mut("$ifcInputs").unwrap();let DslValue::Array(rows)=value else{panic!("rows")};*rows[0].get_mut("value").unwrap().get_mut("value").unwrap()=DslValue::String(if mode==0{"userdefined"}else{"UNKNOWN"}.into());}
        let DslValue::Object(mut fields)=request.args else{panic!("args")};let ordinal=fields.iter().position(|(key,_)|key=="value").unwrap();let event=editing::SnapshotEditEvent::SetValue{path:"/entities/0".into(),value:fields.remove(ordinal).1};
        let mut caller=OriginalWorkFixture::new();let mut work=Work::new(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
        inputs(&snapshot,event,&mut caller,&mut work,|input,caller,work|{let mut refused=false;for turn in 0..100000{match super::super::turn(caller,1,false,|cx|work.step(input,cx)){Err(_)=>{refused=true;break;},Ok(ArtifactCommandWorkStep::Complete(_))=>panic!("invalid enum candidate published"),_=>{}}assert!(turn<99999);}assert!(refused);assert!(!work.publication_validated);super::super::constituent_optional_shape_tests::close_with_denials(caller,work);});caller.finish();
    }
}
'''
f.write_text(s,encoding='utf8')
