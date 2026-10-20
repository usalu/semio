#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the conc-nx set (Nx pinned generation reuse, pool without hold-and-wait, shared cache custody) (stdlib only, cross-platform).

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
group = args[args.index("--group") + 1] if "--group" in args else None
mode = next((a for a in args if a in ("check", "diff", "apply")), "check")
spec = json.load(open(os.path.join(HERE, "staged", "ops.json"), encoding="utf8"))
GROUPS = {"1": ["1"], "2": ["2-put", "2-daemon"], "2-put": ["2-put"], "2-daemon": ["2-daemon"], None: ["1", "2-put", "2-daemon"]}
if group not in GROUPS:
    sys.exit("unknown --group %r (1, 2, 2-put, 2-daemon)" % group)
SELECTED = GROUPS[group]
spec["ops"] = [row for row in spec["ops"] if row.get("group", "1") in SELECTED]
spec["new"] = {source: target for source, target in spec["new"].items() if spec.get("groups", {}).get(source, "1") in SELECTED}
REQUIRES = {"2-put": ("withCompilerSharedHeadroomV1", "conc-cargo group lease (capacity module exports withCompilerSharedHeadroomV1)")}
STAGED = os.path.join(HERE, "staged", "new")


def read(path):
    with open(os.path.join(ROOT, path), encoding="utf8") as handle:
        return handle.read()


def running():
    out = subprocess.run(["ps", "-axo", "pid,command"], capture_output=True, text=True).stdout if os.name != "nt" else ""
    needles = ("repo show projects", "repo exec", "dependencies sync", "dependencies lock", "repo tools setup", "repo run", "nx.js")
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


def requirements():
    missing = []
    capacity = os.path.join(ROOT, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "⚡️caching", "💾️storage", "🔒️enforcement", "🏗️compiler", "📏️capacity", "🟦️.ts")
    if "2-put" in SELECTED and "--ignore-requires" not in args:
        needle, text = REQUIRES["2-put"]
        if not os.path.exists(capacity) or needle not in open(capacity, encoding="utf8").read():
            missing.append("2-put needs " + text)
    return missing


def main():
    blocked = requirements()
    if blocked and mode in ("apply", "diff"):
        print("BLOCKED: " + "; ".join(blocked) + " (apply C/conc-cargo-tools group lease first, or use --group 1 / 2-daemon)")
        sys.exit(2)
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
            temporary = destination + ".conc-nx.tmp"
            with open(temporary, "w", encoding="utf8", newline="") as handle:
                handle.write(text)
            temporaries.append((temporary, destination))
        for target, data, state in new:
            if state != "pending":
                continue
            destination = os.path.join(ROOT, target)
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            temporary = destination + ".conc-nx.tmp"
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
    print("APPLIED: %d files edited, %d files created" % (len(spec["bases"]), len([1 for _, _, state in new if state == "pending"])))


main()
