"""CVF-VMC scratch: U-values (thermal uValue semantics) of conditioned regions and operating/emitter summary of my 14 assets."""
import json,glob
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
for f in sorted(glob.glob(S+"*/*/🔣️.json")):
    a=json.load(open(f))["asset"]
    if a["id"] not in "city-car city-bus cargo-bike excavator tower-crane forklift cantilever-balcony glulam-beam steel-column stair-flight sliding-door tilt-window glass-balustrade tied-canopy".split(): continue
    for r in a["thermal"]["regions"]:
        if r.get("conditioned"):
            R=sum(l["resistanceM2KW"] for l in r["layers"])
            print(a["id"],r["id"],"U=%.2f"%(1/(R+r["conditioned"]["interiorFilmM2KW"]+0.04)),"Tint",r["conditioned"]["interiorTempC"],"layers",[l["id"] for l in r["layers"]])
