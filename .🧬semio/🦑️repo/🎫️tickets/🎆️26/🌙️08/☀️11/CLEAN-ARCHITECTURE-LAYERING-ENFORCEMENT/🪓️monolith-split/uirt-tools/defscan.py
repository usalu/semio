"""🔎️ Definition scan: non-pub definitions of one crate whose name is referenced from a file of another crate."""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from union import CRATES,closure
from oracle import REPO,RT
from widen import strip_comments
base=f"{REPO}/{RT}"
owner={}
for crate,rel in CRATES.items():
    for f in closure(f"{base}/{rel}",base): owner.setdefault(f,crate)
files=collections.defaultdict(list)
for f,c in owner.items():
    if os.path.exists(f) and f.endswith(".rs"): files[c].append(f)
def defs(path):
    out=[]
    lines=open(path,encoding="utf-8").read().split("\n")
    block=None
    for i,l in enumerate(lines):
        if l and not l[0].isspace():
            if l.startswith("}"): block=None
            m=re.match(r"^(pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(fn|struct|enum|const|static|type|trait)\s+([A-Za-z_]\w*)",l)
            if m:
                vis=(m.group(1) or "").strip()
                if vis!="pub": out.append((i+1,m.group(3),"item",vis or "private",l.strip()[:90]))
                block=("struct",m.group(3)) if m.group(2)=="struct" else None
                continue
            m=re.match(r"^impl\b",l)
            block=("impl",) if m and " for " not in l.split("{")[0] else None
        elif block and l.startswith("    ") and not l.startswith("     "):
            if block[0]=="struct":
                m=re.match(r"^    (pub(?:\([^)]*\))?\s+)?([a-z_]\w*)\s*:",l)
                if m and (m.group(1) or "").strip()!="pub": out.append((i+1,m.group(2),"field",(m.group(1) or "private").strip(),block[1]+"."+m.group(2)))
            else:
                m=re.match(r"^    (pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:unsafe\s+)?(?:async\s+)?fn\s+([a-z_]\w*)",l)
                if m and (m.group(1) or "").strip()!="pub": out.append((i+1,m.group(2),"method",(m.group(1) or "private").strip(),l.strip()[:90]))
    return out
texts={c:"\n".join(strip_comments(open(f,encoding="utf-8").read()) for f in fs) for c,fs in files.items()}
hits=0
ORDER=["semio_framework_ui_runtime_exchange","semio_framework_ui_runtime_presentation","semio_framework_ui_runtime_reconcile_primitives","semio_framework_ui_runtime_reconcile_cursor","semio_framework_ui_runtime_reconcile","semio_framework_ui_runtime"]
for c,fs in files.items():
    for f in fs:
        if "🧪️tests" in f or "/🧫️" in f or f.endswith("/🔄️transaction/🦀️.rs") or "🧰️testkit" in f: continue
        for line,name,kind,vis,desc in defs(f):
            for d,txt in texts.items():
                if ORDER.index(d)<=ORDER.index(c): continue
                if kind=="item": pat=r"(?<![A-Za-z0-9_])"+name+r"(?![A-Za-z0-9_])"
                elif kind=="field": pat=r"\."+name+r"(?![A-Za-z0-9_(])|[{,]\s*"+name+r"\s*[:,}]"
                else: pat=r"(?:\.|::)"+name+r"\s*\("
                if re.search(pat,txt):
                    hits+=1
                    print(f"{c.replace('semio_framework_ui_runtime','')or 'facade':24} {kind:6} {vis:10} {f.replace(base+'/','')}:{line} {desc}  <- referenced in {d.replace('semio_framework_ui_runtime','') or 'facade'}")
print("hits",hits)
