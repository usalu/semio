"""🐾 Produce source-hash presentation and numeric thermal pose sheets for the current fourteen animals."""
from hashlib import sha256
from pathlib import Path
import json
import os
import shutil
import subprocess
import sys

ticket=Path(__file__).resolve().parent
generated=ticket/"🗑️generated"
views=generated/"living-current-views"
catalog=next(ticket.parents[6].glob("🎓*/*architecture/*staffage/*catalog/*animals"))
secondary={"dairy-cow":"lying-resting","domestic-cat":"sitting-tail-wrapped","goat":"browsing-upright","hedgehog":"rolled","herring-gull":"gliding","horse":"grazing-head-down","labrador-dog":"sitting-waiting","mallard-duck":"upending","pig":"lying-wallowing","pigeon":"wings-half-open","red-fox":"pounce-flight","red-squirrel":"bound-extension","roe-deer":"bounding-gathered","sheep":"grazing"}
motion={"dairy-cow":"walk-phase-3","domestic-cat":"stalk-phase-3","goat":"climbing-ledge","hedgehog":"walk-3","herring-gull":"touchdown","horse":"walk-phase-3","labrador-dog":"trot-phase-3","mallard-duck":"paddle-3","pig":"walk-phase-3","pigeon":"takeoff-liftoff","red-fox":"trot-phase-3","red-squirrel":"running","roe-deer":"bound-phase-3","sheep":"walk-phase-3"}
for cat in sorted(catalog.glob("*/*.json")):
    name=cat.parent.name
    if sys.argv[1:] and name not in sys.argv[1:]:continue
    raw=cat.read_bytes()
    digest=sha256(raw).hexdigest()[:10]
    a=json.loads(raw)["asset"]
    for pose in (a["presentation"]["defaultPose"],secondary[name],motion[name]):
        prefix=f"{name}-{pose}-all-presentation-shaded-temperature-{digest}"
        target=views/(prefix+".png")
        if not target.exists():
            argv=[sys.executable,ticket/"equipment-guard.py",shutil.which("bun"),"--smol",ticket/"living-visual.ts",name,"all",pose,cat,"--presentation","--shaded","--temperature"]
            with (generated/("atlas-"+name+".log")).open("w") as log:
                subprocess.run(argv,env=dict(os.environ,EQUIPMENT_HEADROOM_PERCENT="42",EQUIPMENT_RSS_MIB="470"),stdout=log,stderr=log,check=True)
        if raw!=cat.read_bytes():raise RuntimeError(name+" changed during review")
        print(target.name,flush=True)
