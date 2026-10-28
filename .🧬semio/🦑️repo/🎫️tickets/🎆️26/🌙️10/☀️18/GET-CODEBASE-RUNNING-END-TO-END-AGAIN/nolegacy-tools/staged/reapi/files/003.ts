import {afterEach,expect,test} from "bun:test";
import {createServer} from "node:net";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {networkInterfaces,tmpdir} from "node:os";
import {join} from "node:path";
import {GRPC_CODES_V1,GrpcErrorV1} from "../📡️wire/🚪️grpc/🟦️.ts";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from "../📡️wire/📦️protobuf/🟦️.ts";
import {createHash} from "node:crypto";
import {testScratchRootV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {requireRemoteExecutionShapeV1,remoteExecutionHostPropertiesV1} from "../🟦️.ts";
import {eventuallyV1,NAMESPACE_V1,startClusterV1,textOfV1,type ClusterV1} from "./🧰️cluster/🟦️.ts";
import {selectRemoteExecutionSandboxV1} from "../🔒️sandbox/🟦️.ts";

const sha=(data:Uint8Array|string)=>createHash("sha256").update(data).digest("hex"),clusters:ClusterV1[]=[],scratch:string[]=[];
const start=async(...parameters:Parameters<typeof startClusterV1>)=>{const cluster=await startClusterV1(...parameters);clusters.push(cluster);return cluster;};
afterEach(async()=>{for(const cluster of clusters.splice(0))await cluster.stop();for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=()=>{const path=mkdtempSync(join(testScratchRootV1("rexec-inhouse"),"scratch-"));scratch.push(path);return path;};
const code=async(promise:Promise<unknown>):Promise<number>=>{try{await promise;return 0;}catch(error){return (error as GrpcErrorV1).code;}};
const HOST=remoteExecutionHostPropertiesV1();
const ISOLATED={sandbox:"isolated-root" as const};

test("capabilities advertise execution, SHA-256 and the API versions Bazel negotiates, and bad calls are refused with proper statuses",async()=>{
 const cluster=await start({workers:0}),caps=await cluster.client.unary(`${NAMESPACE_V1}.Capabilities/GetCapabilities`,{});
 expect(caps.execution_capabilities).toMatchObject({exec_enabled:true,digest_function:1});
 expect(caps.cache_capabilities).toMatchObject({digest_functions:[1],max_batch_total_size_bytes:4194304,symlink_absolute_path_strategy:2});
 expect([caps.low_api_version.major,caps.low_api_version.minor,caps.high_api_version.major,caps.high_api_version.minor]).toEqual([2,1,2,3]);
 expect(await code(cluster.client.unary(`${NAMESPACE_V1}.ActionCache/GetActionResult`,{instance_name:"Bad Instance!",action_digest:{hash:"a".repeat(64),size_bytes:1}}))).toBe(GRPC_CODES_V1.INVALID_ARGUMENT);
 expect(await code(cluster.client.unary(`${NAMESPACE_V1}.ActionCache/GetActionResult`,{instance_name:"main",action_digest:{hash:"a".repeat(64),size_bytes:1}}))).toBe(GRPC_CODES_V1.NOT_FOUND);
 expect(await code(cluster.client.unary(`${NAMESPACE_V1}.ContentAddressableStorage/FindMissingBlobs`,{instance_name:"main",blob_digests:[{hash:"zz",size_bytes:1}]}))).toBe(GRPC_CODES_V1.INVALID_ARGUMENT);
 expect(await code(cluster.client.unary(`${NAMESPACE_V1}.ActionCache/GetActionResult`,{instance_name:"main",action_digest:{hash:"a".repeat(64),size_bytes:1},digest_function:"SHA512"}))).toBe(GRPC_CODES_V1.INVALID_ARGUMENT);
});

test("an action runs on a worker, its outputs land in the store, the result is cached and the repeat is answered from the action cache without a worker",async()=>{
 const cluster=await start({workers:1}),action=await cluster.action({arguments:["/bin/sh","-c","cat in.txt > out.txt; echo hi; echo err >&2"],files:{"in.txt":"input\n"},outputPaths:["out.txt"],platform:HOST.map(row=>({...row}))});
 const first=await cluster.execute(action);
 expect(first.response.status.code).toBe(0);expect(first.response.cached_result).toBe(false);expect(first.stages).toContain("EXECUTING");expect(first.stages.at(-1)).toBe("COMPLETED");
 const result=first.response.result;
 expect([result.exit_code,textOfV1(result.stdout_raw),textOfV1(result.stderr_raw),result.execution_metadata.worker]).toEqual([0,"hi\n","err\n","worker-1"]);
 expect(result.output_files).toHaveLength(1);expect(result.output_files[0]).toMatchObject({path:"out.txt",is_executable:false,digest:{hash:sha("input\n"),size_bytes:6}});
 const stored=await cluster.content.read("main",{hash:sha("input\n"),size:6});expect(textOfV1(stored)).toBe("input\n");
 const cached=await cluster.client.unary(`${NAMESPACE_V1}.ActionCache/GetActionResult`,{instance_name:"main",action_digest:action});expect(cached.exit_code).toBe(0);
 const assigned=()=>cluster.events.filter(row=>row.startsWith("assigned")).length;
 expect(assigned()).toBe(1);
 const second=await cluster.execute(action);
 expect(second.response.cached_result).toBe(true);expect(second.stages).not.toContain("EXECUTING");expect(assigned()).toBe(1);
 const forced=await cluster.execute(action,{skipCacheLookup:true});expect(forced.response.cached_result).toBe(false);expect(assigned()).toBe(2);
});

test("a failing action reports its exit code and is not cached; a missing executable is reported as 127",async()=>{
 const cluster=await start({workers:1}),failing=await cluster.action({arguments:["/bin/sh","-c","echo nope >&2; exit 3"]});
 const run=await cluster.execute(failing);
 expect([run.response.status.code,run.response.result.exit_code,textOfV1(run.response.result.stderr_raw)]).toEqual([0,3,"nope\n"]);
 expect(await code(cluster.client.unary(`${NAMESPACE_V1}.ActionCache/GetActionResult`,{instance_name:"main",action_digest:failing}))).toBe(GRPC_CODES_V1.NOT_FOUND);
 const again=await cluster.execute(failing);expect(again.response.cached_result).toBe(false);
 const missing=await cluster.execute(await cluster.action({arguments:["tools/does-not-exist"]}));
 expect(missing.response.result.exit_code).toBe(127);
});

test("directory outputs come back as a Tree with children, executable bits and symbolic links; symlink outputs are reported; a REAPI 2.0 command is refused",async()=>{
 const cluster=await start({workers:1}),script="mkdir -p out/sub; echo A > out/sub/a.txt; echo B > out/b.sh; chmod +x out/b.sh; ln -s sub/a.txt out/link; ln -s out/b.sh top-link; mkdir -p out/empty";
 for(const declared of [{outputFiles:["top-link"]},{outputDirectories:["out"]},{outputFiles:["top-link"],outputPaths:["top-link"]}]){
  const refused=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c",script],...declared}));
  expect(refused.response.status.code,JSON.stringify(declared)).toBe(GRPC_CODES_V1.INVALID_ARGUMENT);
 }
 const action=await cluster.action({arguments:["/bin/sh","-c",script],outputPaths:["out","top-link"]}),run=await cluster.execute(action),result=run.response.result;
 expect(run.response.status.code).toBe(0);
 expect(result.output_directories).toHaveLength(1);expect(result.output_directories[0].path).toBe("out");
 const tree=decodeProtobufV1(REAPI_V1.tree,(await cluster.content.read("main",{hash:result.output_directories[0].tree_digest.hash,size:result.output_directories[0].tree_digest.size_bytes}))!);
 expect(tree.root.files.map((row:any)=>[row.name,row.is_executable])).toEqual([["b.sh",true]]);
 expect(tree.root.directories.map((row:any)=>row.name)).toEqual(["empty","sub"]);expect(tree.root.symlinks).toEqual([expect.objectContaining({name:"link",target:"sub/a.txt"})]);
 expect(tree.children).toHaveLength(2);
 const names=tree.children.flatMap((row:any)=>row.files.map((file:any)=>file.name));expect(names).toEqual(["a.txt"]);
 expect(result.output_symlinks.map((row:any)=>[row.path,row.target])).toEqual([["top-link","out/b.sh"]]);expect(result.output_file_symlinks).toEqual([]);expect(result.output_directory_symlinks).toEqual([]);
 expect(textOfV1(await cluster.content.read("main",{hash:sha("A\n"),size:2}))).toBe("A\n");
});

test("a missing input blob is a FAILED_PRECONDITION that names the blob, so the client can upload it and retry",async()=>{
 const cluster=await start({workers:1}),absent={hash:sha("never uploaded"),size_bytes:14},root=await cluster.upload(encodeProtobufV1(REAPI_V1.directory,{files:[{name:"x.txt",digest:absent}]}));
 const command=await cluster.upload(encodeProtobufV1(REAPI_V1.command,{arguments:["/bin/true"]})),action=await cluster.upload(encodeProtobufV1(REAPI_V1.action,{command_digest:command,input_root_digest:root}));
 const run=await cluster.execute(action),status=run.response.status;
 expect(status.code).toBe(GRPC_CODES_V1.FAILED_PRECONDITION);
 const detail=decodeProtobufV1(REAPI_V1.preconditionFailure,status.details[0].value);expect(detail.violations).toEqual([expect.objectContaining({type:"MISSING",subject:`blobs/${absent.hash}/${absent.size_bytes}`})]);
 const unknownAction=await cluster.execute({hash:sha("no such action"),size_bytes:14});expect(unknownAction.response.status.code).toBe(GRPC_CODES_V1.FAILED_PRECONDITION);
});

test("timeouts kill the whole process tree and report DEADLINE_EXCEEDED; cancelling the call stops a running action",async()=>{
 const cluster=await start({workers:1,worker:ISOLATED}),directory=temporary();
 const slow=(name:string,seconds?:number)=>cluster.action({arguments:["/bin/sh","-c",`(sleep 60 & echo $! > ${directory}/${name}.child; wait) & echo $! > ${directory}/${name}.pid; wait`],...(seconds?{timeoutSeconds:seconds}:{}),salt:name});
 const timed=await cluster.execute(await slow("timed",1));expect(timed.response.status.code).toBe(GRPC_CODES_V1.DEADLINE_EXCEEDED);
 const alive=(path:string)=>{try{process.kill(Number(readFileSync(path,"utf8").trim()),0);return true;}catch{return false;}};
 await eventuallyV1(()=>!alive(`${directory}/timed.pid`)&&!alive(`${directory}/timed.child`),5000,"timed out processes must be gone");
 const abort=new AbortController(),running=cluster.execute(await slow("cancelled"),{signal:abort.signal}).catch(error=>error);
 await eventuallyV1(()=>existsSync(`${directory}/cancelled.child`),8000,"the action must start");abort.abort();await running;
 await eventuallyV1(()=>!alive(`${directory}/cancelled.pid`)&&!alive(`${directory}/cancelled.child`),8000,"a cancelled action's processes must be gone");
 await eventuallyV1(()=>(cluster.server.scheduler.status().counters.cancelled??0)>=1,5000,"the scheduler counts the cancellation");
});

test("identical requests in flight share one execution",async()=>{
 const cluster=await start({workers:1,worker:ISOLATED}),directory=temporary(),action=await cluster.action({arguments:["/bin/sh","-c",`echo run >> ${directory}/runs.txt; sleep 1; echo shared > out.txt`],outputPaths:["out.txt"]});
 const [left,right]=await Promise.all([cluster.execute(action),cluster.execute(action)]);
 expect(readFileSync(`${directory}/runs.txt`,"utf8")).toBe("run\n");
 expect(left.operation).toBe(right.operation);expect(left.response.result.output_files[0].digest).toEqual(right.response.result.output_files[0].digest);
 expect(cluster.server.scheduler.status().counters.deduplicated).toBe(1);
});

test("actions are matched to workers by platform properties, and an action no worker can run fails with a clear status after the grace period",async()=>{
 const cluster=await start({workers:1,scheduler:{unmatchedSeconds:1}});
 const fits=await cluster.execute(await cluster.action({arguments:["/bin/true"],platform:HOST.map(row=>({...row}))}));expect(fits.response.status.code).toBe(0);
 const nowhere=await cluster.execute(await cluster.action({arguments:["/bin/true"],platform:[{name:"OSFamily",value:"plan9"}]}));
 expect(nowhere.response.status.code).toBe(GRPC_CODES_V1.FAILED_PRECONDITION);expect(nowhere.response.status.message).toMatch(/No worker offers the platform OSFamily=plan9/u);
 const late=cluster.execute(await cluster.action({arguments:["/bin/sh","-c","echo late > out.txt"],outputPaths:["out.txt"],platform:[{name:"pool",value:"special"}],salt:"late"}));
 cluster.startWorker({name:"special",properties:[...HOST,{name:"pool",value:"special"}]});
 expect((await late).response.result.execution_metadata.worker).toBe("special");
});

const sandboxed=(()=>{try{return selectRemoteExecutionSandboxV1("auto").name!=="isolated-root";}catch{return false;}})();
test.skipIf(!sandboxed)("the host sandbox confines an action to its input root: no writes elsewhere, no reads of other directories, no network",async()=>{
 const secret=join(temporary(),"secret.txt"),outside=join(temporary(),"outside.txt");writeFileSync(secret,"classified\n");
 let connections=0;const listener=await new Promise<ReturnType<typeof createServer>>(resolve=>{const server=createServer(socket=>{connections++;socket.end();});server.listen(0,"127.0.0.1",()=>resolve(server));});
 const port=(listener.address() as {port:number}).port;
 try{
  const probe=`cat ${secret} > read.txt 2>/dev/null; echo "$?" > read.rc; echo x > ${outside} 2>/dev/null; echo "$?" > write.rc; (exec 3<>/dev/tcp/127.0.0.1/${port}) 2>/dev/null; echo "$?" > net.rc; echo inside > ok.txt`;
  const run=async(sandbox:"auto"|"isolated-root")=>{
   const cluster=await start({workers:1,worker:{sandbox}}),action=await cluster.action({arguments:["/bin/bash","-c",probe],outputPaths:["read.rc","write.rc","net.rc","ok.txt"]}),result=(await cluster.execute(action)).response.result;
   const out:Record<string,string>={};for(const row of result.output_files)out[row.path]=textOfV1(await cluster.content.read("main",{hash:row.digest.hash,size:row.digest.size_bytes})).trim();
   await cluster.stop();clusters.splice(clusters.indexOf(cluster),1);return out;
  };
  const confined=await run("auto");
  expect(confined["ok.txt"]).toBe("inside");expect(confined["read.rc"]).not.toBe("0");expect(confined["write.rc"]).not.toBe("0");expect(confined["net.rc"]).not.toBe("0");expect(existsSync(outside)).toBe(false);
  const connectedBefore=connections;
  const open=await run("isolated-root");
  expect(open["read.rc"],"without a sandbox the same action reads the secret").toBe("0");expect(open["write.rc"]).toBe("0");expect(existsSync(outside)).toBe(true);expect(connections).toBeGreaterThan(connectedBefore);
 }finally{listener.close();}
});

test("a worker started as its own process joins, runs actions and leaves on SIGTERM",async()=>{
 const cluster=await start({workers:0}),work=temporary(),entry=join(import.meta.dir,"..","🏃️entry","🟦️.ts");
 const child=Bun.spawn(["bun",entry,"--join",cluster.server.workerUrl,"--name","proc-1","--concurrency","2","--sandbox","isolated-root","--work-directory",work,"--lang","en"],{stdout:"pipe",stderr:"pipe",cwd:work,env:{...process.env}});
 try{
  await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===1,20000,"the worker process must register");
  const run=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c","echo from-process > out.txt"],outputPaths:["out.txt"]}));
  expect(run.response.result.execution_metadata.worker).toBe("proc-1");expect(textOfV1(await cluster.content.read("main",{hash:sha("from-process\n"),size:13}))).toBe("from-process\n");
  const status=await (await fetch(cluster.server.workerUrl+"/status")).json();requireRemoteExecutionShapeV1("ServerStatus",status,"status");
  expect(status.workers.map((row:any)=>[row.name,row.sandbox,row.platform.split("-").length])).toEqual([["proc-1","isolated-root",2]]);expect(status.counters.completed).toBe(1);
 }finally{child.kill("SIGTERM");}
 expect(await child.exited).toBe(0);
 await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===0,5000,"the worker must deregister");
});

test("work of a worker that dies is re-queued after its lease expires and finished by another worker",async()=>{
 const cluster=await start({workers:0,scheduler:{leaseSeconds:2}}),work=temporary(),entry=join(import.meta.dir,"..","🏃️entry","🟦️.ts"),marker=join(work,"started");
 const child=Bun.spawn(["bun",entry,"--join",cluster.server.workerUrl,"--name","doomed","--concurrency","1","--sandbox","isolated-root","--work-directory",join(work,"w"),"--lang","en"],{stdout:"ignore",stderr:"ignore",cwd:work,env:{...process.env}});
 try{
 await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===1,20000,"the doomed worker must register");
 const run=cluster.execute(await cluster.action({arguments:["/bin/sh","-c",`echo go > ${marker}; sleep 2; echo survived > out.txt`],outputPaths:["out.txt"]}));
 await eventuallyV1(()=>existsSync(marker),15000,"the action must start on the doomed worker");
 child.kill("SIGKILL");cluster.startWorker({name:"rescuer",...ISOLATED});
 const result=await run;
 expect(result.response.status.code).toBe(0);expect(result.response.result.execution_metadata.worker).toBe("rescuer");
 expect(cluster.events.some(row=>row.startsWith("retried"))).toBe(true);expect(cluster.events.some(row=>row.startsWith("worker-lost doomed"))).toBe(true);
 }finally{child.kill("SIGKILL");}
});

