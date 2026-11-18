import json,subprocess,shutil,hashlib,sys
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R126/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R126/"
f=json.load(open(T+"files.json",encoding="utf8"))
bm={"inputs.mjs":"base/inputs.mjs","entries.mjs":"base/entries.mjs","entries.d.mts":"base/entries.d.mts","reuse.mjs":"base/reuse.mjs","reuse.d.mts":"base/reuse.d.mts","reuse-schema.json":"base/reuse-schema.json","limits.json":"base/limits.json","nodes.mjs":"base/nodes.mjs","t-set-aside.ts":"base/t-set-aside.ts"}
sh=lambda p:hashlib.sha256(open(p,'rb').read()).hexdigest()[:12]
for live,name in f["mods"]:
    L="/Users/ueli/Documents/semio/"+live;B=SC+bm[name];S=SC+"stage/"+name
    if sh(L)==sh(B): continue
    r=subprocess.run(["git","merge-file","-p",L,B,S],capture_output=True)
    if r.returncode!=0:
        print("CONFLICT",name);continue
    open(S,"wb").write(r.stdout);shutil.copy(L,B);print("rebased",name,sh(B))
