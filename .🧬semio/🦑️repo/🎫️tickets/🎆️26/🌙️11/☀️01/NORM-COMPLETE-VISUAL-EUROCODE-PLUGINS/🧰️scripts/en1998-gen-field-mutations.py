#!/usr/bin/env python3
"""Generates one `change-<entity>-<field>` mutation leaf per editable EN 1998 snapshot leaf (idempotent: skips kinds already present)."""
import json, os, re, copy, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🫨️en1998"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
EMOJI = "✏️"

def rd(p): return open(p, encoding="utf-8").read()
DRY = bool(os.environ.get("DRY"))
def wr(p, s):
    assert len(p[len('/home/deck/semio/'):].encode()) < 256, 'path of 256 bytes or more: ' + p
    if DRY: return
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)

def camel(snake):
    parts = snake.split("_")
    return parts[0] + "".join(x[:1].upper() + x[1:] for x in parts[1:])
def pascal(snake):
    return "".join(x[:1].upper() + x[1:] for x in snake.split("_"))
def kebab(snake): return snake.replace("_", "-")

# entity: json collection path, rust coll, group crate, selectors, patch types, base fixture, label
ENT = {
 "site":     dict(group="buildings", sel=[], path="site", fx="📏️change-storey-drift-xm", ent_label=("site", "Standort")),
 "building": dict(group="buildings", sel=[("building_index", "Building", "Gebäude", "building")], path="buildings[]", fx="📏️change-storey-drift-xm"),
 "system":   dict(group="buildings", sel=[("building_index", "Building", "Gebäude", "building"), ("system_index", "System", "Aussteifungssystem", "system")], path="buildings[].systems[]", fx="📏️change-storey-drift-xm", nest="systems", nestpatch="En1998SystemDelta, En1998SystemPatch", coll="systems"),
 "storey":   dict(group="buildings", sel=[("building_index", "Building", "Gebäude", "building"), ("storey_index", "Storey", "Geschoss", "storey")], path="buildings[].storeys[]", fx="📏️change-storey-drift-xm", nest="storeys", nestpatch="En1998StoreyDelta, En1998StoreyPatch", coll="storeys"),
 "member":   dict(group="members", sel=[("building_index", "Building", "Gebäude", "building"), ("member_index", "Member", "Bauteil", "member")], path="buildings[].members[]", fx="✅️change-member-detailing", nest="members", nestpatch="En1998MemberDelta, En1998MemberPatch", coll="members"),
 "bridge":   dict(group="bridges_towers", sel=[("index", "Bridge", "Brücke", "bridge")], path="bridges[]", fx="🛑️change-bridge-v-rd-n", top="bridges", T="Bridge"),
 "assessment": dict(group="members", sel=[("index", "Assessment", "Bewertung", "assessment")], path="assessments[]", fx="🏋️change-assessment-rkn", top="assessments", T="Assessment"),
 "tower":    dict(group="bridges_towers", sel=[("index", "Tower", "Turm", "tower")], path="towers[]", fx="↪️change-tower-m-rd-nm", top="towers", T="Tower"),
 "silo":     dict(group="silos_tanks", sel=[("index", "Silo", "Silo", "silo")], path="silos[]", fx="➖️remove-silo", top="silos", T="Silo"),
 "tank":     dict(group="silos_tanks", sel=[("index", "Tank", "Tank", "tank")], path="tanks[]", fx="➖️remove-tank", top="tanks", T="Tank"),
 "foundation": dict(group="foundations", sel=[("index", "Foundation", "Gründung", "foundation")], path="foundations[]", fx="➖️remove-foundation", top="foundations", T="Foundation"),
 "retaining-wall": dict(group="foundations", sel=[("index", "Retaining wall", "Stützwand", "retaining wall")], path="retainingWalls[]", fx="➖️remove-retaining-wall", top="retaining_walls", T="RetainingWall"),
}
VSEL = ("variable_index", "Variable action", "Veränderliche Einwirkung", "variable action")
ENT["storey-variable"] = dict(group="buildings", sel=[("building_index", "Building", "Gebäude", "building"), ("storey_index", "Storey", "Geschoss", "storey"), VSEL], path="buildings[].storeys[].variables[]", fx="📏️change-storey-drift-xm", var="storey", pk="storey")
ENT["bridge-variable"] = dict(group="bridges_towers", sel=[("bridge_index", "Bridge", "Brücke", "bridge"), VSEL], path="bridges[].variables[]", fx="🛑️change-bridge-v-rd-n", var="top", top="bridges", T="Bridge", pk="bridge")
ENT["tower-variable"] = dict(group="bridges_towers", sel=[("tower_index", "Tower", "Turm", "tower"), VSEL], path="towers[].variables[]", fx="↪️change-tower-m-rd-nm", var="top", top="towers", T="Tower", pk="tower")
ENT["soil-layer"] = dict(group="foundations", sel=[("index", "Soil layer", "Bodenschicht", "soil layer")], path="soilLayers[]", fx="➖️remove-foundation", top="soil_layers", T="SoilLayer", row=True, jsonkey="soilLayers")
ENT["diaphragm"] = dict(group="buildings", sel=[("index", "Diaphragm", "Scheibe", "diaphragm")], path="diaphragms[]", fx="📏️change-storey-drift-xm", top="diaphragms", T="Diaphragm", row=True, jsonkey="diaphragms")
ENT["pier"] = dict(group="bridges_towers", sel=[("index", "Pier", "Pfeiler", "pier")], path="piers[]", fx="🛑️change-bridge-v-rd-n", top="piers", T="Pier", row=True, jsonkey="piers")
ENT["bearing"] = dict(group="bridges_towers", sel=[("index", "Bearing", "Lager", "bearing")], path="bearings[]", fx="🛑️change-bridge-v-rd-n", top="bearings", T="Bearing", row=True, jsonkey="bearings")
ENT["joint"] = dict(group="members", sel=[("index", "Joint", "Knoten", "joint")], path="joints[]", fx="📏️change-storey-drift-xm", top="joints", T="Joint", row=True, jsonkey="joints")
ENT["accelerogram"] = dict(group="buildings", sel=[("index", "Accelerogram", "Akzelerogramm", "accelerogram")], path="accelerograms[]", fx="📏️change-storey-drift-xm", top="accelerograms", T="Accelerogram", row=True, jsonkey="accelerograms")
TOPS = {"pier": "piers", "bearing": "bearings", "joint": "joints", "bridge": "bridges", "assessment": "assessments", "tower": "towers", "silo": "silos", "tank": "tanks", "foundation": "foundations", "retaining-wall": "retainingWalls", "soil-layer": "soilLayers", "diaphragm": "diaphragms", "accelerogram": "accelerograms"}
VAR_LABEL = {"category": ("Variable action category", "Kategorie der veränderlichen Einwirkung"), "qk_n": ("Variable action Q_k", "Veränderliche Einwirkung Q_k")}
VARIABLE = dict(id="v-new", category="C", qk_n=250000.0)
GROUP_CRATE = {"buildings": "semio_s_artifact_norm_en1998_mut_buildings", "members": "semio_s_artifact_norm_en1998_mut_members", "bridges_towers": "semio_s_artifact_norm_en1998_mut_bridges_towers", "silos_tanks": "semio_s_artifact_norm_en1998_mut_silos_tanks", "foundations": "semio_s_artifact_norm_en1998_mut_foundations"}
GROUP_DIR = {"buildings": "🏢️buildings", "members": "🧱️members", "bridges_towers": "🗼️bridges-towers", "silos_tanks": "🏭️silos-tanks", "foundations": "🏗️foundations"}

