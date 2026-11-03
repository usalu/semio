#!/usr/bin/env python3
"""🧬 Schema-sync fixer. Phase A: JSON schema := intended current Rust model. Phase B: TS, proto and GraphQL twins := JSON schema.

Usage: python3 -I 🐍️sync.py [--apply] [--phase a|b|ab] [--artifact en1991 ...] [--no-shared] [--report FILE]
Dry-run by default. Every file is re-read right before it is written; unchanged files are never touched.
Rust is never edited. Types the Rust side does not define (macro-defined diff rows, external crates) keep their JSON.
"""
import importlib.util, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("schema_check", os.path.join(HERE, "🐍️check.py"))
C = importlib.util.module_from_spec(spec)
sys.modules["schema_check"] = C
spec.loader.exec_module(C)

TYPEY = {"type", "format", "items", "$ref", "enum", "minimum", "maximum", "const"}
APPLY = False
LOG = []


def qn(n): return n if re.match(r"^[A-Za-z_]\w*$", n) else json.dumps(n)


def dump_indent(j): return json.dumps(j, indent=2, ensure_ascii=False) + "\n"


def dump_compact(d):
    c = lambda v: json.dumps(v, separators=(",", ":"), ensure_ascii=False)
    rows = []
    for k, v in d.items():
        if k in ("$defs", "definitions") and isinstance(v, dict):
            body = ",\n".join(f"    {c(dk)}:{c(dv)}" for dk, dv in v.items())
            rows.append(f"  {c(k)}:{{\n{body}\n  }}")
        else: rows.append(f"  {c(k)}:{c(v)}")
    return "{\n" + ",\n".join(rows) + "\n}\n"


def write_text(path, new, old):
    if new == old: return False
    LOG.append(path)
    if APPLY:
        cur = open(path, encoding="utf-8").read()
        if cur != old:
            LOG.append("SKIPPED (changed meanwhile) " + path)
            return False
        open(path, "w", encoding="utf-8").write(new)
    return True


def int_node(rt):
    n = {"type": "integer"}
    if rt in C.INT_BOUNDS:
        lo, hi = C.INT_BOUNDS[rt]
        n["minimum"] = lo
        if hi is not None: n["maximum"] = hi
    return n


class Builder:
    def __init__(self, defs, defkey, enums, finder):
        self.defs, self.defkey, self.enums, self.finder = defs, defkey, enums, finder
        self.pending = {}

    def node(self, t):
        """🌳️ Returns (node, optional) or None when the Rust type cannot be lowered."""
        t = re.sub(r"\s+", " ", t.strip())
        m = re.match(r"^(?:std::)?(?:option::)?Option<(.*)>$", t)
        if m:
            r = self.node(m.group(1))
            if not r: return None
            n = dict(r[0])
            if isinstance(n.get("type"), str): n["type"] = [n["type"], "null"]
            return n, True
        m = re.match(r"^(?:std::vec::)?Vec<(.*)>$", t)
        if m:
            r = self.node(m.group(1))
            return ({"type": "array", "items": r[0]}, False) if r else None
        m = re.match(r"^(?:Box|Arc|Rc)<(.*)>$", t)
        if m: return self.node(m.group(1))
        if t in ("String", "&str") or t.endswith("::String"): return {"type": "string"}, False
        if t == "bool": return {"type": "boolean"}, False
        if t in ("f32", "f64"): return {"type": "number", "format": "double" if t == "f64" else "float"}, False
        if t in C.INT_RUST: return int_node(t), False
        name = re.sub(r"<.*", "", t.split("::")[-1])
        if name in self.enums: return {"type": "string", "enum": list(self.enums[name])}, False
        if name in self.defs: return {"$ref": f"#/{self.defkey}/{name}"}, False
        cand = self.finder(name)
        if cand is not None:
            self.pending[name] = cand
            self.defs[name] = {}
            return {"$ref": f"#/{self.defkey}/{name}"}, False
        return None


def best_candidate(cands, jrec, enums):
    best = None
    for cand in cands:
        rec = C.Rec(jrec.name, C.rust_fields(cand[1], enums))
        probe = []
        C.compare_type("x", jrec.name, jrec, "rust", rec, probe, rust_meta=True)
        if best is None or len(probe) < len(best[1]): best = (cand, probe, rec)
        if not probe: break
    return best


