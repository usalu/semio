#!/usr/bin/env python3
"""Round 2, part 3: leaks that move behind registry rows (cache proof, nested native launches), the Nx child envelope relocation, and renamed storage kinds. Same contract as wire.py. Loaded by wire2.py, which appends EDITS."""
import os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
from names import B,T,TS,JSON,MJS,n
from wire2 import Drift,sub,head,p,BK
SC=T["SCHEMA"]
def rx(pattern,new,label,flags=re.S):
    def edit(t):
        if re.search(pattern,t,flags)is None:
            return t
        return re.sub(pattern,lambda m:new,t,count=0,flags=flags)
    return edit
def chain(*fns):
    def edit(t):
        for fn in fns:t=fn(t)
        return t
    return edit
def need(t,old,label):
    if old not in t:raise Drift(f"{label}: anchor not found: {old[:90]!r}")

# ---- the cache proof is declared by the registry row
def ci_proof(t):
    if "cacheProof" in t:return t
    t=head(t,f'import {{buildBackendRowV1}} from "../{BK}/{TS}";')
    a=t.find("const nxMarkers=");b=t.find("/** ✅️ Judges a cache proof")
    if a<0 or b<0:raise Drift("ci proof: marker constants not found")
    t=t[:a]+t[b:]
    m=re.search(r' if\(input\.tool==="nx"\)\{.*?rule="spawn-marker";\n',t,re.S)
    if not m:raise Drift("ci proof: rule chain not found")
    new=''' const declared=buildBackendRowV1(input.tool).cacheProof,marker=declared?.logMarker?new RegExp(declared.logMarker,"u"):undefined;
 if(marker?.test(log2))rule="log-marker";
 else if(declared?.activity===true&&s0.activity!==undefined&&s1.activity!==undefined&&s2.activity!==undefined&&s1.activity.executed-s0.activity.executed>0&&s2.activity.executed===s1.activity.executed&&s2.activity.builds>s1.activity.builds)rule="activity";
 else if(declared?.spawnMarker){const executed=new RegExp(declared.spawnMarker.executed,"u"),completed=new RegExp(declared.spawnMarker.completed,"u");if(executed.test(log1)&&!executed.test(log2)&&completed.test(log2))rule="spawn-marker";}
'''
    return t[:m.start()]+new+t[m.end():]
def ci_script(t):
    if "requireBuildBackendIdV1" in t:return t
    t=head(t,f'import {{buildBackendAlternationV1,requireBuildBackendIdV1}} from "../{BK}/{TS}";')
    old='const tool=(values:Map<string,string>):CiToolV1=>{const value=need(values,"tool");if(value!=="nx"&&value!=="bazel")throw Error("Option --tool must be nx or bazel");return value;};'
    t=sub(t,old,'const tool=(values:Map<string,string>):CiToolV1=>requireBuildBackendIdV1(need(values,"tool"));',"ci script tool")
    if t.count("--tool nx|bazel")!=2:raise Drift("ci script: usage text")
    return t.replace("--tool nx|bazel","--tool ${buildBackendAlternationV1()}")

# ---- the store kind of the self-hosted blob API is a blob
def blob_store(t):
    t=t.replace('"ac"|"cas"|"nx"','"ac"|"cas"|"blob"')
    return re.sub(r'(cas:/\^\[0-9a-f\]\{64\}\$/u,)nx:',r'\1blob:',t)
def blob_server(t):
    t=t.replace('kind:"nx"','kind:"blob"').replace('address.kind==="nx"','address.kind==="blob"')
    return t.replace(',nx:{hits:0',',blob:{hits:0')
def blob_tests(t):
    t=t.replace('kind:"nx",key','kind:"blob",key').replace('"default","nx",','"default","blob",')
    return t.replace('counters.nx!','counters.blob!')

# ---- component targets: registered root commands that are not targets
def component_targets(t):
    if "nativeNamespaces" in t:return t
    old='if (!["nx", "repo"].includes(name))'
    need(t,old,"component targets")
    i=t.index("export function rootCommandTargets");j=t.rfind("/**",0,i)
    decl='const nativeNamespaces = JSON.parse(readFileSync(new URL("../../../%s/%s/%s", import.meta.url), "utf8")).backends.flatMap((row) => (row.nativeCaller ? [row.nativeCaller.namespace] : []));\n\n'%(T["BACKENDS"],T["REGISTRY"],JSON)
    t=t[:j]+decl+t[j:]
    return t.replace(old,'if (!["repo", ...nativeNamespaces].includes(name))')

