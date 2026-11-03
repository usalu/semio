"""🗄️ SQLite layer: DDL, Rust and TypeScript codecs, native admission and the recomputed semantic contract (replayed with python `sqlite3`) of the new tables, for artifacts whose snapshot is stored in positional SQLite tables."""
import json, re, sqlite3
from core import Unsupported, balanced_end, line_span, pascal, snake, sub1
from spec import INT_MAX

SQLITE = "🚪️io/🪶️sqlite/📸️snapshot"


def tbl(ctx, ent):
    return f"{ctx.short}_{ent.names.snake}"


def ddl(ctx, ent, parent=None):
    """🪺 DDL text of one entity table (parent: the entity the foreign key points at, None for the document)."""
    fk = f"{ctx.short}_{parent.names.snake}_id INTEGER NOT NULL REFERENCES {ctx.short}_{parent.names.snake}(id)" if parent else f"document_id INTEGER NOT NULL REFERENCES {ctx.short}_document(id)"
    fk = fk.replace(f"{ctx.short}_{parent.names.snake}_id", f"{parent.names.snake}_id") if parent else fk
    cols = [f"id INTEGER PRIMARY KEY, {fk}, ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL"]
    for f in ent.fields:
        if f.ty == "String":
            cols.append(f"{f.name} TEXT NOT NULL")
        elif f.ty == "enum":
            cols.append(f"{f.name} TEXT NOT NULL CHECK({f.name} IN ({','.join(repr(t) for t in f.tokens)}))")
        elif f.ty in INT_MAX:
            cols.append(f"{f.name} INTEGER NOT NULL CHECK({f.name} BETWEEN 0 AND {INT_MAX[f.ty]})")
        else:
            cols.append(f"{f.name} REAL")
    reals = [f for f in ent.fields if f.is_real]
    head = " " + ", ".join(cols) + ("," if reals else "")
    tail = ",\n".join(f" {f.name}_ieee754_bits INTEGER NOT NULL, {f.name}_ieee754_class TEXT NOT NULL" for f in reals)
    return f"CREATE TABLE {tbl(ctx, ent)} (\n{head}\n" + (tail + "\n" if tail else "") + ");\n"


def width(ent):
    return 4 + len(ent.fields)


def floats(ent):
    return [4 + i for i, f in enumerate(ent.fields) if f.is_real]


def const_of(ent):
    return ent.entity.upper() if floats(ent) else "&[]"


def const_ts(ent):
    return ent.entity.upper() if floats(ent) else "[]"


def real_cols(ent):
    return ",".join(f"FloatColumn::Binary64({i})" for i in floats(ent))


def rs_cell(f, v="v"):
    if f.ty == "String":
        return f"Cell::Text(&{v}.{f.name})"
    if f.ty == "f64":
        return f"Cell::Real({v}.{f.name})"
    if f.ty == "enum":
        return f"Cell::Text({f.enum_snake}_text({v}.{f.name}))"
    return f"Cell::Integer(i64::from({v}.{f.name}))"


def rs_read(f, i, var="v"):
    if f.ty == "String":
        return f"{f.name}:text({var},{i},c)?"
    if f.ty == "f64":
        return f"{f.name}:{var}.real({i})?"
    if f.ty == "enum":
        return f"{f.name}:read_{f.enum_snake}(&text({var},{i},c)?)?"
    return f"{f.name}:{f.ty}::try_from({var}.integer({i})?).map_err(|e| invalid(e.to_string()))?"


def ts_read(f, i, const, var="v"):
    if f.ty == "String":
        return f"{f.camel}:artifactSqliteText({var},{i})"
    if f.ty == "f64":
        return f"{f.camel}:readBinary64({var},{i},{const})"
    if f.ty == "enum":
        return f"{f.camel}:choice(artifactSqliteText({var},{i}),{f.enum_const}S)"
    return f"{f.camel}:readUnsigned({var},{i},{INT_MAX[f.ty]}n)"


def ts_write(f, var="v"):
    if f.ty == "enum":
        return f"choice({var}.{f.camel},{f.enum_const}S)"
    return f"unsigned({var}.{f.camel},{INT_MAX[f.ty]})" if f.ty in INT_MAX else f"{var}.{f.camel}"


