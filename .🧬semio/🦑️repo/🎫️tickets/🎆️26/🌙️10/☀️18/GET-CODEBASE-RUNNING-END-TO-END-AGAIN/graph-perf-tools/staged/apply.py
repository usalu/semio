#!/usr/bin/env python3
"""Staged graph-perf edits of the existing library files (superset of the show-perf set; each anchor must occur exactly once at its point of application; idempotent; refuses on drift).

usage: apply.py check | diff | dump-ops | apply [--dry-run] [--ignore-running]"""
import json, os, subprocess, sys
import gpops

ROOT, OPS = gpops.ROOT, gpops.OPS
NEW = getattr(gpops, "NEW", {})
REPLACE = getattr(gpops, "REPLACE", {})
import hashlib
STAGED_NEW = os.path.join(os.path.dirname(os.path.abspath(__file__)), "new")


def replace_state(target, source, base):
    destination = os.path.join(ROOT, target)
    data = open(os.path.join(STAGED_NEW, source), "rb").read()
    expected = open(os.path.join(STAGED_NEW, base)).read().strip()
    current = open(destination, "rb").read()
    if current == data:
        return "applied", data, destination
    return ("pending" if hashlib.sha256(current).hexdigest() == expected else "DRIFT(file changed since staging)"), data, destination


def new_state(source, target):
    destination = os.path.join(ROOT, target)
    data = open(os.path.join(STAGED_NEW, source), "rb").read()
    if not os.path.exists(destination):
        return "pending", data, destination
    return ("applied" if open(destination, "rb").read() == data else "DRIFT(new file differs)"), data, destination


def read(path):
    with open(os.path.join(ROOT, path), encoding="utf8") as handle:
        return handle.read()


def walk(path, ops, text):
    """Yields (index, state) and the resulting text after every pending op of one file."""
    states = []
    for index, (anchor, replacement) in enumerate(ops):
        if replacement in text:
            states.append((index, "applied"))
            continue
        count = text.count(anchor)
        if count != 1:
            states.append((index, "DRIFT(anchor occurs %d times)" % count))
            continue
        states.append((index, "pending"))
        text = text.replace(anchor, replacement)
    return states, text


def running():
    out = subprocess.run(["ps", "-axo", "pid,command"], capture_output=True, text=True).stdout
    return [line.strip()[:140] for line in out.splitlines() if ("repo show projects" in line or "dependencies sync" in line or "dependencies lock" in line or "repo tools setup" in line) and "apply.py" not in line and "ps -axo" not in line]


def main():
    command = sys.argv[1] if len(sys.argv) > 1 else "check"
    if command == "dump-ops-base":
        print(json.dumps({path: [[a, r] for a, r in ops] for path, ops in gpops.load_show_perf().items()}, ensure_ascii=False))
        return
    if command == "dump-ops":
        print(json.dumps({path: [[a, r] for a, r in ops] for path, ops in OPS.items()}, ensure_ascii=False))
        return
    drift, results = False, {}
    for path, ops in OPS.items():
        original = read(path)
        states, final = walk(path, ops, original)
        results[path] = (original, final)
        for index, state in states:
            drift |= state.startswith("DRIFT")
            if command == "check":
                print("%-8s %s#%d" % (state.split("(")[0] if not state.startswith("DRIFT") else state, os.path.basename(os.path.dirname(path)), index))
    for source, target in NEW.items():
        state, _, _ = new_state(source, target)
        drift |= state.startswith("DRIFT")
        if command == "check":
            print("%-8s new %s" % (state.split("(")[0] if not state.startswith("DRIFT") else state, os.path.basename(os.path.dirname(target)) + "/" + os.path.basename(target)))
    for target, (source, base) in REPLACE.items():
        state, _, _ = replace_state(target, source, base)
        drift |= state.startswith("DRIFT")
        if command == "check":
            print("%-8s replace %s" % (state.split("(")[0] if not state.startswith("DRIFT") else state, os.path.basename(os.path.dirname(target)) + "/" + os.path.basename(target)))
    if command == "check":
        sys.exit(1 if drift else 0)
    if command == "diff":
        import difflib
        for target, (source, base) in REPLACE.items():
            state, data, destination = replace_state(target, source, base)
            sys.stdout.writelines(difflib.unified_diff(open(destination, encoding="utf8").read().splitlines(True), data.decode("utf8").splitlines(True), "a/" + target, "b/" + target, n=1))
        for path, (original, final) in results.items():
            sys.stdout.writelines(difflib.unified_diff(original.splitlines(True), final.splitlines(True), "a/" + path, "b/" + path, n=1))
        return
    if command != "apply":
        raise SystemExit("unknown command " + command)
    if drift:
        raise SystemExit("drift: re-stage against the live files; nothing written")
    live = running()
    if live and "--ignore-running" not in sys.argv:
        raise SystemExit("refused: a graph/dependency command is running:\n" + "\n".join(live))
    dry = "--dry-run" in sys.argv
    for source, target in NEW.items():
        state, data, destination = new_state(source, target)
        if state == "applied":
            print("unchanged " + target)
            continue
        print(("would create " if dry else "creating ") + target)
        if not dry:
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            with open(destination, "wb") as handle:
                handle.write(data)
    for target, (source, base) in REPLACE.items():
        state, data, destination = replace_state(target, source, base)
        if state == "applied":
            print("unchanged " + target)
            continue
        print(("would replace " if dry else "replacing ") + target)
        if not dry:
            temporary = destination + ".graphperf.tmp"
            with open(temporary, "wb") as handle:
                handle.write(data)
            os.replace(temporary, destination)
    for path, (original, final) in results.items():
        if final == original:
            print("unchanged " + path)
            continue
        print(("would write " if dry else "writing ") + path)
        if not dry:
            temporary = os.path.join(ROOT, path) + ".graphperf.tmp"
            with open(temporary, "w", encoding="utf8") as handle:
                handle.write(final)
            os.replace(temporary, os.path.join(ROOT, path))


main()
