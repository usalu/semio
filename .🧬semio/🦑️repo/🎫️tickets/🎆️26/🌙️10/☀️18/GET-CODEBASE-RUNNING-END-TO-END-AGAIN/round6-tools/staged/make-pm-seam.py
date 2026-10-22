#!/usr/bin/env python3
"""Copy of the pm-seam set: the round-2 plan drops its edits of files that no longer exist in the tree (the closure test `🚀️bootstrap/🧪️tests/🎯️closure` was removed by the loader/closure change; its only edit swapped the install-schema import for the facade, so the intent is moot with the file)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/pm-seam-tools", "pm-seam")
import shutil
shutil.copyfile(C + "/winpath.py", HERE + "/winpath.py")
root = repo_root()
plans = {n: json.load(open(os.path.join(T, n, "plan.json"), encoding="utf8")) for n in ("staged", "staged2", "staged3") if os.path.isfile(os.path.join(T, n, "plan.json"))}
created = {row.get("path") for plan in plans.values() for row in plan.get("newFiles", [])}
for name, plan in plans.items():
    keep = [row for row in plan.get("files", []) if not (row.get("path") and "\U0001F9EA\ufe0ftests/\U0001F3AF\ufe0fclosure/" in row["path"] and not os.path.exists(os.path.join(root, row["path"])) and row["path"] not in created)]
    dropped = len(plan.get("files", [])) - len(keep)
    if dropped:
        plan["files"] = keep
        json.dump(plan, open(os.path.join(T, name, "plan.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(name, "dropped", dropped)

ROW_OLD = '    "test-javascript-installed-paths": {\n'
ROW_ANCHOR = '    "test-javascript-installed-paths": {"executor"'
CAT_ROW = '    "test-package-manager-seam": {"executor":"repo:script","cache":false,"outputs":[],"dependsOn":[],"options":{"cwd":"\U0001F9F0️framework/\U0001F6CD️products/\U0001F991️repo/\U0001F528️modules/\U0001F4DA️library/⚡️caching","command":"bun ./\U0001F4DC️script.ts test package-manager-seam","forwardAllArgs":false},"metadata":{"semio":{"execution":{"version":1,"kind":"source"},"cache":{"version":1,"class":"live","reason":"exercises-host-resources"}}}},\n'
for name in ("staged", "staged2", "staged3"):
    pp = os.path.join(T, name, "plan.json")
    if not os.path.isfile(pp):
        continue
    plan = json.load(open(pp, encoding="utf8"))
    hit = 0
    for row in plan.get("files", []):
        for e in row.get("edits", []):
            if e.get("old") == ROW_OLD:
                e["old"] = ROW_ANCHOR
                e["new"] = CAT_ROW + ROW_ANCHOR
                hit += 1
    if hit:
        json.dump(plan, open(pp, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(name, "register-target rows re-based:", hit)

def retarget(plan_name, edit_id, fn):
    pp = os.path.join(T, plan_name, "plan.json")
    plan = json.load(open(pp, encoding="utf8"))
    hit = 0
    for row in plan.get("files", []):
        for e in row.get("edits", []):
            if e.get("id") == edit_id:
                fn(e)
                hit += 1
    assert hit == 1, (plan_name, edit_id, hit)
    json.dump(plan, open(pp, "w", encoding="utf8"), ensure_ascii=False, indent=1)


def count_j(e):
    assert e["counts"] == [36] and e["old"] == '"j"'
    e["counts"] = [37]


retarget("staged2", "driver-generation-directory", count_j)


def runtime_fields(e):
    assert e["old"] == "left.bun===right.bun"
    e["old"], e["new"] = 'runtimeFields=["bun","revision"', 'runtimeFields=["manager","revision"'


retarget("staged2", "generation-runtime-compare", runtime_fields)
print("pm-seam copy:", T)