def adm_cell(f, i):
    if f.ty == "String":
        return f"Text({i})"
    if f.ty == "f64":
        return f"Real({i})"
    if f.ty == "enum":
        return f"Text({i})"
    return f"Unsigned({i},{f.ty}::MAX as u64)"


def enum_fns(f):
    arms_t = "".join(f'{f.enum_name}::{v}=>"{t}",' for v, t in zip(f.variants, f.tokens))
    arms_r = "".join(f'"{t}"=>Ok({f.enum_name}::{v}),' for v, t in zip(f.variants, f.tokens))
    return (f"fn {f.enum_snake}_text(a:{f.enum_name})->&'static str{{match a{{{arms_t}}}}}\n"
            f"fn read_{f.enum_snake}(a:&str)->Result<{f.enum_name}, ValueError>{{match a{{{arms_r}_=>Err(invalid(\"{ctx_label(f)} differs\"))}}}}\n")


def ctx_label(f):
    return "EN lab test kind".replace("EN lab test kind", f.enum_snake.replace("_", " "))


def tables_of(ctx):
    return [(ctx.spec.collection, None)] + [(n, ctx.spec.collection) for n in ctx.spec.collection.nested]


def run_ddl(ctx):
    path = ctx.layout.p(SQLITE + "/🗄️.sql")
    text = ctx.plan.read(path)
    if re.search(r"CREATE TABLE\s+%s\b" % re.escape(tbl(ctx, ctx.spec.collection)), text):
        return
    add = "".join(ddl(ctx, ent, parent) for ent, parent in tables_of(ctx))
    ctx.plan.write(path, text.rstrip("\n") + "\n" + add)


