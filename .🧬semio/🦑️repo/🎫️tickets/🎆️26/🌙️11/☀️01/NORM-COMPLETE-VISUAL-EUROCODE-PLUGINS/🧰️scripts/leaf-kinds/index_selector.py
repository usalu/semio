"""Index-selector template of the leaf-kinds generator: set-field, insert-row and remove-row kinds for keyed `list_delta` diffs whose chain mixes id selectors (`member_id`: String) and index selectors (`building_index`: usize).

A selector named `index` or `*_index` is positional (`rows.get(payload.sel)`, diff key read from the row), every other selector is a row key (`rows.iter().find(|row| row.id == payload.sel)`). The diff naming and wrapping of an artifact is a `Style`; leaves may sit behind one optional object (`stirrups.spacing`, patch field `stirrups_spacing`). Used by en1992 and en1990 (`keyed.py` wires them); en1993, en1991 and the others import `Style`, `Link`, `SetKind`, `InsertKind`, `RemoveKind`.
"""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *

TYPES = {"f64": ("number", "Float", "double", "normWireNumber", "number"), "u32": ("integer", "Int", "uint32", 'normWireRange(normWireInteger, {"minimum":0})', "number"), "u8": ("integer", "Int", "uint32", 'normWireRange(normWireInteger, {"minimum":0,"maximum":255})', "number"), "bool": ("boolean", "Boolean", "bool", "normWireBoolean", "boolean"), "String": ("string", "String", "string", "normWireString", "string")}


def is_index(sel):
    return sel == "index" or sel.endswith("_index")


def sel_rust(sel):
    return "usize" if is_index(sel) else "String"


def sel_word(sel):
    return sel[:-6] if sel.endswith("_index") else sel[:-3] if sel.endswith("_id") else sel


def sel_clone(sel):
    return "" if is_index(sel) else ".clone()"


class Link:
    """One step of an addressing chain: the collection field of the parent, its row type, the selector payload field, the human name and the singular used by insert/remove payloads."""

    def __init__(self, coll, row, sel, single, en, de, key="id"):
        self.coll, self.row, self.sel, self.single, self.en, self.de, self.key = coll, row, sel, single, en, de, key


class Style:
    """How an artifact names its diff lanes: `lanes` = pascal of the chain's collections (en1992: `En1992MembersBarsRows/Patch`, top lane `Option`), or `single` = the row's singular (en1990: `En1990MemberDelta/Patch`, top lane bare)."""

    def __init__(self, P, model, diff_path, snap, snapshot_ts, mode="lanes", opt_index=False, item_field=None, tagged=True, ts_prefix=""):
        self.P, self.model, self.diff_path, self.snap, self.snapshot_ts, self.mode, self.opt_index = P, model, diff_path, snap, snapshot_ts, mode, opt_index
        self.tagged = tagged
        self.ts_prefix = ts_prefix
        self.item_field = item_field or (lambda ent: ent.last.single)

    def stem(self, links):
        return "".join(pascal(l.coll) for l in links) if self.mode == "lanes" else pascal(links[-1].single)

    def rows(self, links):
        return self.P + self.stem(links) + ("Rows" if self.mode == "lanes" else "Delta")

    def patch(self, links):
        return self.P + self.stem(links) + "Patch"

    def top(self, expr):
        return "Some(%s)" % expr if self.mode == "lanes" else expr

    def nest(self, expr):
        return "Some(%s)" % expr


class Ent:
    def __init__(self, name, emoji, links, en, de, slug=None, group=None):
        self.name, self.emoji, self.links, self.en, self.de = name, emoji, links, en, de
        self.slug = slug or name
        self.group = group or name

    @property
    def sels(self):
        return [l.sel for l in self.links]

    @property
    def last(self):
        return self.links[-1]


class Leaf:
    """A scalar leaf of an entity row; `path` is the snake path below the row (`["fire","axis_distance"]`), `opt` marks that the first segment is an `Option<struct>`; `ty` is f64/u32/u8/bool/String or `enum:Name`."""

    def __init__(self, ent, path, ty, ui, opt=False, owner=None, kind=None):
        self.ent, self.path, self.ty, self.ui, self.opt, self.owner = ent, path, ty, ui or {}, opt, owner
        self.field = "_".join(path)
        self.access = ".".join(path)
        self.new = "new_" + self.field
        self.newc = camel(self.new)
        self.kind = kind
        self.set_names()

    def set_names(self):
        self.module = self.kind.replace("-", "_")
        self.record = pascal(self.kind)

    @property
    def label(self):
        lab = self.ui.get("label") or {}
        return lab.get("en") or self.field.replace("_", " "), lab.get("de") or self.field.replace("_", " ")

    @property
    def rust_ty(self):
        return self.ty[5:] if self.ty.startswith("enum:") else self.ty

    @property
    def copy(self):
        return self.ty in ("f64", "u32", "u8", "bool") or self.ty.startswith("enum:")


