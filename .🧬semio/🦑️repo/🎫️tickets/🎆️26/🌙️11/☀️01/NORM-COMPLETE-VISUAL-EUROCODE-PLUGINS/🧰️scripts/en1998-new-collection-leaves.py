#!/usr/bin/env python3
"""EN 1998: generates the 37 mutation leaves (insert, remove and one change per field) of the collections isolationSystems, pierBases and criticalRegions through every registry except the oracle manifest (idempotent): leaf directories, group mounts (tie-beams group crate and its text crate), enum, KINDS, TS union and parsers, JSON oneOf, graphql, proto, binary protocol, text arms, EDIT_RULES and the apply vectors with the feature rows. Usage: python3 -I en1998-new-collection-leaves.py"""
import json, os, re, copy

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
TXT = ANY + "/🚪️io/📝️text/🧬️mutations"
BIN = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
FEATURE = ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature"
INS, REM, NUM, STR = "➕️", "➖️", "📍️", "✏️"
GROUP, GROUP_TEXT = "mut_tie_beams", "text_tie_beams"
ALIAS = {"stiffness_n_per_m": "stiffness", "yield_displacement_m": "yield-disp-m", "concrete_design_strength_mpa": "concrete-fcd", "steel_design_yield_mpa": "steel-fyd", "hoop_design_yield_mpa": "hoop-fywd", "longitudinal_steel_area_m2": "long-steel-area", "longitudinal_bar_diameter_m": "long-bar-dia", "longitudinal_bar_max_diameter_m": "long-bar-max", "restrained_bar_count": "restr-bar-count", "restrained_bar_spacing_m": "restr-bar-space"}

COLLS = [
    dict(sing="isolation_system", pl="isolation_systems", pas="IsolationSystem", camel="isolationSystem", camel_pl="isolationSystems", en="isolation system", de="Isolationssystem", row="En1998IsolationSystem",
         sample=dict(id="iso-1", bridgeId="br-1", buildingId="", strengthN=300000.0, stiffnessNPerM=15000000.0, yieldDisplacementM=0.01, capacityM=0.5),
         fields=[("bridge_id", "String", "Isolated bridge", "Isolierte Brücke"), ("building_id", "String", "Isolated building", "Isoliertes Gebäude"), ("strength_n", "f64", "Characteristic strength", "Charakteristische Festigkeit"), ("stiffness_n_per_m", "f64", "Post-yield stiffness", "Steifigkeit nach dem Fließen"), ("yield_displacement_m", "f64", "Yield displacement", "Fließverschiebung"), ("capacity_m", "f64", "Displacement capacity", "Verschiebungskapazität")]),
    dict(sing="pier_base", pl="pier_bases", pas="PierBase", camel="pierBase", camel_pl="pierBases", en="pier base", de="Pfeilerfundament", row="En1998PierBase",
         sample=dict(id="pb-1", bridgeId="br-1", pierId="pier-1", radiusM=2.5, soilVsMS=250.0, soilUnitWeightNM3=19000.0, poissonRatio=0.35),
         fields=[("bridge_id", "String", "Owning bridge", "Zugehörige Brücke"), ("pier_id", "String", "Pier on the base", "Pfeiler auf dem Fundament"), ("radius_m", "f64", "Equivalent footing radius", "Äquivalenter Fundamentradius"), ("soil_vs_m_s", "f64", "Soil shear wave velocity", "Scherwellengeschwindigkeit des Bodens"), ("soil_unit_weight_n_m3", "f64", "Soil unit weight", "Wichte des Bodens"), ("poisson_ratio", "f64", "Soil Poisson ratio", "Querdehnzahl des Bodens")]),
    dict(sing="critical_region", pl="critical_regions", pas="CriticalRegion", camel="criticalRegion", camel_pl="criticalRegions", en="critical region", de="Kritischer Bereich", row="En1998CriticalRegion",
         sample=dict(id="cr-1", buildingId="bldg-office", memberId="col-c1", sectionWidthM=0.4, sectionDepthM=0.5, clearLengthM=3.0, hoopedLengthM=0.75, coreWidthM=0.32, coreDepthM=0.42, axialForceN=800000.0, concreteDesignStrengthMpa=20.0, steelDesignYieldMpa=434.8, hoopDesignYieldMpa=434.8, longitudinalSteelAreaM2=0.0048, longitudinalBarDiameterM=0.016, longitudinalBarMaxDiameterM=0.02, hoopDiameterM=0.01, hoopSpacingM=0.08, hoopLegsX=3.0, hoopLegsY=3.0, restrainedBarCount=8.0, restrainedBarSpacingM=0.11),
         fields=[("building_id", "String", "Owning building", "Zugehöriges Gebäude"), ("member_id", "String", "Beam or column", "Balken oder Stütze"), ("section_width_m", "f64", "Section width", "Querschnittsbreite"), ("section_depth_m", "f64", "Section depth", "Querschnittshöhe"), ("clear_length_m", "f64", "Clear length", "Lichte Länge"), ("hooped_length_m", "f64", "Provided hooped length", "Vorhandene Bügellänge"), ("core_width_m", "f64", "Core width", "Kernbreite"), ("core_depth_m", "f64", "Core depth", "Kernhöhe"), ("axial_force_n", "f64", "Axial force", "Normalkraft"), ("concrete_design_strength_mpa", "f64", "Design concrete strength", "Bemessungsfestigkeit des Betons"), ("steel_design_yield_mpa", "f64", "Design yield strength of the longitudinal steel", "Bemessungs-Streckgrenze der Längsbewehrung"), ("hoop_design_yield_mpa", "f64", "Design yield strength of the hoops", "Bemessungs-Streckgrenze der Bügel"), ("longitudinal_steel_area_m2", "f64", "Longitudinal steel area", "Längsbewehrungsquerschnitt"), ("longitudinal_bar_diameter_m", "f64", "Longitudinal bar diameter", "Längsstabdurchmesser"), ("longitudinal_bar_max_diameter_m", "f64", "Largest longitudinal bar diameter", "Größter Längsstabdurchmesser"), ("hoop_diameter_m", "f64", "Hoop diameter", "Bügeldurchmesser"), ("hoop_spacing_m", "f64", "Hoop spacing", "Bügelabstand"), ("hoop_legs_x", "f64", "Hoop legs in width direction", "Bügelschenkel in Breitenrichtung"), ("hoop_legs_y", "f64", "Hoop legs in depth direction", "Bügelschenkel in Höhenrichtung"), ("restrained_bar_count", "f64", "Restrained bar count", "Anzahl gehaltener Stäbe"), ("restrained_bar_spacing_m", "f64", "Restrained bar spacing", "Abstand gehaltener Stäbe")]),
]


