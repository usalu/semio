import sys,os;sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from edit import *
SRC=SCR+"stage2/"
GS=B+"🪪️graph-state/📦️project-records/";MEM=B+"🧠️memory/"
rd=lambda p:open(p,encoding="utf8").read()
s=Stage(SCR+"stage2-land")
def whole(rel,newtext,oldtext):
    i=str(s.n);s.n+=1;cur=rd(R+"/"+rel)
    assert cur==oldtext,("live file changed since staging",rel)
    open(s.S+"/"+i+".base","w",encoding="utf8").write(oldtext);open(s.S+"/"+i+".new","w",encoding="utf8").write(newtext);s.rows.append((i,rel,"edit"))
s.new(GS+"🧪️tests/🏷️owner-token/🟦️.ts",rd(SRC+"owner-token.test.ts"))
s.new(GS+"🧳️portable/🧪️tests/🔎️token-scan/🟦️.ts",rd(SRC+"token-scan.test.ts"))
whole(GS+"🟦️.ts",rd(SRC+"records.new"),rd(SRC+"records.base"))
whole(GS+"🧳️portable/🟦️.ts",rd(SRC+"portable.new"),rd(SRC+"portable.base"))
whole(MEM+"🔬️sampler/🟨️.mjs",rd(SRC+"sampler.new"),rd(SRC+"sampler.base"))
whole(MEM+"🔬️sampler/🧬️schema/🔣️.json",rd(SRC+"sampler.schema.new"),rd(SRC+"sampler.schema.base"))
whole(MEM+"🔬️sampler/🧪️tests/🟦️.ts",rd(SRC+"sampler.test.new"),rd(SRC+"sampler.test.base"))
WF=B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/"
WK=B+"🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts"
s.new(WF+"🧪️tests/📊️stats/🟦️.ts",rd(SRC+"walkstats.test.ts"))
whole(WF+"🟨️.mjs",rd(SRC+"walkfarm.new"),rd(SRC+"walkfarm.base"))
whole(WK,rd(SRC+"worker.new"),rd(SRC+"worker.base"))
s.finish([WF+"🧪️tests/📊️stats/🟦️.ts",WF+"🧪️tests/🟦️.ts",GS+"🧪️tests/🏷️owner-token/🟦️.ts",GS+"🧳️portable/🧪️tests/🔎️token-scan/🟦️.ts",GS+"🧳️portable/🧪️tests/🟦️.ts",GS+"🧪️tests/🚚️closures/🟦️.ts",MEM+"🔬️sampler/🧪️tests/🟦️.ts",MEM+"📋️slice-report/🧪️tests/🟦️.ts"],
rd(SRC+"note.txt"))
open(SCR+"stage2-land/before.txt","w",encoding="utf8").write(GS+"🧪️tests/🏷️owner-token/🟦️.ts\n"+GS+"🧳️portable/🧪️tests/🔎️token-scan/🟦️.ts\n"+WF+"🧪️tests/📊️stats/🟦️.ts\n")
