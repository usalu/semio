"""🔎 c4p38 definition scan on the LIVE mod-mode children: every private (no `pub`) top-level fn/struct/enum/const/static/type/trait and every private inherent-impl member or struct field whose name another file of the same crate closure mentions must be flagged.
Over-approximation by name (like the splitter). Expected: 0 hits, because the splitter widened exactly those. usage: c4p38defscan.py"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4p38lib import *
from plans38 import PLANS, ORDER
import splitter as S
from rslex import outline, lex
from c4p38 import crate_files_from

hits = 0
for key in ORDER:
    spec = PLANS[key]
    if spec["mode"] != "mod":
        continue
    base_dir = os.path.dirname(spec["rel"])
    kids = {f"{ROOT}/{base_dir}/{c['folder']}/🦀️.rs" for c in spec["children"]}
    files = crate_files_from(spec["lib"], None)
    toks, fields, methods = {}, {}, {}
    for p in files:
        t = read(p)
        f_, a_ = S.tokenize(t)
        toks[p] = a_
        fl, me = S.member_refs(t)
        fields[p], methods[p] = fl, me
    for p in sorted(kids):
        text = read(p).replace("pub(super) ", "") if os.environ.get("C4P38_STRIP") else read(p)
        depths, _ = lex(text)
        ext_all = set().union(*[toks[q] for q in files if q != p])
        ext_f = set().union(*[fields[q] for q in files if q != p])
        ext_m = set().union(*[methods[q] for q in files if q != p])
        ls = text.split("\n")
        for it in outline(text):
            if it["kind"] in ("fn", "struct", "enum", "trait", "const", "static", "type", "union") and not it["vis"] and it["name"] and it["name"] in ext_all:
                hits += 1
                print(f"[{key}] PRIVATE-USED-ELSEWHERE {it['kind']} {it['name']} in {os.path.relpath(p, ROOT)}:{it['start'] + 1}")
            if it["kind"] == "struct":
                for k in range(it["start"], it["end"] + 1):
                    m = re.match(r"^\s+(?!pub\b)(?:#\[[^\]]*\]\s*)?([A-Za-z_]\w*)\s*:", ls[k]) if depths[k] == 1 else None
                    if m and m.group(1) in ext_f and not ls[k].strip().startswith(("//", "#[")):
                        hits += 1
                        print(f"[{key}] PRIVATE-FIELD-USED-ELSEWHERE {it['name']}.{m.group(1)} in {os.path.relpath(p, ROOT)}:{k + 1}")
            if it["kind"] == "impl" and " for " not in ls[it["start"]]:
                for k in range(it["start"], it["end"] + 1):
                    if depths[k] != 1:
                        continue
                    m = re.match(r"^\s+(?!pub\b)((?:const\s+|async\s+|unsafe\s+)*fn\s+(\w+))", ls[k])
                    if m and m.group(2) in ext_m:
                        hits += 1
                        print(f"[{key}] PRIVATE-METHOD-USED-ELSEWHERE {m.group(2)} in {os.path.relpath(p, ROOT)}:{k + 1}")
print("DEFSCAN hits", hits)
