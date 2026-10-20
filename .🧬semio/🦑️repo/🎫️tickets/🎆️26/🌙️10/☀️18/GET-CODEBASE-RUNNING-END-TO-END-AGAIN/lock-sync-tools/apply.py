#!/usr/bin/env python3
"""🧷️ Round-5 manifest batch (lock-sync): anchored, all-or-nothing, dry run by default. Standard library only (macOS, Linux, Windows, devcontainer).
  apply.py                      dry run of every group on the live tree (read only)
  apply.py --write              apply (refuses on drift, while a dependency sync/lock/ensure or show projects runs, or while .nx/installation dangles for the nx group)
  apply.py --undo [--write]     reverse the same groups while the files still hold our result
Groups: manifests (T0 recipe row, root Nx rows, parsers member in all six roots + new package), closure (closure-scoped JS install code), nx (Nx from the pinned installation). --group G (repeatable), --without G (repeatable)."""
import argparse, datetime, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ap = argparse.ArgumentParser()
ap.add_argument("--root", default=os.path.abspath(os.path.join(C, *[".."] * 7)))
ap.add_argument("--plan", default=os.path.join(HERE, "staged", "plan.json"))
ap.add_argument("--group", action="append", default=[]); ap.add_argument("--without", action="append", default=[])
ap.add_argument("--write", action="store_true"); ap.add_argument("--undo", action="store_true")
ap.add_argument("--ignore-running", action="store_true"); ap.add_argument("--verbose", action="store_true")
a = ap.parse_args()
KNOWN = ["manifests", "closure", "nx"]
groups = [g for g in (a.group or KNOWN) if g not in a.without]
if any(g not in KNOWN for g in groups): sys.exit("groups: " + ", ".join(KNOWN))
u16 = lambda s: len(s.encode("utf-16-le")) // 2
if a.write and not a.ignore_running:
    ps = subprocess.run(["ps", "-axo", "args="], capture_output=True, text=True).stdout.splitlines()
    busy = [l for l in ps if any(k in l for k in ("dependencies sync", "dependencies lock", "dependencies ensure", "show projects")) and "apply.py" not in l and "grep" not in l]
    if busy: sys.exit("refusing: a dependency sync/lock/ensure or show projects is running (loaded build modules and manifests must not change under it)\n" + "\n".join(x[:160] for x in busy[:4]))
if a.write and "nx" in groups and not a.undo and not os.path.isdir(os.path.join(a.root, ".nx", "installation")):
    sys.exit("refusing: .nx/installation does not resolve (run `bun repo tools setup nx` first); the nx group moves Nx resolution there")
plan = json.load(open(a.plan, encoding="utf-8"))
drift, results, outputs, created = [], [], {}, {}
for item in plan["files"]:
    edits = [e for e in item["edits"] if e["group"] in groups]
    if not edits: continue
    path = os.path.join(a.root, item["path"])
    if not os.path.isfile(path): drift.append(item["path"] + " (file missing)"); continue
    text = open(path, encoding="utf-8", newline="").read(); changed = False
    for e in edits:
        old, new, counts = e["old"], e["new"], e["counts"]
        n_old, n_new = text.count(old), (text.count(new) if new else 0)
        inserts = bool(new) and old in new
        if a.undo:
            undoable = (n_new in counts and n_new > 0) if inserts else (n_old == 0 and n_new > 0)
            untouched = (n_new == 0 and n_old > 0) if inserts else (n_old in counts and n_old > 0)
            if undoable: text = text.replace(new, old); changed = True; results.append(("undo", e["id"], item["path"]))
            elif untouched: results.append(("not-applied", e["id"], item["path"]))
            else: drift.append(f'{item["path"]} [{e["id"]}] cannot undo: old={n_old} new={n_new}')
            continue
        applied = (n_new in counts and n_new > 0 and n_new >= n_old) if inserts else (n_old == 0 and n_new > 0)
        pending = (n_new == 0 and n_old in counts) if inserts else (n_old in counts and n_old > 0)
        if pending: text = text.replace(old, new); changed = True; results.append(("edit", e["id"], item["path"]))
        elif applied: results.append(("already", e["id"], item["path"]))
        else: drift.append(f'{item["path"]} [{e["id"]}] anchor counts old={n_old} new={n_new}, expected {counts}: {old[:70]!r}')
    if changed: outputs[item["path"]] = text
