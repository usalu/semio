#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the agent-session set (dry run by default).

  python3 apply.py --root <checkout|overlay> [--group core|routing|nx-watchers|service|all] [--write] [--backup DIR]
  python3 apply.py --root <root> --undo DIR

Groups (apply in this order; each needs the ones before it only where said):
  core         the module, `repo session`, the adapters (cargo lane, Bazel preference, test sandbox, leases, capacity, TS compiler, run leases), the MCP surface
  routing      hand-written build-verb lists of the Go and Rust launchers; skipped by itself when the launcher derives its routes from the catalogue (repo-launcher set)
  nx-watchers  Nx watcher namespaces per agent session (optional: detached watcher supervisors must carry the session address first)
  service      the resident repo service (hot-cache set): sessions per issued token, session address in the executor environment; needs the hot-cache set applied
Edits are data (staged/edits.json): anchored hunks that must match exactly once, whole-file replacements guarded by the hash of the file they replace, programmatic edits (progedits.py) that find JSON structure at apply time, new files (staged/files) and deletions. A moved anchor is refused, never guessed; an edit that is already present is reported as applied.
After `--write`: regenerate the help snapshots (`bun <mem-gov-tools>/regen-help.ts <root>`, the `regen-help` step of round 6 does it) and rebuild the repo executable (`repo run repo-mcp:build && repo run repo-mcp:install`) so the MCP tools and `repo session` exist.
"""
import argparse, hashlib, json, os, shutil, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import progedits

ORDER = ["core", "routing", "nx-watchers", "service"]
OPTIONAL = {"routing"}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def running(root):
    out = subprocess.run(["ps", "-axo", "pid=,command="], capture_output=True, text=True).stdout
    return [l.strip()[:160] for l in out.splitlines() if ("dependencies sync" in l or "show projects" in l or "dependencies lock" in l) and "apply.py" not in l and "grep" not in l]


def plan(root, groups, edits):
    state, rows, problems = {}, [], []

    def read(path):
        if path in state:
            return state[path]
        return open(path, "rb").read() if os.path.exists(path) else None

    for edit in edits:
        if edit["group"] not in groups:
            continue
        path = os.path.join(root, edit["file"])
        current = read(path)
        kind, label = edit["kind"], f'{edit["group"]}:{edit["file"]}'
        optional = edit["group"] in OPTIONAL
        if kind == "new":
            blob = open(os.path.join(HERE, "staged", "files", edit["blob"]), "rb").read()
            if current is None:
                rows.append(("pending", "new", label)); state[path] = blob
            elif current == blob:
                rows.append(("applied", "new", label))
            else:
                problems.append(f"conflict: {label} exists with other content")
        elif kind == "delete":
            if current is None:
                rows.append(("applied", "delete", label))
            elif sha(current) == edit["sha"]:
                rows.append(("pending", "delete", label)); state[path] = None
            else:
                problems.append(f"drift: {label} changed since staging; not deleted")
        elif kind == "replace":
            blob = open(os.path.join(HERE, "staged", "files", edit["blob"]), "rb").read()
            if current is None:
                problems.append(f"missing: {label}")
            elif current == blob:
                rows.append(("applied", "replace", label))
            elif sha(current) == edit["base"]:
                rows.append(("pending", "replace", label)); state[path] = blob
            else:
                problems.append(f"drift: {label} is neither the staged base nor the staged result")
        elif kind == "prog":
            if current is None:
                problems.append(f"missing: {label}")
                continue
            text = current.decode("utf8")
            try:
                result = getattr(progedits, edit["function"])(text)
            except Exception as error:
                problems.append(f"prog: {label} {edit['function']}: {error}")
                continue
            if result == text:
                rows.append(("applied", "prog", label))
            else:
                rows.append(("pending", "prog", label)); state[path] = result.encode("utf8")
        elif kind == "hunks":
            if current is None:
                if optional:
                    rows.append(("skipped", "hunks", label + " (file absent)")); continue
                problems.append(f"missing: {label}"); continue
            text = current.decode("utf8")
            todo, done, bad = 0, 0, []
            for index, hunk in enumerate(edit["hunks"]):
                old, new = hunk["old"], hunk["new"]
                present, expected = text.count(new), text.count(old)
                if present >= 1 and old != new and old in new:
                    done += 1
                elif expected == 1:
                    text = text.replace(old, new); todo += 1
                elif present >= 1:
                    done += 1
                else:
                    bad.append(index)
            if bad:
                if optional and todo == 0 and done == 0:
                    rows.append(("skipped", "hunks", label + f" (anchors {bad} absent: the launcher derives its routes)"))
                else:
                    problems.append(f"anchor: {label} hunks {bad} found neither as staged nor as applied")
            elif todo:
                rows.append(("pending", "hunks", label + f" ({todo} hunk(s))")); state[path] = text.encode("utf8")
            else:
                rows.append(("applied", "hunks", label))
    return rows, state, problems


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--root", required=True)
    ap.add_argument("--group", default="core", help="core|routing|nx-watchers|service|all, comma separated")
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--backup")
    ap.add_argument("--undo")
    ap.add_argument("--ignore-running", action="store_true")
    ap.add_argument("-v", action="store_true")
    args = ap.parse_args()
    root = os.path.realpath(args.root)
    if args.undo:
        manifest = json.load(open(os.path.join(args.undo, "manifest.json"), encoding="utf8"))
        for rel in manifest["new"]:
            if os.path.exists(os.path.join(root, rel)):
                os.remove(os.path.join(root, rel))
        for rel, name in manifest["edited"].items():
            os.makedirs(os.path.dirname(os.path.join(root, rel)), exist_ok=True)
            shutil.copyfile(os.path.join(args.undo, "files", name), os.path.join(root, rel))
        print("restored", len(manifest["edited"]), "file(s), removed", len(manifest["new"]), "new file(s)")
        return 0
    groups = ORDER if args.group == "all" else args.group.split(",")
    edits = json.load(open(os.path.join(HERE, "staged", "edits.json"), encoding="utf8"))["edits"]
    rows, state, problems = plan(root, groups, edits)
    counts = {}
    for status, kind, label in rows:
        counts[status] = counts.get(status, 0) + 1
        if status in ("pending", "skipped") or args.v:
            print(f"{status:8} {kind:8} {label}")
    for problem in problems:
        print("REFUSED", problem)
    longest = max((len(os.path.relpath(p, root)) for p in state), default=0)
    print("summary", json.dumps(counts), "problems", len(problems), "longest path", longest)
    if problems:
        return 1
    if not args.write:
        print("dry run ok:", len(state), "file(s) would change")
        return 0
    busy = [] if args.ignore_running else running(root)
    if busy:
        print("REFUSED a dependency sync or graph command is running (pass --ignore-running to override):")
        for line in busy:
            print("  ", line)
        return 1
    backup = args.backup or os.path.join(HERE, "backup-" + time.strftime("%Y%m%d-%H%M%S"))
    os.makedirs(os.path.join(backup, "files"), exist_ok=True)
    manifest = {"new": [], "edited": {}}
    for path in state:
        rel = os.path.relpath(path, root)
        if os.path.exists(path):
            name = sha(rel.encode("utf8"))[:24]
            shutil.copyfile(path, os.path.join(backup, "files", name)); manifest["edited"][rel] = name
        else:
            manifest["new"].append(rel)
    json.dump(manifest, open(os.path.join(backup, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    for path, data in state.items():
        if data is None:
            os.remove(path); continue
        os.makedirs(os.path.dirname(path), exist_ok=True)
        temporary = path + ".agent-session.tmp"
        open(temporary, "wb").write(data)
        os.replace(temporary, path)
    print("written", len(state), "file(s); backup", backup)
    return 0


sys.exit(main())
