"""Merges the hand plan, the relative-import plan and generated full-file writes into one plan; usage: merge-plan.py hand.json rel.json out.json [--root LIVE] [path=generated-file ...]"""
import hashlib, json, os, sys
args = sys.argv[1:]
hand, rel, out = json.load(open(args[0], encoding="utf-8")), json.load(open(args[1], encoding="utf-8")), args[2]
rest = args[3:]
root = os.getcwd()
if rest[:1] == ["--root"]: root, rest = rest[1], rest[2:]
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
taken = {row["path"] for row in hand["writes"]} | {row["path"] for row in hand["edits"]}
clash = [row["path"] for row in rel["files"] if row["path"] in taken]
if clash or rel["problems"]: sys.exit(f"clash {clash} problems {rel['problems']}")
writes = list(hand["writes"])
for item in rest:
    path, source = item.split("=", 1)
    if path in taken: sys.exit(f"clash {path}")
    content = open(source, encoding="utf-8", newline="").read()
    live = os.path.join(root, path)
    before = open(live, encoding="utf-8", newline="").read() if os.path.exists(live) else None
    writes.append({"path": path, "sha256": None if before is None else sha(before), "afterSha256": sha(content), "content": content})
edits = sorted(hand["edits"] + [{"path": row["path"], "sha256": row["sha256"], "afterSha256": row["afterSha256"], "replacements": row["replacements"]} for row in rel["files"]], key=lambda row: row["path"])
json.dump({"version": 1, "writes": sorted(writes, key=lambda row: row["path"]), "edits": edits}, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({"writes": len(writes), "edits": len(edits), "relativeFiles": len(rel["files"])}))
