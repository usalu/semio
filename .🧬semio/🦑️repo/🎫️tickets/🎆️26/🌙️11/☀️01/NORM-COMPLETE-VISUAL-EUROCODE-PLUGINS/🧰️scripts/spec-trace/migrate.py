# 🧭️ One-shot migration of the five legacy trace variants plus the new codes into the shared spec-trace format. Writes `🧭️spec-trace/🔣️.json` and the `🦀️.rs` twin per artifact and prints the per-code counts. The JSON files are the source of truth afterwards; re-running overwrites them.
import collections, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib.util
import spec_ids as S
_spec = importlib.util.spec_from_file_location('spec_trace_codegen', '/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py')
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)

STANDARD = {c: "EN " + c[2:] for c in G.FOLDERS}
CHECKS_DIR = "🧬️schema/💡️inferences/⚖️checks"


def art(code, *parts):
    return os.path.join(G.ART, G.FOLDERS[code], G.ANY, *parts)


def read(path):
    return open(path, encoding="utf-8").read()


def clause_of(code, idv, line):
    text = S.heading(code, line)
    m = None
    if code in ("en1990", "en1991"):
        m = re.match(r"^\*\*" + re.escape(idv) + r"\s+(.+?)\s+/\s", text)
    elif code == "en1992":
        m = re.match(r"^\*\*" + re.escape(idv) + r"\s+—\s+(.+?)\s+—\s", text)
    elif code == "en1993":
        m = re.match(r"^\*\*" + re.escape(idv) + r"\s+(.+?)\s+—\s", text)
    elif code in ("en1994", "en1999"):
        m = re.search(r"cl\.\s+(.+?)(?:\.\s+EN:|:\s|\.\s|\s\()", text) or re.search(r"—\s+(\d[\d.,–\s]*?\d)(?:\.\s|:\s|\s\()", text) or re.search(r"\b(\d:\d[\d.]*|F:\d[\d.]*)", text)
    elif code in ("en1995", "en1996", "en1998"):
        ms = re.findall(r"\(([^()]*\d[^()]*)\)", text)
        ms = [x for x in ms if not re.fullmatch(r"[A-Z]+-\d+.*", x)]
        m = re.match(r"(.*)", ms[-1]) if ms else None
    elif code == "en1997":
        m = re.match(r"^#+\s+(\d+(?:\.\d+)*)\s", text)
    clause = m.group(1).strip() if m else "-"
    return clause[:80] or "-"


def legacy_en1990():
    src = read(art("en1990", CHECKS_DIR, "🧭️spec-trace/🦀️.rs"))
    rows = {}
    for m in re.finditer(r'\("(E90\.\d+)", &\[([^\]]*)\]\),', src):
        rows[m.group(1)] = dict(realisation={"kind": "check", "checks": re.findall(r'"([^"]*)"', m.group(2))}, note="", entity="", clause=None)
    open_ids = re.findall(r'"(E90\.\d+)"', src.split("pub const SPEC_OPEN")[1])
    reasons = {
        "E90.10": "the snapshot has no vocabulary for material strengths X_k and the conversion factor eta, so X_d = eta X_k / gamma_m cannot be checked",
        "E90.11": "the snapshot has no vocabulary for geometric tolerances, so a_d = a_nom plus or minus delta a cannot be checked",
        "E90.12": "the snapshot has no vocabulary for the analysis type and the elastic critical load factor alpha_cr",
    }
    for i in open_ids:
        rows[i] = dict(realisation={"kind": "not-modelled", "reason": reasons[i]}, note="", entity="", clause=None)
    return rows


def legacy_en1991():
    src = read(art("en1991", CHECKS_DIR, "🧭️spec-trace/🦀️.rs"))
    rows = {}
    for m in re.finditer(r'row\("(E1\d\.\d+)", "([^"]*)", &\[([^\]]*)\]\)', src):
        rows[m.group(1)] = dict(realisation={"kind": "check", "checks": re.findall(r'"([^"]*)"', m.group(3))}, note="", entity="", clause=m.group(2))
    return rows


