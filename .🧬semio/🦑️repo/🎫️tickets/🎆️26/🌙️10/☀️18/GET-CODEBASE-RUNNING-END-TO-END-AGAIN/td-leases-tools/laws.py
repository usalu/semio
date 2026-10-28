"""⚖️ [td-leases] differential single-file laws inside one mirror: every file of a list runs bare (`bun test ./file` in its directory, sealed); a file that fails on the missing parent envelope also runs through the envelope fallback (red-g-tools/mrun.sh env). One JSON line per file (pass, fail, failing names, mode) to <out>; compare two outputs with --compare. usage: python3 -I laws.py <mirror> <list> <out.jsonl> [-j N] | python3 -I laws.py --compare <base.jsonl> <new.jsonl>"""
import json, os, re, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
def parse(text):
    nums = {k: int(v) for v, k in re.findall(r"^\s*(\d+) (pass|fail)$", text, re.M)[-2:]}
    fails = sorted({l[7:].split(" [")[0] for l in text.splitlines() if l.startswith("(fail) ")})
    return nums.get("pass", 0), nums.get("fail", 0), fails
def run(mirror, f):
    d, b = os.path.dirname(f), os.path.basename(f)
    r = subprocess.run(["nice", "-n", "10", "python3", "-I", C + "/mirror-tools/mirror.py", "run", mirror, "--peers-ok", "--cwd", d, "--", "bun", "test", "--timeout", "170000", "./" + b], capture_output=True, text=True, timeout=1500)
    text = r.stdout + r.stderr; p, n, fails = parse(text); mode = "plain"
    if "Original parent Script process invocation required" in text:
        e = subprocess.run(["nice", "-n", "10", "sh", C + "/red-g-tools/mrun.sh", mirror, "env", f, "--timeout", "170000"], capture_output=True, text=True, timeout=1500)
        et = e.stdout + e.stderr; ep, en, ef = parse(et.split("\n", 1)[1] if "\n" in et else et)
        first = et.splitlines()[0] if et else ""; tot = re.findall(r"(\d+) (pass|fail)", first)
        if tot: ep, en = int(tot[-2][0]) if len(tot) > 1 else ep, int(tot[-1][0])
        p, n, fails, mode = ep, en, ef, "env"
    return {"file": f, "pass": p, "fail": n, "fails": fails, "mode": mode, "guard": "GUARD FAIL" in text}
if sys.argv[1] == "--compare":
    base = {j["file"]: j for j in map(json.loads, open(sys.argv[2]))}; new = {j["file"]: j for j in map(json.loads, open(sys.argv[3]))}
    for f, n in new.items():
        b = base.get(f)
        if not b: print("NEW       ", n["pass"], n["fail"], f[-110:]); continue
        extra = [x for x in n["fails"] if x not in b["fails"]]
        v = "REGRESSION" if extra or n["fail"] > b["fail"] and not n["fails"] else "BETTER" if n["fail"] < b["fail"] else "SAME"
        print(f"{v:10} base {b['pass']}/{b['fail']} {b['mode']:5} new {n['pass']}/{n['fail']} {n['mode']:5} {f[-100:]}" + (("  NEW FAILS: " + "; ".join(extra)[:300]) if extra else ""))
    sys.exit(0)
mirror, lst, out = sys.argv[1:4]; jobs = int(sys.argv[sys.argv.index("-j") + 1]) if "-j" in sys.argv else 3
files = [l.strip() for l in open(lst) if l.strip()]
with open(out, "w") as sink, ThreadPoolExecutor(jobs) as pool:
    for res in pool.map(lambda f: run(mirror, f), files):
        sink.write(json.dumps(res, ensure_ascii=False) + "\n"); sink.flush(); print(res["pass"], res["fail"], res["mode"], res["file"][-90:], flush=True)
