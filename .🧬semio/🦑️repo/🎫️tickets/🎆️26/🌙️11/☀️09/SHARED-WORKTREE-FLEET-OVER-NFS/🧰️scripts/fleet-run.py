#!/usr/bin/env python3
"""usage: fleet-run.py LOADSECONDS RATE [--faults] [--no-start] -- starts the fleet (unless --no-start), the sampler, the prober and the mixed load, runs the fault timeline (kill -9 five daemons at 20 %, kill -9 one relay at 33 %, group store away 30 s at 47 %, 51st machine joins at 60 %, compact on a server and a worker at 75 % and 80 %), stops the load, then measures quiescence and convergence. Writes $R/run/*.json* and prints a JSON summary."""
import glob, json, os, random, shutil, subprocess, sys, time
import fleet, importlib.util

spec = importlib.util.spec_from_file_location("fleet_load", os.path.join(os.path.dirname(os.path.abspath(__file__)), "fleet-load.py")); fl = importlib.util.module_from_spec(spec); spec.loader.exec_module(fl)
seconds, rate = float(sys.argv[1]), float(sys.argv[2]); faults = "--faults" in sys.argv
R = fleet.R; RUN = R + "/run"
shutil.rmtree(RUN, ignore_errors=True); os.makedirs(RUN)
names = fleet.all_origins(); json.dump(names, open(RUN + "/origins.json", "w"))
here = os.path.dirname(os.path.abspath(__file__)); events = []
def mark(what, **kw):
    events.append({"t": round(time.time() - T0, 1), "what": what, **kw}); print(events[-1], flush=True)

pool = []
for base, ds, fs in os.walk(R + "/master"):
    rel = os.path.relpath(base, R + "/master")
    if rel.startswith(fleet.MODREL) or rel.startswith(".cache"): ds[:] = []; continue
    for f in fs:
        if f.endswith((".ts", ".md", ".json", ".py", ".sh", ".txt", ".rs", ".go")) and os.path.getsize(os.path.join(base, f)) < 200_000: pool.append(os.path.join(rel, f))
random.Random(3).shuffle(pool); pool = pool[:4000]

if "--no-start" not in sys.argv: fleet.start_all()
time.sleep(float(os.environ.get("FLEET_SETTLE", "30")))
T0 = time.time()
sampler = subprocess.Popen([sys.executable, here + "/fleet-sample.py", RUN + "/sample", "10", f"orchestrator={os.getpid()}"], env=os.environ)
prober = subprocess.Popen([sys.executable, here + "/fleet-probe.py", RUN + "/probes.jsonl", RUN + "/origins.json", RUN + "/probe-results.jsonl"])
open(RUN + "/probes.jsonl", "w").close()
load = fl.Load(rate, pool, RUN + "/probes.jsonl")
mark("load-start", seconds=seconds)
for o in names: load.add(o, len(names))

def at(frac): 
    while time.time() - T0 < frac * seconds: time.sleep(0.5)

def run_faults():
    rnd = random.Random(11)
    at(0.20); victims = [rnd.choice(fleet.origins_of(g)) for g in fleet.group_ids()]
    for v in victims: fleet.kill9(v)
    mark("kill9-daemons", names=victims); time.sleep(8)
    for v in victims: fleet.start_daemon(v)
    mark("restart-daemons", names=victims)
    G = fleet.group_ids(); relay_g = G[min(2, len(G) - 1)]; away_g = G[min(3, len(G) - 1)]; away_s = float(os.environ.get("FLEET_AWAY", "30"))
    at(0.33); fleet.kill9(f"relay-{relay_g}"); mark("kill9-relay", name=f"relay-{relay_g}"); time.sleep(10); fleet.start_relay(relay_g); mark("restart-relay")
    at(0.47); away = fleet.store(away_g); os.rename(away, away + ".away"); mark("store-away", group=away_g); time.sleep(away_s); os.rename(away + ".away", away); mark("store-back", group=away_g)
    at(0.60); new = "g2-w10"; os.makedirs(fleet.tree(new), exist_ok=True)
    t = time.time(); p = fleet.run(fleet.tree("g2-s"), "join", "--group", "g2", "--origin", new, "--store", fleet.store("g2"), "--worktree", fleet.tree(new), capture=True)
    mark("join-51st", seconds=round(time.time() - t, 1)); fleet.start_daemon(new); names.append(new); json.dump(names, open(RUN + "/origins.json", "w")); load.add(new, len(names))
    at(0.75); t = time.time(); p = fleet.run("g1-s", "compact", "--collect-blobs", capture=True); mark("compact-server", seconds=round(time.time() - t, 1), out=p.stdout.strip().splitlines()[-1][:200])
    at(0.80); t = time.time(); p = fleet.run(f"{relay_g}-w2", "compact", capture=True); mark("compact-worker", seconds=round(time.time() - t, 1), out=p.stdout.strip().splitlines()[-1][:200])
