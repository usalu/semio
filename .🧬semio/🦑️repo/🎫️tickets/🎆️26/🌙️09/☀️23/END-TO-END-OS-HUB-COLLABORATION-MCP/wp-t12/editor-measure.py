"""📏️ Runs every authored editor-lane vector through the ticket-local harness (production dispatch + computed inverse)
and records the measured `diff` and `messages` into `wp-t12/editor-vectors.json`. A vector whose produced snapshot
differs from its hand-written `after`, whose inverse does not restore its `before`, or whose declared status disagrees
with the produced diagnostics is reported as a finding and left unmeasured, never overwritten."""
import json, os, subprocess, sys, tempfile

root = "/Users/ueli/Documents/semio/"
path = root + ".tmp-ticket/wp-t12/editor-vectors.json"
harness = os.environ.get("T12_HARNESS", root + ".tmp-ticket/wp-t12/target/debug/t12-vector-harness")
spec = json.load(open(path, encoding="utf-8"))
only = {a for a in sys.argv[1:] if a.isdigit()}
findings = []


def normal(value):
    if isinstance(value, bool) or value is None or isinstance(value, str): return value
    if isinstance(value, (int, float)): return float(value)
    if isinstance(value, list): return [normal(v) for v in value]
    return {k: normal(v) for k, v in value.items()}


for index, entry in spec.items():
    if only and index not in only: continue
    for s in entry["scenarios"]:
        with tempfile.TemporaryDirectory() as tmp:
            before, mutation = os.path.join(tmp, "before.json"), os.path.join(tmp, "mutation.json")
            json.dump(s["before"], open(before, "w"))
            json.dump(s["mutation"], open(mutation, "w"))
            run = subprocess.run([harness, "run", index, before, mutation], capture_output=True, text=True)
        where = f"[{index}] {s['kind']}/{s['status']}"
        if run.returncode != 0:
            findings.append(f"{where}: harness refused — {run.stderr.strip()[:400]}")
            continue
        report = json.loads(run.stdout)
        levels = [m.get("level") for m in report["messages"]]
        codes = [m.get("code") for m in report["messages"]]
        problems = []
        if normal(report["after"]) != normal(s["after"]):
            problems.append(f"produced after {json.dumps(report['after'])[:300]} ≠ hand-written {json.dumps(s['after'])[:300]}")
        if normal(report["restored"]) != normal(report["before"]):
            problems.append(f"inverse restored {json.dumps(report['restored'])[:200]} ≠ before")
        refused = any(level in ("error", "fatal") for level in levels)
        if s["status"] == "applied" and (refused or "mutation.no-op" in codes):
            problems.append(f"declared applied, produced {codes} at {levels}")
        if s["status"] == "no-op" and codes != ["mutation.no-op"]:
            problems.append(f"declared no-op, produced {codes} at {levels}")
        if problems:
            findings.append(f"{where}: " + "; ".join(problems))
            s.pop("diff", None)
            s.pop("messages", None)
            continue
        s["diff"] = report["diff"]
        s["messages"] = [{"level": m.get("level"), "code": m.get("code")} for m in report["messages"]]
json.dump(spec, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
measured = sum(1 for e in spec.values() for s in e["scenarios"] if "diff" in s)
total = sum(len(e["scenarios"]) for e in spec.values())
print(f"measured {measured}/{total}; {len(findings)} finding(s)")
for f in findings: print("FINDING", f)
