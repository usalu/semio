"""🔍 c4p29 definition scan (the blind spot of the typeck oracle: privacy is under-reported while earlier errors exist).
For every moved child of a plan: each declaration WITHOUT a visibility (top-level fn/const/static/struct/enum/trait/type/union, brace-struct fields, inherent-impl members) whose name occurs as a raw identifier in ANY other file of the crate closure
(the other children, the root, every other file the lib root mounts, read unmasked: a superset of real uses) is a leftover. Trait-impl members are skipped (their visibility is the trait's). Independent of the widener, which works on masked tokens.
usage: c4pdefscan.py stage|live [key...]"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
import splitter as S
from plans import PLANS, ORDER
from c4psplit import lib_files
from rslex import lex, outline

which = "stage"
args = sys.argv[1:]
if args and args[0] in ("stage", "live"):
    which = args.pop(0)
keys = args or ORDER
BASE = STAGE if which == "stage" else ROOT
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
RX_FIELD = re.compile(r"^\s*(?!pub\b)(?:#\[[^\]]*\]\s*)*([A-Za-z_]\w*)\s*:(?!:)")
RX_MEM = re.compile(r"^\s*(?!pub\b)(?:(?:async|unsafe|const|default|extern(?: \"[^\"]*\")?)\s+)*(fn|const|type)\s+([A-Za-z_]\w*)")
TOP_KINDS = {"fn", "const", "static", "struct", "enum", "trait", "type", "union"}
bad = 0
scanned = 0
for key in keys:
    spec = PLANS[key]
    base_dir = os.path.dirname(spec["rel"])
    mine = {os.path.normpath(ROOT + "/" + spec["rel"])} | {os.path.normpath(f"{ROOT}/{base_dir}/{c['folder']}/🦀️.rs") for c in spec["children"]}
    others_files = [p for p in lib_files(spec["lib"]) if os.path.normpath(p) not in mine]
    texts = {}
    for p in others_files:
        texts[os.path.relpath(p, ROOT)] = read(p)
    texts[spec["rel"]] = read(f"{BASE}/{spec['rel']}")
    for c in spec["children"]:
        rel = f"{base_dir}/{c['folder']}/🦀️.rs"
        texts[rel] = read(f"{BASE}/{rel}")
    idents = {rel: set(IDENT.findall(S.mask(t))) for rel, t in texts.items()}
    leftovers = []
    for c in spec["children"]:
        if c.get("tree"):
            continue
        rel = f"{base_dir}/{c['folder']}/🦀️.rs"
        t = texts[rel]
        ls = t.split("\n")
        others = set()
        for r, s in idents.items():
            if r != rel:
                others |= s
        depths, inlit = lex(t)
        for it in outline(t):
            head = ls[it["start"]:it["end"] + 1]
            if it["kind"] in TOP_KINDS and it["name"] and not it["vis"]:
                scanned += 1
                if it["name"] in others:
                    leftovers.append((c["folder"], it["start"] + 1, "item", it["name"]))
            if it["kind"] == "struct" and "{" in "".join(head[:2]) and not re.search(r"struct\s+\w+(<[^>]*>)?\s*\(", "\n".join(head[:2])):
                for k in range(it["start"] + 1, it["end"] + 1):
                    if depths[k] == 1 and not inlit[k]:
                        m = RX_FIELD.match(ls[k])
                        if m and not ls[k].lstrip().startswith(("//", "#[")):
                            scanned += 1
                            if m.group(1) in others:
                                leftovers.append((c["folder"], k + 1, "field", m.group(1)))
            if it["kind"] == "impl":
                hd = " ".join(l.strip() for l in head[:6]).split("{")[0]
                if re.search(r"\bfor\b", hd):
                    continue
                for k in range(it["start"] + 1, it["end"] + 1):
                    if depths[k] == 1 and not inlit[k]:
                        m = RX_MEM.match(ls[k])
                        if m:
                            scanned += 1
                            if m.group(2) in others:
                                leftovers.append((c["folder"], k + 1, "member", m.group(2)))
    print(f"[{key}] children={len(spec['children'])} cross-file private declarations left: {len(leftovers)}")
    for lo in leftovers[:25]:
        print("   LEFTOVER", lo)
    bad += len(leftovers)
print(f"DEFSCAN scanned={scanned} leftovers={bad}")
sys.exit(1 if bad else 0)
