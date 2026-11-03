"""Lists the registry entries without a row in the catalogue feature of each code. Run with python3 -I coverage_rows.py."""
import glob, re, sys
sys.path.insert(0, "."); from dv_lib import *
for code in ["en1990", "en1991", "en1995", "en1996", "en1997"]:
    body = open(glob.glob(str(art(code) / "🧪️tests/*atalogue*/🥒️.feature"))[0], encoding="utf8").read()
    print(code, len(entries(code)), "without row:", [e["id"] for e in entries(code) if not re.search(r"[\s|]" + re.escape(e["id"]) + r"\s*\|", body)])
