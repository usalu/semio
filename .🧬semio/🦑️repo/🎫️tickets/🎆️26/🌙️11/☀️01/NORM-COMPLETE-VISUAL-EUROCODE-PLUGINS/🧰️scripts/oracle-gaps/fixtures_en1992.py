"""🧫 Writes the EN 1992 check-outcome fixtures `torsion-strut-pass` and `torsion-strut-fail` (rc-building-showcase-pass with beam-B1 turned into a 400 x 800 beam under a torsion moment and two deep beams, DB1 with a 0.4 m bearing plate and DB2 without)."""
import copy, json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
for name, t_k, load in (("torsion-strut-pass", 40000.0, 1.0), ("torsion-strut-fail", 330000.0, 7.0)):
    d = json.load(open(root / "rc-building-showcase-pass/🔣️.json", encoding="utf8"))
    beam = next(m for m in d["members"] if m["id"] == "beam-B1")
    beam.update(width=0.4, height=0.8, effectiveDepth=0.74)
    beam["longitudinal"][0].update(count=8, anchorageLength=1.6, lapLength=2.0)
    beam["longitudinal"].append(dict(beam["longitudinal"][0], id="top", diameter=0.02, count=6, position="Top"))
    beam["stirrups"] = {"diameter": 0.01, "spacing": 0.1, "legs": 2}
    for a in beam["actions"]:
        if a["id"] == "Q":
            a["tK"] = t_k
    deep = copy.deepcopy(beam)
    deep.update(id="beam-DB1", labelEn="Deep beam DB1", labelDe="Wandartiger Träger DB1", width=0.3, height=1.2, effectiveDepth=1.1, span=2.4, bearingWidth=0.4, stirrups={"diameter": 0.01, "spacing": 0.1, "legs": 2})
    deep["longitudinal"] = [dict(beam["longitudinal"][0], diameter=0.016, count=4, anchorageLength=1.2, lapLength=1.5)]
    deep["actions"] = [dict(beam["actions"][0], gKLine=80000.0 * load), dict(beam["actions"][1], qKLine=60000.0 * load, tK=0.0)]
    plain = copy.deepcopy(deep)
    plain.update(id="beam-DB2", labelEn="Deep beam DB2", labelDe="Wandartiger Träger DB2", bearingWidth=0.0)
    d["members"] += [deep, plain]
    out = root / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as handle:
        json.dump(d, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
