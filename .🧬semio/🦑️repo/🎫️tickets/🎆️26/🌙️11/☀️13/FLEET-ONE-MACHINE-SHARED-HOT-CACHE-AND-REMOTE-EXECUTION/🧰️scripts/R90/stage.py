import os,shutil,sys
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";RS=L+"🏗️build/🏡️resident/"
rows=[("1",RS+"🏛️service/🧯️failure/🟨️.mjs","edit"),("2",RS+"🏛️service/🧯️failure/🟨️.d.mts","edit"),("3",RS+"🧬️schema/🔣️.json","edit"),("4",RS+"🧪️tests/🧯️failure/🟦️.ts","edit"),("5",RS+"🧪️tests/🌐️host/🟦️.ts","edit"),("6",RS+"🧪️tests/🌐️host/💥️throwing-host-main.mjs","new")]
S="stage-a";shutil.rmtree(S,ignore_errors=True);os.makedirs(S)
open(S+"/index.txt","w",encoding="utf8").write("".join("%s %s %s\n"%r for r in rows))
for i,rel,kind in rows:
    shutil.copy("work/"+i,S+"/"+i+".new")
    if kind=="edit":shutil.copy("/Users/ueli/Documents/semio/"+rel,S+"/"+i+".base")
open(S+"/tests.txt","w",encoding="utf8").write("\n".join([RS+"🧪️tests/🧯️failure/🟦️.ts",RS+"🧪️tests/🌐️host/🟦️.ts"])+"\n")
open(S+"/before.txt","w",encoding="utf8").write(RS+"🧪️tests/🧯️failure/🟦️.ts\n")
open(S+"/allowed-red.txt","w",encoding="utf8").write("slice memory guard > a plan that would cross\nslice memory guard > a worker growing past\nbounded memory > a full disk reaches the client\n")
open(S+"/note.txt","w",encoding="utf8").write("""- An unexpected failure of the resident service carries its message (name, message, code, at most 300 characters) in the refusal line itself, in English and German (`... (reference 3a9fba88): TypeError: ...; try again ...`), and the service process itself writes the whole block (reference, name, code, message, English and German text, stack; at most 64 KiB) to `<store>/v1/origins/<origin>/service-failure.txt` and `service-failure-<reference>.txt` (newest 16 kept) within the request, from `SEMIO_FLEET_STORE_ROOT` and `SEMIO_FLEET_ORIGIN` of its environment: independent of the `ready` child (R86 landing d is not in the live tree; its `readyFailureBlockV1` is absent). Laws: `resident/🧪️tests/🧯️failure` (3 new) and the real-process law `real process: a resident service that throws ...` in `resident/🧪️tests/🌐️host` (fixture `💥️throwing-host-main.mjs`). The host suite has 3 environmental reds on this machine before and after (slice memory guard x2, full disk).
""")
