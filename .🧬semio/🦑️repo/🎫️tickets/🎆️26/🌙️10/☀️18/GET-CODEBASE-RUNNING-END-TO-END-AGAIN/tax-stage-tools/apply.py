#!/usr/bin/env python3
# 🔧 Applies, checks, diffs or mirrors the staged tax-stage operation manifest (staged/ops.json). Anchored: every operation carries the sha256 of the file it was derived from.
import difflib, hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def load():
    return json.load(open(os.path.join(STAGED, "ops.json"), encoding="utf8"))["ops"]


def live(path):
    full = os.path.join(ROOT, path)
    return read(full) if os.path.isfile(full) else None


def classify(op):
    current = live(op["path"])
    now = None if current is None else sha(current)
    if op["kind"] == "write":
        if now == op["sha"]:
            return "applied"
        return "ready" if now == op["base"] else "conflict"
    if now is None:
        return "applied"
    return "ready" if now == op["base"] else "conflict"


def sync_running():
    listing = subprocess.run(["ps", "-ww", "-eo", "pid,command"], capture_output=True, text=True).stdout
    found = []
    for line in listing.splitlines()[1:]:
        pid, _, command = line.strip().partition(" ")
        words = command.split()
        if not words or str(os.getpid()) == pid:
            continue
        executable = os.path.basename(words[0])
        if executable == "bun" and "dependencies sync" in command:
            found.append(line)
        elif executable in ("sh", "bash", "zsh") and any(name in command for name in ("deps4-run.sh", "deps4-loop.sh")) and "-c" not in words[1:2]:
            found.append(line)
    return found


def cmd_check(args):
    ops = load()
    states = {}
    for op in ops:
        states.setdefault(classify(op), []).append(op["path"])
    for name, paths in sorted(states.items()):
        print("%-9s %d" % (name, len(paths)))
        if name == "conflict" or "-v" in args:
            for path in paths:
                print("   ", path)
    return 0 if not states.get("conflict") else 3


def cmd_diff(args):
    for op in load():
        old = live(op["path"])
        new = read(os.path.join(STAGED, "blobs", op["blob"])) if op["kind"] == "write" else b""
        a = [] if old is None else old.decode("utf8").splitlines(keepends=True)
        b = new.decode("utf8").splitlines(keepends=True)
        if args and not any(arg in op["path"] for arg in args):
            continue
        sys.stdout.writelines(difflib.unified_diff(a, b, "a/" + op["path"], "b/" + op["path"], n=1))


def cmd_apply(args):
    running = sync_running()
    if running and "--ignore-sync" not in args:
        print("refusing: a dependency sync is running (the freeze is not lifted):")
        for line in running:
            print("  " + line[:160])
        return 2
    ops = load()
    states = {op["path"]: classify(op) for op in ops}
    conflicts = [path for path, state in states.items() if state == "conflict"]
    if conflicts:
        print("refusing: targets changed since staging (rerun `bun stage.ts`, then `apply.py check`):")
        for path in conflicts:
            print("  ", path)
        return 3
    pending = [op for op in ops if states[op["path"]] == "ready"]
    if "--dry-run" in args:
        for op in pending:
            print("would", op["kind"], op["path"])
        print("%d pending, %d already applied" % (len(pending), len(ops) - len(pending)))
        return 0
    for op in pending:
        if op["kind"] != "write":
            continue
        target = os.path.join(ROOT, op["path"])
        os.makedirs(os.path.dirname(target), exist_ok=True)
        tmp = target + ".taxstage.tmp"
        with open(tmp, "wb") as handle:
            handle.write(read(os.path.join(STAGED, "blobs", op["blob"])))
        os.replace(tmp, target)
        print("wrote", op["path"])
    for op in pending:
        if op["kind"] != "delete":
            continue
        os.remove(os.path.join(ROOT, op["path"]))
        directory = os.path.dirname(os.path.join(ROOT, op["path"]))
        while directory != ROOT and os.path.isdir(directory) and not os.listdir(directory):
            os.rmdir(directory)
            directory = os.path.dirname(directory)
        print("deleted", op["path"])
    print("applied %d operations" % len(pending))
    return 0


def overlay(real, mirror, entries):
    os.makedirs(mirror, exist_ok=True)
    children = {}
    for parts, payload in entries:
        children.setdefault(parts[0], []).append((parts[1:], payload))
    if os.path.isdir(real):
        for name in os.listdir(real):
            if name not in children:
                os.symlink(os.path.join(real, name), os.path.join(mirror, name))
    for name, rest in children.items():
        if len(rest) == 1 and not rest[0][0]:
            payload = rest[0][1]
            target = os.path.join(mirror, name)
            if isinstance(payload, tuple):
                os.symlink(payload[1], target)
            elif payload is not None:
                with open(target, "wb") as handle:
                    handle.write(payload)
        else:
            overlay(os.path.join(real, name), os.path.join(mirror, name), rest)
    for name in children:
        path = os.path.join(mirror, name)
        if os.path.isdir(path) and not os.path.islink(path) and not os.listdir(path):
            os.rmdir(path)


def real_files(base):
    for directory, dirs, names in os.walk(base):
        for name in [d for d in dirs if d == "node_modules"]:
            yield os.path.relpath(os.path.join(directory, name), ROOT), ("link", os.path.realpath(os.path.join(directory, name)))
        dirs[:] = [d for d in dirs if d != "node_modules"]
        for name in names + dirs:
            full = os.path.join(directory, name)
            if os.path.islink(full):
                yield os.path.relpath(full, ROOT), ("link", os.path.realpath(full))
            elif os.path.isfile(full):
                yield os.path.relpath(full, ROOT), read(full)


def cmd_mirror(args):
    mirror = os.path.abspath(args[0])
    copies = [args[i + 1] for i, value in enumerate(args) if value == "--copy" and i + 1 < len(args)]
    payload = {}
    for relative in copies:
        for path, data in real_files(os.path.join(ROOT, relative)):
            payload[path] = data
    for path in ("package.json", "\U0001F4CB️project.json"):
        payload[path] = read(os.path.join(ROOT, path))
    for op in load():
        payload[op["path"]] = read(os.path.join(STAGED, "blobs", op["blob"])) if op["kind"] == "write" else None
    entries = [(path.split("/"), data) for path, data in payload.items()]
    if os.path.lexists(mirror):
        subprocess.run(["rm", "-rf", mirror], check=True)
    overlay(ROOT, mirror, entries)
    print("mirror", mirror, len(payload), "real files")


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    handlers = {"check": cmd_check, "diff": cmd_diff, "apply": cmd_apply, "mirror": cmd_mirror}
    if command not in handlers:
        raise SystemExit("usage: apply.py check [-v] | diff [substring...] | apply [--dry-run] [--ignore-sync] | mirror DIR [--copy RELATIVE_DIR]...")
    raise SystemExit(handlers[command](sys.argv[2:]) or 0)
