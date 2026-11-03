"""🪆 EN 1999 nested row kinds: insert-/remove- member-action, connection-action, section-element, sheet-action, shell-action.
Rows are addressed by the id of their parent (`memberId`, ...) plus the row id (remove) or the position (insert); the edit rules use the shared `IdRemoval`/`InsertItemRule` parent selectors. Usage: python -I en1999_nested.py [--apply] (merge mode, private stage, append-only crate roots)."""
import copy, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import en1999 as E, en1999_rows as R, wire as W

STAGE_DIR = TICKET + "/🗑️generated/en1999-nested/stage"
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1999/snapshot.json#/$defs/"


def nest(slug, row, ty, pcoll, pty, psel, ncoll, pdiff, en, de, pen, pde, fixture):
    return dict(slug=slug, pas=pascal(slug), snake=row, camel=camel(row), ty=ty, pcoll=pcoll, pcamel=camel(pcoll), psel=psel, pselc=camel(psel), ncoll=ncoll, pdiff=pdiff,
                en=en, de=de, pen=pen, pde=pde, add_de=de + " hinzufügen", rem_de=de + " entfernen", fixture=fixture, group="rows-" + slug,
                doc_en="The complete %s record to add." % en, doc_de="Der vollständige hinzuzufügende Datensatz: %s." % de, rem_en="The %s this mutation removes." % en, rem_de_doc="Entfernt wird: %s." % de)


ROWS = [
    nest("member-action", "action", "MemberAction", "members", "AluminiumMember", "member_id", "actions", "Members", "member action", "Bauteileinwirkung", "member", "Bauteil", "change-member-n-ed"),
    nest("connection-action", "action", "MemberAction", "connections", "AluminiumConnection", "connection_id", "actions", "Connections", "connection action", "Anschlusseinwirkung", "connection", "Anschluss", "change-conn-action-n-k"),
    nest("section-element", "element", "PlateElement", "sections", "AluminiumSection", "section_id", "elements", "Sections", "section element", "Querschnittsteil", "section", "Querschnitt", "change-element-width"),
    nest("sheet-action", "action", "MemberAction", "cold_formed", "ColdFormedSheet", "cold_formed_id", "actions", "ColdFormed", "cold-formed sheet action", "Einwirkung auf kaltgeformtes Blech", "cold-formed sheet", "Kaltgeformtes Blech", "change-sheet-action-n-k"),
    nest("shell-action", "action", "MemberAction", "shells", "AluminiumShell", "shell_id", "actions", "Shells", "shell action", "Schaleneinwirkung", "shell", "Schale", "change-shell-action-n-k"),
]


class NK(R.Kind):
    @property
    def parent_patch(self):
        return "En1999%sPatch" % self.r["pdiff"]

    @property
    def parent_rows(self):
        return "En1999%sRows" % self.r["pdiff"]

    @property
    def nest_rows(self):
        return "En1999%s%sRows" % (self.r["pdiff"], pascal(self.r["ncoll"]))

    @property
    def other(self):
        v = "remove" if self.verb == "add" else "insert"
        return v, "%s_%s" % (v, self.r["slug"].replace("-", "_")), pascal("%s-%s" % (v, self.r["slug"]))

    def fields(self):
        r = self.r
        return "    pub %s: String,\n" % r["psel"] + ("    pub index: u32,\n    pub %s: %s,\n" % (r["snake"], r["ty"]) if self.verb == "add" else "    pub id: String,\n")

    def gql_fields(self):
        r = self.r
        return "  %s: String! @state(class: ARTIFACT)\n" % r["pselc"] + ("  index: Int! @state(class: ARTIFACT)\n  %s: JSON! @state(class: ARTIFACT)\n" % r["camel"] if self.verb == "add" else "  id: String! @state(class: ARTIFACT)\n")

    def proto_fields(self):
        r = self.r
        head = "  // @state artifact\n  string %s = 1;\n" % r["psel"]
        return head + ("  // @state artifact\n  uint32 index = 2;\n  // @state artifact\n  bytes %s = 3;\n" % r["snake"] if self.verb == "add" else "  // @state artifact\n  string id = 2;\n")

    def dsl(self):
        r, rec, ps = self.r, self.record, self.r["psel"]
        if self.verb == "add":
            return ("    %s { %s: String, index: u32, payload_json: String },\n" % (rec, ps),
                    "        En1999Mutation::%s(p) => En1999MutationDsl::%s { %s: p.%s.clone(), index: p.index, payload_json: json_of(&p.%s) },\n" % (rec, rec, ps, ps, r["snake"]),
                    "        En1999MutationDsl::%s { %s, index, payload_json } => En1999Mutation::%s(%s::%s { %s, index, %s: from_json::<%s>(&payload_json) }),\n" % (rec, ps, rec, self.module, rec, ps, r["snake"], r["ty"]))
        return ("    %s { %s: String, id: String },\n" % (rec, ps),
                "        En1999Mutation::%s(p) => En1999MutationDsl::%s { %s: p.%s.clone(), id: p.id.clone() },\n" % (rec, rec, ps, ps),
                "        En1999MutationDsl::%s { %s, id } => En1999Mutation::%s(%s::%s { %s, id }),\n" % (rec, ps, rec, self.module, rec, ps))


