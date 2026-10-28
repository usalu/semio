import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {appendBazelPlanKeyPartsV1,bazelPlanKeyPartsV1,bazelPlanKeyV1} from "../../🟦️.ts";

const parts={version:1,root:"/w",workspace:"/w/.cache/s3",request:{tool:"bazel",args:["run","a:build"]},graph:"a".repeat(64),executor:"b".repeat(64),scopes:{ids:["x"],rows:"c".repeat(64)},capabilities:{buildMode:"prototype",arguments:["repo","run"]},runtime:{platform:"darwin",arch:"arm64"},preload:"d".repeat(64)};

test("The parts of a plan key are digests of their canonical text, equal to an independent Python computation, and two keys that differ differ in exactly the parts whose digests differ",()=>{
 const mine=bazelPlanKeyPartsV1(parts);
 expect(Object.keys(mine)).toEqual(Object.keys(parts).sort());
 const python=spawnSync(process.platform==="win32"?"python":"python3",["-I","-c","import hashlib,json,sys\nparts=json.load(sys.stdin)\nprint(json.dumps({name:hashlib.sha256(json.dumps(parts[name],sort_keys=True,separators=(',',':'),ensure_ascii=False).encode('utf8')).hexdigest()[:12] for name in sorted(parts)}))"],{input:JSON.stringify(parts),encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 expect(JSON.parse(python.stdout)).toEqual(mine);
 const moved={...parts,graph:"e".repeat(64),capabilities:{...parts.capabilities,buildMode:"release"}},other=bazelPlanKeyPartsV1(moved);
 expect(Object.keys(mine).filter(name=>mine[name]!==other[name]).sort()).toEqual(["capabilities","graph"]);
 expect(bazelPlanKeyV1(moved)).not.toBe(bazelPlanKeyV1(parts));
 expect(bazelPlanKeyPartsV1({...parts,request:{args:["run","a:build"],tool:"bazel"}})).toEqual(mine);
});

test("The diagnosis switch appends one line per decision, writes nothing without the switch and never fails a run",()=>{
 const directory=mkdtempSync(join(tmpdir(),"keyparts-")),file=join(directory,"parts.jsonl");
 try{
  appendBazelPlanKeyPartsV1(undefined,"k",parts);appendBazelPlanKeyPartsV1("","k",parts);
  expect(existsSync(file)).toBe(false);
  appendBazelPlanKeyPartsV1(file,bazelPlanKeyV1(parts),parts,5);appendBazelPlanKeyPartsV1(file,"second",{...parts,graph:"f"},6);
  const rows=readFileSync(file,"utf8").trim().split("\n").map(line=>JSON.parse(line) as {at:number;key:string;parts:Record<string,string>});
  expect(rows).toHaveLength(2);
  expect(rows[0]).toMatchObject({at:5,key:bazelPlanKeyV1(parts),parts:bazelPlanKeyPartsV1(parts)});
  expect(rows[1]!.parts.graph).not.toBe(rows[0]!.parts.graph);
  expect(()=>appendBazelPlanKeyPartsV1(join(directory,"missing","x.jsonl"),"k",parts)).not.toThrow();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("The reuse decision of the Bazel script hands the very parts it hashes to the switch",()=>{
 const text=readFileSync(join(import.meta.dirname,"..","..","..","📜️script.ts"),"utf8");
 expect(text).toMatch(/const keyParts=\{version:1,root,workspace,[^;]*preload:hash\(preload\)\},key=bazelPlanKeyV1\(keyParts\);appendBazelPlanKeyPartsV1\(process\.env\.SEMIO_REPO_PLAN_KEY_PARTS,key,keyParts\);/u);
});
