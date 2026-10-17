"""🧭 Groups non-stub resolution/privacy rows of a pos result by (code, missing name) and shows where that name is defined (git grep, tracked+untracked).
A definition flagged `M` sits inside a macro invocation block (heuristic: an unclosed `ident! {`/`ident!(` within 40 lines above), so the oracle cannot see it.
usage: triage.py <result.json> [--cfg lib|test] [--codes E0425,..]"""
import json, re, subprocess, sys, collections, os
ROOT = "/Users/ueli/Documents/semio"
a = sys.argv[1:]
r = json.load(open(a[0], encoding="utf-8"))
only = a[a.index("--cfg") + 1] if "--cfg" in a else None
codes = set(a[a.index("--codes") + 1].split(",")) if "--codes" in a else set("E0425 E0432 E0433 E0412 E0603 E0616 E0624 E0107".split())
CACHE = {}
UNTRACKED = None
def untracked():
    global UNTRACKED
    if UNTRACKED is None:
        out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "--others", "--exclude-standard", "--", ":(glob)**/*.rs", ":!.🧬semio"], cwd=ROOT, capture_output=True, text=True).stdout
        UNTRACKED = [p for p in out.split("\n") if p]
    return UNTRACKED
DEFS = json.load(open("/Users/ueli/.cache/ohp/defs.json", encoding="utf-8"))
LINES = {}
def defs(name):
    if name in CACHE:
        return CACHE[name]
    flagged = []
    for f, ln in DEFS.get(name, [])[:8]:
        macro = False
        try:
            if f not in LINES:
                LINES[f] = open(os.path.join(ROOT, f), encoding="utf-8").read().split("\n")
            lines = LINES[f]
            for k in range(max(0, ln - 41), ln - 1):
                if re.search(r"[A-Za-z_][A-Za-z0-9_]*! *[{(]\s*$", lines[k]) or re.search(r"macro_rules!", lines[k]) or re.match(r"\s*[A-Za-z_][A-Za-z0-9_]*! *\{", lines[k]):
                    macro = True
        except Exception:
            pass
        flagged.append((f, ln, macro))
    CACHE[name] = flagged
    return flagged
def name_of(code, msg):
    m = re.search(r"cannot find (?:type|function|value|struct, variant or union type|tuple struct or tuple variant|unit struct, unit variant or constant|trait|macro|derive macro|attribute) `([^`]+)`", msg)
    if m: return m.group(1).split("::")[-1]
    m = re.search(r"unresolved imports? `([^`]+)`", msg)
    if m: return m.group(1).split("::")[-1].strip("{} ")
    m = re.search(r"(?:function|struct|enum|type|trait|constant|module|static|tuple struct|unit struct|type alias|constant item|macro|trait alias|struct constructor|derive macro|tuple variant) import `([^`]+)` is private", msg) or re.search(r"`([^`]+)` is private", msg) or re.search(r"(?:function|struct|enum|type alias|constant|module|trait|tuple struct) `([^`]+)` is private", msg)
    if m: return m.group(1).split("::")[-1]
    m = re.search(r"cannot find (?:module or crate )?`([^`]+)` in", msg)
    if m: return m.group(1)
    m = re.search(r"use of undeclared (?:type|crate or module) `([^`]+)`", msg)
    if m: return m.group(1)
    m = re.search(r"field `([^`]+)` of struct", msg) or re.search(r"(?:associated function|method|associated constant) `([^`]+)` is private", msg)
    if m: return m.group(1)
    return None
for cfg, e in r["summary"].items():
    if only and cfg != only:
        continue
    g = collections.OrderedDict()
    for d in e["new_list"]:
        if d["stub"] or d.get("inherited") or d["code"] not in codes:
            continue
        msg = re.sub(r"__m_[A-Za-z0-9_]+", "M", d["msg"])
        nm = name_of(d["code"], msg)
        f = "/".join(re.sub(r"^__m_[A-Za-z0-9_]+/", "", d["file"]).split("/")[-3:])
        lt, hs = d.get("line_text"), d.get("hs")
        pre = (lt or "")[:hs] if lt is not None and hs is not None else ""
        if "can only be used in start position" in msg:
            nm = "<A1 nested-use crate>"
        elif d["code"] in ("E0433", "E0425", "E0432") and re.search(r"\.\.\s*(crate::)?$", pre.rstrip() if not pre.rstrip().endswith("crate::") else pre.rstrip()):
            nm = "<A2 ..crate:: rewriter>"
        elif ("in this scope" in msg or " in module `" in msg) and d["code"] in ("E0425", "E0412", "E0433"):
            rel = os.path.normpath(re.sub(r"^__m_[A-Za-z0-9_]+/", "", d["file"]))
            try:
                src = open(os.path.join(ROOT, rel), encoding="utf-8").read()
            except Exception:
                src = ""
            qual = re.search(r"in module `([^`]+)`", msg)
            if qual and (qual.group(1).startswith("crate::__ext") or qual.group(1).startswith("crate::M::__ext") or (not qual.group(1).startswith(("crate", "super", "self", "M::")) and "::" not in qual.group(1))):
                nm = "<A4 alias-of-external module " + qual.group(1) + ">"
            elif not qual and re.search(r"^\s*(?:pub(?:\([a-z: ]+\))? )?use +(?!crate::|super::|self::|std::|core::|alloc::)[A-Za-z_][A-Za-z0-9_:]*::(?:[A-Za-z0-9_:{}, ]*::)?\*;", src, re.M) and nm and nm in msg:
                nm = "<A5 external glob> " + nm
        relp = os.path.normpath(re.sub(r"^__m_[A-Za-z0-9_]+/", "", d["file"]))
        fam = "/".join(relp.split("/")[:3])
        g.setdefault((d["code"], nm or msg[:60]), []).append((f, d["line"], fam))
    print(f"== cfg {cfg}: {len(g)} distinct (code,name) groups")
    for (code, nm), locs in sorted(g.items(), key=lambda kv: (kv[0][0], -len(kv[1]))):
        ds = defs(nm) if nm and re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", nm) else []
        nonm = [(f, ln) for f, ln, m in ds if not m]
        fams = {l[2] for l in locs}
        local = [(f, ln) for f, ln in nonm if any(f.startswith(x + "/") for x in fams)]
        tag = "A" if nm and nm.startswith("<A") else ("A3?" if not ds or not nonm else ("EXT?" if not local else "REAL?"))
        if tag == "REAL?":
            ds = [(f, ln, m) for f, ln, m in ds if (f, ln) in local]
        dtxt = tag + " " + ("undefined" if not ds else "; ".join(f"{'M ' if m else ''}{'/'.join(f.split('/')[-3:])}:{ln}" for f, ln, m in ds[:3]))
        ftxt = ",".join(sorted({l[0][-26:] + ":" + str(l[1]) for l in locs})[:2])
        print(f"{code} x{len(locs)} `{nm}` | {ftxt} | defs: {dtxt}")
