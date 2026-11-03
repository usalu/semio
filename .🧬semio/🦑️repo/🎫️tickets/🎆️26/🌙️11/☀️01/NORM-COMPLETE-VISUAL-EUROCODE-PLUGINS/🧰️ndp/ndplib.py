import json, pathlib
REG = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json")
DOCS = {"1990": "DIN EN 1990/NA", "1991": "DIN EN 1991/NA", "1992": "DIN EN 1992-1-1/NA", "1993": "DIN EN 1993-1-1/NA", "1994": "DIN EN 1994-1-1/NA", "1995": "DIN EN 1995-1-1/NA", "1996": "DIN EN 1996-1-1/NA", "1997": "DIN 1054:2021-04", "1998": "DIN EN 1998-1/NA:2023-11", "1999": "DIN EN 1999-1-1/NA"}
def load():
    return {e["id"]: e for e in json.load(open(REG))["entries"]}
def put(d, fam, key, clause, symbol, rec, de, unit="", note="", part="", locator=None, keep=False):
    i = f"ndp.en{fam}.{key}"
    if keep and i in d: return i
    old = d.get(i)
    d[i] = dict(id=i, code=f"EN {fam}", part=part, clause=clause, symbol=symbol, recommended=rec, naDe=de, unit=unit,
        source=dict(document=DOCS[fam], edition=(old or {}).get("source", {}).get("edition", "to be recorded at verification"), locator=locator or clause),
        status="unverified", specRef=(old or {}).get("specRef", "wave2 ndp: value as used by the artifact"), asWritten=note or (old or {}).get("asWritten", ""), evidence=(old or {}).get("evidence", ""))
    return i
def save(d):
    json.dump(dict(entries=sorted(d.values(), key=lambda e: e["id"])), open(REG, "w"), ensure_ascii=False, indent=1)
