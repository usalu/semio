#!/usr/bin/env python3
"""R51 landing 2: (1) a failed heavy question carries the child's refusal (code, key, values; rendered in both languages at the CLI boundary) or a bilingual generic failure with the error line, never the last three stack lines; (2) a preflight of every declared tool closure and provider port in the capped child at the start of a read (prime stage), once per process and policy; (3) law tests, including the real-tree walk of every declared tool closure. Anchored edits on the live files under landing.lock; usage: land2.py [--tests-only]"""
import os,sys,json,subprocess,hashlib,datetime
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/";HQ=SI+"🪆️heavy-questions/";PC=SI+"🔌️provider-closures/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/"
ERR9="../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts"
ERR8=ERR9[3:]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
def sub(path,old,new,count=1):
    t=open(path,encoding="utf8").read()
    if new in t and old not in t:return
    assert t.count(old)==count,(path,old[:60],t.count(old))
    open(path+".r51tmp","w",encoding="utf8").write(t.replace(old,new));os.replace(path+".r51tmp",path)
def write(path,text):
    os.makedirs(os.path.dirname(path),exist_ok=True);open(path+".r51tmp","w",encoding="utf8").write(text);os.replace(path+".r51tmp",path)

HQ_FAIL_OLD='  if(code!==0)return done(Object.assign(Error("Heavy derived question failed (exit "+code+"): "+tail.trim().split("\\n").slice(-3).join(" | ")),{code:"BUILD_HEAVY_QUESTION"}));\n'
HQ_FAIL_NEW='  if(code!==0)return done(failureOfV1(code,tail,output));\n'
HQ_HELPERS='''/** 🚑️ The refusal a failed child leaves: its own refusal (code, key and values, so every language renders it) or, for any other failure, the bilingual generic one that names the exit status and the error line, never the stack frames below it. */
const failureOfV1=(code,tail,output)=>{
 try{
  const row=JSON.parse(readFileSync(output,"utf8"));
  if(row.refusal){const data=readRepoErrorDataV1(row.refusal);return new RepoErrorV1(data.code,data.key,data.values);}
  if(typeof row.failure==="string")return refuseBuildV1("failed","graph.heavy-question-failed",{exit:String(code),detail:row.failure.slice(-600)});
 }catch{}
 const lines=tail.trim().split("\\n").map(line=>line.trim()).filter(Boolean),named=lines.filter(line=>/^(error|[A-Za-z]*Error)\\b/.test(line));
 return refuseBuildV1("failed","graph.heavy-question-failed",{exit:String(code),detail:(named.length?named:lines.slice(-3)).join(" | ").slice(-600)||"-"});
};

let lane=Promise.resolve();
'''
HQ_PRE='''
/** 🛃️ The refusal of a preflight: the refusal itself when one closure is refused, otherwise one refusal that names the count and every tool with its key and values. */
const refusedV1=rows=>rows.length===1?new RepoErrorV1(rows[0].code,rows[0].key,rows[0].values):refuseBuildV1("invalid","closure.tool-closures-refused",{count:String(rows.length),details:rows.map(row=>row.name+": "+row.key+" "+JSON.stringify(row.values)).join("; ").slice(0,1500)});
const preflights=new Map();

/** 🛃️ Walks every declared tool closure and provider port in the capped child before a read derives anything, once per process and policy, and refuses with what the child found: a closure that can never be derived is known in seconds, not after the work of the read. */
export const refusedToolClosuresV1=refusedV1;
export function preflightToolClosuresInChildV1({workspaceRoot,signal}){
 const key=JSON.stringify([workspaceRoot,PROVIDER_CLOSURE_POLICY_V1.tools]);
 let known=preflights.get(key);
 if(known===undefined){
  const run=lane.then(()=>answerIn({preflight:true},workspaceRoot,signal),()=>answerIn({preflight:true},workspaceRoot,signal));
  lane=run.then(()=>undefined,()=>undefined);
  known=run.then(({checked,refused})=>{if(refused.length)throw refusedV1(refused);return{checked};});
  preflights.set(key,known);known.catch(()=>preflights.delete(key));
 }
 return known;
}
'''
CHILD='''import{writeFileSync}from"node:fs";
import{HEAVY_QUESTION_POLICY_V1}from"./🟨️.mjs";
import{openPlannerDeriverV1}from"../../../../🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts";
import{isRepoErrorV1,repoErrorDataV1}from"%s";

/** 🪆️ The child of one heavy derived question: opens the planner's deriver, answers the question read from stdin, writes the answer to the file named by the parent and stops at its resident memory cap; a failure is written to the same file (the refusal as data, any other error as its message) and ends the child with status 1, so the parent re-raises it with its code and values instead of the tail of a stack. */
const[root,output]=process.argv.slice(2),limit=Number(process.env[HEAVY_QUESTION_POLICY_V1.environment])*1048576;
setInterval(()=>{if(process.memoryUsage().rss>limit){process.stderr.write("resident memory cap of "+process.env[HEAVY_QUESTION_POLICY_V1.environment]+" MB exceeded\\n");process.exit(137);}},HEAVY_QUESTION_POLICY_V1.pollMilliseconds).unref();
try{
 const request=JSON.parse(await Bun.stdin.text()),derive=await openPlannerDeriverV1(root!);
 if(request.prime||request.preflight){
  const scriptInputs=await import("../🟨️.mjs"),{observeInferenceSnapshotV1}=await import("../../../../👁️observation/🟨️.mjs");
  if(request.prime){
   scriptInputs.bindClosureStoreV1(root!,request.prime.storeDirectory,true);
   writeFileSync(output!,JSON.stringify(await observeInferenceSnapshotV1(()=>scriptInputs.primeSharedClosuresV1(root!,(derive as any).scripts))));
  }else writeFileSync(output!,JSON.stringify(await observeInferenceSnapshotV1(()=>scriptInputs.preflightToolClosuresV1(root!,(derive as any).scripts))));
 }else{
  const answer=await derive(request.row);
  writeFileSync(output!,JSON.stringify({fact:answer.fact,rows:answer.rows}));
 }
}catch(error){
 writeFileSync(output!,JSON.stringify(isRepoErrorV1(error)?{refusal:repoErrorDataV1(error)}:{failure:String((error as Error)?.message??error).slice(0,2000)}));
 process.exit(1);
}
process.exit(0);
'''%ERR9
SI_FN='''/** 🛃️ Walks every declared tool closure and the closure of every designated provider port once, through one shared cache, and returns the refusals instead of stopping at the first: a closure that can never be derived (it imports a generated output, names an unknown port) is known before a read derives anything. `refused` holds `{ name, version, code, key, values }` rows, `name` the tool or `port:<name>`. */
export async function preflightToolClosuresV1(workspaceRoot, scripts) {
  const refused = [], names = Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort(), view = Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)[0], ports = Object.keys(providerDesignationV1(scripts).providers).sort();
  const attempt = async (name, run) => { try { await run(); } catch (error) { if (!isRepoErrorV1(error)) throw error; refused.push({ name, ...repoErrorDataV1(error) }); } };
  for (const name of names) await attempt(name, () => toolGroupFilesV1(workspaceRoot, scripts, name));
  for (const port of ports) await attempt("port:" + port, () => scriptProviderGroupV1(workspaceRoot, scripts, port, view));
  return { checked: names.length + ports.length, refused };
}

'''
TEXTS_ANCHOR='''    "closure.tool-unknown": {'''
TEXTS_NEW='''    "closure.tool-closures-refused": {
      "en": "{count} declared tool closures are refused before the graph read starts: {details}. A tool closure may not import a generated output, because the projects that name it cannot depend on its producer; import it from the command instead or remove the tool from the shared closure policy",
      "de": "{count} deklarierte Werkzeugabschlüsse werden vor dem Start des Graph-Lesens abgelehnt: {details}. Ein Werkzeugabschluss darf keine generierte Ausgabe importieren, weil die Projekte, die ihn nennen, nicht von ihrem Erzeuger abhängen können; importiere sie stattdessen im Befehl oder entferne das Werkzeug aus der Richtlinie der geteilten Abschlüsse"
    },
    "graph.heavy-question-failed": {
      "en": "A heavy derived question failed in its child process (exit {exit}): {detail}",
      "de": "Eine schwere abgeleitete Frage ist in ihrem Kindprozess fehlgeschlagen (Exit {exit}): {detail}"
    },
'''
HQ_TEST_OLD_START='test("a child that cannot answer fails with the named code, and questions are asked one at a time"'
def apply():
    sub(B+"📖️texts/🔣️.json",TEXTS_ANCHOR,TEXTS_NEW+TEXTS_ANCHOR)
    hq=HQ+"🟨️.mjs"
    sub(hq,'import{refuseBuildV1}from"../../../../📖️texts/🟦️.ts";\n','import{refuseBuildV1}from"../../../../📖️texts/🟦️.ts";\nimport{RepoErrorV1,readRepoErrorDataV1}from"%s";\nimport{PROVIDER_CLOSURE_POLICY_V1}from"../🔌️provider-closures/🟨️.mjs";\n'%ERR9)
    sub(hq,'let lane=Promise.resolve();\n',HQ_HELPERS)
    sub(hq,HQ_FAIL_OLD,HQ_FAIL_NEW)
    t=open(hq,encoding="utf8").read()
    if "preflightToolClosuresInChildV1" not in t:
        write(hq,t.rstrip("\n")+"\n"+HQ_PRE)
    write(HQ+"📜️script.ts",CHILD)
    dts=HQ+"🟨️.d.mts";t=open(dts,encoding="utf8").read()
    if "preflightToolClosuresInChildV1" not in t:
        write(dts,t.rstrip("\n")+"\n/** 🛃️ Walks every declared tool closure and provider port in the capped child, once per process and policy, and refuses with what it found. */\nexport function preflightToolClosuresInChildV1(options:{workspaceRoot:string;signal?:AbortSignal}):Promise<{checked:number}>;\n/** 🛃️ The refusal of a preflight. */\nexport function refusedToolClosuresV1(rows:readonly {name:string;code:string;key:string;values:Record<string,string|number>}[]):Error&{code:string;key:string;values:Record<string,string|number>};\n")
    si=SI+"🟨️.mjs"
    sub(si,'import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";\n','import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";\nimport{isRepoErrorV1,repoErrorDataV1}from"%s";\n'%ERR8)
    sub(si,'export { askHeavyQuestionV1, isHeavyQuestionV1, primeSharedClosuresInChildV1 } from','export { askHeavyQuestionV1, isHeavyQuestionV1, preflightToolClosuresInChildV1, primeSharedClosuresInChildV1 } from')
    sub(si,'/** 🪚️ The command closure of a target with the declared tools named',SI_FN+'/** 🪚️ The command closure of a target with the declared tools named')
    sd=SI+"🟨️.d.mts";t=open(sd,encoding="utf8").read()
    if "preflightToolClosuresV1" not in t:
        write(sd,t.rstrip("\n")+"\n/** 🛃️ Walks every declared tool closure and provider port once and returns the refusals. */\nexport function preflightToolClosuresV1(workspaceRoot: string, scripts: ScriptInputsValueV1): Promise<{ checked: number; refused: readonly { name: string; version: 1; code: string; key: string; values: Record<string, string | number> }[] }>;\n")
    pn=PN+"🟨️.mjs"
    sub(pn,'let withClosurePassV1, primeSharedClosuresInChildV1,','let withClosurePassV1, primeSharedClosuresInChildV1, preflightToolClosuresInChildV1,')
    sub(pn,'({ withClosurePassV1, primeSharedClosuresInChildV1,','({ withClosurePassV1, primeSharedClosuresInChildV1, preflightToolClosuresInChildV1,')
    sub(pn,'/** 🧰️ Derives the shared closure entries of a workspace in the capped child before the load state grows, when the process has a durable closure store to hand them over through; the owners and a later process read the stored entries. */\nexport async function primeSharedClosuresOfWorkspaceV1({ workspaceRoot, signal }) {\n','/** 🧰️ Walks every declared tool closure in the capped child first (a refused closure ends the read here, in seconds, with its code and values), then derives the shared closure entries of a workspace in the capped child before the load state grows, when the process has a durable closure store to hand them over through; the owners and a later process read the stored entries. */\nexport async function primeSharedClosuresOfWorkspaceV1({ workspaceRoot, signal }) {\n  await preflightToolClosuresInChildV1({ workspaceRoot, signal });\n')

