import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import protobuf from "protobufjs";
import descriptor from "protobufjs/ext/descriptor";
import {mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {decodeProtobufV1,encodeProtobufV1,PROTOBUF_SERVICES_V1,protobufEnumV1,type ProtobufFieldV1} from "../📦️protobuf/🟦️.ts";
import schema from "../📋️messages/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle","🐍️.py"),generator=join(import.meta.dir,"../🐍️generator/🐍️.py"),messages=schema.messages as unknown as Record<string,readonly ProtobufFieldV1[]>,names=Object.keys(messages);

const seeded=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
const WORDS=["","a","hash","größe","日本語","🧬semio","x".repeat(70),"a/b/c"];
function sample(random:()=>number,name:string,depth:number):Record<string,unknown>{
 const out:Record<string,unknown>={};
 for(const field of messages[name]!){
  if((random()<0.35&&!name.endsWith("Entry"))||(field.type==="message"&&depth>=3&&!name.endsWith("Entry")))continue;
  const one=():unknown=>{
   switch(field.type){
    case "string":return WORDS[1+Math.floor(random()*(WORDS.length-1))]!;
    case "bytes":return Uint8Array.from({length:1+Math.floor(random()*40)},()=>Math.floor(random()*256));
    case "bool":return true;
    case "double":return 1.5+Math.floor(random()*100);
    case "float":return 2.5+Math.floor(random()*100);
    case "enum":{const values=Object.values((schema.enums as Record<string,Record<string,number>>)[field.enum!]!).filter(value=>value!==0);return values[Math.floor(random()*values.length)]??0;}
    case "message":return sample(random,field.message!,depth+1);
    case "int32":return random()<0.2?-1-Math.floor(random()*1000):1+Math.floor(random()*2**(random()<0.5?20:30));
    case "int64":return random()<0.2?-1-Math.floor(random()*1000):1+Math.floor(random()*2**(random()<0.5?20:50));
    case "uint32":return 1+Math.floor(random()*2**(random()<0.5?20:31));
    default:return 1+Math.floor(random()*2**(random()<0.5?20:50));
   }
  };
  if(field.repeated){const items=Array.from({length:field.message?.endsWith("Entry")?1:1+Math.floor(random()*3)},one);out[field.name]=items;}else out[field.name]=one();
 }
 return out;
}

function forThird(name:string,value:Record<string,unknown>):Record<string,unknown>{
 const out:Record<string,unknown>={};
 for(const field of messages[name]!){
  const entry=value[field.name];
  if(entry===undefined)continue;
  if(field.type!=="message")out[field.name]=entry;
  else if(field.repeated&&field.message!.endsWith("Entry")){const map:Record<string,unknown>={};for(const row of entry as Record<string,unknown>[]){const target=messages[field.message!]!.find(inner=>inner.name==="value")!;map[String(row.key)]=target.message===undefined?row.value:forThird(target.message,row.value as Record<string,unknown>);}out[field.name]=map;}
  else out[field.name]=field.repeated?(entry as Record<string,unknown>[]).map(item=>forThird(field.message!,item)):forThird(field.message!,entry as Record<string,unknown>);
 }
 return out;
}

function plain(value:unknown):unknown{
 if(value instanceof Uint8Array)return Buffer.from(value).toString("hex");
 if(Array.isArray(value))return value.map(plain);
 if(value&&typeof value==="object"){
  const out:Record<string,unknown>={};
  for(const [key,item] of Object.entries(value as Record<string,unknown>))if(item!==undefined)out[key]=plain(item);
  return out;
 }
 return value;
}

