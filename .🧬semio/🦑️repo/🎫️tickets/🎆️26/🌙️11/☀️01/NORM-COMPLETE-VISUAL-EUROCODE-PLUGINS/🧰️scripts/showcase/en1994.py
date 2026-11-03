"""Generates the EN 1994 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`: one bridge-kind composite structure whose beams, columns, slabs and placement populate every collection and every check family of the spec trace (python3 -I en1994.py)."""
import copy, json, pathlib, re, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, finish, load

ART = artifact("🧩️en1994")
CATALOGUE = ART / "🧬️schema/💡️inferences/⚖️checks/📇️catalogue"
BASE = load(ART / "🧪️tests/🧫️fixtures/🧬️mutations/📏️change-beam-span-m/✅apply/🔣️.after.json")
beam0, col0, slab0 = BASE["beams"][0], BASE["columns"][0], BASE["slabs"][0]


def demo_cases():
    """The full-catalogue demo cases (C94-01..C94-12): one case per entry seeded from the documented example inputs of the vocabulary, as `catalogue_demo_cases()` seeds them (numbers and texts sorted by key)."""
    vocabulary = {row["key"]: row for row in json.loads((CATALOGUE / "📚️inputs/🔣️.json").read_text(encoding="utf8"))["inputs"]}
    rows = []
    for entry, en, de, keys in re.findall(r'Entry \{ id: "(C94-\d+)", part: "[^"]*", clause: "[^"]*", en: "([^"]*)", de: "([^"]*)", inputs: &\[([^\]]*)\]', (CATALOGUE / "🦀️.rs").read_text(encoding="utf8")):
        declared = [vocabulary[k.strip().strip('"')] for k in keys.split(",") if k.strip()]
        numbers = {r["key"]: float(r["example"]) for r in declared if r["kind"] == "number" and r.get("example") is not None}
        texts = {r["key"]: r["example"] for r in declared if r["kind"] != "number" and r.get("example") is not None}
        rows.append({"id": "case-" + entry.lower(), "labelEn": en, "labelDe": de, "entryId": entry, "numbers": [{"id": k, "value": numbers[k]} for k in sorted(numbers)], "texts": [{"id": k, "value": texts[k]} for k in sorted(texts)]})
    return rows


def action(aid, kind, category, stage, q=0.0, f=0.0, ds=0.0, dt=0.0):
    return {"id": aid, "kind": kind, "category": category, "stage": stage, "qAreaPa": q, "fKN": f, "deltaSigmaKPa": ds, "deltaTauKPa": dt}


def beam(bid, **leaves):
    return clone(beam0, id=bid, **leaves)


def with_action(row, new):
    row = copy.deepcopy(row)
    row["actions"].append(new)
    return row


girder_steel = {"designation": "PLATE-GIRDER-1000", "heightM": 1.0, "widthM": 0.4, "twM": 0.016, "tfM": 0.030, "aM2": 0.03904, "wPlYM3": 0.015174, "iYM4": 0.006754, "aVM2": 0.016}
girder_studs = {"diameterM": 0.022, "heightM": 0.15, "fUPa": 450000000.0, "spacingM": 0.2, "count": 120}
base_stud = [s for s in BASE["studs"] if s["beamId"] == beam0["id"]][0]
beam_studs = {"diameterM": base_stud["diameterM"], "heightM": base_stud["heightM"], "fUPa": base_stud["fUPa"], "spacingM": 0.2, "count": 40}


def stud_rows(row, studs):
    """The persisted stud rows of one beam: `count` studs `spacingM` apart, centred on the middle of the designed span (an interior span and a cantilever start behind the neighbouring span)."""
    adjacent = row.get("adjacentSpanM") or row["spanM"]
    origin = adjacent if row["support"] in ("interiorSpan", "cantilever") else 0.0
    centre = origin + row["spanM"] / 2
    return [{"id": f"{row['id']}-stud-{i}", "beamId": row["id"], "xM": round(centre + (i - (studs["count"] - 1) / 2) * studs["spacingM"], 9), "diameterM": studs["diameterM"], "heightM": studs["heightM"], "fUPa": studs["fUPa"]} for i in range(studs["count"])]


