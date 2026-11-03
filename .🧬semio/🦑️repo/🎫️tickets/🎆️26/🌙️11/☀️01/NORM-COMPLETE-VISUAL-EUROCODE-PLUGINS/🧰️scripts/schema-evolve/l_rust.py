"""🦀 Hand-written Rust around the generated structs: model enums and example literal, artifact conversions, diff macros and the absorb/apply/is_empty/inverse arms."""
import re
from core import DIFF, SCHEMA, SNAPSHOT, Names, Unsupported, balanced_end, camel, clone_after, item_at, line_span, pascal, sub1, unit_at


def block_at(text, pattern, flags=re.M):
    """🪭 A brace block that starts at the first line matching `pattern` and ends at the closing brace line of the same indent."""
    m = re.search(pattern, text, flags)
    if not m:
        raise Unsupported("block not found: " + pattern[:100])
    start = text.rfind("\n", 0, m.start()) + 1
    indent = re.match(r" *", text[start:]).group(0)
    end = re.compile(r"^%s\}[^\n]*\n" % indent, re.M).search(text, text.find("\n", m.end()) if "{" in text[m.start():text.find("\n", m.end())] else m.end())
    return start, end.end()


def enum_variant_groups(body):
    """🪬 The variants of an enum body as (attribute lines, name) groups."""
    groups, attrs = [], []
    for line in body.splitlines(True):
        s = line.strip()
        if not s:
            continue
        if s.startswith(("#[", "///")):
            attrs.append(line)
        else:
            groups.append((attrs, re.match(r"\w+", s).group(0)))
            attrs = []
    return groups


CANON = "semio_framework_value::CanonicalJsonTree"


def canonical_json(text, head):
    """🧾 A cloned item header gets the `CanonicalJsonTree` derive and `#[canonical_json(owner = ..)]` (owner learned from the model) when the donor lacked them: insert/change mutation leaves derive it and need it on every item and field type."""
    if CANON in head:
        return head
    owner = re.search(r"#\[canonical_json\(owner = ([\w:]+)\)\]", text)
    if not owner:
        return head
    derive = re.search(r"#\[derive\(([^\n]*?)\)\]\n", head)
    if not derive:
        return head
    head = head[: derive.end(1)] + ", " + CANON + head[derive.end(1):]
    at = head.index("\n", derive.start()) + 1
    return head[:at] + f"#[canonical_json(owner = {owner.group(1)})]\n" + head[at:]


def clone_enum(ctx, text, f):
    """🪫 The enum of an enum field, cloned from a donor enum of the model (derive and attribute lines, variant attribute shape, `impl` blocks of the donor with regenerated match arms)."""
    enums = [m.group(1) for m in re.finditer(r"^pub enum (\w+) \{", text, re.M)]
    if not enums:
        raise Unsupported("no enum in the snapshot model to clone")
    def impls(name):
        return len(re.findall(r"^impl\b[^\n{]*\b%s\s*\{" % name, text, re.M))
    schema_tokens = {k: v["enum"] for k, v in ctx.defs.items() if v.get("type") == "string" and "enum" in v}
    donor = max((e for e in enums if e in schema_tokens), key=lambda e: (impls(e), enums.index(e)), default=enums[-1])
    start, end = item_at(text, r"^pub enum %s \{" % donor)
    header = text[start:end]
    head, _, rest = header.partition("pub enum %s {\n" % donor)
    body = rest.rsplit("}", 1)[0]
    groups = enum_variant_groups(body)
    doc = "/// %s\n" % f.enum.get("doc", f"{ctx.spec.collection.emoji if ctx.spec.collection else ctx.spec.raw.get('leaf', {}).get('emoji', '🔤')} {f.enum_name}.")
    head = canonical_json(text, re.sub(r"(?:^///[^\n]*\n)+", "", head, flags=re.M))
    tokens = schema_tokens.get(donor, [])
    new_body = ""
    for variant, token in zip(f.variants, f.tokens):
        if groups and groups[0][0]:
            dtoken = tokens[0] if tokens else ""
            attrs = "".join(a.replace(groups[0][1], variant).replace(f'"{dtoken}"', f'"{token}"') for a in groups[0][0])
        else:
            attrs = ""
        new_body += f"{attrs}    {variant},\n"
    out = doc + head + "pub enum %s {\n%s}\n" % (f.enum_name, new_body)
    for m in list(re.finditer(r"^impl\b[^\n{]*\b%s\s*\{" % donor, text, re.M)):
        i_start, i_end = item_at(text, r"^%s" % re.escape(m.group(0)))
        block = re.sub(r"^(\s*)///[^\n]*$", r"\1/// 🔤 Wire token of the variant.", text[i_start:i_end].replace(donor, f.enum_name), flags=re.M)
        arm = re.compile(r"^(\s*)(Self|%s)::(\w+)(\s*=>\s*)(.*?),?\n" % re.escape(f.enum_name), re.M)
        arms = list(arm.finditer(block))
        if arms:
            first = arms[0]
            tmpl = first.group(5)
            quoted = re.findall(r'"([^"]*)"', tmpl)
            dtoken = quoted[0] if len(quoted) == 1 else (tokens[[v for v in map(pascal, tokens)].index(first.group(3))] if tokens and first.group(3) in [pascal(t) for t in tokens] else None)
            if dtoken is None or f'"{dtoken}"' not in tmpl:
                continue
            lines = ""
            for variant, token in zip(f.variants, f.tokens):
                rhs = tmpl.replace(f'"{dtoken}"', f'"{token}"') if dtoken else tmpl
                lines += f"{first.group(1)}{first.group(2)}::{variant}{first.group(4)}{rhs},\n"
            block = block[: arms[0].start()] + lines + block[arms[-1].end():]
        out += "\n" + block.rstrip("\n") + "\n"
    return out, end


