"""🆔 `remove-case` by design-case id for the six artifacts that embed the shared `cases` table (en1990, en1991, en1992, en1995, en1996, en1997).

`python -I cases_ids.py` rewrites per artifact: the leaf payload (`index` -> `id`), diff, inverse, schema json and ts twins, the `insert-case` inverse, the edit rules
(`RemoveItemRule cases` -> shared `IdRemoval` plus `resolve_edit`), the mutation root export, the editor wiring (`resolve_edit`), the text codecs, proto/graphql, the unit
tests and the committed vectors (the position becomes the id of the before snapshot's row). Idempotent: a rewritten file no longer matches the old text.

📎 [shared id removals](../../../../../../../../✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs)
"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *
sys.path.insert(0, HERE)
from id_rules import rewrite

EMOJI = {"en1990": "⚖️", "en1991": "🏋️", "en1992": "🏛️", "en1995": "🪵️", "en1996": "🪨️", "en1997": "🌍️"}
CHANGED = []


def sub(path, old, new, count=1, regex=False):
    text = rd(path)
    if regex:
        out, n = re.subn(old, new, text)
    else:
        n = text.count(old)
        out = text.replace(old, new)
    if n == 0:
        return False
    assert count is None or n == count, (path, old, n)
    open(path, "w", encoding="utf-8").write(out)
    CHANGED.append(path)
    return True


def write(path, text):
    if not os.path.exists(path) or rd(path) != text:
        open(path, "w", encoding="utf-8").write(text)
        CHANGED.append(path)


def id_schema(path):
    doc = json.loads(rd(path))
    doc["required"] = ["id"]
    doc["properties"] = {"id": {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": "Case", "de": "Fall"}, "description": {"en": "The catalogue case this mutation addresses, by id.", "de": "Der adressierte Katalogfall, per Kennung."}, "ref": {"kind": "case"}, "group": "target", "order": 10}}}
    write(path, dumps(doc))


TS = '''/** ➖️ `remove-case` wire twin: the leaf payload `RemoveCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface RemoveCase {
  id: string;
}

export const parseRemoveCase: NormWireReader<RemoveCase> = normWireObject<RemoveCase>({ id: normWireRequired(normWireString) });
'''


def artifact(short):
    P = short.replace("en", "En", 1)
    any_ = NORM + "/🗿️artifacts/" + EMOJI[short] + short + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
    mut = any_ + "/🧬️schema/🧬️mutations"
    leaf = mut + "/➖️remove-case"
    sub(leaf + "/🦀️.rs", "removes the catalogue case at an index", "removes the catalogue case by id")
    sub(leaf + "/🦀️.rs", "pub struct RemoveCase { pub index: usize }", "pub struct RemoveCase { pub id: String }")
    sub(leaf + "/🦀️.rs", 'format!("Remove catalogue case #{}", self.index), &format!("Katalogfall #{} entfernen", self.index)', 'format!("Remove catalogue case {}", self.id), &format!("Katalogfall {} entfernen", self.id)')
    sub(leaf + "/🦀️.rs", "vec![self.index.to_string()]", "vec![self.id.clone()]")
    sub(leaf + "/🔺️diff/🦀️.rs", "(&base.cases, payload.index)", "(&base.cases, &payload.id)")
    write(leaf + "/↩️inverse/🦀️.rs", '''use crate::remove_case::RemoveCase;
use crate::insert_case;
use semio_s_artifact_norm_%s_model::%sSnapshot;
pub fn inverse<Op>(payload: &RemoveCase, base: &%sSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_case::InsertCase> {
    Ok((|| {
    let Some((index, case)) = base.cases.iter().enumerate().find(|(_, case)| case.id == payload.id) else { return Vec::new(); };
    vec![Op::from(insert_case::InsertCase { index, case: case.clone() })]

    })())
}
''' % (short, P, P))
    id_schema(leaf + "/🧬️schema/🔣️.json")
    write(leaf + "/🧬️schema/🟦️.ts", TS)
    inv = mut + "/➕️insert-case/↩️inverse/🦀️.rs"
    sub(inv, "    let at = payload.index.min(base.cases.len());\n    vec![Op::from(remove_case::RemoveCase { index: at })]", "    vec![Op::from(remove_case::RemoveCase { id: payload.case.id.clone() })]")
    sub(inv, "pub fn inverse<Op>(payload: &InsertCase, base: &", "pub fn inverse<Op>(payload: &InsertCase, _base: &")
    edit_rules(short, P, mut + "/🧭️edit-rules/🦀️.rs")
    root = mut + "/🦀️.rs"
    sub(root, "pub use crate::edit_rules::EDIT_RULES;", "pub use crate::edit_rules::{resolve_edit, EDIT_RULES};")
    sub(root, "pub use crate::edit_rules::{resolve, EDIT_RULES};", "pub use crate::edit_rules::{resolve_edit, EDIT_RULES};")
    editor(short, any_ + "/✏️editor")
    codecs(short, P, any_)
    tests(short, P, any_)
    vectors(any_ + "/🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-case/✅apply")


def edit_rules(short, P, path):
    text = rd(path)
    out = rewrite(text, short, P, [("cases", "remove-case", "id")])
    if out != text:
        open(path, "w", encoding="utf-8").write(out)
        CHANGED.append(path)


def editor(short, ed):
    for p in glob.glob(ed + "/**/🦀️.rs", recursive=True):
        t = rd(p)
        n = t.replace("mutations::EDIT_RULES.resolve(document, edit)", "mutations::resolve_edit(document, edit)")
        n = re.sub(r"(semio_s_artifact_norm_%s_mutations::)resolve([,)])" % short, r"\1resolve_edit\2", n)
        if n != t:
            open(p, "w", encoding="utf-8").write(n)
            CHANGED.append(p)


def codecs(short, P, any_):
    text = any_ + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    sub(text, 'format!("remove-case index={}", enc_json(&p.index))', 'format!("remove-case id={}", enc_json(&p.id))') if short == "en1990" else None
    sub(text, '"remove-case" => Ok(En1990Mutation::RemoveCase(remove_case::RemoveCase { index: dec_json(arg("index")?)? }))', '"remove-case" => Ok(En1990Mutation::RemoveCase(remove_case::RemoveCase { id: dec_json(arg("id")?)? }))') if short == "en1990" else None
    if short == "en1992":
        sub(text, 'format!("remove-case index={}", p.index)', 'format!("remove-case id={}", enc_token(&p.id))')
        sub(text, '"remove-case" => Ok(En1992Mutation::RemoveCase(remove_case::RemoveCase { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }))', '"remove-case" => Ok(En1992Mutation::RemoveCase(remove_case::RemoveCase { id: dec_json(&get("id")?)? }))')
    if short == "en1997":
        sub(text, "RemoveCase { index: usize },", "RemoveCase { id: String },")
        sub(text, "En1997MutationDsl::RemoveCase { index: payload.index }", "En1997MutationDsl::RemoveCase { id: payload.id.clone() }")
        sub(text, "En1997MutationDsl::RemoveCase { index } => En1997Mutation::RemoveCase(remove_case::RemoveCase { index })", "En1997MutationDsl::RemoveCase { id } => En1997Mutation::RemoveCase(remove_case::RemoveCase { id })")
    if short in ("en1995", "en1996"):
        proto = any_ + "/🧬️schema/🧬️mutations/🛰️.proto"
        gql = any_ + "/🧬️schema/🧬️mutations/🔗️.graphql"
        sub(proto, r"message RemoveCase \{\n  uint32 index = 1;\n\}", "message RemoveCase {\n  string id = 1;\n}", regex=True)
        sub(gql, r"type RemoveCase \{\n  index: Int!?\n\}", "type RemoveCase {\n  id: String\n}", regex=True)


def tests(short, P, any_):
    for p in glob.glob(any_ + "/**/🦀️.rs", recursive=True):
        if "/⚡️" in p or "/target/" in p:
            continue
        t = rd(p)
        n = re.sub(r"(RemoveCase \{) index: 0 \}", r'\1 id: "case-1".into() }', t) if "remove_case::RemoveCase" in t else t
        if n != t:
            open(p, "w", encoding="utf-8").write(n)
            CHANGED.append(p)


def vectors(folder):
    mutation = folder + "/🔣️.mutation.json"
    wire = json.loads(rd(mutation))
    before = json.loads(rd(folder + "/🔣️.before.json"))
    body = wire["RemoveCase"] if "RemoveCase" in wire else wire
    if "index" not in body:
        return
    body["id"] = before["cases"][body.pop("index")]["id"]
    write(mutation, dumps(wire))


if __name__ == "__main__":
    for short in EMOJI:
        artifact(short)
    print("\n".join(sorted(set(p[len(SEMIO) + 1:] for p in CHANGED))))
    print(len(set(CHANGED)), "files")
