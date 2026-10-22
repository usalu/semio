#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the tool-direct-code set (standard library only, cross-platform). Apply AFTER round 6 (tool-iface, runtime-seam, pm-seam, backend-contract, legacy-purge).

usage: apply.py [check|diff|apply] [--write] [--root REPO_ROOT]
  check  (default)  per file: pending | applied | WAIT | DRIFT; exit 0 all applied, 1 pending, 2 drift or a missing precondition
  diff              unified diff of what apply would write
  apply             DRY RUN unless --write; refuses on any drift, any missing precondition or a path over 256 UTF-16 units
A file is pending while its bytes hash to the recorded before-hash (or it is absent for a new file), applied while they hash to the after-hash, DRIFT otherwise.
After a DRIFT (a peer changed one of the files): python3 build/build.py --base <the round-6 overlay or the live tree>, then check again.
Writes go through a temporary sibling and an atomic rename; the mode of an existing file is kept (0644 for new files).
After apply: repo verify tool-spawns (both spawn laws), then the staged tests: bun test <law dir>/🧪️tests/🟦️.ts and python3 <law dir>/🧪️tests/🐍️oracle/🐍️.py check --root ."""
import difflib, hashlib, json, os, stat, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
args = sys.argv[1:]
if "--root" in args:
    ROOT = os.path.abspath(args[args.index("--root") + 1])
mode = next((a for a in args if a in ("check", "diff", "apply")), "check")
manifest = json.load(open(os.path.join(HERE, "staged", "manifest.json"), encoding="utf8"))
REQUIRES = [
    ("🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json", "tool-iface: the tool registry"),
    ("🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts", "tool-iface: runToolV1"),
    ("🧰️framework/🔨️modules/⚙️runtime/🚀️process/🟦️.ts", "runtime-seam: the runtime process family"),
]


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
        if digest(after) != row["afterSha256"]:
            state = "DRIFT(staged file changed)"
        elif current is not None and digest(current) == row["afterSha256"]:
            state = "applied"
        elif current is None and row["beforeSha256"] is None:
            state = "pending"
        elif current is not None and digest(current) == row["beforeSha256"]:
            state = "pending"
        else:
            state = "DRIFT(the file differs from the base this set was cut against)"
        rows.append((row, state, current, after))
    return rows


def main():
    missing = [what for path, what in REQUIRES if not os.path.exists(os.path.join(ROOT, path))]
    rows = plan()
    longest = max(len((os.path.join(ROOT, row["path"])).encode("utf-16-le")) // 2 for row, _, _, _ in rows)
    drift = [r for r in rows if r[1].startswith("DRIFT")]
    pending = [r for r in rows if r[1] == "pending"]
    if mode in ("check", "apply") or not pending:
        for row, state, _, _ in rows:
            if state != "applied" or "--all" in args:
                print("%-8s %-4s %s" % (state.split("(")[0], row["kind"], row["path"][-100:]) + ("  " + state if state.startswith("DRIFT") else ""))
        print("files %d, pending %d, applied %d, drift %d, longest path %d UTF-16 units" % (len(rows), len(pending), len([r for r in rows if r[1] == "applied"]), len(drift), longest))
    for what in missing:
        print("WAIT: precondition missing: " + what + " (apply round 6 first)")
    if longest > 256:
        print("REFUSED: a path exceeds 256 characters")
        sys.exit(2)
    if missing:
        sys.exit(2)
    if drift:
        print("DRIFT: rebuild the set with build/build.py --base <root> after reconciling the files")
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
        for row, state, current, after in pending:
            destination = os.path.join(ROOT, row["path"])
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            temporary = destination + ".tool-direct-code.tmp"
            with open(temporary, "wb") as handle:
                handle.write(after)
            os.chmod(temporary, stat.S_IMODE(os.stat(destination).st_mode) if os.path.exists(destination) else 0o644)
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
