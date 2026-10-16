"""🔍 External-crate idents mentioned by each host module file (and its included tests)."""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from common import *
import deps
H=OS+"🖥️host/"
d=deps.idents(REPO+H+"📦️packages/🦀️rust/Cargo.toml")
extra={"dsl","protocol","store","vcs","value_derive","graph","pack","ui_wgpu","wgpu","semio_framework"}
mods={"host":("🔌️plugin-host",["🧪️tests/🔬️host-unit"]),"backbone":("🗃️document-backbone",[]),"instance":("🧱️instance",["🧪️tests/🔬️instance-unit"]),"raster":("🖼️media-export-raster",["🧪️tests/🔬️media-export-raster-unit","🧪️tests/🔬️media-export-raster-wasip2"]),"simple":("🧾️media-export-simple",[]),"workflow":("🔀️workflow",["🧪️tests/🔬️workflow-unit","🧪️tests/🔬️workflow-standalone"]),"codec":("📡️codec-abi",["🧪️tests/🔬️codec-abi-unit"]),"registry":("🗂️registry",["🧪️tests/🔬️registry-unit"]),"space":("../🔨️modules/🪐️space",[]),"persistence":("💾️persistence",["💾️persistence/🧪️tests/🔬️unit"]),"activation":("🎠️activation",[])}
for m,(dn,tests) in mods.items():
    fl=[H+dn+"/🦀️.rs"]+[H+t+"/🦀️.rs" for t in tests]
    fl=[os.path.normpath(f) for f in fl]
    toks=deps.used(fl)
    print(m,'lib+tests:',sorted(i for i in list(d)+list(extra) if i in toks))
