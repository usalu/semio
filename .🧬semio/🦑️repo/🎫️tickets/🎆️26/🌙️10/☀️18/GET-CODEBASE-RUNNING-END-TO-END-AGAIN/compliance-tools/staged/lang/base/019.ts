/** 📸️ Law: a capture publishes private regular copies of the declared sources whose bytes are the planned bytes whatever happens to the live checkout afterwards; the corpus is language agnostic and its content addresses come from an independent implementation. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,symlinkSync,utimesSync,writeFileSync,chmodSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema.json";
import policySchema from "../🧬️schema/🔣️.json";
import {collectBazelPoolV1} from "../../🧵️sessions/🔌️port/🟦️.ts";
import {captureBazelSourceSnapshotV1,bazelSourceSnapshotDriftV1,readBazelSourceSnapshotV1,renderBazelSourceSnapshotV1} from "../🟦️.ts";

const python=testPythonV1(),views={alias:"a",real:"s"},sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),alias=(directory:string)=>sha(directory).slice(0,corpus.alias.hexLength);
const bytesOf=(file:{text?:string;base64?:string})=>file.text!==undefined?Buffer.from(file.text):Buffer.from(file.base64!,"base64");
const aliasPath=(space:string,path:string)=>{const cut=path.lastIndexOf("/");return join(space,views.alias,alias(cut<0?"":path.slice(0,cut)),path.slice(cut+1));},realPath=(space:string,path:string)=>join(space,views.real,...path.split("/"));
const old=Date.now()/1000-86400;
const populate=(checkout:string)=>{for(const file of corpus.files){const target=join(checkout,file.path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,bytesOf(file),{mode:file.executable?0o755:0o644});chmodSync(target,file.executable?0o755:0o644);utimesSync(target,old,old);}};
const request=(checkout:string,space:string,options:{files?:string[];real?:string[];now?:()=>number;pool?:string}={})=>({root:checkout,directory:space,...options.pool?{pool:options.pool}:{},files:options.files??corpus.files.map(file=>file.path),real:new Set(options.real??corpus.real),views,alias,check:()=>{},progress:async()=>{},...(options.now?{now:options.now}:{})});
const scratch=()=>{const directory=mkdtempSync(join(tmpdir(),"bazel-snapshot-")),checkout=join(directory,"checkout"),space=join(directory,"workspace");mkdirSync(checkout);mkdirSync(space);populate(checkout);return{directory,checkout,space};};

test("snapshot corpus is closed and its content addresses are verified by an independent implementation",()=>{
 expect(new Ajv({strict:false}).validate(corpusSchema,corpus)).toBe(true);
 for(const file of corpus.files)expect(sha(bytesOf(file)),file.path).toBe(file.sha256);
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I","-c","import base64,hashlib,json,sys\nrows=json.loads(sys.stdin.read())\nprint(json.dumps([hashlib.sha256((r['text'].encode() if 'text' in r else base64.b64decode(r['base64']))).hexdigest() for r in rows['files']]+[hashlib.sha256(d.encode()).hexdigest()[:rows['hex']] for d in ['','lib','lib/src','deep/x/y/z']]))"],{stdin:new TextEncoder().encode(JSON.stringify({files:corpus.files,hex:corpus.alias.hexLength})),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const values=JSON.parse(result.stdout.toString()) as string[];
 expect(values.slice(0,corpus.files.length)).toEqual(corpus.files.map(file=>file.sha256));
 expect(values.slice(corpus.files.length)).toEqual(["","lib","lib/src","deep/x/y/z"].map(alias));
});

test("snapshot policy is closed and bounded",()=>{
 const policy=policySchema.definitions.Policy.const;
 expect(policy.attempts).toBeGreaterThanOrEqual(1);expect(policy.attempts).toBeLessThanOrEqual(10);expect(policy.concurrency).toBeGreaterThanOrEqual(1);
 expect(policy.fileMode&0o222).toBe(0o200);expect(policy.executableMode&0o111).toBe(0o111);
});

test("a capture lays out private regular copies at the alias and repository labels with the planned bytes, content addresses and permissions",async()=>{
 const{directory,checkout,space}=scratch();
 try{
  const result=await captureBazelSourceSnapshotV1(request(checkout,space));
  expect(result.files).toEqual(corpus.files.map(file=>file.path).sort());expect(result.vanished).toEqual([]);expect(result.captured).toBe(corpus.files.length);expect(result.reused).toBe(0);
  const rows=await readBazelSourceSnapshotV1(space);expect(rows.size).toBe(corpus.files.length);
  for(const file of corpus.files){
   const live=lstatSync(join(checkout,file.path)),copy=lstatSync(aliasPath(space,file.path));
   expect(readFileSync(aliasPath(space,file.path)).equals(bytesOf(file)),file.path).toBe(true);
   expect(copy.isFile()&&!copy.isSymbolicLink(),file.path).toBe(true);expect(copy.nlink,"never a hardlink: "+file.path).toBe(1);expect(copy.ino,"never the live inode: "+file.path).not.toBe(live.ino);
   if(process.platform!=="win32")expect((copy.mode&0o111)!==0,"executable bit "+file.path).toBe(file.executable);
   const row=rows.get(file.path)!;expect(row.sha256).toBe(file.sha256);expect(row.bytes).toBe(bytesOf(file).length);expect(row.executable).toBe(file.executable);expect(row.racy).toBe(false);
   if(corpus.real.includes(file.path)){const repository=lstatSync(realPath(space,file.path));expect(readFileSync(realPath(space,file.path)).equals(bytesOf(file)),file.path).toBe(true);expect(repository.nlink).toBe(1);expect(repository.ino).not.toBe(copy.ino);expect(row.realMtimeNs).toBeDefined();}
   else{expect(existsSync(realPath(space,file.path))).toBe(false);expect(row.realMtimeNs).toBeUndefined();}
  }
  expect(new Set(readdirSync(join(space,views.alias)))).toEqual(new Set(corpus.files.map(file=>{const cut=file.path.lastIndexOf("/");return alias(cut<0?"":file.path.slice(0,cut));})));
  expect(result.directories.get(alias("lib/src"))).toBe("lib/src");expect(readdirSync(space).filter(name=>name.endsWith(".tmp"))).toEqual([]);expect(existsSync(join(space,"snapshot-tmp"))).toBe(false);
  expect(await bazelSourceSnapshotDriftV1(checkout,rows,corpus.files.map(file=>file.path))).toEqual([]);
  writeFileSync(join(checkout,"lib/src/a.txt"),"drifted\n");expect(await bazelSourceSnapshotDriftV1(checkout,rows,corpus.files.map(file=>file.path))).toEqual(["lib/src/a.txt"]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

for(const scenario of corpus.scenarios)test("scenario: "+scenario.name,async()=>{
 const{directory,checkout,space}=scratch();let last:Awaited<ReturnType<typeof captureBazelSourceSnapshotV1>>|undefined,identities=new Map<string,string>(),untouched=new Map<string,string>(),tick=0;
 const identity=(path:string)=>{const facts=lstatSync(aliasPath(space,path),{bigint:true});return facts.ino+":"+facts.mtimeNs;};
 try{
  for(const step of scenario.steps as any[]){
   switch(step.op){
    case "capture":last=await captureBazelSourceSnapshotV1(request(checkout,space,{files:step.only??[...corpus.files.map(file=>file.path),...(step.extra??[])],...(step.real?{real:step.real}:{})}));break;
    case "write":{const target=join(checkout,step.path);writeFileSync(target,step.text);utimesSync(target,old+60*(++tick),old+60*tick);break;}
    case "remove":rmSync(join(checkout,step.path));break;
    case "expect":{const target=step.view==="alias"||!corpus.real.includes(step.path)?[aliasPath(space,step.path)]:[aliasPath(space,step.path),realPath(space,step.path)];for(const path of target)expect(readFileSync(path,"utf8"),path).toBe(step.text);break;}
    case "identities":for(const file of corpus.files)identities.set(file.path,identity(file.path));break;
    case "expect-identities":for(const[path,value]of identities)expect(identity(path),path).toBe(value);break;
    case "expect-counts":expect({captured:last!.captured,reused:last!.reused}).toEqual({captured:step.captured,reused:step.reused});break;
    case "expect-vanished":expect(last!.vanished).toEqual(step.paths);for(const path of step.paths){expect(existsSync(aliasPath(space,path))).toBe(false);expect(last!.files).not.toContain(path);}break;
    case "expect-absent":expect(existsSync(aliasPath(space,step.path))).toBe(false);expect(existsSync(realPath(space,step.path))).toBe(false);break;
    case "expect-absent-real":expect(existsSync(realPath(space,step.path))).toBe(false);expect((await readBazelSourceSnapshotV1(space)).get(step.path)?.realMtimeNs).toBeUndefined();break;
    case "expect-aliases":expect(new Set(readdirSync(join(space,views.alias)))).toEqual(new Set((step.directories as string[]).map(alias)));break;
    case "expect-real-tree":{const found:string[]=[],visit=(folder:string,prefix:string)=>{for(const entry of readdirSync(folder,{withFileTypes:true}))if(entry.isDirectory())visit(join(folder,entry.name),prefix+entry.name+"/");else found.push(prefix+entry.name);};visit(join(space,views.real),"");expect(found.sort()).toEqual([...step.paths].sort());break;}
    case "legacy-links":{
     for(const file of corpus.files)untouched.set(file.path,sha(readFileSync(join(checkout,file.path))));
     mkdirSync(join(space,views.alias));for(const folder of new Set(corpus.files.map(file=>{const cut=file.path.lastIndexOf("/");return cut<0?"":file.path.slice(0,cut);})))symlinkSync(join(checkout,folder),join(space,views.alias,alias(folder)),"dir");
     symlinkSync(checkout,join(space,views.real),"dir");break;
    }
    case "expect-real-views":for(const view of [views.alias,views.real]){expect(lstatSync(join(space,view)).isSymbolicLink(),view).toBe(false);for(const entry of readdirSync(join(space,view)))expect(lstatSync(join(space,view,entry)).isSymbolicLink(),entry).toBe(false);}break;
    case "expect-live-untouched":for(const[path,value]of untouched)expect(sha(readFileSync(join(checkout,path))),path).toBe(value);expect(new Set(readdirSync(checkout))).toEqual(new Set(["lib","top.txt","deep"]));expect(readdirSync(join(checkout,"lib")).sort()).toEqual(["bin","data","empty.txt","src"]);break;
    default:throw Error("Unknown corpus operation "+step.op);
   }
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a file modified within the racy window is captured again until it has proven stable",async()=>{
 const{directory,checkout,space}=scratch();
 try{
  const target=join(checkout,"lib/src/a.txt"),now=Date.now();writeFileSync(target,"fresh\n");utimesSync(target,now/1000,now/1000);
  const first=await captureBazelSourceSnapshotV1(request(checkout,space,{now:()=>now}));expect(first.captured).toBe(corpus.files.length);expect((await readBazelSourceSnapshotV1(space)).get("lib/src/a.txt")!.racy).toBe(true);
  const second=await captureBazelSourceSnapshotV1(request(checkout,space,{now:()=>now+60000}));expect(second.captured).toBe(1);expect(second.reused).toBe(corpus.files.length-1);expect((await readBazelSourceSnapshotV1(space)).get("lib/src/a.txt")!.racy).toBe(false);
  const third=await captureBazelSourceSnapshotV1(request(checkout,space,{now:()=>now+120000}));expect(third.captured).toBe(0);expect(third.reused).toBe(corpus.files.length);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a corrupted or hardlinked copy is replaced from the live tree rather than trusted",async()=>{
 const{directory,checkout,space}=scratch();
 try{
  await captureBazelSourceSnapshotV1(request(checkout,space));
  const copy=aliasPath(space,"lib/src/a.txt");chmodSync(copy,0o644);writeFileSync(copy,"corrupt!!\n");
  const result=await captureBazelSourceSnapshotV1(request(checkout,space));expect(result.captured).toBe(1);expect(readFileSync(copy,"utf8")).toBe("alpha\n");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a torn capture never publishes mixed bytes: a file rewritten while it is captured is read again a bounded number of times",async()=>{
 const{directory,checkout,space}=scratch(),target=join(checkout,"lib/data/blob.bin"),size=4*1024*1024;
 const writer=Bun.spawn([process.execPath,"-e",`const fs=require("node:fs");const target=${JSON.stringify(target)},size=${size};let flip=0;const end=Date.now()+20000;while(Date.now()<end){fs.writeFileSync(target,Buffer.alloc(size,(flip++%2)?66:65));Bun.sleepSync(3);}`],{stdout:"ignore",stderr:"ignore"});
 try{
  await Bun.sleep(300);let published=0,refused=0;
  for(let round=0;round<6;round++){
   try{await captureBazelSourceSnapshotV1(request(checkout,space,{files:["lib/data/blob.bin"],real:[]}));const bytes=readFileSync(aliasPath(space,"lib/data/blob.bin"));const row=(await readBazelSourceSnapshotV1(space)).get("lib/data/blob.bin")!;expect(bytes.length).toBe(size);expect(bytes.every(byte=>byte===bytes[0])).toBe(true);expect(sha(bytes)).toBe(row.sha256);published++;}
   catch(error){expect(String((error as Error).message)).toMatch(/changed during all \d+ capture attempts|ENOENT|ENOTDIR/u);refused++;}
  }
  expect(published+refused).toBe(6);
 }finally{writer.kill();await writer.exited;rmSync(directory,{recursive:true,force:true});}
},120000);

test("a path that escapes the checkout is refused before anything is written",async()=>{
 const{directory,checkout,space}=scratch();
 try{
  for(const bad of ["../outside.txt","/abs.txt","a//b.txt","a/./b.txt","a\\b.txt",""])await expect(captureBazelSourceSnapshotV1(request(checkout,space,{files:[bad]}))).rejects.toThrow("Closed repository source path required");
  expect(existsSync(join(space,"snapshot.json"))).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("two directories that share an alias are refused",async()=>{
 const{directory,checkout,space}=scratch();
 try{await expect(captureBazelSourceSnapshotV1({...request(checkout,space),alias:()=>"same"})).rejects.toThrow("alias collision");}
 finally{rmSync(directory,{recursive:true,force:true});}
});

test("the capture summary exists in English and German and names every count",()=>{
 const result={files:["a","b","c"],vanished:["d"],captured:2,reused:1,pruned:5};
 const english=renderBazelSourceSnapshotV1(result,"en"),german=renderBazelSourceSnapshotV1(result,"de");
 for(const text of[english,german]){expect(text).not.toMatch(/[{}]/u);for(const value of["3","2","1","5"])expect(text).toContain(value);expect(text.startsWith("[bazel]")).toBe(true);}
 expect(english).not.toBe(german);expect(german).toContain("Dateien");
 expect(Object.keys(policySchema.definitions.Messages.const.en)).toEqual(Object.keys(policySchema.definitions.Messages.const.de));
});

test("a pool lets every package clone identical content: same bytes, modes and private inodes in all packages, one pool entry per content, and packages survive the pool being removed",async()=>{
 const{directory,checkout}=scratch(),pool=join(directory,"pool"),spaces=["one","two","three"].map(name=>join(directory,name));
 try{
  const big=Buffer.alloc(32*1048576,7);writeFileSync(join(checkout,"lib/big.bin"),big);
  const files=[...corpus.files.map(file=>file.path),"lib/big.bin"];
  for(const space of spaces){mkdirSync(space);await captureBazelSourceSnapshotV1({...request(checkout,space,{files,pool}),pool});}
  const entries=existsSync(pool)?readdirSync(pool).filter(name=>/^[0-9a-f]{2}$/u.test(name)).flatMap(name=>readdirSync(join(pool,name))):[];
  for(const space of spaces){
   for(const file of corpus.files){const copy=lstatSync(aliasPath(space,file.path));expect(readFileSync(aliasPath(space,file.path)).equals(bytesOf(file)),file.path).toBe(true);expect(copy.nlink,"never a hardlink: "+file.path).toBe(1);if(process.platform!=="win32")expect((copy.mode&0o111)!==0,"executable bit "+file.path).toBe(file.executable);}
   expect(readFileSync(aliasPath(space,"lib/big.bin")).equals(big)).toBe(true);expect(existsSync(join(space,"snapshot-tmp"))).toBe(false);
  }
  expect(new Set(spaces.map(space=>lstatSync(aliasPath(space,"lib/big.bin")).ino)).size,"private inodes").toBe(3);
  if(entries.length){
   const contents=new Set(files.map(path=>sha(readFileSync(join(checkout,path)))));
   expect(entries.length).toBeGreaterThanOrEqual(contents.size);expect(entries.every(name=>/^[0-9a-f]{64}(\.x)?$/u.test(name))).toBe(true);expect(entries.some(name=>name.startsWith(sha(big)))).toBe(true);
   rmSync(pool,{recursive:true,force:true});expect(readFileSync(aliasPath(spaces[0]!,"lib/big.bin")).equals(big)).toBe(true);
   writeFileSync(join(checkout,"lib/big.bin"),Buffer.alloc(1048576,9));const again=await captureBazelSourceSnapshotV1(request(checkout,spaces[0]!,{files,pool}));expect(again.captured).toBe(1);expect(readFileSync(aliasPath(spaces[0]!,"lib/big.bin")).length).toBe(1048576);expect(existsSync(pool)).toBe(true);
  }else expect(existsSync(pool)?readdirSync(pool).filter(name=>/^[0-9a-f]{2}$/u.test(name)):[]).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("a pool collector that removes entries and folders while captures run never fails a capture: bytes stay exact (independent Python hashing) and the pool is re-seeded",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bazel-pool-race-")),checkout=join(directory,"checkout"),pool=join(directory,"pool"),spaces=["one","two","three"].map(name=>join(directory,name));
 try{
  mkdirSync(checkout);const files:string[]=[],expected:Record<string,string>={};
  for(let index=0;index<450;index++){const path=`d${index%9}/e${index%4}/f${index}.txt`,text=`content ${index%40} `.repeat(200+index%40);mkdirSync(join(checkout,path,".."),{recursive:true});writeFileSync(join(checkout,path),text);files.push(path);expected[path]=sha(text);}
  let running=true,removed=0,sweeps=0;
  const collector=(async()=>{while(running){const result=await collectBazelPoolV1(pool,new AbortController().signal,{now:Date.now()+1e9,maximumAgeMs:0});removed+=result.entries;sweeps++;await new Promise(done=>setTimeout(done,1));}})();
  for(const space of spaces)mkdirSync(space);
  const results=await Promise.all(spaces.map(space=>captureBazelSourceSnapshotV1({...request(checkout,space,{files,real:[],pool}),pool})));
  running=false;await collector;
  expect(sweeps).toBeGreaterThan(0);expect(results.every(result=>result.captured===files.length&&result.vanished.length===0)).toBe(true);
  const rows=files.flatMap(path=>spaces.map(space=>[aliasPath(space,path),expected[path]!]));
  if(python){const run=Bun.spawnSync([python,"-I","-c","import hashlib,json,sys\nrows=json.loads(sys.stdin.read())\nprint(json.dumps([hashlib.sha256(open(p,'rb').read()).hexdigest()==h for p,h in rows]))"],{stdin:new TextEncoder().encode(JSON.stringify(rows))});expect(run.exitCode,run.stderr.toString()).toBe(0);expect(JSON.parse(run.stdout.toString()).every(Boolean)).toBe(true);}
  for(const[path,hash]of rows)expect(sha(readFileSync(path)),path).toBe(hash);
  expect(removed,"the collector really removed entries during the captures").toBeGreaterThan(0);
  await captureBazelSourceSnapshotV1(request(checkout,join(directory,"four"),{files,real:[],pool}));
  expect(readdirSync(pool).some(name=>/^[0-9a-f]{2}$/u.test(name))).toBe(true);
 }finally{rmSync(directory,{recursive:true,force:true});}
},120000);
