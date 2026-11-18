#!/usr/bin/env python3
"""R99 stage: regenerates the edited files from the LIVE files (anchored edits), copies the new files, writes index/base/new/tests/note under the scratch stage directory."""
import os,sys,subprocess,shutil
R="/Users/ueli/Documents/semio"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R99"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R99"
S=SC+"/stage"
shutil.rmtree(S,ignore_errors=True)
for d in("base","new"):os.makedirs(S+"/"+d)
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
PLAT="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/"
SETUP=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/";FUND=SETUP+"💰️funding/"
TEXTS=L+"⚡️caching/📖️texts/🔣️.json"
rows=[]
def add(i,rel,kind,new,base=None):
    open(S+"/new/"+i,"wb").write(new)
    if base is not None:open(S+"/base/"+i,"wb").write(base)
    rows.append("%s %s %s"%(i,rel,kind))
rd=lambda p:open(p,"rb").read()
run=lambda *a:subprocess.run(a,check=True)
run("python3",T+"/edit_platform.py",R+"/"+PLAT+"🟦️.ts",SC+"/platform.live.new.ts")
run("python3",T+"/edit_funding.py",R+"/"+FUND+"🟦️.ts",SC+"/funding.live.new.ts",R+"/"+TEXTS,SC+"/texts.live.new.json")
add("p",PLAT+"🟦️.ts","edit",rd(SC+"/platform.live.new.ts"),rd(R+"/"+PLAT+"🟦️.ts"))
add("f",FUND+"🟦️.ts","edit",rd(SC+"/funding.live.new.ts"),rd(R+"/"+FUND+"🟦️.ts"))
add("t",TEXTS,"edit",rd(SC+"/texts.live.new.json"),rd(R+"/"+TEXTS))
RP=PLAT+"🧮️read-plan/"
add("r1",RP+"🟦️.ts","new",rd(SC+"/rp/🟦️.ts"))
add("r2",RP+"🧬️schema/🔣️.json","new",rd(SC+"/rp/🧬️schema/🔣️.json"))
add("r3",RP+"🧪️tests/🟦️.ts","new",rd(SC+"/rp/🧪️tests/🟦️.ts"))
add("r4",RP+"🧪️tests/🔣️.json","new",rd(SC+"/rp/🧪️tests/🔣️.json"))
add("r5",RP+"🧪️tests/oracle.py","new",rd(SC+"/rp/🧪️tests/oracle.py"))
BF=SETUP+"🧪️tests/⏳️busy-funding/🟦️.ts"
bf=rd(R+"/"+BF);assert b"updatedAtMs:1," in bf
add("b1",BF,"edit",bf.replace(b"updatedAtMs:1,",b"updatedAtMs:Date.now(),"),bf)
add("h1",FUND+"🧪️tests/🫀️heartbeat/🟦️.ts","new",rd(SC+"/hb/🟦️.ts"))
open(S+"/index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
B=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/";DS=L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/"
tests=[BF,RP+"🧪️tests/🟦️.ts",FUND+"🧪️tests/🫀️heartbeat/🟦️.ts",FUND+"🧪️tests/🧬️reused-pid/🟦️.ts",PLAT+"../📸️observation/🧪️tests/🟦️.ts",B+"📥️opening/📤️publication/🧪️tests/♻️phase/🟦️.ts",B+"📥️opening/📤️publication/⚙️setup/🧪️tests/🟦️.ts",B+"📥️opening/📤️publication/⚙️setup/🧪️tests/🌊️moving-tree/🟦️.ts",DS+"🪜️ancestry/🟦️.ts",DS+"📏️work/🟦️.ts",L+"⚡️caching/📖️texts/🧪️tests/🟦️.ts",SETUP+"♻️retirement/🧪️tests/🔁️busy-driver/🟦️.ts",SETUP+"♻️retirement/🧪️tests/🟦️.ts"]
open(S+"/tests.txt","w",encoding="utf8").write("\n".join(os.path.normpath(t) for t in tests)+"\n")
open(S+"/note.txt","w",encoding="utf8").write(open(T+"/note.txt",encoding="utf8").read())
print("staged",len(rows),"files")
