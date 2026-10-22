#!/usr/bin/env python3
"""Stages the agent-session set: compares the base checkout (an unmodified copy of the live tree) with the overlay that carries the finished work and writes
  staged/edits.json   every change as data: anchored hunks, whole-file replacements guarded by the base hash, programmatic edits (progedits.py), new files, deletions
  staged/files/       the content of new files and of whole-file replacements under short names (the real paths can exceed 256 characters inside a ticket)
usage: stage.py --base <ov-base> --new <ov-a> [--groups groups.json]
Groups decide in which apply step a change is made (core | routing | nx-watchers | service)."""
import argparse, difflib, hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SKIP_DIRS = {"node_modules", ".git", "target", "dist", "__pycache__", ".nx"}
PROGRAMMATIC = {
    "catalog": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json",
    "operations_schema": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧬️schema/🔣️.json",
    "surface": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🖼️assets/🏗️build-surface.json",
    "surface_schema": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧬️schema/🏗️build-surface.json",
    "surface_golden": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧪️tests/🧫️fixtures/🏗️build-surface.json",
    "process_project": "🧰️framework/🔨️modules/🏃️process/📋️project.json",
    "process_script": "🧰️framework/🔨️modules/🏃️process/📜️script.ts",
    "entrypoint_fixture": "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧫️fixtures/🚪️entrypoint/🔣️.json",
}
GENERATED = {"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/📦️packages/🦀️rust/Cargo.lock"}
GROUP_RULES = [
    ("nx-watchers", ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/"]),
    ("routing", ["🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧭️repo-cli/", "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/"]),
    ("service", ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🛰️service/"]),
]


def group_of(path):
    for name, prefixes in GROUP_RULES:
        if any(path.startswith(p) for p in prefixes):
            return name
    return "core"


def walk(root):
    for base, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for f in files:
            full = os.path.join(base, f)
            if os.path.islink(full):
                continue
            yield os.path.relpath(full, root)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def hunks(old_text, new_text):
    old_lines, new_lines = old_text.splitlines(keepends=True), new_text.splitlines(keepends=True)
    matcher = difflib.SequenceMatcher(None, old_lines, new_lines, autojunk=False)
    groups = []
    for tag, i1, i2, j1, j2 in matcher.get_opcodes():
        if tag != "equal":
            groups.append([i1, i2, j1, j2])
    merged = []
    for g in groups:
        if merged and g[0] - merged[-1][1] <= 2:
            merged[-1][1], merged[-1][3] = g[1], g[3]
        else:
            merged.append(g)
    result = []
    for i1, i2, j1, j2 in merged:
        before, after = 0, 0
        while True:
            lo, hi = max(0, i1 - before), min(len(old_lines), i2 + after)
            old = "".join(old_lines[lo:hi])
            if old_text.count(old) == 1 and old.strip():
                break
            if before <= after and lo > 0:
                before += 1
            elif hi < len(old_lines):
                after += 1
            elif lo > 0:
                before += 1
            else:
                return None
            if before > 40 and after > 40:
                return None
        new = "".join(old_lines[lo:i1]) + "".join(new_lines[j1:j2]) + "".join(old_lines[i2:hi])
        result.append({"old": old, "new": new})
    return result


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", required=True)
    ap.add_argument("--new", required=True)
    ap.add_argument("--merge", action="store_true", help="keep the edits of other groups in staged/edits.json and replace only those of --force-group")
    ap.add_argument("--force-group", help="assign every found change to this group")
    ap.add_argument("--since", type=float, required=True, help="epoch seconds: only files of the overlay modified after this belong to the set (peers change the checkout while the overlay lives)")
    args = ap.parse_args()
    base, new = os.path.realpath(args.base), os.path.realpath(args.new)
    staged = os.path.join(HERE, "staged")
    if not args.merge:
        shutil.rmtree(os.path.join(staged, "files"), ignore_errors=True)
    os.makedirs(os.path.join(staged, "files"), exist_ok=True)
    base_files, new_files = set(walk(base)), set(walk(new))
    touched = {p for p in new_files if os.stat(os.path.join(new, p)).st_mtime > args.since}
    deleted = set() if args.merge else set(json.load(open(os.path.join(HERE, "deleted.json"), encoding="utf8")))
    previous = json.load(open(os.path.join(staged, "edits.json"), encoding="utf8"))["edits"] if args.merge else []
    if args.merge:
        old_blobs = {e["blob"] for e in previous if "blob" in e and e["group"] != args.force_group}
        for name in os.listdir(os.path.join(staged, "files")):
            pass
    programmatic = set(PROGRAMMATIC.values())
    edits, counter = [], [0]

    existing = [int(n[1:]) for n in os.listdir(os.path.join(staged, "files")) if n.startswith("f")] if args.merge else []
    counter[0] = max(existing, default=0)

    def store(data):
        counter[0] += 1
        name = "f%04d" % counter[0]
        open(os.path.join(staged, "files", name), "wb").write(data)
        return name

    for path in sorted((new_files - base_files) & touched):
        if path in GENERATED or "/.profiled" in path:
            continue
        data = open(os.path.join(new, path), "rb").read()
        edits.append({"kind": "new", "file": path, "group": group_of(path), "blob": store(data), "sha": sha(data), "mode": os.stat(os.path.join(new, path)).st_mode & 0o777})
    for path in sorted((base_files - new_files) & deleted):
        edits.append({"kind": "delete", "file": path, "group": group_of(path), "sha": sha(open(os.path.join(base, path), "rb").read())})
    for path in sorted(base_files & new_files & touched):
        a, b = os.path.join(base, path), os.path.join(new, path)
        if os.stat(a).st_size == os.stat(b).st_size and open(a, "rb").read() == open(b, "rb").read():
            continue
        if path in GENERATED:
            continue
        old_bytes, new_bytes = open(a, "rb").read(), open(b, "rb").read()
        if path in programmatic:
            name = next(k for k, v in PROGRAMMATIC.items() if v == path)
            edits.append({"kind": "prog", "file": path, "group": group_of(path), "function": name, "base": sha(old_bytes), "sha": sha(new_bytes)})
            continue
        try:
            old_text, new_text = old_bytes.decode("utf8"), new_bytes.decode("utf8")
        except UnicodeDecodeError:
            edits.append({"kind": "replace", "file": path, "group": group_of(path), "base": sha(old_bytes), "blob": store(new_bytes), "sha": sha(new_bytes)})
            continue
        found = hunks(old_text, new_text)
        changed = sum(1 for x in difflib.ndiff(old_text.splitlines(), new_text.splitlines()) if x[:1] in "+-")
        if found is None or len(found) > 25 or changed > 0.6 * max(1, len(old_text.splitlines()) + len(new_text.splitlines())):
            edits.append({"kind": "replace", "file": path, "group": group_of(path), "base": sha(old_bytes), "blob": store(new_bytes), "sha": sha(new_bytes)})
        else:
            edits.append({"kind": "hunks", "file": path, "group": group_of(path), "base": sha(old_bytes), "sha": sha(new_bytes), "hunks": found})
    if args.force_group:
        for e in edits:
            e["group"] = args.force_group
    if args.merge:
        kept = [e for e in previous if e["group"] != args.force_group]
        for e in previous:
            if e["group"] == args.force_group and "blob" in e:
                try:
                    os.remove(os.path.join(staged, "files", e["blob"]))
                except FileNotFoundError:
                    pass
        edits = kept + edits
    json.dump({"version": 1, "edits": edits}, open(os.path.join(staged, "edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    kinds = {}
    for e in edits:
        kinds[(e["group"], e["kind"])] = kinds.get((e["group"], e["kind"]), 0) + 1
    print(json.dumps({f"{g}/{k}": n for (g, k), n in sorted(kinds.items())}, ensure_ascii=False))
    longest = max(len(e["file"]) for e in edits)
    print("edits", len(edits), "longest path (chars)", longest)


main()
