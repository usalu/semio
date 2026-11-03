# 🧭️ Spec-trace twin generator and verifier. `python -I 🐍️.py` validates every artifact `🧭️spec-trace/🔣️.json` against the shared schema (third-party `jsonschema`), checks spec order, completeness and that every check fragment occurs in the artifact sources, and fails when a `🦀️.rs` twin is stale; `--write` regenerates the twins.
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
NORM = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
ART = os.path.join(NORM, "🗿️artifacts")
SCHEMA = os.path.join(HERE, "..", "🧬️schema", "🔣️.json")
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any"
FOLDERS = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1993": "🔩️en1993", "en1994": "🧩️en1994", "en1995": "🪵️en1995", "en1996": "🪨️en1996", "en1997": "🌍️en1997", "en1998": "🫨️en1998", "en1999": "🪶️en1999"}
SKIP = re.compile(r"🧬️mutations|/target/|🧪️tests|/tests/|🔮️|🧫️|🧰️|🥒️|(^|/)(🎬️[^/]*|✏️editor|👁️viewer|🚪️io|🖼️assets|📚️examples|🏷️field-meta|🌱️demo|🧭️spec-trace)/")
ENTRY = re.compile(r'(?:\bEntry\s*\{\s*id:|\bentry_of\(|\bentry\()\s*"([A-Za-z][A-Za-z0-9]*-[A-Za-z0-9.]+)"')
LITERAL = re.compile(r'"((?:[^"\\\n]|\\.)*)"')
ENTRY_SHAPE = re.compile(r"[a-z]{1,4}-\d+[a-z]?")


def trace_dir(code):
    return os.path.join(ART, FOLDERS[code], ANY, "🧭️spec-trace")


def lit(text):
    return json.dumps(text, ensure_ascii=False)


def text_rust(t):
    return f"SpecText::new({lit(t['en'])}, {lit(t['de'])})"


def realisation_rust(r):
    k = r["kind"]
    if k == "check":
        return "Realisation::Check(&[" + ", ".join(lit(c) for c in r["checks"]) + "])"
    if k == "data":
        return "Realisation::Data(" + lit(r["path"]) + ")"
    if k == "design-case":
        return "Realisation::DesignCase(" + lit(r["entry"]) + ")"
    return "Realisation::NotModelled(" + text_rust(r["reason"]) + ")"


def render_rust(doc):
    out = [
        "//! 🧭️ Generated from `🔣️.json` by `⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py`: edit the JSON, never this file. See [`semio_s_plugin_norm_compliance::spec_trace`].",
        "",
        "use semio_s_plugin_norm_compliance::spec_trace::{Realisation, SpecText, SpecTrace, SpecTraceTable};",
        "",
        f"/// 🧭️ Traceability of the {doc['standard']} target spec: {len(doc['items'])} ids, each with its clause, entity and realisation.",
        f"pub const TABLE: SpecTraceTable = SpecTraceTable {{ code: {lit(doc['code'])}, standard: {lit(doc['standard'])}, spec: {lit(doc['spec'])}, spec_ids: SPEC_IDS, items: ITEMS }};",
        "",
        "const SPEC_IDS: &[&str] = &[" + ", ".join(lit(i) for i in doc["specIds"]) + "];",
        "",
        "const ITEMS: &[SpecTrace] = &[",
    ]
    for it in doc["items"]:
        out.append(f"    SpecTrace::new({lit(it['id'])}, {lit(it['clause'])}, {lit(it['entity'])}, {realisation_rust(it['realisation'])}, {text_rust(it['description'])}),")
    out.append("];")
    return "\n".join(out) + "\n"


def files(code):
    root = os.path.join(ART, FOLDERS[code])
    found = []
    for dp, dn, fn in os.walk(root):
        if SKIP.search(os.path.relpath(dp, ART) + "/"):
            continue
        for f in fn:
            if f.endswith(".rs"):
                found.append(open(os.path.join(dp, f), encoding="utf-8", errors="replace").read())
    return found


