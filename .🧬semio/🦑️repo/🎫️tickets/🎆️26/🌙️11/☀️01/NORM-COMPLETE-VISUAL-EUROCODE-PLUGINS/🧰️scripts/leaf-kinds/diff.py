"""Regenerates the sparse per-row patch lanes of an id-keyed diff crate from the entity leaves: rust, json, ts, graphql, proto twins."""
import copy, json, re
from common import *

TS_FIELD = {"f64": ("number", "normWireNumber"), "u32": ("number", 'normWireRange(normWireInteger, {"minimum":0})'), "bool": ("boolean", "normWireBoolean"), "String": ("string", "normWireString")}
GQL = {"f64": "Float", "u32": "Int", "bool": "Boolean", "String": "String"}
PROTO = {"f64": "double", "u32": "uint32", "bool": "bool", "String": "string"}


class Patch:
    def __init__(self, P, name, row_def, fields, lanes):
        self.P, self.name, self.row_def, self.fields, self.lanes = P, name, row_def, fields, lanes

    @property
    def full(self):
        return "%s%sPatch" % (self.P, self.name)


def specs(P, ents, leaves, model_crate):
    by = {}
    for e in ents:
        by[e.patch] = Patch(P, e.patch, e.chain[-1][1], [], [])
    for l in leaves:
        by[l.ent.patch].fields.append(l)
    for e in ents:
        if len(e.chain) > 1:
            parent = "".join(pascal(c[0]) for c in e.chain[:-1])
            by[parent].lanes.append((e.chain[-1][0], "%s%sRows" % (P, e.patch)))
    return by


def rust_patch(s, model):
    row = "%s::%s" % (model, s.row_def)
    fty = lambda l: ("%s::%s" % (model, l.rust_ty)) if l.ty.startswith("enum:") else l.ty
    f = s.fields
    lane_fields = "".join("    pub %s: Option<%s>,\n" % (n, r) for n, r in s.lanes)
    cap = "capability" if s.lanes else "_capability"
    commit = "".join("        if let Some(value) = &self.%s { row.%s = value.clone(); }\n" % (l.field, l.access) for l in f)
    commit += "".join("        if let Some(rows) = &self.%s {\n            row.%s = rows.commit_onto(&row.%s, capability).map_err(|error| error.under([\"%s\"]))?;\n        }\n" % (n, n, n, n) for n, r in s.lanes)
    absorb = "".join("        if later.%s.is_some() { self.%s = later.%s; }\n" % ((l.field,) * 3) for l in f)
    absorb += "".join("        if let Some(theirs) = later.%s {\n            match self.%s.as_mut() {\n                Some(mine) => mine.absorb(theirs),\n                None => self.%s = Some(theirs),\n            }\n        }\n        self.%s = self.%s.take().filter(|rows| !rows.is_empty());\n" % ((n,) * 5) for n, r in s.lanes)
    inv = "".join("            %s: self.%s.as_ref().map(|_| row.%s.clone()),\n" % (l.field, l.field, l.access) for l in f)
    inv += "".join("            %s: self.%s.as_ref().map(|rows| rows.inverse(&row.%s)).filter(|rows| !rows.is_empty()),\n" % ((n,) * 3) for n, r in s.lanes)
    empty = " && ".join(["self.%s.is_none()" % l.field for l in f] + ["self.%s.as_ref().map_or(true, |rows| rows.is_empty())" % n for n, r in s.lanes]) or "true"
    nm = s.name_snake if hasattr(s, "name_snake") else None
    return '''/// 🩹️ Sparse patch of the `%(lane)s` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct %(full)s {
%(struct)s%(lane_fields)s}

impl protocol::list_delta::RowPatch<%(row)s> for %(full)s {
    fn commit_into(&self, row: &mut %(row)s, %(cap)s: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
%(commit)s        Ok(())
    }

    fn absorb(&mut self, later: Self) {
%(absorb)s    }

    fn inverse(&self, row: &%(row)s) -> Self {
        Self {
%(inv)s        }
    }

    fn is_empty(&self) -> bool {
        %(empty)s
    }
}
''' % dict(lane=lane_name(s), full=s.full, struct="".join("    pub %s: Option<%s>,\n" % (l.field, fty(l)) for l in f), lane_fields=lane_fields, row=row, cap=cap, commit=commit, absorb=absorb, inv=inv, empty=empty)


