"""🔎️ Lists recent oracle-phase failures, flagging those whose diagnostics mention an outcome status or no-op."""
import json, glob, os, time, collections, sys
base = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tests/results/"
since = time.time() - float(sys.argv[1] if len(sys.argv) > 1 else 7200)
flag, other = [], collections.Counter()
for f in glob.glob(base + "*oracle*/📤️results.jsonl"):
    if os.path.getmtime(f) < since: continue
    for line in open(f):
        r = json.loads(line)
        if r.get("status") not in ("failed", "errored"): continue
        msg = " | ".join((d.get("message") or "") + " " + (d.get("detail") or "")[-400:] for d in r.get("diagnostics", []))
        if any(k in msg for k in ("no-op", "'applied'", '"applied"', "status", "outcome")): flag.append((r["case"], r["scenario"], msg[:500].replace("\n", " ")))
        else: other[(r["case"], msg[:100].replace("\n", " "))] += 1
print("FLAGGED", len(flag))
for x in flag: print(" ", x)
print("OTHER", sum(other.values()))
for k, v in other.most_common(400): print(" ", v, k)
