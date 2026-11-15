import {mkdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {collectNxGraphV1,installNxGraphV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🧶️nx/🟦️.ts";
// usage: bun prove-nx.ts <scratch dir> <node_modules dir containing nx>
const [base,modules]=process.argv.slice(2) as [string,string];rmSync(base,{recursive:true,force:true});mkdirSync(base,{recursive:true});
const nxBin=`${modules}/nx/dist/bin/nx.js`;
function workspace(d:string){mkdirSync(d,{recursive:true});writeFileSync(d+"/package.json",'{"name":"ws","private":true}');writeFileSync(d+"/nx.json",'{"analytics":false}');
 for(const p of ["a","b","c"]){mkdirSync(`${d}/libs/${p}`,{recursive:true});writeFileSync(`${d}/libs/${p}/project.json`,JSON.stringify({name:"proj-"+p,root:"libs/"+p,targets:{build:{command:"echo libs/"+p,options:{cwd:"{workspaceRoot}/libs/"+p}}}}));writeFileSync(`${d}/libs/${p}/f.txt`,"x");}
 symlinkSync(modules,d+"/node_modules");Bun.spawnSync(["git","init","-q"],{cwd:d});}
const A=base+"/A",B=base+"/B";workspace(A);workspace(B);
const nx=async(cwd:string,data:string,reuse:boolean)=>{const p=Bun.spawn(["node",nxBin,"show","projects"],{cwd,env:{...process.env,NX_DAEMON:"false",NX_WORKSPACE_DATA_DIRECTORY:data,NODE_OPTIONS:"--preserve-symlinks --preserve-symlinks-main",...(reuse?{NX_FORCE_REUSE_CACHED_GRAPH:"true"}:{})},stdout:"pipe",stderr:"pipe"});const t=performance.now(),out=await new Response(p.stdout).text(),err=await new Response(p.stderr).text();await p.exited;return {out:out.trim().split("\n").join(","),ms:Math.round(performance.now()-t),err:err.slice(0,200)};};
console.log("native A",await nx(A,base+"/dataA",false));console.log("native B",await nx(B,base+"/dataB-native",false));
const a=(await collectNxGraphV1(base+"/dataA",A))!,b=(await collectNxGraphV1(base+"/dataB-native",B))!,strip=(t:string)=>t.replace(/"computedAt": \d+/g,"");
for(const n of ["nx/project-graph.json","nx/file-map.json","nx/source-maps.json"])console.log(n,"neutral A == neutral B:",strip(Buffer.from(a.find(d=>d.name===n)!.bytes).toString())===strip(Buffer.from(b.find(d=>d.name===n)!.bytes).toString()));
writeFileSync(B+"/libs/a/project.json",JSON.stringify({name:"proj-a-CHANGED",root:"libs/a",targets:{}}));
await installNxGraphV1(base+"/dataB-installed",a,B);
console.log("reuse installed ->",await nx(B,base+"/dataB-installed",true));console.log("recompute       ->",await nx(B,base+"/dataB-installed",false));
