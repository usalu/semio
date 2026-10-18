"""📍 OHR locator: package ident or name -> crate directory (repo relative) and the row file path under it. usage: where.py <pkg_ident> [<sandbox-relative-file>]"""
import json, os, sys
GEN = os.path.dirname(os.path.dirname(os.path.abspath(__file__))) + "/🗑️generated"
idx = json.load(open(GEN + "/ohr-manifest-index.json", encoding="utf-8"))
by = {n.replace("-", "_"): p for n, p in idx.items()}
k = sys.argv[1]
if k.startswith("__m_"):
    k = k[4:]
p = by.get(k)
print(os.path.dirname(p) if p else "NOT FOUND")
