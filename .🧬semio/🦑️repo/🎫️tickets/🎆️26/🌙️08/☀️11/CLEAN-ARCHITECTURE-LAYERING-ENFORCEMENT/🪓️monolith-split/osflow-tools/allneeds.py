"""🔎 Top-level item names (any visibility) of provider files used by a planned crate. usage: allneeds.py <crate> <provider rel file>..."""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from families import *
from usage import toks
from complete import defined
from rlex2 import lex2
from wsan import top_items,classify
def items(p):
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);d={}
    for it in top_items(L,I):
        classify(it)
        if it.get('name') and it['kind'] in('fn','struct','enum','const','static','type','trait','union','mod'):d[it['name']]=it['vis'] or 'priv'
        if it['kind']=='macro':
            m=re.match(r'\s*macro_rules!\s*(\w+)',''.join(L[it['head']:it['end']+1]))
            if m:d[m.group(1)]='macro'
    return d
cid=sys.argv[1];t=set()
own=set()
for f in exists(cid):
    if f.endswith("🧱️core/🦀️.rs") or f.endswith("🎞️session/🦀️.rs"):continue
    t|=toks(f);own|=set(items(f))
for prov in sys.argv[2:]:
    d=items(FL+prov)
    used={k:v for k,v in d.items() if k in t and k not in own}
    print(prov.split('/')[0],used)
