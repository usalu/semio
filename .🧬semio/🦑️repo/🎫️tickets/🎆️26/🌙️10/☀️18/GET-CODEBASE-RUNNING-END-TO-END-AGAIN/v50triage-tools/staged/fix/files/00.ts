/** 👷️ Contract: the hermetic runner serves the JSON worker protocol for output-free actions with a service. Staged inputs persist between requests and follow every edit, removal and dependency snapshot change by their stat signature; the service module is imported once and keeps its state; a failure or a cancellation drops the staged state; the loopback service refuses a wrong token and a path outside the execution root; the same manifest run as an ordinary process through a flagfile gives the same verdict.
 * @see ../../🏭️runner/🍞️bun/🟦️.ts — the runner */
import {afterEach,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {BAZEL_HERMETIC_RUNNER} from "../../🏭️runner/🟦️.ts";

const roots:string[]=[],servers:{kill():void}[]=[];
afterEach(()=>{for(const server of servers.splice(0))server.kill();for(const root of roots.splice(0))rmSync(root,{recursive:true,force:true});});

const alias="a/0123456789",service=`let served = 0, released = 0;
export function createHermeticServiceV1(options) {
  return {
    async handle(request, signal) { served++; return { served, released, project: request.project, resolved: typeof options.resolve }; },
    release() { released++; }
  };
}
`;
const client=`import { connect } from "node:net";
import { existsSync, readFileSync } from "node:fs";
const [host, port, token] = (process.env.SEMIO_TEST_SERVICE ?? "").split(":");
const root = process.env.SEMIO_WORKSPACE_ROOT;
const mode = process.env.SEMIO_TEST_MODE ?? "";
const a = existsSync(root + "/lib/a.txt") ? readFileSync(root + "/lib/a.txt", "utf8").trim() : "absent";
if (mode === "fail") process.exit(3);
if (mode === "sleep") await new Promise(resolve => setTimeout(resolve, 30000));
const reply = await new Promise(resolve => {
  if (!port) { resolve("{}"); return; }
  const socket = connect(Number(port), host);
  let text = "";
  socket.setEncoding("utf8");
  socket.on("data", chunk => text += chunk);
  socket.on("end", () => resolve(text.trim()));
  socket.on("error", error => resolve(JSON.stringify({ ok: false, message: String(error) })));
  const cwd = mode === "escape" ? "/" : root + "/lib";
  socket.write(JSON.stringify({ token: mode === "token" ? "wrong" : token, request: { cwd, project: "p" } }) + "\\n");
});
console.log("REPLY " + JSON.stringify({ a, root, reply: JSON.parse(reply), mode }));
`;

type Files=Record<string,string>;
function sandbox(files:Files,manifest:Record<string,unknown>={}):string{
 const root=mkdtempSync(join(tmpdir(),"hermetic-worker-"));roots.push(root);
 for(const[name,text]of Object.entries(files)){mkdirSync(dirname(join(root,alias,name)),{recursive:true});writeFileSync(join(root,alias,name),text);}
 writeFileSync(join(root,"hermetic-runner.mjs"),BAZEL_HERMETIC_RUNNER);
 const scratch=Object.fromEntries(["darwin","linux","win32"].map(platform=>[platform,{base:join(root,"w"),workDigits:8,workspace:"r",separator:"/"}]));
 writeFileSync(join(root,"manifest.json"),JSON.stringify({version:1,id:"fixture:check",project:"fixture",target:"check",root:"lib",args:[],commands:["bun {projectRoot}/client.ts"],outputs:[],inputs:Object.keys(files).map(name=>"lib/"+name),aliases:{lib:alias},scratch,service:{module:"lib/svc.ts",environment:"SEMIO_TEST_SERVICE",idleMilliseconds:0},...manifest}));
 return root;
}

class Worker{
 readonly process;readonly #lines;#id=0;
 constructor(root:string,env:Record<string,string>={}){
  this.process=Bun.spawn([process.execPath,"hermetic-runner.mjs","--persistent_worker"],{cwd:root,env,stdin:"pipe",stdout:"pipe",stderr:"pipe"});servers.push(this.process);
  this.#lines=(async function*(stream:ReadableStream<Uint8Array>){const decoder=new TextDecoder();let buffer="";for await(const chunk of stream){buffer+=decoder.decode(chunk,{stream:true});for(let end=buffer.indexOf("\n");end>=0;end=buffer.indexOf("\n")){yield buffer.slice(0,end);buffer=buffer.slice(end+1);}}})(this.process.stdout);
 }
 post(value:unknown):void{this.process.stdin.write(JSON.stringify(value)+"\n");this.process.stdin.flush();}
 async next():Promise<{exitCode:number;output:string;requestId:number;wasCancelled?:boolean}>{const row=await this.#lines.next();if(row.done)throw new Error("worker ended: "+await new Response(this.process.stderr).text());return JSON.parse(row.value);}
 async request(args:string[]=["manifest.json","out"]):Promise<{exitCode:number;output:string;reply?:any;requestId:number}>{
  const requestId=++this.#id;this.post({arguments:args,requestId});const response=await this.next();expect(response.requestId).toBe(requestId);
  const line=response.output.split("\n").find(row=>row.startsWith("REPLY "));return {...response,...(line?{reply:JSON.parse(line.slice(6))}:{})};
 }
}

const base:Files={"a.txt":"one\n","svc.ts":service,"client.ts":client};

test("a worker keeps its private root, its staged inputs and its service between requests and follows every input change",async()=>{
 const root=sandbox(base),worker=new Worker(root),first=await worker.request();
 expect(first.exitCode,first.output).toBe(0);expect(first.reply.a).toBe("one");expect(first.reply.reply).toEqual({ok:true,result:{served:1,released:0,project:"p",resolved:"function"}});
 const staged=join(first.reply.root,"lib/a.txt"),inode=statSync(staged).ino,second=await worker.request();
 expect(second.reply.root).toBe(first.reply.root);expect(second.reply.reply.result.served).toBe(2);expect(statSync(staged).ino,"an unchanged input is not staged again").toBe(inode);
 expect(first.reply.root).toMatch(/\/w\/[0-9a-f]{8}\/r$/u);
 writeFileSync(join(root,alias,"a.txt"),"two plus\n");
 const edited=await worker.request();expect(edited.reply.a).toBe("two plus");expect(statSync(staged).ino,"an edited input is staged again").not.toBe(inode);
 const manifest=JSON.parse(readFileSync(join(root,"manifest.json"),"utf8"));manifest.inputs=manifest.inputs.filter((name:string)=>name!=="lib/a.txt");writeFileSync(join(root,"manifest2.json"),JSON.stringify(manifest));
 const removed=await worker.request(["manifest2.json","out2"]);expect(removed.reply.a).toBe("absent");expect(existsSync(staged)).toBe(false);
 const snapshot=join(root,"dep");mkdirSync(join(snapshot,"files"),{recursive:true});const id="a".repeat(32);writeFileSync(join(snapshot,"files",id),"from a dependency\n");
 writeFileSync(join(snapshot,"snapshot.json"),JSON.stringify({version:1,roots:["lib/a.txt"],entries:[{path:"lib/a.txt",kind:"file",mode:420,file:id,sha256:"d1"}]}));
 const restored=await worker.request(["manifest2.json","out2",snapshot]);expect(restored.reply.a).toBe("from a dependency");
 writeFileSync(join(snapshot,"files",id),"changed dependency\n");writeFileSync(join(snapshot,"snapshot.json"),JSON.stringify({version:1,roots:["lib/a.txt"],entries:[{path:"lib/a.txt",kind:"file",mode:420,file:id,sha256:"d2"}]}));
 const changed=await worker.request(["manifest2.json","out2",snapshot]);expect(changed.reply.a).toBe("changed dependency");
 const dropped=await worker.request(["manifest2.json","out2"]);expect(dropped.reply.a,"a dependency output that is no longer an input leaves the root").toBe("absent");
 const output=JSON.parse(readFileSync(join(root,"out2/snapshot.json"),"utf8"));expect(output).toEqual({version:1,roots:[],entries:[]});
 expect(dropped.reply.reply.result.served).toBe(7);
 worker.process.stdin.end();expect(await worker.process.exited).toBe(0);
});

test("a failing command answers exit code 1 with its output and drops the staged state, a good request recovers",async()=>{
 const root=sandbox(base),worker=new Worker(root),first=await worker.request(),staged=join(first.reply.root,"lib/a.txt");
 expect(existsSync(staged)).toBe(true);
 const manifest=JSON.parse(readFileSync(join(root,"manifest.json"),"utf8"));manifest.env={SEMIO_TEST_MODE:"fail"};writeFileSync(join(root,"failing.json"),JSON.stringify(manifest));
 const failed=await worker.request(["failing.json","out"]);expect(failed.exitCode).toBe(1);expect(failed.output).toContain("command failed (3)");expect(existsSync(staged),"the private root is removed").toBe(false);
 const again=await worker.request();expect(again.exitCode,again.output).toBe(0);expect(again.reply.reply.result.served,"the service survives a failed request").toBe(2);expect(existsSync(staged)).toBe(true);
});

test("a cancel request stops the running commands and answers as cancelled",async()=>{
 const root=sandbox(base),worker=new Worker(root),manifest=JSON.parse(readFileSync(join(root,"manifest.json"),"utf8"));manifest.env={SEMIO_TEST_MODE:"sleep"};writeFileSync(join(root,"sleeping.json"),JSON.stringify(manifest));
 worker.post({arguments:["sleeping.json","out"],requestId:41});
 await Bun.sleep(1500);const started=Date.now();worker.post({cancel:true,requestId:41});
 const response=await worker.next();expect(response.requestId).toBe(41);expect(response.wasCancelled).toBe(true);expect(response.exitCode).toBe(1);expect(Date.now()-started).toBeLessThan(10000);
 const next=await worker.request();expect(next.exitCode,next.output).toBe(0);
},60000);

test("the loopback service refuses a wrong token and a path outside the execution root",async()=>{
 const root=sandbox(base),worker=new Worker(root);
 for(const mode of["token","escape"]){
  const manifest=JSON.parse(readFileSync(join(root,"manifest.json"),"utf8"));manifest.env={SEMIO_TEST_MODE:mode};writeFileSync(join(root,mode+".json"),JSON.stringify(manifest));
  const answer=await worker.request([mode+".json","out"]);expect(answer.exitCode,answer.output).toBe(0);expect(answer.reply.reply.ok).toBe(false);
  expect(answer.reply.reply.message).toMatch(mode==="token"?/token-refused/u:/path-escapes-root/u);
 }
});

test("a worker refuses actions that declare outputs or no aliased inputs, and idle time releases the service",async()=>{
 const root=sandbox(base),worker=new Worker(root),outputs=JSON.parse(readFileSync(join(root,"manifest.json"),"utf8"));outputs.outputs=["{projectRoot}/out"];writeFileSync(join(root,"outputs.json"),JSON.stringify(outputs));
 const refused=await worker.request(["outputs.json","out"]);expect(refused.exitCode).toBe(1);expect(refused.output).toContain("declares aliased inputs and no outputs");
 const idle=JSON.parse(readFileSync(join(root,"manifest.json"),"utf8"));idle.service.idleMilliseconds=200;writeFileSync(join(root,"idle.json"),JSON.stringify(idle));
 const first=await worker.request(["idle.json","out"]);expect(first.reply.reply.result.released).toBe(0);
 await Bun.sleep(900);const second=await worker.request(["idle.json","out"]);expect(second.reply.reply.result.released,"the service was released while idle").toBe(1);expect(second.reply.reply.result.served).toBe(first.reply.reply.result.served+1);
});

test("the same manifest through a flagfile as an ordinary process gives the verdict and the snapshot of a worker request",async()=>{
 const root=sandbox(base);writeFileSync(join(root,"flagfile"),"manifest.json\nout\n");
 const run=Bun.spawnSync([process.execPath,"hermetic-runner.mjs","@flagfile"],{cwd:root,env:{},stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);expect(JSON.parse(readFileSync(join(root,"out/snapshot.json"),"utf8"))).toEqual({version:1,roots:[],entries:[]});
 expect(existsSync(join(root,"w")) && statSync(join(root,"w")).isDirectory() ? (await import("node:fs")).readdirSync(join(root,"w")).length : 0,"an ordinary process removes its private root").toBe(0);
});

/** 🛎️ Contract: service actions are routed by the table and by nothing else. The table is closed and consistent (the class names a declared engine, the engine module exists and exports the service factory the runner imports, the provider reads the same environment name), the flags put the worker strategy between the remote pool and the sandbox and agree with an independent Python implementation, the manifest and the BUILD carry the service only for a class that has one, and the Starlark rule asks for workers only when told to. */
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import classSchema from "../../../../../🏃️execution/🧬️schema/🔒️hermetic-classes.json";
import manifestSchema from "../../🧬️schema/🔣️.json";
import {HERMETIC_SERVICES_V1,HERMETIC_TABLE_V1,hermeticServiceV1,hermeticStrategiesV1} from "../../../../../🏃️execution/🔒️hermetic/🟦️.ts";
import * as api from "../../🟦️.ts";
import {renderBazelBuildV1} from "../../../📜️script.ts";

const python=testPythonV1(),repository=join(import.meta.dir,"../../../../../../../../../../..");

test("the services table is closed and the class, the engine module, the provider and the runner agree",async()=>{
 const ajv=new Ajv({strict:true}).addSchema(classSchema,"classes"),admit=ajv.compile({$ref:"classes#"});
 expect(admit(HERMETIC_TABLE_V1),JSON.stringify(admit.errors)).toBe(true);
 const engine=HERMETIC_TABLE_V1.services.engines["typescript-check"];
 for(const broken of[{...HERMETIC_TABLE_V1,services:{...HERMETIC_TABLE_V1.services,extra:1}},{...HERMETIC_TABLE_V1,services:{...HERMETIC_TABLE_V1.services,instances:0}},{...HERMETIC_TABLE_V1,services:{...HERMETIC_TABLE_V1.services,engines:{x:{...engine,environment:"PATH"}}}},{...HERMETIC_TABLE_V1,services:{...HERMETIC_TABLE_V1.services,engines:{x:{...engine,idleMilliseconds:-1}}}},{...HERMETIC_TABLE_V1,services:undefined}])expect(admit(broken)).toBe(false);
 const service=hermeticServiceV1("ts-typecheck")!;
 expect(service).toEqual({id:"typescript-check",...engine});expect(hermeticServiceV1("ts-test")).toBeUndefined();
 for(const row of HERMETIC_TABLE_V1.classes){const name=(row as {service?:string}).service;if(name)expect(Object.keys(HERMETIC_TABLE_V1.services.engines),row.id).toContain(name);}
 expect(existsSync(join(repository,service.module)),service.module).toBe(true);
 expect(HERMETIC_TABLE_V1.runtime.inputs,"the engine module and the table are inputs of every hermetic action").toEqual(expect.arrayContaining([service.module,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🔣️.json"]));
 const module=await import(join(repository,service.module));expect(typeof module.createHermeticServiceV1).toBe("function");
 const built=module.createHermeticServiceV1({resolve:()=>({})});expect(typeof built.handle).toBe("function");expect(typeof built.release).toBe("function");expect(built.release()).toBe(0);
 const provider=readFileSync(join(repository,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🔌️providers/🔷️typescript/🟦️.ts"),"utf8");
 expect(provider).toContain('table.services.engines["typescript-check"].environment');
 expect(BAZEL_HERMETIC_RUNNER).toContain("manifest.service.environment");expect(BAZEL_HERMETIC_RUNNER).toContain("createHermeticServiceV1");
});

test.skipIf(!python)("service flags: worker between the remote pool and the sandbox, one instance bound, none without workers; agrees with Python",()=>{
 const oracle="import json,sys\nstrategies,workers,remote,mnemonic,instances=json.loads(sys.stdin.read())\nnames=[('local' if n=='isolated-root' else n) for n in strategies if remote or n!='remote']\nlst=[n for n in names if n=='remote']+(['worker'] if workers else [])+[n for n in names if n!='remote']\nflags=['--strategy=%s=%s'%(mnemonic,','.join(lst))]\nif workers: flags.append('--worker_max_instances=%s=%d'%(mnemonic,instances))\nprint(json.dumps(flags))";
 for(const platform of["darwin","linux","win32"])for(const workers of[true,false])for(const remote of[true,false]){
  const actual=api.bazelServiceFlagsV1({host:{platform},workers,remote}),expected=Bun.spawnSync([python!,"-I","-c",oracle],{stdin:new TextEncoder().encode(JSON.stringify([(HERMETIC_TABLE_V1.strategies as Record<string,string[]>)[platform],workers,remote,HERMETIC_SERVICES_V1.mnemonic,HERMETIC_SERVICES_V1.instances])),stdout:"pipe"});
  expect([...actual],`${platform} workers=${workers} remote=${remote}`).toEqual(JSON.parse(expected.stdout.toString()));
  expect(actual[0]!.includes(",worker,")||actual[0]!.endsWith("=worker")||actual[0]!.includes("=worker,"),"worker listed").toBe(workers);
  expect(hermeticStrategiesV1(platform).length).toBeGreaterThan(0);
 }
 expect(api.bazelServiceFlagsV1({host:{platform:"darwin"},workers:true,remote:true})[0]).toBe("--strategy=RepoHermeticService=remote,worker,sandboxed,local");
 expect(api.bazelServiceFlagsV1({host:{platform:"linux"},workers:true,remote:false})).toEqual(["--strategy=RepoHermeticService=worker,sandboxed,local","--worker_max_instances=RepoHermeticService=2"]);
});

test("the manifest carries the service closed, the BUILD asks for a worker only for service actions and the rule requests workers only then",()=>{
 const ajv=new Ajv({strict:true}).addSchema(manifestSchema,"component"),admit=ajv.compile({$ref:"component#/definitions/Manifest"}),task={id:"p:typecheck",project:"p",target:"typecheck",label:"t_0",dependencies:[],args:[],root:"proj",definition:{execution:{kind:"command",command:"repo ./📜️script.ts typecheck",cwd:"proj"},outputs:[]}} as any;
 const service={module:"m/🟦️.ts",environment:"SEMIO_HERMETIC_X",idleMilliseconds:1000},manifest=api.bazelHermeticManifestV1(task,[],{inputs:["proj/a.ts"],class:"ts-typecheck",providers:{},service});
 expect(admit(manifest),JSON.stringify(admit.errors)).toBe(true);expect(manifest.service).toEqual(service);
 expect("service" in api.bazelHermeticManifestV1(task,[],{inputs:["proj/a.ts"],class:"ts-typecheck",providers:{}})).toBe(false);
 for(const extra of[{...manifest,service:{...service,extra:1}},{...manifest,service:{...service,environment:"PATH"}},{...manifest,service:{module:"m"}}])expect(admit(extra)).toBe(false);
 const action={id:"p:typecheck",label:"t_0",kind:"hermetic" as const,restore:[],cache:{cache:true}} as any,build=(service:boolean)=>renderBazelBuildV1({actions:[service?{...action,service:true}:action],groups:new Map(),bun:"/opt/bun",sourceIdentity:"a".repeat(64),extension:""});
 expect(build(true)).toContain("    worker = True,");expect(build(true)).toContain('    mnemonic = "RepoHermeticService",');expect(build(false)).not.toContain("worker");expect(build(false)).not.toContain("mnemonic");
 const starlark=api.BAZEL_HERMETIC_STARLARK;
 for(const needle of['requirements.update({"supports-workers": "1", "requires-worker-protocol": "json", "supports-worker-cancellation": "1"})','request.use_param_file("@%s", use_always = True)','if ctx.attr.worker:','"worker": attr.bool(default = False)','"mnemonic": attr.string()','requirements = {"block-network": "1"}'])expect(starlark,needle).toContain(needle);
 expect(starlark.indexOf("if ctx.attr.worker:"),"the flagfile is requested only for workers").toBeLessThan(starlark.indexOf("request.use_param_file"));
});

test("the hermeticity judge accepts a worker spawn only for a service action",()=>{
 const host=api.bazelExecutionPlatformByIdV1("darwin-arm64"),tool="tools/bun/darwin-arm64/bun",inputs=["hermetic-runner.mjs","t_1.json"],environmentVariables=Object.entries(manifestSchema.definitions.Contract.const.actionEnvironment).map(([name,value])=>({name,value})),properties=Object.entries(host.properties).map(([name,value])=>({name,value}));
 const spawn={mnemonic:"RepoHermeticService",runner:"worker",commandArgs:[tool,"hermetic-runner.mjs","--persistent_worker"],environmentVariables,inputs:[...inputs.map(path=>path==="hermetic-runner.mjs"?{path,isTool:true}:{path}),{path:tool,isTool:true},{path:"bazel-out/x/t_1.snapshot-0.params"}],platform:{properties},targetLabel:"//:t_1"};
 expect(api.bazelHermeticViolationsV1(spawn,{inputs,tool,properties:host.properties,service:true})).toEqual([]);
 expect(api.bazelHermeticViolationsV1(spawn,{inputs,tool,properties:host.properties})).toEqual(["runner: worker","undeclared input: bazel-out/x/t_1.snapshot-0.params","tool inputs: hermetic-runner.mjs,tools/bun/darwin-arm64/bun"]);
 expect(api.bazelHermeticViolationsV1({...spawn,inputs:[...spawn.inputs,{path:"bazel-out/x/other.params"}]},{inputs,tool,properties:host.properties,service:true})).toEqual(["undeclared input: bazel-out/x/other.params"]);
});
