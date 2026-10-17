"""🔭️ Read-only scan of every path that starts at the facade crate `semio_framework`: first segment histogram, group members, bare imports of an alias. Output: JSON in 🗑️generated/rsl-scan.json."""
import json, os, re, subprocess, sys
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
ALIASES=["ui","abi","action_bus","platform","interaction","onboarding","history_edit","tool_run","io_schema","sqlite_snapshot","mesh_io","source_projection"]
def ls(*a):
    return [p for p in subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-z",*a],capture_output=True,text=True).stdout.split("\0") if p]
files=sorted(set(ls("--","*.rs"))|set(ls("--others","--exclude-standard","--","*.rs")))
files=[f for f in files if not f.startswith(".🧬semio/") and os.path.isfile(f)]
HEAD=re.compile(r'(?<![A-Za-z0-9_])semio_framework(?![A-Za-z0-9_])')
IDENT=re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
def parse_group(t,i):
    # t[i]=='{' ; returns (items, end) items are raw strings split at top-level commas
    depth=0; start=i+1; items=[]; j=i
    while j<len(t):
        c=t[j]
        if c=='{': depth+=1
        elif c=='}':
            depth-=1
            if depth==0:
                items.append(t[start:j]); return items,j+1
        elif c==',' and depth==1:
            items.append(t[start:j]); start=j+1
        j+=1
    return items,j
stats={}; rows=[]
for f in files:
    try: t=open(f,encoding="utf8").read()
    except Exception: continue
    if "semio_framework" not in t: continue
    for m in HEAD.finditer(t):
        i=m.end()
        if t[i:i+2]!="::": 
            rows.append((f,m.start(),"bare",""));continue
        i+=2
        while t[i:i+1] in " \n\t": i+=1
        if t[i]=='{':
            items,e=parse_group(t,i)
            for it in items:
                it=it.strip()
                mm=IDENT.match(it)
                if mm: rows.append((f,m.start(),"group",mm.group(0)+("::" if it[mm.end():mm.end()+2]=="::" else ("" if it[mm.end():].strip()=="" else "?"))))
        else:
            mm=IDENT.match(t,i)
            if mm:
                nxt=t[mm.end():mm.end()+2]
                rows.append((f,m.start(),"path",mm.group(0)+("::" if nxt=="::" else "")))
            else: rows.append((f,m.start(),"other",t[i:i+10]))
json.dump(rows,open(G+"/rsl-scan.json","w"),ensure_ascii=False)
from collections import Counter
c=Counter((k,s) for f,p,k,s in rows)
for (k,s),n in sorted(c.items(),key=lambda x:-x[1])[:80]: print(n,k,s)
print("files",len({r[0] for r in rows}),"rows",len(rows))
