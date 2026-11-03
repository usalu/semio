"""EN 1993 array-element kinds: insert-/remove-/change- kinds for the eight `Vec` element lists (`actions`, `spectrum`), wired additively next to the leaf kinds. Rerun is idempotent (skips kinds already in the root enum). python -I en1993_arrays.py [--apply]"""
import copy, json, os, re, shutil, sys, types
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import common, leaf as L, model as M, delta as D, wire as W
import en1993 as X, en1993_wire as Wr, en1993_base

art = X.art
ARRAYS = [
    ("SteelJoint", "joint-action", "actions", "JointForceAction", "Joint action", "Anschlusseinwirkung", "jointAction"),
    ("FatigueDetail", "fatigue-band", "spectrum", "FatigueBand", "Fatigue band", "Ermüdungsband", "fatigueBand"),
    ("ColdFormedMember", "cf-action", "actions", "ForceAction", "Cold-formed action", "Kaltprofil-Einwirkung", "coldFormedAction"),
    ("PlatedPanel", "plated-action", "actions", "ForceAction", "Plated panel action", "Plattenfeld-Einwirkung", "platedAction"),
    ("TensionComponent", "tension-action", "actions", "ForceAction", "Tension component action", "Zugglied-Einwirkung", "tensionAction"),
    ("TowerLeg", "tower-action", "actions", "ForceAction", "Tower leg action", "Turmstiel-Einwirkung", "towerAction"),
    ("SteelPile", "pile-action", "actions", "ForceAction", "Pile action", "Pfahl-Einwirkung", "pileAction"),
    ("CraneRunway", "crane-action", "actions", "ForceAction", "Crane runway action", "Kranbahn-Einwirkung", "craneAction"),
]
Op = "Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff>"
ColdOp = "Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff>"
DERIVE = '''#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]'''
MOUNTS = '''
//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
'''
ERR = 'return protocol::MutationOutcome::error("mutation.target-missing", "%s", Vec::<String>::new());'


class K:
    pass


