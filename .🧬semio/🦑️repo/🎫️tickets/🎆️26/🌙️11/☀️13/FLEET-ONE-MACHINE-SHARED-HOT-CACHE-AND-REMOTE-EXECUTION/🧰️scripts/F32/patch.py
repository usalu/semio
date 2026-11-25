import os,sys,shutil
root="/Users/ueli/Documents/semio"
T=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=root+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
target=S+"/🟦️.ts"
dest=S+"/📜️journal/🧷️append"
start='const path=join(access.backing.root,"storage-events.jsonl");check();const named=await statCleanupFile(path,{bigint:true});'
end='await file.sync();check();});'
imp='import{createJournalReceiptV1}from"./📜️journal/🔖️revision/🟦️.ts";'
newimp=imp+'\nimport{retireChildFundingJournalV1}from"./📜️journal/🧷️append/🟦️.ts";'
call='const path=join(access.backing.root,"storage-events.jsonl"),retired=await retireChildFundingJournalV1({path,first:observed.text,observe:async()=>(await readCleanupJournal(access.backing.root,Math.min(source.options.policy.capacityBytes,access.backing.maximumPhysicalBytes),check,()=>checkAcceptedPhysicalCleanupV1(enrollment))).text,suffixFor:text=>nativeStorageChildRetirementJournalV1(text,source.options.policy.capacityBytes,source.allocation).slice(text.length),limit:cleanupProtocol.journalMaximumBytes,check,withDescriptor:withArtifactDescriptorV1});'
oldfinal='final.text!==observed.text+suffix'
newfinal='final.text!==retired.observed+retired.suffix'
text=open(target,encoding="utf8").read()
a=text.index(start);b=text.index(end,a)+len(end)
assert text.count(start)==1 and text.count(imp)==1 and text.count(oldfinal)==1 and text.index(oldfinal)>b
if "--apply" not in sys.argv:
    print("anchors ok");sys.exit(0)
text=text[:a]+call+text[b:]
old='import{lstat as statCleanupFile,open as openCleanupFile}from"node:fs/promises";'
assert text.count(old)==1
text=text.replace(old,'import{lstat as statCleanupFile}from"node:fs/promises";')
text=text.replace(oldfinal,newfinal).replace(imp,newimp)
os.makedirs(dest+"/🧪️tests",exist_ok=True);os.makedirs(dest+"/🧬️schema",exist_ok=True)
for rel in ["🟦️.ts","🧬️schema/🔣️.json","🧪️tests/🟦️.ts"]:
    shutil.copyfile(T+"/🧰️scripts/F32/staged/"+rel,dest+"/"+rel)
open(target,"w",encoding="utf8").write(text)
print("applied")
