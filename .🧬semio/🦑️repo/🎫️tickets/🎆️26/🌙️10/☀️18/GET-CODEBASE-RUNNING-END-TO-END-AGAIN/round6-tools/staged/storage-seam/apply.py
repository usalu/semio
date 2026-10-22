#!/usr/bin/env python3
"""🧷 Anchored apply of the staged storage-seam set (wave 1021a).

  apply.py [--root DIR] [--group g1,g2|all] [--list]            dry run (default): status of every operation, nothing written
  apply.py [--root DIR] [--group ...] --write [--backup DIR]    applies all selected groups all-or-nothing; originals go to DIR
  apply.py [--root DIR] --undo DIR                              restores the originals saved in DIR and removes the files it created

Groups (groups/*.py) declare EDITS (path, old, new[, count]; `old` must occur exactly `count` times) and NEW (repo-relative
files under staged/files/). A group whose REQUIRES files are missing is BLOCKED (its predecessor set is not applied yet) and
is skipped by `--group all`; naming it explicitly makes the run refuse. Any drift (anchor neither pending nor applied, a new
file that exists with other bytes) refuses the whole run. The live checkout is only written with --root omitted AND --write
AND no `repo show|dependencies|exec` process running; use an overlay (--root /private/tmp/storage-seam/...) for verification."""
import hashlib, importlib.util, json, os, subprocess, sys, tempfile, time

HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = "/Users/ueli/Documents/semio"
STAGED = os.path.join(HERE, "staged", "files")
sys.path.insert(0, os.path.abspath(os.path.join(HERE, "..", "..", "..")))
from winpath import resolve


def load_groups():
    groups = {}
    folder = os.path.join(HERE, "groups")
    import re
    for name in sorted(os.listdir(folder), key=lambda n: [int(t) if t.isdigit() else t for t in re.split(r'(\d+)', n)]):
        if not name.endswith(".py") or name.startswith("_"):
            continue
        spec = importlib.util.spec_from_file_location("grp_" + name[:-3], os.path.join(folder, name))
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        groups[module.NAME] = module
    return groups


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def state_of(text, old, new, count):
    if new == "":
        return "applied" if text.count(old) == 0 else "pending" if text.count(old) == count else "drift"
    if old in new:
        if text.count(new) == count:
            return "applied"
        return "pending" if text.count(old) == count else "drift"
    if text.count(old) == count:
        return "pending"
    return "applied" if text.count(old) == 0 and text.count(new) >= count else "drift"


def escapes(root, path):
    """True when the write target resolves outside the root through a symlinked ancestor (an overlay must never write through to the checkout)."""
    real_root = os.path.realpath(root)
    probe = os.path.join(root, path)
    while not os.path.lexists(probe):
        probe = os.path.dirname(probe)
    real = os.path.realpath(probe)
    return not (real == real_root or real.startswith(real_root + os.sep))


def plan_group(root, module, memory=None, created=None):
    created = set() if created is None else created
    rows, blocked = [], [p for p in getattr(module, "REQUIRES", []) if not os.path.exists(os.path.join(root, p)) and p not in created]
    memory = {} if memory is None else memory
    for path in dict.fromkeys([item[0] for item in module.EDITS] + list(module.NEW)):
        if escapes(root, path):
            rows.append(("drift", path, "resolves outside the root through a symlink"))
    for item in module.EDITS:
        path, old, new = item[0], item[1], item[2]
        count = item[3] if len(item) > 3 else 1
        full = os.path.join(root, path)
        if not os.path.exists(full):
            rows.append(("blocked" if blocked else "drift", path, "target missing"))
            continue
        text = memory.get(path)
        if text is None:
            text = read(full).decode("utf-8")
        state = state_of(text, old, new, count)
        if state == "pending":
            rows.append(("pending", path, old[:60]))
            memory[path] = text.replace(old, new)
            continue
        rows.append((state, path, old[:60] if state == "applied" else "anchor %d/%d: %r" % (text.count(old), count, old[:80])))
        memory[path] = text
    for path, old_sha in getattr(module, "REWRITES", []):
        target, source = os.path.join(root, path), resolve(STAGED, path)
        if escapes(root, path):
            rows.append(("drift", path, "resolves outside the root through a symlink"))
        elif not os.path.exists(source) or not os.path.exists(target):
            rows.append(("blocked" if blocked else "drift", path, "rewrite source or target missing"))
        else:
            current = hashlib.sha256(read(target)).hexdigest()
            if current == hashlib.sha256(read(source)).hexdigest():
                rows.append(("applied", path, "rewrite"))
            elif current == old_sha:
                rows.append(("pending", path, "rewrite"))
            else:
                rows.append(("drift", path, "rewrite target has other bytes"))
    for path in module.NEW:
        target, source = os.path.join(root, path), resolve(STAGED, path)
        if not os.path.exists(source):
            rows.append(("drift", path, "staged source missing"))
        elif not os.path.exists(target):
            rows.append(("pending", path, "new file"))
        elif read(target) == read(source):
            rows.append(("applied", path, "new file"))
        else:
            rows.append(("drift", path, "new file exists with other bytes"))
    if blocked:
        rows = [("blocked", row[1], row[2]) for row in rows]
    else:
        created.update(path for path in module.NEW if not os.path.exists(os.path.join(root, path)))
    return rows, blocked


def running(root):
    if os.path.realpath(root) != os.path.realpath(LIVE):
        return []
    out = subprocess.run(["pgrep", "-fl", r"repo (show|dependencies|exec|run)"], capture_output=True, text=True).stdout
    return [line for line in out.splitlines() if "pgrep" not in line and "apply.py" not in line]


