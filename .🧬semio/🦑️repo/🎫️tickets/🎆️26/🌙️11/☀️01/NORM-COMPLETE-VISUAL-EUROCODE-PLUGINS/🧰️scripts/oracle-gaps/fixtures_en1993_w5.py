"""🧫 Writes the EN 1993 check-outcome fixtures of the fifth oracle wave: `joint-components` (end plate joints: thin plate, grouped rows, beam web in tension with a transformation parameter and a highly stressed column web, a joint without explicit rows) and `frame-second-order` / `frame-second-order-fail` (the showcase portal with node loads that put alpha_cr below 10)."""
import copy, json, pathlib
art = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
base = json.load(open(art / "🎪️showcase/🔣️.json", encoding="utf8"))


def write(name, doc):
    out = art / "📄️oracle-documents" / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write("\n")


def frame(h, v):
    d = copy.deepcopy(base)
    for n in d["frameNodes"]:
        if n["loadZ"]:
            n["loadZ"] = -v
        if n["loadX"]:
            n["loadX"] = h
    return d


write("frame-second-order", frame(5.0e4, 1.0e6))
write("frame-second-order-fail", frame(1.8e5, 1.4e6))
d = frame(4.0e4, 1.2e6)
d["frameBars"][1].update(releaseStart="Pinned", releaseEnd="Pinned")
write("frame-second-order-pinned-beam", d)

d = copy.deepcopy(base)
ep = next(j for j in d["joints"] if j["id"] == "joint-ep")
rows = [r for r in d["jointRows"] if r["jointId"] == "joint-ep"]


def joint(jid, **kw):
    j = copy.deepcopy(ep)
    j.update(id=jid, **kw)
    d["joints"].append(j)
    return j


def row(rid, jid, base_row, **kw):
    r = copy.deepcopy(base_row)
    r.update(id=rid, jointId=jid, **kw)
    d["jointRows"].append(r)


joint("joint-ep2", plateThickness=0.012, momentDemand=130000.0, assumedClass="Pinned")
for r in rows:
    row(r["id"] + "-b", "joint-ep2", r)
joint("joint-ep3", momentDemand=95000.0, assumedClass="SemiRigid", compressionLever=0.30, beamMomentResistance=160000.0)
row("row-31", "joint-ep3", rows[0], lever=0.30)
row("row-32", "joint-ep3", rows[1], lever=0.21, pitchToPrevious=0.09, groupWithPrevious=1, rowType="FirstBelow")
row("row-33", "joint-ep3", rows[1], lever=0.12, pitchToPrevious=0.09, groupWithPrevious=1, rowType="Inner")
joint("joint-ep4", beamWebThickness=0.0071, beamWebFy=355000000.0, transformationParameter=0.8, columnWebStress=290000000.0, bracedFrame=0, assumedClass="SemiRigid", momentDemand=60000.0)
for r in rows:
    row(r["id"] + "-d", "joint-ep4", r, alphaFactor=5.8)
joint("joint-ep5", momentDemand=110000.0, assumedClass="Rigid", boltRows=3, pitch=0.09)
write("joint-components", d)

d = copy.deepcopy(base)
c1 = d["craneRunways"][0]
c2 = copy.deepcopy(c1)
c2.update(id="crane-2", maxWheelLoad=420000.0, stressRange=110000000.0, detailCategoryStress=71000000.0, railFixed=0, craneNu=0.8)
c3 = copy.deepcopy(c1)
c3.update(id="crane-3", maxWheelLoad=230000.0, webThickness=0.008, wheelEccentricity=0.04, stressRange=55000000.0)
d["craneRunways"] += [c2, c3]
write("crane-runway-fail", d)