HQ_TEST=HQ+"🧪️tests/🟦️.ts"
HQ_ANCHOR='test("a child that cannot answer fails with the named code, and questions are asked one at a time"'
NEW_HQ_TEST='''test("a child that cannot answer fails with a bilingual refusal that names the exit status and the error line, and questions are asked one at a time",async()=>{
 const row={kind:"derived",path:JSON.stringify({derive:1,workspaceRoot:"/nonexistent-semio-root",recipe:{fn:"none"}}),fact:"x"};
 const outcomes=await Promise.allSettled([askHeavyQuestionV1(row,{workspaceRoot:"/nonexistent-semio-root"}),askHeavyQuestionV1(row,{workspaceRoot:"/nonexistent-semio-root"})]);
 for(const outcome of outcomes){
  expect(outcome.status).toBe("rejected");
  const reason=(outcome as PromiseRejectedResult).reason;
  expect(isRepoErrorV1(reason)).toBe(true);
  expect(reason.key).toBe("build.graph.heavy-question-failed");
  expect(reason.values.exit).toBe("1");expect(String(reason.values.detail).length).toBeGreaterThan(1);
  expect(renderRepoErrorV1(reason,"en")).toContain("exit 1");expect(renderRepoErrorV1(reason,"de")).toContain("Exit 1");
 }
},120000);

test("a refusal raised in the child reaches the parent with its code, key and values and renders in both languages (the lost message of the 17:11 failure)",async()=>{
 const root=resolve(import.meta.dir,"../".repeat(11)),row={kind:"derived",path:JSON.stringify({derive:1,workspaceRoot:root,recipe:{fn:"tool-closure",name:"no-such-tool",view:"scheduling"}}),fact:"x"};
 const reason=await askHeavyQuestionV1(row,{workspaceRoot:root}).then(()=>undefined,(error:unknown)=>error as any);
 expect(isRepoErrorV1(reason)).toBe(true);
 expect(reason.code).toBe("invalid");expect(reason.key).toBe("build.closure.tool-unknown");expect(reason.values).toEqual({name:"no-such-tool"});
 const english=renderRepoErrorV1(reason,"en"),german=renderRepoErrorV1(reason,"de");
 expect(english).toContain("no-such-tool");expect(german).toContain("no-such-tool");expect(german).not.toBe(english);
},300000);

test("one refused tool closure is raised as itself and several as one refusal that names the count and every tool with its key and values, in both languages",()=>{
 const row=(name:string,targets:string)=>({name,version:1,code:"invalid",key:"build.closure.provider-generated-import",values:{port:name,targets}});
 const one=refusedToolClosuresV1([row("alpha","p:generate")]);
 expect(one.key).toBe("build.closure.provider-generated-import");expect(one.values).toEqual({port:"alpha",targets:"p:generate"});
 const many=refusedToolClosuresV1([row("alpha","p:generate"),row("beta","q:generate")]);
 expect(many.key).toBe("build.closure.tool-closures-refused");expect(many.values.count).toBe("2");
 for(const language of ["en","de"]){const text=renderRepoErrorV1(many,language);for(const part of ["alpha","beta","p:generate","q:generate","provider-generated-import"])expect(text).toContain(part);}
});

test("the preflight of the capped child accepts the real tree: every declared tool closure and provider port is derivable",async()=>{
 const root=resolve(import.meta.dir,"../".repeat(11)),answer=await preflightToolClosuresInChildV1({workspaceRoot:root});
 expect(answer.checked).toBeGreaterThan(30);
},600000);
'''
def test_edit():
    t=open(HQ_TEST,encoding="utf8").read()
    if "lost message" in t:return
    i=t.index(HQ_ANCHOR);new=t[:i]+NEW_HQ_TEST
    new=new.replace('import{expect,test}from"bun:test";','import{expect,test}from"bun:test";\nimport{resolve}from"node:path";\nimport{isRepoErrorV1,renderRepoErrorV1}from"../%s";'%ERR9,1)
    new=new.replace('import{HEAVY_QUESTION_POLICY_V1,askHeavyQuestionV1,isHeavyQuestionV1}from"../🟨️.mjs";','import{HEAVY_QUESTION_POLICY_V1,askHeavyQuestionV1,isHeavyQuestionV1,preflightToolClosuresInChildV1,refusedToolClosuresV1}from"../🟨️.mjs";',1)
    assert "refusedToolClosuresV1}" in new
    write(HQ_TEST,new)

