#!/usr/bin/env python3
"""EN 1995 wave 2: generates the 12 mutation leaves of the catalogue-case entity and the ULS equation (crate mut-cases), registers them in every aggregate, fixture catalogue, manifest and test list, removes change-member-m-crit from the registries, and hand-fixes every existing fixture snapshot/diff to the new document shape."""
import json, os, re, glob, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🪵️en1995/"
ANY = ART + "🏅️standards/🔖️1/🪆️subsets/✳️any/"
MUT = ANY + "🧬️schema/🧬️mutations/"
FIX = ANY + "🧪️tests/🧫️fixtures/🧬️mutations/"
OWNER = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
def rd(p): return open(p, encoding="utf-8").read()
def wr(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf-8").write(s)
def snake(k): return k.replace("-", "_")
def pascal(k): return "".join(x[:1].upper() + x[1:] for x in k.split("-"))
def camel(s): p = s.split("_"); return p[0] + "".join(x[:1].upper() + x[1:] for x in p[1:])
GROUP = "📇️cases"; CRATE = "semio_s_artifact_norm_en1995_mut_cases"
SNAPREF = "https://json.schemas.assets.semio-tech.com/s/norm/en1995/snapshot.json#/$defs/"
CASE = {"id": "case-kser", "labelEn": "Slip modulus of the support dowels", "labelDe": "Verschiebungsmodul der Auflagerdübel", "entryId": "T-83", "numbers": [{"id": "d", "value": 12.0}, {"id": "rho_m1", "value": 420.0}], "texts": [{"id": "kind", "value": "dowel"}]}
NEWCASE = {"id": "case-t45", "labelEn": "Notched support", "labelDe": "Ausgeklinktes Auflager", "entryId": "T-45", "numbers": [{"id": "v_d", "value": 20000.0}, {"id": "b", "value": 120.0}, {"id": "h", "value": 240.0}, {"id": "h_ef", "value": 180.0}, {"id": "x", "value": 30.0}, {"id": "sc", "value": 1.0}], "texts": [{"id": "class", "value": "C24"}, {"id": "duration", "value": "medium"}]}

TARGET_CASE = ("caseId", "String", "case", "Case", "Fall", "The catalogue case this mutation addresses.", "Der Katalogfall, den diese Mutation adressiert.")
def target_prop(ref, en, de, den, dde, order):
    return {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "ref": {"kind": ref}, "group": "target", "order": order}}
def index_prop(en, de, den, dde, order=20):
    return {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "step": 1, "precision": 0, "group": "target", "order": order}}
def ref_prop(defn, en, de, den, dde, order):
    return {"$ref": SNAPREF + defn, "x-semio-ui": {"role": "value", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "group": "record", "order": order}}
def str_prop(en, de, den, dde, group, order=30, widget="text", **kw):
    return {"type": "string", "x-semio-ui": {"widget": widget, "role": "value", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "group": group, "order": order, **kw}}
