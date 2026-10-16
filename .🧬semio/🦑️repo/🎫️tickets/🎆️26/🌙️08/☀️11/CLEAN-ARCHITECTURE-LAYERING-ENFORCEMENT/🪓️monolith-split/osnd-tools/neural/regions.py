"""🗺️ Region-level reference graph of the neural engine root file (name based)."""
import sys,os,collections
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from items import parse
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs"
L,items=parse(P)
defs={}
for it in items:
    if it['kind'] in('struct','enum','fn','const','static','type','trait','union','macro'):defs[it['name']]=it['region'].split('/')[0] or 'root'
reg=collections.defaultdict(lambda:collections.defaultdict(set))
size=collections.Counter()
for it in items:
    r=it['region'].split('/')[0] or 'root'
    size[r]+=it['bytes']
    for n in it['toks']:
        if n in defs and defs[n]!=r:reg[r][defs[n]].add(n)
for r in size:
    print(f"== {r} ({size[r]} bytes)")
    for o,names in sorted(reg[r].items()):print(f"   -> {o}: {len(names)}: {', '.join(sorted(names))[:300]}")
