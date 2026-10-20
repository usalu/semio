"""Turns the edits made in an overlay into an anchored apply plan for apply.py: for every file that differs from the hash recorded at overlay creation, minimal unique-context replacements (so the plan still applies when a peer edited elsewhere in the file: apply.py --allow-anchored), for every new file a full write. Usage: diff-plan.py <overlay> <base.json> <live-root> <out.json> [--exclude substring ...]"""
import difflib, hashlib, json, os, sys
overlay, base_path, live, out = sys.argv[1:5]
exclude = ["node_modules", "/.cache/", "🤖️generated", "tsconfig.check.json", ".storybook/"] + (sys.argv[6:] if len(sys.argv) > 5 and sys.argv[5] == "--exclude" else [])
base = json.load(open(base_path, encoding="utf-8"))
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
digest = lambda path: hashlib.sha256(open(path, "rb").read()).hexdigest()
read = lambda path: open(path, encoding="utf-8", newline="").read()
writes, edits = [], []
for directory, names, files in os.walk(overlay):
    names[:] = [name for name in names if name not in ("node_modules", ".cache")]
    for file in sorted(files):
        path = os.path.join(directory, file)
        rel = os.path.relpath(path, overlay)
        if any(part in "/" + rel for part in exclude): continue
        if rel not in base:
            writes.append({"path": rel, "sha256": None, "afterSha256": sha(read(path)), "content": read(path)})
            continue
        if digest(path) == base[rel]: continue
        before_text, after_text = read(live and os.path.join(live, rel)) if False else None, read(path)
        live_text = read(os.path.join(live, rel))
        if live_text == after_text: continue
        if hashlib.sha256(live_text.encode("utf-8")).hexdigest() != base[rel]:
            sys.exit(f"live file changed since the overlay was copied: {rel}; rebuild the overlay edits on the new tree")
        old_lines, new_lines = live_text.splitlines(keepends=True), after_text.splitlines(keepends=True)
        working, delta, rows = list(old_lines), 0, []
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, old_lines, new_lines, autojunk=False).get_opcodes():
            if tag == "equal": continue
            start, end = i1 + delta, i2 + delta
            text = "".join(working)
            context = 0 if i2 > i1 else 1
            while True:
                lo, hi = max(0, start - context), min(len(working), end + context)
                anchor = "".join(working[lo:hi])
                if anchor and text.count(anchor) == 1: break
                if lo == 0 and hi == len(working): sys.exit(f"no unique anchor for {rel}")
                context += 1
            replacement = "".join(working[lo:start]) + "".join(new_lines[j1:j2]) + "".join(working[end:hi])
            rows.append({"old": anchor, "new": replacement, "count": 1})
            working[start:end] = new_lines[j1:j2]
            delta += (j2 - j1) - (i2 - i1)
        assert "".join(working) == after_text, rel
        edits.append({"path": rel, "sha256": base[rel], "afterSha256": sha(after_text), "replacements": rows})
json.dump({"version": 1, "writes": sorted(writes, key=lambda row: row["path"]), "edits": sorted(edits, key=lambda row: row["path"])}, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({"writes": len(writes), "edits": len(edits), "replacements": sum(len(row["replacements"]) for row in edits)}))
