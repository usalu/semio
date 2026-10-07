#!/usr/bin/env bun
import {resolve,relative,dirname} from "node:path";
import type RustParser from "web-tree-sitter";
import {runBudgetedTestCommand} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";

const root=resolve(import.meta.dir,"../../../../../../../.."),test=resolve(import.meta.dir,"🧪️tests/🟦️.ts");
if(process.argv.slice(2).join(" ")==="syntax"){
 const {default:Parser}=await import("web-tree-sitter");
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(new Uint8Array(await Bun.file(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",root)),"out/tree-sitter-rust.wasm")).arrayBuffer())));
 const providers=await Bun.file(resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-before.json")).json() as {rows:Array<{path:string}>};
 const paths=providers.rows.filter(row=>row.path.endsWith(".rs")).map(row=>"prod-drafts/"+row.path);
 paths.push("🧭️producer/🗂️catalog/🧪️tests/🦀️.rs","🧭️producer/📥️source/🧪️tests/🦀️.rs","🧭️producer/🏪️store/🧪️tests/🦀️.rs","🔌️capture/🦀️.rs","🔌️capture/👥️context/🦀️.rs");
 const rows=[];
 for(const path of paths){
  const parse=(source:string)=>{const tree=parser.parse(source),errors:Array<{type:string;line:number;column:number;text:string}>=[];
   const inspect=(node:RustParser.SyntaxNode)=>{if(node.type==="ERROR"||node.isMissing())errors.push({type:node.type,line:node.startPosition.row+1,column:node.startPosition.column+1,text:node.text.slice(0,200)});for(const child of node.children)if(child.hasError()||child.type==="ERROR"||child.isMissing())inspect(child);};
   if(tree.rootNode.hasError())inspect(tree.rootNode);tree.delete();return errors;
  };
  const source=await Bun.file(resolve(import.meta.dir,path)).text(),errors=parse(source);
  let baselineErrors:ReturnType<typeof parse>=[];if(path.startsWith("prod-drafts/"))baselineErrors=parse(await Bun.file(resolve(root,path.slice("prod-drafts/".length))).text());
  const inheritedGrammarLimitation=errors.length>0&&JSON.stringify(errors)===JSON.stringify(baselineErrors);
  if(await Bun.file(resolve(import.meta.dir,path)).text()!==source)throw Error("draft changed during grammar proof: "+path);
  const hash=new Bun.CryptoHasher("sha256");hash.update(source);rows.push({path,bytes:Buffer.byteLength(source),sha256:hash.digest("hex"),errors,baselineErrors,inheritedGrammarLimitation});
 }
 parser.delete();await Bun.write(resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/independent-rust-draft-grammar.json"),JSON.stringify({time:new Date().toISOString(),oracle:"installed web-tree-sitter 0.20.8 with tree-sitter-wasms 0.1.13 Rust grammar; syntax only, no native compiler or runtime",rows},null,2)+"\n");
 console.log("[DEBUG] independent Rust draft grammar",rows.length,"sources; new grammar faults",rows.filter(row=>row.errors.length&&!row.inheritedGrammarLimitation),"inherited grammar limitations",rows.filter(row=>row.inheritedGrammarLimitation).map(row=>({path:row.path,errors:row.errors})));
 process.exit(rows.some(row=>row.errors.length&&!row.inheritedGrammarLimitation)?1:0);
}
if(process.argv.slice(2).join(" ")==="providers-refresh-store"){
 const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",receiptPath=resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-before.json"),original=await Bun.file(receiptPath).text();
 const receipt=JSON.parse(original) as {rows:Array<{path:string;before:string;sha256:string;bytes:number}>},row=receipt.rows.find(row=>row.path===path);if(!row)throw Error("Store source not captured");
 const file=resolve(import.meta.dir,"prod-drafts",path),draft=await Bun.file(file).text(),current=await Bun.file(resolve(root,path)).text();
 const removed="    pub use super::mounted_pack_session::{RetainedTypedPackCloseStep, RetainedTypedPackOwner, RetainedTypedPackSession};",inserted="    pub use super::mounted_pack_session::{RetainedTypedPackAllocationError, RetainedTypedPackCloseStep, RetainedTypedPackOwner, RetainedTypedPackSession};";
 if(row.before.indexOf(removed)===-1||row.before.indexOf(removed)!==row.before.lastIndexOf(removed)||row.before.replace(removed,inserted)!==draft)throw Error("Store local delta differs from reviewed explicit export");
 if(current.indexOf(removed)===-1||current.indexOf(removed)!==current.lastIndexOf(removed))throw Error("Store concurrent export needs source review");
 const after=current.replace(removed,inserted),observedBefore=row.before;
 if(await Bun.file(receiptPath).text()!==original||await Bun.file(file).text()!==draft||await Bun.file(resolve(root,path)).text()!==current)throw Error("Store full byte guard changed");
 const previousAdmission=resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-store-concurrent-refresh-admission.json"),previousBefore=resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-store-before-concurrent-refresh.json");
 if(await Bun.file(previousAdmission).exists())await Bun.write(resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-store-concurrent-refresh-history-"+Date.now()+".json"),JSON.stringify({previousAdmission:await Bun.file(previousAdmission).json(),previousBefore:await Bun.file(previousBefore).json()},null,2)+"\n");
 await Bun.write(resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-store-before-concurrent-refresh.json"),original);
 await Bun.write(file,after);row.before=current;row.bytes=Buffer.byteLength(current);const hash=new Bun.CryptoHasher("sha256");hash.update(current);row.sha256=hash.digest("hex");
 await Bun.write(receiptPath,JSON.stringify(receipt,null,2)+"\n");
 await Bun.write(resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-store-concurrent-refresh-admission.json"),JSON.stringify({time:new Date().toISOString(),authorship:"observed concurrent Store changes, not this lane; one reviewed local export reapplied",path,observedBefore,observedCurrent:current,localBefore:draft,localAfter:after,inverseExact:after.replace(inserted,removed)===current},null,2)+"\n");
 console.log("[DEBUG] full concurrent Store bytes preserved, one explicit local export reapplied; actual source untouched");process.exit(0);
}
if(["catalog-ready","source-ready","store-ready"].includes(process.argv.slice(2).join(" "))){
 const command=process.argv.slice(2).join(" "),source=command==="source-ready",store=command==="store-ready",name=store?"store":source?"source":"catalog";
 const owner=store?"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📥️mounted-pack/":"🧰️framework/🔨️modules/🎒️pack/📐️format/",base=store?"🧭️producer/🏪️store/":source?"🧭️producer/📥️source/":"🧭️producer/🗂️catalog/",rows=[];
 for(const suffix of ["🟦️.ts","🧪️tests/🦀️.rs","🧬️schema/🔣️.json","🧫️fixtures/🔣️.json"]){
  const input=base+suffix,path=owner+input;if(await Bun.file(resolve(root,path)).exists())throw new Error("Catalog source already exists: "+path);
  let after=await Bun.file(resolve(import.meta.dir,input)).text();if(suffix==="🟦️.ts"){const canonical=store?relative(dirname(resolve(root,path)),resolve(root,"🧰️framework/🔨️modules/🎒️pack/⚠️error/🟦️.ts")).replaceAll("\\","/"):"../../../⚠️error/🟦️.ts";after=after.replace('from "../../🟦️.ts"','from "'+canonical+'"');}
  const hash=new Bun.CryptoHasher("sha256");hash.update(after);rows.push({path,input,before:null,after,bytes:Buffer.byteLength(after),sha256:hash.digest("hex"),edit:{start:0,end:0,removed:"",inserted:after},inverseExact:true});
 }
 for(const row of rows)if(await Bun.file(resolve(root,row.path)).exists())throw new Error("Catalog source appeared during capture: "+row.path);
 await Bun.write(resolve(import.meta.dir,"🧫️fixtures/🧬️"+name+"-draft-ready.json"),JSON.stringify({time:new Date().toISOString(),phase:"unmounted "+name+" typed source metadata; broader producer retirement remains incomplete",mountReady:false,nativeNewLaws:1,nativeExecuted:false,nativeWholeRoute:store?"bun nx run @semio-tech/framework-os-kernel:test-native --skip-nx-cache":"bun nx run @semio-tech/framework-pack-rs:test --skip-nx-cache -- --lib",rows},null,2)+"\n");
 console.log("[DEBUG] unmounted "+name+" full-source/inverse receipt",rows.length,"rows; one native law authored only");process.exit(0);
}
if(process.argv.slice(2).join(" ")==="providers-drift"){
 const capture=await Bun.file(resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-before.json")).json() as {rows:Array<{path:string;before:string}>};
 const rows=[];
 for(const source of capture.rows){
  const current=await Bun.file(resolve(root,source.path)).text();if(current===source.before)continue;
  const oldLines=source.before.split("\n"),newLines=current.split("\n");let prefix=0,oldEnd=oldLines.length,newEnd=newLines.length;
  while(prefix<oldEnd&&prefix<newEnd&&oldLines[prefix]===newLines[prefix])prefix++;
  while(oldEnd>prefix&&newEnd>prefix&&oldLines[oldEnd-1]===newLines[newEnd-1]){oldEnd--;newEnd--;}
  const before=oldLines.slice(prefix,oldEnd),after=newLines.slice(prefix,newEnd),width=after.length+1;
  if((before.length+1)*width>12000000)throw new Error("bounded source diff exceeded limit: "+source.path);
  const matrix=new Uint32Array((before.length+1)*width);
  for(let i=before.length-1;i>=0;i--)for(let j=after.length-1;j>=0;j--)matrix[i*width+j]=before[i]===after[j]?matrix[(i+1)*width+j+1]+1:Math.max(matrix[(i+1)*width+j],matrix[i*width+j+1]);
  const hunks=[];let i=0,j=0;
  while(i<before.length||j<after.length){
   if(i<before.length&&j<after.length&&before[i]===after[j]){i++;j++;continue;}
   const start=i,newStart=j;
   while(i<before.length||j<after.length){if(i<before.length&&j<after.length&&before[i]===after[j])break;if(j===after.length||(i<before.length&&matrix[(i+1)*width+j]>=matrix[i*width+j+1]))i++;else j++;}
   hunks.push({start:start+prefix,newStart:newStart+prefix,removed:before.slice(start,i).join("\n"),inserted:after.slice(newStart,j).join("\n"),contextBefore:oldLines.slice(Math.max(0,start+prefix-2),start+prefix),contextAfter:oldLines.slice(i+prefix,i+prefix+2)});
  }
  rows.push({path:source.path,before:source.before,current,hunks});console.log("[DEBUG] observed concurrent provider input",source.path,JSON.stringify(hunks,null,2));
 }
 await Bun.write(resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-concurrent-input-drift.json"),JSON.stringify({time:new Date().toISOString(),authorship:"observed concurrent changes, not this lane",rows},null,2)+"\n");
 process.exit(0);
}
if(process.argv.slice(2).join(" ")==="providers-refresh-concurrent"){
 const receiptPath=resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-before.json"),original=await Bun.file(receiptPath).text(),receipt=JSON.parse(original) as {rows:Array<{path:string;before:string;sha256:string;bytes:number}>};
 const drift=await Bun.file(resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-concurrent-input-drift.json")).json() as {rows:Array<{path:string;before:string;current:string;hunks:Array<{removed:string;inserted:string;contextBefore:string[];contextAfter:string[]}>}>};
 const planned=[];
 for(const row of drift.rows){
  const source=receipt.rows.find(source=>source.path===row.path);if(!source||source.before!==row.before||await Bun.file(resolve(root,row.path)).text()!==row.current)throw new Error("concurrent input changed before draft refresh: "+row.path);
  const file=resolve(import.meta.dir,"prod-drafts",row.path),before=await Bun.file(file).text();let after=before;
  if(row.path.includes("📡️replication/⚙️codec/")){
   if(row.hunks.length!==1||!after.includes('semio_framework_deflate::DeflateError::OutputLimitExceeded=>PackError::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"retained deflate physical ceiling"))'))throw new Error("typed physical constructor conflict lacks explicit preserved source branch");
  }else if(before===row.before)after=row.current;
  else for(const hunk of row.hunks){
   const removed=hunk.removed||[...hunk.contextBefore,...hunk.contextAfter].join("\n"),inserted=hunk.removed?hunk.inserted:[...hunk.contextBefore,hunk.inserted,...hunk.contextAfter].join("\n");
   if(!removed||after.indexOf(removed)<0||after.indexOf(removed)!==after.lastIndexOf(removed))throw new Error("concurrent source hunk overlaps local draft: "+row.path);
   after=after.replace(removed,inserted);
  }
  planned.push({path:row.path,file,before,after,observedBefore:row.before,observedCurrent:row.current});
 }
 if(await Bun.file(receiptPath).text()!==original)throw new Error("provider receipt changed during refresh");
 for(const row of planned)if(await Bun.file(row.file).text()!==row.before||await Bun.file(resolve(root,row.path)).text()!==row.observedCurrent)throw new Error("draft/source changed during refresh: "+row.path);
 await Bun.write(resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-before-concurrent-refresh.json"),original);
 for(const row of planned){await Bun.write(row.file,row.after);const source=receipt.rows.find(source=>source.path===row.path)!;source.before=row.observedCurrent;source.bytes=Buffer.byteLength(source.before);const hash=new Bun.CryptoHasher("sha256");hash.update(source.before);source.sha256=hash.digest("hex");}
 await Bun.write(receiptPath,JSON.stringify(receipt,null,2)+"\n");
 await Bun.write(resolve(import.meta.dir,"../🗑️generated/pack-producer-authority/provider-concurrent-refresh-admission.json"),JSON.stringify({time:new Date().toISOString(),authorship:"observed concurrent inputs, not authored here; local drafts preserve these producer changes",rows:planned.map(({file,...row})=>row)},null,2)+"\n");
 console.log("[DEBUG] unmounted drafts refreshed against",planned.length,"concurrent current sources; actual source unchanged");process.exit(0);
}
if(process.argv.slice(2).join(" ")==="providers-canonical"){
 const capture=await Bun.file(resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-before.json")).json() as {rows:Array<{path:string;before:string}>};
 const path="🧰️framework/🔨️modules/🎒️pack/⚠️error/";
 const rust=capture.rows.find(row=>row.path===path+"🦀️.rs");
 if(!rust||await Bun.file(resolve(root,rust.path)).text()!==rust.before)throw new Error("canonical current input changed before draft rebase");
 const footer=rust.before.slice(rust.before.indexOf("#[cfg(test)]")),draftRust=await Bun.file(resolve(import.meta.dir,"🦀️.rs")).text();
 if(!footer||!draftRust.includes("#[cfg(test)]"))throw new Error("canonical original/draft test links missing");
 await Bun.write(resolve(import.meta.dir,"prod-drafts",rust.path),draftRust.slice(0,draftRust.indexOf("#[cfg(test)]"))+footer);
 const draftTs=await Bun.file(resolve(import.meta.dir,"🟦️.ts")).text();
 const typeScript=draftTs.replace("../../../../../../../../🧰️framework/🔨️modules/🌱️value/⚠️refusal/🟦️.ts","../../🌱️value/⚠️refusal/🟦️.ts").replace("../../../../../../../../🧰️framework/🔨️modules/⚠️diagnostic/🚧️text-error/🟦️.ts","../../⚠️diagnostic/🚧️text-error/🟦️.ts");
 if(typeScript===draftTs)throw new Error("canonical draft imports were not rebound");
 await Bun.write(resolve(import.meta.dir,"prod-drafts",path+"🟦️.ts"),typeScript);
 console.log("[DEBUG] ticket-only canonical drafts refreshed; both original native owner laws retained; actual production unchanged");
 process.exit(0);
}
if(process.argv.slice(2).join(" ")==="transport-ready"){
 const owner="🧰️framework/🔨️modules/🎒️pack/⚠️error/";
 const paths=["🔌️capture/🦀️.rs","🔌️capture/🟦️.ts","🔌️capture/🚦️provider/🟦️.ts","🔌️capture/🧪️tests/🦀️.rs","🔌️capture/👥️context/🦀️.rs","🔌️capture/👥️context/🟦️.ts","🔌️capture/👥️context/🧪️tests/🦀️.rs","🧬️schema/🔌️capture/🔣️.json","🧬️schema/🔌️capture/🚦️provider/🔣️.json","🧬️schema/🔌️capture/👥️context/🔣️.json","🧬️schema/🔌️capture/👥️context/🎛️policy/🔣️.json","🧬️schema/🔌️capture/👥️context/🧾️claim/🔣️.json","🧬️schema/🔌️capture/👥️context/💥️lifecycle/🔣️.json","🧫️fixtures/🔌️capture/🔣️.json","🧫️fixtures/🔌️capture/🚦️provider/🔣️.json","🧫️fixtures/🔌️capture/👥️context/🔣️.json","🧫️fixtures/🔌️capture/👥️context/🎛️policy/🔣️.json","🧫️fixtures/🔌️capture/👥️context/🧾️claim/🔣️.json","🧫️fixtures/🔌️capture/👥️context/💥️lifecycle/🔣️.json"];
 const rows=[];
 for(const input of paths){
  const path=owner+(input.startsWith("🧬️schema/")?input.replace("🧬️schema/","🧬️schema/🧭️cause/"):input.startsWith("🧫️fixtures/")?input.replace("🧫️fixtures/","🧫️fixtures/🧭️cause/"):input);
  const file=Bun.file(resolve(root,path)),before=await file.exists()?await file.text():null;
  if(before!==null)throw new Error("transport source successor already exists and must be rebased explicitly: "+path);
  let after=await Bun.file(resolve(import.meta.dir,input)).text();
  if(input==="🔌️capture/🧪️tests/🦀️.rs")after=after.replace("../../🧫️fixtures/🔌️capture/🚦️provider/🔣️.json","../../🧫️fixtures/🧭️cause/🔌️capture/🚦️provider/🔣️.json");
  if(input==="🔌️capture/👥️context/🧪️tests/🦀️.rs")after=after.replaceAll("../../../🧫️fixtures/🔌️capture/","../../../🧫️fixtures/🧭️cause/🔌️capture/");
  const hash=new Bun.CryptoHasher("sha256");hash.update(after);
  rows.push({path,input,before,after,bytes:Buffer.byteLength(after),sha256:hash.digest("hex"),edit:{start:0,end:0,removed:"",inserted:after},inverseExact:true});
 }
 for(const row of rows)if(await Bun.file(resolve(root,row.path)).exists())throw new Error("transport source appeared during draft capture: "+row.path);
 const nativeNewLaws=rows.filter(row=>row.input.endsWith("🧪️tests/🦀️.rs")).reduce((count,row)=>count+[...row.after.matchAll(/#\[test\]/g)].length,0);
 await Bun.write(resolve(import.meta.dir,"🧫️fixtures/🧬️transport-draft-ready.json"),JSON.stringify({time:new Date().toISOString(),phase:"unmounted transport/context inputs; new paths remain absent from actual production",mountReady:false,nativeNewLaws,nativeExecuted:false,proposedCanonicalWholeRoster:4+nativeNewLaws,portableNewLaws:11,rows},null,2)+"\n");
 console.log("[DEBUG] unmounted transport full-source/inverse receipt",rows.length,"rows; native laws are authored only");
 process.exit(0);
}
if(process.argv.slice(2).join(" ")==="ready-codec-admission"){
 const previous=await Bun.file(resolve(import.meta.dir,"🧫️fixtures/🧬️test-only-source-ready-imported.json")).json() as {rows:Array<{path:string;after:string}>};
 const rows=[];
 for(const row of previous.rows){
  const before=row.after,after=await Bun.file(resolve(root,row.path)).text();
  let start=0,end=before.length,tail=after.length;
  while(start<end&&start<tail&&before[start]===after[start])start++;
  while(end>start&&tail>start&&before[end-1]===after[tail-1]){end--;tail--;}
  const removed=before.slice(start,end),inserted=after.slice(start,tail),hash=new Bun.CryptoHasher("sha256");hash.update(after);
  rows.push({path:row.path,before,after,bytes:Buffer.byteLength(after),sha256:hash.digest("hex"),edit:{start,end,removed,inserted},inverseExact:after.slice(0,start)+removed+after.slice(tail)===before,changed:before!==after});
 }
 const delta=rows.filter(row=>row.changed);
 if(delta.length!==2||delta.some(row=>!row.path.endsWith("🧭️cause/📡️codec/🔣️.json")))throw new Error("unexpected imported27 delta during codec admission capture");
 const schema=delta.find(row=>row.path.includes("🧬️schema/")),fixture=delta.find(row=>row.path.includes("🧫️fixtures/"));
 if(!schema||!fixture)throw new Error("missing codec schema or corpus");
 const {default:Ajv}=await import("ajv/dist/2020.js"),validate=new Ajv({strict:true}).compile(JSON.parse(schema.after));
 const currentFixture=JSON.parse(fixture.after),previousFixture=JSON.parse(fixture.before);
 if(!validate(currentFixture)||JSON.stringify(currentFixture.cases)!==JSON.stringify(previousFixture.cases))throw new Error("codec successor changed original grammar cases or violates its closed schema");
 for(const row of rows)if(await Bun.file(resolve(root,row.path)).text()!==row.after)throw new Error("source changed during codec admission capture");
 await Bun.write(resolve(import.meta.dir,"🧫️fixtures/🧬️test-only-source-ready-codec-admission.json"),JSON.stringify({time:new Date().toISOString(),phase:"read-only full imported27 successor for observed concurrent retained-admission schema/corpus additions; authorship is not attributed",closedSchemaValidated:true,originalGrammarCasesIntact:true,nativeProducerRowsIntact:true,nativeNewLaws:6,portableNewLaws:4,rows},null,2)+"\n");
 console.log("[DEBUG] exact full imported27 successor captured; two schema/corpus deltas; inverse exact",rows.every(row=>row.inverseExact));
 process.exit(0);
}
if(process.argv.slice(2).join(" ")==="providers-ready"){
 const captured=await Bun.file(resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-before.json")).json() as {rows:Array<{path:string;before:string}>};
 const rows=[];
 for(const source of captured.rows){
  const after=await Bun.file(resolve(import.meta.dir,"prod-drafts",source.path)).text(),current=await Bun.file(resolve(root,source.path)).text(),before=source.before;
  let start=0,end=before.length,tail=after.length;
  while(start<end&&start<tail&&before[start]===after[start])start++;
  while(end>start&&tail>start&&before[end-1]===after[tail-1]){end--;tail--;}
  const removed=before.slice(start,end),inserted=after.slice(start,tail),hash=new Bun.CryptoHasher("sha256");hash.update(after);
  rows.push({path:source.path,before,after,current,bytes:Buffer.byteLength(after),sha256:hash.digest("hex"),edit:{start,end,removed,inserted},inverseExact:after.slice(0,start)+removed+after.slice(tail)===before,currentExact:current===before,changed:before!==after,legacyCandidates:{stringSchema:[...after.matchAll(/\b(?:PackError|Self)::Schema\b/g)].length,tupleIo:[...after.matchAll(/\b(?:PackError|Self)::Io\s*\(/g)].length,untypedLimit:[...after.matchAll(/\b(?:PackError|Self)::LimitExceeded\s*\(/g)].length,kindlessRetained:[...after.matchAll(/\bPackError::RetainedMalformed\s*\{\s*what\s*:/g)].length}});
 }
 await Bun.write(resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-draft-ready.json"),JSON.stringify({time:new Date().toISOString(),phase:"unmounted provider draft currentness and inverse; incomplete production is not admitted",mountReady:false,rows},null,2)+"\n");
 console.log("[DEBUG] unmounted provider draft rows",rows.length,"inverse exact",rows.every(row=>row.inverseExact),"changed",rows.filter(row=>row.changed).map(row=>row.path),"current gaps",rows.filter(row=>!row.currentExact).map(row=>row.path));
 process.exit(0);
}
if(process.argv[2]==="providers-add"){
 const file=resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-before.json"),receipt=await Bun.file(file).json() as {rows:Array<{path:string;draft:string;before:string;sha256:string;bytes:number;priorChanged:boolean}>};
 for(const path of process.argv.slice(3)){
  if(receipt.rows.some(row=>row.path===path))throw new Error("provider already captured: "+path);
  const before=await Bun.file(resolve(root,path)).text(),hash=new Bun.CryptoHasher("sha256");hash.update(before);
  const draft=resolve(import.meta.dir,"prod-drafts",path);
  if(await Bun.file(draft).exists()&&await Bun.file(draft).text()!==before)throw new Error("untracked edited draft must be refreshed explicitly: "+path);
  await Bun.write(draft,before);
  if(await Bun.file(resolve(root,path)).text()!==before)throw new Error("provider source changed during added capture: "+path);
  receipt.rows.push({path,draft:path,before,sha256:hash.digest("hex"),bytes:Buffer.byteLength(before),priorChanged:false});
 }
 await Bun.write(file,JSON.stringify(receipt,null,2)+"\n");
 console.log("[DEBUG] provider input rows now",receipt.rows.length);
 process.exit(0);
}
if(process.argv.slice(2).join(" ")==="providers-capture"){
 const prior=await Bun.file(resolve(import.meta.dir,"current-mixed-producer-and-pattern-candidates.json")).json() as {sources:Array<{path:string;fullSource:string}>};
 const extras=["🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs","🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml","🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml","🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs","🧰️framework/🔨️modules/🎒️pack/⚠️error/🟦️.ts"];
 const rows=[];
 for(const path of new Set([...prior.sources.map(row=>row.path),...extras])){
  const before=await Bun.file(resolve(root,path)).text(),hash=new Bun.CryptoHasher("sha256");hash.update(before);
  const draft=resolve(import.meta.dir,"prod-drafts",path);
  if(await Bun.file(draft).exists())throw new Error("existing provider draft must be refreshed explicitly: "+path);
  await Bun.write(draft,before);
  rows.push({path,draft:path,before,sha256:hash.digest("hex"),bytes:Buffer.byteLength(before),priorChanged:prior.sources.some(row=>row.path===path&&row.fullSource!==before)});
 }
 for(const row of rows)if(await Bun.file(resolve(root,row.path)).text()!==row.before)throw new Error("provider source changed during capture: "+row.path);
 await Bun.write(resolve(import.meta.dir,"🧫️fixtures/🧬️production-provider-before.json"),JSON.stringify({time:new Date().toISOString(),phase:"current guarded provider inputs; unmounted exact source copies",rows},null,2)+"\n");
 console.log("[DEBUG] unmounted provider inputs",rows.length,"current exact rows; independent changes",rows.filter(row=>row.priorChanged).map(row=>row.path));
 process.exit(0);
}
if(["ready","ready-repair","ready-import"].includes(process.argv.slice(2).join(" "))){
 const command=process.argv.slice(2).join(" "),repair=command!=="ready",importRepair=command==="ready-import";
 const previous=await Bun.file(resolve(import.meta.dir,importRepair?"🧫️fixtures/🧬️test-only-source-ready-repaired.json":repair?"🧫️fixtures/🧬️test-only-source-ready-final.json":"🧫️fixtures/🧬️test-only-source-ready.json")).json() as {rows:Array<{path:string;before:string|null;after:string}>};
 const additional=["🧰️framework/🔨️modules/🎒️pack/⚠️error/🧬️schema/🧭️cause/📋️paged/🔣️.json","🧰️framework/🔨️modules/🎒️pack/⚠️error/🧫️fixtures/🧭️cause/📋️paged/🔣️.json","🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs","🧰️framework/🔨️modules/🎒️pack/⚠️error/🧬️schema/🧭️cause/📡️codec/🔣️.json","🧰️framework/🔨️modules/🎒️pack/⚠️error/🧫️fixtures/🧭️cause/📡️codec/🔣️.json","🧰️framework/🔨️modules/🎒️pack/⚠️error/🧬️schema/🧭️cause/🪪️kind/🔣️.json","🧰️framework/🔨️modules/🎒️pack/⚠️error/🧫️fixtures/🧭️cause/🪪️kind/🔣️.json"];
 const rows=[];
 for(const path of new Set([...previous.rows.map(row=>row.path),...additional])){
  const prior=previous.rows.find(row=>row.path===path),after=await Bun.file(resolve(root,path)).text(),before=repair&&prior?prior.after:prior?.before??(path===additional[2]?after:null);
  let start=0,end=before?.length??0,tail=after.length;
  if(before!==null){while(start<end&&start<tail&&before[start]===after[start])start++;while(end>start&&tail>start&&before[end-1]===after[tail-1]){end--;tail--;}}
  const removed=before?.slice(start,end)??"",inserted=after.slice(start,tail),inverse=after.slice(0,start)+removed+after.slice(tail);
  const hash=new Bun.CryptoHasher("sha256");hash.update(after);
  rows.push({path,before,after,bytes:Buffer.byteLength(after),sha256:hash.digest("hex"),edit:{start,end,removed,inserted},inverseExact:before===null?inverse==="":inverse===before,status:before===null?"created":before===after?"unchanged":"updated",priorReadyDrift:prior?prior.after!==after:false});
 }
 const allowed=importRepair?"🧰️framework/🔨️modules/🎒️pack/🔌️io/🧪️tests/🧭️producer-authority/🦀️.rs":"🧰️framework/🔨️modules/🎒️pack/🧪️tests/🧭️producer-authority/🦀️.rs";
 if(repair&&rows.filter(row=>row.priorReadyDrift).some(row=>row.path!==allowed))throw new Error("unexpected concurrent source delta during single test repair");
 await Bun.write(resolve(import.meta.dir,importRepair?"🧫️fixtures/🧬️test-only-source-ready-imported.json":repair?"🧫️fixtures/🧬️test-only-source-ready-repaired.json":"🧫️fixtures/🧬️test-only-source-ready-final.json"),JSON.stringify({time:new Date().toISOString(),phase:importRepair?"actual Pack test-only source-ready after explicit canonical native IO test import":repair?"actual Pack test-only source-ready after single Io arm brace repair":"actual Pack test-only source-ready with borrowed PagedList factories, third-party grammar, optional refusal and exhaustive cause kind",nativeNewLaws:6,portableNewLaws:4,independentGeneratedLaunchAdmission:"Root normal Gen12/check terminal GREEN; GUI60 published",rows},null,2)+"\n");
 console.log("[DEBUG] Pack Ready captured",rows.length,"complete source rows; inverse exact",rows.every(row=>row.inverseExact),"; six native laws pending compiler/runtime");
 process.exit(0);
}
if(process.argv.slice(2).join(" ")!=="reference")throw new Error("expected reference");
await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",test],{cwd:root,budgetMs:30000,throwOnFailure:true});
await runBudgetedTestCommand(process.execPath,["test",test],{cwd:root,budgetMs:30000,throwOnFailure:true});
