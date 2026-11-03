#!/usr/bin/env python3
"""Lifts the EN 1995 `cases` layer onto the shared design-case rows (compliance `design_case::{CaseRow, CaseRowDelta, add_case, remove_case, set_case_number, set_case_text}`).
Run once: /home/deck/semio/.venv/bin/python -I catalogue-bridge-en1995-lift.py [--apply]. Stages every edit in memory, asserts every anchor, writes only with --apply."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import glob, json, os, re, shutil, sys
ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/"
ANY = ART + "🏅️standards/🔖️1/🪆️subsets/✳️any/"
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/"
MUT = ANY + "🧬️schema/🧬️mutations/"
FIX = ANY + "🧪️tests/🧫️fixtures/🧬️mutations/"
OLD = ["insert-case-number", "remove-case-number", "change-case-number-value", "insert-case-text", "remove-case-text", "change-case-text-value"]
OLD_DIRS = {"insert-case-number": "➕️", "remove-case-number": "➖️", "change-case-number-value": "🔢️", "insert-case-text": "➕️", "remove-case-text": "➖️", "change-case-text-value": "🔤️"}
OLD_DIRS = {k: v + k for k, v in OLD_DIRS.items()}
OLD_TYPES = ["InsertCaseNumber", "RemoveCaseNumber", "ChangeCaseNumberValue", "InsertCaseText", "RemoveCaseText", "ChangeCaseTextValue"]
OLD_SNAKE = [k.replace("-", "_") for k in OLD]
NEW = [
    {"kind": "set-case-number", "dir": "🔢️set-case-number", "T": "SetCaseNumber", "snake": "set_case_number", "emoji": "🔢️", "name": "Set Case Number", "record": "SetCaseNumber", "entity": "case-number"},
    {"kind": "set-case-text", "dir": "🔤️set-case-text", "T": "SetCaseText", "snake": "set_case_text", "emoji": "🔤️", "name": "Set Case Text", "record": "SetCaseText", "entity": "case-text"},
]
MOVE = ["change_case_label_en", "change_case_label_de", "change_case_entry_id"]
stage = {}; delete = []
def rd(p): return stage[p] if p in stage else open(p, encoding="utf-8").read()
def wr(p, s): stage[p] = s
def sub1(p, old, new, count=1):
    s = rd(p); assert old in s, (p, old[:80]); wr(p, s.replace(old, new, count))
def resub(p, pat, new, flags=0):
    s = rd(p); n, c = re.subn(pat, new, s, flags=flags); assert c, (p, pat[:80]); wr(p, n)
def rm(p): delete.append(p)
def jd(o): return json.dumps(o, indent=2, ensure_ascii=False)

# ---- model
P = ANY + "🧬️schema/📸️snapshot/🦀️.rs"
s = rd(P)
a = s.index("/// 🔢 One numeric input of a catalogue case"); b = s.index("/// 🔗 Dowel-type timber connection")
s = s[:a] + s[b:]
s = s.replace("use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::AnnexChoice;\npub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};", 1)
assert "pub use semio_s_plugin_norm_compliance::design_case" in s
wr(P, s)

# ---- diff
P = ANY + "🧬️schema/🔺️diff/🦀️.rs"
s = rd(P)
a = s.rfind("protocol::row_patch! {", 0, s.index("En1995CaseNumberPatch of"))
e = s.index("pub En1995CaseDelta {"); b = s.index("\n}\n", e) + 3
s = s[:a].rstrip("\n") + "\n" + s[b:]
assert "pub cases: En1995CaseDelta," in s
s = s.replace("pub cases: En1995CaseDelta,", "pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,")
assert "En1995Case" not in s, [m for m in re.findall(r"En1995Case\w*", s)]
wr(P, s)

# ---- rename CatalogueCase -> CaseRow in rust sources outside the leaves being deleted
for p in glob.glob(ANY + "**/*.rs", recursive=True):
    if "/target/" in p or any(("/" + d + "/") in p for d in OLD_DIRS.values()): continue
    t = rd(p)
    if "CatalogueCase" in t:
        wr(p, re.sub(r"\bCatalogueCase\b", "CaseRow", t))

# ---- timber: neutral conversion from the shared rows
P = ANY + "🧬️schema/⚖️timber/🏗️structure-and-members/🦀️.rs"
s = rd(P)
a = s.index("/// 🗂️ The neutral design case of a document case."); b = s.index("fn evaluate_member")
s = s[:a] + s[b:]
s = s.replace("evaluate_design_case(&design_case_of(c), annex)", "evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(c), annex)")
wr(P, s)
P = ANY + "🧬️schema/🧪️tests/⚖️compliance/🦀️.rs"
sub1(P, "        let case: DesignCase = design_case_of(&bending(\"c\", 1.0));", "        let case: DesignCase = DesignCase::from(&bending(\"c\", 1.0));")
sub1(P, "    use semio_s_artifact_norm_en1995_model::{CaseNumber, CaseText, CaseRow};", "    use semio_s_artifact_norm_en1995_model::{CaseNumber, CaseRow, CaseText};")

# ---- leaves: delete old, rewrite kept, create new
for d in OLD_DIRS.values():
    rm(MUT + d)
    rm(FIX + d)
rm(MUT + "🔢️case-inputs/🦀️.rs") if False else None
IMPORT_RD = "use semio_s_plugin_norm_compliance::design_case::{CaseRowDelta, CaseRowPatch};\n"
def diff_file(d, body_imports, fn_body):
    return body_imports + fn_body
P = MUT + "➕️insert-case/🔺️diff/🦀️.rs"
wr(P, """use crate::insert_case::InsertCase;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_kernel::En1995Cases;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn diff(payload: &InsertCase, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    match semio_s_plugin_norm_compliance::design_case::add_case::<En1995Cases>(&base.cases, payload.index, &payload.case) {
        Ok(cases) => protocol::MutationOutcome::new(En1995Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
""")
P = MUT + "➖️remove-case/🔺️diff/🦀️.rs"
wr(P, """use crate::remove_case::RemoveCase;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn diff(payload: &RemoveCase, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    match semio_s_plugin_norm_compliance::design_case::remove_case(&base.cases, payload.index) {
        Ok(cases) => protocol::MutationOutcome::new(En1995Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
""")
for kind, field in [("change_case_label_en", "label_en"), ("change_case_label_de", "label_de"), ("change_case_entry_id", "entry_id")]:
    P = MUT + {"change_case_label_en": "🏷️change-case-label-en", "change_case_label_de": "🏷️change-case-label-de", "change_case_entry_id": "📇️change-case-entry-id"}[kind] + "/🔺️diff/🦀️.rs"
    s = rd(P)
    s = s.replace("use semio_s_artifact_norm_en1995_diff::{En1995CaseDelta, En1995CasePatch};", "use semio_s_plugin_norm_compliance::design_case::{CaseRowDelta, CaseRowPatch};")
    s = s.replace("En1995CaseDelta::modification", "CaseRowDelta::modification").replace("En1995CasePatch {", "CaseRowPatch {")
    assert "En1995Case" not in s, P
    wr(P, s)

def leaf_rs(L):
    T, k, sn = L["T"], L["kind"], L["snake"]
    if k == "set-case-number":
        fields, val_t, lab_en, lab_de = "pub case_id: String, pub key: String, pub value: Option<f64>", "number", "Set number input {1} of case {0}", "Zahleneingabe {1} von Fall {0} setzen"
    else:
        fields, val_t, lab_en, lab_de = "pub case_id: String, pub key: String, pub value: Option<String>", "text", "Set text input {1} of case {0}", "Texteingabe {1} von Fall {0} setzen"
    return f'''//! {L["emoji"]} `{k}` — sets, adds or (with an empty value) removes one {val_t} input of a design case; the diff and inverse are the shared `design_case` logic.
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct {T} {{ {fields} }}
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for {T} where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::{sn}::{T}> {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "set", entity: "{L["entity"]}", kind: "{k}", record: "{L["record"]}" }};

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {{ semio_framework_ui_locale::LocalizedLabel::native(&format!("{lab_en}", self.case_id, self.key), &format!("{lab_de}", self.case_id, self.key)) }}
    fn target(&self) -> Vec<String> {{ vec![self.case_id.clone(), self.key.clone()] }}
}}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for {T} where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::{sn}::{T}> {{
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> {{ crate::{sn}::diff::diff(self, base) }}
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {{
    Ok({{ crate::{sn}::inverse::inverse(self, base)? 
    }})
}}
}}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
'''
def leaf_diff(L):
    T, sn = L["T"], L["snake"]
    call = "set_case_number(&base.cases, &payload.case_id, &payload.key, payload.value)" if sn == "set_case_number" else "set_case_text(&base.cases, &payload.case_id, &payload.key, payload.value.as_deref())"
    return f'''use crate::{sn}::{T};
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn diff(payload: &{T}, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {{
    match semio_s_plugin_norm_compliance::design_case::{call} {{
        Ok(cases) => protocol::MutationOutcome::new(En1995Diff {{ cases, ..Default::default() }}),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }}
}}
'''
def leaf_inverse(L):
    T, sn = L["T"], L["snake"]
    prev = "previous_number(&base.cases, &payload.case_id, &payload.key)" if sn == "set_case_number" else "previous_text(&base.cases, &payload.case_id, &payload.key)"
    return f'''use crate::{sn}::{T};
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &{T}, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{sn}::{T}> {{
    Ok((|| {{
    if !base.cases.iter().any(|item| item.id == payload.case_id) {{ return Vec::new(); }}
    vec![Op::from({T} {{ case_id: payload.case_id.clone(), key: payload.key.clone(), value: semio_s_plugin_norm_compliance::design_case::{prev} }})]

    }})())
}}
'''
def leaf_json(L, tag):
    return jd({"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + L["dir"], "semanticKind": L["kind"], "displayName": L["name"], "emoji": L["emoji"], "aggregateVariant": L["T"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}) + "\n"
def ui(widget, role, en, de, den, dde, order, group, **kw):
    d = {"role": role, "label": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "group": group, "order": order}
    if widget: d["widget"] = widget
    d.update(kw); return d
def leaf_schema(L):
    num = L["kind"] == "set-case-number"
    vt = ["number", "null"] if num else ["string", "null"]
    val = ui("stepper" if num else None, "value", "Value", "Wert", "The input value in the stored unit of the catalogue vocabulary (empty removes the input).", "Der Eingabewert in der gespeicherten Einheit des Katalogvokabulars (leer entfernt die Eingabe).", 30, "case", **({"step": 1, "precision": 4} if num else {}))
    return jd({"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/{L['kind']}/schema.json", "title": L["T"], "type": "object", "additionalProperties": False, "required": ["caseId", "key", "value"], "properties": {
        "caseId": {"type": "string", "x-semio-ui": ui("reference", "target", "Case", "Fall", "The design case this mutation addresses.", "Der Bemessungsfall, den diese Mutation adressiert.", 10, "target", ref={"kind": "case"})},
        "key": {"type": "string", "x-semio-ui": ui(None, "target", "Input", "Eingabe", "The input key of the case.", "Der Eingabeschlüssel des Falls.", 20, "target")},
        "value": {"type": vt, "x-semio-ui": val}}}) + "\n"
def leaf_ts(L):
    num = L["kind"] == "set-case-number"
    t, r = ("number", "normWireNumber") if num else ("string", "normWireString")
    return f'''/** {L["emoji"]} `{L["kind"]}` wire twin: the leaf payload `{L["T"]}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ normWireNullable, normWireObject, type NormWireReader, normWireRequired, normWireString{", normWireNumber" if num else ""} }} from "@semio-tech/norm-js/registry/contract";

export interface {L["T"]} {{
  caseId: string;
  key: string;
  value: {t} | null;
}}

export const parse{L["T"]}: NormWireReader<{L["T"]}> = normWireObject<{L["T"]}>({{ caseId: normWireRequired(normWireString), key: normWireRequired(normWireString), value: normWireRequired(normWireNullable({r})) }});
'''
TAGS = {"set-case-number": 72, "set-case-text": 73}
for L in NEW:
    d = MUT + L["dir"] + "/"
    wr(d + "🦀️.rs", leaf_rs(L)); wr(d + "🔺️diff/🦀️.rs", leaf_diff(L)); wr(d + "↩️inverse/🦀️.rs", leaf_inverse(L))
    wr(d + "🔣️.json", leaf_json(L, TAGS[L["kind"]])); wr(d + "🧬️schema/🔣️.json", leaf_schema(L)); wr(d + "🧬️schema/🟦️.ts", leaf_ts(L))

# ---- crates: mut-cases keeps insert, remove, uls; mut-case-inputs gets labels, entry id, set-*
P = MUT + "📇️cases/🦀️.rs"
s = rd(P)
for k in MOVE:
    s = re.sub(r'#\[path = "\.\./[^"]*change-case-%s/🦀️\.rs"\]\npub mod %s;\n' % (k.split("change_case_")[1].replace("_", "-"), k), "", s)
    assert "mod " + k + ";" not in s, k
wr(P, s)
s = rd(MUT + "🔢️case-inputs/🦀️.rs")
a = s.index("//#region 🪢️Mounts"); b = s.index("//#endregion 🪢️Mounts") + len("//#endregion 🪢️Mounts")
mounts = ["🏷️change-case-label-en", "🏷️change-case-label-de", "📇️change-case-entry-id", "🔢️set-case-number", "🔤️set-case-text"]
s = s[:a] + "//#region 🪢️Mounts\n" + "".join(f'#[path = "../{m}/🦀️.rs"]\npub mod {re.sub(r"^[^a-z]+", "", m).replace("-", "_")};\n' for m in sorted(mounts, key=lambda m: re.sub(r"^[^a-z]+", "", m))) + "//#endregion 🪢️Mounts" + s[b:]
s = s.replace("EN 1995 catalogue-case input mutation leaves", "EN 1995 design-case label, entry and input mutation leaves")
wr(MUT + "🔢️case-inputs/🦀️.rs", s)
for crate, extra in [("📇️cases", ["semio-s-artifact-norm-en1995-kernel"]), ("🔢️case-inputs", [])]:
    P = MUT + crate + "/📦️packages/🦀️rust/Cargo.toml"
    s = rd(P)
    add = "".join(f"{x} = {{ workspace = true }}\n" for x in extra)
    sub = "semio-s-artifact-norm-en1995-model = { workspace = true }\n"
    assert sub in s
    s = s.replace(sub, add + sub + "semio-s-plugin-norm-compliance = { workspace = true }\n", 1)
    wr(P, s)
LOCK = ART + "Cargo.lock"
s = rd(LOCK)
for crate, extra in [("mut-cases", ["semio-s-artifact-norm-en1995-kernel"]), ("mut-case-inputs", [])]:
    m = re.search(r'name = "semio-s-artifact-norm-en1995-%s"\nversion = "0.1.0"\ndependencies = \[\n(.*?)\]\n' % crate, s, re.S)
    assert m, crate
    deps = [x.strip().strip(",").strip('"') for x in m.group(1).splitlines()] + extra + ["semio-s-plugin-norm-compliance"]
    new = "".join(f' "{x}",\n' for x in sorted(deps))
    s = s[:m.start(1)] + new + s[m.end(1):]
wr(LOCK, s)

# ---- aggregate: root enum, use lines, KINDS, twins
def kind_order_edit(text, per_old, per_new_anchor=None):
    return text
P = MUT + "🦀️.rs"
s = rd(P)
for sn in OLD_SNAKE:
    s = re.sub(r"use semio_s_artifact_norm_en1995_mut_case_inputs::%s;\n" % sn, "", s)
for k in MOVE:
    s = s.replace("mut_cases::" + k + ";", "mut_case_inputs::" + k + ";")
s = s.replace("use semio_s_artifact_norm_en1995_mut_case_inputs::change_case_entry_id;\n", "use semio_s_artifact_norm_en1995_mut_case_inputs::change_case_entry_id;\n" + "".join(f"use semio_s_artifact_norm_en1995_mut_case_inputs::{L['snake']};\n" for L in NEW), 1)
for T, sn in zip(OLD_TYPES, OLD_SNAKE):
    s, c = re.subn(r"    %s\(%s::%s\),\n" % (T, sn, T), "", s); assert c, T
s = s.replace("    ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId),\n", "    ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId),\n" + "".join(f"    {L['T']}({L['snake']}::{L['T']}),\n" for L in NEW), 1)
for k in OLD:
    s, c = re.subn(r'    "%s",\n' % k, "", s); assert c, k
s = s.replace('    "change-case-entry-id",\n', '    "change-case-entry-id",\n' + "".join(f'    "{L["kind"]}",\n' for L in NEW), 1)
wr(P, s)

P = MUT + "🔣️.json"
s = rd(P)
for T, k in zip(OLD_TYPES, OLD):
    pat = r'    \{\n      "type": "object",\n      "additionalProperties": false,\n      "required": \[\n        "%s"\n      \],\n      "properties": \{\n        "%s": \{\n          "\$ref": "[^"]*"\n        \}\n      \}\n    \},?\n' % (T, T)
    s, c = re.subn(pat, "", s); assert c, T
def one_of(L): return '    {\n      "type": "object",\n      "additionalProperties": false,\n      "required": [\n        "%s"\n      ],\n      "properties": {\n        "%s": {\n          "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/%s/schema.json"\n        }\n      }\n    }' % (L["T"], L["T"], L["kind"])
end = s.rstrip().rfind("\n  ]\n}")
body = s[:end].rstrip().rstrip(",")
if not body.endswith("}"): raise SystemExit("json tail")
s = body + ",\n" + ",\n".join(one_of(L) for L in NEW) + s[end:]
json.loads(s)
wr(P, s)

P = MUT + "🛰️.proto"
s = rd(P)
for T in OLD_TYPES:
    s, c = re.subn(r"message %s \{\n(?:  .*\n)*?\}\n\n" % T, "", s); assert c, T
for T, sn in zip(OLD_TYPES, OLD_SNAKE):
    s, c = re.subn(r"    %s %s = \d+;\n" % (T, sn), "", s); assert c, T
msgs = "message SetCaseNumber {\n  // @state artifact\n  string case_id = 1;\n  // @state artifact\n  string key = 2;\n  // @state artifact\n  optional double value = 3;\n}\n\nmessage SetCaseText {\n  // @state artifact\n  string case_id = 1;\n  // @state artifact\n  string key = 2;\n  // @state artifact\n  optional string value = 3;\n}\n\n"
s = s.replace("message En1995Mutation {", msgs + "message En1995Mutation {", 1)
s = s.replace("    ChangeCaseEntryId change_case_entry_id = 72;\n", "    ChangeCaseEntryId change_case_entry_id = 72;\n    SetCaseNumber set_case_number = 73;\n    SetCaseText set_case_text = 74;\n", 1)
wr(P, s)

P = MUT + "🔗️.graphql"
s = rd(P)
for T in OLD_TYPES:
    s, c = re.subn(r"  \| %s\n" % T, "", s); assert c, T
    s, c = re.subn(r"type %s \{\n(?:  .*\n)*?\}\n\n?" % T, "", s); assert c, T
s = s.replace("  | ChangeCaseEntryId\n", "  | ChangeCaseEntryId\n  | SetCaseNumber\n  | SetCaseText\n", 1)
s = s.rstrip("\n") + "\n\ntype SetCaseNumber {\n  caseId: String! @state(class: ARTIFACT)\n  key: String! @state(class: ARTIFACT)\n  value: Float @state(class: ARTIFACT)\n}\n\ntype SetCaseText {\n  caseId: String! @state(class: ARTIFACT)\n  key: String! @state(class: ARTIFACT)\n  value: String @state(class: ARTIFACT)\n}\n"
wr(P, s)

P = MUT + "🟦️.ts"
s = rd(P)
for T in OLD_TYPES:
    s, c = re.subn(r'import \{ type %s, parse%s \} from "[^"]*";\n' % (T, T), "", s); assert c, T
    s = re.sub(r"  \| \{ %s: %s \}\n" % (T, T), "", s)
    s, c = re.subn(r"  %s: parse%s,\n" % (T, T), "", s); assert c, T
s = s.replace('import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";\n', 'import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";\n' + "".join(f'import {{ type {L["T"]}, parse{L["T"]} }} from "./{L["dir"]}/🧬️schema/🟦️.ts";\n' for L in NEW), 1)
s = s.replace("  ChangeCaseEntryId: parseChangeCaseEntryId,\n", "  ChangeCaseEntryId: parseChangeCaseEntryId,\n" + "".join(f"  {L['T']}: parse{L['T']},\n" for L in NEW), 1)
assert "SetCaseNumber: parseSetCaseNumber" in s
wr(P, s)

# ---- io text, binary
P = ANY + "🚪️io/📝️text/🧬️mutations/🦀️.rs"
s = rd(P)
for sn in OLD_SNAKE: s = re.sub(r"use semio_s_artifact_norm_en1995_mut_case_inputs::%s;\n" % sn, "", s)
for k in MOVE: s = s.replace("mut_cases::" + k + ";", "mut_case_inputs::" + k + ";")
s = s.replace("use semio_s_artifact_norm_en1995_mut_case_inputs::change_case_entry_id;\n", "use semio_s_artifact_norm_en1995_mut_case_inputs::change_case_entry_id;\n" + "".join(f"use semio_s_artifact_norm_en1995_mut_case_inputs::{L['snake']};\n" for L in NEW), 1)
wr(P, s)
P = ANY + "🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
s = rd(P)
for k in OLD: s, c = re.subn(r"record %s tag=\d+\nfield payload bytes\n" % k, "", s); assert c, k
s = s.rstrip("\n") + "\n" + "".join(f"record {L['kind']} tag={TAGS[L['kind']] - 1}\nfield payload bytes\n" for L in NEW)
wr(P, s)

# ---- tests with the mutation lists
NEWLINES = ['        En1995Mutation::SetCaseNumber(semio_s_artifact_norm_en1995_mut_case_inputs::set_case_number::SetCaseNumber { case_id: base.cases[0].id.clone(), key: "f_u".into(), value: Some(400.0) }),\n',
            '        En1995Mutation::SetCaseText(semio_s_artifact_norm_en1995_mut_case_inputs::set_case_text::SetCaseText { case_id: base.cases[0].id.clone(), key: "class".into(), value: Some("C24".into()) }),\n']
for P in [MUT + "🧪️tests/🔬️unit/🦀️.rs", ANY + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", ANY + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️op-round-trip/🦀️.rs"]:
    s = rd(P)
    s, c = re.subn(r"        En1995Mutation::(?:%s)\(.*\),\n" % "|".join(OLD_TYPES), "", s); assert c == 6, (P, c)
    for k in MOVE: s = s.replace("mut_cases::" + k + "::", "mut_case_inputs::" + k + "::")
    m = re.search(r"        En1995Mutation::ChangeCaseEntryId\(.*\),\n", s); assert m, P
    s = s[:m.end()] + "".join(NEWLINES) + s[m.end():]
    s = s.replace("model::CatalogueCase", "model::CaseRow")
    wr(P, s)

# ---- edit rules
P = MUT + "🧭️edit-rules/🦀️.rs"
s = rd(P)
for l in [l for l in s.splitlines(True) if re.search(r'"(insert|remove)-case-(number|text)"|"change-case-(number|text)-value"', l)]: s = s.replace(l, "")
s = s.replace('        SetFieldRule { path: "cases[].entryId", kind: "change-case-entry-id", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },\n', '        SetFieldRule { path: "cases[].entryId", kind: "change-case-entry-id", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },\n        SetFieldRule { path: "cases[].numbers[].value", kind: "set-case-number", selectors: &[SelectorKey::Id("caseId"), SelectorKey::Id("key")], value: "value" },\n        SetFieldRule { path: "cases[].texts[].value", kind: "set-case-text", selectors: &[SelectorKey::Id("caseId"), SelectorKey::Id("key")], value: "value" },\n', 1)
assert "set-case-number" in s and "insert-case-number" not in s
wr(P, s)

# ---- harness / feature / python / rust list
MT = ANY + "🧪️tests/🪵️mutate-en1995-1/"
P = MT + "🥒️.feature"
s = rd(P)
for k in OLD: s, c = re.subn(r"      \| %s \| [^|]*\| ✅apply  \|\n" % k, "", s); assert c == 2, (k, c)
s, c = re.subn(r"(      \| change-case-entry-id \| 📇️change-case-entry-id \| ✅apply  \|\n)", lambda m: m.group(1) + "".join(f"      | {L['kind']} | {L['dir']} | ✅apply  |\n" for L in NEW), s); assert c == 2
wr(P, s)
P = MT + "🐍️.py"
s = rd(P)
for k in OLD:
    s, c = re.subn(r'    "%s",\n' % k, "", s); assert c, k
    s, c = re.subn(r'    "%s": \("[^"]*", "✅apply"\),\n' % k, "", s); assert c, k
s = s.replace('    "change-case-entry-id",\n', '    "change-case-entry-id",\n' + "".join(f'    "{L["kind"]}",\n' for L in NEW), 1)
s = re.sub(r'(    "change-case-entry-id": \("[^"]*", "✅apply"\),\n)', lambda m: m.group(1) + "".join(f'    "{L["kind"]}": ("{L["dir"]}", "✅apply"),\n' for L in NEW), s, count=1)
wr(P, s)
P = MT + "🦀️.rs"
s = rd(P)
for k in OLD:
    s, c = re.subn(r'    "%s",\n' % k, "", s); assert c, k
    s, c = re.subn(r'    \("%s", \(\n(?:        include_str!.*\n){4}    \)\),\n' % k, "", s); assert c, k
s = s.replace('    "change-case-entry-id",\n', '    "change-case-entry-id",\n' + "".join(f'    "{L["kind"]}",\n' for L in NEW), 1)
ent = lambda L: f'    ("{L["kind"]}", (\n' + "".join(f'        include_str!("../🧫️fixtures/🧬️mutations/{L["dir"]}/✅apply/🔣️.{x}.json"),\n' for x in ["before", "mutation", "after", "outcome"]) + "    )),\n"
m = re.search(r'    \("change-case-entry-id", \(\n(?:        include_str!.*\n){4}    \)\),\n', s); assert m
s = s[:m.end()] + "".join(ent(L) for L in NEW) + s[m.end():]
wr(P, s)
P = ANY + "🧪️tests/📇️catalogue-en1995-1/🥒️.feature"
sub1(P, "🔢️change-case-number-value", "🔢️set-case-number")

# ---- oracle manifest, taxonomy, derive schema-document index
P = ANY + "🔮️oracles/🔣️.json"
d = json.loads(rd(P)); cat = d["mutationCatalogs"][0]
cat["vectors"] = [v for v in cat["vectors"] if v["mutationId"] not in OLD]; cat["kinds"] = [k for k in cat["kinds"] if k not in OLD]
man = d["mutationManifests"][0]; man["mutations"] = [m for m in man["mutations"] if m["id"] not in OLD]
tmpl = next(m for m in man["mutations"] if m["id"] == "change-case-entry-id")
ci = cat["kinds"].index("change-case-entry-id") + 1
vi = next(i for i, v in enumerate(cat["vectors"]) if v["mutationId"] == "change-case-entry-id") + 1
mi = next(i for i, m in enumerate(man["mutations"]) if m["id"] == "change-case-entry-id") + 1
import copy
for j, L in enumerate(NEW):
    cat["kinds"].insert(ci + j, L["kind"])
    cat["vectors"].insert(vi + j, {"mutationId": L["kind"], "sourceMutationDirectoryName": L["dir"], "mutationDirectoryName": L["dir"], "scenarios": [{"id": L["kind"], "directoryName": "✅apply"}]})
    m = copy.deepcopy(tmpl); m["id"] = L["kind"]; m["productionDispatch"] = {"operation": L["kind"], "bridgeVersion": 1, "variant": L["T"]}; man["mutations"].insert(mi + j, m)
raw = rd(P); wr(P, jd(d) + ("\n" if raw.endswith("\n") else ""))
P = NORM + "🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
raw = rd(P); d = json.loads(raw)
rows = [r for r in d["rows"] if not (r.get("artifact") == "🪵️en1995" and r.get("kind") in OLD)]
idx = max(i for i, r in enumerate(rows) if r.get("artifact") == "🪵️en1995" and r.get("kind") == "change-case-entry-id")
tr = next(r for r in rows if r.get("artifact") == "🪵️en1995" and r.get("kind") == "change-case-entry-id")
new = []
for L in NEW:
    r = copy.deepcopy(tr); r.update({"aggregateVariant": L["T"], "entity": L["entity"], "kind": L["kind"], "module": L["dir"], "record": L["record"], "type": L["T"], "verb": "set", "source": tr["source"].replace("📇️change-case-entry-id", L["dir"])}); new.append(r)
d["rows"] = rows[:idx + 1] + new + rows[idx + 1:]
wr(P, jd(d) + ("\n" if raw.endswith("\n") else ""))
P = "/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json"
s = rd(P)
for k, dn in OLD_DIRS.items():
    s, c = re.subn(r'      "https://json\.schemas\.assets\.semio-tech\.com/s/norm/en1995/mutation/%s/schema\.json": "[^"]*",?\n' % re.escape(k), "", s); assert c == 1, k
anchor = re.search(r'      "https://json\.schemas\.assets\.semio-tech\.com/s/norm/en1995/mutation/change-case-entry-id/schema\.json": "([^"]*)",?\n', s); assert anchor
add = "".join(f'      "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/{L["kind"]}/schema.json": "{anchor.group(1).replace("📇️change-case-entry-id", L["dir"])}",\n' for L in NEW)
line = anchor.group(0)
if not line.rstrip("\n").endswith(","): add = add.rstrip(",\n") + "\n"; s = s.replace(line, line.rstrip("\n") + ",\n", 1); line = line.rstrip("\n") + ",\n"
s = s.replace(line, line + add, 1)
wr(P, s)

# ---- fixtures for the two new kinds (wire diffs equal the removed change-value fixtures)
for L, src, tkey, nkey, newv in [(NEW[0], "🔢️change-case-number-value", "ChangeCaseNumberValue", "numberId", 16.0), (NEW[1], "🔤️change-case-text-value", "ChangeCaseTextValue", "textId", "nail")]:
    for sub in ["📸️snapshot/⬅️before", "📸️snapshot/➡️after", "🔺️diff", "🎯️outcome"]:
        wr(FIX + L["dir"] + "/✅apply/" + sub + "/🔣️.json", open(FIX + src + "/✅apply/" + sub + "/🔣️.json", encoding="utf-8").read())
    key = "d" if L["kind"] == "set-case-number" else "kind"
    wr(FIX + L["dir"] + "/✅apply/🔣️.mutation.json", jd({L["T"]: {"caseId": "case-kser", "key": key, "value": newv}}))

if "--apply" not in sys.argv:
    print("staged", len(stage), "files; delete", len(delete)); sys.exit(0)
for p, s in stage.items():
    os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf-8").write(s)
for p in delete:
    shutil.rmtree(p) if os.path.isdir(p) else os.remove(p)
print("applied", len(stage), "files; deleted", len(delete))
