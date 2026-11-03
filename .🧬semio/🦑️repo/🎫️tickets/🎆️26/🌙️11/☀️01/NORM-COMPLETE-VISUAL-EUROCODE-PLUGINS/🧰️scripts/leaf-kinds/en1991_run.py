"""EN 1991 leaf-kinds F: python -I en1991_run.py [--apply]. Id-selector set-field kinds for every content leaf without a kind (six wave 3 collections, specialVehicles, floors, roofs, windFaces, selfWeightElements, accidental variant blocks), insert (index + item) and id-only remove kinds for the seven new collections. Private stage; id removal goes through `IdRemoval` + `resolve_edit` like en1992."""
import copy, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I, keyed as K, keyed_fixtures as X

SHORT, P = "en1991", "En1991"
STAGE = TICKET + "/🗑️generated/leaf-kinds/stage-f/en1991"
ROOT = NORM + "/🗿️artifacts/🏋️en1991"
MODEL = "semio_s_artifact_norm_en1991_model"
ST = I.Style(P, MODEL, "semio_s_artifact_norm_en1991_diff", "En1991Snapshot", "../../../📸️snapshot/🟦️.ts", mode="single", opt_index=False, tagged=False, item_field=lambda e: "item", ts_prefix=P)
ANY = ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
_s = rd(MUT + "/🦀️.rs")
ANCHOR_CRATE = re.search(r"^use (semio_s_artifact_norm_en1991_mut_[a-z0-9_]+)::change_floor_area;", _s, re.M).group(1).replace("_", "-")
_p = re.search(r'\[workspace\.dependencies\.%s\]\npath = "([^"]+)"' % ANCHOR_CRATE, rd(ROOT + "/Cargo.toml")).group(1)
ANCHOR_DIR = _p.split("/🧬️mutations/")[1].split("/")[0]
art = K.Art(SHORT, "🏋️", ST, SHORT, ANCHOR_DIR, ANCHOR_CRATE, "change-floor-area", None)

