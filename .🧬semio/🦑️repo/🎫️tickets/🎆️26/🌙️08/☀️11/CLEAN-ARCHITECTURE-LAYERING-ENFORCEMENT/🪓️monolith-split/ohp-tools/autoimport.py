"""📥 Repairs unresolved names (E0425/E0412/E0422/E0423/E0433 type or value) by inserting the single import rustc itself suggests into the failing repo file.
The oracle union maps every workspace crate to crate::__m_<pkg>; the suggestion is rewritten to `crate::` (same crate) or `<pkg>::` (other family crate). Suggestions through the stub layer (__ext) and ambiguous (several candidates) are skipped and reported.
usage: autoimport.py <family> <label> <cfg> [--apply] [--only SUBSTR] [--skip SUBSTR]"""
import json, os, re, sys, collections
ROOT = "/Users/ueli/Documents/semio"
GEN = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
a = sys.argv[1:]
fam, label, cfg = a[:3]
apply = "--apply" in a
only = a[a.index("--only") + 1] if "--only" in a else None
skip = a[a.index("--skip") + 1] if "--skip" in a else None
codes = {"E0425", "E0412", "E0422", "E0423"}
want = collections.OrderedDict()
skipped = collections.Counter()
for line in open(f"{GEN}/ohp-{fam}-{label}-cur-{cfg}-final.err", encoding="utf-8", errors="replace"):
    if not line.startswith("{"):
        continue
    try:
        d = json.loads(line)
    except Exception:
        continue
    if d.get("level") != "error" or (d.get("code") or {}).get("code") not in codes:
        continue
    pr = [s for s in d["spans"] if s["is_primary"]]
    if not pr or pr[0].get("expansion"):
        continue
    sug = []
    for ch in d.get("children", []):
        if re.match(r"^consider importing this (constant|function|struct|enum|type alias|trait|static|union)$", ch["message"]):
            for s in ch["spans"]:
                r = s.get("suggested_replacement")
                if r and r.startswith("use "):
                    sug.append(r.strip())
    if len(sug) != 1:
        skipped["ambiguous" if sug else "no-suggestion"] += 1
        continue
    u = sug[0]
    fn = pr[0]["file_name"]
    m = re.match(r"^__m_([A-Za-z0-9_]+)/(.*)$", fn)
    if not m:
        skipped["not-in-crate"] += 1
        continue
    pkg, rel = m.group(1), os.path.normpath(m.group(2))
    if "__ext" in u:
        skipped["via-stub"] += 1
        continue
    mm = re.match(r"^use crate::__m_([A-Za-z0-9_]+)::(.*);$", u)
    if not mm:
        skipped["not-crate-path"] += 1
        continue
    tgt, rest = mm.group(1), mm.group(2)
    new = f"use crate::{rest};" if tgt == pkg else f"use {tgt}::{rest};"
    if only and only not in rel: continue
    if skip and skip in rel: continue
    if not os.path.exists(os.path.join(ROOT, rel)):
        skipped["no-file"] += 1
        continue
    want.setdefault(rel, collections.OrderedDict()).setdefault(new, 0)
    want[rel][new] += 1
tot = 0
for rel, uses in want.items():
    p = os.path.join(ROOT, rel)
    txt = open(p, encoding="utf-8").read()
    todo = [u for u in uses if u not in txt]
    if not todo:
        continue
    lines = txt.split("\n")
    idx = None
    for i, l in enumerate(lines):
        if re.match(r"^use super::\*;\s*$", l):
            idx = i + 1; break
    if idx is None:
        for i, l in enumerate(lines):
            if re.match(r"^use [^;]+;\s*$", l):
                idx = i + 1
        if idx is None:
            skipped["no-anchor"] += 1
            continue
    print(f"{rel.split('artifacts/')[-1][-75:]}: +{len(todo)}")
    for u in todo[:6]:
        print("     ", u[:170])
    tot += len(todo)
    if apply:
        lines[idx:idx] = todo
        open(p, "w", encoding="utf-8").write("\n".join(lines))
        open(GEN + "/ohp-touched-imports.txt", "a", encoding="utf-8").write(rel + "\n")
print("total imports", tot, "skipped", dict(skipped), "APPLIED" if apply else "dry")
