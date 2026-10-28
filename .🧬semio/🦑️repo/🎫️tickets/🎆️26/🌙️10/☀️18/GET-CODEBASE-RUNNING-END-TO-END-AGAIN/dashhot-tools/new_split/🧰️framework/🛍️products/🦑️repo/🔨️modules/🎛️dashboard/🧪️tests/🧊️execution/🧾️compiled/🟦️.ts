import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {appendFileSync,chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {withBuildTestInvocationV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import {captureDashboardSources,changedDashboardSourceV1,currentDashboardArtifactV1,digestDashboardSources,installDashboardArtifactV1,staleDashboard,type DashboardExecutionOptions} from "../../../../📚️library/📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts";
import installationSchema from "../../../../📚️library/📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/🎛️dashboard/📦️installation/🧬️schema/🔣️json/🟦️.ts";
import {buildDashboardArtifactV1} from "../../../🛤️task-routes/🟦️.ts";
import project from "../../../📦️packages/🦀️rust/📋️project.json" with {type:"json"};
import fixture from "../../🧫️fixtures/🧊️execution/🧾️compiled.json" with {type:"json"};

const name=process.platform==="win32"?"semio.exe":"semio",bytes="fixture executable",sha=(data:string|Buffer)=>createHash("sha256").update(data).digest("hex"),digest=sha(bytes),protocol=installationSchema.definitions.ArtifactProtocol.const;
const mode=():"prototype"|"release"=>{const value=process.env.SEMIO_BUILD_MODE;if(value!=="prototype"&&value!=="release")throw Error("Original dashboard fixture mode required");return value;};
let own:string|undefined;
const scratch=()=>{const root=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!root)throw Error("Original dashboard test artifact root required");return own??=mkdtempSync(join(root,"c"+process.pid.toString(36)+"-"));};
afterAll(()=>{if(own)rmSync(own,{recursive:true,force:true});});

/** 🌱️ Writes the fixture tree, starts the recorded build after it and leaves both the built artifact and an installation of the same bytes. */
async function recordedTree(label:string){
 const root=mkdtempSync(join(scratch(),label+"-")),selected=mode(),state=join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",selected);
 for(const[path,content]of Object.entries(fixture.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}
 Bun.sleepSync(5);const builtAtMs=Date.now();Bun.sleepSync(5);
 const sources=await captureDashboardSources(join(root,fixture.package),root),artifactPath=`.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/${selected}/${protocol.directory}/${name}`,installedPath=`.cache/b/t/m/dashboard-cli/${selected}/${digest}/${name}`;
 for(const path of[artifactPath,installedPath]){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),bytes);chmodSync(join(root,path),0o755);}
 const compiled={builtAtMs,sources:fixture.compiled,digests:fixture.compiled.map(path=>sha(readFileSync(join(root,path))))};
 const artifact={version:5,platform:process.platform,mode:selected,path:artifactPath,sources,compiled,digest,bytes:Buffer.byteLength(bytes),permissions:statSync(join(root,artifactPath)).mode&0o777},installed={...artifact,path:installedPath};
 writeFileSync(join(state,protocol.directory,protocol.record),JSON.stringify(artifact));writeFileSync(join(state,"installed.json"),JSON.stringify(installed));
 return{root,selected,sources,artifact,packageRoot:join(root,fixture.package),artifactExecutable:join(root,artifactPath),installedExecutable:join(root,installedPath)};
}

/** ✏️ Applies one authored mutation to a fixture tree. */
function mutate(root:string,kind:string,path:string|null):void{
 if(kind==="none"||path===null)return;const file=join(root,path);
 if(kind==="write")appendFileSync(file," ");
 else if(kind==="read")readFileSync(file);
 else if(kind==="remove")rmSync(file);
 else if(kind==="rewrite-same"){const swap=file+".swap";writeFileSync(swap,readFileSync(file));renameSync(swap,file);}
 else if(kind==="replace-keep-times"){const before=statSync(file),swap=file+".swap";writeFileSync(swap,readFileSync(file,"utf8").replace("{}","[]"));utimesSync(swap,before.atime,before.mtime);renameSync(swap,file);}
 else throw Error("Authored mutation kind required: "+kind);
}

