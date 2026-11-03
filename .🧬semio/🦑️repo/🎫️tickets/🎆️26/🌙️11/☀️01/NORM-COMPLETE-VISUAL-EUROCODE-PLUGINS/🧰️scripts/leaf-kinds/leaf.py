"""Per-kind files of an id-keyed artifact: payload leaf, diff, inverse, schema json/ts, catalogue json."""
from common import *

TYPES = {"f64": ("number", "Float", "double"), "u32": ("integer", "Int", "uint32"), "bool": ("boolean", "Boolean", "bool"), "String": ("string", "String", "string")}


def gql_ty(leaf):
    return TYPES.get(leaf.ty, ("string", "String", "string"))[1]


def proto_ty(leaf):
    return TYPES.get(leaf.ty, ("string", "String", "string"))[2]


def rust_default_sample(leaf, value):
    if leaf.ty == "f64":
        return repr(float(value))
    if leaf.ty == "u32":
        return "%du32" % value
    if leaf.ty == "bool":
        return "true" if value else "false"
    if leaf.ty.startswith("enum:"):
        return "semio_s_artifact_norm_%s_model::%s::%s" % (A.id, leaf.rust_ty, pascal(value))
    return 'String::from("%s")' % value


class A:
    id = "en1999"


def init(art_id):
    A.id = art_id


def crate_model():
    return "semio_s_artifact_norm_%s_model" % A.id


def crate_diff():
    return "semio_s_artifact_norm_%s_diff" % A.id


def snap():
    return "%sSnapshot" % A.id.capitalize().replace("en", "En")


def Pfx():
    return A.id.replace("en", "En", 1)


def leaf_rs(leaf):
    e = leaf.ent
    en, de = leaf.label
    fields = "".join("    pub %s: String,\n" % s for s in e.sels) + "    pub %s: %s,\n" % (leaf.new, leaf.rust_ty if not leaf.ty.startswith("enum:") else crate_model() + "::" + leaf.rust_ty)
    mod, rec, kind = leaf.module, leaf.record, leaf.kind
    S = Pfx() + "Snapshot"
    return '''//! %(em)s `%(kind)s` mutation leaf: sets `%(access)s` of one %(ent_en)s.

use %(diff)s::%(P)sDiff;
use %(model)s::%(S)s;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct %(rec)s {
%(fields)s}

impl<Op> protocol::MutationKind<%(S)s, Op> for %(rec)s where Op: protocol::Mutation<%(S)s, Diff = %(diff)s::%(P)sDiff> + From<crate::%(mod)s::%(rec)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "%(entity)s",
        kind: "%(kind)s",
        record: "Changed%(rec_tail)s",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("%(lab_en)s", "%(lab_de)s")
    }
}

impl<Op> protocol::ColdMutationKind<%(S)s, Op> for %(rec)s where Op: protocol::ColdMutation<%(S)s, ReferenceDiff = %(diff)s::%(P)sDiff> + From<crate::%(mod)s::%(rec)s> {
    fn diff(&self, base: &%(S)s) -> protocol::MutationOutcome<%(P)sDiff> {
        crate::%(mod)s::diff::diff(self, base)
    }
    fn inverse(&self, base: &%(S)s) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::%(mod)s::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
''' % dict(em=e.emoji, kind=kind, access=leaf.access, ent_en=e.en.lower(), diff=crate_diff(), model=crate_model(), P=Pfx(), S=snap(), rec=rec, mod=mod, fields=fields,
           entity=e.name + "-" + kebab(leaf.field), rec_tail=rec[len("Change"):], lab_en=label_en(leaf), lab_de=label_de(leaf))


def label_en(leaf):
    return "Change %s of the %s" % (leaf.label[0].replace('"', "'"), leaf.ent.en.lower())


def label_de(leaf):
    return "%s: %s ändern" % (leaf.ent.de, leaf.label[1].replace('"', "'"))


def walk(leaf, var="base"):
    """(let-else lines binding r0..rN, final variable)"""
    e = leaf.ent
    out, src = [], var
    for i, (coll, _, sel) in enumerate(e.chain):
        out.append('    let Some(r%d) = %s.%s.iter().find(|row| row.id == payload.%s) else {\n        return protocol::MutationOutcome::error("mutation.target-missing", "%s", Vec::<String>::new());\n    };' % (i, src, coll, sel, sel[:-3].replace("_", "-")))
        src = "r%d" % i
    return out, src


def diff_rs(leaf):
    e, P = leaf.ent, Pfx()
    lets, last = walk(leaf)
    val = "payload.%s%s" % (leaf.new, "" if leaf.copy else ".clone()")
    names = set()
    inner = "%s%sPatch { %s: Some(%s), ..Default::default() }" % (P, e.patch, leaf.field, val)
    names.add("%s%sPatch" % (P, e.patch))
    names.add("%s%sRows" % (P, e.patch))
    expr = "%s%sRows::modification(&payload.%s, %s)" % (P, e.patch, e.chain[-1][2], inner)
    for i in range(len(e.chain) - 2, -1, -1):
        pname = "".join(pascal(c[0]) for c in e.chain[: i + 1])
        names.add("%s%sPatch" % (P, pname))
        names.add("%s%sRows" % (P, pname))
        expr = "%s%sRows::modification(&payload.%s, %s%sPatch { %s: Some(%s), ..Default::default() })" % (P, pname, e.chain[i][2], P, pname, e.chain[i + 1][0], expr)
    guard = ""
    if leaf.ty == "f64":
        guard = '    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n' % leaf.new
    imports = ", ".join(sorted(names | {P + "Diff"}))
    return '''//! 🔺️ `%(kind)s` diff: patches `%(access)s` of the addressed %(ent)s row.

use crate::%(mod)s::%(rec)s;
use %(diff)s::{%(imports)s};
use %(model)s::%(S)s;

pub fn diff(payload: &%(rec)s, base: &%(S)s) -> protocol::MutationOutcome<%(P)sDiff> {
%(guard)s%(lets)s
    if %(last)s.%(access)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The %(ent)s already has this value.");
    }
    protocol::MutationOutcome::new(%(P)sDiff { %(top)s: Some(%(expr)s), ..Default::default() })
}
''' % dict(kind=leaf.kind, access=leaf.access, ent=e.en.lower(), mod=leaf.module, rec=leaf.record, diff=crate_diff(), imports=imports, model=crate_model(), S=snap(), P=P, guard=guard, lets="\n".join(lets), last=last, new=leaf.new, top=e.chain[0][0], expr=expr)


