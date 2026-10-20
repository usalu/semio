#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the iface-wrap set (stdlib only, cross-platform).

usage: apply.py [check|diff|apply] [--write] [--through GROUP] [--root REPO_ROOT]
  check  (default)  per file: pending | applied | DRIFT; exit 0 all applied, 1 pending, 2 drift
  diff              unified diff of what apply would write
  apply             DRY RUN unless --write; refuses on any drift or a path over 256 UTF-16 units
Groups are ordered (tokio, json, wasm, react); --through GROUP applies every group up to and including GROUP (default: all).
A file has one step per group that touches it. It is pending while its bytes hash to the before-hash of its first
step or to the after-hash of any step before its target step, applied at the target step's after-hash, DRIFT otherwise.
Files are written through a temporary sibling and an atomic rename."""
import difflib, hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
args = sys.argv[1:]
if "--root" in args:
    ROOT = os.path.abspath(args[args.index("--root") + 1])
mode = next((a for a in args if a in ("check", "diff", "apply")), "check")
manifest = json.load(open(os.path.join(HERE, "staged", "manifest.json"), encoding="utf8"))
groups = manifest["groups"]
through = args[args.index("--through") + 1] if "--through" in args else groups[-1]
allowed = groups[: groups.index(through) + 1]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    target = os.path.join(ROOT, path)
    if not os.path.exists(target):
        return None
    with open(target, "rb") as handle:
        return handle.read()


def plan():
    rows = []
    for row in manifest["files"]:
        steps = [s for s in row["steps"] if s["group"] in allowed]
        if not steps:
            continue
        current = read(row["path"])
        after = open(os.path.join(HERE, "staged", steps[-1]["after"]), "rb").read()
        earlier_after = {s["afterSha256"] for s in steps[:-1]} | set(row["steps"][0].get("alsoPendingFrom", []))
        if digest(after) != steps[-1]["afterSha256"]:
            state = "DRIFT(staged file changed)"
        elif current is not None and digest(current) == steps[-1]["afterSha256"]:
            state = "applied"
        elif current is None and steps[0]["beforeSha256"] is None:
            state = "pending"
        elif current is not None and (digest(current) == steps[0]["beforeSha256"] or digest(current) in earlier_after):
            state = "pending"
        else:
            state = "DRIFT(live file differs from the staged base)"
        rows.append((row, state, current, after))
    return rows


def main():
    rows = plan()
    longest = max(len(os.path.join(ROOT, row["path"]).encode("utf-16-le")) // 2 for row, _, _, _ in rows)
    drift = [r for r in rows if r[1].startswith("DRIFT")]
    pending = [r for r in rows if r[1] == "pending"]
    if mode in ("check", "apply") or not pending:
        for row, state, _, _ in rows:
            if state != "applied" or "--all" in args:
                print("%-8s %s" % (state.split("(")[0], row["path"][-100:]) + ("  " + state if state.startswith("DRIFT") else ""))
        print("groups %s: files %d, pending %d, applied %d, drift %d, longest path %d UTF-16 units" % ("+".join(allowed), len(rows), len(pending), len([r for r in rows if r[1] == "applied"]), len(drift), longest))
    if longest > 256:
        print("REFUSED: a path exceeds 256 characters")
        sys.exit(2)
    if drift:
        print("DRIFT: rebuild the set with build.mjs after reconciling the live files")
        sys.exit(2)
    if mode == "diff":
        for row, state, current, after in pending:
            before = (current or b"").decode("utf8").splitlines(True)
            sys.stdout.writelines(difflib.unified_diff(before, after.decode("utf8").splitlines(True), "a/" + row["path"], "b/" + row["path"], n=1))
        sys.exit(0)
    if mode == "check":
        sys.exit(1 if pending else 0)
    if "--write" not in args:
        print("DRY RUN: nothing written (use --write)")
        sys.exit(1 if pending else 0)
    temporaries = []
    try:
        for row, state, _, after in pending:
            destination = os.path.join(ROOT, row["path"])
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            temporary = destination + ".iface-wrap.tmp"
            with open(temporary, "wb") as handle:
                handle.write(after)
            temporaries.append((temporary, destination))
        for temporary, destination in temporaries:
            os.replace(temporary, destination)
        temporaries = []
    finally:
        for temporary, _ in temporaries:
            if os.path.exists(temporary):
                os.remove(temporary)
    print("APPLIED: %d files written" % len(pending))


main()
