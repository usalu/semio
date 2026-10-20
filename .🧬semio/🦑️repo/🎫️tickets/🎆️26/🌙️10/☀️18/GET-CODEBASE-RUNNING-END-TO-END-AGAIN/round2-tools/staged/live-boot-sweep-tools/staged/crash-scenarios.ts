import assert from"node:assert/strict";
import{createHash}from"node:crypto";
import{chmodSync,closeSync,constants,existsSync,fsyncSync,linkSync,lstatSync,mkdirSync,openSync,readFileSync,readdirSync,renameSync,rmdirSync,unlinkSync,writeFileSync,writeSync}from"node:fs";
import{join}from"node:path";
import{createNativeDirectoryJournaledPortV1}from"../../../../../📁️directory/🪟️journal/🟦️.ts";
import type{NativeWindowsHostPortV1}from"../../../../../📁️directory/🪟️host/🟦️.ts";
import{appendNativeBootLedgerV1,loadNativeBootLedgerV1,nativeBootLedgerCommittedV1,nativeBootLedgerPathV1,planNativeBootRecoveryV1,recoverNativeBootV1}from"../../🟦️.ts";
import{createNativeBootFsPortV1,type NativeBootEffectsV1}from"../../🧯️act/🟦️.ts";
import{observeNativeBootTargetsV1}from"../../📡️observe/🟦️.ts";

type Operation=Readonly<{n:number;kind:string;detail:string}>;
/** 💥️ Describes one ceremony that is killed at every state-changing call and then recovered. */
export interface CrashScenarioV1{
 readonly id:string;
 setup(directory:string):void;
 run(directory:string):void|Promise<void>;
 recover(directory:string):Promise<void>;
 invariant(directory:string,info:Readonly<{point:number;completed:boolean;operations:readonly Operation[]}>):void|Promise<void>;
}

const BOOT="1E4A0CBE-3710-481F-9614-F8F1FCBD61D0",sha=(bytes:Buffer|string)=>createHash("sha256").update(bytes).digest("hex");
const effects:NativeBootEffectsV1={retireTicket:async()=>"done",retireSettledClaims:async()=>"done",recoverDirectorySwap:async()=>"done",detachImage:async()=>"done"};
const syncDirectory=(path:string)=>{const fd=openSync(path,constants.O_RDONLY|(constants.O_DIRECTORY??0));try{fsyncSync(fd);}finally{closeSync(fd);}};
const clock=()=>({bootSession:BOOT,bootEpochMs:Date.now()});
const before=(info:Readonly<{point:number;completed:boolean;operations:readonly Operation[]}>,kind:string)=>info.completed||info.operations.some(row=>row.n<info.point&&row.kind===kind);

const oldReceipt=Buffer.from('{"version":1,"generation":"old"}\n'),newReceipt=Buffer.from('{"version":1,"generation":"new"}\n');
const publicationRoot=(directory:string)=>join(directory,"p"),publicationRecovery=(directory:string)=>{
 const root=publicationRoot(directory),ledgerPath=nativeBootLedgerPathV1(join(directory,"boot-recovery"),root);
 return{root,ledgerPath,port:createNativeBootFsPortV1({clock:clock(),targets:[{kind:"publication",scope:"p",root,writerFree:()=>true}],effects})};
};
const writeDurable=(path:string,bytes:Buffer)=>{const fd=openSync(path,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL,0o600);try{writeSync(fd,bytes);fsyncSync(fd);}finally{closeSync(fd);}};

