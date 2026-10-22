#!/usr/bin/env python3
"""Light law pass on the overlay after the simulated chain (sandboxed: nothing below the checkout can be written; one process at a time, `nice -n 10`, cap per suite).
Each suite is a test file of the overlay; a red suite is re-run on the pristine base (same file) to separate regressions from pre-existing reds.
usage: laws.py [--cap 150] [--only substring] ; result: C/🗑️generated/round6-apply/laws.json"""
import glob, json, os, re, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
OV, BASE = os.environ.get("LAWS_OV", "/private/tmp/i6/ov"), "/private/tmp/i6/base"
import glob as _g
PINNED_BUN = (_g.glob(LIVE + "/.cache/b/t/m/bun/*/bun") or ["bun"])[0]
args = sys.argv[1:]
cap = int(args[args.index("--cap") + 1]) if "--cap" in args else 150
only = args[args.index("--only") + 1] if "--only" in args else ""
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
P = "🧰️framework/🔨️modules/🏃️process"
B = L + "/🏗️build"
SUITES = [
    ("tool-iface", P + "/🧰️tools/🧪️tests/📖️registry/🟦️.ts"),
    ("tool-iface", P + "/🧰️tools/🧪️tests/🏃️run/🟦️.ts"),
    ("tool-iface", P + "/🧰️tools/🧪️tests/🔎️resolve/🟦️.ts"),
    ("tool-iface", P + "/🧰️tools/🧪️tests/🔍️scan/🟦️.ts"),
    ("tool-iface", P + "/🧰️tools/🧪️tests/🔌️funnel/🟦️.ts"),
    ("tool-iface", L + "/⚖️laws/🧰️direct-tool-spawn/🧪️tests/🟦️.ts"),
    ("operations", B + "/⚙️operations/🧪️tests/🟦️.ts"),
    ("runtime-seam", "🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🟦️.ts"),
    ("pm-seam", L + "/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🧪️tests/🟦️.ts"),
    ("backend-contract", B + "/🔌️backends/🧪️tests/🟦️.ts"),
    ("backend-contract", B + "/📐️conformance/🧪️tests/🟦️.ts"),
    ("rcache-server", B + "/🌐️remote-cache/🧪️tests/🟦️.ts"),
    ("cache-gc", B + "/🧹️cache-gc/🧪️tests/🟦️.ts"),
    ("cache-gc", L + "/⚡️caching/💾️storage/🧾️ledger/🧪️tests/🟦️.ts"),
    ("cache-gc", L + "/🏗️build/🧹️cache-gc/🧪️tests/📁️directory/🟦️.ts"),
    ("polyglot-units", B + "/🧩️units/🧪️tests/🟦️.ts"),
    ("perf-slo", B + "/📏️incremental/🏎️bench/🧪️tests/🟦️.ts"),
    ("agent-session", P + "/🪪️agent-sessions/🧪️tests/🟦️.ts"),
    ("hot-cache", B + "/🏡️resident/🧪️tests/🟦️.ts"),
    ("mem-gov", B + "/🌡️host-capacity/🧪️tests/🟦️.ts"),
    ("revision-fix", B + "/📥️inputs/🔏️revision/🧪️tests/🧓️stale-reader/🟦️.ts"),
]
import re as _re
_go = open(C + "/graph-parallel-tools/go.sh", encoding="utf8").read()
for _m in _re.finditer(r'"\$([BL])/([^"]+)"', _go):
    SUITES.append(("graph-parallel", (B if _m.group(1) == "B" else L) + "/" + _m.group(2)))
LAWS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests"


def run(root, rel):
    path = os.path.join(root, rel)
    if not os.path.isfile(path):
        return {"missing": True}
    t0 = time.time()
    try:
        r = subprocess.run(["sandbox-exec", "-p", '(version 1)(allow default)(deny file-write* (subpath "%s"))' % LIVE, "nice", "-n", "10", "perl", "-e", "alarm %d; exec @ARGV" % cap, PINNED_BUN, "test", path],
                           capture_output=True, text=True, encoding="utf8", errors="replace", cwd=root, env={**os.environ, "SEMIO_REPOSITORY_ROOT": root, "PATH": "/private/tmp/i6/bin:" + os.environ["PATH"]}, timeout=cap + 30)
        out = r.stdout + r.stderr
        p = re.search(r"(\d+) pass", out)
        f = re.search(r"(\d+) fail", out)
        return {"rc": r.returncode, "pass": int(p.group(1)) if p else None, "fail": int(f.group(1)) if f else None, "secs": round(time.time() - t0), "tail": out.strip()[-300:]}
    except subprocess.TimeoutExpired:
        return {"rc": -9, "timeout": True, "secs": cap}


results = []
for name, rel in SUITES:
    if only and only not in name and only not in rel:
        continue
    row = {"set": name, "suite": rel[-90:], "overlay": run(OV, rel)}
    if row["overlay"].get("missing"):
        row["verdict"] = "no such suite"
    elif row["overlay"].get("rc") == 0:
        row["verdict"] = "green"
    else:
        row["base"] = run(BASE, rel)
        row["verdict"] = "red in both (pre-existing)" if row["base"].get("rc") not in (0, None) and not row["base"].get("missing") else ("red only after the chain (CHECK)" if not row["base"].get("missing") else "red, new suite")
    results.append(row)
    print("%-16s %-28s %s  overlay pass=%s fail=%s %ss" % (name, row["verdict"], rel[-46:], row["overlay"].get("pass"), row["overlay"].get("fail"), row["overlay"].get("secs")), flush=True)
os.makedirs(C + "/🗑️generated/round6-apply", exist_ok=True)
json.dump(results, open(C + "/🗑️generated/round6-apply/laws.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
