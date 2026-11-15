exch=[
("""import {generationSignatureV1,transformGenerationV1,type GenerationDocumentV1} from "./🧳️portable/🟦️.ts";""","""import {documentNameV1,generationSignatureV1,neutralizeGenerationFromMemoryV1,neutralKeyV1,transformGenerationDocumentV1,transformGenerationV1,type GenerationDocumentV1,type NeutralKnownV1,type NeutralRowV1} from "./🧳️portable/🟦️.ts";
import {profilePhaseV1} from "../../../⏱️profile/🟨️.mjs";"""),
("""import {readFileSync,readdirSync} from "node:fs";""","""import {readFileSync,readdirSync,renameSync,rmSync,writeFileSync} from "node:fs";"""),
("""
/** ⬆️ Uploads the newest sealed generation of one store""","""
/** 🧠️ The neutral documents an earlier upload of this store made (original name to neutral name and size), valid only under the key of the root and the code that made them; empty where the file is missing, damaged or of another key. */
function readNeutralMemoV1(file:string,key:string|undefined):NeutralKnownV1{
 if(key===undefined)return new Map();
 try{const value=JSON.parse(readFileSync(file,"utf8")) as {version:number;key:string;entries:[string,string,number][]};if(value.version!==1||value.key!==key||!Array.isArray(value.entries))return new Map();return new Map(value.entries.filter(row=>Array.isArray(row)&&typeof row[0]==="string"&&/^[0-9a-f]{64}$/u.test(row[1])&&Number.isSafeInteger(row[2])).map(row=>[row[0],{name:row[1],size:row[2]}] as const));}catch{return new Map();}
}
/** 💾️ Remembers the neutral documents of the generation just uploaded, replacing the older memory atomically; losing the write only costs the next upload a full rewrite. */
function writeNeutralMemoV1(file:string,key:string|undefined,known:NeutralKnownV1):void{
 if(key===undefined)return;
 const temporary=file+"."+process.pid+".tmp";
 try{writeFileSync(temporary,JSON.stringify({version:1,key,entries:[...known].map(([original,row])=>[original,row.name,row.size])}));renameSync(temporary,file);}catch{rmSync(temporary,{force:true});}
}
class NeutralStaleV1 extends Error{}

/** ⬆️ Uploads the newest sealed generation of one store"""),
(""" const read:GenerationDocumentV1[]=[];
 await each(paths,async path=>{const bytes=await files.read(join(root,path));if(!bytes||bytes.length>policy.maximumDocumentBytes)throw Error("generation document unavailable: "+path);if(!path.includes(sha(bytes)))throw Error("generation document differs from its name: "+path);read.push({path,bytes});});
 const moved=transformGenerationV1(read,location.workspace,"neutralize"),rows=moved.documents.map(item=>({path:item.path,hash:sha(item.bytes),size:item.bytes.length,bytes:item.bytes})).sort((a,b)=>a.path<b.path?-1:1);
 const snapshot=await location.cursors?.snapshotFor(local.generation).catch(()=>undefined),shipped=snapshot!==undefined&&snapshot.bytes.length<=policy.maximumSnapshotBytes?snapshot:undefined;
 if(shipped)rows.push({path:`s/${shipped.instance}.snapshot.br`,hash:sha(shipped.bytes),size:shipped.bytes.length,bytes:shipped.bytes});
 const manifestName=moved.manifestPath.slice(P.manifests.length+1,-5),missing=new Set((await content.missing(instance,rows.map(row=>({hash:row.hash,size:row.size})),signal)).map(digest=>digest.hash));let bytesSent=0;
 await each(rows.filter(row=>missing.has(row.hash)),async row=>{await content.write(instance,{hash:row.hash,size:row.size},row.bytes,signal);bytesSent+=row.size;});""",""" const snapshot=await location.cursors?.snapshotFor(local.generation).catch(()=>undefined),shipped=snapshot!==undefined&&snapshot.bytes.length<=policy.maximumSnapshotBytes?snapshot:undefined;
 const memoFile=root+".neutral.json",memoKey=neutralKeyV1(location.workspace);
 const neutral=async(known:NeutralKnownV1)=>{
  const wanted=paths.filter(path=>path.startsWith(P.manifests+"/")||!known.has(documentNameV1(path))),bytes=new Map<string,Uint8Array>();
  await profilePhaseV1("graph.upload.read",()=>each(wanted,async path=>{const data=await files.read(join(root,path));if(!data||data.length>policy.maximumDocumentBytes)throw Error("generation document unavailable: "+path);if(!path.includes(sha(data)))throw Error("generation document differs from its name: "+path);bytes.set(path,data);}));
  const moved=profilePhaseV1("graph.upload.transform",()=>neutralizeGenerationFromMemoryV1({paths,bytes,root:location.workspace,known})),rows:NeutralRowV1[]=[...moved.rows].sort((a,b)=>a.path<b.path?-1:1);
  if(shipped)rows.push({path:`s/${shipped.instance}.snapshot.br`,hash:sha(shipped.bytes),size:shipped.bytes.length,bytes:shipped.bytes});
  const missing=new Set((await profilePhaseV1("graph.upload.missing",()=>content.missing(instance,rows.map(row=>({hash:row.hash,size:row.size})),signal))).map(digest=>digest.hash)),sending=new Map<string,Uint8Array>();
  await each(rows.filter(row=>missing.has(row.hash)&&row.bytes===undefined),async row=>{
   const data=await files.read(join(root,row.source!));if(!data||!row.source!.includes(sha(data)))throw new NeutralStaleV1("remembered document unavailable");
   const again=transformGenerationDocumentV1(row.source!,data,location.workspace,"neutralize");if(sha(again)!==row.hash)throw new NeutralStaleV1("remembered document differs");sending.set(row.hash,again);
  });
  return{moved,rows,missing,sending};
 };
 let staged;try{staged=await neutral(readNeutralMemoV1(memoFile,memoKey));}catch(error){if(!(error instanceof NeutralStaleV1))throw error;rmSync(memoFile,{force:true});staged=await neutral(new Map());}
 const{moved,rows,missing,sending}=staged;
 const manifestName=moved.manifestPath.slice(P.manifests.length+1,-5);let bytesSent=0;
 await profilePhaseV1("graph.upload.write",()=>each(rows.filter(row=>missing.has(row.hash)),async row=>{const data=row.bytes??sending.get(row.hash)!;await content.write(instance,{hash:row.hash,size:row.size},data,signal);bytesSent+=row.size;}));
 writeNeutralMemoV1(memoFile,memoKey,moved.known);"""),
]
