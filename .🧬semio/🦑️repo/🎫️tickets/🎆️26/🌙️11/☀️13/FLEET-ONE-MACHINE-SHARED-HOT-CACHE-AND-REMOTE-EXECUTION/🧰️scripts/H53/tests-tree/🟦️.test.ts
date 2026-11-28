/** 🌲️ Contract: the warden state names the heaviest processes of the repository (`wardenTreeV1`), the same rows an independent Python oracle selects from the same table, and each row is valid against the schema.
 * @see ./🐍️.py */
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {wardenTreeV1,type HostWardenRowV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

const root="/work/semio",row=(pid:number,parent:number,command:string,memoryKilobytes:number,cpuSeconds?:number):HostWardenRowV1=>({pid,parent,group:parent,memoryKilobytes,stopped:false,elapsedSeconds:pid%97,...cpuSeconds===undefined?{}:{cpuSeconds},start:`s${pid}`,command});
const table=[
 row(1,0,"/sbin/init",9000,5),
 row(10,1,`bun ${root}/door/script.ts repo tools setup all`,220000,40),
 row(11,10,`bun ${root}/door/script.ts repo capacity run`,300000,900),
 row(12,11,"cargo install trunk --locked",1500000,22000),
 row(13,12,"rustc --crate-name syn",400000,3000),
 row(14,12,"rustc --crate-name tokio",400000,3000),
 row(20,1,"/usr/bin/firefox",3000000,9000),
 row(21,20,"firefox-content",800000,2000),
 row(30,1,`/home/x/.bun/bin/bun ${root}/graph/pool.ts`,150000),
 row(31,30,"bun child",120000),
 row(40,99,"orphan below a missing parent",1000),
 row(50,1,"cargo install wasm-bindgen-cli --locked",1200000,40000),
 row(51,50,"cc -c openssl.c",90000,8000),
];
const run=(limit:number,roots:number[]=[])=>{
 const input=Bun.spawnSync([testPythonV1()??Bun.which("python3")??"python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),String(limit)],{stdin:new TextEncoder().encode(JSON.stringify({rows:table,workspaces:[root],roots}))});
 expect(input.exitCode).toBe(0);
 return JSON.parse(input.stdout.toString()) as unknown[];
};

test("the tree is the repository processes by cpu then memory, as the independent oracle selects them",()=>{
 for(const limit of[3,12]){
  const ours=wardenTreeV1(table,[root],limit);
  expect(ours.map(each=>each.pid)).toEqual((run(limit) as {pid:number}[]).map(each=>each.pid));
  const rooted=wardenTreeV1(table,[root],limit,[50]);
  expect(rooted.map(each=>each.pid)).toEqual((run(limit,[50]) as {pid:number}[]).map(each=>each.pid));
  expect(ours.length).toBeLessThanOrEqual(limit);
 }
 const all=wardenTreeV1(table,[root],12).map(each=>each.pid);
 expect(all).toEqual([12,13,14,11,10,30,31]);
 expect(all).not.toContain(20);
 expect(all).not.toContain(40);
 expect(all).not.toContain(50);
 expect(wardenTreeV1(table,[root],12,[50]).map(each=>each.pid)).toEqual([50,12,51,13,14,11,10,30,31].sort((a,b)=>{const cpu=(pid:number)=>table.find(each=>each.pid===pid)!.cpuSeconds??-1;return cpu(b)-cpu(a)||a-b;}));
});

test("a row is valid against the schema, a long command is cut, a loop of parents ends",()=>{
 const long=wardenTreeV1([row(5,5,`${root} ${"x".repeat(500)}`,2048,1)],[root]);
 expect(long).toHaveLength(1);
 expect(long[0]!.command).toHaveLength(240);
 expect(long[0]!.megabytes).toBe(2);
 for(const each of wardenTreeV1(table,[root],12))expect(validateJsonSchemaSubset({$ref:"#/definitions/TreeRow"},each,schema as never)).toEqual([]);
 expect(wardenTreeV1([row(7,8,"a",1),row(8,7,"b",1)],[root])).toEqual([]);
 expect(wardenTreeV1(table,[])).toEqual([]);
 const shown=wardenTreeV1([row(60,1,`/home/x/.cache/semio/runtimes/bun/${"a".repeat(64)}/bun ${root}/door/script.ts repo tools setup all`,1024,1)],[root]);
 expect(shown[0]!.command).toBe("bun ~/door/script.ts repo tools setup all");
});
