#!/usr/bin/env python3
"""Language-agnostic gate: python3 -I check.py  -> exit 0 only when no MediaPortSpec label and no terminology_document segment is a plain string
and every case row in localized-manifest-cases.json has two distinct-or-cognate non-blank cells. Stdlib only; run after localize.py --write."""
import json,re,os,sys
import os as _o
R=_o.environ.get("A11Y_ROOT","/Users/ueli/Documents/semio/"); H=os.path.dirname(os.path.abspath(__file__))
cases=json.load(open(H+"/localized-manifest-cases.json")); bad=[]
rows=[c for segs in cases["breadcrumbs"]["reuse"].values() for c in segs]+cases["ports"]
for c in rows:
    if not c.get("en","").strip() or not c.get("de","").strip(): bad.append(("blank",c))
for r in cases["resolve"]:
    segs=cases["breadcrumbs"][r["terminology"]][r["app"]]
    if " · ".join(s[r["locale"]] for s in segs)!=r["breadcrumb"]: bad.append(("resolve",r))
for f in os.popen(f"cd {R!r} && command grep -rl 'MediaPortSpec\\|terminology_document(' --include='*.rs' 🧰️framework ✏️s 2>/dev/null").read().split("\n"):
    if not f: continue
    s=open(R+f).read()
    for m in re.finditer(r'MediaPortSpec\s*\{[^{}]*?\blabel:\s*"[^"]*"',s): bad.append(("plain-port-label",f,m.group(0)[-40:]))
    for m in re.finditer(r'MediaPortSpec\s*\{[^{}]*?\blabel:\s*[A-Za-z_][A-Za-z0-9_]*\.into\(\)',s): bad.append(("non-literal-port-label",f,m.group(0)[-40:]))
    for m in re.finditer(r'\.terminology_document\("[^"]+",\s*\[\s*"',s): bad.append(("plain-breadcrumb",f))
for f in os.popen(f"cd {R!r} && command grep -rl 'Locale::En\\|Locale::De' --include='*.rs' 🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow 2>/dev/null").read().split("\n"):
    if not f or "🧪️tests" in f: continue
    for m in re.finditer(r'\blabel\s*:\s*[\w.]+\.resolve\(\s*[\w:]*Terminology::\w+\s*,\s*[\w:]*Locale::(?:En|De)\s*\)\.to_string\(\)',open(R+f).read()): bad.append(("fixed-locale-label-stored-in-workflow",f,m.group(0)))
print("violations:",len(bad)); [print(b) for b in bad[:int(os.environ.get("A11Y_LIST","40"))]]; sys.exit(1 if bad else 0)
