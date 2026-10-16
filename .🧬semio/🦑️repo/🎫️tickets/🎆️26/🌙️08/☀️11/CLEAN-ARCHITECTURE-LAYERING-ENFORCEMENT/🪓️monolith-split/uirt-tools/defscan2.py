"""🔎️ Definition scan (refined): private fields/methods of lower-crate types that a downstream file of another crate names, minus names the downstream crate defines itself or std provides."""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import defscan as D
from widen import strip_comments
ORDER=D.ORDER
if os.environ.get('SEED'): D.texts['semio_framework_ui_runtime_reconcile']+='\nlet _ = state.cursor.as_ref().unwrap().base_revision;\nlet _ = shadow.retire_scalar;\n'
STD=set("len get get_mut is_empty iter take first last last_mut pop push insert remove contains keys values clone default new as_ref as_mut unwrap map from into drop step close_step terminal_is_empty id key value root surface revision text index ops".split())
def members(f):
    out=[]
    for (line,name,kind,vis,desc) in D.defs(f):
        if kind in("field","method"): out.append((name,kind,vis,f,line,desc))
    return out
allm=collections.defaultdict(list)
own_names=collections.defaultdict(set)
for c,fs in D.files.items():
    for f in fs:
        for (line,name,kind,vis,desc) in D.defs(f): own_names[c].add(name)
        t=open(f,encoding="utf-8").read()
        for m in re.finditer(r"^\s+(?:pub(?:\([^)]*\))?\s+)?(?:fn\s+)?([a-z_][a-z0-9_]*)\s*[:(]",t,re.M): own_names[c].add(m.group(1))
hits=0
seen=set()
for c,fs in D.files.items():
    for f in fs:
        if any(x in f for x in ("🧪️tests","🧰️testkit","🔄️transaction")): continue
        for (name,kind,vis,_,line,desc) in members(f):
            if name in STD: continue
            for d,txt in D.texts.items():
                if ORDER.index(d)<=ORDER.index(c): continue
                if name in own_names[d]: continue
                pat=r"\."+name+r"(?![A-Za-z0-9_(])" if kind=="field" else r"\."+name+r"\s*\("
                if re.search(pat,txt):
                    key=(c,name,d)
                    if key in seen: continue
                    seen.add(key); hits+=1
                    print(f"{c.replace('semio_framework_ui_runtime','rt')} {kind} {vis} {f.replace(D.base+'/','')}:{line} {desc[:60]} <- {d.replace('semio_framework_ui_runtime','rt')}")
print("hits",hits)
