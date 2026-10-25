"""Writes C/table-cache-d.md: file -> suite -> latest verdict -> cause. usage: ledger.py <summary.txt>... (later summaries override earlier ones); notes come from cache-d-tools/notes.json {path-substring: "cause / fix"}"""
import json,os,re,sys
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/"
root="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
table=json.load(open(root+"🗂️suites/🔣️.json",encoding="utf8"))
import subprocess
files=[l for l in subprocess.run(["bun",C+"cache-d-tools/discover.ts"],capture_output=True,text=True).stdout.split("\n") if l]
def owner(f):
    explicit=[s["id"] for s in table["suites"] if f in s.get("files",[])]
    if explicit:return explicit[0]
    best=(-1,"?")
    for s in table["suites"]:
        for t in s.get("trees",[]):
            if f.startswith(t+"/") and len(t)>best[0]:best=(len(t),s["id"])
    return best[1]
verdict={}
for path in sys.argv[1:]:
    for l in open(path,encoding="utf8"):
        c=l.rstrip("\n").split("\t")
        if len(c)>3 and c[0]!="DONE":
            sig=""
            if c[1]!="rc=0" and c[4].startswith("log=f"):
                try:
                    text=open(os.path.join(os.path.dirname(path),c[4][4:]),encoding="utf8",errors="replace").read()
                    errs=[m.group(1).strip() for m in re.finditer(r"^(?:error: |[A-Za-z]*Error: )(.*)$",text,re.M) if not re.match(r"^\d+ \|",m.group(1)) and not m.group(1).startswith("Isolated build test")]
                    sig=(errs[0] if errs else "")[:110].replace("|","/")
                except OSError:pass
            verdict[c[0]]=(c[1].replace("rc=","rc "),c[2],path.split("/")[-2] if path.endswith("summary.txt") else path,sig)
notes=json.load(open(C+"cache-d-tools/notes.json",encoding="utf8")) if os.path.exists(C+"cache-d-tools/notes.json") else {}
CATEGORIES=[
 (r"SDK|tooling identity|Nx namespace|immutable Nx|Nx tooling","CATEGORY env-official: needs SEMIO_NX_TOOLING_IDENTITY / Nx SDK capabilities of the official target environment (a probe with the installed identity turned `📦️artifacts/🔒️bounded` 7/7 green); verify through the official route"),
 (r"Nested persistent image","CATEGORY nested-image law: the test opens a native image below an open image (diagnostics/products); needs scratch outside images or the parent child-API"),
 (r"ticket|Closed authorizer relative|escapes workspace","CATEGORY checkout-resident scratch: the test needs sources/fixtures below the workspace; route `scratch: checkout`"),
 (r"setup-all|Original authority|purpose","CATEGORY original-authority gate: needs an Original born as `tools setup all` or an issued purpose; no test route supplies it"),
 (r"admitted Source resolution|semioCaptured","CATEGORY admitted-program: runs only inside a compiled authorizer bundle"),
 (r"is not defined|ReferenceError","CATEGORY extracted-function port drift: production function gained free names; test ports/extraction list stale"),
 (r"Original process admission closed|first failure","CATEGORY poisoned Original: an earlier failing/expected-refusal test closed the shared Original; isolate or fix the first failure"),
 (r"toContain|toEqual|toBe|Expected","CATEGORY roster/assertion drift: stale expectation vs refactored production"),
]
def category(sig):
    for pattern,text in CATEGORIES:
        if re.search(pattern,sig):return text
    return ""
def note(f):
    best=""
    for k,v in notes.items():
        if k in f and len(k)>=len(best):best=k
    return notes.get(best,"") if best else ""
out=["# table-cache-d: caching test files -> suite -> verdict -> cause","","Verdict = latest census row (`k*` = authored-envelope census, `official` = official route). `rc 0` green.","","| file | suite | verdict | counts | cause / fix |","|---|---|---|---|---|"]
tally={}
for f in files:
    v=verdict.get(f,("not run","","","")) 
    tally[v[0]]=tally.get(v[0],0)+1
    out.append(f"| `{f}` | {owner(f)} | {v[0]} ({v[2]}) | {v[1]} | {note(f) or (('first error: '+v[3]+' ; '+category(v[3])) if v[3] else '')} |")
out.insert(2,"Totals: "+", ".join(f"{k}: {n}" for k,n in sorted(tally.items())))
open(C+"table-cache-d.md","w",encoding="utf8").write("\n".join(out)+"\n")
print(tally)
