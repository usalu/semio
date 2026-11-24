"""📊 Summarises the JSON lines of catalog-scan.ts: assets per finding code and category, rest against posed, the construction of the shapes behind the bone findings, the anchor classes, the missing cues and the assets without any hard finding. usage: python3 scan-summary.py <scan.jsonl>"""
import collections
import json
import sys

rows = [json.loads(line) for line in open(sys.argv[1], encoding="utf-8") if line.strip()]
HARD = lambda code: not code.startswith("group-drift")
by_category = collections.Counter(row["category"] for row in rows)
print("assets scanned:", len(rows), dict(sorted(by_category.items())))
print("seconds: total %.0f, mean %.1f, max %.1f" % (sum(r["ms"] for r in rows) / 1000, sum(r["ms"] for r in rows) / 1000 / max(1, len(rows)), max(r["ms"] for r in rows) / 1000))
errors = [(r["id"], r["errors"][:2]) for r in rows if r["errors"]]
print("assets with evaluation or resolve errors:", len(errors), errors[:10])

def table(title, counter):
    print("\n## " + title)
    for key, value in sorted(counter.items(), key=lambda item: (-item[1], item[0])):
        print(f"{value}\t{key}")

code_assets = collections.defaultdict(set)
code_assets_rest = collections.defaultdict(set)
code_category = collections.Counter()
findings = collections.Counter()
causes = {}
for row in rows:
    mine = set()
    for scale, counts in row["geometry"].items():
        for code, n in counts.items():
            findings[code] += n
            if HARD(code):
                code_assets[code].add(row["id"])
                mine.add(code)
    for code in (row.get("rest") or {}):
        if HARD(code):
            code_assets_rest[code].add(row["id"])
    copies = {(f["scale"], f["view"]) for f in row["derived"] if f["against"].startswith("1:")}
    inband = {(f["scale"], f["view"]) for f in row["derived"] if not f["against"].startswith("1:")}
    if copies:
        code_assets["derived-view-band-copy"].add(row["id"]); mine.add("derived-view-band-copy")
    if inband:
        code_assets["derived-view-in-band"].add(row["id"]); mine.add("derived-view-in-band")
    row["copies"], row["inband"] = copies, inband
    cue = {(c["mode"]) for c in row["cue"] if "unavailable" not in c}
    for mode in cue:
        code_assets[f"analysis-cue ({mode}, default pose)"].add(row["id"]); mine.add(f"analysis-cue ({mode}, default pose)")
    unsuitable = {c["scale"] for c in row["cue"] if "unavailable" in c}
    if unsuitable:
        code_assets["preset scale unavailable in the resolver (" + ",".join(sorted({c["unavailable"] for c in row["cue"] if "unavailable" in c})) + ")"].add(row["id"])
    for code in mine:
        code_category[(code, row["category"])] += 1
    causes[row["id"]] = mine

print("\n## assets with a hard finding, per code (all poses of every authored scale; cue at the default pose only)")
print("code\tassets\tof them already at the rest pose\t" + "\t".join(sorted(by_category)))
for code, assets in sorted(code_assets.items(), key=lambda item: -len(item[1])):
    print(f"{code}\t{len(assets)}\t{len(code_assets_rest.get(code, ())) if code in code_assets_rest or code.startswith(('sync', 'anchor', 'dimension')) else '-'}\t" + "\t".join(str(code_category[(code, category)]) for category in sorted(by_category)))

families = {"sync": lambda c: c.startswith("sync-"), "anchor": lambda c: c == "anchor-landmark", "copy": lambda c: c.startswith("derived-view"), "cue": lambda c: c.startswith("analysis-cue"), "dimension": lambda c: c.startswith("dimension-") or c.endswith("-empty") or c == "evaluate-error"}
print("\n## assets per family and category")
print("family\tassets\t" + "\t".join(sorted(by_category)))
for name, test in families.items():
    hit = [row for row in rows if any(test(code) for code in causes[row["id"]])]
    per = collections.Counter(row["category"] for row in hit)
    print(f"{name}\t{len(hit)}\t" + "\t".join(str(per[category]) for category in sorted(by_category)))
