import {readFileSync} from "node:fs";
import {resolve} from "node:path";
const N=process.argv[2],list=readFileSync(process.argv[3],"utf8").split("\n").filter(Boolean);
const m=await import(resolve(N,"🟨️.mjs")),t=await import(resolve(N,"🟦️.ts"));
const view=t.nativeHostFilesystemViewV1(process.cwd());
for(const file of list){
  const manifest=readFileSync(file,"utf8");
  for(const block of manifest.matchAll(/^\[\[package\.metadata\.semio\.playground\]\]\s*$([\s\S]*?)(?=^\[|$(?![\s\S]))/gmu))
    for(const field of ["nativeHost","mcpHost"]){
      let host;try{host=m.parsePlaygroundNativeHostV1(block[1],field);}catch(e){console.log("PARSE",file,field,String(e.message));continue;}
      if(!host)continue;
      try{m.admitPlaygroundNativeHostV1(host,o=>m.nativeHostSourceFactsV1(o,view));}
      catch(e){let facts;try{facts=m.nativeHostSourceFactsV1(host.cratePath,view);}catch(x){facts="facts error: "+x.message}
        console.log("FAIL",file,field,e.message,JSON.stringify(host),JSON.stringify(facts));}
    }
}
console.log("scanned",list.length);
