"""Writes the seven apply vectors of the design-case mutation leaves under the en1994 mutation fixtures. Idempotent (overwrites). Run: /home/deck/semio/.venv/bin/python -I cases_vectors.py"""
import copy, json, os
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
FIX = ROOT + "/🧪️tests/🧫️fixtures/🧬️mutations"
inputs = {r["key"]: r for r in json.load(open(ROOT + "/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/📚️inputs/🔣️.json"))["inputs"]}
base = json.load(open(FIX + "/📍️insert-placement/✅apply/🔣️.before.json"))
assert base["cases"] == [], "base must carry an empty case list"
ENTRY = {"C94-09": ("Floor beam natural frequency", "Eigenfrequenz Deckenträger", ["l_span", "g_line", "q_line", "psi_vib", "i_short", "f_min"]), "C94-10": ("Continuous beam LTB", "Biegedrillknicken Durchlaufträger", ["sec_h", "sec_b", "sec_tf", "sec_tw", "sec_a", "sec_iy", "fy", "gamma_m1", "m_ed", "l_ltb", "i2_slab", "a_beam", "k_alpha", "steel_kind"])}


def row(case_id, entry):
    en, de, keys = ENTRY[entry]
    numbers = [{"id": k, "value": float(inputs[k]["example"])} for k in keys if inputs[k]["kind"] == "number"]
    texts = [{"id": k, "value": inputs[k]["example"]} for k in keys if inputs[k]["kind"] == "choice"]
    return {"id": case_id, "labelEn": en, "labelDe": de, "entryId": entry, "numbers": numbers, "texts": texts}


A, B = row("case-c94-09", "C94-09"), row("case-c94-10", "C94-10")
EMPTY = {"removed": [], "inserted": [], "moved": [], "modified": []}
patch = lambda **kw: {"labelEn": None, "labelDe": None, "entryId": None, "numbers": EMPTY, "texts": EMPTY, **kw}
doc = lambda cases: {**base, "cases": cases}


def write(kind_dir, mutation, before, after, delta):
    d = FIX + "/" + kind_dir + "/✅apply"
    os.makedirs(d, exist_ok=True)
    for name, value in (("before", before), ("mutation", mutation), ("after", after), ("diff", {"cases": delta}), ("outcome", {"status": "applied"})):
        open(d + "/🔣️." + name + ".json", "w", encoding="utf8").write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


write("➕️insert-case", {"InsertCase": {"index": 99, "case": B}}, doc([A]), doc([A, B]), {**EMPTY, "inserted": [{"index": 1, "row": B}]})
write("➖️remove-case", {"RemoveCase": {"id": A["id"]}}, doc([A, B]), doc([B]), {**EMPTY, "removed": [{"id": A["id"], "index": 0}]})
after = copy.deepcopy(A)
after["numbers"] = [{**n, "value": 4.0} if n["id"] == "f_min" else n for n in after["numbers"]]
write("🔢️change-case-number", {"ChangeCaseNumber": {"caseId": A["id"], "key": "f_min", "value": 4.0}}, doc([A]), doc([after]), {**EMPTY, "modified": [{"id": A["id"], "patch": patch(numbers={**EMPTY, "modified": [{"id": "f_min", "patch": {"value": 4.0}}]})}]})
after = copy.deepcopy(B)
after["texts"] = [{**t, "value": "welded"} if t["id"] == "steel_kind" else t for t in after["texts"]]
write("🔤️change-case-text", {"ChangeCaseText": {"caseId": B["id"], "key": "steel_kind", "value": "welded"}}, doc([B]), doc([after]), {**EMPTY, "modified": [{"id": B["id"], "patch": patch(texts={**EMPTY, "modified": [{"id": "steel_kind", "patch": {"value": "welded"}}]})}]})
after = {**A, "labelEn": "Floor beam natural frequency (revised)"}
write("🏷️change-case-label-en", {"ChangeCaseLabelEn": {"caseId": A["id"], "newValue": after["labelEn"]}}, doc([A]), doc([after]), {**EMPTY, "modified": [{"id": A["id"], "patch": patch(labelEn=after["labelEn"])}]})
after = {**A, "labelDe": "Eigenfrequenz Deckenträger (überarbeitet)"}
write("🏷️change-case-label-de", {"ChangeCaseLabelDe": {"caseId": A["id"], "newValue": after["labelDe"]}}, doc([A]), doc([after]), {**EMPTY, "modified": [{"id": A["id"], "patch": patch(labelDe=after["labelDe"])}]})
after = {**A, "entryId": "C94-04"}
write("📇️change-case-entry-id", {"ChangeCaseEntryId": {"caseId": A["id"], "newValue": "C94-04"}}, doc([A]), doc([after]), {**EMPTY, "modified": [{"id": A["id"], "patch": patch(entryId="C94-04")}]})
print("ok")
