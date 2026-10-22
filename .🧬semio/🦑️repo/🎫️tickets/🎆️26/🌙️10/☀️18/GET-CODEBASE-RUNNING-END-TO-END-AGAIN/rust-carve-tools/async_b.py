import sys,os,re,json
sys.path.insert(0,os.path.dirname(__file__))
from lib import *
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
A=F+"/🔨️modules/⏳️async"
GROUPS={
 "semio_framework_async_testkit":{"ManualRuntime"},
 "semio_framework_async_sync":{"oneshot","Semaphore","Notify","Notified","OwnedPermit","AcquireOwned"},
 "semio_framework_async_slots":{"ShardedMap","boxed_slots","boxed_fixed_slots","FixedSlotTableBudget","assert_fixed_slot_tables","on_bounded_stack","OriginalFixedSlotPages"},
}
GROUPS["semio_framework_async_testkit"]={"ManualRuntime"}
PKG={"semio_framework_async_testkit":("semio-framework-async-testkit",A+"/🎭️testkit/📦️packages/🦀️rust"),
     "semio_framework_async_sync":("semio-framework-async-sync",A+"/🚦️sync-primitives/📦️packages/🦀️rust"),
     "semio_framework_async_slots":("semio-framework-async-slots",A+"/🧱️boxed-slots/📦️packages/🦀️rust")}
files=json.load(open(C+"/🗑️generated/rust-carve-x/async-user-files.json"))
owners=json.load(open(C+"/🗑️generated/rust-carve-x/async-owners-final.json"))
manifest_needs={}
unresolved=[]
for f in files:
    p=R+"/"+f
    s=get(p)
    s=s.replace("semio_framework_async::testkit::ManualRuntime","semio_framework_async::ManualRuntime")
    s2,used=split_use(s,"semio_framework_async",GROUPS)
    texts[p]=s2
    if not used:
        unresolved.append(("nochange",f));continue
    own=owners.get(f) or []
    if len(own)==1:m=R+"/"+own[0]
    else:
        m=owner_manifest(p);unresolved.append(("heuristic",f,m))
    manifest_needs.setdefault(m,set()).update(used)
for m,needs in manifest_needs.items():
    for crate in sorted(needs):
        name,pkgdir=PKG[crate]
        try:add_dep_like(m,"semio-framework-async",name,pkgdir)
        except AssertionError as e:print("NO-ANCHOR",m.replace(R+"/",""),[f[-80:] for f in files if (owners.get(f) or [None])[0]==m.replace(R+"/","")  or owner_manifest(R+"/"+f)==m])
for u in unresolved:print(u[0],u[1][-90:],(u[2] or "")[-80:] if len(u)>2 else "")
print({m.replace(R+"/","")[-70:]:sorted(n) for m,n in manifest_needs.items()})
flush()