def sync_record(name, node, cands, enums, builder, tagfirst, state_ok=True):
    """🔧️ Mutates node (a JSON record schema). Returns True when changed."""
    if not cands: return False
    jf = {}
    props = node.get("properties") or {}
    jr = C.Rec(name, [])
    req = set(node.get("required", []))
    for k, v in props.items():
        ty, nullable, bounds = C.json_type(v, {}, {})
        jr.fields.append(C.Field(k, ty, optional=(k not in req) or nullable))
    best = best_candidate(cands, jr, enums)
    cand = best[0]
    fs = cand[1][0]
    if any(f[4] for f in fs): LOG.append(f"SKIP {name}: flatten field"); return False
    rnames = [f[0] for f in fs]
    overlap = len(set(rnames) & set(props))
    if props and overlap * 2 < len(set(props)): LOG.append(f"SKIP {name}: low overlap {overlap}/{len(props)} with {cand[0]}"); return False
    newprops, newreq = {}, []
    changed = False
    for k in list(props):
        if k in tagfirst:
            newprops[k] = props[k]
            if k in req: newreq.append(k)
    for wire, ftype, st, raw, flat in fs:
        old = props.get(wire)
        rty, ropt, rt = C.rust_ty(ftype, enums)
        if old is not None:
            oty, onull, ob = C.json_type(old, {}, {})
            same = C.cmp_field_types(oty, rty, "rust") is None and ((wire not in req) or onull) == ropt
            if same and rty.kind == "integer" and rt in C.INT_BOUNDS: same = C.bounds_within(ob, C.INT_BOUNDS[rt])
            if same and rty.kind == "ref" and oty.kind == "ref": same = C.PREFIX.sub("", oty.name) == C.PREFIX.sub("", rty.name)
            if same and rty.kind == "enum" and oty.kind == "enum": same = oty.members == rty.members
            if same and rty.kind == "array" and oty.kind == "array": same = C.cmp_field_types(oty, rty, "rust") is None
            if same:
                newprops[wire] = old
                if not ropt or (wire in req and onull): newreq.append(wire)
                continue
            tonly = C.cmp_field_types(oty, rty, "rust") is None and not (rty.kind == "integer" and rt in C.INT_BOUNDS and not C.bounds_within(ob, C.INT_BOUNDS[rt]))
            if tonly and rty.kind in ("ref", "enum", "array", "string", "number", "boolean", "integer"):
                n = dict(old)
                if not ropt and isinstance(n.get("type"), list): n["type"] = next(x for x in n["type"] if x != "null")
                newprops[wire] = n
                if not ropt or (wire in req and onull): newreq.append(wire)
                changed = True
                continue
        low = builder.node(ftype)
        if low is None:
            if old is not None:
                newprops[wire] = old
                if wire in req: newreq.append(wire)
            continue
        lowered, opt = low
        if old is not None:
            n = {k: v for k, v in old.items() if k not in TYPEY}
            n2 = dict(lowered)
            if lowered.get("type") == "integer" and old.get("type") in ("integer", ["integer", "null"]):
                for bnd in ("minimum", "maximum"):
                    if bnd in old and rt in C.INT_BOUNDS and C.bounds_within((old.get("minimum"), old.get("maximum")), C.INT_BOUNDS[rt]): n2[bnd] = old[bnd]
            if lowered.get("type") == "number" and old.get("type") in ("number", ["number", "null"]):
                for bnd in ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"):
                    if bnd in old: n2[bnd] = old[bnd]
            newprops[wire] = {**n2, **n}
            changed = True
        else:
            n = dict(lowered)
            if st: n["x-semio-state"] = st
            newprops[wire] = n
            changed = True
        if not opt: newreq.append(wire)
        elif isinstance(lowered.get("type"), list) and wire in req: newreq.append(wire)
    if list(newprops) != list(props): changed = True
    if newreq != list(node.get("required", [])): changed = True
    if not changed: return False
    node["properties"] = newprops
    if newreq: node["required"] = newreq
    else: node.pop("required", None)
    if "type" not in node: node["type"] = "object"
    return True


