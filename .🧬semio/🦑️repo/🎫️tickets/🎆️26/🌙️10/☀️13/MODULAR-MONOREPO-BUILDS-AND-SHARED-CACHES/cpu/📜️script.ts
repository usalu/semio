import {resolve} from "node:path";

const workspace=resolve(import.meta.dir,"../../../../../../../.."),processRoot=resolve(workspace,"🧰️framework/🔨️modules/🏃️process"),source=resolve(processRoot,"🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/🟦️.ts"),child=resolve(processRoot,"🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/👷️child/📜️script.ts");
const {createScriptProcessEnvelope,withScriptProcessEnvelope,scriptInvocationEnvironment}=await import(resolve(processRoot,"🧭️routing/📥️invocation/🏃️process/🟦️.ts"));
const {scriptCompilerAdmissionControlV1}=await import(resolve(processRoot,"🏗️build/🎟️capacity/🟦️.ts"));
const {captureOwnedCommandResultV1}=await import(resolve(processRoot,"🎛️owned-execution/📥️capture/🟦️.ts"));
const command=process.argv[2],custodian=command?.startsWith("custodian-"),strict=command?.endsWith("strict");if(!["strict","runtime","custodian-strict","custodian-runtime"].includes(command??""))throw Error("Expected strict, runtime, custodian-strict or custodian-runtime");
const custodianSource=resolve(processRoot,"🏗️build/🎟️capacity/🛟️custodian/🧪️tests/🟦️.ts"),custodianEntry=resolve(processRoot,"🏗️build/🎟️capacity/🛟️custodian/📜️script.ts");
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"ticket:compiler-capacity-inheritance:"+command,maximumElapsedMilliseconds:30000},{workspaceRoot:workspace},Date.now()),async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"test:compiler-capacity-inheritance:"+command,0),args=strict?[Bun.resolveSync("typescript/bin/tsc",workspace),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...(custodian?[custodianSource,custodianEntry,...["👷️worker","🛑️cancel","🎭️forged","⏱️deadline"].map(name=>resolve(processRoot,"🏗️build/🎟️capacity/🛟️custodian/🧪️tests",name,"📜️script.ts"))]:[source,child])]:[resolve(processRoot,"📜️script.ts"),"test",custodian?"compiler-capacity-custodian":"compiler-capacity-inheritance"];
 const result=await captureOwnedCommandResultV1(process.execPath,args,workspace,{control,output:{maximumBytes:1048576,maximumLines:8192},env:scriptInvocationEnvironment(original,{...process.env,SEMIO_TEST_ARTIFACT_DIR:resolve(import.meta.dir,"../🗑️generated")})});
 process.stdout.write(result.stdout);process.stderr.write(result.stderr);if(result.status!==0||result.signal!==null)throw Error(`Compiler capacity inheritance ${command} gate failed: status=${result.status}, signal=${result.signal}`);
});
