#!/usr/bin/env python3
"""🧷️ ide-free batch (wave 1021a): anchored, all-or-nothing, dry run by default. Standard library only (macOS, Linux, Windows, devcontainer).
  apply.py                     dry run of every group on the live tree (read only)
  apply.py --write             apply (refuses on drift, on frozen trees without --allow-frozen, while a dependency sync/lock or show projects runs)
  apply.py --undo [--write]    reverse the same groups while the files still hold our result
Groups (extension and manifests are OPT-IN: removing the VS Code extension is a user decision; they travel together, manifests also reach the round-5 lock-sync batch through merge-lock-sync.py --with-extension): plans (429 plans into their tickets: moves, archive tickets, ticket.json source->local; one all-or-nothing group), code (plan source and MCP surfaces, tests, library registries),
        guard (ide-census, dashboard authority), leftovers (.claude LSP, .github/hooks, .gitignore, tsconfig, python), agent-client (editor -> agent-client), extension (VS Code extension out of the repo),
        manifests (round-5 manifest rows: use --group manifests only together with lock-sync/run.py, see result-ide-free.md).
--group G (repeatable), --without G (repeatable), --allow-frozen (the coordinator's GO for the library/process trees)."""
import argparse, datetime, hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
KNOWN = ["plans", "code", "guard", "leftovers", "agent-client", "extension", "manifests"]
FROZEN = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", "🧰️framework/🔨️modules/🏃️process/"]
ap = argparse.ArgumentParser()
ap.add_argument("--root", default=os.path.abspath(os.path.join(C, *[".."] * 7)))
ap.add_argument("--plan", default=os.path.join(HERE, "staged", "plan.json"))
ap.add_argument("--group", action="append", default=[]); ap.add_argument("--without", action="append", default=[])
ap.add_argument("--write", action="store_true"); ap.add_argument("--undo", action="store_true")
ap.add_argument("--allow-frozen", action="store_true"); ap.add_argument("--ignore-running", action="store_true")
ap.add_argument("--verbose", action="store_true"); ap.add_argument("--limit-path", type=int, default=256)
a = ap.parse_args()
OPT_IN = ["extension", "manifests"]
groups = [g for g in (a.group or [k for k in KNOWN if k not in OPT_IN]) if g not in a.without]
if any(g not in KNOWN for g in groups): sys.exit("groups: " + ", ".join(KNOWN))
if ("extension" in groups) != ("manifests" in groups) and not a.write is False and not os.environ.get("IDE_FREE_PARTIAL_D1"):
    sys.exit("refusing: decision D1 needs the groups extension and manifests together (manifests also reach the round-5 batch through merge-lock-sync.py); set IDE_FREE_PARTIAL_D1=1 to override")
u16 = lambda s: len(s.encode("utf-16-le")) // 2
sha = lambda b: hashlib.sha256(b).hexdigest()
plan = json.load(open(a.plan, encoding="utf-8"))
absr = lambda rel: os.path.join(a.root, rel)
rd = lambda rel: open(absr(rel), encoding="utf-8", newline="").read()
drift, results = [], []

if a.write and not a.ignore_running:
    ps = subprocess.run(["ps", "-axo", "args="], capture_output=True, text=True).stdout.splitlines()
    busy = [l for l in ps if any(k in l for k in ("dependencies sync", "dependencies lock", "dependencies ensure", "show projects")) and "apply.py" not in l and "grep" not in l]
    if busy: sys.exit("refusing: a dependency sync/lock/ensure or show projects is running\n" + "\n".join(x[:160] for x in busy[:4]))

edits = [e for e in plan.get("edits", []) if e["group"] in groups]
creates = [c for c in plan.get("creates", []) if c["group"] in groups]
deletes = [d for d in plan.get("deletes", []) if d["group"] in groups]
moves = [m for m in plan.get("moves", []) if m["group"] in groups]
rmdirs = [r for r in plan.get("rmdirs", []) if r["group"] in groups]
writes = [w for w in plan.get("writes", []) if w["group"] in groups]

touched = {w["path"] for w in writes} | {e["path"] for e in edits} | {c["path"] for c in creates} | {d["path"] for d in deletes} | {m["src"] for m in moves} | {m["dst"] for m in moves}
frozen = sorted(p for p in touched if any(p.startswith(f) for f in FROZEN))
if a.write and frozen and not a.allow_frozen:
    sys.exit(f"refusing: {len(frozen)} path(s) under the live-frozen library/process trees (first: {frozen[0][:120]}); pass --allow-frozen after the coordinator's GO")
too_long = [p for p in touched if u16(os.path.abspath(absr(p))) > a.limit_path]
for p in too_long: drift.append(p + " exceeds the path limit")