def example_literals(ctx, text):
    """🪪 Every struct literal of the snapshot that sets the reference collection (`joints: vec![]`, `Vec::new()`, or a populated example) gets the new collection as an empty list in the same spelling."""
    n, r = ctx.spec.collection.names, ctx.ref
    if re.search(r"(?<![\w.])(?<!pub )%s:\s*(?:vec!\[\]|Vec::new\(\)|Vec::default\(\))" % n.plural_snake, text):
        return text
    edits = []
    for m in re.finditer(r"(?<![\w.])(?<!pub )%s:\s*" % r.plural_snake, text):
        if text[max(0, m.start() - 4):m.start()] == "pub ":
            continue
        end = item_end(text, m.end())
        value = text[m.end():end].rstrip().rstrip(",").strip()
        empty = "Vec::new()" if value.startswith("Vec::") else "vec![]"
        line_start = text.rfind("\n", 0, m.start()) + 1
        if text[line_start:m.start()].strip() == "":
            edits.append((text.find("\n", end - 1) + 1 if not text[end:].startswith("\n") else end + 1, text[line_start:m.start()] + f"{n.plural_snake}: {empty},\n"))
        else:
            edits.append((end, f" {n.plural_snake}: {empty},"))
    for at, add in sorted(edits, reverse=True):
        text = text[:at] + add + text[at:]
    return text


LITERAL_SKIP = ("/🧬️schema/🔺️diff/", "twin", "/target/", "🪶️sqlite/📸️snapshot/🦀️.rs", "🧬️schema/📸️snapshot/🦀️.rs")


def snapshot_literal_spans(text, snapshot):
    """🧾 (start, end) of the body of every `Snapshot { .. }` / `Self { .. }` struct literal (impl headers and return types excluded)."""
    out = []
    for m in re.finditer(r"(?<![\w:])(?:\w+::)*(?:%s|Self)\s*\{" % re.escape(snapshot), text):
        before = text[max(0, m.start() - 80):m.start()]
        if re.search(r"\bimpl\b[^{;]*$|\bfor\s+$|->\s*$|\bstruct\s+$|\benum\s+$|\buse\s+[^;]*$", before):
            continue
        depth, i = 0, m.end() - 1
        while i < len(text):
            depth += text[i] == "{"
            depth -= text[i] == "}"
            if depth == 0:
                break
            i += 1
        out.append((m.end(), i))
    return out


def snapshot_literals(ctx):
    """🧾 Every struct literal of the snapshot in the artifact's hand-written Rust (examples, fixtures, tests; the model file and the SQLite codec have their own layers) that sets the reference collection gets the new collection as an empty list in the same spelling, so no literal misses a field."""
    from core import walk_files
    import l_patch
    n, r = ctx.spec.collection.names, ctx.ref
    snap = ctx.layout.snapshot_name
    for path in walk_files(ctx.layout.dir, (".rs",)):
        if any(x in path for x in LITERAL_SKIP):
            continue
        text = ctx.plan.read(path)
        if snap not in text or r.plural_snake not in text:
            continue
        edits = []
        for a, b in snapshot_literal_spans(text, snap):
            items = l_patch.split_top(text[a:b])
            keys = [re.match(r"\s*(\w+)\s*:", i).group(1) if re.match(r"\s*(\w+)\s*:", i) else None for i in items]
            if n.plural_snake in keys or r.plural_snake not in keys:
                continue
            value = items[keys.index(r.plural_snake)].split(":", 1)[1].strip()
            empty = "Vec::new()" if value.startswith("Vec::") else "vec![]"
            offset = 0
            spans = []
            for item in items:
                at = text.index(item, a + offset)
                spans.append((at, at + len(item)))
                offset = at + len(item) - a
            end = spans[keys.index(r.plural_snake)][1]
            line_start = text.rfind("\n", 0, spans[keys.index(r.plural_snake)][0]) + 1
            multiline = "\n" in text[a:b] and text[line_start:spans[keys.index(r.plural_snake)][0]].strip() == ""
            tail = text[end:end + 1]
            edits.append((end + 1 if tail == "," else end, ("" if tail == "," else ",") + (("\n" + text[line_start:spans[keys.index(r.plural_snake)][0]]) if multiline else " ") + f"{n.plural_snake}: {empty}" + ("," if tail == "," else "")))
        for at, add in sorted(edits, reverse=True):
            text = text[:at] + add + text[at:]
        if edits:
            ctx.plan.write(path, text)
    ctx.plan.note(f"snapshot literals: `{n.plural_snake}: vec![]` added to the struct literals of the artifact's hand-written Rust")


