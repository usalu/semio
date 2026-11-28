#!/usr/bin/env python3
"""H99 landing edit of the build script offload: wires the probe verdict (stage/buildscript/probe) into offloadBuildScriptRunV1 and removes the superseded source-marker policy.
Usage: apply-offload.py <buildscript dir to edit>. Run on a scratch copy; the landing diff is `diff -ru` of the result against the real directory."""
import sys, json, re
b = sys.argv[1]
f = b + "/☁️offload/🟦️.ts"; s = open(f).read()
def sub(old, new):
    global s
    assert old in s, old[:70]
    s = s.replace(old, new, 1)
sub('import policySchema from "./🧬️schema/🔣️.json" with {type:"json"};', 'import policySchema from "./🧬️schema/🔣️.json" with {type:"json"};\nimport {BUILD_PROBE_POLICY_V1,buildScriptVerdictV1,rustcStandInTextV1} from "../🧭️probe/🟦️.ts";\nimport {buildDependencyNamesV1,buildScriptSourcePathV1} from "../🧭️probe/🗝️key/🟦️.ts";')
a = s.index("/** 🔎️ Why the source text of a build script"); e = s.index("/** 🔎️ Why the bytes of a compiled")
s = s[:a] + s[e:]
sub('const sourceMarkers=policy.sourceMarkers.map(row=>({reason:row.reason,pattern:new RegExp(row.pattern,"u")}));\n', '')
sub("/** ☁️ The authored policy of a build script run on another machine: what a hermetic script may read, the markers of the ones that are not, and the layout of the worker's private root.", "/** ☁️ The authored policy of a build script run on another machine: the layout of the worker's private root, the environment a command carries and the markers of compiled scripts that are not hermetic (what a script may ask of the host is `🧭️probe`).")
sub('export function buildScriptBinaryRefusalV1(bytes:Uint8Array):string|undefined{\n const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength);\n for(const row of binaryMarkers)if(buffer.includes(row.bytes))return row.reason;',
    'export function buildScriptBinaryRefusalV1(bytes:Uint8Array,answered:readonly string[]=[]):string|undefined{\n const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength);\n for(const row of binaryMarkers)if(!answered.includes(row.reason)&&buffer.includes(row.bytes))return row.reason;')
sub('`undefined` when none is. */\nexport function buildScriptBinaryRefusalV1', '`undefined` when none is; the reasons in `answered` (the compiler questions the stand-in answers) do not count. */\nexport function buildScriptBinaryRefusalV1')
sub('  if(policy.environment.refused.some(prefix=>name===prefix||name.startsWith(prefix)))return{refusal:"environment:"+name.slice(0,40)};\n', '')
sub('input:Readonly<{real:string;args:readonly string[];environment:Environment;cwd:string;root:string;lane:string;crate:string;key:string;outDir:string;', 'input:Readonly<{real:string;args:readonly string[];environment:Environment;toolchain:string;cwd:string;root:string;lane:string;crate:string;key:string;outDir:string;')
a = s.index("/** 📜️ The source file of a package"); e = s.index("/** 🔎️ Why the bytes of a compiled")
s = s[:a] + s[e:]
sub('return refuse("source");', 'return refuse("host-bound:source");')
sub('return refuse("build-dependency:"+row);', 'return refuse("host-bound:build-dependency:"+row);')
old = ' const sourceRefusal=buildScriptSourceRefusalV1(source);if(sourceRefusal!==undefined)return refuse(sourceRefusal);\n let binary:Buffer;try{binary=readFileSync(input.real);}catch{return refuse("unreadable-input");}\n const binaryRefusal=buildScriptBinaryRefusalV1(binary);if(binaryRefusal!==undefined)return refuse(binaryRefusal);\n const variables=buildScriptEnvironmentV1(environment,source);if("refusal" in variables)return refuse(variables.refusal);'
new = ''' const verdict=buildScriptVerdictV1({source,dependencies:buildDependencyNamesV1(manifest),environment,declared:declaredName,include:path=>{try{return readFileSync(join(manifestDir,path),"utf8");}catch{return undefined;}}});
 if(verdict.kind!=="portable")return refuse(verdict.reason);
 let binary:Buffer;try{binary=readFileSync(input.real);}catch{return refuse("unreadable-input");}
 const binaryRefusal=buildScriptBinaryRefusalV1(binary,verdict.standIn?["compiler-probe"]:[]);if(binaryRefusal!==undefined)return refuse(binaryRefusal);
 const variables=buildScriptEnvironmentV1(environment,source);if("refusal" in variables)return refuse(variables.refusal);
 const probe=BUILD_PROBE_POLICY_V1.standIn,standIn=verdict.standIn?{text:rustcStandInTextV1()}:undefined;'''