test("tokens: no token is refused, a read token may look but not write, a write token executes, and the worker API needs the worker token",async()=>{
 const tokens={read:"read-secret-1",write:"write-secret-2",worker:"worker-secret-3"},cluster=await start({workers:1,tokens});
 const call=(method:string,request:object,token?:string)=>cluster.client.unary(method,request,token?{metadata:{authorization:`Bearer ${token}`}}:{});
 const lookup={instance_name:"main",action_digest:{hash:"a".repeat(64),size_bytes:1}};
 expect(await code(call(`${NAMESPACE_V1}.ActionCache/GetActionResult`,lookup))).toBe(GRPC_CODES_V1.UNAUTHENTICATED);
 expect(await code(call(`${NAMESPACE_V1}.ActionCache/GetActionResult`,lookup,"wrong"))).toBe(GRPC_CODES_V1.UNAUTHENTICATED);
 expect(await code(call(`${NAMESPACE_V1}.ActionCache/GetActionResult`,lookup,tokens.read))).toBe(GRPC_CODES_V1.NOT_FOUND);
 expect(await code(call(`${NAMESPACE_V1}.ActionCache/UpdateActionResult`,{...lookup,action_result:{exit_code:0}},tokens.read))).toBe(GRPC_CODES_V1.PERMISSION_DENIED);
 expect(await code(cluster.client.upload("google.bytestream.ByteStream/Write",[{resource_name:`main/uploads/x/blobs/${"a".repeat(64)}/1`,data:new Uint8Array(1),finish_write:true}],{metadata:{authorization:`Bearer ${tokens.read}`}}))).toBe(GRPC_CODES_V1.PERMISSION_DENIED);
 const run=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c","echo authorised > out.txt"],outputPaths:["out.txt"]}));expect(run.response.status.code).toBe(0);
 expect((await fetch(cluster.server.workerUrl+"/v1/workers/x/poll",{method:"POST"})).status).toBe(401);
 expect((await fetch(cluster.server.workerUrl+"/status")).status).toBe(401);
 expect((await fetch(cluster.server.workerUrl+"/status",{headers:{authorization:`Bearer ${tokens.read}`}})).status).toBe(200);
 await expect(startClusterV1({tokens:{}, server:{host:"0.0.0.0"}})).rejects.toThrow(/Tokens are required/u);
});

