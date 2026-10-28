"""📡 Passive fleet sampler: every N seconds one JSON line with the live Bazel servers (pid, RSS, slot), Bazel clients, workers, Bun chain RSS and swap. Reads only `ps` and `sysctl`; starts nothing."""
import json, re, subprocess, sys, time

def table():
    text = subprocess.run(["ps", "-axo", "pid=,ppid=,rss=,etime=,command="], capture_output=True, text=True, errors="replace").stdout
    rows = []
    for line in text.splitlines():
        match = re.match(r"\s*(\d+)\s+(\d+)\s+(\d+)\s+(\S+)\s+(.*)$", line)
        if match:
            rows.append((int(match[1]), int(match[2]), int(match[3]), match[4], match[5]))
    return rows

def swap():
    out = subprocess.run(["sysctl", "-n", "vm.swapusage"], capture_output=True, text=True).stdout
    used = re.search(r"used = ([\d.]+)M", out)
    return float(used[1]) if used else -1

def main():
    log, seconds, step = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    end = time.time() + seconds
    with open(log, "a", buffering=1) as handle:
        while time.time() < end:
            rows = table()
            servers, clients, workers, bun = [], 0, 0, 0
            bunRss = 0
            for pid, ppid, rss, etime, command in rows:
                if command.startswith("bazel("):
                    base = re.search(r"--output_base=(\S+)", command)
                    servers.append({"pid": pid, "rssMb": rss // 1024, "etime": etime, "slot": base[1].rsplit("/", 1)[-1] if base else "?", "mine": bool(base and "scratchpad" in base[1])})
                elif re.search(r"/bazel/bin/[0-9a-f]+/bazel (--max_idle|--host_jvm|--install_base|--output)", command) or "/bazel/bin/" in command.split(" ")[0]:
                    clients += 1
                elif "bazel-workers" in command or "--persistent_worker" in command:
                    workers += 1
                elif "/bun/" in command.split(" ")[0] or command.startswith("bun "):
                    bun += 1
                    bunRss += rss
            handle.write(json.dumps({"t": int(time.time()), "servers": servers, "serverRssMb": sum(s["rssMb"] for s in servers), "clients": clients, "workers": workers, "bun": bun, "bunRssMb": bunRss // 1024, "swapMb": swap()}) + "\n")
            time.sleep(step)

main()
