import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdtempSync,mkdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import probeSchema from "../../../../../⚡️caching/🔁️verification/🧬️schema/🔣️.json" with {type:"json"};
import probeDocument from "../../../../../⚡️caching/🔁️verification/🔣️probe.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {bazelCacheProbeV1,bazelProbeArgumentsV1,bazelProbeDigestsV1,bazelProbeInputNameV1,bazelProbeLeavesV1,bazelProbeWorkspaceV1,renderBazelProbeV1} from "../🟦️.ts";
import type {CacheProbeSpecV1} from "../../../../⚙️operations/🟦️.ts";

const specification=probeDocument as CacheProbeSpecV1,python=spawnSync("python3",["-c","import ast,hashlib"]).status===0;
const read=(file:string)=>(fixture.contents as Record<string,string>)[file]===undefined?undefined:new TextEncoder().encode((fixture.contents as Record<string,string>)[file]);

test("the neutral probe specification renders the exact ASCII Bazel workspace of the fixture, and nothing in it names a fixture file",()=>{
 expect(new Ajv({strict:false}).validate(probeSchema,specification)).toBe(true);
 const rendered=renderBazelProbeV1(specification,fixture.workspace,fixture.options.module);
 expect(JSON.parse(JSON.stringify(rendered))).toEqual(fixture.files);
 expect(Object.keys(rendered)).toEqual(Object.keys(fixture.files));
 for(const path of ["MODULE.bazel","probe.bzl","BUILD.bazel"])expect(/^[\x00-\x7f]*$/u.test(rendered[path]!),path).toBe(true);
 expect(rendered["BUILD.bazel"]).not.toContain("📜️");
});

