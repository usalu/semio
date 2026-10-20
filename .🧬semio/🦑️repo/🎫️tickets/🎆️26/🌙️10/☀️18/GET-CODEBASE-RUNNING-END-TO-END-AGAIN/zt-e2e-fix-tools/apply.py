"""Applies plan.json to a repo root: content-hash check per target, refuses on drift, idempotent, --dry-run, --root DIR, --include-frozen."""
import argparse, hashlib, json, os, subprocess, sys

parser = argparse.ArgumentParser()
parser.add_argument("--root", default=os.getcwd())
parser.add_argument("--plan", default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "plan.json"))
parser.add_argument("--dry-run", action="store_true")
parser.add_argument("--include-frozen", action="store_true", help="also edit package.json/bun.lock (only after PAUSE LIFTED)")
parser.add_argument("--ignore-running", action="store_true", help="skip the running dependency sync / show projects guard")
args = parser.parse_args()
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()

if not args.dry_run and not args.ignore_running:
    running = subprocess.run(["ps", "-axo", "args="], capture_output=True, text=True).stdout.splitlines()
    busy = [line for line in running if ("dependencies sync" in line or "dependencies lock" in line or "show projects" in line) and "apply.py" not in line and "grep" not in line]
    if busy:
        sys.exit("refusing: a dependency sync or show projects is running\n" + "\n".join(busy[:3]))

plan = json.load(open(args.plan, encoding="utf-8"))
actions, drift = [], []
for item in plan["files"]:
    if item["frozen"] and not args.include_frozen:
        actions.append(("skip-frozen", item["path"], None)); continue
    path = os.path.join(args.root, item["path"])
    text = open(path, encoding="utf-8", newline="").read(); digest = sha(text)
    if digest == item["afterSha256"]:
        actions.append(("already", item["path"], None)); continue
    if digest != item["sha256"]:
        drift.append(item["path"]); continue
    for row in item["replacements"]:
        found = text.count(row["old"])
        if found != row["count"]:
            drift.append(f'{item["path"]} (anchor count {found} != {row["count"]}: {row["old"][:60]!r})'); break
        text = text.replace(row["old"], row["new"])
    else:
        if sha(text) != item["afterSha256"]: drift.append(item["path"] + " (result hash)")
        else: actions.append(("edit", item["path"], text))
for item in plan["newFiles"]:
    path = os.path.join(args.root, item["path"])
    if os.path.exists(path):
        (actions.append(("already", item["path"], None)) if sha(open(path, encoding="utf-8", newline="").read()) == item["sha256"] else drift.append(item["path"] + " (exists, differs)"))
    else:
        actions.append(("create", item["path"], item["content"]))
if drift:
    print("DRIFT, nothing written:"); [print("  " + d) for d in drift]; sys.exit(2)
counts = {}
for kind, path, content in actions:
    counts[kind] = counts.get(kind, 0) + 1
    if kind in ("edit", "create") and not args.dry_run:
        full = os.path.join(args.root, path); os.makedirs(os.path.dirname(full), exist_ok=True)
        open(full, "w", encoding="utf-8", newline="").write(content)
print(("DRY RUN " if args.dry_run else "APPLIED ") + json.dumps(counts))