def run_rust(ctx):
    S, short = ctx.spec.collection, ctx.short
    path = ctx.layout.p(SQLITE + "/🦀️.rs")
    text = ctx.plan.read(path)
    if f'"{tbl(ctx, S)}"' in text:
        return
    n_tables = len(re.findall(r"CREATE TABLE", ctx.plan.read(ctx.layout.p(SQLITE + "/🗄️.sql")))) - len(tables_of(ctx))
    names = [S.entity] + [n.entity for n in S.nested] + [f.enum_name for f in S.all_enums]
    text = sub1(text, r"(use crate::\{[^}]*)\};", lambda m: m.group(1) + "".join(", " + n for n in names if not re.search(r"\b%s\b" % n, m.group(1))) + "};", what="use crate::{..} list")
    for f in S.all_enums:
        if f"fn {f.enum_snake}_text" not in text:
            lines = list(re.finditer(r"^fn read_\w+\(a:&str\)[^\n]*\n", text, re.M))
            end = lines[-1].end()
            text = text[:end] + enum_fns(f) + text[end:]
    consts = list(re.finditer(r"^const \w+:&\[FloatColumn\]=[^\n]*\n", text, re.M))
    if not consts:
        raise Unsupported("no FloatColumn constants in the sqlite Rust codec")
    add = "".join(f"const {const_of(e)}:&[FloatColumn]=&[{real_cols(e)}];\n" for e, _ in tables_of(ctx) if floats(e))
    text = text[: consts[-1].end()] + add + text[consts[-1].end():]
    cells = lambda e, v: ",".join(rs_cell(f, v) for f in e.fields)
    write = ""
    if S.nested:
        n = S.nested[0]
        inner = f'for(k,a)in v.{n.names.plural_snake}.iter().enumerate(){{p.insert_float("{tbl(ctx, n)}",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),{cells(n, "a")}],{const_of(n)})?;}}'
        write = f'  for(n,v)in self.{S.names.plural_snake}.iter().enumerate(){{let id=p.insert_float("{tbl(ctx, S)}",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&v.id),{cells(S, "v")}],{const_of(S)})?;{inner}}}\n'
    else:
        write = f'  for(n,v)in self.{S.names.plural_snake}.iter().enumerate(){{p.insert_float("{tbl(ctx, S)}",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&v.id),{cells(S, "v")}],{const_of(S)})?;}}\n'
    anchor = ctx.anchor_snake
    m = re.search(r"^  for\(n,[a-z]\)in self\.%s\.iter\(\)\.enumerate\(\)[^\n]*\n" % anchor, text, re.M)
    if not m:
        raise Unsupported(f"write loop of `{anchor}` not found")
    end = m.end()
    while not re.match(r"  Ok\(\(\)\)", text[end:]):
        end = text.index("\n", end) + 1
        if end > len(text) - 5:
            raise Unsupported("end of write_sqlite_rows not found")
    text = text[:end] + write + text[end:]
    text = sub1(text, r"(for count in\[[^\]]*self\.%s\.len\(\))\]" % anchor, lambda m: m.group(1) + f",self.{S.names.plural_snake}.len()]", what="preflight row count list")
    nest_clone = ""
    if S.nested:
        n = S.nested[0]
        nest_loop = re.search(r"for v in &self\.\w+\{admit_rows\(&mut b,&mut rows,v\.%s\.len\(\)\)\?;\}" % ctx.nested_ref_names().plural_snake, text)
        if not nest_loop:
            raise Unsupported("nested admit_rows loop of the reference")
        text = text[: nest_loop.end()] + f"for v in &self.{S.names.plural_snake}{{admit_rows(&mut b,&mut rows,v.{n.names.plural_snake}.len())?;}}" + text[nest_loop.end():]
    strings = "".join(f"b.repeated(v.{f.name}.len(),24)?;" for f in S.fields if f.ty == "String")
    nested_b = "".join(f"for a in &v.{n.names.plural_snake}{{b.add(8192)?;b.repeated(a.id.len(),24)?;}}" for n in S.nested)
    bound = f"for v in &self.{S.names.plural_snake}{{b.add(32768)?;b.repeated(v.id.len(),24)?;{strings}{nested_b}}}"
    text = sub1(text, r"(for v in &self\.%s\{b\.add\(\d+\)\?;[^\n]*?\}(?:\}|)(?=for v in &self\.%s\{b\.add))" % (ctx.ref.plural_snake, ctx.after_ref_snake), lambda m: m.group(1) + bound, what="byte bound loop after the reference")
    first = 0
    if S.nested:
        n = S.nested[0]
        reads_n = ",".join([f"id:text(a,3,c)?"] + [rs_read(f, 4 + i, "a") for i, f in enumerate(n.fields)])
        head = f'  let {S.names.plural_snake}_rows=entities(d,"{tbl(ctx, S)}",{width(S)},{const_of(S)},c)?;let mut {n.names.plural_snake}_{S.names.plural_snake}=groups(entities(d,"{tbl(ctx, n)}",{width(n)},{const_of(n)},c)?,&{S.names.plural_snake}_rows,c)?;\n'
        reads_s = ",".join([f"id:text(v,3,c)?"] + [rs_read(f, 4 + i) for i, f in enumerate(S.fields)])
        loop = (f'  let mut {S.names.plural_snake}=Vec::new();for v in ordered({S.names.plural_snake}_rows.into_values(),1,c)?{{let mut {n.names.plural_snake}=Vec::new();for a in ordered({n.names.plural_snake}_{S.names.plural_snake}.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{{checkpoint(c,{n.names.plural_snake}.len(),0)?;{n.names.plural_snake}.push({n.entity}{{{reads_n}}});}}{S.names.plural_snake}.push({S.entity}{{{reads_s},{n.names.plural_snake}}});}}\n')
        block = head + loop
    else:
        reads_s = ",".join([f"id:text(v,3,c)?"] + [rs_read(f, 4 + i) for i, f in enumerate(S.fields)])
        block = f'  let mut {S.names.plural_snake}=Vec::new();for v in ordered(entities(d,"{tbl(ctx, S)}",{width(S)},{const_of(S)},c)?.into_values(),1,c)?{{checkpoint(c,{S.names.plural_snake}.len(),0)?;{S.names.plural_snake}.push({S.entity}{{{reads_s}}});}}\n'
    at = text.index("  let result=Self{")
    text = text[:at] + block + text[at:]
    text = sub1(text, r"(let result=Self\{[^\n]*?,%s)(\};c\.checkpoint)" % anchor, lambda m: m.group(1) + f",{S.names.plural_snake}" + m.group(2), what="result struct")
    text = re.sub(r"(SQLITE_SCHEMA,)%d(,)" % n_tables, lambda m: m.group(1) + str(n_tables + len(tables_of(ctx))) + m.group(2), text)
    ctx.plan.write(path, text)


