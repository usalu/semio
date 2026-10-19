#!/usr/bin/env python3
"""🔪 Rust function phase splitter. Cuts one long `fn` body into contiguous statement-run phases (free functions after the owning impl/fn), keeps every statement verbatim and in order,
threads cross-phase locals as parameters and returned values/structs.
usage: rsfn.py table <file> <fn> [--nth N]        depth-1 statements: line range, kind, declared names, used earlier names
       rsfn.py plan  <spec.json>                  per phase IN/OUT/type report (no write)
       rsfn.py apply <spec.json>                  rewrites the file (refuses when the function text changed since `expect_sha`)
spec: {file, function, nth?, expect_sha?, imports?, types:{name:type}, modes:{name:"ref"|"mut"|"val"}, keep:[stmtStartLine...], phases:[{name, emoji, doc, from, to, ret?, struct?, fields?:[name...], tail?:true, rewrite?:[[old,new]...]}]}"""
import hashlib, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

KW = set("let mut ref if else for while loop match fn move return break continue in as true false self Self pub crate super use mod struct enum impl trait where unsafe async await dyn const static type".split())


def lex(text):
    toks = []
    n = len(text)
    i = 0
    line = 1
    isid = lambda c: c.isalnum() or c == "_"
    while i < n:
        c = text[i]
        if c == "\n":
            line += 1; i += 1; continue
        if c.isspace():
            i += 1; continue
        if text.startswith("//", i):
            while i < n and text[i] != "\n": i += 1
            continue
        if text.startswith("/*", i):
            d = 1; i += 2
            while i < n and d:
                if text.startswith("/*", i): d += 1; i += 2
                elif text.startswith("*/", i): d -= 1; i += 2
                else:
                    if text[i] == "\n": line += 1
                    i += 1
            continue
        if c == '"':
            s = i; sl = line; i += 1
            while i < n and text[i] != '"':
                if text[i] == "\\": i += 1
                if i < n and text[i] == "\n": line += 1
                i += 1
            i += 1
            toks.append(("str", text[s:i], sl, s, i)); continue
        if c.isalpha() or c == "_":
            j = i
            while j < n and isid(text[j]): j += 1
            w = text[i:j]
            if w in ("r", "br", "cr") and j < n and text[j] in '"#':
                k = j; h = 0
                while k < n and text[k] == "#": h += 1; k += 1
                if k < n and text[k] == '"':
                    close = '"' + "#" * h
                    e = text.find(close, k + 1)
                    e = n if e < 0 else e + len(close)
                    sl = line; line += text.count("\n", i, e)
                    toks.append(("str", text[i:e], sl, i, e)); i = e; continue
            toks.append(("id", w, line, i, j)); i = j; continue
        if c.isdigit():
            j = i
            while j < n and (isid(text[j]) or (text[j] == "." and j + 1 < n and text[j + 1].isdigit())): j += 1
            toks.append(("num", text[i:j], line, i, j)); i = j; continue
        if c == "'":
            if i + 1 < n and text[i + 1] == "\\":
                j = i + 2
                while j < n and text[j] != "'": j += 1
                toks.append(("str", text[i:j + 1], line, i, j + 1)); i = j + 1; continue
            if i + 2 < n and text[i + 2] == "'":
                toks.append(("str", text[i:i + 3], line, i, i + 3)); i += 3; continue
            j = i + 1
            while j < n and isid(text[j]): j += 1
            toks.append(("life", text[i:j], line, i, j)); i = j; continue
        toks.append(("p", c, line, i, i + 1)); i += 1
    return toks


def find_fn(toks, name, nth=0):
    hits = [k for k in range(len(toks) - 1) if toks[k][1] == "fn" and toks[k + 1][1] == name]
    k = hits[nth]
    d = 0; m = k + 2; op = -1
    while m < len(toks):
        t = toks[m][1]
        if t in "([": d += 1
        elif t in ")]": d -= 1
        elif d == 0 and t == ";": break
        elif d == 0 and t == "{": op = m; break
        m += 1
    b = 0
    for q in range(op, len(toks)):
        if toks[q][1] == "{": b += 1
        elif toks[q][1] == "}":
            b -= 1
            if b == 0: return k, op, q
    raise SystemExit("unbalanced")


def params_of(toks, k, op):
    s = k + 2
    while toks[s][1] != "(": s += 1
    d = 0; cur = []; out = []; e = s
    for q in range(s, op):
        t = toks[q][1]
        if t in "([<{" and not (t == "<" and False): d += 1 if t != "<" else 0
        if t in ")]}" : d -= 1
        if d == 0 and t == ")":
            e = q; break
        if q > s: cur.append(q)
    return s, e


