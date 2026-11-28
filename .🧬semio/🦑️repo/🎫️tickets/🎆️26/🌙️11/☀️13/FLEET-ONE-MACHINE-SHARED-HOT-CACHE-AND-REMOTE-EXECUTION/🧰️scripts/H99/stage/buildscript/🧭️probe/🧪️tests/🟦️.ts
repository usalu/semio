/** 🧭️ A build script that only asks the compiler for its version, reads declared or keyed-absent environment, or names its host-bound reason, runs on another machine with the same key as on any host. The oracles: the real `rustc` (the stand-in must answer byte-identically), a Python model over every build script of the cargo cache (`🐍️.py`), and two simulated hosts that must compute one key. The rules are language-agnostic fixtures. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🔣️.json" with {type:"json"};
import {BUILD_PROBE_POLICY_V1,buildScriptProbeV1,buildScriptVerdictV1,expandedSourceV1,rustcStandInAnswerV1,rustcStandInTextV1,toolchainDigestV1} from "../🟦️.ts";
import {buildRunKeyInputV1} from "../../🔑️run-key/🟦️.ts";
import {portableKeyV1,portableRootsV1} from "../../../🛰️fleet/🪪️identity/🟦️.ts";

const declared=(name:string):boolean=>["TARGET","HOST","PROFILE","OPT_LEVEL","DEBUG","NUM_JOBS","OUT_DIR","CARGO_MANIFEST_DIR"].includes(name)||/^(CARGO_PKG_|CARGO_CFG_|CARGO_FEATURE_)/u.test(name);

test("the policy validates against its own schema", () => {
 const ajv=new Ajv({strict:false}),validate=ajv.compile({...schema.definitions.Policy,definitions:schema.definitions});
 expect(validate(BUILD_PROBE_POLICY_V1)).toBe(true);
});

test("every fixture script gets its declared verdict", () => {
 for(const row of fixtures.scripts){
  const verdict=buildScriptVerdictV1({source:row.source,dependencies:row.dependencies,environment:row.environment as Record<string,string>,declared,include:path=>(row as {include?:Record<string,string>}).include?.[path]}),probe=buildScriptProbeV1({source:expandedSourceV1(row.source,path=>(row as {include?:Record<string,string>}).include?.[path]),dependencies:row.dependencies,targetOs:(row.environment as Record<string,string>).CARGO_CFG_TARGET_OS}).class;
  expect({name:row.name,kind:verdict.kind,probe}).toEqual({name:row.name,kind:row.kind,probe:row.probe});
  if(verdict.kind==="host-bound"||verdict.kind==="needs-toolchain")expect({name:row.name,reason:verdict.reason}).toEqual({name:row.name,reason:(row as {reason:string}).reason});
  if(verdict.kind==="host-bound")expect(verdict.en.length>0&&verdict.de.length>0).toBe(true);
  if(verdict.kind==="portable"){
   const wanted=row as {standIn:boolean;absent?:string[];carry?:string[][]};
   expect({name:row.name,standIn:verdict.standIn,absent:verdict.absent,carry:verdict.carry}).toEqual({name:row.name,standIn:wanted.standIn,absent:wanted.absent??[],carry:wanted.carry??[]});
  }
 }
});

test("the stand-in model answers the fixture arguments, and refuses the rest", () => {
 for(const row of fixtures.standIn.answers){
  const answer=rustcStandInAnswerV1(fixtures.standIn.identity,row.arguments);
  expect(answer).toBe(row.output===null?undefined:row.output==="all"?fixtures.standIn.identity:row.output);
 }
});

const workDirectory=():string=>mkdtempSync(join(tmpdir(),"h99-"));

test("the shell stand-in answers like the model and like the real compiler (oracle: rustc itself)", () => {
 const real=(...args:string[])=>spawnSync("rustc",args,{encoding:"utf8"});
 const identity=real("-vV");
 if(identity.status!==0)return;
 const directory=workDirectory();
 try{
  mkdirSync(join(directory,"in/tools"),{recursive:true});mkdirSync(join(directory,"out-dir"),{recursive:true});
  const tool=join(directory,"in/tools/rustc");writeFileSync(tool,rustcStandInTextV1());chmodSync(tool,0o755);writeFileSync(tool+".identity",identity.stdout);
  const stand=(...args:string[])=>spawnSync(tool,args,{encoding:"utf8",env:{PATH:process.env.PATH!,OUT_DIR:join(directory,"out-dir")}});
  for(const args of [["--version"],["-V"],["-vV"],["--version","--verbose"]]){
   const mine=stand(...args),oracle=real(...args);
   expect({args,status:mine.status,out:mine.stdout}).toEqual({args,status:oracle.status,out:oracle.stdout});
   expect(mine.stdout).toBe(rustcStandInAnswerV1(identity.stdout,args)!);
  }
  const refused=stand("--print","cfg");
  expect(refused.status).toBe(BUILD_PROBE_POLICY_V1.standIn.exitStatus);
  expect(readFileSync(join(directory,"out-dir",BUILD_PROBE_POLICY_V1.standIn.refusedMarker),"utf8")).toBe("");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const packageOn=(directory:string,source:string):string=>{
 mkdirSync(directory,{recursive:true});
 writeFileSync(join(directory,"Cargo.toml"),'[package]\nname="probe"\nversion="0.1.0"\nedition="2021"\n');writeFileSync(join(directory,"build.rs"),source);
 return directory;
};
const runKey=(host:Readonly<{home:string;lane:string;rustc:string;manifest:string;wrapper?:string;extra?:Record<string,string>}>,identity:string)=>{
 const environment:Record<string,string>={TARGET:"aarch64-apple-darwin",HOST:"aarch64-apple-darwin",PROFILE:"release",RUSTC:host.rustc,HOME:host.home,CARGO_MANIFEST_DIR:host.manifest,CARGO_PKG_VERSION:"1.0.228",...host.wrapper===undefined?{}:{RUSTC_WRAPPER:host.wrapper},...host.extra};
 return portableKeyV1(buildRunKeyInputV1({toolchain:identity,identity:"unit:"+"a".repeat(64),args:[],environment,laneRoot:host.lane}),portableRootsV1({cwd:host.manifest,workspace:undefined,hints:[host.manifest,host.home]}));
};

test("two simulated hosts compute one run key for a version-probing script; another toolchain, keyed value or script splits it, a host-bound script keeps its plain key", () => {
 const directory=workDirectory(),{identity}=fixtures.standIn,gitSource=fixtures.scripts.find(row=>row.name==="git-revision")!.source;
 try{
  const first=packageOn(join(directory,"alpha/pkg"),fixtures.scripts[0]!.source),second=packageOn(join(directory,"beta/other/pkg"),fixtures.scripts[0]!.source);
  const hostA={home:"/Users/alpha",lane:"/Users/alpha/.cache/lane-1",rustc:"/Users/alpha/.rustup/toolchains/x/bin/rustc",manifest:first};
  const hostB={home:"/home/beta",lane:"/home/beta/work/lane-9",rustc:"/opt/rust/bin/rustc",manifest:second,wrapper:"/home/beta/bin/sccache"};
  expect(runKey(hostA,identity)).toBe(runKey(hostB,identity));
  expect(runKey(hostA,identity.replace("1.99.0","1.99.1"))).not.toBe(runKey(hostA,identity));
  expect(runKey({...hostA,extra:{DEP_PRETTYPLEASE02_VERSION:"0.2"}},identity)).not.toBe(runKey(hostA,identity));
  const asks=packageOn(join(directory,"gamma/pkg"),fixtures.scripts[0]!.source.replace('"--version"','"--version\").arg(\"-vV"'));
  expect(runKey({...hostA,manifest:asks},identity)).toBe(runKey({...hostB,manifest:packageOn(join(directory,"delta/pkg"),fixtures.scripts[0]!.source.replace('"--version"','"--version\").arg(\"-vV"'))},identity));
  const bound=packageOn(join(directory,"eps/pkg"),gitSource),boundOther=packageOn(join(directory,"zeta/x/pkg"),gitSource);
  expect(runKey({...hostA,manifest:bound},identity)).not.toBe(runKey({...hostB,manifest:boundOther},identity));
  expect(toolchainDigestV1(identity)).toMatch(/^[0-9a-f]{64}$/u);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
