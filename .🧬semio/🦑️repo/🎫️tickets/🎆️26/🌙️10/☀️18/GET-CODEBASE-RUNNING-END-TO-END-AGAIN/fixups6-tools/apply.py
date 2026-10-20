#!/usr/bin/env python3
# usage: apply.py check | apply [--dry-run] [--ignore-sync]   (check and apply --dry-run write nothing; any other argument is refused)
import hashlib, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()
FLAGS = {"--dry-run", "--ignore-sync"}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    try:
        with open(path, "rb") as handle:
            return handle.read()
    except FileNotFoundError:
        return None


def sync_running():
    listing = subprocess.run(["ps", "-ww", "-eo", "pid,command"], capture_output=True, text=True).stdout
    found = []
    for line in listing.splitlines()[1:]:
        pid, _, command = line.strip().partition(" ")
        words = command.split()
        if not words or str(os.getpid()) == pid:
            continue
        executable = os.path.basename(words[0])
        if executable == "bun" and ("dependencies sync" in command or "dependencies lock" in command):
            found.append(line)
        elif executable in ("sh", "bash", "zsh") and any(name in command for name in ("deps4-run.sh", "deps4-loop.sh", "deps4-loop2.sh")) and "-c" not in words[1:2]:
            found.append(line)
    return found


def main(argv):
    mode = argv[0] if argv else "check"
    unknown = [word for word in argv[1:] if word not in FLAGS]
    if mode not in ("check", "apply") or unknown:
        raise SystemExit("usage: apply.py check | apply [--dry-run] [--ignore-sync]" + (" (refused: " + " ".join(unknown) + "; this set has no rebase mode)" if unknown else ""))
    dry = "--dry-run" in argv[1:]
    with open(os.path.join(HERE, "manifest.tsv"), encoding="utf8") as handle:
        rows = [line.rstrip("\n").split("\t") for line in handle if line.strip()]
    base = [row for row in rows if row[0] == "BASE"][0]
    files = [row for row in rows if row[0] == "FILE"]
    live = read(os.path.join(ROOT, base[2]))
    live_sha = sha(live) if live is not None else None
    state = []
    for _, staged_sha, rel in files:
        current = read(os.path.join(ROOT, rel))
        state.append((rel, staged_sha, None if current is None else sha(current)))
    applied = all(current == staged_sha for _, staged_sha, current in state)
    if mode == "check":
        if applied:
            print("applied: every staged file is on disk")
            return 0
        if live_sha == base[1]:
            print("not applied: live base file matches the staged anchor (apply is safe)")
            return 1
        print("drift: live base file changed since staging (%s != %s): rerun restage.sh" % (live_sha, base[1]))
        return 2
    running = sync_running()
    if running and "--ignore-sync" not in argv[1:]:
        print("refusing: a dependency sync or lock is running (the freeze is not lifted):")
        for line in running:
            print("  " + line[:160])
        return 2
    if applied:
        print("already applied")
        return 0
    if live_sha != base[1]:
        print("ABORT: anchor mismatch for %s\n  staged against %s\n  live is        %s\nrerun restage.sh" % (base[2], base[1], live_sha))
        return 3
    for rel, staged_sha, current in state:
        if rel != base[2] and current is not None and current != staged_sha:
            print("ABORT: %s exists with different content (not part of the staged base)" % rel)
            return 3
    payload = []
    for rel, staged_sha, _ in state:
        data = read(os.path.join(HERE, "staged", rel))
        if data is None or sha(data) != staged_sha:
            print("ABORT: staged file changed after the manifest was written: " + rel)
            return 3
        payload.append((rel, data))
    for rel, data in payload:
        print(("would write " if dry else "write ") + rel + " (%d bytes)" % len(data))
    if dry:
        print("dry-run complete")
        return 0
    for rel, data in payload:
        destination = os.path.join(ROOT, rel)
        os.makedirs(os.path.dirname(destination), exist_ok=True)
        tmp = destination + ".staged-tmp"
        with open(tmp, "wb") as handle:
            handle.write(data)
        os.replace(tmp, destination)
    print("applied %d files" % len(payload))
    return 0


sys.exit(main(sys.argv[1:]))