def lane_name(s):
    return re.sub(r"([A-Z])", lambda m: "_" + m.group(1).lower(), s.name).strip("_").split("_")[-1]


BLOCK = r"(?P<doc>[ \t]*/// 🩹️ Sparse patch of the `\w+` row addressed by its key\.\n)#\[derive[^\n]*\n(?:#\[[^\n]*\n)*pub struct (?P<name>%s) \{.*?\n\}\n\nimpl protocol::list_delta::RowPatch<[^>]+> for (?P=name) \{.*?\n\}\n"


def patch_rust(text, P, by, model, clone_from, new_lanes):
    """Replaces each existing Patch block; clones the Rows lane of `clone_from` for lanes that do not exist yet."""
    for s in by.values():
        pat = re.compile(BLOCK % s.full, re.S)
        m = pat.search(text)
        block = rust_patch(s, model)
        if m:
            text = text[: m.start()] + block + text[m.end():]
        else:
            text = add_lane_rust(text, P, s, block, clone_from, by)
    return text


def add_lane_rust(text, P, s, block, clone_from, by):
    src = "%s%s" % (P, clone_from)
    m = re.search(r"protocol::list_delta! \{\n[^\n]*\n[^\n]*\n[^\n]*\n    pub %sRows \{[^\n]*\}\n\}\n" % src, text)
    assert m, "clone source rows lane missing: " + src
    rows = m.group(0).replace(src, s.full[: -len("Patch")]).replace("`%s`" % lane_name_of(clone_from), "`%s`" % lane_name(s))
    parent = [p for p in by.values() if any(r == s.full[: -5] + "Rows" for _, r in p.lanes)][0]
    anchor = "/// 🩹️ Sparse patch of the `%s` row addressed by its key.\n" % lane_name(parent)
    i = text.index(anchor)
    return text[:i] + block + "\n" + rows + "\n" + text[i:]


def lane_name_of(clone_from):
    return re.sub(r"([A-Z])", lambda m: "_" + m.group(1).lower(), clone_from.replace("Patch", "")).strip("_").split("_")[-1]


def camel_ty(l):
    return l.rust_ty


def json_patch(s, snapshot_url, P):
    props, req = {}, []
    for l in s.fields:
        props[camel(l.field)] = {"anyOf": [{"$ref": "%s#/$defs/%s/properties/%s" % (snapshot_url, l.owner_def, camel(l.path[-1]))}, {"type": "null"}]}
        req.append(camel(l.field))
    for n, r in s.lanes:
        props[camel(n)] = {"anyOf": [{"$ref": "#/$defs/%s" % r}, {"type": "null"}]}
        req.append(camel(n))
    return {"type": "object", "additionalProperties": False, "required": req, "properties": props}


def patch_json(text, P, by, snapshot_url, clone_from):
    d = json.loads(text)
    defs = d["$defs"]
    for s in by.values():
        if s.full not in defs:
            base = "%s%s" % (P, clone_from)
            for suffix in ("Inserted", "Modified", "Moved", "Removed", "Rows"):
                new = copy.deepcopy(json.loads(json.dumps(defs[base + suffix]).replace(base, s.full[:-5])))
                defs[s.full[:-5] + suffix] = new
        defs[s.full] = json_patch(s, snapshot_url, P)
    d["$defs"] = dict(sorted(defs.items()))
    return compact_json(d)


def ts_patch(s):
    fl = "".join("  %s: %s | null;\n" % (camel(l.field), "number" if l.ty in ("f64", "u32") else "boolean" if l.ty == "bool" else l.rust_ty if l.ty.startswith("enum:") else "string") for l in s.fields)
    fl += "".join("  %s: %s | null;\n" % (camel(n), r) for n, r in s.lanes)
    iface = "export interface %s {\n%s}\n" % (s.full, fl)
    rd = []
    for l in s.fields:
        r = "normWireRef(() => parse%s)" % l.rust_ty if l.ty.startswith("enum:") else TS_FIELD[l.ty][1]
        rd.append("%s: normWireRequired(normWireNullable(%s))" % (camel(l.field), r))
    for n, r in s.lanes:
        rd.append("%s: normWireRequired(normWireNullable(normWireRef(() => parse%s)))" % (camel(n), r))
    parser = "export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });\n" % (s.full, s.full, s.full, ", ".join(rd))
    return iface, parser


