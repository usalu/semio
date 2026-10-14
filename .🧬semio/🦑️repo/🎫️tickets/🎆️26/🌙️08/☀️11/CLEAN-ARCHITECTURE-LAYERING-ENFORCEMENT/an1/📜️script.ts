import {strict as assert} from "node:assert";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync,mkdirSync,openSync,closeSync,existsSync} from "node:fs";
import {join,resolve} from "node:path";
import {pathToFileURL} from "node:url";

if(!import.meta.main||process.argv.length!==3||process.argv[2]!=="capture-root-dispatched")throw Error("Actor Context Native GUI1 requires Root's explicit capture dispatch");
const root=resolve(import.meta.dir,"../../../../../../../.."),ticket=resolve(import.meta.dir,".."),directory=join(ticket,"🗑️generated/an-gui1"),caller=join(ticket,"actor-context-native-gui-caller1.json"),inherited={...process.env};
const privateKeys=new Set(["SEMIO_SCRIPT_PROCESS_INVOCATION","SEMIO_SCRIPT_CAPABILITIES","SEMIO_OWNER_ARGUMENTS","SEMIO_NX_COMMAND_DIRECTORY","SEMIO_NX_COMMAND_RUNNER","SEMIO_NX_GRAPH_SOURCE_MEMBERSHIP","SEMIO_NX_SELECTED_GRAPH_RECEIPT","SEMIO_NX_GRAPH_SEAL_REUSE","SEMIO_BUILD_GRAPH_PATH","SEMIO_BUILD_GRAPH_DIGEST","SEMIO_BUILD_REUSE_GRAPH","SEMIO_CARGO_PREPARATION_ACTIVE","SEMIO_CARGO_PREPARATION_OBSERVATION","SEMIO_CARGO_PREPARATION_STORAGE","SEMIO_BUILD_INVOCATION_PID","SEMIO_OWNER_POLICY_RECEIPT","SEMIO_PROCESS_OWNER_CONTEXT","SEMIO_NX_NATIVE_SOURCE_SNAPSHOT","SEMIO_COMPILER_RESOURCE_ROOT","SEMIO_BAZEL_SELECTION_CHAIN"]);
for(const key of Object.keys(inherited))if(privateKeys.has(key)||key.startsWith("SEMIO_SOURCE_CLOSURE_")||key.startsWith("SEMIO_CARGO_LOCK_")||key.startsWith("NX_TASK_")||/^(?:SEMIO|NX)_.*(?:PRIVATE|CUSTODY)/u.test(key))throw Error("Preexisting issued authority refused: "+key);
const expectedRow={
  "name": "🏛️Actor · General Context Native",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun repo run @semio-tech/framework-actor-rs:test-original-context-native",
  "cwd": "${workspaceFolder}",
  "env": {
    "SEMIO_SCRIPT_POLICY": "{\"version\":1,\"owner\":\"actor:original-context:caller\",\"maximumElapsedMilliseconds\":600000}",
    "SEMIO_SCRIPT_CAPABILITIES": "{\"workspaceRoot\":\"${workspaceFolder}\",\"arguments\":[\"repo\",\"run\",\"@semio-tech/framework-actor-rs:test-original-context-native\"],\"selection\":[\"repo\",\"run\",\"@semio-tech/framework-actor-rs:test-original-context-native\"],\"buildMode\":\"prototype\",\"native\":{\"artifactDirectory\":\"${workspaceFolder}/.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture\",\"transport\":{\"maximumBytes\":134217728,\"maximumLines\":1000000},\"child\":{\"maximumElapsedMilliseconds\":600000},\"network\":{\"offline\":true}},\"launches\":[[\"repo\",\"run\",\"@semio-tech/framework-actor-rs:test-original-context-native\"]]}",
    "SEMIO_BUILD_MODE": "prototype",
    "CARGO_TARGET_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture",
    "CARGO_BUILD_BUILD_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture",
    "CARGO_NET_OFFLINE": "true",
    "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/⚡️cache/tests/actor-context"
  },
  "presentation": {
    "group": "4_gate",
    "order": 900.8276
  }
};

