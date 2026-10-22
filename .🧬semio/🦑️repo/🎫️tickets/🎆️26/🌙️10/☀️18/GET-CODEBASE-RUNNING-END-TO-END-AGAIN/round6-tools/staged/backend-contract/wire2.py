#!/usr/bin/env python3
"""Round 2 anchored edits (registry-declared default backend, generic harnesses, relocations). Same contract as wire.py: pure text->text functions, idempotent, Drift on a missing anchor. Loaded by wire.py, which appends EDITS."""
import json,os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
from names import B,T,TS,JSON,MJS,n
class Drift(Exception):pass
BK=T["BACKENDS"];SC=T["SCHEMA"]
def sub(text,old,new,label):
    if new in text and old not in text:return text
    c=text.count(old)
    if c!=1:raise Drift(f"{label}: anchor found {c} times: {old[:100]!r}")
    return text.replace(old,new)
def head(text,line):
    return text if line in text else line+"\n"+text
def p(*parts):return "/".join(parts)

# ---- the default backend is declared by the registry
def default_invocation(t):
    if "policy.defaultTool" not in t:return t
    t=head(t,f'import {{buildBackendDefaultIdV1}} from "../{BK}/{TS}";')
    return t.replace("policy.defaultTool","buildBackendDefaultIdV1()")
def default_policy(t):
    if '"defaultTool"' not in t:return t
    return re.sub(r'\n[ \t]*"defaultTool": *"[a-z-]+",?',"",t,count=1)
BUILD_POLICY_NEW='''"BuildPolicy": {
      "type": "object",
      "required": [
        "version"
      ],
      "properties": {
        "version": {
          "const": 1
        }
      },
      "additionalProperties": true
    },'''
def default_schema(t):
    if '"defaultTool"' not in t:return t
    old=re.search(r'"BuildPolicy": \{.*?\n    \},',t,flags=re.S)
    if not old:raise Drift("build schema: BuildPolicy not found")
    return t[:old.start()]+BUILD_POLICY_NEW+t[old.end():]
def default_portable_test(t):
    if "buildPolicy.defaultTool" not in t:return t
    t=head(t,f'import {{buildBackendDefaultIdV1}} from "../../../{BK}/{TS}";')
    return t.replace("buildPolicy.defaultTool","buildBackendDefaultIdV1()")
RUST_OLD_TOOL='''pub fn is_build_tool(value: &str) -> bool {
    build_policy_contract()["$defs"]["BuildTool"]["enum"].as_array().is_some_and(|tools| tools.iter().any(|tool| tool.as_str() == Some(value)))
}'''
RUST_NEW_TOOL='''pub fn is_build_tool(value: &str) -> bool {
    build_backend_registry()["backends"].as_array().is_some_and(|rows| rows.iter().any(|row| row["id"].as_str() == Some(value)))
}

/// 🔌️ Reads the authored registry of build backends, the only place that lists them.
fn build_backend_registry() -> &'static serde_json::Value {
    static REGISTRY: OnceLock<serde_json::Value> = OnceLock::new();
    REGISTRY.get_or_init(|| serde_json::from_str(include_str!("../../%s/%s/%s")).expect("neutral build backend registry"))
}'''%(BK,T["REGISTRY"],JSON)
RUST_OLD_DEFAULT='''    value["defaultTool"].as_str().filter(|tool| is_build_tool(tool)).map(str::to_string).ok_or_else(|| std::io::Error::other("workspace neutral default tool required"))'''
RUST_NEW_DEFAULT='''    build_backend_registry()["backends"].as_array().and_then(|rows| rows.iter().find(|row| row["default"].as_bool() == Some(true))).and_then(|row| row["id"].as_str()).filter(|tool| is_build_tool(tool)).map(str::to_string).ok_or_else(|| std::io::Error::other("neutral default build backend required"))'''
def default_rust(t):
    if "build_backend_registry" in t:return t
    t=sub(t,RUST_OLD_TOOL,RUST_NEW_TOOL,"rust is_build_tool")
    return sub(t,RUST_OLD_DEFAULT,RUST_NEW_DEFAULT,"rust default")
