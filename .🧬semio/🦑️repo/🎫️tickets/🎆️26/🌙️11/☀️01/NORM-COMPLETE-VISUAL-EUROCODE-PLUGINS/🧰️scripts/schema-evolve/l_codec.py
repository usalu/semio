"""🧾 Learned codec layer: the SQLite DDL and the Rust and TypeScript codecs of a new collection are cloned from the statements of a reference collection of the same artifact (nested collections: `l_nestedsql`, the pair clone). The field dependent regions of a cloned statement (cell lists, struct literals, bound lists, column definitions) are regenerated from item templates learned from the reference itself, so every artifact keeps its own dialect; anything that does not line up reports `Unsupported`."""
import re
from core import Names, Unsupported, camel, pascal, snake
from lex import code_mask, fn_bodies, match_close, split_items, statements, tail_expression


def unwrap(node):
    """🎁 The value node of an optional member (`oneOf`/`anyOf` of the value and null)."""
    for key in ("oneOf", "anyOf"):
        parts = node.get(key)
        if parts and len(parts) == 2 and {"type": "null"} in parts:
            return next(p for p in parts if p != {"type": "null"})
    return node


def kind_of(node, defs):
    node = unwrap(node)
    if node.get("type") == "string" and "enum" in node:
        return "enum"
    if "$ref" in node:
        d = defs.get(node["$ref"].rsplit("/", 1)[1], {})
        return "enum" if d.get("type") == "string" and "enum" in d else None
    return {"number": "f64", "string": "String", "boolean": "bool"}.get(node.get("type")) or (("u8" if node.get("maximum") == 255 else "u32") if node.get("type") == "integer" else None)


class F:
    """📣 A cell field of a row entity: Rust name (snake), TS name (camel), kind, enum def name."""

    def __init__(self, camel_name, kind, enum_def=None, unit_id=None, tokens=None):
        self.camel, self.snake, self.kind, self.enum_def, self.tokens = camel_name, snake(camel_name), kind, enum_def, tokens

    def name(self, lang):
        return self.snake if lang == "rs" else self.camel


def schema_fields(ctx, entity, scalars=False):
    out = []
    for name, node in ctx.defs[entity]["properties"].items():
        node = unwrap(node)
        if scalars and (node.get("type") in ("array", "object") or "$ref" in node and ctx.defs.get(node["$ref"].rsplit("/", 1)[1], {}).get("type") == "object"):
            continue
        if "$ref" in node and ctx.defs.get(node["$ref"].rsplit("/", 1)[1], {}).get("type") == "object" or node.get("type") == "object":
            sub = ctx.defs[node["$ref"].rsplit("/", 1)[1]] if "$ref" in node else node
            for cname, cnode in sub.get("properties", {}).items():
                if cnode.get("type") not in ("array", "object"):
                    kind = kind_of(cnode, ctx.defs)
                    enum_def = (cnode["$ref"].rsplit("/", 1)[1] if "$ref" in cnode else pascal(cname)) if kind == "enum" else None
                    tokens = (cnode["enum"] if "enum" in cnode else ctx.defs[enum_def]["enum"]) if kind == "enum" else None
                    out.append(F(cname, kind, enum_def, tokens=tokens))
            continue
        if name == "id" or node.get("type") in ("array", "object"):
            continue
        kind = kind_of(node, ctx.defs)
        enum_def, tokens = None, None
        if kind == "enum":
            enum_def = node["$ref"].rsplit("/", 1)[1] if "$ref" in node else node.get("x-semio-rust", {}).get("type") or pascal(name)
            tokens = node["enum"] if "enum" in node else ctx.defs[enum_def]["enum"]
        out.append(F(name, kind, enum_def, tokens=tokens))
    return out


def spec_fields(S):
    return [F(f.camel, f.ty, f.enum_name if f.ty == "enum" else None, tokens=f.tokens if f.ty == "enum" else None) for f in S.fields]


def word(name):
    return re.compile(r"(?<![\w])%s(?![\w])" % re.escape(name))


class Found:
    def __init__(self, i, j, items, first, count, fields, hits=None):
        self.i, self.j, self.items, self.first, self.count, self.fields = i, j, items, first, count, fields
        self.hits = hits if hits is not None else list(range(first, first + count))


def find_list(text, mask, lo, hi, fields, lang, sep=",", tolerant=False):
    """📢 The smallest bracket group in [lo, hi) holding one `sep` item per field in one consecutive block (an item mentions its field name); the block order is the codec's column order, which may differ from the schema order."""
    best = None
    for i in range(lo, hi):
        if not (mask[i] and text[i] in "[{("):
            continue
        j = match_close(text, mask, i)
        if j >= hi:
            continue
        items = split_items(text, mask, i + 1, j, sep)
        hits, used = [], set()
        for k, (a, b) in enumerate(items):
            cand = [n for n, fl in enumerate(fields) if n not in used and word(fl.name(lang)).search(text[a:b])]
            if cand:
                hits.append((k, cand[0]))
                used.add(cand[0])
        if len(hits) == len(fields) and fields and [h[0] for h in hits] == list(range(hits[0][0], hits[0][0] + len(fields))):
            if best is None or j - i < best.j - best.i:
                best = Found(i, j, items, hits[0][0], len(fields), [fields[h[1]] for h in hits])
        elif tolerant and len(hits) == len(fields) and fields and derived_between(text, items, hits, fields, lang):
            if best is None or j - i < best.j - best.i:
                best = Found(i, j, items, hits[0][0], hits[-1][0] - hits[0][0] + 1, [fields[h[1]] for h in hits], [h[0] for h in hits])
    return best


def derived_between(text, items, hits, fields, lang):
    """🪢 Items between the field items that only restate a field (`resolved(members,bridge.memberId)`, a lookup of an id cell) belong to the codec's block."""
    hit_pos = {h[0] for h in hits}
    for k in range(hits[0][0], hits[-1][0] + 1):
        body = text[items[k][0]:items[k][1]]
        if k not in hit_pos and not re.fullmatch(r"\s*(?:[a-z_]\w*|\"\w+\")\s*", body) and not any(word(f.name(lang)).search(body) for f in fields):
            return False
    return True


def swap(text, found, new_items):
    a, b = found.items[found.first][0], found.items[found.first + found.count - 1][1]
    gap = text[found.items[found.first][1]:found.items[found.first + 1][0]] if found.count > 1 else ","
    return text[:a] + gap.join(new_items) + text[b:]


def item_texts(text, found):
    return [text[found.items[k][0]:found.items[k][1]] for k in found.hits]


VAR_RX = (r"\b(\w+)\.%s\b", r"\b(\w+)\.(?:real|text|integer)\(", r"\w+\((\w+),\s*\d+", r"\b(\w+)\.\w+\b")


def find_var(item, name):
    for rx in VAR_RX:
        m = re.search(rx % re.escape(name) if "%s" in rx else rx, item)
        if m:
            return m.group(1)
    return None


def template_of(item, f, lang, idx):
    """📡 The item of a field as a template: the field name becomes `{n}`, the column index `{i}`, the row variable `{v}`; an enum field also abstracts its helper function (`{H}`), a TypeScript Binary64 read its constant (`{C}`)."""
    var = find_var(item, f.name(lang))
    helper = None
    name = re.escape(f.name(lang))
    if f.kind == "enum":
        for rx in (r"^Cell::Text\((\w+)\(", r"^%s:\s*(read_\w+|\w+)\(" % name, r"^(\w+)\(\w+\.%s\)$" % name, r"\bchoice\([^;]*?,([A-Z][A-Z0-9_]*)\)"):
            m = re.search(rx, item)
            if m and m.group(1) not in ("Text", "choice", "text"):
                helper = m.group(1)
                item = item[: m.start(1)] + "\x00" + item[m.end(1):]
                break
    t = word(f.name(lang)).sub("{n}", item).replace("\x00", "{H}")
    if idx is not None:
        t = re.sub(r"(?<![\w.])%d(?![\w.])" % idx, "{i}", t)
    if f.kind == "enum" and f.tokens:
        t = re.sub(r"\[\s*" + r"\s*,\s*".join('"%s"' % re.escape(x) for x in f.tokens) + r"\s*\]", "{TOK}", t)
    const = None
    m = re.search(r"readBinary64\(\w+,\{i\},([A-Z][A-Z0-9_]*)\)", t)
    if m:
        const = m.group(1)
        t = t.replace(const, "{C}", 1)
    if var:
        t = re.sub(r"(?<![\w.])%s(?=\.|,|\))" % re.escape(var), "{v}", t, count=0)
    return t, helper, var, const


def fill_item(t, f, lang, idx, helper=None, var=None, const=None):
    if "{TOK}" in t:
        t = t.replace("{TOK}", "[" + ",".join('"%s"' % x for x in (f.tokens or [])) + "]")
    return t.replace("{n}", f.name(lang)).replace("{i}", str(idx) if idx is not None else "").replace("{H}", helper or "").replace("{v}", var or "v").replace("{C}", const or "")


SQLITE = "🚪️io/🪶️sqlite/📸️snapshot"
INT_MAX = {"u8": 255, "u32": 4294967295}


def tables_of(sql):
    return re.findall(r"CREATE TABLE\s+(\w+)", sql)


def table_forms(ctx, names):
    from core import singular_of
    singular = snake(singular_of(names.plural_camel))
    return [f"{ctx.short}_{snake(names.entity)}", snake(names.entity), f"{ctx.short}_{singular}"]


def table_of(ctx, names, sql):
    tables = tables_of(sql)
    for cand in table_forms(ctx, names):
        if cand in tables:
            return cand
    stem = snake(re.sub(r"^En\d{4}", "", names.entity))
    near = sorted((t for t in tables if t.startswith(ctx.short + "_") and len(t) > len(ctx.short) + 4 and stem.startswith(t[len(ctx.short) + 1:]) and "_" in t[len(ctx.short) + 1:]), key=len, reverse=True)
    if near:
        return near[0]
    raise Unsupported(f"table of {names.entity} not found in the DDL")


def entity_form(ctx, names):
    s = snake(names.entity)
    return s if s.startswith(ctx.short + "_") else f"{ctx.short}_{s}"


def new_table(ctx, ref, ref_tbl, new):
    from core import singular_of
    if ref_tbl == entity_form(ctx, ref):
        return entity_form(ctx, new)
    return f"{ctx.short}_{snake(singular_of(new.plural_camel))}"


def def_name(d):
    return d.split()[0]


def def_kind(d):
    if "_ieee754_bits" in d:
        return "bits"
    if "_ieee754_class" in d:
        return "class"
    if re.search(r"CHECK\s*\(\s*\w+\s+IN\s*\(", d):
        return "enum"
    if "INTEGER" in d and re.search(r"\b255\b", d):
        return "u8"
    if "INTEGER" in d and re.search(r"\b4294967295\b", d):
        return "u32"
    if re.search(r"\bTEXT NOT NULL$", d):
        return "String"
    if re.search(r"\bREAL$", d):
        return "f64"
    return None


def ddl_body(sql, name):
    m = re.search(r"CREATE TABLE\s+%s\s*\(" % re.escape(name), sql)
    if not m:
        raise Unsupported(f"table {name} not found in the DDL")
    mask = code_mask(sql, "ts")
    close = match_close(sql, mask, m.end() - 1)
    return m, close, split_items(sql, mask, m.end(), close)


