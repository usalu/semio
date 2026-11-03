"""🔌 Wiring layer: outline, incremental units, duplicate-id integrity, EN + DE field metadata with its choice lists, and the optional scene kit row. Every site is found by pattern: the reference sibling collection is the model, and when the reference is not registered in a registry, the last registered collection of the schema order is."""
import re
from core import SCHEMA, Names, Unsupported, balanced_end, clone_after, line_span, pascal, snake, sub1, walk_files
from l_rust import clone_mentions, item_end, split_top


def registered(ctx, text, names):
    """🫒 True when a registry in `text` lists the collection: a quoted name or a field access."""
    return bool(re.search(r'"(?:%s|%s)"|\.%s\b' % (names.plural_snake, names.plural_camel, names.plural_snake), text))


def source_for(ctx, text):
    """🫑 The collection to clone from: the reference when it is registered in `text`, otherwise the last registered collection."""
    cands = [ctx.ref] + [Names(p, e) for p, e in reversed(ctx.collections) if p != ctx.ref.plural_camel]
    own = ctx.spec.collection.names
    for names in cands:
        if names.plural_camel != own.plural_camel and registered(ctx, text, names):
            return names
    return None


def item_around(text, pos):
    """🫐 Span (start, end) of the list or chain item that contains `pos`: bounded by commas, `+`, `|` or brackets at the item's own depth."""
    depth, i = 0, pos
    while i > 0:
        ch = text[i - 1]
        if ch in ")]}":
            depth += 1
        elif ch in "([{":
            if depth == 0:
                break
            depth -= 1
        elif depth == 0 and ch in ",+|":
            break
        i -= 1
    start = i
    end = item_end(text, pos)
    seg = text[pos:end]
    m = re.search(r"\s[+|]\s", seg)
    if m and not text[start:pos].strip().endswith(("(",)):
        end = pos + m.start()
    return start, end


def clone_registry_items(ctx, text, src):
    """🫅 Clones every list/chain item and statement that registers `src` (quoted names, `snapshot.src.len()` chain items, `.src` statements) for the new collection."""
    n = ctx.spec.collection.names
    if registered(ctx, text, n):
        return text
    fields = [camel_or_snake for f in ctx.defs[src.entity]["properties"] if f != "id" for camel_or_snake in (f, snake(f))]
    unsafe = re.compile(r"(?<![\w])(?:%s)(?![\w])" % "|".join(re.escape(x) for x in fields)) if fields else None
    edits = []
    seen = []
    for m in re.finditer(r'"(?:%s|%s)"' % (src.plural_snake, src.plural_camel), text):
        s, e = item_around(text, m.start())
        s += len(text[s:]) - len(text[s:].lstrip())
        chunk = text[s:e]
        if unsafe and unsafe.search(chunk.replace(m.group(0), "")):
            continue
        if any(a <= m.start() < b for a, b in seen):
            continue
        seen.append((s, e))
        line_start = text.rfind("\n", 0, s) + 1
        line_end = text.find("\n", e)
        whole = text[line_start:s].strip() == "" and text[e:line_end].strip() in ("", ",", ")") and "\n" not in chunk.strip()
        if whole:
            edits.append((line_end + 1, src.rename(text[line_start:line_end + 1], n, text)))
        else:
            sep = "," if text[e:e + 1] == "," or "," in text[max(0, s - 2):s] else (" |" if "|" in text[max(0, s - 3):s + 1] or text[e:e + 3].strip().startswith("|") else ",")
            edits.append((e, f"{sep} " + src.rename(chunk.strip(), n, text)))
    for m in re.finditer(r"snapshot\.%s\.len\(\)" % src.plural_snake, text):
        if any(a <= m.start() < b for a, b in seen):
            continue
        e = m.end()
        line_start = text.rfind("\n", 0, m.start()) + 1
        line_end = text.find("\n", e)
        if text[line_start:m.start()].strip() in ("+", ""):
            edits.append((line_end + 1, src.rename(text[line_start:line_end + 1], n, text)))
        else:
            edits.append((e, " + " + src.rename(m.group(0), n, text)))
    for at, add in sorted(edits, reverse=True):
        text = text[:at] + add + text[at:]
    return text


