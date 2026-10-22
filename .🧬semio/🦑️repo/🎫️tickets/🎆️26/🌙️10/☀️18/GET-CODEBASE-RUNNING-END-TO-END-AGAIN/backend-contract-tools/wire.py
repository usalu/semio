#!/usr/bin/env python3
"""Anchored edits of live files that replace every closed backend list and every backend branch in neutral code by the registry. Each edit is a pure text->text function: idempotent (already applied -> unchanged), refusing (Drift) when its anchor is missing or ambiguous, so it composes with the other staged sets that touch the same files."""
import json,os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,scriptslices as S
from names import n,B,T,TS,JSON,MJS
class Drift(Exception):pass
def count(text,anchor):return text.count(anchor)
def sub(text,old,new,label,all_=False):
    if new in text and old not in text:return text
    c=count(text,old)
    if c==0:raise Drift(f"{label}: anchor not found: {old[:100]!r}")
    if c>1 and not all_:raise Drift(f"{label}: anchor found {c} times: {old[:100]!r}")
    return text.replace(old,new)
def rsub(text,pattern,new,label):
    if new in text and not re.search(pattern,text):return text
    found=re.findall(pattern,text)
    if len(found)!=1:raise Drift(f"{label}: pattern matched {len(found)} times: {pattern[:100]!r}")
    return re.sub(pattern,lambda m:new,text,count=1)
def head(text,line,label):
    return text if line in text else line+"\n"+text
BK=T["BACKENDS"]
# ---- invocation
def invocation(t):
    t=head(t,f'import {{requireBuildBackendIdV1}} from "../{BK}/{TS}";',"invocation import")
    t=sub(t,'{tool:"nx"|"bazel";args:string[];workers?','{tool:string;args:string[];workers?',"invocation return type")
    t=sub(t,'if(tool!=="nx"&&tool!=="bazel")throw Error(`Unknown build tool: ${tool}`);','requireBuildBackendIdV1(tool);',"invocation check")
    t=sub(t,'SEMIO_BUILD_TOOL:caller.tool??"nx"','SEMIO_BUILD_TOOL:caller.tool??policy.defaultTool',"invocation ancestor default")
    return t
def operations(t):return sub(t,'export type BuildToolNameV1="nx"|"bazel";','export type BuildToolNameV1=string;',"operations tool name")
def caller(t):
    t=sub(t,'readonly tool:"nx"|"bazel"; readonly workspaceRoot','readonly tool:string; readonly workspaceRoot',"caller capability")
    t=sub(t,'tool:"nx"|"bazel",environment:NodeJS.ProcessEnv','tool:string,environment:NodeJS.ProcessEnv',"caller create")
    t=sub(t,'tool:"nx"|"bazel",value:unknown','tool:string,value:unknown',"caller read")
    return t
def orchestration(t):return sub(t,'!["nx","bazel"].includes(tool)||','typeof tool!=="string"||!/^[a-z][a-z0-9-]{0,31}$/u.test(tool)||',"orchestration tool check")
def orchestration_types(t):return sub(t,'workspaceRoot:string,tool:"nx"|"bazel",value:un','workspaceRoot:string,tool:string,value:un',"orchestration d.mts tool")
def remote_command(t):
    t=sub(t,'readonly tool:"nx"|"bazel";','readonly tool:string;',"remote tool")
    return sub(t,'readonly protocol:"bazel-http"|"nx-self-hosted";','readonly protocol:string;',"remote protocol")
