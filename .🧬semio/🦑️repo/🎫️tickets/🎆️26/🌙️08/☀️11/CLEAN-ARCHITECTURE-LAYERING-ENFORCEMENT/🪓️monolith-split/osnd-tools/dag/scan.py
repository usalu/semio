"""🔍 Which dependency idents each DAG crate closure mentions (lib, test, wasm)."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from common import *
import deps
D=OS+"🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/"
old=deps.idents(REPO+D+"📦️packages/🦀️rust/Cargo.toml")
extra={"graph","dsl","protocol","store"}
def files(entry,pkg,cfgs):
    s={}
    for c in cfgs:
        for r in closure(entry,pkg,c):
            if r[0].endswith(".rs") and not r[0].startswith("/"):s[r[0]]=1
    return list(s)
for name,entry,pkg in [("child",D+"🌿️vcs/🦀️.rs",D+"🌿️vcs"),("parent",D+"🦀️.rs",D+"📦️packages/🦀️rust")]:
    for label,cfgs in [("lib",[""]),("test",["test"]),("wasm",["target_arch=wasm32,target_os=unknown,target_family=wasm"])]:
        toks=deps.used(files(entry,pkg,cfgs))
        print(name,label,sorted(i for i in list(old)+list(extra) if i in toks))
