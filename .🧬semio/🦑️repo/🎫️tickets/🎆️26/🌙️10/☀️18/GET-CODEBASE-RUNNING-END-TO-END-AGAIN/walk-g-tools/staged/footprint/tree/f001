"""🐍️ Independent oracle of the memory footprint observers (Python standard library only): parses the recorded and authored outputs of the platform tools and sums the footprint of a process tree the way the observers must. usage: python3 -I oracle.py <fixtures.json> -> JSON on stdout."""
import json
import re
import sys

MEGABYTE = 1048576


def footprint(text):
    found, pid = {}, None
    for line in text.splitlines():
        header = re.search(r"\[(\d+)\]: [^\[]*?Footprint: (\d+) B", line)
        if header:
            pid = int(header.group(1))
            found[pid] = int(header.group(2))
            continue
        kernel = re.fullmatch(r"\s*phys_footprint: (\d+) B\s*", line)
        if kernel and pid is not None:
            found[pid] = int(kernel.group(1))
    return found


def smaps(text, fields):
    values = []
    for field in fields:
        match = re.search(r"^" + re.escape(field) + r":\s+(\d+) kB\s*$", text, re.M)
        values.append(int(match.group(1)) if match else None)
    return None if values[0] is None else sum(v or 0 for v in values)


def table(text):
    return [tuple(int(x) for x in m.groups()) for m in (re.fullmatch(r"\s*(\d+)\s+(\d+)\s+(\d+)\s*", line) for line in text.splitlines()) if m]


def members(rows, root):
    if not any(pid == root for pid, _, _ in rows):
        return []
    order, seen, pending = [], set(), [root]
    while pending:
        pid = pending.pop()
        if pid in seen:
            continue
        seen.add(pid)
        order.append(pid)
        pending.extend(child for child, parent, _ in rows if parent == pid)
    return order


def darwin(ps, output, root):
    rows = table(ps)
    kids = members(rows, root)
    if not kids:
        return None
    bytes_by_pid, resident = footprint(output), {pid: kb * 1024 for pid, _, kb in rows}
    if not bytes_by_pid:
        return sum(resident[p] for p in kids) // 1024 // 1024
    return sum(bytes_by_pid.get(p, resident[p]) for p in kids) // MEGABYTE


def linux(files, listing, root, fields):
    rows = []
    for name in listing:
        if not name.isdigit():
            continue
        status = files.get("/proc/" + name + "/status")
        if status is None:
            continue
        parent = re.search(r"^PPid:\s+(\d+)\s*$", status, re.M)
        resident = re.search(r"^VmRSS:\s+(\d+)\s+kB\s*$", status, re.M)
        if parent:
            rows.append((int(name), int(parent.group(1)), int(resident.group(1)) if resident else 0))
    kids = members(rows, root)
    if not kids:
        return None
    total = 0
    for pid in kids:
        text = files.get("/proc/%d/smaps_rollup" % pid)
        value = smaps(text, fields) if text is not None else None
        total += value if value is not None else next(kb for p, _, kb in rows if p == pid)
    return total // 1024


def windows(text, root):
    rows = table(text)
    kids = members(rows, root)
    return None if not kids else sum(kb for pid, _, kb in rows if pid in kids) // 1024


def main():
    fixture = json.load(open(sys.argv[1], encoding="utf8"))
    fields = fixture["linux"]["fields"]
    report = {
        "darwin": [{"bytes": {str(k): v for k, v in footprint(case["output"]).items()}, "megabytes": darwin(case["ps"], case["output"], case["root"])} for case in fixture["darwin"]["cases"]],
        "linux": {"smaps": [smaps(text, fields) for text in fixture["linux"]["smaps"]], "trees": [linux(case["files"], case["listing"], case["root"], fields) for case in fixture["linux"]["trees"]]},
        "win32": [windows(fixture["win32"]["table"], root) for root in fixture["win32"]["roots"]],
    }
    json.dump(report, sys.stdout)


main()