# entity -> [(snake, type)] ; existing leaves skipped
FIELDS = {
 "site": [("s_ap_r","f64"),("a_gr","f64"),("de_ground_combo","DeGroundCombo"),("en_ground_type","String"),("en_spectrum_type","String"),("importance_class","String"),("v_s30_m_s","f64")],
 "building": [("name","String"),("plan_width_m","f64"),("plan_length_m","f64"),("t1_method","String"),("t1_given_s","f64"),("ct","f64"),("drift_limit_class","String"),("nu","f64"),("multiple_resisting_systems","bool"),("claims_simple_masonry","bool"),("accidental_eccentricity_ratio","f64"),("setback_ratio","f64"),("plan_envelope_gap_ratio","f64"),("joint_gap_m","f64"),("nonstructural_weight_n","f64"),("nonstructural_height_m","f64"),("nonstructural_period_s","f64"),("nonstructural_q","f64"),("nonstructural_resistance_n","f64"),("column_moment_sum_nm","f64"),("beam_moment_sum_nm","f64"),("overturning_base_x_m","f64"),("overturning_base_y_m","f64"),("tie_down_moment_nm","f64")],
 "system": [("direction","String"),("system_type","String"),("material","String"),("ductility_class","String"),("q0","f64"),("alpha_u_over_alpha_1","f64"),("k_w","f64")],
 "storey": [("height_m","f64"),("correlated_occupancy","bool"),("stiffness_y","f64"),("centre_of_mass_x_m","f64"),("centre_of_mass_y_m","f64"),("centre_of_stiffness_x_m","f64"),("centre_of_stiffness_y_m","f64"),("drift_y_m","f64"),("shear_resistance_n","f64")],
 "member": [("material","String"),("role","String"),("min_dimension_m","f64"),("rho","f64"),("rho_prime","f64"),("omega_wd","f64"),("steel_section_class","u8"),("fastener_diameter_mm","f64"),("timber_thickness_mm","f64"),("wall_thickness_m","f64"),("wall_effective_height_m","f64")],
 "bridge": [("period_ratio","f64"),("fundamental_period_s","f64"),("bearing_d_rd_m","f64"),("permanent_gk_n","f64"),("correlated_occupancy","bool"),("pier_height_m","f64"),("pier_moment_rd_nm","f64"),("pier_shear_rd_n","f64"),("deck_length_m","f64"),("deck_ei_nm2","f64")],
 "assessment": [("knowledge_level","String"),("limit_state","String"),("supported_building_id","String"),("gamma_el","f64"),("chord_rotation_demand_rad","f64"),("chord_rotation_yield_rad","f64"),("chord_rotation_ultimate_rad","f64")]+[(x,"f64") for x in ['section_depth_m', 'section_width_m', 'effective_depth_m', 'shear_span_m', 'axial_force_n', 'tension_steel_area_m2', 'compression_steel_area_m2', 'bar_diameter_mm', 'steel_yield_mpa', 'concrete_strength_mpa', 'stirrup_leg_area_m2', 'stirrup_spacing_m', 'stirrup_yield_mpa', 'core_width_m', 'core_depth_m', 'restrained_bar_count', 'restrained_bar_spacing_m', 'diagonal_reinforcement_ratio', 'shear_cracking_before_yield']],
 "tower": [("height_m","f64"),("is_chimney","bool"),("q_nominal","f64"),("permanent_gk_n","f64"),("correlated_occupancy","bool"),("ei_nm2","f64"),("base_rotational_stiffness_nm","f64")],
 "silo": [("height_m","f64"),("radius_m","f64"),("permanent_gk_n","f64"),("content_qk_n","f64"),("content_category","String"),("filling_ratio","f64"),("n_rd_n","f64"),("v_rd_n","f64"),("q_nominal","f64")],
 "tank": [("height_m","f64"),("radius_m","f64"),("permanent_gk_n","f64"),("content_qk_n","f64"),("content_category","String"),("filling_ratio","f64"),("v_rd_n","f64"),("shell_thickness_m","f64"),("shell_strength_mpa","f64")],
 "foundation": [("supported_building_id","String"),("area_m2","f64"),("p_rd_pa","f64"),("passive_resistance_n","f64"),("k_foundation","f64"),("k_soil","f64"),("liq_n160","f64"),("liq_fines_percent","f64"),("liq_depth_m","f64"),("liq_water_depth_m","f64"),("liq_unit_weight_n_m3","f64"),("liq_magnitude","f64")]+[(x,"f64") for x in ["liq_layer_thickness_m","footing_width_m","bearing_axial_n","bearing_shear_n","bearing_moment_nm","bearing_n_max_n","soil_friction_angle_deg","pile_diameter_m","pile_length_m","pile_modulus_pa","pile_subgrade_modulus_n_m3","pile_head_shear_n","pile_head_fixed","pile_interface_depth_m","pile_upper_vs_m_s","pile_lower_vs_m_s","pile_moment_rd_nm","base_friction_angle_deg","base_pore_pressure_pa"]],
 "soil-layer": [("foundation_id","String"),("thickness_m","f64"),("unit_weight_n_m3","f64"),("shear_wave_velocity_m_s","f64"),("n1_60","f64"),("fines_percent","f64")],
 "diaphragm": [("building_id","String"),("storey_id","String"),("material","String"),("thickness_m","f64"),("span_m","f64"),("depth_m","f64"),("opening_width_m","f64"),("shear_strength_mpa","f64"),("chord_area_m2","f64"),("chord_strength_mpa","f64"),("gamma_d","f64")],
 "pier": [("bridge_id","String"),("x_m","f64"),("height_m","f64"),("section_b_m","f64"),("section_h_m","f64"),("modulus_mpa","f64"),("moment_rd_nm","f64"),("shear_rd_n","f64")],
 "bearing": [("bridge_id","String"),("pier_id","String"),("device","String"),("x_m","f64"),("stiffness_n_per_m","f64"),("d_rd_m","f64"),("seat_length_m","f64"),("min_support_length_m","f64")],
 "joint": [("building_id","String"),("storey_id","String"),("column_id","String"),("beam_id","String"),("x_m","f64"),("y_m","f64"),("column_moment_rd_nm","f64"),("beam_moment_rd_nm","f64")],
 "accelerogram": [("building_id","String"),("label","String"),("samples","String"),("dt_s","f64"),("scale_factor","f64")],
 "storey-variable": [("category","String"),("qk_n","f64")],
 "bridge-variable": [("category","String"),("qk_n","f64")],
 "tower-variable": [("category","String"),("qk_n","f64")],
 "retaining-wall": [("height_m","f64"),("phi_deg","f64"),("soil_gamma","f64"),("r","f64"),("h_rd_n_per_m","f64"),("water_height_m","f64"),("saturated_gamma","f64"),("permeability_m_s","f64")],
}
STR_ALT = {"direction":"y","system_type":"wall","material":"steel","ductility_class":"dcm","t1_method":"rayleigh","drift_limit_class":"brittle","role":"beam","importance_class":"III","en_ground_type":"C","en_spectrum_type":"type2","content_category":"E","knowledge_level":"kl2","limit_state":"sd","name":"Renamed","category":"C","device":"sliding"}
ENUM_ALT = {"de_ground_combo":("DeGroundCombo::CS","C-S")}
NEWCRATES = {
 "buildings-envelope": ("🏢️", "buildings", [("site", "v_s30_m_s"), ("building", "setback_ratio"), ("building", "plan_envelope_gap_ratio"), ("building", "joint_gap_m"), ("building", "column_moment_sum_nm"), ("building", "beam_moment_sum_nm")]),
 "buildings-nonstructural": ("🏢️", "buildings", [("building", "nonstructural_" + x) for x in ["weight_n", "height_m", "period_s", "q", "resistance_n"]]),
 "members-wall": ("🧱️", "members", [("member", x) for x in ["fastener_diameter_mm", "timber_thickness_mm", "wall_thickness_m", "wall_effective_height_m"]] + [("assessment", "chord_rotation_" + x) for x in ["demand_rad", "yield_rad", "ultimate_rad"]]),
 "foundations-liq": ("🏗️", "foundations", [("foundation", "liq_" + x) for x in ["n160", "fines_percent", "depth_m", "water_depth_m", "unit_weight_n_m3", "magnitude"]]),
 "assess-section": ("🧱️", "members", [("assessment", x) for x in ["section_depth_m","section_width_m","effective_depth_m","shear_span_m","axial_force_n","diagonal_reinforcement_ratio","shear_cracking_before_yield"]]),
 "assess-steel": ("🧱️", "members", [("assessment", x) for x in ["tension_steel_area_m2","compression_steel_area_m2","bar_diameter_mm","steel_yield_mpa","concrete_strength_mpa"]]),
 "assess-confine": ("🧱️", "members", [("assessment", x) for x in ["stirrup_leg_area_m2","stirrup_spacing_m","stirrup_yield_mpa","core_width_m","core_depth_m","restrained_bar_count","restrained_bar_spacing_m"]]),
 "foundations-bearing": ("🏗️", "foundations", [("foundation", x) for x in ["liq_layer_thickness_m","footing_width_m","bearing_axial_n","bearing_shear_n","bearing_moment_nm","bearing_n_max_n","soil_friction_angle_deg"]]),
 "foundations-pile": ("🏗️", "foundations", [("foundation", x) for x in ["pile_diameter_m","pile_length_m","pile_modulus_pa","pile_subgrade_modulus_n_m3","pile_interface_depth_m"]]),
 "foundations-pile-head": ("🏗️", "foundations", [("foundation", x) for x in ["pile_head_shear_n","pile_head_fixed","pile_upper_vs_m_s","pile_lower_vs_m_s","pile_moment_rd_nm"]]),
 "foundations-soil": ("🏗️", "foundations", [("soil-layer", x) for x in ["foundation_id", "thickness_m", "unit_weight_n_m3", "shear_wave_velocity_m_s", "n1_60", "fines_percent", "insert", "remove"]]),
 "buildings-variables": ("🏢️", "buildings", [("storey-variable", x) for x in ["category", "qk_n", "insert", "remove"]]),
 "bridges-variables": ("🗼️", "bridges_towers", [("bridge-variable", x) for x in ["category", "qk_n", "insert", "remove"]]),
 "towers-variables": ("🗼️", "bridges_towers", [("tower-variable", x) for x in ["category", "qk_n", "insert", "remove"]]),
 "bridges-piers": ("🗼️", "bridges_towers", [("pier", x) for x in ["insert", "remove", "bridge_id", "x_m", "height_m", "section_b_m", "section_h_m", "modulus_mpa", "moment_rd_nm", "shear_rd_n"]]),
 "bridges-bearings": ("🗼️", "bridges_towers", [("bearing", x) for x in ["insert", "remove", "bridge_id", "pier_id", "device", "x_m", "stiffness_n_per_m", "d_rd_m", "seat_length_m", "min_support_length_m"]]),
 "members-joints": ("🔩️", "members", [("joint", x) for x in ["insert", "remove", "building_id", "storey_id", "column_id", "beam_id", "x_m", "y_m", "column_moment_rd_nm", "beam_moment_rd_nm"]]),
 "silos-tanks-shell": ("🏭️", "silos_tanks", [("tank", "shell_thickness_m"), ("tank", "shell_strength_mpa")]),
 "towers-foundation": ("🗼️", "bridges_towers", [("tower", "base_rotational_stiffness_nm")]),
 "bridges-deck": ("🗼️", "bridges_towers", [("bridge", "deck_length_m"), ("bridge", "deck_ei_nm2")]),
 "bridges-pier": ("🗼️", "bridges_towers", [("bridge", "pier_height_m"), ("bridge", "pier_moment_rd_nm"), ("bridge", "pier_shear_rd_n"), ("tower", "ei_nm2")]),
 "diaphragms-core": ("🧱️", "buildings", [("diaphragm", x) for x in ["insert", "remove", "building_id", "storey_id", "material", "thickness_m"]]),
 "diaphragms-resistance": ("🧱️", "buildings", [("diaphragm", x) for x in ["span_m", "depth_m", "opening_width_m", "shear_strength_mpa", "chord_area_m2", "chord_strength_mpa", "gamma_d"]]),
 "accelerograms": ("📈️", "buildings", [("accelerogram", x) for x in ["insert", "remove", "building_id", "label", "samples", "dt_s", "scale_factor"]]),
 "foundations-sliding": ("🏗️", "foundations", [("foundation", x) for x in ["base_friction_angle_deg", "base_pore_pressure_pa"]]),
 "foundations-water": ("🏗️", "foundations", [("retaining-wall", x) for x in ["water_height_m", "saturated_gamma", "permeability_m_s"]]),
 "buildings-overturning": ("🏢️", "buildings", [("building", x) for x in ["overturning_base_x_m", "overturning_base_y_m", "tie_down_moment_nm"]]),
}
CRATE_OF = {leaf: name for name, (_, _, ls) in NEWCRATES.items() for leaf in ls}
ENT_WORD = {"retaining-wall": "retaining-wall"}