def row_structs(ctx, text):
    """🧱 Models that keep their row records as hand-readable structs (the reference row is a `pub struct` of the model file) get the row struct of the new entity (and its nested rows) cloned from the reference header with the spec's fields; models whose structs are generated from the schema twins are left alone."""
    S, r = ctx.spec.collection, ctx.ref
    if not re.search(r"^pub struct %s \{" % r.entity, text, re.M):
        return text

    def struct(ent, ref_entity):
        if re.search(r"^pub struct %s \{" % ent.entity, text, re.M):
            return None
        start, end = item_at(text, r"^pub struct %s \{" % ref_entity)
        block = text[start:end]
        head = block[:block.index("pub struct ")]
        head = canonical_json(text, re.sub(r"(?:^///[^\n]*\n)+", "", head, flags=re.M))
        body = "    pub id: String,\n"
        for f in ent.fields:
            ty = f.enum_name if f.ty == "enum" else f.rust_ty
            body += (f"    /// {f.doc}\n" if f.doc else "") + f"    pub {f.name}: {ty},\n"
        for n in ent.nested:
            body += f"    pub {n.names.plural_snake}: Vec<{n.entity}>,\n"
        return f"/// {ent.emoji} {ent.doc}\n" + head + f"pub struct {ent.entity} {{\n{body}}}\n"
    pairs = [(n, ctx.nested_ref_names().entity) for n in S.nested]
    for ent, ref_entity in pairs + [(S, r.entity)]:
        new = struct(ent, ref_entity)
        if new:
            start, end = item_at(text, r"^pub struct %s \{" % ref_entity)
            text = text[:end] + "\n" + new + text[end:]
    return text


def model_rs(ctx):
    L, S = ctx.layout, ctx.spec.collection
    path = L.p(SNAPSHOT + "/🦀️.rs")
    text = ctx.plan.read(path)
    for f in S.all_enums:
        if re.search(r"pub enum %s \{" % f.enum_name, text):
            continue
        out, end = clone_enum(ctx, text, f)
        last = max([m.end() for m in re.finditer(r"^impl\b[^\n{]*\b\w+ \{", text, re.M)] + [end])
        text = text[:end] + "\n" + out + text[end:]
    text = row_structs(ctx, text)
    text = example_literals(ctx, text)
    ctx.plan.write(path, text)
    ctx.plan.note("model: enums " + ", ".join(f.enum_name for f in S.all_enums) + f" and the example literal `{S.names.plural_snake}: vec![]`")


def artifact_rs(ctx):
    L, S = ctx.layout, ctx.spec.collection
    path = L.p(SCHEMA + "/🦀️.rs")
    text = ctx.plan.read(path)
    n, r = S.names, ctx.ref
    if re.search(r"\bpub %s:" % n.plural_snake, text):
        return

    def use(m):
        names = [x.strip() for x in m.group(2).split(",")]
        if S.entity in names:
            return m.group(0)
        names.append(S.entity)
        return m.group(1) + ", ".join(sorted(names)) + m.group(3)
    text = re.sub(r"(use %s::\{)([^}]*\b%s\b[^}]*)(\};)" % (re.escape(L.model_crate), r.entity), use, text, count=1)
    text = clone_mentions(ctx, text, with_access=False)
    text = re.sub(r"(pub %s: Vec<)((?:\w+::)*)%s>" % (n.plural_snake, re.escape(r.entity)), lambda m: m.group(1) + m.group(2) + S.entity + ">", text)
    ctx.plan.write(path, text)
    ctx.plan.note("artifact conversions and `use` list")


