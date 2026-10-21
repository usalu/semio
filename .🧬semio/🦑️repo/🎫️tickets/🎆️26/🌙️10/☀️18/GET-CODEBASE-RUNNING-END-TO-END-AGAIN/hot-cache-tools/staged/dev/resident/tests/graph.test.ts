import {afterAll,describe,expect,test} from "bun:test";
import {appendFileSync,existsSync,mkdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {POLICY_V1} from "../protocol/policy.mjs";
import {createFilterV1,createWorkspaceStateV1} from "../state/index.mjs";
import {createResidentHoldsV1,dirtyPathsFromJournalV1,membershipFromJournalV1} from "../graph-hooks/index.mjs";
import {stateConfigurationFromBuildGraphV1} from "../ports/graph.mjs";
import {residentGraphSeamsV1} from "../graph-hooks/host/index.mjs";
import {cleanup,pick,rng,snapshot,tempRoot,writeTree} from "./common.ts";

const GENERATIONS=process.env.SEMIO_GENERATIONS_MODULE??"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const roots:string[]=[];afterAll(()=>roots.forEach(cleanup));

describe("neutral build graph to owners and tasks",()=>{
 const graph={version:1,projects:{
  app:{name:"app",root:"apps/app",targets:{build:{execution:{kind:"command",command:"build app"},dependsOn:["^build","codegen"],outputs:["{projectRoot}/dist","{workspaceRoot}/.out/app","{projectRoot}/gen/**","../escape","/abs"]},codegen:{execution:{kind:"command",command:"gen"}},test:{execution:{kind:"command",command:"t"},dependsOn:["build",{target:"build",dependencies:true},{target:"lint",projects:["lib"]}],cache:false},serve:{execution:{kind:"command",command:"s"},continuous:true}}},
  lib:{name:"lib",root:"libs/lib",targets:{build:{execution:{kind:"command",command:"build lib"},dependsOn:["^build","missing"]},lint:{execution:{kind:"command",command:"lint"}}}},
  base:{name:"base",root:".",targets:{build:{execution:{kind:"command",command:"b"}}}},
 },dependencies:[{source:"app",target:"lib"},{source:"lib",target:"base"},{source:"app",target:"app"},{source:"app",target:"ghost"}]} as any;
 const config=stateConfigurationFromBuildGraphV1(graph,{environmentPaths:["bun.lock"]});
 const owner=(name:string)=>config.owners.find(o=>o.name===name)!;
 test("owners, direct dependencies and the workspace root project",()=>{
  expect(config.owners.map(o=>o.name)).toEqual(["app","base","lib"]);
  expect(owner("app").deps).toEqual(["lib"]);expect(owner("lib").deps).toEqual(["base"]);expect(owner("base").root).toBe("");
  expect(config.env).toEqual({paths:["bun.lock"],prefixes:[]});
 });
 test("dependsOn forms: ^target, own target, object forms, missing targets and self edges",()=>{
  const task=(o:string,t:string)=>owner(o).tasks.find((x:any)=>x.name===t)!;
  expect(task("app","build").dependsOn).toEqual(["app:codegen","lib:build"]);
  expect(task("app","test").dependsOn).toEqual(["app:build","lib:build","lib:lint"]);
  expect(task("lib","build").dependsOn).toEqual(["base:build"]);
 });
 test("outputs are workspace-relative, concrete and inside the workspace; cacheability follows cache and continuous",()=>{
  const task=(o:string,t:string)=>owner(o).tasks.find((x:any)=>x.name===t)!;
  expect(task("app","build").outputs).toEqual([".out/app","apps/app/dist"]);
  expect(task("app","test").cacheable).toBe(false);expect(task("app","serve").cacheable).toBe(false);expect(task("app","build").cacheable).toBe(true);
  expect(task("app","build").command).not.toEqual(task("lib","build").command);
 });
});

describe("membership from the journal",()=>{
 const filter=createFilterV1({names:["node_modules"]});
 const excluded=(rel:string)=>filter.excluded(rel)||rel===".semio-fence"||rel.startsWith(".semio-fence/");
 const frontierOf=(root:string,capturedAt=1)=>{const s=snapshot(root,excluded);return{version:1,capturedAt,directories:[...s.directories].map(([p,[m,i]])=>[p,m,i]).sort((a:any,b:any)=>a[0]<b[0]?-1:1),members:[...s.files].sort((a,b)=>a[0]<b[0]?-1:1)};};
 for(const seed of[11,12,13])test(`seed ${seed}: the patched frontier equals a fresh capture and the changes equal diffFrontiers`,async()=>{
  const root=tempRoot("graph-membership-");roots.push(root);
  writeTree(root,{"a/p/x.ts":"1","a/q/y.ts":"2","b/z.ts":"3","top.txt":"t","a/📋️project.json":"{}"});
  const state=createWorkspaceStateV1({root,instance:"i",policy:POLICY_V1,filter,fence:".semio-fence"});state.configure({owners:[{name:"all",root:"",deps:[],tasks:[]}],env:{paths:[],prefixes:[]}});
  const listing=()=>{const f=frontierOf(root);return{directories:f.directories,members:f.members};};state.load(listing());
  const previousFrontier=frontierOf(root),previous={manifest:{membership:"prev-digest"},frontier:previousFrontier},cursor=state.cursor(),random=rng(seed);
  let n=0;
  for(let step=0;step<40;step++){
   const rows:[string,number,string][]=[];
   const files=[...snapshot(root,excluded).files.keys()];
   const op=pick(random,["add","del","mod","mv","mkdir","rmdir"] as const);
   if(op==="add"){const p=pick(random,["a/p","a/q","b","."])+"/n"+(n++)+".ts";writeFileSync(root+"/"+p.replace("./",""),"n");rows.push(["a",0,p.replace("./","")]);}
   else if(op==="del"&&files.length){const f=pick(random,files);rmSync(root+"/"+f);rows.push(["r",0,f]);}
   else if(op==="mod"&&files.length){const f=pick(random,files);appendFileSync(root+"/"+f,"m");rows.push(["m",0,f]);}
   else if(op==="mv"&&files.length){const f=pick(random,files),t="b/mv"+(n++)+".ts";renameSync(root+"/"+f,root+"/"+t);rows.push(["n",0,f],["n",0,t]);}
   else if(op==="mkdir"){const p="a/g"+(n++);mkdirSync(root+"/"+p+"/s",{recursive:true});writeFileSync(root+"/"+p+"/s/f.ts","f");rows.push(["a",1,p]);}
   else if(op==="rmdir"){const dirs=[...snapshot(root,excluded).directories.keys()].filter(d=>/^a\/g\d+$/u.test(d));if(dirs.length){const d=pick(random,dirs);rmSync(root+"/"+d,{recursive:true});rows.push(["r",1,d]);}}
   await state.apply(rows,Date.now());
  }
  const reply:any=state.since(cursor);
  const result=membershipFromJournalV1(previous as any,{...reply,cursor:state.cursor()},{now:2})!;
  expect(result).toBeDefined();
  const fresh=frontierOf(root,2);
  expect(result.frontier.members).toEqual(fresh.members);
  expect(result.frontier.directories).toEqual(fresh.directories);
  const expected=new Set<string>();
  const was=new Map(previousFrontier.members as [string,number][]),now=new Map(fresh.members as [string,number][]);
  for(const[p,m]of now){if(!was.has(p))expected.add("added "+p);else if(was.get(p)!==m)expected.add("changed "+p);}
  for(const p of was.keys())if(!now.has(p))expected.add("removed "+p);
  expect(new Set(result.changes.filter(c=>!c.directory).map(c=>c.kind+" "+c.path))).toEqual(expected);
  if(existsSync(GENERATIONS)){
   const generations:any=await import(GENERATIONS);
   const reference=generations.diffFrontiersV1(previousFrontier,fresh).map((c:any)=>c.kind+(c.directory?" dir ":" file ")+c.path).sort();
   expect(result.changes.map(c=>c.kind+(c.directory?" dir ":" file ")+c.path).sort()).toEqual(reference);
   const statEntry=async(path:string)=>{try{const s=require("node:fs").lstatSync(root+"/"+path);return{directory:s.isDirectory(),file:s.isFile(),mtimeMs:s.mtimeMs,ino:s.ino,mode:s.mode};}catch{return undefined;}};
   expect(await generations.frontierHoldsV1({...result.frontier,capturedAt:Date.now()+1e7},statEntry,{racyMilliseconds:0})).toBe(true);
  }
  expect(result.digest===previous.manifest.membership).toBe(expected.size===0);
 },30000);

 test("an incomplete journal reply yields no membership; no changes keep the previous frontier object and digest",()=>{
  const previous={manifest:{membership:"d"},frontier:{version:1,capturedAt:1,directories:[],members:[]}} as any;
  expect(membershipFromJournalV1(previous,{complete:false,changes:[]})).toBeUndefined();
  const same=membershipFromJournalV1(previous,{complete:true,changes:[]})!;
  expect(same.frontier).toBe(previous.frontier);expect(same.digest).toBe("d");expect(same.changes).toEqual([]);
 });

 test("config files are flagged and directory-only changes keep the members digest",()=>{
  const previous={manifest:{membership:"d"},frontier:{version:1,capturedAt:1,directories:[["",1,1]],members:[["a/Cargo.toml",33188]]}} as any;
  const r=membershipFromJournalV1(previous,{complete:true,changes:[{path:"a/Cargo.toml",kind:"changed",directory:false,mode:33261},{path:"",kind:"changed",directory:true,stamp:[2,1]}]},{now:5})!;
  expect(r.changes).toEqual([{path:"",kind:"changed",directory:true},{path:"a/Cargo.toml",kind:"changed",directory:false,config:true}]);
  const dirOnly=membershipFromJournalV1(previous,{complete:true,changes:[{path:"",kind:"changed",directory:true,stamp:[3,1]}]})!;
  expect(dirOnly.digest).toBe("d");
 });
});

describe("fact checks from the journal",()=>{
 test("rows whose path or directory moved go to the fallback, the rest are trusted without it",async()=>{
  const dirty=dirtyPathsFromJournalV1({complete:true,changes:[{path:"a/x.ts",kind:"changed",directory:false},{path:"b/new",kind:"added",directory:true},{path:"c/gone.ts",kind:"removed",directory:false}]})!;
  expect(dirty.has("a/x.ts")).toBe(true);expect(dirty.has("a")).toBe(true);expect(dirty.has("b/new/deep/file")).toBe(true);expect(dirty.has("c")).toBe(true);expect(dirty.has("d/other.ts")).toBe(false);expect(dirty.has("a/y.ts")).toBe(false);
  const asked:string[][]=[];
  const holds=createResidentHoldsV1({dirty:async()=>dirty,fallback:async rows=>{asked.push(rows.map(r=>r.path));return true;}});
  expect(await holds([{path:"d/other.ts"},{path:"a/y.ts"}])).toBe(true);expect(asked).toEqual([]);
  expect(await holds([{path:"d/other.ts"},{path:"a/x.ts"}])).toBe(true);expect(asked).toEqual([["a/x.ts"]]);
  const unknown=createResidentHoldsV1({dirty:async()=>undefined,fallback:async rows=>{asked.push(rows.map(r=>r.path));return false;}});
  expect(await unknown([{path:"q"}])).toBe(false);expect(asked.at(-1)).toEqual(["q"]);
  expect(dirtyPathsFromJournalV1({complete:false,changes:[]})).toBeUndefined();
 });
});

describe("host seams",()=>{
 const frontier={version:1,capturedAt:1,directories:[["",1,1]],members:[["a/x.ts",33188]]};
 const previous={manifest:{membership:"D"},frontier} as any;
 const counting=()=>{const calls={membership:0,holds:[] as string[][],adopted:[] as string[][],asked:[] as string[]};return calls;};
 test("first call falls back after asking the cursor; the next call is answered by the journal without the stat pass",async()=>{
  const calls=counting(),replies:any[]=[{instance:"i",seq:4},{complete:true,changes:[],cursor:{instance:"i",seq:9}},{complete:true,changes:[],cursor:{instance:"i",seq:9}}];
  const seams=residentGraphSeamsV1({root:"/w",membership:async()=>{calls.membership++;return{digest:"D",frontier,changes:[]};},holds:async()=>true,adopt:files=>calls.adopted.push(files),ask:async(command,args)=>{calls.asked.push(command+" "+args.join(" "));return command==="adopt-frontier"?undefined:replies.shift();}});
  const first=await seams.membership(undefined);
  expect(first.digest).toBe("D");expect(calls.membership).toBe(1);expect(calls.asked).toEqual(["cursor ","adopt-frontier D i:4"]);
  const second=await seams.membership(previous);
  expect(calls.membership).toBe(1);expect(second.frontier).toBe(frontier);expect(calls.adopted).toEqual([["a/x.ts"]]);expect(calls.asked.slice(2)).toEqual(["dirty --digest D","adopt-frontier D i:9"]);
 });
 test("an incomplete answer, a missing service and mode off all fall back to the stat code",async()=>{
  let stat=0;const membership=async()=>{stat++;return{digest:"E",frontier,changes:[]};};
  const incomplete=residentGraphSeamsV1({root:"/w",membership,holds:async()=>true,ask:async()=>({complete:false,reason:"digest"})});
  await incomplete.membership(previous);expect(stat).toBe(1);
  const down=residentGraphSeamsV1({root:"/w",membership,holds:async()=>true,ask:async()=>undefined});
  await down.membership(previous);expect(stat).toBe(2);
  const off=residentGraphSeamsV1({root:"/w",membership,holds:async()=>true,environment:{SEMIO_BUILD_SERVICE:"off"}});
  await off.membership(previous);expect(stat).toBe(3);
 });
 test("holds trusts rows the journal proves untouched, asks the old checker only about the rest, and coalesces parallel chunks into one question",async()=>{
  const asked:string[]=[],checked:string[][]=[];
  const seams=residentGraphSeamsV1({root:"/w",membership:async()=>({digest:"D",frontier,changes:[]}),holds:async rows=>{checked.push(rows.map(r=>r.path));return true;},ask:async command=>{asked.push(command);return command==="dirty"?{complete:true,changes:[{path:"a/x.ts",kind:"changed",directory:false}]}:undefined;}});
  await seams.membership(undefined);
  const results=await Promise.all([seams.holds([{path:"b/y.ts"},{path:"a/x.ts"}]),seams.holds([{path:"c/z.ts"}]),seams.holds([{path:"d/w.ts"}])]);
  expect(results).toEqual([true,true,true]);expect(checked).toEqual([["a/x.ts"]]);expect(asked.filter(c=>c==="dirty").length).toBe(1);
  const before=residentGraphSeamsV1({root:"/w",membership:async()=>({}),holds:async rows=>{checked.push(rows.map(r=>r.path));return false;},ask:async()=>undefined});
  expect(await before.holds([{path:"q"}])).toBe(false);
  const unreachable=residentGraphSeamsV1({root:"/w",membership:async()=>({digest:"D",frontier,changes:[]}),holds:async()=>false,ask:async()=>undefined});
  await unreachable.membership(undefined);
  expect(await unreachable.holds([{path:"q"}])).toBe(false);
 });
});