L = I.Link
COLLS = [
    ("floors", "FloorArea", "floor", "floor", "Floor area", "Geschossfläche", "🏢", "gravity-content", False),
    ("self_weight_elements", "SelfWeightElement", "self-weight-element", "self-weight", "Self-weight element", "Eigengewichtselement", "🧱", "gravity-content", False),
    ("roofs", "RoofArea", "roof", "roof", "Roof", "Dach", "🏠", "roof-content", False),
    ("wind_faces", "WindFace", "wind-face", "wind-face", "Wind face", "Windfläche", "🌬", "wind-face-coefficients", False),
    ("special_vehicles", "SpecialVehicle", "special-vehicle", "special-vehicle", "Special vehicle", "Sonderfahrzeug", "🚛", "special-vehicles", True),
    ("wind_bridges", "WindBridge", "wind-bridge", "wind-bridge", "Bridge wind case", "Brückenwindfall", "🌉", "wind-bridges", True),
    ("dynamic_elements", "DynamicElement", "dynamic-element", "dynamic-element", "Dynamic element", "Schwingungsanfälliges Bauteil", "🌀", "dynamic-elements", True),
    ("impact_actions", "ImpactAction", "impact-action", "impact-action", "Impact action", "Anpralleinwirkung", "💥", "impact-actions", True),
    ("internal_explosions", "InternalExplosion", "internal-explosion", "internal-explosion", "Internal explosion", "Innenexplosion", "🧨", "internal-explosions", True),
    ("thermal_sections", "ThermalSection", "thermal-section", "thermal-section", "Thermal section", "Temperaturquerschnitt", "🌡", "thermal-sections", True),
    ("helicopter_landings", "HelicopterLanding", "helicopter-landing", "helicopter", "Helicopter landing", "Hubschrauberlandung", "🚁", "helicopter-landings", True),
    ("tie_runs", "TieRun", "tie-run", "tie-run", "Tie run", "Zuggliedstrang", "🔗", "tie-runs", True),
    ("construction_loads", "ConstructionLoad", "construction-load", "construction-load", "Construction load", "Bauzustandslast", "🏗", "construction-loads", True),
    ("falling_loads", "FallingLoad", "falling-load", "falling", "Falling load", "Fallende Last", "⬇", "falling-loads", True),
    ("execution_imperfections", "ExecutionImperfection", "execution-imperfection", "imperfection", "Execution imperfection", "Ausführungsimperfektion", "📐", "execution-imperfections", True),
    ("derailment_cases", "DerailmentCase", "derailment-case", "derailment", "Derailment case", "Entgleisungsfall", "🚆", "derailment-cases", True),
    ("crane_hoists", "CraneHoist", "crane-hoist", "crane-hoist", "Crane hoist", "Kranhubwerk", "🏗", "crane-hoists", True),
    ("footbridges", "Footbridge", "footbridge", "footbridge", "Footbridge", "Fußgängerbrücke", "🚶", "footbridges", True),
    ("rail_dynamics", "RailDynamic", "rail-dynamic", "rail-dynamic", "Rail span", "Eisenbahnfeld", "🛤", "rail-dynamics", True),
    ("load_dispersals", "LoadDispersal", "load-dispersal", "load-dispersal", "Load dispersal", "Lastausbreitung", "📏", "load-dispersals", True),
    ("bulk_solids", "BulkSolid", "bulk-solid", "bulk-solid", "Bulk solid", "Schüttgut", "🌾", "bulk-solids", True),
    ("rotating_machines", "RotatingMachine", "rotating-machine", "rotating-machine", "Rotating machine", "Rotierende Maschine", "⚙", "rotating-machines", True),
]
FAMILIES = {"tie-runs": {"spacing": "loading", "span": "loading", "dead_load": "loading", "imposed_load": "loading", "psi": "loading", "kind": "position", "x": "position", "y": "position", "provided_force": "position"}, "wind-bridges": {"traffic_kind": "loading", "height_above_ground": "loading", "span": "loading", "assumed_force": "loading"}, "dynamic-elements": {"mass_per_length": "response", "strouhal": "response", "assumed_cscd": "response"}, "impact-actions": {"height": "position", "x": "position", "y": "position", "assumed_force": "position"}}
ABBREV = [("stopping-distance", "stopping"), ("design-force", "force"), ("execution-imperfection", "imperfection"), ("construction-load", "construction"), ("helicopter-landing", "helicopter"), ("derailment-case", "derailment"), ("concentrated", "conc"), ("height-above-ground", "ground-height"), ("internal-explosion", "explosion"), ("dynamic-element", "dynamic"), ("thermal-section", "thermal")]
SAMPLE = {"designation": "4 x 150"}
VARIANTS = [("impact", "vehicle_mass", "Vehicle mass", "Fahrzeugmasse", "change-accidental-vehicle-mass"), ("impact", "vehicle_speed", "Vehicle speed", "Fahrzeuggeschwindigkeit", "change-accidental-vehicle-speed"), ("explosion", "explosion_mass", "Explosion charge mass", "Explosionsmasse", "change-accidental-explosion-mass"), ("explosion", "standoff", "Standoff distance", "Abstand", "change-accidental-standoff"), ("explosion", "assumed_pressure", "Assumed accidental pressure", "Angenommener außergewöhnlicher Druck", "change-accidental-assumed-pressure")]


def patch_types():
    s = rd(ANY + "/🧬️schema/🔺️diff/🦀️.rs")
    out = {}
    for m in re.finditer(r"pub En1991(\w+)Patch of [\w:]+ \{ set \{ ([^}]*) \} \}", s):
        out[m.group(1)] = [(a.split(": ")[0], a.split(": ")[1]) for a in m.group(2).split(", ")]
    return out


def meta_labels():
    s = rd(ANY + "/✏️editor/🏷️field-meta/🦀️.rs")
    return {m.group(1): (m.group(2), m.group(3)) for m in re.finditer(r'"([\w\[\]\.]+)" => FieldSpec::\w+\("([^"]+)", "([^"]+)"', s)}


