import assert from "node:assert/strict";
import {constants} from "node:fs";
import {readFile,writeFile,mkdir,statfs,lstat,open,rename} from "node:fs/promises";
import {createHash} from "node:crypto";
import {execFile} from "node:child_process";
import {createRequire} from "node:module";
import {dirname,join,resolve} from "node:path";
import {Readable,Transform,Writable} from "node:stream";
import {pipeline} from "node:stream/promises";
import {createGzip,createGunzip} from "node:zlib";
import {promisify} from "node:util";

type Controls={maximumBytes:number;maximumWork:number;budgetMs:number;cancelled?:boolean;chunkBytes?:number};
type Identity={device:string;inode:string;bytes:string;mtimeNs:string;ctimeNs:string};
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),require=createRequire(join(root,"package.json")),[command,epoch]=process.argv.slice(2),sha=(body:string|Buffer)=>createHash("sha256").update(body).digest("hex");
assert.ok(command==="laws"||command==="archive"||command==="guards");assert.match(epoch??"",/^\d+$/u);
const {validateJsonSchemaSubset}=await import(join(root,"🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
const validate=(schema:any,value:any)=>{assert.deepEqual(validateJsonSchemaSubset(schema,value),[]);assert.equal(new(require("ajv"))({strict:true}).compile(schema)(value),true);};
class Budget{
  readonly signal=new AbortController();readonly started=performance.now();bytes=0;work=0;lastProgress=0;timer:ReturnType<typeof setTimeout>;cancel=()=>this.signal.abort(Error("cancelled"));
  constructor(readonly controls:Controls){this.timer=setTimeout(()=>this.signal.abort(Error("deadline")),controls.budgetMs);process.on("SIGINT",this.cancel);process.on("SIGTERM",this.cancel);}
  take(bytes=0,work=1){if(this.controls.cancelled||this.signal.signal.aborted)throw this.signal.signal.reason??Error("cancelled");if(performance.now()-this.started>=this.controls.budgetMs)throw Error("deadline");if(this.bytes+bytes>this.controls.maximumBytes)throw Error("byte-capacity");if(this.work+work>this.controls.maximumWork)throw Error("work-capacity");this.bytes+=bytes;this.work+=work;const now=performance.now();if(now-this.lastProgress>=2000){console.log(JSON.stringify({progress:{bytes:this.bytes,work:this.work}}));this.lastProgress=now;}}
  close(){clearTimeout(this.timer);process.off("SIGINT",this.cancel);process.off("SIGTERM",this.cancel);}
}
const meter=(budget:Budget,hash=undefined as ReturnType<typeof createHash>|undefined)=>new Transform({transform(chunk,_encoding,done){try{budget.take(chunk.length);hash?.update(chunk);done(null,chunk);}catch(error){done(error as Error);}}});
const encode=async(body:Buffer,controls:Controls)=>{const budget=new Budget(controls),parts:Buffer[]=[];try{budget.take();await pipeline(Readable.from([body]),meter(budget),createGzip({chunkSize:1048576}),meter(budget),new Writable({write(chunk,_encoding,done){parts.push(Buffer.from(chunk));done();}}),{signal:budget.signal.signal});return Buffer.concat(parts);}finally{budget.close();}};
const identity=(value:any):Identity=>({device:String(value.dev),inode:String(value.ino),bytes:String(value.size),mtimeNs:String(value.mtimeNs),ctimeNs:String(value.ctimeNs)});
const readChunks=async function*(handle:any,bytes:number,chunkBytes:number,budget:Budget){let offset=0;while(offset<bytes){budget.take();const buffer=Buffer.allocUnsafe(Math.min(chunkBytes,bytes-offset)),result=await handle.read(buffer,0,buffer.length,offset);assert.ok(result.bytesRead>0);offset+=result.bytesRead;yield buffer.subarray(0,result.bytesRead);}assert.equal(offset,bytes);};
const output=join(ticket,"🗑️generated/root-closed-output-archive",command+"-"+epoch);await mkdir(output,{recursive:false});
const guards=new Map<string,string>(),guard=async(path:string)=>{const body=await readFile(path,"utf8");guards.set(path,body);return body;},exactGuards=async()=>{for(const[path,body]of guards)assert.ok(await readFile(path,"utf8")===body,"Input advanced: "+path);},producer=await guard(import.meta.path);
const censusFor=(budget:Budget,request:Controls,output:string)=>async(path:string,id:number,phase:string)=>{
    const exec=promisify(execFile);
    assert.ok(process.platform==="darwin"||process.platform==="linux","Native open-handle census unavailable on this host");
    budget.take();const timeout=Math.max(1,Math.ceil(request.budgetMs-(performance.now()-budget.started)));
    const processes=await exec("ps",["-axo","pid=,ppid=,lstart=,command="],{timeout,maxBuffer:4*1048576,signal:budget.signal.signal});budget.take(Buffer.byteLength(processes.stdout)+Buffer.byteLength(processes.stderr));assert.equal(processes.stderr,"");
    let opened:{stdout:string;stderr:string;code:number};try{const result=await exec("lsof",["-nP","--",path],{timeout,maxBuffer:4*1048576,signal:budget.signal.signal});opened={stdout:result.stdout,stderr:result.stderr,code:0};}catch(error){const value=error as any;assert.equal(value.code,1);opened={stdout:value.stdout,stderr:value.stderr,code:value.code};}
    budget.take(Buffer.byteLength(opened.stdout)+Buffer.byteLength(opened.stderr));
    await writeFile(join(output,"census-"+id+"-"+phase+".json"),JSON.stringify({path,processes,opened,observedAt:new Date().toISOString(),futureLeaseClaimed:false}));
    assert.equal(opened.code,1);assert.equal(opened.stdout,"");assert.equal(opened.stderr,"");
  };
if(command==="laws"){
  const fixtureSource=await guard(join(import.meta.dir,"🧫️fixtures/🔣️.json")),schemaSource=await guard(join(import.meta.dir,"🧬️schema/🧪️laws.json")),fixture=JSON.parse(fixtureSource);validate(JSON.parse(schemaSource),fixture);
  const guardFixtureSource=await guard(join(import.meta.dir,"🧫️fixtures/🛡️guards.json")),guardSchemaSource=await guard(join(import.meta.dir,"🧬️schema/🛡️guard-archive.json")),guardFixture=JSON.parse(guardFixtureSource);for(const row of guardFixture.cases){assert.ok(Object.hasOwn(row,"authoritySchema"),"Explicit guard fixture authority required");const guardSchema=row.authoritySchema,guardOracle=new(require("ajv"))({strict:true}).compile(guardSchema);assert.equal(validateJsonSchemaSubset(guardSchema,row.carrier).length===0,row.expectedValid,row.id);assert.equal(guardOracle(row.carrier),row.expectedValid,row.id);assert.deepEqual(JSON.parse(JSON.stringify(row.carrier)),row.carrier);}
  const rows:any[]=[];let failures=0;
  for(const row of fixture.cases){const body=Buffer.from(row.bodyBase64,"base64");assert.equal(body.toString("base64"),row.bodyBase64);let actual:string;try{const encoded=await encode(body,row),decoded=Buffer.from(require("fflate").gunzipSync(encoded));assert.ok(decoded.equals(body));actual="round-trip";}catch(error){actual=(error as Error).message;}rows.push({id:row.id,expected:row.expected,actual});if(actual!==row.expected)failures++;}
  await exactGuards();const capacity=await statfs(ticket);await writeFile(join(output,"terminal.json"),JSON.stringify({producer,fixtureSource,schemaSource,guardFixtureSource,guardSchemaSource,guardMetadataLaws:guardFixture.cases.length,rows,failures,firstPartySchema:true,strictAjv:true,thirdPartyOracle:"fflate",availableBytes:capacity.bavail*capacity.bsize}));console.log(JSON.stringify({rows,failures}));process.exitCode=failures?1:0;
}else if(command==="archive"){
  const requestSource=await guard(join(import.meta.dir,"🔣️.json")),policySource=await guard(join(import.meta.dir,"🧬️schema/🔣️policy.json")),carrierSource=await guard(join(import.meta.dir,"🧬️schema/📦️archive.json")),request=JSON.parse(requestSource),carrierSchema=JSON.parse(carrierSource);validate(JSON.parse(policySource),request);
  const auditSource=await guard(join(ticket,request.audit)),audit=JSON.parse(auditSource),closureReport=join(ticket,"ROOT-CLOSED-OUTPUT-LOSSLESS-ARCHIVE-AUDIT-20.md");await guard(closureReport);
  assert.equal(request.cases.length,audit.files.length);assert.equal(new Set(request.cases.map((row:any)=>row.path)).size,request.cases.length);
  for(const row of request.cases){const expected=audit.files.find((candidate:any)=>candidate.epoch===row.epoch);assert.ok(expected);for(const key of Object.keys(row))assert.equal(row[key],expected[key]);assert.equal(row.path,join(ticket,"🗑️generated/root-launch-seed/registration-"+row.epoch+".json"));assert.notEqual(row.epoch,225);}
  const budget=new Budget(request),rows:any[]=[],admission=join(output,"admission.json"),phaseJournal=join(output,"custody.json");
  const census=censusFor(budget,request,output);
  const readOwned=async(row:any,expectedIdentity?:Identity)=>{
    budget.take();const pathStat=await lstat(row.path,{bigint:true});assert.ok(pathStat.isFile()&&!pathStat.isSymbolicLink());const found=identity(pathStat);
    assert.equal(found.device,String(row.device));assert.equal(found.inode,String(row.inode));assert.equal(found.bytes,String(row.bytes));if(expectedIdentity)assert.deepEqual(found,expectedIdentity);
    const handle=await open(row.path,constants.O_RDONLY|constants.O_NOFOLLOW);try{assert.deepEqual(identity(await handle.stat({bigint:true})),found);const body=await handle.readFile();budget.take(body.length);assert.equal(body.length,row.bytes);assert.equal(sha(body),row.sha256);assert.deepEqual(identity(await handle.stat({bigint:true})),found);assert.deepEqual(identity(await lstat(row.path,{bigint:true})),found);return{body,identity:found};}finally{await handle.close();}
  };
  const journal=async(phase:string,error?:string)=>writeFile(phaseJournal,JSON.stringify({version:1,phase,rows,error,bytes:budget.bytes,work:budget.work,atomicBatchClaimed:false}));
  try{
    budget.take();await writeFile(admission,JSON.stringify({version:1,guards:[...guards].map(([path,body])=>({path,body,sha256:sha(body)})),finiteEpochs:request.cases.map((row:any)=>row.epoch),sourcePublicationAuthority:false,atomicBatchClaimed:false}),{flag:"wx"});
    await journal("admitted");
    for(const row of request.cases){
      await exactGuards();await census(row.path,row.epoch,"before");
      const original=await readOwned(row),value=JSON.parse(original.body.toString("utf8"));assert.equal(value.phase,row.phase);assert.equal(value.producer.path,join(ticket,"root-launch-seed-inputs/📜️script.ts"));assert.equal(typeof value.producer.source,"string");assert.equal(value.pairs.length,2);
      for(const pair of value.pairs){assert.ok([join(root,".vscode/🧩️launch.seed.jsonc"),join(root,".vscode/launch.json")].includes(pair.path));assert.equal(sha(pair.before),pair.beforeSha256);assert.equal(sha(pair.after),pair.afterSha256);assert.ok(pair.inverse===pair.before);}
      assert.equal(new Set(value.pairs.map((pair:any)=>pair.path)).size,2);
      const payloadPath=join(output,"registration-"+row.epoch+".json.gz"),receiptPath=join(output,"verification-"+row.epoch+".json"),payload=await open(payloadPath,"wx"),source=await open(row.path,constants.O_RDONLY|constants.O_NOFOLLOW),encodedHash=createHash("sha256"),sourceHash=createHash("sha256");let encodedBytes=0;
      try{assert.deepEqual(identity(await source.stat({bigint:true})),original.identity);await pipeline(Readable.from(readChunks(source,row.bytes,request.chunkBytes,budget)),meter(budget,sourceHash),createGzip({chunkSize:request.chunkBytes}),meter(budget,encodedHash),new Writable({write(chunk,_encoding,done){encodedBytes+=chunk.length;payload.writeFile(chunk).then(()=>done(),error=>done(error));}}),{signal:budget.signal.signal});await payload.sync();assert.equal(sourceHash.digest("hex"),row.sha256);assert.deepEqual(identity(await source.stat({bigint:true})),original.identity);}finally{await source.close();await payload.close();}
      const decodedHash=createHash("sha256"),compressedHash=createHash("sha256"),compressed=await open(payloadPath,"r");let decodedBytes=0;
      try{await pipeline(Readable.from(readChunks(compressed,encodedBytes,request.chunkBytes,budget)),meter(budget,compressedHash),createGunzip({chunkSize:request.chunkBytes}),new Writable({write(chunk,_encoding,done){try{budget.take(chunk.length);decodedBytes+=chunk.length;assert.ok(decodedBytes<=row.bytes);decodedHash.update(chunk);done();}catch(error){done(error as Error);}}}),{signal:budget.signal.signal});}finally{await compressed.close();}
      assert.equal(decodedBytes,row.bytes);assert.equal(decodedHash.digest("hex"),row.sha256);const encodedSha256=encodedHash.digest("hex");assert.equal(compressedHash.digest("hex"),encodedSha256);assert.equal(Number((await lstat(payloadPath)).size),encodedBytes);
      const carrier={version:1,kind:"root-closed-json-gzip",originalPath:row.path,custodyPhase:row.phase,epoch:row.epoch,producerPath:value.producer.path,producerSha256:sha(value.producer.source),closureReport,originalIdentity:original.identity,decodedBytes,decodedSha256:row.sha256,payloadPath,encodedBytes,encodedSha256,verificationReceipt:receiptPath,atomicBatchClaimed:false};validate(carrierSchema,carrier);
      await writeFile(receiptPath,JSON.stringify({version:1,phase:"verified-before-publication",carrier,admission,fullOriginalBytesPreserved:true,decodedLengthExact:true,decodedHashExact:true,nativeRoundTrip:true,atomicBatchClaimed:false}),{flag:"wx"});
      await census(row.path,row.epoch,"publication");await exactGuards();await readOwned(row,original.identity);budget.take();
      const carrierPath=join(dirname(row.path),".registration-"+row.epoch+"-archive-"+epoch+".json"),carrierBody=JSON.stringify(carrier),temporary=await open(carrierPath,"wx");try{await temporary.writeFile(carrierBody);await temporary.sync();}finally{await temporary.close();}
      await rename(carrierPath,row.path);assert.ok(await readFile(row.path,"utf8")===carrierBody);const decodedCheck=await open(payloadPath,"r");assert.equal(Number((await decodedCheck.stat()).size),encodedBytes);await decodedCheck.close();
      rows.push({epoch:row.epoch,originalPath:row.path,custodyPhase:row.phase,decodedBytes,decodedSha256:row.sha256,encodedBytes,encodedSha256,payloadPath,verificationReceipt:receiptPath,carrierSha256:sha(carrierBody)});
      await writeFile(receiptPath,JSON.stringify({version:1,phase:"published",carrier,admission,fullOriginalBytesPreserved:true,decodedLengthExact:true,decodedHashExact:true,nativeRoundTrip:true,atomicBatchClaimed:false}));await journal("publishing");
      console.log(JSON.stringify({archived:row.epoch,count:rows.length,total:request.cases.length,decodedBytes,encodedBytes}));
    }
    await exactGuards();await journal("completed");const capacity=await statfs(ticket);await writeFile(join(output,"terminal.json"),JSON.stringify({phase:"completed",rows,producerSha256:sha(producer),requestSha256:sha(requestSource),auditSha256:sha(auditSource),fullOriginalBytesPreserved:true,sourceFilesModified:0,atomicBatchClaimed:false,bytes:budget.bytes,work:budget.work,availableBytes:capacity.bavail*capacity.bsize}));console.log(JSON.stringify({completed:rows.length,decodedBytes:rows.reduce((sum,row)=>sum+row.decodedBytes,0),encodedBytes:rows.reduce((sum,row)=>sum+row.encodedBytes,0),work:budget.work}));
  }catch(error){await journal("refused-or-partial",(error as Error).message);throw error;}finally{budget.close();}
}else{
  const requestSource=await guard(join(import.meta.dir,"guards.json")),policySource=await guard(join(import.meta.dir,"🧬️schema/🛡️guard-policy.json")),carrierSource=await guard(join(import.meta.dir,"🧬️schema/🛡️guard-archive.json")),request=JSON.parse(requestSource),carrierSchema=JSON.parse(carrierSource);validate(JSON.parse(policySource),request);
  const inventorySource=await guard(request.inventory),inventory=JSON.parse(inventorySource);assert.deepEqual(request.cases,inventory.rows);assert.match(request.interfaceEpoch,/^[0-9]+$/u);assert.ok(request.cases.length>=1&&request.cases.length<=10);assert.equal(new Set(request.cases.map((row:any)=>row.path)).size,request.cases.length);assert.equal(inventory.archivalExecuted,false);assert.equal(inventory.sourceWrites,0);
  for(const row of request.cases){assert.equal(row.epoch,request.interfaceEpoch);assert.equal(row.producer.path,join(ticket,"cargo-inputs/📥️current-successor-"+row.epoch+"/📜️script.ts"));assert.equal(row.path,join(ticket,"🗑️generated/controlled-cargo-callers/current-cut-observation-"+row.command+(row.position==="post"?"-post":"")+"-"+row.epoch+".json"));assert.ok([0,1].includes(row.physicalOuterClosure.exit_code));assert.equal(sha(await guard(row.producer.path)),row.producer.hash);await guard(row.consumerProof);}
  const budget=new Budget(request),census=censusFor(budget,request,output),rows:any[]=[],admission=join(output,"admission.json"),phaseJournal=join(output,"custody.json");
  const hashOwned=async(row:any)=>{
    budget.take();const before=await lstat(row.path,{bigint:true});assert.ok(before.isFile()&&!before.isSymbolicLink());assert.deepEqual(identity(before),row.identity);
    const handle=await open(row.path,constants.O_RDONLY|constants.O_NOFOLLOW),hash=createHash("sha256");let bytes=0;
    try{assert.deepEqual(identity(await handle.stat({bigint:true})),row.identity);for await(const chunk of readChunks(handle,row.bytes,request.chunkBytes,budget)){budget.take(chunk.length);bytes+=chunk.length;hash.update(chunk);}assert.equal(bytes,row.bytes);assert.equal(hash.digest("hex"),row.sha256);assert.deepEqual(identity(await handle.stat({bigint:true})),row.identity);assert.deepEqual(identity(await lstat(row.path,{bigint:true})),row.identity);}finally{await handle.close();}
  };
  const journal=async(phase:string,error?:string)=>writeFile(phaseJournal,JSON.stringify({version:1,phase,rows,error,bytes:budget.bytes,work:budget.work,atomicBatchClaimed:false}));
  try{
    budget.take();await writeFile(admission,JSON.stringify({version:1,guards:[...guards].map(([path,body])=>({path,body,sha256:sha(body)})),sourcePublicationAuthority:false,failedHistoricalDiagnosisPromoted:false,atomicBatchClaimed:false}),{flag:"wx"});await journal("admitted");
    for(let index=0;index<request.cases.length;index++){
      const row=request.cases[index];await exactGuards();await census(row.path,index,"before");await hashOwned(row);
      const payloadPath=join(output,"guard-"+index+"-"+row.epoch+".json.gz"),receiptPath=join(output,"verification-"+index+".json"),payload=await open(payloadPath,"wx"),source=await open(row.path,constants.O_RDONLY|constants.O_NOFOLLOW),encodedHash=createHash("sha256"),sourceHash=createHash("sha256");let encodedBytes=0;
      try{assert.deepEqual(identity(await source.stat({bigint:true})),row.identity);await pipeline(Readable.from(readChunks(source,row.bytes,request.chunkBytes,budget)),meter(budget,sourceHash),createGzip({chunkSize:request.chunkBytes}),meter(budget,encodedHash),new Writable({write(chunk,_encoding,done){encodedBytes+=chunk.length;payload.writeFile(chunk).then(()=>done(),error=>done(error));}}),{signal:budget.signal.signal});await payload.sync();assert.equal(sourceHash.digest("hex"),row.sha256);assert.deepEqual(identity(await source.stat({bigint:true})),row.identity);}finally{await source.close();await payload.close();}
      const decodedHash=createHash("sha256"),compressedHash=createHash("sha256"),compressed=await open(payloadPath,"r");let decodedBytes=0;
      try{await pipeline(Readable.from(readChunks(compressed,encodedBytes,request.chunkBytes,budget)),meter(budget,compressedHash),createGunzip({chunkSize:request.chunkBytes}),new Writable({write(chunk,_encoding,done){try{budget.take(chunk.length);decodedBytes+=chunk.length;assert.ok(decodedBytes<=row.bytes);decodedHash.update(chunk);done();}catch(error){done(error as Error);}}}),{signal:budget.signal.signal});}finally{await compressed.close();}
      assert.equal(decodedBytes,row.bytes);assert.equal(decodedHash.digest("hex"),row.sha256);const encodedSha256=encodedHash.digest("hex");assert.equal(compressedHash.digest("hex"),encodedSha256);assert.equal(Number((await lstat(payloadPath)).size),encodedBytes);
      const carrier={version:1,kind:"interface-closed-guard-gzip",epoch:row.epoch,originalPath:row.path,command:row.command,position:row.position,producer:row.producer,originalIdentity:row.identity,decodedBytes,decodedSha256:row.sha256,payloadPath,encodedBytes,encodedSha256,custodyInventoryPath:request.inventory,physicalOuterClosure:row.physicalOuterClosure,childClosure:row.childClosure,verificationReceipt:receiptPath,atomicBatchClaimed:false};validate(carrierSchema,carrier);
      await writeFile(receiptPath,JSON.stringify({version:1,phase:"verified-before-publication",carrier,admission,fullOriginalBytesPreserved:true,decodedLengthExact:true,decodedHashExact:true,nativeRoundTrip:true,atomicBatchClaimed:false}),{flag:"wx"});
      await census(row.path,index,"publication");await exactGuards();await hashOwned(row);budget.take();
      const carrierPath=join(dirname(row.path),".closed-guard-"+row.epoch+"-"+index+"-archive-"+epoch+".json"),carrierBody=JSON.stringify(carrier),temporary=await open(carrierPath,"wx");try{await temporary.writeFile(carrierBody);await temporary.sync();}finally{await temporary.close();}
      await rename(carrierPath,row.path);assert.ok(await readFile(row.path,"utf8")===carrierBody);
      rows.push({index,originalPath:row.path,command:row.command,position:row.position,decodedBytes,decodedSha256:row.sha256,encodedBytes,encodedSha256,payloadPath,verificationReceipt:receiptPath,physicalOuterClosure:row.physicalOuterClosure,childClosure:row.childClosure,carrierSha256:sha(carrierBody)});
      await writeFile(receiptPath,JSON.stringify({version:1,phase:"published",carrier,admission,fullOriginalBytesPreserved:true,decodedLengthExact:true,decodedHashExact:true,nativeRoundTrip:true,atomicBatchClaimed:false}));await journal("publishing");console.log(JSON.stringify({archivedGuard:index,count:rows.length,total:request.cases.length,decodedBytes,encodedBytes}));
    }
    await exactGuards();await journal("completed");const capacity=await statfs(ticket);await writeFile(join(output,"terminal.json"),JSON.stringify({phase:"completed",rows,producerSha256:sha(producer),requestSha256:sha(requestSource),inventorySha256:sha(inventorySource),fullOriginalBytesPreserved:true,sourceFilesModified:0,failedHistoricalDiagnosisPromoted:false,atomicBatchClaimed:false,bytes:budget.bytes,work:budget.work,availableBytes:capacity.bavail*capacity.bsize}));console.log(JSON.stringify({completed:rows.length,decodedBytes:rows.reduce((sum,row)=>sum+row.decodedBytes,0),encodedBytes:rows.reduce((sum,row)=>sum+row.encodedBytes,0),work:budget.work}));
  }catch(error){await journal("refused-or-partial",(error as Error).message);throw error;}finally{budget.close();}
}
