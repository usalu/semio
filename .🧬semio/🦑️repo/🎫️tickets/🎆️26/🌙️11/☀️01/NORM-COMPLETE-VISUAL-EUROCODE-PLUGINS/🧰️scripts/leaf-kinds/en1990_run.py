"""EN 1990 profile and driver: python -I en1990_run.py [--apply]. Typed set-field kinds per row leaf (selector `id`, effects by `index`), root scalars, insert/remove-row kinds for the new collections, retirement of the seven whole-collection `change-<collection>` kinds."""
import copy, glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I, keyed as K, keyed_fixtures as X

ART = "en1990"
STAGE_A = TICKET + "/🗑️generated/leaf-kinds/stage-a/en1990"
MODEL = "semio_s_artifact_norm_en1990_model"
ST = I.Style("En1990", MODEL, "semio_s_artifact_norm_en1990_diff", "En1990Snapshot", "../../../📸️snapshot/🟦️.ts", mode="single", opt_index=True, tagged=True, item_field=lambda e: "item", ts_prefix="En1990")
art = K.Art(ART, "⚖️", ST, "en1990", "🏗️members", "semio-s-artifact-norm-en1990-mut-members", "change-annex", None)

L = I.Link
PERM = L("permanents", "PermanentAction", "id", "permanent", "Permanent action", "Ständige Einwirkung")
VARI = L("variables", "VariableAction", "id", "variable", "Variable action", "Veränderliche Einwirkung")
ACCI = L("accidentals", "AccidentalAction", "id", "accidental", "Accidental action", "Außergewöhnliche Einwirkung")
SEIS = L("seismics", "SeismicAction", "id", "seismic", "Seismic action", "Erdbebeneinwirkung")
MEMB = L("members", "Member", "id", "member", "Member", "Bauteil")
BRID = L("bridge_sls", "BridgeSls", "id", "bridge_sls", "Bridge serviceability", "Brückengebrauchstauglichkeit")
EFFE = L("effects", "MemberEffect", "index", "effect", "Member effect", "Bauteilwirkung")
FATI = L("fatigue_spectra", "FatigueSpectrum", "id", "fatigue_spectrum", "Fatigue spectrum", "Ermüdungskollektiv")
RELI = L("reliability_analyses", "ReliabilityAnalysis", "id", "reliability_analysis", "Reliability analysis", "Zuverlässigkeitsanalyse")
TEST = L("test_series", "TestSeries", "id", "test_series", "Test series", "Versuchsreihe")
PLAC = L("placements", "Placement", "id", "placement", "Placement", "Platzierung")
FACT = L("factor_overrides", "FactorOverride", "id", "factor_override", "Factor override", "Faktorüberschreibung")
TRAF = L("traffic_groups", "TrafficGroup", "id", "traffic_group", "Traffic group", "Verkehrsgruppe")

E = I.Ent
ENTS = [
    E("permanent", "⚓️", [PERM], "Permanent action", "Ständige Einwirkung", slug="permanent", group="actions-fields"),
    E("variable", "🏋️", [VARI], "Variable action", "Veränderliche Einwirkung", slug="variable", group="actions-fields"),
    E("accidental", "💥", [ACCI], "Accidental action", "Außergewöhnliche Einwirkung", slug="accidental", group="actions-fields"),
    E("seismic", "🌋", [SEIS], "Seismic action", "Erdbebeneinwirkung", slug="seismic", group="actions-fields"),
    E("member", "🏗️", [MEMB], "Member", "Bauteil", slug="member", group="members"),
    E("bridge-sls", "🌉", [BRID], "Bridge serviceability", "Brückengebrauchstauglichkeit", slug="bridge", group="bridge-sls"),
    E("effect", "🔗", [EFFE], "Member effect", "Bauteilwirkung", slug="effect", group="effects-fatigue"),
    E("fatigue-spectrum", "🔄", [FATI], "Fatigue spectrum", "Ermüdungskollektiv", slug="fatigue", group="effects-fatigue"),
    E("reliability-analysis", "🎲", [RELI], "Reliability analysis", "Zuverlässigkeitsanalyse", slug="reliability", group="reliability-analyses"),
    E("test-series", "🧪", [TEST], "Test series", "Versuchsreihe", slug="test", group="test-series"),
    E("placement", "📍", [PLAC], "Placement", "Platzierung", slug="placement", group="placements"),
    E("factor-override", "🎚", [FACT], "Factor override", "Faktorüberschreibung", slug="factor-override", group="factor-overrides"),
    E("traffic-group", "🚦", [TRAF], "Traffic group", "Verkehrsgruppe", slug="traffic-group", group="traffic-groups"),
    E("design-basis", "🏛️", [], "Design basis", "Entwurfsgrundlage", slug="", group="design-basis"),
]
BY = {e.name: e for e in ENTS}
MEMBER_FAMILIES = {"identity": ["label_en", "label_de", "span", "precamber"], "resistance": ["rd_str", "rd_geo", "rd_equ_stab", "rd_equ_destab", "rd_fat"], "deflection": ["deflection_w1", "deflection_w2", "deflection_w3", "deflection_limit_ratio", "deflection_limit_ratio_w3"], "vibration": ["sls_limit_char", "sls_limit_freq", "sls_limit_qp", "vibration_frequency", "vibration_frequency_min"], "horizontal": ["frame_height", "frame_drift", "storey_height", "storey_drift"], "material": ["material_strength_k", "material_conversion_factor", "material_partial_factor", "material_design_strength"], "geometry": ["nominal_dimension", "dimension_tolerance", "design_dimension"], "analysis": ["member_alpha_cr", "second_order_analysis"]}
RAIL = ["track_gauge", "line_speed", "ballast_depth", "deck_mass", "natural_frequency", "end_rotation", "basic_wind_speed", "damping_ratio"]
TOP = ["structure_kind"]
ABBREV = [("bridge-bridge", "bridge"), ("vibration-frequency", "vib-freq"), ("deflection", "defl"), ("acceleration", "accel"), ("distribution", "dist"), ("conversion", "conv"), ("variation", "var"), ("limit-ratio", "ratio"), ("material", "mat"), ("justification", "justif")]
LIMIT = 35
ROWS = {"bridge-sls": "bridge-fatigue", "fatigue-spectrum": "bridge-fatigue", "reliability-analysis": "analysis", "test-series": "analysis", "placement": "analysis", "factor-override": "factor-overrides", "traffic-group": "traffic-groups"}
INLINE_ENUMS = {("TrafficGroup", "group"): "LoadGroup"}
RETIRE = ["change-permanents", "change-variables", "change-accidentals", "change-seismics", "change-members", "change-bridge-sls", "change-effects"]


