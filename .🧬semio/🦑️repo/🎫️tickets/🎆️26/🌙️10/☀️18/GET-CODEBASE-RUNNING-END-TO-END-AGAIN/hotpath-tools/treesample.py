"""🔬 Read-only observer of the coordinator's measured product runs: `treesample.py <label-substring> <out.jsonl> [max-seconds]`.

Waits for a `product.py` whose argv names a label containing the substring, then samples its process tree every 0.2 s with `ps` until it exits and appends one JSON line per run: per process (role by position: door, executor, graph child, planner, bazel, other) the first/last seen time, last CPU seconds and peak RSS, plus the tree peak and the per-role RSS at that peak. Starts nothing, kills nothing; keeps one row per pid; aborts above 300 MB of its own RSS."""
import json, os, re, resource, subprocess, sys, time

LABEL, OUT = sys.argv[1], sys.argv[2]
LIMIT = float(sys.argv[3]) if len(sys.argv) > 3 else 540.0
ROW = re.compile(r"\s*(\d+)\s+(\d+)\s+(\S+)\s+(\d+)\s+(.*)$")


def seconds(text):
    days, _, text = text.rpartition("-")
    total = 0.0
    for part in text.split(":"):
        total = total * 60 + float(part)
    return (int(days) if days else 0) * 86400 + total


def table():
    rows = {}
    for line in subprocess.run(["ps", "-axo", "pid=,ppid=,time=,rss=,command="], capture_output=True, text=True, errors="replace").stdout.splitlines():
        m = ROW.match(line)
        if m:
            rows[int(m.group(1))] = (int(m.group(2)), seconds(m.group(3)), int(m.group(4)), m.group(5))
    return rows


def role(rows, pid):
    command = rows[pid][3]
    if command.startswith("bun repo"):
        return "launcher"
    if "door/" in command:
        return "door"
    if "--eval" in command:
        depth, up = 0, rows[pid][0]
        while up in rows and "--eval" in rows[up][3]:
            depth, up = depth + 1, rows[up][0]
        return ["executor", "graph-or-planner", "nested"][min(depth, 2)]
    if "/bun " in command:
        return "bun:" + command.split("/bun ", 1)[1][-60:]
    for key in ("bazel", "java", "repo-watch", "hdiutil", "git", "sh -c", "python"):
        if key in command:
            return key.split()[0]
    return "other:" + os.path.basename(command.split()[0])[:24]


def main():
    deadline = time.time() + LIMIT
    target = None
    while target is None and time.time() < deadline:
        for pid, (_, _, _, command) in table().items():
            words = command.split()
            if any(w.endswith("product.py") and i + 1 < len(words) and words[i + 1] == LABEL for i, w in enumerate(words)) and not command.startswith("sh "):
                target = pid
                break
        else:
            time.sleep(0.5)
    if target is None:
        print(json.dumps({"label": LABEL, "found": False}))
        return
    t0, procs, peak, peak_roles, label = time.time(), {}, 0, {}, None
    while time.time() < deadline:
        rows = table()
        if target not in rows:
            break
        label = label or LABEL
        kids, mine, stack = {}, set(), [target]
        for p, r in rows.items():
            kids.setdefault(r[0], []).append(p)
        while stack:
            pid = stack.pop()
            mine.add(pid)
            stack.extend(kids.get(pid, []))
        now, total, roles = time.time() - t0, 0, {}
        for pid in mine - {target}:
            ppid, cpu, rss, command = rows[pid]
            r = procs.setdefault(pid, {"role": role(rows, pid), "first": round(now, 2), "cpu": 0.0, "rss_mb": 0, "command": command[:60] + " ... " + command[-100:]})
            r["last"], r["cpu"], r["rss_mb"] = round(now, 2), max(r["cpu"], cpu), max(r["rss_mb"], rss // 1024)
            total += rss
            roles[r["role"]] = roles.get(r["role"], 0) + rss // 1024
        if total > peak:
            peak, peak_roles = total, roles
        if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 300 * 1024 * 1024:
            sys.exit("treesample: own RSS above 300 MB")
        time.sleep(0.2)
    record = {"label": label, "wall_s": round(time.time() - t0, 1), "peak_tree_mb": peak // 1024, "peak_roles_mb": peak_roles, "processes": sorted(procs.values(), key=lambda r: r["first"])}
    with open(OUT, "a", encoding="utf8") as stream:
        stream.write(json.dumps(record, ensure_ascii=False) + "\n")
    print(json.dumps({k: record[k] for k in ("label", "wall_s", "peak_tree_mb", "peak_roles_mb")}))
    for r in record["processes"]:
        print(f'  {r["role"]:<18} {r["first"]:6.1f}-{r["last"]:6.1f}s cpu {r["cpu"]:6.2f}s rss {r["rss_mb"]:5d} MB  {r["command"][:70]}')


main()
