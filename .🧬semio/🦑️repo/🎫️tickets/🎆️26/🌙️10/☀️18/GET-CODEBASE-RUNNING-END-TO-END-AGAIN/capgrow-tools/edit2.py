"""🌱️ capgrow round 2: a run may grow right after its admission (the admission reservation counts in full while it ramps, so nothing is counted twice); only the reservation of a previous growth must ramp before the next one. `python3 -I edit2.py <root>` (idempotent, refuses on a missing anchor)."""
import os,sys
ROOT=sys.argv[1];HC="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
def edit(rel,pairs):
    path=os.path.join(ROOT,rel);text=open(path,encoding="utf8").read();mode=os.stat(path).st_mode&0o7777;changed=False
    for old,new in pairs:
        if new in text:continue
        if text.count(old)!=1:sys.exit(f"anchor missing or not unique in {rel}: {old[:120]!r}")
        text=text.replace(old,new);changed=True
    if changed:
        temporary=path+".capgrow.tmp"
        with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
        os.chmod(temporary,mode);os.replace(temporary,path)
    print(("edited " if changed else "already ")+rel)
edit(HC+"/📏️decision/🟦️.ts",[
 ("`ramp` while the grant still ramps up (its last reservation is not yet observed, so it cannot grow twice on the same memory)",
  "`ramp` while the reservation of its previous growth still ramps up (not yet observed, so it cannot grow twice on the same memory; the admission reservation counts in full while it ramps, so a run may grow right after it was admitted)"),
 ("if(replaced===undefined||ramping(replaced)){refuse(\"ramp\");continue;}",
  "if(replaced===undefined||replaced.grownAtMs!==undefined&&ramping(replaced)){refuse(\"ramp\");continue;}"),
])
edit(HC+"/🔼️upgrade/🟦️.ts",[
 ("after its last reservation was observed (the ramp)","after the reservation of its previous growth was observed (its ramp; the admission reservation counts in full while it ramps)"),
])
edit(HC+"/🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py",[
 ("if replaced is None or ramping(replaced):count(\"growRamp\")","if replaced is None or (replaced.get(\"grownAtMs\") is not None and ramping(replaced)):count(\"growRamp\")"),
])
edit(HC+"/🧪️tests/🔼️upgrade/🌱️grow/🐍️oracle/🐍️.py",[
 ("    if rng.random()<0.2:g[\"arrivedAtMs\"]=g[\"admittedAtMs\"]-rng.choice([0,800])\n",
  "    if rng.random()<0.2:g[\"arrivedAtMs\"]=g[\"admittedAtMs\"]-rng.choice([0,800])\n    if rng.random()<0.5:g[\"grownAtMs\"]=g.get(\"resumedAtMs\",g[\"admittedAtMs\"])\n"),
])
