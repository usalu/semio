#!/usr/bin/env python3
"""🧬 Schema-sync checker: the JSON schema (source of truth) against the Rust, TypeScript, proto and GraphQL twins.

Usage: python3 -I 🐍️check.py [--root DIR] [--artifact en1991 ...] [--facet snapshot ...] [--all] [--limit N] [--summary] [--json OUT]
Exit code 1 when any mismatch is reported.

Compared per type: field names (wire camelCase), order, type, optional/required, enum members and order, state class.
"""
import json, os, re, sys

def find_norm():
    env = os.environ.get("SEMIO_NORM_ROOT")
    if env: return env
    here = os.path.dirname(os.path.abspath(__file__))
    while True:
        if os.path.basename(here) == "📕️norm": return here
        up = os.path.dirname(here)
        if up == here: break
        here = up
    return os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 9, "✏️s", "🔌️plugins", "📕️norm"))


NORM = find_norm()
ARTIFACTS = ["en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"]
SKIP_PARTS = {"node_modules", "🧫️fixtures", "🧪️tests", "target", "🗑️generated"}
INT_RUST = {"u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize", "u128", "i128"}
INT_BOUNDS = {"u8": (0, 255), "u16": (0, 65535), "u32": (0, 4294967295), "u64": (0, None), "usize": (0, None)}
FLOAT_RUST = {"f32", "f64"}


def words(s):
    s = s.replace("-", "_")
    out = []
    for part in s.split("_"):
        out += re.findall(r"[A-Z]+(?![a-z])|[A-Z]?[a-z]+|\d+", part) or ([part] if part else [])
    return out


def camel(s):
    segs = s.split("_")
    return segs[0] + "".join(x[:1].upper() + x[1:] for x in segs[1:])


def apply_case(name, rule):
    w = [x.lower() for x in words(name)]
    if rule == "camelCase": return camel(name) if "_" in name or name[:1].islower() else name[:1].lower() + name[1:]
    if rule == "PascalCase": return "".join(x.capitalize() for x in w)
    if rule == "snake_case": return "_".join(w)
    if rule == "kebab-case": return "-".join(w)
    if rule == "SCREAMING_SNAKE_CASE": return "_".join(w).upper()
    if rule == "lowercase": return name.lower()
    if rule == "UPPERCASE": return name.upper()
    return name


def norm_member(s):
    return re.sub(r"[^a-z0-9]", "", s.lower())


class Ty:
    """🏷️ Canonical type: kind in string|number|integer|boolean|array|enum|ref|opaque|any."""
    __slots__ = ("kind", "inner", "members", "name")

    def __init__(self, kind, inner=None, members=None, name=None):
        self.kind, self.inner, self.members, self.name = kind, inner, members, name

    def show(self):
        if self.kind == "array": return f"[{self.inner.show() if self.inner else '?'}]"
        if self.kind == "enum": return "enum(" + "|".join(self.members) + ")" + (f":{self.name}" if self.name else "")
        if self.kind in ("ref", "opaque"): return self.name
        return self.kind


PREFIX = re.compile(r"^(?:En\d{4}|Din\d+|Iso\d+|Vdi\d+)(?=[A-Z])")


def find_rec(recs, name):
    if name in recs: return recs[name]
    for n, r in recs.items():
        if PREFIX.sub("", n) == name or n.endswith(name) and PREFIX.match(n) and PREFIX.sub("", n) == name: return r
    return None


def compatible(a, b):
    """⚖️ Soft equality: opaque and any match everything of a plausible kind; refs compare by name; enums by members."""
    if a.kind == "any" or b.kind == "any": return True
    if a.kind == "opaque" or b.kind == "opaque":
        o = b if a.kind == "opaque" else a
        return o.kind in ("string", "enum", "ref", "opaque", "integer", "number", "boolean", "array")
    if a.kind == "ref" and b.kind == "enum" or a.kind == "enum" and b.kind == "ref":
        e, r = (a, b) if a.kind == "enum" else (b, a)
        return not e.name or norm_member(e.name) == norm_member(r.name)
    if a.kind != b.kind: return False
    if a.kind == "array": return compatible(a.inner, b.inner) if a.inner and b.inner else True
    if a.kind == "enum": return a.members == b.members
    if a.kind == "ref": return norm_member(a.name) == norm_member(b.name)
    return True


class Field:
    __slots__ = ("name", "ty", "optional", "state", "extra")

    def __init__(self, name, ty, optional=False, state=None, extra=None):
        self.name, self.ty, self.optional, self.state, self.extra = name, ty, optional, state, extra or {}


class Rec:
    def __init__(self, name, fields, origin=""):
        self.name, self.fields, self.origin = name, fields, origin


class Module:
    def __init__(self):
        self.recs = {}
        self.enums = {}


