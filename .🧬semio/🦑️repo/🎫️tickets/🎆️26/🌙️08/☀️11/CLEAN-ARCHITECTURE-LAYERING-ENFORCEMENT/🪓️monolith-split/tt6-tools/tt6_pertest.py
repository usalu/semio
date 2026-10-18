"""🧪 Runs every test of the listed files in its own repository process envelope (tt6_envrun.ts allows one Original process birth per bun process), P workers. usage: tt6_pertest.py <phase> <list-file> [workers]; results in 🗑️generated/tt6-pt-<phase>.tsv (file<TAB>name<TAB>pass|fail|timeout|none)"""
import os, re, subprocess, sys, concurrent.futures as cf
R = "/Users/ueli/Documents/semio"
T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "..", "🗑️generated")
phase, lst = sys.argv[1], sys.argv[2]
workers = int(sys.argv[3]) if len(sys.argv) > 3 else 3
env = dict(os.environ, SEMIO_NX_FIXED_STORAGE=f"{R}/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m", NX_CACHE_DIRECTORY=f"{R}/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts", SEMIO_TEST_ARTIFACT_DIR=f"{R}/.cache/tt6-art")
jobs = []
for rel in (l.strip() for l in open(lst, encoding="utf8")):
    if not rel: continue
    src = open(os.path.join(R, rel), encoding="utf8").read()
    for m in re.finditer(r"^\s*(?:test|it)(?:\.\w+)?\(\s*(\"(?:[^\"\\]|\\.)*\"|'(?:[^'\\]|\\.)*'|`[^`$]*`)", src, re.M):
        name = m.group(1)[1:-1].encode().decode("unicode_escape") if "\\" in m.group(1) else m.group(1)[1:-1]
        jobs.append((rel, name))
def run(job):
    rel, name = job
    try:
        p = subprocess.run(["perl", "-e", "alarm 600; exec @ARGV", "--", "nice", "-n", "10", "bun", f"{T}/tt6_envrun.ts", "./" + rel, "-t", re.escape(name)], cwd=R, env=env, capture_output=True, text=True, timeout=700)
        out = p.stdout + p.stderr
        status = "pass" if re.search(r"^\s*[1-9]\d* pass", out, re.M) and not re.search(r"^\s*[1-9]\d* fail", out, re.M) else ("fail" if re.search(r"^\s*[1-9]\d* fail", out, re.M) else "none")
        first = next((l for l in out.splitlines() if l.startswith("error:")), "")
        return rel, name, status, first[:160]
    except subprocess.TimeoutExpired:
        return rel, name, "timeout", ""
with cf.ThreadPoolExecutor(workers) as ex, open(os.path.join(G, f"tt6-pt-{phase}.tsv"), "w", encoding="utf8") as out:
    for r in ex.map(run, jobs):
        out.write("\t".join(r) + "\n"); out.flush()
open(os.path.join(G, f"tt6-pt-{phase}.done"), "w").write("done\n")
