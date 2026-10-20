"""Anchored apply of staged/ (repo-mcp-build). Dry run by default; --write applies all-or-nothing; --undo reverses while the files still hold our result.
Every file under staged/ is a whole file at its repository path. A file listed in base-hashes.json is a modification and applies only while the live file still has the recorded SHA-256 (drift is refused); any other file is new and applies only where the path is absent.
Standard library only (macOS, Linux, Windows, devcontainer)."""
import argparse, datetime, hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
parser = argparse.ArgumentParser()
parser.add_argument("--root", default=os.getcwd())
parser.add_argument("--write", action="store_true", help="apply (default is a dry run)")
parser.add_argument("--undo", action="store_true", help="restore the recorded originals and remove the new files while they still hold our result")
parser.add_argument("--ignore-running", action="store_true", help="skip the running show projects / dependency guard")
args = parser.parse_args()
sha = lambda data: hashlib.sha256(data).hexdigest()
staged_root = os.path.join(HERE, "staged")
base = json.load(open(os.path.join(HERE, "base-hashes.json"), encoding="utf-8"))
originals = os.path.join(HERE, "originals")

if args.write and not args.ignore_running:
    running = subprocess.run(["ps", "-axo", "args="], capture_output=True, text=True).stdout.splitlines() if os.name != "nt" else []
    busy = [line for line in running if ("dependencies sync" in line or "dependencies lock" in line or "show projects" in line) and "apply.py" not in line and "grep" not in line]
    if busy:
        sys.exit("refusing: a dependency sync or show projects is running\n" + "\n".join(busy[:3]))

files = []
for folder, _, names in os.walk(staged_root):
    for name in names:
        relative = os.path.relpath(os.path.join(folder, name), staged_root)
        files.append(relative)
files.sort()
plan, drift = [], []
for relative in files:
    staged = open(os.path.join(staged_root, relative), "rb").read()
    live_path = os.path.join(args.root, relative)
    live = open(live_path, "rb").read() if os.path.isfile(live_path) else None
    modification = relative in base
    if args.undo:
        if live is None:
            plan.append(("absent", relative, None))
        elif live == staged:
            plan.append(("undo", relative, None))
        elif modification and sha(live) == base[relative]:
            plan.append(("not-applied", relative, None))
        else:
            drift.append(f"{relative}: holds neither our result nor the recorded original")
    elif live == staged:
        plan.append(("already", relative, staged))
    elif modification:
        if live is not None and sha(live) == base[relative]:
            plan.append(("edit", relative, staged))
        else:
            drift.append(f"{relative}: live sha256 {sha(live)[:12] if live else 'missing'} != recorded {base[relative][:12]}")
    elif live is None:
        plan.append(("create", relative, staged))
    else:
        drift.append(f"{relative}: exists and differs; refusing to overwrite a file this set did not record")
for state, relative, _ in plan:
    print(f"{state:12} {relative}")
if drift:
    print("\nDRIFT (nothing written):", file=sys.stderr)
    for line in drift:
        print("  " + line, file=sys.stderr)
    sys.exit(2)
if not args.write:
    print(f"\ndry run: {len(plan)} files, no drift. Re-run with --write.")
    sys.exit(0)
stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
if args.undo:
    for state, relative, _ in plan:
        live_path = os.path.join(args.root, relative)
        if state != "undo":
            continue
        backup = os.path.join(originals, relative)
        if relative in base:
            shutil.copy2(backup, live_path)
        else:
            os.remove(live_path)
    print("undone")
    sys.exit(0)
os.makedirs(originals, exist_ok=True)
written = []
try:
    for state, relative, staged in plan:
        if state not in ("edit", "create"):
            continue
        live_path = os.path.join(args.root, relative)
        if state == "edit":
            backup = os.path.join(originals, relative)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.isfile(backup):
                shutil.copy2(live_path, backup)
        os.makedirs(os.path.dirname(live_path), exist_ok=True)
        temporary = live_path + ".stage-" + stamp
        with open(temporary, "wb") as handle:
            handle.write(staged)
        os.replace(temporary, live_path)
        written.append(relative)
except Exception:
    print("failed; restoring", file=sys.stderr)
    for relative in written:
        live_path = os.path.join(args.root, relative)
        backup = os.path.join(originals, relative)
        if relative in base and os.path.isfile(backup):
            shutil.copy2(backup, live_path)
        elif relative not in base and os.path.isfile(live_path):
            os.remove(live_path)
    raise
print(f"applied {len(written)} files; originals kept in {originals}")