def split_top(s, sep=","):
    out, depth, cur, i, instr = [], 0, [], 0, False
    while i < len(s):
        c = s[i]
        if instr:
            cur.append(c)
            if c == "\\": cur.append(s[i + 1]); i += 1
            elif c == '"': instr = False
        elif c == '"': instr = True; cur.append(c)
        elif c in "<([{": depth += 1; cur.append(c)
        elif c in ">)]}":
            if c == ">" and i > 0 and s[i - 1] in "-=": cur.append(c)
            else: depth -= 1; cur.append(c)
        elif c == sep and depth == 0: out.append("".join(cur)); cur = []
        else: cur.append(c)
        i += 1
    if "".join(cur).strip(): out.append("".join(cur))
    return out


def strip_line_comments(s):
    return re.sub(r"(^|\s)//[^\n]*", r"\1", s)


def resolve_ref(name, enums):
    e = enums.get(name)
    return Ty("enum", members=e, name=name) if e is not None else Ty("ref", name=name)


# ------------------------------------------------------------------ JSON
REF_CTX = {"dir": None}
_ref_cache = {}


def resolve_property_ref(ref):
    """🧭️ `.../<facet>.json#/<pointer containing properties>`: the node of the sibling facet schema of the current unit."""
    m = re.search(r"/([a-z]+)\.json#(/.*)$", ref)
    d = REF_CTX["dir"]
    if not m or not d: return None
    facet = {"snapshot": "📸️snapshot", "diff": "🔺️diff", "artifact": ""}.get(m.group(1))
    if facet is None: return None
    base = d
    if os.path.basename(d) in ("📸️snapshot", "🔺️diff", "💡️inferences"): base = os.path.dirname(d)
    p = os.path.join(base, facet, "🔣️.json") if facet else os.path.join(base, "🔣️.json")
    if p not in _ref_cache:
        try: _ref_cache[p] = json.load(open(p, encoding="utf-8"))
        except Exception: _ref_cache[p] = {}
    node = _ref_cache[p]
    for seg in m.group(2).strip("/").split("/"):
        if not isinstance(node, dict) or seg not in node: return None
        node = node[seg]
    return node


def json_type(node, defs, enums):
    """🧩️ Returns (Ty, nullable, bounds)."""
    nullable = False
    for key in ("anyOf", "oneOf"):
        br = node.get(key)
        if isinstance(br, list) and any(isinstance(x, dict) and x.get("type") == "null" for x in br):
            rest = [x for x in br if not (isinstance(x, dict) and x.get("type") == "null")]
            if len(rest) == 1:
                ty, _, bo = json_type(rest[0], defs, enums)
                return ty, True, bo
    t = node.get("type")
    if isinstance(t, list):
        nullable = "null" in t
        t = next((x for x in t if x != "null"), None)
    if "$ref" in node:
        if "/properties/" in node["$ref"]:
            tgt = resolve_property_ref(node["$ref"])
            if tgt is None: return Ty("any"), nullable, None
            ty, nl, bo = json_type(tgt, defs, enums)
            return ty, nullable or nl, bo
        n = node["$ref"].split("/")[-1]
        return resolve_ref(n, enums), nullable, None
    if "enum" in node:
        return Ty("enum", members=[str(x) for x in node["enum"]]), nullable, None
    if "const" in node and isinstance(node["const"], str):
        return Ty("enum", members=[node["const"]]), nullable, None
    if t == "array":
        it = node.get("items")
        inner = json_type(it, defs, enums)[0] if isinstance(it, dict) else None
        return Ty("array", inner=inner), nullable, None
    if t in ("string", "number", "integer", "boolean"):
        return Ty(t), nullable, (node.get("minimum"), node.get("maximum")) if t == "integer" else None
    if t == "object": return Ty("object"), nullable, None
    return Ty("any"), nullable, None


def parse_json(path):
    j = json.load(open(path, encoding="utf-8"))
    if "$schema" not in j: return None
    defs = j.get("$defs") or j.get("definitions") or {}
    enums = {n: [str(x) for x in d["enum"]] for n, d in defs.items() if isinstance(d, dict) and "enum" in d}
    m = Module()
    m.enums = enums

    def rec(name, node):
        req = set(node.get("required", []))
        fs = []
        for k, v in (node.get("properties") or {}).items():
            ty, nullable, bounds = json_type(v, defs, enums)
            fs.append(Field(k, ty, optional=(k not in req) or nullable, state=v.get("x-semio-state"), extra={"bounds": bounds}))
        m.recs[name] = Rec(name, fs)

    if j.get("properties") is not None: rec(j.get("title") or "Root", j)
    for n, d in defs.items():
        if isinstance(d, dict) and "properties" in d: rec(n, d)
    m.title = j.get("title")
    return m


# ------------------------------------------------------------------ Rust
RS_ITEM = re.compile(r"((?:[ \t]*(?:#\[[^\n]*\]|///[^\n]*)[ \t]*\n)*)[ \t]*pub(?:\([a-z]+\))?[ \t]+(struct|enum)[ \t]+(\w+)(?:<[^>{]*>)?[ \t]*(\{|\(|;)", re.M)
_rs_cache = {}


def rust_attr_rename_all(attrs):
    m = re.findall(r'rename_all\s*=\s*"([^"]+)"', attrs)
    return m[0] if m else None


