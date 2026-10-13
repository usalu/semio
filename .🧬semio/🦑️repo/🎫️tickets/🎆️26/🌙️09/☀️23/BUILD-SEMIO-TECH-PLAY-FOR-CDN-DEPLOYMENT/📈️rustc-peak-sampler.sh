#!/bin/bash
# Samples every 2 s the RSS of each running rustc by crate name (keeping per-crate peaks) plus machine CPU, rustc count and swap.
exec python3 - <<'PY'
import subprocess, time, os
base = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/"
peaks = {}
while True:
    out = subprocess.run(["ps", "-Ao", "rss=,pcpu=,command="], capture_output=True, text=True).stdout.splitlines()
    cpu = 0.0; n = 0
    for line in out:
        parts = line.split(None, 2)
        if len(parts) < 3: continue
        try: rss, pc, cmd = int(parts[0]), float(parts[1]), parts[2]
        except ValueError: continue
        cpu += pc
        toks = cmd.split()
        if toks and toks[0].endswith("/rustc") and "--crate-name" in toks:
            n += 1
            name = toks[toks.index("--crate-name") + 1]
            peaks[name] = max(peaks.get(name, 0), rss)
    with open(base + "rustc-peaks.tsv.tmp", "w") as f:
        for k, v in sorted(peaks.items(), key=lambda kv: -kv[1]): f.write(f"{v/1048576:.2f}G\t{k}\n")
    os.replace(base + "rustc-peaks.tsv.tmp", base + "rustc-peaks.tsv")
    swap = subprocess.run(["sysctl", "-n", "vm.swapusage"], capture_output=True, text=True).stdout.split()[5]
    with open(base + "cpu-timeline.tsv", "a") as f: f.write(f"{time.strftime('%H:%M:%S')}\t{cpu:.0f}\t{n}\t{swap}\n")
    time.sleep(2)
PY
