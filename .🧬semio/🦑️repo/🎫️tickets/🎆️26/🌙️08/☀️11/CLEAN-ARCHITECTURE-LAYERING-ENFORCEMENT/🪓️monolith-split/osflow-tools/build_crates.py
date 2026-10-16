"""🏗️ Declares every new os-flow producer crate and (re)writes manifests and project files. usage: build_crates.py [id...]"""
import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from cg import *
CR={}
def mk(cid,node,desc,deps=(),dev=(),internal=(),lib_rel="🦀️.rs",tdeps=(),role="product"):
    c=Crate(cid,node,desc,role,lib_rel)
    for k in deps:c.deps.append(old_dep(k) if isinstance(k,str) else old_dep(*k))
    for k in tdeps:c.deps.append(old_dep(k[0],"dep",k[1]))
    for k in dev:c.devdeps.append(old_dep(k) if isinstance(k,str) else old_dep(*k))
    c.internal=[(f"semio-framework-os-flow-{i}",FL+CR[i].node+"/"+PKG) for i in internal]
    CR[cid]=c;return c

from families import LIB,TESTS,FL as _FL
from manifests import idents_used
def auto(cid,node,desc,internal=(),idev=(),extra=(),dev_extra=(),lib_rel="🦀️.rs",role="product"):
    c=Crate(cid,node,desc,role,lib_rel)
    libs=[_FL+p for p in LIB[cid] if os.path.exists(_FL+p)]
    tests=[_FL+p for p in TESTS.get(cid,[]) if os.path.exists(_FL+p)]
    ul=idents_used(libs);ut=idents_used(tests) if tests else set()
    seen=set()
    for d in DEPS:
        if d["where"]!="dep":continue
        k=(d["key"],d["cfg"])
        if (d["ident"] in ul or d["key"] in extra) and k not in seen:
            seen.add(k);c.deps.append(d)
    depkeys={d["key"] for d in c.deps if d["cfg"] is None}
    for d in DEPS:
        if d["where"]=="dev":
            if d["key"] in depkeys or d["ident"] in ut or d["key"] in dev_extra:c.devdeps.append(d)
    # test-only normal deps become dev deps
    if tests:
        for d in DEPS:
            if d["where"]=="dep" and d["cfg"] is None and d["ident"] in ut and d["ident"] not in ul and not any(x["key"]==d["key"] for x in c.devdeps):c.devdeps.append(d)
    c.internal=[(f"semio-framework-os-flow-{i}",_FL+CR[i].node+"/"+PKG) for i in internal]
    c.idev=[(f"semio-framework-os-flow-{i}",_FL+CR[i].node+"/"+PKG) for i in idev]
    CR[cid]=c;return c

mk("geometry","🌐️geometry","Flow geometry port: supplied geometry work, retained receipts and port retirement",
   deps=["semio-framework-value","semio-framework","semio-framework-job"],dev=["serde_json",("semio-framework-job","dev")])
mk("mesh","🎒️mesh","Flow preview mesh record codec and bounded transport envelope",
   deps=["semio-framework-dsl-record","semio-framework-pack-json","semio-framework-value","semio-framework","neural_engine","base64","semio-framework-os-kernel"])
mk("drawing","🖍️drawing","Flow drawing kernel: path segments, scene store, boolean, export and trace",
   deps=["semio-framework-pack","semio-framework-pack-json","semio-framework-value","semio-framework-value-derive","neural_engine","semio-framework-2d","serde","serde_json"])
mk("operators","🗃️operators","Flow operator catalogue support: extension manifest schema, operator registry, catalogue projections and channel-eval bridge",
   deps=["semio-framework-pack-json","semio-framework-value","semio-framework-artifact-flow-flow","semio-framework-value-derive","semio-framework","semio-framework-replication","neural_engine","ui_wgpu","serde","serde_json"],
   tdeps=[("ui_wgpu","cfg(not(all(target_arch = \"wasm32\", target_env = \"p2\")))")],
   dev=[("semio-framework-artifact-flow-flow","dev"),("semio-framework-replication","dev")])