def rust_ty(t, enums, structs_known=None):
    t = t.strip()
    t = re.sub(r"^&?'?\w*\s*", "", t) if t.startswith("&") else t
    m = re.match(r"^(?:std::)?(?:option::)?Option<(.*)>$", t, re.S)
    if m:
        ty, _, rt = rust_ty(m.group(1), enums)
        return ty, True, rt
    m = re.match(r"^(?:std::vec::)?Vec<(.*)>$", t, re.S)
    if m: return Ty("array", inner=rust_ty(m.group(1), enums)[0]), False, "Vec"
    m = re.match(r"^(?:Box|Arc|Rc)<(.*)>$", t, re.S)
    if m: return rust_ty(m.group(1), enums)
    if t in ("String", "&str", "str") or t.endswith("::String"): return Ty("string"), False, "String"
    if t == "bool": return Ty("boolean"), False, "bool"
    if t in FLOAT_RUST: return Ty("number"), False, t
    if t in INT_RUST: return Ty("integer"), False, t
    name = re.split(r"::", t)[-1]
    name = re.sub(r"<.*", "", name)
    if name in enums: return Ty("enum", members=enums[name], name=name), False, name
    return Ty("ref", name=name), False, name


def parse_rust_text(text, global_enums=None):
    """📇️ Returns (structs name->(fields raw), enums name->members wire list)."""
    text = strip_line_comments(text)
    structs, enums_raw = {}, {}
    for m in RS_ITEM.finditer(text):
        attrs, kind, name, opener = m.group(1), m.group(2), m.group(3), m.group(4)
        if opener != "{": continue
        i = m.end()
        depth, j = 1, i
        while j < len(text) and depth:
            if text[j] == "{": depth += 1
            elif text[j] == "}": depth -= 1
            j += 1
        body = text[i:j - 1]
        rule = rust_attr_rename_all(attrs)
        if kind == "enum":
            members, complex_ = [], False
            for it in split_top(body):
                it = it.strip()
                if not it: continue
                ats = "".join(re.findall(r"#\[[^\]]*(?:\[[^\]]*\][^\]]*)*\]", it))
                head = re.sub(r"#\[[^\n]*\]\s*", "", it).strip()
                mm = re.match(r"(?:r#)?(\w+)", head)
                if not mm: continue
                vn = mm.group(1)
                if re.match(r"\w+\s*[\(\{]", head): complex_ = True
                ren = re.findall(r'(?:serde|value)\s*\([^\n]*?rename\s*=\s*"([^"]+)"', ats)
                wire = ren[0] if ren else apply_case(vn, rule) if rule else vn
                if re.search(r"serde\([^)]*skip\b", ats): continue
                members.append(wire)
            enums_raw[name] = (members, complex_)
        else:
            fs = []
            for it in split_top(body):
                it = it.strip()
                if not it: continue
                ats = "".join(re.findall(r"#\[(?:[^\[\]]|\[[^\]]*\])*\]", it))
                head = re.sub(r"#\[(?:[^\[\]]|\[[^\]]*\])*\]\s*", "", it).strip()
                mm = re.match(r"pub(?:\([a-z]+\))?\s+(?:r#)?(\w+)\s*:\s*(.*)$", head, re.S)
                if not mm: continue
                fname, ftype = mm.group(1), re.sub(r"\s+", " ", mm.group(2).strip())
                if re.search(r"(serde|value)\([^)]*\bskip\b", ats) or "#[serde(skip" in ats: continue
                ren = re.findall(r'(?:serde|value)\s*\([^\n]*?\brename\s*=\s*"([^"]+)"', ats)
                st = re.findall(r"#\[state\((\w+)\)\]", ats)
                wire = ren[0] if ren else apply_case(fname, rule) if rule else fname
                fs.append((wire, ftype, st[0] if st else None, fname, "flatten" in ats))
            dflt = bool(re.search(r"serde\([^\n]*\bdefault\b", attrs))
            structs[name] = (fs, bool(rule), dflt)
    return structs, enums_raw


def rust_files(d):
    out = []
    for r, ds, fs in os.walk(d):
        ds[:] = [x for x in ds if x not in SKIP_PARTS]
        if "🦀️.rs" in fs: out.append(os.path.join(r, "🦀️.rs"))
    return out


_global = {"enums": None}


def global_enums():
    if _global["enums"] is None:
        enums = {}
        for r, ds, fs in (w for root in (os.path.join(NORM, "⚖️compliance"), NORM) for w in os.walk(root)):
            ds[:] = [x for x in ds if x not in SKIP_PARTS and x != "📦️packages"]
            for f in fs:
                if f != "🦀️.rs": continue
                p = os.path.join(r, f)
                try: t = open(p, encoding="utf-8").read()
                except Exception: continue
                if "pub enum" not in t: continue
                _, er = parse_rust_text(t)
                for n, (mem, cx) in er.items():
                    if not cx: enums.setdefault(n, mem)
        _global["enums"] = enums
    return _global["enums"]


def artifact_index(base):
    if base in _rs_cache: return _rs_cache[base]
    structs, enums = {}, {}
    for p in rust_files(base):
        try: t = open(p, encoding="utf-8").read()
        except Exception: continue
        if "pub struct" not in t and "pub enum" not in t: continue
        s, e = parse_rust_text(t)
        for n, v in s.items(): structs.setdefault(n, []).append((p, v))
        for n, (mem, cx) in e.items():
            if not cx: enums.setdefault(n, mem)
    _rs_cache[base] = (structs, enums)
    return structs, enums