def num_prop(en, de, den, dde, group, order=30):
    return {"type": "number", "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "step": 1, "precision": 4, "group": group, "order": order}}

L = []
def leaf(**kw): L.append(kw)

leaf(emoji="🧮️", kind="change-uls-equation", doc="changes `ulsEquation` (ULS combination equation: 6.10 or 6.10a/b with xi = 0.85) of the document.", fields=[("new_value", "String", "string")],
  props={"newValue": str_prop("ULS combination equation", "Grundkombination (GZT)", "6.10, or 6.10ab for the governing one of EN 1990 equations 6.10a and 6.10b (xi = 0.85).", "6.10 oder 6.10ab für die maßgebende der Gleichungen 6.10a und 6.10b nach EN 1990 (xi = 0,85).", "combination", 10, widget="select", options=["6.10", "6.10ab"])},
  partner="self", verb="change", entity="uls-equation", record="ChangedUlsEquation", imports="", delta_imports="",
  label=('"Change ULS combination equation to {}", self.new_value', '"GZT-Kombinationsgleichung auf {} ändern", self.new_value'), target="vec![self.new_value.clone()]",
  diff='''    if !["6.10", "6.10ab"].contains(&payload.new_value.as_str()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown ULS equation {}.", payload.new_value), vec![payload.new_value.clone()]);
    }
    if base.uls_equation == payload.new_value {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "ULS equation already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { uls_equation: Some(payload.new_value.clone()), ..Default::default() })''',
  inv='''    vec![Op::from(ChangeUlsEquation { new_value: base.uls_equation.clone() })]''', diffuse="")

leaf(emoji="➕️", kind="insert-case", doc="inserts one catalogue case into the case table.", fields=[("index", "usize", "int"), ("case", "CatalogueCase", "ref:CatalogueCase")],
  props={"index": index_prop("Position", "Position", "Zero-based list position at which the record is inserted.", "Nullbasierte Listenposition, an der der Datensatz eingefügt wird.", 10), "case": ref_prop("CatalogueCase", "Case", "Fall", "The complete catalogue case to insert.", "Der vollständige einzufügende Katalogfall.", 20)},
  partner="remove-case", verb="insert", entity="case", record="InsertedCase", imports="CatalogueCase, ", delta_imports="En1995CaseDelta",
  label=('"Insert catalogue case {} at #{}", self.case.id, self.index', '"Katalogfall {} an #{} einfügen", self.case.id, self.index'), target="vec![self.case.id.clone()]",
  diff='''    if base.cases.iter().any(|existing| existing.id == payload.case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Case id {} already exists.", payload.case.id), [payload.case.id.clone()]);
    }
    let at = payload.index.min(base.cases.len());
    protocol::MutationOutcome::new(En1995Diff { cases: En1995CaseDelta::insertion(at, payload.case.clone()), ..Default::default() })''',
  inv='''    let at = payload.index.min(base.cases.len());
    vec![Op::from(remove_case::RemoveCase { index: at })]''', diffuse="use crate::remove_case;")

leaf(emoji="➖️", kind="remove-case", doc="removes the catalogue case at an index.", fields=[("index", "usize", "int")],
  props={"index": index_prop("Case", "Fall", "Zero-based position of the catalogue case in its list.", "Nullbasierte Position des Katalogfalls in der Liste.", 10)},
  partner="insert-case", verb="remove", entity="case", record="RemovedCase", imports="", delta_imports="En1995CaseDelta",
  label=('"Remove catalogue case #{}", self.index', '"Katalogfall #{} entfernen", self.index'), target="vec![self.index.to_string()]",
  diff='''    if payload.index >= base.cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Case index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { cases: En1995CaseDelta::removal(&base.cases, payload.index), ..Default::default() })''',
  inv='''    if payload.index >= base.cases.len() { return Vec::new(); }
    vec![Op::from(insert_case::InsertCase { index: payload.index, case: base.cases[payload.index].clone() })]''', diffuse="use crate::insert_case;")

def case_str(kind, emoji, field, patch, en, de, den, dde, labsrc):
    leaf(emoji=emoji, kind=kind, doc=f"changes `{camel(field)}` ({en}) on one addressed catalogue case.", fields=[("case_id", "String", "string"), ("new_value", "String", "string")],
      props={"caseId": target_prop("case", "Case", "Fall", "The catalogue case this mutation addresses.", "Der Katalogfall, den diese Mutation adressiert.", 10), "newValue": str_prop(en, de, den, dde, "case", 20)},
      partner="self", verb="change", entity=f"case-{camel(field)}", record="Changed" + pascal(kind[7:]), imports="", delta_imports="En1995CaseDelta, En1995CasePatch",
      label=(f'"Change {en} of case {{}}", self.case_id', f'"{de} von Fall {{}} ändern", self.case_id'), target="vec![self.case_id.clone()]",
      diff=f'''    let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
    }};
    protocol::MutationOutcome::new(En1995Diff {{ cases: En1995CaseDelta::modification(&case.id, En1995CasePatch {{ {field}: Some(payload.new_value.clone()), ..Default::default() }}), ..Default::default() }})''',
      inv=f'''    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else {{ return Vec::new(); }};
    vec![Op::from({pascal(kind)} {{ case_id: payload.case_id.clone(), new_value: item.{field}.clone() }})]''', diffuse="")
case_str("change-case-label-en", "🏷️", "label_en", "label_en", "Label (EN)", "Bezeichnung (EN)", "English display name of the case.", "Englische Anzeigebezeichnung des Falls.", None)
case_str("change-case-label-de", "🏷️", "label_de", "label_de", "Label (DE)", "Bezeichnung (DE)", "German display name of the case.", "Deutsche Anzeigebezeichnung des Falls.", None)
case_str("change-case-entry-id", "📇️", "entry_id", "entry_id", "Catalogue entry", "Katalogeintrag", "Verification id T-nn of the EN 1995 catalogue.", "Nachweis-Id T-nn des EN-1995-Katalogs.", None)

def row_leaves(sing, sing_de, plural, tbl, tyname, valty, nest_delta, nest_patch, insert_emoji="➕️", remove_emoji="➖️", change_emoji="🔢️"):
    T = tyname
    leaf(emoji=insert_emoji, kind=f"insert-case-{sing}", doc=f"inserts one {sing} input into a catalogue case.", fields=[("case_id", "String", "string"), ("index", "usize", "int"), (sing, T, f"ref:{T}")],
      props={"caseId": target_prop("case", "Case", "Fall", "The catalogue case this mutation addresses.", "Der Katalogfall, den diese Mutation adressiert.", 10), "index": index_prop("Position", "Position", "Zero-based list position at which the record is inserted.", "Nullbasierte Listenposition, an der der Datensatz eingefügt wird."), sing: ref_prop(T, f"{sing_de[0]}", f"{sing_de[1]}", f"The complete {sing} input to insert.", f"Die vollständige einzufügende Eingabe ({sing_de[1]}).", 30)},
      partner=f"remove-case-{sing}", verb="insert", entity=f"case-{sing}", record=f"InsertedCase{pascal(sing)}", imports=f"{T}, ", delta_imports=f"En1995CaseDelta, {nest_delta}, En1995CasePatch",
      label=(f'"Insert {sing} input {{}} into case {{}} at #{{}}", self.{sing}.id, self.case_id, self.index', f'"Eingabe {{}} ({sing_de[1]}) in Fall {{}} an #{{}} einfügen", self.{sing}.id, self.case_id, self.index'), target=f"vec![self.case_id.clone(), self.{sing}.id.clone()]",
      diff=f'''    let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
    }};
    if case.{tbl}.iter().any(|existing| existing.id == payload.{sing}.id) {{
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {{}} already exists.", payload.{sing}.id), [payload.{sing}.id.clone()]);
    }}
    let at = payload.index.min(case.{tbl}.len());
    protocol::MutationOutcome::new(En1995Diff {{ cases: En1995CaseDelta::modification(&case.id, En1995CasePatch {{ {tbl}: {nest_delta}::insertion(at, payload.{sing}.clone()), ..Default::default() }}), ..Default::default() }})''',
      inv=f'''    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else {{ return Vec::new(); }};
    let at = payload.index.min(item.{tbl}.len());
    vec![Op::from(remove_case_{sing}::RemoveCase{pascal(sing)} {{ case_id: payload.case_id.clone(), index: at }})]''', diffuse=f"use crate::remove_case_{sing};")
    leaf(emoji=remove_emoji, kind=f"remove-case-{sing}", doc=f"removes the {sing} input at an index from a catalogue case.", fields=[("case_id", "String", "string"), ("index", "usize", "int")],
      props={"caseId": target_prop("case", "Case", "Fall", "The catalogue case this mutation addresses.", "Der Katalogfall, den diese Mutation adressiert.", 10), "index": index_prop("Input", "Eingabe", f"Zero-based position of the {sing} input within the case.", f"Nullbasierte Position der Eingabe ({sing_de[1]}) im Fall.")},
      partner=f"insert-case-{sing}", verb="remove", entity=f"case-{sing}", record=f"RemovedCase{pascal(sing)}", imports="", delta_imports=f"En1995CaseDelta, {nest_delta}, En1995CasePatch",
      label=(f'"Remove {sing} input #{{}} from case {{}}", self.index, self.case_id', f'"Eingabe ({sing_de[1]}) #{{}} aus Fall {{}} entfernen", self.index, self.case_id'), target="vec![self.case_id.clone(), self.index.to_string()]",
      diff=f'''    let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
    }};
    if payload.index >= case.{tbl}.len() {{
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Case {sing} index {{}} out of range.", payload.index), vec![payload.case_id.clone()]);
    }}
    protocol::MutationOutcome::new(En1995Diff {{ cases: En1995CaseDelta::modification(&case.id, En1995CasePatch {{ {tbl}: {nest_delta}::removal(&case.{tbl}, payload.index), ..Default::default() }}), ..Default::default() }})''',
      inv=f'''    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id) else {{ return Vec::new(); }};
    if payload.index >= item.{tbl}.len() {{ return Vec::new(); }}
    vec![Op::from(insert_case_{sing}::InsertCase{pascal(sing)} {{ case_id: payload.case_id.clone(), index: payload.index, {sing}: item.{tbl}[payload.index].clone() }})]''', diffuse=f"use crate::insert_case_{sing};")
    rs_t, ts_t = ("f64", "number") if sing == "number" else ("String", "string")
    leaf(emoji=change_emoji, kind=f"change-case-{sing}-value", doc=f"changes `value` of one {sing} input of a catalogue case.", fields=[("case_id", "String", "string"), (f"{sing}_id", "String", "string"), ("new_value", rs_t, ts_t)],
      props={"caseId": target_prop("case", "Case", "Fall", "The catalogue case this mutation addresses.", "Der Katalogfall, den diese Mutation adressiert.", 10), f"{sing}Id": target_prop(f"case-{sing}", "Input", "Eingabe", "The input key of the case.", "Der Eingabeschlüssel des Falls.", 20), "newValue": (num_prop("Value", "Wert", "The new input value in the units of the catalogue entry (N, mm, N/mm², degrees).", "Der neue Eingabewert in den Einheiten des Katalogeintrags (N, mm, N/mm², Grad).", "case") if sing == "number" else str_prop("Value", "Wert", "The new text input (strength class, duration class, material, shape …).", "Die neue Texteingabe (Festigkeitsklasse, Dauerklasse, Material, Form …).", "case"))},
      partner="self", verb="change", entity=f"case-{sing}-value", record=f"ChangedCase{pascal(sing)}Value", imports="", delta_imports=f"En1995CaseDelta, {nest_delta}, {nest_patch}, En1995CasePatch",
      label=(f'"Change {sing} input {{}} of case {{}}", self.{sing}_id, self.case_id', f'"Eingabe {{}} ({sing_de[1]}) von Fall {{}} ändern", self.{sing}_id, self.case_id'), target=f"vec![self.case_id.clone(), self.{sing}_id.clone()]",
      diff=f'''    let Some(case) = base.cases.iter().find(|item| item.id == payload.case_id) else {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case id.", vec![payload.case_id.clone()]);
    }};
    let Some(row) = case.{tbl}.iter().find(|item| item.id == payload.{sing}_id) else {{
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown case {sing} id.", vec![payload.case_id.clone(), payload.{sing}_id.clone()]);
    }};
    protocol::MutationOutcome::new(En1995Diff {{ cases: En1995CaseDelta::modification(&case.id, En1995CasePatch {{ {tbl}: {nest_delta}::modification(&row.id, {nest_patch} {{ value: Some(payload.new_value{'' if sing == 'number' else '.clone()'}), ..Default::default() }}), ..Default::default() }}), ..Default::default() }})''',
      inv=f'''    let Some(item) = base.cases.iter().find(|item| item.id == payload.case_id).and_then(|item| item.{tbl}.iter().find(|row| row.id == payload.{sing}_id)) else {{ return Vec::new(); }};
    vec![Op::from(ChangeCase{pascal(sing)}Value {{ case_id: payload.case_id.clone(), {sing}_id: payload.{sing}_id.clone(), new_value: item.value{'' if sing == 'number' else '.clone()'} }})]''', diffuse="")
row_leaves("number", ("Number", "Zahl"), "numbers", "numbers", "CaseNumber", "f64", "En1995CaseNumberDelta", "En1995CaseNumberPatch")
row_leaves("text", ("Text", "Text"), "texts", "texts", "CaseText", "String", "En1995CaseTextDelta", "En1995CaseTextPatch", change_emoji="🔤️")

TAG0 = 66
for i, l in enumerate(L):
    l["tag"] = TAG0 + i; l["T"] = pascal(l["kind"]); l["mod"] = snake(l["kind"]); l["dir"] = l["emoji"] + l["kind"]

def rust_leaf(l):
    T, mod = l["T"], l["mod"]
    flds = ", ".join(f"pub {n}: {t}" for n, t, _ in l["fields"])
    partner = l["mod"] if l["partner"] == "self" else snake(l["partner"])
    pT = T if l["partner"] == "self" else pascal(l["partner"])
    imp = f"use semio_s_artifact_norm_en1995_model::{{{l['imports']}En1995Snapshot}};"
    le, ld = l["label"]
    return f'''//! {l["emoji"]} `{l["kind"]}` — {l["doc"]}
{imp}
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct {T} {{ {flds} }}
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for {T} where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::{partner}::{pT}> {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "{l["verb"]}", entity: "{l["entity"]}", kind: "{l["kind"]}", record: "{l["record"]}" }};

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {{ semio_framework_ui_locale::LocalizedLabel::native(&format!({le}), &format!({ld})) }}
    fn target(&self) -> Vec<String> {{ {l["target"]} }}
}}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for {T} where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::{partner}::{pT}> {{
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> {{ crate::{mod}::diff::diff(self, base) }}
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {{
    Ok({{ crate::{mod}::inverse::inverse(self, base)? 
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
def rust_diff(l):
    di = l["delta_imports"]
    di_line = f"use semio_s_artifact_norm_en1995_diff::{{{di}}};\n" if di else ""
    return f'''use crate::{l["mod"]}::{l["T"]};
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
{di_line}pub fn diff(payload: &{l["T"]}, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {{
{l["diff"]}
}}
'''
def rust_inv(l):
    partner = l["mod"] if l["partner"] == "self" else snake(l["partner"])
    pT = l["T"] if l["partner"] == "self" else pascal(l["partner"])
    extra = l["diffuse"] + "\n" if l["diffuse"] else ""
    base_unused = "_payload" if "payload" not in l["inv"] else "payload"
    return f'''use crate::{l["mod"]}::{l["T"]};
{extra}use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>({base_unused}: &{l["T"]}, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{partner}::{pT}> {{
    Ok((|| {{
{l["inv"]}

    }})())
}}
'''
def manifest(l):
    return json.dumps({"schemaVersion": 1, "owner": OWNER + l["dir"], "semanticKind": l["kind"], "displayName": " ".join(w.capitalize() for w in l["kind"].split("-")), "emoji": l["emoji"], "aggregateVariant": l["T"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": l["tag"], "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}, indent=2, ensure_ascii=False) + "\n"
def payload_schema(l):
    return json.dumps({"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/{l['kind']}/schema.json", "title": l["T"], "type": "object", "additionalProperties": False, "required": list(l["props"]), "properties": l["props"]}, indent=2, ensure_ascii=False) + "\n"
def ts_payload(l):
    names = {"string": "normWireString", "number": "normWireNumber", "int": "normWireRange(normWireInteger, {\"minimum\":0})"}
    imps = {"normWireObject", "normWireRequired"}; refs = []; ifl = []; prs = []
    for n, rt, tt in l["fields"]:
        c = camel(n)
        if tt.startswith("ref:"):
            r = tt[4:]; refs.append(r); ifl.append(f"  {c}: {r};"); prs.append(f"{c}: normWireRequired(parse{r})")
        else:
            ifl.append(f"  {c}: {'number' if tt in ('number','int') else 'string'};"); prs.append(f"{c}: normWireRequired({names[tt]})")
            imps |= {"string": {"normWireString"}, "number": {"normWireNumber"}, "int": {"normWireInteger", "normWireRange"}}[tt]
    il = ", ".join(sorted(list(imps) + ["type NormWireReader"], key=lambda s: s.replace("type ", "").lower()))
    ref_imp = f'import {{ {", ".join(f"type {r}, parse{r}" for r in refs)} }} from "../../../📸️snapshot/🟦️.ts";\n' if refs else ""
    T = l["T"]
    return f'''/** {l["emoji"]} `{l["kind"]}` wire twin: the leaf payload `{T}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ {il} }} from "@semio-tech/norm-js/registry/contract";
{ref_imp}
export interface {T} {{
{chr(10).join(ifl)}
}}

export const parse{T}: NormWireReader<{T}> = normWireObject<{T}>({{ {", ".join(prs)} }});
'''

if os.path.exists(MUT + L[0]["dir"]): sys.exit("leaves already generated")
for l in L:
    d = MUT + l["dir"] + "/"
    wr(d + "🦀️.rs", rust_leaf(l)); wr(d + "🔺️diff/🦀️.rs", rust_diff(l)); wr(d + "↩️inverse/🦀️.rs", rust_inv(l))
    wr(d + "🔣️.json", manifest(l)); wr(d + "🧬️schema/🔣️.json", payload_schema(l)); wr(d + "🧬️schema/🟦️.ts", ts_payload(l))

gd = MUT + GROUP + "/"
mounts = "".join(f'#[path = "../{l["dir"]}/🦀️.rs"]\npub mod {l["mod"]};\n' for l in sorted(L, key=lambda x: x["kind"]))
wr(gd + "🦀️.rs", f'''//! 📇️ EN 1995 catalogue-case and combination mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
{mounts}//#endregion 🪢️Mounts
''')
cargo = rd(MUT + "🔩️connections/📦️packages/🦀️rust/Cargo.toml").replace("mut-connections", "mut-cases").replace("EN 1995 connections mutation leaves", "EN 1995 catalogue-case mutation leaves")
wr(gd + "📦️packages/🦀️rust/Cargo.toml", cargo)

def edit(p, f):
    s = rd(p); n = f(s); assert n != s, p; wr(p, n)
# aggregates
uses = "".join(f"use {CRATE}::{l['mod']};\n" for l in L)
variants = "".join(f"    {l['T']}({l['mod']}::{l['T']}),\n" for l in L)
kinds = "".join(f'    "{l["kind"]}",\n' for l in L)
def f(s):
    s = s.replace("use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_fk;\n", "use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_fk;\n" + uses, 1)
    s = s.replace("    ChangeConnectionActionFK(change_connection_action_fk::ChangeConnectionActionFK),\n", "    ChangeConnectionActionFK(change_connection_action_fk::ChangeConnectionActionFK),\n" + variants, 1)
    return s.replace('    "change-connection-action-fk",\n];', '    "change-connection-action-fk",\n' + kinds + "];", 1)
edit(MUT + "🦀️.rs", f)
def f(s):
    return s.replace("use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_fk;\n", "use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_fk;\n" + uses, 1)
edit(ANY + "🚪️io/📝️text/🧬️mutations/🦀️.rs", f)
# json schema aggregate
p = MUT + "🔣️.json"; d = json.loads(rd(p))
for l in L: d["oneOf"].append({"type": "object", "additionalProperties": False, "required": [l["T"]], "properties": {l["T"]: {"$ref": f"https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/{l['kind']}/schema.json"}}})
wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
# ts aggregate
def f(s):
    imp = "".join(f'import {{ type {l["T"]}, parse{l["T"]} }} from "./{l["dir"]}/🧬️schema/🟦️.ts";\n' for l in L)
    s = s.replace("import { type ChangeConnectionDiameter,", imp + "import { type ChangeConnectionDiameter,", 1)
    s = s.replace("  | { ChangeConnectionActionFK: ChangeConnectionActionFK }\n", "  | { ChangeConnectionActionFK: ChangeConnectionActionFK }\n" + "".join(f"  | {{ {l['T']}: {l['T']} }}\n" for l in L), 1)
    return s.replace("  ChangeConnectionActionFK: parseChangeConnectionActionFK,\n", "  ChangeConnectionActionFK: parseChangeConnectionActionFK,\n" + "".join(f"  {l['T']}: parse{l['T']},\n" for l in L), 1)
edit(MUT + "🟦️.ts", f)
GT = {"String": "String!", "f64": "Float!", "usize": "Int!"}
def gq(l):
    rows = "".join(f"  {camel(n)}: {'JSON!' if t in ('CatalogueCase','CaseNumber','CaseText') else GT[t]} @state(class: ARTIFACT)\n" for n, t, _ in l["fields"])
    return f"type {l['T']} {{\n{rows}}}\n"
def f(s):
    s = s.replace("  | ChangeConnectionActionFK\n", "  | ChangeConnectionActionFK\n" + "".join(f"  | {l['T']}\n" for l in L), 1)
    return s.rstrip("\n") + "\n\n" + "\n".join(gq(l) for l in L)
edit(MUT + "🔗️.graphql", f)
PT = {"String": "string", "f64": "double", "usize": "uint32"}
def pm(l):
    rows = ""; i = 1
    for n, t, _ in l["fields"]:
        rows += f"  // @state artifact\n  {'bytes' if t in ('CatalogueCase','CaseNumber','CaseText') else PT[t]} {n} = {i};\n"; i += 1
    return f"message {l['T']} {{\n{rows}}}\n"
def f(s):
    msgs = "\n".join(pm(l) for l in L)
    idx = s.index("message En1995Mutation")
    s = s[:idx] + msgs + "\n" + s[idx:]
    oneof = "".join(f"    {l['T']} {l['mod']} = {l['tag'] + 1};\n" for l in L)
    return s.replace("    ChangeConnectionActionFK change_connection_action_fk = 66;\n", "    ChangeConnectionActionFK change_connection_action_fk = 66;\n" + oneof, 1)
edit(MUT + "🛰️.proto", f)
edit(ANY + "🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", lambda s: s.rstrip("\n") + "\n" + "".join(f"record {l['kind']} tag={l['tag']}\nfield payload bytes\n" for l in L))
