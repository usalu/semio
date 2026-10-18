"""🌾 OHF harvest: reads pos result JSON (GEN/ohr-<family>-<label>-result.json) and prints the NON-STUB rows of the target error codes as TSV
(code, cfg, crate, repo-relative file, line, normalised message, offending source text). Read only; writes GEN/ohr-rows-<family>-<label>.tsv.
usage: harvest.py <family> <label> [--codes E0425,...] [--all]"""
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.dirname(HERE) + "/🗑️generated"
TARGET = "E0425 E0432 E0433 E0412 E0599 E0061 E0308 E0603 E0616 E0624 E0107 E0282 E0451".split()
a = sys.argv[1:]
fam, label = a[0], a[1]
codes = TARGET
if "--codes" in a:
    codes = a[a.index("--codes") + 1].split(",")
r = json.load(open(f"{GEN}/ohr-{fam}-{label}-result.json", encoding="utf-8"))
rows = []
for cfg, e in r["summary"].items():
    for d in e.get("new_list", []):
        if d["stub"] and "--all" not in a:
            continue
        if "--all" not in a and d["code"] not in codes:
            continue
        m = re.match(r"^__m_([A-Za-z0-9_]+)/(.*)$", d["file"])
        pkg, rel = (m.group(1), m.group(2)) if m else ("", d["file"])
        msg = re.sub(r"\s+", " ", d["msg"])
        rows.append((d["code"], cfg, pkg, rel, d["line"], msg, (d.get("line_text") or "").strip()[:160], d.get("label", "")[:100]))
rows.sort(key=lambda x: (x[2], x[3], x[4], x[0]))
out = f"{GEN}/ohr-rows-{fam}-{label}.tsv"
with open(out, "w", encoding="utf-8") as f:
    for x in rows:
        f.write("\t".join(str(y) for y in x) + "\n")
c = collections.Counter(x[0] for x in rows)
print(len(rows), dict(c), out)
