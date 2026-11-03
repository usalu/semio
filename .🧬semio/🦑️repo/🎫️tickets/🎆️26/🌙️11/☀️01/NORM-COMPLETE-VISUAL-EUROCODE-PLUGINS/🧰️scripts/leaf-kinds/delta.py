"""Delta-macro variant of the leaf-kinds generator: id-keyed rows whose diff lane is a `list_delta!` row delta with a sparse `row_patch!` (EN 1993). Reuses `leaf.leaf_rs/inverse_rs/catalog_json`; replaces the diff, the externally tagged wire schemas and the Rust type reading."""
import re
from common import *
import leaf as L, model as M

L.TYPES["u8"] = ("integer", "Int", "uint32")
M.Leaf.copy = property(lambda self: self.ty in ("f64", "u32", "u8", "bool") or self.ty.startswith("enum:"))


def struct_fields(rs, name):
    m = re.search(r"pub struct %s \{(.*?)\n\}" % re.escape(name), rs, re.S)
    return dict(re.findall(r"pub (\w+): ([^,\n]+),", m.group(1))) if m else {}


def rust_name(fields, snake_name):
    for f in fields:
        if snake(camel(f)) == snake_name:
            return f
    return None


def read(schema_json_path, rs_path, ents):
    """📖 Reads the leaves of `ents` from the snapshot schema, naming every path element by its real Rust field and typing it from the struct."""
    defs = json.loads(rd(schema_json_path))["$defs"]
    rs = rd(rs_path)
    leaves, skipped = [], []
    for e in ents:
        top = struct_fields(rs, e.chain[0][1])
        for path, ty, ui, owner in M.leaves_of(defs, e.chain[0][1]):
            names = [rust_name(top, path[0])]
            fields = top
            if len(path) == 2:
                fields = struct_fields(rs, owner)
                names.append(rust_name(fields, path[1]))
            if None in names:
                skipped.append((e.name, "_".join(path)))
                continue
            rty = fields[names[-1]].strip()
            if rty not in ("f64", "u32", "u8", "bool", "String"):
                skipped.append((e.name, "_".join(names) + ":" + rty))
                continue
            l = M.Leaf(e, names, rty, ui, False)
            if len(names) == 2:
                l.kind = M.shorten("change-%s-%s" % (e.slug, kebab(names[1])), M.kind_limit())
                l.module = l.kind.replace("-", "_")
                l.record = pascal(l.kind)
                l.dirname = emoji_dir(e.emoji, l.kind)
            leaves.append(l)
    return leaves, defs, skipped


def diff_rs(leaf, delta, patch):
    """🔺️ Leaf diff: one `Delta::modification(id, Patch { field: Some(v) })` on the addressed row."""
    e = leaf.ent
    coll = e.chain[0][0]
    guard = ""
    if leaf.ty == "f64":
        guard = '    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n' % leaf.new
    nested = len(leaf.path) == 2
    cl = "" if leaf.copy else ".clone()"
    if nested:
        value = "    let mut value = row.%s.clone();\n    value.%s = payload.%s%s;\n" % (leaf.path[0], leaf.path[1], leaf.new, cl)
        some = "value"
    else:
        value, some = "", "payload.%s%s" % (leaf.new, cl)
    return '''//! 🔺️ `%(kind)s` diff: patches `%(access)s` of the addressed %(ent)s row.

use crate::%(mod)s::%(rec)s;
use %(diff)s::{En1993Diff, %(delta)s, %(patch)s};
use %(model)s::En1993Snapshot;

pub fn diff(payload: &%(rec)s, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
%(guard)s    let Some(row) = base.%(coll)s.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "%(ent_kebab)s", Vec::<String>::new());
    };
    if row.%(access)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The %(ent)s already has this value.");
    }
%(value)s    protocol::MutationOutcome::new(En1993Diff { %(coll)s: %(delta)s::modification(payload.id.clone(), %(patch)s { %(pfield)s: Some(%(some)s), ..Default::default() }), ..Default::default() })
}
''' % dict(kind=leaf.kind, access=leaf.access, ent=e.en.lower(), mod=leaf.module, rec=leaf.record, diff=L.crate_diff(), delta=delta, patch=patch, model=L.crate_model(), guard=guard, coll=coll, ent_kebab=e.name, new=leaf.new, value=value, pfield=leaf.path[0], some=some)


def schema_json(leaf, defs):
    """🧬 Externally tagged payload schema: target `id` plus the new value, no discriminator member."""
    e = leaf.ent
    what = e.en.lower()
    props = {"id": {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": e.en, "de": e.de}, "description": {"en": "The %s this mutation addresses, by id." % what, "de": "Das adressierte Element (%s), per Kennung." % e.de}, "ref": {"kind": e.name}, "group": "target", "order": 10}}}
    t = L.TYPES.get(leaf.ty, ("string",))[0]
    p = {"type": t}
    if leaf.ty in ("u32", "u8"):
        p["minimum"] = 0
    if leaf.ty == "u8":
        p["maximum"] = 255
    p["x-semio-ui"] = L.ui_for(leaf, 20)
    props[leaf.newc] = p
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1993/mutation/%s/schema.json" % leaf.kind, "title": leaf.record, "type": "object", "additionalProperties": False, "required": ["id", leaf.newc], "properties": props}


def schema_ts(leaf):
    e = leaf.ent
    rec = leaf.record
    reader = {"f64": "normWireNumber", "u32": 'normWireRange(normWireInteger, {"minimum":0})', "u8": 'normWireRange(normWireInteger, {"minimum":0,"maximum":255})', "bool": "normWireBoolean"}.get(leaf.ty, "normWireString")
    tsty = {"f64": "number", "u32": "number", "u8": "number", "bool": "boolean"}.get(leaf.ty, "string")
    imports = {"normWireObject", "normWireRequired", "normWireString", "type NormWireReader"}
    imports |= {"normWireNumber"} if leaf.ty == "f64" else set()
    imports |= {"normWireInteger", "normWireRange"} if leaf.ty in ("u32", "u8") else set()
    imports |= {"normWireBoolean"} if leaf.ty == "bool" else set()
    imports |= {"normWireString"} if tsty == "string" else set()
    return '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { %s } from "@semio-tech/norm-js/registry/contract";

export interface %s {
  id: string;
  %s: %s;
}

export const parse%s: NormWireReader<%s> = normWireObject<%s>({ id: normWireRequired(normWireString), %s: normWireRequired(%s) });
''' % (e.emoji, leaf.kind, rec, ", ".join(sorted(imports)), rec, leaf.newc, tsty, rec, rec, rec, leaf.newc, reader)
