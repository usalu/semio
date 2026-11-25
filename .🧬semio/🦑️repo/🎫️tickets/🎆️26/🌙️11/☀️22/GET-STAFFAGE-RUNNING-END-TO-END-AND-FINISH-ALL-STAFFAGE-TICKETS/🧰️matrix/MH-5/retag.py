"""🏷️ Follows a catalog-wide change of `asset.tags` that another worker made (2026-11-25 19:09: slugs became `{ id, title }`): retag.py <gen> <catalog humans dir> <id>… checks that the catalog file differs from the copy this batch landed last in `asset.tags` only, then gives the backup the new tags (same serialisation as the backup had) and takes the catalog file as the landed copy. Anything else that differs is reported and nothing is written for that asset."""
import json, sys, shutil
gen, cat = sys.argv[1], sys.argv[2]
for a in sys.argv[3:]:
    f = f"{cat}/{a}/🔣️.json"
    now = json.load(open(f)); old = json.load(open(f"{gen}/landed/{a}.json"))
    if now == old:
        print(a, "unchanged"); continue
    tags = now["asset"]["tags"]; probe = json.loads(json.dumps(old)); probe["asset"]["tags"] = tags
    if probe != now:
        print(a, "REFUSED: differs in more than asset.tags"); continue
    raw = open(f"{gen}/backup/{a}.json").read(); back = json.loads(raw); back["asset"]["tags"] = tags
    text = open(f).read(); indent = len(text.split("\n")[1]) - len(text.split("\n")[1].lstrip()) if "\n" in text else None
    out = json.dumps(back, ensure_ascii=False, indent=indent) if indent else json.dumps(back, ensure_ascii=False, separators=(",", ":"))
    open(f"{gen}/backup/{a}.json", "w").write(out + ("\n" if raw.endswith("\n") else ""))
    shutil.copyfile(f, f"{gen}/landed/{a}.json")
    print(a, "tags followed:", len(tags))