def run_ts(ctx):
    S = ctx.spec.collection
    path = ctx.layout.p(SQLITE + "/🟦️.ts")
    text = ctx.plan.read(path)
    if f'"{tbl(ctx, S)}"' in text:
        return
    n_tables = len(re.findall(r"CREATE TABLE", ctx.plan.read(ctx.layout.p(SQLITE + "/🗄️.sql")))) - len(tables_of(ctx))
    names = [S.entity] + [n.entity for n in S.nested] + [f.enum_name for f in S.all_enums]
    text = sub1(text, r"(import type\{[^}]*)\}from", lambda m: m.group(1) + "".join("," + n for n in names if not re.search(r"\b%s\b" % n, m.group(1))) + "}from", what="TS import type list")
    up = ctx.upper

    def sql_type(e, nested=None):
        reals = [f for f in e.fields if f.is_real]
        omit = [f'"{f.camel}"' for f in reals] + [f'"{camel_of(n)}"' for n in e.nested]
        body = [f"{f.camel}:Binary64" for f in reals] + [f"{camel_of(n)}:{up}Sqlite{n.entity}[]" for n in e.nested]
        if not omit:
            return f"export type {up}Sqlite{e.entity}={e.entity};\n"
        return f"export type {up}Sqlite{e.entity}=Omit<{e.entity},{'|'.join(omit)}>&{{{';'.join(body)}}};\n"
    types = "".join(sql_type(n) for n in S.nested) + sql_type(S)
    text = sub1(text, r"(export type %sSqlite\w+=[^\n]*\n)(?=/\*\*[^\n]*\*/\nexport type %sSqliteCaseNumber|export type %sSqliteSnapshot)" % (up, up, up), lambda m: m.group(1) + types, what="TS entity type declarations")
    text = sub1(text, r"(export type %sSqliteSnapshot=\{[^\n]*?),?\};" % up, lambda m: m.group(1) + f";{S.camel_plural}:{up}Sqlite{S.entity}[]}};", what="TS snapshot type")
    consts = list(re.finditer(r"^const \w+:readonly Ieee754Column\[\]=[^\n]*\n", text, re.M))
    add = "".join(f"const {const_ts(e)}:readonly Ieee754Column[]=[{','.join(map(str, floats(e)))}].map(index=>({{index,width:64}}));\n" for e, _ in tables_of(ctx) if floats(e))
    text = text[: consts[-1].end()] + add + text[consts[-1].end():]
    for f in S.all_enums:
        if f"const {f.enum_const}S:" not in text:
            kinds = list(re.finditer(r"^const \w+S:readonly \w+\[\]=\[[^\n]*\n", text, re.M))
            if not kinds:
                raise Unsupported("no enum choice constants in the sqlite TS codec")
            line = f"const {f.enum_const}S:readonly {f.enum_name}[]=[{','.join(chr(34) + t + chr(34) for t in f.tokens)}];\n"
            text = text[: kinds[-1].end()] + line + text[kinds[-1].end():]
    anchor_c = ctx.anchor
    text = sub1(text, r'(fields\(s,\[[^\]]*"%s")\]\)' % anchor_c, lambda m: m.group(1) + f',"{S.camel_plural}"])', what="root fields list")
    text = sub1(text, r"(,s\.%s)\]\)collection\(v\)" % anchor_c, lambda m: m.group(1) + f",s.{S.camel_plural}])collection(v)", what="collection list")
    cells = lambda e, var: ",".join(ts_write(f, var) for f in e.fields)
    names_s = ",".join(f'"{x}"' for x in ["id"] + [f.camel for f in S.fields] + [camel_of(n) for n in S.nested])
    if S.nested:
        n = S.nested[0]
        names_n = ",".join(f'"{x}"' for x in ["id"] + [f.camel for f in n.fields])
        inner = f'for(let k=0;k<v.{camel_of(n)}.length;k++){{const a=v.{camel_of(n)}[k]!;fields(a,[{names_n}]);await write(out,"{tbl(ctx, n)}",[id,BigInt(k),a.id,{cells(n, "a")}],{const_ts(n)},o)}}'
        write = f' for(let n=0;n<s.{S.camel_plural}.length;n++){{const v=s.{S.camel_plural}[n]!;fields(v,[{names_s}]);collection(v.{camel_of(n)});const id=await write(out,"{tbl(ctx, S)}",[1n,BigInt(n),v.id,{cells(S, "v")}],{const_ts(S)},o);{inner}}}\n'
    else:
        write = f' for(let n=0;n<s.{S.camel_plural}.length;n++){{const v=s.{S.camel_plural}[n]!;fields(v,[{names_s}]);await write(out,"{tbl(ctx, S)}",[1n,BigInt(n),v.id,{cells(S, "v")}],{const_ts(S)},o)}}\n'
    m = re.search(r"^ for\(let n=0;n<s\.%s\.length;n\+\+\)[^\n]*\n" % anchor_c, text, re.M)
    if not m:
        raise Unsupported(f"write loop of `{anchor_c}` not found in the sqlite TS codec")
    text = text[: m.end()] + write + text[m.end():]
    ti = n_tables
    if S.nested:
        n = S.nested[0]
        reads_n = ",".join(["id:artifactSqliteText(a,3)"] + [ts_read(f, 4 + i, const_ts(n), "a") for i, f in enumerate(n.fields)])
        reads_s = ",".join(["id:artifactSqliteText(v,3)"] + [ts_read(f, 4 + i, const_ts(S)) for i, f in enumerate(S.fields)])
        read = (f' const {S.camel_plural}Rows=await entities(t[{ti}]!,{width(S)},{const_ts(S)},o),{n.camel_plural}By{S.entity}=await groups(await entities(t[{ti + 1}]!,{width(n)},{const_ts(n)},o),{S.camel_plural}Rows,o);\n'
                f' for(const v of await ordered({S.camel_plural}Rows.values(),1n,o)){{const {camel_of(n)}:{up}Sqlite{n.entity}[]=[];for(const a of await ordered({n.camel_plural}By{S.entity}.get(v.rowid)??[],v.rowid,o)){{await checkpoint(o,{camel_of(n)}.length);{camel_of(n)}.push({{{reads_n}}})}}{S.camel_plural}.push({{{reads_s},{camel_of(n)}}})}}\n')
    else:
        reads_s = ",".join(["id:artifactSqliteText(v,3)"] + [ts_read(f, 4 + i, const_ts(S)) for i, f in enumerate(S.fields)])
        read = f' for(const v of await ordered((await entities(t[{ti}]!,{width(S)},{const_ts(S)},o)).values(),1n,o)){{await checkpoint(o,{S.camel_plural}.length);{S.camel_plural}.push({{{reads_s}}})}}\n'
    at = text.index(' await artifactSqliteCheckpoint(o,"reconstructSnapshot"')
    text = text[:at] + read + text[at:]
    text = sub1(text, r"(,%s:%sSqlite\w+\[\]=\[\])(;)" % (anchor_c, up), lambda m: m.group(1) + f",{S.camel_plural}:{up}Sqlite{S.entity}[]=[]" + m.group(2), what="TS array declarations")
    text = sub1(text, r"(return\{structureId:[^\n]*?,%s)\};" % anchor_c, lambda m: m.group(1) + f",{S.camel_plural}}};", what="TS result object") if re.search(r"return\{structureId:", text) else text
    ctx.plan.write(path, text)