test("ByteStream moves large blobs both ways with offsets, FindMissingBlobs and batches agree with the store, and empty blobs always exist",async()=>{
 const cluster=await start({workers:0}),size=24*1048576+17,data=new Uint8Array(size);for(let index=0;index<size;index++)data[index]=(index*31+7)&255;
 const digest={hash:sha(data),size_bytes:size},resource=`main/uploads/u1/blobs/${digest.hash}/${size}`;
 const absent=await cluster.client.unary(`${NAMESPACE_V1}.ContentAddressableStorage/FindMissingBlobs`,{instance_name:"main",blob_digests:[digest,{hash:sha(""),size_bytes:0}]});expect(absent.missing_blob_digests).toEqual([digest]);
 const pieces=function*(){for(let offset=0;offset<size;offset+=65536)yield {resource_name:offset===0?resource:"",write_offset:offset,data:data.subarray(offset,offset+65536),finish_write:offset+65536>=size};};
 expect((await cluster.client.upload("google.bytestream.ByteStream/Write",pieces())).committed_size).toBe(size);
 const present=await cluster.client.unary(`${NAMESPACE_V1}.ContentAddressableStorage/FindMissingBlobs`,{instance_name:"main",blob_digests:[digest]});expect(present.missing_blob_digests).toEqual([]);
 const chunks:Uint8Array[]=[];for await(const piece of cluster.client.stream("google.bytestream.ByteStream/Read",{resource_name:`main/blobs/${digest.hash}/${size}`}))chunks.push(piece.data);
 expect(sha(Buffer.concat(chunks))).toBe(digest.hash);
 const tail:Uint8Array[]=[];for await(const piece of cluster.client.stream("google.bytestream.ByteStream/Read",{resource_name:`main/blobs/${digest.hash}/${size}`,read_offset:size-100}))tail.push(piece.data);expect(Buffer.concat(tail)).toEqual(Buffer.from(data.subarray(size-100)));
 expect(await code(cluster.client.upload("google.bytestream.ByteStream/Write",[{resource_name:`main/uploads/u2/blobs/${sha("short")}/10`,data:new TextEncoder().encode("short"),finish_write:true}]))).toBe(GRPC_CODES_V1.INVALID_ARGUMENT);
 const wrong=await cluster.client.unary(`${NAMESPACE_V1}.ContentAddressableStorage/BatchUpdateBlobs`,{instance_name:"main",requests:[{digest:{hash:sha("a"),size_bytes:1},data:new TextEncoder().encode("b")},{digest:{hash:sha("c"),size_bytes:1},data:new TextEncoder().encode("c")}]});
 expect(wrong.responses.map((row:any)=>row.status.code)).toEqual([GRPC_CODES_V1.INVALID_ARGUMENT,0]);
 const read=await cluster.client.unary(`${NAMESPACE_V1}.ContentAddressableStorage/BatchReadBlobs`,{instance_name:"main",digests:[{hash:sha("c"),size_bytes:1},{hash:sha("zzz"),size_bytes:3},{hash:sha(""),size_bytes:0}]});
 expect(read.responses.map((row:any)=>row.status.code)).toEqual([0,GRPC_CODES_V1.NOT_FOUND,0]);expect(textOfV1(read.responses[0].data)).toBe("c");
 const empty:Uint8Array[]=[];for await(const piece of cluster.client.stream("google.bytestream.ByteStream/Read",{resource_name:`main/blobs/${sha("")}/0`}))empty.push(piece.data);expect(empty).toHaveLength(0);
 expect((await cluster.client.unary("google.bytestream.ByteStream/QueryWriteStatus",{resource_name:`main/uploads/u1/blobs/${digest.hash}/${size}`})).complete).toBe(true);
});

