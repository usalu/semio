"""Applies the cli-zt plan to a root: content hash per target, refuses on drift, all-or-nothing with rollback, dry run unless --apply. Usage: apply.py [--root DIR] [--plan plan.json] [--apply] [--allow-anchored] [--ignore-running]"""
import argparse, hashlib, json, os, subprocess, sys

parser = argparse.ArgumentParser()
parser.add_argument("--root", default=os.getcwd())
parser.add_argument("--plan", default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "plan.json"))
parser.add_argument("--apply", action="store_true")
parser.add_argument("--allow-anchored", action="store_true", help="accept a changed target when every anchor still occurs exactly as often as planned")
parser.add_argument("--ignore-running", action="store_true")
args = parser.parse_args()
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
root = os.path.abspath(args.root)

if args.apply and not args.ignore_running:
    running = subprocess.run(["ps", "-axo", "args="], capture_output=True, text=True).stdout.splitlines()
    busy = [line for line in running if ("dependencies sync" in line or "dependencies lock" in line or "show projects" in line) and "apply.py" not in line and "grep" not in line]
    if busy: sys.exit("refusing: a dependency sync or show projects is running\n" + "\n".join(busy[:3]))

plan = json.load(open(args.plan, encoding="utf-8"))
read = lambda path: open(os.path.join(root, path), encoding="utf-8", newline="").read() if os.path.exists(os.path.join(root, path)) else None
actions, drift = [], []
for row in plan["writes"]:
    full = os.path.join(root, row["path"])
    if len(full) > 256: drift.append(f'{row["path"]} (absolute path of {len(full)} characters exceeds 256)'); continue
    current = read(row["path"])
    digest = None if current is None else sha(current)
    if digest == row["afterSha256"]: actions.append(("already", row["path"], None))
    elif digest == row["sha256"]: actions.append(("create" if current is None else "replace", row["path"], row["content"]))
    else: drift.append(f'{row["path"]} (found {digest}, planned {row["sha256"]})')
for row in plan["edits"]:
    current = read(row["path"])
    if current is None: drift.append(f'{row["path"]} (missing)'); continue
    digest = sha(current)
    if digest == row["afterSha256"]: actions.append(("already", row["path"], None)); continue
    if digest != row["sha256"] and not args.allow_anchored: drift.append(f'{row["path"]} (found {digest[:12]}, planned {row["sha256"][:12]})'); continue
    text, broken = current, None
    for change in row["replacements"]:
        found = text.count(change["old"])
        if found != change["count"]: broken = f'{row["path"]} (anchor count {found} != {change["count"]}: {change["old"][:60]!r})'; break
        text = text.replace(change["old"], change["new"])
    if broken: drift.append(broken)
    elif digest == row["sha256"] and sha(text) != row["afterSha256"]: drift.append(row["path"] + " (result hash)")
    else: actions.append(("edit" if digest == row["sha256"] else "edit-anchored", row["path"], text))
summary = {"dryRun": not args.apply, "root": root}
for kind in ("already", "create", "replace", "edit", "edit-anchored"): summary[kind] = sum(1 for action in actions if action[0] == kind)
summary["drift"] = drift
print(json.dumps(summary, ensure_ascii=False, indent=1))
if drift: sys.exit(2)
if not args.apply: sys.exit(0)

backups, created_files, created_dirs = {}, [], []
try:
    for kind, path, text in actions:
        if text is None: continue
        full = os.path.join(root, path)
        backups[path] = read(path)
        directory = os.path.dirname(full)
        missing = []
        while not os.path.isdir(directory): missing.append(directory); directory = os.path.dirname(directory)
        os.makedirs(os.path.dirname(full), exist_ok=True)
        created_dirs.extend(reversed(missing))
        temporary = full + ".cli-zt.tmp"
        open(temporary, "w", encoding="utf-8", newline="").write(text)
        if backups[path] is None: created_files.append(full)
        os.replace(temporary, full)
except BaseException as error:
    for path, before in backups.items():
        full = os.path.join(root, path)
        if before is None:
            if os.path.exists(full): os.remove(full)
        else: open(full, "w", encoding="utf-8", newline="").write(before)
    for directory in reversed(created_dirs):
        try: os.rmdir(directory)
        except OSError: pass
    sys.exit(f"rolled back: {error}")
print("applied")