def remote_index(t):return sub(t,'mechanism:"bazel-flags"|"nx-environment"','mechanism:string',"remote mechanism")
STORAGE_OLD_MARK='else if(tool==="bazel")'
def storage(t):
    if "loadBuildBackendV1" in t:return t
    if 'if(tool==="nx")' not in t or STORAGE_OLD_MARK not in t:raise Drift("storage: unexpected body")
    start=t.index("/** ");head_=t[:start]
    return head_+'''/** 🏦️ Admits each engine's finite physical stores before immutable graph reader custody begins; the selected backend owns what that means for it. */
export async function prepareBuildToolStorageV1(root:string,tool:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,snapshotBytes:number):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);if(!Number.isSafeInteger(snapshotBytes)||snapshotBytes<0||snapshotBytes>536870912)throw Error("Bounded exact graph snapshot bytes required");
 const {loadBuildBackendV1}=await import("../../'''+BK+'/'+TS+'''");
 await (await loadBuildBackendV1(tool)).prepareStorage({root,invocation,phase:phaseControl,args:[],tool},snapshotBytes);
 checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
}
'''
def snapshot(t):
    t=head(t,f'import {{buildNativeSnapshotProtocolV1}} from "../../../{BK}/{TS}";',"snapshot import")
    t=sub(t,'export const SELECTED_NATIVE_SNAPSHOT_ENV="SEMIO_NX_NATIVE_SOURCE_SNAPSHOT";','export const SELECTED_NATIVE_SNAPSHOT_ENV=buildNativeSnapshotProtocolV1()?.variable??"SEMIO_NATIVE_SOURCE_SNAPSHOT";',"snapshot variable")
    return sub(t,'environment.NX_DAEMON!=="false"','Object.entries(buildNativeSnapshotProtocolV1()?.environment??{}).some(([name,value])=>environment[name]!==value)',"snapshot environment")
def rust(t):
    t=head(t,f'import {{buildNativeSnapshotProtocolV1}} from "../../../{BK}/{TS}";',"rust import")
    t=sub(t,'process.env.NX_DAEMON==="true"','Object.entries(buildNativeSnapshotProtocolV1()?.environment??{}).some(([name,value])=>process.env[name]!==undefined&&process.env[name]!==value)',"rust daemon check")
    return sub(t,'{...snapshot.environment,NX_DAEMON:"false"}','{...snapshot.environment,...buildNativeSnapshotProtocolV1()?.environment}',"rust daemon write")
ID='{ "type": "string", "pattern": "^[a-z][a-z0-9-]{0,31}$" }'
def enum_schema(t):
    pat=r'\{\s*"enum":\s*\[\s*"nx",\s*"bazel"\s*\]\s*\}'
    if not re.search(pat,t) and ID in t:return t
    return re.sub(pat,ID,t)
def build_tool_schema(t):
    pat=r'"BuildTool":\s*\{\s*"type":\s*"string",\s*"enum":\s*\[\s*"nx",\s*"bazel"\s*\]\s*\}'
    if not re.search(pat,t) and '"BuildTool": { "type": "string", "pattern"' in t:return t
    if len(re.findall(pat,t))!=1:raise Drift("build schema: BuildTool enum not found once")
    return re.sub(pat,'"BuildTool": { "type": "string", "pattern": "^[a-z][a-z0-9-]{0,31}$" }',t)
def pattern_schema(t):
    if '^(nx|bazel)$' not in t:
        if '"pattern": "^[a-z][a-z0-9-]{0,31}$"' in t:return t
        raise Drift("caller schema: pattern not found")
    return t.replace('"pattern": "^(nx|bazel)$"','"pattern": "^[a-z][a-z0-9-]{0,31}$"')
def remote_schema(t):
    old='"tool": { "enum": ["nx", "bazel"] }'
    new='"tool": '+ID
    if old not in t and new in t:return t
    return sub(t,old,new,"remote-cache schema tool")
def operations_schema(t):
    if '"ToolName": { "enum"' in t:t=sub(t,'    "ToolName": { "enum": ["nx", "bazel"] },\n','    "ToolName": '+ID+',\n',"operations ToolName")
    t=t.replace('"summary", "implementations", "dryRun"','"summary", "dryRun"')
    t=re.sub(r'\n        "implementations": \{\n(?:.*\n)*?        \},',"",t,count=1)
    t=t.replace('    "Implementation": { "enum": ["native", "adapter", "neutral", "missing"] },\n',"")
    return t
