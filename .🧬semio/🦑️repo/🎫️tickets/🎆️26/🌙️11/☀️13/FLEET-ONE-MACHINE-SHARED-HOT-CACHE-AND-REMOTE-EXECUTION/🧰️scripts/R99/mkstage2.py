#!/usr/bin/env python3
"""R99 stage 2: the busy-funding fixture registers its holders with a heartbeat of 1 ms since the epoch; under the heartbeat rule that is a dead owner, so the fixture renews the registration."""
import os,shutil
R="/Users/ueli/Documents/semio"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R99/stage2"
shutil.rmtree(S,ignore_errors=True)
for d in("base","new"):os.makedirs(S+"/"+d)
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SETUP=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/"
BF=SETUP+"🧪️tests/⏳️busy-funding/🟦️.ts"
bf=open(R+"/"+BF,"rb").read();assert b"updatedAtMs:1," in bf
open(S+"/base/b1","wb").write(bf);open(S+"/new/b1","wb").write(bf.replace(b"updatedAtMs:1,",b"updatedAtMs:Date.now(),"))
open(S+"/index.txt","w",encoding="utf8").write("b1 %s edit\n"%BF)
open(S+"/tests.txt","w",encoding="utf8").write(BF+"\n"+SETUP+"♻️retirement/🧪️tests/🔁️busy-driver/🟦️.ts\n"+SETUP+"💰️funding/🧪️tests/🫀️heartbeat/🟦️.ts\n"+SETUP+"💰️funding/🧪️tests/🧬️reused-pid/🟦️.ts\n")
open(S+"/note.txt","w",encoding="utf8").write("- Follow-up to the R99 landing of 23:00: `⏳️busy-funding` fixtures register holders with `updatedAtMs:1`; under the heartbeat rule a registration that old with an unknown start is a dead owner, so the fixtures now renew it (`Date.now()`). Red before: `the wait line names the holder by pid, command and start time in both languages` failed after the landing. Standing-red there: `the real process ancestry equals an independent walk with ps ...` (same before).\n")
print("ok")
