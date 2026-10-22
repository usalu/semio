#!/usr/bin/env python3
"""Runs the touched suites one at a time inside a scratch tree. Usage: suites.py <tree> <out.json> [--only N...]"""
import json, os, re, subprocess, sys, time
tree, out = sys.argv[1], sys.argv[2]
P = "🧰️framework/🔨️modules/🏃️process"; L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SUITES = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "suites.json"), encoding="utf8"))
only = sys.argv[4:] if len(sys.argv) > 3 and sys.argv[3] == "--only" else list(SUITES)
result = {}
for name in only:
    while int(float(subprocess.run(["sysctl", "-n", "vm.loadavg"], capture_output=True, text=True).stdout.split()[1])) >= 30: time.sleep(20)
    start = time.time()
    try:
        run = subprocess.run(["nice", "-n", "10", "bun", "test", "./" + SUITES[name]], cwd=tree, capture_output=True, text=True, timeout=420, env={**os.environ, "SEMIO_TEST_LEVEL": "fundamental"})
        text = run.stdout + run.stderr
        pas = re.search(r"(\d+) pass", text); fail = re.search(r"(\d+) fail", text)
        failed = sorted(set(re.findall(r"\(fail\) (.+?)(?: \[|$)", text, flags=re.M)))
        result[name] = {"exit": run.returncode, "pass": int(pas.group(1)) if pas else 0, "fail": int(fail.group(1)) if fail else 0, "failed": failed[:12], "seconds": round(time.time() - start, 1), "tail": text[-400:] if run.returncode and not pas else ""}
    except subprocess.TimeoutExpired:
        result[name] = {"exit": "timeout", "seconds": 420}
    print(name, result[name].get("pass"), result[name].get("fail"), result[name].get("exit"), flush=True)
    json.dump(result, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
