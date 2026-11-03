"""🪆 Nested collections in the macro dialect of the diff Rust (`scalar_patch!`, `nested_patch!`, `list_delta!`, `row_key!`: en1992, en1994): the parent row group and the child row group of a reference parent are cloned together, the child lane replaces the donor's lanes in the parent's `rows` section, and every new row type gets its `row_key!` entry."""
import re
from core import DIFF, Names, Unsupported, pascal
import l_patch
import l_rust

BLOCK_START = re.compile(r"^(?:scalar_patch|nested_patch|protocol::list_delta)! \{\n", re.M)
KEY_LIST = re.compile(r"^row_key! \{\n((?:.*\n)*?)\}\n", re.M)


def is_macro_dialect(text):
    return "macro_rules! nested_patch" in text and "macro_rules! row_key" in text


def macro_blocks(text):
    """🧱 (start, end, macro name) of every top-level `name! { .. }` call, `end` after the closing brace line."""
    out = []
    for m in BLOCK_START.finditer(text):
        end = text.index("\n}\n", m.end()) + 3
        out.append((m.start(), end, m.group(0).split("!")[0]))
    return out


def family(text, parent, child):
    """🧬 The blocks of the donor parent group: child patch + child rows, parent patch + parent rows (spans in text order)."""
    wanted = {
        "child": (f"{parent.plural_pascal}{child.plural_pascal}Patch", f"{parent.plural_pascal}{child.plural_pascal}Rows"),
        "parent": (f"{parent.plural_pascal}Patch", f"{parent.plural_pascal}Rows"),
    }
    found = {}
    for start, end, macro in macro_blocks(text):
        body = text[start:end]
        for role, names in wanted.items():
            for n in names:
                if re.search(r"\b\w*%s\b" % re.escape(n), body) and re.search(r"(?:^|\s)(?:pub\s+)?\w*%s(?:\s+for\s|\s+\{)" % re.escape(n), body):
                    found[(role, n.endswith("Patch"))] = (start, end)
    if len(found) != 4:
        raise Unsupported(f"nested donor {parent.plural_camel}/{child.plural_camel}: found {len(found)} of 4 blocks (child and parent Patch and Rows)")
    return found


def fields_text(ctx, fields):
    model = ctx.layout.model_crate + "::"
    return ", ".join(f"{f.name}: {model + f.enum_name if f.ty == 'enum' else f.rust_ty}" for f in fields)


def row_key_entries(text, entries):
    """🔑 `<NewKey> for <NewRow>;` behind the donor's line of the `row_key!` list for every (donor key, new key, new row) in `entries` (skipped when present)."""
    m = KEY_LIST.search(text)
    if not m:
        raise Unsupported("no row_key! list in the diff Rust")
    body = m.group(1)
    for donor, key, row in entries:
        if re.search(r"^\s*%s for \w+;" % re.escape(key), body, re.M):
            continue
        line = re.search(r"^(\s*)%s for \w+;\n" % re.escape(donor), body, re.M)
        if not line:
            raise Unsupported(f"row_key! entry of the donor key {donor} not found")
        entry = f"{line.group(1)}{key} for {row};\n"
        body = body[: line.end()] + entry + body[line.end():]
    return text[: m.start(1)] + body + text[m.end(1):]


def locate(ctx, parent, child):
    """🧭 The diff file holding the four blocks of the donor group (the main file, or one of its row module files)."""
    last = None
    for path in [ctx.layout.p(DIFF + "/🦀️.rs")] + [p for p in l_rust.diff_tree_files(ctx) if not p.endswith(DIFF + "/🦀️.rs")]:
        text = ctx.plan.read(path)
        if "list_delta!" not in text:
            continue
        try:
            return path, text, family(text, parent, child)
        except Unsupported as error:
            last = error
    raise last or Unsupported("no diff file holds the donor group")


def clone(ctx):
    S, L = ctx.spec.collection, ctx.layout
    main = L.p(DIFF + "/🦀️.rs")
    if any(f"{S.names.plural_pascal}Patch for" in ctx.plan.read(p) for p in l_rust.diff_tree_files(ctx)):
        return
    parent = ctx.nref
    child = ctx.nested_ref_names()
    new_child = S.nested[0].names
    path, text, found = locate(ctx, parent, child)
    order = sorted(found.items(), key=lambda kv: kv[1][0])
    last = max(end for _, end in found.values())
    add, entries = "", []
    for (role, _is_patch), (start, end) in order:
        original = text[start:end]
        chunk = original.replace(parent.plural_pascal + child.plural_pascal, S.names.plural_pascal + new_child.plural_pascal)
        chunk = child.rename(chunk, new_child, text)
        chunk = parent.rename(chunk, S.names, text)
        if role == "child":
            fields = fields_text(ctx, S.nested[0].fields)
            if chunk.startswith("scalar_patch"):
                chunk = re.sub(r"(for [\w:]+ \{)[^}\n]*(\})", lambda m: f"{m.group(1)} {fields} {m.group(2)}", chunk, count=1)
        elif chunk.startswith("nested_patch"):
            chunk = re.sub(r"set \{[^}]*\}", lambda m: "set { %s }" % fields_text(ctx, S.fields), chunk, count=1)
            chunk = re.sub(r"(within|optional) \{[^}]*\}", lambda m: f"{m.group(1)} {{  }}", chunk)
        donor_key = re.search(r"\bby (\w+Key)\b", original)
        if donor_key:
            row = (new_child if role == "child" else S.names).entity
            key = (re.match(r"En\d{4}", donor_key.group(1)) or re.match("", "")).group(0) + row + "Key"
            chunk = re.sub(r"\bby \w+Key\b", "by " + key, chunk)
            entries.append((donor_key.group(1), key, row))
        add += "\n" + chunk
    text = text[:last] + add + text[last:]
    ctx.plan.write(path, l_patch.normalize(text))
    main_text = ctx.plan.read(main)
    ctx.plan.write(main, row_key_entries(main_text, entries))
    saved = ctx.ref
    ctx.ref = parent
    try:
        for p in l_rust.diff_tree_files(ctx):
            l_rust.clone_mention_lines(ctx, p[len(L.any) + 1:])
    finally:
        ctx.ref = saved
    ctx.plan.note(f"diff Rust (macro dialect): child and parent Patch/Rows blocks cloned from {parent.plural_camel}/{child.plural_camel} in {ctx.plan.rel(path).rsplit('/', 2)[-2]}, row_key! entries " + ", ".join(k for _, k, _ in entries))
