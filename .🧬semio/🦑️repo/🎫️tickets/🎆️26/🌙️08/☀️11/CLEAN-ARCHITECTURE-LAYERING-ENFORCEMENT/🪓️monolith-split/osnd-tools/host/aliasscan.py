"""🔍 Which crate idents (dependency keys and extern-crate aliases) each host module and its included tests mention in code."""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from common import *
import deps
H=OS+"🖥️host/"
d=deps.idents(REPO+H+"📦️packages/🦀️rust/Cargo.toml")
names=set(d)|{"dsl","protocol","store","vcs","value_derive","semio_framework_os_kernel"}
mods={"backbone":("🗃️document-backbone",[]),"instance":("🧱️instance",["🧪️tests/🔬️instance-unit"]),"raster":("🖼️media-export-raster",["🧪️tests/🔬️media-export-raster-unit","🧪️tests/🔬️media-export-raster-wasip2"]),"simple":("🧾️media-export-simple",[]),"workflow":("🔀️workflow",["🧪️tests/🔬️workflow-unit","🧪️tests/🔬️workflow-standalone"]),"codec":("📡️codec-abi",["🧪️tests/🔬️codec-abi-unit"]),"registry":("🗂️registry",["🧪️tests/🔬️registry-unit"]),"host":("🔌️plugin-host",["🧪️tests/🔬️host-unit"]),"space":("../🔨️modules/🪐️space",[]),"persistence":("💾️persistence",["💾️persistence/🧪️tests/🔬️unit"]),"activation":("🎠️activation",[]),"root":("../📦️packages/🦀️rust",[])}
from rlex2 import lex2
def toks_by(f):
    t=open(f,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')]
    s=set()
    for info in lex2(L):
        for n,_ in info['toks']:s.add(n)
    return s
for m,(dn,tests) in mods.items():
    lib=os.path.normpath(REPO+H+dn+"/🦀️.rs");libt=toks_by(lib)
    tt=set()
    for t in tests:tt|=toks_by(os.path.normpath(REPO+H+t+"/🦀️.rs"))
    print(f"{m:11s} lib:",sorted(n for n in names if n in libt),"| tests-only:",sorted(n for n in names if n in tt and n not in libt))