test("an action without inputs or outputs, an empty input root and a working directory all work; paths that escape the input root are refused",async()=>{
 const cluster=await start({workers:1}),plain=await cluster.execute(await cluster.action({arguments:["/bin/echo","plain"]}));expect(textOfV1(plain.response.result.stdout_raw)).toBe("plain\n");
 const nested=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c","pwd > where.txt; cat data.txt >> where.txt"],files:{"sub/dir/data.txt":"payload\n"},workingDirectory:"sub/dir",outputPaths:["where.txt"]}));
 expect(nested.response.status.code).toBe(0);const file=nested.response.result.output_files[0];expect(textOfV1(await cluster.content.read("main",{hash:file.digest.hash,size:file.digest.size_bytes}))).toMatch(/sub\/dir\npayload\n$/u);
 const escape=await cluster.execute(await cluster.action({arguments:["/bin/true"],outputPaths:["../escape.txt"]}));expect(escape.response.status.code).toBe(GRPC_CODES_V1.INVALID_ARGUMENT);
 const linked=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c","rm -rf a; ln -s /etc a"],outputPaths:["a/hosts"]}));
 expect(linked.response.status.code,"an output reached through a symbolic link out of the root is refused, not uploaded").toBe(GRPC_CODES_V1.INVALID_ARGUMENT);expect(linked.response.status.message).toMatch(/leaves the input root/u);
 const inputs=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c","./tool.sh > out.txt"],files:{"tool.sh":{content:"#!/bin/sh\necho tool-ran\n",executable:true}},outputPaths:["out.txt"]}));expect(inputs.response.status.code).toBe(0);
 const out=inputs.response.result.output_files[0];expect(textOfV1(await cluster.content.read("main",{hash:out.digest.hash,size:out.digest.size_bytes}))).toBe("tool-ran\n");
});

