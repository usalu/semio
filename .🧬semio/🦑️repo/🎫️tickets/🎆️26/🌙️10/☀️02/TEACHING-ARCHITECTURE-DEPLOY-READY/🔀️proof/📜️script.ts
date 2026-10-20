import assert from "node:assert/strict";
import{readFileSync,writeFileSync}from"node:fs";
import{resolve,join}from"node:path";
import{pathToFileURL}from"node:url";
import{createHash}from"node:crypto";
import{createRequire}from"node:module";

const ticket=resolve(import.meta.dir,".."),root=process.cwd(),reference=process.argv[2];
assert(/^🗑️generated\/[a-zA-Z0-9.-]+\.json$/u.test(reference??""));
const handoff=JSON.parse(readFileSync(join(ticket,reference),"utf8")),fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️.json"),"utf8")),owner="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory",paths=["🔀️exchange/🟦️.ts","🧬️schema/🔣️.json","🧪️tests/🟦️.ts","🧪️tests/🧫️fixtures/🔣️.json"].map(path=>owner+"/"+path),hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
assert.equal(handoff.version,1);assert.equal(handoff.rows.length,4);
const captures=paths.map(path=>{const row=handoff.rows.find((value:any)=>value.path===path);assert(row);const bytes=readFileSync(join(root,path));assert.equal(bytes.length,row.bytes);assert.equal(hash(bytes),row.sha256);return{path,bytes,sha256:row.sha256};});
const exchange=captures[0],schema=JSON.parse(captures[1].bytes.toString()),protocol=schema.definitions.Protocol.const,ts=createRequire(join(root,"package.json"))("typescript"),parsed=ts.createSourceFile(exchange.path,exchange.bytes.toString(),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS),factory=parsed.statements.filter((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text==="createNativeDirectoryExchangePortV1");
assert.equal(factory.length,1);assert.equal(parsed.parseDiagnostics.length,0);
const factorySource=factory[0].getText(parsed);assert(factorySource.startsWith("export function "));
const factoryJs=new Bun.Transpiler({loader:"ts",target:"node"}).transformSync(factorySource.slice("export ".length));

async function nodeProof(input:any){
 const assert=(await import("node:assert/strict")).default,fs=await import("node:fs"),path=await import("node:path");
 assert.equal(process.release.name,"node");assert.equal((process.versions as any).bun,undefined);
 const actual=await import(input.exchangeUrl),base=fs.mkdtempSync(path.join(input.generated,"exchange-neutral-")),first=path.join(base,"first"),second=path.join(base,"second"),results:any[]=[];
 try{
  fs.mkdirSync(first);fs.mkdirSync(second);
  const identity=(value:string)=>{const row=fs.lstatSync(value,{bigint:true});return[String(row.dev),String(row.ino)];},before=[identity(first),identity(second)],port=actual.createNativeDirectoryExchangePortV1();
  assert.equal(port.kind,"exchange");assert(Object.isFrozen(port));assert.equal(port.pending(base),false);assert.equal(port.recover(base),"clean");assert.equal(port.prepareVolume(base),undefined);
  if(process.platform==="darwin"||process.platform==="linux")port.flush(base);
  let refusal:any;try{port.swap(first,second);}catch(error){refusal=error;}
  assert(refusal);if(process.platform==="darwin"||process.platform==="linux"){assert.equal(refusal.code,"MODULE_NOT_FOUND");assert.match(refusal.message,/bun:ffi/u);}else assert.match(refusal.message,/requires Linux or macOS/u);
  assert.deepEqual([identity(first),identity(second)],before);
  results.push({id:"actual-node-source",node:process.version,platform:process.platform,construction:true,nonSwap:true,swapRefused:true,code:refusal.code??null,message:refusal.message,identitiesUnchanged:true});
  const construct=(bind:()=>unknown,flush:(value:string)=>void,platform:string)=>new Function("nativeDirectoryExchangeCallV1","flushNativePosixDirectoryV1","exchangeRow","protocol",input.factoryJs+";return createNativeDirectoryExchangePortV1;")(bind,flush,()=>input.protocol.exchange[platform],input.protocol);
  for(const row of input.fixture.lifecycle){
   let binds=0,calls=0,refusals=0,defaultFlush=0,injectedFlush=0;const seen:any[]=[],failure=new Error("controlled binder failure"),call={exchange:(a:string,b:string)=>{calls++;seen.push([a,b]);return{status:0,errno:0};}},create=construct(()=>{binds++;if(binds<=row.failures)throw failure;return call;},()=>{defaultFlush++;},"linux"),ports=[];
   for(let index=0;index<row.ports;index++){const port=create(row.injected?call:undefined,row.injectedFlush?()=>{injectedFlush++;}:undefined);ports.push(port);assert(Object.isFrozen(port));assert.equal(port.kind,"exchange");assert.equal(port.pending("neutral"),false);assert.equal(port.recover("neutral"),"clean");port.prepareVolume("neutral");port.flush("neutral");}
   assert.equal(binds,0,"Construction and non-swap methods must not bind");
   for(const index of row.swaps){try{ports[index].swap("first","second");}catch(error){assert.equal(error,failure,"Failed bind preserves exact reason");refusals++;}}
   assert.equal(binds,row.binds);assert.equal(calls,row.calls);assert.equal(refusals,row.refusals);assert.equal(defaultFlush,row.injectedFlush?0:row.ports);assert.equal(injectedFlush,row.injectedFlush?row.ports:0);assert.deepEqual(seen,Array.from({length:row.calls},()=>["first","second"]));results.push({id:row.id,binds,calls,refusals,defaultFlush,injectedFlush});
  }
  for(const row of input.fixture.outcomes){
   let binds=0,calls=0;const create=construct(()=>{binds++;throw Error("Injected call must bypass default binder");},()=>{},row.platform),port=create({exchange:(a:string,b:string)=>{calls++;assert.deepEqual([a,b],["first","second"]);return{status:row.status,errno:row.errno};}});let failure:any;
   try{port.swap("first","second");}catch(error){failure=error;}
   const outcome=!failure?"success":failure.message.startsWith(input.protocol.refusals.unsupported)?"unsupported":"failed";assert.equal(outcome,row.outcome);assert.equal(calls,1);assert.equal(binds,0);if(outcome==="unsupported")assert.equal(failure.message,input.protocol.refusals.unsupported+" (errno "+row.errno+")");if(outcome==="failed")assert.equal(failure.message,"Directory exchange failed (status "+row.status+", errno "+row.errno+")");results.push({id:row.id,outcome,calls,binds,message:failure?.message??null});
  }
  console.log(JSON.stringify({results,node:process.version,scope:"Actual Node Source import and actual factory body under declared test ports; no Native swap or product build"}));
 }finally{fs.rmSync(base,{recursive:true,force:true});}
}

const input={exchangeUrl:pathToFileURL(join(root,exchange.path)).href,generated:join(ticket,"🗑️generated"),protocol,fixture,factoryJs},driver=new Bun.Transpiler({loader:"ts",target:"node"}).transformSync("const proof="+nodeProof.toString()+";"),node=Bun.spawn(["node","--input-type=module","-e",driver+"let input='';for await(const chunk of process.stdin)input+=chunk;await proof(JSON.parse(input)).catch(error=>{console.error(error);process.exitCode=1;});"],{stdin:"pipe",stdout:"pipe",stderr:"pipe"}),timeout=setTimeout(()=>node.kill(),20000);node.stdin.write(JSON.stringify(input));node.stdin.end();
let stdout:string,stderr:string,status:number;try{[stdout,stderr,status]=await Promise.all([new Response(node.stdout).text(),new Response(node.stderr).text(),node.exited]);}finally{clearTimeout(timeout);}
assert.equal(status!,0,stderr!);const observed=JSON.parse(stdout!);
const oracleProgram="import json,sys,sqlite3\nv=json.load(sys.stdin);c=sqlite3.connect(':memory:');c.execute('create table outcomes(id text,platform text,status integer,errno integer)');c.execute('create table unsupported(platform text,errno integer)');c.executemany('insert into outcomes values(?,?,?,?)',[(r['id'],r['platform'],r['status'],r['errno'])for r in v['fixture']['outcomes']]);c.executemany('insert into unsupported values(?,?)',[(p,e)for p in ['linux','darwin']for e in v['protocol']['exchange'][p]['unsupported']]);o=[dict(zip(['id','outcome'],r))for r in c.execute(\"select id,case when status=0 then 'success' when exists(select 1 from unsupported u where u.platform=o.platform and u.errno=o.errno)then 'unsupported' else 'failed' end from outcomes o order by rowid\")];c.execute('create table lifecycle(id text,ports integer,failures integer,injected integer,swaps integer)');c.executemany('insert into lifecycle values(?,?,?,?,?)',[(r['id'],r['ports'],r['failures'],int(r['injected']),len(r['swaps']))for r in v['fixture']['lifecycle']]);l=[dict(zip(['id','binds','calls','refusals'],r))for r in c.execute('select id,case when injected then 0 else min(swaps,ports+failures)end,max(0,swaps-failures),min(swaps,failures)from lifecycle order by rowid')];print(json.dumps({'outcomes':o,'lifecycle':l}))";
const oracle=Bun.spawn(["python3","-c",oracleProgram],{stdin:"pipe",stdout:"pipe",stderr:"pipe"});oracle.stdin.write(JSON.stringify({fixture,protocol}));oracle.stdin.end();const[out,err,code]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);assert.equal(code,0,err);const expected=JSON.parse(out);
for(const row of expected.outcomes)assert.equal(observed.results.find((value:any)=>value.id===row.id).outcome,row.outcome);
for(const row of expected.lifecycle){const actual=observed.results.find((value:any)=>value.id===row.id);for(const key of["binds","calls","refusals"])assert.equal(actual[key],row[key]);}
for(const row of captures)assert.equal(hash(readFileSync(join(root,row.path))),row.sha256,"Candidate Source read fence");
const result={version:1,scope:observed.scope,node:observed.node,sourceRows:captures.map(({path,sha256,bytes})=>({path,sha256,bytes:bytes.length})),factorySourceSha256:hash(factorySource),factoryJsSha256:hash(factoryJs),fixtureSha256:hash(readFileSync(join(import.meta.dir,"🧫️.json"))),results:observed.results,oracle:expected,stderr:stderr!};writeFileSync(join(ticket,"🗑️generated/2026-10-20-release-exchange-node-proof.json"),JSON.stringify(result,null,2)+"\n");console.log("[DEBUG] "+JSON.stringify({cases:result.results.length,node:result.node,sourceRows:result.sourceRows.length,factorySourceSha256:result.factorySourceSha256,oracle:"Python SQLite protocol/lifecycle",scope:result.scope}));
