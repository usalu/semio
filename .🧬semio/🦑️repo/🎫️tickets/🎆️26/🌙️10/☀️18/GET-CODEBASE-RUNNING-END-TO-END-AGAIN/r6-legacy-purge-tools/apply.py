#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the legacy-purge set (stdlib only, cross-platform).

usage: apply.py [check|diff|apply] [--write] [--root REPO_ROOT]
  check  (default)  per file: pending | applied | WAIT(<set>) | DRIFT; exit 0 all applied, 1 pending, 2 drift or wait
  diff              unified diff of what apply would write
  apply             DRY RUN unless --write; refuses on any drift, any WAIT or a path over 256 UTF-16 units
A file is pending while its bytes hash to the recorded before-hash, applied while they hash to the after-hash.
A file based on another staged set waits (WAIT) while it still hashes to that set's live base: apply that set first.
Deletions apply while the file hashes to the recorded before-hash. Writes go through a temporary sibling and an atomic rename."""
import difflib, hashlib, json, os, stat, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
args = sys.argv[1:]
if "--root" in args:
    ROOT = os.path.abspath(args[args.index("--root") + 1])
mode = next((a for a in args if a in ("check", "diff", "apply")), "check")
manifest = json.load(open(os.path.join(HERE, "staged", "manifest.json"), encoding="utf8"))

def digest(data):
    return hashlib.sha256(data).hexdigest()

def read(path):
    target = os.path.join(ROOT, path)
    return open(target, "rb").read() if os.path.exists(target) else None

def plan():
    rows = []
    for row in manifest["files"]:
        current = read(row["path"])
        after = open(os.path.join(HERE, "staged", "after", row["after"]), "rb").read()
        have = digest(current) if current is not None else None
        if digest(after) != row["afterSha256"]:
            state = "DRIFT(staged file changed)"
        elif have == row["afterSha256"]:
            state = "applied"
        elif have == row["beforeSha256"]:
            state = "pending"
        elif "requires" in row and have == row["liveBaseSha256"]:
            state = "WAIT(%s)" % row["requires"]
        else:
            state = "DRIFT(live file differs from the staged base)"
        rows.append((row, state, current, after))
    for row in manifest["deleted"]:
        current = read(row["path"])
        have = digest(current) if current is not None else None
        state = "applied" if current is None else "pending" if have == row["beforeSha256"] else "DRIFT(live file differs from the staged base)"
        rows.append(({"path": row["path"], "kind": "delete"}, state, current, None))
    return rows

def main():
    rows = plan()
    longest = max(len(os.path.join(ROOT, row["path"]).encode("utf-16-le")) // 2 for row, _, _, _ in rows)
    drift = [r for r in rows if r[1].startswith("DRIFT")]
    wait = [r for r in rows if r[1].startswith("WAIT")]
    pending = [r for r in rows if r[1] == "pending"]
    for row, state, _, _ in rows:
        if state != "applied" or "--all" in args:
            print("%-8s %-6s %s" % (state.split("(")[0], row["kind"], row["path"][-100:]) + ("  " + state if state.startswith(("DRIFT", "WAIT")) else ""))
    print("files %d, pending %d, applied %d, wait %d, drift %d, longest path %d UTF-16 units" % (len(rows), len(pending), len([r for r in rows if r[1] == "applied"]), len(wait), len(drift), longest))
    if longest > 256:
        print("REFUSED: a path exceeds 256 characters")
        sys.exit(2)
    if drift or wait:
        print("REFUSED: reconcile the live files and rebuild with build/build.py" if drift else "REFUSED: apply the named set first, then rebuild with build/build.py")
        sys.exit(2)
    if mode == "diff":
        for row, state, current, after in pending:
            before = (current or b"").decode("utf8").splitlines(True)
            later = (after or b"").decode("utf8").splitlines(True)
            sys.stdout.writelines(difflib.unified_diff(before, later, "a/" + row["path"], "b/" + row["path"], n=1))
        sys.exit(0)
    if mode == "check":
        sys.exit(1 if pending else 0)
    if "--write" not in args:
        print("DRY RUN: nothing written (use --write)")
        sys.exit(1 if pending else 0)
    temporaries = []
    removals = []
    try:
        for row, state, current, after in pending:
            destination = os.path.join(ROOT, row["path"])
            if after is None:
                removals.append(destination)
                continue
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            temporary = destination + ".legacy-purge.tmp"
            with open(temporary, "wb") as handle:
                handle.write(after)
            os.chmod(temporary, stat.S_IMODE(os.stat(destination).st_mode) if os.path.exists(destination) else 0o644)
            temporaries.append((temporary, destination))
        for temporary, destination in temporaries:
            os.replace(temporary, destination)
        temporaries = []
        for destination in removals:
            os.remove(destination)
    finally:
        for temporary, _ in temporaries:
            if os.path.exists(temporary):
                os.remove(temporary)
    print("APPLIED: %d files written, %d removed" % (len(pending) - len(removals), len(removals)))

main()
