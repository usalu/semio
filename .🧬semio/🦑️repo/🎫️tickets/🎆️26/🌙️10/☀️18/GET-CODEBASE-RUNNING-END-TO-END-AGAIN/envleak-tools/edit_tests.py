#!/usr/bin/env python3
"""Edits the existing law files in a work root: the witness law for ambient variables, the stale fingerprint harness and its new action-key law, and the suite registry. `edit_tests.py <work-root>`. Idempotent."""
import json, os, sys

R = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BZ = R + "/🔧️tools/🏗️bazel"
root = os.path.abspath(sys.argv[1])

def edit(rel, pairs):
    path = os.path.join(root, rel)
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        if new in text:
            continue
        if text.count(old) != 1:
            sys.exit(f"anchor not unique/missing in {rel}: {old[:100]!r} ({text.count(old)})")
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)

witness_law = '''test("an ambient host, agent or session variable and the identity of the agent are never an input: reading it (also through a spread) leaves no row and no enumeration entry, and its change never breaks the witness while another variable still does",async()=>{
 const{base,root}=world(),ambient={CLAUDE_FIXTURE_ID:"a",ANTHROPIC_FIXTURE_URL:"u",XPC_FIXTURE_FLAGS:"0x0",__CFFixture:"b",TERM_SESSION_ID:"w0t0p0",SECURITYSESSIONID:"q",LaunchInstanceID:"l"},identity={SEMIO_AGENT_SESSION:"claude/s1",AI_AGENT:"claude-code"},given={...ambient,...identity,SEMIO_FIXTURE_VALUE:"a"};
 try{
  const witness=run(base,root,`const copy={...process.env};for(const name of Object.keys(copy))process.env[name];globalThis.__semioWorldWitnessV1.finish();`,given);
  const names=new Set(witness.observations.filter(row=>row[0]==="e").map(row=>row[1])),enumeration=witness.observations.find(row=>row[0]==="E")![2];
  for(const name of [...Object.keys(ambient),...Object.keys(identity)]){expect(names.has(name),name).toBe(false);expect(enumeration.includes(name+"="),name).toBe(false);}
  expect(names.has("SEMIO_FIXTURE_VALUE")).toBe(true);expect(enumeration.includes("SEMIO_FIXTURE_VALUE=")).toBe(true);
  const now={...process.env,...given};
  expect((await verifyWorldWitnessV1(witness,{signal,environment:now})).holds).toBe(true);
  expect((await verifyWorldWitnessV1(witness,{signal,environment:{...now,CLAUDE_FIXTURE_ID:"b",XPC_FIXTURE_FLAGS:"0x1",TERM_SESSION_ID:"w0t1p0",ANTHROPIC_FIXTURE_URL:undefined,SEMIO_AGENT_SESSION:"codex/s9",AI_AGENT:undefined}})).holds).toBe(true);
  expect((await verifyWorldWitnessV1(witness,{signal,environment:{...now,SEMIO_FIXTURE_VALUE:"b"}})).holds).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
});
'''
anchor = 'test("an allowed spawn and an explicit observation are recorded,'
edit(R + "/🔭️witness/🧪️tests/🟦️.ts", [(anchor, witness_law + anchor)])

fp = BZ + "/🧪️tests/📥️fingerprint/🟦️.ts"
edit(fp, [
    ('import {createNativeBuildCallerCapabilitiesV1,issueBuildTaskEnvironmentV1} from "../../../../📥️caller/🟦️.ts";\n',
     'import {createNativeBuildCallerCapabilitiesV1,issueBuildTaskEnvironmentV1} from "../../../../📥️caller/🟦️.ts";\nimport {runBazelRuntimeCommandV1,type BazelCommandCapturerV1} from "../../🌿️environment/🟦️.ts";\nimport ambientCorpus from "./🔣️ambient.json";\nimport ambientSchema from "./🧬️ambient.json";\n'),
    ('definition("issueBazelTaskEnvironmentV1")+"\\n"+definition("bazelTaskBaseEnvironmentV1")+"\\n"+definition("taskFingerprint")',
     'definition("issueBazelTaskEnvironmentV1")+"\\n"+definition("bazelTaskBaseEnvironmentV1")+"\\n"+definition("bazelRuntimeStdoutV1")+"\\n"+definition("taskFingerprint")'),
    ('const constant=(name:string)',
     'const keyed=(capture:(program:string,args:string[],root:string,options:{env:NodeJS.ProcessEnv})=>Promise<string>)=>({worldWitnessV1:()=>undefined,runBazelRuntimeCommandV1:(root:string,runtime:string,environment:NodeJS.ProcessEnv,control:Parameters<typeof runBazelRuntimeCommandV1>[3])=>runBazelRuntimeCommandV1(root,runtime,environment,control,(async(program,args,cwd,options)=>({stdout:await capture(program,[...args],cwd,{env:options.env as NodeJS.ProcessEnv}),stderr:"",status:0,signal:null})) satisfies BazelCommandCapturerV1)});\nconst constant=(name:string)'),
])
text = open(os.path.join(root, fp), encoding="utf8").read()
if "...keyed(" not in text:
    marker = "captureOwnedCommandV1:"
    assert text.count(marker) == 3, text.count(marker)
    text = text.replace("captureOwnedCommandV1:async()=>outputs[position++]}", "...keyed(async()=>outputs[position++])}")
    text = text.replace('captureOwnedCommandV1:async()=>""}', '...keyed(async()=>"")}')
    old = 'captureOwnedCommandV1:async(_program:string,_arguments:string[],_root:string,options:{env:NodeJS.ProcessEnv})=>{runtime.push(options.env);'
    assert text.count(old) == 1
    start = text.index(old)
    end = text.index("return output;}},code=", start)
    body = text[start + len("captureOwnedCommandV1:"):end + len("return output;}")]
    text = text[:start] + "..." + "keyed(" + body + ")" + text[end + len("return output;}"):]
    assert text.count(marker) == 0, text.count(marker)
    open(os.path.join(root, fp), "w", encoding="utf8").write(text)

