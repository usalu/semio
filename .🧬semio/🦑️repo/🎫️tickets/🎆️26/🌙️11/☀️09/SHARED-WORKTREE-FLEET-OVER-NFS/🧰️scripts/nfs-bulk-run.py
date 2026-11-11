#!/usr/bin/env python3
"""usage: nfs-bulk-run.py DIRECTION -- host-side driver (Lima). DIRECTION w2s: writer is w1 on the worker, readers w2, w3 (same machine) and the server tree; s2w: writer is the server tree, readers w1, w2, w3. Runs, one after the other: 2000-file burst, delete of it, rename of a 500-file directory, exec bit, long 4-byte emoji path. Prints one JSON line per operation with the ms until each reader saw it (clock offsets measured with nfs-clock.py before and after)."""
import json, os, subprocess, sys, time

LIMA = {"LIMA_HOME": os.path.expanduser("~/.semio-fleet-vm")}
direction = sys.argv[1]
ONLY = set(sys.argv[2:]) or {"burst2000", "delete", "rename500", "exec", "long"}
TREES = {"w1": ("wrk", "/srv/semio"), "w2": ("wrk", "/srv/w2"), "w3": ("wrk", "/srv/w3"), "srv": ("srv", "/srv/semio")}
writer = "w1" if direction == "w2s" else "srv"
readers = ["w2", "w3", "srv"] if direction == "w2s" else ["w1", "w2", "w3"]

def shell(vm, *cmd):
    return subprocess.Popen(["limactl", "shell", vm, "--", "sudo", "-u", "semio", *cmd], env={**os.environ, **LIMA}, stdout=subprocess.PIPE, text=True)

def offset():
    out = subprocess.run(["limactl", "shell", "wrk", "--", "python3", "/tmp/nfs-clock.py", "measure", "192.168.104.1", "7777", "600"], env={**os.environ, **LIMA}, capture_output=True, text=True).stdout
    return json.loads(out)["offset_ms"]

def run(kind, role, name, *args):
    vm, tree = TREES[name]
    return shell(vm, "python3", "/tmp/nfs-bulk.py", kind, role, tree, *args)

def result(p):
    return json.loads(p.communicate()[0].strip().splitlines()[-1])

def skew(name, off):
    return off / 1000.0 if TREES[name][0] == "srv" else 0.0

def measure(kind, writer_args, reader_args, writer_role="W"):
    off1 = offset()
    procs = {r: run(kind, "R", r, *reader_args) for r in readers}
    time.sleep(2)
    w = result(run(kind, writer_role, writer, *writer_args))
    seen = {r: result(p) for r, p in procs.items()}
    off = (off1 + offset()) / 2
    lat = {}
    for r, v in seen.items():
        s = v.get("seen")
        lat[r] = None if s is None else round((s - skew(r, off) - (w["t0"] - skew(writer, off))) * 1000)
    return {"writer": w, "latency_ms": lat, "offset_ms": round(off, 1)}

N = 2000
out = {"direction": direction, "writer": writer, "readers": readers}

def report(name, value):
    out[name] = value; print(json.dumps({name: value}, ensure_ascii=False), flush=True)

def rename500():
    result(run("rename", "W", writer, "500"))
    ready = {r: run("rename", "P", r, "500", "120") for r in readers}
    for p in ready.values(): result(p)
    time.sleep(3)
    off1 = offset(); procs = {r: run("rename", "R", r, "500", "120") for r in readers}; time.sleep(1)
    m = result(run("rename", "M", writer, "500"))
    seen = {r: result(p) for r, p in procs.items()}; off = (off1 + offset()) / 2
    return {"latency_ms": {r: (None if v.get("seen") is None else round((v["seen"] - skew(r, off) - (m["t0"] - skew(writer, off))) * 1000)) for r, v in seen.items()}, "offset_ms": round(off, 1)}

if "burst2000" in ONLY: report("burst2000", measure("create", ["nfs-burst", str(N)], ["nfs-burst", str(N), "180"]))
if "delete" in ONLY: report("delete", measure("delete", ["nfs-burst"], ["nfs-burst", "180"]))
if "rename500" in ONLY: report("rename500", rename500())
if "exec" in ONLY: report("exec", measure("exec", ["3"], ["60"]))
if "long" in ONLY: report("long", measure("long", [], ["120"]))
