#!/usr/bin/env python3
"""Fleet harness for the scale rehearsal. usage: fleet.py build|start|stop|kill NAME|restart NAME|pids|join NAME. Env: S (scratch dir), FLEET_GROUPS (5), FLEET_PER_GROUP (10), FLEET_MODSRC (module directory to run instead of the repository's), FLEET_RELAY_CONC (relay --concurrency). All data below $S/rehearsal."""
import glob, json, os, shutil, signal, subprocess, sys, time
from concurrent.futures import ThreadPoolExecutor

S = os.environ["S"]
R = os.environ.get("R", S + "/rehearsal")
REPO = "/Users/ueli/Documents/semio"
GROUPS = int(os.environ.get("FLEET_GROUPS", "5"))
PER = int(os.environ.get("FLEET_PER_GROUP", "10"))
POLL, SCAN, DEBOUNCE = os.environ.get("FLEET_POLL", "250"), os.environ.get("FLEET_SCAN", "300"), os.environ.get("FLEET_DEBOUNCE", "150")
MODREL = os.path.relpath(glob.glob(REPO + "/*framework/*products/*repo/*modules/*shared-worktree")[0], REPO)
REPLICA = os.path.basename(glob.glob(os.path.join(REPO, MODREL, "*replica"))[0])
SCRIPT = f"{MODREL}/{REPLICA}/" + os.path.basename(glob.glob(os.path.join(REPO, MODREL, REPLICA, "*script.ts"))[0])
BOUNDARY = glob.glob(os.path.join(REPO, MODREL, "*boundary", "*.ts"))[0]
SHARED_TS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "fleet-shared.ts")
PIDS = R + "/pids.json"

def group_ids(): return [f"g{i+1}" for i in range(GROUPS)]
def origins_of(g): return [f"{g}-s"] + [f"{g}-w{i}" for i in range(1, PER)]
def all_origins(): return [o for g in group_ids() for o in origins_of(g)]
def group_of(o): return o.split("-")[0]
def tree(o): return f"{R}/trees/{o}"
def store(g): return f"{R}/stores/{g}"

def run(origin_or_tree, *args, check=True, capture=False, env=None):
    cwd = origin_or_tree if origin_or_tree.startswith("/") else tree(origin_or_tree)
    p = subprocess.run(["bun", f"./{SCRIPT}", *args], cwd=cwd, capture_output=capture, text=True, env=env)
    if check and p.returncode: raise SystemExit(f"failed {args} in {cwd}: {p.returncode}\n{p.stderr if capture else ''}")
    return p

def pids():
    try: return json.load(open(PIDS))
    except OSError: return {}
def save_pids(d): json.dump(d, open(PIDS, "w"))

def alive(pid):
    try: os.kill(pid, 0); return True
    except ProcessLookupError: return False
    except PermissionError: return True

def spawn(name, origin_for_cwd, args):
    os.makedirs(R + "/logs", exist_ok=True)
    out = open(f"{R}/logs/{name}.out", "ab"); err = open(f"{R}/logs/{name}.err", "ab")
    prof = ["--cpu-prof", "--cpu-prof-dir=" + R + "/prof"] if name == os.environ.get("FLEET_PROF") else []
    p = subprocess.Popen(["bun", *prof, f"./{SCRIPT}", *args], cwd=tree(origin_for_cwd), stdout=out, stderr=err, stdin=subprocess.DEVNULL, start_new_session=True, env={**os.environ, **({"BUN_JSC_forceRAMSize": os.environ["FLEET_RAM"]} if os.environ.get("FLEET_RAM") else {})})
    d = pids(); d[name] = p.pid; save_pids(d)
    return p.pid

def start_daemon(o):
    return spawn(o, o, ["run", "--poll", POLL, "--scan", SCAN, "--debounce", DEBOUNCE])

def start_relay(g):
    peers = ",".join(f"{h}={store(h)}" for h in group_ids() if h != g)
    conc = ["--concurrency", os.environ["FLEET_RELAY_CONC"]] if os.environ.get("FLEET_RELAY_CONC") else []
    return spawn(f"relay-{g}", f"{g}-s", ["relay", "--group", g, "--store", store(g), "--peers", peers, "--worktree", R + "/relay-none", "--poll", POLL, *conc])

def stop(names=None, wait=20):
    d = pids(); names = names or list(d)
    for n in names:
        if n in d and alive(d[n]): os.kill(d[n], signal.SIGTERM)
    t0 = time.time()
    while time.time() - t0 < wait and any(alive(d[n]) for n in names if n in d): time.sleep(0.3)
    for n in names:
        if n in d and alive(d[n]): os.kill(d[n], signal.SIGKILL)
    save_pids({k: v for k, v in d.items() if k not in names})