def build(c0):
    """Builds the kind specs: list of K with file texts and metadata."""
    defs = c0.defs
    rs = rd(art.any + "/🧬️schema/📸️snapshot/🦀️.rs")
    out = []
    for df, slug, arr, ed, en, de, item in ARRAYS:
        if ("pub %s: Vec<%s>" % (arr, ed)) not in rs:
            continue
        row = c0.by_def[df]
        coll = row.chain[0][0]
        patch = c0.patches[df]
        delta = c0.deltas[patch]
        em = row.emoji
        ent = M.Ent(slug, em, [(coll, df, "id"), (arr, ed, "element_id")], en, de, slug=slug)
        efields = D.struct_fields(rs, ed)
        pref = "Joint" if False else ""
        rec_ins, rec_rem = pascal("insert-" + slug), pascal("remove-" + slug)
        crate = art.crate + slug.replace("cf-action", "cold-formed-actions").replace("joint-action", "joint-actions").replace("fatigue-band", "fatigue-spectrum").replace("plated-action", "plated-actions").replace("tension-action", "tension-actions").replace("tower-action", "tower-actions").replace("pile-action", "pile-actions").replace("crane-action", "crane-actions")
        group = crate[len(art.crate):]
        gdir = art.mut + "/" + emoji_dir(em, group)
        specs = []
        ins_k, rem_k = "insert-" + slug, "remove-" + slug
        mk = lambda kind, ent_=ent: types.SimpleNamespace(kind=kind, record=pascal(kind), module=kind.replace("-", "_"), dirname=emoji_dir("➕️" if kind.startswith("insert") else "➖️" if kind.startswith("remove") else em, kind), ent=ent_, crate=crate, group=group, gdir=gdir, files={}, row=row, coll=coll, arr=arr, ed=ed, item=item, df=df)
        # insert
        i = mk(ins_k)
        i.imp = ", %s" % ed
        i.fields = "    pub id: String,\n    pub index: Option<usize>,\n    pub element: %s,\n" % ed
        i.other = " + From<crate::%s::%s>" % (rem_k.replace("-", "_"), rec_rem)
        i.verb, i.entity = "insert", slug
        i.files["🦀️.rs"] = '''//! ➕️ `%(kind)s` mutation leaf: inserts one %(en)s element into the addressed row.

use semio_s_artifact_norm_en1993_model::{%(ed)s, En1993Snapshot};

%(derive)s
pub struct %(rec)s {
    pub id: String,
    pub index: Option<usize>,
    pub element: %(ed)s,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for %(rec)s where %(op)s + From<crate::%(rmod)s::%(rrec)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "%(kind_e)s", kind: "%(kind)s", record: "Inserted%(rtail)s" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert %(enl)s into {}", self.id), &format!("%(de)s in {} einfügen", self.id))
    }
    fn target(&self) -> Vec<String> { vec![self.id.clone(), self.element.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for %(rec)s where %(cold)s + From<crate::%(rmod)s::%(rrec)s> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::%(mod)s::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { crate::%(mod)s::inverse::inverse(self, base) }
}
%(mounts)s''' % dict(kind=i.kind, en=en, enl=en.lower(), de=de, ed=ed, derive=DERIVE, rec=i.record, op=Op, cold=ColdOp, rmod=rem_k.replace("-", "_"), rrec=rec_rem, kind_e=slug, rtail=i.record[len("Insert"):], mod=i.module, mounts=MOUNTS)
        i.files["🔺️diff/🦀️.rs"] = '''//! ➕️ `%(kind)s` diff: inserts the element into the addressed row's list, clamped to its end.

use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, %(delta)s, %(patch)s};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &%(rec)s, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.%(coll)s.iter().find(|row| row.id == payload.id) else {
        %(err)s
    };
    if row.%(arr)s.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%(en)s id {} already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    let mut value = row.%(arr)s.clone();
    let at = payload.index.unwrap_or(usize::MAX).min(value.len());
    value.insert(at, payload.element.clone());
    protocol::MutationOutcome::new(En1993Diff { %(coll)s: %(delta)s::modification(payload.id.clone(), %(patch)s { %(arr)s: Some(value), ..Default::default() }), ..Default::default() })
}
''' % dict(kind=i.kind, mod=i.module, rec=i.record, delta=delta, patch=patch, coll=coll, arr=arr, en=en, err=ERR % slug)
        i.files["↩️inverse/🦀️.rs"] = '''use crate::%(mod)s::%(rec)s;
use crate::%(rmod)s;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &%(rec)s, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(rmod)s::%(rrec)s> {
    Ok(match base.%(coll)s.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(%(rmod)s::%(rrec)s { id: payload.id.clone(), index: payload.index.unwrap_or(usize::MAX).min(row.%(arr)s.len()) })],
        None => Vec::new(),
    })
}
''' % dict(mod=i.module, rec=i.record, rmod=rem_k.replace("-", "_"), rrec=rec_rem, coll=coll, arr=arr)
        i.schema = ("insert", ed)
        specs.append(i)
        # remove
        r = mk(rem_k)
        r.files["🦀️.rs"] = '''//! ➖️ `%(kind)s` mutation leaf: removes the %(en)s element at an index from the addressed row.

use semio_s_artifact_norm_en1993_model::En1993Snapshot;

%(derive)s
pub struct %(rec)s {
    pub id: String,
    pub index: usize,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for %(rec)s where %(op)s + From<crate::%(imod)s::%(irec)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "%(kind_e)s", kind: "%(kind)s", record: "Removed%(rtail)s" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove %(enl)s #{} of {}", self.index, self.id), &format!("%(de)s #{} von {} entfernen", self.index, self.id))
    }
    fn target(&self) -> Vec<String> { vec![self.id.clone(), self.index.to_string()] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for %(rec)s where %(cold)s + From<crate::%(imod)s::%(irec)s> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::%(mod)s::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { crate::%(mod)s::inverse::inverse(self, base) }
}
%(mounts)s''' % dict(kind=r.kind, en=en, enl=en.lower(), de=de, derive=DERIVE, rec=r.record, op=Op, cold=ColdOp, imod=i.module, irec=i.record, kind_e=slug, rtail=r.record[len("Remove"):], mod=r.module, mounts=MOUNTS)
        r.files["🔺️diff/🦀️.rs"] = '''//! ➖️ `%(kind)s` diff: removes the element at the index from the addressed row's list.

use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, %(delta)s, %(patch)s};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &%(rec)s, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.%(coll)s.iter().find(|row| row.id == payload.id) else {
        %(err)s
    };
    if payload.index >= row.%(arr)s.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("%(kind_e)s index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut value = row.%(arr)s.clone();
    value.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { %(coll)s: %(delta)s::modification(payload.id.clone(), %(patch)s { %(arr)s: Some(value), ..Default::default() }), ..Default::default() })
}
''' % dict(kind=r.kind, mod=r.module, rec=r.record, delta=delta, patch=patch, coll=coll, arr=arr, kind_e=slug, err=ERR % slug)
        r.files["↩️inverse/🦀️.rs"] = '''use crate::%(mod)s::%(rec)s;
use crate::%(imod)s;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &%(rec)s, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(imod)s::%(irec)s> {
    Ok(match base.%(coll)s.iter().find(|row| row.id == payload.id).and_then(|row| row.%(arr)s.get(payload.index)) {
        Some(element) => vec![Op::from(%(imod)s::%(irec)s { id: payload.id.clone(), index: Some(payload.index), element: element.clone() })],
        None => Vec::new(),
    })
}
''' % dict(mod=r.module, rec=r.record, imod=i.module, irec=i.record, coll=coll, arr=arr)
        r.schema = ("remove", ed)
        specs.append(r)
        # set element field kinds
        for name, p in defs[ed]["properties"].items():
            if name == "id":
                continue
            f = snake(name)
            ty = efields[f].strip()
            l = M.Leaf(ent, [f], ty, p.get("x-semio-ui") or {}, False)
            l.kind = M.shorten("change-%s-%s" % (slug, kebab(f)), M.kind_limit())
            l.module, l.record, l.dirname = l.kind.replace("-", "_"), pascal(l.kind), emoji_dir(em, l.kind)
            s = types.SimpleNamespace(kind=l.kind, record=l.record, module=l.module, dirname=l.dirname, ent=ent, crate=crate, group=group, gdir=gdir, files={}, row=row, coll=coll, arr=arr, ed=ed, item=item, df=df, leaf=l, schema=("set", ed))
            cl = "" if l.copy else ".clone()"
            guard = '    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n' % l.new if ty == "f64" else ""
            s.files["🦀️.rs"] = L.leaf_rs(l)
            s.files["🔺️diff/🦀️.rs"] = '''//! 🔺️ `%(kind)s` diff: patches `%(arr)s[].%(f)s` of the addressed row's list.

use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, %(delta)s, %(patch)s};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &%(rec)s, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
%(guard)s    let Some(row) = base.%(coll)s.iter().find(|row| row.id == payload.id) else {
        %(err)s
    };
    let Some(at) = row.%(arr)s.iter().position(|element| element.id == payload.element_id) else {
        %(err)s
    };
    if row.%(arr)s[at].%(f)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The element already has this value.");
    }
    let mut value = row.%(arr)s.clone();
    value[at].%(f)s = payload.%(new)s%(cl)s;
    protocol::MutationOutcome::new(En1993Diff { %(coll)s: %(delta)s::modification(payload.id.clone(), %(patch)s { %(arr)s: Some(value), ..Default::default() }), ..Default::default() })
}
''' % dict(kind=l.kind, arr=arr, f=f, mod=l.module, rec=l.record, delta=delta, patch=patch, guard=guard, coll=coll, err=ERR % slug, new=l.new, cl=cl)
            s.files["↩️inverse/🦀️.rs"] = L.inverse_rs(l)
            sj = D.schema_json(l, defs)
            sj["required"].insert(1, "elementId")
            sj["properties"] = {"id": sj["properties"]["id"], "elementId": {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": en, "de": de}, "description": {"en": "The %s element this mutation addresses, by id." % en.lower(), "de": "Das adressierte Element (%s), per Kennung." % de}, "ref": {"kind": slug}, "group": "target", "order": 15}}, **{k: v for k, v in sj["properties"].items() if k != "id"}}
            s.files["🧬️schema/🔣️.json"] = dumps(sj)
            s.files["🧬️schema/🟦️.ts"] = D.schema_ts(l).replace("  id: string;\n", "  id: string;\n  elementId: string;\n").replace("{ id: normWireRequired(normWireString), ", "{ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), ")
            specs.append(s)
        out += specs
    return out


def struct_schema(s, defs):
    ed = s.ed
    ref = {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1993/snapshot.json#/$defs/" + ed}
    id_prop = {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": s.ent.en, "de": s.ent.de}, "ref": {"kind": s.ent.name}, "group": "target", "order": 10}}
    if s.schema[0] == "insert":
        props = {"id": id_prop, "index": {"type": ["integer", "null"], "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Position", "de": "Position"}, "step": 1, "precision": 0, "group": "target", "order": 20}}, "element": dict(ref, **{"x-semio-ui": {"role": "value", "label": {"en": s.ent.en, "de": s.ent.de}, "group": "record", "order": 30}})}
        req = ["id", "element"]
    else:
        props = {"id": id_prop, "index": {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "target", "label": {"en": "Position", "de": "Position"}, "step": 1, "precision": 0, "group": "target", "order": 20}}}
        req = ["id", "index"]
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1993/mutation/%s/schema.json" % s.kind, "title": s.record, "type": "object", "additionalProperties": False, "required": req, "properties": props}


def struct_ts(s):
    ed = s.ed
    if s.schema[0] == "insert":
        return '''/** ➕️ `%(k)s` wire twin: the leaf payload `%(r)s`, exactly as `./🦀️.rs` writes it.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type %(e)s, parse%(e)s } from "../../../📸️snapshot/🟦️.ts";

export interface %(r)s {
  id: string;
  index?: number | null;
  element: %(e)s;
}

export const parse%(r)s: NormWireReader<%(r)s> = normWireObject<%(r)s>({ id: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), element: normWireRequired(parse%(e)s) });
''' % dict(k=s.kind, r=s.record, e=ed)
    return '''/** ➖️ `%(k)s` wire twin: the leaf payload `%(r)s`, exactly as `./🦀️.rs` writes it.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface %(r)s {
  id: string;
  index: number;
}

export const parse%(r)s: NormWireReader<%(r)s> = normWireObject<%(r)s>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
''' % dict(k=s.kind, r=s.record)


def run():
    shutil.rmtree(X.STAGE93, ignore_errors=True)
    c0 = X.main()
    F = c0.F
    F.written = set()
    existing_root = set(re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M))
    specs = [s for s in build(c0) if s.kind not in existing_root]
    kinds_all = [k for k in re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M)] + [s.kind for s in specs]
    tags = {k: i for i, k in enumerate(kinds_all)}
    crates = {}
    base_toml = art.mut + "/🪝️cranes/📦️packages/🦀️rust/Cargo.toml"
    for s in specs:
        crates.setdefault(s.group, []).append(s)
    names = {}
    for g, ss in crates.items():
        e = ss[0].ent
        body = "//! %s EN 1993 %s element mutation kinds.\n\n" % (e.emoji, e.en.lower()) + Wr.PREAMBLE + "//#region 🪢️Mounts\n" + "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (s.dirname, s.module) for s in ss) + "//#endregion 🪢️Mounts\n"
        F.write(ss[0].gdir + "/🦀️.rs", body)
        F.write(ss[0].gdir + "/📦️packages/🦀️rust/Cargo.toml", W.crate_toml(art, ss[0].crate, base_toml))
        names[g] = (ss[0].crate, ss[0].gdir)
    for s in specs:
        d = art.mut + "/" + s.dirname
        for rel, text in s.files.items():
            F.write(d + "/" + rel, text)
        if s.schema[0] != "set":
            F.write(d + "/🧬️schema/🔣️.json", dumps(struct_schema(s, c0.defs)))
            F.write(d + "/🧬️schema/🟦️.ts", struct_ts(s))
        cat = L.catalog_json(types.SimpleNamespace(kind=s.kind, record=s.record, dirname=s.dirname, ent=s.ent), tags[s.kind], Wr.ROOT_REL)
        cat["displayName"] = s.record
        F.write(d + "/🔣️.json", dumps(cat))
    c = types.SimpleNamespace(F=F, retired=[], row_kinds=[], new_leaves=specs, leaf_by_kind={s.kind: s for s in specs}, crate_of={s.kind: s.crate for s in specs}, row_mods={}, kinds_all=kinds_all, tags=tags, defs=c0.defs, by_def=c0.by_def, patches=c0.patches, patch_fields=c0.patch_fields, items=c0.items, ents=c0.ents, keep=[])
    Wr.register(F, c, names)
    Wr.root_rs(F, c)
    Wr.root_ts(F, c)
    Wr.root_json(F, c)
    Wr.text_codec(F, c)
    Wr.oracle(F, c)
    Wr.schema_index(F, c)
    harness(F, c)
    appenders(F, c)
    fixtures(F, c, c0)
    print(len(specs), "kinds", len(names), "crates", len(F.written), "files")
    if "--apply" in sys.argv:
        print("applied", F.apply())