def phase_a(unit):
    key, base, d, twins = unit
    if "🦀️.rs" not in twins and not (os.path.basename(d) == "🧬️schema" and "🧬️mutations" in d): return
    path = os.path.join(d, "🔣️.json")
    text = open(path, encoding="utf-8").read()
    j = json.loads(text)
    if "$schema" not in j: return
    fmt = "indent" if dump_indent(j) == text else "compact" if dump_compact(j) == text else "indent"
    defkey = "$defs" if "$defs" in j else "definitions" if "definitions" in j else "$defs"
    defs = j.get(defkey, {})
    C.REF_CTX["dir"] = d
    jm = C.parse_json(path)
    rm = C.build_rust(d, base, list(jm.recs))
    enums = rm.enums
    cache = {}

    def finder(name):
        if name in cache: return cache[name]
        own, _ = C.rust_module(d, base, [name])
        ix, _ = C.artifact_index(base)
        c = own.get(name) or ix.get(name) or C.shared_index().get(name)
        cache[name] = c
        return c

    builder = Builder(defs, defkey, enums, finder)
    changed = False
    targets = []
    if j.get("properties") is not None: targets.append((j.get("title"), j))
    for n, dn in list(defs.items()):
        if isinstance(dn, dict) and "properties" in dn: targets.append((n, dn))
    for n, nd in targets:
        cands = rm.cands.get(n)
        if not cands:
            if os.path.basename(d) == "🔺️diff" and nd.get("properties") and "required" not in nd and n != j.get("title"):
                nd["required"] = list(nd["properties"])
                changed = True
            continue
        if sync_record(n, nd, cands, enums, builder, {"mutation"}): changed = True
    guard = 0
    while builder.pending and guard < 50:
        guard += 1
        name, cand = builder.pending.popitem()
        nd = {"type": "object", "additionalProperties": False}
        defs[name] = nd
        sync_record(name, nd, cand, enums, builder, set())
        changed = True
    if defs and defkey not in j: j[defkey] = defs
    if changed:
        write_text(path, dump_compact(j) if fmt == "compact" else dump_indent(j), text)


# ------------------------------------------------------------------ Phase B: twins
def reader_expr(f, ts_aliases=None):
    ty = f.ty

    def inner(t, node):
        if t.kind == "string": return "normWireString"
        if t.kind == "boolean": return "normWireBoolean"
        if t.kind == "number": return "normWireNumber"
        if t.kind == "integer":
            b = {k: v for k, v in (("minimum", node.get("minimum")), ("maximum", node.get("maximum"))) if v is not None} if node else {}
            return f"normWireRange(normWireInteger, {json.dumps(b, separators=(',', ':'))})" if b else "normWireInteger"
        if t.kind == "enum": return "normWireLiteral(" + ", ".join(json.dumps(m, ensure_ascii=False) for m in t.members) + ")"
        if t.kind == "array":
            return f"normWireArray({inner(t.inner, (node or {}).get('items') if node else None)})" if t.inner else "normWireArray(normWireJson)"
        if t.kind == "ref": return f"normWireRef(() => parse{t.name})"
        return "normWireJson"

    base = inner(ty, f.extra.get("node"))
    nd = f.extra.get("node") or {}
    return f"normWireNullable({base})" if isinstance(nd.get("type"), list) and "null" in nd["type"] else base


def ts_type_str(t, prefix_lookup):
    if t.kind in ("string", "boolean"): return t.kind
    if t.kind in ("number", "integer"): return "number"
    if t.kind == "enum": return " | ".join(json.dumps(m, ensure_ascii=False) for m in t.members)
    if t.kind == "array":
        i = ts_type_str(t.inner, prefix_lookup) if t.inner else "unknown"
        return f"({i})[]" if " | " in i else f"{i}[]"
    if t.kind == "ref": return prefix_lookup(t.name)
    return "unknown"


def jnodes(j):
    defkey = "$defs" if "$defs" in j else "definitions"
    out = {}
    if j.get("properties") is not None: out[j.get("title")] = j
    for n, d in (j.get(defkey) or {}).items():
        if isinstance(d, dict) and "properties" in d: out[n] = d
    return out


