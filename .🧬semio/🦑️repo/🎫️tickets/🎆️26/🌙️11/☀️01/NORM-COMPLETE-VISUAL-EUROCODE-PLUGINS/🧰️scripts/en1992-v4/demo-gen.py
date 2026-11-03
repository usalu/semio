"""Generates cases/🌱️demo/🦀️.rs and the demo-document fixture `cases` from cases/🌱️demo/🔣️.json and the input vocabulary. Run with python3 -I."""
import glob, json, pathlib, sys
ROOT = pathlib.Path("/home/deck/semio")
ART = next(ROOT.glob("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any"))
CHK = ART / "🧬️schema/💡️inferences/⚖️checks"
DEMO = CHK / "🗄️registry/🗂️cases/🌱️demo"
FIX = ART / "🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json"
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "catalogue-bridge-v2"))
import catalogue_doc as D
data = json.loads((DEMO / "🔣️.json").read_text(encoding="utf-8"))["entries"]
def f(v): return repr(float(v))
rows = []
for entry, e in data.items():
    nums = ", ".join(f'("{k}", {f(v)})' for k, v in e.get("numbers", {}).items())
    txts = ", ".join(f'("{k}", "{v}")' for k, v in e.get("texts", {}).items())
    vecs = ", ".join(f'("{k}", &[{", ".join(f(x) for x in v)}])' for k, v in e.get("vectors", {}).items())
    rows.append(f'    Demo {{ entry: "{entry}", numbers: &[{nums}], texts: &[{txts}], vectors: &[{vecs}] }},')
rs = '''//! 🌱️ Demo values of the design cases beyond the scalar examples of the vocabulary: the vector inputs, the text choices an evaluator reads without a default, and the entries whose key means something else than its vocabulary example. Seeded demo inputs of a plausible reinforced-concrete beam, not normative values. Generated from `./🔣️.json` by `🧰️scripts/en1992-v4/demo-gen.py`. See [`super`].

use semio_s_plugin_norm_compliance::design_case::CaseValues;

struct Demo {
    entry: &'static str,
    numbers: &'static [(&'static str, f64)],
    texts: &'static [(&'static str, &'static str)],
    vectors: &'static [(&'static str, &'static [f64])],
}

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
(DEMO / "🦀️.rs").write_text(rs, encoding="utf-8")
vocab = json.loads((CHK / "🗄️registry/📚️inputs/🔣️.json").read_text(encoding="utf-8"))
import re
src = "".join(p.read_text(encoding="utf-8") for p in CHK.rglob("🦀️.rs") if "🧪️tests" not in str(p) and "🗂️cases" not in str(p))
ents = [{"id": m.group(1), "en": json.loads('"%s"' % m.group(2)), "de": json.loads('"%s"' % m.group(3)), "inputs": re.findall(r'"([^"]+)"', m.group(4))} for m in re.finditer(r'entry_of\("([^"]+)", "[^"]*", "[^"]*", "((?:[^"\\]|\\.)*)", "((?:[^"\\]|\\.)*)", &\[([^\]]*)\]', src)]
reach = (CHK / "📋️member-evaluation/🧷️entity-cases/📍️reach/🦀️.rs").read_text(encoding="utf-8")
realised = set(re.findall(r'\("([^"]+)", Route::(?:Entity|Check)\(', reach))
cases = D.seed_cases(vocab, [e for e in ents if e["id"] not in realised], D.demo_id)
for case in cases:
    demo = data.get(case["entryId"], {})
    nums = {n["id"]: n["value"] for n in case["numbers"]}
    nums.update({k: float(v) for k, v in demo.get("numbers", {}).items()})
    for key, items in demo.get("vectors", {}).items():
        nums.update({f"{key}.{i}": float(x) for i, x in enumerate(items)})
    txts = {t["id"]: t["value"] for t in case["texts"]}
    txts.update(demo.get("texts", {}))
    case["numbers"] = [{"id": k, "value": nums[k]} for k in sorted(nums)]
    case["texts"] = [{"id": k, "value": txts[k]} for k in sorted(txts)]
doc = json.loads(FIX.read_text(encoding="utf-8"))
doc["cases"] = cases
FIX.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(len(ents), "entries,", sum(len(c["numbers"]) for c in cases), "number rows")
