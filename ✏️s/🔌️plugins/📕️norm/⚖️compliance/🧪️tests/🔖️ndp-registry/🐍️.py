import json, os, pathlib, re, sys
HERE = pathlib.Path(__file__).resolve().parent
REGISTRY = HERE.parent.parent / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json"
TABLE = REGISTRY.parent / "🗃️table"
CAP = 1.0e6
TOKEN = re.compile(r'Number\(([^)]*)\)|Unset|Text\("((?:[^"\\]|\\.)*)"\)')

def entries():
    return json.loads(REGISTRY.read_text())["entries"]

def ratio(demand, capacity):
    if capacity < 2.220446049250313e-16:
        return 0.0 if demand < 2.220446049250313e-16 else CAP
    return min(demand / capacity, CAP)

def verdict(computed, limit, ndps, status_by_id, annex=None):
    util = ratio(computed, limit)
    status = "Pass" if util <= 1.0 else "Fail"
    if status == "Pass" and any(status_by_id.get(i) != "verified" for i in ndps):
        status = "UnverifiedBasis"
    return status, util

def read_number(by_id, id_, annex):
    entry = by_id.get(id_)
    if entry is None:
        return None, ("unregistered", f"NDP {id_} is not registered", f"NDP {id_} ist nicht registriert")
    value = entry["recommended" if annex == "en" else "naDe"]
    label = "EN" if annex == "en" else "DE-NA"
    if value is None:
        return None, ("no-value", f"NDP {id_} has no value for annex {label}", f"NDP {id_} hat keinen Wert für den Anhang {label}")
    if not isinstance(value, (int, float)):
        return None, ("not-numeric", f"NDP {id_} is not a number for annex {label}", f"NDP {id_} ist für den Anhang {label} keine Zahl")
    return float(value), None

def read_common(by_id, id_):
    (en, en_gap), (de, de_gap) = read_number(by_id, id_, "en"), read_number(by_id, id_, "de")
    if en_gap or de_gap:
        return None, [g for g in (en_gap, de_gap) if g]
    return (en, []) if en == de else (None, [("divergent", f"NDP {id_} differs between the annexes and needs the annex choice", f"NDP {id_} weicht zwischen den Anhängen ab und braucht die Anhangswahl")])

def refusal(computed, limit, gaps):
    return ("UnverifiedBasis", ratio(computed, limit)) if gaps else None

def minimum(computed, minimum_):
    util = 0.0 if minimum_ <= 0 else ratio(minimum_, computed)
    return ("Pass", util) if computed >= minimum_ else ("Fail", max(util, 1.0))

def generated_rows():
    rows = {}
    for crate in sorted(TABLE.glob("*/🦀️.rs")):
        for line in crate.read_text(encoding="utf-8").splitlines():
            m = re.match(r'    ([cd])\("([^"]+)",', line)
            if m:
                tokens = [(a and float(a), b) if a or b is not None else None for a, b in TOKEN.findall(line)] if False else [x for x in TOKEN.finditer(line)]
                values = [float(x.group(1)) if x.group(1) is not None else x.group(2) if x.group(0).startswith("Text") else None for x in tokens]
                whole = ".ver()," in line.rstrip() + ","
                rows[m.group(2)] = (m.group(1), values, (whole or ".ver_en()" in line, whole or ".ver_de()" in line))
    return rows

def annex_verified(x):
    per = x.get("annexStatus")
    if per is None:
        return (x["status"] == "verified",) * 2
    return per["en"]["status"] == "verified", per["naDe"]["status"] == "verified"

def generated_tables_equal_registry(e):
    rows = generated_rows()
    assert set(rows) == {x["id"] for x in e}, "generated tables hold other ids than the registry: run the ticket codegen"
    for x in e:
        kind, values, verified = rows[x["id"]]
        assert verified == annex_verified(x), x["id"]
        assert (x["status"] == "verified") == all(verified), x["id"]
        want = [x["recommended"], x["naDe"]] if kind == "d" else [x["recommended"]]
        assert values[:len(want)] == [float(v) if isinstance(v, (int, float)) else v for v in want], (x["id"], values, want)

def evidence_invariants(e):
    for x in e:
        for side, ok in zip(("en", "naDe"), annex_verified(x)):
            if ok and "annexStatus" in x:
                assert x["annexStatus"][side]["evidence"].strip(), (x["id"], side)
        if x["status"] == "verified":
            evidence, edition = x["evidence"].strip(), x["source"]["edition"].strip()
            assert evidence and "CARRIED-OVER" not in evidence and "no independent source" not in evidence, x["id"]
            assert edition and not edition.startswith("to be recorded"), x["id"]

