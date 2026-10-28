"""🪪 [td-leases] batch `envelope`: a test file declares the process birth its suite grants, so a bare `bun test ./file` runs under the same capabilities instead of failing on the missing parent envelope. usage: python3 -I edit_envelope.py <root>  (anchored edits, each anchor must occur exactly once; idempotent refusal when already applied)"""
import os, sys
root = sys.argv[1]
P = "🧰️framework/🔨️modules/🏃️process/"
def edit(rel, pairs):
    path = os.path.join(root, rel)
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        n = text.count(old)
        if n != 1: sys.exit(f"anchor x{n} in {rel}: {old[:80]!r}")
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)
    print("edited", rel)
H = P + "🧪️testing/📥️invocation/🟦️.ts"
edit(H, [
 ('import {relative} from "node:path";\n',
  'import {mkdtempSync,rmSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {join,relative,resolve} from "node:path";\n'),
 ('import {receiveScriptProcessInvocation,requireIssuedScriptInvocationV1,',
  'import {SCRIPT_PROCESS_INVOCATION_ENV,createScriptProcessEnvelope,withScriptProcessEnvelope,receiveScriptProcessInvocation,requireIssuedScriptInvocationV1,'),
 ('const pending=new Set<Pending>(),',
  '/** 🪪️ The process birth a test file declares for a run without an issuing parent (a bare `bun test ./file`): its owner, its elapsed budget and exactly the authored capabilities its suite row grants (`artifacts` is `SEMIO_TEST_ARTIFACT_DIR`, else a private temporary directory published as `SEMIO_TEST_ARTIFACT_DIR` like the suite runner does and removed after the run). */\nexport type BuildTestEnvelopeV1=Readonly<{owner:string;maximumElapsedMilliseconds:number;capabilities:Readonly<{artifacts?:true;command?:string}>}>;\nlet declared:BuildTestEnvelopeV1|undefined;\n/** 🧾️ Declares the birth of this test process before its first callback (files of one bare multi-file run may repeat the identical declaration, one birth serves them all, a different one is refused): without a parent envelope the receiver is born in process from it, beneath a real parent the inherited capabilities must carry exactly the declared ones, so a suite row and its file cannot drift apart. */\nexport function declareBuildTestEnvelopeV1(envelope:BuildTestEnvelopeV1):void{\n if(typeof envelope.owner!=="string"||envelope.owner===""||!Number.isSafeInteger(envelope.maximumElapsedMilliseconds)||envelope.maximumElapsedMilliseconds<=0||Object.keys(envelope.capabilities).some(key=>key!=="artifacts"&&key!=="command"))throw Error("Finite closed build test envelope required");\n const exact=Object.freeze({owner:envelope.owner,maximumElapsedMilliseconds:envelope.maximumElapsedMilliseconds,capabilities:Object.freeze({...(envelope.capabilities.artifacts?{artifacts:true as const}:{}),...(envelope.capabilities.command!==undefined?{command:envelope.capabilities.command}:{})})});\n if(declared!==undefined){if(JSON.stringify(declared)===JSON.stringify(exact))return;throw Error("One test process receives one birth: "+exact.owner+" declares another envelope than the one "+declared.owner+" declared");}\n if(ready!==undefined)throw Error("Build test envelope declared after the first callback: "+exact.owner);\n declared=exact;\n}\n/** 🐣️ Receives the parent\'s envelope, or, when no parent issued one and the file declared its birth, issues that declared birth in process. */\nfunction receiveBuildTestBirthV1(body:(original:IssuedScriptInvocationV1)=>Promise<void>):Promise<void>{\n const envelope=declared;\n if(envelope===undefined)return receiveScriptProcessInvocation(process.env,body);\n if(process.env[SCRIPT_PROCESS_INVOCATION_ENV]!==undefined)return receiveScriptProcessInvocation(process.env,original=>{const actual=original.capabilities as{artifacts?:unknown;command?:unknown};if(actual.command!==envelope.capabilities.command||(envelope.capabilities.artifacts===true)!==(typeof actual.artifacts==="string"))throw Error("Inherited build test capabilities differ from the declared envelope of "+envelope.owner);return body(original);});\n const configured=process.env.SEMIO_TEST_ARTIFACT_DIR,artifacts=envelope.capabilities.artifacts?configured?resolve(configured):mkdtempSync(join(tmpdir(),"semio-build-test-")):undefined;\n if(artifacts!==undefined&&!configured)process.env.SEMIO_TEST_ARTIFACT_DIR=artifacts;\n return withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:envelope.owner,maximumElapsedMilliseconds:envelope.maximumElapsedMilliseconds},{...(artifacts!==undefined?{artifacts}:{}),...(envelope.capabilities.command!==undefined?{command:envelope.capabilities.command}:{})},Date.now()),body).finally(()=>{if(artifacts!==undefined&&!configured){delete process.env.SEMIO_TEST_ARTIFACT_DIR;rmSync(artifacts,{recursive:true,force:true});}});\n}\nconst pending=new Set<Pending>(),'),
 ('lifetime=receiveScriptProcessInvocation(process.env,async original=>{', 'lifetime=receiveBuildTestBirthV1(async original=>{'),
])
L = P + "🔒️leases/"
declare = lambda suite, caps: 'declareBuildTestEnvelopeV1({owner:"test:' + suite + '",maximumElapsedMilliseconds:300000,capabilities:' + caps + '});\n'
for rel in ["♻️retirement/🧪️tests/📋️claims/🟦️.ts", "♻️retirement/🧪️tests/📋️frontier/🟦️.ts", "♻️retirement/🧪️tests/📋️inventory/🟦️.ts"]:
    last = 'import{captureOwnedCommandResultV1}from"../../../../🎛️owned-execution/📥️capture/🟦️.ts";\n'
    edit(L + rel, [('import{registerBuildTestLawV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";', 'import{declareBuildTestEnvelopeV1,registerBuildTestLawV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";'), (last, last + declare("leases", "{artifacts:true}"))])