sub(old, new)
sub('const rootFile=join(scratch,"root"),inputs=new Map(files);inputs.set(policy.layout.input+"/"+policy.layout.executable,input.real);',
'''const rootFile=join(scratch,"root"),inputs=new Map(files);inputs.set(policy.layout.input+"/"+policy.layout.executable,input.real);
  if(standIn!==undefined){const standInFile=join(scratch,"rustc"),identityFile=join(scratch,"rustc.identity");writeFileSync(standInFile,standIn.text);chmodSync(standInFile,0o755);writeFileSync(identityFile,input.toolchain);inputs.set(probe.file,standInFile);inputs.set(probe.identity,identityFile);}''')
sub('variables:variables.variables,root,', 'variables:[...variables.variables,...verdict.carry],root,')
sub('  const marker=join(input.outDir,policy.layout.marker);\n  rmSync(marker,{force:true});', '''  const marker=join(input.outDir,policy.layout.marker),refused=join(input.outDir,probe.refusedMarker),unanswered=existsSync(refused);
  rmSync(marker,{force:true});rmSync(refused,{force:true});
  if(unanswered){discard();return refuse("toolchain-probe:unanswered-arguments");}''')
open(f, "w").write(s)
pj = b + "/☁️offload/🧬️schema/🔣️.json"; j = json.load(open(pj))
pol = j["policy"]; pd = j["definitions"]["Policy"]
for k in ("sourceMarkers", "allEnvironment", "environmentRead"):
    pol.pop(k, None); pd["properties"].pop(k, None)
    if k in pd["required"]: pd["required"].remove(k)
pol["environment"].pop("refused"); e = pd["properties"]["environment"]; e["required"].remove("refused"); e["properties"].pop("refused")
for r in pol["binaryMarkers"]:
    if r["text"] == "RUSTC": r["reason"] = "compiler-probe"
pol["buildDependencies"] = [x for x in pol["buildDependencies"] if x not in ("autocfg", "version_check", "rustc_version")]
i = pol["tool"]["lines"].index("chmod 755 in/run"); pol["tool"]["lines"][i + 1:i + 1] = ["[ -f in/tools/rustc ] && chmod 755 in/tools/rustc && export RUSTC=\"$root/in/tools/rustc\""]
j["description"] = j["description"].replace("the markers of the scripts that are not hermetic (they spawn programs, reach the network or the host's libraries, read outside their package) and the stand-in tool", "the markers of compiled scripts that are not hermetic and the stand-in tool")
json.dump(j, open(pj, "w"), ensure_ascii=False, indent=2); open(pj, "a").write("\n")
tf = b + "/☁️offload/🧪️tests/🟦️.ts"; t = open(tf).read()
t = t.replace("buildScriptSourceRefusalV1,", "")
for old, new in (('detail:"reads-outside-manifest"', 'detail:"host-bound:host-path"'), ('build-script: reads-outside-manifest', 'build-script: host-bound:host-path'), ('detail:"environment:DEP_Z_ROOT"', 'detail:"host-bound:host-value"')):
    assert old in t, old
    t = t.replace(old, new)
