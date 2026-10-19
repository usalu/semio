import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {apply,parse,prop,setProps,type Edit} from "../cache-fw-tools/jsonspan.ts";
const root=process.cwd(),C=process.argv[2]!,write=process.argv.includes("--write"),{storeReceiptOutputV1}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🟨️.mjs"));
const rows:{project:string;target:string;file:string;cargo:boolean;intent:boolean}[]=JSON.parse(readFileSync(join(C,"🗑️generated/store-out/classify.json"),"utf8"));
const stage=new Set(["semio-framework-os-infinite:fonts","@semio-tech/framework-renderer-wgpu:wasm","@semio-tech/repo-coordinator:build","@semio-tech/repo-test-dotnet:build","@semio-tech/mit-bestand-praesentation-projektetage:build","os-hub-admin:build","@semio-tech/mit-bestand-demonstrator:build","@semio-tech/semio-tech-play:build"]),excluded=new Set(["@semio-tech/framework-os-mcp-rs:build-release"]);
const declared=new Set(rows.filter(row=>row.cargo&&!row.intent&&!excluded.has(row.project+":"+row.target)||stage.has(row.project+":"+row.target)).map(row=>row.project+":"+row.target));
const byFile=new Map<string,string[]>();
for(const row of rows)if(declared.has(row.project+":"+row.target))byFile.set(row.file,[...byFile.get(row.file)??[],row.target]);
const seen=new Set(rows.map(row=>row.project+":"+row.target));
for(const name of stage)if(!seen.has(name))throw Error("Unclassified declared target: "+name);
console.log("declared targets: "+declared.size);
let changed=0;
for(const [file,targets] of byFile){
 const path=join(root,file),text=readFileSync(path,"utf8"),document=parse(text),edits:Edit[]=[],targetsNode=prop(document,"targets")!.value;
 for(const target of targets){const node=prop(targetsNode,target)!.value;edits.push(...setProps(text,node,[["outputs",[storeReceiptOutputV1(target)]]],["cache","inputs"]));}
 const next=apply(text,edits);JSON.parse(next);
 if(next!==text){changed+=1;console.log((write?"write ":"plan ")+file+" "+targets.join(","));if(write)writeFileSync(path,next);}
}
console.log("manifests changed: "+changed);
