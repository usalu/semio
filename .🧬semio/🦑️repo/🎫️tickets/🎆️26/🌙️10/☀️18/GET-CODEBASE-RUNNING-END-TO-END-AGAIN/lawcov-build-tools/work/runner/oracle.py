"""🧺️ Independent Python oracle of the cache-gc runner (standard library only): it re-implements the bounded tree walk, the hold probes, the undeclared-entry discovery and the collection plan from the written rules, reads the same trees through its own `os.lstat`, and answers JSON requests on stdin. `lock <file> <mode>` instead holds a real SQLite lock through Python's sqlite3 until stdin closes."""
import json, os, re, sys

FILES_STAT = os.lstat


def allocated(info):
    blocks = getattr(info, "st_blocks", 0)
    return blocks * 512 if blocks else info.st_size


def children(path):
    try:
        return sorted(os.listdir(path))
    except (FileNotFoundError, NotADirectoryError, PermissionError):
        return None


def measure(path, limit):
    import stat
    try:
        first = FILES_STAT(path)
    except FileNotFoundError:
        return {"present": False, "bytes": 0, "entries": 0, "partial": False, "sqlite": [], "queues": [], "counted": [], "recencyMs": 0}
    if not stat.S_ISDIR(first.st_mode):
        link = stat.S_ISLNK(first.st_mode)
        return {"present": True, "bytes": 0 if link else allocated(first), "entries": 1, "partial": False,
                "sqlite": [path] if path.endswith(".sqlite") else [], "queues": [], "counted": [path],
                "recencyMs": max(first.st_mtime, first.st_atime) * 1000}
    total, entries, seen, counted, sqlite, queues = 0, 0, set(), [], [], []
    recency, directories = 0.0, first.st_mtime * 1000
    every = 0
    stack = [path]
    while stack:
        directory = stack.pop()
        names = children(directory)
        if names is None:
            continue
        if directory.endswith(".queue"):
            queues.append(directory)
        for name in names:
            every += 1
            full = os.path.join(directory, name)
            info = FILES_STAT(full)
            if stat.S_ISDIR(info.st_mode):
                stack.append(full)
                directories = max(directories, info.st_mtime * 1000)
                continue
            if stat.S_ISLNK(info.st_mode):
                continue
            if info.st_nlink > 1:
                key = (info.st_dev, info.st_ino)
                if key in seen:
                    continue
                seen.add(key)
            total += allocated(info)
            counted.append(full)
            recency = max(recency, info.st_mtime * 1000, info.st_atime * 1000)
            if name.endswith(".sqlite"):
                sqlite.append(full)
    partial = every > limit
    return {"present": True, "bytes": total, "entries": limit + 1 if partial else every, "partial": partial,
            "sqlite": sorted(sqlite), "queues": sorted(queues), "counted": sorted(counted),
            "recencyMs": recency if counted else directories}


TICKET = re.compile(r"^[0-9a-z]{10}-[0-9a-z]{7}-[A-Za-z0-9._-]{1,64}$")


def held(root, alive, held_sqlite):
    info = measure(root, 10 ** 9)
    for file in info["sqlite"]:
        if os.path.relpath(file, root).replace(os.sep, "/") in held_sqlite:
            return "sqlite:" + os.path.relpath(file, root).replace(os.sep, "/")
    for queue in info["queues"]:
        for name in sorted(os.listdir(queue)):
            if TICKET.match(name):
                pid = int(name.split("-")[1], 36)
                if pid > 0 and pid in alive:
                    return "pid:" + str(pid)
    return None


def split(path):
    return [part for part in path.replace("\\", "/").split("/") if part]


def inside(parent, child):
    return len(parent) <= len(child) and child[:len(parent)] == parent