clean = [row for row in rows if not causes[row["id"]]]
per = collections.Counter(row["category"] for row in clean)
print(f"no hard finding\t{len(clean)}\t" + "\t".join(str(per[category]) for category in sorted(by_category)))
combos = collections.Counter(" + ".join(sorted(name for name, test in families.items() if any(test(code) for code in causes[row["id"]]))) or "none" for row in rows)
table("assets per combination of families", combos)

sync = collections.Counter()
sync_assets = collections.defaultdict(set)
for row in rows:
    for key, n in (row.get("syncClasses") or {}).items():
        check, pose, kind, twin = [part.strip() for part in key.split("|")]
        if check.startswith("group-drift"):
            continue
        label = f"{pose} | {kind} | {twin}"
        sync[label] += n
        sync_assets[label].add(row["id"])
print("\n## per-bone sync findings (hard tier) by pose, construction of the front or plan shape and shape id: findings, assets")
for label, n in sorted(sync.items(), key=lambda item: -item[1]):
    print(f"{n}\t{len(sync_assets[label])}\t{label}")
only_other = [row["id"] for row in rows if (row.get("syncClasses") or {}) and any(not k.startswith("group") for k in row["syncClasses"]) and all(k.startswith("group") or k.endswith("other shape id") for k in row["syncClasses"])]
print("assets whose per-bone findings all compare shapes of different ids:", len(only_other), only_other[:12])
bone_assets = {row["id"] for row in rows if any(code.startswith("sync-bone") for code in causes[row["id"]])}
extent_only = [row["id"] for row in rows if any(code in ("sync-forward", "sync-height", "sync-lateral") for code in causes[row["id"]]) and row["id"] not in bone_assets]
print("assets with an extent sync finding and no per-bone finding:", len(extent_only), extent_only[:12])

anchors = collections.Counter()
anchor_assets = collections.defaultdict(set)
for row in rows:
    for key, n in (row.get("anchorClasses") or {}).items():
        kind, view, cls = [part.strip() for part in key.split("|")]
        anchors[f"{kind} | {cls}"] += n
        anchor_assets[f"{kind} | {cls}"].add(row["id"])
print("\n## anchor-landmark findings by anchor kind and where the anchor lies: findings, assets")
for label, n in sorted(anchors.items(), key=lambda item: -item[1]):
    print(f"{n}\t{len(anchor_assets[label])}\t{label}")
with_anchor = [row for row in rows if row.get("anchorClasses")]
only_descendant = [row["id"] for row in with_anchor if all(key.endswith("on a descendant bone") for key in row["anchorClasses"])]
only_bound = [row["id"] for row in with_anchor if all(not key.endswith("off every drawn shape") for key in row["anchorClasses"])]
print("assets with anchor findings:", len(with_anchor), "| all on a descendant bone:", len(only_descendant), "| all on some drawn shape (descendant or other bone):", len(only_bound))
anchor_and_sync = [row["id"] for row in with_anchor if any(code.startswith("sync") for code in causes[row["id"]])]
print("assets with anchor findings that also have a sync finding:", len(anchor_and_sync))

entries = sum(len(row["copies"]) for row in rows)
print("\n## derived views")
print("band-copy: assets", len(code_assets["derived-view-band-copy"]), "scale-view entries", entries, "-> preset cells", entries * 7)
print("in-band: assets", len(code_assets["derived-view-in-band"]), "scale-view entries", sum(len(row["inband"]) for row in rows))
for row in rows:
    if row["inband"]:
        print("  in-band", row["category"], row["id"], sorted({(f["scale"], f["view"], f["against"], f["transform"], f["distance"]) for f in row["derived"] if not f["against"].startswith("1:")})[:4])
print("band-copy assets:", ", ".join(f"{row['id']} ({len(row['copies'])})" for row in rows if row["copies"]))
table("findings (not cells) per code", findings)
for name, test in families.items():
    print(f"\n{name}: " + ", ".join(sorted(row["id"] for row in rows if any(test(code) for code in causes[row["id"]]))))
