"""🍃 Leaf layer: one typed field is added to an existing row entity through schema JSON, diff patch, model literal, SQLite codecs (appended behind the last scalar column, so no stored index moves), field metadata, fixtures and DSL assets."""
import json, os, re
import jsonspan as js
from core import DIFF, SCHEMA, SNAPSHOT, Names, Unsupported, camel, pascal, snake, sub1, walk_files, balanced_end
from ctx import append_member, append_item, dumps, get, parse, span_edit
import l_sqlite
from spec import INT_MAX, need_labels


def target(ctx):
    plural = ctx.entity_path
    hit = [c for c in ctx.collections if c[0] == plural]
    if not hit:
        raise Unsupported(f"entity path {plural!r} is not a collection of {ctx.short} (collections: {', '.join(c[0] for c in ctx.collections)})")
    ent = hit[0][1]
    props = ctx.defs[ent]["properties"]
    scalars = [k for k, v in props.items() if v.get("type") != "array"]
    ctx.leaf_entity, ctx.leaf_plural = ent, plural
    enum_prepare(ctx)
    ctx.leaf_present = ctx.spec.leaf.camel in props
    ctx.leaf_after = ctx.spec.leaf_after or scalars[-1]
    if ctx.leaf_present:
        return ent
    if ctx.leaf_after != scalars[-1]:
        raise Unsupported(f"`after` must be the last scalar field of {ent} ({scalars[-1]}) so no positional SQLite index moves")
    if ctx.spec.leaf.camel in props:
        ctx.leaf_present = True
    ctx.leaf_present = ctx.spec.leaf.camel in props
    return ent


def enum_prepare(ctx):
    """🧲 A typed choice field: an enum of the snapshot schema is reused with its tokens, labels and Rust variant names, any other name needs its values in the spec and is created by the layers."""
    f = ctx.spec.leaf
    if f.ty != "enum" or hasattr(ctx, "enum_new"):
        return
    existing = ctx.defs.get(f.enum_name)
    ctx.enum_new = not (existing and "enum" in existing)
    if ctx.enum_new:
        if not f.values:
            raise Unsupported(f"enum {f.enum_name} is new: the spec must list its values (token and EN + DE label)")
        return
    model = ctx.plan.read(ctx.layout.p(SNAPSHOT + "/🦀️.rs"))
    m = re.search(r"^pub enum %s \{\n((?:(?:    #\[[^\n]*\]\n)*    \w+,\n)+)\}" % re.escape(f.enum_name), model, re.M)
    variants = re.findall(r"^    (\w+),$", m.group(1), re.M) if m else None
    if variants and len(variants) != len(existing["enum"]):
        variants = None
    f.adopt(existing, variants)
    if "default" in ctx.spec.raw["leaf"]:
        ctx.spec.leaf_default = ctx.spec.raw["leaf"]["default"]
    else:
        ctx.spec.leaf_default = f.sample


def insert_after_member(text, obj, after, key, value):
    if obj.get(key):
        return text
    return js.insert_member(text, obj, after, key, dumps(value, text))


def insert_after_item(text, arr, after_value, value):
    names = [json.loads(text[i.start:i.end]) for i in arr.items]
    if value in names:
        return text
    return js.insert_item(text, arr, names.index(after_value), dumps(value, text))


