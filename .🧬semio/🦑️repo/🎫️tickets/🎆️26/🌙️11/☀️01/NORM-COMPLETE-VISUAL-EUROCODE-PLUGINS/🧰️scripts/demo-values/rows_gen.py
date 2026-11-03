"""Catalogue feature rows from the demo cases: for every registry entry not yet covered by the catalogue feature, the demo case inputs go through the independent Python oracle and its named results become `catalogue-example` rows (expected values are the oracle's, never the Rust's). Entries the oracle cannot evaluate, or whose inputs are vectors the example step cannot spell, are listed. Run with the repo venv python -I: rows_gen.py <code> [--write]."""
import glob, re, sys
sys.path.insert(0, "."); from dv_lib import *
import oracle_probe as P
code = sys.argv[1]; write = "--write" in sys.argv
annex = "de"
feature = next(pathlib.Path(p) for p in glob.glob(str(art(code) / "🧪️tests/*atalogue*/🥒️.feature")))
body = feature.read_text(encoding="utf8")
covered = {e["id"] for e in entries(code) if re.search(r"[\s|]" + re.escape(e["id"]) + r"\s*\|", body)}
doc = json.loads(demo_doc_path(code).read_text(encoding="utf-8"))
rows, skipped = [], {}
def fmt(x):
    return repr(float(x)).removesuffix(".0") if float(x) == int(x) and abs(x) < 1e15 else repr(float(x))
for case in doc["cases"]:
    entry = case["entryId"]
    if entry in covered: continue
    status, out = P.res.get(entry, ("?", None))
    if status != "OK":
        skipped[entry] = status; continue
    parts = [f'{n["id"]}={fmt(n["value"])}' for n in case["numbers"]] + [f'{t["id"]}:{t["value"]}' for t in case["texts"]]
    slug = re.sub(r"[^a-z0-9]+", "-", entry.lower()).strip("-")
    for key, value in out.items():
        value = float(value)
        if value != value or abs(value) == float("inf"): continue
        rows.append("      | %s-%s-%s | %s | %s | %s | %s | %.12g | 1e-5 |" % (slug, re.sub(r"[^a-z0-9]+", "-", key.lower()).strip("-") + ("-uc" if key != key.lower() else ""), annex, entry, annex, ";".join(parts), key, value))
print(code, len(rows), "rows for", len({r.split("|")[2] for r in rows}), "entries;", len(skipped), "skipped")
for e, why in skipped.items(): print("  skip", e, why)
if write and rows:
    ids = set(re.findall(r"\|\s*([a-z0-9-]+)\s*\|\s*" + ("E|M" if False else "") , body))
    lines = body.rstrip("\n").split("\n")
    last = max(i for i, line in enumerate(lines) if re.match(r"\s+\|.*\|\s*$", line))
    lines[last + 1 : last + 1] = rows
    feature.write_text("\n".join(lines) + "\n", encoding="utf8")
    print("written to", feature)
