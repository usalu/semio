// Runs ONE suite of the build route table the way 🗂️suites/📜️script.ts does (same runProcessTestSuiteV1 call, same remote), in an envelope of its own like `repo test-file`'s, so no project graph is read.
// usage: repo <this file> -- <suite id>     (SEMIO_TEST_FILE_OFFLOAD=never|auto|always)
import {join,resolve} from "node:path";
const CHECKOUT="/Users/ueli/Documents/semio",LIB=join(CHECKOUT,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),PROCESS=join(CHECKOUT,"🧰️framework/🔨️modules/🏃️process"),SUITES=join(LIB,"🏗️build/🗂️suites"),id=process.argv[process.argv.length-1]!;
const load=(path:string)=>import(path);
const routing=await load(join(PROCESS,"📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),runner=await load(join(PROCESS,"📦️packages/🟦️typescript/🧪️testing/🗂️suites/🏃️run/🟦️.ts")),suitesModule=await load(join(SUITES,"🟦️.ts")),birth=await load(join(LIB,"🧪️testing/🌱️genuine-birth/🚀️launchers/🟦️.ts")),workers=await load(join(LIB,"🏗️build/🌡️host-capacity/🧪️workers/🟦️.ts")),caller=await load(join(LIB,"🏗️build/📥️caller/📤️child/🟦️.ts")),providers=await load(join(LIB,"🏃️process/🔌️providers/🟦️.ts"));
const remoteModule=await load(join(LIB,"🏗️build/🔬️test-file/🌐️remote/🗂️suite/🟦️.ts")).catch(()=>undefined);
const environment:NodeJS.ProcessEnv={...process.env,SEMIO_BUILD_MODE:process.env.SEMIO_BUILD_MODE??"prototype",SEMIO_WORKSPACE_ROOT:CHECKOUT};
for(const name of ["SEMIO_SCRIPT_PROCESS_INVOCATION","SEMIO_SCRIPT_CAPABILITIES","SEMIO_SCRIPT_POLICY","SEMIO_PROCESS_OWNER_CONTEXT"])delete environment[name];
providers.issueRepositoryScriptProvidersV1(environment);
const table=suitesModule.readBuildTestTableV1(),suite=table.suites.find((row:{id:string})=>row.id===id);
if(!suite)throw Error("unknown suite "+id);
const capabilities=await caller.createBuildBootstrapCapabilitiesV1(CHECKOUT,["run","repo:test-"+id],environment);
const started=performance.now();
await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"suite-driver",maximumElapsedMilliseconds:3600000},capabilities,Date.now()),async(invocation:unknown)=>{
 const checkout=runner.processTestCheckoutV1(SUITES,CHECKOUT),launchers=birth.genuineBirthLaunchersV1(join(LIB,"🧪️testing/🌱️genuine-birth/📜️script.ts"),checkout),remote=remoteModule===undefined?undefined:await remoteModule.createSuiteRemoteV1(checkout,suite.id,environment);
 await routing.advanceScriptInvocation(invocation,"suite-driver:"+id,"running");
 await runner.withProcessTestOutputV1(checkout,"build-contract",(output:string)=>runner.runProcessTestSuiteV1({invocation,root:suitesModule.buildTestRootV1,checkout,table,suite:suite.id,owner:"build",output,environment,launchers,workers:{learning:workers.testWorkerLearningV1(environment)},...(remote===undefined?{}:{remote})}),suite.scratch);
});
process.stderr.write(`[suite-driver] ${id} done in ${Math.round((performance.now()-started)/100)/10} s\n`);
