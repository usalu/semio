"""🧹 [isolate] resumable foreground sweep: runs each repo-relative test file of <list> in mirror <name> (plain `bun test`, then the envelope runner when the file asks for the parent invocation), one JSON line per file appended to 🗑️generated/isolate/sweep-<tag>.jsonl (pass, fail, failing names, live-store refusals, seconds); files already in that ledger are skipped; stops starting new files after <budget> seconds. usage: sweep.py <name> <list> <tag> [budget=540] [--only-missing]"""
import json, os, re, subprocess, sys, time
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
name, listing, tag = sys.argv[1:4]
budget = float(sys.argv[4]) if len(sys.argv) > 4 and not sys.argv[4].startswith("-") else 540.0
G = os.path.join(C, "🗑️generated", "isolate"); ledger = os.path.join(G, "sweep-%s.jsonl" % tag)
done = set()
if os.path.exists(ledger):
    for line in open(ledger, encoding="utf-8"): done.add(json.loads(line)["file"])
files = [l.strip() for l in open(listing, encoding="utf-8") if l.strip() and not l.startswith("#")]
start = time.time(); env = dict(os.environ, SEMIO_MIRROR_HOME=os.environ.get("SEMIO_MIRROR_HOME", "/private/tmp/m"))
def run(f, mode, i):
    t = "%s-%03d-%s" % (tag, i, mode); began = time.time()
    try: subprocess.run(["sh", os.path.join(C, "isolate-tools", "run.sh"), name, mode, f, t], env=env, capture_output=True, text=True, timeout=900)
    except subprocess.TimeoutExpired: pass
    log = open(os.path.join(G, "log", t + ".log"), encoding="utf-8", errors="replace").read()
    nums = dict((k, int(v)) for v, k in re.findall(r"^ *(\d+) (pass|fail|skip)$", log, re.M))
    fails = sorted(set(l[len("(fail) "):].rsplit(" [", 1)[0] for l in log.splitlines() if l.startswith("(fail)")))
    return {"file": f, "mode": mode, "pass": nums.get("pass", 0), "fail": nums.get("fail", 0), "skip": nums.get("skip", 0), "fails": fails, "refusals": log.count("reached the live repository store"), "envelope": "Original parent Script process invocation required" in log, "loaderror": "error: " in log and not nums, "seconds": round(time.time() - began, 1), "log": t}
for i, f in enumerate(files):
    if f in done: continue
    if time.time() - start > budget: print("budget reached; rerun to resume"); break
    if not os.path.exists(os.path.join(env["SEMIO_MIRROR_HOME"], name, f)): row = {"file": f, "absent": True}
    else:
        row = run(f, "plain", i)
        if row["envelope"]: row = run(f, "env", i)
    with open(ledger, "a", encoding="utf-8") as out: out.write(json.dumps(row, ensure_ascii=False) + "\n")
    print("%-5s %3s/%-3s r%-3s %6.1fs %s" % (row.get("mode", "-"), row.get("pass", "-"), row.get("fail", "-"), row.get("refusals", "-"), row.get("seconds", 0), f[-90:]), flush=True)
left = [f for f in files if f not in done and not any(json.loads(l)["file"] == f for l in open(ledger, encoding="utf-8"))] if os.path.exists(ledger) else files
print("remaining", len(left))