def rust_module(unit_dir, art_base, wanted):
    """🗂️ Collects structs for the wanted type names: first the unit's own Rust files, then the artifact index."""
    cands = {}
    own = [os.path.join(unit_dir, "🦀️.rs")]
    if os.path.basename(unit_dir) == "🧬️schema" and "🧬️mutations" in unit_dir: own.append(os.path.join(os.path.dirname(unit_dir), "🦀️.rs"))
    own_structs, own_enums = {}, {}
    for p in own:
        if os.path.exists(p):
            s, e = parse_rust_text(open(p, encoding="utf-8").read())
            for n, v in s.items(): own_structs.setdefault(n, []).append((p, v))
            for n, (mem, cx) in e.items():
                if not cx: own_enums[n] = mem
    return own_structs, own_enums


_shared = {}


def shared_index():
    if "ix" not in _shared:
        ix = {}
        for r, ds, fs in os.walk(NORM):
            ds[:] = [x for x in ds if x not in SKIP_PARTS and x != "🗿️artifacts" and x != "📦️packages"]
            if "🦀️.rs" in fs:
                p = os.path.join(r, "🦀️.rs")
                try: t = open(p, encoding="utf-8").read()
                except Exception: continue
                if "pub struct" in t:
                    for n, v in parse_rust_text(t)[0].items(): ix.setdefault(n, []).append((p, v))
        _shared["ix"] = ix
    return _shared["ix"]


def build_rust(unit_dir, art_base, wanted_names):
    own_structs, own_enums = rust_module(unit_dir, art_base, wanted_names)
    ix_structs, ix_enums = artifact_index(art_base) if art_base else ({}, {})
    enums = dict(global_enums())
    enums.update(ix_enums)
    enums.update(own_enums)
    m = Module()
    m.enums = enums
    sources = {}
    for n in wanted_names:
        c = own_structs.get(n) or ix_structs.get(n) or shared_index().get(n)
        if not c: continue
        sources[n] = c
    m.cands = sources
    m.enums = enums
    return m


def rust_fields(cand, enums):
    fs, has_rule, dflt = cand
    out = []
    for wire, ftype, st, raw, flat in fs:
        ty, opt, rt = rust_ty(ftype, enums)
        out.append(Field(wire, ty, optional=opt, state=st, extra={"rust": rt, "raw": raw}))
    return out


# ------------------------------------------------------------------ TypeScript
def ts_type(t, aliases, interfaces):
    t = t.strip()
    while t.startswith("(") and t.endswith(")") and len(split_top(t[1:-1], "|")) >= 1 and t.count("(") == t.count(")") and split_top(t, "|") == [t]: t = t[1:-1].strip()
    nullable = False
    parts = [p.strip() for p in split_top(t, "|")]
    parts2 = []
    for p in parts:
        if p in ("null", "undefined"): nullable = True
        else: parts2.append(p)
    if not parts2: return Ty("any"), nullable
    if len(parts2) > 1 and all(re.match(r'^"[^"]*"$', p) or re.match(r"^'[^']*'$", p) for p in parts2):
        return Ty("enum", members=[p[1:-1] for p in parts2]), nullable
    if len(parts2) == 1:
        t = parts2[0]
        if t.startswith("(") and t.endswith(")"):
            ty, nl2 = ts_type(t[1:-1], aliases, interfaces)
            return ty, nullable or nl2
    else: return Ty("any"), nullable
    m = re.match(r"^(?:readonly\s+)?(.*)\[\]$", t, re.S)
    if m: return Ty("array", inner=ts_type(m.group(1).strip("() "), aliases, interfaces)[0]), nullable
    m = re.match(r"^(?:Readonly)?Array<(.*)>$", t, re.S)
    if m: return Ty("array", inner=ts_type(m.group(1), aliases, interfaces)[0]), nullable
    if t in ("string", "number", "boolean"): return Ty(t), nullable
    if t in ("bigint",): return Ty("integer"), nullable
    if t in aliases: return Ty("enum", members=aliases[t], name=t), nullable
    if re.match(r"^[A-Z]\w*$", t): return Ty("ref", name=t), nullable
    return Ty("any"), nullable


