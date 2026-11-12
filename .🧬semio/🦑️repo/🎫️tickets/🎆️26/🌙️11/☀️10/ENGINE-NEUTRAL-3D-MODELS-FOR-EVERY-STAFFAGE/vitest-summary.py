"""Summarises a vitest JSON report: counts per status and the sorted `file > test` lines of every failed test; with a second report, prints the failures that exist only in the first."""
import json
import sys


def load(path):
    report = json.load(open(path, encoding="utf8"))
    counts = {"passed": 0, "failed": 0, "skipped": 0, "other": 0}
    failed = set()
    for file in report["testResults"]:
        name = file["name"].split("/🧪️tests/")[-1]
        for test in file["assertionResults"]:
            status = test["status"]
            counts[status if status in counts else "other"] += 1
            if status == "failed":
                failed.add(f"{name} > {test['fullName']}")
        if file["status"] == "failed" and not file["assertionResults"]:
            failed.add(f"{name} > (file failed to load) {file.get('message', '')[:120]}")
    return counts, failed


counts, failed = load(sys.argv[1])
print(counts, "files", len(json.load(open(sys.argv[1], encoding="utf8"))["testResults"]))
if len(sys.argv) > 2:
    _, other = load(sys.argv[2])
    print("only in first:", *sorted(failed - other), sep="\n  ")
    print("only in second:", *sorted(other - failed), sep="\n  ")
else:
    print(*sorted(failed), sep="\n  ")
