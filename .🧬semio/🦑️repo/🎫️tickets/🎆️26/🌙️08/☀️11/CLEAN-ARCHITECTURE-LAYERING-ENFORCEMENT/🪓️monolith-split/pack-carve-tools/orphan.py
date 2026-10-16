"""⚖️ Coherence scan: every impl in a carved crate must target a local type (inherent) or a local trait or local type (trait impl)."""
import re, collections
from lib import *
from crates import *

def strip_generics(s):
    d = 0; out = ""
    for ch in s:
        if ch == "<": d += 1
        elif ch == ">": d -= 1
        elif d == 0: out += ch
    return out

def check(keys=None):
    local = {}; traits = {}; implsof = {}
    for key, *_ in CRATES:
        if not exists_stage_or_base(libroot(key)): continue
        types = set(); trs = set(); impls = []
        for f in crawl(libroot(key)):
            t = read_stage_or_base(f)
            def walk(text):
                for it in items(text):
                    if it["kind"] in ("struct", "enum", "type", "union") and it["name"]: types.add(it["name"])
                    if it["kind"] == "trait" and it["name"]: trs.add(it["name"])
                    if it["kind"] == "impl": impls.append((f, it["name"] or ""))
                    if it["kind"] == "mod" and "{" in it["text"]:
                        m = re.search(r"\{(.*)\}\s*$", it["text"], re.S)
                        if m: walk(m.group(1))
            walk(t)
        local[key] = types; traits[key] = trs; implsof[key] = impls
    bad = []
    for key, impls in implsof.items():
        for f, head in impls:
            h = strip_generics(head)
            h = re.sub(r"\bwhere\b.*", "", h)
            if "for" in re.findall(r"\bfor\b", h):
                tr, ty = re.split(r"\bfor\b", h, maxsplit=1)
            else:
                tr, ty = None, h
            tyb = re.findall(r"[A-Za-z_][A-Za-z_0-9]*", ty.replace("dyn ", "").replace("&", " "))
            tyname = tyb[-1] if tyb else ""
            trb = re.findall(r"[A-Za-z_][A-Za-z_0-9]*", tr or "")
            trname = trb[-1] if trb else None
            ok = tyname in local[key] or (trname and trname in traits[key])
            if not ok:
                bad.append((key, f.split("/")[-2], head[:90], "type=" + tyname, "trait=" + str(trname)))
    return bad

if __name__ == "__main__":
    for b in check(): print(*b)