def appenders(F, c):
    ss = c.new_leaves
    p = art.mut + "/🔗️.graphql"
    t = F.read(p).rstrip("\n") + "\n"
    for s in ss:
        if s.schema[0] == "set":
            t += "type %s { id: String! @state(class: ARTIFACT) elementId: String! @state(class: ARTIFACT) %s: %s! @state(class: ARTIFACT) }\n" % (s.record, s.leaf.newc, L.gql_ty(s.leaf))
        elif s.schema[0] == "insert":
            t += "type %s { id: String! @state(class: ARTIFACT) index: Int @state(class: ARTIFACT) }\n" % s.record
        else:
            t += "type %s { id: String! @state(class: ARTIFACT) index: Int! @state(class: ARTIFACT) }\n" % s.record
    F.write(p, t)
    p = art.mut + "/🛰️.proto"
    t = F.read(p)
    last = max(int(x) for x in re.findall(r"= (\d+);\n  \}\n\}\n$", t) or re.findall(r"= (\d+);", t.split("message En1993Mutation {")[1]))
    msgs = ""
    one = ""
    for n, s in enumerate(ss, last + 1):
        if s.schema[0] == "set":
            msgs += "message %s { string id = 1; string element_id = 2; %s %s = 3; }\n" % (s.record, "string" if s.leaf.ty == "String" else L.proto_ty(s.leaf), s.leaf.new)
        elif s.schema[0] == "insert":
            msgs += "message %s { string id = 1; uint32 index = 2; bytes payload = 3; }\n" % s.record
        else:
            msgs += "message %s { string id = 1; uint32 index = 2; }\n" % s.record
        one += "    %s %s = %d;\n" % (s.record, s.kind.replace("-", "_"), n)
    head, tail = t.split("message En1993Mutation {", 1)
    tail = tail.rstrip("\n")
    assert tail.endswith("  }\n}"), tail[-30:]
    F.write(p, head.rstrip("\n") + "\n" + msgs + "\nmessage En1993Mutation {" + tail[: -len("  }\n}")] + one + "  }\n}\n")
    p = art.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    t = F.read(p)
    n = len(re.findall(r"^record ", t, re.M))
    F.write(p, t + "".join("record %s tag=%d\nfield payload bytes\n" % (s.kind, n + i) for i, s in enumerate(ss)))
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    t = F.read(p)
    sets = "".join('        SetFieldRule { path: "%s[].%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("id"), SelectorKey::Id("elementId")], value: "%s" },\n' % (camel(s.coll), s.arr, camel(s.leaf.path[0]), s.kind, s.leaf.newc) for s in ss if s.schema[0] == "set")
    ins = "".join('        InsertItemRule { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("id")], index: "index", item: "element" },\n' % (camel(s.coll), s.arr, s.kind) for s in ss if s.schema[0] == "insert")
    rem = "".join('        RemoveItemRule { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("id")], index: "index" },\n' % (camel(s.coll), s.arr, s.kind) for s in ss if s.schema[0] == "remove")
    t = rep(t, "    ],\n    insert_item:", sets + "    ],\n    insert_item:")
    t = rep(t, "    ],\n    remove_item:", ins + "    ],\n    remove_item:")
    t = rep(t, "    ],\n};\n\n//#region 🧭️LeafEdits", rem + "    ],\n};\n\n//#region 🧭️LeafEdits")
    F.write(p, t)
    p = TAXONOMY
    tax = json.loads(F.read(p))
    last = max(i for i, x in enumerate(tax["rows"]) if x["artifact"] == art.name)
    rows = []
    for s in ss:
        verb = s.kind.split("-")[0]
        rows.append({"aggregateVariant": s.record, "artifact": art.name, "entity": s.ent.name + ("-" + kebab(s.leaf.field) if s.schema[0] == "set" else ""), "kind": s.kind, "module": s.dirname, "physicalLayout": "direct", "record": {"change": "Changed", "insert": "Inserted", "remove": "Removed"}[verb] + s.record[len(verb):], "source": "%s/%s/%s/🦀️.rs" % (art.name, art.rel, s.dirname), "standard": "🔖️1", "subset": "✳️any", "type": s.record, "verb": verb})
    tax["rows"][last + 1: last + 1] = rows
    F.write(p, dumps(tax))
    for s in ss:
        pass


