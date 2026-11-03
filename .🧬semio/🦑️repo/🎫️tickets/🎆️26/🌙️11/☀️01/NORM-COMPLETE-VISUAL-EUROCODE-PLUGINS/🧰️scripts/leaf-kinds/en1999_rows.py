"""EN 1999 row insert/remove kinds (insert-/remove- weld run, outline vertex, placement) cloned from the insert-member / remove-member shape."""
import copy, json, re, sys
from common import *
import leaf as L, wire as W

GROUP = "detail-rows"
GROUP_EMOJI = "🗂️"
ROWS = [
    dict(slug="weld-run", pas="WeldRun", snake="weld_run", coll="weld_runs", collp="WeldRuns", camel="weldRun", ccoll="weldRuns", ty="WeldRun", en="weld run", de="Schweißnaht", add_de="Schweißnaht hinzufügen", rem_de="Schweißnaht entfernen", doc_en="The complete weld run record to add.", doc_de="Der vollständige hinzuzufügende Schweißnaht-Datensatz.", rem_en="The weld run this mutation removes.", rem_de_doc="Die Schweißnaht, die diese Mutation entfernt."),
    dict(slug="outline-vertex", pas="OutlineVertex", snake="outline_vertex", coll="outline_vertices", collp="OutlineVertices", camel="outlineVertex", ccoll="outlineVertices", ty="SectionVertex", en="outline vertex", de="Umrissknoten", add_de="Umrissknoten hinzufügen", rem_de="Umrissknoten entfernen", doc_en="The complete section outline vertex record to add.", doc_de="Der vollständige hinzuzufügende Umrissknoten-Datensatz.", rem_en="The outline vertex this mutation removes.", rem_de_doc="Der Umrissknoten, den diese Mutation entfernt."),
    dict(slug="placement", pas="Placement", snake="placement", coll="placements", collp="Placements", camel="placement", ccoll="placements", ty="Placement", en="placement", de="Platzierung", add_de="Platzierung hinzufügen", rem_de="Platzierung entfernen", doc_en="The complete placement record to add.", doc_de="Der vollständige hinzuzufügende Platzierungs-Datensatz.", rem_en="The placement this mutation removes.", rem_de_doc="Die Platzierung, die diese Mutation entfernt."),
]


class Ent:
    def __init__(self, r):
        self.name, self.emoji, self.en, self.de = r["slug"], GROUP_EMOJI, r["en"], r["de"]


class Kind:
    def __init__(self, r, verb):
        self.r, self.verb = r, verb
        self.emoji = "➕" if verb == "add" else "➖"
        self.kind = "%s-%s" % ("insert" if verb == "add" else verb, r["slug"])
        self.module = self.kind.replace("-", "_")
        self.record = pascal(self.kind)
        self.dirname = self.emoji + self.kind
        self.ent = Ent(r)
        self.field = self.kind.replace("-", "_")
        self.covered = False

    def fields(self):
        return "    pub index: u32,\n    pub %s: %s,\n" % (self.r["snake"], self.r["ty"]) if self.verb == "add" else "    pub id: String,\n"

    def gql_fields(self):
        return "  index: Int! @state(class: ARTIFACT)\n  %s: JSON! @state(class: ARTIFACT)\n" % self.r["camel"] if self.verb == "add" else "  id: String! @state(class: ARTIFACT)\n"

    def proto_fields(self):
        return "  // @state artifact\n  uint32 index = 1;\n  // @state artifact\n  bytes %s = 2;\n" % self.r["snake"] if self.verb == "add" else "  // @state artifact\n  string id = 1;\n"

    def dsl(self):
        """The text-codec triple (enum variant, to-dsl arm, from-dsl arm) of the kind."""
        r, rec = self.r, self.record
        if self.verb == "add":
            return ("    %s { index: u32, payload_json: String },\n" % rec,
                    "        En1999Mutation::%s(p) => En1999MutationDsl::%s { index: p.index, payload_json: json_of(&p.%s) },\n" % (rec, rec, r["snake"]),
                    "        En1999MutationDsl::%s { index, payload_json } => En1999Mutation::%s(%s::%s { index, %s: from_json::<%s>(&payload_json) }),\n" % (rec, rec, self.module, rec, r["snake"], r["ty"]))
        return ("    %s { id: String },\n" % rec,
                "        En1999Mutation::%s(p) => En1999MutationDsl::%s { id: p.id.clone() },\n" % (rec, rec),
                "        En1999MutationDsl::%s { id } => En1999Mutation::%s(%s::%s { id }),\n" % (rec, rec, self.module, rec))

    @property
    def other(self):
        v = "remove" if self.verb == "add" else "insert"
        return v, "%s_%s" % (v, self.r["snake"]), pascal("%s-%s" % (v, self.r["slug"]))