t = re.sub(r"\n for\(const row of fixtures\.sources\)expect\(buildScriptSourceRefusalV1[^\n]*", "", t)
open(tf, "w").write(t)
tj = b + "/☁️offload/🧪️tests/🔣️.json"; x = json.load(open(tj)); x.pop("sources")
for r in x["environments"]:
    if r["name"] in ("dependency metadata", "links"): r["refusal"] = None; r["variables"] = [["CARGO_MANIFEST_DIR", "@root/in/manifest"], ["OUT_DIR", "@root/out/out-dir"], ["PATH", "/usr/bin:/bin:/usr/sbin:/sbin"]]
for r in x["binaries"]:
    if r["name"] == "compiler variable": r["reason"] = "compiler-probe"
json.dump(x, open(tj, "w"), ensure_ascii=False, indent=1); open(tj, "a").write("\n")

t = open(tf).read()
t = t.replace("offloadBuildScriptRunV1({real:pack.binary,args:[],", "offloadBuildScriptRunV1({real:pack.binary,args:[],toolchain:IDENTITY,")
t = t.replace("type Package=Readonly", '''const COMPILER=join(spawnSync("rustc",["--print","sysroot"],{encoding:"utf8"}).stdout.trim(),"bin","rustc"),IDENTITY=spawnSync(COMPILER,["-vV"],{encoding:"utf8"}).stdout;
const ASKS_VERSION=`use std::{env,fs,process::Command};
fn main(){
 let out=env::var("OUT_DIR").unwrap();
 let text=String::from_utf8(Command::new(env::var_os("RUSTC").unwrap()).arg("--version").output().unwrap().stdout).unwrap();
 let minor:u32=text.split('.').nth(1).unwrap().parse().unwrap();
 fs::write(format!("{}/minor.txt",out),format!("{}",minor)).unwrap();
 println!("cargo:rustc-cfg=rustc_minor_{}",minor);
}
`;
const ASKS_CFG=`use std::{env,process::Command};
fn main(){
 let out=Command::new(env::var_os("RUSTC").unwrap()).arg("--print").arg("cfg").output().unwrap();
 println!("cargo:warning={}",out.stdout.len());
}
`;

type Package=Readonly''', 1)
marker = 'test("a script that reads a path outside its package stays here'
assert marker in t
t = t.replace(marker, '''test("a script that asks the compiler for its version runs on another machine and gets the answer of the declared toolchain, byte for byte",async()=>{
 const pack=packageOf("asks-version",ASKS_VERSION),outDir=join(scratch,"asks-version","out"),extra={RUSTC:COMPILER},expected=local(pack,outDir,extra);
 expect(expected.status).toBe(0);
 const wanted=tree(outDir);rmSync(outDir,{recursive:true,force:true});mkdirSync(outDir,{recursive:true});
 await ready();
 const answer=await offloadBuildScriptRunV1({real:pack.binary,args:[],toolchain:IDENTITY,environment:cargoEnvironment(pack.manifest,outDir,extra),cwd:pack.manifest,root:join(scratch,"offload"),lane:"d",crate:"asks-version",key:"v".repeat(64),outDir});
 expect(answer,JSON.stringify(answer)).toMatchObject({kind:"done"});
 expect((answer as {stdout:string}).stdout).toBe(expected.stdout);
 expect([...tree(outDir)]).toEqual([...wanted]);
 expect(existsSync(join(outDir,BUILD_PROBE_POLICY_V1.standIn.refusedMarker))).toBe(false);
},120000);

test("a script that asks the compiler more than its version runs here, the output of the worker withdrawn",async()=>{
 const pack=packageOf("asks-cfg",ASKS_CFG),outDir=join(scratch,"asks-cfg","out");mkdirSync(outDir,{recursive:true});
 await ready();
 const answer=await offloadBuildScriptRunV1({real:pack.binary,args:[],toolchain:IDENTITY,environment:cargoEnvironment(pack.manifest,outDir,{RUSTC:COMPILER}),cwd:pack.manifest,root:join(scratch,"offload"),lane:"d",crate:"asks-cfg",key:"c".repeat(64),outDir});
 expect(answer).toEqual({kind:"local",reason:"not-portable",detail:"toolchain-probe:unanswered-arguments"});
 expect(readdirSync(outDir)).toEqual([]);
},120000);

'''+marker, 1)
t = t.replace('import {runBuildScriptV1}', 'import {BUILD_PROBE_POLICY_V1} from "../../🧭️probe/🟦️.ts";\nimport {runBuildScriptV1}', 1)
open(tf, "w").write(t)

