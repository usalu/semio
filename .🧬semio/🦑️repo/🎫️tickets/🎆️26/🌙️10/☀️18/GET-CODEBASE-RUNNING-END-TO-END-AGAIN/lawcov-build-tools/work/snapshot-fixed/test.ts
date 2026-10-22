/** 🧷️ Contract: a captured authorizer snapshot round-trips real captured sources, is accepted only when it equals what an independent Python builder produces and is refused with its own message for every fault in `./📋️faults.json`; the unit's own entry roles in `./🔣️.json` are closed, and the corpus is closed by its schema. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1} from "@LIB/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import {compileNxAuthorizerSnapshotV1,readNxAuthorizerSnapshotV1,snapshotNxAuthorizerSourcesV1} from "@UNIT/🟦️.ts";
import unitSchema from "@UNIT/🧬️schema/🔣️.json";
import policy from "@UNIT/🔣️policy.json";
import own from "@UNIT/🧪️tests/🔣️.json";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(request:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:512*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};
const textOf=(content:any):string=>"repeat" in content?content.repeat.repeat(content.times):content.text;
let workspace="";
beforeAll(()=>{workspace=realpathSync(mkdtempSync(join(tmpdir(),"auth-snap-")));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the corpus is closed by its schema, the policy is the pinned one and the unit's own cases are well formed",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,snapshots:[{...corpus.snapshots[0],patch:{explode:1}}]})).toBe(false);
 expect(policy).toEqual({maximumSourceBytes:16777216,maximumFileBytes:4194304,maximumSnapshotBytes:24117248,maximumOutputBytes:8388608});
 for(const row of own.entryRoles)expect(ajv.validate({$defs:unitSchema.$defs,$ref:"#/$defs/EntryRoleCase"},row),row.id).toBe(true);
 expect(ajv.validate({$defs:unitSchema.$defs,$ref:"#/$defs/EntryRoleCase"},{...own.entryRoles[0],importedMain:true})).toBe(false);
});

test("every fault is refused with its own message and valid snapshots are accepted exactly as built by the Python builder",()=>{
 const built=ask({op:"build",rows:corpus.snapshots});
 for(const[index,row]of corpus.snapshots.entries()){
  let value:any=null;const message=refusal(()=>{value=readNxAuthorizerSnapshotV1(built[index]);});
  expect(message,row.name).toBe(row.error);
  if(row.error===""){
   expect(value.digest,row.name).toBe(built[index].digest);expect(value.files.map((file:any)=>file.path),row.name).toEqual(row.sources.map(source=>source.path));
   expect(createHash("sha256").update(JSON.stringify(value.manifest)+"\n").digest("hex"),row.name).toBe(value.digest);
   expect(Object.isFrozen(value.files),row.name).toBe(true);expect(Object.isFrozen(value.manifest),row.name).toBe(true);
  }
 }
});

test("the data an untrusted snapshot may carry is closed: accessors, foreign prototypes, symbols, deep nesting and oversized arrays are refused",()=>{
 const good=ask({op:"build",rows:[corpus.snapshots[1]]})[0],clone=()=>structuredClone(good);
 const getter=clone();Object.defineProperty(getter,"digest",{get:()=>good.digest,enumerable:true});
 const foreign=Object.assign(Object.create({inherited:true}),clone()),symbolic=Object.assign(clone(),{[Symbol("x")]:1});
 const sparse=clone();sparse.files=new Array(1);const deep:any={};let cursor=deep;for(let level=0;level<8;level++){cursor.next={};cursor=cursor.next;}
 for(const[name,value]of Object.entries({getter,foreign,symbolic,sparse,deep,number:7,nothing:null,text:"snapshot"}))expect(refusal(()=>readNxAuthorizerSnapshotV1(value)),name).not.toBe("");
 expect(refusal(()=>readNxAuthorizerSnapshotV1(clone()))).toBe("");
});

test("real captured sources round-trip through a snapshot with the closure and digest the Python hash computes",async()=>{
 const root=join(workspace,"w");
 for(const file of corpus.workspace.files){mkdirSync(dirname(join(root,...file.path.split("/"))),{recursive:true});writeFileSync(join(root,...file.path.split("/")),textOf(file.content));}
 const captured=await captureNxAuthorizerSourcesV1(root,corpus.workspace.entry,new AbortController().signal);
 expect(captured.manifest.sources.map(source=>source.path)).toEqual(corpus.workspace.closure);
 const expected=ask({op:"closure",files:corpus.workspace.files,paths:[corpus.workspace.entry,...corpus.workspace.closure.filter(path=>path!==corpus.workspace.entry)]});
 expect(captured.manifest.sources).toEqual(expected.manifest.sources);expect(captured.digest).toBe(expected.digest);
 const snapshot=snapshotNxAuthorizerSourcesV1(captured),back=readNxAuthorizerSnapshotV1(JSON.parse(JSON.stringify(snapshot)));
 expect(back.digest).toBe(captured.digest);expect(back.files.map(file=>Buffer.from(file.base64,"base64").toString("utf8"))).toEqual(corpus.workspace.closure.map(path=>textOf(corpus.workspace.files.find(file=>file.path===path)!.content)));
 const bytes=nxAuthorizerCapturedBytesV1(captured);bytes.get("a.ts")![0]=0;expect(Buffer.from(nxAuthorizerCapturedBytesV1(captured).get("a.ts")!).toString("utf8")).toBe(textOf(corpus.workspace.files[0]!.content));
});

test("the compiler refuses unknown targets and entry roles before it asks for any authority",async()=>{
 const snapshot=ask({op:"build",rows:[corpus.snapshots[1]]})[0],control={} as never;
 for(const target of corpus.roles.targets.refused)await expect(compileNxAuthorizerSnapshotV1(workspace,snapshot,control,target as never,"module")).rejects.toThrow("Closed authorizer compiler target required");
 for(const role of corpus.roles.refused)await expect(compileNxAuthorizerSnapshotV1(workspace,snapshot,control,"node",role as never)).rejects.toThrow("Closed authorizer entry role required");
 for(const role of corpus.roles.valid)await expect(compileNxAuthorizerSnapshotV1(workspace,snapshot,control,"node",role as never)).rejects.not.toThrow(/Closed authorizer (compiler target|entry role) required/u);
 expect(own.refusedEntryRoles.map(String).sort()).toEqual(corpus.roles.refused.filter(role=>role!==0).map(String).sort().filter(item=>own.refusedEntryRoles.map(String).includes(item)).sort());
});

