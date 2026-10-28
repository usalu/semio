import json,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
pol=json.load(open("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"))
files=[f for f in json.load(open(C+"/🗑️generated/rust-g/native-closure-raw.json"))]
print("raw native closure",len(files))
for d in pol["domains"]:
    for p in d["prefixes"]:
        n=sum(1 for f in files if f.startswith(p))
        if n: print(d["id"].ljust(20),str(n).rjust(4),p[-90:])
