#!/usr/bin/env python3
"""🏊️ Anchored, all-or-nothing apply of the graph farm set (stdlib only, cross-platform).

usage: apply.py [check|diff|apply] [--write] [--root REPO_ROOT] [--ignore-running] [--spec NAME]
  check  (default)  per file: pending | applied | DRIFT; exit 0 applied, 1 pending, 2 drift
  diff              unified diff of what apply would write
  apply             DRY RUN unless --write; refuses on any drift; refuses while a graph command runs (--ignore-running)
Every anchor must occur exactly once at its point of application or its replacement must already be present (idempotent). Files are written through a temporary file and a rename; an error rolls everything back."""
import difflib, json, os, pathlib, subprocess, sys, tempfile

HERE = pathlib.Path(__file__).resolve().parent
args = sys.argv[1:]
root = HERE
while not (root / ".git").exists() and root != root.parent:
    root = root.parent
if "--root" in args:
    root = pathlib.Path(args[args.index("--root") + 1]).resolve()
mode = next((a for a in args if a in ("check", "diff", "apply")), "check")
write = "--write" in args
name = args[args.index("--spec") + 1] if "--spec" in args else "ops"
spec = json.loads((HERE / ("staged/" + name + ".json")).read_text(encoding="utf8"))


def running():
    if os.name == "nt":
        return []
    out = subprocess.run(["ps", "-axo", "pid,command"], capture_output=True, text=True).stdout
    needles = ("repo show projects", "repo exec", "dependencies sync", "dependencies lock", "repo tools setup")
    return [line.strip()[:140] for line in out.splitlines() if any(n in line for n in needles) and "apply.py" not in line and "ps -axo" not in line]


plan, drift, applied = [], [], 0
by_file = {}
for op in spec["edits"]:
    by_file.setdefault(op["file"], []).append(op)
for file, items in by_file.items():
    path = root / file
    if not path.exists():
        drift.append((file, "missing file"))
        continue
    text = path.read_text(encoding="utf8")
    new = text
    for op in items:
        anchor, replacement = op["anchor"], op["replacement"]
        if replacement in new:
            applied += 1
            continue
        count = new.count(anchor)
        if count != 1:
            drift.append((file, "anchor occurs %d times: %r" % (count, anchor[:70])))
            continue
        new = new.replace(anchor, replacement)
    if new != text:
        plan.append((path, text, new))
for item in spec["newFiles"]:
    path = root / item["file"]
    data = (HERE / "staged" / item["from"]).read_text(encoding="utf8") if not item["from"].startswith("staged/") else (HERE / item["from"]).read_text(encoding="utf8")
    if len(str(path)) > 256:
        drift.append((item["file"], "path longer than 256 characters"))
    if path.exists():
        if path.read_text(encoding="utf8") != data:
            drift.append((item["file"], "exists with different content"))
        continue
    plan.append((path, None, data))
print("%d file(s) pending, %d op(s) already applied, %d drift" % (len(plan), applied, len(drift)))
for file, why in drift:
    print("DRIFT", file[-90:], why)
if mode == "diff":
    for path, old, new in plan:
        sys.stdout.writelines(difflib.unified_diff((old or "").splitlines(True), new.splitlines(True), str(path)[-60:] + " (live)", str(path)[-60:] + " (staged)", n=1))
if mode == "apply":
    if drift:
        raise SystemExit("refused: drift")
    busy = [] if "--ignore-running" in args else running()
    if busy:
        raise SystemExit("refused: a graph command is running:\n" + "\n".join(busy))
    if not write:
        print("dry run (add --write to apply)")
        sys.exit(0)
    done = []
    try:
        for path, old, new in plan:
            path.parent.mkdir(parents=True, exist_ok=True)
            fd, temporary = tempfile.mkstemp(dir=path.parent, prefix=".graph-parallel-")
            os.close(fd)
            pathlib.Path(temporary).write_text(new, encoding="utf8")
            os.replace(temporary, path)
            done.append((path, old))
    except Exception:
        for path, old in done:
            if old is None:
                path.unlink()
            else:
                path.write_text(old, encoding="utf8")
        raise
    print("applied", len(done))
sys.exit(2 if drift else (1 if plan and mode == "check" else 0))
