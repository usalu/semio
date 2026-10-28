import {mkdirSync,writeFileSync,mkdtempSync,readFileSync,realpathSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const plugin=await import(B+"🕸️graph/📥️inference/🟨️.mjs"),api=await import(B+"👁️observation/🟨️.mjs");await plugin.libraryBootstrap;
const root=mkdtempSync(join(realpathSync(tmpdir()),"toml-")),path=join(root,"Cargo.toml");
const f=JSON.parse(readFileSync(B+"👁️observation/🧪️tests/🧫️fixtures/🔣️.json","utf8")).tomlReceiving;
for(const row of f.cases.filter((r:any)=>!r.accepted)){writeFileSync(path,row.source);
 try{await api.captureObservedSourceInputsV1(()=>plugin.cacheInternals.nativeDependencyRoots(".",root,false,new Map(),new Map()),undefined,false);}catch(e:any){console.log(row.name,JSON.stringify(e.tomlObservation),e.cause?.message?.slice(0,200));}}
