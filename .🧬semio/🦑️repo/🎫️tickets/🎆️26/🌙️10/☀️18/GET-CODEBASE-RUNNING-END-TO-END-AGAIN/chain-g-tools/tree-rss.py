#!/usr/bin/env python3
"""Samples the process tree RSS of one command (or an attached pid) and prints peak tree RSS, peak process count and per-process peaks."""
import json, os, subprocess, sys, time, signal

def snapshot():
    out = subprocess.run(["ps", "-axo", "pid=,ppid=,rss=,command="], capture_output=True, text=True).stdout
    rows = {}
    for line in out.splitlines():
        parts = line.split(None, 3)
        if len(parts) < 3:
            continue
        try:
            pid, ppid, rss = int(parts[0]), int(parts[1]), int(parts[2])
        except ValueError:
            continue
        rows[pid] = (ppid, rss, parts[3] if len(parts) > 3 else "")
    return rows

def label(command):
    if " --eval " in command or command.endswith(" --eval"):
        return "loader"
    if "door/" in command and "script.ts" in command:
        return "door"
    if "bazel" in command and "install_base" in command:
        return "bazel-server"
    head = command.split()[0] if command else "?"
    return os.path.basename(head)[:40] + " " + " ".join(command.split()[1:3])[:60]

def descendants(rows, root):
    children = {}
    for pid, (ppid, _, _) in rows.items():
        children.setdefault(ppid, []).append(pid)
    found, stack = [], [root]
    while stack:
        pid = stack.pop()
        if pid in rows:
            found.append(pid)
        stack.extend(children.get(pid, []))
    return found

def main():
    args = sys.argv[1:]
    interval, out, attach = 0.25, None, None
    while args and args[0].startswith("--") and args[0] != "--":
        flag = args.pop(0)
        if flag == "--interval":
            interval = float(args.pop(0))
        elif flag == "--out":
            out = args.pop(0)
        elif flag == "--attach":
            attach = int(args.pop(0))
    if args and args[0] == "--":
        args.pop(0)
    proc = None
    if attach is None:
        proc = subprocess.Popen(args, start_new_session=True)
        root = proc.pid
    else:
        root = attach
    peak_total, peak_count, per, series = 0, 0, {}, {}
    timeline, start = [], time.time()
    stop_at = None
    while True:
        rows = snapshot()
        members = descendants(rows, root)
        total = sum(rows[p][1] for p in members)
        peak_total = max(peak_total, total)
        peak_count = max(peak_count, len(members))
        timeline.append([round(time.time() - start, 2), len(members), total // 1024])
        for p in members:
            if rows[p][1] // 1024 >= 50:
                series.setdefault(p, []).append([round(time.time() - start, 2), rows[p][1] // 1024])
            entry = per.setdefault(p, {"label": label(rows[p][2]), "ppid": rows[p][0], "peak_mb": 0, "last_mb": 0, "first": round(time.time() - start, 2), "last": 0})
            entry["peak_mb"] = max(entry["peak_mb"], rows[p][1] // 1024)
            entry["last_mb"] = rows[p][1] // 1024
            entry["last"] = round(time.time() - start, 2)
        if proc is not None and proc.poll() is not None:
            break
        if attach is not None and attach not in rows:
            break
        time.sleep(interval)
    summary = {"wall_s": round(time.time() - start, 2), "peak_tree_mb": peak_total // 1024, "peak_process_count": peak_count,
               "exit": proc.returncode if proc else None, "processes": per, "timeline": timeline, "series": series}
    if out:
        with open(out, "w") as handle:
            json.dump(summary, handle)
    print(f"wall {summary['wall_s']}s  peak tree RSS {summary['peak_tree_mb']} MB  peak processes {peak_count}  exit {summary['exit']}")
    for pid, entry in sorted(per.items(), key=lambda item: item[1]["first"]):
        print(f"  pid {pid:>6} ppid {entry['ppid']:>6} peak {entry['peak_mb']:>5} MB last {entry['last_mb']:>5} MB  t {entry['first']:>7}..{entry['last']:<7} {entry['label']}")

main()
