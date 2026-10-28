#!/usr/bin/env python3
"""Starts one `bun repo` run under the tree sampler, waits until it queues for the graph builder custody, samples it for --hold seconds and cancels it with SIGINT (only its own `bun repo` process)."""
import os, signal, subprocess, sys, time
here = os.path.dirname(os.path.abspath(__file__))
hold, out, log = 20.0, None, None
args = sys.argv[1:]
while args and args[0].startswith("--"):
    flag = args.pop(0)
    if flag == "--hold": hold = float(args.pop(0))
    elif flag == "--out": out = args.pop(0)
    elif flag == "--log": log = args.pop(0)
    elif flag == "--": break
if args and args[0] == "--": args.pop(0)
sampler = subprocess.Popen([sys.executable, os.path.join(here, "tree-rss.py"), "--interval", "0.5"] + (["--out", out] if out else []) + ["--", "sh", "-c", " ".join(args) + " > /dev/null 2> " + log])
seen = None
deadline = time.time() + 1500
while time.time() < deadline and sampler.poll() is None:
    try:
        text = open(log, errors="ignore").read()
    except FileNotFoundError:
        text = ""
    if "Waiting for access to repository-graph-facts" in text:
        seen = time.time(); break
    time.sleep(2)
if seen:
    time.sleep(hold)
kids = subprocess.run(["pgrep", "-P", str(sampler.pid)], capture_output=True, text=True).stdout.split()
for sh in kids:
    for run in subprocess.run(["pgrep", "-P", sh], capture_output=True, text=True).stdout.split():
        try: os.kill(int(run), signal.SIGINT)
        except ProcessLookupError: pass
sampler.wait()
print("reached graph queue:", bool(seen))
