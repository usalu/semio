import sys,os,json
sys.path.insert(0,os.path.dirname(__file__))
from lib import R
from locks import *
G=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-carve-x/"
A=R+"/🧰️framework/🔨️modules/⏳️async"
ba=json.load(open(G+"manifests-async.json"))
ba={k:tuple(v) for k,v in ba.items()}
for d in ["⏰️timer-wheel","🚦️sync-primitives","🧱️boxed-slots","🎭️testkit"]:
    p=A+"/"+d+"/📦️packages/🦀️rust/Cargo.toml"
    ba[p]=(None,open(p,encoding="utf8").read())
p=A+"/📦️packages/🦀️rust/Cargo.toml"
new=open(p,encoding="utf8").read()
ba[p]=(new.replace("semio-framework-async-timer = { workspace = true }\n",""),new)
p=A+"/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml"
new=open(p,encoding="utf8").read()
old="\n".join(l for l in new.split("\n") if not l.startswith(("semio-framework-async-slots","semio-framework-async-sync","semio-framework-async-testkit","semio-framework-async-timer")))
ba[p]=(old,new)
ch=diff_manifests(ba)
json.dump(ch,open(G+"async-lock-spec.json","w"),ensure_ascii=False,indent=1)
print(len(ch),"packages changed")
apply="apply" in sys.argv
n=0
for lk in locks():
    try:
        if apply_to_lock(lk,ch,apply):n+=1
    except Exception as e:
        print("ERR",lk.replace(R+"/",""),e)
print(("applied" if apply else "would change"),n,"locks")
