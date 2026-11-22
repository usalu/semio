import re,sys
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E19/"
pat=re.compile(r"Original parent Script process invocation required|artifact directory required|ARTIFACT_DIR is required|artifacts required|Original native caller capabilities required|Original ticket required|fixture authority required|captured runtime entry provider required|Short test sandbox root required|Original artifact namespace|Original immutable SDK required")
out={}
for b in open(G+"logs.txt").read().split("=== ")[1:]:
    h,_,body=b.partition("\n")
    if " rc=0 " in h: continue
    if pat.search(body): out[h.split(" rc=")[0]]=1
print("\n".join(out))