def lc(text):
    return text[:1].lower() + text[1:] if len(text) > 1 and text[1].islower() else text


def row_ty(st, name):
    return "%s::%s" % (st.model, name)


def doc_head(emoji, text):
    return "//! %s %s\n\n" % (emoji, text)


MUT_DERIVE = '''#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
'''

MOUNTS = '''
//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
'''


def struct_rs(rec, fields):
    return "pub struct %s {\n%s}\n" % (rec, "".join("    pub %s: %s,\n" % f for f in fields))


def kind_impls(st, k, label_en_expr, label_de_expr, inv_module, inv_rec, target_exprs, verb, entity_slug, record_tail):
    tgt = "".join("    fn target(&self) -> Vec<String> {\n        vec![%s]\n    }\n" % ", ".join(target_exprs) for _ in [0] if target_exprs)
    return '''impl<Op> protocol::MutationKind<%(S)s, Op> for %(rec)s where Op: protocol::Mutation<%(S)s, Diff = %(D)s> + From<crate::%(inv_module)s::%(inv_rec)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "%(verb)s",
        entity: "%(entity)s",
        kind: "%(kind)s",
        record: "%(tail)s",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(%(en)s, %(de)s)
    }
%(tgt)s}

impl<Op> protocol::ColdMutationKind<%(S)s, Op> for %(rec)s where Op: protocol::ColdMutation<%(S)s, ReferenceDiff = %(D)s> + From<crate::%(inv_module)s::%(inv_rec)s> {
    fn diff(&self, base: &%(S)s) -> protocol::MutationOutcome<%(D)s> {
        crate::%(mod)s::diff::diff(self, base)
    }
    fn inverse(&self, base: &%(S)s) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::%(mod)s::inverse::inverse(self, base)
    }
}
''' % dict(S=st.snap, D=st.diff_path + "::" + st.P + "Diff", rec=k.record, inv_module=inv_module, inv_rec=inv_rec, verb=verb, entity=entity_slug, kind=k.kind, tail=record_tail, en=label_en_expr, de=label_de_expr, tgt=tgt, mod=k.module)


def walk(ent, st, var="base"):
    """let-else chain binding r0..rN; returns (lines, key exprs per link, last var)."""
    out, keys, src = [], [], var
    for i, l in enumerate(ent.links):
        if is_index(l.sel):
            out.append('    let Some(r%d) = %s.%s.get(payload.%s) else {\n        return protocol::MutationOutcome::error("mutation.target-missing", format!("%s {} does not exist.", payload.%s), Vec::<String>::new());\n    };' % (i, src, l.coll, l.sel, l.en, l.sel))
            keys.append("&r%d.%s" % (i, l.key))
        else:
            out.append('    let Some(r%d) = %s.%s.iter().find(|row| row.%s == payload.%s) else {\n        return protocol::MutationOutcome::error("mutation.target-missing", format!("%s {} does not exist.", payload.%s), [payload.%s.clone()]);\n    };' % (i, src, l.coll, l.key, l.sel, l.en, l.sel, l.sel))
            keys.append("&payload.%s" % l.sel)
        src = "r%d" % i
    return out, keys, src


def nest_diff(ent, st, keys, inner_patch_expr):
    """The diff top lane value wrapping `inner_patch_expr` (a Patch expression of the last link) down to the diff root."""
    links = ent.links
    expr = "%s::modification(%s, %s)" % (st.rows(links), keys[-1], inner_patch_expr)
    for i in range(len(links) - 2, -1, -1):
        sub = links[: i + 1]
        expr = "%s::modification(%s, %s { %s: %s, ..Default::default() })" % (st.rows(sub), keys[i], st.patch(sub), links[i + 1].coll, st.nest(expr))
    return expr


def diff_names(ent, st, extra=()):
    names = {st.P + "Diff"}
    for i in range(len(ent.links)):
        sub = ent.links[: i + 1]
        names |= {st.rows(sub)}
        if i < len(ent.links):
            names |= {st.patch(sub)}
    return names | set(extra)


