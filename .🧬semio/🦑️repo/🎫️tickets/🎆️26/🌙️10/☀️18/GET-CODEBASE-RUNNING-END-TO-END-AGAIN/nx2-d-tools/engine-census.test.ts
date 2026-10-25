import {test} from "bun:test";
import {readdirSync,statSync} from "node:fs";
import {nxBuildEngineInputsV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs";
test("census",()=>{
 const inputs=nxBuildEngineInputsV1(),files=(inputs.filter((v:any)=>typeof v==="string") as string[]).map(f=>f.slice(16));
 const dirs=new Map<string,string[]>();for(const f of files){const d=f.slice(0,f.lastIndexOf("/"));(dirs.get(d)??dirs.set(d,[]).get(d)!).push(f);}
 let siblingExtra=0,siblingTotal=0;
 for(const [d,fs] of dirs){const all=readdirSync(d).filter(n=>statSync(d+"/"+n).isFile());siblingTotal+=all.length;siblingExtra+=all.length-fs.length;}
 const bytes=[...dirs.keys()].reduce((n,d)=>n+Buffer.byteLength(d)+20,0);
 console.log(JSON.stringify({files:files.length,dirs:dirs.size,dirGlobBytes:bytes,siblingFilesInDirs:siblingTotal,notInClosure:siblingExtra,testsFiles:files.filter(f=>f.includes("/🧪️tests/")).length}));
});