for n in plan.get("newFiles", []):
    if n["group"] not in groups: continue
    path = os.path.join(a.root, n["path"])
    content = open(os.path.join(HERE, "staged", "new", n["path"]), encoding="utf-8", newline="").read()
    present = os.path.isfile(path) and open(path, encoding="utf-8", newline="").read()
    if u16(os.path.abspath(path)) > 256: drift.append(n["path"] + " exceeds 256 UTF-16 units")
    elif a.undo:
        if present == content: created[n["path"]] = None; results.append(("remove", n["group"], n["path"]))
        elif present is False: results.append(("not-applied", n["group"], n["path"]))
        else: drift.append(n["path"] + " differs from our file, not removing")
    elif present is False: created[n["path"]] = content; results.append(("create", n["group"], n["path"]))
    elif present == content: results.append(("already", n["group"], n["path"]))
    else: drift.append(n["path"] + " exists and differs from the staged file")
for rel, text in outputs.items():
    if rel.endswith(".json"):
        try:
            doc = json.loads(text)
            ws = doc.get("workspaces") if rel.endswith("package.json") else None
            if isinstance(ws, list) and (ws != sorted(ws) or len(ws) != len(set(ws))): drift.append(rel + ": workspaces would be unsorted or duplicated")
        except ValueError as err: drift.append(rel + ": result is not valid JSON: " + str(err))
if drift:
    print("DRIFT, nothing written:"); [print("  " + d) for d in drift]; sys.exit(2)
if a.verbose: [print(f"  {k:11} {i:36} {p[-100:]}") for k, i, p in results]
tally = {}
for k, _, _ in results: tally[k] = tally.get(k, 0) + 1
if a.write:
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    backup = os.path.join(C, "🗑️generated", "lock-sync", "backup", stamp)
    originals = {rel: open(os.path.join(a.root, rel), encoding="utf-8", newline="").read() for rel in outputs}
    for rel, text in originals.items():
        dst = os.path.join(backup, rel); os.makedirs(os.path.dirname(dst), exist_ok=True); open(dst, "w", encoding="utf-8", newline="").write(text)
    done = []
    try:
        for rel, text in outputs.items():
            full = os.path.join(a.root, rel); tmp = full + ".lock-sync.tmp"
            open(tmp, "w", encoding="utf-8", newline="").write(text); os.replace(tmp, full); done.append(rel)
        for rel, content in created.items():
            full = os.path.join(a.root, rel)
            if content is None: os.remove(full)
            else: os.makedirs(os.path.dirname(full), exist_ok=True); open(full, "w", encoding="utf-8", newline="").write(content)
            done.append(rel)
    except Exception as err:
        for rel in done:
            if rel in originals: open(os.path.join(a.root, rel), "w", encoding="utf-8", newline="").write(originals[rel])
            elif rel in created and created[rel] is not None: os.remove(os.path.join(a.root, rel))
        sys.exit("ROLLED BACK: " + repr(err))
    ledger = os.path.join(C, "🗑️generated", "lock-sync", "ledger.jsonl")
    with open(ledger, "a", encoding="utf-8") as f: f.write(json.dumps({"at": stamp, "undo": a.undo, "groups": groups, "files": sorted(list(outputs) + list(created)), "backup": backup}, ensure_ascii=False) + "\n")
print(("UNDONE " if a.undo and a.write else "UNDO DRY RUN " if a.undo else "APPLIED " if a.write else "DRY RUN ") + json.dumps({"groups": groups, "files": len(outputs) + len(created), **tally}, ensure_ascii=False))
if a.write and not a.undo and outputs: print("next: python3 lockdelta.py (predicted lock delta), then run.sh")
