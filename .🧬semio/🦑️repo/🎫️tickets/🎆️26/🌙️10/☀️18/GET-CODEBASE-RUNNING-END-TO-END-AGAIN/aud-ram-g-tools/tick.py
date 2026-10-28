"""📡 One read-only RAM/CPU census tick over repo-related processes (ps, vm_stat, sysctl, memory_pressure)."""
import json, os, re, subprocess, sys, time
from datetime import datetime

REPO = "/Users/ueli/Documents/semio"
TOOL_NAMES = {"bun", "bazel", "cargo", "rustc", "rustdoc", "rust-analyzer", "java", "vite", "nx", "playwright", "proctor", "esbuild", "tsc"}
SCAN_NAMES = {"rg", "grep", "bfs", "du", "find", "tee"}
SHELLS = {"sh", "bash", "zsh"}
TEST_ROLES = {"test-runner", "browser", "test-server"}
ACTION_OVERRIDE_EXCLUDE = TEST_ROLES
LANE_RE = re.compile(r"/([a-z0-9][a-z0-9-]*)-tools/")
WINDOW = 5.0
ROLE_DEFS = "wrapper loader script graph-child nx action bazel-server bazel-client cargo-rustc test-runner browser test-server vite node scan-tool lane-script other bun-other".split()


def secs(s):
    days = 0
    if "-" in s:
        d, s = s.split("-", 1)
        days = int(d)
    total = 0.0
    for part in s.split(":"):
        total = total * 60 + float(part)
    return days * 86400 + total


def snap():
    out = subprocess.run(["ps", "-axwwo", "pid=,ppid=,etime=,rss=,time=,command="], capture_output=True, check=True).stdout.decode("utf-8", "replace")
    procs = {}
    for line in out.splitlines():
        f = line.split(None, 5)
        if len(f) < 5:
            continue
        pid = int(f[0])
        procs[pid] = {"pid": pid, "ppid": int(f[1]), "age": secs(f[2]), "rss": int(f[3]), "cpu_s": secs(f[4]), "cmd": f[5] if len(f) > 5 else ""}
    return procs


def base(cmd):
    return os.path.basename(cmd.split(None, 1)[0]) if cmd else ""


def repo_related(p):
    c = p["cmd"]
    if "Claude.app" in c or ".claude/shell-snapshots" in c or "aud-ram-g-tools" in c:
        return False
    if REPO in c or "/.cache/b/" in c:
        return True
    if base(c) in SHELLS and "-tools/" in c and "tickets/" in c:
        return True
    b = base(c)
    return b in TOOL_NAMES or b.startswith("bazel(")


def role_of(c):
    b = base(c)
    args = c.split()
    if b.startswith("bazel("):
        return "bazel-server"
    if b == "bazel":
        return "bazel-client"
    if b == "java":
        return "bazel-server"
    if b in ("cargo", "rustc", "rustdoc", "rust-analyzer"):
        return "cargo-rustc"
    if b == "proctor":
        return "test-server"
    if b in SCAN_NAMES:
        return "scan-tool"
    if b in SHELLS and "-tools/" in c:
        return "lane-script"
    if "ms-playwright" in c or "chrome-headless" in c:
        return "browser"
    if b == "node" and "playwright" in c:
        return "test-runner"
    if b in ("node", "bun") and "vite" in c:
        return "vite"
    if b == "nx" or "/nx/" in c or "nx daemon" in c:
        return "nx"
    if b == "node":
        return "node"
    if b == "bun":
        a1 = args[1] if len(args) > 1 else ""
        if a1 in ("repo", "exec"):
            return "wrapper"
        if a1 == "--eval":
            return "loader"
        if "graph" in c.lower():
            return "graph-child"
        if a1.startswith(".") or "/" in a1:
            return "script"
        return "bun-other"
    return "other"


def lane_of_chain(chain, procs):
    for q in chain:
        m = LANE_RE.search(procs[q]["cmd"])
        if m and base(procs[q]["cmd"]) in SHELLS:
            return m.group(1)
    for q in chain:
        m = LANE_RE.search(procs[q]["cmd"])
        if m:
            return m.group(1)
    return None


def chain_of(pid, procs):
    out, q = [], pid
    while q in procs and q != 1 and len(out) < 64:
        out.append(q)
        q = procs[q]["ppid"]
    return out


def short(c):
    return c.replace(REPO, "$R")[:170]


def vm_info():
    info = {}
    for line in subprocess.run(["vm_stat"], capture_output=True).stdout.decode("utf-8", "replace").splitlines():
        m = re.match(r"^(.*?):\s+(\d+)\.?$", line.strip())
        if m:
            info[m.group(1).strip('" ')] = int(m.group(2))
    sw = subprocess.run(["sysctl", "-n", "vm.swapusage"], capture_output=True).stdout.decode()
    m = re.search(r"total = ([\d.]+)M\s+used = ([\d.]+)M", sw)
    if m:
        info["swap_total_mb"], info["swap_used_mb"] = float(m.group(1)), float(m.group(2))
    mp = subprocess.run(["memory_pressure", "-Q"], capture_output=True).stdout.decode("utf-8", "replace")
    m = re.search(r"free percentage: (\d+)%", mp)
    if m:
        info["mem_free_pct"] = int(m.group(1))
    return info


