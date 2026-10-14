import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync,mkdirSync,realpathSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,join,relative,resolve} from "node:path";
const root=process.env.SEMIO_WORKSPACE_ROOT!,output=process.env.SEMIO_TEST_ARTIFACT_DIR!;
if(!root||!output)throw Error("Original workspace and ticket artifact directory required");
const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),fixture=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8")),scratch=join(output,"cx"),require=createRequire(join(root,"package.json")),sdk=createRequire(join(root,".nx/installation/package.json"));
if(process.argv[2]==="prepare"){
 mkdirSync(scratch,{recursive:true});
 writeFileSync(join(scratch,"package.json"),JSON.stringify({name:"configured-input-oracle",private:true,type:"module"}));
 writeFileSync(join(scratch,"nx.json"),JSON.stringify({useDaemonProcess:false}));
 writeFileSync(join(scratch,"project.json"),JSON.stringify({name:"configured-input-oracle",targets:{verify:{executor:"nx:run-commands",options:{command:`bun ${JSON.stringify(import.meta.path)} run`,cwd:root}}}}));
 console.log("[DEBUG] Isolated configured-input Nx oracle fixture prepared");
}else if(process.argv[2]==="census"){
 const path=process.argv[3];if(!path)throw Error("Exact prior pin required");const graph=JSON.parse(readFileSync(path,"utf8")),scalar:any[]=[];
 for(const[name,project]of Object.entries(graph.projects)as any)for(const[target,definition]of Object.entries(project.targets)as any){if(typeof definition.execution?.args==="string")scalar.push({project:name,target,args:definition.execution.args});for(const[configuration,override]of Object.entries(definition.configurations??{})as any)if(typeof override.execution?.args==="string")scalar.push({project:name,target,configuration,args:override.execution.args});}
 writeFileSync(join(output,"scalar-argument-census.json"),JSON.stringify({path,projects:Object.keys(graph.projects).length,scalar},null,2)+"\n");console.log("[DEBUG] Raw prior graph scalar-args census "+JSON.stringify(scalar));
}else{
 const workspace=realpathSync(scratch);
 for(const[path,bytes]of Object.entries(fixture.files)){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),String(bytes));}
 writeFileSync(join(workspace,"owner/📋️project.json"),JSON.stringify(fixture.project));
 const Ajv=require("ajv").default,valid=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(library,"🏗️build/🧬️schema/📋️project/🔣️.json"),"utf8")));
 assert(valid(fixture.project),JSON.stringify(valid.errors));
 const {inferReadOnlyBuildProjectSourcesV1,cacheInternals}=await import(join(library,"🏗️build/🕸️graph/📥️inference/🟨️.mjs")),{receiveScriptProcessInvocation}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
 const project=await receiveScriptProcessInvocation(process.env,(invocation:any)=>inferReadOnlyBuildProjectSourcesV1(fixture.project,"owner",workspace,invocation));
 assert(project,"Read-only production source inference must return the authored fixture owner");
 const target=project.targets.build,hasher=sdk("nx/src/hasher/task-hasher"),patterns=hasher.getTargetInputs({}, {data:project},"build").selfInputs,minimatch=require("minimatch").minimatch,expanded=target.inputs.filter((entry:any)=>typeof entry==="string"&&!entry.startsWith("^")).flatMap((name:string)=>project.namedInputs[name]?hasher.expandNamedInput(name,project.namedInputs):[{fileset:name}]),inventory=new Set<string>();
 for(const entry of expanded)if(entry.env?.startsWith("SEMIO_SOURCE_CLOSURE_")){const key=entry.env.slice("SEMIO_SOURCE_CLOSURE_".length),record=JSON.parse(readFileSync(join(workspace,".🧬semio/🦑️repo/⚡️cache/build-input-closures",key+".json"),"utf8"));for(const path of record.files)inventory.add(path);}
 const normalized=patterns.map((pattern:string)=>pattern.replaceAll("{projectRoot}",project.root).replaceAll("{workspaceRoot}/",""));
 const covered=(path:string)=>inventory.has(path)||(normalized.some((pattern:string)=>!pattern.startsWith("!")&&minimatch(path,pattern,{dot:true}))&&!normalized.some((pattern:string)=>pattern.startsWith("!")&&minimatch(path,pattern.slice(1),{dot:true})));
 const coveredFiles=Object.keys(fixture.files).filter(covered).sort(),hash=()=>{const node=createHash("sha256"),bun=new Bun.CryptoHasher("sha256");for(const path of coveredFiles){const bytes=readFileSync(join(workspace,path));node.update(path+"\0").update(bytes).update("\0");bun.update(path+"\0").update(bytes).update("\0");}const value=node.digest("hex");assert.equal(value,bun.digest("hex"));return value;};
 const configured=sdk("nx/src/utils/params").combineOptionsForExecutor({},"release",target,sdk("nx/src/executors/run-commands/schema.json"),null,null),entry=join(workspace,configured.cwd,"📜️script.ts"),oracle=await require("esbuild").build({entryPoints:[entry],absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent"}),oracleFiles=Object.keys(oracle.metafile.inputs).map(path=>relative(workspace,resolve(workspace,path)).replaceAll("\\","/")).sort();
 assert.deepEqual(oracleFiles,[...fixture.configuredSources].sort());
 const run=()=>{const child=Bun.spawnSync([process.execPath,entry],{cwd:join(workspace,configured.cwd),stdout:"pipe",stderr:"pipe"});assert.equal(child.exitCode,0,new TextDecoder().decode(child.stderr));return new TextDecoder().decode(child.stdout).trim();};
 const before={hash:hash(),execution:run()};writeFileSync(join(workspace,fixture.mutation.path),fixture.mutation.bytes);const after={hash:hash(),execution:run()},missing=oracleFiles.filter(path=>!covered(path));
 const native=cacheInternals.nativeTargetCommandInputs(fixture.project.targets.build,workspace,[]),generic=cacheInternals.genericTargetCommandInputs(fixture.project.targets.build,workspace,[]),evidence={schemaAdmitted:true,readOnlyProductionSourceInference:true,configured:{command:configured.command,cwd:configured.cwd},patterns,coveredFiles,oracleFiles,missing,before,after,native,generic};
 writeFileSync(join(output,"configured-input-evidence.json"),JSON.stringify(evidence,null,2)+"\n");console.log("[DEBUG] Configured source authority witness "+JSON.stringify(evidence));
 assert.notEqual(before.execution,after.execution,"Actual configured command must consume changed external source");assert.deepEqual(missing,[],"Every configured entry/import must be covered by inferred base inputs");assert.notEqual(before.hash,after.hash,"Changed configured bytes must change task source identity");
 for(const path of oracleFiles){assert(native.includes("{workspaceRoot}/"+path));assert(generic.includes("{workspaceRoot}/"+path));}
 console.log("[DEBUG] Configured command/cwd closure agrees with actual inference, Nx input expansion, esbuild and execution-byte hash oracles");
}
