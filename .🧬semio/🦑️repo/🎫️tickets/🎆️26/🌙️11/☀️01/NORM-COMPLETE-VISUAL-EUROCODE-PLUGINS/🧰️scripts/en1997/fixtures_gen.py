import sys, json, copy, os, glob
sys.path.insert(0, ".")
from spec import *
from rows import *
import io, contextlib
with contextlib.redirect_stdout(io.StringIO()):
    from dsl_assets import DEMO
FX = S + "/🧪️tests/🧫️fixtures/🧬️mutations"

def jrow(e, row):
    d = {"id": row["id"]}
    for f in e.fields:
        v = row[f.name]
        d[f.camel] = int(v) if f.kind == "u" else (str(v) if f.kind == "s" else float(v))
    if e.nested:
        n = e.nested
        d[camel(n[0])] = [jrow(n[2], x) for x in row[n[0]]]
    return d

NEWKEYS = {e.camel_coll: [jrow(e, r) for r in DEMO[e.coll]] for e in ENTITIES}

def dump(path, obj):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(json.dumps(obj, ensure_ascii=False, indent=2) + "\n")

# 1. existing snapshot fixtures and diffs
for p in glob.glob(FX + "/**/*.json", recursive=True):
    if "📸️snapshot" in p:
        d = json.load(open(p, encoding="utf8"))
        if "anchors" in d: continue
        for k, v in NEWKEYS.items(): d[k] = copy.deepcopy(v)
        dump(p, d)
    elif "/🔺️diff/" in p:
        d = json.load(open(p, encoding="utf8"))
        if "anchors" in d: continue
        for e in ENTITIES: d[e.camel_coll] = None
        dump(p, d)

BASE = json.load(open(FX + "/📥insert-pile/✅apply/🔣️.before.json", encoding="utf8"))
for k in NEWKEYS: assert k in BASE
# the insert-pile before has one pile; base for other kinds is the same document
ENT = {e.struct: e for e in ENTITIES}
EXIST = {
    "SoilLayer": ("layers", None), "SpreadFoundation": ("footings", None), "Pile": ("piles", None),
    "RetainingWall": ("retainingWalls", None), "Slope": ("slopes", None), "UpliftCase": ("upliftCases", None),
}
def coll_key(r): return r.camel_coll
def row0(r): return copy.deepcopy(BASE[coll_key(r)][0])

def modify(r, row):
    row = copy.deepcopy(row)
    st = r.struct
    if st == "SoilLayer": row["gamma"] = 19500.0
    elif st == "SpreadFoundation": row["loadCases"][0]["verticalPermanent"] = 450000.0
    elif st == "Pile": row["testProfiles"][0]["shaftResistance"] = 1700000.0
    elif st == "RetainingWall": row["surcharge"] = 15000.0
    elif st == "Slope": row["length"] = 16.0
    elif st == "UpliftCase": row["porePressure"] = 45000.0
    elif st == "Anchor": row["tests"][0]["pullOutResistance"] = 700000.0
    elif st == "EmbeddedWall": row["embedment"] = 6.5
    elif st == "PileGroup": row["spacing"] = 2.0
    elif st == "LabTest": row["points"][0]["shearStress"] = 32000.0
    elif st == "LayerWater": row["headDifference"] = 1.5
    return row

def rows_delta(removed=(), inserted=()):
    return {"removed": [{"id": i, "index": n} for i, n in removed], "inserted": [{"index": n, "row": row} for n, row in inserted], "moved": [], "modified": []}

def diff_doc(r, delta):
    d = {"structureId": None, "geotechnicalCategory": None, "designSituation": None, "designApproach": None, "annex": None, "groundwaterLevel": None, "investigationDepth": None,
         "layers": None, "footings": None, "piles": None, "retainingWalls": None, "slopes": None, "upliftCases": None}
    for e in ENTITIES: d[e.camel_coll] = None
    d[r.camel_coll] = delta
    return d

def lit(r, v):
    st = struct(r, v)
    return st[0].lower() + st[1:]

for r, v in newkinds():
    base = copy.deepcopy(BASE)
    ck = coll_key(r)
    d = f"{FX}/{dirname(r, v)}"
    item = camel(r.item)
    if v == "insert":
        new = row0(r); new["id"] = new["id"] + "-2"
        after = copy.deepcopy(base); after[ck].append(new)
        mut = {"mutation": lit(r, v), "index": len(base[ck]), item: new}
        dump(d + "/✅apply/🔣️.before.json", base)
        dump(d + "/✅apply/🔣️.after.json", after)
        dump(d + "/✅apply/🔣️.mutation.json", mut)
        dump(d + "/✅apply/🔣️.diff.json", diff_doc(r, rows_delta(inserted=[(len(base[ck]), new)])))
        dump(d + "/✅apply/🔣️.outcome.json", {"status": "applied"})
        dump(d + "/⛔dupe/🔣️.before.json", after)
        dump(d + "/⛔dupe/🔣️.after.json", after)
        dump(d + "/⛔dupe/🔣️.mutation.json", mut)
        dump(d + "/⛔dupe/🔣️.outcome.json", {"status": "rejected", "code": "mutation.duplicate-id", "messages": [{"level": "fatal", "code": "mutation.duplicate-id"}]})
        os.makedirs(d + "/⛔dupe/🔺️diff", exist_ok=True)
        open(d + "/⛔dupe/🔺️diff/🚫️.absent", "w").close()
    elif v == "remove":
        gone = base[ck][0]
        after = copy.deepcopy(base); after[ck] = after[ck][1:]
        dump(d + "/✅apply/🔣️.before.json", base)
        dump(d + "/✅apply/🔣️.after.json", after)
        dump(d + "/✅apply/🔣️.mutation.json", {"mutation": lit(r, v), "index": 0})
        dump(d + "/✅apply/🔣️.diff.json", diff_doc(r, rows_delta(removed=[(gone["id"], 0)])))
        dump(d + "/✅apply/🔣️.outcome.json", {"status": "applied"})
    else:
        old = base[ck][0]
        new = modify(r, old)
        after = copy.deepcopy(base); after[ck][0] = new
        dump(d + "/✅apply/🔣️.before.json", base)
        dump(d + "/✅apply/🔣️.after.json", after)
        dump(d + "/✅apply/🔣️.mutation.json", {"mutation": lit(r, v), item: new})
        dump(d + "/✅apply/🔣️.diff.json", diff_doc(r, rows_delta(removed=[(old["id"], 0)], inserted=[(0, new)])))
        dump(d + "/✅apply/🔣️.outcome.json", {"status": "applied"})
print("fixtures ok")