def ddl_templates(sql):
    """📠 Column definition templates by kind, learned from every table: the column name becomes `{n}`."""
    out = {}
    for t in tables_of(sql):
        _, _, items = ddl_body(sql, t)
        for a, b in items:
            d = sql[a:b]
            k = def_kind(d)
            if k and k not in out:
                n = def_name(d)
                base = n[: -len("_ieee754_bits")] if k == "bits" else n[: -len("_ieee754_class")] if k == "class" else n
                out[k] = d.replace(base, "{n}", 1) if k in ("bits", "class") else d.replace(n, "{n}")
    return out


def ddl_generic(ctx, sql, ref, ref_tbl, ref_fields, new_tbl, new_fields):
    m, close, items = ddl_body(sql, ref_tbl)
    defs = [sql[a:b] for a, b in items]
    gaps = [sql[items[k][1]:items[k + 1][0]] for k in range(len(items) - 1)]
    first_gap, last_gap = sql[m.end():items[0][0]], sql[items[-1][1]:close]
    roles = ["X" if k < 4 else {"bits": "B", "class": "C"}.get(def_kind(d), "F") for k, d in enumerate(defs)]
    role_gap = {}
    for k, g in enumerate(gaps):
        role_gap.setdefault((roles[k], roles[k + 1]), g)
    common = max(set(gaps), key=gaps.count) if gaps else ", "
    grouped = True
    seen_c = False
    for r in roles:
        if r in "BC":
            seen_c = True
        elif r == "F" and seen_c:
            grouped = False
    tmpl = ddl_templates(sql)
    out_defs, out_roles = [d for d in defs[:4]], ["X"] * 4
    reals = [f for f in new_fields if f.kind == "f64"]

    def field_def(f):
        k = f.kind
        if k == "enum":
            t = tmpl.get("enum")
            if not t:
                raise Unsupported("no enum column definition to learn from in the DDL")
            vals_m = re.search(r"IN\s*\(([^)]*)\)", t)
            sep = "," if re.search(r"','", vals_m.group(1)) else ", "
            vals = sep.join("'%s'" % v for v in f.tokens)
            return re.sub(r"IN\s*\(([^)]*)\)", lambda mm: mm.group(0)[: mm.group(0).index("(") + 1] + vals + ")", t.replace("{n}", f.snake))
        t = tmpl.get(k)
        if not t and k in INT_MAX:
            other = tmpl.get("u8" if k == "u32" else "u32")
            t = other and re.sub(r"\b(?:255|4294967295)\b", str(INT_MAX[k]), other)
        if not t:
            raise Unsupported(f"no {k} column definition to learn from in the DDL")
        return t.replace("{n}", f.snake)

    def pair(f):
        return [tmpl["bits"].replace("{n}", f.snake), tmpl["class"].replace("{n}", f.snake)] if "bits" in tmpl else (_ for _ in ()).throw(Unsupported("no ieee754 column definitions in the DDL"))

    if grouped:
        for f in new_fields:
            out_defs.append(field_def(f)); out_roles.append("F")
        for f in reals:
            b, c = pair(f)
            out_defs += [b, c]; out_roles += ["B", "C"]
    else:
        for f in new_fields:
            out_defs.append(field_def(f)); out_roles.append("F")
            if f.kind == "f64":
                b, c = pair(f)
                out_defs += [b, c]; out_roles += ["B", "C"]
    body = first_gap
    for k, d in enumerate(out_defs):
        body += d + (role_gap.get((out_roles[k], out_roles[k + 1]), common) if k + 1 < len(out_defs) else "")
    body += last_gap
    stmt_end = sql.find(";", close) + 1
    head = sql[sql.rfind("CREATE TABLE", 0, m.end()) if False else m.start():m.end()]
    text = head.replace(ref_tbl, new_tbl) + body + sql[close:stmt_end] + "\n"
    return text


def upper(s):
    return s.upper()


def enum_helpers(text, ref_enum_fields, lang):
    """📟 Naming of the enum helper functions learned from the reference: (suffix of the write helper after the snake enum name, prefix and suffix of the read helper)."""
    return None


def helper_names(enum_name, dialect):
    s = snake(enum_name)
    return dialect["w_pre"] + s + dialect["w_suf"], dialect["r_pre"] + s + dialect["r_suf"]


def learn_dialect(text, ref_fields):
    """📞 How this artifact names the enum helpers (`member_kind` / `read_member_kind`, `x_text` / `read_x`): from the helpers of the reference enum fields, else the common spelling."""
    dialect = {"w_pre": "", "w_suf": "_text", "r_pre": "read_", "r_suf": ""}
    for f in ref_fields:
        if f.kind != "enum":
            continue
        s = snake(f.enum_def)
        m = re.search(r"\bfn\s+(\w*%s\w*)\s*\(\s*\w+\s*:\s*(?:\w+::)*%s\s*\)" % (re.escape(s), re.escape(f.enum_def)), text)
        r = re.search(r"\bfn\s+(\w*%s\w*)\s*\(\s*\w+\s*:\s*&str\s*\)" % re.escape(s), text)
        found = False
        if m and s in m.group(1):
            a, b = m.group(1).split(s, 1)
            dialect["w_pre"], dialect["w_suf"] = a, b
            found = True
        if r and s in r.group(1):
            a, b = r.group(1).split(s, 1)
            dialect["r_pre"], dialect["r_suf"] = a, b
            found = True
        if found:
            break
    return dialect


def enum_fn_text(f, dialect, model_prefix=""):
    w, r = helper_names(f.enum_def, dialect)
    arms_t = "".join(f'{f.enum_def}::{pascal(t)}=>"{t}",' for t in f.tokens)
    arms_r = "".join(f'"{t}"=>Ok({f.enum_def}::{pascal(t)}),' for t in f.tokens)
    return (f"fn {w}(a:{f.enum_def})->&'static str{{match a{{{arms_t}}}}}\n"
            f"fn {r}(a:&str)->Result<{f.enum_def}, ValueError>{{match a{{{arms_r}_=>Err(invalid(\"{snake(f.enum_def).replace('_', ' ')} differs\"))}}}}\n")


