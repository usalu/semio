#!/usr/bin/env python3
"""Runs the deferred round-2 verifications that need neither `bun repo`, Bazel nor Cargo, strictly one suite at a time (foreground, wall limit per suite, resumable).
usage: verify-deferred.py <plan> [--budget SECONDS] [--cap SECONDS]
  plans: law (testfix-law edited suites, plain bun test with the repo env), real21 / env168 (testfix-defect lists through the genuine-birth harness), lawred (the red suites of `law` again through the harness, which supplies the process envelope), misc (envelope suites of sync-incr, bzl-default, bzl-rbe, pg-chain)
  stops (exit 4) when load > 25 or free+inactive memory < 20 %; exit 3 when the budget ran out before the list ended; results append to C/🗑️generated/round2-apply/deferred-run-<plan>.jsonl and finished files are skipped on the next call."""
import json, os, re, subprocess, sys, tempfile, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
GEN = C + "/🗑️generated/round2-apply"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
D = L + "/⚡️caching/🚀️bootstrap/📦️dependencies"
args = sys.argv[1:]
plan = args[0]
budget = int(args[args.index("--budget") + 1]) if "--budget" in args else 300
cap = args[args.index("--cap") + 1] if "--cap" in args else "240"


def lines(path):
    return [l.rstrip("\n") for l in open(path, encoding="utf8") if l.strip()]


MISC = [("tfd", D + "/💾️storage/🧪️tests/🟦️.ts"), ("tfd", D + "/🧪️tests/🟦️.ts"), ("tfd", D + "/🧾️incremental/🧪️tests/🟦️.ts"), ("tfd", B + "/🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts"),
        ("tfd", B + "/🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts"), ("tfd", B + "/🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts"), ("tfd", B + "/📥️caller/🧪️tests/📥️birth/🟦️.ts"), ("tfd", B + "/⚙️operations/🧪️tests/🟦️.ts"),
        ("tfd", B + "/🧪️tests/🎟️phase/🟦️.ts"), ("tfd", B + "/🏃️execution/🐳️service/🧪️tests/🟦️.ts"), ("tfl", "✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts")]
PLANS = {
    "law": [("tfl", f) for f in lines(C + "/testfix-law-tools/suites-edited.txt")],
    "real21": [("tfd", f) for f in lines(C + "/testfix-defect-tools/suites-real21.txt")],
    "env168": [("tfd", f) for f in lines(C + "/testfix-defect-tools/suites-env168.txt")],
    "misc": MISC,
    "fixtests": [("tfd", B + "/🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts"), ("tfd", B + "/📥️caller/🧪️tests/📥️birth/🟦️.ts")],
}


_law = GEN + "/deferred-run-law.jsonl"
if os.path.isfile(_law):
    PLANS["lawred"] = [("tfd", r["file"]) for r in map(json.loads, open(_law, encoding="utf8")) if r["rc"] != 0]


def gate():
    load = float(re.findall(r"[\d.]+", subprocess.run(["sysctl", "-n", "vm.loadavg"], capture_output=True, text=True).stdout)[0])
    stat = subprocess.run(["vm_stat"], capture_output=True, text=True).stdout
    page = int(re.search(r"page size of (\d+)", stat).group(1))
    pages = {k: int(v) for k, v in re.findall(r"Pages (free|inactive|speculative):\s+(\d+)", stat)}
    total = int(subprocess.run(["sysctl", "-n", "hw.memsize"], capture_output=True, text=True).stdout) // page
    free = (pages["free"] + pages["inactive"] + pages["speculative"]) / total
    return load, free


out = GEN + "/deferred-run-%s.jsonl" % plan
done = set()
if os.path.isfile(out):
    done = {json.loads(l)["file"] for l in open(out, encoding="utf8")}
todo = [(kind, f) for kind, f in PLANS[plan] if f not in done]
started = time.time()
for kind, f in todo:
    load, free = gate()
    if load > 25 or free < 0.20:
        print("STOP: load %.1f free %.0f %%" % (load, free * 100))
        sys.exit(4)
    if time.time() - started > budget:
        print("budget used, %d left" % len([1 for k, g in todo if g not in {json.loads(l)["file"] for l in open(out, encoding="utf8")}]) if os.path.isfile(out) else len(todo))
        sys.exit(3)
    source = os.path.join("/Users/ueli/Documents/semio", f)
    text = open(source, encoding="utf8", errors="replace").read() if os.path.isfile(source) else ""
    if "🦀️cargo" in f or "🍎️image" in f or re.search(r"hdiutil|runCargo|\bcargo\b.*spawn|spawn.*\bcargo\b|attachNative|detachNative", text):
        open(out, "a", encoding="utf8").write(json.dumps({"file": f, "runner": kind, "rc": None, "skipped": "hdiutil/cargo user: not while show waits"}, ensure_ascii=False) + "\n")
        print("SKIP  hdiutil/cargo user", f[-70:], flush=True)
        continue
    listing = tempfile.NamedTemporaryFile("w", suffix=".txt", delete=False, encoding="utf8")
    listing.write(f + "\n")
    listing.close()
    scratch = tempfile.mktemp(suffix=".jsonl")
    t0 = time.time()
    if kind == "tfl":
        cmd = [sys.executable, C + "/testfix-law-tools/run.py", listing.name, scratch, cap]
        env = {**os.environ, "LIVE": "1", "CONC": "1"}
    else:
        cmd = [sys.executable, C + "/testfix-defect-tools/run.py", listing.name, scratch, "--jobs", "1", "--cap", cap]
        env = os.environ
    run = subprocess.run(cmd, capture_output=True, text=True, encoding="utf8", errors="replace", env=env, timeout=int(cap) * 4 + 120)
    rec = {}
    if os.path.isfile(scratch):
        rec = json.loads(open(scratch, encoding="utf8").read().strip().split("\n")[-1])
        os.remove(scratch)
    rec.update(file=f, runner=kind, wall=round(time.time() - t0, 1), load=load, free=round(free, 2))
    if not rec.get("rc") and "rc" not in rec:
        rec["rc"] = -9
        rec["errs"] = [(run.stdout + run.stderr)[-300:]]
    os.remove(listing.name)
    open(out, "a", encoding="utf8").write(json.dumps(rec, ensure_ascii=False) + "\n")
    print("%-5s rc=%s pass=%s fail=%s %5.0fs load=%.1f free=%.0f%% %s" % ("GREEN" if rec["rc"] == 0 else "RED", rec["rc"], rec.get("pass_"), rec.get("fail"), rec["wall"], load, free * 100, f[-70:]), flush=True)
print("plan %s finished" % plan)