def harness(F, c):
    h = art.harness
    new = [s.kind for s in c.new_leaves]
    p = h + "/🦀️.rs"
    s = F.read(p)
    i = s.index("const KINDS: &[&str] = &[")
    j = s.index("\n];", i)
    s = s[:j] + "\n" + "".join('    "%s",\n' % k for k in new).rstrip("\n") + s[j:]
    arms = "".join('    ("%s", fixture!("%s")),\n' % (k, c.leaf_by_kind[k].dirname) for k in new)
    s = rep(s, "];\n\n/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector", arms + "];\n\n/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector")
    total = len(re.findall(r'^    "[a-z0-9-]+",$', s[i:s.index("\n];", i)], re.M))
    s = re.sub(r"current \d+-kind", "current %d-kind" % total, s, count=1)
    F.write(p, s)
    p = h + "/🐍️.py"
    s = F.read(p)
    s = rep(s, "\n]\n\n#: 🧫️ The committed specification vectors", "\n" + "".join('    "%s",\n' % k for k in new).rstrip("\n") + "\n]\n\n#: 🧫️ The committed specification vectors")
    s = rep(s, '    "change-annex": ("🌍️change-annex", "✅apply"),\n', '    "change-annex": ("🌍️change-annex", "✅apply"),\n' + "".join('    "%s": ("%s", "✅apply"),\n' % (k, c.leaf_by_kind[k].dirname) for k in new))
    F.write(p, s)
    p = h + "/🥒️.feature"
    s = F.read(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (k, c.leaf_by_kind[k].dirname) for k in new)
    s = re.sub(r"(      \| remove-crane-runway +\|[^\n]*\n)", lambda m: m.group(1) + rows, s)
    s = re.sub(r"The \d+ `✅apply` vectors", "The %d `✅apply` vectors" % total, s, count=1)
    F.write(p, s)