class RustCodec:
    def __init__(self, ctx, ref, ref_tbl, new_tbl, ref_fields, new_fields, sql, target=None):
        L = ctx.layout
        self.target = target or ctx.spec.collection
        self.ctx, self.ref, self.new = ctx, ref, (self.target.names if self.target else ref)
        self.path = L.p(SQLITE + "/🦀️.rs")
        self.text = ctx.plan.read(self.path)
        self.ref_tbl, self.new_tbl, self.rf, self.nf = ref_tbl, new_tbl, ref_fields, new_fields
        self.dialect = learn_dialect(self.text, list(ref_fields) + [F(n, "enum", n) for n, d in ctx.defs.items() if d.get("type") == "string" and "enum" in d])
        self.sql = sql
        self._lib = {}
        self.bound = {}
        self.ref_const = None
        from core import singular_of
        self.slug_ref = Names(ref.plural_camel, pascal(singular_of(ref.plural_camel)))
        self.slug_new = Names(self.new.plural_camel, pascal(singular_of(self.new.plural_camel)))
        self.all_stmts = []
        self.all_stmts = self.stmts()[0]

    unquoted = "snake"

    def rename(self, s, evidence=None):
        s = s.replace(self.ref_tbl, self.new_tbl)
        s = self.ref.rename(s, self.new, evidence or self.text, self.unquoted)
        s = self.slug_ref.rename(s, self.slug_new, evidence or self.text, self.unquoted)
        for words in {self.ref.kebab.replace("-", " "), self.slug_ref.kebab.replace("-", " ")}:
            if words:
                s = re.sub(r"(?<![\w])%s(?![\w])" % re.escape(words), self.target.singular_en if self.target else words, s)
        return re.sub(r"(?<![\w])[A-Z][A-Z0-9_]*(?![\w])", lambda m: self.const_name(m.group(0)) if self.is_const(m.group(0)) else m.group(0), s)

    def unique_bindings(self, stmt, original):
        """📛 A cloned statement must not rebind a name of the original statement (`let traffic_rows` stays `let traffic_rows`): such bindings take the new collection's name."""
        for name, new in self.bound.items():
            stmt = re.sub(r"(?<![\w.])%s(?![\w])" % re.escape(name), new, stmt)
        head = re.match(r"\s*let\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\s*=", stmt)
        for m in ([head] if head else []):
            name = m.group(1)
            if re.search(r"\blet\s+(?:mut\s+)?%s\s*=" % re.escape(name), original) or name in self.bound:
                new = f"{self.slug_new.snake}_{name.split('_')[-1] if name.split('_')[-1] in ('rows', 'groups', 'row') else 'rows'}"
                self.bound[name] = new
                stmt = re.sub(r"(?<![\w.])%s(?![\w])" % re.escape(name), new, stmt)
        return stmt

    def is_const(self, token):
        if not hasattr(self, "_consts"):
            self._consts = set(re.findall(r"\bconst (\w+)\s*:\s*&\[FloatColumn\]", self.text))
        return token in self._consts

    def stmts(self):
        mask = code_mask(self.text, "rs")
        out = []
        for o, c in fn_bodies(self.text, mask, "rs"):
            for a, b in statements(self.text, mask, o, c):
                out.append((a, b))
        return out, mask

    def dedicated(self, s):
        r = self.ref
        if f'"{self.ref_tbl}"' in s or re.search(r"\b%s\s*\{" % re.escape(r.entity), s):
            return True
        head = s.split("{", 1)[0]
        if s.startswith("for") and "[" not in head and not re.search(r"\.len\(\)", head) and re.search(r"\bin\s+(?:[&\w.]*\(&?)?(?:[&\w.]*\.)?(?:%s|%s_rows)\b" % (re.escape(r.plural_snake), re.escape(r.snake)), head):
            return True
        if re.match(r"let\s+(?:mut\s+)?\w*\b%s\w*\s*=" % re.escape(r.plural_snake), s) and not re.search(r"Self\s*\{", s):
            return True
        if re.match(r"let\s+(?:mut\s+)?\w+\s*=\s*groups\(\s*&\s*%s_rows\b" % re.escape(r.snake), s):
            return True
        return False

    def region_templates(self, items, fields, lang, base, id_item=None):
        """📚 kind -> (template, helper, var, const) from the field items of a region and its id item (a String cell by definition)."""
        out = {}
        if id_item:
            t = template_of(id_item, F("id", "String"), lang, None if base is None else (base[0] if isinstance(base, list) else base) - 1)
            if base is None or "{i}" in t[0]:
                out["String"] = t
        for k, f in enumerate(fields):
            t = template_of(items[k], f, lang, None if base is None else base[k] if isinstance(base, list) else base + k)
            if base is not None and "{i}" not in t[0]:
                continue
            out.setdefault(f.kind, t)
        return out

    def library(self, rtype, lang):
        """📙 Templates of every kind learned from the statements of the other entities of the artifact."""
        key = (rtype, lang)
        if key in self._lib:
            return self._lib[key]
        lib = {}
        for plural, entity in self.ctx.collections:
            names = Names(plural, entity)
            if names.plural_camel == self.new.plural_camel:
                continue
            try:
                tbl = table_of(self.ctx, names, self.sql)
            except Unsupported:
                continue
            fields = schema_fields(self.ctx, entity)
            if not fields or any(f.kind is None for f in fields):
                continue
            for s0, e0 in self.all_stmts:
                s = self.text[s0:e0]
                if rtype == "W" and f'"{tbl}"' in s and (re.search(r"insert\w*\(", s) or "write(" in s):
                    m = re.search(r'"%s"' % re.escape(tbl), s)
                    f = find_list(s, code_mask(s, lang), m.end(), len(s), fields, lang)
                elif rtype == "R" and lang == "rs" and re.search(r"\b%s\s*\{" % re.escape(entity), s):
                    m = re.search(r"\b%s\s*\{" % re.escape(entity), s)
                    f = find_list(s, code_mask(s, "rs"), m.end() - 1, len(s), fields, lang)
                elif rtype == "R" and lang == "ts" and re.search(r"\b%s\.push\(\s*\{" % re.escape(plural), s):
                    m = re.search(r"\b%s\.push\(\s*\{" % re.escape(plural), s)
                    f = find_list(s, code_mask(s, "ts"), m.end() - 1, len(s), fields, lang)
                else:
                    continue
                if f:
                    for kind, v in self.region_templates(item_texts(s, f), f.fields, lang, 4 if rtype == "R" else None).items():
                        lib.setdefault(kind, v)
        self._lib[key] = lib
        return lib

    def new_const(self):
        rc = getattr(self, "ref_const", None)
        return self.const_name(rc) if rc and any(g.kind == "f64" for g in self.nf) else None

    def enum_fallback(self, templ, lang, rtype):
        """📘 An enum item composed from the String item of the same region when no entity of the artifact has an enum cell: the value is stored as its token through the generated helper functions."""
        text_t = templ.get("String") or self.library(rtype, lang).get("String")
        if not text_t:
            return None
        t, _, var, const = text_t
        if lang == "rs" and rtype == "W":
            return (t.replace("&{v}.{n}", "{H}({v}.{n})"), "x", var, const)
        if lang == "rs" and rtype == "R":
            return ("{n}:{H}(&" + t.split(":", 1)[1] + ")?", "read_x", var, const)
        if lang == "ts" and rtype == "W":
            return ("{v}.{n}", None, var, const)
        if lang == "ts" and rtype == "R":
            return (t + " as {T}", None, var, const)
        return None

    def new_items(self, templ, lang, base, rtype, ref_var):
        lib = None
        items = []
        for k, f in enumerate(self.nf):
            tpl = templ.get(f.kind)
            if tpl is None:
                lib = lib or self.library(rtype, lang)
                tpl = lib.get(f.kind)
            if tpl is None and f.kind == "enum":
                tpl = self.enum_fallback(templ, lang, rtype)
            if tpl is None:
                raise Unsupported(f"no {f.kind} item of the {lang} codec ({rtype}) to learn from in any entity of the artifact")
            t, helper, var, const = tpl
            if f.kind == "enum" and lang == "ts" and rtype == "R" and "choice(" not in t and re.search(r"\bfunction choice\b", self.text):
                t = "{n}:choice(artifactSqliteText({v},{i}),{H})"
            idx = None if base is None else base + k
            h = None
            if f.kind == "enum" and lang == "ts":
                m_ = re.search(r"\bconst (\w+)\s*:\s*readonly %s\[\]" % re.escape(f.enum_def), self.text)
                h = m_.group(1) if m_ else f"{snake(f.enum_def).upper()}S"
            elif f.kind == "enum":
                w, r = helper_names(f.enum_def, self.dialect)
                h = r if (helper or "").startswith("read_") or (self.dialect["r_pre"] and (helper or "").startswith(self.dialect["r_pre"]) and rtype == "R") else w
            items.append(fill_item(t, f, lang, idx, h, ref_var, self.new_const()).replace("{T}", f.enum_def or ""))
        return items

    def regen(self, stmt):
        """📗 Regenerates the field regions of a renamed (still reference-field) statement."""
        mask = code_mask(stmt, "rs")
        rf, nf = self.rf, self.nf
        done = 0
        m = re.search(r'"%s"' % re.escape(self.new_tbl), stmt)
        if m and re.search(r"insert\w*\(", stmt):
            f = find_list(stmt, mask, m.end(), len(stmt), rf, "rs")
            if f:
                items = item_texts(stmt, f)
                var = find_var(stmt[f.items[f.first - 1][0]:f.items[f.first - 1][1]], "id") if f.first else None
                stmt = swap(stmt, f, self.new_items(self.region_templates(items, f.fields, "rs", None, stmt[f.items[f.first - 1][0]:f.items[f.first - 1][1]] if f.first else None), "rs", None, "W", var))
                done += 1
                mask = code_mask(stmt, "rs")
                if any(g.kind == "f64" for g in nf):
                    close = match_close(stmt, mask, stmt.index("[", m.end()) if "&[" not in stmt[m.end():] else stmt.index("&[", m.end()) + 1)
                    stmt = stmt[: close + 1] + re.sub(r"^(\s*,\s*)(?:&\[\]|[A-Z][A-Z0-9_]*)", lambda mm: mm.group(1) + self.const_name(re.sub(r"[^A-Z0-9_]", "", mm.group(0)) or "X") if "[" not in mm.group(0) else mm.group(1) + self.const_name("X"), stmt[close + 1:])
                    mask = code_mask(stmt, "rs")
        m = re.search(r"\b%s\s*\{" % re.escape(self.new.entity), stmt)
        if m:
            f = find_list(stmt, mask, m.end() - 1, len(stmt), rf, "rs")
            if f:
                items = item_texts(stmt, f)
                idrow = stmt[f.items[f.first - 1][0]:f.items[f.first - 1][1]] if f.first else ""
                var = find_var(idrow, "id")
                stmt = swap(stmt, f, self.new_items(self.region_templates(items, f.fields, "rs", 4, idrow or None), "rs", 4, "R", var))
                done += 1
                mask = code_mask(stmt, "rs")
        strs = [f for f in rf if f.kind in ("String", "enum")]
        nstr = [f for f in nf if f.kind in ("String", "enum")]
        swapped = False
        if strs and re.search(r"\b(repeated|add|admit_strings)\(|for\s+\w+\s+in\s*\[", stmt):
            for sep in (",", ";"):
                f = find_list(stmt, mask, 0, len(stmt), strs, "rs", sep)
                if f:
                    t = {}
                    for k, g in enumerate(strs):
                        t.setdefault(g.kind, template_of(item_texts(stmt, f)[k], g, "rs", None))
                    items = []
                    own = self.nf
                    for g in nstr:
                        tpl = t.get(g.kind) or t.get("String")
                        if tpl is None:
                            raise Unsupported(f"no {g.kind} bound item to learn from")
                        tt, helper, var, const = tpl
                        if g.kind == "enum" and "{H}" not in tt:
                            tt = tt.replace("&{v}.{n}", "{H}({v}.{n})") if "&{v}.{n}" in tt else tt.replace("{v}.{n}", "{H}({v}.{n})")
                        h = helper_names(g.enum_def, self.dialect)[0] if g.kind == "enum" else None
                        items.append(fill_item(tt, g, "rs", None, h, var))
                    stmt = swap(stmt, f, items)
                    done += 1
                    swapped = True
                    break
        if nstr and not swapped and re.search(r"\bfor\s+\w+\s+in\s*\[", stmt):
            m = re.search(r"(\bfor\s+\w+\s+in\s*\[)([^\]]*)(\])", stmt)
            idm = m and re.search(r"&?(\w+)\.id\b", m.group(2))
            if idm:
                extra = [fill_item("{H}({v}.{n})" if g.kind == "enum" else "&{v}.{n}", g, "rs", None, helper_names(g.enum_def, self.dialect)[0] if g.kind == "enum" else None, idm.group(1)) for g in nstr]
                stmt = stmt[: m.end(2)] + "," + ",".join(extra) + stmt[m.end(2):]
                done += 1
        m = re.search(r"entities\(\s*[^,()]+,\s*(?:\"%s\"|[\w\[\]!]+)\s*,\s*(\d+)\s*,\s*([^,()]+(?:\[[^\]]*\])?)\s*," % re.escape(self.new_tbl), stmt)
        if m:
            stmt = stmt[: m.start(1)] + str(4 + len(nf)) + stmt[m.end(1):m.start(2)] + self.floats_name(m.group(2)) + stmt[m.end(2):]
            done += 1
        return stmt, done

    def floats_name(self, old):
        reals = [f for f in self.nf if f.kind == "f64"]
        if not reals:
            return "&[]" if old.strip().startswith("&[") or old.strip() == "&[]" else old
        c = old.strip()
        if c.startswith("&["):
            return self.const_name(c)
        return self.const_name(c)

    def const_name(self, old):
        from core import singular_of
        pairs = [(snake(singular_of(self.ref.plural_camel)).upper(), snake(singular_of(self.new.plural_camel)).upper()), (snake(self.ref.entity).upper(), snake(self.new.entity).upper())]
        for a, b in pairs:
            if b and b in old:
                return old
            if a and a in old:
                return old.replace(a, b)
        return pairs[0][1]

    const_rx = r"\bconst (\w+)\s*:\s*&\[FloatColumn\]"
    empty_consts = "&[]"

    def drop_consts(self, stmt):
        """🪹 A new entity without float columns reads and writes with the empty column list: every float-column constant a cloned statement still names (the reference's or the renamed one that is never declared) becomes the empty list."""
        if any(f.kind == "f64" for f in self.nf):
            return stmt
        existing = set(re.findall(self.const_rx, self.text))
        names = existing | {self.const_name(c) for c in existing}
        return re.sub(r"(?<![\w.$])[A-Z][A-Z0-9_]*(?![\w$])", lambda m: self.empty_consts if m.group(0) in names else m.group(0), stmt)

    def float_consts(self):
        """📖 The `const X:&[FloatColumn]=&[..];` lines of the reference (name -> span) used by the cloned statements."""
        return list(re.finditer(r"^const (\w+)\s*:\s*&\[FloatColumn\]\s*=\s*&\[[^\n]*\];\n", self.text, re.M))


def indent_of(text, pos):
    ls = text.rfind("\n", 0, pos) + 1
    return re.match(r"[ \t]*", text[ls:pos]).group(0) if text[ls:pos].strip() == "" else ""


def insert_after(text, end, new, same_line_ok=True):
    """📔 Insertion text of a cloned statement after the statement that ends at `end`: a new line when the statement ends its line, else a space."""
    nl = text[end:end + 1] == "\n" or text[end:end + 2] == "\r\n"
    return ("\n" + new) if nl else (" " + new)


def ensure_rust_use(text, names):
    """📥 Names that no `use` list of the module imports yet are appended to the `use crate::{..}` list."""
    missing = [n for n in names if not re.search(r"^use [^;]*\b%s\b[^;]*;" % re.escape(n), text, re.M)]
    if not missing:
        return text
    m = re.search(r"^use crate::\{([^}]*)\};", text, re.M)
    if not m:
        return text
    return text[: m.end(1)] + ", " + ", ".join(missing) + text[m.end(1):]