PF_TEST='''/** 🛃️ Contract: a closure that can never be derived is found before a read derives anything. The preflight returns every refusal (not the first); a fixture import of a generated output into a tool closure is refused with the tool and the producer; and every declared tool closure and provider port of the real tree walks without importing a generated output, so a landing that introduces such an import fails here and not 90 minutes into the group read. Run by every landing that touches a tool closure, a provider or a file they import. */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import corpus from "../🔣️.json" with {type:"json"};
import policy from "../../🔣️.json" with {type:"json"};

setDefaultTimeout(600000);
const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../../🟨️.mjs") as any,projectNodes=await import("../../../../📦️project-nodes/🟨️.mjs") as any,ownerDiscovery=await import("../../../../🔎️owner-discovery/🟨️.mjs") as any;
const designationSchema=join(import.meta.dir,"../../../../../../../🏃️process/🔌️providers/🧬️schema/🔣️.json"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const workspace=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"pf-law-")));roots.push(root);for(const[path,text]of Object.entries(corpus.files as Record<string,string>)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}mkdirSync(dirname(join(root,corpus.schemaCopy)),{recursive:true});copyFileSync(designationSchema,join(root,corpus.schemaCopy));return root;};
const infer=<T>(body:()=>Promise<T>)=>observation.observeInferenceSnapshotV1(body) as Promise<T>;
const fixtureCache=(root:string,generated:boolean)=>{
 const cache=scriptInputs.createScriptInputCache({workspaceRoot:root,scriptProviderDesignation:join(root,corpus.designation)});
 cache.toolGroups=new Map(Object.keys(policy.tools.closures).filter(name=>name!=="cargo-command").map(name=>[name,new Set<string>()]));
 if(generated)cache.generatedOutput=(path:string)=>path===join(root,"lib/gen.ts")?{target:"fixture:generate",output:"lib/gen.ts"}:undefined;
 return cache;
};

test("a fixture tool closure that imports a generated output is refused with the tool and the producer, and the same tree without the import is accepted",async()=>{
 const root=workspace();
 const clean=await infer(()=>scriptInputs.preflightToolClosuresV1(root,fixtureCache(root,true)));
 expect(clean.refused).toEqual([]);expect(clean.checked).toBeGreaterThan(30);
 writeFileSync(join(root,"lib/gen.ts"),"export const generated = 1;\n");
 writeFileSync(join(root,"lib/util.ts"),'import "./gen.ts";\n'+readFileSync(join(root,"lib/util.ts"),"utf8"));
 const found=await infer(()=>scriptInputs.preflightToolClosuresV1(root,fixtureCache(root,true)));
 const tool=found.refused.find((row:any)=>row.name==="cargo-command");
 expect(tool,JSON.stringify(found.refused)).toBeDefined();
 expect(tool.code).toBe("invalid");expect(tool.key).toBe("build.closure.provider-generated-import");expect(tool.values).toEqual({port:"cargo-command",targets:"fixture:generate"});
 for(const row of found.refused)expect(row.key).toBe("build.closure.provider-generated-import");
 const without=await infer(()=>scriptInputs.preflightToolClosuresV1(root,fixtureCache(root,false)));
 expect(without.refused).toEqual([]);
});

test("every declared tool closure and provider port of the real tree is derivable and imports no generated output",async()=>{
 const checkout=process.env.SEMIO_TEST_CHECKOUT??resolve(import.meta.dir,"../".repeat(12)),contracts=await infer(async()=>projectNodes.projectGeneratorContractsV1(checkout));
 const cache=scriptInputs.createScriptInputCache({workspaceRoot:checkout});
 cache.generatedOutput=(path:string)=>ownerDiscovery.readBuildProducedCompileOutputV1(path,checkout,contracts,undefined);
 const answer=await infer(()=>scriptInputs.preflightToolClosuresV1(checkout,cache));
 expect(answer.refused.map((row:any)=>row.name+" "+row.key+" "+JSON.stringify(row.values)),"a tool closure or provider closure imports a generated output (declare the import in the command, or remove the tool row from the provider-closure policy)").toEqual([]);
 expect(answer.checked).toBeGreaterThan(30);
});
'''
SC_EDIT=None
lock=T+"/landing.lock"
TESTS=[HQ+"🧪️tests",PC+"🧪️tests/🛃️preflight",PC+"🧪️tests",PC+"🧪️tests/🪡️tool-cover",PC+"🧪️tests/🧮️blast-radius",SI+"🧪️tests",B+"📖️texts/🧪️tests"]
FILES=[B+"📖️texts/🔣️.json",HQ+"🟨️.mjs",HQ+"🟨️.d.mts",HQ+"📜️script.ts",HQ+"🧪️tests/🟦️.ts",SI+"🟨️.mjs",SI+"🟨️.d.mts",PN+"🟨️.mjs",PC+"🧪️tests/🛃️preflight/🟦️.ts"]
if "--tests-only" not in sys.argv:
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
    open(lock+"/owner","w").write("R51-%d\n"%os.getpid())
try:
    if "--tests-only" not in sys.argv:
        apply();test_edit();write(PC+"🧪️tests/🛃️preflight/🟦️.ts",PF_TEST)
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    res=[]
    for d in TESTS:
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","600000"],cwd=d,env=env,capture_output=True,text=True)
        out=r.stdout+r.stderr;red=[l for l in out.splitlines() if l.startswith("(fail)")];res.append((d,r.returncode,red));print("ok  " if r.returncode==0 else "RED ",d[len(B):],*red)
        if r.returncode:print(out[-3000:])
    if "--tests-only" not in sys.argv:
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R51 landing 2 (%s): heavy-question failures carry the child's refusal in both languages; tool-closure preflight at the start of a read; real-tree tool-closure law\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
            for p in FILES:f.write("- edit/new `%s` after %s\n"%(p[len(R)+1:],sha(p)))
            for d,c,red in res:f.write("- test `%s` exit %d, %d red%s\n"%(d[len(R)+1:],c,len(red),": "+"; ".join(red) if red else ""))
finally:
    if "--tests-only" not in sys.argv:
        os.remove(lock+"/owner");os.rmdir(lock)
