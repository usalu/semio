"""Item/member/field/import scanner for Rust sources (masked text, brace matching). Read-only helper of the otcp tools.
scan(text) -> dict(items=[...], members=[...], fields=[...], imports=[...]) with absolute offsets into text."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
from stubgen import mask

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
VIS = re.compile(r"pub\s*(?:\(\s*(?:crate|super|self|in\s+[^)]*)\s*\))?")
MODS = re.compile(r"(?:(?:default|async|unsafe)\s+|const\s+(?=(?:async|unsafe|extern|fn)\b)|extern\s*(?:\"[^\"]*\")?\s+(?=(?:fn|unsafe|async)\b))*")
KW = re.compile(r"(fn|struct|enum|trait|impl|mod|use|const|static|type|union|macro_rules\s*!|extern\s+crate)\b")


def match_close(m, i, open_ch, close_ch):
    d = 0
    n = len(m)
    while i < n:
        c = m[i]
        if c == open_ch: d += 1
        elif c == close_ch:
            d -= 1
            if d == 0: return i
        i += 1
    return n - 1


def skip_ws(m, i):
    while i < len(m) and m[i] in " \t\r\n": i += 1
    return i


def skip_attrs(m, i):
    attrs = []
    while True:
        i = skip_ws(m, i)
        if m.startswith("#", i) and (m.startswith("#[", i) or m.startswith("#![", i)):
            j = m.index("[", i)
            k = match_close(m, j, "[", "]")
            attrs.append((i, k + 1))
            i = k + 1
        else:
            return i, attrs


def find_header_end(m, i, stops="{;"):
    """first top-level (paren/bracket/angle-insensitive but paren/bracket aware) stop char index from i."""
    d = 0
    n = len(m)
    while i < n:
        c = m[i]
        if c in "([": d += 1
        elif c in ")]": d -= 1
        elif d == 0 and c in stops:
            return i
        i += 1
    return n


def parse(m, start, end, ctx, out, text):
    i = start
    while i < end:
        i, attrs = skip_attrs(m, i)
        if i >= end: break
        hpos = i
        vm = VIS.match(m, i)
        vis = ""
        vstart = vend = i
        if vm and (m[vm.end():vm.end() + 1] in " \t\r\n(" or True) and re.match(r"pub\b", m[i:i + 4]):
            vis = m[vm.start():vm.end()].replace(" ", "")
            vstart, vend = vm.start(), vm.end()
            i = vm.end()
        i = skip_ws(m, i)
        mm = MODS.match(m, i)
        j = mm.end()
        km = KW.match(m, j)
        if not km:
            i = find_header_end(m, i, ";}") + 1
            continue
        kw = re.sub(r"\s+", " ", km.group(1)).replace(" !", "!")
        kwpos = km.start()
        j = km.end()
        decl_start = attrs[0][0] if attrs else (vstart if vis else mm.start())
        rec = dict(kind=kw, vis=vis, vpos=(vstart, vend), hpos=hpos, kwpos=kwpos, attrs=[text[a:b] for a, b in attrs], ctx=ctx, start=decl_start)
        if kw in ("fn", "const", "static", "type", "struct", "enum", "union", "trait", "mod", "macro_rules!"):
            nm = re.compile(r"\s*(" + IDENT + ")").match(m, j)
            if nm and nm.group(1) == "mut" and kw == "static":
                nm = re.compile(r"\s*mut\s+(" + IDENT + ")").match(m, j)
            rec["name"] = nm.group(1) if nm else None
        if kw == "use":
            e = find_header_end(m, j, ";")
            rec["end"] = e + 1
            rec["text"] = text[j:e].strip()
            out["imports"].append(rec)
            i = e + 1
            continue
        if kw == "extern crate":
            e = find_header_end(m, j, ";")
            i = e + 1
            continue
        if kw in ("const", "static", "type"):
            e = find_header_end(m, j, ";")
            rec["end"] = e + 1
            (out["members"] if ctx.startswith("impl:") or ctx.startswith("trait:") else out["items"]).append(rec)
            i = e + 1
            continue
        if kw == "fn":
            e = find_header_end(m, j, "{;")
            if m[e:e + 1] == "{":
                e = match_close(m, e, "{", "}")
            rec["end"] = e + 1
            rec["body"] = (find_header_end(m, j, "{;"), e)
            (out["members"] if ctx.startswith("impl:") or ctx.startswith("trait:") else out["items"]).append(rec)
            i = e + 1
            continue
        if kw in ("struct", "union"):
            e = find_header_end(m, j, "{;(")
            if m[e:e + 1] == "(":
                p = match_close(m, e, "(", ")")
                rec["tuple"] = (e, p)
                wh = find_header_end(m, p + 1, ";{")
                e = wh
                if m[e:e + 1] == "{":
                    e = match_close(m, e, "{", "}")
            elif m[e:e + 1] == "{":
                b = e
                e = match_close(m, e, "{", "}")
                rec["braces"] = (b, e)
            rec["end"] = e + 1
            out["items"].append(rec)
            i = e + 1
            continue
        if kw == "enum":
            e = find_header_end(m, j, "{;")
            b = e
            e = match_close(m, e, "{", "}")
            rec["braces"] = (b, e)
            rec["end"] = e + 1
            out["items"].append(rec)
            i = e + 1
            continue
        if kw == "trait":
            e = find_header_end(m, j, "{;")
            b = e
            e = match_close(m, e, "{", "}")
            rec["end"] = e + 1
            out["items"].append(rec)
            parse(m, b + 1, e, "trait:" + (rec.get("name") or ""), out, text)
            i = e + 1
            continue
        if kw == "impl":
            e = find_header_end(m, j, "{;")
            head = text[j:e]
            b = e
            e = match_close(m, e, "{", "}")
            rec["end"] = e + 1
            rec["head"] = head.strip()
            hm = re.sub(r"<[^{}]*?>", " ", m[j:b], count=0)
            is_trait = re.search(r"\bfor\b", m[j:b]) is not None
            rec["trait_impl"] = is_trait
            tgt = m[j:b]
            if is_trait:
                tgt = tgt.split(" for ", 1)[-1] if " for " in tgt else re.split(r"\bfor\b", tgt, 1)[-1]
            tm = re.search(r"(" + IDENT + r")\s*(?:<|where|$|\{)", tgt.strip() + " ")
            ty = re.findall(IDENT, re.sub(r"<[^<>]*>", "", re.sub(r"<[^<>]*>", "", tgt)))
            rec["self_ty"] = [t for t in ty if t not in ("where", "dyn", "mut", "impl")][0] if ty else None
            out["impls"].append(rec)
            parse(m, b + 1, e, "impl:" + str(rec["self_ty"]) + (":trait" if is_trait else ""), out, text)
            i = e + 1
            continue
        if kw == "mod":
            e = find_header_end(m, j, "{;")
            if m[e:e + 1] == "{":
                b = e
                e = match_close(m, e, "{", "}")
                rec["end"] = e + 1
                out["items"].append(rec)
                parse(m, b + 1, e, ctx + "::" + (rec.get("name") or "?") if ctx else rec.get("name") or "?", out, text)
                i = e + 1
            else:
                rec["end"] = e + 1
                out["items"].append(rec)
                i = e + 1
            continue
        if kw == "macro_rules!":
            e = find_header_end(m, j, "{(")
            if m[e:e + 1] == "{":
                e = match_close(m, e, "{", "}")
                rec["end"] = e + 1
            else:
                e = match_close(m, e, "(", ")")
                e = find_header_end(m, e, ";")
                rec["end"] = e + 1
            out["items"].append(rec)
            i = e + 1
            continue
        i = j


def struct_fields(m, rec, text):
    """named fields of a struct record: [(name, vis, vpos, namepos)] ; tuple fields: [("0", vis, vpos, pos)]"""
    out = []
    if "braces" in rec:
        b, e = rec["braces"]
        i = b + 1
        while i < e:
            i, attrs = skip_attrs(m, i)
            if i >= e: break
            vm = re.match(r"pub\s*(?:\(\s*(?:crate|super|self|in\s+[^)]*)\s*\))?", m[i:e])
            vis, vpos = "", (i, i)
            if vm and re.match(r"pub\b", m[i:i + 4]):
                vis = vm.group(0).replace(" ", "")
                vpos = (i, i + vm.end())
                i += vm.end()
            i = skip_ws(m, i)
            nm = re.compile(IDENT).match(m, i)
            if not nm:
                i += 1
                continue
            out.append((nm.group(0), vis, vpos, nm.start()))
            d = 0
            k = nm.end()
            while k < e:
                c = m[k]
                if c in "([{<": d += 1
                elif c in ")]}": d -= 1
                elif c == ">" and m[k - 1] != "-": d -= 1
                elif c == "," and d <= 0: break
                k += 1
            i = k + 1
    elif "tuple" in rec:
        b, e = rec["tuple"]
        i = b + 1
        idx = 0
        while i < e:
            i, attrs = skip_attrs(m, i)
            if i >= e: break
            vm = re.match(r"pub\s*(?:\(\s*(?:crate|super|self|in\s+[^)]*)\s*\))?", m[i:e])
            vis, vpos = "", (i, i)
            if vm and re.match(r"pub\b", m[i:i + 4]):
                vis = vm.group(0).replace(" ", "")
                vpos = (i, i + vm.end())
                i += vm.end()
            out.append((str(idx), vis, vpos, i))
            idx += 1
            d = 0
            k = i
            while k < e:
                c = m[k]
                if c in "([{<": d += 1
                elif c in ")]}": d -= 1
                elif c == ">" and m[k - 1] != "-": d -= 1
                elif c == "," and d <= 0: break
                k += 1
            i = k + 1
    return out


def scan(text):
    m = mask(text)
    out = dict(items=[], members=[], imports=[], impls=[], m=m)
    parse(m, 0, len(m), "", out, text)
    return out


if __name__ == "__main__":
    t = open(sys.argv[1], encoding="utf-8").read()
    r = scan(t)
    for k in ("items", "impls", "members", "imports"):
        for x in r[k]:
            print(k, x["kind"], x.get("name") or x.get("self_ty") or x.get("text", "")[:50], repr(x["vis"]), x["ctx"], text_line := t.count("\n", 0, x["kwpos"]) + 1 if "kwpos" in x else "")
