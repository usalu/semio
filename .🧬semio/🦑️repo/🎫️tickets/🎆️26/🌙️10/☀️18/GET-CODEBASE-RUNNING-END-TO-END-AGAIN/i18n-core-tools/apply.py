#!/usr/bin/env python3
"""Applies the staged i18n-core plan (`staged/plan.json`, blobs in `staged/blobs/`) to a root, all or nothing.

usage: apply.py [--step core|<name>] [--root ROOT] [--yes] [--undo] [--json]
  default      dry run: prints what would be created, edited, already applied and refused; writes nothing
  --yes        writes every file (temporary file first, then rename), backs up the originals to
               C/🗑️generated/i18n-core-apply/<stamp>/backup and rolls everything back when one write fails
  --undo       restores the originals of the newest ledger under ROOT's tag and removes the created files
Refusals (exit 2, nothing written): a file whose current hash is neither the planned base nor the planned result (drift:
re-run `build.py --root ROOT` after the round that changed it), a created file that exists with other content, a path
longer than 256 characters, a blob that does not match its recorded hash.
ROOT defaults to the live checkout. The plan was derived from the tree named in plan.json (`built-for`)."""
import json, os, shutil, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from lib import LIVE, MAX_PATH, sha

C = os.path.dirname(HERE)
STAGED = os.path.join(HERE, "staged")
GENERATED = os.path.join(C, "🗑️generated", "i18n-core-apply")


def read_bytes(path):
    with open(path, "rb") as handle:
        return handle.read()


def main(argv):
    root = os.path.abspath(argv[argv.index("--root") + 1]) if "--root" in argv else LIVE
    yes, undo, as_json = "--yes" in argv, "--undo" in argv, "--json" in argv
    global STAGED
    if "--step" in argv and argv[argv.index("--step") + 1] != "core":
        STAGED = os.path.join(HERE, "staged-" + argv[argv.index("--step") + 1])
    tag = sha(root)[:12] + ("-" + argv[argv.index("--step") + 1] if "--step" in argv else "")
    if undo:
        return undo_last(root, tag, yes)
    plan = json.load(open(os.path.join(STAGED, "plan.json"), encoding="utf8"))
    refused, pending, done = [], [], []
    for row in plan["created"] + plan["edited"]:
        rel = row["path"]
        blob = os.path.join(STAGED, "blobs", row["after"])
        if len(rel) > MAX_PATH or len(rel.encode("utf8")) > MAX_PATH:
            refused.append((rel, "path longer than 256 characters"))
            continue
        if not os.path.isfile(blob) or sha(read_bytes(blob)) != row["after"]:
            refused.append((rel, "blob missing or changed"))
            continue
        dst = os.path.join(root, rel)
        exists = os.path.isfile(dst)
        current = sha(read_bytes(dst)) if exists else None
        if current == row["after"]:
            done.append(rel)
        elif "base" in row:
            if current is None:
                refused.append((rel, "edited file is missing"))
            elif current != row["base"]:
                refused.append((rel, "drift: current %s is neither base %s nor result" % (current[:12], row["base"][:12])))
            else:
                pending.append(row)
        elif exists:
            refused.append((rel, "exists with other content"))
        else:
            pending.append(row)
    if as_json:
        print(json.dumps({"root": root, "pending": [r["path"] for r in pending], "done": done, "refused": refused}, ensure_ascii=False))
    else:
        print("root:", root)
        print("plan built for:", plan.get("built-for"))
        print("to create: %d, to edit: %d, already applied: %d, refused: %d" % (sum(1 for r in pending if "base" not in r), sum(1 for r in pending if "base" in r), len(done), len(refused)))
        for rel, why in refused[:40]:
            print("  REFUSED", rel, "-", why)
    if refused:
        print("REFUSED: nothing written")
        return 2
    if not yes:
        print("dry run; pass --yes to write")
        return 0
    if not pending:
        print("nothing to do")
        return 0
    stamp = time.strftime("%Y%m%d-%H%M%S")
    ledger_dir = os.path.join(GENERATED, tag, stamp)
    backup = os.path.join(ledger_dir, "backup")
    ledger = {"root": root, "created": [], "edited": []}
    written = []
    try:
        for row in pending:
            dst = os.path.join(root, row["path"])
            if "base" in row:
                keep = os.path.join(backup, row["path"])
                os.makedirs(os.path.dirname(keep), exist_ok=True)
                shutil.copy2(dst, keep)
        temp = []
        for row in pending:
            dst = os.path.join(root, row["path"])
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            side = os.path.join(os.path.dirname(dst), ".i18n-core-" + sha(row["path"])[:10] + ".tmp")
            with open(side, "wb") as handle:
                handle.write(read_bytes(os.path.join(STAGED, "blobs", row["after"])))
            temp.append((side, dst, row))
        for side, dst, row in temp:
            os.replace(side, dst)
            written.append(row)
            (ledger["edited"] if "base" in row else ledger["created"]).append({"path": row["path"], "after": row["after"], **({"base": row["base"]} if "base" in row else {})})
        for row in pending:
            if sha(read_bytes(os.path.join(root, row["path"]))) != row["after"]:
                raise RuntimeError("written file differs: " + row["path"])
    except BaseException as error:
        for side, dst, row in locals().get("temp", []):
            if os.path.exists(side):
                os.remove(side)
        for row in written:
            dst = os.path.join(root, row["path"])
            if "base" in row:
                shutil.copy2(os.path.join(backup, row["path"]), dst)
            elif os.path.exists(dst):
                os.remove(dst)
        print("ROLLED BACK:", error)
        return 1
    os.makedirs(ledger_dir, exist_ok=True)
    json.dump(ledger, open(os.path.join(ledger_dir, "ledger.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("applied: %d created, %d edited; ledger %s" % (len(ledger["created"]), len(ledger["edited"]), ledger_dir))
    return 0


def undo_last(root, tag, yes):
    base = os.path.join(GENERATED, tag)
    if not os.path.isdir(base):
        print("no ledger for", root)
        return 2
    stamp = sorted(os.listdir(base))[-1]
    ledger = json.load(open(os.path.join(base, stamp, "ledger.json"), encoding="utf8"))
    refused = []
    for row in ledger["created"] + ledger["edited"]:
        dst = os.path.join(root, row["path"])
        if not os.path.isfile(dst) or sha(read_bytes(dst)) != row["after"]:
            refused.append(row["path"])
    print("undo %s: %d created, %d edited, %d changed since" % (stamp, len(ledger["created"]), len(ledger["edited"]), len(refused)))
    if refused:
        for rel in refused[:20]:
            print("  CHANGED SINCE", rel)
        return 2
    if not yes:
        print("dry run; pass --yes to restore")
        return 0
    for row in ledger["edited"]:
        shutil.copy2(os.path.join(base, stamp, "backup", row["path"]), os.path.join(root, row["path"]))
    for row in ledger["created"]:
        os.remove(os.path.join(root, row["path"]))
    os.rename(os.path.join(base, stamp), os.path.join(base, stamp + ".undone"))
    print("restored")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
