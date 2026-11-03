import json,re,sys,collections
sys.path.insert(0,".")
from mk import crate_of,R
import a,b
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad"
o=[x for x in json.load(open(S+"/cur.json")) if re.search(r'unwrap|expect|panic!|unreachable',x[2]) and 'tickets' not in x[0]]
edits={}
for n,(spec,_) in a.S.items():
    for rel,e in spec.items():edits.setdefault(rel,[]).extend(e)
import c
import d
import f,g,h
for rel,e,_ in b.B+c.C+d.D+f.F+g.G+h.H:edits.setdefault(rel,[]).extend(e)
pat=re.compile(r'\.unwrap\(\)|\.expect\(|panic!\(|unreachable!')
rows=[];fixed=0;total=0
for f,n,_ in o:
    L=open(R+"/"+f,encoding="utf8").read().split("\n");line=L[n-1]
    k=len(pat.findall(line));total+=k
    t=line
    for e in edits.get(f,[]):
        old,new=e[0],e[1]
        if old in t:t=t.replace(old,new)
    k2=len(pat.findall(t));fixed+=k-k2
    if k2:
        why="?"
        p=f
        if "🧰️testing" in p or "cold-laws" in p or "🔮️oracles" in p or "📤️jobs" in p or "🧪" in p:why="test-support module (move under 🧪️tests/ or cfg(test))"
        elif "✨️derive" in p or "⚙️expansion" in p or "🔀️enum" in p:why="proc-macro: emit syn::Error"
        elif "unreachable!" in t and re.search(r'phase|=>unreachable|_=>unreachable|else\{unreachable',t):why="phase/state machine: typed enum makes arm unrepresentable"
        elif "panic!" in t:why="embedded-resource/drop-path panic: Result at construction"
        elif re.search(r'\|[^|]*\|[^|]*\.(unwrap|expect)\(',t) or "project(" in t or "RetainedWorkOutcome" in t:why="closure/outcome signature: needs fallible variant of the API (ref-returning closure or RetainedWorkOutcome)"
        elif "checked_" in t or "try_into" in t or "Layout" in t:why="arithmetic/layout bound: saturating or typed bound error"
        else:why="Option owner invariant: Result<_,ValueError::invariant> (same recipe as patched sites)"
        rows.append((crate_of(f),why,f,n))
print("total hit-matches",total,"fixed matches",fixed,"remaining sites",len(rows))
c=collections.Counter((r[0],r[1]) for r in rows)
out=collections.defaultdict(list)
for (cr,why),v in sorted(c.items()):out[cr].append(f"{why} x{v}")
json.dump({cr:v for cr,v in out.items()},open("remaining.json","w"),ensure_ascii=False,indent=1)
byc=collections.Counter(r[0] for r in rows)
for cr,v in byc.most_common(12):print(v,cr)
cw=collections.Counter(r[1] for r in rows)
for w,v in cw.most_common():print(v,w)