def patch_ts(text, P, by, clone_from):
    base = "%s%s" % (P, clone_from)
    enums = set()
    for s in by.values():
        enums |= {l.rust_ty for l in s.fields if l.ty.startswith("enum:")}
        iface, parser = ts_patch(s)
        pi = re.compile(r"export interface %s \{.*?\n\}\n" % s.full, re.S)
        pp = re.compile(r"export const parse%s: [^\n]*\n" % s.full)
        if pi.search(text):
            text = pi.sub(lambda m: iface, text, 1)
            text = pp.sub(lambda m: parser, text, 1)
        else:
            stem = s.full[:-5]
            first = first_iface(text, P, by, s)
            ifaces, parsers = "", ""
            for suffix in ("Inserted", "Modified", "Moved", "Patch", "Removed", "Rows"):
                m = re.search(r"export interface %s%s \{.*?\n\}\n\n" % (base, suffix), text, re.S)
                ifaces += iface + "\n" if suffix == "Patch" else m.group(0).replace(base, stem)
                parsers += parser if suffix == "Patch" else re.search(r"export const parse%s%s: [^\n]*\n" % (base, suffix), text).group(0).replace(base, stem)
            pp = text.index("export const parse%s:" % first)
            text = text[:pp] + parsers + text[pp:]
            pi = text.index("export interface %s {" % first)
            text = text[:pi] + ifaces + text[pi:]
    for e in sorted(enums):
        line = next(ln for ln in text.split("\n")[:12] if "type AluminiumConnection" in ln)
        if "type %s," % e not in line:
            text = text.replace(line, line.replace("type FatigueDetail,", "type FatigueDetail, type %s," % e, 1).replace("parseFatigueDetail,", "parseFatigueDetail, parse%s," % e, 1), 1)
    return text


def first_iface(text, P, by, s):
    parent = [p for p in by.values() if any(r == s.full[:-5] + "Rows" for _, r in p.lanes)][0]
    m = re.search(r"export interface (%s\w*) \{" % parent.full[:-5], text)
    return m.group(1)


def gql_patch(s):
    fl = "".join("  %s: %s\n" % (camel(l.field), "String" if l.ty.startswith("enum:") else GQL[l.ty]) for l in s.fields)
    fl += "".join("  %s: %s\n" % (camel(n), r) for n, r in s.lanes)
    return "type %s {\n%s}\n" % (s.full, fl)


def proto_patch(s):
    fl, i = "", 1
    for l in s.fields:
        fl += "  optional %s %s = %d;\n" % ("string" if l.ty.startswith("enum:") else PROTO[l.ty], l.field, i)
        i += 1
    for n, r in s.lanes:
        fl += "  optional %s %s = %d;\n" % (r, n, i)
        i += 1
    return "message %s {\n%s}\n" % (s.full, fl)


def patch_blocks(text, P, by, clone_from, kind):
    base = "%s%s" % (P, clone_from)
    for s in by.values():
        blk = gql_patch(s) if kind == "gql" else proto_patch(s)
        head = "type %s {" % s.full if kind == "gql" else "message %s {" % s.full
        pat = re.compile(re.escape(head) + r".*?\n\}\n", re.S)
        if pat.search(text):
            text = pat.sub(lambda m: blk, text, 1)
            continue
        stem = s.full[:-5]
        kw = "type" if kind == "gql" else "message"
        parts = []
        for suffix in ("Patch", "Removed", "Inserted", "Moved", "Modified", "Rows"):
            m = re.search(r"%s %s%s \{.*?\n\}\n" % (kw, re.escape(base), suffix), text, re.S)
            parts.append(blk if suffix == "Patch" else m.group(0).replace(base, stem))
        parent = [p for p in by.values() if any(r == stem + "Rows" for _, r in p.lanes)][0]
        pos = text.index("%s %s {" % (kw, parent.full))
        text = text[:pos] + "\n".join(parts) + "\n" + text[pos:]
    return text
