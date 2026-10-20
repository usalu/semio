#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the graph-speed set (stdlib only, cross-platform).

usage: apply.py [check|diff|apply] [--write] [--root REPO_ROOT] [--ignore-running]
  check  (default)  per file/op state: pending | applied | DRIFT; exit 0 applied, 1 pending, 2 drift
  diff              unified diff of what apply would write
  apply             DRY RUN unless --write; refuses on any drift; refuses while a graph/dependency command runs (--ignore-running)
Every anchor must occur exactly once at its point of application; a replacement that is already present marks the op applied (idempotent).
Nothing is written unless every op and every new file is pending or applied; files are written through a temporary file and an atomic rename."""
import difflib, hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
args = sys.argv[1:]
if "--root" in args:
    ROOT = os.path.abspath(args[args.index("--root") + 1])
mode = next((a for a in args if a in ("check", "diff", "apply")), "check")
SET = args[args.index("--spec") + 1] if "--spec" in args else "ops"
spec = json.load(open(os.path.join(HERE, "staged", SET + ".json"), encoding="utf8"))
STAGED = os.path.join(HERE, "staged", "new" if SET == "ops" else SET + "-new")


def read(path):
    with open(os.path.join(ROOT, path), encoding="utf8") as handle:
        return handle.read()


def running():
    out = subprocess.run(["ps", "-axo", "pid,command"], capture_output=True, text=True).stdout if os.name != "nt" else ""
    needles = ("repo show projects", "repo exec", "dependencies sync", "dependencies lock", "repo tools setup")
    return [line.strip()[:140] for line in out.splitlines() if any(n in line for n in needles) and "apply.py" not in line and "ps -axo" not in line]


def plan():
    states, texts, new = [], {}, []
    for row in spec["ops"]:
        path = row["path"]
        if path not in texts:
            texts[path] = read(path)
        text = texts[path]
        if row["replacement"] in text:
            states.append((path, "applied"))
        elif text.count(row["anchor"]) == 1:
            states.append((path, "pending"))
            texts[path] = text.replace(row["anchor"], row["replacement"])
        else:
            states.append((path, "DRIFT(anchor occurs %d times): %r" % (text.count(row["anchor"]), row["anchor"][:60])))
    for source, target in spec["new"].items():
        data = open(os.path.join(STAGED, source), "rb").read()
        destination = os.path.join(ROOT, target)
        if not os.path.exists(destination):
            new.append((target, data, "pending"))
        else:
            new.append((target, data, "applied" if open(destination, "rb").read() == data else "DRIFT(new file differs)"))
    return states, texts, new


def main():
    states, texts, new = plan()
    longest = max(len(os.path.join(ROOT, p)) for p in list(texts) + [t for t, _, _ in new])
    drift = [s for s in states if s[1].startswith("DRIFT")] + [(t, s) for t, _, s in new if s.startswith("DRIFT")]
    pending = [s for s in states if s[1] == "pending"] + [(t, s) for t, _, s in new if s == "pending"]
    if mode in ("check", "apply") or not pending:
        for path, state in states:
            print("%-8s %s" % (state.split("(")[0], path[-96:]) + ("" if not state.startswith("DRIFT") else "  " + state))
        for target, _, state in new:
            print("%-8s %s (new)" % (state.split("(")[0], target[-96:]) + ("" if not state.startswith("DRIFT") else "  " + state))
        print("ops %d (pending %d), new files %d, files %d, longest path %d UTF-16 units" % (len(states), len([s for s in states if s[1] == "pending"]), len(new), len(texts), longest))
    if longest > 256:
        print("REFUSED: a path exceeds 256 characters")
        sys.exit(2)
    if drift:
        print("DRIFT: re-run ops_gen.py after reconciling the live files")
        sys.exit(2)
    if mode == "diff":
        for path, text in texts.items():
            before = read(path)
            if before != text:
                sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), text.splitlines(True), "a/" + path, "b/" + path, n=1))
        sys.exit(0)
    if mode == "check":
        sys.exit(1 if pending else 0)
    if "--write" not in args:
        print("DRY RUN: nothing written (use --write)")
        sys.exit(1 if pending else 0)
    active = running()
    if active and "--ignore-running" not in args:
        print("REFUSED: a graph or dependency command is running:\n  " + "\n  ".join(active))
        sys.exit(2)
    temporaries = []
    try:
        for path, text in texts.items():
            if read(path) == text:
                continue
            destination = os.path.join(ROOT, path)
            temporary = destination + ".graph-speed.tmp"
            with open(temporary, "w", encoding="utf8", newline="") as handle:
                handle.write(text)
            temporaries.append((temporary, destination))
        for target, data, state in new:
            if state != "pending":
                continue
            destination = os.path.join(ROOT, target)
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            temporary = destination + ".graph-speed.tmp"
            with open(temporary, "wb") as handle:
                handle.write(data)
            temporaries.append((temporary, destination))
        for temporary, destination in temporaries:
            os.replace(temporary, destination)
        temporaries = []
    finally:
        for temporary, _ in temporaries:
            if os.path.exists(temporary):
                os.remove(temporary)
    print("APPLIED: %d files written" % len([1 for _ in spec["bases"]]))


main()