def outline_files(ctx):
    return ctx.layout.find("/💡️inferences/🧾outline/🦀️.rs")


def rows(ctx):
    paths = outline_files(ctx)
    if not paths:
        raise Unsupported("no outline module found")
    done = 0
    for path in paths:
        text = ctx.plan.read(path)
        src = source_for(ctx, text)
        if not src:
            ctx.plan.note(f"outline {ctx.layout.rel(path)}: lists no collection of this artifact, nothing to add")
            continue
        new = clone_registry_items(ctx, text, src)
        if new != text:
            ctx.plan.write(path, new)
            done += 1
    ctx.plan.note(f"outline: collection name and row count cloned from the registered siblings ({done} files)")


def units(ctx):
    S, n, r = ctx.spec.collection, ctx.spec.collection.names, ctx.ref
    paths = [p for p in ctx.layout.find("/🧮️units/🦀️.rs") if "ROW_COLLECTIONS" in ctx.plan.read(p)]
    if not paths:
        ctx.plan.note("incremental units: this artifact keeps no row-collection registry (check units are owned by the checks of a collection)")
        return
    for path in paths:
        text = ctx.plan.read(path)
        if f'kind("{n.camel}", "{n.plural_camel}"' in text:
            done = True
            continue
        if f'"{n.plural_camel}"' in text and not re.search(r'^    kind\("%s", "%s", ' % (re.escape(r.camel), r.plural_camel), text, re.M):
            continue
        src = ctx.ref if re.search(r'rows = group\(rows, &d\.%s, "%s"' % (ctx.ref.plural_snake, ctx.ref.plural_camel), text) else None
        m = re.search(r"const ROW_COLLECTIONS: \[&str; (\d+)\] = \[([^\]]*)\];", text)
        text = text.replace(m.group(0), f'const ROW_COLLECTIONS: [&str; {int(m.group(1)) + 1}] = [{m.group(2)}, "{n.plural_camel}"];', 1)
        text = re.sub(r'(Owners::new\(&\[[^\]]*"%s")' % ctx.ref.plural_camel, lambda m: m.group(1) + f', "{n.plural_camel}"', text, count=1)
        if src:
            text = clone_after(text, line_span(text, r'^ +rows = group\(rows, &d\.%s, "%s"[^\n]*\n' % (ctx.ref.plural_snake, ctx.ref.plural_camel)), ctx.ref, n)
        ctx.plan.write(path, text)
    ctx.plan.note("incremental units: row collection, owners, group")


CALL = r"push_duplicate_ids\("


def call_statement(text, names):
    """🫄 Span of the `push_duplicate_ids(..);` statement whose arguments mention the collection (single or multi line)."""
    for m in re.finditer(r"^[ \t]*%s" % CALL, text, re.M):
        end = balanced_end(text, m.start())
        chunk = text[m.start():end]
        if re.search(r'"%s"' % names.plural_camel, chunk) and not chunk.strip().startswith("//"):
            return m.start(), end
    return None


