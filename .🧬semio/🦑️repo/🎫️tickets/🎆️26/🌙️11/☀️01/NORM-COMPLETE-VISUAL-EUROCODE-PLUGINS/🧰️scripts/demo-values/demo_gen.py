"""Generates, per code, cases/🌱️demo/🔣️.json (demo values beyond the vocabulary examples), cases/🌱️demo/🦀️.rs and the `cases` of the committed catalogue demo document from the value tables `values_<code>.py`, the committed catalogue fixture (EN 1990, 1995, 1997) and the input vocabulary. Run with python3 -I demo_gen.py <code> [--check]."""
import importlib, json, pathlib, re, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from dv_lib import *
import catalogue_doc_local as D

code = sys.argv[1]
check = "--check" in sys.argv
CHANGED = []


def emit(path, text):
    old = path.read_text(encoding="utf-8") if path.exists() else None
    if old != text:
        CHANGED.append(path)
        if not check:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8")


def union(old, new):
    """Additive union: every existing key/row/entry keeps its value, only what is missing is added."""
    if isinstance(old, dict) and isinstance(new, dict):
        return {**{k: union(v, new[k]) if k in new else v for k, v in old.items()}, **{k: v for k, v in new.items() if k not in old}}
    return old

T = importlib.import_module("values_" + code)
A = art(code)
es, v = entries(code), vocab(code)
CASES_DIR = {"en1990": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases", "en1991": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases", "en1995": "🧬️schema/⚖️kernel/📇️catalogue/🗂️cases", "en1996": "🧬️schema/📇️masonry-catalogue/🗂️cases", "en1997": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases"}[code]
DEMO = A / CASES_DIR / "🌱️demo"
FIX = demo_doc_path(code)
fixture_first = getattr(T, "FIXTURE", False)
fx = {}
fxp = A / "🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json"
if fixture_first and fxp.exists():
    raw = json.loads(fxp.read_text(encoding="utf-8"))
    for item in (raw if isinstance(raw, list) else raw["cases"]):
        body = item.get("case", item)
        annex = body.get("annex", "En")
        if item["entry"] not in fx or (annex == "De" and fx[item["entry"]][0] != "De"):
            fx[item["entry"]] = (annex, body.get("num", {}), body.get("text", {}))
series_bases = {r["key"] for r in v["inputs"] if r["kind"] == "series"}

def lookup(pool, entry, key, scope, generic=True):
    for k in (f"{entry}:{key}", f"{key}@{entry}", f"{key}@{scope}") + ((key,) if generic else ()):
        if k in pool:
            return pool[k]
    return None

data, missing = {}, []
for e in es:
    nums, txts, vecs = {}, {}, {}
    extra = getattr(T, "EXTRA", {}).get(e["id"], {})
    fnum, ftxt = (fx.get(e["id"]) or ("", {}, {}))[1:]
    roots = {re.sub(r"\d+$", "", i) for i in e["inputs"]}
    if e["id"] in fx:
        nums.update({k: float(x) for k, x in fnum.items() if k in e["inputs"] or re.sub(r"\d+$", "", k) in roots})
        txts.update({k: str(x) for k, x in ftxt.items() if k in e["inputs"]})
        skipped = [k for k in list(fnum) + list(ftxt) if k not in nums and k not in txts]
        if skipped:
            print("  fixture keys outside the registry inputs of", e["id"], skipped)
    for k in ([] if e["id"] in fx else e["inputs"]):
        r = spec(v, e["id"], k)
        if r is None:
            missing.append((e["id"], k, "no vocabulary row"))
            continue
        kind, sc = r["kind"], r.get("scope", "")
        rootkey = re.sub(r"\d+$", "", k)
        if kind == "series":
            mine = {n: x for n, x in fnum.items() if re.fullmatch(re.escape(rootkey) + r"\d+", n)}
            if mine:
                nums.update({n: float(x) for n, x in mine.items()})
            elif "example" not in r:
                missing.append((e["id"], k, "series"))
            continue
        if k in extra:
            continue
        if kind == "vector":
            val = lookup(T.VECTORS, e["id"], k, sc, "example" not in r)
            if val is not None:
                vecs[k] = [float(x) for x in val]
            elif "example" not in r:
                missing.append((e["id"], k, "vector"))
        elif kind == "number":
            val = fnum.get(k) if k in fnum else lookup(T.V, e["id"], k, sc, "example" not in r)
            if val is not None:
                nums[k] = float(val)
            elif "example" not in r:
                missing.append((e["id"], k, "number"))
        else:
            val = ftxt.get(k) if k in ftxt else lookup(getattr(T, "TEXTS", {}), e["id"], k, sc, "example" not in r)
            if val is not None:
                txts[k] = str(val)
            elif "example" not in r:
                missing.append((e["id"], k, kind))
    for k, x in extra.items():
        nums[k] = float(x)
    out = {}
    for k, x in sorted(nums.items()):
        r = spec(v, e["id"], k)
        if r is None or r.get("example") != x or r["kind"] != "number":
            out.setdefault("numbers", {})[k] = x
    for k, x in sorted(txts.items()):
        r = spec(v, e["id"], k)
        if r is None or r.get("example") != x:
            out.setdefault("texts", {})[k] = x
    if vecs:
        out["vectors"] = vecs
    if out:
        data[e["id"]] = out
if missing:
    print("UNRESOLVED", len(missing))
    for m in missing:
        print("  ", *m)
note = f"Seeded demo inputs beyond the vocabulary examples of a plausible {code.upper()} design situation, not normative. Sole source of 🌱️demo/🦀️.rs and the catalogue demo-document fixture (generated by 🧰️scripts/demo-values/demo_gen.py)."
prev = DEMO / "🔣️.json"
if prev.exists():
    kept = json.loads(prev.read_text(encoding="utf-8")).get("entries", {})
    data = {k: union(kept[k], data[k]) if k in kept and k in data else kept.get(k, data.get(k)) for k in list(dict.fromkeys(list(data) + list(kept)))}
emit(DEMO / "🔣️.json", json.dumps({"note": note, "entries": data}, indent=1, ensure_ascii=False) + "\n")
def f(x): return repr(float(x))
rows = []
for entry, e in data.items():
    nums = ", ".join(f'("{k}", {f(x)})' for k, x in e.get("numbers", {}).items())
    txts = ", ".join(f'("{k}", "{x}")' for k, x in e.get("texts", {}).items())
    vecs = ", ".join(f'("{k}", &[{", ".join(f(x) for x in xs)}])' for k, xs in e.get("vectors", {}).items())
    rows.append(f'    Demo {{ entry: "{entry}", numbers: &[{nums}], texts: &[{txts}], vectors: &[{vecs}] }},')
rs = f'''//! 🌱️ Demo values of the design cases beyond the vocabulary examples: the numbers, text choices and vectors an evaluator reads without a default. Seeded demo inputs of a plausible {code.upper()} design situation, not normative values. Generated from `./🔣️.json` by `🧰️scripts/demo-values/demo_gen.py`. See [`super`].

use semio_s_plugin_norm_compliance::design_case::CaseValues;

struct Demo {{
    entry: &'static str,
    numbers: &'static [(&'static str, f64)],
    texts: &'static [(&'static str, &'static str)],
    vectors: &'static [(&'static str, &'static [f64])],
}}

const DEMOS: &[Demo] = &[
''' + "\n".join(rows) + '''
];

/// 🌱️ The demo values of one catalogue entry on top of the vocabulary examples.
pub fn demo_values(entry_id: &str, values: CaseValues) -> CaseValues {
    let Some(demo) = DEMOS.iter().find(|demo| demo.entry == entry_id) else { return values };
    let values = demo.numbers.iter().fold(values, |values, (key, value)| values.with_number(key, *value));
    let values = demo.texts.iter().fold(values, |values, (key, value)| values.with_text(key, value));
    demo.vectors.iter().fold(values, |values, (key, items)| values.with_vector(key, items))
}
'''
emit(DEMO / "🦀️.rs", rs)
cases = D.seed_cases(v, es, D.demo_id)
for case in cases:
    demo = data.get(case["entryId"], {})
    nums = {n["id"]: n["value"] for n in case["numbers"]}
    nums.update({k: float(x) for k, x in demo.get("numbers", {}).items()})
    for key, items in demo.get("vectors", {}).items():
        nums.update({f"{key}.{i}": float(x) for i, x in enumerate(items)})
    txts = {t["id"]: t["value"] for t in case["texts"]}
    txts.update(demo.get("texts", {}))
    case["numbers"] = [{"id": k, "value": nums[k]} for k in sorted(nums)]
    case["texts"] = [{"id": k, "value": txts[k]} for k in sorted(txts)]
if FIX.exists():
    doc = json.loads(FIX.read_text(encoding="utf-8"))
else:
    doc = json.loads((A / "🧪️tests/🧫️fixtures/🧬️mutations/🕳️change-cutout-face/✅apply/🔣️.before.json").read_text(encoding="utf-8"))
old_cases = {c.get("id", c["entryId"]): c for c in doc.get("cases", [])}
merged = []
for case in cases:
    key = case.get("id", case["entryId"])
    have = old_cases.pop(key, None)
    if have:
        for rows in ("numbers", "texts"):
            ids = {r["id"] for r in have.get(rows, [])}
            have[rows] = have.get(rows, []) + [r for r in case[rows] if r["id"] not in ids]
        case = have
    merged.append(case)
doc["cases"] = merged + list(old_cases.values())
emit(FIX, json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
for path in CHANGED:
    print("would change" if check else "changed", path)
if check and (CHANGED or missing):
    sys.exit(1)
print(code, len(es), "entries,", len(data), "with demo values,", sum(len(c["numbers"]) + len(c["texts"]) for c in cases), "value rows,", len(missing), "unresolved")
