#!/usr/bin/env python3
"""📏️ Passive sampler: peak resident memory per compile or test unit of everything that runs on this machine (every agent's builds), polled with ps. Usage: 📏️process-peaks.py <out.json> [interval seconds]. Stops on SIGTERM and writes the file every 30 s."""
import json, os, re, signal, subprocess, sys, time
out = sys.argv[1]
every = float(sys.argv[2]) if len(sys.argv) > 2 else 1.0
peaks = {}
stop = False
signal.signal(signal.SIGTERM, lambda *_: globals().__setitem__("stop", True))
def unit(cmd):
    m = re.search(r"--crate-name[ =](\S+)", cmd)
    if m and "rustc" in cmd:
        kind = "rustc-test" if "--test" in cmd.split() else "rustc"
        return f"{kind}:{m.group(1)}"
    if re.search(r"(^|/)(rustc|cargo|rust-lld|ld|ld64|link|lld|wasm-ld|wasm-opt|wasm-bindgen)( |$)", cmd):
        return "tool:" + re.search(r"(rustc|cargo|rust-lld|wasm-ld|wasm-opt|wasm-bindgen|ld64|ld)( |$)", cmd).group(1)
    m = re.search(r"vitest\S* run .*?--config (\S+)", cmd)
    if m: return "vitest-main:" + m.group(1)
    if "vitest" in cmd and ("forks" in cmd or "workers" in cmd or "processChild" in cmd): return "vitest-worker:" + (re.search(r"(🧪️tests/[^/ ]+)", cmd) or [None, ""])[1]
    if re.search(r"(^|/)tsc( |$)|typescript/lib/tsc|tsgo", cmd): return "typescript:" + (re.search(r"-p (\S+)|--project (\S+)", cmd) or [None, ""])[0]
    if re.search(r"vite( |/bin|\.js)|rolldown|esbuild", cmd): return "bundler:" + (re.search(r"(vite|rolldown|esbuild)", cmd)[1])
    return None
def dump():
    json.dump(dict(sorted(peaks.items(), key=lambda kv: -kv[1]["peakMb"])), open(out + ".tmp", "w"), ensure_ascii=False, indent=1)
    os.replace(out + ".tmp", out)
last = 0
while not stop:
    now = time.time()
    for line in subprocess.run(["ps", "-axo", "pid=,rss=,command="], capture_output=True, text=True).stdout.splitlines():
        parts = line.strip().split(None, 2)
        if len(parts) < 3: continue
        pid, rss, cmd = int(parts[0]), int(parts[1]) / 1024, parts[2]
        if "process-peaks" in cmd: continue
        u = unit(cmd)
        if u is None: continue
        e = peaks.setdefault(u, {"peakMb": 0, "samples": 0, "atS": 0, "cmd": ""})
        e["samples"] += 1
        if rss > e["peakMb"]: e.update(peakMb=round(rss), atS=int(now), cmd=cmd[:300], pid=pid)
    if now - last > 30: dump(); last = now
    time.sleep(every)
dump()