def legacy_en1993():
    src = read(art("en1993", CHECKS_DIR, "📏️parts/🦀️.rs"))
    entries = {}
    for m in re.finditer(r'entry!\("([^"]+)", "([^"]*)", "([^"]*)", Realisation::(\w+)\((.*?)\), "([^"]*)"\),\n', src):
        i, clause, entity, kind, arg, note = m.groups()
        r = {"kind": {"Check": "check", "Data": "data", "NotModelled": "not-modelled"}[kind]}
        if kind == "Check":
            r["checks"] = re.findall(r'"([^"]*)"', arg)
        elif kind == "Data":
            r["path"] = json.loads(arg)
        else:
            r["reason"] = json.loads(arg)
        entries[i] = dict(realisation=r, note=note, entity=entity, clause=clause)
    traced = {i["id"]: i for i in json.load(open(art("en1993", "🧭️spec-trace/🔣️.json"), encoding="utf-8"))["items"]}
    norm = lambda p: p.rstrip(".")
    for i, e in entries.items():
        j = traced.get(i)
        if not j:
            continue
        r, jr = e["realisation"], None
        if j["kind"] == "checked" and j["checks"]:
            jr = j["checks"]
        notes = [n for n in (e["note"], j["note"]) if n]
        if jr:
            if r["kind"] == "check":
                have = {norm(p) for p in r["checks"]}
                r["checks"] += [p for p in jr if norm(p) not in have]
            else:
                lead = "partial" if r["kind"] == "not-modelled" else "also"
                detail = r["reason"] if r["kind"] == "not-modelled" else r["path"]
                notes.append(f"{lead} {r['kind']}: {detail}")
                e["realisation"] = {"kind": "check", "checks": list(jr)}
        merged = []
        for n in notes:
            if not any(n in k or k in n for k in merged):
                merged = [k for k in merged if k not in n] + [n]
        e["note"] = "; ".join(merged)
    return entries


def legacy_en1995():
    src = read(art("en1995", CHECKS_DIR, "🧭️traceability/🦀️.rs"))
    rows = {}
    for m in re.finditer(r'\("(T-\d+)", Realisation::(\w+)\(?(?:"([^"]*)")?\)?\),', src):
        i, kind, prefix = m.groups()
        if kind == "Case":
            rows[i] = dict(realisation={"kind": "design-case", "entry": i}, note="", entity="cases", clause=None)
        else:
            rows[i] = dict(realisation={"kind": "check", "checks": [prefix]}, note="", entity="members" if kind == "Member" else "connections", clause=None)
    return rows


def legacy_en1998():
    src = read(art("en1998", "🧬️schema/💡️inferences/🧭️spec-trace/🦀️.rs"))
    rows = {}
    pat = re.compile(r'SpecTrace \{ id: "([^"]+)", clause: "([^"]*)", en: "([^"]*)", de: "([^"]*)", kind: TraceKind::(\w+), checks: &\[([^\]]*)\], note: "([^"]*)" \},')
    for m in pat.finditer(src):
        i, clause, en, de, kind, checks, note = m.groups()
        cs = re.findall(r'"([^"]*)"', checks)
        if kind == "Kernel":
            r = {"kind": "data", "path": "part_1::accelerogram_set_acceptance"}
        else:
            r = {"kind": "check", "checks": cs}
        rows[i] = dict(realisation=r, note=note, entity="", clause=clause)
    return rows


def legacy_en1999():
    src = read(art("en1999", CHECKS_DIR, "📏️parts/🦀️.rs"))
    rows = {}
    for m in re.finditer(r'\("(A-\d+)", "([^"]*)", Realisation::(Check|Data)\("([^"]*)"\)\),', src):
        i, clause, kind, arg = m.groups()
        r = {"kind": "check", "checks": [arg]} if kind == "Check" else {"kind": "data", "path": arg}
        rows[i] = dict(realisation=r, note="", entity="", clause=clause.replace("EN 1999-1-1 §", "").replace("EN 1999-1-", "1-") if False else clause)
    return rows


LEGACY = {"en1990": legacy_en1990, "en1991": legacy_en1991, "en1993": legacy_en1993, "en1995": legacy_en1995, "en1998": legacy_en1998, "en1999": legacy_en1999}
EXTRA_IDS = {"en1995": [("T-98", 381)]}
CASE_CODES = {"en1990", "en1991", "en1992", "en1995", "en1996", "en1997"}
ENTRY_RX = r'(?:\bid:\s*|entry_of\(\s*|Entry::new\(\s*|\bentry\(\s*)"([A-Z][A-Za-z0-9\-.]*)"'