def split_params(text, toks, s, e):
    inner = text[toks[s][4]:toks[e][3]]
    out = []; d = 0; cur = ""
    for ch in inner:
        if ch in "([{<": d += 1
        elif ch in ")]}>": d -= 1
        if ch == "," and d == 0:
            out.append(cur); cur = ""
        else:
            cur += ch
    if cur.strip(): out.append(cur)
    res = {}
    for p in out:
        m = re.match(r"\s*(?:mut\s+)?(\w+)\s*:\s*(.*)\Z", p, re.S)
        if m: res[m.group(1)] = " ".join(m.group(2).split())
    return res


def statements(toks, op, cl):
    stmts = []
    q = op + 1
    while q < cl:
        s = q
        while toks[q][1] == "#" and toks[q + 1][1] == "[":
            d = 0; q += 1
            while True:
                if toks[q][1] == "[": d += 1
                elif toks[q][1] == "]":
                    d -= 1
                    if d == 0: q += 1; break
                q += 1
        head = toks[q][1]
        block_like = head in ("if", "match", "for", "while", "loop", "unsafe", "{") or (head == "'" )
        d = 0; e = q
        while e < cl:
            t = toks[e][1]
            if t in "([{": d += 1
            elif t in ")]}":
                d -= 1
            if d == 0 and t == ";":
                break
            if d == 0 and t == "}" and e + 1 <= cl:
                nx = toks[e + 1][1] if e + 1 < len(toks) else ""
                if e + 1 == cl:
                    break
                if nx == ";":
                    e += 1; break
                if block_like and nx not in (".", "?", "else", ")", ",") :
                    break
            e += 1
        if e >= cl: e = cl - 1
        stmts.append((s, e))
        q = e + 1
    return stmts


def _bare_use(toks, s, e, name):
    for q in range(s, e + 1):
        t = toks[q]
        if t[0] == "id" and t[1] == name:
            pv = toks[q - 1][1] if q else ""
            nx = toks[q + 1][1]
            if pv == "." or pv == "&" or (pv == "mut" and toks[q - 2][1] == "&"):
                continue
            if nx in (".", "[") or (nx == ":" and toks[q + 2][1] != ":" and pv in ("{", ",")):
                continue
            return True
    return False


def pattern_names(toks, s, e):
    names = []
    q = s
    while q < e and toks[q][1] == "#" and toks[q + 1][1] == "[":
        d = 0
        q += 1
        while True:
            if toks[q][1] == "[": d += 1
            elif toks[q][1] == "]":
                d -= 1
                if d == 0:
                    q += 1
                    break
            q += 1
    if toks[q][1] != "let":
        return names, False
    q += 1
    mut = False
    d = 0
    while q < e:
        t = toks[q]
        if t[1] in "([{": d += 1
        elif t[1] in ")]}": d -= 1
        if d == 0 and t[1] in ("=", ":", ";") and not (t[1] == ":" and toks[q + 1][1] == ":"):
            break
        if t[0] == "id" and t[1] not in KW and t[1] != "_":
            nx = toks[q + 1][1]
            if nx not in ("(", "{", ":") and not t[1][0].isupper():
                names.append(t[1])
            elif nx == ":" and toks[q + 2][1] != ":" and d == 0 and not t[1][0].isupper():
                names.append(t[1])
        if t[1] == "mut": mut = True
        q += 1
    return names, mut


def analyse(text, fname, nth):
    toks = lex(text)
    k, op, cl = find_fn(toks, fname, nth)
    ps, pe = params_of(toks, k, op)
    params = split_params(text, toks, ps, pe)
    sts = statements(toks, op, cl)
    rows = []
    for (s, e) in sts:
        rows.append(dict(s=s, e=e, a=toks[s][2], b=toks[e][2], head=toks[[x for x in range(s, e + 1) if toks[x][1] not in ("#", "[", "]") or False][0]][1]))
    for r in rows:
        r["decl"], r["mut"] = pattern_names(toks, r["s"], r["e"])
        r["isfnlike"] = False
    for i, r in enumerate(rows):
        if i and rows[i - 1]["b"] == r["a"]:
            raise SystemExit("statements share line %d" % r["a"])
    return toks, k, op, cl, params, rows


