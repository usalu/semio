
#[test]
fn native_decode_explicit_one_slot_preserves_success_and_refusal_physical_owners(){
    use semio_framework_value::{ErasedSnapshotRetirement,SnapshotRetirementStep};
    use semio_framework_value::native_decoding::NativeDecodeRetirementRecipient;
    use std::{cell::Cell,io::Write,process::{Command,Stdio}};
    #[repr(C)]
    struct Pages{pages:[Option<Box<[u8;4096]>>;3],next:usize}
    impl ErasedSnapshotRetirement for Pages{
        fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
            if items==0||bytes<4096{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
            if self.next==3{return Ok(SnapshotRetirementStep::Complete);}
            self.pages[self.next].take();self.next+=1;Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:4096})
        }
        fn terminal_is_empty(&self)->bool{self.pages.iter().all(Option::is_none)}
        fn next_close_byte_demand(&self)->usize{if self.terminal_is_empty(){0}else{4096}}
    }
    impl Drop for Pages{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"actual page owner was dropped");}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🫴️recipient.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🫴️recipient.json")).unwrap();
    let wrapper=std::mem::size_of::<Pages>();assert_eq!(wrapper,4*std::mem::size_of::<usize>());
    let mut actual=Vec::new();
    for row in fixture["cases"].as_array().unwrap(){
        let mode=row["mode"].as_str().unwrap();let entered=Cell::new(0);
        let checkpoints=Cell::new(0);let mut callback=|progress:NativeDecodeProgress|{checkpoints.set(checkpoints.get()+1);mode!="canceled"&&(mode!="canceled-after-wrapper"||checkpoints.get()<2)&&(mode!="interior-canceled"||progress.completed<256)};
        let mut recipient=NativeDecodeRetirementRecipient::new();
        let mut control=NativeDecodeControl::new(if mode=="over-limit"{wrapper-1}else if mode=="initial-4096"{4096}else{65536},&mut callback);
        if mode!="missing"{control.install_retirement_recipient(&mut recipient).unwrap();}
        let operation=|control:&mut NativeDecodeControl<'_>|{
            entered.set(entered.get()+1);
            if let Err(error)=control.charge(3*4096){return(Err(error),None);}
            let owner=Pages{pages:[Some(Box::new([1;4096])),Some(Box::new([2;4096])),Some(Box::new([3;4096]))],next:0};
            let result=if mode=="interior-canceled"{control.begin_stage(256).and_then(|_|control.advance(256))}else if mode=="refusal"{Err(ValueError::new(ValueRefusalKind::InvalidValue,"actual decoder refusal"))}else{Ok(())};
            (result,Some(Box::new(owner)as Box<dyn ErasedSnapshotRetirement>))
        };
        let mut result=control.with_retirement_owner(wrapper,operation);
        if mode=="occupied"{result=control.with_retirement_owner(wrapper,|_|{entered.set(entered.get()+1);(Ok(()),None)});}
        let error=result.err().map(|error|match error.kind{ValueRefusalKind::OwnershipLimit=>"ownership",ValueRefusalKind::Canceled=>"canceled",ValueRefusalKind::InvalidValue=>"invalid",_=>panic!("unexpected recipient error")});
        let owned=control.owned_bytes();drop(control);
        let retained=recipient.has_owner();
        assert_eq!(entered.get(),row["entered"].as_u64().unwrap());assert_eq!(error.map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),row["error"]);assert_eq!(retained,row["retained"].as_bool().unwrap());
        if retained{
            for grant in [(0,4096),(1,4095)]{assert_eq!(recipient.close_step(grant.0,grant.1).unwrap(),SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});assert!(!recipient.terminal_is_empty());}
            let mut bytes=0;for _ in 0..8{match recipient.close_step(1,4096).unwrap(){SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);bytes+=released_bytes;},SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Blocked=>panic!("actual paid page owner blocked")}}
            assert!(recipient.terminal_is_empty());assert_eq!(bytes,3*4096+wrapper);assert_eq!(owned,3*4096+wrapper);
        }else{assert!(recipient.terminal_is_empty());assert_eq!(owned,if matches!(mode,"initial-4096"|"canceled-after-wrapper"){wrapper}else{0});}
        actual.push(serde_json::json!({"entered":entered.get(),"error":error,"retained":retained}));
    }
    let script="import Ajv from 'ajv';import {Database} from 'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('closed recipient schema');const db=new Database(':memory:');db.run('CREATE TABLE recipient(slot INTEGER PRIMARY KEY CHECK(slot=1))');const out=x.fixture.cases.map(c=>{db.run('DELETE FROM recipient');const before=['missing','canceled','canceled-after-wrapper','over-limit'].includes(c.mode);let entered=0,error=before?(c.mode.startsWith('canceled')?'canceled':'ownership'):c.mode==='refusal'?'invalid':c.mode==='interior-canceled'?'canceled':c.mode==='initial-4096'?'ownership':null;if(!before){if(c.mode!=='initial-4096')db.run('INSERT INTO recipient VALUES(1)');entered++;if(c.mode==='occupied'){try{db.run('INSERT INTO recipient VALUES(1)');throw Error('duplicate admitted');}catch(e){if(String(e).includes('duplicate admitted'))throw e;error='ownership';}}}return {entered,error,retained:db.query('SELECT COUNT(*) AS n FROM recipient').get().n===1};});db.close();await Bun.write(Bun.stdout,JSON.stringify(out));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
    println!("[DEBUG] explicit one-slot decoder recipient preserves success/refusal, absent/full/canceled/over-limit and actual initial4096 retain zero partial pages, actual three4096 pages plus wrapper physically drain under1/4096");
}

