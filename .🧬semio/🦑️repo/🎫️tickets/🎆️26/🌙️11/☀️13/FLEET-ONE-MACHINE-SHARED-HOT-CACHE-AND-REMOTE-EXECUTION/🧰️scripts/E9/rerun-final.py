import json, os, subprocess, sys, time
ROOT = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
G = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E9"
BUN = "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
src, out, budget = sys.argv[1], sys.argv[2], float(sys.argv[3])
files = [json.loads(l)["file"] for l in open(G + "/" + src) if json.loads(l)["rc"] != 0]
done = set(json.loads(l)["file"] for l in open(G + "/" + out)) if os.path.exists(G + "/" + out) else set()
env = {k: v for k, v in os.environ.items() if not k.startswith("SEMIO_FLEET") and not k.startswith("SEMIO_REMOTE_CACHE")}
env["SEMIO_TEST_ARTIFACT_DIR"] = "/private/tmp/e9a"
os.makedirs(env["SEMIO_TEST_ARTIFACT_DIR"], exist_ok=True)
start = time.time()
for f in files:
    if f in done: continue
    if time.time() - start > budget: break
    while os.getloadavg()[0] > 40 and time.time() - start < budget: time.sleep(20)
    d, b = os.path.split(ROOT + "/" + f); t0 = time.time()
    try:
        p = subprocess.run([BUN, "run", HERE + "/envelope.ts", d + "/" + b], cwd=d, env=env, capture_output=True, text=True, errors="replace", timeout=330)
        rc, txt = p.returncode, p.stdout + p.stderr
    except subprocess.TimeoutExpired:
        rc, txt = -9, "WALL-TIMEOUT"
    open(G + "/logs3.txt", "a").write("=== %s rc=%s %.1fs\n%s\n" % (f, rc, time.time() - t0, txt[-3500:] if rc else txt[-300:]))
    open(G + "/" + out, "a").write(json.dumps({"file": f, "rc": rc, "sec": round(time.time() - t0, 1)}, ensure_ascii=False) + "\n")
