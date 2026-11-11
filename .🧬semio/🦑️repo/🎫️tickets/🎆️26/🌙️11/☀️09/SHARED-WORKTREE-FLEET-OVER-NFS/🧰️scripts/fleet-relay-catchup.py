#!/usr/bin/env python3
"""usage: fleet-relay-catchup.py SECONDS RATE CONC... -- on the built fleet: for each CONC, start the daemons without relays, run the mixed load for SECONDS (events pile up in every group store), stop the load, start the relays with --concurrency CONC and time until every origin head is identical in all stores (the relay catch-up end to end); prints one JSON line per CONC."""
import glob, importlib.util, json, os, random, sys, time
import fleet
spec = importlib.util.spec_from_file_location("fleet_load", os.path.join(os.path.dirname(os.path.abspath(__file__)), "fleet-load.py")); fl = importlib.util.module_from_spec(spec); spec.loader.exec_module(fl)
seconds, rate = float(sys.argv[1]), float(sys.argv[2]); names = fleet.all_origins()
pool = []
for base, ds, fs in os.walk(fleet.R + "/master"):
    rel = os.path.relpath(base, fleet.R + "/master")
    if rel.startswith(fleet.MODREL) or rel.startswith(".cache"): ds[:] = []; continue
    pool += [os.path.join(rel, f) for f in fs if f.endswith((".ts", ".md", ".json")) and os.path.getsize(os.path.join(base, f)) < 200_000]
random.Random(3).shuffle(pool); pool = pool[:4000]
def heads():
    seen = {}
    for g in fleet.group_ids():
        for h in glob.glob(f"{fleet.store(g)}/v1/origins/*/head.json"):
            try: seen.setdefault(h.split("/origins/")[1].split("/")[0], set()).add(open(h).read())
            except OSError: return None
    return seen
def equal():
    s = heads(); return s is not None and all(len(v) == 1 for v in s.values()) and len(s) >= len(names)
for conc in sys.argv[3:]:
    for o in names: fleet.start_daemon(o)
    time.sleep(20)
    load = fl.Load(rate, pool, fleet.R + "/catchup-probes.jsonl", big_budget=0)
    for o in names: load.add(o, len(names))
    time.sleep(seconds); load.stop.set(); load.join(); time.sleep(6)
    behind = sum(1 for v in (heads() or {}).values() if len(v) > 1)
    os.environ["FLEET_RELAY_CONC"] = conc; t = time.time()
    for g in fleet.group_ids(): fleet.start_relay(g)
    while not equal() and time.time() - t < 600: time.sleep(0.1)
    print(json.dumps({"concurrency": int(conc), "ops": load.total(), "origins_behind_at_start": behind, "catchup_seconds": round(time.time() - t, 2)}), flush=True)
    fleet.stop()