test.skipIf(!python)("the rendered Starlark parses as Python syntax and every shell script passes sh -n",()=>{
 const directory=mkdtempSync(join(tmpdir(),"bzl-probe-test-"));
 try{
  for(const[path,text]of Object.entries(fixture.files)){
   const target=join(directory,path);mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);
   if(path.endsWith(".bzl")||path.endsWith(".bazel")){const result=spawnSync("python3",["-c","import ast,sys;ast.parse(open(sys.argv[1],encoding='utf-8').read())",target]);expect(result.status,path+" "+result.stderr).toBe(0);}
   if(path.endsWith(".sh")){const result=spawnSync("sh",["-n",target]);expect(result.status,path+" "+result.stderr).toBe(0);}
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("named input groups expand to the distinct leaves a recursive SQL query derives",()=>{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE target(name TEXT PRIMARY KEY, spec TEXT)");
  for(const target of specification.targets)database.query("INSERT INTO target VALUES(?1,?2)").run(target.name,JSON.stringify(target.inputs??[]));
  const groups=JSON.stringify(specification.namedInputs);
  for(const target of specification.targets){
   const rows=database.query(`WITH RECURSIVE walk(path, entry) AS (
     SELECT printf('%03d', CAST(key AS INTEGER)), value FROM json_each((SELECT spec FROM target WHERE name = ?1))
     UNION ALL
     SELECT walk.path || '.' || printf('%03d', CAST(member.key AS INTEGER)), member.value FROM walk, json_each(?2, '$.' || json_extract(walk.entry, '$.named')) AS member WHERE json_extract(walk.entry, '$.named') IS NOT NULL
   ) SELECT entry FROM walk WHERE json_extract(entry, '$.named') IS NULL ORDER BY path`).all(target.name,groups) as {entry:string}[];
   const sql=[...new Map(rows.map(row=>[row.entry,JSON.parse(row.entry)])).values()];
   expect(bazelProbeLeavesV1(specification,target),target.name).toEqual(sql);
   expect((fixture.leaves as Record<string,unknown>)[target.name]).toEqual(sql);
  }
 }finally{database.close();}
 expect(()=>bazelProbeLeavesV1({...specification,namedInputs:{a:[{named:"a"}]}},{name:"x",cache:true,inputs:[{named:"a"}],outputs:[],command:"x"})).toThrow("Acyclic");
 expect(()=>bazelProbeLeavesV1(specification,{name:"x",cache:true,inputs:[{named:"absent"}],outputs:[],command:"x"})).toThrow("Declared named");
});

test.skipIf(!python)("input digests and names agree with hashlib, an unchanged unrelated file never enters a key, and uncached targets carry a per-run nonce",()=>{
 const digests=bazelProbeDigestsV1(specification,read,fixture.environment,fixture.nonce);
 expect(JSON.parse(JSON.stringify(digests))).toEqual(fixture.digests);
 const code="import hashlib,json,sys;print(json.dumps([hashlib.sha256(v.encode('utf-8')).hexdigest() for v in json.loads(sys.argv[1])]))";
 const names=Object.keys(fixture.inputNames),sources=names.filter(name=>name.startsWith("file:")).map(name=>(fixture.contents as Record<string,string>)[name.slice(5)]!);
 const expected=JSON.parse(spawnSync("python3",["-c",code,JSON.stringify(sources)]).stdout.toString()) as string[];
 sources.forEach((_,index)=>expect(digests[(fixture.inputNames as Record<string,string>)[names.filter(name=>name.startsWith("file:"))[index]!]!]).toBe(expected[index]+"\n"));
 for(const[key,name]of Object.entries(fixture.inputNames))expect(name).toBe("inputs/i-"+createHash("sha256").update(key).digest("hex").slice(0,12)+".txt");
 expect(Object.keys(fixture.inputNames).some(key=>key.includes("unrelated"))).toBe(false);
 expect(digests["inputs/volatile.txt"]).toBe(fixture.nonce+"\n");
 expect(bazelProbeDigestsV1(specification,read,{CACHE_PROBE_MODE:"one"},fixture.nonce)[bazelProbeInputNameV1({env:"CACHE_PROBE_MODE"})]).toBe("env=one\n");
 expect(bazelProbeDigestsV1(specification,()=>undefined,{},fixture.nonce)[bazelProbeInputNameV1({file:"source.json"})]).toBe("missing\n");
});

test("the Bazel command line reads only the fixed module catalogue, never the network, and bounds the disk cache by the probe's own size",()=>{
 const actual=bazelProbeArgumentsV1(fixture.options,specification,fixture.workspace,"build");
 expect(actual).toEqual(fixture.arguments.build);
 expect(actual).toContain("--repository_disable_download");expect(actual).toContain("--lockfile_mode=error");expect(actual).toContain("--batch");
 expect(actual.some(value=>value==="--experimental_disk_cache_gc_max_size=64M")).toBe(true);
 expect(()=>bazelProbeArgumentsV1(fixture.options,{...specification,maxCacheSize:"lots"},fixture.workspace,"build")).toThrow("Sized");
 expect(bazelProbeWorkspaceV1(fixture.workspaceName.fixture,fixture.workspaceName.instance,fixture.workspaceName.base)).toBe(fixture.workspaceName.expected);
 expect(bazelProbeWorkspaceV1(fixture.workspaceName.fixture,"other",fixture.workspaceName.base)).not.toBe(fixture.workspaceName.expected);
});

test("the probe tool declares Bazel and hands the runner one closed wire naming the fixture, task and ambient mode",()=>{
 const tool=bazelCacheProbeV1(fixture.options),invocation=tool.invocation(specification,"build","/tmp/fx",{CACHE_PROBE_MODE:"two",PATH:"/bin"});
 expect([tool.tool,tool.label,tool.version()]).toEqual(["bazel","Bazel","8.4.2"]);
 expect(invocation.command).toBe(process.execPath);expect(invocation.arguments).toHaveLength(1);
 const wire=JSON.parse(invocation.environment.SEMIO_BAZEL_PROBE!);
 expect(wire).toMatchObject({task:"build",fixture:"/tmp/fx",environment:{CACHE_PROBE_MODE:"two"},options:fixture.options});
 expect(invocation.environment.PATH).toBe("/bin");
});