NORM = HERE.parent.parent.parent
NEGATIVE_CONTROLS = {"ndp.does.not.exist", "ndp.nobody.unknown", "ndp.en1990.a2.group.gr9.road_traffic"}
STRING_PREFIX_PROBES = {"ndp.en1993.gamma-m"}
SKIPPED_DIRS = {"target", ".git", "🗑️generated", "__pycache__", "🗃️table"}
LITERAL = re.compile(r'"(ndp\.[A-Za-z0-9_][A-Za-z0-9_.\-]*[A-Za-z0-9_])"')
DIRECT = re.compile(r'(?:\bndp|ndp_number|ndp_number_opt|ndp_common|ndp_is_verified|ndp_is_verified_for|ndp_lookup|ndp_awaits_value)\(\s*(?:[A-Za-z_:]+,\s*)?"(ndp\.[^"{]+)"')
CELL = re.compile(r'ndp_(?:cell|cell_opt|cell_common)\(\s*"(ndp\.[^"{]+)"\s*,\s*&\[((?:\s*"[^"{}]*"\s*,?)*)\s*\]')

def code_lookups():
    direct, literal, cells = {}, {}, {}
    for root, dirs, files in os.walk(NORM):
        dirs[:] = [d for d in dirs if d not in SKIPPED_DIRS]
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = pathlib.Path(root) / name
            text = path.read_text(encoding="utf-8")
            for pattern, sink in ((DIRECT, direct), (LITERAL, literal)):
                for m in pattern.finditer(text):
                    sink.setdefault(m.group(1), []).append(path.name)
            for m in CELL.finditer(text):
                cells.setdefault(m.group(1) + "." + ".".join(re.findall(r'"([^"]*)"', m.group(2))), []).append(path.name)
    return direct, literal, cells

BLIND_ID = re.compile(r'(?<![\w.])(?:ndp_common|ndp)\(\s*"(ndp\.[^"{]+)"')
BLIND_TEMPLATE = re.compile(r'(?<![\w.])ndp_common\(\s*&format!\(\s*"(ndp\.[^"{]*)\{')
BLIND_CELL = re.compile(r'ndp_cell_common\(\s*"(ndp\.[^"{]+)"\s*,\s*(&\[(?:\s*"[^"{}]*"\s*,?)*\s*\])?')
BLIND_LIMIT = re.compile(r'normative_limit\(\s*&\[((?:\s*"[^"{}]*"\s*,)*(?:\s*"[^"{}]*")?)(\s*\])?')
WRAPPED_TABLES = {"ndp.en1996.limit"}

def blind_reads(ids):
    exact, prefixes = {}, {}
    for root, dirs, files in os.walk(NORM):
        dirs[:] = [d for d in dirs if d not in SKIPPED_DIRS]
        for name in files:
            if not name.endswith(".rs"):
                continue
            text = (pathlib.Path(root) / name).read_text(encoding="utf-8")
            for m in BLIND_ID.finditer(text):
                exact[m.group(1)] = name
            for m in BLIND_TEMPLATE.finditer(text):
                if m.group(1).count(".") >= 3:
                    prefixes[m.group(1)] = name
            for m in BLIND_CELL.finditer(text):
                if m.group(2):
                    exact[m.group(1) + "." + ".".join(re.findall(r'"([^"]*)"', m.group(2)))] = name
                elif m.group(1) not in WRAPPED_TABLES:
                    prefixes[m.group(1) + "."] = name
            for m in BLIND_LIMIT.finditer(text):
                keys = ".".join(re.findall(r'"([^"]*)"', m.group(1)))
                if m.group(2):
                    exact["ndp.en1996.limit." + keys] = name
                else:
                    prefixes["ndp.en1996.limit." + keys + "."] = name
    rows = {i for i in exact if i in ids} | {i for p in prefixes for i in ids if i.startswith(p)}
    assert rows, "static scan found no annex-blind reads: the scan patterns are stale"
    return sorted(rows)

def every_id_referenced_by_a_lookup_in_code_is_registered(e):
    ids = {x["id"] for x in e}
    direct, literal, cells = code_lookups()
    assert direct and literal, "static scan found no lookups: the scan patterns are stale"
    exact = {i: f for i, f in {**direct, **cells}.items() if i not in NEGATIVE_CONTROLS}
    orphans = sorted(i for i in exact if i not in ids)
    assert not orphans, f"lookups of unregistered NDP ids (typo, rename or missing row): {orphans[:10]}"
    families = sorted(i for i in literal if i not in NEGATIVE_CONTROLS | STRING_PREFIX_PROBES and i not in ids and not any(r.startswith(i + ".") for r in ids))
    assert not families, f"NDP id literals that match no registry row or table prefix: {families[:10]}"
    return len(exact), len(literal)

