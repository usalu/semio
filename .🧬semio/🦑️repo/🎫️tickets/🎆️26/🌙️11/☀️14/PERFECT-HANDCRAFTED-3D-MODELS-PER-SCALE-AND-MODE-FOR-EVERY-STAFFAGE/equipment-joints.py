"""🧱 Separate manufactured parts at their physical joints, preserving each assembly's nominal module."""
from pathlib import Path
import runpy
ticket=Path(__file__).resolve().parent
helpers=runpy.run_path(str(ticket/"equipment-sections.py"))
for name in ("masonry-wall-with-lintel","sliding-door","sectional-garage-door"):
    path,asset=helpers["document"](name)
    parts=[part.copy() for part in asset["volumes"]["50"]["body"]]
    for part in parts:
        if name=="masonry-wall-with-lintel" and part["material"]=="block" and part.get("repeat",{}).get("translation")==[.25,0,0]:
            part["size"]=[.248,*part["size"][1:]]
        elif name=="sliding-door" and part["id"]=="track-slot-brush":
            part["size"]=[3.084,.016,.011]
        elif name=="sectional-garage-door" and part["id"]=="coil":
            part["profile"]=[[round(x*.975,7),r] for x,r in part["profile"]] if max(x for x,r in part["profile"])==.004 else part["profile"]
    for file,prefix in [(path,["asset","volumes"]),(ticket/"volumes"/asset["category"]/(name+".json"),["volumes"])]:
        helpers["write"](file,[(prefix+["50","body"],parts)])

