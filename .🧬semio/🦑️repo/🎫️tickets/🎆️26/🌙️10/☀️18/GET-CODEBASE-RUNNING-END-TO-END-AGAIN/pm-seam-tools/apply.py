#!/usr/bin/env python3
"""🧷️ Anchored, all-or-nothing apply of the pm-seam set (see C/result-pm-seam.md). Standard library only (macOS, Linux, Windows, devcontainer).

  apply.py check [--group G]...              read only; rc 0 applied, rc 1 pending, rc 2 refused (drift)
  apply.py apply [--group G]... [--write]    dry run unless --write; refuses on drift and while a dependency ensure|sync|lock or show projects runs
  apply.py undo [--group G]... [--write]     reverses the edits and removes the new files while they still hold our result
  apply.py mirror DIR [--lock-sync] [--apply] scratch tree: real copies of the library and framework modules, symlinks elsewhere, optionally with the round-5 closure batch and this set applied
Groups: seam (new library), bootstrap, graph, storage, register (suite target and table row), ratchet (the allowlist cut from the mirror). --root R overrides the repository root.
Phase 1 computes every new text in memory; any refusal prints REFUSED and writes nothing. Phase 2 writes through temp files and os.replace with rollback."""
import argparse, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.abspath(os.path.join(HERE, '..')))
from winpath import resolve
sys.path.insert(0, HERE)
import paths
KNOWN = ["seam", "bootstrap", "graph", "storage", "register", "ratchet"]
FULL = [paths.LIBRARY, "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🔨️modules/🧬️schema", "🧰️framework/🔨️modules/📚️compiler"]
u16 = lambda s: len(s.encode("utf-16-le")) // 2


def parse():
    ap = argparse.ArgumentParser(add_help=False)
    ap.add_argument("command", nargs="?")
    ap.add_argument("target", nargs="?")
    ap.add_argument("--root", default=paths.ROOT)
    ap.add_argument("--group", action="append", default=[])
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--lock-sync", action="store_true")
    ap.add_argument("--ignore-running", action="store_true")
    return ap.parse_args()


def busy():
    out = subprocess.run(["ps", "-axo", "args="], capture_output=True, text=True).stdout.splitlines()
    marks = ("dependencies sync", "dependencies lock", "dependencies ensure", "show projects")
    return [l for l in out if any(m in l for m in marks) and "apply.py" not in l and "grep" not in l]


def compute(root, groups, undo):
    plan = json.load(open(os.path.join(HERE, "staged", "plan.json"), encoding="utf-8"))
    drift, results, outputs, created = [], [], {}, {}
    for item in plan["files"]:
        edits = [e for e in item["edits"] if e["group"] in groups]
        if not edits:
            continue
        path = os.path.join(root, item["path"])
        if not os.path.isfile(path):
            drift.append(item["path"] + " (file missing)")
            continue
        text, changed = open(path, encoding="utf-8", newline="").read(), False
        for e in edits:
            old, new, counts = e["old"], e["new"], e["counts"]
            n_old, n_new = text.count(old), text.count(new)
            inserts = old in new
            if undo:
                undoable = (n_new in counts and n_new > 0) if inserts else (n_old == 0 and n_new > 0)
                untouched = (n_new == 0 and n_old > 0) if inserts else (n_old in counts and n_old > 0)
                if undoable:
                    text, changed = text.replace(new, old), True
                    results.append(("undo", e["id"], item["path"]))
                elif untouched:
                    results.append(("not-applied", e["id"], item["path"]))
                else:
                    drift.append("%s [%s] cannot undo: old=%d new=%d" % (item["path"], e["id"], n_old, n_new))
                continue
            applied = (n_new in counts and n_new > 0 and n_new >= n_old) if inserts else (n_old == 0 and n_new > 0)
            pending = (n_new == 0 and n_old in counts) if inserts else (n_old in counts and n_old > 0)
            if pending:
                text, changed = text.replace(old, new), True
                results.append(("edit", e["id"], item["path"]))
            elif applied:
                results.append(("already", e["id"], item["path"]))
            else:
                drift.append("%s [%s] anchor counts old=%d new=%d, expected %s: %r" % (item["path"], e["id"], n_old, n_new, counts, old[:70]))
        if changed:
            outputs[item["path"]] = text
    for n in plan["newFiles"]:
        if n["group"] not in groups:
            continue
        path = os.path.join(root, n["path"])
        content = open(resolve(os.path.join(HERE, "staged", "new"), n["path"]), encoding="utf-8", newline="").read()
        present = os.path.isfile(path) and open(path, encoding="utf-8", newline="").read()
        if u16(os.path.abspath(path)) > 256:
            drift.append(n["path"] + " exceeds 256 UTF-16 units")
        elif undo:
            if present == content:
                created[n["path"]] = None
                results.append(("remove", n["group"], n["path"]))
            elif present is False:
                results.append(("not-applied", n["group"], n["path"]))
            else:
                drift.append(n["path"] + " holds foreign content; cannot undo")
        elif present is False:
            created[n["path"]] = content
            results.append(("new", n["group"], n["path"]))
        elif present == content:
            results.append(("already", n["group"], n["path"]))
        else:
            drift.append(n["path"] + " exists with foreign content")
    return drift, results, outputs, created


