#!/usr/bin/env python3
"""usage: fleet-sample.py OUTDIR INTERVAL [name=pid ...] -- every INTERVAL s: rss and cpu seconds of every pid in pids.json (plus extras) into samples.csv; every 3rd round also store size and journal segment counts into stores.csv and host load/free memory into host.csv. Stops when OUTDIR/stop exists."""
import glob, os, subprocess, sys, time
import fleet

outdir, interval = sys.argv[1], float(sys.argv[2]); extra = dict(a.split("=") for a in sys.argv[3:])
os.makedirs(outdir, exist_ok=True)
sam = open(f"{outdir}/samples.csv", "a", buffering=1); sto = open(f"{outdir}/stores.csv", "a", buffering=1); host = open(f"{outdir}/host.csv", "a", buffering=1)
sam.write("t,name,pid,rss_kb,cpu_s,log_bytes\n"); sto.write("t,group,kb,journal_kb,segments,origins\n"); host.write("t,load1,free_mb,swap_used_mb,total_rss_mb\n")
def secs(s):
    parts = s.split(":"); v = float(parts[-1]) + 60 * int(parts[-2])
    return v + (3600 * int(parts[-3]) if len(parts) > 2 else 0)
t0 = time.time(); rounds = 0
while not os.path.exists(f"{outdir}/stop"):
    t = round(time.time() - t0, 1); names = {**{n: p for n, p in fleet.pids().items()}, **{n: int(p) for n, p in extra.items()}}
    by = {p: n for n, p in names.items()}
    ps = subprocess.run(["ps", "-o", "pid=,rss=,cputime=", "-p", ",".join(map(str, by))], capture_output=True, text=True).stdout.split("\n")
    total = 0
    for line in ps:
        f = line.split()
        if len(f) == 3:
            nm = by[int(f[0])]
            try: lb = os.stat(f"{fleet.R}/logs/{nm}.out").st_size
            except OSError: lb = 0
            sam.write(f"{t},{nm},{f[0]},{f[1]},{secs(f[2])},{lb}\n"); total += int(f[1])
    if rounds % 3 == 0:
        for g in fleet.group_ids():
            s = fleet.store(g)
            try:
                kb = int(subprocess.run(["du", "-sk", s], capture_output=True, text=True).stdout.split()[0]); jk = int(subprocess.run(["du", "-sk", s + "/v1/origins"], capture_output=True, text=True).stdout.split()[0])
                segs = len(glob.glob(s + "/v1/origins/*/segments/*.jsonl")); orig = len(glob.glob(s + "/v1/origins/*/head.json"))
                sto.write(f"{t},{g},{kb},{jk},{segs},{orig}\n")
            except (OSError, IndexError): pass
        vm = subprocess.run(["vm_stat"], capture_output=True, text=True).stdout
        free = int(vm.split("Pages free:")[1].split()[0].rstrip(".")) * 16384 // 1048576 + int(vm.split("Pages inactive:")[1].split()[0].rstrip(".")) * 16384 // 1048576
        swap = subprocess.run(["sysctl", "-n", "vm.swapusage"], capture_output=True, text=True).stdout.split("used = ")[1].split("M")[0]
        host.write(f"{t},{os.getloadavg()[0]:.2f},{free},{swap},{total // 1024}\n")
    rounds += 1; time.sleep(max(0, interval - 0.3))