mk("vcs","🌿️vcs","Flow document VCS: retained operations, DSL, store, canonical forms and the forms bridge",
   deps=["semio-framework-pack-json","semio-framework-value","semio-framework-artifact-flow-flow","semio-framework-replication","neural_engine","semio-framework-os-kernel"],
   dev=["semio-framework-pack","serde_json",("semio-framework-trace","dev"),("semio-framework-async-macros","dev"),("semio-framework-os-kernel","dev"),("semio-framework-artifact-flow-flow","dev"),("semio-framework-replication","dev")])
auto("vcs-laws","🌿️vcs/⚖️laws","Flow VCS laws: the retained VCS operation, canonical history and lifecycle suite over the whole VCS producer",internal=["vcs"])
mk("extensions","🧩️extensions/🕸️wasm","Flow extension evaluation: request admission, retained evaluation jobs, completions and failure projections",
   deps=["semio-framework-pack","semio-framework-pack-json","semio-framework-value","semio-framework-artifact-flow-flow","semio-framework","semio-framework-job","semio-framework-plugin","semio-framework-replication","neural_engine","serde_json"],
   internal=["operators"],
   dev=["semio-framework-dsl-record","serde",("semio-framework-job","dev"),("semio-framework-trace","dev"),("semio-framework-artifact-flow-flow","dev"),("semio-framework-replication","dev")])
auto("extensions-laws","🧩️extensions/🕸️wasm/⚖️laws","Flow extension laws: the SDK request, raw recipient and invocation wire suite over the whole extension evaluation producer",internal=["extensions"])
auto("host-source","🖥️host/📥️evaluation-source","Flow host evaluation source: funded execution projection, builtin seeds and the evaluation work continuation")
auto("host-core","🖥️host/🧱️core","Flow host core: the retained FlowHost, graph editing, evaluation steps, history and close ladder with host errors",internal=["host-source","operators","extensions","geometry","vcs"])
auto("host-read","🖥️host/📥️evaluation-source/🔎️read","Flow host source read: funded current-output admission against the original source and baseline owners",internal=["host-core","host-source","operators"])
auto("host-session","🖥️host/🎞️session","Flow host session: evaluation session, tick and preview turns, publication custody and intake receivers",internal=["host-core","host-source","host-read","operators","geometry"])
auto("host-laws","🖥️host/⚖️laws","Flow host laws: retirement, work continuation and source read suites over a retained host with its session",internal=["host-core","host-read","host-session","host-source","operators","drawing"])
auto("host","🖥️host","Flow host: canvas editing, evaluation session and host errors composed from the core, read and session producers",internal=["host-core","host-read","host-session","operators","drawing","vcs"])
auto("wasm-domain","🕸️wasm/🧩️domain-adapter","Flow wasm session domain: adapter state, argument preflight, program state, VCS feature, surface presentation and protocol contract",internal=["host-core","vcs","geometry"])
auto("wasm-actions-a","🕸️wasm/🎬️actions-a","Flow wasm session actions 2504 to 2560 with the retained dag input codecs",internal=["wasm-domain","host-core"])
auto("wasm-actions-b","🕸️wasm/🎞️actions-b","Flow wasm session actions 2561 to 2610",internal=["wasm-domain","host-core","geometry","drawing"])
auto("wasm","🕸️wasm","Flow wasm session: bridge runtime, domain trait implementation, operation dispatch and linear-memory exports",internal=["wasm-domain","wasm-actions-a","wasm-actions-b","host-core","vcs","drawing"],idev=["host"])
if __name__=="__main__":
    ids=sys.argv[1:] or list(CR)
    for i in ids:
        write_crate(CR[i]);print("wrote",CR[i].name)
