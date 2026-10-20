import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "./🔣️.json" with {type:"json"};
import {captureFetchedV1,classifyFetchedLockV1,fetchedHoldsV1,fetchedLockPathV1,fetchedPackagesV1,readFetchedReceiptV1,type FetchedPortV1,type FetchedReceiptV1} from "../🟦️.ts";

const request="a".repeat(64),pause=()=>new Promise<void>(done=>setTimeout(done,15));
const portOf=(workspace:string,home:string):FetchedPortV1=>({
 read:async path=>{try{return new Uint8Array(readFileSync(join(workspace,path)));}catch{return undefined;}},
 stamp:path=>{try{const row=lstatSync(path,{bigint:true});return row.isDirectory()||row.isFile()?{dev:String(row.dev),ino:String(row.ino),mtimeNs:String(row.mtimeNs)}:undefined;}catch{return undefined;}},
 list:path=>{try{return readdirSync(path,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>entry.name);}catch{return undefined;}},
});
const lockOf=(name:string)=>(fixture.locks as Record<string,string>)[name]!;

for(const[name,expected]of Object.entries(fixture.classes))test("fetched lock classification: "+name,()=>{expect(classifyFetchedLockV1(lockOf(name))).toBe(expected as "registry"|"unsupported");});

for(const row of fixture.owners)test("fetched lock owner: "+row.name,async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"fetched-owner-"));
 try{for(const[path,text]of Object.entries(row.files)){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text);}expect(await fetchedLockPathV1(row.manifest,portOf(workspace,workspace).read)).toBe(row.lock);}finally{rmSync(workspace,{recursive:true,force:true});}
});

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
const oracleSource=`
import hashlib,json,os,re,sys
workspace,home,receipt=sys.argv[1],sys.argv[2],json.loads(sys.argv[3])
locks=[];names=set()
for l in receipt["locks"]:
    data=open(os.path.join(workspace,l["path"]),"rb").read();locks.append({"path":l["path"],"digest":hashlib.sha256(data).hexdigest()})
    for block in data.decode().split("[[package]]")[1:]:
        if re.search(r"^source = ",block,re.M):
            names.add(re.search(r'^name = "([^"]+)"',block,re.M).group(1)+"-"+re.search(r'^version = "([^"]+)"',block,re.M).group(1))
rows=[];cache=os.path.join(home,"registry","cache")
for name in sorted(names):
    for index in sorted(os.listdir(cache)):
        path=os.path.join(cache,index,name+".crate")
        if os.path.lexists(path):
            s=os.lstat(path);rows.append(["registry/cache/"+index+"/"+name+".crate",str(s.st_dev),str(s.st_ino),str(s.st_mtime_ns)]);break
print(json.dumps({"locks":locks,"registry":{"digest":hashlib.sha256(json.dumps(rows,separators=(",",":")).encode()).hexdigest(),"count":len(rows)}}))
`;

test("fetched receipt: follows the lockfile and the locked packages through a mutation sequence, and ignores every other package",async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"fetched-ws-")),home=mkdtempSync(join(tmpdir(),"fetched-home-"));
 try{
  mkdirSync(join(workspace,"app"),{recursive:true});writeFileSync(join(workspace,"Cargo.toml"),"[workspace]\nmembers = [\"app\"]\n");writeFileSync(join(workspace,"app","Cargo.toml"),"[package]\nname = \"app\"\nversion = \"0.1.0\"\n");
  for(const index of["idx","idx2"]){mkdirSync(join(home,"registry","cache",index),{recursive:true});mkdirSync(join(home,"registry","src",index),{recursive:true});}
  const port=portOf(workspace,home);let receipt:FetchedReceiptV1|undefined,lock="registry";writeFileSync(join(workspace,"Cargo.lock"),lockOf(lock));
  for(const step of fixture.sequence as Array<{op:string;lock?:string;defined?:boolean;expect?:boolean;name?:string;index?:string}>){
   await pause();
   const crate=()=>join(home,"registry","cache",step.index??"idx",step.name!);
   if(step.op==="writeLock"){lock=step.lock!;writeFileSync(join(workspace,"Cargo.lock"),lockOf(lock));}
   else if(step.op==="addCrate")writeFileSync(crate(),"crate");
   else if(step.op==="replaceCrate"){rmSync(crate());writeFileSync(crate(),"crate again");}
   else if(step.op==="removeCrate")rmSync(crate());
   else if(step.op==="addSource")mkdirSync(join(home,"registry","src","idx",step.name!));
   else if(step.op==="capture"){writeFileSync(join(workspace,"Cargo.lock"),lockOf(step.lock!));const got=await captureFetchedV1({request,manifests:["app/Cargo.toml"],home},port);expect(got!==undefined).toBe(step.defined!);if(got){receipt=got;lock=step.lock!;if(python){const independent=JSON.parse(spawnSync(python,["-I","-c",oracleSource,workspace,home,JSON.stringify(got)],{encoding:"utf8"}).stdout);expect(independent.locks.map((row:{digest:string})=>row.digest)).toEqual(got.locks.map(row=>row.digest));expect(independent.registry).toEqual(got.registry);}}}
   else if(step.op==="holds")expect(await fetchedHoldsV1(receipt!,home,port)).toBe(step.expect!);
  }
 }finally{rmSync(workspace,{recursive:true,force:true});rmSync(home,{recursive:true,force:true});}
});

test("fetched packages: names and versions of registry packages only",()=>{
 expect(fetchedPackagesV1(lockOf("registry"))).toEqual(["serde-1.0.0"]);expect(fetchedPackagesV1(lockOf("path"))).toEqual([]);expect(fetchedPackagesV1(lockOf("broken"))).toBeUndefined();
});

for(const row of fixture.tamper)test("fetched receipt admission: "+row.name,async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"fetched-adm-")),home=mkdtempSync(join(tmpdir(),"fetched-adm-home-"));
 try{
  mkdirSync(join(home,"registry","cache","idx"),{recursive:true});writeFileSync(join(home,"registry","cache","idx","serde-1.0.0.crate"),"crate");writeFileSync(join(workspace,"Cargo.toml"),"[workspace]\n");writeFileSync(join(workspace,"Cargo.lock"),lockOf("registry"));
  const receipt=await captureFetchedV1({request,manifests:["Cargo.toml"],home},portOf(workspace,home));expect(receipt).toBeDefined();
  const text=JSON.stringify(row.mutate==="extra"?{...receipt,extra:true}:receipt);
  expect(readFetchedReceiptV1(text,row.mutate==="request"?"b".repeat(64):request)!==undefined).toBe(row.admitted);
 }finally{rmSync(workspace,{recursive:true,force:true});rmSync(home,{recursive:true,force:true});}
});
