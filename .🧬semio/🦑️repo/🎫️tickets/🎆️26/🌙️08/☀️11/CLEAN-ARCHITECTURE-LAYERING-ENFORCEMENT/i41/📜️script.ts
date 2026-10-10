import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,isAbsolute,join,relative,resolve} from "node:path";

const started=performance.now();
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),output=join(ticket,"🗑️generated/i41"),require=createRequire(join(root,"package.json")),ts=require("typescript"),[route,run]=process.argv.slice(2),controller=new AbortController(),stop=()=>controller.abort(),observerPath=join(root,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts"),{observePhysicalFileV1}=await import(observerPath);
const input=JSON.parse(readFileSync(join(import.meta.dir,"manifest.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"schema.json"),"utf8")),admit=new(require("ajv/dist/2020").default)({strict:true}).compile(schema);
assert.equal(admit(input),true,JSON.stringify(admit.errors));
assert.ok(Object.hasOwn(input.routes,route??""),"Unknown in-place route");
assert.ok(run&&/^[a-z][a-z0-9-]{0,31}$/u.test(run),"Explicit unique run identity required");
assert.equal(input.originalLaws.length,41);assert.equal(input.originalRows.length,177);assert.equal(input.retirements.length,8);assert.equal(input.portableCases.length,36);
process.once("SIGINT",stop);process.once("SIGTERM",stop);
const directory=checked(join(output,run)),tests=input.routes[route!].map((path:string)=>checked(join(root,path))),argv=(route==="fresh-red"||route==="fresh-receiving")?[process.execPath,tests[0]!,"test-fresh-component"]:[process.execPath,"test",...tests,"--timeout",String(route==="invocation"?15000:input.controls.bunTimeoutMilliseconds),...(route==="actor-red"?["--test-name-pattern","actor component custody"]:route==="actor-green"?["--test-name-pattern","actor component custody|ordinary selected Dev actor acquisition|read-only current Dev actors"]:route==="current-red"?["--test-name-pattern","actual durable Cargo producer acquires"]:route==="current-green"?["--test-name-pattern","durable Cargo discovery|actual durable Cargo producer|actual Trunk compiled transformation|actual Cargo proc-macro receipts|ordinary selected Dev actor acquisition|read-only current Dev actors"]:[])];
let chargedUnits=0,chargedOutputBytes=0;
const timeout=setTimeout(stop,input.controls.phaseBudgetMilliseconds);
timeout.unref();
assert.equal(existsSync(directory),false,"A closed epoch cannot be replayed");mkdirSync(directory,{recursive:true});
assert.ok(route==="receiving"||route==="strict"||route==="controller"||route==="controller-test"||route==="async-red"||route==="async-defining"||route==="actor-red"||route==="actor-green"||route==="current-red"||route==="current-green"||route==="cargo-red"||route==="storage-red"||route==="storage-green"||route==="cargo-receiving"||route==="native-dependencies"||route==="command-red"||route==="command-green"||route==="broad-command-green"||route==="broad-command-red"||route==="writer-red"||route==="writer-green"||route==="fresh-red"||route==="fresh-receiving"||route==="native-orchestration"||route==="invocation"||route==="exact-cargo-control","Broader routes await complete explicit dynamic/fixture source admission");
const bindings=route==="exact-cargo-control"?input.exactCargoBindings:route==="invocation"?input.invocationBindings:route==="native-orchestration"?input.nativeOrchestrationBindings:(route==="fresh-red"||route==="fresh-receiving")?input.freshBindings:(route==="writer-red"||route==="writer-green")?input.writerBindings:(route==="broad-command-red"||route==="broad-command-green")?input.broadCommandBindings:(route==="command-red"||route==="command-green")?input.commandBindings:route==="native-dependencies"?input.nativeDependencyBindings:(route==="storage-red"||route==="storage-green")?input.storageBindings:(route==="cargo-red"||route==="cargo-receiving")?input.cargoBindings:route==="receiving"?input.receivingBindings:route==="strict"?input.strictBindings:(route==="actor-red"||route==="actor-green"||route==="current-red"||route==="current-green"||route==="cargo-red")?input.actorBindings:(route==="async-red"||route==="async-defining")?input.asyncBindings:input.controllerBindings;
const absent=route==="broad-command-red"?checked(join(root,input.broadCommandAbsentSource)):route==="command-red"?checked(join(root,input.commandAbsentSource)):route==="storage-red"?checked(join(root,input.storageAbsentSource)):route==="controller"?checked(join(root,input.controllerAbsentSource)):route==="async-red"?checked(join(root,input.asyncAbsentSource)):null;if(absent)assert.equal(existsSync(absent),false,"Defining red preimage changed before dispatch");
const verdictEpoch=run==="native-owner-current-verdict"||run==="native-owner-current-verdict2"||run==="native-owner-interruption"||run==="native-owner-interruption2"||run==="native-owner-interruption3",currentReceiverEpoch=verdictEpoch||run==="native-owner-current-green",featureEpoch=currentReceiverEpoch||run==="native-owner-feature-red"||run==="native-owner-feature-green"||run==="native-owner-feature-green2"||run==="native-owner-feature-green3"||run==="native-owner-feature-green4"||run==="native-owner-feature-green5",custody=featureEpoch?JSON.parse(readFileSync(join(import.meta.dir,verdictEpoch?"feature-verdict-custody.json":currentReceiverEpoch?"feature-current-custody.json":"feature-custody.json"),"utf8")):null;
if(custody){const admits=new(require("ajv").default)({strict:true}).compile(JSON.parse(readFileSync(join(import.meta.dir,verdictEpoch?"feature-verdict-custody-schema.json":currentReceiverEpoch?"feature-current-custody-schema.json":"feature-custody-schema.json"),"utf8")));assert.equal(admits(custody),true,JSON.stringify(admits.errors));}
function receivingSelectors():{path:string;sha256:string;bytes:number}[]{
 return currentReceiverEpoch?custody.testedSelectors.map((selector:any)=>{
 const path=checked(join(root,selector.path)),source=readFileSync(path,"utf8"),syntax=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true);
 assert.equal(syntax.parseDiagnostics.length,0,"Actual receiving selector syntax required");
 const declaration=syntax.statements.find((node:any)=>node.name?.text===selector.declaration),selected=selector.kind==="declaration"?declaration:declaration?.members.find((node:any)=>node.name?.text===selector.method)?.body.statements.find((node:any)=>ts.isIfStatement(node)&&node.expression.getText(syntax)===selector.condition);
 assert.ok(selected,"Actual original receiving selector required");
 const bytes=Buffer.from(selected.getText(syntax));return {path:selector.path,sha256:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.length};
 }):[];
}
const selectorBefore=receivingSelectors(),expectedSelectors=currentReceiverEpoch?custody.testedSelectors.map((row:any)=>({path:row.path,sha256:row.sha256,bytes:row.bytes})):[];
assert.deepEqual(selectorBefore,expectedSelectors,"Original receiving selector advanced before child acquisition");
const criticalPaths=new Set<string>(custody?.criticalSources.map((path:string)=>checked(join(root,path)))??[]);
const paths=await closure([...tests,observerPath,...bindings.map((path:string)=>checked(join(root,path))),...criticalPaths]),before=await claims([...paths,import.meta.filename,join(import.meta.dir,"manifest.json"),join(import.meta.dir,"schema.json"),...criticalPaths]);
writeFileSync(checked(join(directory,"before.json")),JSON.stringify({epoch:41,route,run,argv,controls:input.controls,claims:before,criticalSources:featureEpoch?[...criticalPaths]:null,sourceCustody:custody?.mode??"exact-complete",observedConsumers:custody?.observedConsumers??[],testedSelectors:custody?.testedSelectors??[],selectorBefore,absentSource:absent},null,2));
if(featureEpoch){
 assert.equal([...criticalPaths].every(path=>before.some(row=>row.path===path)),true,"Complete original feature source custody required");
 assert.match(process.argv[4]??"",/^[a-f0-9]{64}$/u,"Literal original feature contract digest required");
 const digest=createHash("sha256").update(JSON.stringify(before.filter(row=>criticalPaths.has(row.path)).map(row=>({path:relative(root,row.path).replaceAll("\\","/"),sha256:row.sha256,bytes:row.bytes})))).digest("hex");
 assert.equal(digest,process.argv[4],"Original feature contract advanced before child acquisition");
}
if(run==="exact-cargo-control-green"||run==="exact-cargo-control-current"||run==="exact-cargo-control-red4"||run==="exact-cargo-control-red3"||run==="exact-cargo-control-red2"||run==="exact-cargo-control-red"||run==="invocation-authority-red2"||run==="invocation-finite-current"||run==="invocation-finite-current2"||run==="native-owner-progress-red"||run==="native-owner-progress-red2"||run==="native-owner-progress-red3"||run==="native-owner-progress-red4"||run==="native-owner-progress-red5"||run==="native-owner-progress-red6"||run==="native-owner-progress-red7"||run==="native-owner-progress-red8"){
 assert.match(process.argv[4]??"",/^[a-f0-9]{64}$/u,"Literal original complete source/input digest required");
 const digest=createHash("sha256").update(JSON.stringify(before.map(row=>({path:relative(root,row.path).replaceAll("\\","/"),sha256:row.sha256,bytes:row.bytes})))).digest("hex");
 assert.equal(digest,process.argv[4],"Literal original complete source/input preimage advanced before child acquisition");
}
if(run==="invocation-authority-red")for(const original of input.authorityPreimage)assert.equal(before.find(row=>row.path===checked(join(root,original.path)))?.sha256,original.sha256,"Original zero/fractional authority defining preimage advanced before dispatch");
if(run==="native-owner-red")assert.equal(before.find(row=>row.path===checked(join(root,input.nativeOwnerPreimage.path)))?.sha256,input.nativeOwnerPreimage.sha256,"Original native receiving owner defining preimage advanced before dispatch");
if(run==="invocation-process-red")assert.equal(before.find(row=>row.path===checked(join(root,input.processPreimage.path)))?.sha256,input.processPreimage.sha256,"Original process capability factory defining preimage advanced before dispatch");
let child:ReturnType<typeof Bun.spawn>|undefined;
try{
 controller.signal.throwIfAborted();
 const artifact=(route==="fresh-red"||route==="fresh-receiving")?checked(join(ticket,run==="fresh-policy-red"?"🗑️generated/gf1":run==="fresh-policy-red2"?"🗑️generated/gf2":run==="fresh-policy-green"?"🗑️generated/gf3":run==="fresh-policy-green2"?"🗑️generated/gf4":run==="fresh-policy-green3"?"🗑️generated/gf5":run==="fresh-policy-green4"?"🗑️generated/gf6":run==="fresh-policy-green5"?"🗑️generated/gf7":"🗑️generated/gf8")):route==="broad-command-green"?checked(join(ticket,run==="broad-command-green"?"🗑️generated/gb1":run==="broad-command-green2"?"🗑️generated/gb2":run==="broad-command-green3"?"🗑️generated/gb3":"🗑️generated/gb4")):(route==="writer-red"||route==="writer-green")?checked(join(ticket,run==="writer-red"?"🗑️generated/gp6":run==="writer-red2"?"🗑️generated/gp7":run==="writer-green"?"🗑️generated/gp8":run==="writer-green2"?"🗑️generated/gp9":run==="writer-source-red"?"🗑️generated/gp10":run==="writer-source-red2"?"🗑️generated/gpa":run==="writer-green3"?"🗑️generated/gpb":run==="writer-custody-red"?"🗑️generated/gpc":"🗑️generated/gpd")):route==="native-orchestration"?checked(join(ticket,run==="native-orchestration-red"?"🗑️generated/go1":run==="native-orchestration-green"?"🗑️generated/go2":run==="native-owner-red"?"🗑️generated/go3":run==="native-owner-progress-red2"?"🗑️generated/go5":run==="native-owner-progress-red3"?"🗑️generated/go6":run==="native-owner-progress-red4"?"🗑️generated/go7":run==="native-owner-progress-red5"?"🗑️generated/go8":run==="native-owner-progress-red6"?"🗑️generated/go9":run==="native-owner-progress-red7"?"🗑️generated/goa":run==="native-owner-progress-red8"?"🗑️generated/gob":run==="native-owner-feature-red"?"🗑️generated/goc":run==="native-owner-feature-green"?"🗑️generated/god":run==="native-owner-feature-green2"?"🗑️generated/goe":run==="native-owner-feature-green3"?"🗑️generated/gof":run==="native-owner-feature-green4"?"🗑️generated/gog":run==="native-owner-feature-green5"?"🗑️generated/goh":run==="native-owner-current-green"?"🗑️generated/goi":run==="native-owner-current-verdict"?"🗑️generated/goj":run==="native-owner-current-verdict2"?"🗑️generated/gok":run==="native-owner-interruption"?"🗑️generated/gol":run==="native-owner-interruption2"?"🗑️generated/gom":run==="native-owner-interruption3"?"🗑️generated/gon":"🗑️generated/go4")):route==="native-dependencies"?checked(join(ticket,"🗑️generated/gn5")):route==="cargo-receiving"?checked(join(ticket,run==="cargo-receiving"?"🗑️generated/gr4":run==="cargo-receiving2"?"🗑️generated/gr5":run==="cargo-receiving3"?"🗑️generated/gr6":run==="cargo-owner-red"?"🗑️generated/gr7":run==="cargo-owner-red2"?"🗑️generated/gr8":run==="cargo-owner-red3"?"🗑️generated/gr9":"🗑️generated/gra")):route==="cargo-red"?checked(join(ticket,run==="cargo-red"?"🗑️generated/gr":run==="cargo-red2"?"🗑️generated/gr2":"🗑️generated/gr3")):route==="actor-red"?checked(join(ticket,"🗑️generated/ar")):route==="actor-green"?checked(join(ticket,"🗑️generated/ag")):route==="current-red"?checked(join(ticket,"🗑️generated/cr")):route==="current-green"?checked(join(ticket,run==="current-green"?"🗑️generated/cg":run==="current-green2"?"🗑️generated/cg2":run==="current-green3"?"🗑️generated/cg3":run==="current-green4"?"🗑️generated/cg4":run==="current-green5"?"🗑️generated/cg5":run==="current-green6"?"🗑️generated/cg6":run==="current-green7"?"🗑️generated/cg7":run==="current-green8"?"🗑️generated/cg8":run==="current-green9"?"🗑️generated/cg9":run==="current-green10"?"🗑️generated/cga":"🗑️generated/cgb")):directory;if(artifact!==directory){assert.equal(existsSync(artifact),false,"Actor fixture epoch cannot be replayed");mkdirSync(artifact,{recursive:true});}
 const env={...process.env,SEMIO_TEST_LEVEL:input.controls.testLevel,SEMIO_TEST_ARTIFACT_DIR:artifact};delete env.SEMIO_GENERAL_SOURCE_PAIRS;
 child=Bun.spawn(argv,{cwd:root,env,stdout:"pipe",stderr:"pipe"});
 const terminate=async()=>{if(child&&child.exitCode===null){const {terminateOwnedProcessTree}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts"));terminateOwnedProcessTree(child.pid);}};
 controller.signal.addEventListener("abort",terminate,{once:true});
 const receiverDeadline=route==="invocation"?setTimeout(stop,15000):undefined;
 const [stdout,stderr,code]=await Promise.all([retain(child.stdout as ReadableStream<Uint8Array>,checked(join(directory,"stdout.log"))),retain(child.stderr as ReadableStream<Uint8Array>,checked(join(directory,"stderr.log"))),child.exited]);
 if(receiverDeadline!==undefined)clearTimeout(receiverDeadline);
 controller.signal.removeEventListener("abort",terminate);
 const selectorAfter=receivingSelectors(),after=await claims(before.map(row=>row.path)),advances=before.filter((row,index)=>row.sha256!==after[index]!.sha256||row.dev!==after[index]!.dev||row.ino!==after[index]!.ino);if(absent)assert.equal(existsSync(absent),false,"Defining red source advanced during receiver");
 writeFileSync(checked(join(directory,"receipt.json")),JSON.stringify({epoch:41,route,run,argv,childCode:code,stdout,stderr,after,advances,criticalAdvances:featureEpoch?advances.filter(row=>criticalPaths.has(row.path)):advances,sourceCustody:custody?.mode??"exact-complete",observedConsumers:custody?.observedConsumers??[],testedSelectors:custody?.testedSelectors??[],selectorBefore,selectorAfter,fullDependencyExact:advances.length===0,elapsedMilliseconds:performance.now()-started,sourceWrites:0,sourceBodiesSerialized:0,controls:input.controls},null,2));
 console.log("[DEBUG] "+JSON.stringify({route,run,childCode:code,sourceClaims:before.length,advances:advances.length}));
 assert.equal(featureEpoch?advances.filter(row=>criticalPaths.has(row.path)).length:advances.length,0,"Required defining source advanced during current in-place receiver");assert.deepEqual(selectorAfter,expectedSelectors,"Original receiving selector advanced during current receiver");assert.equal(code,0,"Actual in-place receiver refused");
}catch(error){
 controller.abort();if(child&&child.exitCode===null){const {terminateOwnedProcessTree}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts"));terminateOwnedProcessTree(child.pid);await child.exited;}
 if(!existsSync(checked(join(directory,"receipt.json"))))writeFileSync(checked(join(directory,"receipt.json")),JSON.stringify({epoch:41,route,run,childCode:child?.exitCode??null,error:String(error),controls:input.controls,sourceWrites:0,sourceBodiesSerialized:0}));throw error;
}finally{clearTimeout(timeout);process.off("SIGINT",stop);process.off("SIGTERM",stop);}

/** 📏️ Enforces the complete absolute path contract before any owned output is allocated. */
function checked(path:string):string{assert.ok(path.length<=256,"Path exceeds 256 UTF16 units: "+path);return path;}

/** 🧾️ Hashes current physical files without retaining or serializing source bodies. */
async function claims(paths:string[]):Promise<any[]>{
 const rows=[];
 for(const path of [...new Set(paths)].sort()){
  checkpoint();const before=lstatSync(path,{bigint:true});assert.ok(before.isFile()&&!before.isSymbolicLink());let physicalWork=0;
  const observed=await observePhysicalFileV1(path,{maxBytes:input.controls.maximumOwnedBytes,maxWork:input.controls.maximumUnits-chargedUnits,chunkBytes:65536,cancelled:()=>controller.signal.aborted,remainingMs:()=>input.controls.phaseBudgetMilliseconds-(performance.now()-started),onProgress:(event:any)=>{assert.ok(event.work>=physicalWork);chargedUnits+=event.work-physicalWork;physicalWork=event.work;checkpoint();}});
  const after=lstatSync(path,{bigint:true});assert.deepEqual([before.dev,before.ino,before.size,before.mtimeNs,before.ctimeNs],[after.dev,after.ino,after.size,after.mtimeNs,after.ctimeNs]);assert.equal(BigInt(observed.byteLength),after.size);
  rows.push({path,sha256:observed.sha256,bytes:observed.byteLength,physicalObservation:"same-descriptor and linked-path checked by first-party observer",dev:String(after.dev),ino:String(after.ino),mtimeNs:String(after.mtimeNs),ctimeNs:String(after.ctimeNs)});if(rows.length%32===0)console.log("[DEBUG] in-place source claims="+rows.length);
 }
 return rows;
}

/** 🔎️ Follows literal first-party receiving imports in their actual tracked locations. */
async function closure(entries:string[]):Promise<string[]>{
 const queue=entries.map(path=>({path,depth:0})),seen=new Set<string>();let units=0;
 while(queue.length){
  checkpoint();const {path,depth}=queue.shift()!;if(seen.has(path))continue;assert.ok(depth<=input.controls.maximumDepth);assert.ok(++units<=input.controls.maximumUnits);checked(path);seen.add(path);
  if(!/\.[cm]?[jt]sx?$/u.test(path))continue;const bytes=readFileSync(path);assert.ok(bytes.length<=input.controls.maximumOwnedBytes);const tree=ts.createSourceFile(path,bytes.toString("utf8"),ts.ScriptTarget.Latest,true);assert.equal(tree.parseDiagnostics.length,0,path);
  const visit=(node:any)=>{let spec:string|undefined;if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))spec=node.moduleSpecifier.text;if(ts.isCallExpression(node)&&node.arguments.length===1&&ts.isStringLiteral(node.arguments[0])&&(node.expression.kind===ts.SyntaxKind.ImportKeyword||node.expression.getText(tree)==="require"))spec=node.arguments[0].text;if(spec?.startsWith(".")){const declared=resolve(dirname(path),spec);if(absent!==null&&declared===absent){assert.equal(existsSync(declared),false);return;}const target=Bun.resolveSync(spec,dirname(path)),rel=relative(root,target);assert.ok(!rel.startsWith("..")&&!isAbsolute(rel));queue.push({path:target,depth:depth+1});}ts.forEachChild(node,visit);};visit(tree);await new Promise<void>(done=>setImmediate(done));
 }
 return [...seen];
}

/** 📋️ Retains only terminal streams and their byte digests under the owning output directory. */
async function retain(stream:ReadableStream<Uint8Array>,path:string):Promise<{path:string;bytes:number;sha256:string}>{
 const writer=Bun.file(path).writer(),hash=createHash("sha256");let bytes=0;
 try{for await(const chunk of stream){checkpoint();hash.update(chunk);bytes+=chunk.byteLength;chargedOutputBytes+=chunk.byteLength;assert.ok(chargedOutputBytes<=input.controls.maximumOutputBytes,"Combined terminal streams exceed admitted output bytes");writer.write(chunk);await writer.flush();}}finally{await writer.end();}
 return{path,bytes,sha256:hash.digest("hex")};
}

/** ⏳️ Enforces the complete phase deadline, cancellation and cumulative work authority. */
function checkpoint():void{controller.signal.throwIfAborted();assert.ok(performance.now()-started<=input.controls.phaseBudgetMilliseconds,"Phase deadline exceeded");assert.ok(++chargedUnits<=input.controls.maximumUnits,"Phase work authority exhausted");}