def classify(ledger, root, path):
    parts = split(path)
    if len(parts) == 1 and parts[0] in ledger["roots"][root]["ignored"]:
        return "ignored"
    for area in ledger["areas"]:
        if area["root"] != root:
            continue
        if "pattern" in area and re.search(area["pattern"], parts[0]):
            return "area"
        if "directory" not in area or area["kind"] == "nested":
            continue
        if inside(split(area["directory"]), parts):
            return "area"
    for retirement in ledger["retirements"]:
        if retirement["root"] == root and inside(split(retirement["directory"]), parts):
            return "retired"
    homes = [split(a["directory"]) for a in ledger["areas"] if a["root"] == root and a["kind"] != "nested" and "directory" in a]
    homes += [split(r["directory"]) for r in ledger["retirements"] if r["root"] == root]
    return "container" if any(inside(parts, home) for home in homes) else "undeclared"


def root_directory(workspace, ledger, root):
    return os.path.join(workspace, *split(ledger["roots"][root]["directory"]))


class Container(Exception):
    pass


def undeclared(workspace, ledger, root):
    base = root_directory(workspace, ledger, root)
    found, pending = [], [""]
    while pending:
        prefix = pending.pop(0)
        directory = os.path.join(base, *split(prefix)) if prefix else base
        names = children(directory)
        if names is None:
            if prefix and os.path.lexists(directory) and not os.path.isdir(directory):
                raise Container(root + ":" + prefix)
            continue
        for name in names:
            path = prefix + "/" + name if prefix else name
            kind = classify(ledger, root, path)
            if kind == "undeclared":
                found.append(path)
            elif kind == "container":
                pending.append(path)
    return found


def collect(workspace, ledger, now, dry_run, alive, held_sqlite):
    targets = []

    def probe(info):
        for file in info["sqlite"]:
            if os.path.relpath(file, workspace).replace(os.sep, "/") in held_sqlite:
                return "sqlite:" + os.path.relpath(file, workspace).replace(os.sep, "/")
        for queue in info["queues"]:
            for name in sorted(os.listdir(queue)):
                if TICKET.match(name):
                    pid = int(name.split("-")[1], 36)
                    if pid > 0 and pid in alive:
                        return "pid:" + str(pid)
        return None

    def observe(base, names, custody):
        units = []
        for name in names:
            info = measure(os.path.join(base, *split(name)), 5000000)
            if not info["present"]:
                continue
            units.append({"name": name, "bytes": info["bytes"], "recency": info["recencyMs"],
                          "held": None if custody == "lease" else probe(info)})
        return units

    for area in ledger["areas"]:
        collect_rule = area.get("collect")
        if area.get("managed") or not collect_rule:
            continue
        if "directory" in area:
            base = os.path.join(workspace, *split(ledger["roots"][area["root"]]["directory"]), *split(area["directory"]))
            names = children(base) or []
        else:
            base = root_directory(workspace, ledger, area["root"])
            names = [n for n in (children(base) or []) if re.search(area["pattern"], n)]
        budget = area.get("budget", {}).get("bytes")
        targets.append({"id": area["id"], "kind": "area", "base": base, "budget": budget, "age": collect_rule["unusedAgeMs"],
                        "quiet": collect_rule["quietMs"], "units": observe(base, names, collect_rule["custody"])})
    for retirement in ledger["retirements"]:
        base = os.path.join(root_directory(workspace, ledger, retirement["root"]), *split(retirement["directory"]))
        names = children(base)
        if names is None:
            continue
        targets.append({"id": retirement["root"] + ":" + retirement["directory"], "kind": "retired", "base": base, "budget": None, "age": 0,
                        "quiet": retirement["quietMs"], "units": observe(base, names, "quiet")})
    for root in ("repository", "native"):
        names = undeclared(workspace, ledger, root)
        if names:
            spec = ledger["roots"][root]
            targets.append({"id": root + ":undeclared", "kind": "undeclared", "base": root_directory(workspace, ledger, root), "budget": None,
                            "age": spec["undeclaredUnusedAgeMs"], "quiet": spec["undeclaredQuietMs"], "units": observe(root_directory(workspace, ledger, root), names, "quiet")})
    removed, kept = {}, {}
    for target in targets:
        reason_base = {"retired": "retired", "undeclared": "undeclared"}.get(target["kind"], "age")
        blocked = lambda unit: "held" if unit["held"] else ("quiet" if now - unit["recency"] < target["quiet"] else None)
        gone, here = set(), {}
        units = sorted(target["units"], key=lambda u: u["name"])
        for unit in units:
            if target["age"] is not None and now - unit["recency"] >= target["age"]:
                reason = blocked(unit)
                if reason:
                    here[unit["name"]] = reason
                else:
                    removed[target["id"] + "/" + unit["name"]] = reason_base
                    gone.add(unit["name"])
        remaining = sum(u["bytes"] for u in units if u["name"] not in gone)
        if target["budget"] is not None and remaining > target["budget"]:
            for unit in sorted([u for u in units if u["name"] not in gone], key=lambda u: (u["recency"], u["name"])):
                if remaining <= target["budget"]:
                    break
                reason = blocked(unit)
                if reason:
                    here[unit["name"]] = reason
                    continue
                removed[target["id"] + "/" + unit["name"]] = "budget"
                gone.add(unit["name"])
                remaining -= unit["bytes"]
        for name, reason in here.items():
            if name not in gone:
                kept[target["id"] + "/" + name] = reason
        target["gone"] = gone
    prefixes = []
    for target in targets:
        for name in target["gone"]:
            prefixes.append(os.path.join(target["base"], *split(name)))
    everything = set()
    for top, directories, files in os.walk(workspace):
        for name in directories + files:
            everything.add(os.path.relpath(os.path.join(top, name), workspace).replace(os.sep, "/"))
    if not dry_run:
        for prefix in prefixes:
            rel = os.path.relpath(prefix, workspace).replace(os.sep, "/")
            everything = {p for p in everything if p != rel and not p.startswith(rel + "/")}
        for target in targets:
            if target["kind"] == "retired" and len(target["gone"]) == len(target["units"]):
                rel = os.path.relpath(target["base"], workspace).replace(os.sep, "/")
                everything.discard(rel)
    return {"removed": removed, "kept": kept, "remaining": sorted(everything)}