# ---- invocation namespace
def invocation_namespace(t):
    if "buildBackendNativeNamespacesV1" in t:return t
    t=head(t,f'import {{buildBackendNativeNamespacesV1}} from "../{BK}/{TS}";')
    return sub(t,'!["repo","nx"].includes(namespace!)','!["repo",...buildBackendNativeNamespacesV1()].includes(namespace!)',"invocation namespace")

# ---- orchestration caller authority reads the registry
def orchestration_registry(t):
    if "nativeSchemas" in t:return t
    old_tail=',new URL("../../../%s/%s/%s/%s/%s/%s",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL))],authorityCaptures='%(names.n(0x26A1,"caching"),names.n(0x1F680,"bootstrap"),names.n(0x1F50C,"nx-plugin"),names.n(0x1F4E4,"arguments"),names.n(0x1F9EC,"schema"),names.f(0x1F4E5,"native-caller.json"))
    t=sub(t,old_tail,',registryPath,...nativeCallers.map(row=>row.url)],authorityCaptures=',"orchestration authority paths")
    reg='new URL("../../%s/%s/%s",origin)'%(T["BACKENDS"],T["REGISTRY"],JSON)
    prefix='const origin=physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL),registryPath=%s,nativeCallers=JSON.parse(captureBoundedSourceRevisionV1(fileURLToPath(registryPath),65536,true).text).backends.filter(row=>row.nativeCaller).map(row=>({id:row.id,namespace:row.nativeCaller.namespace,url:new URL(row.nativeCaller.schema,new URL("../../%s/%s",origin))}));\n'%(reg,T["BACKENDS"],TS)
    t=sub(t,"const authorityPaths=",prefix+"const authorityPaths=","orchestration prefix")
    t=sub(t,"nxSchema=JSON.parse(authorityCaptures[3].text),","nativeSchemas=new Map(nativeCallers.map((row,index)=>[row.id,{namespace:row.namespace,schema:JSON.parse(authorityCaptures[4+index].text)}])),","orchestration schemas")
    return sub(t,'tool==="nx"&&conforms(nxSchema,caller)&&caller.arguments[0]==="nx"','nativeSchemas.has(tool)&&conforms(nativeSchemas.get(tool).schema,caller)&&caller.arguments[0]===nativeSchemas.get(tool).namespace',"orchestration predicate")

# ---- the Nx child envelope moves into the Nx module
NX_ENVELOPE=names.B+"/"+T["TOOLS"]+"/"+T["NX"]+"/"+T["ENVELOPE"]+"/"+TS
def caller_without_nx_envelope(t):
    a=t.find("/** 📨️ Serializes only predeclared Nx child data")
    if a<0:return t
    b=t.index("/** 🗺️ Plans one selected request")
    t=t[:a]+t[b:]
    for line in (f'import nxCallerSchema from"../../{names.n(0x26A1,"caching")}/{names.n(0x1F680,"bootstrap")}/{names.n(0x1F50C,"nx-plugin")}/{names.n(0x1F4E4,"arguments")}/{names.n(0x1F9EC,"schema")}/{names.f(0x1F4E5,"native-caller.json")}" with {{ type: "json" }};\n',
                 f'import {{isStoreRepairSelectionV1}} from "../{names.n(0x1F9FE,"store-receipt")}/{names.f(0x1F7E8,".mjs")}";\n'):
        need(t,line,"caller import")
        t=t.replace(line,"")
    return sub(t,"scriptProcessEnvironment, readScriptProcessEnvelope, type ScriptProcessEnvelope, type ScriptInvocation","scriptProcessEnvironment, type ScriptInvocation","caller process imports")
