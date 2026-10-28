"""🧪 Runs the test directories of all batches inside mirror gq-main (sealed via mirror.py run) and records pass/fail lines: python3 -I runtests.py <label> <start> <end>"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
label, start, end = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
dirs = []
for g in sorted(os.listdir(os.path.join(HERE, "staged"))):
    for l in open(os.path.join(HERE, "staged", g, "tests.txt"), encoding="utf-8").read().split("\n"):
        if l and l not in dirs: dirs.append(l)
GEN = os.environ.get("GQ_GEN", "germanqa"); MIRROR = os.environ.get("GQ_MIRROR", "gq-main")
out_path = os.path.join(C, "🗑️generated", GEN, f"tests-{label}.json")
res = json.load(open(out_path, encoding="utf-8")) if os.path.exists(out_path) else {}
art = os.path.join(C, "🗑️generated", GEN, "a"); os.makedirs(art, exist_ok=True)
env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=art)
only = json.load(open(sys.argv[4], encoding="utf-8")) if len(sys.argv) > 4 else None
for i, d in enumerate(dirs[start:end], start=start):
    if (only is not None and d not in only): continue
    if d in res and only is None: continue
    cmd = [sys.executable, "-I", os.path.join(C, "mirror-tools", "mirror.py"), "run", MIRROR, "--cwd", d, "--peers-ok", "--", "bun", "test", "--timeout", "60000", "./🟦️.ts"]
    try: p = subprocess.run(cmd, capture_output=True, text=True, timeout=900, env=env)
    except subprocess.TimeoutExpired: res[d] = {"timeout": True}; json.dump(res, open(out_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1); print(i, d[-60:], "TIMEOUT"); continue
    txt = p.stdout + p.stderr
    fails = sorted(set(re.findall(r"^\(fail\) (.*?)(?: \[[\d.]+ms\])?$", txt, re.M)))
    m = re.findall(r"^\s*(\d+) (pass|fail)$", txt, re.M)
    res[d] = {"exit": p.returncode, "fails": fails, "counts": {k: int(v) for v, k in m}}
    json.dump(res, open(out_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(i, d[-60:], res[d]["counts"], len(fails), "fails")
print("done", len(dirs))
