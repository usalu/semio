"""EN 1997 nested row kinds: insert-/remove-<row> for loadCases, testProfiles, tests, points addressed by parent id plus position."""
import copy, json, os, re, shutil, sys, types
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import en1997 as E, wire as W

from en1997 import PRIVATE_STAGE
art = E.art
NEST = [
    ("load-case", "load_case", "LoadCase", "footings", "SpreadFoundation", "footing_id", "load_cases", "FoundationLoadCase", "Footing load case", "Fundament-Lastfall", "Footing", "Fundament"),
    ("test-profile", "test_profile", "TestProfile", "piles", "Pile", "pile_id", "test_profiles", "PileTestProfile", "Pile test profile", "Pfahl-Probebelastungsprofil", "Pile", "Pfahl"),
    ("anchor-test", "anchor_test", "AnchorTest", "anchors", "Anchor", "anchor_id", "tests", "AnchorTest", "Anchor test", "Ankerprüfung", "Anchor", "Anker"),
    ("lab-point", "lab_point", "LabPoint", "lab_tests", "LabTest", "lab_test_id", "points", "LabPoint", "Laboratory test point", "Laborversuchspunkt", "Laboratory test", "Laborversuch"),
]
GROUP = "nested-rows"
P = "En1997"


class K:
    def __init__(self, verb, row):
        slug, snake_row, pas, pcoll, pdef, psel, ncoll, ndef, en, de, pen, pde = row
        self.verb, self.slug, self.row_snake, self.pas, self.pcoll, self.pdef, self.psel, self.ncoll, self.ndef, self.en, self.de, self.pen, self.pde = verb, slug, snake_row, pas, pcoll, pdef, psel, ncoll, ndef, en, de, pen, pde
        self.kind = "%s-%s" % (verb, slug)
        self.module = self.kind.replace("-", "_")
        self.record = pascal(self.kind)
        self.dirname = emoji_dir("➕" if verb == "insert" else "➖", self.kind)
        self.ent = types.SimpleNamespace(emoji="🪆", en="nested row", name=slug, sels=[self.psel])
        self.field = self.slug.replace("-", "_")
        self.ty = "row"
        self.newc = camel(snake_row)

    @property
    def parent_patch(self):
        return "%s%sPatch" % (P, pascal(self.pcoll))

    @property
    def parent_rows(self):
        return "%s%sRows" % (P, pascal(self.pcoll))

    @property
    def nest_rows(self):
        return "%s%s%sRows" % (P, pascal(self.pcoll), pascal(self.ncoll))


def leaf_rs(k):
    ins = k.verb == "insert"
    other = ("remove" if ins else "insert") + "_" + k.slug.replace("-", "_")
    otherrec = pascal(("remove" if ins else "insert") + "-" + k.slug)
    fields = "    pub %s: String,\n    pub index: usize,\n" % k.psel + ("    pub %s: semio_s_artifact_norm_en1997_model::%s,\n" % (k.row_snake, k.ndef) if ins else "")
    en = "Insert %s" % k.en.lower() if ins else "Remove %s" % k.en.lower()
    de = "%s einfügen" % k.de if ins else "%s entfernen" % k.de
    return '''//! 🪆 `%(kind)s` payload.

use semio_s_artifact_norm_en1997_diff::En1997Diff;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct %(rec)s {
%(fields)s}

impl<Op> protocol::MutationKind<En1997Snapshot, Op> for %(rec)s where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::%(mod)s::%(rec)s> + From<crate::%(other)s::%(otherrec)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "%(verb)s", entity: "%(slug)s", kind: "%(kind)s", record: "%(past)s",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("%(en)s", "%(de)s")
    }
}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for %(rec)s where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::%(mod)s::%(rec)s> + From<crate::%(other)s::%(otherrec)s> {
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::%(mod)s::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::%(mod)s::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
''' % dict(kind=k.kind, rec=k.record, fields=fields, mod=k.module, other=other, otherrec=otherrec, verb=k.verb, slug=k.slug, past=("Inserted" if ins else "Removed") + k.pas, en=en, de=de)


