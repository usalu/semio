"""threads-watch.py -- <cmd...> : runs cmd and records every distinct `-Z threads=N` value seen on a running rustc/scheduler command line (ps), plus SEMIO_COMPILER_PARALLELISM is not visible in ps, so the cargo log line `N of M cores admitted` is read from the log separately."""
import subprocess, sys, time, re, collections
cmd = sys.argv[sys.argv.index("--") + 1:]
proc = subprocess.Popen(cmd)
seen = collections.Counter()
while proc.poll() is None:
    for line in subprocess.run(["ps", "-axo", "command="], capture_output=True, text=True).stdout.splitlines():
        if "--crate-name" in line:
            first = line.split()[0].rsplit("/", 1)[-1]
            for m in re.finditer(r"-Z ?threads=([0-9]+)", line):
                seen[(first, m.group(1))] += 1
    time.sleep(0.3)
print("rc", proc.returncode, "(process, threads) seen:", {f"{a}:{b}": n for (a, b), n in seen.items()})