def schema(ctx):
    L, f, ent, after = ctx.layout, ctx.spec.leaf, ctx.leaf_entity, ctx.leaf_after

    def doc(node_fn):
        def go(text):
            root = parse(text)
            rec = get(root, "$defs").get(ent).value
            props = rec.get("properties").value
            if props.get(f.camel):
                return text
            text = insert_after_member(text, props, after, f.camel, node_fn())
            rec = get(parse(text), "$defs").get(ent).value
            text = insert_after_item(text, rec.get("required").value, after, f.camel)
            if f.ty == "enum" and ctx.enum_new:
                text = append_member(text, get(parse(text), "$defs"), f.enum_name, f.enum_def())
            return text
        return go
    span_edit(ctx.plan, L.p(SNAPSHOT + "/🔣️.json"), doc(f.snapshot_node))
    span_edit(ctx.plan, L.p(SCHEMA + "/🔣️.json"), doc(f.artifact_node))
    base = ctx.snapshot["$id"]
    import l_schema

    def diff(text):
        root = parse(text)
        defs = get(root, "$defs")
        rx = l_schema.family_regex(ctx, Names(ctx.leaf_plural, ent))
        key = next((k.key for k in defs.members if rx.match(k.key) and k.key.endswith("Patch")), None)
        if not key:
            ctx.plan.note(f"diff: {ent} has no Patch definition in the diff schema; nothing to extend")
            ctx.leaf_has_patch = False
            return text
        ctx.leaf_has_patch = True
        patch = defs.get(key).value
        props = patch.get("properties").value
        if props.get(f.camel):
            return text
        first = json.loads(text[props.members[0].value.start:props.members[0].value.end]) if props.members else None
        blob = json.dumps(first) if first else json.dumps({"anyOf": [{"$ref": f"{base}#/$defs/{ent}/properties/{f.camel}"}, {"type": "null"}]})
        node = json.loads(re.sub(r"/\$defs/\w+/properties/\w+", f"/$defs/{ent}/properties/{f.camel}", blob))
        if "x-semio-rust" in node:
            node["x-semio-rust"] = {"type": f"{ctx.layout.model_crate}::{f.enum_name}" if f.ty == "enum" else f.rust_ty}
        if props.get(after):
            text = insert_after_member(text, props, after, f.camel, node)
        else:
            text = append_member(text, props, f.camel, node)
        patch = get(parse(text), "$defs").get(key).value
        req = patch.get("required")
        if not req:
            return text
        listed = [json.loads(text[i.start:i.end]) for i in req.value.items]
        if f.camel in listed:
            return text
        if after in listed:
            return insert_after_item(text, req.value, after, f.camel)
        return js.insert_item(text, req.value, len(listed) - 1, dumps(f.camel, text))
    span_edit(ctx.plan, L.p(DIFF + "/🔣️.json"), diff)
    ctx.plan.note(f"schema: {ent}.{f.camel} after {after} in snapshot, artifact and diff Patch")


def extend_patch_impls(text, struct, name):
    """📩 Adds the scalar leaf `name` to the hand-written `RowPatch` impl of `struct` (`commit_into`, `absorb`, `inverse`, `is_empty`) behind the last scalar line of each method; blocks that already mention it stay untouched."""
    head = re.search(r"^impl [^\n]*RowPatch<[^\n]*> for %s \{\n" % re.escape(struct), text, re.M)
    if not head:
        return text
    end = text.index("\n}\n", head.end())
    block = text[head.end():end]
    if f"self.{name}" in block:
        return text

    def after_last(block, pattern, line):
        hits = list(re.finditer(pattern, block, re.M))
        if not hits:
            raise Unsupported(f"RowPatch impl of {struct}: no scalar line to anchor `{line.strip()}`")
        at = hits[-1].end()
        return block[:at] + "\n" + line + block[at:]

    block = after_last(block, r"^( +)if let Some\(value\) = &self\.\w+ \{ row\.[\w.]+ = value\.clone\(\); \}$", f"        if let Some(value) = &self.{name} {{ row.{name} = value.clone(); }}")
    block = after_last(block, r"^ +if later\.\w+\.is_some\(\) \{ self\.\w+ = later\.\w+; \}$", f"        if later.{name}.is_some() {{ self.{name} = later.{name}; }}")
    block = after_last(block, r"^ +\w+: self\.\w+\.as_ref\(\)\.map\(\|_\| row\.[\w.]+\.clone\(\)\),$", f"            {name}: self.{name}.as_ref().map(|_| row.{name}.clone()),")
    empty = re.search(r"^( +self\.\w+\.is_none\(\)[^\n]*)$", block, re.M)
    if not empty:
        raise Unsupported(f"RowPatch impl of {struct}: no is_empty chain")
    block = block[:empty.end()] + f" && self.{name}.is_none()" + block[empty.end():]
    return text[:head.end()] + block + text[end:]


