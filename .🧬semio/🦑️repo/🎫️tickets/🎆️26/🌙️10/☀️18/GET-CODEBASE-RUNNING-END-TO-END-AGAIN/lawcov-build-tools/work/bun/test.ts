/** 🥟️ Contract: the pinned Bun release is selected per platform, architecture and libc and its executable is extracted from the release ZIP exactly like Python's `zipfile` does, with every malformed archive refused with its own message; the language-agnostic cases live in `./🔣️.json`. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {bunArchiveExecutable,bunDistribution} from "@UNIT/🟦️.ts";
import manifestSchema from "@UNIT/🧬️schema/🔣️.json";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE"),release=manifestSchema.definitions.Release.const;
const ask=(request:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:128*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};
const sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

test("the corpus is closed by its schema and the pinned release by the unit's own schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,archives:[{...corpus.archives[0],mutations:[{op:"central",entry:"x",field:"colour",value:1}]}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,distributions:corpus.distributions.slice(1)})).toBe(false);
 expect(ajv.validate({...manifestSchema,definitions:undefined},release),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate({...manifestSchema,definitions:undefined},{...release,distributions:release.distributions.slice(1)})).toBe(false);
});

test("every platform selects the authored distribution, named as the release names it, and nothing else is selected",()=>{
 const names=ask({op:"names",rows:corpus.distributions});
 for(const[index,row]of corpus.distributions.entries()){
  const picked=bunDistribution(release.version,row.platform,row.arch,row.musl);
  expect(picked.archive,row.archive).toBe(row.archive);expect(names[index],row.archive).toBe(row.archive);
  expect(picked).toEqual(release.distributions.find(item=>item.archive===row.archive)!);
 }
 expect(new Set(release.distributions.map(row=>row.sha256)).size).toBe(release.distributions.length);
 expect(release.distributions.map(row=>row.archive).sort()).toEqual(corpus.distributions.map(row=>row.archive).sort());
 expect(release.release).toBe("https://github.com/oven-sh/bun/releases/download/bun-v"+release.version);
 const unsupported=ask({op:"names",rows:corpus.unsupported});
 for(const[index,row]of corpus.unsupported.entries()){
  const version=row.versionOffset===0?release.version:release.version.replace(/\d+$/u,part=>String(Number(part)+1));
  expect(refusal(()=>bunDistribution(version,row.platform,row.arch,row.musl)),row.name).toBe("Unsupported pinned Bun: "+version+" "+row.platform+"/"+row.arch+(row.musl?"/musl":""));
  if(row.versionOffset===0)expect(unsupported[index],row.name).toBeNull();
 }
});

test("the executable is extracted exactly like zipfile reads it and every malformed archive is refused with its own message",()=>{
 const built=ask({op:"archive",rows:corpus.archives.map(row=>({members:row.members,mutations:row.mutations,select:row.select}))});
 for(const[index,row]of corpus.archives.entries()){
  const archive=Buffer.from(built[index].archive,"base64");let payload:Buffer|null=null;
  const message=refusal(()=>{payload=bunArchiveExecutable(archive,row.select);});
  expect(message,row.name).toBe(row.error);
  if(row.error===""){
   expect(sha(payload!),row.name).toBe(row.sha256);expect(payload!.length,row.name).toBe(row.bytes);expect(built[index].reference,row.name).toBe(row.sha256);
   expect(ask({op:"sha256",texts:[payload!.toString("base64")]})[0],row.name).toBe(row.sha256);
  }
 }
});

test("a sample of unrelated random bytes never extracts a member and never throws anything but the declared messages",()=>{
 const declared=new Set(["Bun archive has no selected executable","Invalid Bun ZIP directory","Invalid Bun ZIP executable","Invalid Bun ZIP size"]);
 let seed=7;const next=()=>(seed=(seed*1103515245+12345)&0x7fffffff)&0xff;
 for(let round=0;round<200;round++){
  const bytes=Buffer.from(Array.from({length:30+round},next)),message=refusal(()=>bunArchiveExecutable(bytes,"x"));
  expect(declared.has(message)||/range|zlib|bounds/iu.test(message),message).toBe(true);
 }
});
