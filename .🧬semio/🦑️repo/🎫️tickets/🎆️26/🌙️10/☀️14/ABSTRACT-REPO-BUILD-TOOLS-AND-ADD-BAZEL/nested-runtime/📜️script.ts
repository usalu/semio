import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFile,mkdir,writeFile} from "node:fs/promises";
import {isAbsolute,join,resolve} from "node:path";
import fixture from "./🔣️.json";
const [tool,path,digest]=process.argv.slice(2),root=process.env.SEMIO_WORKSPACE_ROOT,ticket=resolve(import.meta.dir,".."),output=join(ticket,"🗑️generated");
if(process.argv.length!==5||!fixture.tools.includes(tool!)||!path||!isAbsolute(path)||!/^[a-f0-9]{64}$/u.test(digest??""))throw Error("Expected nx|bazel, original absolute all-workspace pin and SHA-256");
if(!root||!isAbsolute(root))throw Error("Original absolute workspace required");
assert.equal(process.env.SEMIO_TEST_ARTIFACT_DIR,output,"Original ticket artifact directory required");
assert.equal(process.env.SEMIO_BUILD_GRAPH_PATH,path,"Original all-workspace pin handoff required");
assert.equal(process.env.SEMIO_BUILD_GRAPH_DIGEST,digest,"Original all-workspace digest handoff required");
assert.equal(process.env.SEMIO_BUILD_REUSE_GRAPH,"true","Original all-workspace reuse required");
if(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION!==undefined)throw Error("Standalone nested runtime must issue its own original matching-backend envelope");
assert.equal(fixture.maximumElapsedMilliseconds,120000);
const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),args=["--tool="+tool,"exec","--projects="+fixture.parentProject,"--","bun","repo","--tool="+tool,"run",fixture.target,...fixture.childOptions],environment={...process.env,SEMIO_BUILD_TOOL:tool,SEMIO_SCRIPT_CAPABILITIES:undefined};
const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),{createBuildBootstrapCapabilitiesV1}=await import(join(library,"🏗️build/📥️caller/📤️child/🟦️.ts")),{readBuildGraphV1}=await import(join(library,"🏗️build/🕸️graph/🟦️.ts")),{buildInvocationGraphSelectionV1}=await import(join(library,"🏗️build/🕸️graph/🚪️admission/🟦️.ts")),{runOwnedCommand}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts"));
await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"repository:build",maximumElapsedMilliseconds:fixture.maximumElapsedMilliseconds},await createBuildBootstrapCapabilitiesV1(root,args,environment),Date.now()),async original=>{
 routing.checkScriptInvocation(original);const bytes=await readFile(path);assert.equal(createHash("sha256").update(bytes).digest("hex"),digest,"Original all-workspace bytes changed");const graph=readBuildGraphV1(JSON.parse(bytes.toString("utf8"))),selection=buildInvocationGraphSelectionV1(args.slice(1));assert.deepEqual(selection,{kind:"all"});assert.deepEqual(graph.admission?.selection,selection,"Exact all-workspace parent admission required");assert.equal(graph.projects[fixture.parentProject]?.root,".","Parent public exec requires real workspace cwd");const [project,target]=fixture.target.split(":");assert(graph.projects[project!]?.targets[target!],"Real existing nested target required");
 const receipt={scope:fixture.scope,tool,path,digest,parentSelection:selection,childSelection:buildInvocationGraphSelectionV1(["run",fixture.target,...fixture.childOptions]),command:["bun","repo",...args],originalMaximumElapsedMilliseconds:original.policy.maximumElapsedMilliseconds,status:"running",canonicalQualification:false};await mkdir(output,{recursive:true});await writeFile(join(output,"nested-runtime-"+tool+".json"),JSON.stringify(receipt,null,2)+"\n");console.log("[DEBUG] Actual original public nested runtime "+JSON.stringify(receipt));
 try{await runOwnedCommand(process.execPath,["repo",...args],root,"nested-runtime:public:"+tool,routing.scriptInvocationBudget(original,0),{signal:original.control.signal,env:routing.scriptInvocationEnvironment(original,{...environment,SEMIO_SCRIPT_POLICY:JSON.stringify(original.policy)})});receipt.status="complete";}
 catch(error){receipt.status="refused";throw error;}
 finally{await writeFile(join(output,"nested-runtime-"+tool+".json"),JSON.stringify(receipt,null,2)+"\n");}
});
