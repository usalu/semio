"""🧪️ Checks the Python admission model against the stored TypeScript corpus (hand cases and 160 seeded cases computed by the SQLite oracle)."""
import json,os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from admission_model import evaluate
FIXTURE="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json"
f=json.load(open(FIXTURE));bad=0
for item in f["cases"]+f["corpus"]+f["rampCases"]+f["rampCorpus"]:
    got=evaluate(f["policy"],item["host"],item["ledger"],item["queue"],item["nowMs"])
    if got!=item["expect"]:
        bad+=1
        if bad<4: print("DIFF",item["name"],json.dumps(got)[:300],json.dumps(item["expect"])[:300])
print("cases",len(f["cases"])+len(f["corpus"])+len(f["rampCases"])+len(f["rampCorpus"]),"diffs",bad)
sys.exit(1 if bad else 0)