# field-meta labels
fm = rd(ANY + "/🏷️field-meta/🦀️.rs")
LABEL = {}
for m in re.finditer(r'\("([^"]+)",\s*m!\("([^"]*)",\s*"([^"]*)"', fm):
    LABEL[m.group(1)] = (m.group(2), m.group(3))

def jpath(ent, f):
    return ENT[ent]["path"] + "." + camel(f) if ENT[ent]["path"] != "site" else "site." + camel(f)

def label_for(ent, f):
    p = jpath(ent, f)
    if p in LABEL: return LABEL[p]
    if ENT[ent].get("var") and f in VAR_LABEL: return VAR_LABEL[f]
    return (f.replace("_", " "), f.replace("_", " "))

def json_type_schema(t):
    if t == "f64": return {"type": "number"}
    if t == "bool": return {"type": "boolean"}
    if t == "u8": return {"type": "integer", "minimum": 0, "maximum": 255}
    return {"type": "string"}

def cl(t): return ".clone()" if t not in ("f64", "bool", "u8") else ""

fixtures_cache = {}
def load_fixture(fx):
    base = FIX + "/" + fx + "/✅apply"
    return json.loads(rd(base + "/🔣️.before.json")), base

def dumps(o): return json.dumps(o, indent=2, ensure_ascii=False) + "\n"

KIND_ABBREV = [("nonstructural", "ns"), ("plan-envelope", "plan-env"), ("column-moment", "col-moment"), ("chord-rotation", "chord"), ("ultimate", "ult"), ("effective", "eff"), ("unit-weight", "unit-wt"), ("shear-wave-velocity", "vs"), ("soil-layer-unit-wt", "soil-unit-wt"), ("compression-steel", "as2"), ("tension-steel", "as1"), ("diagonal-reinforcement", "diag-reinf"), ("shear-cracking-before-yield", "shear-crack"), ("concrete-strength", "fc"), ("stirrup", "strp"), ("restrained-bar", "rbar"), ("soil-friction-angle", "soil-phi"), ("pile-subgrade-modulus", "pile-ks"), ("pile-interface-depth", "pile-iface"), ("pile-upper", "pile-up"), ("pile-lower", "pile-low"), ("bearing-moment", "bear-moment"), ("bearing-axial", "bear-axial"), ("bearing-shear", "bear-shear"), ("liq-layer-thickness", "liq-layer"), ("overturning-base", "ot-base"), ("tie-down-moment", "tiedown"), ("shear-strength", "shear-str"), ("chord-strength", "chord-str"), ("base-rotational-stiffness", "base-rot-stiff"), ("min-support-length", "min-support"), ("shell-thickness", "shell-t"), ("shell-strength", "shell-str")]
records = []   # collected leaves
for ent, flist in FIELDS.items():
    for snake, ty in flist:
        e = ENT[ent]
        kind = "change-%s-%s" % (ent, kebab(snake))
        for a, b in KIND_ABBREV: kind = kind.replace(a, b)
        module = kind.replace("-", "_")
        record = pascal(kind.replace("-", "_"))
        records.append(dict(ent=ent, snake=snake, ty=ty, kind=kind, module=module, record=record, dirname=EMOJI + kind))

LABELS = {"insert": ("Insert {w}", "{d} einfügen", "➕️"), "remove": ("Remove {w}", "{d} entfernen", "➖️")}
for ent in ("storey-variable", "bridge-variable", "tower-variable"):
    for op in ("insert", "remove"):
        kind = "%s-%s" % (op, ent)
        records.append(dict(ent=ent, snake=op, ty="op", op=op, kind=kind, module=kind.replace("-", "_"), record=pascal(kind.replace("-", "_")), dirname=LABELS[op][2] + kind))

def op_sel(r): return ENT[r["ent"]]["sel"][:-1] if r.get("op") == "insert" else ENT[r["ent"]]["sel"]

def wire_of(r):
    if r.get("row"): return row_wire(r)
    if r.get("op") == "insert": return [s[0] for s in op_sel(r)] + ["index", "variable"]
    if r.get("op") == "remove": return [s[0] for s in op_sel(r)]
    return [s[0] for s in ENT[r["ent"]]["sel"]] + ["new_" + r["snake"]]

VMODEL = "semio_s_artifact_norm_en1998_model::En1998VariableAction"
def parent_chain(e):
    if e["var"] == "storey": return "base.buildings.get(payload.building_index).and_then(|b| b.storeys.get(payload.storey_index))"
    return "base.%s.get(payload.%s)" % (e["top"], e["sel"][0][0])

def var_diff_rs(r):
    e = ENT[r["ent"]]; rec = r["record"]; mod = r["module"]; op = r.get("op"); f = r["snake"]
    head = "//! %s `%s` diff — patches the `variables` list of the addressed %s; a missing target is a `mutation.target-missing`.\n\n" % (LABELS[op][2] if op else EMOJI, r["kind"], e["sel"][-2][3])
    if e["var"] == "storey":
        head += "use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998StoreyDelta, En1998StoreyPatch};\n"
        pre = """    let Some(building) = base.buildings.get(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new());
    };
    let Some(row) = building.storeys.get(payload.storey_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new());
    };
"""
        tail = """    let nested = En1998StoreyDelta::modification(&row.id, En1998StoreyPatch { variables: Some(variables), ..Default::default() });
    let patch = En1998BuildingPatch { storeys: nested, ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&building.id, patch), ..Default::default() })
"""
    else:
        T = e["T"]
        head += "use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998%sDelta, En1998%sPatch};\n" % (T, T)
        pre = """    let Some(row) = base.%s.get(payload.%s) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "%s", Vec::<String>::new());
    };
""" % (e["top"], e["sel"][0][0], e["sel"][0][3])
        tail = """    let patch = En1998%sPatch { variables: Some(variables), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { %s: En1998%sDelta::modification(&row.id, patch), ..Default::default() })
""" % (T, e["top"], T)
    head += "use semio_s_artifact_norm_en1998_model::En1998Snapshot;\nuse crate::%s::%s;\n\npub fn diff(payload: &%s, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {\n" % (mod, rec, rec)
    if op == "insert":
        mid = """    if row.variables.iter().any(|existing| existing.id == payload.variable.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Variable action id {} already exists.", payload.variable.id), [payload.variable.id.clone()]);
    }
    let mut variables = row.variables.clone();
    let index = payload.index.unwrap_or(usize::MAX).min(variables.len());
    variables.insert(index, payload.variable.clone());
"""
    elif op == "remove":
        mid = """    if payload.variable_index >= row.variables.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("variable #{}", payload.variable_index), [payload.variable_index.to_string()]);
    }
    let mut variables = row.variables.clone();
    variables.remove(payload.variable_index);
"""
    else:
        mid = """    let mut variables = row.variables.clone();
    let Some(variable) = variables.get_mut(payload.variable_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "variable", Vec::<String>::new());
    };
    variable.%s = payload.new_%s%s;
""" % (f, f, cl(r["ty"]))
    return head + pre + mid + tail + "}\n"

def var_inverse_rs(r):
    e = ENT[r["ent"]]; rec = r["record"]; mod = r["module"]; op = r.get("op"); f = r["snake"]
    sels = [s[0] for s in e["sel"]]; parent = sels[:-1]
    pc = parent_chain(e)
    fields = "".join("%s: payload.%s, " % (x, x) for x in parent)
    if op == "insert":
        other = "remove_" + r["ent"].replace("-", "_"); orec = pascal(other)
        body = "    let len = %s.map(|p| p.variables.len()).unwrap_or(0);\n    Ok(vec![Op::from(%s::%s { %svariable_index: payload.index.unwrap_or(usize::MAX).min(len) })])\n" % (pc, other, orec, fields)
    elif op == "remove":
        other = "insert_" + r["ent"].replace("-", "_"); orec = pascal(other)
        body = "    Ok(match %s.and_then(|p| p.variables.get(payload.variable_index)) {\n        Some(item) => vec![Op::from(%s::%s { %sindex: Some(payload.variable_index), variable: item.clone() })],\n        None => Vec::new(),\n    })\n" % (pc, other, orec, fields)
    else:
        other = mod; orec = rec
        body = "    Ok(match %s.and_then(|p| p.variables.get(payload.variable_index)) {\n        Some(item) => vec![Op::from(%s { %svariable_index: payload.variable_index, new_%s: item.%s%s })],\n        None => Vec::new(),\n    })\n" % (pc, rec, fields, f, f, cl(r["ty"]))
    use = "use crate::%s;\n" % other if op else ""
    return "//! ↩️ Inverse for `%s`.\nuse crate::%s::%s;\nuse semio_s_artifact_norm_en1998_model::En1998Snapshot;\n%s\npub fn inverse<Op>(payload: &%s, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%s::%s> {\n%s}\n" % (r["kind"], mod, rec, use, rec, other, orec, body)

DE_WORD = {"storey-variable": "Geschoss-Nutzlast", "bridge-variable": "Brücken-Nutzlast", "tower-variable": "Turm-Nutzlast"}
OP_TEMPLATE = """//! %(emoji)s `%(kind)s` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct %(rec)s {
%(fields)s}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for %(rec)s where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::%(other)s::%(orec)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "%(op)s",
        entity: "%(ent)s",
        kind: "%(kind)s",
        record: "%(rec)s",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("%(en)s", "%(de)s")
    }
    fn target(&self) -> Vec<String> {
        vec![%(target)s]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for %(rec)s where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::%(other)s::%(orec)s> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::%(mod)s::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::%(mod)s::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
"""