/** 🎛️ The execution options of one fixture root under the test's original invocation. */
const optionsOf=(original:Parameters<Parameters<typeof withBuildTestInvocationV1>[0]>[0],root:string):DashboardExecutionOptions=>({workspace:root,mode:mode(),network:{offline:true},control:scriptCompilerAdmissionControlV1(original,"dashboard:fixture",120000),invocation:original});

test("a file of the freshness basis is changed exactly when it is missing or touched after the build started with other bytes; the installation is also stale on its stat digest",async()=>{
 const database=new Database(":memory:");
 try{for(const[index,row]of fixture.mutations.entries()){
  const tree=await recordedTree("m"+index);expect([...tree.sources.roots]).toEqual(fixture.roots);mutate(tree.root,row.kind,row.path);
  const facts=fixture.compiled.map((path,position)=>{try{const facts=statSync(join(tree.root,path));return{missing:0,mtime:facts.mtimeMs,ctime:facts.ctimeMs,now:sha(readFileSync(join(tree.root,path))),then:tree.artifact.compiled.digests[position]};}catch{return{missing:1,mtime:0,ctime:0,now:"",then:""};}});
  const independent=database.query("select count(*) changed from json_each(?1) where json_extract(value,'$.missing')=1 or (max(json_extract(value,'$.mtime'),json_extract(value,'$.ctime'))>=?2 and json_extract(value,'$.now')<>json_extract(value,'$.then'))").get(JSON.stringify(facts),tree.artifact.compiled.builtAtMs) as {changed:number};
  expect(Boolean(independent.changed),row.name).toBe(row.compiled);
  expect(changedDashboardSourceV1(tree.root,tree.artifact.compiled)!==undefined,row.name).toBe(row.compiled);
  expect(await digestDashboardSources(tree.root,tree.sources)!==tree.sources.digest,row.name).toBe(row.digest);
  const reason=await staleDashboard(tree.root,tree.selected);expect(reason!==undefined,row.name).toBe(row.digest||row.compiled);
  if(row.compiled&&!row.digest)expect(reason,row.name).toBe(`its compiled source ${row.path} changed after the build that produced it`);
  expect(currentDashboardArtifactV1(tree.root,tree.selected)===undefined,row.name).toBe(row.compiled);
 }}finally{database.close();}
});

test("a current artifact answers its executable only while its recorded bytes and permissions are in place",async()=>{
 for(const[index,row]of fixture.executables.entries()){
  const tree=await recordedTree("e"+index);
  if(row.kind==="remove")rmSync(tree.artifactExecutable);else if(row.kind==="append")appendFileSync(tree.artifactExecutable,"!");else if(row.kind==="chmod")chmodSync(tree.artifactExecutable,0o444);
  const answer=currentDashboardArtifactV1(tree.root,tree.selected);
  expect(answer?.executable,row.name).toBe(row.current?tree.artifactExecutable:undefined);if(answer)expect(answer.compiled,row.name).toBe(fixture.compiled.length);
 }
});

