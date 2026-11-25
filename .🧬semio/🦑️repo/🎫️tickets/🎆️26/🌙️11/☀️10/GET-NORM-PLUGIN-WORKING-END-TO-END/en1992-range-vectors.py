import json, shutil, sys
fixtures = sys.argv[1]
for leaf in ("⭕️change-bar-layer-lap-share", "💧change-member-bearing-width"):
    for role in ("before", "after"):
        shutil.copyfile(f"{fixtures}/{leaf}/✅apply/🔣️.before.json", f"{fixtures}/{leaf}/⛔range/🔣️.{role}.json")
    outcome = {"status": "rejected", "code": "mutation.invariant", "messages": [{"level": "fatal", "code": "mutation.invariant"}]}
    open(f"{fixtures}/{leaf}/⛔range/🔣️.outcome.json", "w", encoding="utf-8", newline="\n").write(json.dumps(outcome, indent=2, ensure_ascii=False) + "\n")
print("ok")