def kinds():
    return [Kind(r, v) for r in ROWS for v in ("add", "remove")]


def pseudo_group(ks):
    return {GROUP: ks}


def header(k, desc):
    return '''//! %(e)s `%(kind)s` mutation leaf%(desc)s.

use semio_s_artifact_norm_en1999_diff::En1999Diff;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
''' % dict(e=k.emoji, kind=k.kind, desc=desc)


def leaf_rs(k):
    r = k.r
    ov, omod, orec = k.other
    add = k.verb == "add"
    head = header(k, "").replace("\n\nuse semio_s_artifact_norm_en1999_model::En1999Snapshot;", "\nuse semio_s_artifact_norm_en1999_model::En1999Snapshot;" + ("\nuse semio_s_artifact_norm_en1999_model::%s;" % r["ty"] if add else ""))
    fields = k.fields()
    label_en = ("Add " if add else "Remove ") + r["en"]
    label_de = r["add_de"] if add else r["rem_de"]
    rec_tail = ("Added" if add else "Removed") + pascal(r["slug"])
    inv = '''    fn inverse(&self, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::%(mod)s::inverse::inverse(self, base)?

    })
}''' % dict(mod=k.module)
    return head + '''
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct %(rec)s {
%(fields)s}

impl<Op> protocol::MutationKind<En1999Snapshot, Op> for %(rec)s where Op: protocol::Mutation<En1999Snapshot, Diff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::%(omod)s::%(orec)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "%(verb)s",
        entity: "%(slug)s",
        kind: "%(kind)s",
        record: "%(tail)s",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("%(len)s", "%(lde)s")
    }
}

impl<Op> protocol::ColdMutationKind<En1999Snapshot, Op> for %(rec)s where Op: protocol::ColdMutation<En1999Snapshot, ReferenceDiff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::%(omod)s::%(orec)s> {
    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::%(mod)s::diff::diff(self, base)
    }
%(inv)s
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
''' % dict(rec=k.record, fields=fields, omod=omod, orec=orec, verb=k.verb, slug=r["slug"], kind=k.kind, tail=rec_tail, len=label_en, lde=label_de, mod=k.module, inv=inv)


def diff_rs(k):
    r = k.r
    rows = "En1999%sRows" % r["collp"]
    if k.verb == "add":
        body = '''    if base.%(coll)s.iter().any(|existing| existing.id == payload.%(snake)s.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%(cap)s id {} already exists.", payload.%(snake)s.id), [payload.%(snake)s.id.clone()]);
    }
    if (payload.index as usize) > base.%(coll)s.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.%(coll)s.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { %(coll)s: Some(%(rows)s::insertion(payload.index as usize, payload.%(snake)s.clone())), ..Default::default() })
''' % dict(coll=r["coll"], snake=r["snake"], cap=r["en"].capitalize(), rows=rows)
    else:
        body = '''    let Some(index) = base.%(coll)s.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown %(en)s id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { %(coll)s: Some(%(rows)s::removal(&base.%(coll)s, index)), ..Default::default() })
''' % dict(coll=r["coll"], en=r["en"], rows=rows)
    return '''//! 🔺️ `%(kind)s` diff.

use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, %(rows)s};

pub fn diff(payload: &%(rec)s, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
%(body)s}
''' % dict(kind=k.kind, mod=k.module, rec=k.record, rows=rows, body=body)