PROBE_NEW='''  private async probeTool(): Promise<CacheProbeToolV1> {
    const { buildBackendDefaultIdV1, loadBuildBackendV1 } = await import("../../../%s/%s/%s"), selected = process.env.SEMIO_CACHE_PROBE_TOOL ?? process.env.SEMIO_BUILD_TOOL ?? buildBackendDefaultIdV1();
    if (validateJsonSchemaSubset(probeSchema, probeSpecification).length) throw new Error("Closed cache probe specification required");
    const backend = await loadBuildBackendV1(selected);
    if (!backend.cacheProbe) throw new Error(`No cache probe implements build tool ${selected}`);
    return backend.cacheProbe({ root: this.repoRoot, invocation: this.invocation });
  }

'''%(names.BUILD,BK,TS)
def probe(t):
    if "backend.cacheProbe" in t:return t
    a=t.find("  private async probeTool(): Promise<CacheProbeToolV1> {");b=t.find("  private async verify(output: string)")
    if a<0 or b<0 or 'if (selected === "nx") {' not in t[a:b]:raise Drift("cache verify: probeTool not found")
    return t[:a]+PROBE_NEW+t[b:]

# ---- the cache ledger declares, sizes and collects the engine's area
def _object_end(text,start):
    depth=0;inside=False;escaped=False
    for i in range(start,len(text)):
        c=text[i]
        if inside:
            if escaped:escaped=False
            elif c=="\\":escaped=True
            elif c=='"':inside=False
            continue
        if c=='"':inside=True
        elif c=="{":depth+=1
        elif c=="}":
            depth-=1
            if depth==0:return i+1
    raise Drift("unbalanced object")
SEMIO_AREA='''{
      "id": "semio-actions",
      "root": "repository",
      "kind": "nested",
      "directory": "semio/actions",
      "partition": "products",
      "budget": { "basisPoints": 2000 },
      "managed": { "owner": "semio" },
      "writers": ["%s/%s/%s/%s/%s"],
      "purpose": { "en": "Action cache of the in-house build engine: outputs by digest and entries by action key.", "de": "Aktions-Cache der hauseigenen Build-Engine: Ausgaben nach Digest und Einträge nach Aktionsschlüssel." }
    }'''%(names.B,T["TOOLS"],T["SEMIO"],T["STORE"],TS)
def ledger_area(t):
    if '"semio-actions"' in t:return t
    i=t.find('"id": "bazel-actions"')
    if i<0:raise Drift("ledger: bazel-actions area not found")
    start=t.rfind("{",0,i);end=_object_end(t,start)
    return t[:end]+",\n    "+SEMIO_AREA+t[end:]
def ledger_schema(t):
    m=re.search(r'"owner": \{ "enum": \[([^\]]*)\] \}',t)
    if not m or '"bazel"' not in m.group(1):raise Drift("ledger owner enum: not found")
    if '"semio"' in m.group(1):return t
    return t[:m.start(1)]+m.group(1).replace('"bazel", ','"bazel", "semio", ',1)+t[m.end(1):]
GC_OLD='return Promise.resolve(undefined);'
GC_NEW='return import("../../%s/%s").then(async registry=>registry.buildBackendIdsV1().includes(owner)?(await registry.loadBuildBackendV1(owner)).collectCache?.({root,invocation,phase:phaseControl,dryRun,signal:phaseControl.signal}):undefined);'%(BK,TS)
def gc_delegate(t):
    return t if ".collectCache?." in t else sub(t,GC_OLD,GC_NEW,"cache gc delegate")

EDITS=[
 (p(B,T["INVOCATION"],TS),default_invocation),
 (p(B,"🔣️policy.json"),default_policy),
 (p(B,SC,JSON),default_schema),
 (p(B,"🧪️tests","🕸️graph","🧭️portable-cli",TS),default_portable_test),
 (p(B,"🕸️graph","🦀️rust","🦀️.rs"),default_rust),
 (p(names.L,"⚡️caching","🔁️verification","📋️orchestration",TS),probe),
 (p(names.L,"⚡️caching","💾️storage","🧾️ledger","🔣️policy.json"),ledger_area),
 (p(names.L,"⚡️caching","💾️storage","🧾️ledger",SC,JSON),ledger_schema),
 (p(B,"🧹️cache-gc",T["COMMAND"],TS),gc_delegate),
]
