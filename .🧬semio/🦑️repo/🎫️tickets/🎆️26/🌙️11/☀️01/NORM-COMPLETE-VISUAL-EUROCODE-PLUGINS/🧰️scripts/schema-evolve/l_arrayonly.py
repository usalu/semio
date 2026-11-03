"""🕳️ Leaf on an array-only entity: a row type whose only scalar cell is the id (en1991 `accidentalCases`: id plus the child blocks `impact`/`explosion`). Its statements write the row with the plain `insert` and read it with the empty float list, so the first scalar leaf turns them into the float-aware forms and shifts the record field ids of the child blocks in the native admission."""
import re
from core import Unsupported, snake
from lex import code_mask, match_close, split_items
import l_codec


def const_name(text, entity, lang):
    """🏷️ Name of the new float-column constant, spelled like the artifact's other constants (`FLOOR_FLOATS` or `FLOOR`)."""
    rx = r"\bconst ([A-Z][A-Z0-9_]*)\s*:\s*" + (r"&\[FloatColumn\]" if lang == "rs" else r"readonly Ieee754Column")
    existing = re.findall(rx, text)
    suffix = "_FLOATS" if existing and sum(n.endswith("_FLOATS") for n in existing) * 2 >= len(existing) else ""
    return snake(entity).upper() + suffix


def add_const(text, name, idx, lang):
    rx = r"^const [A-Z][A-Z0-9_]*\s*:\s*" + (r"&\[FloatColumn\][^\n]*\n" if lang == "rs" else r"readonly Ieee754Column[^\n]*\n")
    found = list(re.finditer(rx, text, re.M))
    if not found:
        raise Unsupported(f"no float-column constant in the {lang} codec to place `{name}` behind")
    if re.search(r"^const %s\s*:" % re.escape(name), text, re.M):
        return text
    line = f"const {name}: &[FloatColumn] = &[FloatColumn::Binary64({idx})];\n" if lang == "rs" else f"const {name}:readonly Ieee754Column[]=[{idx}].map(index=>({{index,width:64}}));\n"
    return text[: found[-1].end()] + line + text[found[-1].end():]


