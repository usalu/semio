"""🔬 Isolated per-crate-root analysis: the unresolved external crates must equal the manifest dependencies; no in-crate resolution error."""
import re,sys,os,collections,json
sys.path.insert(0,'.')
import oracle
CLI=oracle.CLI
ROOTS={
 "semio-framework-repo-cli-event-stream":"📡️event-stream/🦀️.rs",
 "semio-framework-repo-cli-command-framework":"🧭️repo-cli/🎮️command-framework/🦀️.rs",
 "semio-framework-repo-cli-tree-source-and-search":"🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs",
 "semio-framework-repo-cli-mcp-verb":"🔌️mcp-verb/🦀️.rs",
 "semio-framework-repo-cli-verbs":"🧭️repo-cli/🚦️verbs/🦀️.rs",
 "semio-framework-repo-cli":"🦀️.rs",
}
def deps_of(crate):
    node=ROOTS[crate].rsplit("/",1)[0] if crate!="semio-framework-repo-cli" else ""
    toml=open(CLI+(node+"/" if node else "")+"📦️packages/🦀️rust/Cargo.toml",encoding="utf-8").read()
    sec=toml.split("[dependencies]")[1].split("[dev-dependencies]")[0]
    return {m.group(1).replace("-","_") for m in re.finditer(r'^([a-z0-9_-]+) = ',sec,re.M)}
ok=True
for cfg,extra in (("native",[]),("test",["--test"])):
    for crate,rel in ROOTS.items():
        rows,err=oracle.run(CLI+rel,extra,crate.replace("-","_"))
        unresolved=set()
        other=collections.Counter()
        for r in rows:
            if r["level"]!="error": continue
            m=re.search(r"cannot find module or crate `([A-Za-z0-9_]+)`|unresolved import `([A-Za-z0-9_]+)`|use of unresolved module or unlinked crate `([A-Za-z0-9_]+)`",r["msg"]+" "+r["label"])
            if m: unresolved.add(next(g for g in m.groups() if g)); continue
            if re.match(r"cannot find attribute `serde`|cannot find derive macro|aborting due",r["msg"]): 
                if "serde" in r["msg"]: unresolved.add("serde")
                continue
            other[(r["code"],r["msg"][:80])]+=1
        decl=deps_of(crate)
        used=unresolved-{"crate","self","super"}
        missing=used-decl; unused=decl-used
        print(f"[{cfg}] {crate}: unresolved={len(used)} declared={len(decl)} undeclared-use={sorted(missing)} unused-decl={sorted(unused)} other-errors={sum(other.values())}")
        for k,v in other.items(): print("    other",v,k)
        ok&= not missing and not other
print("OK" if ok else "CHECK")
