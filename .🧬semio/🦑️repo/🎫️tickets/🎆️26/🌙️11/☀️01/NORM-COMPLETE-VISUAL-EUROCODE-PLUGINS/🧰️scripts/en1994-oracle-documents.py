"""Committed oracle documents of the EN 1994 `check-outcome` case: python -I en1994-oracle-documents.py [--write]

Each document is the span-m after-snapshot of the mutation fixtures with a few hand-chosen leaves, so every row stays a plain committed snapshot. Without --write the independent oracle prints the utilisations of the interesting rows for tuning; with --write the documents land in `🧪️tests/🧫️fixtures/📄️oracle-documents/<name>/🔣️.json`.
"""
import copy, importlib.util, json, pathlib, sys

SUBSET = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any")
BASE = SUBSET / "🧪️tests/🧫️fixtures/🧬️mutations/📏️change-beam-span-m/✅apply/🔣️.after.json"
OUT = SUBSET / "🧪️tests/🧫️fixtures/📄️oracle-documents"
spec = importlib.util.spec_from_file_location("oracle", SUBSET / "🔮️oracles/🚦️check-outcome/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
base = json.loads(BASE.read_text(encoding="utf8"))


def variant(edit):
    doc = copy.deepcopy(base)
    edit(doc)
    return doc


def beam(doc, **kw):
    doc["beams"][0].update(kw)


def action(doc, coll, aid, **kw):
    next(a for a in doc[coll][0]["actions"] if a["id"] == aid).update(kw)


def encased_leaves(doc):
    c = doc["columns"][0]
    c.update(kind="encased", outerSizeM=0.45, sectionHeightM=0.3, sectionWidthM=0.3, sectionTwM=0.011, sectionTfM=0.019, rebarCoverM=0.05, steelAM2=0.0149, concreteAM2=0.45 * 0.45 - 0.0149 - 0.0012, iM4=2.517e-4, wallThicknessM=0.019)


DOCS = {
    "beam-bending-fail": lambda d: action(d, "beams", "Q-office", qAreaPa=20000.0),
    "beam-deflection-fail": lambda d: (beam(d, spanM=16.0), action(d, "beams", "Q-office", qAreaPa=1200.0)),
    "beam-end-span": lambda d: beam(d, support="end_span"),
    "beam-end-span-hogging-fail": lambda d: (beam(d, support="end_span", asHoggingM2PerM=0.00002), action(d, "beams", "Q-office", qAreaPa=12000.0)),
    "beam-interior-span": lambda d: beam(d, support="interior_span", adjacentSpanM=8.0),
    "beam-cantilever": lambda d: beam(d, support="cantilever", spanM=4.0),
    "beam-cantilever-fail": lambda d: (beam(d, support="cantilever", spanM=4.0), action(d, "beams", "Q-office", qAreaPa=22000.0)),
    "column-filled": lambda d: action(d, "columns", "Q-col", nKN=400000.0, mKNm=60000.0),
    "column-filled-fail": lambda d: action(d, "columns", "Q-col", nKN=600000.0, mKNm=330000.0),
    "column-encased": encased_leaves,
    "column-encased-fail": lambda d: (encased_leaves(d), action(d, "columns", "Q-col", nKN=2500000.0, mKNm=600000.0)),
    "beam-partial-connection": lambda d: beam(d, studs={**d["beams"][0]["studs"], "totalCount": 16}),
    "beam-point-load-offset": lambda d: beam(d, actions=d["beams"][0]["actions"] + [dict(id="P-off", kind="imposed", category="B", stage="composite", qAreaPa=0.0, fKN=120000.0, deltaSigmaKPa=0.0, deltaTauKPa=0.0, xM=2.0, yM=0.0)]),
    "slab-continuous": lambda d: d["slabs"][0].update(support="continuous_2_span"),
    "slab-bending-fail": lambda d: action(d, "slabs", "Q-slab", qAreaPa=28000.0),
    "slab-shear-fail": lambda d: (d["slabs"][0].update(mFactor=60.0, kFactor=0.0), action(d, "slabs", "Q-slab", qAreaPa=12000.0)),
}

if __name__ == "__main__":
    for name, edit in DOCS.items():
        doc = variant(edit)
        rows = {r["id"].rsplit(".", 1)[0] if False else r["id"]: r["utilization"] for r in oracle.evaluate(doc)["checks"]}
        print(name)
        for rid, u in rows.items():
            print("   %-52s %.4f" % (rid, u))
        if "--write" in sys.argv:
            target = OUT / name / "🔣️.json"
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