def sources(code):
    return "\n".join(files(code))


def catalogue(text):
    return {m.upper() for m in ENTRY.findall(text)}


def bridge_wired(code, text):
    return re.search(r'format!\("' + code + r'\.\{\}\.[^"]*",\s*e\.id\.to_ascii_lowercase\(\)', text) is not None and "CaseCatalogue for" in text


def literals(text):
    return LITERAL.findall(text)


def token(tok, lits):
    pattern = re.compile(r"(?<![A-Za-z0-9])" + re.escape(tok) + r"(?![A-Za-z0-9])")
    return any(pattern.search(lit) for lit in lits)


def composed(code, c, texts):
    parts = c.rstrip(".").split(".")
    tail, mid = ".".join(parts[1:]), ".".join(parts[1:-1])
    for text in texts:
        lits = literals(text)
        if any(c.rstrip(".") in lit for lit in lits) or token(tail, lits):
            return True
        if token(parts[-1], lits) and (not mid or token(mid, lits)):
            return True
    return False


def emitted(code, c, text, texts=None):
    parts = c.rstrip(".").split(".")
    entries = catalogue(text)
    for at in (0, 1):
        if len(parts) > at and (at == 0 or parts[0] == code) and (parts[at].upper() in entries or ENTRY_SHAPE.fullmatch(parts[at])):
            rest = parts[at + 1:]
            return parts[at].upper() in entries and bridge_wired(code, text) and all(f'"{seg}"' in text for seg in rest)
    return composed(code, c, [text] if texts is None else texts)


def verify(code, doc, text=None):
    import jsonschema
    schema = json.load(open(SCHEMA, encoding="utf-8"))
    jsonschema.Draft7Validator(schema).validate(doc)
    problems = []
    ids = [i["id"] for i in doc["items"]]
    if ids != [i for i in doc["specIds"] if i in set(ids)]:
        problems.append("items are not in spec order")
    if sorted(ids) != sorted(doc["specIds"]):
        problems.append(f"untraced {sorted(set(doc['specIds']) - set(ids))} unknown {sorted(set(ids) - set(doc['specIds']))}")
    for it in doc["items"]:
        for t in [it["description"]] + ([it["realisation"]["reason"]] if it["realisation"]["kind"] == "not-modelled" else []):
            if t["en"] == t["de"]:
                problems.append(f"{it['id']}: en and de text are identical")
        r = it["realisation"]
        if r["kind"] == "not-modelled" and any(len(r["reason"][k].strip()) < 20 for k in ("en", "de")):
            problems.append(f"{it['id']}: reason shorter than 20 characters")
    texts = files(code)
    text = "\n".join(texts) if text is None else text
    for it in doc["items"]:
        r = it["realisation"]
        if r["kind"] == "check":
            for c in r["checks"]:
                if not emitted(code, c, text, texts):
                    problems.append(f"{it['id']}: fragment {c} not in the artifact sources")
    return problems


def write(code, doc):
    d = trace_dir(code)
    os.makedirs(d, exist_ok=True)
    with open(os.path.join(d, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    with open(os.path.join(d, "🦀️.rs"), "w", encoding="utf-8") as f:
        f.write(render_rust(doc))


if __name__ == "__main__":
    bad = 0
    for code in FOLDERS:
        p = os.path.join(trace_dir(code), "🔣️.json")
        if not os.path.exists(p):
            print(code, "missing")
            bad += 1
            continue
        doc = json.load(open(p, encoding="utf-8"))
        problems = verify(code, doc)
        if "--write" in sys.argv:
            write(code, doc)
        elif open(os.path.join(trace_dir(code), "🦀️.rs"), encoding="utf-8").read() != render_rust(doc):
            problems.append("🦀️.rs twin is stale")
        print(code, len(doc["items"]), "ok" if not problems else problems)
        bad += bool(problems)
    sys.exit(1 if bad else 0)