class SetKind:
    """`change-<entity>-<leaf>`: selectors + `new_<leaf>`."""

    verb = "change"

    def __init__(self, leaf):
        self.leaf, self.ent = leaf, leaf.ent
        self.kind, self.module, self.record = leaf.kind, leaf.module, leaf.record
        self.dirname = emoji_dir(self.ent.emoji, self.kind)
        self.group = self.ent.group
        self.inverse_of = self

    def fields(self, st):
        l = self.leaf
        ty = row_ty(st, l.rust_ty) if l.ty.startswith("enum:") else l.rust_ty
        return [(s, sel_rust(s)) for s in self.ent.sels] + [(l.new, ty)]

    @property
    def wire_fields(self):
        l = self.leaf
        return [(s, "index" if is_index(s) else "ref", s) for s in self.ent.sels] + [(l.new, l.ty, l.newc)]

    def leaf_rs(self, st):
        l, e = self.leaf, self.ent
        en, de = l.label
        en = lc(en)
        what = e.en.lower()
        tgt = ["self.%s.to_string()" % s if is_index(s) else "self.%s.clone()" % s for s in e.sels]
        body = doc_head(e.emoji, "`%s` mutation leaf: sets `%s` of one %s." % (self.kind, l.access, what))
        body += "use %s::%s;\n\n" % (st.model, st.snap)
        body += MUT_DERIVE + struct_rs(self.record, self.fields(st)) + "\n"
        body += kind_impls(st, self, '"Change %s of the %s"' % (en.replace('"', "'"), what), '"%s: %s ändern"' % (e.de, de.replace('"', "'")), self.module, self.record, tgt, "change", e.name + "-" + kebab(l.field), "Changed" + self.record[len("Change"):])
        return body + MOUNTS

    def diff_rs(self, st):
        l, e = self.leaf, self.ent
        lets, keys, last = walk(e, st)
        val = "payload.%s%s" % (l.new, "" if l.copy else ".clone()")
        cur = last
        pre = ""
        if l.opt:
            obj = l.path[0]
            pre = '    let Some(object) = %s.%s.as_ref() else {\n        return protocol::MutationOutcome::error("mutation.target-missing", "The optional %s is absent.", Vec::<String>::new());\n    };\n' % (last, obj, obj.replace("_", " "))
            cur = "object"
            cmp = "object.%s" % ".".join(l.path[1:])
        else:
            cmp = "%s.%s" % (last, l.access)
        guard = ""
        if l.ty == "f64":
            guard = '    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n' % l.new
        inner = "%s { %s: Some(%s), ..Default::default() }" % (st.patch(e.links), l.field, val)
        expr = nest_diff(e, st, keys, inner)
        names = sorted(diff_names(e, st))
        return doc_head(e.emoji, "`%s` diff: patches `%s` of the addressed %s row." % (self.kind, l.access, e.en.lower())) + '''use crate::%(mod)s::%(rec)s;
use %(diffp)s::{%(names)s};
use %(model)s::%(S)s;

pub fn diff(payload: &%(rec)s, base: &%(S)s) -> protocol::MutationOutcome<%(P)sDiff> {
%(guard)s%(lets)s%(pre)s    if %(cmp)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The %(ent)s already has this value.");
    }
    protocol::MutationOutcome::new(%(P)sDiff { %(top)s: %(wrap)s, ..Default::default() })
}
''' % dict(mod=self.module, rec=self.record, diffp=st.diff_path, names=", ".join(names), model=st.model, S=st.snap, P=st.P, guard=guard, lets="".join(x + "\n" for x in lets), pre=pre, cmp=cmp, new=l.new, ent=e.en.lower(), top=e.links[0].coll, wrap=st.top(expr))

    def inverse_rs(self, st):
        l, e = self.leaf, self.ent
        lines, src = [], "base"
        for i, k in enumerate(e.links):
            f = "%s.%s.get(payload.%s)?" % (src, k.coll, k.sel) if is_index(k.sel) else "%s.%s.iter().find(|row| row.%s == payload.%s)?" % (src, k.coll, k.key, k.sel)
            lines.append("    let r%d = %s;" % (i, f))
            src = "r%d" % i
        keep = "".join("%s: payload.%s%s, " % (s, s, sel_clone(s)) for s in e.sels)
        cl = "" if l.copy else ".clone()"
        if l.opt:
            lines.append("    let object = %s.%s.as_ref()?;" % (src, l.path[0]))
            prev = "object.%s%s" % (".".join(l.path[1:]), cl)
        else:
            prev = "%s.%s%s" % (src, l.access, cl)
        return doc_head("↩️", "`%s` inverse: restores the previous `%s`." % (self.kind, l.access)) + '''use crate::%(mod)s::%(rec)s;
use %(model)s::%(S)s;

pub fn inverse<Op>(payload: &%(rec)s, base: &%(S)s) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(mod)s::%(rec)s> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &%(rec)s, base: &%(S)s) -> Option<%(rec)s> {
%(lines)s
    Some(%(rec)s { %(keep)s%(new)s: %(prev)s })
}
''' % dict(mod=self.module, rec=self.record, model=st.model, S=st.snap, lines="\n".join(lines), keep=keep, new=l.new, prev=prev)

    def schema_json(self, st, defs, art):
        return schema_json(self, art, self.wire_props(defs), "Change %s" % self.record[len("Change"):], st)

    def wire_props(self, defs):
        l = self.leaf
        t = TYPES.get(l.ty, ("string",))[0]
        p = {"type": t}
        if l.ty.startswith("enum:"):
            p["enum"] = defs[l.rust_ty]["enum"]
        if l.ty == "u32":
            p["minimum"] = 0
        if l.ty == "u8":
            p["minimum"], p["maximum"] = 0, 255
        p["x-semio-ui"] = ui_for(l)
        return {l.newc: p}


