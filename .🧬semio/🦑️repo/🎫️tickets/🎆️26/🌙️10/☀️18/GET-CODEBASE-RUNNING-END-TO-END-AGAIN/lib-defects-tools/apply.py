#!/usr/bin/env python3
# usage: apply.py check | overlay DIR | apply [--dry-run] [--ignore-sync]   (check and apply --dry-run write nothing; any other flag is refused)
import hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()
PATCHES = os.path.join(HERE, "staged", "patches")
FLAGS = {"--dry-run", "--ignore-sync"}


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


def load():
    out = []
    for name in sorted(os.listdir(PATCHES)):
        if name.endswith(".json"):
            out.extend(json.load(open(os.path.join(PATCHES, name), encoding="utf-8")))
    merged = {}
    for spec in out:
        merged.setdefault(spec["file"], []).extend(spec["edits"])
    return merged


def settle(text, edit, rel):
    old, new, want = edit["old"], edit["new"], edit["count"]
    have = text.count(old)
    if old in new and text.count(new) >= want:
        return text
    if have == want:
        return text.replace(old, new)
    if have == 0 and text.count(new) >= want:
        return text
    raise SystemExit("anchor mismatch (%d != %d) in %s: %r" % (have, want, rel, old[:80]))


def patched(rel, edits):
    path = os.path.join(ROOT, rel)
    if not os.path.isfile(path):
        raise SystemExit("target missing: " + rel)
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    for edit in sorted((e for e in edits if "start" in e), key=lambda e: e["start"], reverse=True):
        start, end = edit["start"], edit["end"]
        if text[start:end] == edit["old"]:
            text = text[:start] + edit["new"] + text[end:]
        elif text[start:start + len(edit["new"])] != edit["new"]:
            raise SystemExit("span mismatch in %s at %d: %r" % (rel, start, edit["old"][:80]))
    for edit in edits:
        if "start" not in edit:
            text = settle(text, edit, rel)
    return text


def current(rel):
    with open(os.path.join(ROOT, rel), encoding="utf-8") as handle:
        return handle.read()


def main(argv):
    mode = argv[0] if argv else "check"
    rest = argv[1:]
    positional = [word for word in rest if not word.startswith("-")]
    flags = {word for word in rest if word.startswith("-")}
    unknown = flags - FLAGS
    if unknown:
        raise SystemExit("refusing unknown flag(s) %s (this set has no rebase mode)" % ", ".join(sorted(unknown)))
    dry = "--dry-run" in flags
    merged = load()
    if mode == "check":
        for rel, edits in merged.items():
            patched(rel, edits)
        print("ok", len(merged), "files")
    elif mode == "overlay":
        out = positional[0]
        os.makedirs(out, exist_ok=True)
        mapping = {}
        for rel, edits in merged.items():
            staged = os.path.join(out, hashlib.sha256(rel.encode()).hexdigest()[:16] + os.path.splitext(rel)[1])
            with open(staged, "w", encoding="utf-8") as handle:
                handle.write(patched(rel, edits))
            mapping[os.path.join(ROOT, rel)] = staged
        with open(os.path.join(out, "map.json"), "w", encoding="utf-8") as handle:
            json.dump(mapping, handle, ensure_ascii=False)
        print("overlay", len(mapping), "files ->", os.path.join(out, "map.json"))
    elif mode == "apply":
        running = sync_running()
        if running and "--ignore-sync" not in flags:
            print("refusing: a dependency sync is running (the freeze is not lifted):")
            for line in running:
                print("  " + line[:160])
            return 2
        only = set(positional)
        results = {rel: patched(rel, edits) for rel, edits in merged.items() if not only or rel in only}
        changed = [rel for rel, text in results.items() if current(rel) != text]
        for rel in changed:
            print(("would patch " if dry else "patch ") + rel)
        if dry:
            print("dry-run complete: %d of %d files would change" % (len(changed), len(results)))
            return 0
        for rel in changed:
            target = os.path.join(ROOT, rel)
            tmp = target + ".lib-defects.tmp"
            with open(tmp, "w", encoding="utf-8") as handle:
                handle.write(results[rel])
            os.replace(tmp, target)
        print("applied %d of %d files" % (len(changed), len(results)))
    else:
        raise SystemExit("usage: apply.py check | overlay DIR | apply [--dry-run] [--ignore-sync] [files]")
    return 0


sys.exit(main(sys.argv[1:]))
