import json,subprocess,sys,os
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
os.chdir("/Users/ueli/Documents/semio")
g=json.load(open(C+"/1102-tools/phase/wco/groups.json"))
pri=["📐️cad","🌊️flow","🌍️gis","🌀️procedural","🎬️sequence","🪵️sourcing","🏗️fem"]
keys=sorted(g,key=lambda k:(0 if k.startswith("🧰️") else 1 if any(p in k for p in pri) else 2,k))
for n,k in enumerate(keys):
    label=f"W-wco-{n}"
    man=k if not k.startswith("🧰️") else "🌎️hub/Cargo.toml"
    args=[]
    for c in g[k]:args+=["-p",c]
    subprocess.run([C+"/1102-tools/chk.sh",label,man,"wasm"]+args)
    open(C+"/1102-tools/phase/wco/index.txt","a").write(f"{label} {k} rc={open(C+'/🗑️generated/1102/'+label+'.rc').read().strip()}\n")
open(C+"/1102-tools/phase/wco/chain.done","w").write("done")
