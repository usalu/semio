import json, os, subprocess, sys, time
ROOT = "/Users/ueli/Documents/semio"
T = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E9"
BUN = "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
budget = float(sys.argv[1]); out = sys.argv[2] if len(sys.argv) > 2 else "results.jsonl"
files = [l.rstrip("\n") for l in open(T + "/all.txt") if l.strip() and l.endswith(".ts\n")]
done = set()
if os.path.exists(T + "/" + out):
    for l in open(T + "/" + out): done.add(json.loads(l)["file"])
env = {k: v for k, v in os.environ.items() if k not in ("SEMIO_FLEET_GROUP","SEMIO_FLEET_NFS_HOST","SEMIO_FLEET_NETWORK","SEMIO_REMOTE_CACHE_ENDPOINT","SEMIO_REMOTE_CACHE_ACCESS","SEMIO_REMOTE_CACHE_INSTANCE","SEMIO_REMOTE_CACHE_TOKEN")}
env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"] = "0"
env.setdefault("SEMIO_TEST_ARTIFACT_DIR", "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-kfaIfm/ccee8eb5-3363-4a84-af9e-53e38d236627/scratchpad/E9/artifacts")
os.makedirs(env["SEMIO_TEST_ARTIFACT_DIR"], exist_ok=True)
start = time.time(); n = 0
for f in files:
    if f in done: continue
    if time.time() - start > budget: break
    while os.getloadavg()[0] > 40 and time.time() - start < budget: time.sleep(20)
    d, b = os.path.split(ROOT + "/" + f)
    t0 = time.time()
    try:
        p = subprocess.run([BUN, "test", "./" + b, "--timeout", "180000"], cwd=d, env=env, capture_output=True, text=True, errors="replace", timeout=300)
        rc, txt = p.returncode, p.stdout + p.stderr
    except subprocess.TimeoutExpired as e:
        rc, txt = -9, "WALL-TIMEOUT 300s"
    el = time.time() - t0
    open(T + "/logs.txt", "a").write("=== %s rc=%s %.1fs\n%s\n" % (f, rc, el, txt[-3000:] if rc else txt[-300:]))
    open(T + "/" + out, "a").write(json.dumps({"file": f, "rc": rc, "sec": round(el, 1)}, ensure_ascii=False) + "\n")
    n += 1
print("ran", n, "remaining", len([f for f in files if f not in done]) - n)
