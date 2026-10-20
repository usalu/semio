import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {spawnSync} from "node:child_process";
import {dirname,join} from "node:path";
import {type BigIntStats,lstatSync,lutimesSync,mkdirSync,mkdtempSync,readFileSync,readlinkSync,renameSync,rmSync,symlinkSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {withBuildTestInvocationV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import * as issued from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as leases from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import installSchema from "../../🧬️schema/📥️install.json";
import {publishJavascriptDependencyPreservingSourceDescriptorV1,readJavascriptDependencyPreservationSourceV1,requireJavascriptDependencyEndpointFenceV1,requireJavascriptDependencyPreservationSourceUnchangedV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";

type Row={sourceDirectory:string;kind:string;stamp:string|null;target:string|null};
type Observation={rows:Row[];parents:{path:string;identity:string}[];ancestors:{path:string;identity:string}[]};

const stamp=(row:BigIntStats)=>JSON.stringify([row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].map(String));
const anchor=(row:BigIntStats)=>JSON.stringify([row.dev,row.ino,row.mode].map(String));

const layout=(root:string)=>{
 const paths={root,g:join(root,"g"),p0:join(root,"g","p0"),p1:join(root,"g","p1"),p2:join(root,"g","p2"),store:join(root,"store")};
 for(const path of[paths.g,paths.p0,paths.p1,paths.p2,paths.store,join(paths.store,"t0"),join(paths.store,"t1"),join(paths.store,"t2"),join(paths.store,"other")])mkdirSync(path);
 const endpoints=[0,1,2].map(index=>join(root,"g","p"+index,"node_modules"));for(const[index,endpoint]of endpoints.entries())symlinkSync(join(paths.store,"t"+index),endpoint);
 return{...paths,e0:endpoints[0]!,e1:endpoints[1]!,e2:endpoints[2]!,endpoints};
};

const observe=(root:string,endpoints:readonly string[]):Observation=>{
 const rows=endpoints.map(sourceDirectory=>{const facts=lstatSync(sourceDirectory,{bigint:true,throwIfNoEntry:false});return{sourceDirectory,kind:facts?facts.isSymbolicLink()?"link":facts.isDirectory()?"directory":"file":"missing",stamp:facts?stamp(facts):null,target:facts?.isSymbolicLink()?readlinkSync(sourceDirectory):null};}),parents=new Map<string,string>(),ancestors=new Map<string,string>();
 for(const endpoint of endpoints){const parent=dirname(endpoint),chain:string[]=[];for(let path=parent;path.length>=root.length;path=dirname(path)){chain.push(path);if(path===root)break;}for(const path of chain.reverse()){const facts=lstatSync(path,{bigint:true,throwIfNoEntry:false});if(!ancestors.has(path))ancestors.set(path,facts?anchor(facts):"missing");}const facts=lstatSync(parent,{bigint:true,throwIfNoEntry:false});parents.set(parent,facts?stamp(facts):"missing");}
 return{rows,parents:[...parents].map(([path,identity])=>({path,identity})),ancestors:[...ancestors].map(([path,identity])=>({path,identity}))};
};

const maps=(observation:Observation)=>({expected:new Map(observation.rows.map(row=>[row.sourceDirectory,row])),parents:new Map(observation.parents.map(row=>[row.path,row.identity])),ancestors:new Map(observation.ancestors.map(row=>[row.path,row.identity]))});

const accepts=(body:()=>void)=>{try{body();return true;}catch{return false;}};

/** 🚧️ Proves the per-binding endpoint fence refuses every change of the endpoint just published and leaves each other row to the full comparison that follows. */
test("dependency endpoint fence is local while the full comparison stays complete",()=>{
 const validate=new Ajv({strict:true}).compile(corpus.schema),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency fence artifact root required");
 for(const row of corpus.cases){
  expect(validate(row),row.id).toBe(true);const root=mkdtempSync(join(artifacts,"ef-"));
  try{
   const fixture=layout(root),before=maps(observe(root,fixture.endpoints)),future=new Date(Date.now()+5000),target=row.target==="none"?"":(fixture as unknown as Record<string,string>)[row.target]!;
   switch(row.effect){
    case"retarget":unlinkSync(target);symlinkSync(join(fixture.store,"other"),target);break;
    case"remove":unlinkSync(target);break;
    case"replace-with-directory":unlinkSync(target);mkdirSync(target);break;
    case"touch-link":lutimesSync(target,future,future);break;
    case"replace-directory":{const endpoint=join(target,"node_modules"),destination=readlinkSync(endpoint);renameSync(target,target+".moved");mkdirSync(target);symlinkSync(destination,endpoint);break;}
    case"add-sibling":writeFileSync(join(target,"sibling"),"x");break;
    case"swap-for-link":renameSync(target,target+".moved");symlinkSync(target+".moved",target);break;
    case"touch-directory":utimesSync(target,future,future);break;
    case"none":break;
   }
   const after=observe(root,fixture.endpoints);
   expect(accepts(()=>requireJavascriptDependencyEndpointFenceV1(fixture.e0,before.expected,before.parents,before.ancestors)),row.id+" endpoint fence").toBe(row.endpoint);
   expect(accepts(()=>requireJavascriptDependencyPreservationSourceUnchangedV1(after,before.expected,before.parents,before.ancestors)),row.id+" full comparison").toBe(row.full);
   expect(row.full||!row.endpoint||row.target!=="e0",row.id).toBe(true);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

/** 🐍 Reproduces every stamp, anchor and link target of the observer with an independent Python lstat walk. */
test("dependency endpoint fence stamps agree with an independent Python lstat",()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency fence artifact root required");const root=mkdtempSync(join(artifacts,"ep-"));
 try{
  const fixture=layout(root),observation=observe(root,fixture.endpoints),script="import os,sys,json\nout=[]\nfor p in json.loads(sys.argv[1]):\n s=os.lstat(p)\n out.append([json.dumps([str(getattr(s,f)) for f in ['st_dev','st_ino','st_mode','st_nlink','st_size','st_mtime_ns','st_ctime_ns']],separators=(',',':')),os.readlink(p) if os.path.islink(p) else None])\nprint(json.dumps(out))",child=spawnSync("python3",["-c",script,JSON.stringify(fixture.endpoints)],{encoding:"utf8"});
  expect(child.status).toBe(0);const oracle=JSON.parse(child.stdout)as [string,string|null][];
  expect(observation.rows.map(row=>[row.stamp,row.target])).toEqual(oracle);
  const before=maps(observation);for(const endpoint of fixture.endpoints)requireJavascriptDependencyEndpointFenceV1(endpoint,before.expected,before.parents,before.ancestors);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🧮 Checks the pure full comparison against SQLite JSON equality and a first-three-fields identity oracle. */
test("dependency preservation source comparison agrees with SQLite JSON oracles",()=>{
 const validate=new Ajv({strict:true}).compile(corpus.compareSchema),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency fence artifact root required");const root=mkdtempSync(join(artifacts,"ec-")),database=new Database(":memory:");
 try{
  const fixture=layout(root),baseline=observe(root,fixture.endpoints),before=maps(baseline);
  for(const row of corpus.compare){
   expect(validate(row),row.id).toBe(true);const current:Observation=structuredClone(baseline);
   if(row.field==="rows"){if(row.edit==="reverse")current.rows.reverse();else current.rows.pop();}
   if(row.field==="parents"){if(row.edit==="add")current.parents.push({path:join(root,"extra"),identity:current.parents[0]!.identity});else{const identity=JSON.parse(current.parents[0]!.identity)as string[];identity[row.edit==="inode"?1:5]="9999999999999999999";current.parents[0]={path:current.parents[0]!.path,identity:JSON.stringify(identity)};}}
   if(row.field==="ancestors"){if(row.edit==="drop-last")current.ancestors.pop();else{const identity=JSON.parse(current.ancestors[0]!.identity)as string[];identity[2]="1";current.ancestors[0]={path:current.ancestors[0]!.path,identity:JSON.stringify(identity)};}}
   const actual=accepts(()=>requireJavascriptDependencyPreservationSourceUnchangedV1(current,before.expected,before.parents,before.ancestors));
   const oracle=row.field==="parents"&&row.edit!=="add"?Boolean((database.query("select json_extract(?1,'$[0]')=json_extract(?2,'$[0]') and json_extract(?1,'$[1]')=json_extract(?2,'$[1]') and json_extract(?1,'$[2]')=json_extract(?2,'$[2]') ok").get(current.parents[0]!.identity,baseline.parents[0]!.identity)as{ok:number}).ok):Boolean((database.query("select json(?1)=json(?2) ok").get(JSON.stringify(row.field==="rows"?current.rows:row.field==="parents"?current.parents:current.ancestors),JSON.stringify(row.field==="rows"?baseline.rows:row.field==="parents"?baseline.parents:baseline.ancestors))as{ok:number}).ok);
   expect(actual,row.id).toBe(row.accepted);expect(oracle,row.id).toBe(row.accepted);
  }
 }finally{database.close();rmSync(root,{recursive:true,force:true});}
});

/** 🔒️ Keeps the complete fence at every interval boundary and after the last binding, and removes the partial fence mode. */
test("dependency producer keeps the full fence at every interval and after the final verification",async()=>{
 const ts=await import("typescript"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),file=ts.createSourceFile("storage.ts",source,ts.ScriptTarget.Latest,true),producer=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="runJavascriptDependencyStoreProducerV1")!.getText(file);
 const interval=installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval;expect(Number.isSafeInteger(interval)&&interval>=1&&interval<=64).toBe(true);
 expect(producer).toContain("if(++published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else{checkpoint();requireJavascriptDependencyEndpointFenceV1(binding.sourceDirectory,expected,parents,ancestors);checkpoint();}");
 expect(producer).not.toContain("fence=async(full");expect(producer).not.toContain("fence(false");
 const loop=producer.indexOf("requireJavascriptDependencyEndpointFenceV1("),tail=producer.slice(loop);
 expect(tail.indexOf("await verifyReady();")).toBeGreaterThan(0);expect(tail.indexOf("await verifyReady();")).toBeLessThan(tail.indexOf("for(const scope of requested)"));expect(tail.slice(tail.indexOf("for(const scope of requested)"))).toContain("await fence();");
 const verify=producer.slice(producer.indexOf("verifyReady=async()=>{"));expect(verify.slice(0,verify.indexOf("};"))).toContain("await fence();");
 const definition=producer.slice(producer.indexOf("fence=async()=>{"),producer.indexOf("material=("));
 for(const needed of["await store.recheck()","await custody.recheck()","recheckJavascriptDependencyInputsV1(row.plan.sources,control)","requireJavascriptDependencyPreservationSourceUnchangedV1(await readJavascriptDependencyPreservationSourceV1(","verifyJavascriptDependencyDescriptorGroupV1(","recheckJavascriptDependencyPreservationsDescriptorV1(held"])expect(definition).toContain(needed);
});

/** 🛤️ Narrows publication ancestry to the lineage of the published endpoint and leaves every other ancestor to the full comparison that follows. */
test("dependency publication checks the endpoint lineage and the full comparison catches every other ancestor",async()=>{
 const validate=new Ajv({strict:true}).compile(corpus.publishSchema),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency fence artifact root required");
 await withBuildTestInvocationV1(async original=>{
  const control=issued.createIssuedResourceControlV1(original,"dependencies:publication-lineage",0),failures:string[]=[];
  for(const row of corpus.publish){
   expect(validate(row),row.id).toBe(true);const root=mkdtempSync(join(artifacts,"pl-")),workspace=join(root,"s"),cache=join(root,"n"),first=join(workspace,"a","node_modules"),second=join(workspace,"a","m","node_modules"),targets=[join(cache,"t1"),join(cache,"t2"),join(cache,"t3")];
   try{
    for(const path of[join(workspace,"a","m"),...targets])mkdirSync(path,{recursive:true});symlinkSync(targets[0]!,first,"dir");symlinkSync(targets[1]!,second,"dir");
    const check=()=>{issued.requireIssuedResourceControlInvocationV1(control,original);issued.requireIssuedResourceControlRefinementV1(control,control);leases.checkResourceLeaseControl(control);},context=Object.freeze({root:cache,invocation:original,control,check,recheck:async()=>{check();}}),ownership={version:1,scopes:[{manifest:"package.json",packages:["a","a/m"]}]};
    const source=await readJavascriptDependencyPreservationSourceV1(workspace,context,control,ownership),previous=source.rows.find(entry=>entry.sourceDirectory===first)!,facts={sourceSha256:source.sha256,generation:"a".repeat(64),readySha256:"b".repeat(64),inputSha256:"c".repeat(64),recipeSha256:"d".repeat(64),fundingSha256:"e".repeat(64)};
    let calls=0,fired=false;const mutate=()=>{switch(row.mutation){
      case"swap-lineage":renameSync(workspace,workspace+".moved");symlinkSync(workspace+".moved",workspace,"dir");break;
      case"swap-unrelated":{const folder=join(workspace,"a","m");renameSync(folder,folder+".moved");mkdirSync(folder);symlinkSync(targets[1]!,second,"dir");break;}
      case"retarget-endpoint":unlinkSync(first);symlinkSync(targets[2]!,first,"dir");break;
      case"none":break;}};
    const checkpoint=()=>{check();if(!fired&&row.mutation!=="none"&&++calls>=row.at){fired=true;mutate();}};
    let published=true;try{await publishJavascriptDependencyPreservingSourceDescriptorV1({sourceDirectory:first,targetDirectory:targets[0]!},previous,source,facts,checkpoint);}catch{published=false;}
    if(row.mutation!=="none"&&!fired&&row.publish)failures.push(row.id+": mutation never fired");
    if(row.mutation!=="none"&&!fired&&!row.publish)failures.push(row.id+": mutation never fired");
    if(published!==row.publish)failures.push(row.id+": publication expected "+(row.publish?"accepted":"refused"));
    const expected=new Map(source.rows.map(entry=>[entry.sourceDirectory,entry])),parents=new Map(source.parents.map(entry=>[entry.path,entry.identity])),ancestors=new Map(source.ancestors.map(entry=>[entry.path,entry.identity]));
    let full=true;try{requireJavascriptDependencyPreservationSourceUnchangedV1(await readJavascriptDependencyPreservationSourceV1(workspace,context,control,ownership),expected,parents,ancestors);}catch{full=false;}
    if(full!==row.full)failures.push(row.id+": full comparison expected "+(row.full?"accepted":"refused"));
   }finally{rmSync(root,{recursive:true,force:true});}
  }
  expect(failures).toEqual([]);
 });
},120000);
