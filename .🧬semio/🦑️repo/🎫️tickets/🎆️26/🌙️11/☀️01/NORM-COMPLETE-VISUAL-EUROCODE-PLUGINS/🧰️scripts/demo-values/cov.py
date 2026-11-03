import sys, re, glob
sys.path.insert(0, "."); from dv_lib import *
for code in DIRS:
    es = entries(code); ids = [e["id"] for e in es]
    feats = [p for p in glob.glob(str(art(code) / "🧪️tests/*atalogue*/🥒️.feature"))]
    body = "".join(open(p, encoding="utf8").read() for p in feats)
    cov = [i for i in ids if re.search(r"[\s|]" + re.escape(i) + r"\s*\|", body)]
    print(code, len(ids), "entries", len(cov), "covered", [pathlib.Path(p).parent.name for p in feats], "uncovered", [i for i in ids if i not in cov][:100])