def analyse_block(text, fname, nth, block_line, ctx):
    toks = lex(text)
    k, op, cl = find_fn(toks, fname, nth)
    q = next(i for i in range(op, cl) if toks[i][2] == block_line and toks[i][1] in ("for", "while", "loop", "if", "match", "unsafe"))
    d = 0
    bop = None
    for i in range(q, cl):
        t = toks[i][1]
        if t in "([": d += 1
        elif t in ")]": d -= 1
        elif d == 0 and t == "{":
            bop = i
            break
    b = 0
    bcl = None
    for i in range(bop, len(toks)):
        if toks[i][1] == "{": b += 1
        elif toks[i][1] == "}":
            b -= 1
            if b == 0:
                bcl = i
                break
    sts = statements(toks, bop, bcl)
    rows = []
    for (s_, e_) in sts:
        rows.append(dict(s=s_, e=e_, a=toks[s_][2], b=toks[e_][2], head=toks[s_][1]))
    for r in rows:
        r["decl"], r["mut"] = pattern_names(toks, r["s"], r["e"])
    for i, r in enumerate(rows):
        if i and rows[i - 1]["b"] == r["a"]:
            raise SystemExit("statements share line %d" % r["a"])
    return toks, k, op, cl, dict(ctx), rows


def line_text(text):
    return text.split("\n")


def used_in(toks, s, e, names):
    got = {}
    import rscap
    for q in range(s, e + 1):
        t = toks[q]
        if t[0] == "str":
            for n in rscap.captures(toks, q):
                if n in names: got[n] = got.get(n, 0) + 1
            continue
        if t[0] == "id" and t[1] in names:
            if q > 0 and toks[q - 1][1] == "." and not (q > 1 and toks[q - 2][1] == "."):
                continue
            if (q > 1 and toks[q - 1][1] == ":" and toks[q - 2][1] == ":") or (toks[q + 1][1] == ":" and toks[q + 2][1] == ":"):
                continue
            if toks[q + 1][1] == ":" and toks[q + 2][1] != ":" and toks[q - 1][1] in ("{", ","):
                if toks[q - 1][1] == "{" or toks[q - 1][1] == ",":
                    got.setdefault(t[1], 0)
                    got[t[1]] += 1
                    continue
            got[t[1]] = got.get(t[1], 0) + 1
    return got


def cmd_table(file, fname, nth, block=None):
    text = open(file, encoding="utf-8").read()
    if block:
        toks, k, op, cl, params, rows = analyse_block(text, fname, nth, block, {})
    else:
        toks, k, op, cl, params, rows = analyse(text, fname, nth)
    print("fn", fname, "lines", toks[k][2], "-", toks[cl][2], "params", list(params))
    declared = {p: -1 for p in params}
    for i, r in enumerate(rows):
        used = used_in(toks, r["s"], r["e"], set(declared))
        print("%d-%d\t%d\t%s\tdecl=%s\tuse=%s" % (r["a"], r["b"], r["b"] - r["a"] + 1, r["head"], ",".join(r["decl"]), ",".join("%s@%s" % (u, declared[u] if declared[u] < 0 else rows[declared[u]]["a"]) for u in used)))
        for d in r["decl"]: declared[d] = i


INFER = [
    (r"\.create_bind_group_layout\(", "gpu_device::BindGroupLayout"), (r"\.create_shader_module\(", "gpu_device::ShaderModule"), (r"\.create_pipeline_layout\(", "gpu_device::PipelineLayout"),
    (r"\.create_render_pipeline\(", "gpu_device::RenderPipeline"), (r"\.create_buffer_init\(", "gpu_device::Buffer"), (r"\.create_buffer\(", "gpu_device::Buffer"), (r"\.create_texture\(", "gpu_device::Texture"),
    (r"\.create_sampler\(", "gpu_device::Sampler"), (r"\.create_bind_group\(", "gpu_device::BindGroup"), (r"\.create_view\(", "gpu_device::TextureView"),
]


def infer_type(stmt_text):
    stmt_text = re.sub(r"^(\s*#\[[^\n]*\]\s*)+", "", stmt_text)
    rhs = stmt_text.split("=", 1)[1] if "=" in stmt_text else stmt_text
    head = rhs.strip()[:160]
    for rx, ty in INFER:
        if re.match(r"^[\w\.&\(\)\s]*?" + rx, head): return ty
    return None