t = open(tf).read()
t = t.replace("buildScriptEnvironmentV1,buildScriptSourcePathV1,", "buildScriptEnvironmentV1,", 1)
t = t.replace('import {BUILD_PROBE_POLICY_V1} from "../../🧭️probe/🟦️.ts";', 'import {BUILD_PROBE_POLICY_V1} from "../../🧭️probe/🟦️.ts";\nimport {buildScriptSourcePathV1} from "../../🧭️probe/🗝️key/🟦️.ts";', 1)
open(tf, "w").write(t)

rk = b + "/🔑️run-key/🟦️.ts"; r = open(rk).read()
def rsub(old, new):
    global r
    assert old in r, old[:70]
    r = r.replace(old, new, 1)
rsub("export function buildRunKeyInputV1(input:Readonly<{toolchain:string;identity:string;args:readonly string[];environment:Environment;laneRoot:string|undefined}>):Parameters<typeof unitKeyV1>[0]{\n return{toolchain:input.toolchain,normalized:[\"--semio-build-script-run\",...input.args],sourceSha256:\"run:\"+input.identity,externs:[],environment:buildKeyEnvironmentV1(input.environment,input.laneRoot)};",
     "export function buildRunKeyInputV1(input:Readonly<{toolchain:string;identity:string;args:readonly string[];environment:Environment;laneRoot:string|undefined}>):Parameters<typeof unitKeyV1>[0]{\n const rows=buildKeyEnvironmentV1(input.environment,input.laneRoot);\n return{toolchain:input.toolchain,normalized:[\"--semio-build-script-run\",...input.args],sourceSha256:\"run:\"+input.identity,externs:[],environment:buildScriptProbeKeyV1({environment:input.environment,toolchain:input.toolchain,keyEnvironment:rows})??rows};")
rsub('import {buildKeyEnvironmentV1,sha256V1,type unitKeyV1} from "../../🔏️key/🟦️.ts";', 'import {buildKeyEnvironmentV1,sha256V1,type unitKeyV1} from "../../🔏️key/🟦️.ts";\nimport {buildScriptProbeKeyV1} from "../🧭️probe/🗝️key/🟦️.ts";')
rsub("the run's environment with the lane, the compiler flags and the tool paths neutralised.", "the run's environment with the lane, the compiler flags and the tool paths neutralised (for a script that only asks the compiler for its version or reads keyed environment: without compiler wrappers, plus the carried and absent names and the toolchain digest, `🧭️probe`).")
open(rk, "w").write(r)

bs = b + "/🟦️.ts"; c = open(bs).read()
def csub(old, new):
    global c
    assert old in c, old[:70]
    c = c.replace(old, new, 1)
csub("runOffloadedV1({real,digest,args,environment,cwd,root,lane,crate,key,outDir,", "runOffloadedV1({real,digest,args,environment,toolchain:keyInput.toolchain,cwd,root,lane,crate,key,outDir,")
csub("async function runOffloadedV1(input:Readonly<{real:string;digest:string|null;args:readonly string[];environment:Environment;cwd:string;", "async function runOffloadedV1(input:Readonly<{real:string;digest:string|null;args:readonly string[];environment:Environment;toolchain:string;cwd:string;")
csub("offloadBuildScriptRunV1({real:input.real,args:input.args,environment:input.environment,cwd:input.cwd,", "offloadBuildScriptRunV1({real:input.real,args:input.args,environment:input.environment,toolchain:input.toolchain,cwd:input.cwd,")
open(bs, "w").write(c)