def parse_ts(path):
    text = open(path, encoding="utf-8").read()
    aliases = {}
    for m in re.finditer(r"export type (\w+)\s*=\s*([^;]+);", text):
        parts = [p.strip() for p in split_top(m.group(2), "|")]
        if parts and all(re.match(r'^"[^"]*"$', p) for p in parts): aliases[m.group(1)] = [p[1:-1] for p in parts]
    m = Module()
    m.aliases = aliases
    m.broken = set()
    m.readers = {}
    states = {}
    for im in re.finditer(r"export interface (\w+)(?:<[^>]*>)?(?:\s+extends [^{]+)?\s*\{", text):
        i = im.end()
        depth, j = 1, i
        while j < len(text) and depth:
            if text[j] == "{": depth += 1
            elif text[j] == "}": depth -= 1
            j += 1
        body = text[i:j - 1]
        fs = []
        for item in split_top(body, ";"):
            st = re.findall(r"@state (\w+)", item)
            clean = re.sub(r"/\*.*?\*/", "", item, flags=re.S)
            clean = strip_line_comments(clean).strip()
            mm = re.match(r'^(?:readonly\s+)?(?:"([^"]+)"|(\w+))(\?)?\s*:\s*(.*)$', clean, re.S)
            if not mm: continue
            name = mm.group(1) or mm.group(2)
            ty, nullable = ts_type(re.sub(r"\s+", " ", mm.group(4)).strip(), aliases, None)
            docm = re.search(r"/\*\*.*?\*/", item, flags=re.S)
            fs.append(Field(name, ty, optional=bool(mm.group(3)) or nullable, state=st[0] if st else None, extra={"raw": re.sub(r"\s+", " ", clean), "doc": docm.group(0) if docm else None}))
        m.recs[im.group(1)] = Rec(im.group(1), fs)
    for rm in re.finditer(r"export const parse(\w+):\s*NormWireReader<(\w+)>\s*=\s*normWireObject<\w+>\(\{", text):
        i = rm.end()
        depth, j = 1, i
        while j < len(text) and depth:
            if text[j] == "{": depth += 1
            elif text[j] == "}": depth -= 1
            j += 1
        rf = []
        seg = re.sub(r'"[^"]*"', '""', text[i:j - 1])
        if seg.count("(") != seg.count(")"): m.broken.add(rm.group(2))
        for it in split_top(text[i:j - 1]):
            mm = re.match(r'^\s*(?:"([^"]+)"|(\w+))\s*:\s*(.*)$', it, re.S)
            if not mm: continue
            val = mm.group(3).strip()
            lit = re.search(r"normWireLiteral\(([^)]*)\)", val)
            rb = re.search(r"normWireRange\(\w+,\s*(\{[^}]*\})\)", val)
            try: rbd = json.loads(rb.group(1)) if rb else None
            except Exception: rbd = None
            rf.append((mm.group(1) or mm.group(2), val.startswith("normWireRequired("), [x.strip().strip('"') for x in lit.group(1).split(",")] if lit else None, rbd, val))
        m.readers[rm.group(2)] = rf
    return m


# ------------------------------------------------------------------ proto
PROTO_SCALAR = {"double": "number", "float": "number", "string": "string", "bool": "boolean", "bytes": "string"}
PROTO_INT = {"int32", "int64", "uint32", "uint64", "sint32", "sint64", "fixed32", "fixed64", "sfixed32", "sfixed64"}


def block_body(text, i):
    depth, j = 1, i
    while j < len(text) and depth:
        if text[j] == "{": depth += 1
        elif text[j] == "}": depth -= 1
        j += 1
    return text[i:j - 1]


def parse_proto(path):
    text = open(path, encoding="utf-8").read()
    m = Module()
    m.numbers = {}
    enums = {}
    for em in re.finditer(r"\benum\s+(\w+)\s*\{", text):
        body = strip_line_comments(block_body(text, em.end()))
        mem = [x for x in re.findall(r"(\w+)\s*=\s*\d+", body)]
        mem = [x for x in mem if not x.endswith("_UNSPECIFIED")]
        own = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", em.group(1)).upper() + "_"
        if mem and all(x.startswith(own) and len(x) > len(own) for x in mem): mem = [x[len(own):] for x in mem]
        elif len(mem) > 1 and all("_" in x for x in mem) and len({x.split("_")[0] for x in mem}) == 1: mem = [x.split("_", 1)[1] for x in mem]
        elif len(mem) > 1:
            pre = os.path.commonprefix(mem)
            if "_" in pre and not any(x == pre for x in mem): mem = [x[pre.rindex("_") + 1:] for x in mem]
        enums[em.group(1)] = mem
    m.enums = enums
    for mm in re.finditer(r"\bmessage\s+(\w+)\s*\{", text):
        body = block_body(text, mm.end())
        fs, nums = [], []
        pat = re.compile(r"(?:(?://|/\*)\s*@state\s+(\w+)\s*(?:\*/)?\s*)?(?:(?://[^\n]*\n|/\*(?!\s*@state).*?\*/)\s*)*(repeated\s+|optional\s+)?([\w.]+)\s+(\w+)\s*=\s*(\d+)\s*(?:\[[^\]]*\])?\s*;", re.S)
        for fm in pat.finditer(body):
            st, mod, t, name, num = fm.groups()
            if t in ("reserved", "option", "syntax", "package", "import"): continue
            if t in PROTO_SCALAR: ty = Ty(PROTO_SCALAR[t])
            elif t in PROTO_INT: ty = Ty("integer")
            elif t in enums: ty = Ty("enum", members=enums[t], name=t)
            else: ty = Ty("ref", name=t.split(".")[-1])
            if mod and mod.strip() == "repeated": ty = Ty("array", inner=ty)
            fs.append(Field(camel(name), ty, optional=bool(mod and mod.strip() == "optional"), state=st, extra={"snake": name}))
            nums.append(int(num))
        m.recs[mm.group(1)] = Rec(mm.group(1), fs)
        m.numbers[mm.group(1)] = nums
    return m