def diff_rs(ctx):
    """📩 The typed field list of the entity's Patch: `Patch of Model::E { set { .. } }`, `Patch for Model::E { .. }` (scalar patch) or a plain `pub struct XPatch { pub f: Option<T>, .. }`."""
    f, after = ctx.spec.leaf, ctx.leaf_after
    rust_ty = f"{ctx.layout.model_crate}::{f.enum_name}" if f.ty == "enum" else f.rust_ty
    path = ctx.layout.p(DIFF + "/🦀️.rs")
    ent = ctx.leaf_entity
    mount = [p for p in __import__("l_rust").diff_tree_files(ctx)]
    done = False
    for p in [path] + [x for x in mount if x != path]:
        text = ctx.plan.read(p)
        t = re.escape(ent)
        new = None
        m = re.search(r"(Patch of [\w:]*%s \{ set \{)([^}]*)(\})" % t, text) or re.search(r"(Patch for [\w:]*%s \{)([^}]*)(\})" % t, text)
        if m:
            if re.search(r"(?<![\w])%s: " % f.name, m.group(2)):
                return
            new = text[: m.end(2)] + f", {f.name}: {rust_ty}" + text[m.end(2):] if m.group(2).strip() else text[: m.end(2)] + f" {f.name}: {rust_ty} " + text[m.end(2):]
        else:
            m = re.search(r"pub struct (?:En\d{4})?%sPatch \{\n((?:\s+pub \w+: Option<[^\n]*,\n)+)\s*\}" % re.escape(Names(ctx.leaf_plural, ent).plural_pascal), text) or re.search(r"pub struct \w*%sPatch \{\n((?:\s+pub \w+: Option<[^\n]*,\n)+)\s*\}" % re.escape(re.sub(r"^En\d{4}", "", ent)), text)
            if m:
                if re.search(r"pub %s: " % f.name, m.group(1)):
                    return
                indent = re.match(r"\s+", m.group(1)).group(0).replace("\n", "")
                new = text[: m.end(1)] + f"{indent}pub {f.name}: Option<{rust_ty}>,\n" + text[m.end(1):]
                new = extend_patch_impls(new, re.search(r"pub struct (\w+)", m.group(0)).group(1), f.name)
        if new:
            ctx.plan.write(p, new)
            done = True
            break
    if not done and getattr(ctx, "leaf_has_patch", True):
        raise Unsupported("Patch declaration of " + ent + " not found in the diff Rust")


def model_enum(ctx):
    """🧬 A new enum is cloned from a donor enum of the model (derives, attributes, `impl` arms regenerated)."""
    import l_rust
    f = ctx.spec.leaf
    if f.ty != "enum" or not ctx.enum_new:
        return
    path = ctx.layout.p(SNAPSHOT + "/🦀️.rs")
    text = ctx.plan.read(path)
    if re.search(r"pub enum %s \{" % f.enum_name, text):
        return
    out, end = l_rust.clone_enum(ctx, text, f)
    ctx.plan.write(path, text[:end] + "\n" + out + text[end:])
    ctx.plan.note(f"model: enum {f.enum_name} ({', '.join(f.tokens)}) cloned from a donor enum")


def literal(ctx):
    """📨 Every struct literal of the entity in the snapshot model (example data) gets the new field behind the field it follows."""
    import l_rust
    from lex import code_mask, match_close, split_items
    f, after = ctx.spec.leaf, snake(ctx.leaf_after)
    path = ctx.layout.p(SNAPSHOT + "/🦀️.rs")
    text = ctx.plan.read(path)
    val = {"f64": lambda: repr(float(ctx.spec.leaf_default)), "String": lambda: json.dumps(ctx.spec.leaf_default) + ".into()"}.get(f.ty, lambda: str(ctx.spec.leaf_default))()
    if f.ty == "enum":
        val = f"{f.enum_name}::{f.variants[0]}"
    mask = code_mask(text, "rs")
    edits = []
    for m in re.finditer(r"\b%s\s*\{" % re.escape(ctx.leaf_entity), text):
        if not mask[m.start()] or re.match(r"\s*(?:pub\s+)?struct", text[max(0, m.start() - 12):m.start()]) or text[max(0, m.start() - 8):m.start()].strip().endswith(("struct", "enum", "impl", "for")):
            continue
        close = match_close(text, mask, m.end() - 1)
        items = split_items(text, mask, m.end(), close)
        keys = [re.match(r"\s*(\w+)\s*:", text[a:b]) for a, b in items]
        if any(k and k.group(1) == f.name for k in keys):
            continue
        hit = next((i for i, k in enumerate(keys) if k and k.group(1) == after), None)
        if hit is None:
            continue
        a, b = items[hit]
        multiline = "\n" in text[a:close + 1]
        sep = "\n" + re.match(r" *", text[text.rfind("\n", 0, a) + 1:]).group(0) if multiline and text[a - 1] != "{" and "\n" in text[text.rfind(",", 0, a):a] else " "
        edits.append((b + (1 if text[b:b + 1] == "," else 0), (sep if text[b:b + 1] == "," else "") + (f"{f.name}: {val}," if text[b:b + 1] == "," else f", {f.name}: {val}")))
    for at, add in sorted(edits, reverse=True):
        text = text[:at] + add + text[at:]
    ctx.plan.write(path, text)


