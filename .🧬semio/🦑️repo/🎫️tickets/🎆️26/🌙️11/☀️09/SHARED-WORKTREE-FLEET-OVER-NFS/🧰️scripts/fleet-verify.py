#!/usr/bin/env python3
"""usage: fleet-verify.py ORIGINS.json [PARALLEL] -- runs `verify --json` in every tree (daemons may keep running), counts files in .cache/f/lost per tree, prints a JSON summary with wall time, peak RSS (/usr/bin/time -l) and drift per tree."""
import json, os, re, subprocess, sys, time
from concurrent.futures import ThreadPoolExecutor
import fleet

names = json.load(open(sys.argv[1])); par = int(sys.argv[2]) if len(sys.argv) > 2 else 4
def one(o):
    t = time.time()
    p = subprocess.run(["/usr/bin/time", "-l", "bun", f"./{fleet.SCRIPT}", "verify", "--json"], cwd=fleet.tree(o), capture_output=True, text=True)
    rss = re.search(r"(\d+)\s+maximum resident set size", p.stderr)
    try: rep = json.loads(p.stdout.strip().splitlines()[-1])
    except Exception: rep = {"ok": False, "raw": p.stdout[-200:] + p.stderr[-200:]}
    lost = sum(len(fs) for _, _, fs in os.walk(f"{fleet.tree(o)}/.cache/f/lost"))
    return o, {"ok": rep.get("ok"), "checked": rep.get("checked"), "drift": len(rep.get("drift", [])), "wall_s": round(time.time() - t, 1), "rss_mib": round(int(rss.group(1)) / 1048576) if rss else None, "lost_files": lost, "exit": p.returncode}
with ThreadPoolExecutor(par) as ex: res = dict(ex.map(one, names))
bad = {k: v for k, v in res.items() if not v["ok"] or v["drift"]}
print(json.dumps({"trees": len(res), "ok": len(res) - len(bad), "bad": bad, "wall_s_max": max(v["wall_s"] for v in res.values()), "rss_mib_max": max(v["rss_mib"] or 0 for v in res.values()), "lost_files_total": sum(v["lost_files"] for v in res.values()), "lost_files_max": max(v["lost_files"] for v in res.values()), "lost_trees": sum(1 for v in res.values() if v["lost_files"])}))
