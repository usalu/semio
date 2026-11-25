"""📏️ Samples the resident size and the charged footprint (phys_footprint, what the warden cancels on) of every rustc process compiling one of the named crates until the stop file appears, and prints the peak per crate.
usage: mem-sample.py <stop-file> <out-json> <crate_name>..."""
import json, os, re, subprocess, sys, time
stop, out, names = sys.argv[1], sys.argv[2], sys.argv[3:]
peak = {n: {"rssMiB": 0, "footprintMiB": 0, "samples": 0} for n in names}
def footprint(pid):
    try:
        t = subprocess.run(["footprint", "-p", str(pid)], capture_output=True, text=True, timeout=5).stdout
    except Exception:
        return 0
    m = re.search(r"Footprint:\s*([\d.]+)\s*(KB|MB|GB)", t)
    if not m: return 0
    v = float(m.group(1)); return v * {"KB": 1 / 1024, "MB": 1, "GB": 1024}[m.group(2)]
while not os.path.exists(stop):
    ps = subprocess.run(["ps", "-axo", "pid=,rss=,command="], capture_output=True, text=True).stdout.split("\n")
    for line in ps:
        for n in names:
            if f"--crate-name {n} " in line and "rustc" in line:
                pid, rss = line.split(None, 2)[:2]
                p = peak[n]; p["samples"] += 1
                p["rssMiB"] = max(p["rssMiB"], int(rss) // 1024)
                p["footprintMiB"] = max(p["footprintMiB"], round(footprint(int(pid))))
    json.dump(peak, open(out, "w"))
    time.sleep(0.5)
json.dump(peak, open(out, "w"))
