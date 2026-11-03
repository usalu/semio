"""🧫 Writes the EN 1998 check-outcome fixtures `families-pass`, `families-fail`, `families-en-fail` and `families-zone0` (showcase with covered or under-sized bridge piers, silo, tank, foundation, tower and retrofit capacity, a ground class mismatch, a derived a_gR warning, an EN-annex combo mismatch and the very low seismicity exemption)."""
import copy, json, pathlib
base = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
show = json.load(open(base / "🎪️showcase/🔣️.json", encoding="utf8"))
show["towers"][0]["baseRotationalStiffnessNm"] = 0.0
passing = copy.deepcopy(show)
passing["silos"][0]["nRdN"] = 6.0e6
for pier in passing["piers"]:
    pier["shearRdN"] = 4.0e6
passing["towers"][0]["mRdNm"] = 2.4e6
failing = copy.deepcopy(show)
failing["site"]["deGroundCombo"] = "A-R"
failing["site"]["aGr"] = 0.8
failing["tanks"][0].update(vRdN=3.0e5, fillingRatio=0.998)
failing["towers"][0]["mRdNm"] = 3.0e5
failing["foundations"][0]["kFoundation"] = 4.0e5
failing["assessments"][0]["rKN"] = 1.0e5
en_fail = copy.deepcopy(passing)
en_fail["annex"] = "En"
en_fail["site"].update(enGroundType="c", deGroundCombo="C-T", aGr=0.6)
zero = copy.deepcopy(passing)
zero["site"].update(sApR=0.0, aGr=0.0)
for name, doc in (("families-pass", passing), ("families-fail", failing), ("families-en-fail", en_fail), ("families-zone0", zero)):
    out = base / "🚦️check-outcome" / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as handle:
        json.dump(doc, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
