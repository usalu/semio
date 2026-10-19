#!/usr/bin/env python3
# Anchored staging of TypeScript fixes: `patches.json` is the source of truth (path -> ordered anchored ops).
#   stage.py add <relative-path>...        copy current real files into the scratch mirror (+ base snapshot) for editing
#   stage.py mkpatch                       diff edited mirror copies against their base snapshot into patches.json + staged/
#   stage.py check                         verify every anchor applies exactly once (or is already applied) on the current tree
#   stage.py apply [--dry-run] [--ignore-sync]   write the patched files (refuses while a dependency sync runs)
#   stage.py export OUT.json               write {absolute path: patched text} for tscheck.mjs
#   stage.py mirror DIR                    build a scratch overlay: real dirs along patched paths, symlinks elsewhere
import difflib, hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()
TICKET = os.path.dirname(HERE)
WORK = os.path.join(TICKET, "\U0001F5D1️generated", "tsc-stage-work")
EDIT = os.path.join(WORK, "edit")
BASE = os.path.join(WORK, "base")
PATCHES = os.path.join(HERE, "patches.json")
STAGED = os.path.join(HERE, "staged")


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load():
    return json.load(open(PATCHES, encoding="utf-8")) if os.path.exists(PATCHES) else {}


def cmd_add(paths):
    for relative in paths:
        source = os.path.join(ROOT, relative)
        for root, name in ((EDIT, "edit"), (BASE, "base")):
            target = os.path.join(root, relative)
            if os.path.exists(target) and name == "edit":
                continue
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(target, "wb") as handle:
                handle.write(read(source))
        print("added", relative)


def walk_files(top):
    for directory, _dirs, names in os.walk(top):
        for name in names:
            yield os.path.relpath(os.path.join(directory, name), top)


def ops_for(base_text, edit_text):
    base_lines = base_text.splitlines(keepends=True)
    edit_lines = edit_text.splitlines(keepends=True)
    offsets = [0]
    for line in base_lines:
        offsets.append(offsets[-1] + len(line))
    matcher = difflib.SequenceMatcher(None, base_lines, edit_lines, autojunk=False)
    ops = []
    for tag, i1, i2, j1, j2 in matcher.get_opcodes():
        if tag == "equal":
            continue
        start, stop = offsets[i1], offsets[i2]
        old, new = base_text[start:stop], "".join(edit_lines[j1:j2])
        prefix = 0
        limit = min(len(old), len(new))
        while prefix < limit and old[prefix] == new[prefix]:
            prefix += 1
        suffix = 0
        while suffix < limit - prefix and old[len(old) - 1 - suffix] == new[len(new) - 1 - suffix]:
            suffix += 1
        start += prefix
        stop -= suffix
        new = new[prefix:len(new) - suffix]
        left, right = 0, 0
        while True:
            anchor = base_text[max(0, start - left):min(len(base_text), stop + right)]
            if anchor and base_text.count(anchor) == 1:
                break
            if start - left <= 0 and stop + right >= len(base_text):
                raise SystemExit("no unique anchor for hunk at offset %d" % start)
            left += 12
            right += 12
        before = base_text[max(0, start - left):start]
        after = base_text[stop:min(len(base_text), stop + right)]
        ops.append([anchor, before + new + after])
    return ops


def cmd_mkpatch():
    patches = load()
    for relative in sorted(walk_files(EDIT)):
        base = read(os.path.join(BASE, relative)).decode("utf-8")
        edit = read(os.path.join(EDIT, relative)).decode("utf-8")
        if base == edit:
            patches.pop(relative, None)
            continue
        patches[relative] = {"baseSha256": sha(base.encode("utf-8")), "ops": ops_for(base, edit)}
        print("patch", relative, len(patches[relative]["ops"]), "ops")
    with open(PATCHES, "w", encoding="utf-8") as handle:
        json.dump(patches, handle, ensure_ascii=False, indent=1)
    cmd_refresh()


def cmd_refresh():
    if os.path.isdir(STAGED):
        for name in os.listdir(STAGED):
            os.remove(os.path.join(STAGED, name))
    os.makedirs(STAGED, exist_ok=True)
    index = {}
    for number, (relative, row) in enumerate(sorted(load().items()), 1):
        target, text = patched(relative, row)
        flat = "%02d-%s" % (number, os.path.basename(target).encode("ascii", "ignore").decode() or "file")
        index[flat] = target
        with open(os.path.join(STAGED, flat), "w", encoding="utf-8") as handle:
            handle.write(text)
    with open(os.path.join(STAGED, "index.json"), "w", encoding="utf-8") as handle:
        json.dump(index, handle, ensure_ascii=False, indent=1)