# ------------------------------------------------------------------ GraphQL
GQL_SCALAR = {"JSON": "json", "String": "string", "ID": "string", "Float": "number", "Int": "integer", "Boolean": "boolean"}


def gql_ty(t, enums):
    t = t.strip()
    optional = not t.endswith("!")
    t = t.rstrip("!")
    if t.startswith("["):
        inner, _ = gql_ty(t[1:-1], enums)
        return Ty("array", inner=inner), optional
    if t in GQL_SCALAR: return Ty(GQL_SCALAR[t]), optional
    if t in enums: return Ty("enum", members=enums[t], name=t), optional
    return Ty("ref", name=t), optional


def parse_graphql(path):
    text = open(path, encoding="utf-8").read()
    text = re.sub(r"(^|\n)\s*#[^\n]*", r"\1", text)
    m = Module()
    enums = {}
    for em in re.finditer(r"\benum\s+(\w+)\s*\{([^}]*)\}", text):
        enums[em.group(1)] = em.group(2).split()
    m.enums = enums
    for tm in re.finditer(r"\b(?:type|input)\s+(\w+)(?:\s+implements [^{]+)?\s*\{([^}]*)\}", text):
        fs = []
        for fm in re.finditer(r"(\w+)\s*(?:\([^)]*\))?\s*:\s*([\[\]\w!]+)\s*((?:@\w+(?:\([^)]*\))?\s*)*)", tm.group(2)):
            ty, opt = gql_ty(fm.group(2), enums)
            sm = re.search(r"@state\(class:\s*(\w+)\)", fm.group(3))
            fs.append(Field(fm.group(1), ty, optional=opt, state=sm.group(1).lower() if sm else None))
        m.recs[tm.group(1)] = Rec(tm.group(1), fs)
    return m


# ------------------------------------------------------------------ comparison
def strict_enum_ok(a, b):
    return a == b


def cmp_field_types(j, o, twin):
    """💬️ Returns a message or None."""
    if o.kind == "opaque": return None
    if twin in ("proto", "graphql") and (o.kind == "json" or (twin == "proto" and o.kind == "string" and j.kind in ("array", "ref")) or (twin == "graphql" and o.kind == "string" and j.kind == "ref")): return None
    if twin == "ts" and j.kind == "integer" and o.kind == "number": return None
    if j.kind == "any" or o.kind == "any": return None
    if j.kind == "object":
        if o.kind == "ref" and o.name not in ("Value", "NormJson", "Json", "JsonValue", "Map", "HashMap", "BTreeMap"): return f"untyped json object where {twin} has record {o.name}"
        return None
    if j.kind == "array" and o.kind == "array":
        if j.inner and o.inner: return cmp_field_types(j.inner, o.inner, twin)
        return None
    if j.kind == "enum" and o.kind == "enum":
        if j.members != o.members:
            if sorted(j.members) == sorted(o.members): return f"enum order json={j.members} {twin}={o.members}"
            if [norm_member(x) for x in j.members] == [norm_member(x) for x in o.members] and twin in ("rust", "ts"): return f"enum spelling json={j.members} {twin}={o.members}"
            if [norm_member(x) for x in j.members] == [norm_member(x) for x in o.members]: return None
            return f"enum members json={j.members} {twin}={o.members}"
        return None
    if j.kind == "enum" and o.kind in ("string", "ref"):
        if twin in ("rust",) and o.kind == "ref": return f"unresolved enum type {o.name} (json enum {j.members})"
        if o.kind == "string": return None if twin in ("proto", "graphql") else f"json enum {j.members} vs {twin} string"
        return None
    if j.kind == "string" and o.kind == "enum": return None
    if j.kind == "ref" and o.kind == "ref":
        return None if PREFIX.sub("", j.name) == PREFIX.sub("", o.name) else f"ref json={j.name} {twin}={o.name}"
    if j.kind == "ref" and o.kind == "enum": return None
    if j.kind == "enum" and o.kind == "ref": return None
    if j.kind != o.kind:
        if j.kind == "number" and o.kind == "integer" and twin in ("graphql",): return f"type json={j.show()} {twin}={o.show()}"
        return f"type json={j.show()} {twin}={o.show()}"
    return None


