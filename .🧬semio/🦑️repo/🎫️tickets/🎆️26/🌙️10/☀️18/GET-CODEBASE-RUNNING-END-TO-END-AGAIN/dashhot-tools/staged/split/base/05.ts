import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {appendFileSync,chmodSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {withBuildTestInvocationV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import {captureDashboardSources,changedDashboardSourceV1,currentDashboardV1,digestDashboardSources,staleDashboard,type DashboardExecutionOptions} from "../../../../📚️library/📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts";
import installationSchema from "../../../../📚️library/📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/🎛️dashboard/📦️installation/🧬️schema/🔣️json/🟦️.ts";
import {buildAndInstallDashboard} from "../../../🛤️task-routes/🟦️.ts";
import project from "../../../📦️packages/🦀️rust/📋️project.json" with {type:"json"};
import fixture from "../../🧫️fixtures/🧊️execution/🧾️compiled.json" with {type:"json"};

const name=process.platform==="win32"?"semio.exe":"semio",bytes="fixture executable";
const mode=():"prototype"|"release"=>{const value=process.env.SEMIO_BUILD_MODE;if(value!=="prototype"&&value!=="release")throw Error("Original dashboard fixture mode required");return value;};
let own:string|undefined;
const scratch=()=>{const root=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!root)throw Error("Original dashboard test artifact root required");return own??=mkdtempSync(join(root,"c"+process.pid.toString(36)+"-"));};
afterAll(()=>{if(own)rmSync(own,{recursive:true,force:true});});

/** 🌱️ Writes the fixture tree, starts the recorded build after it and installs a record of it beside an immutable fixture executable. */
async function installedTree(label:string){
 const root=mkdtempSync(join(scratch(),label+"-")),selected=mode();
 for(const[path,content]of Object.entries(fixture.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}
 Bun.sleepSync(5);const builtAtMs=Date.now();Bun.sleepSync(5);
 const sources=await captureDashboardSources(join(root,fixture.package),root),digest="b".repeat(64),path=`.cache/b/t/m/dashboard-cli/${selected}/${digest}/${name}`,executable=join(root,path);
 mkdirSync(dirname(executable),{recursive:true});writeFileSync(executable,bytes);chmodSync(executable,0o755);
 const record={version:4,platform:process.platform,mode:selected,path,sources,compiled:{builtAtMs,sources:fixture.compiled},digest,bytes:Buffer.byteLength(bytes),permissions:statSync(executable).mode&0o777},file=join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",selected,"installed.json");
 mkdirSync(dirname(file),{recursive:true});writeFileSync(file,JSON.stringify(record));
 return{root,selected,sources,record,executable};
}

/** ✏️ Applies one authored mutation to a fixture tree. */
function mutate(root:string,kind:string,path:string|null):void{
 if(kind==="none"||path===null)return;const file=join(root,path);
 if(kind==="write")appendFileSync(file," ");
 else if(kind==="read")readFileSync(file);
 else if(kind==="remove")rmSync(file);
 else if(kind==="replace-keep-times"){const before=statSync(file),swap=file+".swap";writeFileSync(swap,readFileSync(file,"utf8").replace("{}","[]"));utimesSync(swap,before.atime,before.mtime);renameSync(swap,file);}
 else throw Error("Authored mutation kind required: "+kind);
}

test("an installation is stale exactly when a digested source or a file its executable was compiled from changed after the build started",async()=>{
 const database=new Database(":memory:");
 try{for(const[index,row]of fixture.mutations.entries()){
  const tree=await installedTree("m"+index);expect([...tree.sources.roots]).toEqual(fixture.roots);mutate(tree.root,row.kind,row.path);
  const facts=fixture.compiled.map(path=>{try{const facts=statSync(join(tree.root,path));return{missing:0,mtime:facts.mtimeMs,ctime:facts.ctimeMs};}catch{return{missing:1,mtime:0,ctime:0};}});
  const independent=database.query("select count(*) changed from json_each(?1) where json_extract(value,'$.missing')=1 or max(json_extract(value,'$.mtime'),json_extract(value,'$.ctime'))>?2").get(JSON.stringify(facts),tree.record.compiled.builtAtMs) as {changed:number};
  expect(Boolean(independent.changed),row.name).toBe(row.compiled);
  expect(changedDashboardSourceV1(tree.root,tree.record.compiled)!==undefined,row.name).toBe(row.compiled);
  expect(await digestDashboardSources(tree.root,tree.sources)!==tree.sources.digest,row.name).toBe(row.digest);
  const reason=await staleDashboard(tree.root,tree.selected);expect(reason!==undefined,row.name).toBe(row.digest||row.compiled);
  if(row.compiled&&!row.digest)expect(reason,row.name).toBe(`its compiled source ${row.path} changed after the build that produced it`);
  expect(currentDashboardV1(tree.root,tree.selected,tree.root,await captureDashboardSources(join(tree.root,fixture.package),tree.root))===undefined,row.name).toBe(row.digest||row.compiled);
 }}finally{database.close();}
});

test("a current installation answers its immutable executable only while its recorded bytes and permissions are in place",async()=>{
 for(const[index,row]of fixture.executables.entries()){
  const tree=await installedTree("e"+index);
  if(row.kind==="remove")rmSync(tree.executable);else if(row.kind==="append")appendFileSync(tree.executable,"!");else if(row.kind==="chmod")chmodSync(tree.executable,0o444);
  const answer=currentDashboardV1(tree.root,tree.selected,tree.root,tree.sources);
  expect(answer?.executable,row.name).toBe(row.current?tree.executable:undefined);if(answer)expect(answer.compiled,row.name).toBe(fixture.compiled.length);
 }
});

test("the dashboard compile publishes exactly the sources record the installation reads, and an installation record requires it",()=>{
 expect(project.targets.compile.metadata.semio.rustBuildIntent.sourcesRecord).toBe(installationSchema.definitions.CompiledSourceProtocol.const.record);
 const validate=new Ajv({strict:true}).compile(installationSchema),record={version:4,platform:"darwin",mode:"prototype",path:`.cache/b/t/m/dashboard-cli/prototype/${"c".repeat(64)}/semio`,sources:{roots:["app"],manifests:["Cargo.lock"],digest:"1:"+"d".repeat(40)},compiled:{builtAtMs:1,sources:["app/🦀️.rs"]},digest:"c".repeat(64),bytes:1,permissions:0o755};
 expect(validate(record),JSON.stringify(validate.errors)).toBe(true);
 expect(validate(Object.fromEntries(Object.entries(record).filter(([key])=>key!=="compiled")))).toBe(false);
 expect(validate({...record,version:3})).toBe(false);
 expect(validate({...record,compiled:{builtAtMs:1,sources:[]}})).toBe(false);
});

test("a build of a current installation runs no nested compile and says so in one line, a stale one goes on to build",async()=>withBuildTestInvocationV1(async original=>{
 const current=await installedTree("b0"),stale=await installedTree("b1");mutate(stale.root,"write","schema/🔣️.json");
 const options=(root:string):DashboardExecutionOptions=>({workspace:root,mode:mode(),network:{offline:true},control:scriptCompilerAdmissionControlV1(original,"dashboard:fixture",120000),invocation:original});
 const lines:string[]=[],write=process.stderr.write;process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as typeof process.stderr.write;
 try{expect(await buildAndInstallDashboard(join(current.root,fixture.package),current.root,options(current.root))).toBe(current.executable);}finally{process.stderr.write=write;}
 const said=lines.filter(line=>line.startsWith(`[dashboard] ${mode()} `));expect(said).toHaveLength(1);expect(said[0]).toContain(String(Number.parseInt(current.sources.digest)));expect(said[0]).toContain(String(fixture.compiled.length));
 await expect(buildAndInstallDashboard(join(stale.root,fixture.package),stale.root,options(stale.root))).rejects.toThrow("📋️project.json");
}));
test("an installation is stale once the Cargo profile policy its compile consumed changes",async()=>{
 const {renderCargoProfileConfigurationV1}=await import("../../../../📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts");
 const target=({"darwin:arm64":"aarch64-apple-darwin","darwin:x64":"x86_64-apple-darwin","linux:arm64":"aarch64-unknown-linux-gnu","linux:x64":"x86_64-unknown-linux-gnu","win32:x64":"x86_64-pc-windows-msvc"} as Record<string,string>)[process.platform+":"+process.arch]!;
 const policy=renderCargoProfileConfigurationV1({version:1,mode:mode(),target,producer:"artifact",operation:"build"},"Cargo.toml","repository").plan.sources.map(row=>row.path);
 expect(policy.length).toBeGreaterThan(0);expect(installationSchema.definitions.SourceInputProtocol.const.rootFiles).toEqual(expect.arrayContaining(policy));
 for(const[index,path]of policy.entries()){
  const tree=await installedTree("q"+index),file=join(tree.root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",tree.selected,"installed.json"),capture=()=>captureDashboardSources(join(tree.root,fixture.package),tree.root);
  for(const each of policy){mkdirSync(dirname(join(tree.root,each)),{recursive:true});writeFileSync(join(tree.root,each),"{}");}
  writeFileSync(file,JSON.stringify({...tree.record,sources:await capture()}));
  expect(currentDashboardV1(tree.root,tree.selected,tree.root,await capture())?.executable,path).toBe(tree.executable);
  mutate(tree.root,"write",path);
  expect(currentDashboardV1(tree.root,tree.selected,tree.root,await capture()),path).toBeUndefined();
  expect(await staleDashboard(tree.root,tree.selected),path).toStartWith("its sources changed since installation");
 }
});
