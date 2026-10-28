#!/usr/bin/env python3
"""📊 Prints one line per module and step of the given run summaries and exits 1 when any step failed: python3 -I summary.py <summary.json>..."""
import json, sys
bad = 0
for path in sys.argv[1:]:
    data = json.load(open(path, encoding="utf8"))
    print(f"== {data['label']} (scratch {data['scratch']})")
    for row in data["results"]:
        name = "/".join(row["module"].split("/")[-3:])
        if "plan_error" in row:
            print(f"  PLAN-ERROR {name}: {row['plan_error']}")
            bad += 1
        for step in row["steps"]:
            counts = step.get("counts")
            detail = f" top {counts['top_pass']}/{counts['top_fail']}/{counts['top_skip']}" if counts else ""
            print(f"  {'ok  ' if step['exit'] == 0 else 'FAIL'} {name} {step['step']} {step['seconds']}s{detail}")
            for failing in step.get("failing", []):
                if "/" not in failing["test"]:
                    print(f"       red: {failing['test']}")
            bad += step["exit"] != 0
sys.exit(1 if bad else 0)
