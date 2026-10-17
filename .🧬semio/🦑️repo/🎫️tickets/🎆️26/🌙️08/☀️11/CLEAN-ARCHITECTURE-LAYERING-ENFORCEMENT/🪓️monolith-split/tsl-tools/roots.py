import json,os,sys
R="/Users/ueli/Documents/semio/"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/"
def load(n): return json.load(open(G+n,encoding='utf8'))["files"]
def rel(p): return p[len(R):] if p.startswith(R) else p
out={}
for n,o in (("psc-tsconfig-after2.json","psc"),("sdk2-tsconfig-after.json","sdk2"),("tte-tsconfig-psc-live.json","tte-psc-live"),("tte-tsconfig-sdk2-live.json","tte-sdk2-live")):
    files=[rel(f) for f in load(n)]
    live=[f for f in files if os.path.isfile(R+f)]
    dead=[f for f in files if not os.path.isfile(R+f)]
    out[o]=live
    open(G+f"tsl-roots-{o}.txt","w",encoding="utf8").write("\n".join(live)+"\n")
    print(n,"listed",len(files),"live",len(live),"stale",len(dead))
    for d in dead: print("   stale:",d)
dropped=[rel(l.strip()) for l in open(G+"tte-psc-dropped-roots.txt",encoding='utf8') if l.strip()]
print("tte dropped roots now existing:",[(d.split('/')[-2],os.path.isfile(R+d)) for d in dropped])
print("psc-live vs psc: psc-only",len(set(out['psc'])-set(out['tte-psc-live'])),"live-only",len(set(out['tte-psc-live'])-set(out['psc'])))
