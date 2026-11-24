"""🗒️ Writes the per-asset appendix of the diagnosis from the scan: one table row per asset with a hard finding. usage: python3 appendix.py <scan.jsonl>"""
import collections
import json
import sys

rows = sorted((json.loads(line) for line in open(sys.argv[1]) if line.strip()), key=lambda row: (row["category"], row["id"]))
print("| Category | Asset | Sync (rest / posed findings; shapes) | Anchors (findings; where) | Band copy | In-band | Cue |")
print("|---|---|---|---|---|---|---|")
for row in rows:
    sync = collections.Counter()
    kinds = set()
    other = 0
    for key, n in (row.get("syncClasses") or {}).items():
        check, pose, kind, twin = [part.strip() for part in key.split("|")]
        if check.startswith("group"):
            continue
        sync[pose] += n
        kinds.add(kind)
        other += n if twin == "other shape id" else 0
    extent = sum(n for counts in row["geometry"].values() for code, n in counts.items() if code in ("sync-forward", "sync-height", "sync-lateral"))
    where = collections.Counter()
    for key, n in (row.get("anchorClasses") or {}).items():
        where[key.split("|")[2].strip().replace("on a descendant bone", "descendant").replace("on another bone", "other bone").replace("off every drawn shape", "off")] += n
    copies = sorted({f"{f['scale']} {f['view']}" for f in row["derived"] if f["against"].startswith("1:")})
    inband = sorted({f"{f['scale']} {f['view']}" for f in row["derived"] if not f["against"].startswith("1:")})
    cue = sorted({c["scale"] for c in row["cue"] if "unavailable" not in c})
    if not (sync or extent or where or copies or inband or cue):
        continue
    sync_text = "" if not (sync or extent) else f"{sync['rest']} / {sync['posed']}; {'+'.join(sorted(kinds)) or 'extent only'}" + (f"; {other} other id" if other else "") + (f"; {extent} extent" if extent else "")
    anchor_text = "" if not where else f"{sum(where.values())}; " + ", ".join(f"{k} {v}" for k, v in where.most_common())
    print(f"| {row['category']} | {row['id']} | {sync_text} | {anchor_text} | {', '.join(copies)} | {', '.join(inband)} | {', '.join(cue)} |")