class VariantKind(I.SetKind):
    def __init__(self, leaf, block):
        super().__init__(leaf)
        self.block = block

    def diff_rs(self, st):
        l = self.leaf
        guard = '    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n' % l.new
        return I.doc_head(self.ent.emoji, "`%s` diff: patches `%s` of the %s block of the addressed accidental case." % (self.kind, l.field, self.block)) + '''use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1991_diff::{En1991AccidentalCaseDelta, En1991AccidentalCasePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &%(rec)s, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
%(guard)s    let Some(r0) = base.accidental_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Accidental case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(current) = r0.%(b)s.first() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The accidental case has no %(b)s block.", [payload.id.clone()]);
    };
    if current.%(f)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The accidental case already has this value.");
    }
    let mut %(b)s = r0.%(b)s.clone();
    %(b)s[0].%(f)s = payload.%(new)s;
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: En1991AccidentalCaseDelta::modification(&payload.id, En1991AccidentalCasePatch { %(b)s: Some(%(b)s), ..Default::default() }), ..Default::default() })
}
''' % dict(mod=self.module, rec=self.record, guard=guard, b=self.block, f=l.field, new=l.new)

    def inverse_rs(self, st):
        l = self.leaf
        return I.doc_head("↩️", "`%s` inverse: restores the previous `%s`." % (self.kind, l.field)) + '''use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &%(rec)s, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%(mod)s::%(rec)s> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &%(rec)s, base: &En1991Snapshot) -> Option<%(rec)s> {
    let r0 = base.accidental_cases.iter().find(|row| row.id == payload.id)?;
    let current = r0.%(b)s.first()?;
    Some(%(rec)s { id: payload.id.clone(), %(new)s: current.%(f)s })
}
''' % dict(mod=self.module, rec=self.record, b=self.block, f=l.field, new=l.new)


def fits(ent, kind):
    d = art.fix + "/" + emoji_dir(ent.emoji, kind) + "/✅apply/🔣️.before.json"
    return len(d[len(SEMIO) + 1:].encode()) < 256


def shorten(ent, kind):
    for a, b in ABBREV:
        if fits(ent, kind):
            break
        kind = kind.replace(a, b)
    assert fits(ent, kind), kind
    return kind


def build():
    schema = json.loads(rd(ANY + "/🧬️schema/📸️snapshot/🔣️.json"))
    defs = schema["$defs"]
    types, labels = patch_types(), meta_labels()
    rules = rd(MUT + "/🧭️edit-rules/🦀️.rs")
    covered = set(re.findall(r'SetFieldRule \{ path: "([^"]+)"', rules))
    ents, kinds, rows = [], [], {}
    for coll, row, single, slug, en, de, emoji, group, new in COLLS:
        link = L(coll, row, "id", single, en, de)
        ent = I.Ent(slug, emoji, [link], en, de, slug=slug, group=group)
        ents.append((ent, coll, row))
        props = defs[row]["properties"]
        sample = {"id": slug + "-1"}
        ts = dict(types[pascal(single)])
        for f, ty in types[pascal(single)]:
            cc = camel(f)
            path = "%s[].%s" % (camel(coll), cc)
            sample[cc] = SAMPLE.get(f, {"f64": 1.5, "u8": 1, "u32": 1, "bool": False, "String": "x"}[ty])
            if path in covered:
                continue
            ui = dict(props[cc].get("x-semio-ui") or {})
            if path in labels:
                ui["label"] = {"en": labels[path][0], "de": labels[path][1]}
            leaf = I.Leaf(ent, [f], ty, ui, owner=row, kind="change-%s-%s" % (slug, kebab(f)))
            leaf.kind = shorten(ent, leaf.kind)
            leaf.set_names()
            k = I.SetKind(leaf)
            k.family = FAMILIES.get(group, {}).get(f, "")
            kinds.append(k)
        rows[coll] = sample
        if new:
            ins, rm = I.pair(ent, shorten(ent, "insert-" + slug), shorten(ent, "remove-" + slug), ST, by_index=False)
            for x in (ins, rm):
                x.group, x.family = group, ""
            kinds += [ins, rm]
    acc = I.Ent("accidental", "🚗", [L("accidental_cases", "AccidentalCase", "id", "accidental-case", "Accidental case", "Außergewöhnlicher Fall")], "Accidental case", "Außergewöhnlicher Fall", slug="accidental", group="accidental-case-variants")
    variants = []
    for block, f, en, de, kind in VARIANTS:
        path = "accidentalCases[].%s[].%s" % (block, camel(f))
        if path in covered:
            continue
        leaf = I.Leaf(acc, [f], "f64", {"label": {"en": en, "de": de}}, owner="Accidental" + block.capitalize(), kind=shorten(acc, kind))
        k = VariantKind(leaf, block)
        k.family = ""
        kinds.append(k)
        variants.append(k)
    return defs, ents, kinds, variants, rows


