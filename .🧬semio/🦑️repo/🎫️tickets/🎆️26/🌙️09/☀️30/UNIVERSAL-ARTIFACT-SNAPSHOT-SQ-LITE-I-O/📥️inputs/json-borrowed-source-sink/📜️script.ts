import {resolve} from "node:path";
const ticket=resolve(import.meta.dir,"../..");
const root=resolve(ticket,"../../../../../../..");
const input=import.meta.dir;
const module="🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs";
const source="🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🫳️borrowed/🦀️.rs";
const tests="🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🔬️unit/🦀️.rs";
const fixture="🧰️framework/🔨️modules/🎒️pack/🔤️json/🧫️fixtures/🫳️source-sink.json";
const command=process.argv[2];
const pairs:{path:string,before:string,after:string}[]=[];
async function pair(path:string,after:string){const file=Bun.file(`${root}/${path}`);pairs.push({path,before:await file.exists()?await file.text():"",after});}
if(command==="demand"){
 await pair(fixture,JSON.stringify({word:"ä🧩\n\"\\\u0000",repeats:1024,prefix:"source-prefix:",maximumDepth:128,maximumOutputBytes:65536,cancelAt:1024,sinkRefuseAt:2048},null,2)+"\n");
 const before=await Bun.file(`${root}/${tests}`).text();if(before.includes("borrowed_json_source_sink_keeps"))throw Error("owning demand already mounted");
 await pair(tests,before+"\n"+await Bun.file(`${input}/law.rs`).text());
}else if(command==="debug-qualification"){
 const before=await Bun.file(`${root}/${tests}`).text();
 const original="without producer heap ownership; caller output backing is independently preowned";
 if(before.split(original).length!==2)throw Error("exact own debug qualification");
 await pair(tests,before.replace(original,"without admitted source or payload mirror; caller output backing is independently preowned and refusal ownership is excluded"));
}else if(command==="item-policy-demand"){
 const previous=await Bun.file(`${root}/${tests}`).text();
 if(previous.includes("maximumItems"))throw Error("own item policy demand already mounted");
 let after=previous.replace('let depth=fixture["maximumDepth"]','let depth=fixture["maximumDepth"]');
 const anchor='let depth=fixture["maximumDepth"]';
 if(!after.includes(anchor))throw Error("exact demand depth anchor");
 after=after.replace('let depth=fixture["maximumDepth"].as_u64().unwrap()as usize;', 'let depth=fixture["maximumDepth"].as_u64().unwrap()as usize;let items=fixture["maximumItems"].as_u64().unwrap();');
 for(const [before,following] of [['&source,maximum,depth,&mut','&source,maximum,depth,items,&mut'],['&source,limit,depth,&mut','&source,limit,depth,items,&mut'],['&source,maximum,1,&mut','&source,maximum,1,items,&mut']])after=after.replaceAll(before,following);
 const debug='    println!("[DEBUG] Original JSON source';
 if(after.split(debug).length!==2)throw Error("exact own debug anchor");
 after=after.replace(debug,'    let mut output=Vec::new();let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut accept);\n    assert_eq!(write_json_source_into(&source,maximum,depth,fixture["refusedCollectionItems"].as_u64().unwrap(),&mut|bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<\'_>|{output.extend_from_slice(bytes);Ok::<_,ValueError>(())},&mut control).unwrap_err().kind,ValueRefusalKind::WorkLimit);assert!(independent.as_bytes().starts_with(&output));\n'+debug);
 await pair(tests,after);
 const data=await Bun.file(`${root}/${fixture}`).json();data.maximumItems=32;data.refusedCollectionItems=6;await pair(fixture,JSON.stringify(data,null,2)+"\n");
}else if(command==="stage-provider"||command==="provider"){
 const before=await Bun.file(`${root}/${module}`).text();
 const join="#[path = \"🛫️encode/🫳️borrowed/🦀️.rs\"]\nmod borrowed_source_sink;\npub use borrowed_source_sink::write_json_source_into;\n\n";
 if(before.includes("mod borrowed_source_sink"))throw Error("producer already mounted");
 const anchor="/// 🧵️ Measures and writes the same owned source through bounded canonical writer transitions.";
 if(before.split(anchor).length!==2)throw Error("exact existing JsonWriteSource boundary");
 await pair(module,before.replace(anchor,join+anchor));
 await pair(source,await Bun.file(`${input}/producer.rs`).text());
}else throw Error("exact demand, stage-provider or provider command required");
await Bun.write(`${input}/${command}-guarded-pairs.json`,JSON.stringify({state:command==="stage-provider"?"Held":"ExactGuardedSourceFamily",pairs},null,2)+"\n");
if(command!=="stage-provider"){
 for(const pair of pairs){const file=Bun.file(`${root}/${pair.path}`);if((await file.exists()?await file.text():"")!==pair.before)throw Error(`fresh guard ${pair.path}`);}
 for(const pair of pairs)await Bun.write(`${root}/${pair.path}`,pair.after);
}
console.log("[DEBUG] "+JSON.stringify({command,paths:pairs.length,productionWrites:command==="stage-provider"?0:pairs.length}));