texts, out_edits = {}, {}
for e in edits:
    if e["path"] not in texts:
        if not os.path.isfile(absr(e["path"])): drift.append(e["path"] + " (file missing)"); texts[e["path"]] = None; continue
        texts[e["path"]] = rd(e["path"])
    text = texts[e["path"]]
    if text is None: continue
    old, new, counts = e["old"], e["new"], e["counts"]
    n_old, n_new = text.count(old), (text.count(new) if new else 0)
    inserts = bool(new) and old in new
    if a.undo:
        undoable = (n_new in counts and n_new > 0) if inserts else (n_old == 0 and n_new > 0 and n_new in counts)
        untouched = (n_new == 0 and n_old in counts and n_old > 0) if inserts else (n_old in counts and n_old > 0)
        if undoable and new != "": text = text.replace(new, old); results.append(("undo", e["id"], e["path"])); texts[e["path"]] = text; out_edits[e["path"]] = text
        elif undoable and new == "": drift.append(f'{e["path"]} [{e["id"]}] cannot undo a deletion by anchors (use the backup)')
        elif untouched: results.append(("not-applied", e["id"], e["path"]))
        else: drift.append(f'{e["path"]} [{e["id"]}] cannot undo: old={n_old} new={n_new}')
        continue
    pending = (n_new == 0 and n_old in counts) if inserts else (n_old in counts and n_old > 0)
    applied = (n_new in counts and n_new > 0 and n_new >= n_old) if inserts else (n_old == 0 and (n_new > 0 or new == ""))
    if pending: text = text.replace(old, new); results.append(("edit", e["id"], e["path"])); texts[e["path"]] = text; out_edits[e["path"]] = text
    elif applied: results.append(("already", e["id"], e["path"]))
    else: drift.append(f'{e["path"]} [{e["id"]}] anchor counts old={n_old} new={n_new}, expected {counts}: {old[:70]!r}')

for w in writes:
    new = open(os.path.join(HERE, "staged", "new", w["from"]), encoding="utf-8", newline="").read()
    if not os.path.isfile(absr(w["path"])):
        moved = [m for m in plan.get("moves", []) if m["src"] == w["path"] and os.path.isfile(absr(m["dst"])) and sha(open(absr(m["dst"]), "rb").read()) == m.get("sha256After")]
        if moved: results.append(("already", w["id"], w["path"]))
        else: drift.append(w["path"] + " (file missing)")
        continue
    cur = rd(w["path"])
    if a.undo: drift.append(w["path"] + " [" + w["id"] + "] whole-file write: restore from the backup (C/🗑️generated/ide-free/backup)"); continue
    if cur == new: results.append(("already", w["id"], w["path"]))
    elif sha(cur.encode("utf-8")) == w["sha256"]:
        if w["path"] in out_edits: drift.append(w["path"] + " is both edited and rewritten")
        out_edits[w["path"]] = new; results.append(("write", w["id"], w["path"]))
    else: drift.append(w["path"] + " [" + w["id"] + "] changed since staging (sha256 differs)")

new_files = {}
for c in creates:
    content = c["content"] if "content" in c else open(os.path.join(HERE, "staged", "new", c["from"]), encoding="utf-8", newline="").read()
    present = rd(c["path"]) if os.path.isfile(absr(c["path"])) else None
    if a.undo:
        if present == content: new_files[c["path"]] = None; results.append(("remove", c["id"], c["path"]))
        elif present is None: results.append(("not-applied", c["id"], c["path"]))
        else: drift.append(c["path"] + " differs from our file, not removing")
    elif present is None: new_files[c["path"]] = content; results.append(("create", c["id"], c["path"]))
    elif present == content: results.append(("already", c["id"], c["path"]))
    else: drift.append(c["path"] + " exists and differs from the staged file")

removals, renames = {}, []
for d in deletes:
    if a.undo: results.append(("backup-only", d["id"], d["path"])); continue
    if os.path.isfile(absr(d["path"])):
        if sha(open(absr(d["path"]), "rb").read()) == d["sha256"]: removals[d["path"]] = True; results.append(("delete", d["id"], d["path"]))
        else: drift.append(d["path"] + " changed since staging (sha256 differs), not deleting")
    else: results.append(("already", d["id"], d["path"]))
