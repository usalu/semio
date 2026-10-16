"""🔗 Which module files reference which item names of which other module files (root = the engine root file)."""
import sys,os,collections
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from items import parse
N="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/"
mods={"root":"🦀️.rs","retirement":"🧵️retirement/🦀️.rs","topology":"🧭️topology/🦀️.rs","input":"📥️input/🦀️.rs","evaluation":"⏱️evaluation/🦀️.rs","wave":"⏱️evaluation/🌊️wave/🦀️.rs","hash":"⏱️evaluation/🌊️wave/🔑️hash/🦀️.rs","cache_insert":"🌱️cache/🦀️.rs","tree_copy":"🌳️tree/📋️copy/🦀️.rs","cold":"🧊️cold/🦀️.rs","registry":"📔️registry/🦀️.rs","output":"📔️registry/✅️output/🦀️.rs","admission":"📔️registry/🏗️admission/🦀️.rs","lookup":"📖️lookup/🦀️.rs","projection":"📤️projection/🦀️.rs","native":"🚦️native/🦀️.rs","failure":"⚠️failure/🦀️.rs"}
defs={};toks={}
for m,f in mods.items():
    L,items=parse(N+f)
    d={}
    for it in items:
        if it['kind'] in('struct','enum','fn','const','static','type','trait','union','macro'):d[it['name']]=1
    defs[m]=d
    s=set()
    for it in items:s|=it['toks']
    toks[m]=s
for m in mods:
    row=[]
    for o in mods:
        if o==m:continue
        hit=sorted(n for n in defs[o] if n in toks[m] and len(n)>3 and not n.startswith('_'))
        if hit:row.append(f"{o}[{len(hit)}]")
    print(f"{m:13s} uses: "+" ".join(row))