function script(args:string[],input?:string):string{
 const result=Bun.spawnSync([python!,"-I",...args],{stdin:input===undefined?"ignore":new TextEncoder().encode(input),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);return result.stdout.toString();
}

const varint=(data:Uint8Array,start:number):[number,number]=>{let result=0,scale=1,index=start;for(;;){const byte=data[index++]!;result+=(byte&127)*scale;scale*=128;if(byte<128)return [result,index];}};
const withoutExtensions=(file:Uint8Array):Uint8Array=>{
 const kept:Uint8Array[]=[];let index=0;
 while(index<file.length){
  const begin=index,[key,afterKey]=varint(file,index),wire=key%8;index=afterKey;
  if(wire===2){const [size,afterSize]=varint(file,index);index=afterSize+size;}else if(wire===0)index=varint(file,index)[1];else throw Error("wire");
  if(Math.floor(key/8)!==7)kept.push(file.subarray(begin,index));
 }
 return Buffer.concat(kept);
};
const descriptors=(()=>{
 const parts:Uint8Array[]=[];
 for(const encoded of Object.values(fixture.files)){const data=withoutExtensions(Buffer.from(encoded,"base64")),head=[10];let size=data.length;while(size>=128){head.push((size&127)|128);size>>=7;}head.push(size);parts.push(Uint8Array.from(head),data);}
 return Buffer.concat(parts);
})();
const root=(()=>{
 const set=(descriptor as any).FileDescriptorSet.decode(descriptors),built=(protobuf.Root as any).fromDescriptor(set,{keepCase:true});
 return built as protobuf.Root;
})();
const third=(name:string)=>{const type=root.lookupType(name);type.resolveAll();return type;};

test("the wire schema is exactly what the generator derives from the descriptors Bazel's own classes embed",()=>{
 const scratch=mkdtempSync(join(tmpdir(),"rxi-wire-")),source=join(scratch,"descriptors.json"),target=join(scratch,"messages.json");
 try{
  writeFileSync(source,JSON.stringify(fixture));script([generator,source,target]);
  expect(JSON.parse(readFileSync(target,"utf8"))).toEqual(JSON.parse(JSON.stringify(schema)));
 }finally{rmSync(scratch,{recursive:true,force:true});}
});

test("the services the protocol needs are all declared with their streaming shape",()=>{
 const services=PROTOBUF_SERVICES_V1;
 expect(Object.keys(services["build.bazel.remote.execution.v2.Execution"]!)).toEqual(["Execute","WaitExecution"]);
 expect(services["build.bazel.remote.execution.v2.Execution"]!.Execute).toMatchObject({serverStreaming:true,clientStreaming:false});
 expect(services["google.bytestream.ByteStream"]!.Write).toMatchObject({clientStreaming:true,serverStreaming:false});
 expect(services["google.bytestream.ByteStream"]!.Read).toMatchObject({clientStreaming:false,serverStreaming:true});
 for(const service of Object.values(services))for(const method of Object.values(service)){expect(messages[method.request]).toBeDefined();expect(messages[method.response]).toBeDefined();}
});

test("every message encodes to the same bytes as the third-party protobuf library and decodes back, over seeded random values",()=>{
 const random=seeded(20261021);let count=0;
 for(const name of names.filter(name=>!name.endsWith("Entry"))){
  for(let round=0;round<6;round++){
   const value=sample(random,name,0),mine=encodeProtobufV1(name,value),type=third(name),theirs=type.encode(type.fromObject(forThird(name,value) as any)).finish();
   const left=Buffer.from(mine).toString("hex"),right=Buffer.from(theirs).toString("hex");let at=0;while(at<left.length&&left[at]===right[at])at++;
   expect(left,`${name} round ${round} differs at hex offset ${at}: mine ${left.slice(Math.max(0,at-24),at+40)} third-party ${right.slice(Math.max(0,at-24),at+40)}`).toBe(right);
   const back=decodeProtobufV1(name,mine),viaThird=type.toObject(type.decode(theirs),{longs:Number,enums:Number,bytes:Array,defaults:false,arrays:false,objects:false});
   expect(plain(encodeProtobufV1(name,back))).toEqual(plain(mine));
   expect(Buffer.from(encodeProtobufV1(name,decodeProtobufV1(name,Uint8Array.from(theirs)))).toString("hex")).toBe(Buffer.from(mine).toString("hex"));
   void viaThird;count++;
  }
 }
 expect(count).toBeGreaterThan(500);
});

test.skipIf(!python)("the independent Python decoder reads every encoding to the same value, and agrees with the TypeScript decoder on thousands of damaged inputs",()=>{
 const random=seeded(7),rows:{message:string;hex:string}[]=[];
 for(const name of names)for(let round=0;round<4;round++){
  const bytes=encodeProtobufV1(name,sample(random,name,0));rows.push({message:name,hex:Buffer.from(bytes).toString("hex")});
  for(let damage=0;damage<4;damage++){
   const copy=Uint8Array.from(bytes);
   if(damage===0&&copy.length>1)rows.push({message:name,hex:Buffer.from(copy.subarray(0,Math.floor(random()*copy.length))).toString("hex")});
   else if(copy.length>0){const index=Math.floor(random()*copy.length);copy[index]=Math.floor(random()*256);rows.push({message:name,hex:Buffer.from(copy).toString("hex")});}
  }
 }
 const answers=script([oracle,join(import.meta.dir,"../📋️messages/🔣️.json")],rows.map(row=>JSON.stringify(row)).join("\n")+"\n").trim().split("\n").map(line=>JSON.parse(line) as {ok:boolean;value?:unknown});
 expect(answers.length).toBe(rows.length);
 let valid=0,invalid=0;
 rows.forEach((row,index)=>{
  const answer=answers[index]!;let mine:unknown,ok=true;
  try{mine=plain(decodeProtobufV1(row.message,Uint8Array.from(Buffer.from(row.hex,"hex"))));}catch{ok=false;}
  expect(ok,`${row.message} ${row.hex}`).toBe(answer.ok);
  if(ok){valid++;expect(strip(mine),`${row.message} ${row.hex}`).toEqual(strip(answer.value));}else invalid++;
 });
 expect(valid).toBeGreaterThan(500);expect(invalid).toBeGreaterThan(20);
});

function strip(value:unknown):unknown{
 if(Array.isArray(value))return value.map(strip);
 if(value&&typeof value==="object"){const out:Record<string,unknown>={};for(const [key,item] of Object.entries(value as Record<string,unknown>))if(item!==undefined&&item!==null)out[key]=strip(item);return out;}
 return value;
}

test("proto3 rules: defaults are omitted, repeated scalars are packed, enums accept member names, unknown fields and wrong wire types are skipped, malformed input throws",()=>{
 expect(encodeProtobufV1("build.bazel.remote.execution.v2.Digest",{hash:"",size_bytes:0}).length).toBe(0);
 expect(Buffer.from(encodeProtobufV1("build.bazel.remote.execution.v2.Digest",{hash:"ab",size_bytes:300})).toString("hex")).toBe("0a026162" + "10ac02");
 const caps=encodeProtobufV1("build.bazel.remote.execution.v2.CacheCapabilities",{digest_functions:["SHA256","SHA1"],max_batch_total_size_bytes:4194304});
 expect(Buffer.from(caps).toString("hex")).toBe("0a020102"+"2080808002");
 expect(decodeProtobufV1("build.bazel.remote.execution.v2.CacheCapabilities",caps).digest_functions).toEqual([1,2]);
 expect(decodeProtobufV1("build.bazel.remote.execution.v2.CacheCapabilities",Uint8Array.from([8,1,8,2])).digest_functions).toEqual([1,2]);
 expect(()=>encodeProtobufV1("build.bazel.remote.execution.v2.GetTreeRequest",{page_size:2**31})).toThrow();
 expect(decodeProtobufV1("build.bazel.remote.execution.v2.CacheCapabilities",Uint8Array.from([0x08,0x01,0x08,0x02,0x20,0x05])).max_batch_total_size_bytes).toBe(5);
 expect(decodeProtobufV1("build.bazel.remote.execution.v2.Digest",Uint8Array.from([0x78,0x01,0x0a,0x01,0x61])).hash).toBe("a");
 expect(()=>decodeProtobufV1("build.bazel.remote.execution.v2.Digest",Uint8Array.from([0x0a,0x05,0x61]))).toThrow();
 expect(()=>decodeProtobufV1("build.bazel.remote.execution.v2.Digest",Uint8Array.from([0x0a,0x01,0xff]))).toThrow();
 expect(()=>encodeProtobufV1("build.bazel.remote.execution.v2.Nope",{})).toThrow();
 expect(protobufEnumV1("build.bazel.remote.execution.v2.ExecutionStage.Value","EXECUTING")).toBe(3);
 expect(decodeProtobufV1("build.bazel.remote.execution.v2.Digest",encodeProtobufV1("build.bazel.remote.execution.v2.Digest",{hash:"f",size_bytes:2**40})).size_bytes).toBe(2**40);
 expect(decodeProtobufV1("build.bazel.remote.execution.v2.GetTreeRequest",encodeProtobufV1("build.bazel.remote.execution.v2.GetTreeRequest",{page_size:-1})).page_size).toBe(-1);
});

import {connectGrpcV1,GRPC_CODES_V1,GrpcErrorV1,startGrpcServerV1,type GrpcServerV1} from "../🚪️grpc/🟦️.ts";
import {gzipSync} from "node:zlib";

const AC="build.bazel.remote.execution.v2.ActionCache/GetActionResult",CAPS="build.bazel.remote.execution.v2.Capabilities/GetCapabilities",WAIT="build.bazel.remote.execution.v2.Execution/WaitExecution",READ="google.bytestream.ByteStream/Read",WRITE="google.bytestream.ByteStream/Write";
let observed:{aborted:boolean;metadata?:string}={aborted:false};

async function listen(extra:Record<string,(input:any,call:any)=>any>={},authorize?:(call:{method:string;metadata:Readonly<Record<string,string>>})=>void):Promise<GrpcServerV1>{
 return startGrpcServerV1({host:"127.0.0.1",port:0,authorize,handlers:{
  [AC]:async request=>{
   if(request.instance_name==="fail")throw new GrpcErrorV1(GRPC_CODES_V1.FAILED_PRECONDITION,"bad ünïcode %message",Uint8Array.from([1,2,3]));
   if(request.instance_name==="crash")throw new Error("boom");
   return {exit_code:request.action_digest.size_bytes,stdout_raw:new TextEncoder().encode(request.instance_name)};
  },
  [CAPS]:async(_,call)=>{observed.metadata=call.metadata["x-probe"];return {high_api_version:{major:2,minor:3}};},
  [WAIT]:async function*(request,call){
   call.signal.addEventListener("abort",()=>{observed.aborted=true;},{once:true});
   for(let index=0;index<Number(request.name.replace("operations/",""));index++){yield {name:`${request.name}/${index}`,done:false};}
   if(request.name==="operations/0")await new Promise(resolve=>setTimeout(resolve,3000));
  },
  [READ]:async function*(request){const total=Number(request.resource_name),block=new Uint8Array(65536).fill(7);for(let sent=0;sent<total;sent+=block.length)yield {data:block};},
  [WRITE]:async requests=>{let size=0,name="";for await(const message of requests){if(message.resource_name)name=message.resource_name;size+=message.data.length;}return {committed_size:size,...(name===""?{}:{})};},
  ...extra,
 }});
}

test("gRPC: unary, server-streaming and client-streaming calls round trip with statuses, details, deadlines, cancellation, compression and large messages",async()=>{
 observed={aborted:false};
 const server=await listen(),client=connectGrpcV1(server.url);
 try{
  expect(await client.unary(AC,{instance_name:"main",action_digest:{hash:"a",size_bytes:42}})).toMatchObject({exit_code:42});
  const answer=await client.unary(AC,{instance_name:"ünï",action_digest:{hash:"a",size_bytes:1}});expect(new TextDecoder().decode(answer.stdout_raw)).toBe("ünï");
  const failed=await client.unary(AC,{instance_name:"fail"}).catch(error=>error as GrpcErrorV1);expect(failed).toBeInstanceOf(GrpcErrorV1);expect([failed.code,failed.message,Array.from(failed.details??[])]).toEqual([GRPC_CODES_V1.FAILED_PRECONDITION,"bad ünïcode %message",[1,2,3]]);
  expect((await client.unary(AC,{instance_name:"crash"}).catch(error=>error as GrpcErrorV1)).code).toBe(GRPC_CODES_V1.INTERNAL);
  expect((await client.unary("google.longrunning.Operations/GetOperation",{name:"x"}).catch(error=>error as GrpcErrorV1)).code).toBe(GRPC_CODES_V1.UNIMPLEMENTED);
  expect((await client.unary(CAPS,{},{metadata:{"x-probe":"yes"}})).high_api_version).toEqual({major:2,minor:3,patch:0,prerelease:""});expect(observed.metadata).toBe("yes");
  const streamed:string[]=[];for await(const operation of client.stream(WAIT,{name:"operations/3"}))streamed.push(operation.name);expect(streamed).toEqual(["operations/3/0","operations/3/1","operations/3/2"]);
  const controller=new AbortController(),first=client.stream(WAIT,{name:"operations/0"},{signal:controller.signal});
  const reading=(async()=>{try{for await(const _ of first);}catch(error){return error;}return undefined;})();
  setTimeout(()=>controller.abort(),150);await reading;await Bun.sleep(100);expect(observed.aborted,"a cancelled call aborts the handler's signal").toBe(true);
  const late=await (async()=>{for await(const _ of client.stream(WAIT,{name:"operations/0"},{timeoutMilliseconds:200}));})().catch(error=>error as GrpcErrorV1) as GrpcErrorV1;expect(late.code).toBe(GRPC_CODES_V1.DEADLINE_EXCEEDED);
  const total=20*1048576;let read=0;for await(const piece of client.stream(READ,{resource_name:String(total)}))read+=piece.data.length;expect(read).toBe(total);
  const upload=await client.upload(WRITE,function*(){yield {resource_name:"r",data:new Uint8Array(3*1048576).fill(1)};yield {data:new Uint8Array(1048576).fill(2)};}());expect(upload.committed_size).toBe(4*1048576);
 }finally{client.close();await server.stop();}
});

test("gRPC: the server decompresses gzip frames, refuses oversized messages and unauthorized calls, and answers a plain curl client over HTTP/2 prior knowledge",async()=>{
 const server=await listen({},call=>{if(call.metadata.authorization!=="Bearer secret"&&call.method.endsWith("GetActionResult"))throw new GrpcErrorV1(GRPC_CODES_V1.UNAUTHENTICATED,"Unauthorized");}),url=server.url.replace("grpc://","http://");
 try{
  const body=encodeProtobufV1("build.bazel.remote.execution.v2.GetActionResultRequest",{instance_name:"main",action_digest:{hash:"h",size_bytes:9}}),framed=(payload:Uint8Array,flag:number)=>{const out=new Uint8Array(5+payload.length);new DataView(out.buffer).setUint32(1,payload.length);out[0]=flag;out.set(payload,5);return out;};
  const call=async(frame:Uint8Array,extra:string[]=[])=>{
   const scratch=mkdtempSync(join(tmpdir(),"rxi-curl-")),input=join(scratch,"in.bin");writeFileSync(input,frame);
   const process=Bun.spawn(["curl","-s","--http2-prior-knowledge","-i","-X","POST",`${url}/${AC}`,"-H","content-type: application/grpc","-H","te: trailers",...extra,"--data-binary",`@${input}`],{stdout:"pipe",stderr:"pipe"}),text=await new Response(process.stdout).text();
   await process.exited;rmSync(scratch,{recursive:true,force:true});return text;
  };
  const ok=await call(framed(body,0),["-H","authorization: Bearer secret"]);expect(ok).toMatch(/HTTP\/2 200/u);expect(ok).toMatch(/grpc-status: 0/u);
  const zipped=await call(framed(gzipSync(body),1),["-H","authorization: Bearer secret","-H","grpc-encoding: gzip"]);expect(zipped).toMatch(/grpc-status: 0/u);
  expect(await call(framed(gzipSync(body),1),["-H","authorization: Bearer secret"])).toMatch(/grpc-status: 12/u);
  expect(await call(framed(body,0))).toMatch(/grpc-status: 16/u);
  const huge=new Uint8Array(40*1048576);expect(await call(framed(huge,0),["-H","authorization: Bearer secret"])).toMatch(/grpc-status: 8/u);
  expect(await call(framed(body,0).subarray(0,9),["-H","authorization: Bearer secret"])).toMatch(/grpc-status: 13/u);
 }finally{await server.stop();}
});

test.skipIf(Bun.spawnSync(["openssl","version"],{stdout:"ignore",stderr:"ignore"}).exitCode!==0)("gRPC over TLS: a client that trusts the certificate calls the server, an untrusting client and a plain client are refused, and curl negotiates h2 over ALPN",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"rxi-tls-")),certificate=join(directory,"cert.pem"),key=join(directory,"key.pem");
 try{
  const made=Bun.spawnSync(["openssl","req","-x509","-newkey","rsa:2048","-nodes","-keyout",key,"-out",certificate,"-days","1","-subj","/CN=localhost","-addext","subjectAltName=DNS:localhost,IP:127.0.0.1"],{stdout:"ignore",stderr:"pipe"});
  expect(made.exitCode,made.stderr.toString()).toBe(0);
  const server=await startGrpcServerV1({host:"127.0.0.1",port:0,tls:{certificate,key},handlers:{[AC]:async request=>({exit_code:request.action_digest.size_bytes})}});
  try{
   expect(server.url.startsWith("grpcs://")).toBe(true);
   const trusting=connectGrpcV1(server.url.replace("127.0.0.1","localhost"),{certificateAuthority:certificate});
   try{expect((await trusting.unary(AC,{instance_name:"main",action_digest:{hash:"a",size_bytes:7}})).exit_code).toBe(7);}finally{trusting.close();}
   const stranger=connectGrpcV1(server.url.replace("127.0.0.1","localhost"));
   try{expect((await stranger.unary(AC,{instance_name:"main",action_digest:{hash:"a",size_bytes:7}}).catch(error=>error as GrpcErrorV1) as GrpcErrorV1).code).toBe(GRPC_CODES_V1.UNAVAILABLE);}finally{stranger.close();}
   const plain=connectGrpcV1(server.url.replace("grpcs://","grpc://"));
   try{expect((await plain.unary(AC,{instance_name:"main",action_digest:{hash:"a",size_bytes:7}},{timeoutMilliseconds:1500}).catch(error=>error as GrpcErrorV1) as GrpcErrorV1).code).not.toBe(0);}finally{plain.close();}
   const body=encodeProtobufV1("build.bazel.remote.execution.v2.GetActionResultRequest",{instance_name:"main",action_digest:{hash:"h",size_bytes:9}}),frame=new Uint8Array(5+body.length);new DataView(frame.buffer).setUint32(1,body.length);frame.set(body,5);writeFileSync(join(directory,"in.bin"),frame);
   const curl=Bun.spawn(["curl","-s","-i","--cacert",certificate,"--http2","-X","POST",`https://localhost:${server.port}/${AC}`,"-H","content-type: application/grpc","-H","te: trailers","--data-binary",`@${join(directory,"in.bin")}`],{stdout:"pipe",stderr:"pipe"}),text=await new Response(curl.stdout).text();await curl.exited;
   expect(text).toMatch(/HTTP\/2 200/u);expect(text).toMatch(/grpc-status: 0/u);
  }finally{await server.stop();}
 }finally{rmSync(directory,{recursive:true,force:true});}
});