def main():
    out_dir, idx, total = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    a = snap()
    time.sleep(WINDOW)
    b = snap()
    me = os.getpid()
    cand = {}
    for pid, p in b.items():
        if pid == me or not repo_related(p):
            continue
        cand[pid] = p
    rows = {}
    for pid, p in cand.items():
        if pid in a:
            cpu = max(0.0, p["cpu_s"] - a[pid]["cpu_s"]) / WINDOW * 100
        else:
            cpu = p["cpu_s"] / max(p["age"], 1.0) * 100
        role = role_of(p["cmd"])
        chain = chain_of(pid, b)
        rows[pid] = {"pid": pid, "ppid": p["ppid"], "role": role, "rss": p["rss"], "cpu": round(cpu, 1), "age": int(p["age"]), "cmd": p["cmd"], "chain": chain}
    bazel_pids = {pid for pid, r in rows.items() if r["role"] == "bazel-server"}
    for pid, r in rows.items():
        if r["role"] not in ACTION_OVERRIDE_EXCLUDE and any(q in bazel_pids for q in r["chain"][1:]):
            r["role"] = "action"
    client_lanes = {}
    for pid, r in rows.items():
        if r["role"] == "bazel-client":
            client_lanes.setdefault("set", set()).add(lane_of_chain(r["chain"], b) or "unattributed")
    shared_lane = next(iter(client_lanes.get("set", {"bazel-shared"}))) if len(client_lanes.get("set", set())) == 1 else "bazel-shared"
    for pid, r in rows.items():
        lane = lane_of_chain(r["chain"], b)
        top = None
        for q in r["chain"]:
            if q in rows:
                top = q
        if lane is None:
            if any(q in bazel_pids for q in r["chain"]):
                lane = shared_lane
            elif any(base(b[q]["cmd"]) == "claude" or "/claude" in b[q]["cmd"].split(None, 1)[0] for q in r["chain"]):
                lane = "direct-claude-shell"
            else:
                lane = "orphan"
        r["lane"] = lane
        r["root"] = top if top is not None else pid
    roles = {}
    for r in rows.values():
        s = roles.setdefault(r["role"], {"n": 0, "rss_kb": 0, "idle_n": 0, "idle_rss_kb": 0})
        s["n"] += 1
        s["rss_kb"] += r["rss"]
        if r["cpu"] < 1.0:
            s["idle_n"] += 1
            s["idle_rss_kb"] += r["rss"]
    runs = {}
    for r in rows.values():
        key = f'{r["lane"]}|{r["root"]}'
        rr = runs.setdefault(key, {"lane": r["lane"], "root": r["root"], "root_cmd": short(rows[r["root"]]["cmd"])[:110] if r["root"] in rows else "", "size": 0, "rss_kb": 0, "cpu": 0.0, "root_age": 0})
        rr["size"] += 1
        rr["rss_kb"] += r["rss"]
        rr["cpu"] = round(rr["cpu"] + r["cpu"], 1)
        if r["root"] in rows:
            rr["root_age"] = rows[r["root"]]["age"]
    procs = [[r["pid"], r["ppid"], r["role"], r["lane"], r["root"], r["rss"], r["cpu"], r["age"], short(r["cmd"])] for r in sorted(rows.values(), key=lambda x: -x["rss"])]
    lanes = {}
    for r in rows.values():
        s = lanes.setdefault(r["lane"], {"n": 0, "rss_kb": 0})
        s["n"] += 1
        s["rss_kb"] += r["rss"]
    rec = {
        "idx": idx, "ts": datetime.now().strftime("%H:%M:%S"), "n": len(rows),
        "rss_kb": sum(r["rss"] for r in rows.values()),
        "idle_n": sum(1 for r in rows.values() if r["cpu"] < 1.0),
        "idle_rss_kb": sum(r["rss"] for r in rows.values() if r["cpu"] < 1.0),
        "roles": roles, "runs": list(runs.values()), "lanes": lanes, "vm": vm_info(), "procs": procs,
    }
    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, "samples.jsonl"), "a", encoding="utf-8") as fh:
        fh.write(json.dumps(rec, ensure_ascii=False) + "\n")
    vm = rec["vm"]
    print(f'{rec["ts"]} tick {idx}/{total}: repo={rec["n"]} rss={rec["rss_kb"]/1048576:.2f}GB idle={rec["idle_n"]} '
          f'swap_used={vm.get("swap_used_mb", 0):.0f}MB compressor={vm.get("Pages occupied by compressor", 0)*16384/1073741824:.2f}GB '
          f'free={vm.get("mem_free_pct", -1)}% runs={len(runs)} lanes={",".join(sorted(lanes))[:200]}', flush=True)


if __name__ == "__main__":
    main()