def rd(p): return open(p, encoding="utf-8").read()
def wr(p, s):
    assert len(p[len("/home/deck/semio/"):].encode()) < 240, p
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)
def kebab(s): return s.replace("_", "-")
def pas(s): return "".join(x[:1].upper() + x[1:] for x in s.split("_"))
def camel(s): return (lambda p: p[:1].lower() + p[1:])(pas(s))


def kinds():
    out = []
    for c in COLLS:
        out.append(dict(c=c, verb="insert", kind=f"insert-{kebab(c['sing'])}", name=f"Insert{c['pas']}", emoji=INS))
        out.append(dict(c=c, verb="remove", kind=f"remove-{kebab(c['sing'])}", name=f"Remove{c['pas']}", emoji=REM))
        for f, t, en, de in c["fields"]:
            out.append(dict(c=c, verb="change", f=f, t=t, fen=en, fde=de, kind=f"change-{kebab(c['sing'])}-{ALIAS.get(f, kebab(f))}", name=f"Change{c['pas']}{pas(f)}", emoji=STR if t == "String" else NUM))
    return out


KINDS = kinds()
for k in KINDS:
    k["dir"] = k["emoji"] + k["kind"]
    k["mod"] = k["kind"].replace("-", "_")


def derive():
    return "#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]\n#[canonical_json(owner = semio_framework_pack_json)]\n#[cfg_attr(any(test, feature = \"test-support\"), derive(serde::Serialize, serde::Deserialize))]\n#[mutation_leaf(contract = ::protocol)]\n#[cfg_attr(any(test, feature = \"test-support\"), serde(rename_all = \"camelCase\"))]\n#[value(rename_all = \"camelCase\")]\n"


def label(k):
    c = k["c"]
    if k["verb"] == "insert": return f"Insert {c['en']}", f"{c['de']} einfügen"
    if k["verb"] == "remove": return f"Remove {c['en']}", f"{c['de']} entfernen"
    return f"Change {c['en']}: {k['fen'][:1].lower() + k['fen'][1:]}", f"{c['de']}: {k['fde']} ändern"


