import sys,os;sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from edit import *
SRC=SCR+"stage1/"
OB=B+"👁️observation/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/";MEM=B+"🧠️memory/"
rd=lambda p:open(p,encoding="utf8").read()
s=Stage(SCR+"stage1-land")
s.new(OB+"📥️reuse/🧺️coalesce/🟨️.mjs",rd(SRC+"new/coalesce.mjs"))
s.new(OB+"📥️reuse/🧺️coalesce/🟨️.d.mts",rd(SRC+"new/coalesce.d.mts"))
s.new(OB+"🧪️tests/🧺️presence-batch/🟦️.ts",rd(SRC+"new/presence.test.ts"))
s.new(MEM+"🔬️sampler/🟨️.mjs",rd(SRC+"new/sampler.mjs"))
s.new(MEM+"🔬️sampler/🧬️schema/🔣️.json",rd(SRC+"new/sampler.schema.json"))
s.new(MEM+"🔬️sampler/🧪️tests/🟦️.ts",rd(SRC+"new/sampler.test.ts"))
live=lambda rel:rd(R+"/"+rel)
def whole(rel,newtext,oldtext):
    i=str(s.n);s.n+=1;cur=live(rel)
    assert cur==oldtext,("live file changed since staging",rel)
    open(s.S+"/"+i+".base","w",encoding="utf8").write(oldtext);open(s.S+"/"+i+".new","w",encoding="utf8").write(newtext);s.rows.append((i,rel,"edit"))
whole(OB+"📥️reuse/🟨️.mjs",rd(SRC+"reuse.new"),rd(SRC+"reuse.base"))
whole(OB+"📥️reuse/🟨️.d.mts",rd(SRC+"reuse.d.new"),rd(SRC+"reuse.d.base"))
REQ=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts"
whole(REQ,rd(SRC+"required.new"),rd(SRC+"required.base"))
s.edit(PN+"🟨️.mjs",[
 ('import { inferenceInventoryReferencesHoldV1 } from "../../../../🏗️build/👁️observation/📥️reuse/🟨️.mjs";','import { inferenceInventoryReferencesHoldV1, inferenceInventoryPresenceStatsV1 } from "../../../../🏗️build/👁️observation/📥️reuse/🟨️.mjs";',1),
 ('import { describeCollectionV1, registerSliceCollectionsV1, registerSliceReportSourceV1 } from "../../../🧠️memory/📋️slice-report/🟨️.mjs";','import { describeCollectionV1, registerSliceCollectionsV1, registerSliceReportSourceV1 } from "../../../🧠️memory/📋️slice-report/🟨️.mjs";\nimport { startSliceSamplerV1 } from "../../../🧠️memory/🔬️sampler/🟨️.mjs";',1),
 ('const reportOff=registerSliceReportSourceV1("owners",','const samplerOff=startSliceSamplerV1(),reportOff=registerSliceReportSourceV1("owners",',1),
 ('scriptCollections:largestOf(scripts),farm:','presence:inferenceInventoryPresenceStatsV1(workspaceRoot,context.phaseControl),scriptCollections:largestOf(scripts),farm:',1),
 ('finally{reportOff();collectionsOff();','finally{reportOff();samplerOff();collectionsOff();',1)])
WK=B+"🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts"
s.edit(WK,[('const{layerVerdictCountsV1,closureCollectionsV1}=await import(','const{layerVerdictCountsV1,closureCollectionsV1,closureEntryCountsV1}=await import(',1),('report.registerSliceReportSourceV1("layers",()=>layerVerdictCountsV1());','report.registerSliceReportSourceV1("layers",()=>layerVerdictCountsV1());\nreport.registerSliceReportSourceV1("closureCounts",()=>closureEntryCountsV1());',1)])
GS=B+"🪪️graph-state/📦️project-records/"
s.new(GS+"🧪️tests/🏷️owner-token/🟦️.ts",rd(SCR+"stage2/owner-token.test.ts"))
whole(GS+"🟦️.ts",rd(SCR+"stage2/records.new"),rd(SCR+"stage2/records.base"))
s.finish([OB+"🧪️tests/🧺️presence-batch/🟦️.ts",MEM+"🔬️sampler/🧪️tests/🟦️.ts",OB+"🧪️tests/📥️reuse/🟦️.ts",OB+"🧪️tests/⏱️lookup-clock/🟦️.ts",REQ+"\tneutral actual managed reader callers",PN+"🧪️tests/📥️take/🟦️.ts",PN+"📥️take/🧪️tests/🟦️.ts",MEM+"📋️slice-report/🧪️tests/🟦️.ts",GS+"🧪️tests/🏷️owner-token/🟦️.ts",GS+"🧪️tests/🚚️closures/🟦️.ts",GS+"🧪️tests/🟦️.ts",GS+"🧳️portable/🧪️tests/🟦️.ts"],
"- Fix (R87): a closure entry that travels with a project record names the provider of the producer's source analyses by the token `⟦analysis-owner⟧`; the taker looked for the token only in the record document, so an entry whose record names no provider was stored with the token, its analysis rows never held and every consumer derived the entry again (25 of 607 sampled layers of the server store carry the token; 291 of 303 stale entries of a sample fail on one such row). `seedFrom` now asks for the providers of this machine when any document of the entry carries the token. Law `project-records/🧪️tests/🏷️owner-token` (red on the old code: the stored row keeps the token).\n- Step 1 (R87 `lookup.reusable`): the inventory presence check no longer takes a storage lease per record. `inferenceInventoryReferencesHoldV1` captures the names of the record and asks a presence coalescer (`📥️reuse/🧺️coalesce`, one per storage control and root): every look that asks while a batch runs joins the next batch, one `withRepositoryBuildGraphConsumerV1` checks the union of their names (`missingInventoryNamesV1`: the former body, same ancestry, identity, descriptor and delivery fences), a name found present is remembered for the validation round, a refused batch is asked again caller by caller. Law `🧪️tests/🧺️presence-batch` (answers equal an SQLite model; 48 concurrent callers: at most 2 batches, each distinct name checked once; round memory; refusal isolation; batch cap; structural guards of the real check). The structural law of the storage tests now reads `missingInventoryNamesV1`. Counters in the worker report `owners.presence`; the worker report also carries `closureCounts` (entries derived, loaded, stale, checked).\n- Diagnostic (R87): `🧠️memory/🔬️sampler` starts the engine's sampling profiler in the slice worker and drains it every 15 s into three uptime buckets (before 150 s, 150 to 400 s, later), source `profile` of `slice-memory.json` (top 24 self and inclusive functions with file and line); off with `SEMIO_SLICE_SAMPLER=off`. Law `🧠️memory/🔬️sampler/🧪️tests` (counts equal SQLite, schema, caps, buckets, report source).")
open(SCR+"stage1-land/before.txt","w",encoding="utf8").write(GS+"🧪️tests/🏷️owner-token/🟦️.ts\n")
