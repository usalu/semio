#!/usr/bin/env python3
"""Applies the hermetic class set to the live tree: new files (refused when present with other content) and anchored edits of existing files, all-or-nothing.
usage: apply.py [--write] [--stamp] [--rebase] [--undo] [--force]
  default      dry run: re-derives every edit from the CURRENT live text (each anchor must occur exactly once) and checks the live content hashes against bases.json
  --stamp      records the current live hashes in bases.json (after re-verifying the set on an overlay)
  --rebase     tolerate drift against bases.json (anchors still must match once)
  --write      writes temp files, renames them, keeps a backup under C/🗑️generated/bzl-hermetic/backup and records ledger.json
  --undo       restores the backups when no file changed since the write
  --force      skip the "already applied" short cut (ledger present, all files present, new files identical)"""
import hashlib, json, os, shutil, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits
from targets import NEW, OPTIONAL
ROOT = edits.ROOT
BASES = os.path.join(HERE, "bases.json")
BACKUP = os.path.join(HERE, "..", "🗑️generated", "bzl-hermetic", "backup")
LEDGER = os.path.join(BACKUP, "ledger.json")


def sha(data):
    return hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf8")).hexdigest()


def live(path):
    p = os.path.join(ROOT, path)
    return open(p, "rb").read() if os.path.exists(p) else None


def plan():
    out = {}
    for staged, target in {**NEW, **{k: v for k, v in OPTIONAL.items() if os.path.exists(os.path.join(HERE, "staged", k))}}.items():
        out[target] = open(os.path.join(HERE, "staged", staged), "rb").read()
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
    if os.path.exists(LEDGER) and not ({"--force", "--stamp"} & args):
        recorded = json.load(open(LEDGER))
        absent = [path for path in recorded if live(path) is None]
        changed = [target for staged, target in NEW.items() if live(target) != open(os.path.join(HERE, "staged", staged), "rb").read()]
        if not absent and not changed:
            print("already applied: the ledger lists %d files, all present, every new file identical; nothing to do (--force re-derives the edits)" % len(recorded))
            return
    result = plan()
    bases = json.load(open(BASES)) if os.path.exists(BASES) else {}
    rows = {}
    for path, data in result.items():
        current = live(path)
        rows[path] = None if current is None else sha(current)
    if "--stamp" in args:
        json.dump(rows, open(BASES, "w"), ensure_ascii=False, indent=1)
        print("stamped", len(rows))
        return
    problems = []
    for path, data in result.items():
        current = live(path)
        base = bases.get(path, "unstamped")
        if current is None and path in edits_existing():
            problems.append("missing live file: " + path)
        elif path in edits_existing() and "--rebase" not in args and base != rows[path] and sha(data) != rows[path]:
            problems.append("drift against bases.json: " + path)
        elif path not in edits_existing() and current is not None and current != data:
            problems.append("a different file exists: " + path)
    for line in problems:
        print("REFUSED", line)
    if problems:
        sys.exit(2)
    if "--write" not in args:
        print(f"dry run ok: {len(result)} file(s), {len(edits_existing())} edited, {len(result) - len(edits_existing())} new")
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
            temporary = target + ".hermetic.tmp"
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


def edits_existing():
    return {edits.EXEC, edits.EXEC_SCHEMA, edits.ADAPTER, edits.FIXTURE, edits.RUNNER, edits.LAW_TEST, edits.CONTRACT_TEST}


main()