def catalog(t):
    """Removes every per-tool implementation row (the registry owns them) and the closed tool lists in help text."""
    t=re.sub(r'\n[ \t]*"implementations": \{[^{}]*\},?(?=[ \t]*\n)',"",t)
    t=t.replace('"valueName": "nx|bazel"','"valueName": "{backends}"').replace('"valueName": "nx,bazel"','"valueName": "{backends}"').replace('tools setup [nx|bazel|all]','tools setup [{backends}|all]')
    return t
HELP_OLD='const root="repo [--tool=nx|bazel]",columnLimit=46;'
def help_(t):
    t=head(t,f'import {{buildBackendAlternationV1}} from "../../../{BK}/{TS}";',"help import") if False else t
    if "buildBackendAlternationV1" in t:return t
    t=sub(t,HELP_OLD,'const root="repo [--tool={backends}]",columnLimit=46;\nconst expandBackends=(value:string)=>value.replaceAll("{backends}",buildBackendAlternationV1());',"help root")
    t=t.replace('return lines.join("\\n");','return expandBackends(lines.join("\\n"));')
    t=sub(t,'export function renderBuildHelpV1(catalog:BuildHelpCatalogV1,request:BuildHelpRequestV1,language:string):string{','export function renderBuildHelpV1(catalogSource:BuildHelpCatalogV1,request:BuildHelpRequestV1,language:string):string{\n const catalog=JSON.parse(expandBackends(JSON.stringify(catalogSource))) as BuildHelpCatalogV1;',"help render signature")
    return f'import {{buildBackendAlternationV1}} from "../../{BK}/{TS}";\n'+t
# ---- incremental / parity / ci-report
def incr_model(t):
    t=head(t,f'import {{buildBackendRowV1}} from "../{BK}/{TS}";',"incr import")
    t=sub(t,'export type IncrementalToolV1="nx"|"bazel";','export type IncrementalToolV1=string;',"incr tool")
    t=t.replace('tool==="bazel"?slice.tasks[id]!.dependsOn:slice.tasks[id]!.consumes','buildBackendRowV1(tool).dependencyModel==="all"?slice.tasks[id]!.dependsOn:slice.tasks[id]!.consumes')
    t=t.replace('(tool==="bazel"?task.dependsOn:task.consumes)','(buildBackendRowV1(tool).dependencyModel==="all"?task.dependsOn:task.consumes)')
    return t
def parity_model(t):return sub(t,'export type ParityToolV1="nx"|"bazel";','export type ParityToolV1=string;',"parity tool")
def ci_model(t):return sub(t,'export type CiToolV1="nx"|"bazel";','export type CiToolV1=string;',"ci tool")

def incr_command(depth):
    def edit(t):
        t=head(t,f'import {{buildBackendAlternationV1,buildBackendIdsV1}} from "{"../"*depth}{BK}/{TS}";',"incr command import")
        t=sub(t,'tools:readonly IncrementalToolV1[]=["nx","bazel"],','tools:readonly IncrementalToolV1[]=buildBackendIdsV1(),',"incr command tools")
        t=t.replace("(nx|bazel)","(${buildBackendAlternationV1()})")
        t=t.replace('incrementalCliArgumentsV1(pre.executable,root,script,"nx",step)','incrementalCliArgumentsV1(pre.executable,root,script,tools[0]!,step)')
        return t
    return edit
def child(t):
    pat=r'command==="tools"\?\(tools!=="bazel"\?\["--version"\]:undefined\):command==="reset"\?\(selected\.tool==="nx"\?\["reset"\]:undefined\):selected\.tool==="nx"&&.*?\?nxOperationArgumentsV1\(invocation\.args\):undefined'
    new='await(await import("../../'+BK+'/'+TS+'")).buildBackendBootstrapArgumentsV1(invocation.args,selected.tool)'
    if new in t and not re.search(pat,t):return t
    if len(re.findall(pat,t))!=1:raise Drift("child: bootstrap expression not found once")
    return re.sub(pat,lambda m:new,t,count=1)