def camel_of(n):
    return n.names.plural_camel


def run_admission(ctx):
    S = ctx.spec.collection
    path = ctx.layout.p(SQLITE + "/🛂️admission/🦀️.rs")
    if not ctx.plan.exists(path):
        raise Unsupported("the artifact has no native admission file for the template path")
    text = ctx.plan.read(path)
    if f'"{tbl(ctx, S)}"' in text:
        return
    props = list(ctx.snapshot["properties"])
    field_id = len(props)
    n_tables = len(re.findall(r"CREATE TABLE", ctx.plan.read(ctx.layout.p(SQLITE + "/🗄️.sql")))) - len(tables_of(ctx))
    def cell_list(e):
        return "Text(0)" + "".join("," + adm_cell(f, i + 1) for i, f in enumerate(e.fields))
    if S.nested:
        n = S.nested[0]
        k = 1 + len(S.fields)
        line = (f' for(index,v)in n::list(n::field(root,{field_id})?)?.iter().enumerate(){{let v=n::record(v)?;let parent=n::entity(out,"{tbl(ctx, S)}",1,index,&n::cells(v,[{cell_list(S)}])?,{const_of(S)})?;\n'
                f' for(index,a)in n::list(n::field(v,{k})?)?.iter().enumerate(){{n::entity(out,"{tbl(ctx, n)}",parent,index,&n::cells(n::record(a)?,[{cell_list(n)}])?,{const_of(n)})?;}}}}\n')
    else:
        line = f' for(index,v)in n::list(n::field(root,{field_id})?)?.iter().enumerate(){{n::entity(out,"{tbl(ctx, S)}",1,index,&n::cells(n::record(v)?,[{cell_list(S)}])?,{const_of(S)})?;}}\n'
    at = re.compile(r"^ ?Ok\(\(\)\)\n\}", re.M).search(text, text.index("fn write(")).start()
    text = text[:at] + line + text[at:]
    text = sub1(text, r"for id in (\d+)\.\.=(\d+)\{", lambda m: f"for id in {m.group(1)}..={field_id}{{", what="row count range")
    if S.nested:
        text = sub1(text, r"(for\(id,nested\)in\[[^\]]*)\]", lambda m: m.group(1) + f",({field_id},{1 + len(S.fields)})]", what="nested row pairs")
    text = re.sub(r"(SQLITE_SCHEMA,)%d(,)" % n_tables, lambda m: m.group(1) + str(n_tables + len(tables_of(ctx))) + m.group(2), text)
    ctx.plan.write(path, text)


