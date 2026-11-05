/** 🧭️ Trace only unconditional first-party Cargo edges from current Puzzle acceptance owners. */
import {readFile,writeFile} from "node:fs/promises";
import {dirname,resolve,relative,join} from "node:path";
const root=process.cwd(),cache=new Map<string,any>(),unresolved=new Map<string,any>();
async function manifest(file:string){if(!cache.has(file))cache.set(file,Bun.TOML.parse(await readFile(file,"utf8")));return cache.get(file);}
async function workspace(file:string,m:any){
 if(typeof m.package?.workspace==="string")return resolve(dirname(file),m.package.workspace,"Cargo.toml");
 for(let dir=dirname(file);dir.startsWith(root);dir=dirname(dir)){
  const path=join(dir,"Cargo.toml");try{if((await manifest(path)).workspace)return path}catch(error){if(error.code!=="ENOENT")throw error}
  if(dir===root)break;
 }return undefined;
}
async function edges(file:string,includeDev:boolean,potential=false){
 const m=await manifest(file),wfile=await workspace(file,m),w=wfile?await manifest(wfile):undefined,out=[];
 for(const section of [m,...(potential?Object.values(m.target??{}):[])])for(const kind of ["dependencies","build-dependencies",...(includeDev?["dev-dependencies"]:[])]){
  for(const[alias,raw]of Object.entries(section[kind]??{})){
   let d:any=typeof raw==="object"?raw:{version:raw},base=dirname(file);
   if(d.workspace){const inherited=w?.workspace?.dependencies?.[alias];if(!inherited)throw Error("missing workspace edge "+alias+" at "+file);d={...(typeof inherited==="object"?inherited:{version:inherited}),...d};base=dirname(wfile!);}
   if(typeof d.path!=="string"){if((d.package??alias).startsWith("semio"))unresolved.set(file+"#"+alias,{manifest:relative(root,file),alias,package:d.package??alias,definition:d});continue;}if(!potential&&d.optional)continue;
   const target=resolve(base,d.path,"Cargo.toml");if(!target.startsWith(root+"/"))continue;
   out.push({alias,kind,target,package:(await manifest(target)).package?.name});
  }
 }return out;
}
const starts=[{name:"physical2d-test",path:"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml",dev:true},{name:"hub-native-test",path:"🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml",dev:true},{name:"hub-wasm-production",path:"🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml",dev:false}];
const rows=[];
for(const start of [...starts.map(row=>({...row,potential:false})),...starts.map(row=>({...row,name:row.name+"-all-optional-targets",potential:true}))]){
 const first=resolve(root,start.path),queue=[first],seen=new Set([first]),parents=new Map<string,any>(),reached=[];
 for(let i=0;i<queue.length;i++){
  const file=queue[i];
  if(file.includes("generation2d")){
   const chain=[];for(let at=file;at!==first;){const step=parents.get(at);chain.push({from:relative(root,step.from),...step.edge,target:relative(root,step.edge.target)});at=step.from;}chain.reverse();
   reached.push({manifest:relative(root,file),package:(await manifest(file)).package.name,chain});
  }
  for(const edge of await edges(file,file===first&&start.dev,start.potential)){if(!seen.has(edge.target)){seen.add(edge.target);parents.set(edge.target,{from:file,edge});queue.push(edge.target);}}
 }
 rows.push({root:start.name,visited:seen.size,manifests:[...seen].map(path=>relative(root,path)),reached});
 console.log(JSON.stringify({root:start.name,visited:seen.size,reached:reached.map(row=>row.package)}));
}
await writeFile(resolve(import.meta.dir,"🗑️generated/generation2d-reachability.json"),JSON.stringify({rows,unresolvedFirstPartyRegistryEdges:[...unresolved.values()]},null,2)+"\n");
console.log(JSON.stringify({unresolvedFirstPartyRegistryEdges:[...unresolved.values()]}));
