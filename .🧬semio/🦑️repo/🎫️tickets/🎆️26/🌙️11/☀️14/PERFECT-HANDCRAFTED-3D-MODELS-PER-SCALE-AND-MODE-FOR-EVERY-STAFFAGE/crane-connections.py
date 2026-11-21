"""🏗️ Draw the tower crane's full-size chord outlines and fit the two heel joint spigots to the real lower chords."""
import runpy
import json
import sys
from pathlib import Path

ticket=Path(__file__).resolve().parent
helpers=runpy.run_path(str(ticket/"equipment-sections.py"))
path,asset=helpers["document"]("tower-crane")
published=ticket/"volumes/machinery/tower-crane.json"
author=json.loads(published.read_text())["volumes"]
edits=[]
for scale in ("50","100","200","500"):
    for mode in ("body","structure"):
        before=asset["volumes"][scale][mode]
        after=author[scale][mode]
        if before==after:
            continue
        permitted={"trolley-beam-1","trolley-beam-2","winch-motor-grille","joint-jib-heel","joint-counter-heel",
                   "joint-counter-heel-right","member-jib-chord","member-jib-rail-l","member-jib-rail-r",
                   "member-counter-chord-tl","member-counter-chord-tr","member-counter-chord-bl","member-counter-chord-br"}
        changed={q["id"] for q in before+after if q not in before or q not in after}
        if not changed<=permitted:
            raise ValueError("Review newly edited crane parts "+str(changed-permitted))
        if mode=="body" or scale=="500":
            edits.append((["asset","volumes",scale,mode],after))
for scale in ("50","100","200"):
    parts=author[scale]["structure"].copy()
    i=next(i for i,part in enumerate(parts) if part["id"]=="joint-counter-heel")
    if not any(p["id"]=="joint-counter-heel-right" for p in parts):
        pins=[]
        for z,name in ((.64,"joint-counter-heel"),(-.64,"joint-counter-heel-right")):
            pins.append({"id":name,"kind":"revolve","bone":"counter-jib","material":"steel-s355","role":"body","tone":.4,
                         "profile":[[0,.036],[.12,.036]],"axis":[1,0,0],"origin":[.795,.64,z],"member":"counter-jib","part":"joint"})
        parts[i:i+1]=pins
    edits.append((["asset","volumes",scale,"structure"],parts))
shapes=asset["representations"]["50"]["geometry"]["elevation"].copy()
for i,shape in enumerate(shapes):
    if shape["kind"]=="member" and (shape["id"].startswith(("mast-chord-","counter-chord-","jib-rail-")) or shape["id"]=="jib-chord"):
        shapes[i]={**shape,"filled":True,"penMm":.18}
edits.append((["asset","representations","50","geometry","elevation"],shapes))
if "--apply" in sys.argv:
    helpers["write"](path,edits)
    helpers["write"](ticket/"volumes/machinery/tower-crane.json",[(address[1:],value) for address,value in edits if address[1]=="volumes"])
else:
    candidate=ticket/"🗑️generated/equipment-check/crane-candidate.json"
    candidate.write_text(path.read_text())
    helpers["write"](candidate,edits)