def rust_generic(ctx, ref, ref_tbl, ref_fields, new_fields, sql, target=None, merge=None, capture=None):
    """🧬 Clones the Rust codec statements of the reference for `target` (default: the spec's collection). `merge(rc, clones)` may restructure the cloned statements before they are inserted (nested collections); with `capture` (a list) the clones of a nested child are returned instead of inserted."""
    S = target or ctx.spec.collection
    new_tbl = new_table(ctx, ref, ref_tbl, S.names)
    rc = RustCodec(ctx, ref, ref_tbl, new_tbl, ref_fields, new_fields, sql, S)
    text = rc.text
    if f'"{new_tbl}"' in text:
        return
    stmts, mask = rc.stmts()
    clones, total = [], 0
    for a, b in stmts:
        s = text[a:b]
        if not rc.dedicated(s):
            continue
        new, n = rc.regen(rc.rename(s))
        new = rc.drop_consts(rc.unique_bindings(new, text))
        leftover = [f.snake for f in ref_fields if word(f.snake).search(new) and f.snake not in {g.snake for g in new_fields} and f.snake not in ("id",) and len(f.snake) > 2 and word(f.snake).search(new) and ('.' + f.snake in new or f.snake + ":" in new)]
        if leftover:
            raise Unsupported(f"cloned statement still mentions reference fields {leftover[:3]}: {s[:80]!r}")
        total += n
        clones.append({"a": a, "b": b, "orig": s, "new": new, "ind": indent_of(text, a)})
    if not total:
        raise Unsupported(f"no write/read statement of {ref.entity} found in the Rust codec")
    if not any(f'"{ref_tbl}"' in c["orig"] and re.search(r"insert\w*\(", c["orig"]) for c in clones) or not any(re.search(r"\b%s\s*\{" % re.escape(ref.entity), c["orig"]) and ".push(" in c["orig"] for c in clones):
        raise Unsupported(f"{ref.plural_camel}: the Rust codec lacks its write or its read statement")
    if merge:
        clones = merge(rc, clones)
    if capture is not None:
        capture.extend(clones)
        edits = []
    else:
        edits = [(c["b"], insert_after(text, c["b"], (c["ind"] if text[c["b"]:c["b"] + 1] == "\n" else "") + c["new"])) for c in clones]
    for at, add in sorted(edits, reverse=True):
        text = text[:at] + add + text[at:]
    rc.text = text
    text = rust_extras(rc, text, stmts, mask, capture is None)
    text = ensure_rust_use(text, [S.entity] + [f.enum_def for f in new_fields if f.kind == "enum"])
    ctx.plan.write(rc.path, text)
    return rc


def rust_extras(rc, text, stmts, mask, inline=True):
    ctx, ref, new = rc.ctx, rc.ref, rc.new
    S = ctx.spec.collection
    reals = [f for f in rc.nf if f.kind == "f64"]
    consts = list(re.finditer(r"^const (\w+)\s*:\s*&\[FloatColumn\]\s*=\s*&\[([^\n]*)\];\n", text, re.M))
    used = [m for m in consts if m.group(1) == rc.const_name(m.group(1)) or True]
    ref_up = snake(ref.entity).upper()
    cand = [m for m in consts if ref_up in m.group(1)] or [m for m in consts if re.search(r"\b%s\b" % re.escape(m.group(1)), text[: 0] + "")]
    new_const_line = ""
    if reals:
        base_line = None
        for m in re.finditer(r"\b%s\b" % re.escape(rc.const_name(ref_up)), text):
            pass
        names = set(re.findall(r"\bconst (\w+)\s*:\s*&\[FloatColumn\]", text))
        used_name = None
        wm = re.search(r'"%s"[^;]*?\],\s*([A-Z_0-9]+)\s*[),]' % re.escape(rc.ref_tbl), text)
        if wm:
            used_name = wm.group(1)
        if used_name and used_name in names:
            src = next(m for m in consts if m.group(1) == used_name)
            new_name = rc.const_name(used_name)
            idxs = ",".join(f"FloatColumn::Binary64({4 + k})" for k, f in enumerate(rc.nf) if f.kind == "f64")
            line = re.sub(r"(&\[)[^\n]*(\];)", lambda mm: mm.group(1) + idxs + mm.group(2), src.group(0).replace(used_name, new_name, 1), count=1) if "FloatColumn]" in src.group(0) else src.group(0)
            line = re.sub(r"=\s*(&\[)[^\n]*\];", lambda mm: mm.group(0).split("&[")[0] + "&[" + idxs + "];", src.group(0).replace(used_name, new_name, 1))
            at = src.end()
            if new_name not in names:
                text = text[:at] + line + text[at:]
        else:
            first = consts[-1] if consts else None
            if first is None:
                raise Unsupported("no FloatColumn constant in the Rust codec to clone")
            new_name = snake(new.entity).upper()
            idxs = ",".join(f"FloatColumn::Binary64({4 + k})" for k, f in enumerate(rc.nf) if f.kind == "f64")
            line = re.sub(r"=\s*&\[[^\n]*\];", lambda mm: mm.group(0).split("&[")[0] + "&[" + idxs + "];", first.group(0).replace(first.group(1), new_name, 1))
            text = text[: first.end()] + line + text[first.end():]
    for f in rc.nf:
        if f.kind == "enum":
            w, r = helper_names(f.enum_def, rc.dialect)
            if not re.search(r"\bfn\s+%s\b" % re.escape(w), text):
                reads = list(re.finditer(r"^fn read_\w+\([^\n]*\n", text, re.M))
                at = reads[-1].end() if reads else text.find("\n", text.find("fn invalid")) + 1
                text = text[:at] + enum_fn_text(f, rc.dialect) + text[at:]
    return inline_items(rc, text) if inline else text


def enclosing_open(text, mask, pos):
    """🔓 Index of the unmatched opening bracket in front of `pos`."""
    depth = 0
    for i in range(pos - 1, -1, -1):
        if not mask[i]:
            continue
        if text[i] in ")]}":
            depth += 1
        elif text[i] in "([{":
            if depth == 0:
                return i
            depth -= 1
    return None


def whole_call(text, mask, open_at):
    """📟 (start, end) of the statement `name(args)?;` whose parenthesis opens at `open_at`: a count taken as a call argument (`admit_rows(&mut b,&mut rows,self.x.len())?;`) is cloned as a whole call, never as an extra argument."""
    name = re.search(r"((?:[A-Za-z_][\w.]*\s*=\s*)?[A-Za-z_][\w:.]*)\s*$", text[:open_at])
    if not name:
        return None
    close = match_close(text, mask, open_at)
    tail = re.match(r"\s*\??\s*;", text[close + 1:])
    return (name.start(1), close + 1 + tail.end()) if tail else None


def inline_items(rc, text):
    """📓 Cloned list items in shared statements: `self.joints.len()` in row-count lists, the plural identifier in result literals."""
    ref, new = rc.ref, rc.new
    mask = code_mask(text, "rs")
    edits = []
    stmts = []
    for o, c in fn_bodies(text, mask, "rs"):
        stmts += statements(text, mask, o, c)
        tail = tail_expression(text, mask, o, c)
        if tail:
            stmts.append(tail)
    for a, b in stmts:
        s = text[a:b]
        if rc.dedicated(s) or new.plural_snake in s:
            continue
        for m in re.finditer(r"(?<![\w])(?:self\.|snapshot\.|s\.)?%s(?:\.len\(\))?(?![\w])" % re.escape(ref.plural_snake), s):
            pos = a + m.start()
            if not mask[pos]:
                continue
            lo = pos
            depth = 0
            while lo > a:
                ch = text[lo - 1]
                if ch in ")]}":
                    depth += 1
                elif ch in "([{":
                    if depth == 0:
                        break
                    depth -= 1
                elif ch == "," and depth == 0:
                    break
                lo -= 1
            hi = pos + len(m.group(0))
            depth = 0
            while hi < b:
                ch = text[hi]
                if ch in "([{":
                    depth += 1
                elif ch in ")]}":
                    if depth == 0:
                        break
                    depth -= 1
                elif ch == "," and depth == 0:
                    break
                hi += 1
            item = text[lo:hi]
            if re.fullmatch(r"\s*%s\s*:[^;]*" % re.escape(ref.plural_snake), item) and not item.strip().startswith(ref.plural_snake + ".") and mask[pos]:
                edits.append((hi, "," + ref.rename(item, new, text)))
            elif item.strip() == m.group(0) or item.strip().endswith(m.group(0)):
                opener = enclosing_open(text, mask, lo)
                call = whole_call(text, mask, opener) if opener is not None and text[opener] == "(" else None
                if call:
                    edits.append((call[1], " " + ref.rename(text[call[0]:call[1]], new, text)))
                else:
                    edits.append((hi, "," + ref.rename(item, new, text)))
    for at, add in sorted(set(edits), reverse=True):
        text = text[:at] + add + text[at:]
    return text


