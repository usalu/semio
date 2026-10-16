"""🧪 Assigns every item of the old math-unit test file to the lowest math crate that can host it."""
import re, sys, json
from rsitems import split_items, regions
B="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uiscene/base/🎬️scene/"
math=open(B+"📐️math/🦀️.rs",encoding="utf-8").read()
tests=open(B+"🧪️tests/🔬️math-unit/🦀️.rs",encoding="utf-8").read()
R=regions(math)
CRATE={'🔖️SyncAlgebra':'algebra','Camera':'camera','GumballMath':'camera','LodGrid':'camera','Mesh':'mesh','ScenePass':'pass','ProceduralGrid':'pass','LineDraw':'pass','Culling':'pass','Picking':'picking'}
RANK={'algebra':0,'camera':1,'mesh':1,'pass':2,'picking':3}
lines=math.split('\n')
defs={}
items=split_items(math)
for it in items:
    if it.kind in('marker',): continue
    ln=it.start
    for rn,(s,e) in R.items():
        if s<=ln<=e and rn in CRATE:
            if it.kind in('fn','struct','enum','trait','type','const','static'):
                defs.setdefault(it.name,CRATE[rn])
# children of mesh
for f in ['📐️math/♻️retirement/🦀️.rs','📐️math/📦️resource/🦀️.rs','📐️math/📦️resource/📥️receiving/🦀️.rs']:
    t=open(B+f,encoding='utf-8').read()
    for it in split_items(t):
        if it.kind in('fn','struct','enum','trait','type','const','static'): defs.setdefault(it.name,'mesh')
titems=[it for it in split_items(tests) if it.kind not in('marker','use')]
local={}
for it in titems:
    if it.kind in('fn','struct','const','static') : local[it.name]=it
def used(text):
    return set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',text))
res={}
def crates_of(it,seen=None):
    seen=seen or set()
    if it.name in seen: return set()
    seen.add(it.name)
    ids=used(it.text)
    cs=set()
    for i in ids:
        if i in defs and i!=it.name: cs.add(defs[i])
    for i in ids:
        if i in local and i!=it.name and local[i] is not it: cs|=crates_of(local[i],seen)
    return cs
out=[]
for it in titems:
    cs=crates_of(it)
    if 'test' in it.text[:200] or it.text.lstrip().startswith(('#[test]','///')): pass
    out.append((it.start+1,it.end+1,it.kind,it.name,sorted(cs)))
for o in out: print(*o)
