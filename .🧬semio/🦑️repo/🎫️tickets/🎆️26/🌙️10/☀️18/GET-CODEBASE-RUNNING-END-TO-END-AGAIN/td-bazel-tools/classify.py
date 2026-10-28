"""🧭 [td-bazel] classify every Bazel-tool test file in a mirror under three envelopes: plain `bun test ./f`, art (SEMIO_TEST_ARTIFACT_DIR under the ticket 🗑️generated), env (red-g mrun.sh official envelope). Streams one JSON line per file and mode. usage: classify.py <mirror> [--out FILE] [-j N] [--modes plain,art,env] [--only SUBSTR] [--files LIST] [--bazel] (sets SEMIO_TEST_BAZEL_BINARY to the managed live Bazel)"""
import json, os, re, subprocess, sys, uuid
from concurrent.futures import ThreadPoolExecutor
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
A = sys.argv[1:]
mirror = A[0]
opt = lambda k, d: A[A.index(k) + 1] if k in A else d
out, jobs, modes, only, lst = opt("--out", None), int(opt("-j", "3")), opt("--modes", "plain,art,env").split(","), opt("--only", ""), opt("--files", None)
R = os.path.join(os.environ.get("TMPDIR", "/tmp").rstrip("/"), "sm", mirror)
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
GEN = os.path.join(C, "🗑️generated", "td-bazel")
if "--bazel" in A:
    bz = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(C))))))), ".cache/b/t/m/bazel/bin")
    os.environ["SEMIO_TEST_BAZEL_BINARY"] = os.path.join(bz, sorted(os.listdir(bz))[0], "bazel")
if lst: files = [l.strip() for l in open(lst) if l.strip()]
else: files = sorted(os.path.relpath(os.path.join(d, f), R) for d, _, fs in os.walk(os.path.join(R, B)) for f in fs if f == "🟦️.ts" and "🧪️tests" in d and "🧫️fixtures" not in d)
files = [f for f in files if only in f]
def parse(t):
    nums = dict((k, int(v)) for v, k in re.findall(r"^ *(\d+) (pass|fail|skip)$", t, re.M))
    fails = sorted(set(l[7:].split(" [")[0] for l in t.splitlines() if l.startswith("(fail) ")))
    errs = [l[:240] for l in t.splitlines() if re.match(r"^(error|\w*Error):", l)][:4]
    return nums.get("pass", 0), nums.get("fail", 0), nums.get("skip", 0), fails, errs
def run(f, mode):
    env = dict(os.environ)
    if mode == "env":
        cmd = ["sh", os.path.join(C, "red-g-tools/mrun.sh"), mirror, "env", f, "--timeout", "170000"]
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=900); t = r.stdout + r.stderr
        head = t.split("\n")[0]; nums = dict((k, int(v)) for v, k in re.findall(r"(\d+) (pass|fail|skip)\b", head))
        fails = sorted(set(l[7:].split(" [")[0] for l in t.splitlines() if l.startswith("(fail) ")))
        return nums.get("pass", 0), nums.get("fail", 0), nums.get("skip", 0), fails, [l[:240] for l in t.splitlines() if l.startswith("error:")][:4]
    if mode == "art":
        d = os.path.join(GEN, "a" + uuid.uuid4().hex[:4]); os.makedirs(d, exist_ok=True); env["SEMIO_TEST_ARTIFACT_DIR"] = d
    else: env.pop("SEMIO_TEST_ARTIFACT_DIR", None)
    r = subprocess.run(["nice", "-n", "10", "bun", "test", "./" + f, "--timeout", "170000"], cwd=R, env=env, capture_output=True, text=True, timeout=900)
    return parse(r.stdout + r.stderr)
def one(f):
    rows = []
    for m in modes:
        try: p, fl, s, fails, errs = run(f, m)
        except subprocess.TimeoutExpired: p, fl, s, fails, errs = 0, -1, 0, [], ["TIMEOUT"]
        rows.append({"file": f, "mode": m, "pass": p, "fail": fl, "skip": s, "fails": fails, "err": errs})
    return rows
sink = open(out, "a") if out else None
with ThreadPoolExecutor(jobs) as pool:
    for rows in pool.map(one, files):
        for r in rows:
            print(f"{r['mode']:5} {r['pass']:3}p {r['fail']:2}f {r['skip']:2}s {r['file'][len(B)+1:]} {'|'.join(r['fails'])[:300]} {' '.join(r['err'])[:200] if r['fail'] or not r['pass'] else ''}", flush=True)
            if sink: sink.write(json.dumps(r, ensure_ascii=False) + "\n"); sink.flush()