class TsCodec(RustCodec):
    """📒 The TypeScript counterpart: statements are found by the table literal, `.push({` of the collection and the Binary64 constant, field names are camel."""

    unquoted = "camel"
    const_rx = r"\bconst ([A-Z][A-Z0-9_]*)\s*:\s*readonly Ieee754Column"
    empty_consts = "[]"

    def __init__(self, ctx, ref, ref_tbl, new_tbl, ref_fields, new_fields, sql, n_tables, target=None):
        RustCodec.__init__(self, ctx, ref, ref_tbl, new_tbl, ref_fields, new_fields, sql, target)
        L = ctx.layout
        self.path = L.p(SQLITE + "/🟦️.ts")
        self.text = ctx.plan.read(self.path)
        self.n_tables = n_tables
        self.ref_const = None
        self._lib = {}
        mask = code_mask(self.text, "ts")
        self.all_stmts = []
        for o, c in fn_bodies(self.text, mask, "ts"):
            self.all_stmts += statements(self.text, mask, o, c)
        wm = re.search(r'"%s"[^;]*?\],\s*([A-Z][A-Z0-9_]*)\s*,' % re.escape(ref_tbl), self.text)
        self.ref_const = wm.group(1) if wm else None

    def stmts(self):
        if self.all_stmts:
            return self.all_stmts, code_mask(self.text, "ts")
        mask = code_mask(self.text, "ts")
        out = []
        for o, c in fn_bodies(self.text, mask, "ts"):
            out += statements(self.text, mask, o, c)
        return out, mask

    def is_const(self, token):
        return token == self.ref_const or token in set(re.findall(r"\bconst ([A-Z][A-Z0-9_]*)\s*:\s*readonly Ieee754Column", self.text))

    def dedicated(self, s):
        r = self.ref
        if f'"{self.ref_tbl}"' in s and "write(" in s:
            return True
        if re.search(r"\b%s\.push\(\s*\{" % re.escape(r.plural_camel), s) or re.search(r"\.%s\.push\(\s*\{" % re.escape(r.plural_camel), s):
            return True
        if s.startswith("for") and "[" not in s.split("{", 1)[0].replace("[index]", "") and re.search(r"\.%s\.length|\bof\s+(?:await\s+ordered\()?%s\w*|%sRows|%sGroups" % (re.escape(r.plural_camel), re.escape(self.slug_ref.camel), re.escape(self.slug_ref.camel), re.escape(self.slug_ref.camel)), s.split("{", 1)[0]):
            return True
        return False

    def regen(self, stmt):
        mask = code_mask(stmt, "ts")
        rf, nf = self.rf, self.nf
        done = 0
        m = re.search(r'"%s"' % re.escape(self.new_tbl), stmt)
        if m and "write(" in stmt:
            f = find_list(stmt, mask, m.end(), len(stmt), rf, "ts")
            if f:
                idrow = stmt[f.items[f.first - 1][0]:f.items[f.first - 1][1]] if f.first else ""
                var = find_var(idrow, "id")
                stmt = swap(stmt, f, self.new_items(self.region_templates(item_texts(stmt, f), f.fields, "ts", None, idrow or None), "ts", None, "W", var))
                done += 1
                mask = code_mask(stmt, "ts")
                if self.ref_const:
                    stmt = stmt.replace(self.ref_const, self.const_name(self.ref_const)) if any(g.kind == "f64" for g in nf) else stmt
        fm = re.search(r"\bfields\(\s*\w+\s*,\s*\[", stmt)
        if fm:
            f = find_list(stmt, mask, fm.end() - 1, len(stmt), rf, "ts")
            if f:
                stmt = swap(stmt, f, [f'"{g.camel}"' for g in nf])
                done += 1
                mask = code_mask(stmt, "ts")
        pm = re.search(r"\.push\(\s*\{", stmt)
        if pm:
            f = find_list(stmt, mask, pm.end() - 1, len(stmt), rf, "ts")
            if f:
                idrow = stmt[f.items[f.first - 1][0]:f.items[f.first - 1][1]] if f.first else ""
                var = find_var(idrow, "id")
                tmpl = self.region_templates(item_texts(stmt, f), f.fields, "ts", 4, idrow or None)
                items = self.new_items(tmpl, "ts", 4, "R", var)
                const = self.const_name(self.ref_const) if self.ref_const else None
                items = [re.sub(r"(readBinary64\(\w+,\d+,)[A-Z][A-Z0-9_]*\)", lambda mm: mm.group(1) + (const or "") + ")", it) for it in items]
                stmt = swap(stmt, f, items)
                done += 1
                mask = code_mask(stmt, "ts")
        em = re.search(r"entities\(\s*(\w+)\[(\d+)\]!?,\s*(\d+)\s*,\s*([A-Z][A-Z0-9_]*|\[\])\s*,", stmt)
        if em:
            stmt = stmt[: em.start(2)] + str(self.n_tables) + stmt[em.end(2):em.start(3)] + str(4 + len(nf)) + stmt[em.end(3):em.start(4)] + (self.const_name(em.group(4)) if any(g.kind == "f64" for g in nf) else "[]") + stmt[em.end(4):]
            done += 1
        return stmt, done

    def rename(self, s, evidence=None):
        return RustCodec.rename(self, s, '"aB"')

    def unique_ts(self, stmt):
        for name, new in self.bound.items():
            stmt = re.sub(r"(?<![\w.$])%s(?![\w$])" % re.escape(name), new, stmt)
        head = re.match(r"\s*(?:const|let)\s+", stmt)
        for m in (re.finditer(r"(?:^|,)\s*(?:(?:const|let)\s+)?([a-zA-Z_$][\w$]*)\s*[=:]", stmt.split(";")[0]) if head else []):
            name = m.group(1)
            if re.search(r"\b(?:const|let)\s+%s\s*[=:]" % re.escape(name), self.text):
                new = f"{self.slug_new.camel}{name[0].upper() + name[1:] if name.lower().endswith(('rows', 'groups')) else 'Rows'}"
                self.bound[name] = new
                stmt = re.sub(r"(?<![\w.$])%s(?![\w$])" % re.escape(name), new, stmt)
        return stmt

    def ref_rows_name(self, text):
        """🗝️ The variable the declarator list names the reference rows with (`placementRows`), or None."""
        tables = tables_of(self.sql)
        for m in re.finditer(r"(?<![\w$])(\w+)\s*=\s*await\s+entities\(\s*(\w+)\[(\d+)\]!?,", text):
            if self.ref_tbl in tables and int(m.group(3)) == tables.index(self.ref_tbl):
                return m.group(1)
        return None

    def declarator_calls(self, text):
        """📋 {new rows variable: `await entities(..)` call} of the declarators the reference table has, as they would be cloned."""
        out = {}
        tables = tables_of(self.sql)
        for m in re.finditer(r"(?<![\w$])(\w+)\s*=\s*await\s+entities\(\s*(\w+)\[(\d+)\]!?,\s*(\d+)\s*,\s*([A-Z][A-Z0-9_]*|\[\])\s*,\s*(\w+)\)", text):
            if self.ref_tbl in tables and int(m.group(3)) == tables.index(self.ref_tbl):
                out[self.rename(m.group(1))] = f"await entities({m.group(2)}[{self.n_tables}]!,{4 + len(self.nf)},{self.const_name(m.group(5)) if any(g.kind == 'f64' for g in self.nf) else '[]'},{m.group(6)})"
        return out

    def declarators(self, text, extra=None):
        """📌 `const a=await entities(..),b=await entities(..)` lists: the declarators of the reference table are cloned inline, and so are the `x=await groups(refRows,parent,o)` declarators of the artifacts that group rows by parent (`extra`: {"entities"|"groups": text} appended behind the new declarator, for nested children)."""
        edits = []
        extra = extra or {}
        ref_rows = None
        for m in re.finditer(r"(?<![\w$])(\w+)\s*=\s*await\s+entities\(\s*(\w+)\[(\d+)\]!?,\s*(\d+)\s*,\s*([A-Z][A-Z0-9_]*|\[\])\s*,\s*(\w+)\)", text):
            tables = tables_of(self.sql)
            if self.ref_tbl in tables and int(m.group(3)) == tables.index(self.ref_tbl):
                ref_rows = m.group(1)
                new_name = self.rename(m.group(1))
                item = f"{new_name}=await entities({m.group(2)}[{self.n_tables}]!,{4 + len(self.nf)},{self.const_name(m.group(5)) if any(g.kind == 'f64' for g in self.nf) else '[]'},{m.group(6)})"
                edits.append((m.end(), "," + item + extra.get("entities", "")))
        if ref_rows:
            for m in re.finditer(r"(?<![\w$])(\w+)\s*=\s*await\s+groups\(\s*%s\s*,\s*(\w+)\s*,\s*(\w+)\s*\)" % re.escape(ref_rows), text):
                edits.append((m.end(), "," + f"{self.rename(m.group(1))}=await groups({self.rename(ref_rows)},{m.group(2)},{m.group(3)})" + extra.get("groups", "")))
        for at, add in sorted(edits, reverse=True):
            text = text[:at] + add + text[at:]
        return text, bool(edits)


def ts_type_expr(f):
    return {"String": "string", "f64": "Binary64", "u8": "number", "u32": "number", "bool": "boolean"}.get(f.kind) or f.enum_def


def ts_types(tc, text):
    """📎 The TypeScript row type of the new collection: when the root type names the reference rows by an alias (`prestressSteels:En1992SqliteSteel[]`), a declaration with the new fields is added behind the alias declaration (same shape family: Omit with Binary64 members, Exact alias or explicit object)."""
    ref, S = tc.ref, tc.target
    m = re.search(r"(?<![\w$])%s\s*:\s*([A-Za-z_$][\w$]*)\[\]" % re.escape(ref.plural_camel), text)
    if not m:
        return text, None
    ref_type = m.group(1)
    d = re.search(r"^export type %s\s*=([^\n]*);\n" % re.escape(ref_type), text, re.M)
    if not d:
        return text, None
    short = re.sub(r"^En\d{4}", "", S.entity)
    new_type = re.sub(r"Sqlite\w+$", "Sqlite" + short, ref_type) if "Sqlite" in ref_type else ref_type + short
    if re.search(r"^export type %s\s*=" % re.escape(new_type), text, re.M):
        return text, (ref_type, new_type)
    body = d.group(1)
    reals = [f for f in tc.nf if f.kind == "f64"]
    if body.strip().startswith("Omit<") or "Exact<" in body:
        if reals and body.strip().startswith("Omit<"):
            nb = f"Omit<{S.entity},{'|'.join(chr(34) + f.camel + chr(34) for f in reals)}>&{{{';'.join(f.camel + ':Binary64' for f in reals)}}}"
        elif "Exact<" in body:
            nb = f"Exact<{S.entity}>"
        else:
            nb = S.entity
    elif body.strip().startswith("{"):
        nb = "{id:string;" + ";".join(f"{f.camel}:{ts_type_expr(f)}" for f in tc.nf) + "}"
    else:
        nb = S.entity
    line = f"export type {new_type}={nb};\n"
    text = text[: d.end()] + line + text[d.end():]
    return text, (ref_type, new_type)


def ts_inline(tc, text):
    """📏 Cloned list members of the root type, the import list, quoted name lists and `s.name` arrays that mention the reference collection."""
    ref, new = tc.ref, tc.new
    mask = code_mask(text, "ts")
    edits = []
    stmt_spans = []
    for o, c in fn_bodies(text, mask, "ts"):
        stmt_spans += statements(text, mask, o, c)
    in_dedicated = lambda pos: any(a <= pos < b and tc.dedicated(text[a:b]) for a, b in stmt_spans)
    pats = [r'"%s"' % re.escape(ref.plural_camel), r"(?<![\w$.])%s\s*:" % re.escape(ref.plural_camel), r"(?<![\w$])[sd]\.%s(?![\w$])" % re.escape(ref.plural_camel), r"(?<![\w$])%s(?![\w$])(?!\[\s*\")" % re.escape(ref.entity), r"(?<=[{,])%s(?=[,}])" % re.escape(ref.plural_camel)]
    for pat in pats:
        for m in re.finditer(pat, text):
            pos = m.start()
            if in_dedicated(pos) or re.search(r"\b(?:Omit|Pick|Exact|Readonly|Partial)<$", text[max(0, pos - 9):pos]):
                continue
            line_s = text.rfind("\n", 0, pos) + 1
            lo, depth = pos, 0
            while lo > line_s:
                ch = text[lo - 1]
                if mask[lo - 1] and ch in ")]}>":
                    depth += 1
                elif mask[lo - 1] and ch in "([{<":
                    if depth == 0:
                        break
                    depth -= 1
                elif mask[lo - 1] and ch in ",|;" and depth == 0:
                    break
                lo -= 1
            hi, depth, sep = pos + len(m.group(0)), 0, ","
            line_e = text.find("\n", pos)
            line_e = len(text) if line_e < 0 else line_e
            while hi < line_e:
                ch = text[hi]
                if mask[hi] and ch in "([{<":
                    depth += 1
                elif mask[hi] and ch in ")]}>":
                    if depth == 0:
                        break
                    depth -= 1
                elif mask[hi] and ch in ",|;" and depth == 0:
                    sep = ch
                    break
                hi += 1
            before = text[lo - 1] if lo > line_s and text[lo - 1] in ",|;" else None
            sep = sep if hi < line_e and text[hi] in ",|;" else (before or ",")
            item = text[lo:hi]
            if not item.strip() or "\n" in item:
                continue
            if sep == ";" and re.match(r"\s*[\w$]+\s*(?::[^=;]+)?=", item) and re.search(r"\b(?:const|let|var)\b[^;]*$", text[line_s:lo]):
                sep = ","
            renamed = ref.rename(item, new, text, "camel")
            renamed = tc.slug_ref.rename(renamed, tc.slug_new, text, "camel")
            if renamed != item and renamed.strip() not in text[line_s:line_e if line_e > line_s else len(text)]:
                edits.append((hi, sep + renamed.strip() if not item.startswith(" ") else sep + renamed))
    seen = set()
    for at, add in sorted(set(edits), reverse=True):
        if (at, add) in seen:
            continue
        seen.add((at, add))
        text = text[:at] + add + text[at:]
    return text