def ui_for(leaf, order=None):
    ui = dict(leaf.ui)
    ui.pop("label", None)
    out = {"label": {"en": leaf.label[0], "de": leaf.label[1]}}
    if leaf.ty == "bool":
        out["widget"] = "toggle"
    elif leaf.ty in ("f64", "u32", "u8"):
        out["widget"] = ui.pop("widget", "stepper")
        out.update(ui)
    elif leaf.ty.startswith("enum:"):
        out["widget"] = "select"
    else:
        out["widget"] = "text"
    out.update({"role": "value", "group": "value", "order": order or (10 * (len(leaf.ent.sels) + 1))})
    return out


def target_prop(l, order):
    word = l.en.lower()
    if is_index(l.sel):
        return {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "target", "label": {"en": l.en, "de": l.de}, "description": {"en": "Zero-based position of the %s this mutation addresses." % word, "de": "Nullbasierte Position des adressierten Elements (%s)." % l.de}, "step": 1, "precision": 0, "group": "target", "order": order}}
    return {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": l.en, "de": l.de}, "description": {"en": "The %s this mutation addresses, by id." % word, "de": "Das adressierte Element (%s), per Kennung." % l.de}, "ref": {"kind": (l.single if l.sel == "id" else sel_word(l.sel)).replace("_", "-")}, "group": "target", "order": order}}


def schema_json(k, art, value_props, title, st):
    props = {"mutation": {"const": k.record[:1].lower() + k.record[1:]}} if st.tagged else {}
    req = ["mutation"] if st.tagged else []
    order = 10
    for l in k.ent.links[: k.n_targets] if hasattr(k, "n_targets") else k.ent.links:
        props[camel(l.sel)] = target_prop(l, order)
        req.append(camel(l.sel))
        order += 10
    for n, p in value_props.items():
        props[n] = p
        req.append(n)
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/%s/mutation/%s/schema.json" % (art, k.kind), "title": k.record, "type": "object", "additionalProperties": False, "required": req, "properties": props}


def schema_ts(k, st, wire):
    """`wire`: [(camel name, ts type, reader expr, import names)] of the value fields after the selectors."""
    lit = k.record[:1].lower() + k.record[1:]
    imports = {"normWireObject", "normWireRequired", "type NormWireReader"}
    members = '  mutation: "%s";\n' % lit if st.tagged else ""
    rdr = ['mutation: normWireRequired(normWireLiteral("%s"))' % lit] if st.tagged else []
    imports |= {"normWireLiteral"} if st.tagged else set()
    for l in k.ent.links[: k.n_targets] if hasattr(k, "n_targets") else k.ent.links:
        if is_index(l.sel):
            members += "  %s: number;\n" % camel(l.sel)
            rdr.append("%s: normWireRequired(normWireRange(normWireInteger, {\"minimum\":0}))" % camel(l.sel))
            imports |= {"normWireInteger", "normWireRange"}
        else:
            members += "  %s: string;\n" % camel(l.sel)
            rdr.append("%s: normWireRequired(normWireString)" % camel(l.sel))
            imports.add("normWireString")
    types = set()
    for name, tsty, reader, imps, tys in wire:
        optional = reader.startswith("normWireOptional(")
        members += "  %s%s: %s;\n" % (name, "?" if optional else "", tsty)
        rdr.append("%s: %s" % (name, reader if optional else "normWireRequired(%s)" % reader))
        imports |= set(imps)
        types |= set(tys)
    snap_imp = ""
    if types:
        snap_imp = 'import { %s } from "%s";\n' % (", ".join(sorted("type %s" % t for t in types) + sorted("parse%s" % t for t in types)), st.snapshot_ts)
    return '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { %s } from "@semio-tech/norm-js/registry/contract";
%s
export interface %s {
%s}

export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });
''' % (k.ent.emoji, k.kind, k.record, ", ".join(sorted(imports)), snap_imp, k.record, members, k.record, k.record, k.record, ", ".join(rdr))


def set_ts(k, st):
    l = k.leaf
    if l.ty.startswith("enum:"):
        name = st.ts_prefix + l.rust_ty
        return schema_ts(k, st, [(l.newc, name, "parse" + name, [], [name])])
    t = TYPES[l.ty]
    imps = ["normWireInteger", "normWireRange"] if l.ty in ("u32", "u8") else [t[3]]
    return schema_ts(k, st, [(l.newc, t[4], t[3], imps, [])])


SetKind.schema_ts = lambda self, st: set_ts(self, st)


class RowKind:
    """Shared plumbing of insert/remove kinds: `ent` addresses the collection row; `n_targets` selectors address the PARENT (all links but the last)."""

    def __init__(self, ent, verb, kind):
        self.ent, self.verb_, self.kind = ent, verb, kind
        self.module, self.record = kind.replace("-", "_"), pascal(kind)
        self.dirname = emoji_dir(ent.emoji, kind)
        self.group = ent.group

    @property
    def parent(self):
        return self.ent.links[:-1]

    @property
    def last(self):
        return self.ent.links[-1]

    def parent_pairs(self):
        return [(l.sel, sel_rust(l.sel)) for l in self.parent]

    def parent_walk(self, st):
        sub = Ent(self.ent.name, self.ent.emoji, self.parent, self.ent.en, self.ent.de)
        lets, keys, src = walk(sub, st)
        return lets, keys, src

    def row_expr(self, st, rows_expr_cond):
        pass


class InsertKind(RowKind):
    """`insert-<entity>`: parent selectors, `index` (usize; Option<usize> for `opt_index` styles) and the whole row under `item_field`."""

    def __init__(self, ent, kind, remover, st):
        super().__init__(ent, "insert", kind)
        self.remover = remover
        self.n_targets = len(ent.links) - 1
        self.item = st.item_field(ent)
        self.opt_index = st.opt_index
        self.inverse_of = remover

    def fields(self, st):
        return self.parent_pairs() + [("index", "Option<usize>" if self.opt_index else "usize"), (self.item, row_ty(st, self.last.row))]

    @property
    def wire_fields(self):
        return [(s, "index" if is_index(s) else "ref", s) for s in [l.sel for l in self.parent]] + [("index", "optindex" if self.opt_index else "index", "index"), (self.item, "row:" + self.last.row, self.item_c)]

    @property
    def item_c(self):
        return camel(self.item)

    def leaf_rs(self, st):
        e, l = self.ent, self.last
        tgt = ["self.%s.to_string()" % s if is_index(s) else "self.%s.clone()" % s for s, _ in self.parent_pairs()]
        body = doc_head(e.emoji, "`%s` mutation leaf: inserts one %s row." % (self.kind, l.en.lower()))
        body += "use %s::%s;\n\n" % (st.model, st.snap)
        body += MUT_DERIVE + struct_rs(self.record, self.fields(st)) + "\n"
        body += kind_impls(st, self, '&format!("Insert %s {}", self.%s.id)' % (l.en.lower(), self.item), '&format!("%s {} einfügen", self.%s.id)' % (l.de, self.item), self.remover.module, self.remover.record, tgt, "insert", e.name, "Inserted" + self.record[len("Insert"):])
        return body + MOUNTS

    def diff_rs(self, st):
        e, l = self.ent, self.last
        lets, keys, src = self.parent_walk(st)
        coll = "%s.%s" % (src, l.coll)
        at = "index" if self.opt_index else "payload.index"
        inner = "%s::insertion(%s, payload.%s.clone())" % (st.rows(e.links), at, self.item)
        expr = nest_diff_rows(self.parent, e, st, keys, inner) if self.parent else inner
        names = sorted(diff_names(Ent(e.name, e.emoji, self.parent, e.en, e.de), st) | {st.rows(e.links), st.P + "Diff"}) if self.parent else sorted({st.rows(e.links), st.P + "Diff"})
        dup = '    if %s.iter().any(|existing| existing.%s == payload.%s.%s) {\n        let key = payload.%s.%s.clone();\n        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%s {key} already exists."), [key]);\n    }\n' % (coll, l.key, self.item, l.key, self.item, l.key, l.en)
        if self.opt_index:
            tail = '''    let index = payload.index.unwrap_or(usize::MAX).min(%s.len());
    let outcome = protocol::MutationOutcome::new(%sDiff { %s: %s, ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the %s list; inserted at {index}.", payload.index.unwrap_or(index)))
''' % (coll, st.P, e.links[0].coll, st.top(expr), l.en.lower())
        else:
            tail = '''    if payload.index > %s.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, %s.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(%sDiff { %s: %s, ..Default::default() })
''' % (coll, coll, st.P, e.links[0].coll, st.top(expr))
        return doc_head(e.emoji, "`%s` diff: inserts the %s row; an id the list holds is a `mutation.duplicate-id`." % (self.kind, l.en.lower())) + '''use crate::%(mod)s::%(rec)s;
use %(diffp)s::{%(names)s};
use %(model)s::%(S)s;

pub fn diff(payload: &%(rec)s, base: &%(S)s) -> protocol::MutationOutcome<%(P)sDiff> {
%(lets)s%(dup)s%(tail)s}
''' % dict(mod=self.module, rec=self.record, diffp=st.diff_path, names=", ".join(names), model=st.model, S=st.snap, P=st.P, lets="".join(x + "\n" for x in lets), dup=dup, tail=tail)

    def inverse_rs(self, st):
        e, l = self.ent, self.last
        rm = self.remover
        sels = "".join("%s: payload.%s%s, " % (s, s, sel_clone(s)) for s, _ in self.parent_pairs())
        lets, src = [], "base"
        for i, k in enumerate(self.parent):
            f = "%s.%s.get(payload.%s)?" % (src, k.coll, k.sel) if is_index(k.sel) else "%s.%s.iter().find(|row| row.%s == payload.%s)?" % (src, k.coll, k.key, k.sel)
            needed = self.opt_index and rm.by_index and i == len(self.parent) - 1
            lets.append("    let %sr%d = %s;" % ("" if needed else "_", i, f))
            src = ("" if needed else "_") + "r%d" % i
        if rm.by_index:
            lets.append("    let index = %s;" % ("payload.index.unwrap_or(usize::MAX).min(%s.%s.len())" % (src, l.coll) if self.opt_index else "payload.index"))
            tail = "index"
        else:
            tail = "%s: payload.%s.%s.clone()" % (rm.sel_field, self.item, l.key)
        return doc_head("↩️", "`%s` inverse: removes the inserted row again." % self.kind) + '''use crate::%(mod)s::%(rec)s;
use crate::%(rmod)s::%(rrec)s;
use %(model)s::%(S)s;

pub fn inverse<Op>(payload: &%(rec)s, base: &%(S)s) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(rmod)s::%(rrec)s> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &%(rec)s, base: &%(S)s) -> Option<%(rrec)s> {
%(lets)s    Some(%(rrec)s { %(sels)s%(tail)s })
}
''' % dict(mod=self.module, rec=self.record, rmod=rm.module, rrec=rm.record, model=st.model, S=st.snap, lets="".join(x + "\n" for x in lets), sels=sels, tail=tail)

    def wire_props(self, defs):
        l = self.last
        idx = {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Position", "de": "Position"}, "description": {"en": "Zero-based list position at which the record is inserted.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird."}, "step": 1, "precision": 0, "group": "target", "order": 100}}
        row = {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/%s/snapshot.json#/$defs/%s" % (self.art, l.row), "x-semio-ui": {"role": "value", "label": {"en": l.en, "de": l.de}, "description": {"en": "The complete %s record to insert." % l.en.lower(), "de": "Der vollständige einzufügende Datensatz (%s)." % l.de}, "group": "record", "order": 110}}
        out = {"index": idx}
        out[self.item_c] = row
        return out

    def schema_json(self, st, defs, art):
        self.art = art
        return schema_json(self, art, self.wire_props(defs), "Insert " + self.record[len("Insert"):], st)

    def schema_ts(self, st):
        l = self.last
        idx = ("index", "number | null" if self.opt_index else "number", "normWireOptional(normWireNullable(normWireRange(normWireInteger, {\"minimum\":0})))" if self.opt_index else 'normWireRange(normWireInteger, {"minimum":0})', ["normWireInteger", "normWireRange"] + (["normWireNullable", "normWireOptional"] if self.opt_index else []), [])
        return schema_ts(self, st, [idx, (self.item_c, st.ts_prefix + l.row, "parse" + st.ts_prefix + l.row, [], [st.ts_prefix + l.row])])


def nest_diff_rows(parent_links, ent, st, keys, inner_rows_expr):
    """Wraps a row-lane expression of the last link into modifications of the parents (keys of the parent walk)."""
    links = ent.links
    expr = inner_rows_expr
    for i in range(len(parent_links) - 1, -1, -1):
        sub = links[: i + 1]
        expr = "%s::modification(%s, %s { %s: %s, ..Default::default() })" % (st.rows(sub), keys[i], st.patch(sub), links[i + 1].coll, st.nest(expr))
    return expr


class RemoveKind(RowKind):
    """`remove-<entity>`: parent selectors plus the row by key (`<single>_id`) or, with `by_index`, by position."""

    def __init__(self, ent, kind, st, by_index):
        super().__init__(ent, "remove", kind)
        self.by_index = by_index
        self.sel_field = ent.last.sel
        self.inserter = None
        self.inverse_of = None
        self.st = st

    def fields(self, st):
        return self.parent_pairs() + [("index", "usize")] if self.by_index else self.parent_pairs() + [(self.sel_field, "String")]

    @property
    def wire_fields(self):
        base = [(s, "index" if is_index(s) else "ref", s) for s in [l.sel for l in self.parent]]
        return base + ([("index", "index", "index")] if self.by_index else [(self.sel_field, "ref", self.sel_field)])

    def leaf_rs(self, st):
        e, l = self.ent, self.last
        tgt = ["self.%s.to_string()" % s if is_index(s) else "self.%s.clone()" % s for s, _ in self.parent_pairs()]
        tgt.append("self.index.to_string()" if self.by_index else "self.%s.clone()" % self.sel_field)
        label_en = '&format!("Remove %s {}", self.%s)' % (l.en.lower(), "index" if self.by_index else self.sel_field)
        label_de = '&format!("%s {} entfernen", self.%s)' % (l.de, "index" if self.by_index else self.sel_field)
        body = doc_head(e.emoji, "`%s` mutation leaf: removes one %s row." % (self.kind, l.en.lower()))
        body += "use %s::%s;\n\n" % (st.model, st.snap)
        body += MUT_DERIVE + struct_rs(self.record, self.fields(st)) + "\n"
        body += kind_impls(st, self, label_en, label_de, self.inserter.module, self.inserter.record, tgt, "remove", e.name, "Removed" + self.record[len("Remove"):])
        return body + MOUNTS

    def locate(self, st):
        l = self.last
        lets, keys, src = self.parent_walk(st)
        coll = "%s.%s" % (src, l.coll)
        if self.by_index:
            find = '    let index = payload.index;\n    if index >= %s.len() {\n        return protocol::MutationOutcome::error("mutation.target-missing", "%s index out of range", [index.to_string()]);\n    }\n' % (coll, l.coll)
        else:
            find = '    let Some(index) = %s.iter().position(|row| row.%s == payload.%s) else {\n        return protocol::MutationOutcome::error("mutation.target-missing", format!("%s {} does not exist.", payload.%s), [payload.%s.clone()]);\n    };\n' % (coll, l.key, self.sel_field, l.en, self.sel_field, self.sel_field)
        return lets, keys, coll, find

    def diff_rs(self, st):
        e, l = self.ent, self.last
        lets, keys, coll, find = self.locate(st)
        inner = "%s::removal(&%s, index)" % (st.rows(e.links), coll)
        expr = nest_diff_rows(self.parent, e, st, keys, inner) if self.parent else inner
        sub = Ent(e.name, e.emoji, self.parent, e.en, e.de)
        names = sorted((diff_names(sub, st) if self.parent else set()) | {st.rows(e.links), st.P + "Diff"})
        return doc_head(e.emoji, "`%s` diff: removes the addressed %s row." % (self.kind, l.en.lower())) + '''use crate::%(mod)s::%(rec)s;
use %(diffp)s::{%(names)s};
use %(model)s::%(S)s;

pub fn diff(payload: &%(rec)s, base: &%(S)s) -> protocol::MutationOutcome<%(P)sDiff> {
%(lets)s%(find)s    protocol::MutationOutcome::new(%(P)sDiff { %(top)s: %(wrap)s, ..Default::default() })
}
''' % dict(mod=self.module, rec=self.record, diffp=st.diff_path, names=", ".join(names), model=st.model, S=st.snap, P=st.P, lets="".join(x + "\n" for x in lets), find=find, top=e.links[0].coll, wrap=st.top(expr))

    def inverse_rs(self, st):
        e, l = self.ent, self.last
        ins = self.inserter
        lets = []
        src = "base"
        for i, k in enumerate(self.parent):
            f = "%s.%s.get(payload.%s)?" % (src, k.coll, k.sel) if is_index(k.sel) else "%s.%s.iter().find(|row| row.%s == payload.%s)?" % (src, k.coll, k.key, k.sel)
            lets.append("    let r%d = %s;" % (i, f))
            src = "r%d" % i
        coll = "%s.%s" % (src, l.coll)
        if self.by_index:
            pick = "    let index = payload.index;\n    let row = %s.get(index)?;\n" % coll
        else:
            pick = "    let (index, row) = %s.iter().enumerate().find(|(_, row)| row.%s == payload.%s)?;\n" % (coll, l.key, self.sel_field)
        sels = "".join("%s: payload.%s%s, " % (s, s, sel_clone(s)) for s, _ in self.parent_pairs())
        idx_expr = ": Some(index)" if st.opt_index else ""
        return doc_head("↩️", "`%s` inverse: inserts the removed row back at its position." % self.kind) + '''use crate::%(mod)s::%(rec)s;
use crate::%(imod)s::%(irec)s;
use %(model)s::%(S)s;

pub fn inverse<Op>(payload: &%(rec)s, base: &%(S)s) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(imod)s::%(irec)s> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &%(rec)s, base: &%(S)s) -> Option<%(irec)s> {
%(lets)s%(pick)s    Some(%(irec)s { %(sels)sindex%(idx)s, %(item)s: row.clone() })
}
''' % dict(mod=self.module, rec=self.record, imod=ins.module, irec=ins.record, model=st.model, S=st.snap, lets="".join(x + "\n" for x in lets), pick=pick, sels=sels, idx=idx_expr, item=ins.item)

    def wire_props(self, defs):
        return {}

    def schema_json(self, st, defs, art):
        props = {"mutation": {"const": self.record[:1].lower() + self.record[1:]}} if st.tagged else {}
        req = ["mutation"] if st.tagged else []
        order = 10
        for l in self.parent:
            props[camel(l.sel)] = target_prop(l, order)
            req.append(camel(l.sel))
            order += 10
        last = self.last
        if self.by_index:
            props["index"] = {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "target", "label": {"en": "Position", "de": "Position"}, "description": {"en": "Zero-based position of the %s to remove." % last.en.lower(), "de": "Nullbasierte Position des zu entfernenden Elements (%s)." % last.de}, "step": 1, "precision": 0, "group": "target", "order": order}}
            req.append("index")
        else:
            props[camel(self.sel_field)] = target_prop(last, order)
            req.append(camel(self.sel_field))
        return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/%s/mutation/%s/schema.json" % (art, self.kind), "title": self.record, "type": "object", "additionalProperties": False, "required": req, "properties": props}

    def schema_ts(self, st):
        lit = self.record[:1].lower() + self.record[1:]
        imports = {"normWireObject", "normWireRequired", "type NormWireReader", "normWireString"}
        members = '  mutation: "%s";\n' % lit if st.tagged else ""
        rdr = ['mutation: normWireRequired(normWireLiteral("%s"))' % lit] if st.tagged else []
        imports |= {"normWireLiteral"} if st.tagged else set()
        sels = [(l.sel, is_index(l.sel)) for l in self.parent]
        sels.append(("index", True) if self.by_index else (self.sel_field, False))
        for s, idx in sels:
            if idx:
                members += "  %s: number;\n" % camel(s)
                rdr.append('%s: normWireRequired(normWireRange(normWireInteger, {"minimum":0}))' % camel(s))
                imports |= {"normWireInteger", "normWireRange"}
            else:
                members += "  %s: string;\n" % camel(s)
                rdr.append("%s: normWireRequired(normWireString)" % camel(s))
        return '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { %s } from "@semio-tech/norm-js/registry/contract";

export interface %s {
%s}

export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });
''' % (self.ent.emoji, self.kind, self.record, ", ".join(sorted(imports)), self.record, members, self.record, self.record, self.record, ", ".join(rdr))