def diff_rs(k):
    ins = k.verb == "insert"
    op = "%s::insertion(payload.index, payload.%s.clone())" % (k.nest_rows, k.row_snake) if ins else "%s::removal(&parent.%s, payload.index)" % (k.nest_rows, k.ncoll)
    guard = ""
    if ins:
        guard = '''    if parent.%(n)s.iter().any(|existing| existing.id == payload.%(r)s.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%(en)s id {} already exists.", payload.%(r)s.id), [payload.%(r)s.id.clone()]);
    }
    if payload.index > parent.%(n)s.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.%(n)s.len()), Vec::<String>::new());
    }
''' % dict(n=k.ncoll, r=k.row_snake, en=k.en)
    else:
        guard = '''    if payload.index >= parent.%(n)s.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("%(slug)s #{} missing", payload.index), vec![payload.index.to_string()]);
    }
''' % dict(n=k.ncoll, slug=k.slug)
    return '''use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, %(pp)s, %(pr)s, %(nr)s};

pub fn diff(payload: &%(rec)s, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.%(pc)s.iter().find(|row| row.id == payload.%(ps)s) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("%(pslug)s {} missing", payload.%(ps)s), vec![payload.%(ps)s.clone()]);
    };
%(guard)s    protocol::MutationOutcome::new(En1997Diff { %(pc)s: Some(%(pr)s::modification(&payload.%(ps)s, %(pp)s { %(nc)s: %(op)s, ..Default::default() })), ..Default::default() })
}
''' % dict(mod=k.module, rec=k.record, pp=k.parent_patch, pr=k.parent_rows, nr=k.nest_rows, pc=k.pcoll, ps=k.psel, pslug=k.pen.lower(), guard=guard, nc=k.ncoll, op=op)


def inverse_rs(k):
    ins = k.verb == "insert"
    otherm = ("remove" if ins else "insert") + "_" + k.slug.replace("-", "_")
    otherr = pascal(("remove" if ins else "insert") + "-" + k.slug)
    if ins:
        body = '''    let at = base.%(pc)s.iter().find(|row| row.id == payload.%(ps)s).map_or(0, |parent| payload.index.min(parent.%(nc)s.len()));
    Ok(vec![Op::from(%(oth)s { %(ps)s: payload.%(ps)s.clone(), index: at })])''' % dict(pc=k.pcoll, ps=k.psel, nc=k.ncoll, oth=otherr)
    else:
        body = '''    Ok(match base.%(pc)s.iter().find(|row| row.id == payload.%(ps)s).and_then(|parent| parent.%(nc)s.get(payload.index).cloned()) {
        Some(%(r)s) => vec![Op::from(%(oth)s { %(ps)s: payload.%(ps)s.clone(), index: payload.index, %(r)s })],
        None => Vec::new(),
    })''' % dict(pc=k.pcoll, ps=k.psel, nc=k.ncoll, r=k.row_snake, oth=otherr)
    return '''use crate::%(mod)s::%(rec)s;
use crate::%(otherm)s::%(otherr)s;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &%(rec)s, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(otherm)s::%(otherr)s> {
%(body)s
}
''' % dict(mod=k.module, rec=k.record, otherm=otherm, otherr=otherr, body=body)


def ui(label_en, label_de, desc_en, desc_de, group, order, extra=None):
    d = {"role": "value", "label": {"en": label_en, "de": label_de}, "description": {"en": desc_en, "de": desc_de}, "group": group, "order": order}
    d.update(extra or {})
    return d