def kinds():
    return [NK(r, v) for r in ROWS for v in ("add", "remove")]


def diff_rs(k):
    r = k.r
    names = dict(mod=k.module, rec=k.record, pp=k.parent_patch, pr=k.parent_rows, nr=k.nest_rows, pc=r["pcoll"], ps=r["psel"], nc=r["ncoll"], row=r["snake"], pen=r["pen"], en=r["en"], cap=r["en"].capitalize())
    if k.verb == "add":
        body = '''    if parent.%(nc)s.iter().any(|existing| existing.id == payload.%(row)s.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%(cap)s id {} already exists.", payload.%(row)s.id), [payload.%(row)s.id.clone()]);
    }
    if (payload.index as usize) > parent.%(nc)s.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.%(nc)s.len()), Vec::<String>::new());
    }
    let nested = %(nr)s::insertion(payload.index as usize, payload.%(row)s.clone());
''' % names
    else:
        body = '''    let Some(index) = parent.%(nc)s.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown %(en)s id {}", payload.id), Vec::<String>::new());
    };
    let nested = %(nr)s::removal(&parent.%(nc)s, index);
''' % names
    return '''//! 🔺️ `%(kind)s` diff.

use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, %(pp)s, %(pr)s, %(nr)s};

pub fn diff(payload: &%(rec)s, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.%(pc)s.iter().find(|row| row.id == payload.%(ps)s) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown %(pen)s id {}", payload.%(ps)s), vec![payload.%(ps)s.clone()]);
    };
%(body)s    protocol::MutationOutcome::new(En1999Diff { %(pc)s: Some(%(pr)s::modification(payload.%(ps)s.clone(), %(pp)s { %(nc)s: Some(nested), ..Default::default() })), ..Default::default() })
}
''' % dict(names, kind=k.kind, body=body)


def inverse_rs(k):
    r = k.r
    ov, omod, orec = k.other
    names = dict(pc=r["pcoll"], ps=r["psel"], nc=r["ncoll"], row=r["snake"], omod=omod, orec=orec)
    if k.verb == "add":
        body = "    vec![Op::from(%(omod)s::%(orec)s { %(ps)s: payload.%(ps)s.clone(), id: payload.%(row)s.id.clone() })]\n" % names
        sig = "_base"
    else:
        body = '''    match base.%(pc)s.iter().find(|row| row.id == payload.%(ps)s).and_then(|parent| parent.%(nc)s.iter().enumerate().find(|(_, row)| row.id == payload.id)) {
        Some((index, row)) => vec![Op::from(%(omod)s::%(orec)s { %(ps)s: payload.%(ps)s.clone(), index: index as u32, %(row)s: row.clone() })],
        None => Vec::new(),
    }
''' % names
        sig = "base"
    return '''//! ↩️ `%(kind)s` inverse.

use crate::%(mod)s::%(rec)s;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::%(omod)s;

pub fn inverse<Op>(payload: &%(rec)s, %(sig)s: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(omod)s::%(orec)s> {
    Ok((|| {
%(body)s    })())
}
''' % dict(kind=k.kind, mod=k.module, rec=k.record, omod=omod, orec=orec, sig=sig, body=body)