def build(spec, write):
    file = spec["file"]
    text = open(file, encoding="utf-8").read()
    lines = text.split("\n")
    if spec.get("block_line"):
        toks, k, op, cl, params, rows = analyse_block(text, spec["function"], spec.get("nth", 0), spec["block_line"], spec["context"])
    else:
        toks, k, op, cl, params, rows = analyse(text, spec["function"], spec.get("nth", 0))
    fn_start, fn_end = toks[k][2], toks[cl][2]
    fn_text = text[toks[k][3]:toks[cl][4]]
    if spec.get("expect_sha") and hashlib.sha256(fn_text.encode()).hexdigest() != spec["expect_sha"]:
        raise SystemExit("function text changed since expect_sha")
    types = dict(params)
    types.update(spec.get("types", {}))
    modes = spec.get("modes", {})
    cfgs = spec.get("cfg", {})
    declared_at = {p: -1 for p in params}
    mutdecl = {}
    by_line = {r["a"]: i for i, r in enumerate(rows)}
    problems = []
    for i, r in enumerate(rows):
        for d in r["decl"]:
            declared_at[d] = i
            mutdecl[d] = r["mut"]
            if d not in types:
                t = infer_type(text[toks[r["s"]][3]:toks[r["e"]][4]])
                if t:
                    types[d] = t
    keep = set(spec.get("keep", []))
    plan = []
    cover = {}
    for ph in spec["phases"]:
        if ph["from"] not in by_line or ph["to"] not in by_line:
            problems.append("phase %s: from/to is not a statement start line (%s, %s)" % (ph["name"], ph["from"], ph["to"]))
            continue
        a, b = by_line[ph["from"]], by_line[ph["to"]]
        plan.append(dict(ph=ph, a=a, b=b))
        for x in range(a, b + 1):
            if x in cover:
                problems.append("statement at line %d in two phases" % rows[x]["a"])
            cover[x] = ph["name"]
    for i, r in enumerate(rows):
        if i not in cover and r["a"] not in keep:
            problems.append("statement at line %d is neither kept nor in a phase" % r["a"])
    n_stm = len(rows)
    plan.sort(key=lambda p: p["a"])
    for p in plan:
        a, b = p["a"], p["b"]
        inside = {d for x in range(a, b + 1) for d in rows[x]["decl"]}
        before = {n for n, i in declared_at.items() if i < a}
        used = used_in(toks, rows[a]["s"], rows[b]["e"], before)
        p["ins"] = sorted((n for n in used if n not in p["ph"].get("drop_ins", [])), key=lambda n: (declared_at[n], list(params).index(n) if n in params else 0, n))
        p["auto_val"] = [n for n in p["ins"] if n not in modes and not types.get(n, "").startswith("&") and _bare_use(toks, rows[a]["s"], rows[b]["e"], n)]
        later = {}
        for x in range(b + 1, n_stm):
            for n, c in used_in(toks, rows[x]["s"], rows[x]["e"], inside).items():
                later[n] = later.get(n, 0) + c
        outs = [d for x in range(a, b + 1) for d in rows[x]["decl"] if d in later]
        seen = set()
        p["outs"] = [d for d in outs if not (d in seen or seen.add(d))]
        p["is_tail"] = b == n_stm - 1 and p["ph"].get("tail", False)
        p["closure_outs"] = [d for d in p["outs"] if re.search(r"\blet\s+(mut\s+)?%s\s*=\s*(\|\||move\s*\||\|)" % d, text[toks[rows[declared_at[d]]["s"]][3]:toks[rows[declared_at[d]]["e"]][4]])]
        for q in range(rows[a]["s"], rows[b]["e"] + 1):
            if toks[q][1] in ("return", "continue", "break") and toks[q][0] == "id" and toks[q][2] not in p["ph"].get("allow_control", []):
                problems.append("phase %s: `%s` at line %d" % (p["ph"]["name"], toks[q][1], toks[q][2]))
        for n in p["ins"] + p["outs"]:
            if n not in types:
                problems.append("phase %s: no type for `%s`" % (p["ph"]["name"], n))
        if p["closure_outs"]:
            problems.append("phase %s: closures escape: %s" % (p["ph"]["name"], p["closure_outs"]))
    root_indent = re.match(r"\s*", lines[rows[0]["a"] - 1]).group(0)
    dedent = len(root_indent) - 4
    gen = []
    childgen = {}
    calls = {}
    for p in plan:
        ph = p["ph"]
        a, b = p["a"], p["b"]
        first = rows[a]["a"]
        while first - 2 >= 0 and lines[first - 2].strip().startswith("//") and (a == 0 or first - 1 > rows[a - 1]["b"]):
            first -= 1
        body = lines[first - 1: rows[b]["b"]]
        body = [(l[dedent:] if l.startswith(" " * dedent) else l) for l in body]
        refnames = [n for n in p["ins"] if modes.get(n, "val" if n in p["auto_val"] else "ref") == "ref" and not types[n].startswith("&")]
        for n in refnames:
            rx = re.compile(r"(?<![&\w])&%s\b(?!\s*[\.\[\(!]|::)" % re.escape(n))
            body = [rx.sub(n, l) for l in body]
        for old, new in ph.get("rewrite", []):
            body = [l.replace(old, new) for l in body]
        pl, args = [], []
        for n in p["ins"]:
            mo = modes.get(n, "val" if n in p["auto_val"] else "ref")
            ty = types[n]
            if mo == "ref":
                if ty.startswith("&"):
                    pl.append("%s: %s" % (n, ty)); args.append(n)
                else:
                    pl.append("%s: &%s" % (n, ty)); args.append("&" + n)
            elif mo == "mut":
                pl.append("%s: &mut %s" % (n, ty)); args.append("&mut " + n)
            else:
                pl.append("%s%s: %s" % ("mut " if n in ph.get("mutparams", []) else "", n, ty)); args.append(n)
        outs = p["outs"]
        struct = ph.get("struct")
        mutpfx = lambda n: "mut " if n in spec.get("mutbind", []) else ""
        cfgpfx = lambda n: (cfgs[n] + " ") if n in cfgs else ""
        if p["is_tail"]:
            ret, tail, bind = ph.get("ret", ""), "", ""
        elif struct:
            ret = struct
            tail = "%s { %s }" % (struct, ", ".join(cfgpfx(n) + n for n in outs))
            bind = "%s { %s }" % (struct, ", ".join(cfgpfx(n) + mutpfx(n) + n for n in outs))
        elif not outs:
            ret, tail, bind = "", "", ""
        elif len(outs) == 1:
            ret, tail, bind = types[outs[0]], outs[0], mutpfx(outs[0]) + outs[0]
        else:
            ret = "(%s)" % ", ".join(types[n] for n in outs)
            tail = "(%s)" % ", ".join(outs)
            bind = "(%s)" % ", ".join(mutpfx(n) + n for n in outs)
        child = ph.get("child")
        vis = "pub(super) " if child else ""
        afn = "async fn" if ph.get("async") else "fn"
        gen = ph.get("generics", "")
        sig = "%s%s %s%s(%s)%s {" % (vis, afn, ph["name"], gen, ", ".join(pl), (" -> " + ret) if ret else "")
        if len(sig) > 160:
            sig = "%s%s %s%s(\n    %s,\n)%s {" % (vis, afn, ph["name"], gen, ",\n    ".join(pl), (" -> " + ret) if ret else "")
        code = ["/// %s" % ph["doc"], sig] + body + (["    " + tail] if tail else []) + ["}"]
        bucket = gen if not child else childgen.setdefault(child["dir"], dict(doc=child["doc"], items=[], names=[]))["items"]
        if child:
            childgen[child["dir"]]["names"].append(ph["name"])
        if struct:
            fields = [x for n in outs for x in (([("    " + cfgs[n])] if n in cfgs else []) + ["    %s%s: %s," % (vis, n, types[n])])]
            bucket.append(["/// %s" % ph["struct_doc"], "%sstruct %s {" % (vis, struct)] + fields + ["}"])
            if child:
                childgen[child["dir"]]["names"].append(struct)
        bucket.append(code)
        call = "%s(%s)%s" % (ph["name"], ", ".join(args), ".await" if ph.get("async") else "")
        calls[a] = (b, ("%slet %s = %s;" % (root_indent, bind, call)) if bind else (("%s%s" % (root_indent, call)) if p["is_tail"] else "%s%s;" % (root_indent, call)))
    for p in plan:
        print("phase %-44s stmts %4d-%-4d lines %4d ins=%s outs=%s val=%s" % (p["ph"]["name"], rows[p["a"]]["a"], rows[p["b"]]["b"], rows[p["b"]]["b"] - rows[p["a"]]["a"] + 1, len(p["ins"]), len(p["outs"]), p["auto_val"]))
        if "-v" in sys.argv:
            print("    ins", p["ins"]); print("    outs", p["outs"])
    for q in problems:
        print("PROBLEM", q)
    if problems or not write:
        return not problems
    first0 = rows[0]["a"]
    while first0 - 2 >= 0 and lines[first0 - 2].strip().startswith("//"):
        first0 -= 1
    result = lines[: first0 - 1]
    i = 0
    while i < n_stm:
        if i in calls:
            b, call = calls[i]
            result.append(call)
            i = b + 1
        else:
            r = rows[i]
            first = r["a"]
            while first - 2 >= 0 and lines[first - 2].strip().startswith("//") and (i == 0 or first - 1 > rows[i - 1]["b"]):
                first -= 1
            result.extend(lines[first - 1: r["b"]])
            i += 1
    result.extend(lines[rows[-1]["b"]:])
    shift = len(result) - len(lines)
    owner_end = None
    if _owner_is_impl(lines, fn_start):
        for li in range(fn_end, len(lines)):
            if lines[li].rstrip() == "}":
                owner_end = li
                break
    if owner_end is None:
        owner_end = fn_end - 1
    pos = owner_end + shift + 1
    helpers = "\n\n".join("\n".join(g) for g in gen).split("\n") if gen else []
    out = result[:pos] + ([""] + helpers if gen else []) + result[pos:]
    if childgen:
        import os
        up = [i for i, l in enumerate(out) if l.strip() == "use super::*;"]
        ins = (up[0] + 1) if up else 0
        modlines = [""]
        for d, c in childgen.items():
            mname = re.sub(r"[^0-9a-zA-Z]+", "_", re.sub(r"^[^a-zA-Z]+", "", d)).strip("_")
            modlines += ['#[path = "%s/🦀️.rs"]' % d, "mod %s;" % mname, "use %s::{%s};" % (mname, ", ".join(c["names"])), ""]
            cdir = os.path.join(os.path.dirname(file), os.path.splitext(os.path.basename(file))[0] and "", d) if False else os.path.join(os.path.dirname(file), d)
            os.makedirs(cdir, exist_ok=True)
            body = ["%s" % c["doc"], "", "use super::*;", ""] + "\n\n".join("\n".join(g) for g in c["items"]).split("\n") + [""]
            cf = os.path.join(cdir, "🦀️.rs")
            if os.path.exists(cf):
                raise SystemExit("child exists: " + cf)
            open(cf, "w", encoding="utf-8").write("\n".join(body))
        out = out[:ins] + modlines[:-1] + out[ins:]
    open(file, "w", encoding="utf-8").write("\n".join(out))
    return True