def ts_generic(ctx, ref, ref_tbl, ref_fields, new_fields, sql, n_tables, target=None, merge=None, capture=None):
    """🧬 TypeScript counterpart of `rust_generic` (same `target`, `merge`, `capture` contract)."""
    S = target or ctx.spec.collection
    new_tbl = new_table(ctx, ref, ref_tbl, S.names)
    tc = TsCodec(ctx, ref, ref_tbl, new_tbl, ref_fields, new_fields, sql, n_tables, S)
    text = tc.text
    if f'"{new_tbl}"' in text:
        return
    clones, total = [], 0
    pre = re.search(r"(?<![\w$])%s\s*:\s*([A-Za-z_$][\w$]*)\[\]" % re.escape(ref.plural_camel), text)
    short = re.sub(r"^En\d{4}", "", S.entity)
    pre_pair = (pre.group(1), re.sub(r"Sqlite\w+$", "Sqlite" + short, pre.group(1)) if "Sqlite" in pre.group(1) else pre.group(1) + short) if pre else None
    for a, b in tc.all_stmts:
        s = text[a:b]
        if not tc.dedicated(s):
            continue
        new, n = tc.regen(tc.rename(s))
        new = tc.drop_consts(tc.unique_ts(new))
        if pre_pair:
            new = re.sub(r"(?<![\w$])%s(?![\w$])(?=\[\])" % re.escape(pre_pair[0]), pre_pair[1], new)
        total += n
        clones.append({"a": a, "b": b, "orig": s, "new": new})
    if not total:
        raise Unsupported(f"no write/read statement of {ref.entity} found in the TypeScript codec")
    if not any(f'"{ref_tbl}"' in c["orig"] and "write(" in c["orig"] for c in clones) or not any(re.search(r"\.push\(\s*\{", c["orig"]) for c in clones):
        raise Unsupported(f"{ref.plural_camel}: the TypeScript codec lacks its write or its read statement")
    if merge:
        clones = merge(tc, clones)
    if capture is not None:
        capture.extend(clones)
        clones = []
    edits = [(c["b"], (" " if text[c["b"]:c["b"] + 1] != "\n" else "\n ") + c["new"]) for c in clones]
    for at, add in sorted(edits, reverse=True):
        text = text[:at] + add + text[at:]
    tc.text = text
    if capture is None:
        text, _ = tc.declarators(text, getattr(tc, "declarator_extra", None))
    else:
        tc.decls = tc.declarator_calls(text)
    reals = [f for f in new_fields if f.kind == "f64"]
    consts = list(re.finditer(r"^const ([A-Z][A-Z0-9_]*):readonly Ieee754Column\[\]=\[([0-9,]*)\](\.map\([^\n]*\);)\n", text, re.M))
    if reals and tc.ref_const:
        src = next((m for m in consts if m.group(1) == tc.ref_const), None)
        base = src or (consts[-1] if consts else None)
        if base is None:
            raise Unsupported("no Ieee754Column constant in the TypeScript codec to clone")
        name = tc.const_name(tc.ref_const)
        if not re.search(r"^const %s:" % re.escape(name), text, re.M):
            idx = ",".join(str(4 + k) for k, f in enumerate(new_fields) if f.kind == "f64")
            text = text[: base.end()] + f"const {name}:readonly Ieee754Column[]=[{idx}]{base.group(3)}\n" + text[base.end():]
    for f in new_fields:
        if f.kind == "enum":
            cname = f"{snake(f.enum_def).upper()}S"
            if not re.search(r"^const %s:" % cname, text, re.M):
                kinds = list(re.finditer(r"^const \w+S:readonly \w+\[\]=\[[^\n]*\n", text, re.M))
                if not kinds:
                    raise Unsupported("no enum choice constants in the TypeScript codec")
                line = f"const {cname}:readonly {f.enum_def}[]=[{','.join(chr(34) + t + chr(34) for t in f.tokens)}];\n"
                text = text[: kinds[-1].end()] + line + text[kinds[-1].end():]
    text, type_pair = ts_types(tc, text)
    tc.type_pair = type_pair
    if capture is None:
        text = ts_inline(tc, text)
        if type_pair:
            text = text.replace(f"{S.names.plural_camel}:{type_pair[0]}[]", f"{S.names.plural_camel}:{type_pair[1]}[]")
    names = [S.entity] + [f.enum_def for f in new_fields if f.kind == "enum"]

    def imp(m):
        have = [x.strip() for x in m.group(2).split(",") if x.strip()]
        add = [n for n in names if n not in have and not any(h.split(" as ")[-1] == n for h in have)]
        return m.group(1) + m.group(2).rstrip() + ("," if add and m.group(2).strip() else "") + ",".join(add) + m.group(3)
    text = re.sub(r"(import type\s*\{)([^}]*\b%s\b[^}]*)(\})" % re.escape(ref.entity), imp, text, count=1)
    ctx.plan.write(tc.path, text)
    return tc


def adm_generic(ctx, ref, ref_tbl, ref_fields, new_fields, new_tbl, field_id):
    """📐 Native admission: the statement that admits the reference rows is cloned with the new table, root field id and cell kinds."""
    import l_sqlite
    L = ctx.layout
    path = L.p(SQLITE + "/🛂️admission/🦀️.rs")
    if not ctx.plan.exists(path):
        return False
    text = ctx.plan.read(path)
    if f'"{new_tbl}"' in text:
        return True
    mask = code_mask(text, "rs")
    for o, c in fn_bodies(text, mask, "rs"):
        for a, b in statements(text, mask, o, c):
            s = text[a:b]
            if f'"{ref_tbl}"' not in s or "n::entity(" not in s:
                continue
            new = s.replace(ref_tbl, new_tbl)
            new = re.sub(r"n::field\(\s*root\s*,\s*\d+\s*\)", f"n::field(root,{field_id})", new, count=1)
            opener = new.find("[Text(0)")
            if opener < 0:
                raise Unsupported("admission cell list of the reference not found")
            closer = match_close(new, code_mask(new, "rs"), opener)
            cm = type("Span", (), {"start": lambda self, g: opener + 1, "end": lambda self, g: closer})()
            spec_fields_ = ctx.spec.collection.fields
            typed = "Enumeration(" in text
            cell = lambda f, i: "Enumeration(%d,&[%s])" % (i, ",".join('"%s"' % t for t in f.tokens)) if typed and f.ty == "enum" else l_sqlite.adm_cell(f, i)
            cells = "Text(0)" + "".join("," + cell(f, i + 1) for i, f in enumerate(spec_fields_))
            new = new[: cm.start(1)] + cells + new[cm.end(1):]
            cst = re.search(r",\s*([A-Z][A-Z0-9_]*|&\[\])\s*\)\s*\?\s*;", new)
            if cst:
                reals = any(f.kind == "f64" for f in new_fields)
                const = snake(ctx.spec.collection.names.entity).upper() if False else None
                old = cst.group(1)
                nc = (old.replace(snake(ref.entity).upper(), snake(ctx.spec.collection.names.entity).upper()) if old != "&[]" else "&[]")
                if reals and old == "&[]":
                    nc = snake(ctx.spec.collection.names.entity).upper()
                if not reals:
                    nc = "&[]"
                new = new[: cst.start(1)] + nc + new[cst.end(1):]
            text = text[:b] + ("\n" if text[b:b + 1] == "\n" else " ") + new + text[b:][0:0] + text[b:]
            ctx.plan.write(path, text)
            return True
    raise Unsupported("admission statement of the reference table not found")


def candidates(ctx):
    S = ctx.spec.collection
    return [Names(p, e) for p, e in reversed(ctx.collections) if p not in ("cases", S.plural)]


def try_refs(ctx, fn, what):
    """🛠 Runs `fn(ref, table, ref_fields)` for the reference candidates (last collection first) until one aligns with the codec statements; the failures of all candidates are reported together."""
    sql = ctx.plan.read(ctx.layout.p(SQLITE + "/🗄️.sql"))
    errors = []
    for ref in candidates(ctx):
        try:
            tbl = table_of(ctx, ref, sql)
            rf = schema_fields(ctx, ref.entity)
            if not rf or any(f.kind is None for f in rf):
                continue
            return fn(ref, tbl, rf, sql), ref
        except Unsupported as error:
            errors.append(f"{ref.plural_camel}: {error}")
    raise Unsupported(f"{what}: no reference collection aligns with the statements ({'; '.join(errors[:3])})")


def semantic_path(ctx):
    return ctx.layout.p(SQLITE + "/🧪️tests/🧫️fixtures/🎛️semantic.json")


def contract(ctx, old_sql, new_sql, row_delta=None):
    """🪒 Replays the DDL with python `sqlite3` into `schemaBytes` and `tableWidths`; per-case `valueBytes` grow by the bytes of the new cells of the rows the sample holds (`row_delta`: table -> (rows by case id, bytes per row))."""
    import l_sqlite
    path = semantic_path(ctx)
    if not ctx.plan.exists(path):
        ctx.plan.note("contract: this artifact keeps no semantic.json (control/choices fixtures only)")
        return
    import json
    sem = json.loads(ctx.plan.read(path))
    nb, nw = l_sqlite.replay(new_sql)
    changed = False
    if (sem["schemaBytes"], sem["tableWidths"]) != (nb, nw):
        sem["schemaBytes"], sem["tableWidths"] = nb, nw
        changed = True
    notes = []
    for case in sem.get("cases", []):
        for table, (rows_by_case, per_row) in (row_delta or {}).items():
            rows = rows_by_case.get(case["id"], rows_by_case.get("*", 0))
            if rows is None:
                notes.append(f"case {case['id']}: rows of {table} unknown")
                continue
            if rows * per_row:
                case["valueBytes"] += rows * per_row
                changed = True
    if not changed:
        return
    write_contract(ctx, sem)
    ctx.plan.note(f"contract replayed with python sqlite3: schemaBytes {nb}, {len(nw)} tables" + (" (" + "; ".join(notes) + ")" if notes else ""))


def write_contract(ctx, sem):
    """✍️ Writes `semantic.json` and the contract literals of the TypeScript test (schema bytes, table widths, cases)."""
    import json
    ctx.plan.write(semantic_path(ctx), json.dumps(sem, indent=2, ensure_ascii=False) + "\n")
    ts_path = ctx.layout.p(SQLITE + "/🧪️tests/🟦️.ts")
    if ctx.plan.exists(ts_path):
        t = ctx.plan.read(ts_path)
        t = re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+', lambda m: m.group(1) + str(sem["schemaBytes"]), t)
        t = re.sub(r'(normSemanticContract\["tableWidths"\]\)\.toEqual\()\{[^}]*\}', lambda m: m.group(1) + json.dumps(sem["tableWidths"], separators=(",", ":")), t)
        t = re.sub(r'(normSemanticContract\["cases"\]\)\.toEqual\()\[.*?\]\)', lambda m: m.group(1) + json.dumps(sem["cases"], separators=(",", ":")) + ")", t, count=1)
        ctx.plan.write(ts_path, t)
    tables, columns = len(sem["tableWidths"]), max(sem["tableWidths"].values())

    def short(m, actual):
        return m.group(1) + str(int(m.group(2)) if int(m.group(2)) < actual else actual - 1) + m.group(3)
    for rel, accepted, shorts in (("🧪️tests/🟦️.ts", r"(maxSchemaBytes:normSemanticContract\.schemaBytes,maxTables:)\d+(,maxColumns:)\d+", ((r"(\{\.\.\.limits,maxTables:)(\d+)(\})", tables), (r"(\{\.\.\.limits,maxColumns:)(\d+)(\})", columns))),
                                  ("🧪️tests/🦀️.rs", r"(max_schema_bytes:contract\[\"schemaBytes\"\]\.as_u64\(\)\.unwrap\(\)as usize,max_tables:)\d+(,max_columns:)\d+", ((r"(SqliteDatabaseLimits\{max_tables:)(\d+)(,\.\.limits\})", tables), (r"(SqliteDatabaseLimits\{max_columns:)(\d+)(,\.\.limits\})", columns)))):
        path = ctx.layout.p(SQLITE + "/" + rel)
        if not ctx.plan.exists(path):
            continue
        text = ctx.plan.read(path)
        text = re.sub(accepted, lambda m: m.group(1) + str(tables) + m.group(2) + str(columns), text)
        for rx, actual in shorts:
            text = re.sub(rx, lambda m, a=actual: short(m, a), text)
        ctx.plan.write(path, text)