/** 🪟️ Reads one literal registered row without evaluating executable Source or rewriting launch inventories. */
function readRow(path:string):Readonly<{row:typeof expectedRow;sha256:string}> {
 const source=readFileSync(join(root,path),"utf8"),marker=JSON.stringify("name")+": "+JSON.stringify(expectedRow.name);assert.equal(source.split(marker).length-1,1,"One exact registered Repo caller required");const start=source.lastIndexOf("{",source.indexOf(marker));let depth=0,quoted=false,escaped=false;
 for(let end=start;end<source.length;end++){const value=source[end]!;if(quoted){if(escaped)escaped=false;else if(value==="\\")escaped=true;else if(value==='"')quoted=false;}else if(value==='"')quoted=true;else if(value==="{")depth++;else if(value==="}"&&--depth===0)return{row:JSON.parse(source.slice(start,end+1)),sha256:createHash("sha256").update(source).digest("hex")};}throw Error("Registered Repo caller row is incomplete");
}
const seed=readRow(".vscode/🧩️launch.seed.jsonc"),mirror=readRow(".vscode/launch.json"),row=mirror.row;
assert.deepEqual(seed.row,expectedRow);assert.deepEqual(row,expectedRow);assert.equal(process.cwd(),root);
const command="bun repo run @semio-tech/framework-actor-rs:test-original-context-native",argv=command.split(" "),selection=argv.slice(1),policy=JSON.parse(row.env.SEMIO_SCRIPT_POLICY),authored=JSON.parse(row.env.SEMIO_SCRIPT_CAPABILITIES);
assert.equal(row.command,command);assert.equal(row.cwd,"${workspaceFolder}");assert.deepEqual(policy,{version:1,owner:"actor:original-context:caller",maximumElapsedMilliseconds:600000});
assert.deepEqual(authored,{workspaceRoot:"${workspaceFolder}",arguments:selection,selection,buildMode:"prototype",native:{artifactDirectory:"${workspaceFolder}/.🧬semio/🦑️repo/⚡️cache/cargo/canonical-architecture",transport:{maximumBytes:134217728,maximumLines:1000000},child:{maximumElapsedMilliseconds:600000},network:{offline:true}},launches:[selection]});
assert.equal(row.env.SEMIO_BUILD_MODE,"prototype");assert.equal(row.env.CARGO_NET_OFFLINE,"true");assert.equal(row.env.CARGO_TARGET_DIR,authored.native.artifactDirectory);assert.equal(row.env.CARGO_BUILD_BUILD_DIR,authored.native.artifactDirectory);
if(existsSync(directory)||existsSync(caller))throw Error("Fresh Actor Context Native GUI1 output ownership required");
const frontier=["🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🟦️.ts", "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts", "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/🧪️execution/🔣️.json", "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/📜️script.ts"];
const observe=()=>({kind:"read-only-source-observation-not-authority",files:frontier.map(path=>{const bytes=readFileSync(join(root,path));return{path,sha256:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.length};})});
mkdirSync(directory,{recursive:false});writeFileSync(caller,JSON.stringify({version:1,row,seedSha256:seed.sha256,launchSha256:mirror.sha256,outputRedirectOnly:directory,sourceObservations:observe(),qualification:"Initial authored GUI input; no process or test acceptance inferred"},null,2)+"\n",{flag:"wx"});
const expand=(value:string)=>value.replaceAll("${workspaceFolder}",root.replaceAll("\\","/")),expandedAuthoring={...authored,workspaceRoot:expand(authored.workspaceRoot),native:{...authored.native,artifactDirectory:expand(authored.native.artifactDirectory)}},environment={...inherited,...Object.fromEntries(Object.entries(row.env).map(([key,value])=>[key,key==="SEMIO_SCRIPT_CAPABILITIES"?JSON.stringify(expandedAuthoring):expand(value)])),SEMIO_TEST_ARTIFACT_DIR:directory};
try{
 const {createBuildBootstrapCapabilitiesV1}=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts")).href),capabilities=await createBuildBootstrapCapabilitiesV1(root,selection,environment),encoded=JSON.stringify(capabilities);
 assert.deepEqual(capabilities,{...expandedAuthoring,tool:"nx",workspaceRoot:root,native:{...expandedAuthoring.native,artifactDirectory:resolve(root,expandedAuthoring.native.artifactDirectory)}});
 writeFileSync(join(directory,"factory.json"),encoded+"\n",{flag:"wx"});
 const descriptor=openSync(join(directory,"current.log"),"wx"),started=Date.now();console.log("[DEBUG] Registered Actor Context Native GUI1 started "+JSON.stringify({argv,initialPolicy:policy,factory:join(directory,"factory.json"),log:join(directory,"current.log")}));let status:number,signal:string|null;
 try{const child=Bun.spawn(argv,{cwd:root,env:{...environment,SEMIO_SCRIPT_CAPABILITIES:encoded},stdin:"ignore",stdout:descriptor,stderr:descriptor});writeFileSync(join(directory,"started.json"),JSON.stringify({pid:child.pid,startedAt:new Date(started).toISOString(),argv,initialPolicy:policy},null,2)+"\n",{flag:"wx"});status=await child.exited;signal=child.signalCode;}finally{closeSync(descriptor);}
 const result={exitCode:status,signal,elapsedMilliseconds:Date.now()-started,initialPolicy:policy,factorySha256:createHash("sha256").update(encoded).digest("hex"),registeredCommand:command,qualification:"Raw process outcome; no inferred Store/native/assertion acceptance"};writeFileSync(join(directory,"result.json"),JSON.stringify(result,null,2)+"\n",{flag:"wx"});console.log("[DEBUG] Registered Actor Context Native GUI1 terminal "+JSON.stringify(result));process.exitCode=status;
}catch(error){writeFileSync(join(directory,"recorder-error.json"),JSON.stringify({message:error instanceof Error?error.message:String(error),qualification:"Recorder or factory failure is not a test result"},null,2)+"\n",{flag:"wx"});throw error;}
finally{writeFileSync(join(directory,"terminal-source-observations.json"),JSON.stringify(observe(),null,2)+"\n",{flag:"wx"});}