last = 'import schema from"./🧬️schema/🔣️.json"with{type:"json"};\n'
edit(L + "♻️retirement/🧹️sweep/🧪️tests/🟦️.ts", [('import{withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";', 'import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";'), (last, last + declare("leases", "{artifacts:true}"))])
helper = 'import{withBuildTestInvocationV1}from"../../../🧪️testing/📥️invocation/🟦️.ts";\n'
edit(L + "🧪️tests/🎟️writer/🟦️.ts", [(helper, 'import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../🧪️testing/📥️invocation/🟦️.ts";\n' + declare("resource-lease-writer", '{artifacts:true,command:"writer"}'))])
T = P + "🧪️testing/📥️invocation/🧪️tests/"
def create(rel, text):
    path = os.path.join(root, rel)
    if os.path.exists(path): sys.exit("exists: " + rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(text)
    print("created", rel)
create(T + "🧫️declared/🟦️.ts", '''/** 🪪️ Child fixture of the declared-birth law: declares the envelope `SEMIO_DECLARED_CASE` names and reports what its one callback receives. */
import{test}from"bun:test";
import{existsSync}from"node:fs";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../🟦️.ts";
const kind=process.env.SEMIO_DECLARED_CASE,writer={owner:"test:declared-writer",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true as const,command:"writer"}};
if(kind==="artifacts")declareBuildTestEnvelopeV1({owner:"test:declared-artifacts",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
else if(kind!=="undeclared")declareBuildTestEnvelopeV1(writer);
if(kind==="repeated")declareBuildTestEnvelopeV1({owner:writer.owner,maximumElapsedMilliseconds:writer.maximumElapsedMilliseconds,capabilities:{command:"writer",artifacts:true}});
if(kind==="different")declareBuildTestEnvelopeV1({...writer,capabilities:{artifacts:true}});
test("declared birth "+kind,()=>withBuildTestInvocationV1(async original=>{const facts=original.capabilities as{artifacts?:string;command?:string};console.log("[TRACE] Declared birth "+JSON.stringify({artifacts:facts.artifacts??null,command:facts.command??null,published:process.env.SEMIO_TEST_ARTIFACT_DIR??null,exists:facts.artifacts!==undefined&&existsSync(facts.artifacts)}));}));
''')
edit(T + "🟦️.ts", [
 ('import{withBuildTestInvocationV1}from"../🟦️.ts";\n', 'import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../🟦️.ts";\nimport{spawnSync}from"node:child_process";\nimport{existsSync}from"node:fs";\nimport{tmpdir}from"node:os";\ndeclareBuildTestEnvelopeV1({owner:"test:testing",maximumElapsedMilliseconds:300000,capabilities:{artifacts:true}});\n'),
 ('\n}),30000);\n', '''\n}),30000);
/** 🪪️ A file that declares its birth runs bare under exactly the declared capabilities (a private artifact directory published as SEMIO_TEST_ARTIFACT_DIR and gone afterwards, or the configured one kept), one process accepts only one birth, an undeclared bare file is still refused, and beneath a real parent the inherited capabilities must carry the declared ones. */
test("a declared birth runs a bare file under exactly its declared capabilities and is held to them beneath a real parent",()=>withBuildTestInvocationV1(async original=>{
 const facts=original.capabilities as{artifacts?:unknown};if(typeof facts.artifacts!=="string")throw Error("Authored declared-birth fixture artifacts required");
 const fixture=join(import.meta.dir,"🧫️declared/🟦️.ts"),control=createIssuedResourceControlV1(original,"fixture:declared-birth",120000),bare=Object.fromEntries(Object.entries(process.env).filter(([name])=>name!=="SEMIO_SCRIPT_PROCESS_INVOCATION"&&name!=="SEMIO_TEST_ARTIFACT_DIR")),configured=await mkdtemp(join(facts.artifacts,"declared-"));
 const run=(kind:string,env:Record<string,string|undefined>)=>{const result=spawnSync(process.execPath,["test",fixture],{cwd:process.cwd(),env:{...env,TMPDIR:tmpdir(),SEMIO_DECLARED_CASE:kind},encoding:"utf8",timeout:60000});return Promise.resolve({status:result.status,stdout:result.stdout,stderr:result.stderr});},child=(kind:string)=>captureOwnedCommandResultV1(process.execPath,["test",fixture],process.cwd(),{control,env:scriptInvocationEnvironment(original,{...bare,TMPDIR:tmpdir(),SEMIO_DECLARED_CASE:kind}),output:{maximumBytes:65536,maximumLines:512}});
 const births=(stdout:string)=>stdout.split(/\\r?\\n/u).filter(line=>line.startsWith("[TRACE] Declared birth ")).map(line=>JSON.parse(line.slice("[TRACE] Declared birth ".length)) as{artifacts:string|null;command:string|null;published:string|null;exists:boolean});
 try{
  const owned=await run("writer",bare),[birth]=births(owned.stdout);expect(owned.status,owned.stderr.slice(-4000)).toBe(0);expect(births(owned.stdout).length).toBe(1);expect(birth!.command).toBe("writer");expect(typeof birth!.artifacts).toBe("string");expect(birth!.published).toBe(birth!.artifacts);expect(birth!.exists).toBe(true);expect(existsSync(birth!.artifacts!)).toBe(false);
  const kept=await run("writer",{...bare,SEMIO_TEST_ARTIFACT_DIR:configured});expect(kept.status,kept.stderr.slice(-4000)).toBe(0);expect(births(kept.stdout)).toEqual([{artifacts:configured,command:"writer",published:configured,exists:true}]);expect(existsSync(configured)).toBe(true);
  const repeated=await run("repeated",bare);expect(repeated.status,repeated.stderr.slice(-4000)).toBe(0);expect(births(repeated.stdout).length).toBe(1);
  const different=await run("different",bare);expect(different.status).not.toBe(0);expect(different.stderr).toContain("One test process receives one birth");expect(births(different.stdout)).toEqual([]);
  const undeclared=await run("undeclared",bare);expect(undeclared.status).not.toBe(0);expect(undeclared.stderr).toContain("Original parent Script process invocation required");expect(births(undeclared.stdout)).toEqual([]);
  const inherited=await child("artifacts");expect(inherited.status,inherited.stderr.slice(-4000)).toBe(0);expect(births(inherited.stdout)).toEqual([{artifacts:facts.artifacts,command:null,published:null,exists:true}]);
  const drifted=await child("writer");expect(drifted.status).not.toBe(0);expect(drifted.stderr).toContain("Inherited build test capabilities differ from the declared envelope of test:declared-writer");expect(births(drifted.stdout)).toEqual([]);
 }finally{await rm(configured,{recursive:true,force:true});}
}),120000);
'''),
])
edit(P + "🧪️testing/🗂️suites/🔣️.json", [('    "🧪️testing/📥️invocation/🧪️tests/🧫️harness/🟦️.ts"\n  ]', '    "🧪️testing/📥️invocation/🧪️tests/🧫️declared/🟦️.ts",\n    "🧪️testing/📥️invocation/🧪️tests/🧫️harness/🟦️.ts"\n  ]')])
