"""🔗 Cross-module identifier references among the OS host modules (name-based over-approximation)."""
import sys,os,collections
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from common import *
from items import parse
H=REPO+OS+"🖥️host/"
mods={"host":"🔌️plugin-host","backbone":"🗃️document-backbone","instance":"🧱️instance","raster":"🖼️media-export-raster","simple":"🧾️media-export-simple","workflow":"🔀️workflow","codec":"📡️codec-abi","registry":"🗂️registry"}
defs={};toks={}
for m,d in mods.items():
    L,items=parse(H+d+"/🦀️.rs")
    names={}
    for it in items:
        if it['kind'] in('struct','enum','fn','const','static','type','trait','macro','union'):names[it['name']]=L[it['head']].strip().split(' ')[0]
    defs[m]=names;toks[m]=set().union(*[it['toks'] for it in items]) if items else set()
tot=collections.defaultdict(lambda:collections.defaultdict(set))
for m in mods:
    for o in mods:
        if m==o:continue
        hit=sorted(n for n in defs[o] if n in toks[m] and len(n)>3)
        if hit:print(f"{m} -> {o}: {len(hit)}: {', '.join(hit[:12])}{' ...' if len(hit)>12 else ''}")