def inverse_rs(k):
    r = k.r
    ov, omod, orec = k.other
    if k.verb == "add":
        body = "    vec![Op::from(%s::%s { id: payload.%s.id.clone() })]\n" % (omod, orec, r["snake"])
        sig = "_base"
    else:
        body = '''    if let Some((index, row)) = base.%(coll)s.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(%(omod)s::%(orec)s { index: index as u32, %(snake)s: row.clone() })]
    } else { Vec::new() }
''' % dict(coll=r["coll"], omod=omod, orec=orec, snake=r["snake"])
        sig = "base"
    return '''//! ↩️ `%(kind)s` inverse.

use crate::%(mod)s::%(rec)s;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::%(omod)s;

pub fn inverse<Op>(payload: &%(rec)s, %(sig)s: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(omod)s::%(orec)s> {
    Ok((|| {
%(body)s
    })())
}
''' % dict(kind=k.kind, mod=k.module, rec=k.record, omod=omod, orec=orec, sig=sig, body=body)


def schema_json(k):
    r = k.r
    lit = k.record[:1].lower() + k.record[1:]
    base = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1999/mutation/%s/schema.json" % k.kind, "title": k.record, "type": "object", "additionalProperties": False}
    if k.verb == "add":
        props = {
            "mutation": {"const": lit},
            "index": {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Position", "de": "Position"}, "description": {"en": "Zero-based list position at which the record is inserted.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird."}, "step": 1, "precision": 0, "group": "target", "order": 10}},
            r["camel"]: {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1999/snapshot.json#/$defs/" + r["ty"], "x-semio-ui": {"role": "value", "label": {"en": r["en"].capitalize(), "de": r["de"]}, "description": {"en": r["doc_en"], "de": r["doc_de"]}, "group": "record", "order": 20}},
        }
        req = ["mutation", "index", r["camel"]]
    else:
        props = {
            "mutation": {"const": lit},
            "id": {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": r["en"].capitalize(), "de": r["de"]}, "description": {"en": r["rem_en"], "de": r["rem_de_doc"]}, "ref": {"kind": r["slug"]}, "group": "target", "order": 10}},
        }
        req = ["mutation", "id"]
    return dumps({**base, "required": req, "properties": props})


def schema_ts(k):
    r = k.r
    lit = k.record[:1].lower() + k.record[1:]
    head = '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
''' % (k.emoji, k.kind, k.record)
    if k.verb == "add":
        return head + '''import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";
import { type %(ty)s, parse%(ty)s } from "../../../📸️snapshot/🟦️.ts";

export interface %(rec)s {
  mutation: "%(lit)s";
  index: number;
  %(camel)s: %(ty)s;
}

export const parse%(rec)s: NormWireReader<%(rec)s> = normWireObject<%(rec)s>({ mutation: normWireRequired(normWireLiteral("%(lit)s")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), %(camel)s: normWireRequired(parse%(ty)s) });
''' % dict(rec=k.record, lit=lit, ty=r["ty"], camel=r["camel"])
    return head + '''import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface %(rec)s {
  mutation: "%(lit)s";
  id: string;
}

export const parse%(rec)s: NormWireReader<%(rec)s> = normWireObject<%(rec)s>({ mutation: normWireRequired(normWireLiteral("%(lit)s")), id: normWireRequired(normWireString) });
''' % dict(rec=k.record, lit=lit)


def write_kinds(art, F, ks, tags, gen=None):
    gen = gen or sys.modules[__name__]
    for k in ks:
        d = art.mut + "/" + k.dirname
        F.write(d + "/🦀️.rs", leaf_rs(k))
        F.write(d + "/🔺️diff/🦀️.rs", gen.diff_rs(k))
        F.write(d + "/↩️inverse/🦀️.rs", gen.inverse_rs(k))
        F.write(d + "/🧬️schema/🔣️.json", gen.schema_json(k))
        F.write(d + "/🧬️schema/🟦️.ts", gen.schema_ts(k))
        cat = L.catalog_json(k, tags[k.kind], "✏️s/🔌️plugins/📕️norm/🗿️artifacts/" + art.name + "/" + art.rel)
        cat["displayName"] = ("Add " if k.verb == "add" else "Remove ") + k.r["pas"]
        F.write(d + "/🔣️.json", dumps(cat))


def wire_twins(art, F, ks, groups_of):
    P = art.P
    p = art.mut + "/🦀️.rs"
    s = F.read(p)
    uses = "".join("use %s::%s;\n" % (groups_of[k.kind].replace("-", "_"), k.module) for k in ks)
    m = re.search(r"\nuse [\w:]+;\n\n//#region 🔖️Mutations", s)
    s = s[: m.start() + 1] + "".join(u for u in uses.splitlines(True) if u not in s) + s[m.start() + 1:]
    s = rep(s, "\n}\n\npub const KINDS", "\n" + "".join("    %s(%s::%s),\n" % (k.record, k.module, k.record) for k in ks).rstrip("\n") + "\n}\n\npub const KINDS", 1)
    s = rep(s, "\n];\n//#endregion 🔖️Mutations", "\n" + "".join('    "%s",\n' % k.kind for k in ks).rstrip("\n") + "\n];\n//#endregion 🔖️Mutations", 1)
    F.write(p, s)
    p = art.mut + "/🟦️.ts"
    s = F.read(p)
    imports = "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (k.record, k.record, k.dirname) for k in ks)
    i = s.index("\nexport type %sMutation =" % P)
    s = s[:i].rstrip("\n") + "\n" + imports + s[i:]
    j = s.index(";\n", s.index("export type %sMutation =" % P))
    s = s[:j] + "".join("\n  | %s" % k.record for k in ks) + s[j:]
    kk = s.index("\n});", s.index("normWireTagged"))
    s = s[:kk] + "".join("\n  %s: parse%s," % (k.record[:1].lower() + k.record[1:], k.record) for k in ks) + s[kk:]
    F.write(p, s)
    p = art.mut + "/🔣️.json"
    j = json.loads(F.read(p))
    for k in ks:
        j["oneOf"].append({"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1999/mutation/%s/schema.json" % k.kind})
    F.write(p, "{\n" + ",\n".join("  %s:%s" % (json.dumps(a), json.dumps(v, ensure_ascii=False, separators=(",", ":")) if a != "oneOf" else "[\n" + ",\n".join("    " + json.dumps(x, separators=(",", ":")) for x in v) + "\n  ]") for a, v in j.items()) + "\n}\n")
    p = art.mut + "/🔗️.graphql"
    s = F.read(p)
    u = s.index("\n\n# KINDS:")
    s = s[:u] + "".join("\n  | %s" % k.record for k in ks) + s[u:]
    s = re.sub(r"(# KINDS: [^\n]*)", lambda m: m.group(1) + "".join(", " + k.kind for k in ks), s, 1)
    blocks = ""
    for k in ks:
        flds = k.gql_fields()
        blocks += 'type %s {\n  # kind: "%s"\n%s}\n\n' % (k.record, k.kind, flds)
    F.write(p, s.rstrip("\n") + "\n\n" + blocks.rstrip("\n") + "\n")
    p = art.mut + "/🛰️.proto"
    s = F.read(p)
    nums = [int(x) for x in re.findall(r"= (\d+);", s[s.index("message %sMutation {" % P):])]
    n = max(nums)
    blocks, oneof = "", ""
    for k in ks:
        flds = k.proto_fields()
        blocks += "// kind: %s\nmessage %s {\n%s}\n\n" % (k.kind, k.record, flds)
        n += 1
        oneof += "    %s %s = %d;\n" % (k.record, k.kind.replace("-", "_"), n)
    i = s.index("message %sMutation {" % P)
    s = s[:i] + blocks + s[i:]
    j = s.index("  }\n}", i)
    s = s[:j] + oneof + s[j:]
    F.write(p, s)


def edit_rules(art, F, ks):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = F.read(p)
    ins = "".join('        InsertItemRule { path: "%s", kind: "%s", selectors: &[], index: "index", item: "%s" },\n' % (k.r["ccoll"], k.kind, k.r["camel"]) for k in ks if k.verb == "add")
    s = rep(s, "    ],\n    remove_item:", ins + "    ],\n    remove_item:", 1)
    rem = "".join('        IdRemoval { path: "%s", kind: "%s", selectors: &[], id: "id" },\n' % (k.r["ccoll"], k.kind) for k in ks if k.verb == "remove")
    s = rep(s, '        IdRemoval { path: "members", kind: "remove-member", selectors: &[], id: "id" },\n', '        IdRemoval { path: "members", kind: "remove-member", selectors: &[], id: "id" },\n' + rem, 1)
    F.write(p, s)


def text_codec(art, F, ks, groups_of):
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    mods = sorted(k.module for k in ks)
    have = s[s.index("use semio_s_artifact_norm_en1999_model::{"): s.index("};", s.index("use semio_s_artifact_norm_en1999_model::{"))]
    types = sorted({k.r["ty"] for k in ks if re.search(r"\b%s\b" % k.r["ty"], have) is None})
    s = rep(s, "use semio_s_artifact_norm_en1999_model::{", "use %s::{%s};\nuse semio_s_artifact_norm_en1999_model::{%s" % (groups_of[ks[0].kind].replace("-", "_"), ", ".join(mods), "".join(t + ", " for t in types)), 1)
    variants, to_dsl, from_dsl = "", "", ""
    for k in ks:
        v, t, f = k.dsl()
        variants, to_dsl, from_dsl = variants + v, to_dsl + t, from_dsl + f
    s = rep(s, "\n}\n\nimpl OpText for En1999MutationDsl", "\n" + variants.rstrip("\n") + "\n}\n\nimpl OpText for En1999MutationDsl", 1)
    a = "    }\n}\n\npub fn en1999_mutation_from_dsl"
    s = rep(s, a, to_dsl + a, 1)
    a = "    }\n}\n\nimpl OpText for En1999Mutation {"
    s = rep(s, a, from_dsl + a, 1)
    F.write(p, s)


def fixtures(art, F, ks, base):
    for k in ks:
        r = k.r
        d = art.fix + "/" + k.dirname + "/✅apply"
        rows = base[r["ccoll"]]
        top = {key: None for key in base_top(base)}
        if k.verb == "add":
            row = copy.deepcopy(rows[0])
            row["id"] = row["id"] + "-2"
            before = copy.deepcopy(base)
            after = copy.deepcopy(base)
            index = len(rows)
            after[r["ccoll"]] = rows + [row]
            mut = {"mutation": k.record[:1].lower() + k.record[1:], "index": index, r["camel"]: row}
            top[r["ccoll"]] = {"removed": [], "inserted": [{"index": index, "row": row}], "moved": [], "modified": []}
            dupe = d.replace("✅apply", "⛔dupe")
            dmut = {"mutation": mut["mutation"], "index": 0, r["camel"]: copy.deepcopy(rows[0])}
            F.write(dupe + "/🔣️.before.json", dumps(base))
            F.write(dupe + "/🔣️.mutation.json", dumps(dmut))
            F.write(dupe + "/🔣️.after.json", dumps(base))
            F.write(dupe + "/🔣️.outcome.json", dumps({"status": "rejected", "code": "mutation.duplicate-id", "messages": [{"level": "fatal", "code": "mutation.duplicate-id"}]}))
            F.write(dupe + "/🔺️diff/🚫️.absent", "")
        else:
            before = copy.deepcopy(base)
            after = copy.deepcopy(base)
            after[r["ccoll"]] = rows[1:]
            mut = {"mutation": k.record[:1].lower() + k.record[1:], "id": rows[0]["id"]}
            top[r["ccoll"]] = {"removed": [{"id": rows[0]["id"], "index": 0}], "inserted": [], "moved": [], "modified": []}
        F.write(d + "/🔣️.before.json", dumps(before))
        F.write(d + "/🔣️.mutation.json", dumps(mut))
        F.write(d + "/🔣️.after.json", dumps(after))
        F.write(d + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(d + "/🔣️.diff.json", dumps(top))


def base_top(base):
    return [k for k in TOP_KEYS]


TOP_KEYS = []


def harness_dupes(art, F, ks):
    adds = [k for k in ks if k.verb == "add"]
    p = art.harness + "/🦀️.rs"
    s = F.read(p)
    s = W.append_list(s, "const ROWS: &[&str] = &[", "".join('    "%s-dupe",\n' % k.kind for k in adds))
    arms = ""
    for k in adds:
        arms += '        "%s-dupe" => (\n' % k.kind + "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/⛔dupe/🔣️.%s.json"),\n' % (k.dirname, x) for x in ["before", "mutation", "after", "outcome"]) + "        ),\n"
    a = '        other => panic!("mutate-en1999-1: no committed fixture'
    s = rep(s, a, arms + a, 1)
    F.write(p, s)
    p = art.harness + "/🐍️.py"
    s = F.read(p)
    s = rep(s, '    "insert-member-dupe": ("insert-member", "➕insert-member", "⛔dupe"),\n', '    "insert-member-dupe": ("insert-member", "➕insert-member", "⛔dupe"),\n' + "".join('    "%s-dupe": ("%s", "%s", "⛔dupe"),\n' % (k.kind, k.kind, k.dirname) for k in adds), 1)
    F.write(p, s)
    p = art.harness + "/🥒️.feature"
    s = F.read(p)
    rows = "".join("      | %s-dupe | %s | ⛔dupe  |\n" % (k.kind, k.dirname) for k in adds)
    s = rep(s, "      | insert-member-dupe               | ➕insert-member                     | ⛔dupe   |\n", "      | insert-member-dupe               | ➕insert-member                     | ⛔dupe   |\n" + rows, 1)
    F.write(p, s)


def taxonomy(art, F, ks):
    t = json.loads(F.read(TAXONOMY))
    name = art.name
    last = max(i for i, x in enumerate(t["rows"]) if x["artifact"] == name)
    have = {r["kind"] for r in t["rows"] if r["artifact"] == name}
    new = [{"aggregateVariant": k.record, "artifact": name, "entity": k.r["slug"], "kind": k.kind, "module": k.dirname, "physicalLayout": "direct", "record": ("Added" if k.verb == "add" else "Removed") + pascal(k.r["slug"]), "source": "%s/%s/%s/🦀️.rs" % (name, art.rel, k.dirname), "standard": "🔖️1", "subset": "✳️any", "type": k.record, "verb": k.verb} for k in ks if k.kind not in have]
    t["rows"][last + 1: last + 1] = new
    F.write(TAXONOMY, dumps(t))


def oracle(art, F, ks):
    p = art.any + "/🔮️oracles/🔣️.json"
    o = json.loads(F.read(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    for k in ks:
        src = "%s-member" % ("insert" if k.verb == "add" else k.verb)
        if k.kind in cat["kinds"]:
            continue
        v = copy.deepcopy([x for x in cat["vectors"] if x["mutationId"] == src][0])
        v["mutationId"] = k.kind
        v["sourceMutationDirectoryName"] = v["mutationDirectoryName"] = k.dirname
        for sc in v["scenarios"]:
            if sc["id"].endswith("-dupe"):
                sc["id"] = k.kind + "-dupe"
        cat["vectors"].append(v)
        cat["kinds"].append(k.kind)
        m = copy.deepcopy([x for x in man["mutations"] if x["id"] == src][0])
        m["id"] = k.kind
        m["productionDispatch"]["operation"] = k.kind
        m["productionDispatch"]["variant"] = k.record
        man["mutations"].append(m)
    F.write(p, dumps(o))