CONF_FLAG="""    {
      "name": "--keep",
      "value": "none",
      "operations": ["conformance"],
      "description": { "en": "conformance run: keep the scratch workspaces for inspection.", "de": "conformance run: behält die Arbeitsverzeichnisse zur Untersuchung." }
    },
"""
CONF_OP='''    {
      "name": "conformance",
      "requires": "dependencies",
      "synopsis": "conformance list|plan|run",
      "summary": { "en": "Judge every build backend against the same language-agnostic scenarios: plan, run, cache hit and miss, outputs, test results, affected, cancellation, progress, concurrency and remote cache.", "de": "Beurteilt jedes Build-Backend an denselben sprachunabhängigen Szenarien: Plan, Lauf, Cache-Treffer und -Fehlgriff, Ausgaben, Testergebnisse, Betroffene, Abbruch, Fortschritt, Nebenläufigkeit und Remote-Cache." },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "subcommands": [
        { "name": "list", "synopsis": "conformance list", "summary": { "en": "List the scenarios and what each backend claims for them.", "de": "Listet die Szenarien und den Anspruch jedes Backends auf." } },
        { "name": "plan", "synopsis": "conformance plan [--scenario=<ids|all>]", "summary": { "en": "Print the backend and scenario pairs a run would judge.", "de": "Gibt die Paare aus Backend und Szenario aus, die ein Lauf beurteilen würde." } },
        { "name": "run", "synopsis": "conformance run [--scenario=<ids|all>]", "summary": { "en": "Run the scenarios in isolated workspaces and judge each backend against its registry claim.", "de": "Führt die Szenarien in isolierten Arbeitsverzeichnissen aus und beurteilt jedes Backend an seinem Anspruch in der Registry." } }
      ],
      "flags": ["--scenario", "--implementations", "--keep", "--file", "--json"],
      "examples": [
        { "command": "repo conformance list", "description": { "en": "List the conformance scenarios.", "de": "Listet die Konformitätsszenarien auf." } },
        { "command": "repo conformance run --implementations=semio", "description": { "en": "Judge the in-house engine against every scenario.", "de": "Beurteilt die hauseigene Engine an allen Szenarien." } }
      ],
      "notes": [
        { "en": "A backend is judged against its claim in the backend registry: a claimed capability that fails is a regression, a declared gap that fails is reported as a gap, a declared gap that passes is a stale claim, and what the backend gives no way to observe is reported as unobserved.", "de": "Ein Backend wird an seinem Anspruch in der Backend-Registry gemessen: eine beanspruchte Fähigkeit, die scheitert, ist ein Rückschritt, eine erklärte Lücke, die scheitert, wird als Lücke gemeldet, eine erklärte Lücke, die besteht, ist ein veralteter Anspruch, und was das Backend nicht beobachtbar macht, wird als nicht beobachtet gemeldet." }
      ]
    },
'''
def conformance_catalog(t):
    if '"name": "conformance",' not in t:
        anchor='    {\n      "name": "dependencies",\n'
        if t.count(anchor)!=1:raise Drift("conformance catalog: dependencies anchor not found once")
        t=t.replace(anchor,CONF_OP+anchor,1)
    if '"name": "--keep",' not in t:
        anchor='    {\n      "name": "--scope",\n'
        if t.count(anchor)!=1:raise Drift("conformance catalog: --scope anchor not found once")
        t=t.replace(anchor,CONF_FLAG+anchor,1)
    for flag in ("--file","--json","--scenario","--implementations"):
        i=t.index('"name": "%s",'%flag);j=t.index('"operations": [',i);k=t.index("]",j)
        if '"conformance"' not in t[j:k]:t=t[:j]+t[j:k]+', "conformance"'+t[k:]
    t=t.replace("(default: both)","(default: every registered backend)").replace("(Standard: beide)","(Standard: alle registrierten Backends)")
    return t
