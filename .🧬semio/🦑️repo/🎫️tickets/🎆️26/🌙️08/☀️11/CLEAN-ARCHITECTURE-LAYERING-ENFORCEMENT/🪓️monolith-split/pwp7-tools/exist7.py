import sys, os, re, json, collections
sys.path.insert(0,'.')
from model7 import PLUG, GEN, mask
sys.path.insert(0,'/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pwp6-tools')
from sig6 import match_close
TYPES=['RenderCtx','RenderRef','DispatchCtx','TypedCtx','TimeTravelCtx','TimeTravelRef','ToolRunCtx','ToolRunRef','CommandCtx','LadderCtx','LadderRef','ExtensionCtx','ExtensionRef']
def files():
    for base in ['🧵️instance','🪜️maintenance-ladder','⏪️time-travel/🧵️instance','⏯️tool-run/🧵️instance']:
        for dp,dn,fn in os.walk(PLUG+base):
            dn[:]=[d for d in dn if d not in ('📦️packages','🧪️tests')]
            for f in fn:
                if f.endswith('.rs'): yield os.path.join(dp,f)
def blocks(text):
    M=mask(text); out=[]
    for m in re.finditer(r'\bimpl\s*<[^{;]*?>\s*('+'|'.join(TYPES)+r')\s*<[^{]*\{',M):
        op=m.end()-1; cp=match_close(M,op,'{','}')
        out.append((m.group(1),text[op:cp+1],M[op:cp+1]))
    return out
defined=collections.defaultdict(set); bodies=[]
for f in files():
    t=open(f,encoding='utf-8').read()
    for ty,txt,M in blocks(t):
        for m in re.finditer(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)',M): defined[ty].add(m.group(1))
        bodies.append((f,ty,txt,M))
# families: Ctx and Ref of one crate: Ctx members may call Ref via shared(); Ref members call only Ref members
FAM={'RenderCtx':'RenderRef','TimeTravelCtx':'TimeTravelRef','ToolRunCtx':'ToolRunRef','LadderCtx':'LadderRef'}
bad=collections.Counter(); rows=[]
for f,ty,txt,M in bodies:
    for m in re.finditer(r'(?<![\w.])self\s*\.\s*(\w+)\s*(?:::<[^>]*>)?\s*\(',M):
        n=m.group(1)
        if n in defined[ty]: continue
        rows.append((os.path.relpath(f,PLUG)[-60:],ty,n))
        bad[(ty,n)]+=1
for (ty,n),c in sorted(bad.items()): print(c,ty,n)
print('total unknown',sum(bad.values()))