def compare_type(unit, tname, jrec, twin, trec, out, check_state=True, rust_meta=None):
    jn = [f.name for f in jrec.fields if not (twin in ("proto", "graphql") and not re.match(r"[A-Za-z_]", f.name))]
    tn = [f.name for f in trec.fields]
    base = f"{unit} {tname} [{twin}]"
    if jn != tn:
        miss = [x for x in jn if x not in tn]
        extra = [x for x in tn if x not in jn]
        if miss or extra:
            out.append((base, "fields", f"json-only={miss} {twin}-only={extra}"))
        else:
            out.append((base, "order", f"json={jn} {twin}={tn}"))
    tmap = {f.name: f for f in trec.fields}
    for f in jrec.fields:
        o = tmap.get(f.name)
        if not o: continue
        msg = cmp_field_types(f.ty, o.ty, twin)
        if msg: out.append((base, "type", f"{f.name}: {msg}"))
        if f.optional != o.optional and not (twin == "proto" and o.ty.kind == "array"):
            if twin == "graphql" and f.ty.kind == "array" and not f.optional: pass
            out.append((base, "optional", f"{f.name}: json optional={f.optional} {twin} optional={o.optional}"))
        if f.ty.kind == "integer" and rust_meta and twin == "rust":
            b = f.extra.get("bounds")
            rt = o.extra.get("rust")
            if rt in INT_BOUNDS and not bounds_within(b, INT_BOUNDS[rt]):
                out.append((base, "bounds", f"{f.name}: json bounds={b} rust {rt}={INT_BOUNDS[rt]}"))
        if check_state and f.state and o.state and norm_member(f.state) != norm_member(o.state):
            out.append((base, "state", f"{f.name}: json={f.state} {twin}={o.state}"))
        elif check_state and f.state and twin in ("rust", "ts", "proto", "graphql") and o.state is None and twin != "ts":
            pass


def bounds_within(b, limit):
    lo, hi = b if b else (None, None)
    if lo is None or lo < limit[0]: return False
    if limit[1] is not None and (hi is None or hi > limit[1]): return False
    return True


def discover_units(artifacts, include_shared, facets, all_arts):
    units = []
    roots = []
    arts_dir = os.path.join(NORM, "🗿️artifacts")
    for a in sorted(os.listdir(arts_dir)):
        key = re.sub(r"^[^a-z]+", "", a)
        key = re.sub(r"[^a-z0-9]", "", key)
        if all_arts or key in artifacts: roots.append((key, os.path.join(arts_dir, a)))
    if include_shared:
        for n in sorted(os.listdir(NORM)):
            if n == "🗿️artifacts" or n.startswith(".") or not os.path.isdir(os.path.join(NORM, n)): continue
            roots.append(("shared:" + n, os.path.join(NORM, n)))
    for key, base in roots:
        for r, ds, fs in os.walk(base):
            ds[:] = [x for x in ds if x not in SKIP_PARTS]
            if "🔣️.json" not in fs: continue
            twins = [t for t in ("🟦️.ts", "🛰️.proto", "🔗️.graphql") if t in fs]
            leaf_rs = os.path.basename(r) == "🧬️schema" and os.path.exists(os.path.join(os.path.dirname(r), "🦀️.rs")) and "🧬️mutations" in r
            if "🦀️.rs" in fs or leaf_rs: twins.append("🦀️.rs")
            if not twins: continue
            if facets and os.path.basename(r) not in facets and not any(f in r for f in facets): continue
            units.append((key, base, r, twins))
    return units


def macro_defined(d, name):
    p = os.path.join(d, "🦀️.rs")
    if not os.path.exists(p): return False
    return re.search(r"\b" + re.escape(name) + r"\b", open(p, encoding="utf-8").read()) is not None