def sqlite_en1997(ctx):
    L, f, after, short = ctx.layout, ctx.spec.leaf, snake(ctx.leaf_after), ctx.short
    table = f"{short}_{snake(ctx.leaf_entity)}"
    base = L.p(l_sqlite.SQLITE)
    if not f.is_real:
        raise Unsupported("sqlite leaf layer supports f64 fields (the EN + DE codecs of other types need the add-collection templates)")

    def sql(text):
        block = re.search(r"CREATE TABLE %s \(.*?\n\);" % table, text, re.S)
        if not block:
            raise Unsupported(f"table {table} not found in the DDL")
        b = block.group(0)
        if re.search(r"\b%s REAL" % f.name, b):
            return text
        b = sub1(b, r"(\b%s REAL,)" % after, lambda m: m.group(1) + f" {f.name} REAL,", what=f"{after} REAL in {table}")
        b = sub1(b, r"(%s_ieee754_class TEXT NOT NULL)(\n\);)" % after, lambda m: m.group(1) + f",\n {f.name}_ieee754_bits INTEGER NOT NULL, {f.name}_ieee754_class TEXT NOT NULL" + m.group(2), what="ieee columns")
        return text.replace(block.group(0), b)
    old_sql = ctx.plan.read(base + "/🗄️.sql")
    span_edit(ctx.plan, base + "/🗄️.sql", sql)
    rs = ctx.plan.read(base + "/🦀️.rs")
    columns = int(re.search(r'entities\(d,"%s",(\d+),' % table, rs).group(1))
    const = re.search(r'entities\(d,"%s",\d+,(\w+),' % table, rs).group(1)
    def rust(t):
        if re.search(r"\b%s:\w\.real\(" % f.name, t):
            return t
        t = sub1(t, r"(Cell::Real\((\w)\.%s\))(\],%s\))" % (after, const), lambda m: m.group(1) + f",Cell::Real({m.group(2)}.{f.name})" + m.group(3), what="write cell")
        t = sub1(t, r"(const %s:&\[FloatColumn\]=&\[[^\]]*)\];" % const, lambda m: m.group(1) + f",FloatColumn::Binary64({columns})];", what="float columns")
        t = sub1(t, r'(entities\(d,"%s",)%d,' % (table, columns), lambda m: m.group(1) + f"{columns + 1},", what="entity width")
        return sub1(t, r"(%s:(\w)\.real\(%d\)\?)" % (after, columns - 1), lambda m: m.group(1) + f",{f.name}:{m.group(2)}.real({columns})?", what="read cell")
    span_edit(ctx.plan, base + "/🦀️.rs", rust)
    def ts(t):
        if re.search(r"\b%s:readBinary64\(" % f.camel, t):
            return t
        t = sub1(t, r"(export type \w+=Omit<\w+,(?:\"\w+\"\|)*\"%s\")(>&\{[^}]*\b%s:Binary64)" % (camel(after), camel(after)), lambda m: m.group(1) + f'|"{f.camel}"' + m.group(2) + f";{f.camel}:Binary64", what="TS row type")
        t = sub1(t, r"(const %s:readonly Ieee754Column\[\]=\[[0-9,]*?)\]\.map" % const, lambda m: m.group(1) + f",{columns}].map", what="TS float columns")
        t = sub1(t, r'(fields\(\w,\[[^\]]*"%s")\]\);(await write\(out,"%s")' % (camel(after), table), lambda m: m.group(1) + f',"{f.camel}"]);' + m.group(2), what="TS fields list")
        t = sub1(t, r"(,\w\.%s)(\],%s,o\))" % (camel(after), const), lambda m: m.group(1) + f",{m.group(1)[1]}.{f.camel}" + m.group(2), what="TS write cells")
        t = sub1(t, r"(entities\(t\[\d+\]!,)%d,%s," % (columns, const), lambda m: m.group(1) + f"{columns + 1},{const},", what="TS entity width")
        return sub1(t, r"(%s:readBinary64\((\w),%d,%s\))" % (camel(after), columns - 1, const), lambda m: m.group(1) + f",{f.camel}:readBinary64({m.group(2)},{columns},{const})", what="TS read")
    span_edit(ctx.plan, base + "/🟦️.ts", ts)
    def adm(t):
        m = re.search(r'^[^\n]*n::entity\(out,"%s",[^\n]*$' % table, t, re.M)
        line = m.group(0)
        cells = re.search(r"\[(Text\(0\)[^\]]*)\]\)\?,%s\)" % const, line)
        count = cells.group(1).count("(")
        if count > len([k for k, v in ctx.defs[ctx.leaf_entity]["properties"].items() if v.get("type") != "array"]):
            return t
        grown = line.replace(cells.group(1), cells.group(1) + f",Real({count})", 1)
        return t.replace(line, grown, 1)
    span_edit(ctx.plan, base + "/🛂️admission/🦀️.rs", adm)
    contract(ctx, old_sql, ctx.plan.read(base + "/🗄️.sql"))


