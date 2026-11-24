"""Independent check of the en1998 scene golden `full_showcase.txt` against the showcase document: every entity id and count is derived from the JSON rows, never from the projection."""
import json, re, sys, collections

ANY = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
doc = json.load(open(f"{ANY}/🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"))
golden = [line.split("|") for line in open(f"{ANY}/🎬️scene/🧪️tests/🧫️fixtures/full_showcase.txt").read().splitlines() if line]
entities = [g for g in golden if g[0] != "annotation"]
ids = [g[0] for g in entities]
problems = []

if len(ids) != len(set(ids)):
    problems.append(f"duplicate entity ids: {[i for i, n in collections.Counter(ids).items() if n > 1]}")

collections_with_rows = ["buildings", "bridges", "assessments", "silos", "tanks", "foundations", "retainingWalls", "towers", "joints", "bearings", "piers", "accelerograms", "diaphragms", "soilLayers", "placements", "tieBeams", "slopes", "isolationSystems", "pierBases", "criticalRegions"]
rows = {c: [r["id"] for r in doc[c]] for c in collections_with_rows}
all_rows = {i for v in rows.values() for i in v}
bases = {i.split(":")[0] for i in ids}

phantom = sorted(b for b in bases if b not in all_rows and b != "project-site")
if phantom:
    problems.append(f"entities whose base id is no document row: {phantom}")
undrawn = sorted(r for r in all_rows if r not in bases and r not in {p["id"] for p in doc["placements"]} and r not in {l["id"] for l in doc["soilLayers"]} and r not in {a["id"] for a in doc["accelerograms"]})
if undrawn:
    problems.append(f"document rows without an entity: {undrawn}")

for b in doc["buildings"]:
    for s in b["storeys"]:
        slab = f"{b['id']}:{s['id']}"
        if slab not in ids:
            problems.append(f"missing slab {slab}")
        for sy in b["systems"]:
            for tag in (f"drift-{sy['id']}", f"force-{sy['id']}"):
                if f"{slab}:{tag}" not in ids:
                    problems.append(f"missing {slab}:{tag}")
    storeys = len(b["storeys"])
    drawn = [i for i in ids if i.startswith(f"{b['id']}:") and re.fullmatch(r"[^:]+:[^:]+:drift-[^:]+", i) and not i.endswith(":drift-map")]
    if b["systems"] and len(drawn) != storeys * len(b["systems"]):
        problems.append(f"{b['id']}: {len(drawn)} drift bars for {storeys} storeys x {len(b['systems'])} systems")

for a in doc["assessments"]:
    if a["id"] not in ids:
        problems.append(f"missing assessment row entity {a['id']}")
for r in doc["criticalRegions"]:
    if f"{r['id']}:region" not in ids or not any(i.startswith(f"{r['id']}:hoop:") for i in ids):
        problems.append(f"region {r['id']} lacks its member glyph or hoops")
for i in doc["isolationSystems"]:
    for tag in ("", ":ghost-plus", ":ghost-minus"):
        if f"{i['id']}{tag}" not in ids:
            problems.append(f"missing {i['id']}{tag}")
for p in doc["pierBases"]:
    for tag in ("", ":footing"):
        if f"{p['id']}{tag}" not in ids:
            problems.append(f"missing {p['id']}{tag}")

print(f"{len(ids)} entities, {len(all_rows)} document rows, {len(problems)} problems")
for p in problems:
    print(" -", p)
sys.exit(1 if problems else 0)
