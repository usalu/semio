#!/usr/bin/env python3
"""Applies the short-path set to the live tree: new files (refused when present with other content) and anchored edits of existing files, all-or-nothing.
usage: apply.py [--write] [--stamp] [--rebase] [--undo]
  default   dry run: re-derives every edit from the CURRENT live text (each anchor must occur exactly once) and refuses on drift against bases.json
  --stamp   records the current live hashes in bases.json (after re-verifying the set on an overlay)
  --rebase  tolerate drift against bases.json (anchors still must match once)
  --write   writes temp files, renames them, keeps a backup under C/🗑️generated/bzl-paths/backup and records ledger.json
  --undo    restores the backups when no file changed since the write"""
import hashlib, json, os, shutil, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits
ROOT = edits.ROOT
BASES = os.path.join(HERE, "bases.json")
BACKUP = os.path.join(HERE, "..", "..", "🗑️generated", "bzl-paths", "backup")
LEDGER = os.path.join(BACKUP, "ledger.json")


def sha(data):
    return hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf8")).hexdigest()


def live(path):
    p = os.path.join(ROOT, path)
    return open(p, "rb").read() if os.path.exists(p) else None


def plan():
    out = {}
    for staged, target in edits.NEW.items():
        if os.path.exists(os.path.join(HERE, staged)):
            out[target] = open(os.path.join(HERE, staged), "rb").read()
    for target, text in edits.build().items():
        out[target] = text.encode("utf8")
    return out


def main():
    args = set(sys.argv[1:])
    if "--undo" in args:
        ledger = json.load(open(LEDGER))
        for path, row in ledger.items():
            if live(path) is not None and sha(live(path)) != row["after"]:
                sys.exit("refused: changed since the write: " + path)
        for path, row in ledger.items():
            if row["before"] is None:
                os.remove(os.path.join(ROOT, path))
            else:
                shutil.copyfile(os.path.join(BACKUP, row["backup"]), os.path.join(ROOT, path))
        print("restored", len(ledger), "files")
        return
    existing = set(edits.EXISTING)
    if all(live(path) is not None and edits_applied(path) for path in existing):
        print("already applied")
        return
    result = plan()
    bases = json.load(open(BASES)) if os.path.exists(BASES) else {}
    rows = {path: (None if live(path) is None else sha(live(path))) for path in result}
    if "--stamp" in args:
        json.dump(rows, open(BASES, "w"), ensure_ascii=False, indent=1)
        print("stamped", len(rows))
        return
    problems = []
    for path, data in result.items():
        current = live(path)
        if path in existing:
            if current is None:
                problems.append("missing live file: " + path)
            elif "--rebase" not in args and bases.get(path) != rows[path]:
                problems.append("drift against bases.json: " + path)
        elif current is not None and current != data:
            problems.append("a different file exists: " + path)
    for line in problems:
        print("REFUSED", line)
    if problems:
        sys.exit(2)
    if "--write" not in args:
        print(f"dry run ok: {len(result)} file(s), {len(existing)} edited, {len(result) - len(existing)} new")
        return
    os.makedirs(BACKUP, exist_ok=True)
    ledger = {}
    for path, data in result.items():
        current = live(path)
        if current == data:
            continue
        backup = sha(path)[:16]
        if current is not None:
            open(os.path.join(BACKUP, backup), "wb").write(current)
        ledger[path] = {"before": None if current is None else sha(current), "after": sha(data), "backup": backup}
    staging = []
    try:
        for path, data in result.items():
            if path not in ledger:
                continue
            target = os.path.join(ROOT, path)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            temporary = target + ".paths.tmp"
            open(temporary, "wb").write(data)
            staging.append((temporary, target))
        for temporary, target in staging:
            os.replace(temporary, target)
    except Exception:
        for temporary, _ in staging:
            if os.path.exists(temporary):
                os.remove(temporary)
        raise
    json.dump(ledger, open(LEDGER, "w"), ensure_ascii=False, indent=1)
    print("written", len(ledger), "file(s); undo with --undo")


def edits_applied(path):
    marker = {edits.RUNNER: "manifest.aliases", edits.EXEC: "BAZEL_PATH_PLAN_V1", edits.EXEC_SCHEMA: '"aliases"', edits.STORAGE: "bazelOutputRootV1", edits.ADAPTER: "bazelPathLawV1", edits.PATHS_TEST: "names aliased sources for host-state and hermetic actions alike", edits.LAW_TEST: "bazelSourceAliasLabelV1", edits.CLASS_LAW: "bazelSourceAliasLabelV1"}[path]
    return marker in live(path).decode("utf8")


main()