ambient_law = '''
test("Bazel action keys never depend on an ambient host, agent or session variable or on a diagnostic switch, only on the declared inputs",async()=>{
 expect(new Ajv({strict:true}).validate(ambientSchema,ambientCorpus)).toBe(true);const root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{SEMIO_BUILD_MODE:"prototype"}),task={id:"fixture:build",project:"fixture",target:"build",root:"fixture",args:[],definition:{execution:{kind:"command",command:"repo ./📜️script.ts",env:{}},inputs:[{env:"FIXTURE_DECLARED"},{runtime:"fixture-runtime"}]}},fingerprints:string[]=[];
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:ambient-fingerprint",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async original=>{
  for(const row of ambientCorpus.rows){
   const base=Object.fromEntries(Object.entries({...process.env,...row.ambient}).filter(([_key,value])=>value!==null)),bindings={runtimeIdentityV1,bazelNativeToolchainClassV1:()=>undefined,createHash,randomUUID,bazelClosureMarkerV1:()=>undefined,bazelCargoClassV1:()=>undefined,bazelCargoToolchainDigestV1:async()=>"",bazelGoClassV1:()=>false,bazelGoToolchainDigestV1:async()=>"",process:{...process,env:base},bazelTaskInputsV1:(_graph:unknown,selected:typeof task)=>selected.definition.inputs,issueBuildTaskEnvironmentV1,buildExecEnvironmentV1,resolve,checkScriptInvocation,...keyed(async(_program,_arguments,_root,options)=>"v1.2.3\\n"+(options.env.SEMIO_REPO_PROFILE===undefined?"":JSON.stringify({pid:randomUUID()})+"\\n")+(Object.keys(options.env).some(name=>name.startsWith("CLAUDE"))?"agent\\n":""))},run=Function(...Object.keys(bindings),fingerprintCode())(...Object.values(bindings));
   fingerprints.push(await run(root,{},task,[],new Map(),original,createIssuedResourceControlV1(original,"bazel:ambient-fingerprint",0),true,"implementation"));
  }
 });
 for(const[index,row]of ambientCorpus.rows.entries())expect(fingerprints[index]===fingerprints[0],row.name).toBe(row.sameAsFirst);
},30000);
'''
text = open(os.path.join(root, fp), encoding="utf8").read()
if "never depend on an ambient host" not in text:
    text = text.rstrip("\n") + "\n" + ambient_law
    open(os.path.join(root, fp), "w", encoding="utf8").write(text)

corpus = {"rows": [
    {"name": "no ambient variable", "ambient": {}, "sameAsFirst": True},
    {"name": "the phase profile as json", "ambient": {"SEMIO_REPO_PROFILE": "json"}, "sameAsFirst": True},
    {"name": "the phase profile as table", "ambient": {"SEMIO_REPO_PROFILE": "1"}, "sameAsFirst": True},
    {"name": "an agent session", "ambient": {"CLAUDE_CODE_SESSION_ID": "s-1", "CLAUDECODE": "1", "ANTHROPIC_BASE_URL": "https://example.invalid"}, "sameAsFirst": True},
    {"name": "another agent session with desktop and terminal variables", "ambient": {"CLAUDE_CODE_SESSION_ID": "s-2", "XPC_FLAGS": "0x1", "__CFBundleIdentifier": "app", "TERM_SESSION_ID": "w1t0p0", "SECURITYSESSIONID": "q2"}, "sameAsFirst": True},
    {"name": "an undeclared custom variable", "ambient": {"FIXTURE_UNDECLARED": "x"}, "sameAsFirst": True},
    {"name": "the declared variable", "ambient": {"FIXTURE_DECLARED": "x"}, "sameAsFirst": False},
    {"name": "the build mode", "ambient": {"SEMIO_BUILD_MODE": "release"}, "sameAsFirst": True},
]}
dir_ = os.path.join(root, BZ, "🧪️tests/📥️fingerprint")
json.dump(corpus, open(os.path.join(dir_, "🔣️ambient.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
open(os.path.join(dir_, "🔣️ambient.json"), "a").write("\n")
ambient_schema = {"$schema": "http://json-schema.org/draft-07/schema#", "type": "object", "additionalProperties": False, "required": ["rows"], "properties": {"rows": {"type": "array", "minItems": 2, "items": {"type": "object", "additionalProperties": False, "required": ["name", "ambient", "sameAsFirst"], "properties": {"name": {"type": "string", "minLength": 1}, "ambient": {"type": "object", "additionalProperties": {"type": ["string", "null"]}}, "sameAsFirst": {"type": "boolean"}}}}}}
json.dump(ambient_schema, open(os.path.join(dir_, "🧬️ambient.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
open(os.path.join(dir_, "🧬️ambient.json"), "a").write("\n")

suites = os.path.join(root, R, "🗂️suites/🔣️.json")
text = open(suites, encoding="utf8").read()
old = '        "🔧️tools/🏗️bazel/🎬️executor",\n'
new = old + '        "🔧️tools/🏗️bazel/🌿️environment",\n'
if '"🔧️tools/🏗️bazel/🌿️environment"' not in text:
    assert text.count(old) == 1
    text = text.replace(old, new)
watch = '        "👁️observation"\n'
if '"🔭️witness"' not in text:
    assert text.count(watch) == 1
    text = text.replace(watch, '        "👁️observation",\n        "🔭️witness"\n')
open(suites, "w", encoding="utf8").write(text)
