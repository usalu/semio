import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import din16798 as d
v = d.vectors()
for r in v["pmv"]: print(r)
for r in v["inversion"]:
    if r["rustClampedToBracket"] or not r["referenceInBracket"] or (r["referenceC"] and abs(r["referenceC"]-r["rustTranscriptC"])>1e-2): print(r)
for r in v["bandEdges"]: print(r["season"], r["category"], r["edge"], r["t_op"], round(r["pmv"],3), r["pmvLimit"])
for r in v["co2"]: print(r)
print(v["saturation"][:3])
