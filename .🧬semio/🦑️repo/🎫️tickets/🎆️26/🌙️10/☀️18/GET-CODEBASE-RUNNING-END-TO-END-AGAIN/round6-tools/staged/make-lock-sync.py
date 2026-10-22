#!/usr/bin/env python3
"""Lock-sync plan (closure-14 -- the `createRequire(join(root,"package.json")),parsers=` loader -- is dropped when the parser witness no longer has that loader: it reads installed package directories of the root scope only, so the scoped-root intent is moot); for the round-6 tree: the round-5 batch + the ide-free manifest rows (merge-lock-sync.py), with the closure edit of the build script re-anchored on the text the cli-latency profile hooks leave (profilePhaseV1 around the ensure call). Writes staged/lock-sync-plan.json."""
import json, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

subprocess.run([sys.executable, C + "/ide-free-tools/merge-lock-sync.py"], check=True, capture_output=True)
plan = json.load(open(C + "/ide-free-tools/staged/lock-sync-plan-merged.json", encoding="utf8"))
OLD = 'if(buildOperationRequiresV1(command)==="dependencies"){const {ensureBuildEnvironmentV1}=await import("./🚀️bootstrap/🟦️.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}'
NEW = 'if(buildOperationRequiresV1(command)==="dependencies"){const {ensureBuildEnvironmentV1,buildEnvironmentPlanV1}=await import("./🚀️bootstrap/🟦️.ts"),environmentPlan=buildEnvironmentPlanV1(root,selected.tool,process.env);await ensureBuildEnvironmentV1(root,invocation,phaseControl,environmentPlan.install,undefined,environmentPlan.scope);}'
HOOK_OLD = 'if(buildOperationRequiresV1(command)==="dependencies"){const {ensureBuildEnvironmentV1}=await profilePhaseV1("dependencies.ensure-load",()=>import("./🚀️bootstrap/🟦️.ts"));await profilePhaseV1("dependencies.ensure",()=>ensureBuildEnvironmentV1(root,invocation,phaseControl));}'
HOOK_NEW = 'if(buildOperationRequiresV1(command)==="dependencies"){const {ensureBuildEnvironmentV1,buildEnvironmentPlanV1}=await profilePhaseV1("dependencies.ensure-load",()=>import("./🚀️bootstrap/🟦️.ts")),environmentPlan=buildEnvironmentPlanV1(root,selected.tool,process.env);await profilePhaseV1("dependencies.ensure",()=>ensureBuildEnvironmentV1(root,invocation,phaseControl,environmentPlan.install,undefined,environmentPlan.scope));}'
hits = [e for f in plan["files"] for e in f["edits"] if e["id"] == "closure-closure-3" and "ensureBuildEnvironmentV1" in e["old"]]
assert len(hits) == 1 and hits[0]["old"] == OLD and hits[0]["new"] == NEW
hits[0]["old"], hits[0]["new"] = HOOK_OLD, HOOK_NEW
_dropped = 0
for _f in plan["files"]:
    _src = os.path.join(repo_root(), _f["path"])
    _text = open(_src, encoding="utf8").read() if os.path.isfile(_src) else ""
    _keep = []
    for _e in _f["edits"]:
        if (_e["id"] == "closure-closure-14" or "createRequire" in _e["old"]) and _e["old"] not in _text and _e["new"] not in _text:
            _dropped += 1
            continue
        _keep.append(_e)
    _f["edits"] = _keep
print("closure-14 dropped:", _dropped)
json.dump(plan, open(HERE + "/lock-sync-plan.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("lock-sync plan:", HERE + "/lock-sync-plan.json")