def main():
    F = K.KFiles(STAGE)
    defs, ents, kinds, variants, rows = build()
    existing = set(K.kinds_of(art, F))
    kinds = [k for k in kinds if k.kind not in existing]
    if not kinds:
        print("nothing to do: all EN 1991 kinds exist")
        return
    assert len({k.kind for k in kinds}) == len(kinds)
    tags = K.binary_protocol_keep(art, F, kinds, set())
    existing_crates = set(re.findall(r'"(semio-s-artifact-norm-en1991-mut-[a-z0-9-]+)"', rd(ROOT + "/Cargo.toml")))
    groups = K.group_kinds(art, kinds, existing_crates)
    names = K.create_crates(art, F, groups)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    K.register(art, F, names)
    K.leaf_files(art, F, kinds, defs, tags)
    K.root_wire(art, F, kinds, groups_of)
    K.edit_rules(art, F, [k for k in kinds if not isinstance(k, I.RemoveKind)])
    edit_rules_post(F, kinds, variants)
    K.harness(art, F, kinds)
    K.oracle(art, F, kinds)
    K.taxonomy(art, F, kinds)
    fixtures(F, kinds, variants, defs, rows)
    editor_wire(F)
    print("kinds", len(kinds), "crates", len(names), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply())


def edit_rules_post(F, kinds, variants):
    p = MUT + "/🧭️edit-rules/🦀️.rs"
    s = F.read(p)
    for k in variants:
        old = 'SetFieldRule { path: "accidentalCases[].%s.%s", kind: "%s", selectors: &[SelectorKey::Id("id")]' % (k.block, camel(k.leaf.field), k.kind)
        old = old.replace("accidentalCases[].", "accidentalCases[].", 1)
        pat = re.compile(r'SetFieldRule \{ path: "accidentalCases\[\]\.%s", kind: "%s", selectors: &\[SelectorKey::Id\("id"\)\]' % (camel(k.leaf.field), k.kind))
        assert pat.search(s), k.kind
        s = pat.sub('SetFieldRule { path: "accidentalCases[].%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("id"), SelectorKey::Implied]' % (k.block, camel(k.leaf.field), k.kind), s)
    rem = ""
    for k in kinds:
        if isinstance(k, I.RemoveKind):
            rem += '        IdRemoval { path: "%s", kind: "%s", selectors: &[], id: "id" },\n' % (camel(k.ent.last.coll), k.kind)
    block = '''
/// 🆔️ A collection whose remove kind addresses the row by its id instead of its position.
#[derive(Clone, Copy, Debug)]
pub struct IdRemoval {
    pub path: &'static str,
    pub kind: &'static str,
    pub id: &'static str,
}

/// 🆔️ Every collection of this family removed by row id.
pub const ID_REMOVALS: &[IdRemoval] = &[
%s];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &En1991Snapshot, edit: &NormEdit) -> Result<Vec<En1991Mutation>, String> {
    if let NormEdit::RemoveItem { path, index } = edit {
        if let Some(rule) = ID_REMOVALS.iter().find(|rule| rule.path == path) {
            let tree = semio_framework_value::ToValue::to_value(document);
            let mut cursor = &tree;
            for segment in path.split('.') {
                let semio_framework_value::DslValue::Object(entries) = cursor else { return Err(format!("expected object before '{segment}' in path '{path}'")) };
                cursor = &entries.iter().find(|(key, _)| key == segment).ok_or_else(|| format!("missing field '{segment}' in path '{path}'"))?.1;
            }
            let semio_framework_value::DslValue::Array(items) = cursor else { return Err(format!("'{path}' is not a list")) };
            let semio_framework_value::DslValue::Object(row) = items.get(*index).ok_or_else(|| format!("'{path}' has no row {index}"))? else { return Err(format!("row {index} of '{path}' is not a record")) };
            let id = row.iter().find(|(key, _)| key == "id").map(|(_, value)| value.clone()).ok_or_else(|| format!("row {index} of '{path}' has no id"))?;
            return <En1991Mutation as protocol::Mutation<En1991Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
''' % rem
    if "ID_REMOVALS" not in s:
        s = s.replace("use semio_s_plugin_norm_app_surface::{InsertItemRule, NormEditRules,", "use semio_s_plugin_norm_app_surface::{InsertItemRule, NormEdit, NormEditRules,", 1)
        s = s.replace("\n/// 📚 Every path", "use crate::En1991Mutation;\nuse semio_s_artifact_norm_en1991_model::En1991Snapshot;\n\n/// 📚 Every path", 1)
        s = s.rstrip("\n") + "\n" + block
    else:
        s = rep(s, "ID_REMOVALS: &[IdRemoval] = &[\n", "ID_REMOVALS: &[IdRemoval] = &[\n" + rem, 1)
    F.write(p, s)
    p = MUT + "/🦀️.rs"
    s = F.read(p)
    s = s.replace("pub use crate::edit_rules::EDIT_RULES;", "pub use crate::edit_rules::{resolve_edit, EDIT_RULES};", 1)
    F.write(p, s)


