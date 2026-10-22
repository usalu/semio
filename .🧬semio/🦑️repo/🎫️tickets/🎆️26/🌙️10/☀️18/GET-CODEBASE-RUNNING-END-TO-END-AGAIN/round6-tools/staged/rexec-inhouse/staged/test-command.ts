import {expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../../🏃️execution/🧬️schema/🌐️remote-execution.json";
import catalog from "../../../⚙️operations/🔣️catalog.json";
import policy from "../../../🔣️policy.json";
import {connectGrpcV1} from "../../📡️wire/🚪️grpc/🟦️.ts";
import {encodeProtobufV1,decodeProtobufV1,REAPI_V1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import texts from "../../📖️texts/🔣️.json";
import {readRemoteExecutionPolicyV1,requireRemoteExecutionShapeV1} from "../../🟦️.ts";
import {eventuallyV1,startClusterV1} from "../../🧪️tests/🧰️cluster/🟦️.ts";
import {parseRemoteExecutionArgumentsV1,REMOTE_EXECUTION_OPTIONS_V1,remoteExecutionStatusV1,remoteExecutionTextV1,runRemoteExecutionCommandV1,type RemoteExecutionCommandContextV1} from "../🟦️.ts";

const definitions=schema.definitions as Record<string,unknown>,here=import.meta.dir,ajv=new Ajv({strict:true,allErrors:true,validateFormats:false});
const independent=(name:string,value:unknown):boolean=>{const validate=ajv.compile({$ref:`#/definitions/${name}`,definitions} as never);return validate(value) as boolean;};
const repository=(name:string,value:unknown):boolean=>validateJsonSchemaSubset({$ref:`#/definitions/${name}`},value,schema as never).length===0;
const sha=(data:Uint8Array|string)=>createHash("sha256").update(data).digest("hex");

test("the schema, the authored policy and the message catalogue agree between the repository validator and Ajv, which also refuse every drifted variant alike",()=>{
 const policyValue=readRemoteExecutionPolicyV1();
 for(const [name,value] of [["Policy",policyValue],["TextCatalogue",texts]] as const){expect(independent(name,value),name).toBe(true);expect(repository(name,value),name).toBe(true);}
 const clone=()=>JSON.parse(JSON.stringify(policyValue)) as any;
 const drifts:((value:any)=>void)[]=[
  value=>{delete value.server.port;},value=>{value.server.port=70000;},value=>{value.server.extra=1;},value=>{value.worker.materialize="link";},value=>{value.worker.sandbox="docker";},
  value=>{value.environment.endpoint="endpoint";},value=>{value.server.defaultInstance="Main";},value=>{value.version=2;},value=>{value.worker.concurrency=-1;},value=>{delete value.environment;},
 ];
 for(const [index,drift] of drifts.entries()){const value=clone();drift(value);expect(independent("Policy",value),`drift ${index}`).toBe(false);expect(repository("Policy",value),`drift ${index}`).toBe(false);}
 const catalogue=JSON.parse(JSON.stringify(texts)) as any;catalogue.messages["error.syntax"].de="";expect(independent("TextCatalogue",catalogue)).toBe(false);expect(repository("TextCatalogue",catalogue)).toBe(false);
 const registration={version:1,name:"w",host:"h",platform:"darwin-arm64",properties:[{name:"OSFamily",value:"macos"}],concurrency:2,sandbox:"isolated-root",runtime:"bun 1"};
 for(const [name,value] of [["WorkerRegistration",registration],["Heartbeat",{version:1,running:["a.1"]}],["TaskResult",{version:1,outcome:"completed",status:{code:0,message:""},actionResult:"AAEC"}]] as const){expect(independent(name,value),name).toBe(true);expect(repository(name,value),name).toBe(true);}
 for(const [name,value] of [["WorkerRegistration",{...registration,sandbox:"chroot"}],["WorkerRegistration",{...registration,extra:1}],["Heartbeat",{version:1}],["TaskResult",{version:1,outcome:"done",status:{code:0,message:""},actionResult:null}],["TaskResult",{version:1,outcome:"completed",status:{code:0,message:""},actionResult:"!!"}]] as const){expect(independent(name,value),name).toBe(false);expect(repository(name,value),name).toBe(false);}
});

test("the catalogue row lists exactly the options the command accepts, every flag is defined for the operation, and the examples start with repo",()=>{
 const operation=(catalog.operations as any[]).find(row=>row.name==="remote-execution");
 expect(operation.synopsis).toBe("remote-execution serve|status|workers");expect(operation.subcommands.map((row:any)=>row.name)).toEqual(["serve","status","workers"]);
 const accepted=(name:"serve"|"status"|"workers")=>[...REMOTE_EXECUTION_OPTIONS_V1[name].valued,...REMOTE_EXECUTION_OPTIONS_V1[name].plain].filter(flag=>flag!=="--lang");
 for(const subcommand of operation.subcommands)expect([...subcommand.flags].sort(),subcommand.name).toEqual(accepted(subcommand.name).sort());
 expect([...operation.flags].sort()).toEqual([...new Set(["serve","status","workers"].flatMap(name=>accepted(name as "serve")))].sort());
 const defined=new Map((catalog.flags as any[]).map(row=>[row.name,row]));
 for(const flag of operation.flags){const row=defined.get(flag);expect(row,flag).toBeDefined();expect(row.operations,flag).toContain("remote-execution");expect(row.description.en.length).toBeGreaterThan(10);expect(row.description.de.length).toBeGreaterThan(10);}
 for(const row of catalog.flags as any[])if(!operation.flags.includes(row.name))expect(row.operations??[]).not.toContain("remote-execution");
 for(const example of operation.examples)expect(example.command.startsWith("repo remote-execution ")).toBe(true);
 expect(operation.notes.every((note:any)=>note.en&&note.de)).toBe(true);
 expect(JSON.stringify(operation)).not.toMatch(/\bdocker\b|Buildbarn|\bcompose\b|\bcontainers?\b/iu);
});

test("every message the command can print exists in English and German with the same placeholders, and no unused message is left",()=>{
 const source=readFileSync(join(here,"..","🟦️.ts"),"utf8"),keys=new Set<string>();
 for(const match of source.matchAll(/(?:say|both)\("([a-z.-]+)"/gu))keys.add(match[1]!);
 for(const match of source.matchAll(/"@([a-z.-]+)"/gu))keys.add(match[1]!);
 for(const stage of["sandbox","registered","started","finished","cancelled","reconnecting","stopped"])keys.add(`workers.${stage}`);
 const catalogue=texts.messages as Record<string,{en:string;de:string}>;
 for(const key of keys)expect(catalogue[key],key).toBeDefined();
 for(const [key,row] of Object.entries(catalogue)){
  const holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
  expect(holes(row.en),key).toEqual(holes(row.de));expect(keys.has(key),`unused message ${key}`).toBe(true);
  expect(row.de).not.toBe(row.en.length>40?row.en:"\0");
 }
});

test("arguments are parsed strictly and every refusal is spelled in both languages",()=>{
 expect(parseRemoteExecutionArgumentsV1(["serve","--grpc-port=0","--worker-port","0"]).values).toEqual({"--grpc-port":"0","--worker-port":"0"});
 for(const bad of[[],["run"],["serve","--nope"],["serve","--grpc-port"],["serve","--grpc-port","--worker-port"],["serve","--grpc-port=1","--grpc-port=2"],["status","--bind","x"],["workers","--cache","x"]]){
  let message="";try{parseRemoteExecutionArgumentsV1(bad);}catch(error){message=(error as Error).message;}
  expect(message,JSON.stringify(bad)).not.toBe("");const lines=message.split("\n");expect(lines).toHaveLength(2);expect(lines[0]).not.toBe(lines[1]);
 }
 expect(remoteExecutionTextV1("de","serve.stopped")).toBe("Remote-Execution-Dienst beendet.");expect(()=>remoteExecutionTextV1("en","nope")).toThrow();expect(()=>remoteExecutionTextV1("en","serve.open")).toThrow(/variable missing/u);
});

const context=(root:string,overrides:Partial<RemoteExecutionCommandContextV1>&{lines?:string[]}={}):RemoteExecutionCommandContextV1=>({root,environment:{PATH:process.env.PATH},signal:new AbortController().signal,progress:async()=>{},write:line=>{overrides.lines?.push(line);},spawnWorker:()=>{throw Error("no worker");},...overrides});

test("status describes the contract, probes the endpoint with the protocol's capabilities call and reads the scheduler; nothing configured means local execution",async()=>{
 const root=mkdtempSync("/private/tmp/rexec-inhouse/cmd-");
 try{
  const empty=await remoteExecutionStatusV1([],context(root)) as any;
  expect(empty).toMatchObject({configured:false,contract:null,scheduler:null,grpc:null,server:null,token:null});expect(independent("Status",empty)).toBe(true);
  const cluster=await startClusterV1({workers:1,tokens:{write:"write-secret-9",worker:"worker-secret-8"}});
  try{
   const environment={PATH:process.env.PATH,SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_EXECUTION_SCHEDULER:cluster.server.workerUrl,SEMIO_REMOTE_EXECUTION_WORKER_TOKEN:"worker-secret-8",SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN:"write-secret-9",SEMIO_REMOTE_EXECUTION_PROPERTIES:"pool=gpu"};
   await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===1,15000,"worker");
   const status=await remoteExecutionStatusV1([],context(root,{environment})) as any;
   expect(independent("Status",status)).toBe(true);expect(repository("Status",status)).toBe(true);
   expect(status).toMatchObject({configured:true,contract:{endpoint:cluster.server.grpcUrl,instance:"main",properties:[{name:"pool",value:"gpu"}],jobs:128},grpc:{capabilities:true,execution:true,apiVersions:["2.0","2.3"],digestFunctions:["SHA256"],probe:{reachable:true}},server:{service:"semio-remote-execution",workers:[{name:"worker-1"}]},token:{present:true}});
   expect(JSON.stringify(status),"secrets never appear").not.toMatch(/secret/u);
   const dead=await remoteExecutionStatusV1([],context(root,{environment:{...environment,SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:9",SEMIO_REMOTE_EXECUTION_SCHEDULER:"http://127.0.0.1:9"}})) as any;
   expect(dead).toMatchObject({grpc:{capabilities:false,probe:{reachable:false}},server:null});expect(independent("Status",dead)).toBe(true);
  }finally{await cluster.stop();}
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("serve starts the store, the service and a worker process, prints what a client sets and a machine joins with in the chosen language, runs an action and stops cleanly when cancelled",async()=>{
 const root=mkdtempSync("/private/tmp/rexec-inhouse/cmd-"),controller=new AbortController(),lines:string[]=[],stages:string[]=[],entry=join(here,"..","..","🏃️entry","🟦️.ts"),children:ReturnType<typeof Bun.spawn>[]=[];
 try{
  const running=runRemoteExecutionCommandV1(["serve","--grpc-port","0","--worker-port","0","--local-workers","1","--sandbox","isolated-root","--concurrency","2","--lang","de"],{
   root,environment:{PATH:process.env.PATH},signal:controller.signal,progress:async stage=>{stages.push(stage);},write:line=>{lines.push(line);},
   spawnWorker:(args,environment)=>{const child=Bun.spawn(["bun",entry,...args],{cwd:root,env:{...process.env,...environment},stdout:"pipe",stderr:"pipe"});children.push(child);return {exited:child.exited,stop:async()=>{child.kill("SIGTERM");await child.exited;}};},
  });
  await eventuallyV1(()=>lines.some(line=>line.includes("Beitritt")||line.includes("tritt")||line.includes("repo remote-execution workers --join")),30000,"serve must print how to join");
  const endpoint=/^\[remote-execution\] listening (grpc:\/\/\S+)$/mu.exec(lines.join("\n"))?.[1],join=/--join (http:\/\/\S+)$/mu.exec(lines.join("\n"))?.[1];
  expect(endpoint).toMatch(/^grpc:\/\/127\.0\.0\.1:\d+$/u);expect(join).toMatch(/^http:\/\/127\.0\.0\.1:\d+$/u);
  expect(lines).toContain(`SEMIO_REMOTE_EXECUTION_ENDPOINT=${endpoint}`);expect(lines.some(line=>/^SEMIO_REMOTE_EXECUTION_PLATFORM=(darwin|linux|windows)-(arm64|x64)$/u.test(line))).toBe(true);expect(lines).toContain("SEMIO_REMOTE_EXECUTION_INSTANCE=main");
  expect(lines.some(line=>line.startsWith("Ausführungsdienst bereit"))).toBe(true);expect(lines.some(line=>line.startsWith("Gemeinsamer Speicher:"))).toBe(true);expect(lines.some(line=>/^Execution service ready/u.test(line))).toBe(false);
  expect(stages).toEqual(expect.arrayContaining(["remote-execution:open","remote-execution:cache","remote-execution:workers","remote-execution:serving"]));
  await eventuallyV1(async()=>{const status=await (await fetch(join!+"/status")).json() as any;return status.workers.length===1;},30000,"the local worker process must register");
  const client=connectGrpcV1(endpoint!);
  try{
   const put=async(bytes:Uint8Array)=>{const digest={hash:sha(bytes),size_bytes:bytes.length};await client.unary("build.bazel.remote.execution.v2.ContentAddressableStorage/BatchUpdateBlobs",{instance_name:"main",requests:[{digest,data:bytes}]});return digest;};
   const command=await put(encodeProtobufV1(REAPI_V1.command,{arguments:["/bin/echo","served"]})),action=await put(encodeProtobufV1(REAPI_V1.action,{command_digest:command}));
   let response:any;for await(const row of client.stream("build.bazel.remote.execution.v2.Execution/Execute",{instance_name:"main",action_digest:action}))if(row.done)response=decodeProtobufV1(REAPI_V1.executeResponse,row.response.value);
   expect(new TextDecoder().decode(response.result.stdout_raw)).toBe("served\n");expect(response.result.execution_metadata.worker).toMatch(/-local-1$/u);
  }finally{client.close();}
  controller.abort();
  expect((await running).exitCode).toBe(0);
  expect(lines.some(line=>line==="Remote-Execution-Dienst beendet.")).toBe(true);
  for(const child of children)expect(await child.exited).toBe(0);
 }finally{controller.abort();for(const child of children)child.kill("SIGKILL");rmSync(root,{recursive:true,force:true});}
});

test("workers refuses to start without a scheduler and serve refuses to listen beyond loopback without tokens, in both languages",async()=>{
 const root=mkdtempSync("/private/tmp/rexec-inhouse/cmd-");
 try{
  await expect(runRemoteExecutionCommandV1(["workers"],context(root))).rejects.toThrow(/SEMIO_REMOTE_EXECUTION_SCHEDULER[\s\S]*SEMIO_REMOTE_EXECUTION_SCHEDULER/u);
  await expect(runRemoteExecutionCommandV1(["serve","--bind","0.0.0.0"],context(root))).rejects.toThrow(/Tokens are required[\s\S]*Tokens sind|Außerhalb von Loopback/u);
  await expect(runRemoteExecutionCommandV1(["serve","--tls-certificate","x.pem"],context(root))).rejects.toThrow(/TLS needs[\s\S]*TLS braucht/u);
  await expect(runRemoteExecutionCommandV1(["serve","--local-workers","65"],context(root))).rejects.toThrow(/whole number from 0 to 64[\s\S]*ganze Zahl von 0 bis 64/u);
 }finally{rmSync(root,{recursive:true,force:true});}
});

void policy;
