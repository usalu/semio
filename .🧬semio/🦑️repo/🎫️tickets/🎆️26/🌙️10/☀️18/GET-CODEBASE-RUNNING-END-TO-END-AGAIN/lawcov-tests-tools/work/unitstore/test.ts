/** 📦 Contract: the content-addressed unit cache of Rust compilations behaves as the authored scenarios in `./🔣️.json` say (a compilation published in one lane is restored in another with its lane and output paths rewritten, a changed dependency, environment value or removed object is a miss, only the newest variants of a key are kept, pruning removes broken entries and stale temporaries) while an independent Python auditor re-derives the store invariants from the bytes on disk (every object is named by its digest, no entry points at a missing object, nothing lane specific is stored in a template or a stream), every entry and event validates against the closed schema under Ajv, directory trees digest exactly as the Python walk does, and a stamp memoises a file digest by size, time and inode. */
import {afterAll,beforeAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "@LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import cacheSchema from "@LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json";
import {digestFileV1,emitUnitEventV1,listObjectsV1,lookupBuildRunV1,lookupUnitV1,publishBuildRunV1,pruneEntriesV1,publishUnitV1,removeObjectsV1,restoreBuildRunV1,restoreUnitV1,treeDigestV1,type BuildEntryV1,type UnitEntryV1} from "@UNIT/🟦️.ts";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const policy=UNIT_CACHE_POLICY_V1,layout=policy.layout;
const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(requests:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({requests}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const ajv=new Ajv({strict:true});ajv.addSchema(cacheSchema);
const validator=(name:string)=>{const validate=ajv.getSchema(`${cacheSchema.$id}#/definitions/${name}`);if(!validate)throw Error("Missing schema definition "+name);return validate;};
const sha=(data:string|Uint8Array)=>createHash("sha256").update(data).digest("hex");
let base="";
beforeAll(()=>{base=realpathSync(mkdtempSync(join(tmpdir(),"unit-store-")));});
afterAll(()=>{rmSync(base,{recursive:true,force:true});});

test("the corpus is closed by its schema",()=>{
 const ajv2=new Ajv({strict:false});
 expect(ajv2.validate(corpusSchema,corpus),JSON.stringify(ajv2.errors)).toBe(true);
 expect(ajv2.validate(corpusSchema,{...corpus,scenarios:[{...corpus.scenarios[0],steps:[{op:"explode"}]}]})).toBe(false);
 expect(ajv2.validate(corpusSchema,{...corpus,lanes:["laneA","laneC"]})).toBe(false);
});

for(const[index,scenario]of [...corpus.scenarios,...corpus.runs].entries()){
 test((index<corpus.scenarios.length?"scenario ":"build script run scenario ")+(index+1)+": "+scenario.name,()=>{
  const root=join(base,"s"+index),cache=join(root,"cache"),roots:Record<string,string>={laneA:join(root,"laneA"),laneB:join(root,"laneB")};
  for(const lane of Object.values(roots))for(const[path,text]of Object.entries(scenario.tree)){mkdirSync(dirname(join(lane,path)),{recursive:true});writeFileSync(join(lane,path),text);}
  mkdirSync(cache,{recursive:true});
  const fill=(text:string,lane="A")=>text.replaceAll("{laneA}",roots.laneA!).replaceAll("{laneB}",roots.laneB!).replaceAll("{outA}",join(roots.laneA!,"out")).replaceAll("{outB}",join(roots.laneB!,"out")).replaceAll("{lane}",roots["lane"+lane]!).replaceAll("{out}",join(roots["lane"+lane]!,"out"));
  let last:UnitEntryV1|undefined,lastRun:BuildEntryV1|undefined;
  const objectOf=(name:string)=>last!.outputs.find(row=>row.name===name)!.sha256;
  for(const[position,raw]of scenario.steps.entries()){
   const step=raw as any,label=`${scenario.name} #${position} ${step.op}`;
   const lane=step.lane??"A",laneRoot=roots["lane"+lane]!;
   if(step.op==="sleep"){Bun.sleepSync(step.milliseconds);}
   else if(step.op==="mutate"){mkdirSync(dirname(join(laneRoot,step.file)),{recursive:true});writeFileSync(join(laneRoot,step.file),step.content);}
   else if(step.op==="remove"){rmSync(join(laneRoot,step.file));}
   else if(step.op==="publish"){
    const outDir=join(laneRoot,"out");rmSync(outDir,{recursive:true,force:true});mkdirSync(outDir,{recursive:true});
    for(const[name,text]of Object.entries(step.outputs as Record<string,string>)){if((step.skipWrite??[]).includes(name))continue;writeFileSync(join(outDir,name),fill(text,lane));if(step.executable.includes(name))chmodSync(join(outDir,name),0o755);}
    const entry=publishUnitV1(cache,{key:step.key,crate:step.crate,cwd:join(laneRoot,"pkg"),outDir,laneRoot,outputs:Object.keys(step.outputs).map(name=>join(outDir,name)),stdout:fill(step.stdout,lane),stderr:fill(step.stderr,lane),stampDirectory:undefined});
    expect(entry!==undefined,label).toBe(step.expect?.published??true);
    if(entry){
     last=entry;expect(validator("Entry")(entry),label+" "+JSON.stringify(validator("Entry").errors)).toBe(true);
     const expected=step.expect;if(!expected)continue;
     expect(entry.crate,label).toBe(expected.crate);
     expect(entry.outputs.map(row=>({name:row.name,template:row.template===true,executable:row.executable})),label).toEqual(expected.outputs);
     expect(entry.dependencies.map(row=>row.path),label).toEqual(expected.dependencies);
     for(const row of entry.dependencies)expect(row.sha256,label).toBe(sha(readFileSync(fill(row.path.replace("@@semio-lane@@","{lane}"),lane))));
     expect(entry.environment,label).toEqual(expected.environment);
     expect(entry.stdout!==null,label).toBe(expected.stdout);expect(entry.stderr!==null,label).toBe(expected.stderr);
    }
   }
   else if(step.op==="lookup"||step.op==="restore"){
    if(step.file!==undefined){mkdirSync(dirname(join(laneRoot,step.file)),{recursive:true});writeFileSync(join(laneRoot,step.file),step.content);}
    const environment=Object.fromEntries(Object.entries(step.env as Record<string,string>).map(([name,value])=>[name,fill(value,lane)]));
    const found=lookupUnitV1(cache,step.key,{cwd:join(laneRoot,"pkg"),laneRoot,environment,stampDirectory:undefined});
    if(step.op==="lookup"){expect(found!==undefined?"hit":"miss",label).toBe(step.expect);if(found)expect(found.key).toBe(step.key);}
    else{
     expect(found,label).toBeDefined();
     const outDir=join(laneRoot,"out"),streams=restoreUnitV1(cache,found!,{outDir,laneRoot,stampDirectory:undefined});
     for(const[name,wanted]of Object.entries(step.expect.files as Record<string,{text:string;executable:boolean}>)){
      expect(readFileSync(join(outDir,name),"utf8"),label+" "+name).toBe(fill(wanted.text,lane));
      expect((statSync(join(outDir,name)).mode&0o111)!==0,label+" "+name+" executable").toBe(process.platform==="win32"?(statSync(join(outDir,name)).mode&0o111)!==0:wanted.executable);
     }
     expect(streams.stdout,label).toBe(fill(step.expect.stdout,lane));expect(streams.stderr,label).toBe(fill(step.expect.stderr,lane));
    }
   }
   else if(step.op==="runPublish"){
    if(step.onlyPosix&&process.platform==="win32")continue;
    const outDir=join(laneRoot,"out-dir");rmSync(outDir,{recursive:true,force:true});mkdirSync(outDir,{recursive:true});
    for(const[name,spec]of Object.entries(step.out as Record<string,any>)){
     const path=join(outDir,name);mkdirSync(dirname(path),{recursive:true});
     if(spec.symlink!==undefined)symlinkSync(spec.symlink,path);
     else{writeFileSync(path,spec.text!==undefined?fill(spec.text,lane):Buffer.concat([Buffer.from(spec.hex,"hex"),Buffer.from(spec.embeds?fill(spec.embeds,lane):"")]));if(spec.executable)chmodSync(path,0o755);}
    }
    const entry=publishBuildRunV1(cache,{key:step.key,package:"x",cwd:join(laneRoot,"pkg"),laneRoot,directives:step.directives,environment:step.env,outDir,stdout:fill(step.stdout,lane),stderr:fill(step.stderr,lane),stampDirectory:undefined});
    expect(entry!==undefined,label).toBe(step.expect.published);
    if(entry){
     lastRun=entry;expect(validator("BuildEntry")(entry),label+" "+JSON.stringify(validator("BuildEntry").errors)).toBe(true);
     expect(entry.facts.map(row=>({path:row.path,kind:row.kind})),label).toEqual(step.expect.facts);
     expect(entry.environment,label).toEqual(step.expect.environment);
     expect(entry.outputs.map(row=>({path:row.path,template:row.template,executable:row.executable})),label).toEqual(step.expect.outputs);
    }
   }
   else if(step.op==="runLookup"||step.op==="runRestore"){
    if(step.file!==undefined){mkdirSync(dirname(join(laneRoot,step.file)),{recursive:true});writeFileSync(join(laneRoot,step.file),step.content);}
    const found=lookupBuildRunV1(cache,step.key,{cwd:join(laneRoot,"pkg"),laneRoot,environment:step.env,stampDirectory:undefined});
    if(step.op==="runLookup")expect(found!==undefined?"hit":"miss",label).toBe(step.expect);
    else{
     expect(found,label).toBeDefined();
     const outDir=join(laneRoot,"out-dir"),streams=restoreBuildRunV1(cache,found!,{outDir,laneRoot});
     for(const[name,wanted]of Object.entries(step.expect.files as Record<string,any>)){
      const bytes=readFileSync(join(outDir,name));
      if(wanted.text!==undefined)expect(bytes.toString("utf8"),label+" "+name).toBe(fill(wanted.text,lane));else expect(bytes.toString("hex"),label+" "+name).toBe(wanted.hex);
      if(process.platform!=="win32")expect((statSync(join(outDir,name)).mode&0o111)!==0,label+" "+name+" executable").toBe(wanted.executable===true);
     }
     expect(streams.stdout,label).toBe(fill(step.expect.stdout,lane));expect(streams.stderr,label).toBe(fill(step.expect.stderr,lane));
    }
   }
   else if(step.op==="removeObject"){expect(removeObjectsV1(cache,[objectOf(step.output)]),label).toBe(step.expect);}
   else if(step.op==="removeDigests"){expect(removeObjectsV1(cache,(step.digests as string[]).map(digest=>digest.startsWith("{object:")?objectOf(digest.slice(8,-1)):digest)),label).toBe(step.expect);}
   else if(step.op==="prune"){expect(pruneEntriesV1(cache,Date.now()),label).toEqual(step.expect);}
   else if(step.op==="plant"){const path=join(cache,step.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,step.content);if(step.ageMilliseconds){const when=new Date(Date.now()-step.ageMilliseconds);utimesSync(path,when,when);}}
   else if(step.op==="exists"){expect(existsSync(join(cache,step.path)),label).toBe(step.expect);}
   else if(step.op==="objects"){const rows=listObjectsV1(cache);expect(rows.length,label).toBe(step.expect.count);for(const row of rows){expect(row.digest).toMatch(/^[a-f0-9]{64}$/u);expect(row.bytes).toBeGreaterThanOrEqual(0);}}
   else if(step.op==="audit"){
    const [seen]=ask([{kind:"audit",root:cache,layout,forbidden:[roots.laneA,roots.laneB]}]);
    expect(seen,label).toEqual(step.expect);
    for(const kind of [layout.entries])for(const shard of existsSync(join(cache,kind))?readdirSync(join(cache,kind)):[])for(const key of readdirSync(join(cache,kind,shard)))for(const file of readdirSync(join(cache,kind,shard,key)))expect(validator("Entry")(JSON.parse(readFileSync(join(cache,kind,shard,key,file),"utf8"))),label+" "+file).toBe(true);
   }
   else throw Error("unknown step "+step.op);
  }
 });
}

test("a directory tree digests as the sorted walk of the Python auditor, by content and not by time, and overflows past the file bound",()=>{
 const requests:any[]=[];
 for(const[index,row]of corpus.tree.entries()){
  const root=join(base,"t"+index);mkdirSync(root,{recursive:true});
  for(const directory of (row as any).directories??[])mkdirSync(join(root,directory),{recursive:true});
  for(const[path,text]of Object.entries(row.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
  if(process.platform!=="win32")for(const[path,target]of Object.entries((row as any).links??{})){mkdirSync(dirname(join(root,path)),{recursive:true});symlinkSync(target,join(root,path));}
  requests.push({kind:"tree",root,options:row.options});
 }
 const answers=ask(requests);
 for(const[index,row]of corpus.tree.entries()){
  const root=requests[index].root,options={...row.options,stampDirectory:undefined};
  const digest=treeDigestV1(root,options);
  if((row as any).overflow){expect(digest,row.name).toBeUndefined();expect(answers[index].digest,row.name).toBeNull();continue;}
  expect(digest,row.name).toBe(answers[index].digest);
  expect(digest,row.name).toMatch(/^[a-f0-9]{64}$/u);
  const stamps=join(base,"stamps"+index);expect(treeDigestV1(root,{...options,stampDirectory:stamps}),row.name+" with stamps").toBe(digest);expect(treeDigestV1(root,{...options,stampDirectory:stamps}),row.name+" from stamps").toBe(digest);
  if((row as any).touch){for(const name of Object.keys(row.files)){const when=new Date(Date.now()+3_600_000);utimesSync(join(root,name),when,when);}expect(treeDigestV1(root,options),row.name).toBe(digest);}
  else{
   const ignored=(path:string)=>row.options.ignoredExtensions.some(extension=>path.endsWith(extension))||path.split("/").slice(0,-1).some(part=>row.options.ignoredDirectories.includes(part));
   for(const path of Object.keys(row.files).filter(ignored)){writeFileSync(join(root,path),row.files[path]+"!");expect(treeDigestV1(root,options),row.name+" after a change of the ignored "+path).toBe(digest);}
   const first=Object.keys(row.files).find(path=>!ignored(path));
   if(first!==undefined){writeFileSync(join(root,first),row.files[first]+"!");expect(treeDigestV1(root,options),row.name+" after a content change").not.toBe(digest);}
  }
 }
});

test("a file digest is the sha256 of its bytes, memoised by size, time and inode, and a missing file or a directory has none",()=>{
 const directory=join(base,"stamp");mkdirSync(directory,{recursive:true});const file=join(directory,"f.bin"),stamps=join(directory,"stamps");
 writeFileSync(file,"hello");
 expect(digestFileV1(file,undefined)).toBe(sha("hello"));expect(digestFileV1(file,stamps)).toBe(sha("hello"));
 const files=[];for(const shard of readdirSync(stamps))for(const name of readdirSync(join(stamps,shard)))files.push(join(stamps,shard,name));
 expect(files.length).toBe(1);
 const info=statSync(file,{bigint:true}),mark=`${info.size}:${info.mtimeNs}:${info.ino}`;
 expect(readFileSync(files[0]!,"utf8")).toBe(mark+" "+sha("hello"));
 writeFileSync(files[0]!,mark+" "+"f".repeat(64));
 expect(digestFileV1(file,stamps),"a stamp that matches size, time and inode is trusted").toBe("f".repeat(64));
 writeFileSync(file,"hello world");expect(digestFileV1(file,stamps),"a changed file is hashed again").toBe(sha("hello world"));
 expect(digestFileV1(join(directory,"missing"),stamps)).toBeNull();expect(digestFileV1(directory,stamps)).toBeNull();
 const empty=join(directory,"empty");writeFileSync(empty,"");expect(digestFileV1(empty,undefined)).toBe(sha(""));
});

test("every decision is appended as one validated event line and an unwritable cache root is ignored",()=>{
 const root=join(base,"events");mkdirSync(root,{recursive:true});
 emitUnitEventV1(root,{lane:"l1",kind:"hit",crate:"x",key:"a".repeat(64),milliseconds:5,waitedMs:0});
 emitUnitEventV1(root,{lane:"l1",kind:"miss",crate:"y",key:null,milliseconds:7,waitedMs:2,reason:"changed"});
 const lines=readFileSync(join(root,layout.events,"l1.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(lines.length).toBe(2);
 for(const line of lines){expect(validator("Event")(line),JSON.stringify(validator("Event").errors)).toBe(true);expect(line.pid).toBe(process.pid);expect(line.version).toBe(1);expect(line.atMs).toBeGreaterThan(0);}
 expect(lines.map(line=>line.kind)).toEqual(["hit","miss"]);
 const blocked=join(base,"blocked");writeFileSync(blocked,"x");
 expect(()=>emitUnitEventV1(blocked,{lane:"l1",kind:"hit",crate:"x",key:null,milliseconds:1,waitedMs:0})).not.toThrow();
});