def match_brace_end(text, i):
    depth = 1
    while i < len(text) and depth:
        if text[i] == "{": depth += 1
        elif text[i] == "}": depth -= 1
        i += 1
    return i


def phase_b_ts(d, jm, jj):
    path = os.path.join(d, "🟦️.ts")
    text = open(path, encoding="utf-8").read()
    tm = C.parse_ts(path)
    new = text
    nodes = jnodes(jj)
    for n, jr in jm.recs.items():
        tr = C.find_rec(tm.recs, n)
        if tr is None or (not jr.fields and not tr.fields): continue
        probe = []
        C.compare_type("x", n, jr, "ts", tr, probe)
        rd = tm.readers.get(tr.name)
        rprobe = []
        if rd is not None:
            if [x[0] for x in rd] != [f.name for f in jr.fields]: rprobe.append(1)
            else:
                for f, (rn, req, lit, rbd, _raw) in zip(jr.fields, rd):
                    if req == f.optional: rprobe.append(1)
                    if lit and f.ty.kind == "enum" and lit != f.ty.members: rprobe.append(1)
                    node = nodes[n]["properties"].get(f.name, {})
                    want = {k: v for k, v in (("minimum", node.get("minimum")), ("maximum", node.get("maximum"))) if v is not None} if f.ty.kind == "integer" else None
                    if rbd and f.ty.kind == "integer" and rbd != (want or {}): rprobe.append(1)
        broken = tr.name in tm.broken
        if broken: rd = None
        if not probe and not rprobe and not broken: continue
        props = nodes[n]["properties"]
        old_docs = {}
        im = re.search(r"export interface " + re.escape(tr.name) + r"(?:<[^>]*>)?\s*\{", new)
        end = match_brace_end(new, im.end())
        body = new[im.end():end - 1]
        multiline = "\n" in body.strip()
        for it in C.split_top(body, ";"):
            mm = re.search(r'(?:"([^"]+)"|\b(\w+))\??\s*:', re.sub(r"/\*.*?\*/", "", it, flags=re.S))
            if mm:
                doc = re.search(r"/\*\*.*?\*/", it, flags=re.S)
                old_docs[mm.group(1) or mm.group(2)] = doc.group(0) if doc else None

        def pref(refname, tm=tm):
            for x in tm.recs:
                if C.PREFIX.sub("", x) == C.PREFIX.sub("", refname): return x
            return refname

        lines, rentries = [], []
        oldf = {f.name: f for f in tr.fields}
        oldr = {x[0]: x for x in (rd or [])}
        for f in jr.fields:
            f.extra["node"] = props.get(f.name)
            o = oldf.get(f.name)
            ok = o is not None and C.cmp_field_types(f.ty, o.ty, "ts") is None and f.optional == o.optional
            if ok:
                lines.append((o.extra.get("doc"), o.extra["raw"]))
            else:
                tys = ts_type_str(f.ty, pref)
                if f.ty.kind == "enum":
                    alias = next((a for a, mem in tm.aliases.items() if mem == f.ty.members), None)
                    if alias: tys = alias
                nullable = isinstance(props.get(f.name, {}).get("type"), list)
                ftxt = f"{qn(f.name)}{'?' if f.optional and not nullable else ''}: {tys}{' | null' if nullable else ''}"
                doc = (o.extra.get("doc") if o else None) or (f"/** @state {f.state} */" if f.state else None)
                lines.append((doc, ftxt))
            r0 = oldr.get(f.name)
            node = props.get(f.name, {})
            want = {k: v for k, v in (("minimum", node.get("minimum")), ("maximum", node.get("maximum"))) if v is not None} if f.ty.kind == "integer" else None
            rok = ok and r0 is not None and r0[1] == (not f.optional) and (not r0[2] or r0[2] == f.ty.members) and (not r0[3] or r0[3] == (want or {}))
            if rok: rentries.append(f"{f.name}: {r0[4].strip()}")
            else:
                rv = reader_expr(f)
                rentries.append(f"{qn(f.name)}: " + (f"normWireOptional({rv})" if f.optional else f"normWireRequired({rv})"))
        if multiline:
            nb = "\n" + "".join((f"  {doc}\n" if doc else "") + f"  {ftxt};\n" for doc, ftxt in lines)
        else:
            nb = " " + " ".join(f"{ftxt};" for _, ftxt in lines) + " "
        new = new[:im.end()] + nb + new[end - 1:]
        rm = re.search(r"export const parse" + re.escape(tr.name) + r":\s*NormWireReader<\w+>\s*=\s*normWireObject<\w+>\(\{", new)
        if rm:
            rend = match_brace_end(new, rm.end())
            new = new[:rm.end()] + " " + ", ".join(rentries) + " " + new[rend - 1:]
    for e in extras_of(jm, tm.recs):
        nor = re.sub(r"\nexport const parse" + re.escape(e) + r"\b[^\n]*\n", "\n", new)
        dropped = drop_extras(nor, [e], r"export interface {name}(?:<[^>]*>)?\s*\{{")
        if dropped != nor: new = dropped
    mentioned = lambda n: re.search(r"\b" + re.escape(n) + r"\b", new) is not None
    for n, jr in jm.recs.items():
        if C.find_rec(tm.recs, n) is not None or not jr.fields or mentioned(n): continue
        props = nodes[n]["properties"]
        body, ents = [], []
        for f in jr.fields:
            f.extra["node"] = props.get(f.name)
            nullable = isinstance(props.get(f.name, {}).get("type"), list)
            body.append(f"  {qn(f.name)}{'?' if f.optional and not nullable else ''}: {ts_type_str(f.ty, lambda x: x)}{' | null' if nullable else ''};")
            rv = reader_expr(f)
            ents.append(f"{qn(f.name)}: " + (f"normWireOptional({rv})" if f.optional else f"normWireRequired({rv})"))
        new = new.rstrip("\n") + f"\n\nexport interface {n} {{\n" + "\n".join(body) + f"\n}}\n\nexport const parse{n}: NormWireReader<{n}> = normWireObject<{n}>({{ " + ", ".join(ents) + " });\n"
    used = set(re.findall(r"\bnormWire\w+\b", new))
    im = re.search(r"import \{([^}]*)\} from \"@semio-tech/norm-js/registry/contract\";", new)
    if im:
        have = {x.strip().replace("type ", "") for x in im.group(1).split(",") if x.strip()}
        add = sorted(x for x in used if x not in have and x != "NormWireReader")
        if add:
            names = [x.strip() for x in im.group(1).split(",") if x.strip()] + add
            new = new[:im.start(1)] + " " + ", ".join(names) + " " + new[im.end(1):]
    write_text(path, new, text)


