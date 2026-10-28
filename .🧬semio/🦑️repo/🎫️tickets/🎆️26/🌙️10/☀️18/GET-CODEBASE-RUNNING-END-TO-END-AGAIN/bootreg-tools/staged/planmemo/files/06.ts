/** 🧠 Contract: Cargo planning serves `cargo metadata` and `--unit-graph` from its memo exactly while nothing Cargo derived them from moved (source text and unrelated files or environment keep it; manifests, lock, auto-discovered targets, build scripts, Cargo-read environment, configuration and features move it), and whatever it serves equals what Cargo itself plans right now.
 * @see ../🟦️.ts
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {cargoPlanMemoV1} from "../🟦️.ts";
import {actualCargoCompilerPlanningPortV1,captureCargoCompilationGraphV1,cargoPlanningOutputBytesV1,type CargoCompilerPlanningPortV1} from "../../🟦️.ts";
import {registerCargoLockEnsurerV1} from "../../../../../../🦀️cargo-workspace/🟦️.ts";
import * as routing from "../../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../🔒️leases/🟦️.ts";

const channel=/channel\s*=\s*"([^"]+)"/u.exec(readFileSync(join(process.cwd(),"rust-toolchain.toml"),"utf8"))![1]!;
const pause=()=>new Promise(resolve=>setTimeout(resolve,schema.definitions.Protocol.const.racyMilliseconds+300));
function counted(port:CargoCompilerPlanningPortV1){const seen={plans:0};return{seen,port:{...port,async capture(program:string,args:readonly string[],...rest:[string,Readonly<Record<string,string|undefined>>,never,number]){if(args.includes("metadata")||args.includes("--unit-graph"))seen.plans++;return port.capture(program,args,...rest);}} as CargoCompilerPlanningPortV1};}
function workspace(root:string){
 const put=(path:string,body:string)=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),body);};
 put("Cargo.toml",'[workspace]\nresolver="2"\nmembers=["app","core"]\n');
 put("app/Cargo.toml",'[package]\nname="memo-app"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nmemo-core={path="../core"}\n');put("app/src/lib.rs","pub fn app() -> u8 { memo_core::core() }\n");
 put("core/Cargo.toml",'[package]\nname="memo-core"\nversion="0.1.0"\nedition="2021"\n[features]\nextra=[]\n');put("core/src/lib.rs","pub fn core() -> u8 { 1 }\n");
 return put;
}

test("the record schema accepts what the memo writes",async()=>{expect(new Ajv({strict:false}).compile(schema)({version:1,key:"0".repeat(64),inputs:[{kind:"text",path:"/x/Cargo.toml",answer:"absent"}],metadata:{},units:{}})).toBe(true);});

test("planning is served from the memo exactly while Cargo's inputs hold, and every served plan equals Cargo's own plan now",async()=>{
 registerCargoLockEnsurerV1(async()=>"current");
 const base=realpathSync(mkdtempSync(join(tmpdir(),"cpm-")));
 try{for(const row of fixture.cases)await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"cargo:plan-memo-law",maximumElapsedMilliseconds:120000},{native:{fixture:"cargo-plan-memo"}},Date.now()),async original=>{
  const control:ResourceLeaseControl=routing.createIssuedResourceControlV1(original,"cargo:plan-memo-law",0);
  const root=join(base,row.id.replaceAll(" ","-")),put=workspace(root),memo=cargoPlanMemoV1(join(base,"memo-"+root.length+row.id.length,row.id.replaceAll(" ","-")));let environment:Record<string,string|undefined>={...process.env,RUSTUP_TOOLCHAIN:channel,CARGO_TARGET_DIR:join(root,"..",row.id.replaceAll(" ","-")+"-target"),CARGO_NET_OFFLINE:"true"};let args=["test","--no-run","--manifest-path",join(root,"Cargo.toml"),"-p","memo-app"];
  expect(spawnSync("cargo",["generate-lockfile","--offline","--manifest-path",join(root,"Cargo.toml")],{env:environment,encoding:"utf8"}).status).toBe(0);
  const plan=(port:CargoCompilerPlanningPortV1)=>captureCargoCompilationGraphV1({workspaceRoot:root,cwd:root,program:"cargo",prefix:[],args,environment,maximumOutputBytes:cargoPlanningOutputBytesV1},control,port);
  await pause();const first=counted(actualCargoCompilerPlanningPortV1({plans:memo}));await plan(first.port);expect([row.id,"cold",first.seen.plans]).toEqual([row.id,"cold",2]);
  if(row.edit==="source")put("core/src/lib.rs","pub fn core() -> u8 { 2 }\n");
  if(row.edit==="unrelated")put("notes/readme.md","notes\n");
  if(row.edit==="manifest")put("core/Cargo.toml",readFileSync(join(root,"core/Cargo.toml"),"utf8")+"more=[]\n");
  if(row.edit==="workspace")put("Cargo.toml",readFileSync(join(root,"Cargo.toml"),"utf8")+'[profile.dev]\nopt-level=1\n');
  if(row.edit==="lock")put("Cargo.lock",readFileSync(join(root,"Cargo.lock"),"utf8")+"\n");
  if(row.edit==="test-target")put("app/tests/smoke.rs","#[test] fn smoke() {}\n");
  if(row.edit==="nested-binary"){mkdirSync(join(root,"app/src/bin/tool"),{recursive:true});await plan(counted(actualCargoCompilerPlanningPortV1({plans:memo})).port);await pause();put("app/src/bin/tool/main.rs","fn main() {}\n");}
  if(row.edit==="build-script")put("core/build.rs","fn main() {}\n");
  if(row.edit==="environment")environment={...environment,CARGO_PROFILE_TEST_DEBUG:"0"};
  if(row.edit==="unrelated-environment")environment={...environment,SEMIO_PLAN_MEMO_LAW:"1"};
  if(row.edit==="configuration")put(".cargo/config.toml","[profile.dev]\ndebug=0\n");
  if(row.edit==="features")args=[...args,"--features","memo-core/extra"];
  const again=counted(actualCargoCompilerPlanningPortV1({plans:memo})),served=await plan(again.port),fresh=await plan(actualCargoCompilerPlanningPortV1());
  expect([row.id,again.seen.plans===0]).toEqual([row.id,row.served]);
  expect([row.id,served]).toEqual([row.id,fresh]);
 });}finally{rmSync(base,{recursive:true,force:true});}
},300_000);