def fixtures(F, c, c0):
    defs = c0.defs
    base = en1993_base.base_doc(art, defs)
    diff0 = json.loads(rd(art.fix + "/➕️insert-crane-runway/✅apply/🔣️.diff.json"))
    lanes = lambda: {"removed": [], "inserted": [], "moved": [], "modified": []}
    top = ["annex"] + [camel(e.chain[0][0]) for e in c0.ents]
    for k in diff0:
        if k not in top:
            top.append(k)
    for s in c.new_leaves:
        key = camel(s.coll)
        row = base[key][0]
        els = row[s.arr]
        if not els:
            els = row[s.arr] = [X.synth(defs, s.ed, s.arr[:1] + "-1", [])]
            if "loadCaseId" in els[0]:
                els[0]["loadCaseId"] = base["loadCases"][0]["id"]
        before, after = copy.deepcopy(base), copy.deepcopy(base)
        first = els[0]
        patch_json = {camel(f): None for f in c0.patch_fields[c0.patches[s.df]]}
        ar = after[key][0][s.arr]
        if s.schema[0] == "set":
            f = camel(s.leaf.path[0])
            v = X.new_value(s.leaf, first[f])
            ar[0][f] = v
            mut = {s.record: {"id": row["id"], "elementId": first["id"], s.leaf.newc: v}}
        elif s.schema[0] == "insert":
            extra = copy.deepcopy(first)
            extra["id"] = first["id"] + "-2"
            ar.append(extra)
            mut = {s.record: {"id": row["id"], "index": len(ar) - 1, "element": extra}}
        else:
            extra = copy.deepcopy(first)
            extra["id"] = first["id"] + "-2"
            before[key][0][s.arr] = before[key][0][s.arr] + [extra]
            mut = {s.record: {"id": row["id"], "index": len(ar)}}
        patch_json[s.arr] = after[key][0][s.arr]
        diff = {k: (None if k == "annex" else lanes()) for k in top}
        diff[key]["modified"] = [{"id": row["id"], "patch": patch_json}]
        out = art.fix + "/" + s.dirname + "/✅apply"
        F.write(out + "/🔣️.before.json", dumps(before))
        F.write(out + "/🔣️.mutation.json", dumps(mut))
        F.write(out + "/🔣️.after.json", dumps(after))
        F.write(out + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(out + "/🔣️.diff.json", dumps(diff))


if __name__ == "__main__":
    run()