def op_leaf_rs(r):
    op = r["op"]
    other = ("remove_" if op == "insert" else "insert_") + r["ent"].replace("-", "_")
    fields = "".join("    pub %s: usize,\n" % s[0] for s in op_sel(r))
    if op == "insert": fields += "    pub index: Option<usize>,\n    pub variable: %s,\n" % VMODEL
    return OP_TEMPLATE % dict(emoji=LABELS[op][2], kind=r["kind"], rec=r["record"], fields=fields, other=other, orec=pascal(other), op=op, ent=r["ent"], en=LABELS[op][0].format(w=r["ent"].replace("-", " ")), de=LABELS[op][1].format(d=DE_WORD[r["ent"]]), target='"%s".into()' % r["kind"] if op == "insert" else "self.variable_index.to_string()", mod=r["module"])

def op_schema_json(r):
    op = r["op"]; props = {"mutation": {"const": r["record"][:1].lower() + r["record"][1:]}}; req = ["mutation"]; order = 10
    for (name, lab_en, lab_de, what) in op_sel(r):
        props[camel(name)] = {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": lab_en, "de": lab_de}, "description": {"en": "Zero-based position of the %s in its list." % what, "de": "Nullbasierte Position in der Liste (%s)." % lab_de}, "step": 1, "precision": 0, "group": "target", "order": order}}
        req.append(camel(name)); order += 10
    if op == "insert":
        props["index"] = {"type": ["integer", "null"], "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Position", "de": "Position"}, "description": {"en": "Zero-based list position at which the record is inserted. Empty appends; positions past the end append.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird. Ohne Angabe am Ende; Positionen hinter dem Ende hängen an."}, "step": 1, "precision": 0, "group": "target", "order": order}}
        props["variable"] = {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1998/snapshot.json#/$defs/En1998VariableAction", "x-semio-ui": {"role": "value", "label": {"en": "Variable action", "de": "Veränderliche Einwirkung"}, "description": {"en": "The complete variable action Q_k,i to insert.", "de": "Die vollständige einzufügende veränderliche Einwirkung Q_k,i."}, "group": "record", "order": order + 10}}
        req.append("variable")
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/%s/schema.json" % r["kind"], "title": r["record"], "type": "object", "additionalProperties": False, "required": req, "properties": props}

def op_schema_ts(r):
    op = r["op"]; rec = r["record"]; sel = op_sel(r); lit = rec[:1].lower() + rec[1:]
    imports = {"normWireInteger", "normWireLiteral", "normWireObject", "normWireRange", "normWireRequired"}
    members = '  mutation: "%s";\n' % lit + "".join("  %s: number;\n" % camel(s[0]) for s in sel)
    rdr = 'mutation: normWireRequired(normWireLiteral("%s")), ' % lit + "".join('%s: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), ' % camel(s[0]) for s in sel)
    extra = ""
    if op == "insert":
        imports |= {"normWireNullable", "normWireOptional"}
        members += "  index?: number | null;\n  variable: En1998VariableAction;\n"
        rdr += 'index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), variable: normWireRequired(parseEn1998VariableAction), '
        extra = 'import { type En1998VariableAction, parseEn1998VariableAction } from "../../../📸️snapshot/🟦️.ts";\n'
    imp = ", ".join(sorted(imports | {"type NormWireReader"}))
    return "/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`\n * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.\n * @see ./🔣️.json */\nimport { %s } from \"@semio-tech/norm-js/registry/contract\";\n%s\nexport interface %s {\n%s}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });\n" % (LABELS[op][2], r["kind"], rec, imp, extra, rec, members, rec, rec, rec, rdr.rstrip(", ").rstrip() + (" " if False else ""))

def var_fixture(r, tag):
    ent = r["ent"]; e = ENT[ent]; op = r.get("op"); f = camel(r["snake"])
    snap, base = load_fixture(e["fx"])
    getp = (lambda s: s["buildings"][0]["storeys"][0]) if e["var"] == "storey" else (lambda s: s[e["top"]][0])
    after = copy.deepcopy(snap); vs = getp(after)["variables"]
    mut = {"mutation": r["record"][:1].lower() + r["record"][1:]}
    sels = [s[0] for s in e["sel"]]
    if op == "insert":
        for x in sels[:-1]: mut[camel(x)] = 0
        nv = {"id": VARIABLE["id"], "category": VARIABLE["category"], "qkN": VARIABLE["qk_n"]}
        mut["index"] = 1; mut["variable"] = nv; vs.insert(1, nv)
        r["sample"] = "%s, index: Some(1usize), variable: %s { id: String::from(\"%s\"), category: String::from(\"%s\"), qk_n: %r }" % (", ".join("%s: 0usize" % x for x in sels[:-1]), VMODEL, nv["id"], nv["category"], nv["qkN"])
    elif op == "remove":
        for x in sels: mut[camel(x)] = 0
        vs.pop(0)
        r["sample"] = ", ".join("%s: 0usize" % x for x in sels)
    else:
        for x in sels: mut[camel(x)] = 0
        old = vs[0][f]; newv = new_value(ent, r["snake"], r["ty"], old); r["_new"] = newv
        vs[0][f] = newv; mut["new" + pascal(r["snake"])] = newv
    diff = json.loads(rd(base + "/🔣️.diff.json"))
    for k, v in list(diff.items()):
        if isinstance(v, dict) and "modified" in v: v["modified"] = []; v["removed"] = []; v["inserted"] = []; v["moved"] = []
    np_ = {k: None for k in PATCHKEYS[e["pk"]]}; np_["variables"] = vs
    if e["var"] == "storey":
        b = after["buildings"][0]; p = bpatch()
        p["storeys"] = {"removed": [], "inserted": [], "moved": [], "modified": [{"id": b["storeys"][0]["id"], "patch": np_}]}
        diff["buildings"]["modified"] = [{"id": b["id"], "patch": p}]
    else:
        diff[e["top"]] = {"removed": [], "inserted": [], "moved": [], "modified": [{"id": after[e["top"]][0]["id"], "patch": np_}]}
    return snap, mut, after, diff, base

ROWS = {
 "soil-layer": dict(P="SoilLayer", camel="soilLayers", item="layer", en="soil layer", de="Bodenschicht", first=dict(id="sl-1", foundationId="fd-1", thicknessM=2.0, unitWeightNM3=18000.0, shearWaveVelocityMS=180.0, n160=12.0, finesPercent=10.0), second=dict(id="sl-2", foundationId="fd-1", thicknessM=3.0, unitWeightNM3=18000.0, shearWaveVelocityMS=180.0, n160=12.0, finesPercent=10.0), order=["foundation_id","thickness_m","unit_weight_n_m3","shear_wave_velocity_m_s","n1_60","fines_percent"], keys=dict(n1_60="n160")),
 "diaphragm": dict(P="Diaphragm", camel="diaphragms", item="diaphragm", en="diaphragm", de="Scheibe", first=dict(id="dia-1", buildingId="bldg-office", storeyId="s1", material="concrete", thicknessM=0.2, spanM=0.0, depthM=0.0, openingWidthM=2.4, shearStrengthMpa=1.0, chordAreaM2=0.01, chordStrengthMpa=435.0, gammaD=0.0), second=dict(id="dia-2", buildingId="bldg-office", storeyId="s2", material="concrete", thicknessM=0.2, spanM=0.0, depthM=0.0, openingWidthM=2.4, shearStrengthMpa=1.0, chordAreaM2=0.01, chordStrengthMpa=435.0, gammaD=0.0), order=["building_id","storey_id","material","thickness_m","span_m","depth_m","opening_width_m","shear_strength_mpa","chord_area_m2","chord_strength_mpa","gamma_d"], keys={}),
 "pier": dict(P="Pier", camel="piers", item="pier", en="pier", de="Pfeiler", first=dict(id="pier-1", bridgeId="br-1", xM=8.0, heightM=8.0, sectionBM=1.2, sectionHM=1.2, modulusMpa=30000.0, momentRdNm=6.0e6, shearRdN=1.0e6), second=dict(id="pier-2", bridgeId="br-1", xM=22.0, heightM=8.0, sectionBM=1.2, sectionHM=1.2, modulusMpa=30000.0, momentRdNm=6.0e6, shearRdN=1.0e6), order=["bridge_id","x_m","height_m","section_b_m","section_h_m","modulus_mpa","moment_rd_nm","shear_rd_n"], keys={}),
 "bearing": dict(P="Bearing", camel="bearings", item="bearing", en="bearing", de="Lager", first=dict(id="bear-1", bridgeId="br-1", pierId="pier-1", device="elastomeric", xM=0.0, stiffnessNPerM=2.0e7, dRdM=0.2, seatLengthM=0.6, minSupportLengthM=0.4), second=dict(id="bear-2", bridgeId="br-1", pierId="pier-2", device="elastomeric", xM=0.0, stiffnessNPerM=2.0e7, dRdM=0.2, seatLengthM=0.6, minSupportLengthM=0.4), order=["bridge_id","pier_id","device","x_m","stiffness_n_per_m","d_rd_m","seat_length_m","min_support_length_m"], keys={}),
 "joint": dict(P="Joint", camel="joints", item="joint", en="joint", de="Knoten", first=dict(id="joint-1", buildingId="bldg-office", storeyId="s1", columnId="c1", beamId="b1", xM=4.0, yM=4.0, columnMomentRdNm=9.0e6, beamMomentRdNm=4.0e6), second=dict(id="joint-2", buildingId="bldg-office", storeyId="s1", columnId="c1", beamId="b1", xM=8.0, yM=4.0, columnMomentRdNm=9.0e6, beamMomentRdNm=4.0e6), order=["building_id","storey_id","column_id","beam_id","x_m","y_m","column_moment_rd_nm","beam_moment_rd_nm"], keys={}),
 "accelerogram": dict(P="Accelerogram", camel="accelerograms", item="accelerogram", en="accelerogram", de="Akzelerogramm", first=dict(id="acc-1", buildingId="bldg-office", label="Record A", samples="0 0.5 -0.25 1.75", dtS=0.02, scaleFactor=1.0), second=dict(id="acc-2", buildingId="bldg-office", label="Record B", samples="0 0.5 -0.25 1.75", dtS=0.02, scaleFactor=1.0), order=["building_id","label","samples","dt_s","scale_factor"], keys={}),
}
for ent in ROWS:
    for op in ("insert", "remove"):
        kind = "%s-%s" % (op, ent)
        records.append(dict(ent=ent, snake=op, ty="op", op=op, row=True, kind=kind, module=kind.replace("-", "_"), record=pascal(kind.replace("-", "_")), dirname=LABELS[op][2] + kind))

def row_literal(ent, row):
    R = ROWS[ent]
    fields = ['id: String::from("%s")' % row["id"]]
    for snake in R["order"]:
        v = row[R["keys"].get(snake, camel(snake))]
        fields.append("%s: %s" % (snake, 'String::from("%s")' % v if isinstance(v, str) else repr(float(v))))
    return "semio_s_artifact_norm_en1998_model::En1998%s { %s }" % (R["P"], ", ".join(fields))

def row_wire(r):
    return ["index", ROWS[r["ent"]]["item"]] if r["op"] == "insert" else ["index"]

def row_leaf_rs(r):
    e = ENT[r["ent"]]; op = r["op"]; ent = r["ent"]; R = ROWS[ent]
    other = ("remove_" if op == "insert" else "insert_") + ent.replace("-", "_")
    fields = "    pub index: Option<usize>,\n    pub %s: semio_s_artifact_norm_en1998_model::En1998%s,\n" % (R["item"], R["P"]) if op == "insert" else "    pub index: usize,\n"
    return OP_TEMPLATE % dict(emoji=LABELS[op][2], kind=r["kind"], rec=r["record"], fields=fields, other=other, orec=pascal(other), op=op, ent=ent, en=LABELS[op][0].format(w=R["en"]), de=LABELS[op][1].format(d=R["de"]), target='"%s".into()' % r["kind"] if op == "insert" else "self.index.to_string()", mod=r["module"])

def row_diff_rs(r):
    rec = r["record"]; mod = r["module"]; R = ROWS[r["ent"]]; snake = ENT[r["ent"]]["top"]; P = R["P"]; item = R["item"]
    if r["op"] == "insert":
        return """//! %s `%s` diff — inserts the row at its position, clamped to the end of the collection; an id the document already holds is a `mutation.duplicate-id`.

use crate::%s::%s;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998%sDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &%s, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.%s.iter().any(|existing| existing.id == payload.%s.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%s id {} already exists.", payload.%s.id), [payload.%s.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.%s.len());
    protocol::MutationOutcome::new(En1998Diff { %s: En1998%sDelta::insertion(index, payload.%s.clone()), ..Default::default() })
}
""" % (LABELS["insert"][2], r["kind"], mod, rec, P, rec, snake, item, R["en"].capitalize(), item, item, snake, snake, P, item)
    return """//! %s `%s` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use crate::%s::%s;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998%sDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &%s, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.%s.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("%s #{}", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1998Diff { %s: En1998%sDelta::removal(&base.%s, payload.index), ..Default::default() })
}
""" % (LABELS["remove"][2], r["kind"], mod, rec, P, rec, snake, R["en"], snake, P, snake)

def row_inverse_rs(r):
    rec = r["record"]; mod = r["module"]; ent = r["ent"].replace("-", "_"); R = ROWS[r["ent"]]; snake = ENT[r["ent"]]["top"]; item = R["item"]
    if r["op"] == "insert":
        other = "remove_" + ent; orec = pascal(other)
        body = "    Ok(vec![Op::from(%s::%s { index: payload.index.unwrap_or(usize::MAX).min(base.%s.len()) })])\n" % (other, orec, snake)
    else:
        other = "insert_" + ent; orec = pascal(other)
        body = "    Ok(match base.%s.get(payload.index) {\n        Some(item) => vec![Op::from(%s::%s { index: Some(payload.index), %s: item.clone() })],\n        None => Vec::new(),\n    })\n" % (snake, other, orec, item)
    return "//! ↩️ Inverse for `%s`.\nuse crate::%s::%s;\nuse semio_s_artifact_norm_en1998_model::En1998Snapshot;\nuse crate::%s;\n\npub fn inverse<Op>(payload: &%s, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%s::%s> {\n%s}\n" % (r["kind"], mod, rec, other, rec, other, orec, body)

def row_schema_json(r):
    R = ROWS[r["ent"]]
    props = {"mutation": {"const": r["record"][:1].lower() + r["record"][1:]}}; req = ["mutation"]
    if r["op"] == "insert":
        props["index"] = {"type": ["integer", "null"], "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Position", "de": "Position"}, "description": {"en": "Zero-based list position at which the record is inserted. Empty appends; positions past the end append.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird. Ohne Angabe am Ende; Positionen hinter dem Ende hängen an."}, "step": 1, "precision": 0, "group": "target", "order": 10}}
        props[R["item"]] = {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1998/snapshot.json#/$defs/En1998%s" % R["P"], "x-semio-ui": {"role": "value", "label": {"en": R["en"].capitalize(), "de": R["de"]}, "description": {"en": "The complete %s record to insert." % R["en"], "de": "Der vollständige einzufügende Datensatz (%s)." % R["de"]}, "group": "record", "order": 20}}
        req.append(R["item"])
    else:
        props["index"] = {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": R["en"].capitalize(), "de": R["de"]}, "description": {"en": "Zero-based position of the %s in its list." % R["en"], "de": "Nullbasierte Position in der Liste (%s)." % R["de"]}, "step": 1, "precision": 0, "group": "target", "order": 10}}
        req.append("index")
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/%s/schema.json" % r["kind"], "title": r["record"], "type": "object", "additionalProperties": False, "required": req, "properties": props}

def row_schema_ts(r):
    rec = r["record"]; lit = rec[:1].lower() + rec[1:]; R = ROWS[r["ent"]]; P = "En1998" + R["P"]
    if r["op"] == "insert":
        imp = "normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired"
        extra = 'import { type %s, parse%s } from "../../../📸️snapshot/🟦️.ts";\n' % (P, P)
        members = '  mutation: "%s";\n  index?: number | null;\n  %s: %s;\n' % (lit, R["item"], P)
        rdr = 'mutation: normWireRequired(normWireLiteral("%s")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), %s: normWireRequired(parse%s)' % (lit, R["item"], P)
    else:
        imp = "normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired"
        extra = ""
        members = '  mutation: "%s";\n  index: number;\n' % lit
        rdr = 'mutation: normWireRequired(normWireLiteral("%s")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0}))' % lit
    return "/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`\n * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.\n * @see ./🔣️.json */\nimport { %s } from \"@semio-tech/norm-js/registry/contract\";\n%s\nexport interface %s {\n%s}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });\n" % (LABELS[r["op"]][2], r["kind"], rec, imp, extra, rec, members, rec, rec, rec, rdr)

def row_fixture(r, tag):
    ent = r["ent"]; R = ROWS[ent]; key = R["camel"]
    snap, base = load_fixture(ENT[ent]["fx"])
    snap[key] = [copy.deepcopy(R["first"])]
    after = copy.deepcopy(snap)
    mut = {"mutation": r["record"][:1].lower() + r["record"][1:]}
    if r["op"] == "insert":
        after[key].append(copy.deepcopy(R["second"])); mut["index"] = 1; mut[R["item"]] = copy.deepcopy(R["second"])
        r["sample"] = "index: Some(1usize), %s: %s" % (R["item"], row_literal(ent, R["second"]))
    else:
        after[key] = []; mut["index"] = 0
        r["sample"] = "index: 0usize"
    diff = row_diff(base, r)
    return snap, mut, after, diff, base

def row_diff(base, r):
    R = ROWS[r["ent"]]; key = R["camel"]
    d = json.loads(rd(base + "/🔣️.diff.json"))
    for k, v in list(d.items()):
        if isinstance(v, dict) and "modified" in v: v["modified"] = []; v["removed"] = []; v["inserted"] = []; v["moved"] = []
    if r["op"] == "insert": d[key]["inserted"] = [{"index": 1, "row": copy.deepcopy(R["second"])}]
    else: d[key]["removed"] = [{"id": R["first"]["id"], "index": 0}]
    return d

def leaf_rs(r):
    if r.get("row"): return row_leaf_rs(r)
    if r.get("op"): return op_leaf_rs(r)
    return set_leaf_rs(r)

def set_leaf_rs(r):
    e = ENT[r["ent"]]; ty = r["ty"]
    en, de = label_for(r["ent"], r["snake"])
    sel = e["sel"]
    fields = "".join("    pub %s: usize,\n" % s[0] for s in sel) + "    pub new_%s: %s,\n" % (r["snake"], "semio_s_artifact_norm_en1998_model::" + ty if ty.startswith("De") else ty)
    return '''//! %s `%s` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct %s {
%s}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for %s where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::%s::%s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "%s-%s",
        kind: "%s",
        record: "%s",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change %s", "%s ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["%s".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for %s where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::%s::%s> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::%s::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::%s::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
''' % (EMOJI, r["kind"], r["record"], fields, r["record"], r["module"], r["record"], r["ent"], kebab(r["snake"]), r["kind"], r["record"], en.replace('"', "'"), de.replace('"', "'"), r["kind"], r["record"], r["module"], r["record"], r["module"], r["module"])

def getters(r):
    """(let-bindings to find the row, row variable access expr list)"""
    ent = r["ent"]; e = ENT[ent]
    if ent == "site": return None
    return e

def diff_rs(r):
    if r.get("row"): return row_diff_rs(r)
    if ENT[r["ent"]].get("var"): return var_diff_rs(r)
    ent = r["ent"]; e = ENT[ent]; f = r["snake"]; ty = r["ty"]; rec = r["record"]; mod = r["module"]
    val = "payload.new_%s%s" % (f, cl(ty))
    head = "//! %s `%s` diff — patches the one field of the addressed row; a missing target is a `mutation.target-missing`.\n\nuse crate::%s::%s;\n" % (EMOJI, r["kind"], mod, rec)
    if ent == "site":
        return head + '''use semio_s_artifact_norm_en1998_diff::En1998Diff;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &%s, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut site = base.site.clone();
    site.%s = %s;
    if site == base.site {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The site already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { site: Some(site), ..Default::default() })
}
''' % (rec, f, val)
    if ent == "building":
        return head + '''use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &%s, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(building) = base.buildings.get(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new());
    };
    let patch = En1998BuildingPatch { %s: Some(%s), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&building.id, patch), ..Default::default() })
}
''' % (rec, f, val)
    if "nest" in e:
        T = e["nestpatch"].split(", ")
        delta, patch = T
        idx = [s[0] for s in e["sel"]][1]
        label = e["sel"][1][3]
        return head + '''use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, %s, %s};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &%s, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(building) = base.buildings.get(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new());
    };
    let Some(row) = building.%s.get(payload.%s) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "%s", Vec::<String>::new());
    };
    let nested = %s::modification(&row.id, %s { %s: Some(%s), ..Default::default() });
    let patch = En1998BuildingPatch { %s: nested, ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&building.id, patch), ..Default::default() })
}
''' % (delta, patch, rec, e["coll"], idx, label, delta, patch, f, val, e["nest"])
    T = e["T"]; top = e["top"]
    return head + '''use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998%sDelta, En1998%sPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &%s, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.%s.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "%s", Vec::<String>::new());
    };
    let patch = En1998%sPatch { %s: Some(%s), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { %s: En1998%sDelta::modification(&row.id, patch), ..Default::default() })
}
''' % (T, T, rec, top, e["sel"][0][3], T, f, val, top, T)

def inverse_rs(r):
    if r.get("row"): return row_inverse_rs(r)
    if ENT[r["ent"]].get("var"): return var_inverse_rs(r)
    ent = r["ent"]; e = ENT[ent]; f = r["snake"]; ty = r["ty"]; rec = r["record"]; mod = r["module"]
    head = "//! ↩️ Inverse for `%s`.\nuse crate::%s::%s;\nuse semio_s_artifact_norm_en1998_model::En1998Snapshot;\n\n" % (r["kind"], mod, rec)
    sig = "pub fn inverse<Op>(payload: &%s, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%s::%s> {\n" % (rec, mod, rec)
    selcopy = "".join("%s: payload.%s, " % (s[0], s[0]) for s in e["sel"])
    if ent == "site":
        body = "    Ok(vec![Op::from(%s { new_%s: base.site.%s%s })])\n}\n" % (rec, f, f, cl(ty))
    else:
        if ent == "building": chain = "base.buildings.get(payload.building_index)"
        elif "nest" in e: chain = "base.buildings.get(payload.building_index).and_then(|b| b.%s.get(payload.%s))" % (e["coll"], e["sel"][1][0])
        else: chain = "base.%s.get(payload.index)" % e["top"]
        body = "    Ok(match %s {\n        Some(row) => vec![Op::from(%s { %snew_%s: row.%s%s })],\n        None => Vec::new(),\n    })\n}\n" % (chain, rec, selcopy, f, f, cl(ty))
    return head + sig + body

def schema_json(r):
    if r.get("row"): return row_schema_json(r)
    if r.get("op"): return op_schema_json(r)
    e = ENT[r["ent"]]; f = r["snake"]; ty = r["ty"]
    en, de = label_for(r["ent"], f)
    props = {"mutation": {"const": camel(r["record"][:1].lower() + r["record"][1:] if False else r["record"])}}
    props["mutation"] = {"const": r["record"][:1].lower() + r["record"][1:]}
    req = ["mutation"]
    order = 10
    for (name, lab_en, lab_de, what) in e["sel"]:
        props[camel(name)] = {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": lab_en, "de": lab_de}, "description": {"en": "Zero-based position of the %s in its list." % what, "de": "Nullbasierte Position in der Liste (%s)." % lab_de}, "step": 1, "precision": 0, "group": "target", "order": order}}
        req.append(camel(name)); order += 10
    nf = "new" + pascal(f)
    p = json_type_schema(ty)
    ui = {"role": "value", "label": {"en": en, "de": de}, "group": "value", "order": order}
    if ty == "bool": ui["widget"] = "toggle"
    elif ty in ("f64", "u8"): ui["widget"] = "stepper"; ui["step"] = 1 if ty == "u8" else 0.001; ui["precision"] = 0 if ty == "u8" else 3
    else: ui["widget"] = "text"
    p["x-semio-ui"] = ui
    props[nf] = p; req.append(nf)
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/%s/schema.json" % r["kind"], "title": r["record"], "type": "object", "additionalProperties": False, "required": req, "properties": props}

def schema_ts(r):
    if r.get("row"): return row_schema_ts(r)
    if r.get("op"): return op_schema_ts(r)
    e = ENT[r["ent"]]; ty = r["ty"]; rec = r["record"]
    nf = "new" + pascal(r["snake"])
    tsty = {"f64": "number", "u8": "number", "bool": "boolean"}.get(ty, "string")
    reader = {"f64": "normWireNumber", "u8": "normWireRange(normWireInteger, {\"minimum\":0,\"maximum\":255})", "bool": "normWireBoolean"}.get(ty, "normWireString")
    imports = {"normWireInteger", "normWireLiteral", "normWireObject", "normWireRange", "normWireRequired"}
    if ty == "f64": imports.add("normWireNumber")
    if ty == "bool": imports.add("normWireBoolean")
    if tsty == "string": imports.add("normWireString")
    imp = ", ".join(sorted(imports | {"type NormWireReader"}))
    members = "  mutation: \"%s\";\n" % (rec[:1].lower() + rec[1:]) + "".join("  %s: number;\n" % camel(s[0]) for s in e["sel"]) + "  %s: %s;\n" % (nf, tsty)
    rdr = "mutation: normWireRequired(normWireLiteral(\"%s\")), " % (rec[:1].lower() + rec[1:]) + "".join("%s: normWireRequired(normWireRange(normWireInteger, {\"minimum\":0})), " % camel(s[0]) for s in e["sel"]) + "%s: normWireRequired(%s)" % (nf, reader)
    return '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { %s } from "@semio-tech/norm-js/registry/contract";

export interface %s {
%s}

export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });
''' % (EMOJI, r["kind"], rec, imp, rec, members, rec, rec, rec, rdr)

def catalog_json(r, tag):
    return {"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + r["dirname"], "semanticKind": r["kind"], "displayName": LABELS[r["op"]][0].format(w=r["ent"].replace("-", " ")) if r.get("op") else "Change " + label_for(r["ent"], r["snake"])[0], "emoji": LABELS[r["op"]][2] if r.get("op") else EMOJI, "aggregateVariant": r["record"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}

def new_value(ent, snake, ty, old):
    if ty == "f64":
        v = old * 2.0 if old else 1.5
        if v == old: v = old + 1.0
        return round(v, 6)
    if ty == "bool": return not old
    if ty == "u8": return 1 if old != 1 else 2
    if snake in ENUM_ALT: return ENUM_ALT[snake][1]
    alt = STR_ALT.get(snake)
    if alt is None: alt = "bldg-office" if "building_id" in snake else "x"
    if alt == old: alt = alt + "2"
    return alt

def rust_sample(r, newv):
    ty = r["ty"]
    if ty == "f64": return repr(float(newv)) if "e" not in repr(float(newv)) else repr(float(newv))
    if ty == "bool": return "true" if newv else "false"
    if ty == "u8": return "%du8" % newv
    if r["snake"] in ENUM_ALT: return "semio_s_artifact_norm_en1998_model::" + ENUM_ALT[r["snake"]][0]
    return 'String::from("%s")' % newv

def get_row(snap, r, e, idxs):
    ent = r["ent"]
    if ent == "site": return snap["site"]
    if ent == "building": return snap["buildings"][idxs[0]]
    if "nest" in e: return snap["buildings"][idxs[0]][e["coll"]][idxs[1]]
    return snap[TOPS[ent]][idxs[0]]

def build_fixture(r, tag):
    if r.get("row"): return row_fixture(r, tag)
    if ENT[r["ent"]].get("var"): return var_fixture(r, tag)
    ent = r["ent"]; e = ENT[ent]; f = camel(r["snake"])
    snap, base = load_fixture(e["fx"])
    if ent in ROWS: snap[ROWS[ent]["camel"]] = [copy.deepcopy(ROWS[ent]["first"])]
    nsel = len(e["sel"])
    idxs = [0] * nsel
    if ent == "storey": idxs = [0, 1]
    row = get_row(snap, r, e, idxs)
    assert f in row or r["ent"] == "site", (r["kind"], f, list(row))
    old = row.get(f, "")
    newv = new_value(ent, r["snake"], r["ty"], old)
    r["_new"] = newv
    after = copy.deepcopy(snap)
    arow = get_row(after, r, e, idxs)
    arow[f] = newv
    if ent == "site":
        order = ["seismicZone", "aGr", "deGroundCombo", "enGroundType", "enSpectrumType", "importanceClass"]
        ordered = {k: arow[k] for k in order if k in arow}
        arow.clear(); arow.update(ordered)
    mut = {"mutation": r["record"][:1].lower() + r["record"][1:]}
    for s, i in zip(e["sel"], idxs): mut[camel(s[0])] = i
    mut["new" + pascal(r["snake"])] = newv
    # diff
    tmpl = json.loads(rd(base + "/🔣️.diff.json"))
    diff = copy.deepcopy(tmpl)
    for k, v in list(diff.items()):
        if isinstance(v, dict) and "modified" in v:
            v["modified"] = []; v["removed"] = []; v["inserted"] = []; v["moved"] = []
    if ent == "site":
        diff["site"] = after["site"]
    else:
        emptydelta = {"removed": [], "inserted": [], "moved": [], "modified": []}
        def patch_for(rowobj, keys_nest):
            p = {}
            for k in PATCHKEYS[ent]:
                p[k] = None
            return p
        if ent == "building":
            b = after["buildings"][0]
            p = bpatch(); p[f] = newv
            diff["buildings"]["modified"] = [{"id": b["id"], "patch": p}]
        elif "nest" in e:
            b = after["buildings"][0]; rowid = after["buildings"][0][e["coll"]][idxs[1]]["id"]
            p = bpatch(); np_ = {k: None for k in PATCHKEYS[ent]}; np_[f] = newv
            p[e["nest"]] = {"removed": [], "inserted": [], "moved": [], "modified": [{"id": rowid, "patch": np_}]}
            diff["buildings"]["modified"] = [{"id": b["id"], "patch": p}]
        else:
            top = TOPS[ent]
            rowid = after[top][0]["id"]
            np_ = {k: None for k in PATCHKEYS[ent]}; np_[f] = newv
            diff[top] = {"removed": [], "inserted": [], "moved": [], "modified": [{"id": rowid, "patch": np_}]}
    return snap, mut, after, diff, base

def bpatch():
    empty = {"removed": [], "inserted": [], "moved": [], "modified": []}
    keys = PATCHKEYS["building"]
    keys = keys[:3] + ["systems", "storeys", "members"] + keys[3:]
    return {k: (dict(empty) if k in ("systems", "storeys", "members") else None) for k in keys}

def fix_patch_keys():
    d = rd(ANY + "/🧬️schema/🔺️diff/🦀️.rs")
    names = {"system": "En1998SystemPatch", "storey": "En1998StoreyPatch", "member": "En1998MemberPatch", "bridge": "En1998BridgePatch", "assessment": "En1998AssessmentPatch", "tower": "En1998TowerPatch", "silo": "En1998SiloPatch", "tank": "En1998TankPatch", "foundation": "En1998FoundationPatch", "soil-layer": "En1998SoilLayerPatch", "diaphragm": "En1998DiaphragmPatch", "accelerogram": "En1998AccelerogramPatch", "pier": "En1998PierPatch", "bearing": "En1998BearingPatch", "joint": "En1998JointPatch", "retaining-wall": "En1998RetainingWallPatch", "building": "En1998BuildingPatch"}
    out = {}
    for ent, nm in names.items():
        m = re.search(r"pub %s of [\w:]+ \{ set \{ ([^}]*) \}" % nm, d)
        out[ent] = [camel(x.split(":")[0].strip()) for x in m.group(1).split(", ")]
    return out
PATCHKEYS = fix_patch_keys()

def rep(s, a, b, n=1):
    assert a in s, a[:80]
    return s.replace(a, b, n)

def new_crates(todo):
    base = MUT + "/🏢️buildings-site"
    root_src = rd(base + "/🦀️.rs")
    pre = root_src[root_src.index("#![allow"): root_src.index("//#region 🪢️Mounts")]
    toml = rd(base + "/📦️packages/🦀️rust/Cargo.toml")
    for name, (emoji, agg, leaves) in NEWCRATES.items():
        rs = [r for r in todo if (r["ent"], r["snake"]) in leaves]
        if not rs: continue
        cname = "semio-s-artifact-norm-en1998-mut-" + name
        cdir = MUT + "/" + emoji + name
        mounts = "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (r["dirname"], r["module"]) for r in rs)
        wr(cdir + "/🦀️.rs", "//! %s EN 1998 %s mutation leaves: %s.\n\n" % (emoji, name.replace("-", " "), ", ".join(r["snake"].replace("_", " ") for r in rs[:3])) + pre + "//#region 🪢️Mounts\n" + mounts + "//#endregion 🪢️Mounts\n")
        wr(cdir + "/📦️packages/🦀️rust/Cargo.toml", toml.replace("mut-buildings-site", "mut-" + name).replace('description = "EN 1998 buildings mutation leaves: site"', 'description = "EN 1998 %s mutation leaves"' % name.replace("-", " ")))
        ident = cname.replace("-", "_")
        mods = ", ".join(r["module"] for r in rs)
        ap = MUT + "/" + GROUP_DIR[agg] + "/🦀️.rs"
        a = rd(ap)
        if ident not in a: wr(ap, a.rstrip("\n") + "\n\npub use %s::{%s};\n" % (ident, mods))
        at = MUT + "/" + GROUP_DIR[agg] + "/📦️packages/🦀️rust/Cargo.toml"
        t = rd(at)
        if cname not in t: wr(at, rep(t, "\n\n[dev-dependencies]", "\n%s = { workspace = true }\n\n[dev-dependencies]" % cname))
        rel = os.path.relpath(cdir + "/📦️packages/🦀️rust", ART)
        wp = ART + "/Cargo.toml"
        w = rd(wp)
        if rel not in w:
            site = "🏢️buildings-site/📦️packages/🦀️rust"
            w = rep(w, '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s",\n' % site, '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s",\n    "%s",\n' % (site, rel))
            w = rep(w, "[workspace.lints.rust]", '[workspace.dependencies.%s]\npath = "%s"\n\n[workspace.lints.rust]' % (cname, rel))
            wr(wp, w)
        sp = ART + "/📦️packages/🦀️rust/📜️script.ts"
        sc = rd(sp)
        if '"%s"' % cname not in sc: wr(sp, rep(sc, '"semio-s-artifact-norm-en1998-mutations"', '"%s","semio-s-artifact-norm-en1998-mutations"' % cname))
        op = "/home/deck/semio/📋️build-owners.json"
        o = rd(op)
        if '"name": "%s"' % cname not in o:
            m = re.search(r'    \{\n      "name": "semio-s-artifact-norm-en1998-mut-buildings-site",\n.*?\n    \},\n', o, re.S)
            wr(op, o.replace(m.group(0), m.group(0) + m.group(0).replace("mut-buildings-site", "mut-" + name).replace("🏢️buildings-site", emoji + name), 1))
        lp = ART + "/Cargo.lock"
        l = rd(lp)
        if 'name = "%s"' % cname not in l:
            m = re.search(r'\[\[package\]\]\nname = "semio-s-artifact-norm-en1998-mut-buildings-site"\n.*?\n\n', l, re.S)
            l = l.replace(m.group(0), m.group(0) + m.group(0).replace("mut-buildings-site", "mut-" + name), 1)
            am = re.search(r'name = "semio-s-artifact-norm-en1998-mut-%s"\nversion = "0.1.0"\ndependencies = \[\n(.*?)\]' % agg.replace("_", "-"), l, re.S)
            deps = sorted(set(am.group(1).splitlines()) | {' "%s",' % cname})
            l = l.replace(am.group(0), am.group(0).replace(am.group(1), "\n".join(deps) + "\n"), 1)
            wr(lp, l)


def main():
    # existing kinds
    root_rs = rd(MUT + "/🦀️.rs")
    existing = set(re.findall(r'^\s+"([a-z0-9-]+)",$', root_rs, re.M))
    todo = [r for r in records if r["kind"] not in existing]
    print("to generate:", len(todo))
    taxonomy_p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    tax = json.loads(rd(taxonomy_p))
    # next binary tag
    proto_p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    proto = rd(proto_p)
    tag = max(int(x) for x in re.findall(r"tag=(\d+)", proto)) + 1
    groups = {g: [] for g in GROUP_CRATE}
    uses_root, variants, kinds, kinds_py = [], [], [], []
    text_print, text_parse, text_sample, text_use, text_use2 = [], [], [], [], []
    bin_use, bin_tag, bin_enc_tag, bin_enc, bin_dec, proto_add = [], [], [], [], [], []
    feat_rows, rs_arms, py_rows, oracle_dirs, oracle_kinds, oracle_ops = [], [], [], [], [], []
    ts_imports, ts_union, ts_parsers, edit_rules, root_schema_refs = [], [], [], [], []
    ins_rules, rem_rules = [], []
    for r in todo:
        e = ENT[r["ent"]]; g = e["group"]; d = MUT + "/" + r["dirname"]
        wr(d + "/🦀️.rs", leaf_rs(r)); wr(d + "/🔺️diff/🦀️.rs", diff_rs(r)); wr(d + "/↩️inverse/🦀️.rs", inverse_rs(r))
        wr(d + "/🧬️schema/🔣️.json", dumps(schema_json(r))); wr(d + "/🧬️schema/🟦️.ts", schema_ts(r)); wr(d + "/🔣️.json", dumps(catalog_json(r, tag)))
        snap, mut, after, diff, base = build_fixture(r, tag)
        fx = FIX + "/" + r["dirname"] + "/✅apply"
        wr(fx + "/🔣️.before.json", dumps(snap)); wr(fx + "/🔣️.mutation.json", dumps(mut)); wr(fx + "/🔣️.after.json", dumps(after)); wr(fx + "/🔣️.outcome.json", dumps({"status": "applied"})); wr(fx + "/🔣️.diff.json", dumps(diff))
        groups[g].append(r)
        assert (r["ent"], r["snake"]) in CRATE_OF, r["kind"]
        rec, mod, kind = r["record"], r["module"], r["kind"]
        variants.append("    %s(%s::%s)," % (rec, mod, rec)); kinds.append('    "%s",' % kind)
        uses_root.append((g, mod))
        wf = wire_of(r)
        fmt_args = " ".join("%s={}" % kebab(x) for x in wf)
        enc = ", ".join("enc_json(&p.%s)" % x for x in wf)
        text_print.append('        En1998Mutation::%s(p) => format!("%s %s", %s),' % (rec, kind, fmt_args, enc))
        dec = ", ".join('index: opt_index(&args)?' if x == "index" else '%s: dec_json(&arg("%s")?)?' % (x, kebab(x)) for x in wf)
        text_parse.append('        "%s" => Ok(En1998Mutation::%s(%s { %s })),' % (kind, rec, rec, dec))
        sels = [s[0] for s in e["sel"]]; nf = "new_" + r["snake"]
        sample = r.get("sample") or (", ".join("%s: 0usize" % x for x in sels) + (", " if sels else "") + "%s: %s" % (nf, rust_sample(r, r["_new"])))
        text_sample.append("        En1998Mutation::%s(%s { %s })," % (rec, rec, sample))
        text_use.append("use %s::%s::%s;" % (GROUP_CRATE[g], mod, rec)); text_use2.append("use %s::%s;" % (GROUP_CRATE[g], mod))
        const = "TAG_" + mod.upper()
        bin_use.append("use %s::%s::%s;" % (GROUP_CRATE[g], mod, rec))
        bin_tag.append('const %s: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "%s");\n' % (const, kind))
        bin_enc_tag.append("            En1998Mutation::%s(_) => %s," % (rec, const))
        bin_enc.append("            En1998Mutation::%s(p) => {\n%s            }" % (rec, "".join("                write_json_bin(&mut out, &p.%s);\n" % x for x in wf)))
        bin_dec.append("            %s => {\n%s                Ok(En1998Mutation::%s(%s { %s }))\n            }" % (const, "".join('                let %s = read_json_bin(&mut reader).map_err(|e| malformed("%s", reader.position(), e))?;\n' % (x, x) for x in wf), rec, rec, ", ".join(wf)))
        proto_add.append("record %s tag=%d\nfield payload bytes\n" % (kind, tag))
        feat_rows.append((kind, r["dirname"]))
        rs_arms.append('        "%s" => (\n%s        ),' % (kind, "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (r["dirname"], p) for p in ["before", "mutation", "after", "outcome"])))
        py_rows.append('    "%s": ("%s", "✅apply"),' % (kind, r["dirname"])); kinds_py.append('    "%s",' % kind)
        ts_imports.append('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";' % (rec, rec, r["dirname"]))
        ts_union.append("  | " + rec); ts_parsers.append("  %s: parse%s," % (rec[:1].lower() + rec[1:], rec))
        sel_rules = ", ".join('SelectorKey::Index("%s")' % camel(x) for x in sels)
        parent_rules = ", ".join('SelectorKey::Index("%s")' % camel(x) for x in sels[:-1])
        ppath = e["path"][:-2]
        if r.get("row") and r["op"] == "insert": ins_rules.append('        InsertItemRule { path: "%s", kind: "%s", selectors: &[], index: "index", item: "%s" },' % (ROWS[r["ent"]]["camel"], kind, ROWS[r["ent"]]["item"]))
        elif r.get("row"): rem_rules.append('        RemoveItemRule { path: "%s", kind: "%s", selectors: &[], index: "index" },' % (ROWS[r["ent"]]["camel"], kind))
        elif r.get("op") == "insert": ins_rules.append('        InsertItemRule { path: "%s", kind: "%s", selectors: &[%s], index: "index", item: "variable" },' % (ppath, kind, parent_rules))
        elif r.get("op") == "remove": rem_rules.append('        RemoveItemRule { path: "%s", kind: "%s", selectors: &[%s], index: "variableIndex" },' % (ppath, kind, parent_rules))
        else: edit_rules.append('        SetFieldRule { path: "%s", kind: "%s", selectors: &[%s], value: "new%s" },' % (jpath(r["ent"], r["snake"]).replace("[]", "[]"), kind, sel_rules, pascal(r["snake"])))
        root_schema_refs.append(kind)
        tag += 1
    if not todo: return
    new_crates(todo)
    # root enum
    s = root_rs
    uses = "".join("use %s::%s;\n" % (GROUP_CRATE[g], m) for g, m in uses_root)
    i = s.index("use semio_s_artifact_norm_en1998_mut_buildings::change_annex") if "use semio_s_artifact_norm_en1998_mut_buildings::change_annex" in s else None
    assert i is not None
    s = s[:i] + uses + s[i:]
    s = rep(s, "\n}\n\npub const KINDS", "\n" + "\n".join(variants) + "\n}\n\npub const KINDS", 1)
    k0 = s.index("pub const KINDS")
    k1 = s.index("\n];", k0)
    s = s[:k1] + "\n" + "\n".join(kinds) + s[k1:]
    wr(MUT + "/🦀️.rs", s)
    # root json
    p = MUT + "/🔣️.json"; j = json.loads(rd(p))
    for k in root_schema_refs: j["oneOf"].append({"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/%s/schema.json" % k})
    wr(p, dumps(j))
    # text codec
    p = ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"; s = rd(p)
    s = rep(s, "use semio_s_artifact_norm_en1998_mut_buildings::change_annex::ChangeAnnex;\n", "\n".join(text_use) + "\nuse semio_s_artifact_norm_en1998_mut_buildings::change_annex::ChangeAnnex;\n", 1)
    s = rep(s, "        En1998Mutation::RemoveSilo(p) => format!(", "\n".join(text_print) + "\n        En1998Mutation::RemoveSilo(p) => format!(", 1)
    s = rep(s, '        other => Err(format!("en1998 mutation: unknown keyword', "\n".join(text_parse) + '\n        other => Err(format!("en1998 mutation: unknown keyword', 1)
    s = rep(s, "use semio_s_artifact_norm_en1998_mut_buildings::change_annex;\n", "\n".join(text_use2) + "\nuse semio_s_artifact_norm_en1998_mut_buildings::change_annex;\n", 1)
    wr(p, s)
    wr(proto_p, proto.rstrip("\n") + "\n" + "".join(proto_add))
    # mutate harness
    p = ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs"; s = rd(p)
    z = s.index('    "remove-tower",\n')
    z = s.index("\n];", z)
    s = s[:z] + "\n" + "\n".join(kinds) + s[z:]
    anchor = '        other => panic!("mutate-en1998-1: no committed fixture'
    assert anchor in s
    s = rep(s, anchor, "\n".join(rs_arms) + "\n" + anchor, 1)
    wr(p, s)
    p = ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature"; s = rd(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % x for x in feat_rows)
    a1 = "      | remove-tower-middle-row      | ➖️remove-tower                 | 🔬️middle-row |\n"
    assert a1 in s
    s = rep(s, a1, a1 + rows, 1)
    a2 = "      | remove-tower                 | ➖️remove-tower                 | ✅apply  |\n"
    i = s.rindex(a2)
    s = s[:i] + a2 + rows + s[i + len(a2):]
    wr(p, s)
    p = ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py"; s = rd(p)
    z = s.index('    "remove-tower",\n')
    z = s.index("\n]", z)
    s = s[:z] + "\n" + "\n".join(kinds_py) + s[z:]
    wr(p, s)
    p = ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py"; s = rd(p)
    a = '    "remove-tower-middle-row": ("remove-tower", "➖️remove-tower", "🔬️middle-row"),\n'
    assert a in s
    s = rep(s, a, a + "\n".join(py_rows) + "\n", 1)
    wr(p, s)
    p = ANY + "/🔮️oracles/🔣️.json"; oj = json.loads(rd(p))
    cat = oj["mutationCatalogs"][0]; man = oj["mutationManifests"][0]
    vec_t = [v for v in cat["vectors"] if v["mutationId"] == "change-storey-drift-xm"][0]
    mut_t = [m for m in man["mutations"] if m["id"] == "change-storey-drift-xm"][0]
    for r in todo:
        v = copy.deepcopy(vec_t); v["mutationId"] = r["kind"]; v["sourceMutationDirectoryName"] = r["dirname"]; v["mutationDirectoryName"] = r["dirname"]
        cat["vectors"].append(v); cat["kinds"].append(r["kind"])
        m = copy.deepcopy(mut_t); m["id"] = r["kind"]; m["productionDispatch"]["operation"] = r["kind"]; m["productionDispatch"]["variant"] = r["record"]
        man["mutations"].append(m)
    wr(p, dumps(oj))
    tp = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"; tj = json.loads(rd(tp))
    last = max(i for i, x in enumerate(tj["rows"]) if x["artifact"] == "🫨️en1998")
    new_rows = []
    for r in todo:
        new_rows.append({"aggregateVariant": r["record"], "artifact": "🫨️en1998", "entity": r["ent"] if r.get("op") else r["ent"] + "-" + kebab(r["snake"]), "kind": r["kind"], "module": r["dirname"], "physicalLayout": "direct", "record": r["record"], "source": "🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/🦀️.rs" % r["dirname"], "standard": "🔖️1", "subset": "✳️any", "type": r["record"], "verb": r.get("op", "change")})
    new_rows.sort(key=lambda x: x["module"])
    tj["rows"][last + 1:last + 1] = new_rows
    wr(tp, dumps(tj))
    open(ANY + "/🧬️schema/🧬️mutations/🧭️edit-rules/generated-rules.txt", "w").write("") if False else None
    p = MUT + "/🧭️edit-rules/🦀️.rs"; s = rd(p)
    anchor = re.search(r'        SetFieldRule \{ path: "buildings\[\]\.masonryWallAreaRatio"[^\n]*\n', s)
    assert anchor
    s = s[:anchor.end()] + "\n".join(edit_rules) + "\n" + s[anchor.end():] if edit_rules else s
    if ins_rules:
        a = '        InsertItemRule { path: "silos", kind: "insert-silo", selectors: &[], index: "index", item: "silo" },\n'
        s = rep(s, a, a + "\n".join(ins_rules) + "\n", 1)
    if rem_rules:
        a = '        RemoveItemRule { path: "towers", kind: "remove-tower", selectors: &[], index: "index" },\n'
        s = rep(s, a, a + "\n".join(rem_rules) + "\n", 1)
    wr(p, s)
    print("generated", len(todo), "leaves")

if __name__ == "__main__":
    main()