def write_all(root, outputs, created):
    originals, written = {}, []
    try:
        for relative, text in outputs.items():
            path = os.path.join(root, relative)
            originals[path] = open(path, encoding="utf-8", newline="").read()
            with open(path + ".pm-seam.tmp", "w", encoding="utf-8", newline="") as handle:
                handle.write(text)
            os.replace(path + ".pm-seam.tmp", path)
            written.append(path)
        for relative, content in created.items():
            path = os.path.join(root, relative)
            originals[path] = open(path, encoding="utf-8", newline="").read() if os.path.exists(path) else None
            if content is None:
                os.remove(path)
            else:
                os.makedirs(os.path.dirname(path), exist_ok=True)
                with open(path + ".pm-seam.tmp", "w", encoding="utf-8", newline="") as handle:
                    handle.write(content)
                os.replace(path + ".pm-seam.tmp", path)
            written.append(path)
    except Exception as error:
        for path in written:
            if originals.get(path) is None:
                os.path.exists(path) and os.remove(path)
            else:
                with open(path, "w", encoding="utf-8", newline="") as handle:
                    handle.write(originals[path])
        print("ROLLED BACK after %s" % error)
        return False
    return True


def run(root, groups, write, undo, guard):
    drift, results, outputs, created = compute(root, groups, undo)
    if drift:
        print("REFUSED, nothing written:\n  " + "\n  ".join(drift))
        return 2
    for kind, a, b in results:
        print("%-12s %-34s %s" % (kind, a, b.split("/")[-2] + "/" + b.split("/")[-1]))
    pending = [r for r in results if r[0] in ("edit", "new", "undo", "remove")]
    print("%d operations, %d pending%s" % (len(results), len(pending), "" if write else " (dry run)"))
    if write and pending:
        running = busy() if guard else []
        if running:
            print("REFUSED, a dependency ensure|sync|lock or show projects runs: " + running[0][:160])
            return 2
        if not write_all(root, outputs, created):
            return 2
    return 1 if pending else 0


def mirror(destination, root, lock_sync, with_set):
    plan = json.load(open(os.path.join(HERE, "staged", "plan.json"), encoding="utf-8"))
    targets = {f["path"] for f in plan["files"]} | {n["path"] for n in plan["newFiles"]}
    chain = set()
    for relative in targets:
        parts = relative.split("/")
        for depth in range(1, len(parts)):
            chain.add("/".join(parts[:depth]))
    shutil.rmtree(destination, ignore_errors=True)
    os.makedirs(destination)

    def build(relative_dir):
        source = os.path.join(root, relative_dir) if relative_dir else root
        target = os.path.join(destination, relative_dir) if relative_dir else destination
        os.makedirs(target, exist_ok=True)
        for name in sorted(os.listdir(source)):
            child = (relative_dir + "/" + name) if relative_dir else name
            if child in chain and os.path.isdir(os.path.join(root, child)):
                build(child)
            elif child in targets and os.path.isfile(os.path.join(root, child)):
                shutil.copyfile(os.path.join(root, child), os.path.join(destination, child))
            elif not os.path.exists(os.path.join(destination, child)):
                os.symlink(os.path.join(root, child), os.path.join(destination, child))

    build("")
    skip = shutil.ignore_patterns("node_modules", "dist", "target", ".cache", "*.dmg")
    for relative in FULL:
        place = os.path.join(destination, relative)
        probe = destination
        for part in os.path.relpath(os.path.dirname(place), destination).split(os.sep):
            probe = os.path.join(probe, part)
            if os.path.islink(probe):
                os.unlink(probe)
                os.makedirs(probe)
            elif not os.path.exists(probe):
                os.makedirs(probe)
        if os.path.islink(place):
            os.unlink(place)
        elif os.path.isdir(place):
            shutil.rmtree(place)
        shutil.copytree(os.path.join(root, relative), place, symlinks=True, ignore=skip)
    for name in os.listdir(root):
        if name == "node_modules" and not os.path.lexists(os.path.join(destination, name)):
            os.symlink(os.path.join(root, name), os.path.join(destination, name))
    code = 0
    if lock_sync:
        out = subprocess.run([sys.executable, os.path.join(paths.C, "lock-sync-tools", "apply.py"), "--root", destination, "--group", "closure", "--write", "--ignore-running"], capture_output=True, text=True)
        print("lock-sync closure group in the mirror:", out.stdout.strip().splitlines()[-1] if out.stdout.strip() else out.stderr.strip()[-200:])
        if out.returncode not in (0, 1):
            return 2
    if with_set:
        code = run(destination, KNOWN, True, False, False)
    return code


def main():
    a = parse()
    groups = a.group or KNOWN
    if a.command not in ("check", "apply", "undo", "mirror") or any(g not in KNOWN for g in groups):
        raise SystemExit(__doc__)
    if a.command == "mirror":
        sys.exit(mirror(os.path.abspath(a.target), a.root, a.lock_sync, a.apply))
    sys.exit(run(a.root, groups, a.write and a.command != "check", a.command == "undo", not a.ignore_running))


main()