def sqlite(ctx):
    saved = dict(ctx.plan.files)
    try:
        sqlite_en1997(ctx)
    except (Unsupported, AttributeError, IndexError, TypeError):
        ctx.plan.files.clear()
        ctx.plan.files.update(saved)
        import l_codec
        l_codec.leaf_sqlite_generic(ctx)


def contract(ctx, old_sql, new_sql):
    import l_codec
    table = f"{ctx.short}_{snake(ctx.leaf_entity)}"
    tbl = next((t for t in l_codec.tables_of(new_sql) if t in (table, snake(ctx.leaf_entity), f"{ctx.short}_{snake(ctx.leaf_plural[:-1])}")), table)
    l_codec.contract(ctx, old_sql, new_sql, {tbl: (l_codec.sample_rows(ctx, ctx.leaf_plural) or {"*": None}, l_codec.leaf_value_bytes(ctx.spec.leaf))})


def meta(ctx):
    import l_wiring as W
    f, after = ctx.spec.leaf, camel(ctx.leaf_after)
    path = W.meta_module(ctx, ctx.leaf_plural)
    if not path:
        ctx.plan.note(f"field-meta: {ctx.leaf_plural} has no field-meta rows in this artifact; nothing to extend")
        return
    text = ctx.plan.read(path)
    key = f"{ctx.leaf_plural}[].{f.camel}"
    if re.search(r'"%s"' % re.escape(key), text):
        return
    rows_ = W.parse_rows(text)
    anchor = [r for r in rows_ if r[2].group("key") == f"{ctx.leaf_plural}[].{after}"] or [r for r in rows_ if r[2].group("key").startswith(ctx.leaf_plural + "[]")]
    if not anchor:
        raise Unsupported("field-meta rows of " + ctx.leaf_plural + " not found")
    kind = "choice" if f.ty == "enum" else "quantity" if W.unit_of(f) else "plain"
    const = f.enum_const if f.ty == "enum" else None
    if f.ty == "enum":
        owned = next((m.group(1) for m in re.finditer(r"^\s*const (\w+): &\[NormFieldChoice\] = &\[(.*?)\];", text, re.M | re.S) if [v for v in re.findall(r'(?:choice\(|value: )"([^"]*)"', m.group(2))] == f.tokens), None)
        if owned:
            const = owned
        else:
            text = W.choice_const(text, f)
        rows_ = W.parse_rows(text)
        anchor = [r for r in rows_ if r[2].group("key") == f"{ctx.leaf_plural}[].{after}"] or [r for r in rows_ if r[2].group("key").startswith(ctx.leaf_plural + "[]")]
    block = W.meta_rows_for(ctx, text, ctx.leaf_plural, [(key, (f.en, f.de), kind, W.unit_of(f), const)], ctx.leaf_plural, {const: f} if f.ty == "enum" else None)
    at = anchor[-1][1] + 1
    ctx.plan.write(path, text[:at] + block + text[at:])