def integrity(ctx):
    S, n = ctx.spec.collection, ctx.spec.collection.names
    paths = [p for p in ctx.layout.find("/🦀️.rs") if re.search(r"push_duplicate_ids\(", ctx.plan.read(p)) and "/🧪️tests/" not in p and "🧬️mutations" not in p]
    if not paths:
        ctx.plan.note("integrity: this artifact has no duplicate-id integrity checks")
        return
    wrote = 0
    for path in paths:
        text = ctx.plan.read(path)
        if f'"{n.plural_camel}"' in text and f"doc.{n.plural_snake}" in text or re.search(r'push_duplicate_ids\([^;]*"%s"' % n.plural_camel, text):
            continue
        span, src = None, None
        for names in [ctx.ref] + [Names(p, e) for p, e in reversed(ctx.collections) if p != ctx.ref.plural_camel]:
            if names.plural_camel == n.plural_camel:
                continue
            span = call_statement(text, names)
            if span:
                src = names
                break
        if not span:
            continue
        start, end = span
        chunk = src.rename(text[start:end], n, text)
        chunk = re.sub(r'"%s",\n(\s+)"[^"]*",\n(\s+)"[^"]*",' % n.plural_camel, lambda m: f'"{n.plural_camel}",\n{m.group(1)}"{S.singular_en}",\n{m.group(2)}"{S.singular_de}",', chunk)
        add = chunk
        for nn in S.nested:
            nref = ctx.nested_ref_names()
            ns, ne = loop_span(text, ctx.nref.plural_snake if ctx.nref else None, nref.plural_snake) if ctx.nref else (None, None)
            if ns is None:
                continue
            nested = text[ns:ne].replace(ctx.nref.plural_snake, n.plural_snake).replace(ctx.nref.plural_camel, n.plural_camel).replace(nref.plural_snake, nn.names.plural_snake).replace(nref.plural_camel, nn.names.plural_camel)
            nested = re.sub(r'("%s",\n\s+)"[^"]*",(\n\s+)"[^"]*",' % nn.names.plural_camel, lambda m: f'{m.group(1)}"{nn.singular_en}",{m.group(2)}"{nn.singular_de}",', nested)
            var = re.match(r"\s*for (\w+) in", nested).group(1)
            add += re.sub(r"\b%s\b" % var, "row", nested)
        text = text[:end] + add + text[end:]
        ctx.plan.write(path, text)
        wrote += 1
    ctx.plan.note(f"integrity: duplicate-id checks cloned ({wrote} files)")


def loop_span(text, parent_snake, nested_snake):
    m = re.search(r"^    for \w+ in &doc\.%s \{\n" % parent_snake, text, re.M)
    if not m:
        return None, None
    end = text.find("\n    }\n", m.end()) + len("\n    }\n")
    return m.start(), end


ROW = re.compile(r'^(?P<indent>[ \t]*)(?P<open>\(?)"(?P<key>[^"]+)"(?P<mid>\s*(?:,|=>)\s*)(?P<expr>.*?)(?P<close>\)?,?)[ \t]*$')


def meta_module(ctx, key_hint=None):
    for p in ctx.layout.find("/🦀️.rs"):
        text = ctx.plan.read(p)
        if "/🧪️tests/" in p or "🧬️mutations" in p:
            continue
        if (key_hint and re.search(r'"%s\[\]\.id"' % re.escape(key_hint), text)) or (not key_hint and "fn lookup_field_meta" in text):
            return p
    return None


def parse_rows(text):
    out = []
    for m in re.finditer(r"^.*$", text, re.M):
        r = ROW.match(m.group(0))
        if r and re.search(r'\b(?:FieldSpec::|m\(|mu\(|mc\(|m!\()', r.group("expr")):
            out.append((m.start(), m.end(), r))
    return out


def classify(expr):
    if "UnitId::" in expr:
        return "quantity"
    if "NormFieldChoice" in expr or re.search(r"\b[A-Z][A-Z0-9_]{2,}\b", expr.replace("UnitId", "")):
        return "choice"
    return "plain"


def literal(value):
    return value.replace("\\", "\\\\").replace('"', '\\"')


ITEM = re.compile(r'NormFieldChoice \{ value: "[^"]*", label_en: "(?:[^"\\]|\\.)*", label_de: "(?:[^"\\]|\\.)*" \}|choice\("[^"]*", "(?:[^"\\]|\\.)*", "(?:[^"\\]|\\.)*"\)')


