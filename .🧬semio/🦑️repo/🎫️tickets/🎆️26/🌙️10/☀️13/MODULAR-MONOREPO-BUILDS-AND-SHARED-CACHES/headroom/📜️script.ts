import {realpathSync,readFileSync,mkdirSync} from "node:fs";
import {join,resolve,dirname} from "node:path";
import {createRequire} from "node:module";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,scriptInvocationEnvironment,receiveScriptProcessInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {cargoTargetDirectory} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗂️directories/🟦️.ts";

const routes:Readonly<Record<string,readonly string[]>>={"sdk-snapshot":["test","sdk-snapshot"],"pruning":["test","pruning"],"daemon-journal":["test","daemon-journal"],"daemon-journal-workspace":["test","daemon-journal","workspace"],"renderer-unit":["test-wgpu-unit"],"camera-source":["node-key","source"]},owners:Readonly<Record<string,string>>={"renderer-unit":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts","camera-source":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/⏱️camera/📦️packages/🦀️rust/📜️script.ts"},args=process.argv.slice(2),inner=args[0]==="execute",command=args[inner?1:0]!,mode=args[inner?2:1]??"prototype";
const nativeCommands=["kernel-retirement","prepared-refusal","world-resources"],nativeCommand=nativeCommands.includes(command);
if(args.length>(inner?3:2)||!["prototype","release"].includes(mode)||!(["reclaim","selected-test"].includes(command)||command in routes||nativeCommand))throw Error("Unknown original isolated Nx suite or configuration");
const workspace=resolve(import.meta.dir,"../../../../../../../.."),sdk=realpathSync(join(workspace,".nx/installation")),policy={version:1 as const,owner:"repo:private-cache-headroom",maximumElapsedMilliseconds:600000},identity=JSON.parse(readFileSync(join(sdk,".semio-nx-tooling.json"),"utf8")).digest;
if(!/^[a-f0-9]{64}$/u.test(identity))throw Error("Actual immutable SDK receipt required");
if(inner)await receiveScriptProcessInvocation(process.env,async original=>{
 const artifacts=join(import.meta.dir,"../🗑️generated/isolated",command,mode);mkdirSync(artifacts,{recursive:true});
 const control=scriptCompilerAdmissionControlV1(original,"nx:isolated:"+command,nativeCommand?600000:300000),caching=join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"),owner=nativeCommand?join(import.meta.dir,"../wgpu-mode/📜️script.ts"):owners[command]?join(workspace,owners[command]!):join(caching,"📜️script.ts"),child=command==="selected-test"?["test",join(caching,"🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts")]:[owner,...(nativeCommand?[command]:routes[command]!)];
 process.stdout.write(await captureOwnedCommandV1(process.execPath,child,owners[command]?dirname(owner):workspace,{control,env:{...scriptInvocationEnvironment(original,process.env),SEMIO_TEST_ARTIFACT_DIR:artifacts,SEMIO_BUILD_MODE:mode,SEMIO_NX_TOOLING_IDENTITY:identity,NX_WORKSPACE_ROOT_PATH:workspace}}));
});
else await withScriptProcessEnvelope(createScriptProcessEnvelope(policy,nativeCommand?{workspaceRoot:workspace,buildMode:mode,arguments:["nx","run","repo-private-cache:"+command,"--configuration="+mode],selection:["nx","run","repo-private-cache:"+command,"--configuration="+mode],native:{artifactDirectory:cargoTargetDirectory(workspace),transport:{maximumBytes:134217728,maximumLines:65536},child:{maximumElapsedMilliseconds:policy.maximumElapsedMilliseconds},network:{offline:false}}}:{},Date.now()),async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"cache:headroom:nx",600000);
 const result=await captureOwnedCommandV1("node",[createRequire(join(sdk,"package.json")).resolve("nx/bin/nx.js"),"run",`repo-private-cache:${command}`,"--configuration="+mode],import.meta.dir,{control,env:{...scriptInvocationEnvironment(original,process.env),SEMIO_NX_TOOLING_IDENTITY:identity,SEMIO_BUILD_MODE:mode,NX_DAEMON:"false",NODE_PATH:join(sdk,"node_modules"),NX_WORKSPACE_ROOT_PATH:import.meta.dir,NX_WORKSPACE_DATA_DIRECTORY:join(import.meta.dir,"../🗑️generated/isolated",command,mode,"data"),NX_CACHE_DIRECTORY:join(import.meta.dir,"../🗑️generated/isolated",command,mode,"cache")}});
 process.stdout.write(result);
});