def source_files(code):
    root = os.path.join(G.ART, G.FOLDERS[code])
    for dp, dn, fn in os.walk(root):
        if G.SKIP.search(dp + "/") or "🧭️spec-trace" in dp:
            continue
        for f in fn:
            if f.endswith(".rs"):
                yield os.path.join(dp, f)


def entry_ids(code):
    found = set()
    for p in source_files(code):
        found.update(re.findall(ENTRY_RX, read(p)))
    return found


def families(code):
    fams = collections.Counter()
    for p in source_files(code):
        for m in re.finditer(r'"(%s\.[A-Za-z0-9_.\-]*)' % code, read(p)):
            f = m.group(1).rstrip(".")
            if f.count(".") >= 2:
                fams[f] += 1
    return fams


def clause_tokens(clause):
    part = "2" if re.search(r"\b2:\d", clause) else "1-2" if re.search(r"\bF:\d", clause) else None
    plain = re.sub(r"\b(?:\d|F):(?=\d)", "", clause)
    plain = re.sub(r"EN\s+\d+(?:-\d+)*(?::\d+)?", "", plain)
    return part, [t.rstrip(".") for t in re.findall(r"\d+(?:\.\d+)+", plain)]


def clause_chain(family, code):
    rest = family[len(code) + 1:]
    chain = []
    for seg in rest.split("."):
        if re.fullmatch(r"\d+", seg) or (re.fullmatch(r"\d-\d", seg) and not chain):
            chain.append(seg)
        else:
            break
    return ".".join(chain)


def clause_match(code, clause, fams):
    part, toks = clause_tokens(clause)
    if "?" in clause:
        toks = [t for t in toks if clause.find(t + "?") < 0 and clause.find(t + ".?") < 0]
    exact, near, parent = [], [], []
    for f in sorted(fams):
        chain = clause_chain(f, code)
        if not chain:
            continue
        for t in toks:
            target = f"{part}.{t}" if part else t
            if chain == target:
                exact.append(f)
            elif chain.startswith(target + ".") and (target.count(".") >= 2 or chain.count(".") == target.count(".") + 1):
                near.append(f)
            elif target.startswith(chain + "."):
                parent.append((chain.count("."), f))
    if exact:
        return list(dict.fromkeys(exact))[:4], ""
    if near:
        return list(dict.fromkeys(near))[:4], ""
    if parent:
        deepest = max(depth for depth, _ in parent)
        return list(dict.fromkeys(f for depth, f in parent if depth == deepest))[:4], "the families cover the enclosing clause"
    return [], ""


def chk(*fragments):
    return {"kind": "check", "checks": list(fragments)}


def nm(reason):
    return {"kind": "not-modelled", "reason": reason}


READ = "mapped by reading the check ids the artifact emits"
MANUAL = {
    "en1998": {
        "SEI-02": (chk("en1998.site.aGr.derived", ".zone0"), "zone 0 and the derived reference ground acceleration; " + READ, "site"),
        "SEI-03": ({"kind": "data", "path": "part_1::elastic_response_spectrum"}, "kernel function; the spectrum is drawn by the scene diagrams and blessed by the oracle", ""),
        "SEI-04": ({"kind": "data", "path": "part_1::design_spectrum_horizontal"}, "kernel function; blessed by the oracle", ""),
        "SEI-05": ({"kind": "data", "path": "part_1::vertical_elastic_spectrum"}, "kernel function; drawn by the vertical spectrum diagram", ""),
        "SEI-20": (chk(".regularity.claim", ".regularity.derived", ".planDims", ".elevationRegularity.", ".planRegularity"), READ, "buildings"),
        "SEI-30": (chk(".qLimit", ".qDetail", ".system.na", ".multipleSystems"), READ, "buildings"),
        "SEI-35": (chk(".t1Given", ".t1Rayleigh"), READ, "buildings"),
        "SEI-40": (chk(".lfm"), READ, "buildings"),
        "SEI-41": (chk(".torsion"), READ, "buildings"),
        "SEI-42": ({"kind": "data", "path": "part_1::shear_building_modes and part_1::effective_modal_masses"}, "kernel functions; blessed by the oracle", ""),
        "SEI-43": (chk(".direction"), READ, "buildings"),
        "SEI-44": (chk(".pdelta."), READ, "buildings"),
        "SEI-45": (chk(".drift."), READ, "buildings"),
        "SEI-47": (chk(".joint.gap"), READ, "buildings"),
    },
    "en1994": {
        "C-17": ({"kind": "data", "path": "catalogue::stud_resistance_n and catalogue::sheeting_reduction"}, "k_t and k_l of profiled sheeting feed the stud resistance check en1994.6.6.3.1.prd", ""),
        "C-24": (chk("en1994.6.4.patch"), "patch loading with composite effect; " + READ, ""),
        "C-33": (chk("en1994.9.3.sheeting"), READ, "slabs"),
        "C-36": (chk("en1994.9.7.2.mrd"), "partial connection method shares the sagging resistance check of C-34; " + READ, "slabs"),
        "C-42": (chk("en1994.7.4"), READ, ""),
        "C-43": (chk("en1994.7.4.crack"), READ, ""),
        "C-48": (chk("en1994.2.lm1"), READ, ""),
    },
    "en1999": {
        "A-19": (chk("en1999.6.7.5.patch"), "web bearing under local loads, clause 6.7.5; " + READ, ""),
        "A-27": (chk("en1999.6.7.5.patch"), READ, ""),
        "A-81": (nm("the spec excludes it: the thermal expansion coefficient is a constant of 23e-6 and the creep of aluminium is negligible below 100 degrees Celsius"), "", ""),
    },
}


