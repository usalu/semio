"""📚 POS2 batch driver: runs several family specs (optionally in parallel) through `pos.py run` and prints one summary table
(canary reached, cur/base error totals, NEW per category, quarantined files) from the result JSON files.
usage: pos2batch.py [--jobs N] [--label L] [--cfgs lib,test] [--quarantine] [--no-base] [--iters N] <family> [<family> ...]
Writes only what pos.py writes (GEN/pos2-*)."""
import json, os, subprocess, sys, time
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from poslib import GEN, PREFIX
from pos import is_trait_scope

a = sys.argv[1:]
jobs, label, cfgs, extra, fams = 2, "b1", "lib,test", [], []
i = 0
while i < len(a):
    if a[i] == "--jobs":
        jobs = int(a[i + 1]); i += 2
    elif a[i] == "--label":
        label = a[i + 1]; i += 2
    elif a[i] == "--cfgs":
        cfgs = a[i + 1]; i += 2
    elif a[i] == "--summary":
        jobs = 0; i += 1
    elif a[i] in ("--quarantine", "--no-base"):
        extra.append(a[i]); i += 1
    elif a[i] == "--iters":
        extra += ["--iters", a[i + 1]]; i += 2
    else:
        fams.append(a[i]); i += 1
running, done = {}, []
queue = list(fams) if jobs else []
t0 = time.time()
while queue or running:
    while queue and len(running) < jobs:
        f = queue.pop(0)
        log = open(f"{GEN}/{PREFIX}-{f}-{label}-console.log", "w", encoding="utf-8")
        p = subprocess.Popen([sys.executable, HERE + "/pos.py", "run", f"{HERE}/families/{f}.json", "--cfgs", cfgs, "--label", label, *extra], stdout=log, stderr=subprocess.STDOUT, cwd=HERE)
        running[f] = (p, log)
    for f, (p, log) in list(running.items()):
        if p.poll() is not None:
            log.close()
            del running[f]
            done.append(f)
            print(f"[{round(time.time() - t0)}s] finished {f} rc={p.returncode}", flush=True)
    time.sleep(2)
print()
for f in fams:
    jp = f"{GEN}/{PREFIX}-{f}-{label}-result.json"
    if not os.path.exists(jp):
        print(f"{f}: NO RESULT (see {PREFIX}-{f}-{label}-console.log)")
        continue
    r = json.load(open(jp, encoding="utf-8"))
    for c, e in r["summary"].items():
        news = {}
        for d in e.get("new_list", []):
            if not d.get("stub") and not d.get("inherited"):
                k_ = "trait-scope" if is_trait_scope(d) else d["cat"]
                news[k_] = news.get(k_, 0) + 1
        canary = "yes" if len(e.get("canary_cur") or []) == 3 else "NO"
        print(f"{f:14} {c:5} canary={canary:3} cur={e['cur_total']} noise={e['cur_noise']} base={e.get('base_total', '-')} new={e.get('new', '-')} gone={e.get('gone', '-')} nonstub-new={news} q={ {k: (len(v) if v else v) for k, v in (r.get('quarantined') or {}).items()} }")
