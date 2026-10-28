#!/usr/bin/env python3
"""🐍 Edits of batch `fleetpy` applied in place to a root: python3 -I edit.py <root>. The fleet-status law asks the test toolchain for the Python 3 interpreter (`python3`, `python` or `py -3`, a missing one fails loudly in EN/DE) instead of spawning the literal `python3` that a Windows host does not have, and the suite gets its explicit platform verdict (Windows-capable: no host-bound fact left)."""
import os, sys
ROOT = sys.argv[1]
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
def edit(rel, pairs):
    path = os.path.join(ROOT, rel); text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if new in text: continue
        if text.count(old) != 1: sys.exit("anchor %d× in %s: %s" % (text.count(old), rel, old[:70]))
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8").write(text)
edit(B + "/🚢️fleet-status/🧪️tests/🟦️.ts", [
 ('import {fleetArgumentsV1,runFleetCliV1} from "../⌨️command/🟦️.ts";\n',
  'import {fleetArgumentsV1,runFleetCliV1} from "../⌨️command/🟦️.ts";\nimport {requireTestToolV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";\n'),
 (' const result=spawnSync("python3",[oracle,"generate",String(seed),String(count)],{encoding:"utf8",maxBuffer:1<<28});',
  ' const result=spawnSync(requireTestToolV1("python"),[oracle,"generate",String(seed),String(count)],{encoding:"utf8",maxBuffer:1<<28});'),
])
edit(B + "/🗂️suites/🔣️.json", [
 ('      "id": "build-fleet-status",\n      "budgetMs": 300000,\n      "typecheck": "none",\n      "trees": [\n        "🚢️fleet-status"\n      ]\n    },',
  '      "id": "build-fleet-status",\n      "budgetMs": 300000,\n      "typecheck": "none",\n      "trees": [\n        "🚢️fleet-status"\n      ],\n      "platforms": [\n        "darwin",\n        "linux",\n        "win32"\n      ]\n    },'),
])
print("fleetpy edits applied to", ROOT)