def digest(path):
    rows = []
    for top, directories, files in os.walk(path, followlinks=False):
        for name in directories + files:
            full = os.path.join(top, name)
            info = os.lstat(full)
            kind = "link" if os.path.islink(full) else "dir" if os.path.isdir(full) else "file"
            rows.append([os.path.relpath(full, path).replace(os.sep, "/"), kind, 0 if kind == "dir" else info.st_size, os.readlink(full) if kind == "link" else ""])
    return sorted(rows)


def alive(pid):
    try:
        os.kill(pid, 0)
        return True
    except PermissionError:
        return True
    except OSError:
        return False


def hold(file, mode):
    import sqlite3
    database = sqlite3.connect(file, isolation_level=None, timeout=0)
    database.execute("PRAGMA journal_mode=DELETE")
    database.execute("CREATE TABLE IF NOT EXISTS t(x)")
    if mode == "exclusive":
        database.execute("BEGIN EXCLUSIVE")
    else:
        database.execute("BEGIN")
        database.execute("SELECT * FROM t").fetchall()
    sys.stdout.write("READY\n")
    sys.stdout.flush()
    sys.stdin.read()
    database.execute("ROLLBACK")
    database.close()


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "lock":
        hold(sys.argv[2], sys.argv[3])
        return
    request = json.load(sys.stdin)
    op = request["op"]
    if op == "measure":
        rows = []
        for row in request["rows"]:
            info = measure(row["path"], row["limit"])
            rows.append(info)
        print(json.dumps(rows))
    elif op == "held":
        print(json.dumps([held(r["root"], set(r["alive"]), set(r["heldSqlite"])) for r in request["rows"]]))
    elif op == "undeclared":
        answer = {}
        for root in ("repository", "native"):
            try:
                answer[root] = undeclared(request["workspace"], request["ledger"], root)
            except Container as error:
                answer[root] = {"error": str(error)}
        print(json.dumps(answer))
    elif op == "digest":
        print(json.dumps([digest(p) for p in request["paths"]]))
    elif op == "alive":
        print(json.dumps([alive(p) for p in request["pids"]]))
    elif op == "collect":
        print(json.dumps(collect(request["workspace"], request["ledger"], request["now"], request["dryRun"], set(request["alive"]), set(request["heldSqlite"]))))
    else:
        raise SystemExit("unknown op " + op)


main()
