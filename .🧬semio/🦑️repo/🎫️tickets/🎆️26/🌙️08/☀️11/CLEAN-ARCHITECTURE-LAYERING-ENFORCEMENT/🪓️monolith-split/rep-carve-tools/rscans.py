"""🕵️ Orphan, definition and private-member scans over the staged carve (compensate for what the name-resolution oracle cannot see)."""
import re, collections, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C
import rmanifests

def strip_generics(s):
    d = 0; out = ""
    for ch in s:
        if ch == "<": d += 1
        elif ch == ">": d -= 1
        elif d == 0: out += ch
    return out

def walk_items(text, types, trs, impls, macros, f):
    for it in items(text):
        if it["kind"] in ("struct", "enum", "type", "union") and it["name"]: types.add(it["name"])
        if it["kind"] == "trait" and it["name"]: trs.add(it["name"])
        if it["kind"] == "impl": impls.append((f, it["name"] or ""))
        if it["kind"] is None:
            m = re.search(r"artifact_retire_(?:struct|leaf)!\(\s*([A-Za-z_]\w*)", it["text"])
            if m: macros.append((f, m.group(1)))
        if it["kind"] == "mod" and "{" in it["text"]:
            m = re.search(r"\{(.*)\}\s*$", it["text"], re.S)
            if m: walk_items(m.group(1), types, trs, impls, macros, f)

def orphan():
    bad = []; stats = {}
    for key in C.KEYS:
        types = set(); trs = set(); impls = []; macros = []
        for f in crawl(C.libroot(key)):
            walk_items(read_stage_or_base(f), types, trs, impls, macros, f)
        n = 0
        for f, head in impls:
            h = re.sub(r"\bwhere\b.*", "", strip_generics(head))
            if re.search(r"\bfor\b", h): tr, ty = re.split(r"\bfor\b", h, maxsplit=1)
            else: tr, ty = None, h
            tyb = re.findall(r"[A-Za-z_]\w*", ty.replace("dyn ", "").replace("&", " "))
            tyname = tyb[-1] if tyb else ""
            trb = re.findall(r"[A-Za-z_]\w*", tr or "")
            trname = trb[-1] if trb else None
            n += 1
            if not (tyname in types or (trname and trname in trs)):
                bad.append((key, f.replace(R, ""), head[:90], tyname, trname))
        for f, ty in macros:
            n += 1
            if ty not in types: bad.append((key, f.replace(R, ""), "retire macro", ty, None))
        stats[key] = n
    return bad, stats

def struct_fields(text):
    out = collections.defaultdict(list)
    for it in items(text):
        if it["kind"] == "struct" and it["name"]:
            m = re.search(r"\{(.*)\}\s*$", it["text"], re.S)
            if not m: continue
            depth = 0; cur = ""; parts = []
            for ch in m.group(1):
                if ch in "<([{": depth += 1
                if ch in ">)]}": depth -= 1
                if ch == "," and depth == 0: parts.append(cur); cur = ""
                else: cur += ch
            if cur.strip(): parts.append(cur)
            for p in parts:
                p = re.sub(r"(?s)(///[^\n]*\n|#\[[^\]]*\])", "", p).strip()
                if not p: continue
                mm = re.match(r"(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*:", p)
                if mm: out[it["name"]].append((mm.group(1), p.startswith("pub") and not p.startswith("pub(")))
    return out

def impl_methods(text):
    out = collections.defaultdict(list)
    for it in items(text):
        if it["kind"] != "impl": continue
        head = it["name"] or ""
        if re.search(r"\bfor\b", head): continue
        mt = re.search(r"([A-Za-z_]\w*)(?:\s*<[^>]*>)?\s*$", head)
        t = mt.group(1) if mt else None
        if not t: continue
        for m in re.finditer(r"(?m)^\s*(pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+([A-Za-z_]\w*)", it["text"]):
            out[t].append((m.group(2), bool(m.group(1)) and m.group(1).strip() == "pub"))
    return out

def privscan():
    texts = {}; privs = {}
    rep = rmanifests.run()
    for key in C.KEYS:
        files = crawl(C.libroot(key))
        texts[key] = {f: read_stage_or_base(f) for f in files}
        tops = set(); fields = collections.defaultdict(list); meths = collections.defaultdict(list)
        for f, t in texts[key].items():
            for it in items(t):
                if it["kind"] in ("struct", "enum") and it["name"]: tops.add(it["name"])
            for k, v in struct_fields(t).items(): fields[k] += v
            for k, v in impl_methods(t).items(): meths[k] += v
        privs[key] = {t: ({n for n, p in fields.get(t, []) if not p}, {n for n, p in meths.get(t, []) if not p}) for t in tops}
    finds = []
    for b, txt in texts.items():
        body = "\n".join(txt.values())
        normal, dev = rep[b]
        for a in C.KEYS:
            if a == b or C.ident(a) not in set(normal) | set(dev): continue
            imported = set()
            for m in re.finditer(re.escape(C.ident(a)) + r"::(\{[^}]*\}|[A-Za-z_0-9:]+)", body):
                for n in re.findall(r"[A-Za-z_]\w*", m.group(1)): imported.add(n)
            for t in privs[a]:
                if t not in imported: continue
                pf, pm = privs[a][t]
                for n in sorted(pf):
                    if re.search(r"(\.\s*" + re.escape(n) + r"\b(?!\s*\())|(\b" + re.escape(n) + r"\s*:[^:])", body): finds.append((b, a, t, "field", n))
                for n in sorted(pm):
                    if re.search(r"(\.\s*" + re.escape(n) + r"\s*(::<[^>]*>)?\s*\()|(\b" + re.escape(t) + r"::" + re.escape(n) + r"\b)", body): finds.append((b, a, t, "method", n))
    return finds

if __name__ == "__main__":
    bad, stats = orphan()
    print("ORPHAN impls checked:", sum(stats.values()), "violations:", len(bad))
    for b in bad: print("  ", *b)
    print("PRIVSCAN candidates:")
    for f in privscan(): print("  ", *f)