def conformance_schema(t):return sub(t,'"concurrency", "dependencies"','"concurrency", "conformance", "dependencies"',"operations schema enum conformance")
def conformance_script(t):
    anchor='if(selected.args[0]==="concurrency"){'
    add=' if(selected.args[0]==="conformance"){const{runConformanceCommandV1}=await import("./%s/%s/%s");await runConformanceCommandV1(root,selected.args,parent);return;}\n'%(T["CONF"],T["COMMAND"],TS)
    if '"conformance"){const{runConformanceCommandV1}' in t:return t
    if t.count(anchor)!=1:raise Drift("script: concurrency dispatch not found once")
    return t.replace(anchor,add.lstrip()+" "+anchor,1) if False else t.replace(" "+anchor,add+" "+anchor,1)
def conformance_child(t):
    if 'resolved.args[0]==="conformance"' in t:return t
    anchor='  if(resolved.args[0]==="concurrency"){'
    if t.count(anchor)!=1:raise Drift("child: concurrency validation not found once")
    add='  if(resolved.args[0]==="conformance"){const {buildInvocationRequestsHelpV1}=await import("../../%s/%s");if(!buildInvocationRequestsHelpV1(resolved.args)){const {conformanceRequestV1}=await import("../../%s/%s/%s");conformanceRequestV1(resolved.args);}}\n'%(T["INVOCATION"],TS,T["CONF"],T["COMMAND"],TS)
    return t.replace(anchor,add+anchor,1)

def operations_test(t):
    t=head(t,f'import {{buildBackendIdsV1,buildBackendRowV1}} from "../../{BK}/{TS}";',"operations test import")
    t=sub(t,'test("every neutral operation has a tool state, and unimplemented states are honest"','test("every backend operation claim in the registry names a catalogue operation, and unimplemented states are honest"',"operations test title")
    t=sub(t,'expect(["nx","bazel"].every(tool=>["native","adapter","neutral","missing"].includes(row.implementations[tool as "nx"|"bazel"])),row.name).toBe(true);','expect(buildBackendIdsV1().every(tool=>["native","adapter","neutral","missing"].includes(buildBackendRowV1(tool).operations[row.name]??"neutral")),row.name).toBe(true);',"operations test states")
    t=sub(t,'expect(cache.implementations.bazel).toBe("adapter");','expect(buildBackendRowV1("bazel").operations[cache.name]).toBe("adapter");',"operations test cache")
    t=head(t,'const expandBackends=(value:string)=>value.replaceAll("{backends}",buildBackendIdsV1().join("|"));',"operations test expand")
    t=sub(t,'expect(root.includes(row.synopsis),','expect(root.includes(expandBackends(row.synopsis)),',"operations test root synopsis")
    return sub(t,'expect(own.includes(row.synopsis),row.name)','expect(own.includes(expandBackends(row.synopsis)),row.name)',"operations test own synopsis")
def invocation_test(t):
    t=head(t,f'import {{buildBackendAlternationV1}} from "../../{BK}/{TS}";',"invocation test import")
    return sub(t,'expect(stdout).toContain("repo [--tool=nx|bazel] <command>");','expect(stdout).toContain(`repo [--tool=${buildBackendAlternationV1()}] <command>`);',"invocation test help")

def incr_test(depth):
    def edit(t):
        t=head(t,f'import {{buildBackendIdsV1}} from "{"../"*depth}{BK}/{TS}";',"incr test import")
        t=t.replace('tools:["nx","bazel"],','tools:buildBackendIdsV1(),')
        t=t.replace('expect(row.implementations).toEqual({nx:"neutral",bazel:"neutral"});','expect((row as {implementations?:unknown}).implementations).toBeUndefined();')
        return t
    return edit

def operations_bootstrap(t):
    old='return operation?.requires==="none"&&operation.implementations.nx==="neutral"&&operation.implementations.bazel==="neutral";'
    new='return operation?.requires==="none"&&buildBackendIdsV1().every(id=>(buildBackendRowV1(id).operations[name]??"neutral")==="neutral");'
    if "buildOperationHasNeutralBootstrapV1" not in t:return t
    t=sub(t,old,new,"operations neutral bootstrap")
    return head(t,f'import {{buildBackendIdsV1,buildBackendRowV1}} from "../{BK}/{TS}";',"operations bootstrap import")