def key_and_macros(ctx, prefix):
    """🪩 Clones key struct + impl and the Patch/Rows macros of the reference into the diff Rust file, with regenerated Patch field sets."""
    L, S = ctx.layout, ctx.spec.collection
    path = L.p(DIFF + "/🦀️.rs")
    text = ctx.plan.read(path)
    n, r = S.names, ctx.ref
    model = L.model_crate + "::"
    if f"pub struct {n.entity}Key;" in text:
        return
    k_start, k_end = item_at(text, r"^pub struct %sKey;" % r.entity)
    i_start, i_end = item_at(text, r"^impl protocol::list_delta::KeyOf<[\w:]*%s> for %sKey" % (r.entity, r.entity))
    text = text[:k_end] + "\n" + r.rename(text[k_start:k_end], n) + text[k_end:]
    i_start, i_end = item_at(text, r"^impl protocol::list_delta::KeyOf<[\w:]*%s> for %sKey" % (r.entity, r.entity))
    text = text[:i_end] + "\n" + r.rename(text[i_start:i_end], n) + text[i_end:]

    def sets(fields):
        return ", ".join(f"{f.name}: {model + f.enum_name if f.ty == 'enum' else f.rust_ty}" for f in fields)

    nest = ""
    if S.nested:
        nref = ctx.nested_ref_names()
        for nn in S.nested:
            ks, ke = item_at(text, r"^pub struct %sKey;" % nref.entity)
            if f"pub struct {nn.entity}Key;" not in text:
                text = text[:ke] + "\n" + nref.rename(text[ks:ke], nn.names) + text[ke:]
                is_, ie = item_at(text, r"^impl protocol::list_delta::KeyOf<[\w:]*%s> for %sKey" % (nref.entity, nref.entity))
                text = text[:ie] + "\n" + nref.rename(text[is_:ie], nn.names) + text[ie:]
            text = clone_pair(text, model, prefix, (ctx.nref.plural_pascal + nref.plural_pascal, nref.entity, ctx.nref.plural_snake + "_" + nref.plural_snake), (n.plural_pascal + pascal(nn.plural), nn.entity, n.plural_snake + "_" + nn.names.plural_snake), nn.fields, sets)
            nest = " nest { %s: %s%s%sRows }" % (nn.names.plural_snake, prefix, n.plural_pascal, pascal(nn.plural))
    text = clone_pair(text, model, prefix, (r.plural_pascal, r.entity, r.plural_snake), (n.plural_pascal, n.entity, n.plural_snake), S.fields, sets, nest)
    ctx.plan.write(path, text)


def macro_span(text, pattern):
    """🪨 A `protocol::row_patch! { ... }` / `list_delta! { ... }` macro call that contains the first match of `pattern` (docstring and attribute lines included)."""
    m = re.search(pattern, text, re.M)
    if not m:
        raise Unsupported("macro not found: " + pattern[:100])
    start = text.rfind("\n\n", 0, m.start()) + 2
    end = text.find("\n}\n", m.end()) + 3
    return start, end


def clone_pair(text, model, prefix, ref, new, fields, sets, nest=""):
    """🪧 Clones the Patch and Rows macros of the reference row type (`ref`: plural Pascal, entity, plural snake) for the new one (`new`), with a regenerated `set` list."""
    ref_pl, ref_ent, ref_snake = ref
    new_pl, new_ent, new_snake = new
    if f"pub {prefix}{new_pl}Patch of" in text:
        return text
    ps, pe = macro_span(text, r"^ +pub %s%sPatch of " % (prefix, ref_pl))
    rs_, re_ = macro_span(text, r"^ +pub %s%sRows \{" % (prefix, ref_pl))
    patch = text[ps:pe]
    patch = re.sub(r"set \{[^}]*\}( nest \{[^}]*\})?", lambda m: "set { %s }%s" % (sets(fields), nest), patch)
    patch = patch.replace(prefix + ref_pl, prefix + new_pl)
    patch = re.sub(r"of [\w:]*%s " % ref_ent, "of %s%s " % (model, new_ent), patch)
    patch = patch.replace("`%s`" % ref_snake, "`%s`" % new_snake)
    rows = text[rs_:re_]
    rows = rows.replace(prefix + ref_pl, prefix + new_pl).replace(ref_ent + "Key", new_ent + "Key")
    rows = re.sub(r"[\w:]*::%s\b" % ref_ent, model + new_ent, rows)
    rows = rows.replace("`%s`" % ref_snake, "`%s`" % new_snake)
    last = max(pe, re_)
    return text[:last] + "\n" + patch + "\n" + rows + text[last:]


def lane_arms(ctx):
    L, S = ctx.layout, ctx.spec.collection
    path = L.p(DIFF + "/🦀️.rs")
    text = ctx.plan.read(path)
    n, r = S.names, ctx.ref
    if f"other.{n.plural_snake}" in text:
        return
    spans = [
        block_at(text, r"^ +if let Some\(theirs\) = other\.%s \{" % r.plural_snake),
        block_at(text, r"^ +if let Some\(rows\) = &self\.%s \{" % r.plural_snake),
    ]
    for start, end in sorted(spans, reverse=True):
        text = clone_after(text, (start, end), r, n)
    start, end = line_span(text, r"^ +%s: self\.%s\.as_ref\(\)\.map\(\|rows\| rows\.inverse" % (r.plural_snake, r.plural_snake))
    text = clone_after(text, (start, end), r, n)
    m = re.search(r"&& self\.%s\.as_ref\(\)\.map_or\(true, \|rows\| rows\.is_empty\(\)\)" % r.plural_snake, text)
    if not m:
        raise Unsupported("is_empty chain of the diff")
    text = text[: m.end()] + " && self.%s.as_ref().map_or(true, |rows| rows.is_empty())" % n.plural_snake + text[m.end():]
    ctx.plan.write(path, text)


