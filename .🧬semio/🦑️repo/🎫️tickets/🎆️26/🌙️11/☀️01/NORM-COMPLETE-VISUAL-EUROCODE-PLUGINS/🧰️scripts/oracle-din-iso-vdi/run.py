import sys, json, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import din4108
ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures"
d = din4108.main(f"{ART}/🧮️u-values/🔣️.json", "/tmp/claude-1000/-home-deck-semio/6b110249-de72-4741-9001-cd99bf561284/scratchpad/din4108.json")
for r in d["uValues"]: print(r["id"], round(r["rTotal"],5), round(r["rustRTotal"],5), f'{r["rustDeviationRelative"]:+.4%}', "ratio", round(r["ratioUpperLower"],3), r.get("fixtureAgrees"))
for r in d["fRsi"]: print(r["id"], round(r["fRsiRust"],3), round(r["fRsiStandard"],3), r["rustPasses"], r["standardPasses"])
print(d["fRsiCritical"])
for g in d["glaser"]: print(g["id"], g["reference"], g["referenceSdFloor0_1"]["condensedKgM2"], g["referenceSdFloor0_1"]["evaporatedKgM2"], g["referenceIapws"]["condensedKgM2"], g["rustTranscript"])
