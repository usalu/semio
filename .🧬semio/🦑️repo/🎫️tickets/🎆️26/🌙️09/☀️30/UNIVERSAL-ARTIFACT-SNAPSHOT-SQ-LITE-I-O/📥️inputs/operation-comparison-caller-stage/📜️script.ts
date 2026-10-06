import {resolve} from "node:path";
const ticket=resolve(import.meta.dir,"../..");const root=resolve(ticket,"../../../../../../..");
const source="🧰️framework/🔨️modules/📡️replication/🎮️mutation/📦️bytes/🦀️.rs";
const tests="🧰️framework/🔨️modules/📡️replication/🎮️mutation/📦️bytes/🧪️tests/🦀️.rs";
const fixture="🧰️framework/🔨️modules/📡️replication/🎮️mutation/📦️bytes/🧫️fixtures/🧭️caller-stage.json";
const command=process.argv[2];const pairs:{path:string,before:string,after:string}[]=[];
async function pair(path:string,after:string){const f=Bun.file(`${root}/${path}`);pairs.push({path,before:await f.exists()?await f.text():"",after});}
if(command==="demand"){
 await pair(fixture,JSON.stringify({source:[17],foreign:[18],callerTotal:2,callerBefore:1},null,2)+"\n");
 const before=await Bun.file(`${root}/${tests}`).text();if(before.includes("operation_byte_comparison_preserves_enclosing"))throw Error("demand already mounted");
 await pair(tests,before+`\n#[test]\nfn operation_byte_comparison_preserves_enclosing_workload_on_acceptance_and_refusal(){
    use super::operation_bytes::{OperationByteComparison,OperationByteOutput};
    let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧭️caller-stage.json")).unwrap();
    let source:Vec<u8>=serde_json::from_value(f["source"].clone()).unwrap();let foreign:Vec<u8>=serde_json::from_value(f["foreign"].clone()).unwrap();
    for candidate in [&source,&foreign]{
        let observed=std::cell::Cell::new((0,0));
        let mut accept=|progress:crate::value::native_encoding::NativeEncodeProgress|{observed.set((progress.completed,progress.total));true};
        let mut control=crate::value::NativeEncodeControl::new(0,&mut accept);
        let total=f["callerTotal"].as_u64().unwrap()as usize;control.begin_stage(total).unwrap();control.advance(f["callerBefore"].as_u64().unwrap()as usize).unwrap();
        let mut comparison=OperationByteComparison::new(crate::codec::ByteSpan::from_slice(&source));
        let result=comparison.write_bytes(candidate,&mut control);
        if candidate==&source{result.unwrap();comparison.finish().unwrap();}else{assert_eq!(result.unwrap_err().kind(),crate::value::ValueRefusalKind::InvalidValue);assert_eq!(comparison.position(),0);assert!(comparison.finish().is_err());}
        control.step().expect("exact enclosing caller stage survives nested sink workload");assert_eq!(observed.get(),(total,total));assert_eq!(control.owned_bytes(),0);
    }
    println!("[DEBUG] Exact borrowed comparison restores caller2/2 workload after nested acceptance and original typed refusal, with no source/output ownership transfer");
}\n`);
}else if(command==="fixture-path-join"){
 const misplaced=fixture.replace("/🧪️/🧫️fixtures/","/🧪️🧫️fixtures/");
 const before=await Bun.file(`${root}/${misplaced}`).text();
 await pair(fixture,before);
 await Bun.write(`${import.meta.dir}/fixture-path-removal-pair.json`,JSON.stringify({path:misplaced,before,after:null},null,2)+"\n");
 await Bun.file(`${root}/${misplaced}`).delete();
}else if(command==="fixture-parent-join"){
 const misplaced=fixture.replace("/🧫️fixtures/","/🧪️/🧫️fixtures/");
 const before=await Bun.file(`${root}/${misplaced}`).text();await pair(fixture,before);
 await Bun.write(`${import.meta.dir}/fixture-parent-removal-pair.json`,JSON.stringify({path:misplaced,before,after:null},null,2)+"\n");await Bun.file(`${root}/${misplaced}`).delete();
}else if(command==="provider"){
 const before=await Bun.file(`${root}/${source}`).text();const start=before.indexOf("impl OperationByteOutput for OperationByteComparison<'_>");const end=before.indexOf("\n/// 🎚️",start);if(start<0||end<start)throw Error("exact comparator boundary");
 let block=before.slice(start,end);
 const open="        if let Err(error)=control.begin_stage(bytes.len()){self.refused=true;return Err(error.into());}";
 if(block.split(open).length!==2||block.split("        Ok(())\n    }\n}").length!==2)throw Error("exact comparator workload guard");
 block=block.replace(open,"        control.scoped_stage(|control|{\n"+open).replace("        Ok(())\n    }\n}","        Ok(())\n        })\n    }\n}");
 await pair(source,before.slice(0,start)+block+before.slice(end));
}else throw Error("exact demand/provider command required");
await Bun.write(`${import.meta.dir}/${command}-guarded-pairs.json`,JSON.stringify({pairs},null,2)+"\n");
for(const p of pairs){const f=Bun.file(`${root}/${p.path}`);if((await f.exists()?await f.text():"")!==p.before)throw Error(`fresh ${p.path}`);}
for(const p of pairs)await Bun.write(`${root}/${p.path}`,p.after);
console.log("[DEBUG] "+JSON.stringify({command,paths:pairs.length}));