def birth_test(t):
    if "implementations.nx" not in t:return t
    sql_old="select coalesce((select json_extract(value,'$.requires')='none' and json_extract(value,'$.implementations.nx')='neutral' and json_extract(value,'$.implementations.bazel')='neutral' from json_each(?1,'$.operations') where json_extract(value,'$.name')=?2),0) value"
    sql_new="select coalesce((select json_extract(o.value,'$.requires')='none' and not exists (select 1 from json_each(json_extract(o.value,'$.implementations')) i where i.value!='neutral') from json_each(?1,'$.operations') o where json_extract(o.value,'$.name')=?2),0) value"
    if sql_old not in t:raise Drift("birth test: oracle query not found")
    t=t.replace(sql_old,sql_new)
    t=t.replace('.get(JSON.stringify(operations),row.operation)','.get(JSON.stringify({...operations,operations:operations.operations.map((operation:{name:string})=>({...operation,implementations:Object.fromEntries(buildBackendIdsV1().map((id:string)=>[id,buildBackendRowV1(id).operations[operation.name]??"neutral"]))}))}),row.operation)')
    return head(t,f'import {{buildBackendIdsV1,buildBackendRowV1}} from "../../../{BK}/{TS}";',"birth test import")
def p(*parts):return "/".join(parts)
SC=T["SCHEMA"]
EDITS=[
 (p(B,names.SCRIPT),None),  # replaced below
 (p(B,T["INVOCATION"],TS),invocation),
 (p(B,T["CALLER"],n(0x1F4E4,"child"),TS),child),
 (p(B,T["OPERATIONS"],TS),operations),
 (p(B,T["OPERATIONS"],TS),operations_bootstrap),
 (p(B,T["CALLER"],TS),caller),
 (p(B,T["CALLER"],T["ORCH"],MJS),orchestration),
 (p(B,T["CALLER"],T["ORCH"],n(0x1F7E8,".d.mts")),orchestration_types),
 (p(B,T["TOOLS"],T["STORAGE"],TS),storage),
 (p(B,T["REMOTECACHE"],T["COMMAND"],TS),remote_command),
 (p(B,T["REMOTECACHE"],TS),remote_index),
 (p(B,"📥️inputs","🎯️selected","📦️snapshot",TS),snapshot),
 (p(B,"📥️inputs","🎯️selected","🦀️rust",TS),rust),
 (p(B,SC,JSON),build_tool_schema),
 (p(B,SC,n(0x1F4E5,"authoring.json")),pattern_schema),
 (p(B,SC,n(0x1F4E5,"caller.json")),pattern_schema),
 (p(B,T["EXECUTION"],SC,n(0x1F310,"remote-cache.json")),remote_schema),
 (p(B,T["OPERATIONS"],SC,JSON),operations_schema),
 (p(B,T["OPERATIONS"],n(0x1F523,"catalog.json")),catalog),
 (p(B,T["OPERATIONS"],T["HELP"],TS),help_),
 (p(B,T["OPERATIONS"],n(0x1F523,"catalog.json")),conformance_catalog),
 (p(B,T["OPERATIONS"],SC,JSON),conformance_schema),
 (p(B,names.SCRIPT),conformance_script),
 (p(B,T["CALLER"],n(0x1F4E4,"child"),TS),conformance_child),
 (p(B,T["OPERATIONS"],T["TESTS"],TS),operations_test),
 (p(B,T["CALLER"],T["TESTS"],n(0x1F4E5,"birth"),TS),birth_test),
 (p(B,T["INVOCATION"],T["TESTS"],TS),invocation_test),
 (p(B,T["INCR"],T["TESTS"],TS),incr_test(2)),
 (p(B,T["INCR"],n(0x1F91D,"concurrent"),T["TESTS"],TS),incr_test(3)),
 (p(B,T["INCR"],TS),incr_model),
 (p(B,T["INCR"],SC,JSON),enum_schema),
 (p(B,T["INCR"],T["COMMAND"],TS),incr_command(2)),
 (p(B,T["INCR"],n(0x1F91D,"concurrent"),T["COMMAND"],TS),incr_command(3)),
 (p(B,T["PARITY"],TS),parity_model),
 (p(B,T["PARITY"],SC,JSON),enum_schema),
 (p(B,T["CIREPORT"],TS),ci_model),
 (p(B,T["CIREPORT"],SC,JSON),enum_schema),
]
BASES=json.load(open(os.path.join(here,"src","script.bases.json")))
def script(t):
    s=S.slices(t)
    for k,v in BASES.items():
        if S.digest(S.part(s,k))!=v:raise Drift(f"script: slice {k} changed since the adapters were cut; run gen-backends.py again")
    lines=s["lines"]
    BKI='./'+BK+'/'+TS
    tools=f'''    if(command==="tools"){{
      const {{buildBackendIdsV1,buildBackendAlternationV1,loadBuildBackendV1,requireBuildBackendIdV1}}=await import("{BKI}");
      if(resolved.args[1]!=="setup"||resolved.args.length>3)throw new Error(`Expected repo tools setup [${{buildBackendAlternationV1()}}|all]`);
      const tools=resolved.args[2]??selected.tool;
      if(tools!=="all")requireBuildBackendIdV1(tools);
      for(const id of tools==="all"?buildBackendIdsV1():[tools])await(await loadBuildBackendV1(id)).setup({{root,invocation,phase:phaseControl,args:resolved.args,tool:id}});
      return;
    }}'''
    reset=f'''    if(command==="reset"){{
      if(resolved.args.length!==1)throw new Error("Expected repo reset without task arguments");
      await(await(await import("{BKI}")).loadBuildBackendV1(selected.tool)).reset({{root,invocation,phase:phaseControl,args:resolved.args,tool:selected.tool}});
      return;
    }}'''
    run=f'''    await(await(await import("{BKI}")).loadBuildBackendV1(selected.tool)).execute({{root,invocation,phase:artifactPhase,args:resolved.args,tool:selected.tool,graph,...(resolved.watch===undefined?{{}}:{{watch:resolved.watch}})}});'''
    out=lines[:s["tools"][0]]+tools.split("\n")+reset.split("\n")+lines[s["reset"][1]:s["nx"][0]]+run.split("\n")+lines[s["bazel"][1]:]
    text="\n".join(out)
    alt=r'\[\(await import\("[^"]+"\)\)\.nxRemoteCacheToolV1\(\),\(await import\("[^"]+"\)\)\.bazelRemoteCacheToolV1\(\)\]'
    if re.search(alt,text):text=re.sub(alt,lambda m:f'await Promise.all((await import("{BKI}")).buildBackendIdsV1().map(async id=>(await(await import("{BKI}")).loadBuildBackendV1(id)).remoteCache()))',text,count=1)
    else:
        text=rsub(text,r'selected\.tool==="nx"\?\(await import\("[^"]+"\)\)\.nxRemoteCacheToolV1\(\):\(await import\("[^"]+"\)\)\.bazelRemoteCacheToolV1\(\)',f'(await(await import("{BKI}")).loadBuildBackendV1(selected.tool)).remoteCache()',"script remote tool")
    text=rsub(text,r'const resolveCacheTool=async\(tool:"nx"\|"bazel"\)=>tool==="nx"\?\(await import\("[^"]+"\)\)\.nxBuildCacheToolV1\(\):\(await import\("[^"]+"\)\)\.bazelBuildCacheToolV1\(\);',f'const resolveCacheTool=async(tool:string)=>(await(await import("{BKI}")).loadBuildBackendV1(tool)).cache();',"script cache tool")
    return text
def script_edit(t):
    if "loadBuildBackendV1(selected.tool)).execute(" in t:return t
    return script(t)
EDITS[0]=(EDITS[0][0],script_edit)
def storage_fix(t):return storage(t)

import wire2
EDITS+=wire2.EDITS
import wire3
EDITS+=wire3.EDITS
POST=wire3.POST
REQUIRES=wire3.REQUIRES