def leaf_rs(k):
    c, n, m, ln = k["c"], k["name"], k["mod"], label(k)
    sing = c["sing"]
    other = {"insert": f"remove_{sing}::Remove{c['pas']}", "remove": f"insert_{sing}::Insert{c['pas']}", "change": f"{m}::{n}"}[k["verb"]]
    other_mod = other.split("::")[0]
    if k["verb"] == "insert": fields = f"    pub index: Option<usize>,\n    pub {sing}: semio_s_artifact_norm_en1998_model::{c['row']},\n"
    elif k["verb"] == "remove": fields = "    pub id: String,\n"
    else: fields = f"    pub id: String,\n    pub new_{k['f']}: {k['t']},\n"
    entity = c["camel"] if k["verb"] != "change" else f"{kebab(c['sing'])}-{kebab(k['f'])}"
    target = 'vec!["' + k["kind"] + '".into()]' if k["verb"] == "insert" else "vec![self.id.clone()]"
    return f'''//! {k["emoji"]} `{k["kind"]}` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

{derive()}pub struct {n} {{
{fields}}}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for {n} where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::{other}> {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{
        verb: "{k["verb"]}",
        entity: "{entity}",
        kind: "{k["kind"]}",
        record: "{n}",
    }};

    fn label(&self) -> semio_ui_locale::LocalizedLabel {{
        semio_ui_locale::LocalizedLabel::native("{ln[0]}", "{ln[1]}")
    }}
    fn target(&self) -> Vec<String> {{
        {target}
    }}
}}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for {n} where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::{other}> {{
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {{
        crate::{m}::diff::diff(self, base)
    }}
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {{
        crate::{m}::inverse::inverse(self, base)
    }}
}}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
'''.replace(f"From<crate::{other}>", f"From<crate::{other}>")


def diff_rs(k):
    c, n, m = k["c"], k["name"], k["mod"]
    pl, sing, P = c["pl"], c["sing"], c["pas"]
    use = f"use semio_s_artifact_norm_en1998_diff::{{En1998{P}Delta, En1998Diff"
    if k["verb"] == "insert":
        return f'''//! {k["emoji"]} `{k["kind"]}` diff: inserts the {sing} row; an id the list holds is a `mutation.duplicate-id`.

use crate::{m}::{n};
{use}}};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &{n}, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {{
    if base.{pl}.iter().any(|existing| existing.id == payload.{sing}.id) {{
        let key = payload.{sing}.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{sing} {{key}} already exists."), [key]);
    }}
    let index = payload.index.unwrap_or(usize::MAX).min(base.{pl}.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff {{ {pl}: En1998{P}Delta::insertion(index, payload.{sing}.clone()), ..Default::default() }});
    if payload.index.is_none_or(|requested| requested == index) {{
        return outcome;
    }}
    outcome.warning("mutation.clamped", format!("Position {{}} is past the end of the {sing} list; inserted at {{index}}.", payload.index.unwrap_or(index)))
}}
'''
    if k["verb"] == "remove":
        return f'''//! {k["emoji"]} `{k["kind"]}` diff: removes the addressed {sing} row.

use crate::{m}::{n};
{use}}};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &{n}, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {{
    let Some(index) = base.{pl}.iter().position(|row| row.id == payload.id) else {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{sing} {{}} does not exist.", payload.id), [payload.id.clone()]);
    }};
    protocol::MutationOutcome::new(En1998Diff {{ {pl}: En1998{P}Delta::removal(&base.{pl}, index), ..Default::default() }})
}}
'''
    f, string = k["f"], k["t"] == "String"
    guard = "" if string else f'    if !payload.new_{f}.is_finite() {{\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }}\n'
    val = f"payload.new_{f}.clone()" if string else f"payload.new_{f}"
    return f'''//! {k["emoji"]} `{k["kind"]}` diff: patches `{f}` of the addressed {sing} row.

use crate::{m}::{n};
use semio_s_artifact_norm_en1998_diff::{{En1998Diff, En1998{P}Delta, En1998{P}Patch}};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &{n}, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {{
{guard}    let Some(r0) = base.{pl}.iter().find(|row| row.id == payload.id) else {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{sing} {{}} does not exist.", payload.id), [payload.id.clone()]);
    }};
    if r0.{f} == payload.new_{f} {{
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The {sing} already has this value.");
    }}
    protocol::MutationOutcome::new(En1998Diff {{ {pl}: En1998{P}Delta::modification(&payload.id, En1998{P}Patch {{ {f}: Some({val}), ..Default::default() }}), ..Default::default() }})
}}
'''


