import {expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import {cargoScratchBaseV1} from "../🏖️scratch/🟦️.ts";

const here=fileURLToPath(new URL(".",import.meta.url)),script=resolve(here,"../../📜️script.ts"),cargo=process.env.SEMIO_TEST_CARGO??"cargo",base=cargoScratchBaseV1();
type Row={kind:string;crate:string;reason?:string};
const events=(cache:string,lane:string):Row[]=>{const path=join(cache,"x",lane+".jsonl");return existsSync(path)?readFileSync(path,"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as Row):[];};
/** 🦀️ One Cargo build of the leaf through the unit-cache wrapper in one lane; the inner compiler records every command line before it execs rustc. The scratch workspace lives below `cargoScratchBaseV1`, outside every enclosing Cargo configuration, workspace manifest and toolchain file, so no repository `.cargo/config.toml` reaches Cargo. */
function build(workspace:string,cache:string,lane:string,record:string,extra:Record<string,string>={}):Promise<{code:number;output:string}>{
 const laneRoot=join(cache,"l",lane);mkdirSync(laneRoot,{recursive:true});
 const environment:Record<string,string|undefined>={...process.env,RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_FOCUS:fixture.focus,SEMIO_UNIT_CACHE_INNER:JSON.stringify(["sh","-c",'printf "%s\\n" "$*" >> "$C2_RECORD"; exec rustc "$@"',"sh"]),C2_RECORD:record,CARGO_TARGET_DIR:join(laneRoot,"t"),CARGO_TERM_COLOR:"never",...extra};
 for(const name of["RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","CARGO_INCREMENTAL","RUSTFLAGS","CARGO_ENCODED_RUSTFLAGS"])if(!(name in extra))delete environment[name];
 return new Promise(done=>{const child=spawn(cargo,["build","--manifest-path",join(workspace,"Cargo.toml"),"-p",fixture.focus],{cwd:workspace,env:environment as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});let output="";child.stdout.on("data",chunk=>{output+=chunk;});child.stderr.on("data",chunk=>{output+=chunk;});child.on("close",code=>done({code:code??1,output}));});
}

test.skipIf(process.platform==="win32")("a workspace unit that misses the cache keeps Cargo's incremental session in the lane and is still published, exactly the units plain Cargo compiles incrementally, so an edit of a dependency of the focused crate recompiles it incrementally; a build without incremental compiles none, and the independent oracle agrees",async()=>{
 const root=mkdtempSync(join(base,"ui-")),workspace=join(root,"ws"),cache=join(root,"cache"),record=join(root,"record.log"),quiet=join(root,"quiet.log");
 const crate=(name:string,dependencies:string,source:string)=>{mkdirSync(join(workspace,name,"src"),{recursive:true});writeFileSync(join(workspace,name,"Cargo.toml"),`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\n${dependencies}`);writeFileSync(join(workspace,name,"src","lib.rs"),source);};
 mkdirSync(workspace,{recursive:true});writeFileSync(join(workspace,"Cargo.toml"),`[workspace]\nresolver = "2"\nmembers = [${fixture.crates.map(name=>`"${name}"`).join(",")}]\n[profile.dev]\ndebug = 0\n`);
 crate("core","","pub fn id() -> u32 { 1 }\n");crate("leaf",'core = { path = "../core" }\n',"pub fn f() -> u32 { core::id() + 1 }\n");
 const edited=join(workspace,fixture.edited,"src","lib.rs");
 for(let round=1;round<=fixture.rounds;round++){writeFileSync(edited,readFileSync(edited,"utf8")+`pub fn r${round}() -> u32 { ${round} }\n`);const run=await build(workspace,cache,"f1",record);expect(run.code,run.output).toBe(0);}
 const rows=events(cache,"f1"),misses=rows.filter(row=>row.kind==="miss"&&row.crate===fixture.edited);
 expect(misses.length).toBe(fixture.rounds);expect(misses.every(row=>row.reason?.endsWith(" incremental:lane"))).toBe(true);
 expect(rows.filter(row=>row.kind==="publish"&&row.crate===fixture.edited).length).toBe(fixture.expect.publishedEdits);
 expect(rows.filter(row=>row.kind==="incremental"&&row.crate===fixture.focus).length).toBe(fixture.rounds);
 const kept=new Set(readFileSync(record,"utf8").split("\n").filter(line=>line.includes(fixture.flag)).map(line=>line.split(" ")[line.split(" ").indexOf("--crate-name")+1]!));
 expect([...kept].sort()).toEqual(fixture.expect.incrementalCompiles);
 const sessions=(target:string)=>{const out=new Set<string>();const walk=(directory:string):void=>{for(const name of readdirSync(directory,{withFileTypes:true}))if(name.isDirectory()){if(name.name==="incremental")for(const session of readdirSync(join(directory,name.name)))out.add(session.replace(/-[^-]+$/u,""));else walk(join(directory,name.name));}};walk(target);return[...out].sort();};
 expect(sessions(join(cache,"l","f1","t"))).toEqual(fixture.expect.incrementalCompiles);
 const plain=join(root,"plain"),reference=spawnSync(cargo,["build","--manifest-path",join(workspace,"Cargo.toml"),"-p",fixture.focus],{cwd:workspace,env:{...process.env,CARGO_TARGET_DIR:plain,RUSTC_WRAPPER:"",CARGO_INCREMENTAL:undefined,RUSTFLAGS:undefined},encoding:"utf8"});expect(reference.status,reference.stderr).toBe(0);
 expect(sessions(plain)).toEqual(sessions(join(cache,"l","f1","t")));
 const served=await build(workspace,cache,"g1",join(root,"served.log"),{SEMIO_UNIT_CACHE_FOCUS:"none"});expect(served.code,served.output).toBe(0);
 const other=events(cache,"g1");expect(other.filter(row=>row.kind==="hit").map(row=>row.crate).sort()).toEqual(fixture.expect.served.hits);expect(other.filter(row=>row.kind==="miss").map(row=>row.crate).sort()).toEqual(fixture.expect.served.misses);
 writeFileSync(edited,readFileSync(edited,"utf8")+"pub fn off() -> u32 { 0 }\n");
 const cold=await build(workspace,cache,"n1",quiet,{CARGO_INCREMENTAL:"0"});expect(cold.code,cold.output).toBe(0);
 expect(readFileSync(quiet,"utf8").split("\n").some(line=>line.includes(fixture.flag))).toBe(false);
 expect(events(cache,"n1").filter(row=>row.kind==="miss"&&row.crate===fixture.edited).every(row=>!row.reason?.includes("incremental:lane"))).toBe(true);
 const oracle=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),record,plain],{encoding:"utf8"});expect(oracle.status,oracle.stderr).toBe(0);
 expect(JSON.parse(oracle.stdout)).toEqual({recorded:[...kept].sort(),plain:sessions(plain)});
},180000);