def nested_donors(sql, short):
    """🪆 (parent table, child table) of every nested donor in the DDL: a table whose foreign key points at another entity table instead of the document."""
    out = []
    for t in tables_of(sql):
        m = re.search(r"CREATE TABLE\s+%s\s*\((.*?)\);" % re.escape(t), sql, re.S)
        fk = re.search(r"REFERENCES\s+(\w+)\(id\)", m.group(1)) if m else None
        if fk and fk.group(1) != short + "_document":
            out.append((fk.group(1), t))
    return out


def nested_shape(text, lang, parent, child):
    """🔬 (statements that name both tables, statements that name only the child table) of one codec text, measured on its function-body statements: the child rows are written inline in the parent's statement (a loop inside the parent loop) or in a statement of their own."""
    mask = code_mask(text, lang)
    both = only = 0
    for o, c in fn_bodies(text, mask, lang):
        for a, b in statements(text, mask, o, c):
            s = text[a:b]
            if f'"{child}"' in s:
                if f'"{parent}"' in s:
                    both += 1
                else:
                    only += 1
    return both, only


def nested_report(ctx):
    """🧭 Why a nested collection stops in the learned codec path, measured on the codec text: the nested donors of the DDL and, per donor, in how many Rust and TypeScript statements the child table is written inline in the parent's statement or in a statement of its own."""
    L, S = ctx.layout, ctx.spec.collection
    sql = ctx.plan.read(L.p(SQLITE + "/🗄️.sql"))
    donors = nested_donors(sql, ctx.short)
    if not donors:
        return f"nested rows ({S.nested[0].plural}): no table of the DDL has a foreign key to another entity table, so there is no nested donor to learn the child table, its write loop and its read loop from"
    texts = {lang: (ctx.plan.read(L.p(SQLITE + "/" + f)), key) for lang, f, key in (("Rust", "🦀️.rs", "rs"), ("TypeScript", "🟦️.ts", "ts"))}
    parts = []
    for parent, child in donors[:3]:
        cells = []
        for lang, (text, key) in texts.items():
            both, only = nested_shape(text, key, parent, child)
            cells.append(f"{lang} {both} inline in the {parent} statement, {only} separate")
        parts.append(f"{parent} -> {child} ({'; '.join(cells)})")
    return f"nested rows ({S.nested[0].plural}): {len(donors)} nested donors in the DDL: " + "; ".join(parts) + "; the parent and child statements, the per-parent counting loops of the bound and admission chains and the child groups of the read path are cloned together by the en1997 templates only"


def run_collection(ctx):
    """🧽 The learned codec path for a new flat collection (any artifact whose codecs the en1997 templates do not match)."""
    S = ctx.spec.collection
    L = ctx.layout
    if S.nested:
        import l_nestedsql
        return l_nestedsql.run(ctx)
    sql_path = L.p(SQLITE + "/🗄️.sql")
    sql = ctx.plan.read(sql_path)
    old_sql = sql
    n_tables = len(tables_of(sql))
    nf = spec_fields(S)

    def ddl_step(ref, tbl, rf, sql_):
        return ddl_generic(ctx, sql_, ref, tbl, rf, new_table(ctx, ref, tbl, S.names), nf)
    ddl, ref0 = try_refs(ctx, ddl_step, "sqlite DDL")
    new_sql = sql.rstrip("\n") + "\n" + ddl
    if f"CREATE TABLE {new_table(ctx, ref0, table_of(ctx, ref0, sql), S.names)}" not in sql:
        ctx.plan.write(sql_path, new_sql)
    else:
        new_sql = sql
    _, rref = try_refs(ctx, lambda ref, tbl, rf, s: rust_generic(ctx, ref, tbl, rf, nf, s), "Rust codec")
    _, tref = try_refs(ctx, lambda ref, tbl, rf, s: ts_generic(ctx, ref, tbl, rf, nf, s, n_tables), "TypeScript codec")
    try_refs(ctx, lambda ref, tbl, rf, s: adm_generic(ctx, ref, tbl, rf, nf, new_table(ctx, ref, tbl, S.names), len(ctx.snapshot["properties"])), "native admission") if ctx.plan.exists(L.p(SQLITE + "/🛂️admission/🦀️.rs")) else None
    contract(ctx, old_sql, new_sql)
    ctx.plan.note(f"sqlite codecs cloned from {rref.plural_camel} (Rust), {tref.plural_camel} (TypeScript), {ref0.plural_camel} (DDL)")


def leaf_ddl(ctx, sql, tbl, new):
    """🪥 The DDL of an existing table with one more column definition behind the last field column (and its ieee754 pair, grouped at the end or interleaved as the table does); the gaps between untouched definitions are kept."""
    m, close, items = ddl_body(sql, tbl)
    defs = [sql[a:b] for a, b in items]
    gaps = [sql[items[k][1]:items[k + 1][0]] for k in range(len(items) - 1)]
    roles = ["X" if k < 4 else {"bits": "B", "class": "C"}.get(def_kind(d), "F") for k, d in enumerate(defs)]
    if new.snake in [def_name(d) for d in defs]:
        return sql, False
    tmpl = ddl_templates(sql)
    t = tmpl.get(new.kind) or (re.sub(r"\b(?:255|4294967295)\b", str(INT_MAX[new.kind]), tmpl.get("u8" if new.kind == "u32" else "u32") or "") if new.kind in INT_MAX else None)
    if not t:
        raise Unsupported(f"no {new.kind} column definition to learn from in the DDL")
    field_def = t.replace("{n}", new.snake)
    if new.kind == "enum":
        vals_m = re.search(r"IN\s*\(([^)]*)\)", field_def)
        sep = "," if re.search(r"','", vals_m.group(1)) else ", "
        field_def = re.sub(r"IN\s*\(([^)]*)\)", lambda mm: mm.group(0)[: mm.group(0).index("(") + 1] + sep.join("'%s'" % v for v in new.tokens) + ")", field_def)
    pair = []
    if new.kind == "f64":
        if "bits" not in tmpl:
            raise Unsupported("no ieee754 column definitions in the DDL")
        pair = [tmpl["bits"].replace("{n}", new.snake), tmpl["class"].replace("{n}", new.snake)]
    last_f = max(k for k, r in enumerate(roles) if r in "XF")
    seq = [(k, defs[k], roles[k]) for k in range(len(defs))]
    pos = last_f + 1
    seq.insert(pos, (None, field_def, "F"))
    if pair:
        grouped = all(r in "BC" for r in roles[last_f + 1:])
        at = len(seq) if grouped else pos + 1
        seq[at:at] = [(None, pair[0], "B"), (None, pair[1], "C")]
    role_gap = {}
    for k, g in enumerate(gaps):
        role_gap.setdefault((roles[k], roles[k + 1]), g)
    common = max(set(gaps), key=gaps.count) if gaps else ", "
    first_gap, last_gap = sql[m.end():items[0][0]], sql[items[-1][1]:close]
    body = first_gap
    for i, (orig, d, role) in enumerate(seq):
        body += d
        if i + 1 < len(seq):
            nxt = seq[i + 1]
            body += gaps[orig] if orig is not None and nxt[0] == orig + 1 else role_gap.get((role, nxt[2]), common)
    return sql[: m.end()] + body + last_gap + sql[close:], True


def entity_stmts(tc, text, tbl, ent):
    """🧼 Statements of a codec that belong to the entity: those naming its table literal, constructing its struct, or pushing its rows."""
    out = []
    for a, b in tc.all_stmts:
        s = text[a:b]
        if f'"{tbl}"' in s or re.search(r"\b%s\s*\{" % re.escape(ent), s):
            out.append((a, b))
    return out


def leaf_codec(ctx, lang, names, tbl, fields, new, n_tables=None, cols=None):
    """🧴 In-place addition of one field to the codec statements of an existing entity: one more item in the write, read and field-name lists, the width of the entities call, the float columns."""
    L = ctx.layout
    sql = ctx.plan.read(L.p(SQLITE + "/🗄️.sql"))
    cls = RustCodec if lang == "rs" else TsCodec
    ref = names
    kwargs = (sql,) if lang == "rs" else (sql, n_tables or len(tables_of(sql)))
    tc = cls(ctx, ref, tbl, tbl, fields, [new], *kwargs)
    text = tc.text
    if re.search(r"(?<![\w])%s(?![\w])" % re.escape(new.name(lang)), "".join(text[a:b] for a, b in entity_stmts(tc, text, tbl, ref.entity))):
        return None
    edits, hits = [], 0
    for a, b in entity_stmts(tc, text, tbl, ref.entity):
        s = text[a:b]
        mask = code_mask(s, lang)
        news = s
        m = re.search(r'"%s"' % re.escape(tbl), s)
        if m and (re.search(r"insert\w*\(", s) or "write(" in s):
            f = find_list(s, mask, m.end(), len(s), fields, lang, tolerant=True)
            if f:
                idrow = s[f.items[f.first - 1][0]:f.items[f.first - 1][1]] if f.first else ""
                var = find_var(idrow, "id")
                tmpl = tc.region_templates(item_texts(s, f), f.fields, lang, None, idrow or None)
                item = tc.new_items(tmpl, lang, None, "W", var)[0]
                news = news[: f.items[f.first + f.count - 1][1]] + "," + item + news[f.items[f.first + f.count - 1][1]:]
                hits += 1
        mask = code_mask(news, lang)
        ent_m = re.search(r"\b%s\s*\{" % re.escape(ref.entity), news) if lang == "rs" else re.search(r"\.push\(\s*\{", news)
        if ent_m:
            f = find_list(news, mask, ent_m.end() - 1, len(news), fields, lang, tolerant=True)
            if f:
                idrow = news[f.items[f.first - 1][0]:f.items[f.first - 1][1]] if f.first else ""
                var = find_var(idrow, "id")
                bases = [cols[g.snake] for g in f.fields] if cols else 4
                tmpl = tc.region_templates(item_texts(news, f), f.fields, lang, bases, idrow or None)
                item = tc.new_items(tmpl, lang, cols[new.snake] if cols else 4 + len(fields), "R", var)[0]
                cut = f.items[f.first + f.count - 1][1]
                news = news[:cut] + "," + item + news[cut:]
                hits += 1
        if lang == "ts":
            fm = re.search(r"\bfields\(\s*\w+\s*,\s*\[", news)
            if fm:
                mk = code_mask(news, lang)
                f = find_list(news, mk, fm.end() - 1, match_close(news, mk, fm.end() - 1) + 1, fields, lang, tolerant=True)
                if f:
                    cut = f.items[f.first + f.count - 1][1]
                    news = news[:cut] + f',"{new.camel}"' + news[cut:]
        em = re.search(r"entities\(\s*[^,()]+,\s*(?:\"%s\"|[\w\[\]!]+)\s*,\s*(\d+)\s*," % re.escape(tbl), news)
        if em and (f'"{tbl}"' in news or lang == "ts"):
            news = news[: em.start(1)] + str(int(em.group(1)) + 1) + news[em.end(1):]
            hits += 1
        if news != s:
            edits.append((a, b, news))
    if not hits:
        raise Unsupported(f"no write/read statement of {ref.entity} found in the {lang} codec")
    for a, b, n in sorted(edits, reverse=True):
        text = text[:a] + n + text[b:]
    return tc, text