def schema_json(k):
    ins = k.verb == "insert"
    props = {"mutation": {"const": camel(k.record[:1].lower() + k.record[1:]) if False else k.record[:1].lower() + k.record[1:]}}
    props[camel(k.psel)] = {"type": "string", "x-semio-ui": dict(ui(k.pen, k.pde, "The %s that owns the row, by id." % k.pen.lower(), "Das Elternelement (%s), per Kennung." % k.pde, "target", 10), widget="reference", ref={"kind": k.pcoll[:-1].replace("_", "-")})}
    props["index"] = {"type": "integer", "minimum": 0, "x-semio-ui": dict(ui("Position", "Position", "Zero-based list position of the row.", "Nullbasierte Listenposition der Zeile.", "target", 20), widget="stepper", step=1, precision=0)}
    req = ["mutation", camel(k.psel), "index"]
    if ins:
        props[camel(k.row_snake)] = {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1997/snapshot.json#/$defs/%s" % k.ndef, "x-semio-ui": ui(k.en, k.de, "The complete %s record to insert." % k.en.lower(), "Der vollständige einzufügende Datensatz (%s)." % k.de, "record", 30)}
        req.append(camel(k.row_snake))
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1997/mutation/%s/schema.json" % k.kind, "title": k.record, "type": "object", "additionalProperties": False, "required": req, "properties": props}


def schema_ts(k):
    ins = k.verb == "insert"
    lit = k.record[:1].lower() + k.record[1:]
    imp = "normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString"
    members = '  mutation: "%s";\n  %s: string;\n  index: number;\n' % (lit, camel(k.psel)) + ("  %s: %s;\n" % (camel(k.row_snake), k.ndef) if ins else "")
    rdr = 'mutation: normWireRequired(normWireLiteral("%s")), %s: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0}))' % (lit, camel(k.psel)) + (", %s: normWireRequired(parse%s)" % (camel(k.row_snake), k.ndef) if ins else "")
    row_import = 'import { parse%s, type %s } from "../../../📸️snapshot/🟦️.ts";\n' % (k.ndef, k.ndef) if ins else ""
    return '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { %s } from "@semio-tech/norm-js/registry/contract";
%s
export interface %s {
%s}

export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });
''' % ("➕" if ins else "➖", k.kind, k.record, imp, row_import, k.record, members, k.record, k.record, k.record, rdr)


def catalog(k, tag):
    return {"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/" + art.name + "/" + art.rel + "/" + k.dirname, "semanticKind": k.kind, "displayName": "%s %s" % (k.verb.capitalize(), pascal(k.slug)), "emoji": "➕" if k.verb == "insert" else "➖", "aggregateVariant": k.record, "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}


def rich_pick(base, k):
    for r in base[camel(k.pcoll)]:
        if r.get(camel(k.ncoll)):
            return r
    raise AssertionError(k.kind)


def fixtures(F, ks, base, top):
    for k in ks:
        parent = rich_pick(base, k)
        rows = parent[camel(k.ncoll)]
        row = rows[0]
        after = copy.deepcopy(base)
        ap = next(r for r in after[camel(k.pcoll)] if r["id"] == parent["id"])
        empty = lambda: {"removed": [], "inserted": [], "moved": [], "modified": []}
        nested = empty()
        if k.verb == "insert":
            new = copy.deepcopy(row)
            new["id"] = row["id"] + "-new"
            ap[camel(k.ncoll)] = [new] + rows
            mut = {"mutation": k.record[:1].lower() + k.record[1:], camel(k.psel): parent["id"], "index": 0, camel(k.row_snake): new}
            nested["inserted"] = [{"index": 0, "row": new}]
        else:
            ap[camel(k.ncoll)] = rows[1:]
            mut = {"mutation": k.record[:1].lower() + k.record[1:], camel(k.psel): parent["id"], "index": 0}
            nested["removed"] = [{"id": row["id"], "index": 0}]
        parent_rows = empty()
        parent_rows["modified"] = [{"id": parent["id"], "patch": {camel(k.ncoll): nested}}]
        diff = {x: None for x in top}
        diff[camel(k.pcoll)] = parent_rows
        b = art.fix + "/" + k.dirname + "/✅apply"
        F.write(b + "/🔣️.before.json", dumps(base))
        F.write(b + "/🔣️.mutation.json", dumps(mut))
        F.write(b + "/🔣️.after.json", dumps(after))
        F.write(b + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(b + "/🔣️.diff.json", dumps(diff))


def text_codec(F, ks):
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    crate = "semio_s_artifact_norm_en1997_mut_" + GROUP.replace("-", "_")
    s = rep(s, "use semio_s_artifact_norm_en1997_model::{", "use %s::{%s};\nuse semio_s_artifact_norm_en1997_model::{" % (crate, ", ".join(sorted(k.module for k in ks))), 1)
    have = s[s.index("use semio_s_artifact_norm_en1997_model::{"): s.index("};", s.index("use semio_s_artifact_norm_en1997_model::{"))]
    miss = sorted({k.ndef for k in ks if re.search(r"\b%s\b" % k.ndef, have) is None})
    if miss:
        s = rep(s, "use semio_s_artifact_norm_en1997_model::{", "use semio_s_artifact_norm_en1997_model::{%s, " % ", ".join(miss), 1)
    variants = to_dsl = from_dsl = ""
    for k in ks:
        if k.verb == "insert":
            variants += "    %s {\n        %s: String,\n        index: usize,\n        #[dsl(block)]\n        %s: %s,\n    },\n" % (k.record, k.psel, k.row_snake, k.ndef)
            to_dsl += "        En1997Mutation::%s(payload) => En1997MutationDsl::%s { %s: payload.%s.clone(), index: payload.index, %s: payload.%s.clone() },\n" % (k.record, k.record, k.psel, k.psel, k.row_snake, k.row_snake)
            from_dsl += "        En1997MutationDsl::%s { %s, index, %s } => En1997Mutation::%s(%s::%s { %s, index, %s }),\n" % (k.record, k.psel, k.row_snake, k.record, k.module, k.record, k.psel, k.row_snake)
        else:
            variants += "    %s { %s: String, index: usize },\n" % (k.record, k.psel)
            to_dsl += "        En1997Mutation::%s(payload) => En1997MutationDsl::%s { %s: payload.%s.clone(), index: payload.index },\n" % (k.record, k.record, k.psel, k.psel)
            from_dsl += "        En1997MutationDsl::%s { %s, index } => En1997Mutation::%s(%s::%s { %s, index }),\n" % (k.record, k.psel, k.record, k.module, k.record, k.psel)
    s = rep(s, "\n}\n\nimpl OpText for En1997MutationDsl", "\n" + variants.rstrip("\n") + "\n}\n\nimpl OpText for En1997MutationDsl", 1)
    a = "    }\n}\n\npub fn en1997_mutation_from_dsl"
    s = rep(s, a, to_dsl + a, 1)
    a = "    }\n}\n\nimpl OpText for En1997Mutation {"
    s = rep(s, a, from_dsl + a, 1)
    F.write(p, s)


def edit_rules(F, ks):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = F.read(p)
    ins = "".join('        InsertItemRule { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("%s")], index: "index", item: "%s" },\n' % (camel(k.pcoll), camel(k.ncoll), k.kind, camel(k.psel), camel(k.row_snake)) for k in ks if k.verb == "insert")
    rem = "".join('        RemoveItemRule { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("%s")], index: "index" },\n' % (camel(k.pcoll), camel(k.ncoll), k.kind, camel(k.psel)) for k in ks if k.verb == "remove")
    s = rep(s, "    ],\n    remove_item:", ins + "    ],\n    remove_item:", 1)
    s = rep(s, "    ],\n};\n\n/// 🆔️", rem + "    ],\n};\n\n/// 🆔️", 1)
    F.write(p, s)


def gql(k):
    if k.verb == "insert":
        return "type %s { mutation: String! %s: String! index: Int! %s: %sInput! }\n" % (k.record, camel(k.psel), camel(k.row_snake), k.ndef)
    return "type %s { mutation: String! %s: String! index: Int! }\n" % (k.record, camel(k.psel))


def main():
    F = Files(PRIVATE_STAGE)
    ks = [K(v, r) for r in NEST for v in ("insert", "remove")]
    kinds_all = re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M)
    if ks[0].kind in kinds_all:
        print("already present")
        return
    kinds_all += [k.kind for k in ks]
    tags = {k: i for i, k in enumerate(kinds_all)}
    names = W.create_crates(art, F, {GROUP: ks}, art.mut + "/" + art.anchor_dir)
    E.register(F, names)
    for k in ks:
        d = art.mut + "/" + k.dirname
        F.write(d + "/🦀️.rs", leaf_rs(k))
        F.write(d + "/🔺️diff/🦀️.rs", diff_rs(k))
        F.write(d + "/↩️inverse/🦀️.rs", inverse_rs(k))
        F.write(d + "/🧬️schema/🔣️.json", dumps(schema_json(k)))
        F.write(d + "/🧬️schema/🟦️.ts", schema_ts(k))
        F.write(d + "/🔣️.json", dumps(catalog(k, tags[k.kind])))
    groups_of = {k.kind: names[GROUP][0] for k in ks}
    E.root_twins(F, ks, groups_of, gql)
    g = F.read(art.mut + "/🔗️.graphql")
    inputs = "".join("input %sInput {\n%s}\n" % (n, "".join("  %s: %s\n" % (f, t) for f, t in fl)) for n, fl in [("FoundationLoadCase", [("id", "String!"), ("designSituation", "String!")] + [(x, "Float!") for x in ["verticalPermanent", "verticalVariable", "horizontalPermanent", "horizontalVariable", "momentPermanent", "momentVariable"]]), ("PileTestProfile", [("id", "String!"), ("shaftResistance", "Float!"), ("baseResistance", "Float!")])] if "input %sInput" % n not in g)
    F.write(art.mut + "/🔗️.graphql", rep(g, "union En1997Mutation =", inputs + "union En1997Mutation =", 1))
    edit_rules(F, ks)
    E.text_codec  # per-kind payloads differ; nested variants are written below
    text_codec(F, ks)
    W.binary_protocol(art, F, kinds_all)
    E.harness(F, ks)
    E.oracle(F, ks)
    tax_leaves = [types.SimpleNamespace(record=k.record, kind=k.kind, dirname=k.dirname, ent=types.SimpleNamespace(name=k.slug), field="row") for k in ks]
    W.taxonomy(art, F, tax_leaves)
    base = json.loads(rd(E.RICH))
    top = list(json.loads(rd(art.fix + "/⛰️change-slope-angle/✅apply/🔣️.diff.json")).keys())
    fixtures(F, ks, base, top)
    print("kinds", len(ks), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply())


if __name__ == "__main__":
    shutil.rmtree(PRIVATE_STAGE, ignore_errors=True)
    main()