def ui(label_en, label_de, desc_en, desc_de, group, order, extra=None):
    d = {"role": "value", "label": {"en": label_en, "de": label_de}, "description": {"en": desc_en, "de": desc_de}, "group": group, "order": order}
    d.update(extra or {})
    return d


def schema_json(k):
    r = k.r
    lit = k.record[:1].lower() + k.record[1:]
    props = {"mutation": {"const": lit}, r["pselc"]: {"type": "string", "x-semio-ui": dict(ui(r["pen"].capitalize(), r["pde"], "The %s that owns the row, by id." % r["pen"], "Das Elternelement (%s), per Kennung." % r["pde"], "target", 10), widget="reference", ref={"kind": r["pcoll"].replace("_", "-")})}}
    if k.verb == "add":
        props["index"] = {"type": "integer", "minimum": 0, "x-semio-ui": dict(ui("Position", "Position", "Zero-based list position at which the record is inserted.", "Nullbasierte Listenposition, an der der Datensatz eingefügt wird.", "target", 20), widget="stepper", step=1, precision=0)}
        props[r["camel"]] = {"$ref": SNAP + r["ty"], "x-semio-ui": ui(r["en"].capitalize(), r["de"], r["doc_en"], r["doc_de"], "record", 30)}
        req = ["mutation", r["pselc"], "index", r["camel"]]
    else:
        props["id"] = {"type": "string", "x-semio-ui": dict(ui(r["en"].capitalize(), r["de"], r["rem_en"], r["rem_de_doc"], "target", 20), widget="reference", ref={"kind": r["slug"]})}
        req = ["mutation", r["pselc"], "id"]
    return dumps({"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1999/mutation/%s/schema.json" % k.kind, "title": k.record, "type": "object", "additionalProperties": False, "required": req, "properties": props})


def schema_ts(k):
    r = k.r
    lit = k.record[:1].lower() + k.record[1:]
    add = k.verb == "add"
    head = '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
''' % (k.emoji, k.kind, k.record)
    imp = "normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString" if add else "normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString"
    row_import = 'import { type %s, parse%s } from "../../../📸️snapshot/🟦️.ts";\n' % (r["ty"], r["ty"]) if add else ""
    members = '  mutation: "%s";\n  %s: string;\n' % (lit, r["pselc"]) + ("  index: number;\n  %s: %s;\n" % (r["camel"], r["ty"]) if add else "  id: string;\n")
    rdr = 'mutation: normWireRequired(normWireLiteral("%s")), %s: normWireRequired(normWireString)' % (lit, r["pselc"]) + (', index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), %s: normWireRequired(parse%s)' % (r["camel"], r["ty"]) if add else ", id: normWireRequired(normWireString)")
    return head + '''import { %s } from "@semio-tech/norm-js/registry/contract";
%s
export interface %s {
%s}