def proto_type(f, protoenums, protomsgs, ref_ok):
    t = f.ty
    rt = f.extra.get("rt")
    if t.kind == "string": return "string"
    if t.kind == "boolean": return "bool"
    if t.kind == "number": return "double"
    if t.kind == "integer": return "uint32"
    if t.kind == "enum":
        for n, mem in protoenums.items():
            if [C.norm_member(x) for x in mem] == [C.norm_member(x) for x in t.members]: return n
        return "string"
    if t.kind == "array":
        i = t.inner
        if i and i.kind in ("string", "boolean", "number", "integer", "ref"):
            inner = proto_type(C.Field("x", i, False), protoenums, protomsgs, ref_ok)
            return "repeated " + inner if inner != "string" or i.kind == "string" else "string"
        return "string"
    if t.kind == "ref":
        r = C.find_rec(protomsgs, t.name)
        return r.name if r is not None else "string"
    return "string"


def snake(n):
    return re.sub(r"([A-Z])", r"_\1", n).lower()


def drop_extras(new, extras, pattern):
    for name in extras:
        mm = re.search(pattern.format(name=re.escape(name)), new)
        if not mm: continue
        end = match_brace_end(new, mm.end())
        rest = new[:mm.start()] + new[end:]
        if re.search(r"\b" + re.escape(name) + r"\b", re.sub(r"(^|\n)\s*#[^\n]*|//[^\n]*", "", rest)): continue
        new = re.sub(r"\n{3,}", "\n\n", rest)
    return new