def replay(sql):
    db = sqlite3.connect(":memory:")
    db.executescript(sql)
    rows = db.execute("select name, sql from sqlite_schema where type='table' order by name").fetchall()
    widths = {n: len(db.execute(f"pragma table_info({n})").fetchall()) for n, _ in rows}
    return sum(len(n.encode()) + len(q.encode()) for n, q in rows), dict(sorted(widths.items()))


def run_contract(ctx):
    S = ctx.spec.collection
    L = ctx.layout
    sql_path = L.p(SQLITE + "/🗄️.sql")
    new_sql = ctx.plan.read(sql_path)
    old_sql = new_sql
    for ent, parent in reversed(tables_of(ctx)):
        old_sql = old_sql.replace(ddl(ctx, ent, parent), "")
    sem_path = L.p(SQLITE + "/🧪️tests/🧫️fixtures/🎛️semantic.json")
    sem = json.loads(ctx.plan.read(sem_path))
    old_bytes, old_widths = replay(old_sql)
    new_bytes, new_widths = replay(new_sql)
    stored = (sem["schemaBytes"], sem["tableWidths"])
    if sem["schemaBytes"] == new_bytes and sem["tableWidths"] == new_widths:
        return
    stale = stored != (old_bytes, old_widths)
    sem["schemaBytes"], sem["tableWidths"] = new_bytes, new_widths
    ctx.plan.write(sem_path, json.dumps(sem, indent=2, ensure_ascii=False) + "\n")
    ts_path = L.p(SQLITE + "/🧪️tests/🟦️.ts")
    text = ctx.plan.read(ts_path)
    text = re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+', lambda m: m.group(1) + str(new_bytes), text)
    text = re.sub(r'(normSemanticContract\["tableWidths"\]\)\.toEqual\()\{[^}]*\}', lambda m: m.group(1) + json.dumps(new_widths, separators=(",", ":")), text)
    ctx.plan.write(ts_path, text)
    ctx.plan.note(f"contract replayed with python sqlite3: schemaBytes {old_bytes} -> {new_bytes}, {len(new_widths)} tables" + (f" (the stored value {stored[0]} before this change was stale; per-case valueBytes are not recomputed)" if stale else ""))


def run(ctx):
    S = ctx.spec.collection
    ctx.anchor_snake = ctx.anchor and snake(ctx.anchor)
    colls = [c[0] for c in ctx.collections]
    ctx.after_ref_snake = snake(colls[colls.index(ctx.ref.plural_camel) + 1]) if colls.index(ctx.ref.plural_camel) + 1 < len(colls) else None
    S.camel_plural = S.names.plural_camel
    for n in S.nested:
        n.camel_plural = n.names.plural_camel
    saved = dict(ctx.plan.files)
    try:
        for fn in (run_ddl, run_rust, run_ts, run_admission, run_contract):
            fn(ctx)
    except (Unsupported, sqlite3.OperationalError):
        ctx.plan.files.clear()
        ctx.plan.files.update(saved)
        import l_codec
        l_codec.run_collection(ctx)
