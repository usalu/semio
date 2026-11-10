import{readdirSync,readFileSync,writeFileSync,existsSync}from"node:fs";
import{join,dirname}from"node:path";
const N="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/";
const{createStatIndexV1}=await import(N+"🗂️stat-index/🟨️.mjs");const{createDerivedGraphV1}=await import(N+"🟨️.mjs"),{KIND_RULES_V1}=await import(N+"🧩️kinds/🟨️.mjs"),{createDiskWorldV1}=await import(N+"🌍️world/🟨️.mjs");
const ROOT="/home/deck/semio",SKIP=new Set(["node_modules","target",".git",".cache",".venv","__pycache__",".🧬semio",".r"]);
const status=()=>Object.fromEntries(readFileSync("/proc/self/status","utf8").split("\n").filter(l=>/^Vm(RSS|HWM):/.test(l)).map(l=>[l.split(":")[0],Math.round(parseInt(l.split(/\s+/)[1])/1024)]));
setInterval(()=>{const s=status();console.error("[DEBUG] rss",s.VmRSS,"hwm",s.VmHWM,JSON.stringify(graph.stats()));if(s.VmRSS>3500){console.error("[DEBUG] watchdog exit");process.exit(3);}},5000).unref();
const manifestsFound=[],projectDirs=[];
(function walk(rel){
 for(const e of readdirSync(join(ROOT,rel),{withFileTypes:true})){
  const path=rel===""?e.name:rel+"/"+e.name;
  if(e.isDirectory()){if(!SKIP.has(e.name)&&!e.name.startsWith("bazel-"))walk(path);}
  else if(e.name==="Cargo.toml")manifestsFound.push(path);
  else if(e.name.endsWith("project.json"))projectDirs.push(rel);
 }
})("");
const projects=projectDirs.filter(d=>d!=="").map(root=>({root,manifests:["project.json","Cargo.toml","package.json","go.mod"].map(n=>n==="project.json"?readdirSync(join(ROOT,root)).find(f=>f.endsWith("project.json")):n).filter(Boolean).filter(n=>existsSync(join(ROOT,root,n))).map(n=>root+"/"+n)}));
const statIndex=process.env.STAT_ROOT?createStatIndexV1({root:process.env.STAT_ROOT}):undefined;
var graph=createDerivedGraphV1({rules:KIND_RULES_V1,world:createDiskWorldV1(ROOT,{skip:[".🧬semio",".r"],remember:true,index:statIndex}),trace:true,concurrency:16,statIndex});
const manifestKeys=new Map();
const mkey=path=>{let k=manifestKeys.get(path);if(!k)manifestKeys.set(path,k=graph.key("manifest",path,{}));return k;};
const allManifests=new Set(manifestsFound);for(const p of projects)for(const m of p.manifests)allManifests.add(m);
const out={projects:projects.length,cargoToml:manifestsFound.length,projectJson:projectDirs.length,manifests:allManifests.size,start:status()};
let t=performance.now();
const manifestNodes=await graph.getMany([...allManifests].map(mkey));
out.manifestUnparseable=manifestNodes.filter(n=>n.value&&n.value.error).length;
const projectNodes=await graph.getMany(projects.map(p=>graph.key("project",p.root,p)));
const keys=[];
projects.forEach((p,i)=>{
 const value=projectNodes[i].value;
 const members=[{kind:"project",subject:p.root,params:p}];
 for(const target of Object.keys(value.targets))members.push({kind:"action-key",subject:p.root+":"+target,params:{root:p.root,target,project:p,env:{}}});
 for(const m of members.slice(1))keys.push(graph.key(m.kind,m.subject,m.params));
 keys.push(graph.key("closure",p.root,{members}));
});
const nodes=await graph.getMany(keys);
const cold={wallMs:Math.round(performance.now()-t),...graph.stats(),status:status()};
const globNodes=await graph.getMany(projects.flatMap((p,i)=>Object.keys(projectNodes[i].value.namedInputs).map(name=>graph.key("named-input",p.root+"#"+name,{root:p.root,name,project:p})))),uniq=new Set(globNodes.map(n=>n.value.digest+":"+n.value.files));
cold.globMemberTotal=globNodes.reduce((t,n)=>t+n.value.files,0);cold.globNodes=globNodes.length;cold.globUniqueSets=uniq.size;cold.globUniqueMembers=[...uniq].reduce((t,x)=>t+Number(x.split(":")[1]),0);
Bun.gc(true);cold.afterGc=status();
await graph.flush();if(process.env.COLD_ONLY){console.log(JSON.stringify(cold));process.exit(0);}
out.cold=cold;out.requests={manifests:allManifests.size,projects:projects.length,closuresAndActions:keys.length};
const all=[...[...allManifests].map(mkey),...projects.map(p=>graph.key("project",p.root,p)),...keys];
const stats=()=>{const s=graph.stats();return{rawReads:s.rawReads,computed:s.computed,cutoffHit:s.cutoffHit,revalidated:s.revalidated};};
const delta=(a,b)=>Object.fromEntries(Object.keys(a).map(k=>[k,b[k]-a[k]]));
const kinds=trace=>{const o={};for(const e of trace)if(e.kind==="computed"){const k=e.id.split("\0")[0];o[k]=(o[k]||0)+1;}return o;};
async function step(name,dirty){
 const before=stats();t=performance.now();
 const inv=await graph.invalidate(dirty);await graph.getMany(all);
 const wallMs=Math.round(performance.now()-t),after=stats();
 return{name,wallMs,...delta(before,after),recomputedByKind:kinds(inv.trace),changed:inv.changed.map(k=>k.kind+"|"+k.subject).slice(0,10),changedCount:inv.changed.length};
}
out.steps=[];
if(process.env.FALLBACK_ONLY){globalThis.__mark=()=>{};const one=await step("overflow 1",{paths:[],overflow:true});console.log(JSON.stringify(one.wallMs));const two=await step("overflow 2",{paths:[],overflow:true});console.log(JSON.stringify({first:one.wallMs,second:two.wallMs,computed:two.computed,rawReads:two.rawReads}));process.exit(0);}
graph.drainTrace();
out.steps.push(await step("warm no-op (journal says nothing dirty)",{paths:[]}));
const edits=[["requested crate manifest","🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🪡️field/📦️packages/🦀️rust/Cargo.toml"]];
const own=projects.find(p=>p.manifests.some(m=>m.endsWith("Cargo.toml"))&&Object.keys(projectNodes[projects.indexOf(p)].value.targets).length);
if(own)edits.push(["Cargo.toml of project "+own.root,own.manifests.find(m=>m.endsWith("Cargo.toml"))]);
for(const[label,path]of edits){
 const file=join(ROOT,path),original=readFileSync(file);
 try{
  writeFileSync(file,Buffer.concat([original,Buffer.from("\n# [DEBUG] derived-dag harness comment\n")]));
  out.steps.push(await step(label+": comment line appended",{paths:[path]}));
  writeFileSync(file,Buffer.concat([original,Buffer.from("\n[dependencies.debug-derived-dag]\nversion = \"1\"\n")]));
  out.steps.push(await step(label+": dependency added",{paths:[path]}));
 }finally{writeFileSync(file,original);}
 out.steps.push(await step(label+": restored",{paths:[path]}));
}
out.steps.push(await step("warm no-op without journal (overflow: reread every raw read)",{paths:[],overflow:true}));
out.steps.push(await step("warm no-op without journal, second pass",{paths:[],overflow:true}));
await graph.flush();out.final=status();
writeFileSync(process.argv[2],JSON.stringify(out,null,1));
console.log(JSON.stringify(out,null,1));
