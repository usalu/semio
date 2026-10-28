"""⚖️ [backlog] differential law runner: runs single-file `bun test ./<file>` for a list of repo-relative test files in a mirror and in the baseline mirror (cached per file), prints one verdict line per file: SAME / BETTER / REGRESSION (a failing test name the baseline does not fail) / NEW (file absent in the baseline). A file that needs the process envelope (plain run says 'invocation required') is re-run with red-g's envelope runner (mrun.sh, mode env). Streams, never holds more than one result in memory. usage: difflaw.py <mirror> <list-file|-> [--base blbase] [-j N] [--out FILE]"""
import json, os, re, subprocess, sys, hashlib
from concurrent.futures import ThreadPoolExecutor
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
args = sys.argv[1:]
mirror = args[0]; lst = args[1]
base = args[args.index("--base") + 1] if "--base" in args else "blbase"
jobs = int(args[args.index("-j") + 1]) if "-j" in args else 2
out = args[args.index("--out") + 1] if "--out" in args else None
home = os.path.join(os.environ.get("TMPDIR", "/tmp"), "sm")
cache = C + "/backlog-tools/out/base-cache"; os.makedirs(cache, exist_ok=True)
MRUN = C + "/red-g-tools/mrun.sh"
def run(mir, f, mode):
    r = subprocess.run(["nice", "-n", "10", "sh", MRUN, mir, mode, f, "--timeout", "170000"], capture_output=True, text=True, timeout=600)
    t = r.stdout + r.stderr
    nums = dict((k, int(v)) for v, k in re.findall(r"(\d+) (pass|fail|skip)\b", t.split("\n")[0] if t else ""))
    fails = sorted(set(l[len("(fail) "):].split(" [")[0] for l in t.splitlines() if l.startswith("(fail)")))
    return {"mode": mode, "pass": nums.get("pass", 0), "fail": nums.get("fail", 0), "fails": fails, "err": [l[:160] for l in t.splitlines() if l.startswith("error:")][:2], "raw": t[:400]}
def measure(mir, f):
    if not os.path.exists(f"{home}/{mir}/{f}"): return {"absent": True}
    res = run(mir, f, "plain")
    if "invocation required" in res["raw"] or "Original parent" in res["raw"]:
        env = run(mir, f, "env")
        if env["pass"] + env["fail"] > 0: res = env
    res.pop("raw", None); return res
def baseline(f):
    key = f"{cache}/{hashlib.sha1((base + f).encode()).hexdigest()}.json"
    if os.path.exists(key):
        m = json.load(open(key))
        if m.get("file") == f and m.get("base") == base and os.path.getmtime(key) > os.path.getmtime(f"{home}/{base}") - 1: return m["res"]
    res = measure(base, f); json.dump({"file": f, "base": base, "res": res}, open(key, "w"), ensure_ascii=False); return res
def one(f):
    b = baseline(f); m = measure(mirror, f)
    if m.get("absent"): v = "ABSENT"
    elif b.get("absent"): v = "NEW" + ("" if m["fail"] == 0 else " (fails: %s)" % ", ".join(m["fails"][:3]))
    else:
        new = [x for x in m["fails"] if x not in b["fails"]]
        v = "REGRESSION (new fails: %s)" % "; ".join(new[:4]) if new or (m["fail"] > b["fail"] and not m["fails"]) else "BETTER" if m["fail"] < b["fail"] else "SAME"
    line = {"file": f, "verdict": v.split(" ")[0], "detail": v, "base": None if b.get("absent") else [b["pass"], b["fail"], b["mode"]], "mirror": None if m.get("absent") else [m["pass"], m["fail"], m["mode"]], "err": m.get("err", [])}
    return line
files = [l.strip() for l in (sys.stdin if lst == "-" else open(lst)) if l.strip().endswith(".ts")]
done = set()
if out and os.path.exists(out):
    for l in open(out):
        try: done.add(json.loads(l)["file"])
        except Exception: pass
files = [f for f in files if f not in done]
sink = open(out, "a") if out else None
with ThreadPoolExecutor(jobs) as pool:
    for line in pool.map(one, files):
        s = json.dumps(line, ensure_ascii=False)
        print(f"{line['verdict']:10} base={line['base']} now={line['mirror']} {line['file'].split('/')[-3]}/{line['file'].split('/')[-2]} {'' if line['verdict'] in ('SAME','BETTER','NEW') else line['detail'][:200]}", flush=True)
        if sink: sink.write(s + "\n"); sink.flush()