def blocks_of(text):
    """🪦 Top-level items as (start, end) spans: an item starts at its first doc or attribute line and ends at the line that brings the bracket depth back to zero with `}` or `;` (items that follow each other without a blank line stay apart, an `impl` with blank lines between its functions stays whole)."""
    from lex import code_mask
    mask = code_mask(text, "rs")
    spans, start, depth, pos = [], None, 0, 0
    for line in text.splitlines(True):
        body = line.strip()
        if start is None and body:
            start = pos
        if start is not None:
            for k, ch in enumerate(line):
                if mask[pos + k]:
                    depth += ch in "([{"
                    depth -= ch in ")]}"
            if depth <= 0 and body.endswith(("}", ";")) and not body.startswith("//"):
                spans.append((start, pos + len(line.rstrip("\n"))))
                start, depth = None, 0
        pos += len(line)
    if start is not None:
        spans.append((start, len(text)))
    return spans


def regen_struct_patch(chunk, fields, model):
    """🧱 Patch types written as plain structs (`pub struct EnNPlural Patch { pub x: Option<T>, .. }` with its `commit_into`/`absorb`/`inverse`/`is_empty`): the scalar members, assignment lines and `is_none` chain of the donor row are replaced by the ones of the new row; lines of nested `..Rows` lanes stay."""
    ty = lambda f: model + f.enum_name if f.ty == "enum" else f.rust_ty

    def run(rx, make, text, indent=True):
        lines = list(re.finditer(rx, text, re.M))
        scalar = [m for m in lines if not re.search(r"Rows\b|rows", m.group(0))]
        if not scalar:
            return text
        first, last = scalar[0], scalar[-1]
        pad = re.match(r"[ \t]*", first.group(0)).group(0)
        body = "".join(pad + make(f) + "\n" for f in fields)
        return text[: first.start()] + body + text[last.end():]
    chunk = run(r"^[ \t]*pub \w+: Option<[^\n]*>,\n", lambda f: f"pub {f.name}: Option<{ty(f)}>,", chunk) if re.search(r"^pub struct \w+Patch \{", chunk, re.M) else chunk
    if "fn commit_into" in chunk:
        chunk = run(r"^[ \t]*if let Some\(value\) = &self\.\w+ \{ row\.\w+ = value\.clone\(\); \}\n", lambda f: f"if let Some(value) = &self.{f.name} {{ row.{f.name} = value.clone(); }}", chunk)
        chunk = run(r"^[ \t]*if later\.\w+\.is_some\(\) \{ self\.\w+ = later\.\w+; \}\n", lambda f: f"if later.{f.name}.is_some() {{ self.{f.name} = later.{f.name}; }}", chunk)
        chunk = run(r"^[ \t]*\w+: self\.\w+\.as_ref\(\)\.map\(\|_\| row\.\w+\.clone\(\)\),\n", lambda f: f"{f.name}: self.{f.name}.as_ref().map(|_| row.{f.name}.clone()),", chunk)
        m = re.search(r"(?:self\.\w+\.is_none\(\) && )+", chunk)
        if m:
            chunk = chunk[: m.start()] + "".join(f"self.{f.name}.is_none() && " for f in fields) + chunk[m.end():]
    return chunk