def choice_item(text, v):
    """🫃 One choice entry in the spelling the artifact uses (`NormFieldChoice { .. }` or `choice(..)`)."""
    m = ITEM.search(text)
    if not m:
        raise Unsupported("no NormFieldChoice entry to learn the choice spelling from")
    if m.group(0).startswith("NormFieldChoice"):
        return f'NormFieldChoice {{ value: "{v["token"]}", label_en: "{literal(v["label"]["en"])}", label_de: "{literal(v["label"]["de"])}" }}'
    return f'choice("{v["token"]}", "{literal(v["label"]["en"])}", "{literal(v["label"]["de"])}")'


def row_text(template, key, field_spec, kind, unit_id=None, const=None, items=None):
    """🫂 A field-meta row cloned from a template row of the same class: key, the first two string literals (EN, DE), the unit id or the choice constant (or inline choice array) are replaced."""
    r = template
    expr = r.group("expr")
    strings = list(re.finditer(r'"((?:[^"\\]|\\.)*)"', expr))
    if len(strings) < 2:
        raise Unsupported("field-meta template row has no EN + DE literals")
    en, de = field_spec
    inline = re.search(r"&\[NormFieldChoice.*\]", expr)
    if kind == "choice" and inline and items is not None:
        expr = expr[: inline.start()] + "&[" + ", ".join(items) + "]" + expr[inline.end():]
        strings = list(re.finditer(r'"((?:[^"\\]|\\.)*)"', expr))
    expr = expr[: strings[1].start()] + f'"{literal(de)}"' + expr[strings[1].end():]
    expr = expr[: strings[0].start()] + f'"{literal(en)}"' + expr[strings[0].end():]
    if kind == "quantity" and unit_id:
        expr = re.sub(r"UnitId::\w+", "UnitId::" + unit_id, expr)
    if kind == "choice" and const and not inline:
        expr = re.sub(r"\b[A-Z][A-Z0-9_]{2,}\b", const, expr)
    return f'{r.group("indent")}{r.group("open")}"{key}"{r.group("mid")}{expr}{r.group("close")}\n'


def unit_of(f):
    return f.unit_id if f.is_real and f.unit_id else None


def template_for(rows_, ref_key, kind):
    """🫁 A template row of the wanted class: from the reference collection first, then any collection."""
    for prefer in (True, False):
        for _, _, r in rows_:
            key = r.group("key")
            if prefer and not key.startswith(ref_key):
                continue
            if "[]" not in key and not prefer:
                continue
            if classify(r.group("expr")) == kind:
                return r
    return None


def choice_const(text, f):
    if re.search(r"^\s*const %s: &\[NormFieldChoice\]" % f.enum_const, text, re.M):
        return text
    consts = list(re.finditer(r"^( *)const \w+: &\[NormFieldChoice\] = &\[(?:\n(?:\1    [^\n]*\n)+\1\];\n|[^\n]*\];\n)", text, re.M))
    if not consts:
        return text
    last = consts[-1]
    indent = last.group(1)
    if "\n" in last.group(0).rstrip("\n"):
        body = "".join(f"{indent}    {choice_item(text, v)},\n" for v in f.values)
        block = f"{indent}const {f.enum_const}: &[NormFieldChoice] = &[\n{body}{indent}];\n"
    else:
        block = f"{indent}const {f.enum_const}: &[NormFieldChoice] = &[{', '.join(choice_item(text, v) for v in f.values)}];\n"
    return text[: last.end()] + block + text[last.end():]


def meta_rows_for(ctx, text, base_key, entries, ref_key, enums=None):
    """🫀 Row lines for `entries` = [(key, (en, de), kind, unit_id, const)] as clones of template rows."""
    rows_ = parse_rows(text)
    lines = ""
    for key, spec, kind, unit_id, const in entries:
        tmpl = template_for(rows_, ref_key, kind) or template_for(rows_, ref_key, "plain")
        if not tmpl:
            raise Unsupported(f"no {kind} field-meta row to clone")
        use = kind if template_for(rows_, ref_key, kind) else "plain"
        items = [choice_item(text, v) for v in enums[const].values] if use == "choice" and enums and const in enums else None
        lines += row_text(tmpl, key, spec, use, unit_id, const, items)
    return lines