def pair(ent, insert_kind, remove_kind, st, by_index=False):
    rm = RemoveKind(ent, remove_kind, st, by_index)
    ins = InsertKind(ent, insert_kind, rm, st)
    rm.inserter = ins
    rm.inverse_of = ins
    return ins, rm


class TopKind(SetKind):
    """`change-<field>` of a scalar at the snapshot root (no selectors): diff `Diff { field: Some(v) }`."""

    def leaf_rs(self, st):
        return super().leaf_rs(st).replace(" of the %s\"" % self.ent.en.lower(), "\"").replace(" of one %s." % self.ent.en.lower(), ".")

    def diff_rs(self, st):
        l = self.leaf
        val = "payload.%s%s" % (l.new, "" if l.copy else ".clone()")
        guard = ""
        if l.ty == "f64":
            guard = '    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n' % l.new
        return doc_head(self.ent.emoji, "`%s` diff: sets `%s`." % (self.kind, l.field)) + '''use crate::%(mod)s::%(rec)s;
use %(diffp)s::%(P)sDiff;
use %(model)s::%(S)s;

pub fn diff(payload: &%(rec)s, base: &%(S)s) -> protocol::MutationOutcome<%(P)sDiff> {
%(guard)s    if base.%(f)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The value is unchanged.");
    }
    protocol::MutationOutcome::new(%(P)sDiff { %(f)s: %(wrap)s, ..Default::default() })
}
''' % dict(mod=self.module, rec=self.record, diffp=st.diff_path, P=st.P, model=st.model, S=st.snap, guard=guard, f=l.field, new=l.new, wrap="Some(%s)" % val)

    def inverse_rs(self, st):
        l = self.leaf
        return doc_head("↩️", "`%s` inverse: restores the previous `%s`." % (self.kind, l.field)) + '''use crate::%(mod)s::%(rec)s;
use %(model)s::%(S)s;

pub fn inverse<Op>(payload: &%(rec)s, base: &%(S)s) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(mod)s::%(rec)s> {
    let _ = payload;
    Ok(vec![Op::from(%(rec)s { %(new)s: base.%(f)s%(cl)s })])
}
''' % dict(mod=self.module, rec=self.record, model=st.model, S=st.snap, new=l.new, f=l.field, cl="" if l.copy else ".clone()")