def clone_family_blocks(ctx, ref=None, new=None, fields=None, child=None, vec_child=None):
    """🪤 Duplicates every top-level block that declares a name of the reference row group (key struct and impl, Patch, Rows or Delta macro), renamed, with a regenerated Patch `set` list. `ref`/`new`/`fields` clone the group of another row type (the nested child); `child` = (donor child names, new child names) keeps the `nest { lane: Delta }` link of the donor Patch pointing at the new child group; `vec_child` = (child names, child entity) is for donors that replace the nested array as a whole (`set { actions: Vec<Row> }`), so the child joins the `set` list as an array and has no group."""
    import l_schema
    L, S = ctx.layout, ctx.spec.collection
    path = L.p(DIFF + "/🦀️.rs")
    text = ctx.plan.read(path)
    n, r = new or S.names, ref or ctx.ref
    fields = S.fields if fields is None else fields
    if re.search(r"\b\w*%s(DeltaKey|Key)\b" % re.escape(n.entity), text) or f"{n.plural_pascal}Patch of" in text:
        return
    forms = [r.entity, r.plural_pascal]
    rx = re.compile(r"\b(?:En\d{4})?(?:%s)%s\b" % ("|".join(re.escape(f) for f in forms), l_schema.ROW_SUFFIX))
    model = L.model_crate + "::"
    names = r"(?:En\d{4})?(?:%s)%s\b" % ("|".join(re.escape(f) for f in forms), l_schema.ROW_SUFFIX)
    declares = re.compile(r"^\s*(?:pub\s+(?:struct\s+)?|impl\b[^\n]*\bfor\s+)%s|^\s*(?:En\d{4})?(?:%s)Patch\s+for\s" % (names, "|".join(re.escape(f) for f in forms)), re.M)
    picked = [(s, e) for s, e in blocks_of(text) if declares.search(text[s:e])]
    if not any(re.search(r"Patch|Rows|Delta", text[s:e]) for s, e in picked):
        raise Unsupported("no Key/Patch/Rows/Delta blocks of the reference row group " + r.entity)
    add = ""
    for s, e in picked:
        chunk = text[s:e]
        for pair in ([child] if child and isinstance(child, tuple) else child or []):
            chunk = pair[0].rename(chunk, pair[1], text)
        chunk = r.rename(chunk, n, text)
        if not child:
            chunk = re.sub(r" nest \{[^}]*\}", "", chunk)
        if re.search(r"^\s*set \{", chunk, re.M):
            chunk = re.sub(r"^(\s*)(within|optional|rows) \{[^\n]*\}$", lambda m: f"{m.group(1)}{m.group(2)} {{  }}", chunk, flags=re.M)
        else:
            chunk = re.sub(r"(Patch for [\w:]+ \{)[^}]*(\})", lambda m: m.group(1) + " " + ", ".join(f"{f.name}: {model + f.enum_name if f.ty == 'enum' else f.rust_ty}" for f in fields) + " " + m.group(2), chunk, count=1)
        decl = re.search(r"^\s*pub (?:struct )?(\w+)\b|^impl\b[^\n]*\bfor (\w+)", chunk, re.M)
        name = decl and (decl.group(1) or decl.group(2))
        if name and re.search(r"^\s*pub (?:struct )?%s\b|^impl\b[^\n]*\bfor %s\b" % (re.escape(name), re.escape(name)), text, re.M):
            continue
        extra = [f"{vec_child[0].plural_snake}: Vec<{model}{vec_child[1].entity}>"] if vec_child else []
        chunk = regen_struct_patch(chunk, fields, model)
        chunk = re.sub(r"set \{[^}]*\}", lambda m: "set { %s }" % ", ".join([f"{f.name}: {model + f.enum_name if f.ty == 'enum' else f.rust_ty}" for f in fields] + extra), chunk)
        add += "\n\n" + chunk.strip("\n") if not add else "\n\n" + chunk.strip("\n")
    last = picked[-1][1]
    text = text[:last] + add + text[last:]
    ctx.plan.note(f"diff Rust: {r.entity} row group cloned for {n.entity}") if add else None
    km = re.search(r"^(\s*)(\w*Key) for %s;\n" % re.escape(r.entity), text, re.M)
    if km and not re.search(r"\bfor %s;" % re.escape(n.entity), text):
        text = text[: km.end()] + r.rename(km.group(0), n, text).replace(r.entity, n.entity) + text[km.end():]
    ctx.plan.write(path, text)


def split_top(line, sep):
    """🪠 Spans of the items of `line` separated by `sep` at bracket depth 0."""
    spans, depth, start, i, quote = [], 0, 0, 0, False
    while i < len(line):
        ch = line[i]
        if quote:
            quote = not (ch == '"' and line[i - 1] != "\\")
        elif ch == '"':
            quote = True
        elif ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        elif depth == 0 and line.startswith(sep, i):
            spans.append((start, i))
            start, i = i + len(sep), i + len(sep) - 1
        i += 1
    spans.append((start, len(line)))
    return spans


def item_end(text, start):
    """🪟 End (exclusive, after the comma) of the comma separated item that begins at `start` (stops at an unmatched closing bracket)."""
    depth, i, quote = 0, start, False
    while i < len(text):
        ch = text[i]
        if quote:
            if ch == "\\":
                i += 1
            elif ch == '"':
                quote = False
        elif ch == '"':
            quote = True
        elif ch in "([{":
            depth += 1
        elif ch in ")]}":
            if depth == 0:
                return i
            depth -= 1
        elif ch == "," and depth == 0:
            return i + 1
        i += 1
    return len(text)