def inverse_rs(k):
    c, n, m = k["c"], k["name"], k["mod"]
    pl, sing, P = c["pl"], c["sing"], c["pas"]
    if k["verb"] == "insert":
        other = f"remove_{sing}"
        return f'''//! ↩️ `{k["kind"]}` inverse: removes the inserted row again.

use crate::{m}::{n};
use crate::{other}::Remove{P};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &{n}, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{other}::Remove{P}> {{
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}}

fn previous(payload: &{n}, base: &En1998Snapshot) -> Option<Remove{P}> {{
    let _ = base;
    Some(Remove{P} {{ id: payload.{sing}.id.clone() }})
}}
'''
    if k["verb"] == "remove":
        other = f"insert_{sing}"
        return f'''//! ↩️ `{k["kind"]}` inverse: inserts the removed row back at its position.

use crate::{m}::{n};
use crate::{other}::Insert{P};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &{n}, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{other}::Insert{P}> {{
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}}

fn previous(payload: &{n}, base: &En1998Snapshot) -> Option<Insert{P}> {{
    let (index, row) = base.{pl}.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(Insert{P} {{ index: Some(index), {sing}: row.clone() }})
}}
'''
    f = k["f"]
    cp = ".clone()" if k["t"] == "String" else ""
    return f'''//! ↩️ `{k["kind"]}` inverse: restores the previous `{f}`.

use crate::{m}::{n};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &{n}, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{m}::{n}> {{
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}}

fn previous(payload: &{n}, base: &En1998Snapshot) -> Option<{n}> {{
    let r0 = base.{pl}.iter().find(|row| row.id == payload.id)?;
    Some({n} {{ id: payload.id.clone(), new_{f}: r0.{f}{cp} }})
}}
'''


def schema_json(k, tag):
    c, n = k["c"], k["name"]
    props = {"mutation": {"const": n[0].lower() + n[1:]}}
    req = ["mutation"]
    target = {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": c["pas"], "de": c["de"]}, "description": {"en": f"The {c['camel']} this mutation addresses, by id.", "de": f"Das adressierte Element ({c['de']}), per Kennung."}, "ref": {"kind": c["camel"]}, "group": "target", "order": 10}}
    if k["verb"] == "insert":
        props["index"] = {"type": ["integer", "null"], "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Position", "de": "Position"}, "description": {"en": "Zero-based list position at which the record is inserted. Empty appends; positions past the end append.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird. Ohne Angabe am Ende; Positionen hinter dem Ende hängen an."}, "step": 1, "precision": 0, "group": "target", "order": 10}}
        props[c["camel"]] = {"$ref": f"https://json.schemas.assets.semio-tech.com/s/norm/en1998/snapshot.json#/$defs/{c['row']}", "x-semio-ui": {"role": "value", "label": {"en": c["pas"], "de": c["de"]}, "description": {"en": f"The complete {c['camel']} record to insert.", "de": f"Der vollständige einzufügende Datensatz ({c['de']})."}, "group": "record", "order": 20}, "x-semio-rust": {"type": f"semio_s_artifact_norm_en1998_model::{c['row']}"}}
        req.append(c["camel"])
    elif k["verb"] == "remove":
        props["id"] = target; req.append("id")
    else:
        props["id"] = target
        key = "new" + pas(k["f"])
        ui = {"label": {"en": k["fen"], "de": k["fde"]}, "role": "value", "group": "value", "order": 20}
        if k["t"] == "String":
            props[key] = {"type": "string", "x-semio-ui": {**ui, "widget": "text"}}
        else:
            props[key] = {"type": "number", "x-semio-ui": {**ui, "widget": "stepper", "step": 0.01, "precision": 3}}
        req += ["id", key]
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/{k['kind']}/schema.json", "title": n, "type": "object", "additionalProperties": False, "required": req, "properties": props}


