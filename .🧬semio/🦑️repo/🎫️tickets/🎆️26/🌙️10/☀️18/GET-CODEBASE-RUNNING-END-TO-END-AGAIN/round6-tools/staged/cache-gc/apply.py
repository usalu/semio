#!/usr/bin/env python3
"""🧰️ Anchored, all-or-nothing apply of a staged set: every anchor and every recorded base is checked before the first byte is written; `--record` stores the bases, `--write` applies, `--restore` undoes only files still exactly as written."""
import hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from paths import ROOT, J, NEW, TARGET  # noqa: E402
import patches  # noqa: E402

NEW_FILES = NEW
BASES = J(HERE, "bases.json")
BACKUP = J(HERE, "backup")
MANIFEST = J(BACKUP, "manifest.json")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def bases():
    if not os.path.exists(BASES):
        return {}
    with open(BASES, encoding="utf-8") as handle:
        return json.load(handle)


def record():
    rows = {key: sha(read(J(ROOT, TARGET[key]))) for key in patches.EDITORS}
    rows.update({"file:" + name: sha(read(J(ROOT, target))) for name, target in NEW_FILES.items() if os.path.exists(J(ROOT, target))})
    with open(BASES, "w", encoding="utf-8") as handle:
        json.dump(rows, handle, indent=1, sort_keys=True)
        handle.write("\n")
    print("recorded %d base hash(es) in %s" % (len(rows), BASES))
    return 0


def plan():
    writes, notes, known = {}, [], bases()
    for key, editor in patches.EDITORS.items():
        path = J(ROOT, TARGET[key])
        original = read(path)
        before = original.decode("utf-8")
        if key not in known:
            raise patches.Drift("%s has no recorded base; run --record first" % key)
        after = editor(before)
        if editor(after) != after:
            raise patches.Drift("%s: patch is not idempotent" % key)
        state = "base" if sha(original) in (known.get(key) if isinstance(known.get(key), list) else [known.get(key)]) else "drifted since staging, anchors verified"
        if after != before:
            writes[path] = after.encode("utf-8")
            notes.append("patch  %s (%s)" % (key, state))
        else:
            notes.append("same   %s" % key)
    for name, target in NEW_FILES.items():
        path = J(ROOT, target)
        data = patches.staged(name).encode("utf-8")
        merge = getattr(patches, "MERGES", {}).get(name)
        if merge and os.path.exists(path):
            base_path, normalize, combine = merge
            live_text = read(path).decode("utf-8")
            with open(J(HERE, base_path), encoding="utf-8") as handle:
                base_text = handle.read()
            if normalize(live_text) == normalize(base_text) or normalize(live_text) == normalize(data.decode("utf-8")):
                data = combine(data.decode("utf-8"), live_text).encode("utf-8")
            else:
                raise patches.Drift("%s changed beyond the lines a peer repointed" % name)
        if os.path.exists(path):
            if read(path) == data:
                notes.append("same   %s" % name)
                continue
            base = known.get("file:" + name)
            accepted = base if isinstance(base, list) else [base]
            if not merge and base is None:
                raise patches.Drift("%s exists but has no recorded base; run --record first" % name)
            if not merge and sha(read(path)) not in accepted:
                raise patches.Drift("%s changed since staging" % name)
            notes.append("update %s" % name)
        else:
            notes.append("new    %s" % name)
        writes[path] = data
    return writes, notes


def restore():
    if not os.path.exists(MANIFEST):
        print("nothing to restore")
        return 1
    with open(MANIFEST, encoding="utf-8") as handle:
        rows = json.load(handle)
    changed = []
    for row in rows:
        path = J(ROOT, row["path"])
        current = sha(read(path)) if os.path.exists(path) else None
        if current != row["post"]:
            changed.append(row["path"])
    if changed:
        for name in changed:
            print("refused: %s changed since it was written; nothing was restored" % name)
        return 5
    for row in rows:
        path = J(ROOT, row["path"])
        if row["backup"] is None:
            if os.path.exists(path):
                os.remove(path)
        else:
            with open(path, "wb") as handle:
                handle.write(read(J(BACKUP, row["backup"])))
    print("restored %d file(s)" % len(rows))
    return 0


def write(writes):
    os.makedirs(BACKUP, exist_ok=True)
    rows = []
    for path, data in writes.items():
        name = sha(path.encode())[:16] + ".orig" if os.path.exists(path) else None
        if name is not None:
            with open(J(BACKUP, name), "wb") as handle:
                handle.write(read(path))
        rows.append({"path": os.path.relpath(path, ROOT), "backup": name, "post": sha(data)})
    with open(MANIFEST, "w", encoding="utf-8") as handle:
        json.dump(rows, handle, indent=1)
    staged = []
    try:
        for path, data in writes.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            temporary = path + ".tmp-cache-gc"
            with open(temporary, "wb") as handle:
                handle.write(data)
            staged.append((temporary, path))
        for temporary, path in staged:
            os.replace(temporary, path)
    except BaseException:
        for temporary, _ in staged:
            if os.path.exists(temporary):
                os.remove(temporary)
        for row in rows:
            path = J(ROOT, row["path"])
            if row["backup"] is None:
                if os.path.exists(path) and sha(read(path)) == row["post"]:
                    os.remove(path)
            elif os.path.exists(path) and sha(read(path)) == row["post"]:
                with open(path, "wb") as handle:
                    handle.write(read(J(BACKUP, row["backup"])))
        raise


def running():
    out = subprocess.run(["ps", "-A", "-o", "pid=,command="], capture_output=True, text=True).stdout
    return [line.strip()[:160] for line in out.splitlines() if ("dependencies sync" in line or "show projects" in line or "dependencies lock" in line) and "apply" not in line and "ps -A" not in line]


def main(argv):
    if "--record" in argv:
        return record()
    if "--restore" in argv:
        return restore()
    apply = "--write" in argv
    if apply and "--ignore-runs" not in argv:
        busy = running()
        if busy:
            print("refused: a dependency sync or graph walk is running (any write aborts it):")
            for line in busy:
                print("  ", line)
            return 2
    try:
        writes, notes = plan()
    except patches.Drift as error:
        print("refused: drift: %s" % error)
        return 3
    for note in notes:
        print(note)
    if not apply:
        print("dry-run: %d file(s) would be written; pass --write to apply" % len(writes))
        return 0
    write(writes)
    print("written: %d file(s); next: `bun %s/regen-help.ts` and the tests listed in result-cache-gc.md; undo with `--restore`" % (len(writes), HERE))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
