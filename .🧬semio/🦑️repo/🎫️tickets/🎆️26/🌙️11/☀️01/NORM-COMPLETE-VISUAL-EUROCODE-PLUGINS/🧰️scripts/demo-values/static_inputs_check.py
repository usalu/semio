"""Static proof that the demo cases carry every input the evaluators read with `c.n("key")?` / `c.s("key")?` / `c.pos("key")?` / `c.v("key")?` (the Rust cannot be run). Run with python3 -I static_inputs_check.py <code> [more codes]; lists entry, missing keys. Heuristic: direct reads in the evaluator body and in the helper fns it calls by name in the same file."""
import re, sys
sys.path.insert(0, "."); from dv_lib import *
HELPERS = {"Mat::load": ["class"], "Sec::load": ["b", "h"], "parts_from": ["a1", "i1", "e1", "h1", "s1", "a2", "i2", "e2", "h2"]}
for code in sys.argv[1:]:
    doc = json.loads(demo_doc_path(code).read_text(encoding="utf-8"))
    cases = {c["entryId"]: ({n["id"].split(".")[0] for n in c["numbers"]}, {t["id"] for t in c["texts"]}) for c in doc["cases"]}
    srcs = {p: p.read_text(encoding="utf-8") for p in registry_files(code)}
    allsrc = "\n".join(srcs.values())
    for e in entries(code):
        m = re.search(r'(?:Entry \{ id: |entry_of\()\s*"%s",[^\n]*?(?:eval: |, )(\w+)\s*[}\)]' % re.escape(e["id"]), allsrc)
        if not m: continue
        fm = re.search(r"^(?:pub )?fn %s\(.*?^}\n" % m.group(1), allsrc, re.S | re.M)
        if not fm: continue
        body = fm.group(0)
        nums = set(re.findall(r'\bc\.(?:n|pos|v)\(\s*"([a-z0-9_]+)"\s*\)\s*\?', body)); texts = set(re.findall(r'\bc\.s\(\s*"([a-z0-9_]+)"\s*\)\s*\?', body))
        for h, ks in HELPERS.items():
            if h in body: (texts if h == "Mat::load" else nums).update(ks)
        hn, ht = cases.get(e["id"], (set(), set()))
        missing = sorted([k for k in nums if k not in hn] + ["text:" + k for k in texts if k not in ht])
        if missing and e["id"] in cases: print(code, e["id"], "missing", missing)
