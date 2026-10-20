"""Anchored apply of staged/plan.json (bzl-default). Dry run by default; --write applies all-or-nothing; --undo reverses the same groups.
Groups: prep, setup, commands (safe now, behaviour neutral), manifest (needs `bun repo dependencies lock --all` afterwards), flip (Bazel becomes the default;
needs --flip-gate-passed, apply only after the three products pass end to end on Bazel). `--group all` = prep,setup,commands.
Standard library only (macOS, Linux, Windows, devcontainer)."""
import argparse, datetime, hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
parser = argparse.ArgumentParser()
parser.add_argument("--root", default=os.getcwd())
parser.add_argument("--plan", default=os.path.join(HERE, "staged", "plan.json"))
parser.add_argument("--group", action="append", default=[], help="prep|setup|commands|manifest|flip|all (repeatable)")
parser.add_argument("--write", action="store_true", help="apply (default is a dry run)")
parser.add_argument("--undo", action="store_true", help="reverse the selected groups while the files still hold our result")
parser.add_argument("--flip-gate-passed", action="store_true", help="required to write or undo the flip group: confirms the three products passed on Bazel")
parser.add_argument("--ignore-running", action="store_true", help="skip the running dependency sync / show projects guard")
args = parser.parse_args()
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()

groups = []
for name in args.group:
    groups += ["prep", "setup", "commands"] if name == "all" else [name]
known = {"prep", "setup", "commands", "manifest", "flip"}
if not groups or any(name not in known for name in groups):
    sys.exit("select groups: --group " + "|".join(sorted(known)) + "|all")
if "flip" in groups and args.write and not args.flip_gate_passed:
    sys.exit("refusing: the flip group needs --flip-gate-passed (architecture course, semio-tech play and mit-bestand demonstrator green on --tool=bazel)")
if args.write and not args.ignore_running:
    running = subprocess.run(["ps", "-axo", "args="], capture_output=True, text=True).stdout.splitlines()
    busy = [line for line in running if ("dependencies sync" in line or "dependencies lock" in line or "show projects" in line) and "apply.py" not in line and "grep" not in line]
    if busy:
        sys.exit("refusing: a dependency sync or show projects is running\n" + "\n".join(busy[:3]))

plan = json.load(open(args.plan, encoding="utf-8"))
results, drift, outputs = [], [], {}
for item in plan["files"]:
    edits = [e for e in item["edits"] if e["group"] in groups]
    if not edits:
        continue
    path = os.path.join(args.root, item["path"])
    if not os.path.isfile(path):
        drift.append(item["path"] + " (file missing)")
        continue
    text = open(path, encoding="utf-8", newline="").read()
    changed = False
    for e in edits:
        old, new, counts = e["old"], e["new"], e["counts"]
        n_old, n_new = text.count(old), (text.count(new) if new else 0)
        inserts = bool(new) and old in new
        if args.undo:
            undoable = (n_new in counts and n_new > 0) if inserts else (n_old == 0 and n_new > 0)
            untouched = (n_new == 0 and n_old > 0) if inserts else (n_old in counts and n_old > 0)
            if undoable:
                text = text.replace(new, old); changed = True; results.append(("undo", e["id"], item["path"]))
            elif untouched:
                results.append(("not-applied", e["id"], item["path"]))
            elif e["optional"] and n_old == 0 and n_new == 0:
                results.append(("absent", e["id"], item["path"]))
            else:
                drift.append(f'{item["path"]} [{e["id"]}] cannot undo: anchor counts old={n_old} new={n_new}')
            continue
        if inserts:
            applied, pending = n_new in counts and n_new > 0, n_new == 0 and n_old in counts and n_old > 0
        else:
            applied, pending = n_old == 0 and n_new > 0, n_old in counts and n_old > 0
        if e["optional"] and n_old == 0 and n_new == 0:
            results.append(("absent", e["id"], item["path"]))
        elif pending:
            text = text.replace(old, new); changed = True; results.append(("edit", e["id"], item["path"]))
        elif applied:
            results.append(("already", e["id"], item["path"]))
        else:
            drift.append(f'{item["path"]} [{e["id"]}] anchor counts old={n_old} new={n_new}, expected {counts}: {old[:60]!r}')
    if changed:
        outputs[item["path"]] = text
if drift:
    print("DRIFT, nothing written:")
    for line in drift:
        print("  " + line)
    sys.exit(2)
tally = {}
for kind, _, _ in results:
    tally[kind] = tally.get(kind, 0) + 1
if args.write:
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    backup = os.path.join(HERE, "backup", stamp)
    for rel, text in outputs.items():
        full = os.path.join(args.root, rel)
        saved = os.path.join(backup, rel)
        os.makedirs(os.path.dirname(saved), exist_ok=True)
        open(saved, "w", encoding="utf-8", newline="").write(open(full, encoding="utf-8", newline="").read())
    for rel, text in outputs.items():
        full = os.path.join(args.root, rel)
        temporary = full + ".bzl-default.tmp"
        open(temporary, "w", encoding="utf-8", newline="").write(text)
        os.replace(temporary, full)
print(("UNDONE " if args.undo and args.write else "UNDO DRY RUN " if args.undo else "APPLIED " if args.write else "DRY RUN ") + json.dumps({"groups": groups, "files": len(outputs), **tally}))
if args.write and not args.undo and "manifest" in groups and outputs:
    print("next: bun repo dependencies lock --all   (the lock row and the manifest row must agree)")
if args.write and not args.undo and "flip" in groups and outputs:
    print("next: rebuild the repo MCP server (Go and Rust instructions changed), rerun the doc laws and the files listed in result-bzl-default.md")