test("build is a cacheable artifact producer whose only output is the artifact directory, and both records share one schema that requires the compiled basis with its digests",()=>{
 expect(project.targets.compile.metadata.semio.rustBuildIntent.sourcesRecord).toBe(installationSchema.definitions.CompiledSourceProtocol.const.record);
 const build=project.targets.build as {cache:boolean;outputs:string[];metadata:{semio:{cache:unknown;execution:{kind:string}}}};
 expect([build.cache,build.metadata.semio.cache,build.metadata.semio.execution.kind]).toEqual([true,{version:1,class:"deterministic",reason:"pure-function-of-inputs"},"orchestration"]);
 expect(build.outputs).toEqual([`{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/{options.env.SEMIO_BUILD_MODE}/${protocol.directory}`]);
 expect([project.targets.install.cache,project.targets.install.dependsOn,project.targets.install.metadata.semio.cache.class]).toEqual([false,["build"],"side-effecting"]);
 const validate=new Ajv({strict:true}).compile(installationSchema),record={version:5,platform:"darwin",mode:"prototype",path:`.cache/b/t/m/dashboard-cli/prototype/${"c".repeat(64)}/semio`,sources:{roots:["app"],manifests:["Cargo.lock"],digest:"1:"+"d".repeat(40)},compiled:{builtAtMs:1,sources:["app/🦀️.rs","Cargo.lock"],digests:["e".repeat(64),""]},digest:"c".repeat(64),bytes:1,permissions:0o755};
 expect(validate(record),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...record,path:`.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/prototype/${protocol.directory}/semio`})).toBe(true);
 expect(validate(Object.fromEntries(Object.entries(record).filter(([key])=>key!=="compiled")))).toBe(false);
 expect(validate({...record,compiled:{builtAtMs:1,sources:["app/🦀️.rs"]}})).toBe(false);
 expect(validate({...record,compiled:{...record.compiled,digests:["short",""]}})).toBe(false);
 expect(validate({...record,version:4})).toBe(false);
});

test("installing an artifact the installation already names answers without the tools store and refreshes only its record, and a missing, foreign or stale artifact is refused",async()=>withBuildTestInvocationV1(async original=>{
 const tree=await recordedTree("i0");mutate(tree.root,"rewrite-same","app/🎮️registry/🦀️.rs");
 expect(await staleDashboard(tree.root,tree.selected)).toMatch(/^its sources changed since installation/);
 expect(await installDashboardArtifactV1(tree.packageRoot,tree.root,optionsOf(original,tree.root))).toBe(tree.installedExecutable);
 expect(existsSync(join(tree.root,".cache/b/l"))).toBe(false);
 expect(await staleDashboard(tree.root,tree.selected)).toBeUndefined();
 const stale=await recordedTree("i1");mutate(stale.root,"write","schema/🔣️.json");
 await expect(installDashboardArtifactV1(stale.packageRoot,stale.root,optionsOf(original,stale.root))).rejects.toThrow("dashboard.artifact-behind-sources");
 const foreign=await recordedTree("i2"),file=join(foreign.root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",foreign.selected,protocol.directory,protocol.record);
 writeFileSync(file,JSON.stringify({...foreign.artifact,platform:process.platform==="linux"?"darwin":"linux"}));
 await expect(installDashboardArtifactV1(foreign.packageRoot,foreign.root,optionsOf(original,foreign.root))).rejects.toThrow("dashboard.artifact-required");
 const missing=await recordedTree("i3");rmSync(missing.artifactExecutable);
 await expect(installDashboardArtifactV1(missing.packageRoot,missing.root,optionsOf(original,missing.root))).rejects.toThrow("dashboard.artifact-required");
}));

test("a build of a current artifact runs no nested compile and says so in one line, a stale one goes on to build",async()=>withBuildTestInvocationV1(async original=>{
 const current=await recordedTree("b0"),stale=await recordedTree("b1");mutate(current.root,"rewrite-same","schema/🔣️.json");mutate(stale.root,"write","schema/🔣️.json");
 const lines:string[]=[],write=process.stderr.write;process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as typeof process.stderr.write;
 try{expect(await buildDashboardArtifactV1(current.packageRoot,current.root,optionsOf(original,current.root))).toBe(current.artifactExecutable);}finally{process.stderr.write=write;}
 const said=lines.filter(line=>line.startsWith(`[dashboard] ${mode()} `));expect(said).toHaveLength(1);expect(said[0]).toContain(String(Number.parseInt(current.sources.digest)));expect(said[0]).toContain(String(fixture.compiled.length));
 await expect(buildDashboardArtifactV1(stale.packageRoot,stale.root,optionsOf(original,stale.root))).rejects.toThrow("📋️project.json");
}));