const publicationCeremony:CrashScenarioV1={
 id:"publication-ceremony",
 setup(directory){const root=publicationRoot(directory);mkdirSync(root);writeFileSync(join(root,"image.json"),oldReceipt,{mode:0o600});chmodSync(join(root,"image.json"),0o600);},
 run(directory){
  const root=publicationRoot(directory),base=join(root,".publication");
  mkdirSync(base,{mode:0o700});chmodSync(base,0o700);syncDirectory(root);
  writeDurable(join(base,"intent.json"),Buffer.from(JSON.stringify({version:1,purpose:"current-publication",receiptSha256:sha(oldReceipt)})+"\n"));syncDirectory(base);
  writeDurable(join(base,"receipt.json"),newReceipt);syncDirectory(base);
  renameSync(join(base,"receipt.json"),join(root,"image.json"));syncDirectory(root);
  unlinkSync(join(base,"intent.json"));rmdirSync(base);syncDirectory(root);
 },
 async recover(directory){const{port,ledgerPath}=publicationRecovery(directory);await recoverNativeBootV1({port,ledgerPath,now:Date.now});},
 async invariant(directory,info){
  const{root,port,ledgerPath}=publicationRecovery(directory);
  assert.equal(existsSync(join(root,".publication")),false,"publication residue settled");
  assert.deepEqual(readFileSync(join(root,"image.json")),before(info,"renameSync")?newReceipt:oldReceipt,"the receipt is the old one before the rename and the new one after it");
  assert.equal((await recoverNativeBootV1({port,ledgerPath,now:Date.now})).outcome,"clean","a second recovery changes nothing");
  assert.equal(nativeBootLedgerCommittedV1(ledgerPath,BOOT),true);
 }
};

const simulatedHost=():NativeWindowsHostPortV1=>({moveFile:(from,to)=>{renameSync(from,to);return{ok:true,error:0};},flushDirectory:path=>{syncDirectory(path);return{ok:true,error:0};},setCaseSensitive:()=>({opened:true,error:0,status:0}),availableBytes:()=>({ok:true,error:0,bytes:0}),openExclusive:()=>({ok:true,error:0})});
const swapDirectory=(directory:string)=>join(directory,"d");
const directorySwap:CrashScenarioV1={
 id:"directory-swap-journal",
 setup(directory){const root=swapDirectory(directory);mkdirSync(root);mkdirSync(join(root,"m"));mkdirSync(join(root,"v"));writeFileSync(join(root,"v","marker"),"volume");},
 run(directory){const root=swapDirectory(directory);createNativeDirectoryJournaledPortV1(simulatedHost()).swap(join(root,"m"),join(root,"v"));},
 async recover(directory){createNativeDirectoryJournaledPortV1(simulatedHost()).recover(swapDirectory(directory));},
 invariant(directory,info){
  const root=swapDirectory(directory),port=createNativeDirectoryJournaledPortV1(simulatedHost());
  assert.deepEqual(readdirSync(root).sort(),["m","v"],"no intent or temporary directory remains");
  assert.equal(port.pending(root),false);assert.equal(port.recover(root),"clean");
  const under=["m","v"].filter(name=>existsSync(join(root,name,"marker")));assert.equal(under.length,1,"the volume content is reachable exactly once");
  assert.equal(under[0]==="m",before(info,"renameSync"),"the swap is rolled back before its first rename and completed after it");
 }
};

const recordBytes=Buffer.from('{"version":1,"allocation":"0123456789abcdef0123456789abcdef"}\n');
const recordPaths=(directory:string)=>({final:join(directory,"record.json"),temporary:join(directory,"record.json.new")});
const recordTransaction:CrashScenarioV1={
 id:"record-transaction",
 setup(directory){mkdirSync(directory,{recursive:true});},
 run(directory){const{final,temporary}=recordPaths(directory);writeDurable(temporary,recordBytes);syncDirectory(directory);linkSync(temporary,final);syncDirectory(directory);unlinkSync(temporary);syncDirectory(directory);},
 async recover(directory){
  const{final,temporary}=recordPaths(directory);
  if(!existsSync(final)){if(existsSync(temporary))unlinkSync(temporary);writeDurable(temporary,recordBytes);linkSync(temporary,final);}
  if(existsSync(temporary)&&lstatSync(temporary).nlink===2)unlinkSync(temporary);
 },
 async invariant(directory){
  const{final,temporary}=recordPaths(directory);
  assert.deepEqual(readFileSync(final),recordBytes,"the published record is whole");assert.equal(existsSync(temporary),false);
  const plan=planNativeBootRecoveryV1(await observeNativeBootTargetsV1(clock(),[{kind:"record",scope:"r",final,temporary}]));
  assert.deepEqual(plan.retained.map(row=>[row.verdict,row.reason]),[["keep","published"]]);
 }
};

