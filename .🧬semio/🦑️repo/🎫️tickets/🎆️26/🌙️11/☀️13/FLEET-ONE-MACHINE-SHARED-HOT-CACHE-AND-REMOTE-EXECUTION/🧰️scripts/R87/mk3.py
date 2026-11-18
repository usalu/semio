import sys,os;sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from edit import *
SRC=SCR+"stage3/";MEM=B+"🧠️memory/🔬️sampler/";OB=B+"👁️observation/"
rd=lambda p:open(p,encoding="utf8").read()
s=Stage(SCR+"stage3-land")
def whole(rel,new,old):
    i=str(s.n);s.n+=1;assert rd(R+"/"+rel)==old,rel
    open(s.S+"/"+i+".base","w",encoding="utf8").write(old);open(s.S+"/"+i+".new","w",encoding="utf8").write(new);s.rows.append((i,rel,"edit"))
whole(MEM+"🟨️.mjs",rd(SRC+"s.new"),rd(SRC+"s.base"))
whole(MEM+"🧪️tests/🟦️.ts",rd(SRC+"t.new"),rd(SRC+"t.base"))
whole(OB+"📥️reuse/🟨️.mjs",rd(SRC+"r.new"),rd(SRC+"r.base"))
s.finish([MEM+"🧪️tests/🟦️.ts",OB+"🧪️tests/🧺️presence-batch/🟦️.ts",OB+"🧪️tests/📥️reuse/🟦️.ts",L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts\tneutral actual managed reader callers"],
"- Workers (R87, on the coordinator's report that g1-m10 ends `workspace` tries with an unexpected resident-service failure since s1): the engine's sampling profiler of the slice worker is off unless `SEMIO_SLICE_SAMPLER=on` (it walks the stack of the running thread; its cost and its part in the failure are not proven, so no worker carries it by default); `SEMIO_INVENTORY_COALESCE=off` makes the inventory presence check take its own consumer per call again, to tell the coalescer from the profiler if a failure remains. Law `🔬️sampler/🧪️tests` (a worker does not start the profiler unless asked).")
