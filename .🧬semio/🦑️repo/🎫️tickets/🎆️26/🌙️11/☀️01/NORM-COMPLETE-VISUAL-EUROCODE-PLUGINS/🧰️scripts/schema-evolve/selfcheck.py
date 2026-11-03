"""🔍 Reconstruction check: rebuilds the entity spec of an EXISTING collection from the snapshot schema, runs the field-dependent generators on it and requires their output to occur verbatim in the committed files (the repository is its own oracle for the generators)."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from core import Layout, camel, pascal, snake
from ctx import Ctx
from spec import Entity, Spec
import l_sqlite


PLACE = {"en": "-", "de": "-"}


def entity_from_schema(ctx, plural, entity, nested_plural=None):
    d = ctx.defs[entity]
    fields, nested = [], []
    for name, node in d["properties"].items():
        if name == "id":
            continue
        if node.get("type") == "array" and node.get("items", {}).get("$ref", "").startswith("#/$defs/"):
            n_ent = node["items"]["$ref"].rsplit("/", 1)[1]
            nested.append(entity_from_schema(ctx, name, n_ent))
            continue
        label = node.get("x-semio-ui", {}).get("label") or PLACE
        raw = {"name": snake(name), "label": label}
        if "$ref" in node:
            e = ctx.defs[node["$ref"].rsplit("/", 1)[1]]
            raw.update({"type": "enum", "enum": {"name": e["title"], "values": [{"token": t, "label": {"en": t, "de": t}} for t in e["enum"]]}})
        elif node["type"] == "string":
            raw["type"] = "String"
        elif node["type"] == "integer":
            raw["type"] = "u8" if node.get("maximum") == 255 else "u32"
        else:
            raw["type"] = "f64"
        fields.append(raw)
    return Entity({"name": plural, "entity": entity, "doc": "", "label": PLACE, "id": PLACE, "singular": PLACE, "fields": fields}, plural)


def main(short, plural):
    spec = Spec.__new__(Spec)
    spec.reference = spec.nested_reference = None
    spec.collection = None
    ctx = Ctx(short, spec)
    entity = dict(ctx.collections)[plural]
    ent = entity_from_schema(ctx, plural, entity)
    ent_parent = ent
    sql = ctx.plan.read(ctx.layout.p(l_sqlite.SQLITE + "/🗄️.sql"))
    rs = ctx.plan.read(ctx.layout.p(l_sqlite.SQLITE + "/🦀️.rs"))
    ts = ctx.plan.read(ctx.layout.p(l_sqlite.SQLITE + "/🟦️.ts"))
    adm = ctx.plan.read(ctx.layout.p(l_sqlite.SQLITE + "/🛂️admission/🦀️.rs"))
    bad = 0

    def need(label, text, where):
        nonlocal bad
        ok = text in where
        bad += not ok
        print(("ok   " if ok else "FAIL ") + label + ("" if ok else "\n     " + text[:300]))
    need("DDL " + entity, l_sqlite.ddl(ctx, ent), sql)
    for n in ent.nested:
        need("DDL nested " + n.entity, l_sqlite.ddl(ctx, n, ent), sql)
    need("float const", f"const {l_sqlite.const_of(ent)}:&[FloatColumn]=&[{l_sqlite.real_cols(ent)}];", rs)
    need("rs write cells", ",".join(l_sqlite.rs_cell(f, "v") for f in ent.fields), rs)
    need("rs read cells", ",".join(l_sqlite.rs_read(f, 4 + i) for i, f in enumerate(ent.fields)), rs)
    need("ts read cells", ",".join(l_sqlite.ts_read(f, 4 + i, l_sqlite.const_of(ent)) for i, f in enumerate(ent.fields)), ts)
    need("ts write cells", ",".join(l_sqlite.ts_write(f, "v") for f in ent.fields), ts)
    need("admission cells", "Text(0)" + "".join("," + l_sqlite.adm_cell(f, i + 1) for i, f in enumerate(ent.fields)), adm)
    for f in ent.enums:
        need("rs enum fns " + f.enum_name, l_sqlite.enum_fns(f).split("\n")[0], rs)
        need("ts enum const " + f.enum_name, f"const {f.enum_const}S:readonly {f.enum_name}[]=[{','.join(chr(34) + t + chr(34) for t in f.tokens)}];", ts)
    return bad


if __name__ == "__main__":
    sys.exit(1 if main(sys.argv[1], sys.argv[2]) else 0)
