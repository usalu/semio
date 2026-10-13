#!/usr/bin/env python3
"""🔎️ Wave L1 plugin manifest sweep: `cargo check --lib` natively for every `✏️s/🔌️plugins` package, plus `--target wasm32-wasip2` for every
component crate, through the L1 cargo wrapper; resumable (recorded rows are skipped) and bounded by a time budget.
usage: 🔎️manifest-sweep.py <seconds> <jobs>"""
import concurrent.futures as cf, os, subprocess, sys, time
REPO = "/Users/ueli/Documents/semio"
T = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
OUT = T + "/🗑️generated/l1/manifest-sweep.tsv"
os.makedirs(os.path.dirname(OUT), exist_ok=True)
budget, jobs = int(sys.argv[1]), int(sys.argv[2])
start = time.time()
done = {tuple(l.split("\t")[1:3]) for l in open(OUT) if l.count("\t") >= 2} if os.path.exists(OUT) else set()
manifests = sorted(subprocess.run(["find", "✏️s/🔌️plugins", "-path", "*/📦️packages/🦀️rust/Cargo.toml", "-not", "-path", "*/target/*", "-not", "-path", "*/node_modules/*"], cwd=REPO, capture_output=True, text=True).stdout.split())
def component(m): return "[package.metadata.component]" in open(os.path.join(REPO, m), encoding="utf8").read()
todo = [(m, "native") for m in manifests] + [(m, "wasm") for m in manifests if component(m)]
todo = [row for row in todo if row not in done]

def run(row):
    m, mode = row
    left = int(budget - (time.time() - start))
    if left < 90: return None
    text = open(os.path.join(REPO, m), encoding="utf8").read()
    bins = "[lib]" not in text and "[[bin]]" in text
    args = ["check", "--manifest-path", m, "--bins" if bins else "--lib", "--message-format", "short"] + (["--target", "wasm32-wasip2"] if mode == "wasm" else [])
    if any(l.startswith("component-app-assembly") for l in text.splitlines()): args += ["--features", "component-app-assembly"]
    r = subprocess.run(["bash", T + "/l1-library-xor-component/🦀️cargo.sh", str(left), *args], cwd=REPO, capture_output=True, text=True)
    if r.returncode == 124: return None
    if r.returncode:
        err = next((l for l in (r.stdout + r.stderr).splitlines() if ": error" in l or l.startswith("error")), "")
        return "FAIL\t%s\t%s\t%s\n" % (m, mode, err[:300])
    return "ok\t%s\t%s\t\n" % (m, mode)

with cf.ThreadPoolExecutor(jobs) as ex, open(OUT, "a") as out:
    for row in ex.map(run, todo):
        if row: out.write(row); out.flush()
rows = [l for l in open(OUT) if l.count("\t") >= 2]
print("recorded %d/%d ok=%d fail=%d" % (len(rows), len(manifests) + sum(component(m) for m in manifests), sum(l.startswith("ok") for l in rows), sum(l.startswith("FAIL") for l in rows)))