def rename_extras(new, jm, twin_recs, find):
    """🔁️ A twin type whose name differs from the JSON record of the same fields is renamed to the JSON name."""
    extras = extras_of(jm, twin_recs)
    for e in extras:
        names = {f.name for f in twin_recs[e].fields}
        best, score = None, 0.0
        for y, jr in jm.recs.items():
            if find(twin_recs, y) is not None or not names: continue
            jn = {f.name for f in jr.fields}
            sc = len(jn & names) / max(1, len(jn | names))
            if sc > score: best, score = y, sc
        if best and score >= 0.5: new = re.sub(r"\b" + re.escape(e) + r"\b", best, new)
    return new


def extras_of(jm, twin_recs):
    known = {C.PREFIX.sub("", x) for x in jm.recs} | {C.PREFIX.sub("", x) for x in jm.refd}
    return [n for n in twin_recs if jm.recs and C.PREFIX.sub("", n) not in known and n not in jm.recs and n != "Empty"]


def phase_b_proto(d, jm, jj):
    path = os.path.join(d, "🛰️.proto")
    text = open(path, encoding="utf-8").read()
    pm = C.parse_proto(path)
    new = rename_extras(text, jm, pm.recs, C.find_rec)
    if new != text:
        write_text(path, new, text)
        text = new
        pm = C.parse_proto(path) if APPLY else pm
    for n, jr in jm.recs.items():
        pr = C.find_rec(pm.recs, n)
        if pr is None or (not jr.fields and not pr.fields): continue
        probe = []
        C.compare_type("x", n, jr, "proto", pr, probe)
        if not probe: continue
        mm = re.search(r"\bmessage\s+" + re.escape(pr.name) + r"\s*\{", new)
        end = match_brace_end(new, mm.end())
        body = new[mm.end():end - 1]
        if re.search(r"\b(oneof|message|enum)\b", body): LOG.append(f"SKIP proto {pr.name}: nested constructs"); continue
        oneline = "\n" not in body.strip()
        old_fields = {f.name: f for f in pr.fields}
        parts = []
        for i, f in enumerate(jr.fields, 1):
            o = old_fields.get(f.name)
            ptxt = None
            if o is not None and C.cmp_field_types(f.ty, o.ty, "proto") is None and (o.ty.kind in ("array",) or True):
                ptxt = None
            tstr = proto_type(f, pm.enums, pm.recs, True)
            if o is not None and C.cmp_field_types(f.ty, o.ty, "proto") is None:
                old_line = next((l for l in body.splitlines() if re.search(r"\b" + re.escape(o.extra.get("snake", "")) + r"\s*=\s*\d+", l)), None)
                if old_line and (f.optional == o.optional):
                    m2 = re.match(r"^\s*(?:repeated\s+|optional\s+)?([\w.]+)\s+\w+\s*=", old_line)
                    if m2:
                        tstr = ("repeated " if re.match(r"^\s*repeated", old_line) else "") + m2.group(1)
            opt = "optional " if f.optional and not tstr.startswith("repeated") else ""
            st = f.state or (o.state if o else None)
            parts.append((st, f"{opt}{tstr} {snake(f.name)} = {i};"))
        if oneline:
            nb = " " + " ".join((f"/* @state {st} */ " if st else "") + t for st, t in parts) + " "
        else:
            nb = "\n" + "".join((f"  // @state {st}\n" if st else "") + f"  {t}\n" for st, t in parts)
        new = new[:mm.end()] + nb + new[end - 1:]
    new = drop_extras(new, extras_of(jm, pm.recs), r"\bmessage\s+{name}\s*\{{")
    write_text(path, new, text)


def gql_type(f, gqlenums, gqltypes):
    t = f.ty

    def base(t):
        if t.kind == "string": return "String"
        if t.kind == "boolean": return "Boolean"
        if t.kind == "number": return "Float"
        if t.kind == "integer": return "Int"
        if t.kind == "enum":
            for n, mem in gqlenums.items():
                if [C.norm_member(x) for x in mem] == [C.norm_member(x) for x in t.members]: return n
            return "String"
        if t.kind == "array": return "[" + (base(t.inner) if t.inner else "JSON") + "!]"
        if t.kind == "ref": return t.name if t.name in gqltypes else "JSON"
        return "JSON"

    if t.kind == "array" and (not t.inner or t.inner.kind in ("ref", "array")) and not (t.inner and t.inner.kind == "ref" and t.inner.name in gqltypes): return "[JSON!]" + ("" if f.optional else "!")
    b = base(t)
    return b + ("" if f.optional else "!")


