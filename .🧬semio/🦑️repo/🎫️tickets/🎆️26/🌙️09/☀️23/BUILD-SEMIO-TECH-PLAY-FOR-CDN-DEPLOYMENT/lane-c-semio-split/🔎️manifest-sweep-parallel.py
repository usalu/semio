#!/usr/bin/env python3
"""🔎️ Parallel, resumable plugin manifest sweep (native `cargo check --lib`) through the lane-c cargo wrapper; records one
TSV row per manifest and stops launching new checks when the time budget is spent.
usage: 🔎️manifest-sweep-parallel.py <seconds> <jobs>"""
import concurrent.futures as cf, os, subprocess, sys, time
REPO = "/Users/ueli/Documents/semio"
T = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
OUT = T + "/🗑️generated/lane-c/manifest-sweep.tsv"
budget, jobs = int(sys.argv[1]), int(sys.argv[2])
start = time.time()
done = {l.split("\t")[1] for l in open(OUT) if "\t" in l} if os.path.exists(OUT) else set()
manifests = sorted(subprocess.run(["find", "✏️s/🔌️plugins", "-path", "*/📦️packages/🦀️rust/Cargo.toml", "-not", "-path", "*/target/*"], cwd=REPO, capture_output=True, text=True).stdout.split())
todo = [m for m in manifests if m not in done]

def run(m):
    left = int(budget - (time.time() - start))
    if left < 90:
        return None
    feats = ["--features", "component-app-assembly"] if any(l.startswith("component-app-assembly") for l in open(os.path.join(REPO, m))) else []
    r = subprocess.run(["bash", T + "/lane-c-semio-split/🦀️cargo.sh", str(left), "check", "--manifest-path", m, "--lib", "--message-format", "short"] + feats, cwd=REPO, capture_output=True, text=True)
    if r.returncode == 124:
        return None
    if r.returncode:
        err = next((l for l in (r.stdout + r.stderr).splitlines() if ": error" in l or l.startswith("error")), "")
        return "FAIL\t%s\t%s\n" % (m, err[:300])
    return "ok\t%s\n" % m

with cf.ThreadPoolExecutor(jobs) as ex, open(OUT, "a") as out:
    for row in ex.map(run, todo):
        if row:
            out.write(row); out.flush()
rows = [l for l in open(OUT) if "\t" in l]
print("recorded %d/%d ok=%d fail=%d" % (len(rows), len(manifests), sum(l.startswith("ok") for l in rows), sum(l.startswith("FAIL") for l in rows)))