girder_actions = [
    action("G-steel", "permanent", "self_steel", "construction", q=860.0),
    action("G-wet", "permanent", "wet_concrete", "construction", q=4750.0),
    action("Q-constr", "construction", "construction", "construction", q=1000.0),
    action("G-steel-comp", "permanent", "self_steel", "composite", q=860.0),
    action("G-slab", "permanent", "slab", "composite", q=4275.0),
    action("G-surfacing", "permanent", "finishes", "composite", q=2300.0),
    action("Q-footway", "imposed", "B", "composite", q=2000.0),
    action("FLM3", "fatigue", "flm3", "composite", ds=65000000.0, dt=40000000.0),
]
girder = beam("girder-G1", spanM=25.0, spacingM=3.5, support="simplySupported", steel=girder_steel, slabThicknessM=0.25, ltbLengthM=5.0, nCycles=2000000.0, bridgeLanes=2,
              transverseAsM2PerM=0.0021, asHoggingM2PerM=0.0012, studTransverseSpacingM=0.1, studEdgeDistanceM=0.04, transverseAsTopM2PerM=0.001, endPlateWidthM=0.3, endPlateHeightM=0.3, endZoneLengthM=0.5, actions=girder_actions)

beams = [
    girder,
    with_action(beam("beam-B1-propped", support="simplySupported", spanM=8.0), action("P-equipment", "imposed", "B", "composite", f=20000.0)),
    beam("beam-B2-unpropped", support="simplySupported", construction="unpropped", spanM=8.0, ltbLengthM=5.0),
    beam("beam-B3-end-span", support="endSpan", spanM=10.0, adjacentSpanM=8.0),
    beam("beam-B4-interior-span", support="interiorSpan", spanM=10.0, adjacentSpanM=8.0),
    beam("beam-B5-two-span", support="continuous2Span", spanM=9.0, adjacentSpanM=9.0),
    beam("beam-B6-cantilever", support="cantilever", spanM=4.0),
]

encased = clone(col0, id="col-C2-encased", kind="encased", outerSizeM=0.45, sectionHeightM=0.3, sectionWidthM=0.3, sectionTwM=0.011, sectionTfM=0.019, rebarCoverM=0.05, steelAM2=0.0149, concreteAM2=0.45 * 0.45 - 0.0149 - 0.0012, iM4=2.517e-4, wallThicknessM=0.019)
partial = clone(encased, id="col-C3-partially-encased", kind="partiallyEncased", outerSizeM=0.4, concreteAM2=0.4 * 0.4 - 0.0149 - 0.0012)
columns = [clone(col0, id="col-C1-filled"), encased, partial]

slabs = [
    clone(slab0, id="slab-S1-simple"),
    clone(slab0, id="slab-S2-continuous", support="continuous2Span"),
    with_action(clone(slab0, id="slab-S3-point-load"), action("P-wheel", "imposed", "B", "composite", f=10000.0)),
]

doc = copy.deepcopy(BASE)
for row in beams:
    row["steel"].setdefault("rM", 0.0)
    for a in row["actions"]:
        a.setdefault("xM", 0.0)
for row in slabs:
    for a in row["actions"]:
        a.setdefault("xM", 0.0)
        a.setdefault("yM", 0.0)
doc.update(annex="De", reliabilityClass="RC2", structureKind="bridge", fatigueDetail="reinforcement", fireRating="r60", insulationThicknessM=0.025, beams=beams, columns=columns, slabs=slabs,
           placements=[{"id": "girder-G1", "x": 0.0, "y": 1.75, "z": 0.0, "yaw": 0.0}], studs=[s for row in beams for s in stud_rows(row, girder_studs if row["id"] == "girder-G1" else beam_studs)], cases=demo_cases())
finish(doc, ART, "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json", compact=True)
