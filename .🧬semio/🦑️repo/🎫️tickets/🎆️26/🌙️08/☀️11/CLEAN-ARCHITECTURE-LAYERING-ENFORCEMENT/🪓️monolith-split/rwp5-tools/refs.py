# 🔧️ Reference rewrites after the bundle conversion: self-less associated functions called as free functions, and the visibility widening that the new module boundaries demand.
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, rsedit, ctx, deps
from rsedit import Doc
from blank import blank

def all_rs(root, include_tests=True):
    base = f"{root}/{cfg.TGT}"
    for dp, dn, fn in os.walk(f"{root}"):
        dn[:] = [d for d in dn if d != "node_modules" and (include_tests or d != "🧪️tests")]
        for f in fn:
            if f.endswith(".rs"): yield os.path.join(dp, f)

def rewrite_selfless(root, names, report):
    if not names: return 0
    alt = "|".join(sorted(map(re.escape, names), key=len, reverse=True))
    rx_state = re.compile(r"(?<![A-Za-z_0-9])((?:crate::shell::|super::)?)ShellState\s*::\s*(" + alt + r")(?![A-Za-z_0-9])")
    rx_self = re.compile(r"(?<![A-Za-z_0-9])Self\s*::\s*(" + alt + r")(?![A-Za-z_0-9])")
    n = 0
    for p in all_rs(root):
        t = open(p, encoding="utf-8").read(); b = blank(t)
        rel = os.path.relpath(p, root)
        ed = []
        for m in rx_state.finditer(b): ed.append((m.start(), m.end(), m.group(1) + m.group(2)))
        if "/📥️staging/" in rel and any(rel.split("/📥️staging/")[1].startswith(d) for d in cfg.DIR2CRATE):
            for m in rx_self.finditer(b): ed.append((m.start(), m.end(), m.group(1)))
        if ed:
            for s, e, r in sorted(ed, reverse=True): t = t[:s] + r + t[e:]
            open(p, "w", encoding="utf-8").write(t); n += len(ed)
    report.append(f"self-less call sites rewritten: {n}")
    return n

def widen(root, report):
    corpus = {p: blank(open(p, encoding="utf-8").read()) for p in all_rs(root)}
    mine = {}
    for crate in ctx.BRIDGE | {"shell-view-paint": 0}.keys() if False else list(ctx.BRIDGE) + ["shell-view-paint"]:
        for slug, p in ctx.crate_files(root, crate): mine[p] = crate
    # names defined private in my files
    changes = 0
    for p, crate in mine.items():
        t = open(p, encoding="utf-8").read(); doc = Doc(t)
        edits = []
        for it in doc.items():
            if it["kind"] in ("use", "mod"): continue
            if it["kind"] == "impl":
                if not re.match(r"impl(?:<[^>]*>)?\s+\w+Ctx", it["head"]) and not re.match(r"impl\s+ShellState\b", it["head"]): continue
                for m in doc.members(it):
                    mline = next(k for k in range(m["s"], m["e"] + 1) if re.search(r"\bfn\b", doc.blank[k]))
                    l = doc.lines[mline]
                    mm = re.match(r"^(\s*)((?:async |const |unsafe )*fn |(?:pub(?:\([^)]*\))?\s+)((?:async |const |unsafe )*fn )?)", l)
                    if re.match(r"^\s*(async |const |unsafe )*fn ", l):
                        pat = r"(?<![A-Za-z_0-9])(?:self\s*\.\s*(?:as_\w+\s*\(\s*\)\s*\.\s*)?|Self\s*::\s*|\.\s*)" + re.escape(m["name"]) + r"(?![A-Za-z_0-9])"
                        rx = re.compile(pat)
                        used_elsewhere = any(rx.search(b) for q, b in corpus.items() if q != p)
                        if used_elsewhere: edits.append((mline, "pub(super) "))
                continue
            nm = re.match(r"[A-Za-z_][A-Za-z_0-9]*", it["name"] or "")
            if not nm: continue
            k = next((k for k in range(it["s"], it["e"] + 1) if re.match(r"^(pub|fn|const|static|struct|enum|type|trait|async|unsafe|union)\b", doc.blank[k].strip() or "x") and not doc.blank[k].lstrip().startswith("#")), None)
            if k is None: continue
            l = doc.lines[k]
            if re.match(r"^(fn|const|static|struct|enum|type|trait|async|unsafe|union)\b", l):
                rx = re.compile(r"(?<![A-Za-z_0-9])" + re.escape(nm.group(0)) + r"(?![A-Za-z_0-9])")
                if any(rx.search(b) for q, b in corpus.items() if q != p): edits.append((k, "pub(super) "))
        if edits:
            lines = doc.lines
            for k, pre in sorted(set(edits), reverse=True):
                lines[k] = re.sub(r"^(\s*)", lambda m: m.group(1) + pre, lines[k], count=1)
            open(p, "w", encoding="utf-8").write("\n".join(lines)); changes += len(set(edits))
    report.append(f"private definitions widened to pub(super): {changes}")
