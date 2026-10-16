"""🔎 Definition scan: private fields, methods, associated items and free items of one crate that another crate's sources mention (cross-crate privacy that the resolution oracle cannot see)."""
import os, re, sys
from lib import *
from model import *
from synth import mounts, code_idents
from union import owned_files
from rsitems import split_items

def crate_files(tree, key):
    return sorted(set(owned_files(tree, CRATES[key]["node"] + "/🦀️.rs")))

def struct_private_fields(text):
    out = {}
    for it in split_items(text):
        if it.kind in ("struct", "union"):
            m = re.search(r"\{(.*)\}", it.text, re.S)
            if not m or "(" in it.text.split("{")[0][-3:]:
                pass
            body = m.group(1) if m else ""
            depth = 0
            cur = ""
            fields = []
            for ch in body:
                if ch in "<([{":
                    depth += 1
                elif ch in ">)]}":
                    depth -= 1
                if ch == "," and depth == 0:
                    fields.append(cur)
                    cur = ""
                else:
                    cur += ch
            if cur.strip():
                fields.append(cur)
            for f in fields:
                f = re.sub(r"(//[^\n]*|#\[[^\]]*\])", "", f).strip()
                mm = re.match(r"^(pub(?:\([^)]*\))?\s+)?(\w+)\s*:", f)
                if mm and not mm.group(1):
                    out.setdefault(it.name, []).append(mm.group(2))
                elif mm and mm.group(1) and "(" in mm.group(1):
                    out.setdefault(it.name, []).append(mm.group(2))
            if re.match(r"^\s*(?:pub\s+)?struct\s+\w+\s*\(", re.sub(r"(#\[[^\]]*\]|//[^\n]*)\n", "", it.text)):
                pass
    return out

def impl_private_items(text):
    out = {}
    for it in split_items(text):
        if it.kind == "impl":
            head = re.sub(r"\s+", " ", it.text.split("{")[0])
            if " for " in head:
                continue
            tm = re.search(r"impl(?:<[^>]*>)?\s+(\w+)", head)
            tname = tm.group(1) if tm else "?"
            inner = it.text[it.text.index("{") + 1: it.text.rindex("}")]
            for sub in split_items(inner):
                if sub.kind in ("fn", "const", "type"):
                    stripped = re.sub(r"(?m)^\s*(///[^\n]*|#\[[^\n]*\])\n", "", sub.text)
                    if not re.match(r"\s*pub\b", stripped) or re.match(r"\s*pub\s*\(", stripped):
                        out.setdefault(tname, []).append(sub.name)
    return out

def scan(tree, keys, verbose=True):
    texts = {k: {f: rd(tree + "/" + f) for f in crate_files(tree, k)} for k in keys}
    hits = []
    for d in keys:
        priv_fields, priv_items, priv_free = {}, {}, set()
        for f, t in texts[d].items():
            for s, fl in struct_private_fields(t).items():
                priv_fields.setdefault(s, set()).update(fl)
            for s, fl in impl_private_items(t).items():
                priv_items.setdefault(s, set()).update(fl)
            for n in private_names(t):
                priv_free.add(n)
        for c in keys:
            if c == d:
                continue
            for f, t in texts[c].items():
                code = re.sub(r"//[^\n]*", "", t)
                for s, fl in priv_fields.items():
                    for fld in fl:
                        if re.search(r"\." + fld + r"\b(?!\s*\()", code) or re.search(r"[{,]\s*" + fld + r"\s*:(?!:)", code):
                            hits.append(("field", d, s, fld, c, f))
                for s, fl in priv_items.items():
                    for it in fl:
                        if re.search(r"\." + it + r"\s*\(", code) or re.search(r"\b" + s + r"::" + it + r"\b", code):
                            hits.append(("method", d, s, it, c, f))
                ids = code_idents(t)
                for n in priv_free:
                    if n in ids:
                        hits.append(("free", d, "-", n, c, f))
    return hits

if __name__ == "__main__":
    tree = sys.argv[1]
    keys = sys.argv[2:]
    for h in scan(tree, keys):
        print(*h)
