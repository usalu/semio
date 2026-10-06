
#[test]
fn paged_semantic_text_keeps_actual8194_success_and_partial_cancellation_owners_under4096(){
    use semio_framework_value::{paged_text::PagedText,native_decoding::NativeDecodeRetirementRecipient,ErasedSnapshotRetirement,SnapshotRetirementStep};
    use std::{cell::Cell,io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📝️paged-text.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/📝️paged-text.json")).unwrap();
    let ascii=vec![fixture["ascii"]["byte"].as_u64().unwrap()as u8;fixture["ascii"]["repetitions"].as_u64().unwrap()as usize];
    let unicode=format!("{}{}{}{}","x".repeat(4095),fixture["unicode"]["middle"].as_str().unwrap(),"x".repeat(4093),fixture["unicode"]["tail"].as_str().unwrap()).into_bytes();
    assert_eq!(ascii.len(),8194);assert_eq!(unicode.len(),8194);
    let wrapper=std::mem::size_of::<PagedText<131072>>();let mut actual=Vec::new();
    let drain=|recipient:&mut NativeDecodeRetirementRecipient,expected:usize|{
        assert_eq!(recipient.close_step(0,4096).unwrap(),SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});
        let mut released=0;for _ in 0..8194+128{match recipient.close_step(1,4096).unwrap(){SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;},SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Blocked=>panic!("paged semantic owner blocked")}}
        assert!(recipient.terminal_is_empty());assert_eq!(released,expected);
    };
    for row in fixture["cases"].as_array().unwrap(){
        let input=if row["source"]=="unicode"{&unicode}else{&ascii};let pointer=input.as_ptr();let mode=row["mode"].as_str().unwrap();let phase=Cell::new(0);
        let mut callback=|progress:NativeDecodeProgress|{
            if progress.total==8194&&progress.completed==0{phase.set(phase.get()+1);}
            !(progress.completed>=256&&((mode=="validate-cancel"&&phase.get()==1)||(mode=="materialize-cancel"&&phase.get()>=2)))
        };
        let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(if mode=="initial"{4096}else{131072},&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
        let logical=Cell::new(0);let allocated=Cell::new(0);let output=std::cell::RefCell::new(Vec::new());
        let result=control.with_retirement_owner(wrapper,|control|{
            let mut text=PagedText::<131072>::empty();let result=text.read_from_source(&input.as_slice(),control);logical.set(text.byte_len());allocated.set(text.allocated_bytes());
            if result.is_ok(){let span=text.borrow().unwrap();assert_eq!(span.byte_len(),8194);assert!(span.bytes().eq(input.iter().copied()));let chars=span.chars().collect::<String>();assert_eq!(chars.as_bytes(),input.as_slice());*output.borrow_mut()=span.bytes().collect();}
            else{assert!(text.borrow().is_err());}
            (result,Some(Box::new(text)as Box<dyn ErasedSnapshotRetirement>))
        });
        let owned=control.owned_bytes();drop(control);
        let error=result.err().map(|error|match error.kind{ValueRefusalKind::OwnershipLimit=>"ownership",ValueRefusalKind::Canceled=>"canceled",_=>panic!("unexpected text refusal")});
        assert_eq!(logical.get(),row["logical"].as_u64().unwrap()as usize);assert_eq!(error.map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),row["error"]);assert_eq!(owned,allocated.get()+wrapper);assert_eq!(input.as_ptr(),pointer);assert_eq!(input.len(),8194);
        if mode=="initial"{assert!(allocated.get()>0&&allocated.get()+wrapper<=4096);}if mode=="materialize-cancel"{assert!(allocated.get()>=4096);}
        drain(&mut recipient,allocated.get()+wrapper);
        actual.push(serde_json::json!({"id":row["id"],"error":error,"logical":logical.get(),"octets":output.into_inner()}));
    }
    for bytes in fixture["invalidUtf8"].as_array().unwrap(){let input:Vec<u8>=serde_json::from_value(bytes.clone()).unwrap();let mut text=PagedText::<131072>::empty();let mut accept=|_|true;let mut control=NativeDecodeControl::new(131072,&mut accept);assert_eq!(text.read_from_source(&input.as_slice(),&mut control).unwrap_err().kind,ValueRefusalKind::InvalidValue);assert_eq!(control.owned_bytes(),0);assert!(text.terminal_is_empty());}
    let script="import Ajv from 'ajv';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('closed paged text schema');const f=x.fixture,ascii=Uint8Array.from({length:f.ascii.repetitions},()=>f.ascii.byte),unicode=new TextEncoder().encode(String.fromCharCode(f.unicode.prefixByte).repeat(f.unicode.prefixRepetitions)+f.unicode.middle+String.fromCharCode(f.unicode.suffixByte).repeat(f.unicode.suffixRepetitions)+f.unicode.tail);if(ascii.length!==8194||unicode.length!==8194)throw Error('independent exact extents');for(const v of f.invalidUtf8){let refused=false;try{new TextDecoder('utf-8',{fatal:true}).decode(Uint8Array.from(v));}catch{refused=true;}if(!refused)throw Error('invalid UTF8 accepted');}await Bun.write(Bun.stdout,JSON.stringify(f.cases.map(c=>({id:c.id,error:c.error,logical:c.logical,octets:c.error===null?Array.from(c.source==='ascii'?ascii:unicode):[]}))));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
    println!("[DEBUG] actual8194 semantic UTF8, Unicode across4096 boundary, initial4096 metadata refusal and interior partial-page cancellation preserve exact owned bytes and physically retire every allocation at1/4096");
}