def envelope_importer(old_path,new_path):
    def edit(t):
        if new_path in t and old_path not in t:return t
        pat=re.compile(r'(\{projectOwnedNxProcessEnvelopeV1\}\s*(?:=\s*await\s+)?(?:import\(|from\s*)\s*)"'+re.escape(old_path)+'"')
        if not pat.search(t):raise Drift(f"envelope importer: {old_path}")
        return pat.sub(lambda m:m.group(1)+'"'+new_path+'"',t)
    return edit
def child_namespace(t):
    t=t.replace("buildBackendBootstrapArgumentsV1","buildBackendBootstrapV1")
    if "main.namespace" in t:return t
    t=sub(t,'resolveNxBootstrapLaunchesV1(root,["nx",...main],','resolveNxBootstrapLaunchesV1(root,[main.namespace,...main.arguments],',"child launch resolve")
    return sub(t,'add(["nx",...launch])','add([main.namespace,...launch])',"child launch add")

# ---- native recovery names its reset target from the schema mapping
def cli_recovery(t):
    t=t.replace('Readonly<{kind:"cache-reset";target:"nx";owner:string}>','Readonly<{kind:"cache-reset";target:string;owner:string}>')
    return t.replace('Object.freeze({kind:"cache-reset",target:"nx",owner:','Object.freeze({kind:"cache-reset",target:protocol.reset.targets[target[1] as keyof typeof protocol.reset.targets],owner:')

CB="⚡️caching/🚀️bootstrap"
EDITS=[
 (p(B,T["CIREPORT"],TS),ci_proof),
 (p(B,T["CIREPORT"],"📜️script.ts"),ci_script),
 (p(B,"🕸️graph","📥️inference","🧩️component-targets",MJS),component_targets),
 (p(B,T["INVOCATION"],TS),invocation_namespace),
 (p(B,T["CALLER"],T["ORCH"],MJS),orchestration_registry),
 (p(B,T["CALLER"],TS),caller_without_nx_envelope),
 (p(B,T["CALLER"],"📤️child",TS),child_namespace),
 (p(B,"🚀️bootstrap","⌨️cli",TS),cli_recovery),
 (p(names.L,CB,"📜️script.ts"),envelope_importer("../../🏗️build/📥️caller/🟦️.ts","../../🏗️build/"+T["TOOLS"]+"/"+T["NX"]+"/"+T["ENVELOPE"]+"/"+TS)),
 (p(B,T["CALLER"],T["TESTS"],"📥️birth","📜️script.ts"),envelope_importer("../../🟦️.ts","../../../"+T["TOOLS"]+"/"+T["NX"]+"/"+T["ENVELOPE"]+"/"+TS)),
 (p(B,"🕸️graph","🦀️rust","📥️intent",T["TESTS"],TS),envelope_importer("../../../../📥️caller/🟦️.ts","../../../../"+T["TOOLS"]+"/"+T["NX"]+"/"+T["ENVELOPE"]+"/"+TS)),
]

POST=[
 (p(B,"🧹️cache-gc",T["COMMAND"],TS),"loadBuildBackendV1(owner)",True),
 (p(B,T["CALLER"],"📤️child",TS),"buildBackendBootstrapV1",True),
 (p(B,T["CALLER"],"📤️child",TS),"main.namespace",True),
 (p(B,T["CALLER"],TS),"projectOwnedNxProcessEnvelopeV1",False),
 (p(B,T["CIREPORT"],TS),"declared?.logMarker",True),
 (p(B,T["INVOCATION"],TS),"buildBackendNativeNamespacesV1()",True),
 (p(B,T["CALLER"],T["ORCH"],MJS),"nativeSchemas.get(tool).namespace",True),
 (p(names.L,CB,"📜️script.ts"),T["ENVELOPE"],True),
]
REQUIRES=[(names.FRAMEWORK+"/"+names.n(0x1F528,"modules")+"/"+names.n(0x2699,"runtime")+"/"+names.n(0x1F310,"glob"),"the runtime-seam set (in-house glob and runtime identity) must be applied first"),
 (names.FRAMEWORK+"/"+names.n(0x1F528,"modules")+"/"+names.n(0x1F3C3,"process")+"/"+names.n(0x1F9F0,"tools")+"/"+names.n(0x1F33F,"git"),"the tool-interface git adapter (tool-iface set) must be applied first")]