def leaf_floats(text, lang, tbl, const_hint, idx):
    """🪣 Appends a float column index to the constant the table's write statement passes (`const X:&[FloatColumn]=&[..]` in Rust, `[..].map(..)` in TypeScript)."""
    wm = re.search(r'"%s"[^;]*?\]\s*,\s*([A-Z][A-Z0-9_]*)\s*[,)]' % re.escape(tbl), text)
    if not wm:
        return text, False
    name = wm.group(1)
    if lang == "rs":
        m = re.search(r"(const %s\s*:\s*&\[FloatColumn\]\s*=\s*&\[)([^\]]*)(\])" % re.escape(name), text)
        if not m:
            return text, False
        add = f"FloatColumn::Binary64({idx})"
    else:
        m = re.search(r"(const %s\s*:\s*readonly Ieee754Column\[\]\s*=\s*\[)([^\]]*)(\])" % re.escape(name), text)
        if not m:
            return text, False
        add = str(idx)
    if re.search(r"(?<![\w.])%s(?![\w.])" % re.escape(add), m.group(2)):
        return text, False
    return text[: m.end(2)] + ("," if m.group(2).strip() else "") + add + text[m.end(2):], True


def leaf_ts_type(text, ent, new, plural):
    """🧻 The TypeScript row type of the entity gets the new member (Omit list and Binary64 member, or the explicit object member)."""
    m = re.search(r"(?<![\w$])%s\s*:\s*([A-Za-z_$][\w$]*)\[\]" % re.escape(plural), text)
    if not m:
        return text
    d = re.search(r"^export type %s\s*=([^\n]*);\n" % re.escape(m.group(1)), text, re.M)
    if not d:
        return text
    body = d.group(1)
    if new.camel in body:
        return text
    if body.strip().startswith("Omit<") and new.kind == "f64":
        nb = re.sub(r"(Omit<[^>]*?)(>&\{)", lambda mm: mm.group(1) + f'|"{new.camel}"' + mm.group(2), body, count=1)
        nb = re.sub(r"\}\s*$", f";{new.camel}:Binary64}}", nb)
    elif body.strip().startswith("{"):
        nb = re.sub(r"\}\s*$", f";{new.camel}:{ts_type_expr(new)}}}", body)
    else:
        return text
    return text[: d.start(1)] + nb + text[d.end(1):]


def sample_rows(ctx, plural):
    """🧺 Rows of the collection in the sample document of the contract cases: {case id or '*': rows}; the sample is the `snapshot` of the committed SQLite fixture; `empty*` cases clear the collections."""
    import json
    base = ctx.layout.p(SQLITE + "/🧪️tests/🧫️fixtures")
    path = base + "/🔣️.json"
    if not ctx.plan.exists(path):
        return None
    try:
        doc = json.loads(ctx.plan.read(path))
    except ValueError:
        return None
    snap = doc.get("snapshot") if isinstance(doc, dict) else None
    rows = snap.get(plural) if isinstance(snap, dict) else None
    if not isinstance(rows, list):
        return None
    semantic = semantic_path(ctx)
    cases = json.loads(ctx.plan.read(semantic)).get("cases", []) if ctx.plan.exists(semantic) else []
    return {c["id"]: (0 if c["id"].lower().startswith("empty") else len(rows)) for c in cases}


def leaf_value_bytes(f):
    """🧹 Bytes the new cells of one row add to the logical extent: REAL and integers 8 each, the ieee754 class `finite` 6, text its UTF-8 length."""
    if f.ty == "f64":
        return 8 + 8 + len("finite")
    if f.ty == "String":
        return len(str(f.sample or "").encode())
    if f.ty == "enum":
        return len(str(f.sample or f.tokens[0]).encode())
    return 8


def leaf_array_only(ctx, names, tbl, new, f, idx, sql, new_sql, n_tables):
    """🕳️ Leaf on an entity without scalar cells (see `l_arrayonly`): the Rust, TypeScript and admission statements are converted to the float-aware forms."""
    import l_arrayonly
    if new.kind != "f64":
        raise Unsupported("a leaf on an array-only entity must be f64 (the id is its only other cell)")
    L = ctx.layout
    rs_path, ts_path, adm_path = L.p(SQLITE + "/🦀️.rs"), L.p(SQLITE + "/🟦️.ts"), L.p(SQLITE + "/🛂️admission/🦀️.rs")
    rs_new = l_arrayonly.rust(ctx, names, tbl, new, idx, ctx.plan.read(rs_path), rs_path)
    ctx.plan.write(rs_path, rs_new)
    ctx.plan.write(ts_path, l_arrayonly.typescript(ctx, names, tbl, new, idx, ctx.plan.read(ts_path), n_tables))
    if ctx.plan.exists(adm_path):
        ctx.plan.write(adm_path, l_arrayonly.admission(ctx, names, tbl, new, f, ctx.plan.read(adm_path), l_arrayonly.const_name(rs_new, names.entity, "rs")))
    contract(ctx, sql, new_sql, {tbl: (sample_rows(ctx, ctx.leaf_plural) or {"*": None}, leaf_value_bytes(f))})
    ctx.plan.note(f"sqlite leaf: column {new.snake} added to the array-only table {tbl} (write converted to insert_float, read width and float list, admission cell and shifted child field ids)")


def leaf_sqlite_generic(ctx):
    S = ctx.spec
    f = S.leaf
    names = Names(ctx.leaf_plural, ctx.leaf_entity)
    L = ctx.layout
    sql_path = L.p(SQLITE + "/🗄️.sql")
    sql = ctx.plan.read(sql_path)
    tbl = table_of(ctx, names, sql)
    fields = schema_fields(ctx, ctx.leaf_entity, scalars=True)
    array_only = not fields and any(node.get("type") == "array" for node in ctx.defs[ctx.leaf_entity]["properties"].values())
    if not array_only and (not fields or any(g.kind is None for g in fields)):
        raise Unsupported(f"{ctx.leaf_entity} has fields that are no cells (objects or flattened values); its codec statements cannot be extended generically")
    new = F(f.camel, f.ty, f.enum_name if f.ty == "enum" else None, tokens=f.tokens if f.ty == "enum" else None)
    n_tables = len(tables_of(sql))
    new_sql, changed = leaf_ddl(ctx, sql, tbl, new)
    if not changed:
        return
    ctx.plan.write(sql_path, new_sql)
    _, _, items = ddl_body(new_sql, tbl)
    cols = {}
    for k, (a, b) in enumerate(items):
        cols[def_name(new_sql[a:b])] = k
        cols.setdefault(re.sub(r"_(?:reference|ref)$", "_id", def_name(new_sql[a:b])), k)
    names_ = [def_name(new_sql[a:b]) for a, b in items if not re.search(r"_ieee754_(?:bits|class)$", def_name(new_sql[a:b]))]
    for k, g in enumerate(fields):
        if g.snake not in cols:
            near = [n for n in names_ if n.endswith("_" + g.snake) or n.startswith(g.snake + "_")]
            cols[g.snake] = cols[near[0]] if len(near) == 1 else 4 + k
    idx = cols[new.snake]
    if array_only:
        return leaf_array_only(ctx, names, tbl, new, f, idx, sql, new_sql, n_tables)
    rs = leaf_codec(ctx, "rs", names, tbl, fields, new, cols=cols)
    ts = leaf_codec(ctx, "ts", names, tbl, fields, new, n_tables, cols=cols)
    if rs:
        tc, text = rs
        if new.kind == "f64":
            text, _ = leaf_floats(text, "rs", tbl, None, idx)
        if new.kind == "enum":
            w, r = helper_names(new.enum_def, tc.dialect)
            if not re.search(r"\bfn\s+%s\b" % re.escape(w), text):
                reads = list(re.finditer(r"^fn read_\w+\([^\n]*\n", text, re.M))
                at = reads[-1].end() if reads else text.find("\n", text.find("fn invalid")) + 1
                text = text[:at] + enum_fn_text(new, tc.dialect) + text[at:]
            text = ensure_rust_use(text, [new.enum_def])
        ctx.plan.write(tc.path, text)
    if ts:
        tc, text = ts
        if new.kind == "f64":
            text, _ = leaf_floats(text, "ts", tbl, None, idx)
        if new.kind == "enum":
            if not re.search(r"\bconst \w+\s*:\s*readonly %s\[\]" % re.escape(new.enum_def), text) and re.search(r"\b%sS\b" % snake(new.enum_def).upper(), text):
                kinds = list(re.finditer(r"^const \w+S:readonly \w+\[\]=\[[^\n]*\n", text, re.M))
                if not kinds:
                    raise Unsupported("no enum choice constants in the TypeScript codec")
                line = f"const {snake(new.enum_def).upper()}S:readonly {new.enum_def}[]=[{','.join(chr(34) + x + chr(34) for x in new.tokens)}];\n"
                text = text[: kinds[-1].end()] + line + text[kinds[-1].end():]
            text = re.sub(r"(import type\s*\{)([^}]*\})", lambda m_: m_.group(0) if re.search(r"\b%s\b" % re.escape(new.enum_def), m_.group(2)) else m_.group(1) + m_.group(2)[:-1].rstrip() + "," + new.enum_def + "}", text, count=1)
        text = leaf_ts_type(text, names.entity, new, names.plural_camel)
        ctx.plan.write(tc.path, text)
    adm = L.p(SQLITE + "/🛂️admission/🦀️.rs")
    if ctx.plan.exists(adm):
        import l_sqlite
        at = ctx.plan.read(adm)
        m = re.search(r'n::entity\(out,"%s",[^\n]*?\[(Text\(0\)[^\]]*)\]' % re.escape(tbl), at)
        if m:
            count = m.group(1).count("(") - m.group(1).count("u32::MAX") * 0
            count = len(split_items(m.group(1), [True] * len(m.group(1)), 0, len(m.group(1))))
            cell = l_sqlite.adm_cell(f, count)
            ctx.plan.write(adm, at[: m.end(1)] + "," + cell + at[m.end(1):])
    contract(ctx, sql, new_sql, {tbl: (sample_rows(ctx, ctx.leaf_plural) or {"*": None}, leaf_value_bytes(f))})
    ctx.plan.note(f"sqlite leaf: column {new.snake} added to {tbl} in the DDL, Rust and TypeScript codecs and the admission cells")
