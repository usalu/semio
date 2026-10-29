import json,subprocess,sys
# usage: mkgeo.py <id> ; merges design/<id>-hand.json (hand-authored shapes) with texture mark sets listed in design/<id>-tex.txt (args of texmass.py per line)
i=sys.argv[1]; shapes=json.load(open(f"design/{i}-hand.json"))
try:
    for line in open(f"design/{i}-tex.txt"):
        if line.strip(): shapes+= [json.loads(l) for l in subprocess.check_output(["python3","texmass.py"]+line.split(),text=True).splitlines()]
except FileNotFoundError: pass
import os
if os.path.exists(f"design/{i}-fronds.json"): shapes+=json.load(open(f"design/{i}-fronds.json"))
shapes.sort(key=lambda s:0)  # keep order
open(f"design/{i}-geo.json","w").write('{"geometry":{"elevation":[\n'+",\n".join(json.dumps(s,separators=(",",":")) for s in shapes)+"\n]}}\n")