export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });
''' % (imp, row_import, k.record, members, k.record, k.record, k.record, rdr)


def write_kinds(art, F, ks, tags):
    gen = sys.modules[__name__]
    R.write_kinds(art, F, ks, tags, gen)


def edit_rules(art, F, ks):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = F.read(p)
    ins = "".join('        InsertItemRule { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("%s")], index: "index", item: "%s" },\n' % (k.r["pcamel"], camel(k.r["ncoll"]), k.kind, k.r["pselc"], k.r["camel"]) for k in ks if k.verb == "add")
    rem = "".join('        IdRemoval { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("%s")], id: "id" },\n' % (k.r["pcamel"], camel(k.r["ncoll"]), k.kind, k.r["pselc"]) for k in ks if k.verb == "remove")
    s = rep(s, "    ],\n    remove_item:", ins + "    ],\n    remove_item:", 1)
    s = rep(s, '},\n];\n\n/// 🎯️', "},\n" + rem + "];\n\n/// 🎯️", 1)
    F.write(p, s)


def null_patch(art, k):
    import glob
    path = glob.glob(art.fix + "/*" + k.r["fixture"] + "/✅apply/🔣️.diff.json")[0]
    patch = json.loads(rd(path))[camel(k.r["pcoll"])]["modified"][0]["patch"]
    return {key: None for key in patch}


def fixtures(art, F, ks, base, top):
    for k in ks:
        r = k.r
        parent = next(p for p in base[r["pcamel"]] if p.get(camel(r["ncoll"])))
        rows = parent[camel(r["ncoll"])]
        row = rows[0]
        empty = lambda: {"removed": [], "inserted": [], "moved": [], "modified": []}
        after = copy.deepcopy(base)
        ap = next(p for p in after[r["pcamel"]] if p["id"] == parent["id"])
        nested = empty()
        lit = k.record[:1].lower() + k.record[1:]
        d = art.fix + "/" + k.dirname + "/✅apply"
        if k.verb == "add":
            new = copy.deepcopy(row)
            new["id"] = row["id"] + "-2"
            at = len(rows)
            ap[camel(r["ncoll"])] = rows + [new]
            mut = {"mutation": lit, r["pselc"]: parent["id"], "index": at, r["camel"]: new}
            nested["inserted"] = [{"index": at, "row": new}]
            dupe = d.replace("✅apply", "⛔dupe")
            F.write(dupe + "/🔣️.before.json", dumps(base))
            F.write(dupe + "/🔣️.mutation.json", dumps({"mutation": lit, r["pselc"]: parent["id"], "index": 0, r["camel"]: copy.deepcopy(row)}))
            F.write(dupe + "/🔣️.after.json", dumps(base))
            F.write(dupe + "/🔣️.outcome.json", dumps({"status": "rejected", "code": "mutation.duplicate-id", "messages": [{"level": "fatal", "code": "mutation.duplicate-id"}]}))
            F.write(dupe + "/🔺️diff/🚫️.absent", "")
        else:
            ap[camel(r["ncoll"])] = rows[1:]
            mut = {"mutation": lit, r["pselc"]: parent["id"], "id": row["id"]}
            nested["removed"] = [{"id": row["id"], "index": 0}]
        patch = null_patch(art, k)
        patch[camel(r["ncoll"])] = nested
        modified = empty()
        modified["modified"] = [{"id": parent["id"], "patch": patch}]
        diff = {x: None for x in top}
        diff[r["pcamel"]] = modified
        F.write(d + "/🔣️.before.json", dumps(base))
        F.write(d + "/🔣️.mutation.json", dumps(mut))
        F.write(d + "/🔣️.after.json", dumps(after))
        F.write(d + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(d + "/🔣️.diff.json", dumps(diff))


def main():
    art = E.art
    F = Files(STAGE_DIR, merge=True)
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M))
    ks = [k for k in kinds() if k.kind not in existing]
    if not ks:
        print("nothing to do")
        return
    groups = {}
    for k in ks:
        groups.setdefault(k.r["group"], []).append(k)
    names = W.create_crates(art, F, groups, art.mut + "/" + art.anchor_dir)
    groups_of = {k.kind: names[g][0] for g, gk in groups.items() for k in gk}
    kinds_all = re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M) + [k.kind for k in ks]
    tags = {k: i for i, k in enumerate(kinds_all)}
    W.register(art, F, names)
    write_kinds(art, F, ks, tags)
    R.wire_twins(art, F, ks, groups_of)
    edit_rules(art, F, ks)
    for gk in groups.values():
        R.text_codec(art, F, gk, groups_of)
    W.binary_protocol(art, F, kinds_all)
    W.harness(art, F, ks, None)
    base = json.loads(rd(art.fix + "/🏋️change-member-n-ed/✅apply/🔣️.before.json"))
    top = list(json.loads(rd(art.fix + "/🏋️change-member-n-ed/✅apply/🔣️.diff.json")).keys())
    fixtures(art, F, ks, base, top)
    R.harness_dupes(art, F, ks)
    R.taxonomy(art, F, ks)
    R.oracle(art, F, ks)
    print("kinds", len(ks), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply(), "merged", len(F.merged))


if __name__ == "__main__":
    shutil.rmtree(STAGE_DIR, ignore_errors=True)
    main()