def schema_ts(k):
    c, n = k["c"], k["name"]
    lit = n[0].lower() + n[1:]
    if k["verb"] == "insert":
        return f'''/** {k["emoji"]} `{k["kind"]}` wire twin: the leaf payload `{n}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader }} from "@semio-tech/norm-js/registry/contract";
import {{ type {c["row"]}, parse{c["row"]} }} from "../../../📸️snapshot/🟦️.ts";

export interface {n} {{
  mutation: "{lit}";
  index?: number | null;
  {c["camel"]}: {c["row"]};
}}

export const parse{n}: NormWireReader<{n}> = normWireObject<{n}>({{ mutation: normWireRequired(normWireLiteral("{lit}")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {{"minimum":0}}))), {c["camel"]}: normWireRequired(parse{c["row"]}) }});
'''
    if k["verb"] == "remove":
        return f'''/** {k["emoji"]} `{k["kind"]}` wire twin: the leaf payload `{n}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader }} from "@semio-tech/norm-js/registry/contract";

export interface {n} {{
  mutation: "{lit}";
  id: string;
}}

export const parse{n}: NormWireReader<{n}> = normWireObject<{n}>({{ mutation: normWireRequired(normWireLiteral("{lit}")), id: normWireRequired(normWireString) }});
'''
    key = "new" + pas(k["f"])
    s = k["t"] == "String"
    imp = "normWireLiteral, normWireObject, normWireRequired, normWireString" if s else "normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString"
    return f'''/** {k["emoji"]} `{k["kind"]}` wire twin: the leaf payload `{n}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import {{ {imp}, type NormWireReader }} from "@semio-tech/norm-js/registry/contract";

export interface {n} {{
  mutation: "{lit}";
  id: string;
  {key}: {"string" if s else "number"};
}}

export const parse{n}: NormWireReader<{n}> = normWireObject<{n}>({{ mutation: normWireRequired(normWireLiteral("{lit}")), id: normWireRequired(normWireString), {key}: normWireRequired({"normWireString" if s else "normWireNumber"}) }});
'''


def after(text, anchor_line_sub, new_lines, which="last"):
    lines = text.split("\n")
    idx = [i for i, l in enumerate(lines) if anchor_line_sub in l]
    assert idx, anchor_line_sub
    i = idx[-1] if which == "last" else idx[0]
    return "\n".join(lines[:i + 1] + new_lines + lines[i + 1:])