def main(argv):
    root = LIVE
    if "--root" in argv:
        root = os.path.abspath(argv[argv.index("--root") + 1])
    groups = load_groups()
    if "--undo" in argv:
        folder = os.path.abspath(argv[argv.index("--undo") + 1])
        manifest = json.load(open(os.path.join(folder, "manifest.json")))
        for path in manifest["created"]:
            if os.path.exists(os.path.join(root, path)):
                os.remove(os.path.join(root, path))
        for path in manifest["edited"]:
            with open(os.path.join(root, path), "wb") as handle:
                handle.write(read(os.path.join(folder, "files", path)))
        print("restored", len(manifest["edited"]), "edited and removed", len(manifest["created"]), "created")
        return 0
    wanted = "all"
    if "--group" in argv:
        wanted = argv[argv.index("--group") + 1]
    explicit = wanted != "all"
    names = list(groups) if not explicit else wanted.split(",")
    for name in names:
        if name not in groups:
            print("unknown group", name, "known:", ",".join(groups))
            return 3
    if "--list" in argv:
        for name in groups:
            print(name, "-", groups[name].TITLE)
        return 0
    write = "--write" in argv
    total = {"pending": 0, "applied": 0, "blocked": 0, "drift": 0}
    chosen, report, cumulative, created = [], [], {}, set()
    for name in names:
        rows, blocked = plan_group(root, groups[name], cumulative, created)
        counts = {key: sum(1 for row in rows if row[0] == key) for key in total}
        for key in total:
            if not blocked:
                total[key] += counts[key]
        status = "BLOCKED(requires " + ", ".join(p.split("/")[-2] for p in blocked) + ")" if blocked else "DRIFT" if counts["drift"] else "applied" if counts["pending"] == 0 else "pending"
        report.append((name, status, counts, rows))
        if blocked:
            if explicit and write:
                print("refused: group", name, "is blocked by an unapplied predecessor")
                return 4
            continue
        chosen.append(name)
    byname = {name: (status, counts, rows) for name, status, counts, rows in report}
    for name in list(byname):
        status, counts, rows = byname[name]
        for other, (ostatus, ocounts, _) in byname.items():
            if status == "DRIFT" and name in getattr(groups[other], "SUPERSEDES", []) and ocounts["pending"] == 0 and ocounts["drift"] == 0 and ocounts["blocked"] == 0:
                moved = [("applied", row[1], "superseded by " + other) if row[0] == "drift" else row for row in rows]
                newcounts = {key: sum(1 for row in moved if row[0] == key) for key in counts}
                total["drift"] -= counts["drift"]
                total["applied"] += counts["drift"]
                report = [(n, "applied (superseded by %s)" % other, newcounts, moved) if n == name else (n, st, c, r) for n, st, c, r in report]
                break
    for name, status, counts, rows in report:
        print("%-12s %-34s %s" % (name, status, json.dumps(counts)))
        if status == "DRIFT" or "-v" in argv:
            for row in rows:
                if row[0] in ("drift",) or "-v" in argv:
                    print("   ", row[0], row[1][-90:], row[2])
    if total["drift"]:
        print("refused: drift")
        return 2
    if not write:
        print("dry run: %d pending operation(s) in %d group(s); pass --write to apply" % (sum(1 for n, s, c, r in report if n in chosen for _ in range(c["pending"])), len(chosen)))
        return 0
    guards = running(root)
    if guards:
        print("refused: a repo command is running:", guards[:2])
        return 5
    backup = os.path.abspath(argv[argv.index("--backup") + 1]) if "--backup" in argv else os.path.join(HERE, "backup-" + time.strftime("%Y%m%d-%H%M%S"))
    manifest = {"created": [], "edited": []}
    texts, news = {}, []
    for name in chosen:
        module = groups[name]
        for item in module.EDITS:
            path, old, new = item[0], item[1], item[2]
            count = item[3] if len(item) > 3 else 1
            text = texts.get(path)
            if text is None:
                text = read(os.path.join(root, path)).decode("utf-8")
            texts[path] = text if state_of(text, old, new, count) == "applied" else text.replace(old, new)
        for path in module.NEW:
            if not os.path.exists(os.path.join(root, path)):
                news.append(path)
        for path, _ in getattr(module, "REWRITES", []):
            texts[path] = read(resolve(STAGED, path)).decode("utf-8")
    for path in texts:
        original = read(os.path.join(root, path))
        if original.decode("utf-8") == texts[path]:
            continue
        os.makedirs(os.path.dirname(os.path.join(backup, "files", path)), exist_ok=True)
        with open(os.path.join(backup, "files", path), "wb") as handle:
            handle.write(original)
        manifest["edited"].append(path)
    for path in dict.fromkeys(news):
        manifest["created"].append(path)
    os.makedirs(backup, exist_ok=True)
    json.dump(manifest, open(os.path.join(backup, "manifest.json"), "w"), ensure_ascii=False, indent=1)
    for path in manifest["edited"]:
        full = os.path.join(root, path)
        fd, temporary = tempfile.mkstemp(dir=os.path.dirname(full))
        with os.fdopen(fd, "wb") as handle:
            handle.write(texts[path].encode("utf-8"))
        os.chmod(temporary, os.stat(full).st_mode & 0o7777 if os.path.exists(full) else 0o644)
        os.replace(temporary, full)
    for path in manifest["created"]:
        full = os.path.join(root, path)
        os.makedirs(os.path.dirname(full), exist_ok=True)
        with open(full, "wb") as handle:
            handle.write(read(resolve(STAGED, path)))
    print("wrote", len(manifest["edited"]), "edited and", len(manifest["created"]), "new file(s); backup", backup)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
