import copy, json, sys
S = sys.argv[1] + "/🧪️tests/🧫️fixtures/🧬️mutations"
load = lambda p: json.load(open(p, encoding="utf8"))
def dump(p, v):
    open(p, "w", encoding="utf8").write(json.dumps(v, ensure_ascii=False, indent=2) + "\n")
base = load(S + "/📎️change-annex/✅apply/🔣️.before.json")
template = load(S + "/✏️change-building-name/✅apply/🔣️.diff.json")
A = {"id": "case-c98-01", "labelEn": "Seismic base shear", "labelDe": "Erdbebenschubkraft", "entryId": "C98-01", "numbers": [{"id": "s_d_t1", "value": 0.25}, {"id": "m_total", "value": 1200000.0}], "texts": [{"id": "ground_type", "value": "b"}]}
B = {"id": "case-c98-02", "labelEn": "Storey drift", "labelDe": "Stockwerksverformung", "entryId": "C98-02", "numbers": [{"id": "d_r", "value": 0.012}, {"id": "h_storey", "value": 3.2}], "texts": [{"id": "limit_class", "value": "brittle"}]}
empty = {"removed": [], "inserted": [], "moved": [], "modified": []}
def patch(id, **fields):
    row = {"labelEn": None, "labelDe": None, "entryId": None, "numbers": copy.deepcopy(empty), "texts": copy.deepcopy(empty)}
    for k, v in fields.items(): row[k] = v if k in ("numbers", "texts") else v
    return {"id": id, "patch": row}
def mod(kind, id, **fields):
    return {**copy.deepcopy(empty), "modified": [patch(id, **fields)]}
def rows(items): return copy.deepcopy(items)
def edit(items, id, **fields):
    out = rows(items)
    for r in out:
        if r["id"] == id: r.update(fields)
    return out
def setvalue(items, id, listname, key, value):
    out = rows(items)
    for r in out:
        if r["id"] == id:
            for e in r[listname]:
                if e["id"] == key: e["value"] = value
    return out
def sub(listname, key, value):
    d = copy.deepcopy(empty); d["modified"] = [{"id": key, "patch": {"value": value}}]
    return d
cases = {
    "➕️insert-case": ([A], {"mutation": "insertCase", "index": 99, "case": B}, [A, B], {**copy.deepcopy(empty), "inserted": [{"index": 1, "row": B}]}),
    "➖️remove-case": ([A, B], {"mutation": "removeCase", "id": "case-c98-01"}, [B], {**copy.deepcopy(empty), "removed": [{"id": "case-c98-01", "index": 0}]}),
    "🔢️change-case-number": ([A], {"mutation": "changeCaseNumber", "caseId": "case-c98-01", "key": "s_d_t1", "value": 0.31}, setvalue([A], "case-c98-01", "numbers", "s_d_t1", 0.31), mod("", "case-c98-01", numbers=sub("numbers", "s_d_t1", 0.31))),
    "🔤️change-case-text": ([B], {"mutation": "changeCaseText", "caseId": "case-c98-02", "key": "limit_class", "value": "ductile"}, setvalue([B], "case-c98-02", "texts", "limit_class", "ductile"), mod("", "case-c98-02", texts=sub("texts", "limit_class", "ductile"))),
    "🏷️change-case-label-en": ([A], {"mutation": "changeCaseLabelEn", "caseId": "case-c98-01", "newValue": "Seismic base shear (revised)"}, edit([A], "case-c98-01", labelEn="Seismic base shear (revised)"), mod("", "case-c98-01", labelEn="Seismic base shear (revised)")),
    "🏷️change-case-label-de": ([A], {"mutation": "changeCaseLabelDe", "caseId": "case-c98-01", "newValue": "Erdbebenschubkraft (überarbeitet)"}, edit([A], "case-c98-01", labelDe="Erdbebenschubkraft (überarbeitet)"), mod("", "case-c98-01", labelDe="Erdbebenschubkraft (überarbeitet)")),
    "📇️change-case-entry-id": ([A], {"mutation": "changeCaseEntryId", "caseId": "case-c98-01", "newValue": "C98-04"}, edit([A], "case-c98-01", entryId="C98-04"), mod("", "case-c98-01", entryId="C98-04")),
}
import os
for name, (before, mutation, after, delta) in cases.items():
    d = f"{S}/{name}/✅apply"; os.makedirs(d, exist_ok=True)
    b = copy.deepcopy(base); b["cases"] = before
    a = copy.deepcopy(base); a["cases"] = after
    diff = copy.deepcopy(template); diff["cases"] = delta
    for f, v in (("before", b), ("mutation", mutation), ("after", a), ("diff", diff), ("outcome", {"status": "applied"})): dump(f"{d}/🔣️.{f}.json", v)
