#!/usr/bin/env python3
"""Measures a run that waits at the capacity gate: a private gate directory holds one fake grant that fills the host, the run waits for --wait seconds under the tree sampler, then it is cancelled (SIGINT to the run only)."""
import json, os, signal, subprocess, sys, time, tempfile, re

here = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
wait = 12.0
out = None
while args and args[0].startswith("--"):
    flag = args.pop(0)
    if flag == "--wait":
        wait = float(args.pop(0))
    elif flag == "--out":
        out = args.pop(0)
    elif flag == "--":
        break
if args and args[0] == "--":
    args.pop(0)

directory = tempfile.mkdtemp(prefix="chain-g-cap-", dir=os.environ.get("TMPDIR", "/tmp"))
os.makedirs(os.path.join(directory, "grants"), exist_ok=True)
os.makedirs(os.path.join(directory, "queue"), exist_ok=True)
holder = subprocess.Popen(["sleep", "3600"])
boot = subprocess.run(["sysctl", "-n", "kern.boottime"], capture_output=True, text=True).stdout
sec = int(re.search(r"sec = (\d+)", boot).group(1))
now = int(time.time() * 1000)
grant = {"id": "chaingfake0001", "operation": "run", "jobs": 1, "cpu": 1, "reservedMegabytes": 60000, "admittedAtMs": now,
         "observedMegabytes": None, "pid": holder.pid, "bootAtMs": round(sec) * 1000}
with open(os.path.join(directory, "grants", grant["id"] + ".json"), "w") as handle:
    json.dump(grant, handle)
env = dict(os.environ, SEMIO_HOST_CAPACITY_DIRECTORY=directory)
sampler = subprocess.Popen([sys.executable, os.path.join(here, "tree-rss.py"), "--interval", "0.25"] + (["--out", out] if out else []) + ["--"] + args, env=env)
time.sleep(wait)
children = subprocess.run(["pgrep", "-P", str(sampler.pid)], capture_output=True, text=True).stdout.split()
for child in children:
    try:
        os.kill(int(child), signal.SIGINT)
    except ProcessLookupError:
        pass
sampler.wait()
holder.terminate()
holder.wait()
subprocess.run(["rm", "-rf", directory])
