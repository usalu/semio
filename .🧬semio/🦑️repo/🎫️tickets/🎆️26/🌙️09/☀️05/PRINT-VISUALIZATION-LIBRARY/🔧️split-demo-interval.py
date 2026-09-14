# 🧹 Two finished agents both named a demo table `demo-interval`: CHARTS-A's lane/start/end table
# for the timeline and narrative families, CHARTS-B's label/estimate/lower/upper/weight table for
# the table and uncertainty families. Whichever package loads first won, and the other family read
# columns that were not there. The lane table becomes `demo-lane`; the interval estimate keeps the
# name that describes it.
import io, json, os

PRINT = u"C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint"
LATEX = os.path.join(PRINT, u"\U0001f58b\ufe0flatex")
CATALOG = os.path.join(PRINT, u"\U0001f5bc\ufe0fassets", u"\U0001f523\ufe0fviz-catalog.json")
SCHEMA = os.path.join(PRINT, u"\U0001f9ec\ufe0fschema", u"\U0001f523\ufe0f.json")
LANE_FAMILIES = {"timeline", "narrative"}


def read(path):
    return io.open(path, encoding="utf-8").read()


def write(path, text):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(text)
    os.replace(tmp, path)


renamed = 0
for name in ("semio-viz-charts-bar.sty", "semio-viz-charts-timeline.sty"):
    path = os.path.join(LATEX, name)
    src = read(path)
    renamed += src.count("demo-interval")
    write(path, src.replace("demo-interval", "demo-lane"))

catalog = json.load(io.open(CATALOG, encoding="utf-8"))
moved = 0
for kind in catalog["kinds"]:
    if kind["data"] == "demo-interval" and kind["family"] in LANE_FAMILIES:
        kind["data"] = "demo-lane"
        moved += 1
tmp = os.path.join(os.path.dirname(CATALOG), "catalog.tmp.json")
io.open(tmp, "w", encoding="utf-8", newline="\n").write(json.dumps(catalog, ensure_ascii=False, indent=2) + "\n")
os.replace(tmp, CATALOG)

schema = json.load(io.open(SCHEMA, encoding="utf-8"))
tables = schema["x-semio-demo-tables"]
existing = next(entry for entry in tables if entry["name"] == "demo-interval")
if not any(entry["name"] == "demo-lane" for entry in tables):
    lane = dict(existing)
    lane["name"] = "demo-lane"
    lane["columns"] = ["lane", "label", "start", "end", "state"]
    for key in ("description", "title"):
        if key in lane and isinstance(lane[key], dict):
            lane[key] = {
                "en": "Timeline lanes: one labelled interval per row, with its lane, its start and end and its state.",
                "de": "Zeitstrahl-Spuren: je Zeile ein beschriftetes Intervall mit Spur, Anfang, Ende und Zustand.",
            }
    tables.append(lane)
    tables.sort(key=lambda entry: entry["name"])
tmp = os.path.join(os.path.dirname(SCHEMA), "schema.tmp.json")
io.open(tmp, "w", encoding="utf-8", newline="\n").write(json.dumps(schema, ensure_ascii=False, indent=2) + "\n")
os.replace(tmp, SCHEMA)

print("renamed", renamed, "LaTeX occurrences;", moved, "catalogue entries moved to demo-lane")
print("demo-interval entry shape:", json.dumps(existing, ensure_ascii=False)[:300])