def clone_mentions(ctx, text, with_items=True, with_access=True):
    """🪞 Clones, renamed, every site of `text` that registers the reference collection field: struct fields and literal items (`pub joints: ..`, `joints: self.joints.clone(),`, inline or one per line) and statements that access it (`self.joints.absorb(..)`, a `&&` chain item). Attribute lines above a declaration travel with it."""
    S, r, n = ctx.spec.collection, ctx.ref, ctx.spec.collection.names
    if re.search(r"\bpub %s:|\b%s:|\.%s\b" % (n.plural_snake, n.plural_snake, n.plural_snake), text):
        return text
    edits, covered = [], []
    lines = text.splitlines(True)
    starts, pos = [], 0
    for line in lines:
        starts.append(pos)
        pos += len(line)

    def line_of(offset):
        import bisect
        return bisect.bisect_right(starts, offset) - 1

    if with_items:
        for m in re.finditer(r"(?<![\w.])(?:pub\s+)?%s\s*:\s*(?=\S)" % r.plural_snake, text):
            if any(a <= m.start() < b for a, b in covered):
                continue
            end = item_end(text, m.end())
            li = line_of(m.start())
            whole = text[starts[li]:m.start()].strip() == "" and "\n" in text[m.start():end + 1] or (text[starts[li]:m.start()].strip() == "" and text[end:text.find("\n", end)].strip() == "")
            if whole:
                end = balanced_end(text, starts[li]) if not text[m.end():end].rstrip().endswith(",") else text.find("\n", end) + 1
                begin = starts[li]
                j = li
                while j > 0 and lines[j - 1].lstrip().startswith("#[") and not lines[j - 1].lstrip().startswith("#[cfg"):
                    j -= 1
                    begin = starts[j]
                edits.append((end, r.rename(text[begin:end], n, text)))
            else:
                edits.append((end, " " + r.rename(text[m.start():end].rstrip(), n, text)))
            covered.append((m.start(), end))
    if with_access:
        field = re.compile(r"\.%s(?![\w])" % r.plural_snake)
        last_end = -1
        for index, line in enumerate(lines):
            start = starts[index]
            m = field.search(line)
            if not m or start < last_end or any(a <= start + m.start() < b for a, b in covered):
                continue
            parts = split_top(line.rstrip("\n"), "&&")
            if len(parts) > 1 and not line.strip().startswith("&&"):
                for a, b in parts:
                    if field.search(line[a:b]):
                        edits.append((start + b, " &&" + r.rename(line[a:b], n, text)))
                continue
            end = balanced_end(text, start)
            edits.append((end, r.rename(text[start:end], n, text)))
            last_end = end
    for at, add in sorted(edits, reverse=True):
        text = text[:at] + add + text[at:]
    return text


def clone_mention_lines(ctx, path_suffix="🧬️schema/🔺️diff/🦀️.rs"):
    path = ctx.layout.p(path_suffix)
    text = ctx.plan.read(path)
    new = clone_mentions(ctx, text)
    if new != text:
        ctx.plan.write(path, new)


def diff_tree_files(ctx):
    """🪜 The Rust files of the diff facet (no twins, tests or packages)."""
    from core import walk_files
    root = ctx.layout.p(DIFF)
    return [p for p in walk_files(root, (".rs",)) if not any(x in p[len(root):] for x in ("twin", "🧪️tests", "📦️packages"))]


def diff_modules(ctx):
    """🪝 Row groups kept in their own module files (`📍️placement-rows/🦀️.rs`, mounted by `#[path]`, keyed through a `row_key!` list): the module is cloned with the new names and Patch fields, the mount, key and leaf-list entries follow."""
    import l_schema
    L, S = ctx.layout, ctx.spec.collection
    root = L.p(DIFF)
    main = root + "/🦀️.rs"
    text = ctx.plan.read(main)
    n, r = S.names, ctx.ref
    slug_ref, slug_new = Names(r.plural_camel, pascal(r.plural_camel[:-1] if r.plural_camel.endswith("s") else r.plural_camel)), Names(n.plural_camel, pascal(n.plural_camel[:-1] if n.plural_camel.endswith("s") else n.plural_camel))
    rx = re.compile(r'(?P<block>#\[path = "(?P<dir>[^"]+)/🦀️\.rs"\]\nmod (?P<mod>\w+);\npub use (?P=mod)::\*;\n)')
    pick = None
    for m in rx.finditer(text):
        path = f"{root}/{m.group('dir')}/🦀️.rs"
        if ctx.plan.exists(path) and re.search(r"\b(?:En\d{4})?%sRows\b" % re.escape(r.plural_pascal), ctx.plan.read(path)):
            pick = m
    if not pick:
        raise Unsupported("no diff row module of the reference collection " + r.plural_camel)
    old_dir = pick.group("dir")
    new_dir = slug_ref.rename(old_dir, slug_new, text)
    new_mod = slug_ref.rename(pick.group("mod"), slug_new, text) if pick.group("mod") != slug_ref.rename(pick.group("mod"), slug_new, text) else pick.group("mod") + "_" + n.plural_snake
    if f'"{new_dir}/🦀️.rs"' in text:
        return
    modtext = ctx.plan.read(f"{root}/{old_dir}/🦀️.rs")
    modtext = r.rename(modtext, n, modtext)
    modtext = slug_ref.rename(modtext, slug_new, modtext)
    model = L.model_crate + "::"
    fields = ", ".join(f"{f.name}: {model + f.enum_name if f.ty == 'enum' else f.rust_ty}" for f in S.fields)
    modtext = re.sub(r"(Patch for [\w:]+ \{)[^}]*(\})", lambda mm: mm.group(1) + " " + fields + " " + mm.group(2), modtext, count=1)
    ctx.plan.write(f"{root}/{new_dir}/🦀️.rs", modtext)
    block = pick.group("block")
    new_block = block.replace(old_dir, new_dir).replace(pick.group("mod"), new_mod)
    text = text[: pick.end()] + new_block + text[pick.end():]
    text = re.sub(r'(include_str!\("%s/🦀️\.rs"\))' % re.escape(old_dir), lambda mm: mm.group(1) + f', include_str!("{new_dir}/🦀️.rs")', text, count=1)
    km = re.search(r"^(\s*)((?:En\d{4})?%sKey) for (\w+);\n" % re.escape(r.entity.replace("En1994", "")), text, re.M) or re.search(r"^(\s*)(\w*%s\w*Key) for %s;\n" % (re.escape(slug_ref.entity), re.escape(r.entity)), text, re.M)
    if km:
        line = km.group(0)
        text = text[: km.end()] + slug_ref.rename(r.rename(line, n, text), slug_new, text) + text[km.end():]
    ctx.plan.write(main, text)


