/** 🧾️ Contract: a persisted inference record header is canonical JSON (text equals the serialisation of its own parse) with the closed key order digest, resultSha256, authorityObservations, observations, carries the file fact of what the computation read, is reused unchanged while the file is unchanged and is rewritten when it changes. The scenario lives in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const observation=await import("../../🟨️.mjs") as any;

test("record header fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);});

test("a persisted record header is canonical, ordered, complete and tracks the observed file",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"record-header-")),file=join(root,fixture.file),cachePath=join(root,"np","record.json");
 try{
  const settle=()=>new Promise(accept=>setTimeout(accept,2200));
  writeFileSync(file,fixture.before);await settle();
  let computations=0;const compute=async()=>{computations++;return {text:observation.observedFs.readFileSync(file,"utf8")};};
  const first=await observation.observeInference({cachePath,key:["header"]},compute),header=observation.inferenceHeaderLine(cachePath);
  expect(first).toEqual({text:fixture.before});
  expect(header).not.toBeNull();
  const parsed=JSON.parse(header);
  expect(Object.keys(parsed)).toEqual(fixture.headerKeys);
  expect(JSON.stringify(parsed)).toBe(header);
  expect(parsed.resultSha256).toBe(createHash("sha256").update(JSON.stringify(first)).digest("hex"));
  expect(parsed.observations.some((row:any)=>row.kind==="file"&&row.path===file&&row.fact.sha256===createHash("sha256").update(fixture.before).digest("hex"))).toBe(true);
  const again=await observation.observeInference({cachePath,key:["header"]},compute);
  expect(again).toEqual(first);
  writeFileSync(file,fixture.after);await settle();
  const changed=await observation.observeInference({cachePath,key:["header"]},compute);
  expect(changed).toEqual({text:fixture.after});
  expect(observation.inferenceHeaderLine(cachePath)).not.toBe(header);
  expect(computations).toBeGreaterThanOrEqual(2);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
