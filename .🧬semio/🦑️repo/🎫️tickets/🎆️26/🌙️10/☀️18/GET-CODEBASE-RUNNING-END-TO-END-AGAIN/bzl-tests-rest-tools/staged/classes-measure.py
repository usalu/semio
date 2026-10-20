#!/usr/bin/env python3
"""Turns measured Bazel test runs into class proposals. Reads Bazel build event files (`--build_event_json_file`, newline-delimited JSON) and the test class table; a run of a test is the slowest attempt over its shards, a label needs `minimumRuns` runs, its size is the smallest whose seconds cover `headroom` times the p95 of those runs. Prints JSON: one row per label and the rules that would change the table (evidence `measured`). Usage: measure.py --table classes.json [--workspace DIR] [--minimum-runs N] [--shardable ID,ID] BEP.json..."""
import hashlib, json, math, os, re, sys

def parse(argv):
    options = {"table": None, "workspace": None, "minimum-runs": None, "shardable": ""}
    files = []
    index = 0
    while index < len(argv):
        if argv[index].startswith("--"):
            options[argv[index][2:]] = argv[index + 1]
            index += 2
        else:
            files.append(argv[index])
            index += 1
    return options, files

def percentile(values, fraction):
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, max(0, math.ceil(fraction * len(ordered)) - 1))]

def runs(files):
    seen = {}
    for number, path in enumerate(files):
        slowest = {}
        for line in open(path, encoding="utf-8"):
            event = json.loads(line)
            result = event.get("testResult")
            if result is None or result.get("status") not in ("PASSED", "FLAKY"):
                continue
            ident = event["id"]["testResult"]
            key = (ident["label"], ident.get("run", 1))
            slowest[key] = max(slowest.get(key, 0), int(result.get("testAttemptDurationMillis", "0")))
        for (label, _run), millis in slowest.items():
            seen.setdefault(label, []).append(millis / 1000.0)
    return seen

def main():
    options, files = parse(sys.argv[1:])
    table = json.load(open(options["table"], encoding="utf-8"))
    measure = table["measure"]
    minimum = int(options["minimum-runs"] or measure["minimumRuns"])
    shardable = set(filter(None, options["shardable"].split(",")))
    sizes = sorted(table["sizes"].items(), key=lambda row: row[1]["seconds"])
    identity = {}
    if options["workspace"]:
        for label in runs(files):
            path = os.path.join(options["workspace"], label.split(":")[-1] + ".json")
            if os.path.exists(path):
                task = json.load(open(path, encoding="utf-8")).get("task", {})
                identity[label] = {"id": task.get("id"), "project": task.get("project"), "target": task.get("target")}
    rows, proposal = [], []
    for label, seconds in sorted(runs(files).items()):
        p95 = percentile(seconds, 0.95)
        size = next((name for name, row in sizes if row["seconds"] >= measure["headroom"] * p95), sizes[-1][0])
        row = {"label": label, "runs": len(seconds), "p95Seconds": round(p95, 3), "size": size, **identity.get(label, {})}
        rows.append(row)
        if len(seconds) < minimum or "target" not in row or row["target"] is None:
            continue
        rule = {"id": "measured-" + re.sub(r"[^a-z0-9]+", "-", row["target"].lower()).strip("-")[:40] + "-" + hashlib.sha256(row["project"].encode()).hexdigest()[:6], "evidence": "measured", "reason": "p95 %.1f s over %d runs" % (p95, len(seconds)), "targets": [row["target"]], "projects": [row["project"]], "size": size}
        if row["id"] in shardable and p95 > measure["shardAboveSeconds"]:
            rule["shards"] = min(8, math.ceil(p95 / (measure["shardAboveSeconds"] / 2)))
        proposal.append(rule)
    json.dump({"rows": rows, "proposal": proposal}, sys.stdout, ensure_ascii=False, indent=1)

if __name__ == "__main__":
    main()
