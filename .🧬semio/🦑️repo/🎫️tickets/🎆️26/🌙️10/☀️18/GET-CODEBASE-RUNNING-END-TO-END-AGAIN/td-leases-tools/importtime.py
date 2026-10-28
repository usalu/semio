"""⏱️ [td-leases] median wall time and peak RSS of importing each light door entry in a fresh `bun -e`, N runs each, inside a root. usage: python3 -I importtime.py <root> [runs]"""
import os, resource, statistics, subprocess, sys, time
root = sys.argv[1]; runs = int(sys.argv[2]) if len(sys.argv) > 2 else 11
B = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
for e in ["🔒️leases/⌨️command", "🌡️host-capacity/⌨️command", "🩺️doctor/⌨️command", "📥️caller"]:
    ts = []
    for _ in range(runs):
        s = time.perf_counter(); subprocess.run(["bun", "-e", "await import(process.argv[1])", B + e + "/🟦️.ts"], capture_output=True); ts.append((time.perf_counter() - s) * 1000)
    print(f"{e}: median {statistics.median(ts):.0f} ms, min {min(ts):.0f} ms")