def fixtures(ctx):
    f, after = ctx.spec.leaf, ctx.leaf_after
    ent = ctx.defs[ctx.leaf_entity]
    row_keys = set(ent["properties"])
    patch_keys = row_keys - {"id"}
    value = dumps(ctx.spec.leaf_default, "x: y")
    count = 0
    for path in walk_files(ctx.layout.any, (".json",)):
        rel = path[len(ctx.layout.any) + 1:]
        if rel.startswith("🧬️schema/") and rel.count("/") <= 2 or f'"{after}"' not in ctx.plan.read(path) and f'"{camel(after)}"' not in ctx.plan.read(path):
            continue
        text = ctx.plan.read(path)
        try:
            root = parse(text)
        except (AssertionError, IndexError, ValueError):
            continue
        a = camel(after)
        edits = []
        for _, obj in js.find_objects(root, lambda o: o.get(a) is not None and o.get(f.camel) is None and o.get("properties") is None and {m.key for m in o.members} <= row_keys | {"id"} and ("id" in {m.key for m in o.members} if "id" in row_keys else {m.key for m in o.members} == row_keys - {f.camel}) and not (len(o.members) == len(patch_keys) and {m.key for m in o.members} == patch_keys)):
            m = obj.get(a)
            edits.append((m.value.end, js._separator(text, obj, obj.members.index(m)) + json.dumps(f.camel) + text[m.key_end:m.value.start] + value))
        for _, obj in js.find_objects(root, lambda o: o.get(a) is not None and o.get(f.camel) is None and "id" in row_keys and {m.key for m in o.members} == patch_keys):
            m = obj.get(a)
            edits.append((m.value.end, js._separator(text, obj, obj.members.index(m)) + json.dumps(f.camel) + text[m.key_end:m.value.start] + "null"))
        if edits:
            ctx.plan.write(path, js.apply_edits(text, edits))
            count += 1
    ctx.plan.note(f"fixtures: {count} JSON files get `{f.camel}`")


def assets(ctx):
    f, after = ctx.spec.leaf, ctx.leaf_after
    kebab, akebab = ctx.leaf_plural and re.sub(r"([A-Z])", lambda m: "-" + m.group(1).lower(), ctx.leaf_plural), re.sub(r"([A-Z])", lambda m: "-" + m.group(1).lower(), after)
    ty = "QTY" if f.unit in ("m", "m2", "Pa", "N", "deg") else "NUM" if f.is_real else "TEXT"
    v = ctx.spec.leaf_default
    token = ("%g" % v if f.is_real else str(v)) + (f.unit if ty == "QTY" else "")
    for path in walk_files(ctx.layout.any, (".dsl.semio",)):
        text = ctx.plan.read(path)
        h = re.search(r"^%s \[([^\n]*)\] \{$" % re.escape(kebab), text, re.M)
        if not h or re.search(r"\b%s:" % re.escape(f.name.replace("_", "-")), h.group(1)):
            continue
        cols = h.group(1).split(" ")
        names = [c.split(":")[0] for c in cols]
        if akebab not in names:
            continue
        pos = names.index(akebab)
        cols.insert(pos + 1, f"{f.name.replace('_', '-')}:{ty}")
        end = text.index("\n}", h.end())
        rows = []
        for row in text[h.end():end].split("\n"):
            if not row.strip():
                rows.append(row)
                continue
            indent = re.match(r"\s*", row).group(0)
            body, table = (row[len(indent):].split(" [", 1) + [""])[:2] if " [" in row else (row[len(indent):], "")
            toks = body.split(" ")
            toks.insert(pos + 1, token)
            rows.append(indent + " ".join(toks) + (" [" + table if table else ""))
        ctx.plan.write(path, text[:h.start()] + f"{kebab} [{' '.join(cols)}] {{" + "\n".join(rows) + text[end:])