def _owner_is_impl(lines, fn_start):
    for l in reversed(lines[: fn_start - 1]):
        if re.match(r"^(pub(\([^)]*\))?\s+)?(unsafe\s+)?impl\b", l):
            return True
        if re.match(r"^(pub(\([^)]*\))?\s+)?(fn|struct|enum|mod|trait)\b", l) or (l.strip() == "}" and not l.startswith(" ")):
            return False
    return False


def match_arms(toks, mopen, mclose):
    arms = []
    q = mopen + 1
    while q < mclose:
        ps = q
        d = 0
        while True:
            t = toks[q][1]
            if t in "([{": d += 1
            elif t in ")]}": d -= 1
            elif d == 0 and t == "=" and toks[q + 1][1] == ">": break
            q += 1
        pe = q
        q += 2
        bs = q
        if toks[q][1] == "{":
            d = 0
            while True:
                t = toks[q][1]
                if t == "{": d += 1
                elif t == "}":
                    d -= 1
                    if d == 0: break
                q += 1
            be = q
            q += 1
            block = True
        else:
            d = 0
            while q < mclose:
                t = toks[q][1]
                if t in "([{": d += 1
                elif t in ")]}": d -= 1
                elif d == 0 and t == ",": break
                q += 1
            be = q - 1
            block = False
        has_comma = q < mclose and toks[q][1] == ","
        if has_comma: q += 1
        arms.append(dict(ps=ps, pe=pe, bs=bs, be=be, block=block, comma=has_comma))
    return arms