def editor_wire(F):
    for cmd in ("✏️set-field", "➕insert-item", "➖remove-item", "🩹apply-remedy"):
        p = ANY + "/✏️editor/🎮️commands/" + cmd + "/🦀️.rs"
        s = F.read(p)
        s = s.replace("semio_s_artifact_norm_en1991_mutations::EDIT_RULES.resolve(document, edit)", "semio_s_artifact_norm_en1991_mutations::resolve_edit(document, edit)")
        F.write(p, s)


def fixtures(F, kinds, variants, defs, rows):
    ref = art.fix + "/🗺️change-floor-area/✅apply"
    base = json.loads(rd(ref + "/🔣️.before.json"))
    diff = json.loads(rd(ref + "/🔣️.diff.json"))
    blank = {k: ({"removed": [], "inserted": [], "moved": [], "modified": []} if isinstance(v, dict) else None) for k, v in diff.items()}
    types = patch_types()
    spec = {}
    for coll, row, single, *_ in COLLS:
        spec[pascal(single)] = ([camel(f) for f, _ in types[pascal(single)]], [])
        if not base[camel(coll)]:
            base[camel(coll)] = [copy.deepcopy(rows[coll])]
    spec["AccidentalCase"] = (["impact", "explosion"], [])
    base["accidentalCases"].append({"id": "explosion-1", "impact": [], "explosion": [{"explosionMass": 100.0, "standoff": 5.0, "assumedPressure": 20000.0}]})
    plain = [k for k in kinds if k not in variants]
    byent = {k.ent.last.single: rows for k in plain}
    X.build(art, F, plain, defs, base, spec, blank, lambda k, coll, b: dict(next(r for c, r in rows.items() if camel(c) == camel(k.ent.last.coll)), id=next(r for c, r in rows.items() if camel(c) == camel(k.ent.last.coll))["id"] + "-new"))
    for k in kinds:
        if isinstance(k, I.RemoveKind):
            d = art.fix + "/" + k.dirname + "/✅apply/🔣️.mutation.json"
            gone = base[camel(k.ent.last.coll)][0]["id"]
            F.write(d, dumps({k.record: {"id": gone}}))
    for k in variants:
        l = k.leaf
        i = 0 if k.block == "impact" else len(base["accidentalCases"]) - 1
        after = copy.deepcopy(base)
        cur = after["accidentalCases"][i][k.block]
        old = cur[0][camel(l.field)]
        val = round(old * 2.0, 9)
        cur[0][camel(l.field)] = val
        case = base["accidentalCases"][i]
        rows_lane = {"removed": [], "inserted": [], "moved": [], "modified": [{"id": case["id"], "patch": {"impact": None, "explosion": None, k.block: after["accidentalCases"][i][k.block]}}]}
        d2 = copy.deepcopy(blank)
        d2["accidentalCases"] = rows_lane
        X.write(art, F, k, base, after, {"id": case["id"], l.newc: val}, d2)


if __name__ == "__main__":
    shutil.rmtree(STAGE, ignore_errors=True)
    main()
