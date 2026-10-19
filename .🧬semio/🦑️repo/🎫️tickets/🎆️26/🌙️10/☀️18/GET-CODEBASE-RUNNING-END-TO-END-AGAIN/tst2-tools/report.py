import json,re,collections
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=C+"/🗑️generated/tst2-out/"
def load(n): return {json.loads(l)["file"]:json.loads(l) for l in open(G+n+".jsonl",encoding="utf-8")}
before={}
before.update(load("pri-bun")); before.update(load("rest-bun"))
final=load("final"); tk=load("final-ticketart")
after={f:(tk[f] if f in tk and tk[f]["rc"]==0 else final[f]) for f in final}
def first(r):
    t=open(r["log"],encoding="utf-8",errors="replace").read()
    m=re.search(r"Cannot find (?:package|module) '([^']+)'",t)
    if m:
        s=m.group(1)
        if s.startswith("@semio-tech"): return "NO-DIST","`"+s+"` (package entry is dist/, not built)"
        if s.startswith(".."): return "BROKEN-IMPORT",s[-60:]
        n="/".join(s.split("/")[:2]) if s.startswith("@") else s.split("/")[0]
        return "MISSING-DEP","`"+n+"`"
    if "native result" in t or "NATIVE" in t or "cargo" in t[:20000].lower() and "refused" in t.lower(): pass
    e=re.findall(r"^error: (.{0,110})",t,re.M)
    e=e[0] if e else "?"
    if re.search(r"requires an explicit|native result|native verdict|binary gate|Executable doesn't exist|cargo metadata|cargo -Z|rustc",t[:60000]): return "NEEDS-NATIVE/BROWSER",e
    if re.search(r"required$|required\b",e) and "expect(" not in e: return "HARNESS-ENV",e
    return "SOURCE-LAW-DRIFT",e
def short(f):
    return f.replace("🧰️framework/🛍️products/💻️os/","OS/").replace("🧰️framework/🔨️modules/","M/").replace("✏️s/🔌️plugins/","PL/").replace("🧰️framework/🛍️products/","P/")
ok_b=sum(r["rc"]==0 for r in before.values()); ok_a=sum(r["rc"]==0 for r in after.values())
tp=lambda d:sum(r["pass_"] for r in d.values()); tf=lambda d:sum(r["fail"] for r in d.values())
out=[]
out.append(f"# TS suites per-suite table (tst2)\n\nbefore = first naive pass (plain `bun test`, no runner environment); after = final pass (runner environment + emulated root hoist, then ticket-local artifact dir for the failing rest).\n\nTotals: before {ok_b}/{len(before)} suites green ({tp(before)} pass / {tf(before)} fail tests); after {ok_a}/{len(after)} ({tp(after)} / {tf(after)}).\n")
cls=collections.Counter(); rows=[]
for f,r in sorted(after.items()):
    if r["rc"]!=0:
        k,e=first(r); cls[k]+=1; rows.append((k,short(f),before[f],r,e))
out.append("## Remaining failing suites by class\n\n"+"\n".join(f"- {k}: {v}" for k,v in cls.most_common())+"\n")
for k in [c for c,_ in cls.most_common()]:
    out.append(f"\n### {k}\n\n| suite | before p/f | after p/f | first error |\n|---|---|---|---|")
    for kk,s,b,a,e in rows:
        if kk==k: out.append(f"| `{s}` | {b['pass_']}/{b['fail']} | {a['pass_']}/{a['fail']} | {e.replace('|','/')[:120]} |")
fixed=[f for f in after if after[f]["rc"]==0 and before[f]["rc"]!=0]
out.append(f"\n## Green after, red before ({len(fixed)})\n\nCause of the change per class: harness environment (runner envelope `SEMIO_SCRIPT_PROCESS_INVOCATION`, short `SEMIO_TEST_ARTIFACT_DIR`, `SEMIO_TICKET_DIR`, `SEMIO_BUILD_MODE`) and the emulated root hoist for undeclared packages; source edits are listed in `result-tst2.md`.\n")
out.append("\n".join(f"- `{short(f)}` ({before[f]['pass_']}/{before[f]['fail']} -> {after[f]['pass_']}/{after[f]['fail']})" for f in sorted(fixed)))
open(C+"/result-tst2-suites.md","w",encoding="utf-8").write("\n".join(out)+"\n")
print(ok_b,ok_a,len(fixed),cls)