if faults:
    try: run_faults()
    except Exception as e: mark("fault-error", error=repr(e)[:300])

at(1.0); load.stop.set(); load.join()
t_stop = time.time(); mark("load-stop", ops=load.total(), errors=load.errors)
json.dump({"counts": load.counts, "total": load.total(), "errors": load.errors}, open(RUN + "/ops.json", "w"))

def heads_equal():
    seen = {}
    for g in fleet.group_ids():
        for h in glob.glob(f"{fleet.store(g)}/v1/origins/*/head.json"):
            try: v = open(h).read()
            except OSError: return False
            seen.setdefault(h.split("/origins/")[1].split("/")[0], set()).add(v)
    return all(len(v) == 1 for v in seen.values()) and len(seen) >= len(names)
tokens = {}
t_sent = time.time()
for o in names:
    tokens[o] = f"{o} {os.urandom(6).hex()}\n"
    os.makedirs(f"{fleet.tree(o)}/work/sentinel", exist_ok=True)
    open(f"{fleet.tree(o)}/work/sentinel/{o}", "w").write(tokens[o])
def sentinels_everywhere():
    for r in names:
        for o, tok in tokens.items():
            try:
                if open(f"{fleet.tree(r)}/work/sentinel/{o}").read() != tok: return False
            except OSError: return False
    return True
t_quiet = None
while time.time() - t_sent < 900:
    if sentinels_everywhere(): t_quiet = time.time() - t_sent; break
    time.sleep(0.5)
mark("sentinels-everywhere", seconds=t_quiet)
t_heads = None
while time.time() - t_stop < 900:
    if heads_equal(): t_heads = time.time() - t_stop; break
    time.sleep(1)
mark("stores-equal", seconds=t_heads)
t_full = None
while time.time() - t_stop < 1800:
    r = subprocess.run([sys.executable, here + "/fleet-converge.py", RUN + "/origins.json"], capture_output=True, text=True)
    if r.returncode == 0: t_full = time.time() - t_stop; break
    open(RUN + "/converge.log", "a").write(f"{time.time() - t_stop:.0f}s {r.stdout}\n")
    dname = "diag.json" if not os.path.exists(RUN + "/diag.json") else ("diag-late.json" if time.time() - t_stop > 90 and not os.path.exists(RUN + "/diag-late.json") else None)
    if dname:
        try:
            bad = json.loads(r.stdout)["bad"]; paths = []
            for v in bad.values():
                for lst in v["sample"]: paths += [x for x in lst if x not in paths]
            diag = {}
            for q in paths[:5]:
                ev = subprocess.run(["grep", "-rhF", f'"path":"{q}"'] + glob.glob(fleet.store("g1") + "/v1/origins/*/segments/*.jsonl"), capture_output=True, text=True).stdout.splitlines()
                lg = subprocess.run(["grep", "-hF", q] + glob.glob(R + "/logs/*.out"), capture_output=True, text=True).stdout.splitlines()
                diag[q] = {"exists_on": [o for o in names if os.path.lexists(f"{fleet.tree(o)}/{q}")], "journal_g1": [e[:230] for e in ev[-6:]], "log_lines": [e[:230] for e in lg[-6:]]}
            json.dump(diag, open(RUN + "/" + dname, "w"), indent=1)
        except Exception as e: open(RUN + "/" + dname, "w").write(repr(e))
    time.sleep(5)
mark("trees-equal-by-hash", seconds=t_full, result=r.stdout.strip()[:400])
open(RUN + "/sample/stop", "w").close(); prober.terminate(); time.sleep(1)
json.dump(events, open(RUN + "/events.json", "w"))
print(json.dumps({"events": events}))
