# Runs the registered verify commands of the given sets sequentially from the repo root (same placeholders as staged-merge-tools/plan.ts). usage: verify.py <out dir> <set id>[:<first>-<last>]...
import json, os, subprocess, sys, tempfile, time
ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
out = sys.argv[1]; os.makedirs(out, exist_ok=True)
registry = json.load(open(os.path.join(TICKET, "staged-merge-tools", "sets.json"), encoding="utf-8"))["sets"]
want = {}
for word in sys.argv[2:]:
    name, _, span = word.partition(":")
    low, _, high = span.partition("-")
    want[name] = (int(low), int(high)) if span else (0, 999)
for spec in registry:
    if spec["id"] not in want:
        continue
    for index, item in enumerate(spec.get("verify", [])):
        if not want[spec["id"]][0] <= index <= want[spec["id"]][1]:
            continue
        if item.get("heavy"):
            print(spec["id"], item["name"], "SKIP heavy"); continue
        tmp = tempfile.mkdtemp(prefix="vf-")
        argv = [w.replace("{root}", ROOT).replace("{set}", os.path.join(TICKET, spec["dir"])).replace("{tmp}", tmp) for w in item["argv"]]
        override = os.environ.get("VERIFY_TSCONFIG")
        for at in range(len(argv) - 1):
            if override and argv[at] == "bun" and argv[at + 1] == "test":
                argv[at + 2:at + 2] = ["--tsconfig-override", override]
                break
        log = os.path.join(out, "%s-%02d.log" % (spec["id"], index))
        start = time.time()
        try:
            run = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, timeout=item.get("timeoutSeconds", 600))
            rc, text = run.returncode, run.stdout + run.stderr
        except subprocess.TimeoutExpired as error:
            rc, text = 124, "TIMEOUT\n" + str(error.stdout or "") + str(error.stderr or "")
        open(log, "w", encoding="utf-8").write(text)
        tail = [l for l in text.strip().splitlines() if l.strip()][-4:]
        print("%s / %s: rc=%s (%.0fs, gate=%s)" % (spec["id"], item["name"], rc, time.time() - start, item.get("gate", True)))
        for l in tail: print("    " + l[:200])
        sys.stdout.flush()
