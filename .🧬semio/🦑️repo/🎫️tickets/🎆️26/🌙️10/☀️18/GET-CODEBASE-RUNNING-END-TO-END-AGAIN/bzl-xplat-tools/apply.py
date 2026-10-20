#!/usr/bin/env python3
# usage: apply.py check [--root DIR] | apply [--root DIR] [--dry-run] [--ignore-sync]
#   check and apply --dry-run write nothing. --root DIR targets a COPY of the tree (overlay verification); without it the live repository is the target.
import hashlib, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


def sync_running():
    listing = subprocess.run(["ps", "-ww", "-eo", "pid,command"], capture_output=True, text=True).stdout
    found = []
    for line in listing.splitlines()[1:]:
        pid, _, command = line.strip().partition(" ")
        words = command.split()
        if not words or str(os.getpid()) == pid:
            continue
        if os.path.basename(words[0]) == "bun" and ("dependencies sync" in command or "dependencies lock" in command or "show projects" in command):
            found.append(line)
    return found


def settle(text, old, new, count, rel):
    if text.count(new) >= count:
        return text
    have = text.count(old)
    if have == count:
        return text.replace(old, new)
    raise SystemExit("anchor mismatch (%d != %d) in %s: %r" % (have, count, rel, old[:90]))


def patched(root, rel):
    path = os.path.join(root, rel)
    if not os.path.isfile(path):
        raise SystemExit("target missing: " + rel)
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    for old, new, count in edits.FILE_EDITS.get(rel, []):
        text = settle(text, old, new, count, rel)
    if rel in edits.JSON_EDITS:
        text = edits.JSON_EDITS[rel](text)
    return text


def plan(root):
    out = {}
    for rel in sorted(set(edits.FILE_EDITS) | set(edits.JSON_EDITS)):
        out[rel] = patched(root, rel)
    for rel, name in edits.NEW_FILES.items():
        with open(os.path.join(edits.STAGED, name), encoding="utf-8") as handle:
            out[rel] = handle.read()
    return out


def current(root, rel):
    path = os.path.join(root, rel)
    if not os.path.isfile(path):
        return None
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def main(argv):
    mode = argv[0] if argv else "check"
    rest = argv[1:]
    root = find_root()
    flags = set()
    index = 0
    while index < len(rest):
        word = rest[index]
        if word == "--root":
            root = os.path.abspath(rest[index + 1])
            index += 2
            continue
        flags.add(word)
        index += 1
    unknown = flags - {"--dry-run", "--ignore-sync"}
    if unknown:
        raise SystemExit("refusing unknown flag(s) %s" % ", ".join(sorted(unknown)))
    results = plan(root)
    live = root == find_root()
    if mode == "check":
        print("ok", len(results), "files; changed:", sum(1 for rel, text in results.items() if current(root, rel) != text))
        return 0
    if mode != "apply":
        raise SystemExit("usage: apply.py check | apply [--root DIR] [--dry-run] [--ignore-sync]")
    if live and "--dry-run" not in flags and "--ignore-sync" not in flags:
        running = sync_running()
        if running:
            print("refusing: a dependency sync or show projects is running (the freeze is not lifted):")
            for line in running:
                print("  " + line[:160])
            return 2
    changed = [rel for rel, text in results.items() if current(root, rel) != text]
    for rel in changed:
        print(("would write " if "--dry-run" in flags else "write ") + rel + "  " + hashlib.sha256(results[rel].encode()).hexdigest()[:12])
    if "--dry-run" in flags:
        print("dry-run complete: %d of %d files would change" % (len(changed), len(results)))
        return 0
    for rel in changed:
        target = os.path.join(root, rel)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        tmp = target + ".bzl-xplat.tmp"
        with open(tmp, "w", encoding="utf-8") as handle:
            handle.write(results[rel])
        os.replace(tmp, target)
    print("applied %d of %d files" % (len(changed), len(results)))
    return 0


sys.exit(main(sys.argv[1:]))