def shorten(kind):
    kind = kind.replace("bridge-bridge", "bridge")
    for a, b in ABBREV:
        if len(kind) <= LIMIT:
            break
        kind = kind.replace(a, b)
    assert len(kind) <= LIMIT, kind
    return kind


def scalar(p):
    return {"number": "f64", "integer": "u32", "boolean": "bool", "string": "String"}[p["type"]]


def read_leaves(defs, ent):
    out = []
    for name, p in defs[ent.last.row]["properties"].items():
        if name == "id" or p.get("type") == "array" or (ent.name == "effect" and name not in ("memberId", "actionId", "influence")):
            continue
        ref = p.get("$ref", "").split("/")[-1] or INLINE_ENUMS.get((ent.last.row, name), "")
        if ref and "enum" in defs[ref]:
            leaf = I.Leaf(ent, [snake(name)], "enum:" + ref, p.get("x-semio-ui"), owner=ent.last.row, kind="x")
        else:
            leaf = I.Leaf(ent, [snake(name)], scalar(p), p.get("x-semio-ui"), owner=ent.last.row, kind="x")
        out.append(leaf)
    for l in out:
        l.kind = shorten("change-%s-%s" % (ent.slug, kebab(l.field)))
        l.set_names()
    return out


class EffectKind(I.SetKind):
    effect = True

    def diff_rs(self, st):
        l = self.leaf
        val = "payload.%s%s" % (l.new, "" if l.copy else ".clone()")
        guard = ""
        if l.ty == "f64":
            guard = '    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n' % l.new
        return I.doc_head(self.ent.emoji, "`%s` diff: replaces the addressed effect with the changed copy." % self.kind) + '''use crate::%(mod)s::%(rec)s;
use semio_s_artifact_norm_en1990_diff::En1990RowEdit as _;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990EffectEdit};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &%(rec)s, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
%(guard)s    let Some(row) = base.effects.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member effect {} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if row.%(f)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member effect already has this value.");
    }
    let mut next = row.clone();
    next.%(f)s = %(val)s;
    protocol::MutationOutcome::new(En1990Diff { effects: vec![En1990EffectEdit::replace(payload.index, String::new(), next)], ..Default::default() })
}
''' % dict(mod=self.module, rec=self.record, guard=guard, f=l.field, new=l.new, val=val)


def sample_row(defs, rowdef, rid, member):
    row = {"id": rid}
    for n, p in defs[rowdef]["properties"].items():
        if n == "id":
            continue
        row[n] = member if n == "memberId" else p["enum"][0] if "enum" in p else 1.0 if p.get("type") == "number" else 1 if p.get("type") == "integer" else "normal" if p.get("type") == "string" else None
    return row


def augment(doc, defs):
    member = doc["members"][0]["id"]
    for key, rowdef in (("bridgeSls", "BridgeSls"), ("fatigueSpectra", "FatigueSpectrum"), ("reliabilityAnalyses", "ReliabilityAnalysis"), ("testSeries", "TestSeries"), ("placements", "Placement"), ("factorOverrides", "FactorOverride"), ("trafficGroups", "TrafficGroup")):
        if not doc[key]:
            doc[key] = [sample_row(defs, rowdef, key[:3] + "-1", member)]
    return doc