for m in moves:
    s, t = absr(m["src"]), absr(m["dst"])
    sh = lambda p: sha(open(p, "rb").read())
    if a.undo:
        if os.path.isfile(t) and sh(t) in (m["sha256"], m.get("sha256After", m["sha256"])) and not os.path.exists(s): renames.append((m["dst"], m["src"])); results.append(("unmove", m["id"], m["src"]))
        elif os.path.isfile(s) and not os.path.exists(t): results.append(("not-applied", m["id"], m["src"]))
        else: drift.append(m["src"] + " cannot unmove")
        continue
    if os.path.isfile(s) and not os.path.exists(t):
        if sh(s) == m["sha256"]: renames.append((m["src"], m["dst"])); results.append(("move", m["id"], m["src"]))
        else: drift.append(m["src"] + " changed since staging (sha256 differs)")
    elif os.path.isfile(s) and os.path.isfile(t) and sh(s) == sh(t) == m["sha256"]: renames.append((m["src"], None)); results.append(("move-duplicate", m["id"], m["src"]))
    elif not os.path.exists(s) and os.path.isfile(t) and sh(t) in (m["sha256"], m.get("sha256After", m["sha256"])): results.append(("already", m["id"], m["src"]))
    else: drift.append(f'{m["src"]} -> {m["dst"]}: source/destination state not recognised')
dst_seen = {}
for s, t in renames:
    if t:
        if t in dst_seen: drift.append(t + " is the destination of two moves")
        dst_seen[t] = s

for rel, text in out_edits.items():
    if rel.endswith(".json"):
        try:
            doc = json.loads(text)
            ws = doc.get("workspaces") if rel.endswith("package.json") else None
            if isinstance(ws, list) and (ws != sorted(ws) or len(ws) != len(set(ws))): drift.append(rel + ": workspaces would be unsorted or duplicated")
        except ValueError as err:
            try: json.loads(rd(rel) if False else open(absr(rel), encoding="utf-8").read())
            except ValueError: continue
            drift.append(rel + ": result is not valid JSON: " + str(err))
for rel, content in new_files.items():
    if content is not None and rel.endswith(".json"):
        try: json.loads(content)
        except ValueError as err: drift.append(rel + ": staged file is not valid JSON: " + str(err))

if drift:
    print("DRIFT, nothing written:"); [print("  " + d) for d in drift[:60]]
    if len(drift) > 60: print(f"  ... and {len(drift) - 60} more")
    sys.exit(2)
tally = {}
for k, _, _ in results: tally[k] = tally.get(k, 0) + 1
if a.verbose: [print(f"  {k:12} {i:36} {p[-100:]}") for k, i, p in results]
if a.write:
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    gen = os.path.join(C, "🗑️generated", "ide-free"); backup = os.path.join(gen, "backup", stamp); os.makedirs(gen, exist_ok=True)
    originals = {rel: open(absr(rel), "rb").read() for rel in list(out_edits) + list(removals) if os.path.isfile(absr(rel))}
    for rel, data in originals.items():
        dst = os.path.join(backup, rel); os.makedirs(os.path.dirname(dst), exist_ok=True); open(dst, "wb").write(data)
    done = []
    try:
        for rel, text in out_edits.items():
            full = absr(rel); tmp = full + ".ide-free.tmp"
            open(tmp, "w", encoding="utf-8", newline="").write(text); os.replace(tmp, full); done.append(("edit", rel))
        for rel, content in new_files.items():
            full = absr(rel)
            if content is None: os.remove(full); done.append(("removed", rel))
            else: os.makedirs(os.path.dirname(full), exist_ok=True); open(full, "w", encoding="utf-8", newline="").write(content); done.append(("created", rel))
        for rel in removals: os.remove(absr(rel)); done.append(("deleted", rel))
        for s, t in renames:
            if t is None: os.remove(absr(s)); done.append(("deleted-duplicate", s)); continue
            os.makedirs(os.path.dirname(absr(t)), exist_ok=True); os.replace(absr(s), absr(t)); done.append(("moved", (s, t)))
    except Exception as err:
        for kind, rel in reversed(done):
            if kind == "edit" or kind == "deleted": open(absr(rel), "wb").write(originals[rel])
            elif kind == "created": os.remove(absr(rel))
            elif kind == "moved": os.replace(absr(rel[1]), absr(rel[0]))
            elif kind == "removed": pass
        sys.exit("ROLLED BACK: " + repr(err))
    for r in rmdirs:
        p = absr(r["path"])
        try:
            if os.path.isdir(p):
                for name in sorted(os.listdir(p)):
                    if name == ".DS_Store": os.remove(os.path.join(p, name))
                os.rmdir(p)
        except OSError: pass
    with open(os.path.join(gen, "ledger.jsonl"), "a", encoding="utf-8") as f:
        f.write(json.dumps({"at": stamp, "undo": a.undo, "groups": groups, "counts": tally, "backup": backup}, ensure_ascii=False) + "\n")
print(("UNDONE " if a.undo and a.write else "UNDO DRY RUN " if a.undo else "APPLIED " if a.write else "DRY RUN ") + json.dumps({"groups": groups, **tally}, ensure_ascii=False))
