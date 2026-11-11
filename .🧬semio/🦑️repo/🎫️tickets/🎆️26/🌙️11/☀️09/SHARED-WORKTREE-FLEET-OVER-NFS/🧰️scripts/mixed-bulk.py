#!/usr/bin/env python3
"""usage: mixed-bulk.py WRITER:TREE READER:TREE[,READER:TREE...] [KIND...] -- structure-change latencies between machines (nodes: g1, g2 = Lima VMs, mac = this host). KIND: burst2000 delete rename500 exec long (default all). Clock offsets to this host are measured with nfs-clock.py (port 7777 of the VMs); nfs-bulk.py is the helper that runs on each node (/tmp of the VMs, next to this file on the host)."""
import json, os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
IPS = {"g1": "192.168.64.4", "g2": "192.168.64.5"}
args = sys.argv[1:]
writer_spec, reader_specs = args[0], args[1].split(",")
ONLY = set(args[2:]) or {"burst2000", "delete", "rename500", "exec", "long"}
TREES = {}
for i, spec in enumerate([writer_spec] + reader_specs):
    node, tree = spec.split(":", 1); TREES["w" if i == 0 else "r%d" % i] = (node, tree)
writer = "w"; readers = [k for k in TREES if k != "w"]

def shell(vm, *cmd):
    if vm == "mac": return subprocess.Popen(list(cmd), stdout=subprocess.PIPE, text=True)
    return subprocess.Popen(["limactl", "shell", vm, "--", "sudo", "-u", "semio", *cmd], stdout=subprocess.PIPE, text=True)

def node_offset(node):
    if node == "mac": return 0.0
    out = subprocess.run(["python3", os.path.join(HERE, "nfs-clock.py"), "measure", IPS[node], "7777", "600"], capture_output=True, text=True).stdout
    return json.loads(out)["offset_ms"]

def offsets():
    return {n: node_offset(n) for n in {v[0] for v in TREES.values()}}

def run(kind, role, name, *args):
    vm, tree = TREES[name]
    return shell(vm, "python3", os.path.join(HERE if vm == "mac" else "/tmp", "nfs-bulk.py"), kind, role, tree, *args)

def result(p):
    return json.loads(p.communicate()[0].strip().splitlines()[-1])

def skew(name, off):
    return off[TREES[name][0]] / 1000.0

def measure(kind, writer_args, reader_args, writer_role="W"):
    off1 = offsets()
    procs = {r: run(kind, "R", r, *reader_args) for r in readers}
    time.sleep(2)
    w = result(run(kind, writer_role, writer, *writer_args))
    seen = {r: result(p) for r, p in procs.items()}
    off2 = offsets(); off = {n: (off1[n] + off2[n]) / 2 for n in off1}
    lat = {}
    for r, v in seen.items():
        s = v.get("seen")
        lat[r] = None if s is None else round((s - skew(r, off) - (w["t0"] - skew(writer, off))) * 1000)
    return {"writer": w, "latency_ms": lat, "offset_ms": {n: round(o, 1) for n, o in off.items()}}

N = 2000
out = {"writer": writer_spec, "readers": reader_specs}

def report(name, value):
    out[name] = value; print(json.dumps({name: value}, ensure_ascii=False), flush=True)

def rename500():
    result(run("rename", "W", writer, "500"))
    ready = {r: run("rename", "P", r, "500", "120") for r in readers}
    for p in ready.values(): result(p)
    time.sleep(3)
    off1 = offsets(); procs = {r: run("rename", "R", r, "500", "120") for r in readers}; time.sleep(1)
    m = result(run("rename", "M", writer, "500"))
    seen = {r: result(p) for r, p in procs.items()}; off2 = offsets(); off = {n: (off1[n] + off2[n]) / 2 for n in off1}
    return {"latency_ms": {r: (None if v.get("seen") is None else round((v["seen"] - skew(r, off) - (m["t0"] - skew(writer, off))) * 1000)) for r, v in seen.items()}, "offset_ms": {n: round(o, 1) for n, o in off.items()}}

if "burst2000" in ONLY: report("burst2000", measure("create", ["nfs-burst", str(N)], ["nfs-burst", str(N), "180"]))
if "delete" in ONLY: report("delete", measure("delete", ["nfs-burst"], ["nfs-burst", "180"]))
if "rename500" in ONLY: report("rename500", rename500())
if "exec" in ONLY: report("exec", measure("exec", ["3"], ["60"]))
if "long" in ONLY: report("long", measure("long", [], ["120"]))