def check_unit(key, base, d, twins, out, stats):
    rel = os.path.relpath(d, NORM)
    short = key + ":" + re.sub(r"🏅️standards/🔖️1/🪆️subsets/✳️any/?", "", rel.split("/", 2)[-1] if rel.startswith("🗿️artifacts") else rel)
    try: jm = parse_json(os.path.join(d, "🔣️.json"))
    except Exception as e:
        out.append((short, "json", f"unparsable: {e}")); return
    if jm is None: return
    stats["units"] += 1
    REF_CTX["dir"] = d
    jm = parse_json(os.path.join(d, "🔣️.json"))
    wanted = list(jm.recs)
    reftxt = open(os.path.join(d, "🔣️.json"), encoding="utf-8").read()
    jm.refd = {x.split("/")[-1] for x in re.findall(r'"\$ref":\s*"([^"]+)"', reftxt)}
    known = {PREFIX.sub("", x) for x in jm.recs} | {PREFIX.sub("", x) for x in jm.refd}
    if "🦀️.rs" in twins:
        rm = build_rust(d, None if key.startswith("shared") and False else base, wanted)
        for n in wanted:
            c = rm.cands.get(n)
            if c is None and not jm.recs[n].fields: continue
            if c is None and (os.path.basename(d) == "🔺️diff" or macro_defined(d, n)): stats["macro"] = stats.get("macro", 0) + 1; continue
            if c is None:
                out.append((f"{short} {n} [rust]", "missing-type", "no Rust struct found for json type"))
                continue
            best = None
            for cand in c:
                rec = Rec(n, rust_fields(cand[1], rm.enums))
                probe = []
                jr0 = jm.recs[n]
                jr1 = Rec(n, [f for f in jr0.fields if not (f.name == "mutation" and f.ty.kind == "enum" and len(f.ty.members) == 1)])
                compare_type(short, n, jr1, "rust", rec, probe, rust_meta=True)
                if best is None or len(probe) < len(best[1]): best = (cand, probe, rec)
                if not probe: break
            out.extend(best[1])
            stats["types"] += 1
            if len(c) > 1 and best[1]: stats["ambiguous"] += 1
    if "🟦️.ts" in twins:
        tm = parse_ts(os.path.join(d, "🟦️.ts"))
        for n, jr in jm.recs.items():
            tr = find_rec(tm.recs, n)
            if tr is None and not jr.fields: continue
            if tr is None and re.search(r"\b" + re.escape(n) + r"\b", open(os.path.join(d, "🟦️.ts"), encoding="utf-8").read()): continue
            if tr is None: out.append((f"{short} {n} [ts]", "missing-type", "no TS interface for json type")); continue
            compare_type(short, n, jr, "ts", tr, out)
            if tr.name in tm.broken:
                out.append((f"{short} {n} [ts-reader]", "syntax", "reader has unbalanced parentheses"))
                continue
            rd = tm.readers.get(tr.name)
            if rd is None: continue
            names = [x[0] for x in rd]
            if names != [f.name for f in jr.fields]: out.append((f"{short} {n} [ts-reader]", "fields", f"json={[f.name for f in jr.fields]} reader={names}"))
            else:
                for f, (rn, req, lit, rbd, _raw) in zip(jr.fields, rd):
                    jb = f.extra.get("bounds") if f.ty.kind == "integer" else None
                    want = {k: v for k, v in (("minimum", jb[0]), ("maximum", jb[1])) if v is not None} if jb else None
                    if rbd and f.ty.kind == "integer" and rbd != (want or {}): out.append((f"{short} {n} [ts-reader]", "bounds", f"{f.name}: json bounds={want} reader={rbd}"))
                    if req == f.optional: out.append((f"{short} {n} [ts-reader]", "optional", f"{f.name}: json optional={f.optional} reader required={req}"))
                    if lit and f.ty.kind == "enum" and lit != f.ty.members: out.append((f"{short} {n} [ts-reader]", "type", f"{f.name}: literal {lit} vs json {f.ty.members}"))
        for n in tm.recs:
            if jm.recs and PREFIX.sub("", n) not in known and n not in jm.recs and n not in ("Empty",): out.append((f"{short} {n} [ts]", "extra-type", "TS interface without json type"))
    if "🛰️.proto" in twins:
        pm = parse_proto(os.path.join(d, "🛰️.proto"))
        for n, jr in jm.recs.items():
            pr = find_rec(pm.recs, n)
            if pr is None: continue
            if False: out.append((f"{short} {n} [proto]", "missing-type", "no proto message for json type")); continue
            compare_type(short, n, jr, "proto", pr, out)
            nums = pm.numbers.get(pr.name, [])
            if len(set(nums)) != len(nums): out.append((f"{short} {n} [proto]", "numbering", f"duplicate field numbers {nums}"))
        for n in pm.recs:
            if jm.recs and "🧬️mutations" not in d.replace("🧬️mutations/", "/x/", 0).split("🧬️schema")[-1] and PREFIX.sub("", n) not in known and n not in jm.recs and n != "Empty": out.append((f"{short} {n} [proto]", "extra-type", "proto message without json type"))
    if "🔗️.graphql" in twins:
        gm = parse_graphql(os.path.join(d, "🔗️.graphql"))
        for n, jr in jm.recs.items():
            gr = find_rec(gm.recs, n)
            if gr is None: continue
            if False: out.append((f"{short} {n} [graphql]", "missing-type", "no graphql type for json type")); continue
            compare_type(short, n, jr, "graphql", gr, out)
        for n in gm.recs:
            if jm.recs and PREFIX.sub("", n) not in known and n not in jm.recs: out.append((f"{short} {n} [graphql]", "extra-type", "graphql type without json type"))


def main(argv):
    arts, facets, limit, summary, jout, all_arts, shared = [], [], 60, False, None, False, True
    i = 0
    while i < len(argv):
        a = argv[i]
        if a == "--artifact": arts.append(argv[i + 1]); i += 1
        elif a == "--facet": facets.append(argv[i + 1]); i += 1
        elif a == "--limit": limit = int(argv[i + 1]); i += 1
        elif a == "--json": jout = argv[i + 1]; i += 1
        elif a == "--summary": summary = True
        elif a == "--all": all_arts = True
        elif a == "--no-shared": shared = False
        elif a == "--root":
            global NORM
            NORM = argv[i + 1]; i += 1
        i += 1
    units = discover_units(arts or ARTIFACTS, shared, facets, all_arts and False)
    out, stats = [], {"units": 0, "types": 0, "ambiguous": 0}
    for u in units: check_unit(*u, out, stats)
    by = {}
    for base, cat, msg in out: by.setdefault(cat, 0); by[cat] += 1
    print(f"units checked={stats['units']} rust types={stats['types']} mismatches={len(out)} by category={by}")
    if not summary:
        for base, cat, msg in out[: limit if limit else None]: print(f"{cat:13} {base}: {msg[:400]}")
        if limit and len(out) > limit: print(f"... {len(out) - limit} more (use --limit 0)")
    if jout: json.dump([{"where": b, "category": c, "detail": m} for b, c, m in out], open(jout, "w"), ensure_ascii=False, indent=1)
    return 1 if out else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
