import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {BAZEL_FAILURE_TAIL_POLICY_V1,bazelFailureTailFileV1,clearBazelFailureTailsV1,createBazelFailureTailV1,parseBazelFailureTailV1,readBazelFailureTailsV1,renderBazelFailureTailV1,reportBazelFailureTailsV1,writeBazelFailureTailV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const linesOf=(item:{lines?:string[];generate?:{lines:number;template:string}}):string[]=>item.lines??Array.from({length:item.generate!.lines},(_,n)=>item.generate!.template.replace("{n}",String(n)));
const run=(maximumBytes:number,lines:readonly string[])=>{const tail=createBazelFailureTailV1(maximumBytes);for(const line of lines)tail.push(line);return tail.result();};

test("The tail keeps the last bytes of the output and never cuts a UTF-8 sequence",()=>{
 for(const item of fixture.tails){
  const actual=run(item.maximumBytes,linesOf(item));
  if("expect" in item)expect<unknown>({name:item.name,...actual}).toEqual({name:item.name,...(item as {expect:object}).expect});
  expect(actual.bytes).toBeLessThanOrEqual(item.maximumBytes);
  expect(Buffer.byteLength(actual.text)).toBe(actual.bytes);
 }
 expect(()=>createBazelFailureTailV1(0)).toThrow();
 expect(()=>createBazelFailureTailV1(BAZEL_FAILURE_TAIL_POLICY_V1.maximumTailBytes+1)).toThrow();
});

test("The Python oracle computes the same tails from the same fixture",()=>{
 const oracle=join(dirname(fileURLToPath(import.meta.url)),"🧮️resources","🐍️oracle","🐍️.py"),result=spawnSync(process.platform==="win32"?"python":"python3",["-I",oracle],{input:JSON.stringify(fixture.tails.map(item=>({maximumBytes:item.maximumBytes,...("lines" in item?{lines:item.lines}:{generate:(item as {generate:object}).generate})}))),encoding:"utf8",maxBuffer:1<<26});
 expect(result.status).toBe(0);
 const expected=JSON.parse(result.stdout) as {text:string;bytes:number;droppedBytes:number}[];
 fixture.tails.forEach((item,index)=>expect(run(item.maximumBytes,linesOf(item))).toEqual(expected[index]!));
});

test("A failed task's tail is persisted, read back and rendered in English and German",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-tail-"));
 try{
  for(const task of fixture.files.tasks){const tail=createBazelFailureTailV1(8);for(const line of ["first line "+task,"last "+task.length])tail.push(line);await writeBazelFailureTailV1(directory,task,tail);}
  const rows=await readBazelFailureTailsV1(directory);
  expect(rows.map(row=>row.task)).toEqual([...fixture.files.tasks].sort());
  for(const row of rows){expect(row.bytes).toBeLessThanOrEqual(8);expect(row.droppedBytes).toBeGreaterThan(0);expect(row.file.startsWith(directory)).toBe(true);}
  expect(readdirSync(directory).every(name=>name.endsWith(BAZEL_FAILURE_TAIL_POLICY_V1.fileSuffix)&&!name.includes("/")&&!name.includes("\\"))).toBe(true);
  const printed:string[]=[];expect(await reportBazelFailureTailsV1(directory,"de",text=>printed.push(text))).toBe(3);
  expect(printed.every(text=>text.startsWith("[bazel] Task "))).toBe(true);
  await clearBazelFailureTailsV1(directory);
  expect(await readBazelFailureTailsV1(directory)).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("Distinct task ids never share a tail file and a damaged file is refused",()=>{
 expect(new Set(fixture.files.names.map(bazelFailureTailFileV1)).size).toBe(fixture.files.names.length);
 expect(bazelFailureTailFileV1("@a:b")).toMatch(/^[A-Za-z0-9._-]+\.tail\.log$/u);
 const header=JSON.stringify({version:1,task:"t",bytes:3,droppedBytes:0});
 expect(parseBazelFailureTailV1("f",Buffer.from(header+"\nabc")).text).toBe("abc");
 expect(()=>parseBazelFailureTailV1("f",Buffer.from(header+"\nabcd"))).toThrow();
 expect(()=>parseBazelFailureTailV1("f",Buffer.from("no header"))).toThrow();
 expect(()=>parseBazelFailureTailV1("f",Buffer.from(JSON.stringify({version:2,task:"t",bytes:0,droppedBytes:0})+"\n"))).toThrow();
});

test("The failure report is authored in English and German",()=>{
 for(const item of fixture.rendering)expect(renderBazelFailureTailV1(item.row,item.language as "en"|"de")).toBe(item.expect);
});