def inverse_rs(leaf):
    e = leaf.ent
    chain = "base.%s.iter().find(|row| row.id == payload.%s)" % (e.chain[0][0], e.chain[0][2])
    for coll, _, sel in e.chain[1:]:
        chain += ".and_then(|row| row.%s.iter().find(|row| row.id == payload.%s))" % (coll, sel)
    keep = "".join("%s: payload.%s.clone(), " % (s, s) for s in e.sels)
    return '''//! ↩️ `%(kind)s` inverse: restores the previous `%(access)s`.

use crate::%(mod)s::%(rec)s;
use %(model)s::%(S)s;

pub fn inverse<Op>(payload: &%(rec)s, base: &%(S)s) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(mod)s::%(rec)s> {
    Ok(match %(chain)s {
        Some(row) => vec![Op::from(%(rec)s { %(keep)s%(new)s: row.%(access)s%(cl)s })],
        None => Vec::new(),
    })
}
''' % dict(kind=leaf.kind, access=leaf.access, mod=leaf.module, rec=leaf.record, model=crate_model(), S=snap(), chain=chain, keep=keep, new=leaf.new, cl="" if leaf.copy else ".clone()")


def ui_for(leaf, order):
    ui = dict(leaf.ui)
    ui.pop("label", None)
    out = {"label": {"en": leaf.label[0], "de": leaf.label[1]}}
    if leaf.ty == "bool":
        out["widget"] = "toggle"
    elif leaf.ty in ("f64", "u32"):
        out["widget"] = ui.pop("widget", "stepper")
        out.update(ui)
    elif leaf.ty.startswith("enum:"):
        out["widget"] = "select"
    else:
        out["widget"] = "text"
    out.update({"role": "value", "group": "value", "order": order})
    return out


def schema_json(leaf, defs):
    e = leaf.ent
    props = {"mutation": {"const": leaf.record[:1].lower() + leaf.record[1:]}}
    req = ["mutation"]
    order = 10
    for (coll, d, sel) in e.chain:
        what = sel[:-3].replace("_", " ")
        props[camel(sel)] = {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": what.capitalize(), "de": TARGET_DE.get(sel, what.capitalize())}, "description": {"en": "The %s this mutation addresses, by id." % what, "de": "Das adressierte Element (%s), per Kennung." % TARGET_DE.get(sel, what)}, "ref": {"kind": sel[:-3].replace("_", "-")}, "group": "target", "order": order}}
        req.append(camel(sel))
        order += 10
    t = TYPES.get(leaf.ty, ("string",))[0]
    p = {"type": t}
    if leaf.ty.startswith("enum:"):
        p["enum"] = defs[leaf.rust_ty]["enum"]
    if leaf.ty == "u32":
        p["minimum"] = 0
    p["x-semio-ui"] = ui_for(leaf, order)
    props[leaf.newc] = p
    req.append(leaf.newc)
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/%s/mutation/%s/schema.json" % (A.id, leaf.kind), "title": leaf.record, "type": "object", "additionalProperties": False, "required": req, "properties": props}


TARGET_DE = {}


def schema_ts(leaf):
    e = leaf.ent
    rec = leaf.record
    lit = rec[:1].lower() + rec[1:]
    reader = {"f64": "normWireNumber", "u32": 'normWireRange(normWireInteger, {"minimum":0})', "bool": "normWireBoolean"}.get(leaf.ty, "normWireString")
    tsty = {"f64": "number", "u32": "number", "bool": "boolean"}.get(leaf.ty, "string")
    imports = {"normWireLiteral", "normWireObject", "normWireRequired", "type NormWireReader"}
    imports |= {"normWireNumber"} if leaf.ty == "f64" else set()
    imports |= {"normWireInteger", "normWireRange"} if leaf.ty == "u32" else set()
    imports |= {"normWireBoolean"} if leaf.ty == "bool" else set()
    imports |= {"normWireString"} if tsty == "string" else set()
    members = '  mutation: "%s";\n' % lit + "".join("  %s: string;\n" % camel(s) for s in e.sels) + "  %s: %s;\n" % (leaf.newc, tsty)
    rdr = 'mutation: normWireRequired(normWireLiteral("%s")), ' % lit + "".join("%s: normWireRequired(normWireString), " % camel(s) for s in e.sels) + "%s: normWireRequired(%s)" % (leaf.newc, reader)
    return '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { %s } from "@semio-tech/norm-js/registry/contract";

export interface %s {
%s}

export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });
''' % (e.emoji, leaf.kind, rec, ", ".join(sorted(imports)), rec, members, rec, rec, rec, rdr)


def catalog_json(leaf, tag, root):
    return {"schemaVersion": 1, "owner": root + "/" + leaf.dirname, "semanticKind": leaf.kind, "displayName": "Change " + leaf.record[len("Change"):], "emoji": leaf.ent.emoji, "aggregateVariant": leaf.record, "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}