def build(code):
    ids = S.spec_ids(code)
    for extra_id, extra_line in EXTRA_IDS.get(code, []):
        at = next((k for k, (i, _) in enumerate(ids) if int(i.split("-")[1]) > int(extra_id.split("-")[1])), len(ids))
        ids.insert(at, (extra_id, extra_line))
    legacy = LEGACY.get(code, lambda: {})()
    entries = entry_ids(code) if code in CASE_CODES else set()
    fams = families(code)
    items, origin = [], collections.Counter()
    for idv, line in ids:
        clause = clause_of(code, idv, line)
        row = legacy.get(idv)
        if row:
            origin["legacy"] += 1
            r, note, entity = row["realisation"], row["note"], row["entity"]
            clause = row["clause"] or clause
        elif idv in entries:
            origin["case"] += 1
            r, note, entity = {"kind": "design-case", "entry": idv}, "", "cases"
        else:
            hit, parent = clause_match(code, clause, fams) if clause != "-" else ([], "")
            if idv in MANUAL.get(code, {}):
                origin["manual"] += 1
                r, note, entity = MANUAL[code][idv]
            elif hit:
                origin["clause"] += 1
                r, note, entity = {"kind": "check", "checks": hit}, "matched by clause number" + (f" ({parent})" if parent else "") + "; the artifact owner confirms the family realises this id", ""
            else:
                origin["unlocated"] += 1
                where = f"clause {clause}" if clause != "-" else "its heading"
                r = {"kind": "not-modelled", "reason": f"unlocated: no check family, data table or catalogue entry of the artifact cites {where} or names this id"}
                note, entity = "", ""
        items.append({"id": idv, "clause": clause, "entity": entity, "realisation": r, "note": note})
    extra = sorted(set(legacy) - {i for i, _ in ids})
    doc = {"code": code, "standard": STANDARD[code], "spec": "📐️specs/" + S.SPECS[code][0], "specIds": [i for i, _ in ids], "items": items}
    return doc, origin, extra


if __name__ == "__main__":
    codes = [a for a in sys.argv[1:] if not a.startswith("-")] or list(G.FOLDERS)
    for code in codes:
        doc, origin, extra = build(code)
        problems = G.verify(code, doc)
        kinds = collections.Counter(i["realisation"]["kind"] for i in doc["items"])
        unlocated = sum(1 for i in doc["items"] if i["realisation"].get("reason", "").startswith("unlocated"))
        print(code, len(doc["items"]), dict(kinds), "unlocated", unlocated, "origin", dict(origin), "legacy-not-in-spec", extra)
        for p in problems:
            print("   ", p)
        if "--write" in sys.argv:
            G.write(code, doc)
        if "--show" in sys.argv:
            for i in doc["items"]:
                r = i["realisation"]
                print("   ", i["id"], "|", i["clause"], "|", r["kind"], r.get("checks") or r.get("reason", "")[:60] or r.get("entry") or r.get("path"))