def arms_build(spec, write):
    import os
    file = spec["file"]
    text = open(file, encoding="utf-8").read()
    lines = text.split("\n")
    toks, k, op, cl, params, rows = analyse(text, spec["function"], spec.get("nth", 0))
    types = dict(params)
    types.update(spec.get("types", {}))
    by_line = {r["a"]: i for i, r in enumerate(rows)}
    mq = next(q for q in range(op, cl) if toks[q][1] == "match" and toks[q][2] == spec["match_line"])
    si = len(rows)
    d = 0
    mopen = next(q for q in range(mq, cl) if toks[q][1] == "{")
    b = 0
    for q in range(mopen, len(toks)):
        if toks[q][1] == "{": b += 1
        elif toks[q][1] == "}":
            b -= 1
            if b == 0:
                mclose = q
                break
    arms = match_arms(toks, mopen, mclose)
    declared = list(spec["context"]) if "context" in spec else list(params)
    if "context" not in spec:
        for r in rows[:si]:
            declared += r["decl"]
    method = spec.get("method")
    order = {n: i for i, n in enumerate(declared)}
    problems = []
    edits = []
    childgen = {}
    gen = []
    by_arm_line = {toks[a["ps"]][2]: a for a in arms}
    for ar in spec["arms"]:
        a = by_arm_line.get(ar["line"])
        if not a:
            problems.append("no arm at line %d" % ar["line"]); continue
        binds = {}
        for q in range(a["ps"], a["pe"]):
            if toks[q][0] == "id" and toks[q][1] in ar.get("binds", {}):
                binds[toks[q][1]] = ar["binds"][toks[q][1]]
        for n in ar.get("binds", {}):
            if n not in binds: problems.append("arm %s: bind %s not in pattern" % (ar["name"], n))
        cands = set(declared)
        used = used_in(toks, a["bs"], a["be"], cands - set(ar.get("binds", {})))
        used_binds = [n for n in ar.get("binds", {}) if any(toks[q][1] == n and toks[q][0] == "id" for q in range(a["bs"], a["be"] + 1))]
        ins = sorted(used, key=lambda n: order[n])
        for q in range(a["bs"], a["be"] + 1):
            if not ar.get("allow_returns") and toks[q][1] in ("return", "?", "break", "continue") and toks[q][0] in ("id", "p"):
                problems.append("arm %s: control flow token `%s` at line %d" % (ar["name"], toks[q][1], toks[q][2]))
        for n in ins:
            if n not in types and not any(dv["from"] == n for dv in ar.get("derive", {}).values()): problems.append("arm %s: no type for %s" % (ar["name"], n))
        derive = ar.get("derive", {})
        for dn, dv in derive.items():
            if dv["from"] in ins:
                ins.remove(dv["from"])
        plist = ["%s: %s" % (n, ar["binds"][n]) for n in used_binds] + ["%s: %s" % (n, types.get(n, "?")) for n in ins] + ["%s: %s" % (dn, dv["type"]) for dn, dv in derive.items()]
        args = used_binds + ins + [dv["expr"] for dv in derive.values()]
        rematch = spec.get("rematch")
        if rematch:
            plist = ["%s: %s" % (n, types.get(n, "?")) for n in ins] + ["command: protocol::AppCommand"]
            args = ins + ["command"]
        arm_line = lines[toks[a["ps"]][2] - 1]
        arm_indent = len(arm_line) - len(arm_line.lstrip())
        if a["block"]:
            inner = text[toks[a["bs"]][4]:toks[a["be"]][3]]
            body = inner.split("\n")
            if body and not body[0].strip(): body = body[1:]
            if body and not body[-1].strip(): body = body[:-1]
            body = [(l[arm_indent:] if l.startswith(" " * arm_indent) else l) for l in body]
        else:
            seg = text[toks[a["bs"]][3]:toks[a["be"]][4]].split("\n")
            body = ["    " + seg[0]] + [(l[arm_indent - 4:] if l.startswith(" " * (arm_indent - 4)) else l) for l in seg[1:]]
        for dn, dv in derive.items():
            body = [l.replace(dv["expr"], dn) for l in body]
        if rematch:
            pat_text = text[toks[a["ps"]][3]:toks[a["pe"] - 1][4]]
            body = ["    let %s = command else { unreachable!(\"%s is dispatched only for its own command\") };" % (pat_text, ar["name"])] + body
        ret = ar.get("ret", spec.get("ret", "RetainedNodePaintStep"))
        if method:
            plist = [method["self_param"]] + plist
        kw = "pub(super) async fn" if (method and method.get("async")) or spec.get("async") else "pub(super) fn"
        kw = kw if not (method and method.get("vis")) else kw.replace("pub(super) ", method["vis"] + " ")
        gen = spec.get("generics", "") if (not spec.get("generics_if") or spec["generics_if"] in ins) else ""
        sig = "%s %s%s(%s) -> %s {" % (kw, ar["name"], gen, ", ".join(plist), ret)
        if len(sig) > 160:
            sig = "%s %s%s(\n    %s,\n) -> %s {" % (kw, ar["name"], gen, ",\n    ".join(plist), ret)
        attr = []
        if len(plist) > 7:
            attr = ['#[allow(clippy::too_many_arguments, reason = "%s")]' % spec.get("allow_reason", "one retained paint context plus the arm payload")]
        pre = spec.get("prelude_if")
        if pre and re.search(pre["regex"], "\n".join(body)):
            body = ["    " + pre["line"]] + body
        if spec.get("ensure_semicolon") and body and body[-1].rstrip() and not body[-1].rstrip().endswith((";", "}")):
            body[-1] = body[-1].rstrip() + ";"
        if spec.get("fall_tail"):
            body = body + ["    " + spec["fall_tail"]]
        code = ["/// %s" % ar["doc"]] + attr + [sig] + body + ["}"]
        if method:
            code = [("    " + l) if l.strip() else l for l in code]
        child = ar["child"]
        cg = childgen.setdefault(child["dir"], dict(doc=child["doc"], items=[], names=[]))
        cg["items"].append(code); cg["names"].append(ar["name"])
        args = [spec.get("argexpr", {}).get(x, x) for x in args]
        call = method["call"].format(name=ar["name"], args=", ".join(args)) if method else "%s(%s)%s" % (ar["name"], ", ".join(args), spec.get("call_suffix", ""))
        sc = toks[a["bs"]][3]; ec = toks[a["be"]][4]
        edits.append((sc, ec, call + ("" if a["comma"] else ",")))
        if rematch:
            pat_text = text[toks[a["ps"]][3]:toks[a["pe"] - 1][4]]
            new_pat = re.sub(r"\{[^}]*\}", "{ .. }", pat_text)
            edits.append((toks[a["ps"]][3], toks[a["pe"] - 1][4], new_pat))
        print("arm %-36s lines %3d ins=%s binds=%s" % (ar["name"], toks[a["be"]][2] - toks[a["bs"]][2] + 1, ins, used_binds))
    for q in problems: print("PROBLEM", q)
    if problems or not write: return not problems
    new = text
    for sc, ec, rep in sorted(edits, reverse=True):
        new = new[:sc] + rep + new[ec:]
    ml = new.split("\n")
    up = [i for i, l in enumerate(ml) if l.strip() == "use super::*;"]
    if up:
        ins_at = up[0] + 1
    else:
        ins_at = max(i for i, l in enumerate(ml[:60]) if l.startswith("use ")) + 1
    modlines = []
    for dname, c in childgen.items():
        mname = re.sub(r"[^0-9a-zA-Z]+", "_", re.sub(r"^[^a-zA-Z]+", "", dname)).strip("_")
        modlines += ["", '#[path = "%s/🦀️.rs"]' % dname, "mod %s;" % mname] + ([] if method else ["use %s::%s;" % (mname, ("{%s}" % ", ".join(c["names"])) if len(c["names"]) > 1 else c["names"][0])])
        cdir = os.path.join(os.path.dirname(file), dname)
        os.makedirs(cdir, exist_ok=False)
        items = "\n\n".join("\n".join(g) for g in c["items"]).split("\n")
        if method:
            items = (method["impl_header"].split("\n") + ["{"] if "where" in method["impl_header"] else [method["impl_header"] + " {"]) + items + ["}"]
        body = [c["doc"], "", "use super::*;", ""] + items + [""]
        open(os.path.join(cdir, "🦀️.rs"), "w", encoding="utf-8").write("\n".join(body))
    ml[ins_at:ins_at] = modlines
    open(file, "w", encoding="utf-8").write("\n".join(ml))
    return True


if __name__ == "__main__":
    c = sys.argv[1]
    if c == "table":
        nth = int(sys.argv[sys.argv.index("--nth") + 1]) if "--nth" in sys.argv else 0
        blk = int(sys.argv[sys.argv.index("--block") + 1]) if "--block" in sys.argv else None
        cmd_table(sys.argv[2], sys.argv[3], nth, blk)
    elif c in ("arms", "armsapply"):
        spec = json.load(open(sys.argv[2], encoding="utf-8"))
        sys.exit(0 if arms_build(spec, c == "armsapply") else 1)
    elif c in ("plan", "apply"):
        spec = json.load(open(sys.argv[2], encoding="utf-8"))
        sys.exit(0 if build(spec, c == "apply") else 1)
