#!/usr/bin/env python3
"""Applies the staged pg-chain change set (edits.py) to a repository root or a scratch overlay.

usage: apply.py bases                       record the sha256 of every edited target of the live checkout into bases.json
       apply.py apply [--root R] [--write] [--rebase] [--overwrite]   (dry run unless --write)
       apply.py status [--root R]           report per target: pending | applied | drift | anchor-missing

Edited targets refuse to change when their content drifted from bases.json (unless --rebase, which only requires every anchor to occur exactly once);
a target whose edits are all present is reported applied and left alone; new files are written only when absent or identical (unless --overwrite)."""
import hashlib, json, os, sys

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


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def edited(text, rows):
    state = []
    for old, new in rows:
        o, n = text.count(old), (text.count(new) if new else 0)
        state.append("applied" if (n >= 1 or (not new and o == 0)) else "pending" if o == 1 else "anchor-missing" if o == 0 else "anchor-ambiguous")
    return state


def bases_path():
    return os.path.join(HERE, "bases.json")


def main(argv):
    command = argv[0] if argv else "status"
    flags = argv[1:]
    root = find_root()
    if "--root" in flags:
        root = os.path.abspath(flags[flags.index("--root") + 1])
    dry, rebase, overwrite = "--write" not in flags, "--rebase" in flags, "--overwrite" in flags
    if command == "bases":
        data = {path: sha(read(os.path.join(find_root(), path))) for path in edits.EDITS}
        json.dump(data, open(bases_path(), "w"), indent=1, ensure_ascii=False)
        print(json.dumps({os.path.basename(k): v[:12] for k, v in data.items()}, ensure_ascii=False))
        return 0
    bases = json.load(open(bases_path())) if os.path.exists(bases_path()) else {}
    problems, plan = [], []
    for path, rows in edits.EDITS.items():
        target = os.path.join(root, path)
        if not os.path.isfile(target):
            problems.append((path, "missing"))
            continue
        data = read(target)
        text = data.decode("utf8")
        state = edited(text, rows)
        label = os.path.basename(path)
        if all(item == "applied" for item in state):
            print(f"applied  {label}")
            continue
        if any(item not in ("pending", "applied") for item in state):
            problems.append((path, ",".join(sorted(set(state)))))
            print(f"refuse   {label}: {state}")
            continue
        if not rebase and bases.get(path) and sha(data) != bases[path] and not any(item == "applied" for item in state):
            problems.append((path, "drift"))
            print(f"drift    {label}: live sha {sha(data)[:12]} != staged base {bases[path][:12]} (use --rebase after reviewing)")
            continue
        for (old, new), item in zip(rows, state):
            if item == "pending":
                text = text.replace(old, new, 1)
        plan.append((target, text.encode("utf8")))
        print(f"{'would edit' if dry or command == 'status' else 'edit'}   {label}: {sum(1 for item in state if item == 'pending')} edits")
    for name, relative in edits.NEW.items():
        source = os.path.join(edits.STAGED, name)
        target = os.path.join(root, relative)
        if not os.path.isfile(source):
            problems.append((relative, "staged file missing: " + name))
            continue
        data = read(source)
        if os.path.exists(target):
            if read(target) == data:
                print(f"applied  {os.path.basename(relative)}")
                continue
            if not overwrite:
                problems.append((relative, "exists-different"))
                print(f"refuse   {os.path.basename(relative)}: exists with different content")
                continue
        plan.append((target, data))
        print(f"{'would write' if dry else 'write'}  {os.path.basename(relative)}")
    if problems:
        print(json.dumps(problems, ensure_ascii=False))
    if command == "status":
        return 1 if problems else 0
    if problems:
        return 1
    if dry:
        return 0
    for target, data in plan:
        os.makedirs(os.path.dirname(target), exist_ok=True)
        temporary = target + ".tmp"
        with open(temporary, "wb") as handle:
            handle.write(data)
        os.replace(temporary, target)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