def fixture_drift():
    fixture = json.loads((HERE.parent / "🧫️fixtures" / "🔖️ndp-annex-differences" / "🔣️.json").read_text(encoding="utf-8"))["differing"]
    want = sorted(x["id"] for x in entries() if isinstance(x["recommended"], (int, float)) and isinstance(x["naDe"], (int, float)) and x["recommended"] != x["naDe"])
    return sorted(set(want) - set(fixture)), sorted(set(fixture) - set(want))

def main():
    if "--check" in sys.argv:
        missing, stale = fixture_drift()
        assert not missing and not stale, f"annex-differences fixture drifted from the registry: +{missing[:10]} -{stale[:10]}; regenerate with 🧰️ndp/ndp_annex_differences.py"
        print("ndp fixture in sync", len(entries()), "entries")
        return 0
    e = entries()
    ids = [x["id"] for x in e]
    assert ids == sorted(ids) and len(set(ids)) == len(ids)
    for x in e:
        assert x["code"] and x["clause"].strip() not in ("", "?") and x["source"]["document"] and x["status"] in ("verified", "unverified")
    evidence_invariants(e)
    lookups, literals = every_id_referenced_by_a_lookup_in_code_is_registered(e)
    generated_tables_equal_registry(e)
    status_by_id = {x["id"]: x["status"] for x in e}
    first = [ids[0]]
    for computed, limit, ndps, want, complies in [(0.5, 1.0, [], "Pass", True), (0.5, 1.0, first, "UnverifiedBasis", False), (0.97, 1.0, first, "UnverifiedBasis", False), (2.0, 1.0, first, "Fail", False), (3.0, 0.0, [], "Fail", False), (3.0, -2.0, [], "Fail", False)]:
        got, _ = verdict(computed, limit, ndps, status_by_id)
        assert got == want and (got == "Pass") == complies, (computed, limit, got)
    for computed, minimum_, want, util in [(0.5, 1.0, "Fail", 2.0), (2.0, 1.0, "Pass", 0.5), (1.0, 1.0, "Pass", 1.0)]:
        got, u = minimum(computed, minimum_)
        assert got == want and abs(u - util) < 1e-12
    by_id = {x["id"]: x for x in e}
    de_only = next(x for x in e if x["recommended"] is None and isinstance(x["naDe"], (int, float)))
    value, gap = read_number(by_id, de_only["id"], "en")
    assert value is None and gap == ("no-value", f"NDP {de_only['id']} has no value for annex EN", f"NDP {de_only['id']} hat keinen Wert für den Anhang EN"), gap
    assert read_number(by_id, de_only["id"], "de") == (float(de_only["naDe"]), None)
    assert read_number(by_id, "ndp.does.not.exist", "en")[1][0] == "unregistered"
    assert read_common(by_id, de_only["id"])[0] is None
    for computed, limit, want in [(0.5, 1.0, "UnverifiedBasis"), (2.0, 1.0, "UnverifiedBasis")]:
        assert refusal(computed, limit, [gap])[0] == want and refusal(computed, limit, []) is None
    blind = blind_reads(by_id)
    unequal = [i for i in blind if by_id[i]["recommended"] is None or by_id[i]["naDe"] is None or by_id[i]["recommended"] != by_id[i]["naDe"]]
    assert not unequal, f"annex-blind reads (ndp_common, ndp_cell_common, annex-less ndp) of ids whose EN and DE values are not equal numbers: {unequal[:10]}; read them with the annex choice"
    split = [x for x in e if "annexStatus" in x]
    r3 = next(x for x in e if x["id"] == "ndp.en1997.r3.a")
    assert annex_verified(r3) == (True, False) and r3["status"] == "unverified"
    by_annex = {"en": {x["id"]: "verified" if annex_verified(x)[0] else "unverified" for x in e}, "de": {x["id"]: "verified" if annex_verified(x)[1] else "unverified" for x in e}}
    assert verdict(0.5, 1.0, [r3["id"]], by_annex["en"])[0] == "Pass" and verdict(0.5, 1.0, [r3["id"]], by_annex["de"])[0] == "UnverifiedBasis"
    for x in split:
        assert x["status"] == ("verified" if all(annex_verified(x)) else "unverified"), x["id"]
    print(f"ok: {lookups} direct lookups and {literals} id literals resolve; {len(blind)} annex-blind reads are equal in both annexes; {len(e)} NDP entries, {sum(x['status'] == 'unverified' for x in e)} unverified")

if __name__ == "__main__":
    sys.exit(main())
