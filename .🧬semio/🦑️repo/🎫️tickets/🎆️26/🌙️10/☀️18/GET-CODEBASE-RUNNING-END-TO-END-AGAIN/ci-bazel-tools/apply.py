#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the ci-bazel set (CI workflow, CI report, Bazel test results export).

  python3 apply.py --root <checkout> [--group report|adapter|workflow|all]      dry run (default): reads, writes nothing
  python3 apply.py --root <checkout> --group all --write [--backup DIR]         writes; originals of changed files go to DIR
  python3 apply.py --root <checkout> --undo DIR [--write]                       restores the originals, removes new files

Groups
  report    new module `🧾️ci-report` (neutral report: cache hit rate, cache proof, plan parity, JUnit summary) with schema,
            language-agnostic fixture and tests. Pure additions.
  adapter   new Bazel module `🧾️test-results` (exports every executed test's JUnit XML to SEMIO_BUILD_TEST_RESULTS_DIR) and
            two anchored edits in the Bazel operations module that call it. Needs nothing else.
  workflow  `.github/workflows/build-system.yml` (replaced; the live file must still be the base hash recorded by build.py),
            two local composite actions, one dependabot directory edit. Calls the report, so `report` must be applied before
            or in the same run.

A new file is written only when absent or byte-identical; an edit only when its anchor occurs exactly once; a replaced file
only when it still equals its recorded base. Anything else is drift: nothing is written and the exit code is 1.
Stdlib only: macOS, Linux, Windows, devcontainer.
"""
import argparse, hashlib, json, os, shutil, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
ORDER = ["report", "adapter", "workflow"]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--root", default=os.path.abspath(os.path.join(HERE, *[".."] * 8)))
    parser.add_argument("--group", default="all")
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--dry-run", action="store_true", help="the default; accepted for explicitness")
    parser.add_argument("--backup")
    parser.add_argument("--undo")
    args = parser.parse_args()
    root = args.root
    if args.dry_run and args.write:
        raise SystemExit("--dry-run and --write exclude each other")
    plan = json.load(open(os.path.join(STAGED, "targets.json"), encoding="utf8"))
    groups = ORDER if args.group == "all" else [args.group]
    for group in groups:
        if group not in plan["groups"]:
            raise SystemExit(f"unknown group {group}")
    if "workflow" in groups and "report" not in groups:
        report_live = os.path.join(root, plan["groups"]["report"]["new"][0]["live"])
        if not os.path.exists(report_live):
            raise SystemExit("group workflow needs group report (applied before or in the same run)")
    live = lambda relative: os.path.join(root, *relative.split("/"))

    if args.undo:
        return undo(root, plan, args.undo, args.write, live)

    state, rows, problems = {}, [], []

    def current(path):
        if path in state:
            return state[path]
        return read(path) if os.path.exists(path) else None

    for group in groups:
        spec = plan["groups"][group]
        for row in spec.get("new", []):
            want = read(os.path.join(STAGED, row["staged"]))
            have = current(live(row["live"]))
            if have is None:
                rows.append((group, "new", row["live"])); state[live(row["live"])] = want
            elif have == want:
                rows.append((group, "applied", row["live"]))
            else:
                rows.append((group, "CONFLICT", row["live"])); problems.append(f"{row['live']}: exists with other content")
        for row in spec.get("replace", []):
            want = read(os.path.join(STAGED, row["staged"]))
            have = current(live(row["live"]))
            if have is None:
                rows.append((group, "CONFLICT", row["live"])); problems.append(f"{row['live']}: missing")
            elif have == want:
                rows.append((group, "applied", row["live"]))
            elif sha(have) == row["base"]:
                rows.append((group, "replace", row["live"])); state[live(row["live"])] = want
            else:
                rows.append((group, "DRIFT", row["live"])); problems.append(f"{row['live']}: changed since the base recorded by build.py ({sha(have)[:12]} != {row['base'][:12]})")
        for edit in spec.get("edits", []):
            path = live(edit["live"])
            have = current(path)
            if have is None:
                rows.append((group, "CONFLICT", edit["name"])); problems.append(f"{edit['live']}: missing"); continue
            text = have.decode("utf8")
            if edit["new"] in text and edit["old"] not in text.replace(edit["new"], ""):
                rows.append((group, "applied", edit["name"]))
            elif text.count(edit["old"]) == 1:
                rows.append((group, "edit", edit["name"])); state[path] = text.replace(edit["old"], edit["new"], 1).encode("utf8")
            else:
                rows.append((group, "DRIFT", edit["name"])); problems.append(f"{edit['name']}: anchor occurs {text.count(edit['old'])} times in {edit['live']}")
    for group, status, name in rows:
        print(f"{status:9} {group:9} {name}")
    summary = {}
    for _, status, _ in rows:
        summary[status] = summary.get(status, 0) + 1
    print("summary", json.dumps(summary, sort_keys=True), "problems", len(problems))
    for problem in problems:
        print("  -", problem)
    if problems:
        return 1
    if not args.write:
        print("dry run: nothing written (pass --write)")
        return 0
    backup = args.backup or os.path.join(HERE, "backup-" + time.strftime("%Y%m%d-%H%M%S"))
    manifest = {"created": [], "changed": []}
    for path, data in state.items():
        relative = os.path.relpath(path, root).replace(os.sep, "/")
        if os.path.exists(path):
            target = os.path.join(backup, *relative.split("/"))
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copy2(path, target)
            manifest["changed"].append({"path": relative, "applied": sha(data)})
        else:
            manifest["created"].append({"path": relative, "applied": sha(data)})
    os.makedirs(backup, exist_ok=True)
    json.dump(manifest, open(os.path.join(backup, "manifest.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
    for path, data in state.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        temporary = path + ".ci-bazel.tmp"
        with open(temporary, "wb") as handle:
            handle.write(data)
        os.replace(temporary, path)
    print("written; backup", backup)
    return 0


def undo(root, plan, backup, write, live):
    manifest = json.load(open(os.path.join(backup, "manifest.json"), encoding="utf8"))
    problems = []
    for row in manifest["created"] + manifest["changed"]:
        path = live(row["path"])
        if not os.path.exists(path) or sha(read(path)) != row["applied"]:
            problems.append(f"{row['path']}: no longer holds the applied content")
    for problem in problems:
        print("  -", problem)
    if problems:
        return 1
    print("undo plan:", len(manifest["created"]), "created files removed,", len(manifest["changed"]), "files restored")
    if not write:
        print("dry run: nothing written (pass --write)")
        return 0
    for row in manifest["created"]:
        os.remove(live(row["path"]))
    for row in manifest["changed"]:
        shutil.copy2(os.path.join(backup, *row["path"].split("/")), live(row["path"]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