const lan=Object.values(networkInterfaces()).flat().find(row=>row&&row.family==="IPv4"&&!row.internal)?.address;
test.skipIf(lan===undefined)("a worker on another network interface joins with tokens, fetches inputs from the shared store through the scheduler's address and runs an action",async()=>{
 const tokens={read:"read-secret-1",write:"write-secret-2",worker:"worker-secret-3"},cluster=await start({workers:0,bind:"0.0.0.0",advertise:lan!,tokens}),work=temporary(),entry=join(import.meta.dir,"..","🏃️entry","🟦️.ts");
 expect(cluster.server.workerUrl.startsWith(`http://${lan}:`)).toBe(true);
 const child=Bun.spawn(["bun",entry,"--join",cluster.server.workerUrl,"--name","lan-worker","--concurrency","1","--sandbox","isolated-root","--work-directory",work,"--lang","en"],{stdout:"pipe",stderr:"pipe",cwd:work,env:{...process.env,SEMIO_REMOTE_EXECUTION_WORKER_TOKEN:tokens.worker,SEMIO_REMOTE_CACHE_TOKEN:tokens.write}});
 try{
  await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===1,30000,"the worker must register through the LAN address");
  const run=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c","cat in.txt > out.txt"],files:{"in.txt":"over the network\n"},outputPaths:["out.txt"]}));
  expect(run.response.result.execution_metadata.worker).toBe("lan-worker");expect(textOfV1(await cluster.content.read("main",{hash:sha("over the network\n"),size:17}))).toBe("over the network\n");
  const wrong=await fetch(cluster.server.workerUrl+"/v1/workers",{method:"POST",headers:{authorization:"Bearer nope","content-type":"application/json"},body:"{}"});expect(wrong.status).toBe(401);
 }finally{child.kill("SIGTERM");await child.exited;}
});

