"""Regenerates the En1992 diff crate (rust patches/lanes) and its json/ts/graphql/proto twins from the entity leaves; patch fields are exactly the snapshot leaves."""
import copy, json, re, sys, os
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import diff as D

NESTED = '''/// 🩹️ Sparse patch of one keyed row with direct leaves (`set`), leaves behind an optional object (`within`) and owned row lanes (`rows`): the struct, its commit, coalescing, inverse and emptiness.
macro_rules! nested_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { set { $($f:ident: $t:ty),* $(,)? } within { $($w:ident => $obj:ident . $inner:ident : $wt:ty),* $(,)? } rows { $($lane:ident : $rows:ty = $label:literal),* $(,)? } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
        #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>,)* $(pub $w: Option<$wt>,)* $(pub $lane: Option<$rows>,)* }

        impl protocol::list_delta::RowPatch<$row> for $name {
            #[allow(unused_variables)]
            fn commit_into(&self, row: &mut $row, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$f { row.$f = value.clone(); })*
                $(if let Some(value) = &self.$w { row.$obj.as_mut().ok_or_else(|| missing_target(stringify!($obj)))?.$inner = value.clone(); })*
                $(if let Some(rows) = &self.$lane { row.$lane = rows.commit_onto(&row.$lane, capability).map_err(|error| error.under([$label]))?; })*
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                $(if later.$f.is_some() { self.$f = later.$f; })*
                $(if later.$w.is_some() { self.$w = later.$w; })*
                $(if let Some(theirs) = later.$lane {
                    match self.$lane.as_mut() {
                        Some(mine) => mine.absorb(theirs),
                        None => self.$lane = Some(theirs),
                    }
                    self.$lane = self.$lane.take().filter(|rows| !rows.is_empty());
                })*
            }
            fn inverse(&self, row: &$row) -> Self {
                Self {
                    $($f: self.$f.as_ref().map(|_| row.$f.clone()),)*
                    $($w: self.$w.as_ref().and_then(|_| (|| Some(row.$obj.as_ref()?.$inner.clone()))()),)*
                    $($lane: self.$lane.as_ref().map(|rows| rows.inverse(&row.$lane)).filter(|rows| !rows.is_empty()),)*
                }
            }
            fn is_empty(&self) -> bool { true $(&& self.$f.is_none())* $(&& self.$w.is_none())* $(&& self.$lane.as_ref().map_or(true, |rows| rows.is_empty()))* }
        }
    };
}

'''

KEYS = {"ConcreteGrade": "ConcreteGradeKey", "ReinforcementGrade": "ReinforcementGradeKey", "PrestressSteel": "PrestressSteelKey", "BarLayer": "BarLayerKey", "SectionPoint": "SectionPointKey", "BarPoint": "BarPointKey", "TendonPoint": "TendonPointKey", "StudRow": "StudRowKey", "Opening": "OpeningKey", "LoadCaseActions": "LoadCaseActionsKey", "RcMember": "RcMemberKey", "Anchor": "AnchorKey", "Placement": "PlacementKey"}


RUST_FIELD = {"f_p01k": "f_p0_1k"}


def rust_ty(l, model):
    return "%s::%s" % (model, l.rust_ty) if l.ty.startswith("enum:") else l.ty


def rust_field(l):
    return RUST_FIELD.get(l.field, l.field)


def block(spec, model):
    name, row = spec.full, "%s::%s" % (model, spec.row_def)
    direct = [l for l in spec.fields if not l.opt]
    within = [l for l in spec.fields if l.opt]
    lane = lane_name(spec)
    doc = "/// 🩹️ Sparse patch of the `%s` row addressed by its key." % lane
    if not within and not spec.lanes:
        out = "scalar_patch! {\n    %s\n    %s for %s { %s }\n}\n" % (doc, name, row, ", ".join("%s: %s" % (rust_field(l), rust_ty(l, model)) for l in direct))
    else:
        out = "nested_patch! {\n    %s\n    %s for %s {\n        set { %s }\n        within { %s }\n        rows { %s }\n    }\n}\n" % (doc, name, row, ", ".join("%s: %s" % (rust_field(l), rust_ty(l, model)) for l in direct), ", ".join("%s => %s.%s: %s" % (l.field, l.path[0], ".".join(l.path[1:]), rust_ty(l, model)) for l in within), ", ".join('%s: %s = "%s"' % (n, r, camel(n)) for n, r in spec.lanes))
    R = name[: -len("Patch")]
    out += '''
protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `%(lane)s` list (rows keyed by `id`).
    pub %(R)sRows { removal: %(R)sRemoved, insertion: %(R)sInserted, relocation: %(R)sMoved, modification: %(R)sModified, row: %(row)s, patch: %(name)s, list: Vec<%(row)s>, key: String = by %(key)s }
}
''' % dict(lane=lane, R=R, row=row, name=name, key=KEYS[spec.row_def])
    return out


def lane_name(spec):
    return getattr(spec, "lane", None) or D.lane_name(spec)


def order(by):
    """children before parents (a patch needs its lanes' Rows types defined; macros are order-free but keep it readable)."""
    seen, out = set(), []

    def visit(s):
        if s.full in seen:
            return
        seen.add(s.full)
        for n, r in s.lanes:
            child = [x for x in by.values() if x.full[: -len("Patch")] + "Rows" == r][0]
            visit(child)
        out.append(s)
    for s in by.values():
        visit(s)
    return out


def rust(text, by, model):
    if "macro_rules! nested_patch" not in text:
        i = text.index("scalar_patch! {\n")
        text = text[:i] + NESTED + text[i:]
    a = text.index("scalar_patch! {\n    /// 🩹️")
    b = text.index("/// 🔺️ Keyed sparse diff of the En1992 artifact")
    body = "\n".join(block(s, model) for s in order(by))
    return text[:a] + body + "\n" + text[b:]


def ts_patch(text, P, by, enum_imports):
    base = "%sMembersActions" % P
    for s in by.values():
        iface, parser = D.ts_patch(s)
        pi = re.compile(r"export interface %s \{.*?\n\}\n" % s.full, re.S)
        pp = re.compile(r"export const parse%s: [^\n]*\n" % s.full)
        if pi.search(text):
            text = pi.sub(lambda m: iface, text, 1)
            text = pp.sub(lambda m: parser, text, 1)
        else:
            stem = s.full[:-5]
            first = D.first_iface(text, P, by, s)
            ifaces, parsers = "", ""
            for suffix in ("Inserted", "Modified", "Moved", "Patch", "Removed", "Rows"):
                m = re.search(r"export interface %s%s \{.*?\n\}\n\n" % (base, suffix), text, re.S)
                ifaces += iface + "\n" if suffix == "Patch" else m.group(0).replace(base, stem)
                parsers += parser if suffix == "Patch" else re.search(r"export const parse%s%s: [^\n]*\n" % (base, suffix), text).group(0).replace(base, stem)
            text = text[: text.index("export const parse%s:" % first)] + parsers + text[text.index("export const parse%s:" % first):]
            text = text[: text.index("export interface %s {" % first)] + ifaces + text[text.index("export interface %s {" % first):]
    m = re.search(r'^import \{ ([^}]*) \} from "(\.\./📸️snapshot/🟦️\.ts)";$', text, re.M)
    if m:
        names = set(x.strip() for x in m.group(1).split(","))
        for e in enum_imports:
            names |= {"type %s" % e, "parse%s" % e}
        text = text.replace(m.group(0), 'import { %s } from "%s";' % (", ".join(sorted(names, key=lambda x: x.replace("type ", ""))), m.group(2)))
    return text
