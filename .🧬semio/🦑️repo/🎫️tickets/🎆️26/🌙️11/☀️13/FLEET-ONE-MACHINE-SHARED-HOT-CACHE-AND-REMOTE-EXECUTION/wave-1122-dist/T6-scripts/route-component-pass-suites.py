"""🧭️ Routes the split component-pass test files (T6): one suite per file, reclaim and cancellation lose their name filters, a `test-<id>` target per new suite."""
import json, os, sys

PROCESS = sys.argv[1]
TABLE = os.path.join(PROCESS, "🧪️testing", "🗂️suites", "🔣️.json")
PROJECT = os.path.join(PROCESS, "📋️project.json")
BASE = "📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/"
NEW = {"native-component-streaming": "🌊️streaming", "native-component-jobs": "🏃️jobs", "native-component-retry": "🔁️retry", "native-component-seal": "🔏️seal",
       "native-component-closure": "🧩️closure", "native-component-failures": "💥️failures", "native-component-real": "🦀️real"}
MOVED = {"component-reclaim": "♻️reclaim", "native-queued-cancellation": "🛑️cancellation"}

table = json.load(open(TABLE, encoding="utf8"))
suites = table["suites"]
by_id = {row["id"]: row for row in suites}
launches = by_id["native-artifacts"]["launches"]
for suite, folder in MOVED.items():
    row = by_id[suite]
    row["files"] = [BASE + folder + "/🟦️.ts"]
    row.pop("testNamePattern", None)
at = suites.index(by_id["native-artifacts"]) + 1
for offset, (suite, folder) in enumerate(NEW.items()):
    assert suite not in by_id
    suites.insert(at + offset, {"id": suite, "budgetMs": 300000, "typecheck": "none", "files": [BASE + folder + "/🟦️.ts"], "platforms": ["darwin", "linux", "win32"], "launches": launches})
open(TABLE + ".t6tmp", "w", encoding="utf8").write(json.dumps(table, indent=2, ensure_ascii=False) + "\n")

project = json.load(open(PROJECT, encoding="utf8"))
targets = project["targets"]
template = targets["test-native-artifacts"]
rebuilt = {}
for name, target in targets.items():
    rebuilt[name] = target
    if name == "test-native-artifacts":
        for suite in NEW:
            row = json.loads(json.dumps(template))
            row["options"]["command"] = "repo ./📜️script.ts test " + suite
            rebuilt["test-" + suite] = row
project["targets"] = rebuilt
open(PROJECT + ".t6tmp", "w", encoding="utf8").write(json.dumps(project, indent=2, ensure_ascii=False) + "\n")
os.replace(TABLE + ".t6tmp", TABLE)
os.replace(PROJECT + ".t6tmp", PROJECT)
print(len(suites), len(rebuilt))