const recordObserved:CrashScenarioV1={
 id:"record-transaction-observed",
 setup:recordTransaction.setup,run:recordTransaction.run,recover:async()=>{},
 async invariant(directory){
  const{final,temporary}=recordPaths(directory);
  const plan=planNativeBootRecoveryV1(await observeNativeBootTargetsV1(clock(),[{kind:"record",scope:"r",final,temporary}]));
  assert.equal(plan.steps.length,0,"the sweep never touches a record transaction");
  assert.equal(plan.retained.every(row=>row.verdict==="keep"),true,"every crash state of the record transaction is owner-resumable: "+JSON.stringify(plan.retained));
  if(existsSync(final))assert.deepEqual(readFileSync(final),recordBytes,"a published record is never torn");
 }
};

const recoveryRun:CrashScenarioV1={
 id:"recovery-run",
 setup(directory){publicationCeremony.setup(directory);const root=publicationRoot(directory),base=join(root,".publication");mkdirSync(base,{mode:0o700});chmodSync(base,0o700);writeDurable(join(base,"intent.json"),Buffer.from(JSON.stringify({version:1,receiptSha256:sha(oldReceipt)})+"\n"));writeDurable(join(base,"receipt.json"),newReceipt);},
 async run(directory){const{port,ledgerPath}=publicationRecovery(directory);await recoverNativeBootV1({port,ledgerPath,now:Date.now});},
 async recover(directory){const{port,ledgerPath}=publicationRecovery(directory);await recoverNativeBootV1({port,ledgerPath,now:Date.now});},
 async invariant(directory){
  const{root,ledgerPath}=publicationRecovery(directory);
  assert.equal(existsSync(join(root,".publication")),false);assert.deepEqual(readFileSync(join(root,"image.json")),oldReceipt);
  const state=loadNativeBootLedgerV1(ledgerPath).state;assert.equal(state.corrupt,false,"a killed recovery never corrupts its ledger");assert.equal(state.state,"committed");
 }
};

const ledgerPathOf=(directory:string)=>nativeBootLedgerPathV1(join(directory,"boot-recovery"),directory);
const ledgerAppend:CrashScenarioV1={
 id:"ledger-append",
 setup(directory){
  const path=ledgerPathOf(directory),commit=JSON.stringify({v:1,type:"commit",boot:BOOT,at:9,done:0,skipped:0,retained:0})+"\n",filler=(size:number)=>JSON.stringify({v:1,type:"step",boot:BOOT,index:0,action:"remove-file",target:".publication/intent.json",outcome:"done",detail:"x".repeat(size)})+"\n";
  let text=commit+commit+commit;while(Buffer.byteLength(text)+Buffer.byteLength(filler(150))<=65500)text+=filler(150);
  mkdirSync(join(directory,"boot-recovery"));writeFileSync(path,text,{mode:0o600});
 },
 run(directory){const path=ledgerPathOf(directory);appendNativeBootLedgerV1(path,{v:1,type:"begin",boot:BOOT,at:10,steps:1,plan:sha("a")});appendNativeBootLedgerV1(path,{v:1,type:"commit",boot:BOOT,at:11,done:1,skipped:0,retained:0});},
 async recover(directory){appendNativeBootLedgerV1(ledgerPathOf(directory),{v:1,type:"begin",boot:BOOT,at:12,steps:0,plan:sha("b")});},
 invariant(directory,info){const state=loadNativeBootLedgerV1(ledgerPathOf(directory)).state;assert.equal(state.corrupt,false,"a killed append never corrupts the ledger");assert.equal(state.tornTail,false,"the next append repaired any torn tail");assert.equal(state.state,info.completed?"committed":"incomplete");}
};

/** 🎬️ Lists every ceremony the crash enumeration kills at each state-changing call. */
export const crashScenariosV1:readonly CrashScenarioV1[]=Object.freeze([publicationCeremony,directorySwap,recordTransaction,recordObserved,recoveryRun,ledgerAppend]);