test("a client that lost its stream re-attaches to the running operation by name and receives the result; one that never returns cancels the work after the linger time",async()=>{
 const cluster=await start({workers:1,worker:ISOLATED,scheduler:{lingerSeconds:3}}),directory=temporary(),action=await cluster.action({arguments:["/bin/sh","-c",`echo started > ${directory}/started; sleep 2; echo done > out.txt`],outputPaths:["out.txt"]});
 const abort=new AbortController();let name="";
 const first=(async()=>{try{for await(const row of cluster.client.stream(`${NAMESPACE_V1}.Execution/Execute`,{instance_name:"main",action_digest:action},{signal:abort.signal})){name=row.name;}}catch{}})();
 await eventuallyV1(()=>name!==""&&existsSync(`${directory}/started`),15000,"the action must start");abort.abort();await first;
 let response:any;for await(const row of cluster.client.stream(`${NAMESPACE_V1}.Execution/WaitExecution`,{name}))if(row.done)response=decodeProtobufV1(REAPI_V1.executeResponse,row.response.value);
 expect(response.status.code).toBe(0);expect(response.result.output_files[0].path).toBe("out.txt");expect(cluster.server.scheduler.status().counters.cancelled).toBe(0);
 expect(await code((async()=>{for await(const _ of cluster.client.stream(`${NAMESPACE_V1}.Execution/WaitExecution`,{name:"operations/unknown"}));})())).toBe(GRPC_CODES_V1.NOT_FOUND);
});
