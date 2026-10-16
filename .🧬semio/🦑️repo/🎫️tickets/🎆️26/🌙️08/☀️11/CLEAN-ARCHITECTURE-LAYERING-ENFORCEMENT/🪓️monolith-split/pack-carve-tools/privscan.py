"""🕵️ Definition scan: members (fields, methods, associated fns) that are private in one carved crate but referenced from another."""
import re, sys, collections
from lib import *
from crates import *
from rs import tokenize

def crate_files(key):
    return [f for f in crawl(libroot(key))]

def struct_fields(text):
    """{struct_name: [(field, is_pub)]} for brace structs found at any nesting (regex over tokens)."""
    out = collections.defaultdict(list)
    for it in items(text):
        if it["kind"] == "struct" and it["name"]:
            m = re.search(r"\{(.*)\}\s*$", it["text"], re.S)
            if not m: 
                tup = re.search(r"\((.*)\)\s*;?\s*$", it["text"], re.S)
                continue
            body = m.group(1)
            depth = 0; cur = ""; parts = []
            for ch in body:
                if ch in "<([{": depth += 1
                if ch in ">)]}": depth -= 1
                if ch == "," and depth == 0: parts.append(cur); cur = ""
                else: cur += ch
            if cur.strip(): parts.append(cur)
            for p in parts:
                p = re.sub(r"(?s)(///[^\n]*\n|#\[[^\]]*\])", "", p).strip()
                if not p: continue
                pub = p.startswith("pub")
                mm = re.match(r"(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_][A-Za-z_0-9]*)\s*:", p)
                if mm: out[it["name"]].append((mm.group(1), pub))
    return out

def impl_methods(text):
    """{type: [(method, is_pub)]} from inherent impl blocks at top level."""
    out = collections.defaultdict(list)
    for it in items(text):
        if it["kind"] != "impl": continue
        head = it["name"] or ""
        if "for" in re.split(r"[<>(),:&']", head) and re.search(r"\bfor\b", head): continue
        mt = re.search(r"([A-Za-z_][A-Za-z_0-9]*)(?:<[^>]*>)?$", re.sub(r"^<.*?>", "", head, count=1) if head.startswith("<") else head)
        t = mt.group(1) if mt else None
        if not t: continue
        body = it["text"]
        for m in re.finditer(r"(?m)^\s*(pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+([A-Za-z_][A-Za-z_0-9]*)", body):
            out[t].append((m.group(2), bool(m.group(1)) and m.group(1).strip() == "pub"))
    return out

def run(pairs=None):
    defs = {}
    privs = {}
    texts = {}
    for key, *_ in CRATES:
        root = libroot(key)
        if not exists_stage_or_base(root): continue
        txt = {f: read_stage_or_base(f) for f in crate_files(key)}
        texts[key] = txt
        tops = set(); fields = collections.defaultdict(list); meths = collections.defaultdict(list)
        for f, t in txt.items():
            for it in items(t):
                if it["kind"] in ("struct", "enum") and it["name"]: tops.add(it["name"])
            for k, v in struct_fields(t).items(): fields[k] += v
            for k, v in impl_methods(t).items(): meths[k] += v
        defs[key] = tops
        privs[key] = {t: ({n for n, p in fields.get(t, []) if not p}, {n for n, p in meths.get(t, []) if not p}) for t in tops}
    findings = []
    for b, txt in texts.items():
        body = "\n".join(txt.values())
        idset = ids(body)
        for a, tops in defs.items():
            if a == b: continue
            imported = set()
            for m in re.finditer(re.escape(ident(a)) + r"::(\{[^}]*\}|[A-Za-z_0-9]+)", body):
                for n in re.findall(r"[A-Za-z_][A-Za-z_0-9]*", m.group(1)): imported.add(n)
            used = [t for t in tops if t in imported]
            for t in used:
                pf, pm = privs[a][t]
                for n in sorted(pf):
                    if re.search(r"(\.\s*" + re.escape(n) + r"\b(?!\s*\())|(\b" + re.escape(n) + r"\s*:[^:])", body): findings.append((b, a, t, "field", n))
                for n in sorted(pm):
                    if re.search(r"(\.\s*" + re.escape(n) + r"\s*(::<[^>]*>)?\s*\()|(\b" + re.escape(t) + r"::" + re.escape(n) + r"\b)", body): findings.append((b, a, t, "method", n))
    return findings

if __name__ == "__main__":
    for f in run(): print(*f)
