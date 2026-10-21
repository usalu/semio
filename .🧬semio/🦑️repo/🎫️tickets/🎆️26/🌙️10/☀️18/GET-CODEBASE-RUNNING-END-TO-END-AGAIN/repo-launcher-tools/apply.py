"""Applies the repo-launcher plan to a root: dry run unless --apply; content hashes and anchor counts are checked first, so any drift refuses everything; the apply is all-or-nothing with rollback.
Usage: apply.py [--root DIR] [--group taxonomy,launcher,cargo,rust,bootstrap,docs|all] [--apply] [--ignore-running]
Groups: taxonomy (register the Go member names of the launcher; verify the taxonomy after it), launcher (new module, go.work, cli integration, shared entrypoint vectors) must come first; bootstrap (shell and PowerShell bootstrap, devcontainer, CI) needs it; cargo (rustc wrapper through the launcher) needs it; rust (retire the Rust bun launcher); docs (sweeps of the bun repo spelling)."""
import argparse, hashlib, json, os, re, subprocess, sys
parser = argparse.ArgumentParser()
parser.add_argument("--root", default=os.getcwd())
parser.add_argument("--group", default="all")
parser.add_argument("--apply", action="store_true")
parser.add_argument("--ignore-running", action="store_true")
args = parser.parse_args()
here = os.path.dirname(os.path.abspath(__file__))
root = os.path.abspath(args.root)
sha = lambda data: hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf-8")).hexdigest()
plan = json.load(open(os.path.join(here, "plan.json"), encoding="utf-8"))
names = list(plan["groups"]) if args.group == "all" else args.group.split(",")
for name in names:
    if name not in plan["groups"]: sys.exit(f"unknown group {name}")
if args.apply and not args.ignore_running:
    running = subprocess.run(["ps", "-axo", "args="], capture_output=True, text=True).stdout.splitlines()
    busy = [line for line in running if ("dependencies sync" in line or "dependencies lock" in line or "show projects" in line) and "apply.py" not in line and "grep" not in line]
    if busy: sys.exit("refusing: a dependency sync or show projects is running\n" + "\n".join(busy[:3]))
def read(path):
    full = os.path.join(root, path)
    if os.path.islink(full): raise SystemExit(f"refusing symlink {path}")
    return open(full, encoding="utf-8", newline="").read() if os.path.exists(full) else None
writes, removals, problems, report = {}, [], [], {"already": 0, "create": 0, "edit": 0, "rewrite": 0, "delete": 0}
def current(path): return writes[path][0] if path in writes else read(path)
for name in names:
    group = plan["groups"][name]
    if name in ("bootstrap", "cargo") and "launcher" not in names and not os.path.exists(os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json")): problems.append(f"{name} needs the launcher group")
    for row in group["creates"]:
        full = os.path.join(root, row["path"])
        if len(full) > 256: problems.append(f'{row["path"]} (absolute path of {len(full)} characters exceeds 256)'); continue
        data = open(os.path.join(here, "blobs", row["sha256"]), "rb").read()
        if sha(data) != row["sha256"]: problems.append(f'{row["path"]} (blob changed since the plan)'); continue
        text = data.decode("utf-8"); have = read(row["path"])
        if have is None: writes[row["path"]] = (text, row["mode"], True); report["create"] += 1
        elif have == text: report["already"] += 1
        else: problems.append(f'{row["path"]} (exists with other content)')
    for row in group["edits"]:
        text = current(row["path"])
        if text is None: problems.append(f'{row["path"]} (missing)'); continue
        if "old" in row:
            found = text.count(row["old"])
            done = text.count(row["new"]) >= row["count"] if row["old"] in row["new"] else found == 0 and (row["new"] in text)
            if done: report["already"] += 1; continue
            if found != row["count"]: problems.append(f'{row["path"]} (anchor count {found} != {row["count"]}: {row["old"][:60]!r})'); continue
            after = text.replace(row["old"], row["new"])
        else:
            after, found = re.subn(row["pattern"], row["repl"], text)
            if found == 0: report["already"] += 1; continue
            if found != row["count"]: problems.append(f'{row["path"]} (sweep count {found} != {row["count"]}: {row["pattern"]})'); continue
        mode = writes[row["path"]][1] if row["path"] in writes else os.stat(os.path.join(root, row["path"])).st_mode & 0o777
        writes[row["path"]] = (after, mode, writes.get(row["path"], (None, None, False))[2]); report["edit"] += 1
    for row in group["rewrites"]:
        text = current(row["path"])
        if text is not None and sha(text) == row["afterSha256"]: report["already"] += 1; continue
        if text is None or sha(text) != row["sha256"]: problems.append(f'{row["path"]} (drifted: planned {row["sha256"][:12]})'); continue
        writes[row["path"]] = (row["content"], os.stat(os.path.join(root, row["path"])).st_mode & 0o777, False); report["rewrite"] += 1
    for row in group["deletes"]:
        text = read(row["path"])
        if text is None: report["already"] += 1
        elif sha(text) != row["sha256"]: problems.append(f'{row["path"]} (drifted before delete)')
        else: removals.append(row["path"]); report["delete"] += 1
print(json.dumps({"dryRun": not args.apply, "root": root, "groups": names, **report, "problems": problems}, ensure_ascii=False, indent=1))
if problems: sys.exit(2)
if not args.apply: sys.exit(0)
backups, created = {}, []
try:
    for path, (text, mode, fresh) in writes.items():
        full = os.path.join(root, path); backups[path] = read(path)
        missing = []; directory = os.path.dirname(full)
        while not os.path.isdir(directory): missing.append(directory); directory = os.path.dirname(directory)
        os.makedirs(os.path.dirname(full), exist_ok=True); created.extend(reversed(missing))
        temporary = full + ".launcher.tmp"
        open(temporary, "w", encoding="utf-8", newline="").write(text); os.chmod(temporary, mode)
        os.replace(temporary, full)
    for path in removals:
        full = os.path.join(root, path); backups[path] = read(path); os.remove(full)
except BaseException as error:
    for path, before in backups.items():
        full = os.path.join(root, path)
        if before is None:
            if os.path.exists(full): os.remove(full)
        else:
            os.makedirs(os.path.dirname(full), exist_ok=True); open(full, "w", encoding="utf-8", newline="").write(before)
    for directory in reversed(created):
        try: os.rmdir(directory)
        except OSError: pass
    sys.exit(f"rolled back: {error}")
for path in removals:
    try: os.rmdir(os.path.dirname(os.path.join(root, path)))
    except OSError: pass
print("applied")