def main():
    tag0 = max(int(x) for x in re.findall(r"tag=(\d+)", rd(BIN))) + 1
    proto0 = max(int(x) for x in re.findall(r"= (\d+);", (lambda s: s[s.index("oneof"):s.index("oneof") + 200000])(rd(MUT + "/🛰️.proto")).split("}")[0]))
    done = os.path.exists(MUT + "/" + KINDS[0]["dir"])
    for i, k in enumerate(KINDS):
        d = MUT + "/" + k["dir"]
        if os.path.exists(d): continue
        tag = tag0 + i
        wr(d + "/🦀️.rs", leaf_rs(k))
        wr(d + "/🔺️diff/🦀️.rs", diff_rs(k))
        wr(d + "/↩️inverse/🦀️.rs", inverse_rs(k))
        wr(d + "/🧬️schema/🔣️.json", json.dumps(schema_json(k, tag), indent=2, ensure_ascii=False) + "\n")
        wr(d + "/🧬️schema/🟦️.ts", schema_ts(k))
        lab = label(k)[0]
        wr(d + "/🔣️.json", json.dumps({"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + k["dir"], "semanticKind": k["kind"], "displayName": lab, "emoji": k["emoji"], "aggregateVariant": k["name"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}, indent=2, ensure_ascii=False) + "\n")
    if done:
        fixtures()
        print("leaf directories present; registries untouched; fixtures written")
        return
    new = lambda f: [f(k) for k in KINDS]
    p = MUT + "/🔗️tie-beams/🦀️.rs"
    s = rd(p)
    s = s.replace("//#endregion 🪢️Mounts", "".join(f'#[path = "../{k["dir"]}/🦀️.rs"]\npub mod {k["mod"]};\n' for k in KINDS) + "//#endregion 🪢️Mounts", 1)
    wr(p, s)
    p = MUT + "/🦀️.rs"; s = rd(p)
    s = after(s, "mut_tie_beams::change_tie_beam_span;", new(lambda k: f"use semio_s_artifact_norm_en1998_mut_tie_beams::{k['mod']};"))
    s = after(s, "    ChangeTieBeamSpan(change_tie_beam_span::ChangeTieBeamSpan),", new(lambda k: f"    {k['name']}({k['mod']}::{k['name']}),"))
    s = after(s, '    "change-tie-beam-span",', new(lambda k: f'    "{k["kind"]}",'))
    wr(p, s)
    p = MUT + "/🟦️.ts"; s = rd(p)
    s = after(s, 'from "./📍️change-tie-beam-span/🧬️schema/🟦️.ts";', new(lambda k: f'import {{ type {k["name"]}, parse{k["name"]} }} from "./{k["dir"]}/🧬️schema/🟦️.ts";'))
    s = after(s, "  | ChangeTieBeamSpan", new(lambda k: f"  | {k['name']}"))
    s = after(s, "  changeTieBeamSpan: parseChangeTieBeamSpan,", new(lambda k: f"  {k['name'][0].lower() + k['name'][1:]}: parse{k['name']},"))
    wr(p, s)
    p = MUT + "/🔣️.json"; j = json.loads(rd(p))
    refs = [x.get("$ref", "") for x in j["oneOf"]]
    at = max(i for i, r in enumerate(refs) if "/change-tie-beam-span/" in r)
    for k in reversed(KINDS):
        j["oneOf"].insert(at + 1, {"$ref": f"https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/{k['kind']}/schema.json"})
    wr(p, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    p = MUT + "/🔗️.graphql"; s = rd(p)
    lines = s.split("\n"); assert lines[0].startswith("union En1998Mutation =")
    lines[0] += "".join(f" | {k['name']}" for k in KINDS)
    body = ""
    for k in KINDS:
        c = k["c"]
        if k["verb"] == "insert": fl = f"  index: Int\n  {c['camel']}: {c['row']}!\n"
        elif k["verb"] == "remove": fl = "  id: String!\n"
        else: fl = f"  id: String!\n  new{pas(k['f'])}: {'String!' if k['t'] == 'String' else 'Float!'}\n"
        body += f"type {k['name']} {{\n{fl}}}\n"
    wr(p, "\n".join(lines).rstrip("\n") + "\n" + body)
    p = MUT + "/🛰️.proto"; s = rd(p)
    s = after(s, "    ChangeTieBeamWidth change_tie_beam_width = 321;", [f"    {k['name']} {k['mod']} = {proto0 + 1 + i};" for i, k in enumerate(KINDS)])
    body = ""
    for k in KINDS:
        c = k["c"]
        if k["verb"] == "insert": fl = f"  optional uint64 index = 1;\n  {c['row']} {c['sing']} = 2;\n"
        elif k["verb"] == "remove": fl = "  string id = 1;\n"
        else: fl = f"  string id = 1;\n  {'string' if k['t'] == 'String' else 'double'} new_{k['f']} = 2;\n"
        body += f"message {k['name']} {{\n{fl}}}\n"
    wr(p, s.rstrip("\n") + "\n" + body)
    wr(BIN, rd(BIN).rstrip("\n") + "\n" + "".join(f"record {k['kind']} tag={tag0 + i}\nfield payload bytes\n" for i, k in enumerate(KINDS)))
    p = MUT + "/🧭️edit-rules/🦀️.rs"; s = rd(p)
    s = after(s, 'kind: "change-tie-beam-span"', [f'        SetFieldRule {{ path: "{k["c"]["camel_pl"]}[].{camel(k["f"])}", kind: "{k["kind"]}", selectors: &[SelectorKey::Id("id")], value: "new{pas(k["f"])}" }},' for k in KINDS if k["verb"] == "change"])
    s = after(s, 'kind: "insert-tie-beam"', [f'        InsertItemRule {{ path: "{c["camel_pl"]}", kind: "insert-{kebab(c["sing"])}", selectors: &[], index: "index", item: "{c["camel"]}" }},' for c in COLLS])
    s = after(s, 'kind: "remove-tie-beam"', [f'        IdRemoval {{ path: "{c["camel_pl"]}", kind: "remove-{kebab(c["sing"])}", selectors: &[], id: "id" }},' for c in COLLS])
    wr(p, s)
    p = TXT + "/🔗️tie-beams/🦀️.rs"; s = rd(p)
    s = after(s, "mut_tie_beams::change_tie_beam_span::ChangeTieBeamSpan;", [f"use semio_s_artifact_norm_en1998_mut_tie_beams::{k['mod']}::{k['name']};" for k in KINDS])
    fns = ""
    for k in KINDS:
        c, n, m, kd = k["c"], k["name"], k["mod"], k["kind"]
        if k["verb"] == "insert":
            pr = f'format!("{kd} index={{}} {c["sing"]}={{}}", enc_json(&p.index), enc_json(&p.{c["sing"]}))'
            pa = f'{n} {{ index: opt_index(args)?, {c["sing"]}: dec_json(&arg("{c["sing"]}")?)? }}'
        elif k["verb"] == "remove":
            pr = f'format!("{kd} id={{}}", enc_json(&p.id))'
            pa = f'{n} {{ id: dec_json(&arg("id")?)? }}'
        else:
            kb = kebab(k["f"])
            pr = f'format!("{kd} id={{}} new-{kb}={{}}", enc_json(&p.id), enc_json(&p.new_{k["f"]}))'
            pa = f'{n} {{ id: dec_json(&arg("id")?)?, new_{k["f"]}: dec_json(&arg("new-{kb}")?)? }}'
        fns += f'''
/// ✍️ Prints `{kd}` as an op-text line.
pub fn print_{m}(p: &{n}) -> String {{
    {pr}
}}

/// 📖 Parses the arguments of `{kd}`.
pub fn parse_{m}(args: &Args) -> Result<{n}, String> {{
    let arg = |k: &str| required(args, "{kd}", k);
    Ok({pa})
}}
'''
    wr(p, s.rstrip("\n") + "\n" + fns)
    p = TXT + "/🦀️.rs"; s = rd(p)
    s = after(s, "En1998Mutation::ChangeTieBeamSpan(p) =>", [f"        En1998Mutation::{k['name']}(p) => {GROUP_TEXT}::print_{k['mod']}(p)," for k in KINDS])
    s = after(s, '"change-tie-beam-span" =>', [f'        "{k["kind"]}" => {GROUP_TEXT}::parse_{k["mod"]}(&args).map(En1998Mutation::{k["name"]}),' for k in KINDS])
    wr(p, s)
    p = FEATURE; s = rd(p)
    rows = [f"      | {k['kind']} | {k['dir']} | ✅apply  |" for k in KINDS]
    lines = s.split("\n")
    out = []
    for l in lines:
        out.append(l)
        if l.startswith("      | change-tie-beam-width |"):
            out += rows
    wr(p, "\n".join(out))
    fixtures()
    print("generated", len(KINDS))


def fixtures():
    base = json.loads(rd(FIX + "/➕️insert-tie-beam/✅apply/🔣️.before.json"))
    tmpl = json.loads(rd(FIX + "/📍️change-tie-beam-span/✅apply/🔣️.diff.json"))
    empty = {"removed": [], "inserted": [], "moved": [], "modified": []}
    fmt = lambda d: json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    def diff(coll, delta):
        d = {key: (None if key in ("annex", "site") else copy.deepcopy(empty)) for key in tmpl}
        d[coll] = {**empty, **delta}
        return d
    for k in KINDS:
        c, root = k["c"], FIX + "/" + k["dir"] + "/✅apply/"
        row, coll = dict(c["sample"]), c["camel_pl"]
        b, a = copy.deepcopy(base), copy.deepcopy(base)
        b[coll], a[coll] = [], []
        if k["verb"] == "insert":
            a[coll] = [row]
            mut = {"mutation": k["name"][0].lower() + k["name"][1:], "index": 0, c["camel"]: row}
            dlt = {"inserted": [{"index": 0, "row": row}]}
        elif k["verb"] == "remove":
            b[coll] = [row]
            mut = {"mutation": k["name"][0].lower() + k["name"][1:], "id": row["id"]}
            dlt = {"removed": [{"id": row["id"], "index": 0}]}
        else:
            leaf = camel(k["f"]); old = row[leaf]
            new = old + "-2" if k["t"] == "String" else (old + 1.0 if old != 0 else 1.0)
            b[coll] = [row]; a[coll] = [{**row, leaf: new}]
            mut = {"mutation": k["name"][0].lower() + k["name"][1:], "id": row["id"], "new" + pas(k["f"]): new}
            patch = {camel(f): None for f, *_ in c["fields"]}
            patch[leaf] = new
            dlt = {"modified": [{"id": row["id"], "patch": patch}]}
        wr(root + "🔣️.before.json", fmt(b)); wr(root + "🔣️.after.json", fmt(a))
        wr(root + "🔣️.mutation.json", fmt(mut)); wr(root + "🔣️.diff.json", fmt(diff(coll, dlt)))
        wr(root + "🔣️.outcome.json", fmt({"status": "applied"}))


main()