def schema_sample_row(ctx, entity):
    """📧 A valid default row of an existing entity (ids, zero numbers, first enum token, empty text and lists) for fixtures that need a row of a collection the sample documents leave empty."""
    row = {"id": re.sub(r"(?<!^)([A-Z])", r"-\1", entity).lower() + "-1"}
    for name, node in ctx.defs[entity]["properties"].items():
        if name == "id":
            continue
        if node.get("type") == "array":
            row[name] = []
        elif "$ref" in node and "enum" in ctx.defs.get(node["$ref"].rsplit("/", 1)[1], {}):
            row[name] = ctx.defs[node["$ref"].rsplit("/", 1)[1]]["enum"][0]
        elif "enum" in node:
            row[name] = node["enum"][0]
        elif node.get("type") == "number":
            row[name] = 0.0
        elif node.get("type") == "integer":
            row[name] = 0
        elif node.get("type") == "boolean":
            row[name] = False
        elif node.get("type") == "string":
            row[name] = ""
        else:
            return None
    return row


def known_singular(ctx):
    """🏷️ The EN + DE label an existing kind of the entity gives it (the `id` selector of a set-field kind addressing the collection)."""
    import l_setfield
    from l_mutations import find_dir, tree_files
    for kind in ("f64", "String", "u8", "u32", "enum"):
        for plural, field, slug, _ in l_setfield.candidates(ctx, kind):
            if plural != ctx.leaf_plural:
                continue
            try:
                src, _ = find_dir(ctx, slug)
            except Unsupported:
                continue
            for f in tree_files(src):
                if f.endswith("🧬️schema/🔣️.json"):
                    label = json.loads(open(f, encoding="utf8").read()).get("properties", {}).get("id", {}).get("x-semio-ui", {}).get("label")
                    if label:
                        return dict(label)
    return None


class LeafEntity:
    """📥 Singular labels and sample row of an existing entity, for the set-field kind of a leaf."""

    def __init__(self, ctx):
        raw = ctx.spec.raw["leaf"].get("entity", {})
        words = re.sub(r"^En\d{4}", "", ctx.leaf_entity)
        words = re.sub(r"(?<!^)([A-Z])", r" \1", words).lower()
        self.entity = ctx.leaf_entity
        self.names = Names(ctx.leaf_plural, ctx.leaf_entity)
        singular = raw.get("singular") or known_singular(ctx)
        if not singular:
            raise Unsupported(f"{ctx.leaf_entity}: no EN + DE singular label (give leaf.entity.singular in the spec; no existing kind of the entity carries one)")
        need_labels(singular, f"leaf entity {ctx.leaf_entity} singular")
        self.singular_en, self.singular_de = singular["en"], singular["de"]
        self.fields = []
        self.sample = schema_sample_row(ctx, ctx.leaf_entity)

    def sample_row(self):
        return self.sample


def mutations(ctx):
    slug = ctx.spec.raw["leaf"].get("kindSlug") or re.sub(r"^en\d{4}-", "", re.sub(r"(?<!^)([A-Z])", r"-\1", ctx.leaf_entity).lower())
    import l_mutations
    kind = f"change-{slug}-{ctx.spec.leaf.name.replace('_', '-')}"
    if getattr(ctx, "leaf_present", False) and any(d.endswith(kind) for d in os.listdir(ctx.layout.p(l_mutations.MUT))):
        return
    if ctx.short in l_mutations.DRIVERS:
        ctx.plan.note(f"set-field kind change-{slug}-{ctx.spec.leaf.name.replace('_', '-')} by the leaf-kinds driver after the twins exist")
        ctx.kinds = {"clone": [], "set": [f"change-{slug}-{ctx.spec.leaf.name.replace('_', '-')}"]}
        return
    import l_setfield
    made = l_setfield.clone_setfield(ctx, LeafEntity(ctx), ctx.leaf_plural, ctx.spec.leaf, slug)
    ctx.kinds = {"clone": [], "set": []}
    import l_text
    l_text.run(ctx, None)
    ctx.plan.note(f"set-field kind {made or 'present already'} by the clone driver {l_mutations.driver_of(ctx.short)} from an existing kind of the artifact")


def layers():
    wrapped = []
    for name, fn in (("schema", schema), ("diff", lambda c: (diff_rs(c), __import__("l_patch").run(c))), ("model", lambda c: (model_enum(c), literal(c))), ("sqlite", sqlite), ("wiring", meta), ("fixtures", fixtures), ("assets", assets), ("mutations", mutations)):
        always = name == "mutations"
        wrapped.append((name, (lambda fn, always: lambda c: (target(c), None if getattr(c, "leaf_present", False) and not always else fn(c)))(fn, always)))
    return wrapped