def blank(d):
    return {k: ({"removed": [], "inserted": [], "moved": [], "modified": []} if isinstance(v, dict) else [] if isinstance(v, list) else None) for k, v in d.items()}


def rule_unused():
    return None


def main():
    F = K.KFiles(STAGE_A)
    schema = json.loads(rd(art.any + "/🧬️schema/📸️snapshot/🔣️.json"))
    defs = schema["$defs"]
    defs["LoadGroup"] = {"enum": defs["TrafficGroup"]["properties"]["group"]["enum"]}
    existing = K.kinds_of(art, F)
    dirs, fdirs = [], []
    for k in RETIRE:
        dirs += [os.path.basename(p) for p in glob.glob(art.mut + "/*" + k)]
        fdirs += [os.path.basename(p) for p in glob.glob(art.fix + "/*" + k)]
    if dirs:
        assert len(dirs) == len(RETIRE) and len(fdirs) == len(RETIRE), (dirs, fdirs)
        K.retire(art, F, RETIRE, dirs, fdirs)
        p = art.mut + "/🧪️tests/🔬️unit/🦀️.rs"
        F.write(p, K.drop_lines(F.read(p), r"^        En1990Mutation::Change(Permanents|Variables|Accidentals|Seismics|Members|BridgeSls|Effects)\(.*\n"))
        existing = [k for k in existing if k not in RETIRE]
    rules = rd(art.mut + "/🧭️edit-rules/🦀️.rs")
    covered = set(re.findall(r'SetFieldRule \{ path: "([^"]+)"', rules))
    leaves, new_kinds = [], []
    for e in ENTS:
        if not e.links:
            continue
        for l in read_leaves(defs, e):
            leaves.append(l)
            k = (EffectKind if e.name == "effect" else I.SetKind)(l)
            k.family = next((f for f, fs in MEMBER_FAMILIES.items() if l.field in fs), "") if e.name == "member" else ""
            if e.name == "member":
                assert k.family, l.field
            if e.name == "bridge-sls" and l.field in RAIL:
                k.family = "rail"
            new_kinds.append(k)
    for f in TOP:
        ty = "String"
        l = I.Leaf(BY["design-basis"], [f], ty, (defs and json.loads(rd(art.any + "/🧬️schema/📸️snapshot/🔣️.json"))["properties"][camel(f)].get("x-semio-ui")), kind="change-" + kebab(f))
        k = I.TopKind(l)
        k.family = "structure"
        new_kinds.append(k)
    for name in ("bridge-sls", "fatigue-spectrum", "reliability-analysis", "test-series", "placement", "factor-override", "traffic-group"):
        ins, rm = I.pair(BY[name], "insert-" + name, "remove-" + name, ST, by_index=True)
        for x in (ins, rm):
            x.group, x.family = ROWS[name], "rows"
        new_kinds += [ins, rm]
    new_kinds = [k for k in new_kinds if k.kind not in existing]
    assert len({k.kind for k in new_kinds}) == len(new_kinds), "duplicate kind names"
    tags = K.binary_protocol_keep(art, F, new_kinds, set(RETIRE))
    existing_crates = set(re.findall(r'"(semio-s-artifact-norm-en1990-mut-[a-z0-9-]+)"', rd(art.root + "/Cargo.toml")))
    groups = K.group_kinds(art, new_kinds, existing_crates)
    names = K.create_crates(art, F, groups)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    K.register(art, F, names)
    K.leaf_files(art, F, new_kinds, defs, tags)
    K.root_wire(art, F, new_kinds, groups_of)
    K.edit_rules(art, F, new_kinds)
    K.text_codec(art, F, new_kinds, groups_of)
    K.harness(art, F, new_kinds)
    K.oracle(art, F, new_kinds)
    K.taxonomy(art, F, new_kinds)
    spec = {pascal(e.last.single): ([camel(l.field) for l in leaves if l.ent is e], []) for e in ENTS if e.links}
    base = augment(json.loads(rd(art.fix + "/📎insert-effect/✅apply/🔣️.before.json")), defs)
    template = blank(json.loads(rd(art.fix + "/📎insert-effect/✅apply/🔣️.diff.json")))
    X.build(art, F, new_kinds, defs, base, spec, template, lambda k, coll, b: dict(copy.deepcopy(coll[0]), id=coll[0]["id"] + "-new"))
    print("leaves", len(leaves) + len(TOP), "new kinds", len(new_kinds), "retired", len(RETIRE), "crates", len(names), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply())


if __name__ == "__main__":
    shutil.rmtree(STAGE_A, ignore_errors=True)
    main()