def field_meta(ctx):
    S, r, n = ctx.spec.collection, ctx.ref, ctx.spec.collection.names
    path = meta_module(ctx, r.plural_camel) or meta_module(ctx)
    if not path:
        raise Unsupported("no field-metadata module with rows for the reference collection")
    text = ctx.plan.read(path)
    if re.search(r'"%s"' % n.plural_camel, text):
        return
    entries = [(n.plural_camel, (S.label["en"], S.label["de"]), "plain", None, None), (f"{n.plural_camel}[].id", (S.id_label["en"], S.id_label["de"]), "plain", None, None)]
    for f in S.fields:
        entries.append((f"{n.plural_camel}[].{f.camel}", (f.en, f.de), "choice" if f.ty == "enum" else ("quantity" if unit_of(f) else "plain"), unit_of(f), f.enum_const if f.ty == "enum" else None))
    for nn in S.nested:
        entries.append((f"{n.plural_camel}[].{nn.names.plural_camel}", (nn.label["en"], nn.label["de"]), "plain", None, None))
        entries.append((f"{n.plural_camel}[].{nn.names.plural_camel}[].id", (nn.id_label["en"], nn.id_label["de"]), "plain", None, None))
        for f in nn.fields:
            entries.append((f"{n.plural_camel}[].{nn.names.plural_camel}[].{f.camel}", (f.en, f.de), "choice" if f.ty == "enum" else ("quantity" if unit_of(f) else "plain"), unit_of(f), f.enum_const if f.ty == "enum" else None))
    block = meta_rows_for(ctx, text, r.plural_camel, entries, r.plural_camel, {f.enum_const: f for f in S.all_enums})
    last = [m for m in parse_rows(text) if m[2].group("key") == r.plural_camel or m[2].group("key").startswith(r.plural_camel + "[]")]
    if not last:
        raise Unsupported("field-meta rows of the reference collection not found")
    at = last[-1][1] + 1
    text = text[:at] + block + text[at:]
    for f in S.all_enums:
        text = choice_const(text, f)
    ctx.plan.write(path, text)
    ctx.plan.note("field-meta EN + DE labels, units and choice lists cloned from the reference rows")


def kit(ctx):
    k = ctx.spec.kit
    if not k:
        return
    r, n = ctx.ref, ctx.spec.collection.names
    done = False
    for path in ctx.layout.find("/🧰️kit/🦀️.rs"):
        text = ctx.plan.read(path)
        if f'kind("{n.camel}", "{n.plural_camel}"' in text:
            done = True
            continue
        if f'"{n.plural_camel}"' in text and not re.search(r'^    kind\("%s", "%s", ' % (re.escape(r.camel), r.plural_camel), text, re.M):
            continue
        m = re.search(r'^    kind\("%s", "%s", [^\n]*\n' % (re.escape(r.camel), r.plural_camel), text, re.M)
        if m:
            row = f'    kind("{n.camel}", "{n.plural_camel}", "{k["en"]}", "{k["de"]}", "{k["icon"]}", EntityKind::{k["scene_kind"]}),\n'
            ctx.plan.write(path, text[: m.end()] + row + text[m.end():])
            done = True
    if not done:
        raise Unsupported("kit row of the reference collection not found")


def run(ctx):
    for name, fn in (("rows", rows), ("units", units), ("integrity", integrity), ("field-meta", field_meta), ("kit", kit)):
        ctx.layer("wiring:" + name)
        try:
            fn(ctx)
        except Unsupported as error:
            ctx.unsupported.append(f"wiring:{name}: {error}")
