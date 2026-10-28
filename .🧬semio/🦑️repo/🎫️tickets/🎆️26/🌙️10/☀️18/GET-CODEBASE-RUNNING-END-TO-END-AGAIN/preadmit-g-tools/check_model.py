"""🧮️ The extended Python admission model (the repo oracle `🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py`: ram-g's model plus ramp slots plus quiet requests and upgrades) must still equal every stored case of the earlier TypeScript corpus (hand cases, 160 seeded cases, ramp cases and ramp corpus): the new rules change nothing for a world without quiet or upgrade requests. Usage: python3 check_model.py [root]"""
import importlib.util,json,os,sys
ROOT=sys.argv[1] if len(sys.argv)>1 else "/Users/ueli/Documents/semio"
HC=f"{ROOT}/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
spec=importlib.util.spec_from_file_location("oracle",f"{HC}/🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py");oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
fixture=json.load(open(f"{HC}/🧪️tests/🧫️fixtures/🔣️.json",encoding="utf8"));bad=0;total=0
for key in ("cases","corpus","rampCases","rampCorpus"):
    for item in fixture[key]:
        total+=1;got=oracle.evaluate(fixture["policy"],item["host"],item["ledger"],item["queue"],item["nowMs"])
        if got!=item["expect"]:
            bad+=1
            if bad<4:print("DIFF",item["name"],json.dumps(got)[:300],json.dumps(item["expect"])[:300])
print("cases",total,"diffs",bad)
sys.exit(1 if bad else 0)
