#!/usr/bin/env python3
"""H83: anchored, atomic patch of the fleet-status tests (CLI expectations for the summary default, new size, parity, filter and command tests). Usage: tests.py <fleet-status dir> <tests-append.ts>"""
import os, sys
module, append = sys.argv[1:3]
path = os.path.join(module, "🧪️tests", "🟦️.ts")
text = open(path, encoding="utf-8").read()
if "size law:" in text:
    sys.exit("already applied")
def once(old, new, count=1):
    global text
    assert text.count(old) == count, (old[:70], text.count(old))
    text = text.replace(old, new)
once('import {fleetArgumentsV1,runFleetCliV1} from "../⌨️command/🟦️.ts";', 'import {fleetArgumentsV1,runFleetCliV1} from "../⌨️command/🟦️.ts";\nimport {filterFleetReportV1,fleetOriginsV1,formatFleetSummaryV1,renderFleetJsonV1,selectFleetSectionsV1,summarizeFleetReportV1,validateFleetSelectionV1,validateFleetSummaryV1} from "../🟦️.ts";')
once(' const json=await run(["status","--json"]);', ' const json=await run(["status","--json","--detail"]);')
once('const text=await run(["status","--limit=1"],"de");', 'expect((await run(["status"],"de")).lines.some(line=>line.includes("Builder pid 21"))).toBe(true);\n const text=await run(["status","--detail","--limit=1"],"de");')
once('{json:true,limit:5}', '{json:true,limit:5,detail:false}', 2)
once('{json:true,limit:20}', '{json:true,limit:20,detail:false}')
text = text.rstrip("\n") + "\n" + open(append, encoding="utf-8").read()
worker = os.path.join(module, "🧪️tests", "🛡️guard", "📥️worker", "🟦️.ts")
body = open(worker, encoding="utf-8").read()
assert body.count('["status","--json"]') == 1
for file, content in ((path, text), (worker, body.replace('["status","--json"]', '["status","--json","--detail"]').replace("runs `repo fleet status --json`", "runs `repo fleet status --json --detail`"))):
    tmp = file + ".h83.tmp"
    open(tmp, "w", encoding="utf-8").write(content)
    os.replace(tmp, file)
print("ok")