def phase_b_gql(d, jm, jj):
    path = os.path.join(d, "🔗️.graphql")
    text = open(path, encoding="utf-8").read()
    gm = C.parse_graphql(path)
    new = rename_extras(text, jm, gm.recs, C.find_rec)
    if new != text:
        write_text(path, new, text)
        text = new
        gm = C.parse_graphql(path) if APPLY else gm
    for n, jr in jm.recs.items():
        gr = C.find_rec(gm.recs, n)
        if gr is None or (not jr.fields and not gr.fields): continue
        probe = []
        C.compare_type("x", n, jr, "graphql", gr, probe)
        if not probe: continue
        mm = re.search(r"\b(?:type|input)\s+" + re.escape(gr.name) + r"(?:\s+implements [^{]+)?\s*\{", new)
        end = match_brace_end(new, mm.end())
        body = new[mm.end():end - 1]
        oneline = "\n" not in body.strip()
        old_fields = {f.name: f for f in gr.fields}
        parts = []
        for f in jr.fields:
            o = old_fields.get(f.name)
            t = gql_type(f, gm.enums, gm.recs)
            if o is not None and C.cmp_field_types(f.ty, o.ty, "graphql") is None and f.optional == o.optional:
                m2 = re.search(r"^\s*" + re.escape(f.name) + r"\s*(?:\([^)]*\))?\s*:\s*([\[\]\w!]+)", body, re.M)
                if m2: t = m2.group(1)
            st = f.state or (o.state if o else None)
            parts.append(f"{f.name}: {t}" + (f" @state(class: {st.upper()})" if st else ""))
        nb = " " + " ".join(parts) + " " if oneline else "\n" + "".join(f"  {p}\n" for p in parts)
        new = new[:mm.end()] + nb + new[end - 1:]
    new = drop_extras(new, extras_of(jm, gm.recs), r"\b(?:type|input)\s+{name}(?:\s+implements [^{{]+)?\s*\{{")
    write_text(path, new, text)


def phase_b(unit):
    key, base, d, twins = unit
    path = os.path.join(d, "🔣️.json")
    C.REF_CTX["dir"] = d
    jm = C.parse_json(path)
    if jm is None or not jm.recs: return
    jm.refd = {x.split("/")[-1] for x in re.findall(r'"\$ref":\s*"([^"]+)"', open(path, encoding="utf-8").read())}
    jj = json.load(open(path, encoding="utf-8"))
    if "🟦️.ts" in twins: phase_b_ts(d, jm, jj)
    if "🛰️.proto" in twins: phase_b_proto(d, jm, jj)
    if "🔗️.graphql" in twins: phase_b_gql(d, jm, jj)


def main(argv):
    global APPLY
    arts, shared, phase, rep = [], True, "ab", None
    i = 0
    while i < len(argv):
        a = argv[i]
        if a == "--apply": APPLY = True
        elif a == "--phase": phase = argv[i + 1]; i += 1
        elif a == "--artifact": arts.append(argv[i + 1]); i += 1
        elif a == "--no-shared": shared = False
        elif a == "--report": rep = argv[i + 1]; i += 1
        i += 1
    units = C.discover_units(arts or C.ARTIFACTS, shared, [], False)
    for u in units:
        try:
            if "a" in phase: phase_a(u)
            if "b" in phase: phase_b(u)
        except Exception as e:
            LOG.append(f"ERROR {u[2]}: {type(e).__name__}: {e}")
    print(("applied" if APPLY else "dry-run"), "files changed:", len([x for x in LOG if not x.startswith(("SKIP", "ERROR"))]))
    for x in LOG:
        if x.startswith(("SKIP", "ERROR")): print(x)
    if rep: open(rep, "w", encoding="utf-8").write("\n".join(LOG) + "\n")


if __name__ == "__main__":
    main(sys.argv[1:])