def nested_family(ctx):
    """🪆 Nested collections in the diff dialects of en1991, en1993, en1995, en1998 and en1999 (donor groups named by the artifact, not composed): the group of the donor child row type, then the group of the donor parent with its `nest { lane: Delta }` link retargeted to the new child group; the parent lane is cloned from the donor parent's lane."""
    S = ctx.spec.collection
    if len(S.nested) != 1 or not ctx.nref:
        raise Unsupported("the nested diff clone needs one child and a reference parent with nested rows")
    donor_child, new_child = ctx.nested_ref_names(), S.nested[0].names
    composite = (Names(ctx.nref.plural_camel + pascal(donor_child.plural_camel), donor_child.entity), Names(S.names.plural_camel + pascal(S.nested[0].plural), new_child.entity))
    saved = dict(ctx.plan.files)
    for attempt in ("entity", "composite", "vec"):
        try:
            if attempt == "entity":
                clone_family_blocks(ctx, ref=donor_child, new=new_child, fields=S.nested[0].fields)
                clone_family_blocks(ctx, ref=ctx.nref, new=S.names, fields=S.fields, child=(donor_child, new_child))
            elif attempt == "composite":
                clone_family_blocks(ctx, ref=composite[0], new=composite[1], fields=S.nested[0].fields)
                clone_family_blocks(ctx, ref=ctx.nref, new=S.names, fields=S.fields, child=[composite, (donor_child, new_child)])
            else:
                clone_family_blocks(ctx, ref=ctx.nref, new=S.names, fields=S.fields, vec_child=(new_child, S.nested[0]))
            break
        except Unsupported as error:
            if "no Key/Patch/Rows/Delta blocks" not in str(error) or attempt == "vec":
                raise
            ctx.plan.files.clear()
            ctx.plan.files.update(saved)
    clone_mention_lines(ctx)


def diff_rs(ctx, prefix):
    import l_nested
    if ctx.spec.collection.nested and l_nested.is_macro_dialect(ctx.plan.read(ctx.layout.p(DIFF + "/🦀️.rs"))):
        l_nested.clone(ctx)
    elif ctx.spec.collection.nested:
        saved = dict(ctx.plan.files)
        try:
            key_and_macros(ctx, prefix)
            lane_arms(ctx)
        except Unsupported:
            ctx.plan.files.clear()
            ctx.plan.files.update(saved)
            nested_family(ctx)
    elif "macro_rules! row_key" in ctx.plan.read(ctx.layout.p(DIFF + "/🦀️.rs")) and re.search(r'#\[path = "[^"]+/🦀️\.rs"\]\nmod \w+;', ctx.plan.read(ctx.layout.p(DIFF + "/🦀️.rs"))):
        diff_modules(ctx)
        for path in diff_tree_files(ctx):
            rel = path[len(ctx.layout.any) + 1:]
            clone_mention_lines(ctx, rel)
    else:
        clone_mention_lines(ctx)
        clone_family_blocks(ctx)
    ctx.plan.note(f"diff Rust: key struct, Patch/Rows/Delta macros and absorb/apply/commit/is_empty/inverse arms for {ctx.spec.collection.plural}")