def kill9(name):
    d = pids(); os.kill(d[name], signal.SIGKILL)

def prune_master():
    keep = subprocess.run(["bun", SHARED_TS, R + "/master", BOUNDARY], capture_output=True, text=True, check=True).stdout.split("\n")
    keep = set(x for x in keep if x)
    dirs = set()
    for k in keep:
        p = os.path.dirname(k)
        while p: dirs.add(p); p = os.path.dirname(p)
    removed = 0
    for base, ds, fs in os.walk(R + "/master", topdown=True):
        rel = os.path.relpath(base, R + "/master"); rel = "" if rel == "." else rel
        for d in list(ds):
            q = os.path.join(rel, d)
            if q not in dirs and not os.path.islink(os.path.join(base, d)): shutil.rmtree(os.path.join(base, d)); ds.remove(d); removed += 1
        for f in fs:
            q = os.path.join(rel, f)
            if q not in keep: os.unlink(os.path.join(base, f)); removed += 1
    return len(keep), removed

def make_full_master():
    shutil.rmtree(R + "/master", ignore_errors=True); os.makedirs(R + "/master")
    entries = [e for e in os.listdir(REPO) if e not in (".git", ".cache")]
    def one(e): subprocess.run(["cp", "-Rc", REPO + "/" + e, R + "/master/" + e], check=True)
    with ThreadPoolExecutor(6) as ex: list(ex.map(one, entries))
    return len(entries)

def make_master():
    shutil.rmtree(R + "/master", ignore_errors=True); os.makedirs(R + "/master")
    shutil.copy(REPO + "/.gitignore", R + "/master/.gitignore")
    fw = [p for p in glob.glob(REPO + "/*framework") if os.path.isdir(p)][0]
    subprocess.run(["cp", "-Rc", fw, R + "/master/" + os.path.basename(fw)], check=True)
    return prune_master()

def sync_code():
    shutil.rmtree(R + "/master/" + MODREL, ignore_errors=True)
    subprocess.run(["cp", "-Rc", os.environ.get("FLEET_MODSRC", REPO + "/" + MODREL), R + "/master/" + MODREL], check=True)

def clone(o):
    shutil.rmtree(tree(o), ignore_errors=True)
    subprocess.run(["cp", "-Rc", R + "/master", tree(o)], check=True)

def build():
    t = {}
    t0 = time.time()
    for d in ("trees", "stores", "logs"): shutil.rmtree(f"{R}/{d}", ignore_errors=True); os.makedirs(f"{R}/{d}")
    save_pids({})
    if not os.path.isdir(R + "/master"): print("master", make_master())
    else: sync_code()
    for g in group_ids(): os.makedirs(store(g))
    with ThreadPoolExecutor(8) as ex: list(ex.map(clone, all_origins()))
    t["clone"] = time.time() - t0; t0 = time.time()
    seed = f"{group_ids()[0]}-s"
    run(seed, "seed", "--group", "g1", "--origin", seed, "--store", store("g1"), "--worktree", tree(seed), capture=True)
    run(seed, "snapshot", capture=True)
    t["seed"] = time.time() - t0; t0 = time.time()
    for g in group_ids()[1:]:
        run(seed, "relay", "--group", g, "--store", store(g), "--peers", f"g1={store('g1')}", "--worktree", R + "/relay-none", "--once", capture=True)
    t["relay"] = time.time() - t0; t0 = time.time()
    def join(o):
        g = group_of(o)
        if o == seed: return
        run(o, "join", "--group", g, "--origin", o, "--store", store(g), "--worktree", tree(o), capture=True)
    with ThreadPoolExecutor(6) as ex: list(ex.map(join, all_origins()))
    t["join"] = time.time() - t0
    print(json.dumps(t))

def start_all():
    for o in all_origins(): start_daemon(o)
    for g in group_ids(): start_relay(g)

if __name__ == "__main__":
    c = sys.argv[1]
    if c == "build": build()
    elif c == "master": print(make_master())
    elif c == "full-master": print(make_full_master())
    elif c == "start": start_all()
    elif c == "stop": stop(sys.argv[2:] or None)
    elif c == "kill": kill9(sys.argv[2])
    elif c == "restart":
        n = sys.argv[2]; (start_relay(n[6:]) if n.startswith("relay-") else start_daemon(n))
    elif c == "pids": print(json.dumps(pids()))
