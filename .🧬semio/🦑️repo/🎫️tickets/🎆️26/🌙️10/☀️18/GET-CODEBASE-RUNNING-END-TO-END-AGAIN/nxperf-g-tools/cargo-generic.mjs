import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g=".cache/b/g/m/generations/",cur=JSON.parse(readFileSync(g+"current.json","utf8")),man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
let total=0,owner=0,generic=0,cargoGeneric=0,nativeOwner=0,ownerNoNative=0,nativeNoOwner=0;const ex=[];const kinds={};
for(const [name,row] of Object.entries(man.owners)){
 if(row.kind!=="project")continue;let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
 for(const [t,def] of Object.entries(frag.node?.targets??{})){
  if(def.cache!==true)continue;total++;const e=def.execution??{};
  if(e.nativeOwnerCommand!==undefined){nativeOwner++;if(!e.ownerArguments)nativeNoOwner++;}
  if(e.ownerArguments){owner++;if(e.nativeOwnerCommand===undefined)ownerNoNative++;continue;}
  generic++;const text=JSON.stringify([e.command,e.commands,e.program,e.nativeOwnerCommand]);
  if(/cargo|rustc|wasm-bindgen/i.test(text)){cargoGeneric++;if(ex.length<8)ex.push(name+":"+t+" "+text.slice(0,110));}
 }
}
console.log({total,owner,generic,cargoGeneric,nativeOwner,ownerNoNative,nativeNoOwner});console.log(ex.join("\n"));