def rust(ctx, names, tbl, new, idx, text, path):
    """🦀 Write: `.insert("tbl", &[..id])` becomes `.insert_float("tbl", &[..id, Cell::Real(row.x)], CONST)`; read: the width and the float list of `entities(..)` and the new member of the struct literal."""
    tc = l_codec.RustCodec(ctx, names, tbl, tbl, [], [new], ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🗄️.sql")))
    tc.text = text
    cname = const_name(text, names.entity, "rs")
    m = re.search(r'\.insert\(\s*"%s"\s*,\s*&\[' % re.escape(tbl), text)
    if not m:
        raise Unsupported(f"write statement of {tbl} (`.insert(\"{tbl}\", &[..])`) not found in the Rust codec")
    mask = code_mask(text, "rs")
    bracket = m.end() - 1
    close = match_close(text, mask, bracket)
    var = re.search(r"Cell::Text\(&(\w+)\.id\)", text[bracket:close])
    if not var:
        raise Unsupported("id cell of the write statement not found")
    item = tc.new_items({}, "rs", None, "W", var.group(1))[0]
    paren = match_close(text, mask, text.index("(", m.start()))
    text = text[:paren] + f", {cname}" + text[paren:]
    text = text[:close] + ", " + item + text[close:]
    text = text[: m.start()] + ".insert_float(" + text[m.start() + len(".insert("):]
    em = re.search(r'(entities\(\s*[^,()]+,\s*"%s"\s*,\s*)(\d+)(\s*,\s*)(&\[\]|[A-Z][A-Z0-9_]*)(\s*,)' % re.escape(tbl), text)
    if not em:
        raise Unsupported(f"read statement `entities(.., \"{tbl}\", width, floats, ..)` not found in the Rust codec")
    text = text[: em.start(2)] + str(int(em.group(2)) + 1) + em.group(3) + cname + em.group(5) + text[em.end():]
    lit = re.search(r"\b%s\s*\{" % re.escape(names.entity), text)
    if not lit:
        raise Unsupported(f"struct literal of {names.entity} not found in the Rust codec")
    mask = code_mask(text, "rs")
    lit_close = match_close(text, mask, lit.end() - 1)
    members = split_items(text, mask, lit.end(), lit_close)
    idm = re.search(r"\bid\s*:[^{}]*?\b(\w+)\.text\(3\)", text[members[0][0]:members[0][1]]) if members else None
    if not idm:
        raise Unsupported("id item of the struct literal not found")
    member = tc.new_items({}, "rs", idx, "R", idm.group(1))[0]
    text = text[: members[0][1]] + ", " + member + text[members[0][1]:]
    return add_const(text, cname, idx, "rs")


def typescript(ctx, names, tbl, new, idx, text, n_tables):
    """🟦 Write: `out.insert("tbl", [..], id)` becomes `write(out,"tbl",id,[.., c.x],CONST,options)`; read: width and floats of `entities(..)`, the new member of the pushed literal and of the row interface."""
    sql = ctx.plan.read(ctx.layout.p(l_codec.SQLITE + "/🗄️.sql"))
    tc = l_codec.TsCodec(ctx, names, tbl, tbl, [], [new], sql, n_tables)
    tc.text = text
    cname = const_name(text, names.entity, "ts")
    m = re.search(r'out\.insert\(\s*"%s"\s*,\s*\[([^\]]*)\]\s*,\s*(\w+)\s*\)' % re.escape(tbl), text)
    if not m:
        raise Unsupported(f"write statement `out.insert(\"{tbl}\", [..], id)` not found in the TypeScript codec")
    var = re.search(r"\b(\w+)\.id\b", m.group(1))
    opts = re.search(r"write\(out,\"\w+\",[^;]*?,\[[^\]]*\],[A-Z][A-Z0-9_]*,(\w+)\)", text)
    if not var or not opts:
        raise Unsupported("row variable or options name of the TypeScript write statements not found")
    item = tc.new_items({}, "ts", None, "W", var.group(1))[0]
    text = text[: m.start()] + f'write(out,"{tbl}",{m.group(2)},[{m.group(1)},{item}],{cname},{opts.group(1)})' + text[m.end():]
    em = re.search(r"(entities\(\s*\w+\[%d\]!?,\s*)(\d+)(\s*,\s*)(\[\]|[A-Z][A-Z0-9_]*)(\s*,)" % n_tables_index(sql, tbl), text)
    if not em:
        raise Unsupported(f"read statement `entities(tables[{n_tables_index(sql, tbl)}]!, width, floats, ..)` not found in the TypeScript codec")
    text = text[: em.start(2)] + str(int(em.group(2)) + 1) + em.group(3) + cname + em.group(5) + text[em.end():]
    pm = re.search(r"\b%s\.push\(\s*\{\s*id\s*:\s*artifactSqliteText\((\w+),3\)" % re.escape(names.plural_camel), text)
    if not pm:
        raise Unsupported(f"`{names.plural_camel}.push({{id: ..}})` not found in the TypeScript codec")
    member = re.sub(r"(readBinary64\(\w+,\d+,)\)", lambda mm: mm.group(1) + cname + ")", tc.new_items({}, "ts", idx, "R", pm.group(1))[0])
    text = text[: pm.end()] + "," + member + text[pm.end():]
    text = re.sub(r"(export interface \w*Sqlite%s\{id:string;)" % re.escape(names.entity), lambda mm: mm.group(1) + f"{new.camel}:Binary64;", text, count=1) if re.search(r"export interface \w*Sqlite%s\{id:string;" % re.escape(names.entity), text) else text
    return add_const(text, cname, idx, "ts")


def n_tables_index(sql, tbl):
    return l_codec.tables_of(sql).index(tbl)


def admission(ctx, names, tbl, new, f, text, cname):
    """🛂 Native admission: the entity's cell list gets the new cell and its float list the constant; the record field ids of the child blocks behind the id shift by one (in the row loop and in the row counter)."""
    import l_sqlite
    m = re.search(r'(n::entity\(out,"%s",[^\n]*?n::cells\((\w+),\[)(Text\(0\)[^\]]*)(\]\)\?,)(&\[\]|[A-Z][A-Z0-9_]*)(\))' % re.escape(tbl), text)
    if not m:
        raise Unsupported(f"admission statement of {tbl} not found")
    cells = m.group(3)
    count = cells.count("(")
    new_cell = l_sqlite.adm_cell(f, count)
    text = text[: m.end(3)] + "," + new_cell + text[m.end(3):m.start(5)] + cname + text[m.end(5):]
    row = m.group(2)
    rootm = re.search(r"for\(index,(\w+)\)in n::list\(n::field\(root,(\d+)\)\?\)\?\.iter\(\)\.enumerate\(\)\{[^\n]*n::entity\(out,\"%s\"" % re.escape(tbl), text)
    if not rootm:
        raise Unsupported("root field of the admission row loop not found")
    root_field = rootm.group(2)
    start = rootm.start()
    end = start
    depth = 0
    seen = False
    i = start
    while i < len(text):
        depth += text[i] == "{"
        depth -= text[i] == "}"
        seen = seen or text[i] == "{"
        if seen and depth == 0:
            end = i
            break
        i += 1
    block = text[start:end]
    shifted = re.sub(r"(n::field\(%s,)(\d+)(\))" % re.escape(row), lambda mm: mm.group(1) + str(int(mm.group(2)) + 1) + mm.group(3), block)
    text = text[:start] + shifted + text[end:]
    counter = re.search(r"for \w+ in n::list\(n::field\(root,%s\)\?\)\?\{let (\w+)=n::record\(\w+\)\?;for id in (\d+)\.\.=(\d+)\{" % root_field, text)
    if counter:
        text = text[: counter.start(2)] + str(int(counter.group(2)) + 1) + ".." + "=" + str(int(counter.group(3)) + 1) + text[counter.end(3):]
    return text