def apply_ops(text, ops, label):
    for index, (old, new) in enumerate(ops):
        if old in new and new in text:
            continue
        elif text.count(old) == 1:
            text = text.replace(old, new)
        elif new in text:
            continue
        else:
            raise SystemExit("anchor %s[%d] occurs %d times (expected 1): %s" % (label, index, text.count(old), old[:100].replace("\n", "\\n")))
    return text


SCRIPT = "\U0001F4DC\ufe0fscript.ts"
OWNER = "\U0001F7E6\ufe0f.ts"


def candidates(relative):
    yield relative
    if os.path.basename(relative) == SCRIPT:
        yield os.path.join(os.path.dirname(relative), OWNER)


def patched(relative, row):
    errors = []
    for candidate in candidates(relative):
        path = os.path.join(ROOT, candidate)
        if not os.path.exists(path):
            continue
        try:
            return candidate, apply_ops(read(path).decode("utf-8"), row["ops"], candidate)
        except SystemExit as error:
            errors.append(str(error))
    raise SystemExit("; ".join(errors) or "no target for " + relative)


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


def cmd_check():
    for relative, row in load().items():
        target, text = patched(relative, row)
        state = "unchanged" if text == read(os.path.join(ROOT, target)).decode("utf-8") else "patchable"
        print(state, "(base drifted)" if sha(read(os.path.join(ROOT, target))) != row["baseSha256"] else "", "(retargeted to " + os.path.basename(target) + ")" if target != relative else "", relative)


def cmd_apply(args):
    running = sync_running()
    if running and "--ignore-sync" not in args:
        print("refusing: a dependency sync is running (the freeze is not lifted):")
        for line in running:
            print("  " + line[:160])
        raise SystemExit(2)
    results = dict(patched(relative, row) for relative, row in load().items())
    for relative, text in results.items():
        path = os.path.join(ROOT, relative)
        changed = read(path).decode("utf-8") != text
        print(("would patch" if "--dry-run" in args else "patched") if changed else "unchanged", relative)
        if changed and "--dry-run" not in args:
            tmp = path + ".tscstage.tmp"
            with open(tmp, "w", encoding="utf-8") as handle:
                handle.write(text)
            os.replace(tmp, path)


def cmd_export(out):
    rows = {os.path.join(ROOT, target): text for target, text in (patched(relative, row) for relative, row in load().items())}
    with open(out, "w", encoding="utf-8") as handle:
        json.dump(rows, handle, ensure_ascii=False)
    print("exported", len(rows), "files to", out)


def overlay(real, mirror, entries):
    os.makedirs(mirror, exist_ok=True)
    children = {}
    for parts, payload in entries:
        children.setdefault(parts[0], []).append((parts[1:], payload))
    if os.path.isdir(real):
        for name in os.listdir(real):
            if name in children:
                continue
            link = os.path.join(mirror, name)
            if not os.path.lexists(link):
                os.symlink(os.path.join(real, name), link)
    for name, rest in children.items():
        if len(rest) == 1 and not rest[0][0]:
            with open(os.path.join(mirror, name), "wb") as handle:
                handle.write(rest[0][1])
        else:
            overlay(os.path.join(real, name), os.path.join(mirror, name), rest)


def cmd_mirror(args):
    mirror = os.path.abspath(args[0])
    entries = [(target.split(os.sep), text.encode("utf-8")) for target, text in (patched(relative, row) for relative, row in load().items())]
    for extra in args[1:]:
        entries.append((extra.split(os.sep), read(os.path.join(ROOT, extra))))
    if os.path.lexists(mirror):
        subprocess.run(["rm", "-rf", mirror], check=True)
    overlay(ROOT, mirror, entries)
    print("mirror", mirror, len(entries), "files")


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    if command == "add":
        cmd_add(sys.argv[2:])
    elif command == "mkpatch":
        cmd_mkpatch()
    elif command == "refresh":
        cmd_refresh()
    elif command == "check":
        cmd_check()
    elif command == "apply":
        cmd_apply(sys.argv[2:])
    elif command == "mirror":
        cmd_mirror(sys.argv[2:])
    elif command == "export":
        cmd_export(sys.argv[2])
    else:
        raise SystemExit(__doc__ or "usage")
