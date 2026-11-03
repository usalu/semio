#!/usr/bin/env python3
"""Generates one `change-<entity>-<leaf>` mutation leaf per editable EN 1994 snapshot leaf across every layer (idempotent: kinds already in the dispatch enum are skipped).

Layers touched: leaf crates (payload, diff, inverse, schema json/ts, catalogue json), group crates, dispatch enum + KINDS, mutation json/ts twins,
diff patch structs (beam/column/slab rows) with their json/graphql/proto/ts twins, edit rules, text codec imports, binary protocol tags,
oracle catalogue and manifest, taxonomy fixture, mutate harness (rs, py, feature), testkit, fixtures (before/mutation/after/outcome/diff),
workspace registration (members, dependencies, package router, build owners, consumers).
Run with DRY=1 for a dry run; `python3 -I` is enough, no third-party libraries.
"""
import copy, json, os, re, sys

SEMIO = "/home/deck/semio"
ROOT = SEMIO + "/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🧩️en1994"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
DIFF = ANY + "/🧬️schema/🔺️diff"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
HARNESS = ANY + "/🧪️tests/🧩️mutate-en1994-1"
TAX = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
OWNERS = SEMIO + "/📋️build-owners.json"
DRY = bool(os.environ.get("DRY"))
CRATE_PREFIX = "semio-s-artifact-norm-en1994-mut-"
SCHEMA_ID = "https://json.schemas.assets.semio-tech.com/s/norm/en1994/mutation/%s/schema.json"
SNAPSHOT_REF = "https://json.schemas.assets.semio-tech.com/s/norm/en1994/snapshot.json#/$defs/"


def rd(path):
    return open(path, encoding="utf-8").read()


def wr(path, text):
    if DRY:
        return
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text)


def dumps(obj):
    return json.dumps(obj, indent=2, ensure_ascii=False) + "\n"


def camel(snake):
    parts = snake.split("_")
    return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:])


def pascal(snake):
    return "".join(p[:1].upper() + p[1:] for p in re.split(r"[_-]", snake))


def rep(text, old, new, count=1):
    assert old in text, "anchor missing: " + old[:100]
    return text.replace(old, new, count)


def insert_after_last(text, regex, new_lines):
    last = None
    for match in re.finditer(regex, text, re.M):
        last = match
    assert last is not None, "pattern missing: " + regex
    return text[: last.end()] + new_lines + text[last.end():]


ENT = {
    "beam": dict(coll="beams", rows="Beams", model="CompositeBeam", word="beam", sel=("index", "Composite beam", "Verbundträger"), schema_fx="📏️change-beam-span-m", base_fx="#️⃣change-beam-stud-count", action_fx="🌀️change-beam-action-q-area-pa", action_type="CharacteristicAction"),
    "column": dict(coll="columns", rows="Columns", model="CompositeColumn", word="column", sel=("index", "Composite column", "Verbundstütze"), schema_fx="↪️change-column-kind", base_fx="⬇️change-column-action-force-n", action_fx="⬇️change-column-action-force-n", action_type="ColumnAction"),
    "slab": dict(coll="slabs", rows="Slabs", model="CompositeSlab", word="slab", sel=("index", "Composite slab", "Verbunddecke"), schema_fx="📏change-slab-thickness-m", base_fx="📐️change-slab-action-q-area-pa", action_fx="📐️change-slab-action-q-area-pa", action_type="CharacteristicAction"),
}

UNITS = {
    "m": dict(unit="m", step=0.01, precision=3),
    "m2": dict(unit="m2", step=0.0001, precision=6),
    "m3": dict(unit="m3", step=0.00001, precision=7),
    "m4": dict(unit="m4", step=0.000001, precision=9),
    "m2pm": dict(unit="m2/m", step=0.00001, precision=6),
    "pa": dict(unit="Pa", displayUnit="MPa", displayFactor=1e-06, step=1000000, precision=1),
    "n": dict(unit="N", displayUnit="kN", displayFactor=0.001, step=1000, precision=1),
    "nm": dict(unit="N·m", displayUnit="kN·m", displayFactor=0.001, step=1000, precision=1),
    "ratio": dict(step=0.01, precision=3),
    "cycles": dict(step=100000, precision=0),
    "count": dict(step=1, precision=0),
}


def L(ent, nest, access, ty, kind, unit, en, de, group, emoji, container, patch=None, minimum=True, crate=None, sample=None):
    return dict(ent=ent, nest=nest, access=access, ty=ty, kind=kind, unit=unit, en=en, de=de, group=group, emoji=emoji, container=container, patch=patch or access.replace(".", "_"), minimum=minimum, crate=crate, sample=sample)


B, C, S = "beam", "column", "slab"
CB, CC, CS = "CompositeBeam", "CompositeColumn", "CompositeSlab"
LEAVES = [
    L(B, 0, "spacing_m", "f64", "change-beam-spacing-m", "m", "beam spacing", "Trägerabstand", "beam", "↔️", CB, crate="beam-fields"),
    L(B, 0, "support", "String", "change-beam-support", None, "support condition of the beam", "Lagerungsart des Trägers", "beam", "🪢️", CB, crate="beam-fields", sample="continuous_2_span"),
    L(B, 0, "concrete_f_ck_pa", "f64", "change-beam-concrete-f-ck-pa", "pa", "concrete cylinder strength f_ck of the beam slab", "Zylinderdruckfestigkeit f_ck der Trägerplatte", "beam", "🪨️", CB, crate="beam-fields"),
    L(B, 0, "concrete_e_cm_pa", "f64", "change-beam-concrete-e-cm-pa", "pa", "concrete modulus E_cm of the beam slab", "Elastizitätsmodul E_cm der Trägerplatte", "beam", "🧱️", CB, crate="beam-fields"),
    L(B, 0, "ltb_length_m", "f64", "change-beam-ltb-length-m", "m", "lateral restraint spacing of the steel beam", "Abstand der seitlichen Halterungen des Stahlträgers", "beam", "🌀️", CB, crate="beam-fields"),
    L(B, 0, "as_hogging_m2_per_m", "f64", "change-beam-hogging-rebar-m2-per-m", "m2pm", "hogging reinforcement per metre", "Stützbewehrung je Meter", "beam", "🕸️", CB, crate="beam-fields"),
    L(B, 0, "bar_spacing_m", "f64", "change-beam-bar-spacing-m", "m", "reinforcement bar spacing", "Stababstand der Bewehrung", "beam", "📏️", CB, crate="beam-fields"),
    L(B, 0, "wk_limit_m", "f64", "change-beam-crack-width-limit-m", "m", "crack width limit w_k", "Rissbreitengrenze w_k", "beam", "🧬️", CB, crate="beam-fields"),
    L(B, 0, "n_cycles", "f64", "change-beam-fatigue-cycles", "cycles", "number of fatigue load cycles", "Anzahl der Ermüdungslastwechsel", "beam", "🔁️", CB, crate="beam-fields"),
    L(B, 0, "steel.designation", "String", "change-beam-steel-designation", None, "steel section designation", "Profilbezeichnung des Stahlträgers", "steel-section", "🔩️", "SteelSection", patch="steel", crate="beam-steel", sample="HEB320"),
    L(B, 0, "steel.height_m", "f64", "change-beam-steel-height-m", "m", "steel section height h", "Profilhöhe h des Stahlträgers", "steel-section", "↕️", "SteelSection", crate="beam-steel"),
    L(B, 0, "steel.width_m", "f64", "change-beam-steel-width-m", "m", "steel flange width b", "Flanschbreite b des Stahlträgers", "steel-section", "↔️", "SteelSection", crate="beam-steel"),
    L(B, 0, "steel.tw_m", "f64", "change-beam-steel-web-thickness-m", "m", "steel web thickness t_w", "Stegdicke t_w des Stahlträgers", "steel-section", "🧱️", "SteelSection", crate="beam-steel"),
    L(B, 0, "steel.tf_m", "f64", "change-beam-steel-tf-m", "m", "steel flange thickness t_f", "Flanschdicke t_f des Stahlträgers", "steel-section", "🧱️", "SteelSection", crate="beam-steel"),
    L(B, 0, "steel.a_m2", "f64", "change-beam-steel-area-m2", "m2", "steel section area A", "Querschnittsfläche A des Stahlträgers", "steel-section", "⬛️", "SteelSection", crate="beam-steel"),
    L(B, 0, "steel.w_pl_y_m3", "f64", "change-beam-steel-w-pl-y-m3", "m3", "plastic section modulus W_pl,y", "Plastisches Widerstandsmoment W_pl,y", "steel-section", "📐️", "SteelSection", crate="beam-steel"),
    L(B, 0, "steel.i_y_m4", "f64", "change-beam-steel-second-moment-m4", "m4", "second moment of area I_y", "Flächenträgheitsmoment I_y", "steel-section", "📐️", "SteelSection", crate="beam-steel"),
    L(B, 0, "steel.a_v_m2", "f64", "change-beam-steel-shear-area-m2", "m2", "shear area A_v", "Schubfläche A_v", "steel-section", "✂️", "SteelSection", crate="beam-steel"),
    L(B, 0, "steel.r_m", "f64", "change-beam-steel-root-radius-m", "m", "fillet radius r", "Ausrundungsradius r", "steel-section", "⭕️", "SteelSection", crate="beam-steel"),
    L(B, 0, "sheeting.profile", "String", "change-beam-sheeting-profile", None, "profiled sheeting type", "Profilblechtyp", "sheeting", "🏗️", "ProfiledSheeting", crate="beam-sheeting", sample="re_entrant"),
    L(B, 0, "sheeting.height_m", "f64", "change-beam-sheeting-height-m", "m", "profiled sheeting rib height h_p", "Rippenhöhe h_p des Profilblechs", "sheeting", "🏗️", "ProfiledSheeting", crate="beam-sheeting"),
    L(B, 0, "sheeting.rib_width_m", "f64", "change-beam-sheeting-rib-width-m", "m", "profiled sheeting rib width b_0", "Rippenbreite b_0 des Profilblechs", "sheeting", "🏗️", "ProfiledSheeting", crate="beam-sheeting"),
    L(B, 0, "sheeting.thickness_m", "f64", "change-beam-sheeting-thickness-m", "m", "profiled sheeting thickness t", "Blechdicke t des Profilblechs", "sheeting", "🏗️", "ProfiledSheeting", crate="beam-sheeting"),
    L(B, 0, "sheeting.ribs_parallel_to_beam", "bool", "change-beam-sheeting-ribs-parallel", None, "sheeting ribs parallel to the beam", "Profilblechrippen parallel zum Träger", "sheeting", "🧭️", "ProfiledSheeting", crate="beam-sheeting"),
    L(B, 0, "studs.height_m", "f64", "change-beam-stud-height-m", "m", "headed stud height h_sc", "Höhe h_sc der Kopfbolzendübel", "shear-connection", "🔩️", "HeadedStuds", crate="beam-sheeting"),
    L(B, 0, "studs.count_per_rib", "u32", "change-beam-stud-count-per-rib", "count", "number of studs per rib", "Anzahl der Kopfbolzendübel je Rippe", "shear-connection", "#️⃣", "HeadedStuds", crate="beam-sheeting"),
    L(B, 1, "kind", "String", "change-beam-action-kind", None, "kind of the beam action", "Art der Trägereinwirkung", "actions", "🏋️", "CharacteristicAction", crate="beam-actions", sample="imposed"),
    L(B, 1, "category", "String", "change-beam-action-category", None, "category of the beam action", "Kategorie der Trägereinwirkung", "actions", "🏷️", "CharacteristicAction", crate="beam-actions", sample="finishes"),
    L(B, 1, "stage", "String", "change-beam-action-stage", None, "stage of the beam action", "Bauzustand der Trägereinwirkung", "actions", "🧭️", "CharacteristicAction", crate="beam-actions", sample="composite"),
    L(B, 1, "f_k_n", "f64", "change-beam-action-point-force-n", "n", "characteristic point force of the beam action", "Charakteristische Einzellast der Trägereinwirkung", "actions", "📍️", "CharacteristicAction", minimum=False, crate="beam-actions"),
    L(B, 1, "delta_sigma_k_pa", "f64", "change-beam-action-delta-sigma-pa", "pa", "fatigue stress range of the beam action", "Spannungsschwingbreite der Trägereinwirkung", "actions", "🔁️", "CharacteristicAction", crate="beam-actions"),
    L(B, 1, "delta_tau_k_pa", "f64", "change-beam-action-delta-tau-pa", "pa", "fatigue shear stress range of the beam action", "Schubspannungsschwingbreite der Trägereinwirkung", "actions", "🔁️", "CharacteristicAction", crate="beam-actions"),
    L(C, 0, "length_m", "f64", "change-column-length-m", "m", "column length", "Stützenlänge", "column", "📏️", CC, crate="column-section"),
    L(C, 0, "outer_size_m", "f64", "change-column-outer-size-m", "m", "column outer size", "Außenabmessung der Stütze", "column", "⬜️", CC, crate="column-section"),
    L(C, 0, "wall_thickness_m", "f64", "change-column-wall-thickness-m", "m", "column wall thickness", "Wanddicke der Stütze", "column", "🧱️", CC, crate="column-section"),
    L(C, 0, "steel_a_m2", "f64", "change-column-steel-area-m2", "m2", "column steel area A_a", "Stahlquerschnittsfläche A_a der Stütze", "column", "⬛️", CC, crate="column-section"),
    L(C, 0, "steel_f_y_pa", "f64", "change-column-steel-f-y-pa", "pa", "column steel yield strength f_y", "Streckgrenze f_y des Stützenstahls", "column", "🔩️", CC, crate="column-section"),
    L(C, 0, "concrete_a_m2", "f64", "change-column-concrete-area-m2", "m2", "column concrete area A_c", "Betonquerschnittsfläche A_c der Stütze", "column", "🪨️", CC, crate="column-section"),
    L(C, 0, "concrete_f_ck_pa", "f64", "change-column-concrete-f-ck-pa", "pa", "column concrete strength f_ck", "Betondruckfestigkeit f_ck der Stütze", "column", "🪨️", CC, crate="column-material"),
    L(C, 0, "reinforcement_as_m2", "f64", "change-column-rebar-area-m2", "m2", "column reinforcement area A_s", "Bewehrungsquerschnitt A_s der Stütze", "column", "🕸️", CC, crate="column-material"),
    L(C, 0, "reinforcement_f_yk_pa", "f64", "change-column-rebar-f-yk-pa", "pa", "column reinforcement strength f_yk", "Bewehrungsfestigkeit f_yk der Stütze", "column", "🕸️", CC, crate="column-material"),
    L(C, 0, "i_m4", "f64", "change-column-second-moment-m4", "m4", "column second moment of area", "Flächenträgheitsmoment der Stütze", "column", "📐️", CC, crate="column-material"),
    L(C, 0, "buckling_curve", "String", "change-column-buckling-curve", None, "column buckling curve", "Knickspannungslinie der Stütze", "column", "📉️", CC, crate="column-material", sample="b"),
    L(C, 0, "reinforcement_product", "String", "change-column-rebar-product", None, "reinforcement product (hot_rolled or cold_worked)", "Bewehrungsprodukt (hot_rolled oder cold_worked)", "column", "🧵️", CC, crate="column-material", sample="cold_worked"),
    L(C, 1, "kind", "String", "change-column-action-kind", None, "kind of the column action", "Art der Stützeneinwirkung", "actions", "🏋️", "ColumnAction", crate="column-material", sample="imposed"),
    L(C, 1, "category", "String", "change-column-action-category", None, "category of the column action", "Kategorie der Stützeneinwirkung", "actions", "🏷️", "ColumnAction", crate="column-material", sample="C"),
    L(C, 1, "stage", "String", "change-column-action-stage", None, "stage of the column action", "Bauzustand der Stützeneinwirkung", "actions", "🧭️", "ColumnAction", crate="column-material", sample="construction"),
    L(C, 1, "m_k_nm", "f64", "change-column-action-moment-nm", "nm", "characteristic moment of the column action", "Charakteristisches Moment der Stützeneinwirkung", "actions", "🔄️", "ColumnAction", minimum=False, crate="column-material"),
    L(S, 0, "span_m", "f64", "change-slab-span-m", "m", "slab span", "Deckenspannweite", "slab", "📏️", CS, crate="slab-fields"),
    L(S, 0, "support", "String", "change-slab-support", None, "support condition of the slab", "Lagerungsart der Decke", "slab", "🪢️", CS, crate="slab-fields", sample="continuous_2_span"),
    L(S, 0, "f_ck_pa", "f64", "change-slab-f-ck-pa", "pa", "slab concrete strength f_ck", "Betondruckfestigkeit f_ck der Decke", "slab", "🪨️", CS, crate="slab-fields"),
    L(S, 0, "m_factor", "f64", "change-slab-m-factor", "ratio", "m-k factor m of the slab", "m-k-Beiwert m der Decke", "slab", "🧮️", CS, crate="slab-fields"),
    L(S, 0, "k_factor", "f64", "change-slab-k-factor", "ratio", "m-k factor k of the slab", "m-k-Beiwert k der Decke", "slab", "🧮️", CS, crate="slab-fields"),
    L(S, 0, "as_m2_per_m", "f64", "change-slab-reinforcement-m2-per-m", "m2pm", "slab reinforcement per metre", "Deckenbewehrung je Meter", "slab", "🕸️", CS, crate="slab-fields"),
    L(S, 0, "sheeting.profile", "String", "change-slab-sheeting-profile", None, "slab profiled sheeting type", "Profilblechtyp der Decke", "sheeting", "🏗️", "ProfiledSheeting", crate="slab-sheeting", sample="re_entrant"),
    L(S, 0, "sheeting.height_m", "f64", "change-slab-sheeting-height-m", "m", "slab sheeting rib height h_p", "Rippenhöhe h_p des Deckenprofilblechs", "sheeting", "🏗️", "ProfiledSheeting", crate="slab-sheeting"),
    L(S, 0, "sheeting.rib_width_m", "f64", "change-slab-sheeting-rib-width-m", "m", "slab sheeting rib width b_0", "Rippenbreite b_0 des Deckenprofilblechs", "sheeting", "🏗️", "ProfiledSheeting", crate="slab-sheeting"),
    L(S, 0, "sheeting.thickness_m", "f64", "change-slab-sheeting-thickness-m", "m", "slab sheeting thickness t", "Blechdicke t des Deckenprofilblechs", "sheeting", "🏗️", "ProfiledSheeting", crate="slab-sheeting"),
    L(S, 0, "sheeting.ribs_parallel_to_beam", "bool", "change-slab-sheeting-ribs-parallel", None, "slab sheeting ribs parallel to the span", "Deckenprofilblechrippen parallel zur Spannrichtung", "sheeting", "🧭️", "ProfiledSheeting", crate="slab-sheeting"),
    L(S, 1, "kind", "String", "change-slab-action-kind", None, "kind of the slab action", "Art der Deckeneinwirkung", "actions", "🏋️", "CharacteristicAction", crate="slab-actions", sample="imposed"),
    L(S, 1, "category", "String", "change-slab-action-category", None, "category of the slab action", "Kategorie der Deckeneinwirkung", "actions", "🏷️", "CharacteristicAction", crate="slab-actions", sample="C"),
    L(S, 1, "stage", "String", "change-slab-action-stage", None, "stage of the slab action", "Bauzustand der Deckeneinwirkung", "actions", "🧭️", "CharacteristicAction", crate="slab-actions", sample="construction"),
    L(S, 1, "f_k_n", "f64", "change-slab-action-point-force-n", "n", "characteristic point force of the slab action", "Charakteristische Einzellast der Deckeneinwirkung", "actions", "📍️", "CharacteristicAction", minimum=False, crate="slab-actions"),
    L(S, 1, "delta_sigma_k_pa", "f64", "change-slab-action-delta-sigma-pa", "pa", "fatigue stress range of the slab action", "Spannungsschwingbreite der Deckeneinwirkung", "actions", "🔁️", "CharacteristicAction", crate="slab-actions"),
    L(S, 1, "delta_tau_k_pa", "f64", "change-slab-action-delta-tau-pa", "pa", "fatigue shear stress range of the slab action", "Schubspannungsschwingbreite der Deckeneinwirkung", "actions", "🔁️", "CharacteristicAction", crate="slab-actions"),
L(B, 0, "adjacent_span_m", "f64", "change-beam-adjacent-span-m", "m", "adjacent span length for the equivalent span and statics (0 = equal to the span)", "Länge des Nachbarfelds für Ersatzstützweite und Statik (0 = gleich der Stützweite)", "beam", "↔️", CB, crate="beam-design"),
L(B, 0, "patch_bearing_length_m", "f64", "change-beam-patch-bearing-length-m", "m", "patch load bearing length s_s", "Lasteinleitungslänge s_s der Einzellast", "beam", "📍️", CB, crate="beam-design"),
L(B, 0, "bridge_lanes", "u32", "change-beam-bridge-lanes", "count", "notional lanes loading the girder (0 = from the girder spacing)", "rechnerische Fahrstreifen am Träger (0 = aus dem Trägerabstand)", "beam", "🛣️", CB, crate="beam-design"),
L(C, 0, "introduced_force_n", "f64", "change-column-introduced-force-n", "n", "force introduced into the concrete (0 = from the actions)", "in den Beton eingeleitete Kraft (0 = aus den Einwirkungen)", "column", "⬇️", CC, crate="column-design"),
L(B, 0, "cracked_length_fraction", "f64", "change-beam-cracked-length-fraction", "ratio", "cracked length fraction of each span at internal supports, cl. 5.4.2.3 (0 = uncracked analysis)", "gerissener Längenanteil je Feld an Innenstützen nach 5.4.2.3 (0 = ungerissene Berechnung)", "beam", "〰️", CB, crate="beam-design"),
L(C, 0, "section_height_m", "f64", "change-column-section-height-m", "m", "height of the encased I-section (0 = equivalent section from the area)", "Höhe des einbetonierten I-Profils (0 = Ersatzquerschnitt aus der Fläche)", "column", "↕️", CC, crate="column-design"),
L(C, 0, "section_width_m", "f64", "change-column-section-width-m", "m", "flange width of the encased I-section", "Flanschbreite des einbetonierten I-Profils", "column", "↔️", CC, crate="column-design"),
L(C, 0, "section_tw_m", "f64", "change-column-section-tw-m", "m", "web thickness of the encased I-section", "Stegdicke des einbetonierten I-Profils", "column", "▮", CC, crate="column-design"),
L(C, 0, "section_tf_m", "f64", "change-column-section-tf-m", "m", "flange thickness of the encased I-section", "Flanschdicke des einbetonierten I-Profils", "column", "▬", CC, crate="column-design"),
L(C, 0, "rebar_cover_m", "f64", "change-column-rebar-cover-m", "m", "axis distance of the longitudinal bars from the concrete surface (0 = 0.05 m)", "Achsabstand der Längsbewehrung von der Betonoberfläche (0 = 0,05 m)", "column", "🔘", CC, crate="column-design"),
L(B, 0, "stud_transverse_spacing_m", "f64", "change-beam-stud-cross-spacing-m", "m", "transverse centre-to-centre spacing of the stud rows (0 = not declared)", "Bolzenquerabstand der Bolzenreihen (0 = nicht angegeben)", "beam", "↔️", CB, crate="beam-design"),
L(B, 0, "stud_edge_distance_m", "f64", "change-beam-stud-edge-distance-m", "m", "distance of the stud edge from the flange edge (0 = not declared)", "Randabstand der Bolzen vom Flanschrand (0 = nicht angegeben)", "beam", "📏️", CB, crate="beam-design"),
L(B, 0, "transverse_as_top_m2_per_m", "f64", "change-beam-transverse-as-top", "m2pm", "top layer transverse reinforcement of the deck slab per metre of girder", "obere Querbewehrung je Meter Brücke", "beam", "🕸️", CB, crate="beam-design"),
L(B, 0, "end_plate_width_m", "f64", "change-beam-end-plate-width-m", "m", "width of the girder end plate (0 = no end plate)", "Breite der Endplatte (0 = keine Endplatte)", "beam", "⬜️", CB, crate="beam-design"),
L(B, 0, "end_plate_height_m", "f64", "change-beam-end-plate-height-m", "m", "height of the girder end plate", "Höhe der Endplatte", "beam", "↕️", CB, crate="beam-design"),
L(B, 0, "end_zone_length_m", "f64", "change-beam-end-zone-length-m", "m", "length of the end zone over which the studs act on the end plate", "Länge des Endbereichs der Bolzengruppe", "beam", "➡️", CB, crate="beam-design"),
]

CRATES = {
    "column-design": ("🔗️column-design", "EN 1994 column design input mutation leaves"),
    "beam-design": ("🧭️beam-design", "EN 1994 beam design input mutation leaves"),
    "beam-fields": ("🌉️beam-fields", "EN 1994 beam field mutation leaves"),
    "beam-steel": ("🔩️beam-steel", "EN 1994 beam steel section mutation leaves"),
    "beam-sheeting": ("🏗️beam-sheeting", "EN 1994 beam sheeting and stud mutation leaves"),
    "beam-actions": ("🏋️beam-actions", "EN 1994 beam action mutation leaves"),
    "column-section": ("🏛️column-section", "EN 1994 column section mutation leaves"),
    "column-material": ("🧪️column-material", "EN 1994 column material and action mutation leaves"),
    "slab-fields": ("🧱️slab-fields", "EN 1994 slab field mutation leaves"),
    "slab-sheeting": ("🪟️slab-sheeting", "EN 1994 slab sheeting mutation leaves"),
    "slab-actions": ("🏋️slab-actions", "EN 1994 slab action mutation leaves"),
}

ENTITY_PATCH_ORDER = {
    "beam": ["span_m", "spacing_m", "support", "construction", "steel", "steel_height_m", "steel_width_m", "steel_tw_m", "steel_tf_m", "steel_a_m2", "steel_w_pl_y_m3", "steel_i_y_m4", "steel_a_v_m2", "steel_r_m", "slab_thickness_m", "concrete_f_ck_pa", "concrete_e_cm_pa", "sheeting_profile", "sheeting_height_m", "sheeting_rib_width_m", "sheeting_thickness_m", "sheeting_ribs_parallel_to_beam", "studs_diameter_m", "studs_height_m", "studs_f_u_pa", "studs_count_per_rib", "studs_spacing_m", "studs_total_count", "transverse_as_m2_per_m", "ltb_length_m", "as_hogging_m2_per_m", "bar_spacing_m", "wk_limit_m", "n_cycles", "adjacent_span_m", "patch_bearing_length_m", "bridge_lanes", "cracked_length_fraction", "stud_transverse_spacing_m", "stud_edge_distance_m", "transverse_as_top_m2_per_m", "end_plate_width_m", "end_plate_height_m", "end_zone_length_m"],
    "column": ["kind", "length_m", "outer_size_m", "wall_thickness_m", "steel_a_m2", "steel_f_y_pa", "concrete_a_m2", "concrete_f_ck_pa", "reinforcement_as_m2", "reinforcement_f_yk_pa", "i_m4", "buckling_curve", "introduced_force_n", "section_height_m", "section_width_m", "section_tw_m", "section_tf_m", "rebar_cover_m"],
    "slab": ["span_m", "support", "sheeting_profile", "sheeting_height_m", "sheeting_rib_width_m", "sheeting_thickness_m", "sheeting_ribs_parallel_to_beam", "concrete_thickness_m", "f_ck_pa", "m_factor", "k_factor", "as_m2_per_m"],
}
ACTION_PATCH_ORDER = {
    "beam": ["kind", "category", "stage", "q_area_pa", "f_k_n", "delta_sigma_k_pa", "delta_tau_k_pa"],
    "column": ["kind", "category", "stage", "n_k_n", "m_k_nm"],
    "slab": ["kind", "category", "stage", "q_area_pa", "f_k_n", "delta_sigma_k_pa", "delta_tau_k_pa"],
}
EXISTING_PATCH = {
    ("beam", 0): {"span_m": ("span_m", "f64", "CompositeBeam"), "slab_thickness_m": ("slab_thickness_m", "f64", "CompositeBeam"), "construction": ("construction", "String", "CompositeBeam"), "transverse_as_m2_per_m": ("transverse_as_m2_per_m", "f64", "CompositeBeam"), "studs_diameter_m": ("studs.diameter_m", "f64", "HeadedStuds"), "studs_f_u_pa": ("studs.f_u_pa", "f64", "HeadedStuds"), "studs_spacing_m": ("studs.spacing_m", "f64", "HeadedStuds"), "studs_total_count": ("studs.total_count", "u32", "HeadedStuds")},
    ("column", 0): {"kind": ("kind", "String", "CompositeColumn")},
    ("slab", 0): {"concrete_thickness_m": ("concrete_thickness_m", "f64", "CompositeSlab")},
    ("beam", 1): {"q_area_pa": ("q_area_pa", "f64", "CharacteristicAction")},
    ("column", 1): {"n_k_n": ("n_k_n", "f64", "ColumnAction")},
    ("slab", 1): {"q_area_pa": ("q_area_pa", "f64", "CharacteristicAction")},
}


def patch_fields(ent, nest):
    order = (ACTION_PATCH_ORDER if nest else ENTITY_PATCH_ORDER)[ent]
    known = {l["patch"]: (l["access"], l["ty"], l["container"]) for l in LEAVES if l["ent"] == ent and l["nest"] == nest and l["patch"] != "steel"}
    known.update(EXISTING_PATCH[(ent, nest)])
    out = []
    for name in order:
        if name == "steel":
            out.append((name, "steel", "SteelSection", "whole"))
        else:
            access, ty, container = known[name]
            out.append((name, access, ty, container))
    return out


def rust_ty(ty):
    return {"f64": "f64", "u32": "u32", "bool": "bool", "String": "String"}[ty]


def clone(ty):
    return ".clone()" if ty == "String" else ""


for l in LEAVES:
    l["module"] = l["kind"].replace("-", "_")
    l["record"] = pascal(l["kind"])
    l["dirname"] = l["emoji"] + l["kind"]
    l["payload"] = "new_" + l["access"].split(".")[-1] if l["patch"] != "steel" else "new_designation"
    l["payload_json"] = camel(l["payload"])
    l["tag_name"] = "ChangeBeamSteelDesignation"
    assert len(ANY + "/🧪️tests/🧫️fixtures/🧬️mutations/" + l["dirname"] + "/✅apply/🔣️.before.json") < 256


def patch_apply(access, ty, patch_name):
    if patch_name == "steel":
        return "        if let Some(value) = &self.steel {\n            row.steel = value.clone();\n        }\n"
    return "        if let Some(value) = &self.%s {\n            row.%s = value.clone();\n        }\n" % (patch_name, access)


def patch_struct_block(ent, nest):
    rows = ENT[ent]["rows"]
    fields = patch_fields(ent, nest)
    model = ENT[ent]["model"] if not nest else ("ColumnAction" if ent == "column" else "CharacteristicAction")
    tyname = "En1994%s%sPatch" % (rows, "Actions" if nest else "")
    doc = "/// 🩹️ Sparse patch of the `%s` row at base position `index`.\n" % ("actions" if nest else ENT[ent]["coll"])
    head = "#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]\n#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n#[cfg_attr(test, serde(rename_all = \"camelCase\", default))]\n#[value(rename_all = \"camelCase\", default)]\n"
    s = doc + head + "pub struct %s {\n    pub index: usize,\n" % tyname
    for f in fields:
        if f[3] == "whole":
            s += "    pub steel: Option<semio_s_artifact_norm_en1994_model::SteelSection>,\n"
        else:
            s += "    pub %s: Option<%s>,\n" % (f[0], rust_ty(f[2]))
    if not nest:
        s += "    pub actions: Option<En1994%sActionsRows>,\n" % rows
    s += "}\n\n"
    s += "impl %s {\n" % tyname
    s += "    pub(super) fn apply_to_row(&self, row: &mut semio_s_artifact_norm_en1994_model::%s) -> Result<(), protocol::MutationApplyError> {\n" % model
    for f in fields:
        s += patch_apply(f[1], f[2], f[0]) if f[3] != "whole" else patch_apply("", "", "steel")
    if not nest:
        s += "        if let Some(rows) = &self.actions {\n            row.actions = rows.apply_rows(&row.actions).map_err(|error| error.under([\"actions\"]))?;\n        }\n"
    s += "        Ok(())\n    }\n\n"
    s += "    pub(super) fn merge(&mut self, other: Self) {\n"
    for f in fields:
        if f[3] == "whole":
            s += "        if other.steel.is_some() {\n            self.steel = other.steel;\n" + "".join("            self.%s = None;\n" % g[0] for g in fields if g[0].startswith("steel_")) + "        }\n"
        else:
            s += "        if other.%s.is_some() {\n            self.%s = other.%s;\n        }\n" % (f[0], f[0], f[0])
    if not nest:
        s += "        if let Some(theirs) = other.actions {\n            match self.actions.as_mut() {\n                Some(mine) => mine.absorb_rows(theirs),\n                None => self.actions = Some(theirs),\n            }\n        }\n        self.actions = self.actions.take().filter(|rows| !rows.is_empty());\n"
    s += "    }\n\n"
    s += "    pub(super) fn inverse_from_row(&self, row: &semio_s_artifact_norm_en1994_model::%s, index: usize) -> Self {\n        Self {\n            index,\n" % model
    for f in fields:
        if f[3] == "whole":
            s += "            steel: self.steel.as_ref().map(|_| row.steel.clone()),\n"
        else:
            s += "            %s: self.%s.as_ref().map(|_| row.%s.clone()),\n" % (f[0], f[0], f[1])
    if not nest:
        s += "            actions: self.actions.as_ref().map(|rows| rows.inverse_rows(&row.actions)).filter(|rows| !rows.is_empty()),\n"
    s += "        }\n    }\n}\n\n"
    return s


def update_diff_rust():
    for ent, folder in (("beam", "🏗️beam-rows"), ("column", "🏛️column-rows"), ("slab", "🧱️slab-rows")):
        path = DIFF + "/" + folder + "/🦀️.rs"
        text = rd(path)
        rows = ENT[ent]["rows"]
        coll = ENT[ent]["coll"]
        a0 = text.index("/// 🩹️ Sparse patch of the `actions` row")
        a1 = text.index("/// 🔺️ Positional diff of `actions` rows")
        text = text[:a0] + patch_struct_block(ent, 1) + text[a1:]
        e0 = text.index("/// 🩹️ Sparse patch of the `%s` row" % coll)
        e1 = text.index("/// 📌️ A `%s` row inserted" % coll)
        text = text[:e0] + patch_struct_block(ent, 0) + text[e1:]
        wr(path, text)


def camel_type(ty):
    return {"f64": "number", "u32": "number", "bool": "boolean", "String": "string"}[ty]


def update_diff_schema():
    path = DIFF + "/🔣️.json"
    d = json.loads(rd(path))
    defs = d["$defs"]
    for ent in ENT:
        rows = ENT[ent]["rows"]
        for nest in (1, 0):
            name = "En1994%s%sPatch" % (rows, "Actions" if nest else "")
            props = {"index": {"type": "integer", "minimum": 0}}
            for f in patch_fields(ent, nest):
                if f[3] == "whole":
                    props["steel"] = {"anyOf": [{"$ref": SNAPSHOT_REF + "SteelSection"}, {"type": "null"}]}
                else:
                    props[camel(f[0])] = {"anyOf": [{"$ref": SNAPSHOT_REF + "%s/properties/%s" % (f[3], camel(f[1].split(".")[-1]))}, {"type": "null"}]}
            if not nest:
                props["actions"] = {"anyOf": [{"$ref": "#/$defs/En1994%sActionsRows" % rows}, {"type": "null"}]}
            defs[name] = {"type": "object", "additionalProperties": False, "required": list(props), "properties": props}
    wr(path, dumps(d))

    path = DIFF + "/🔗️.graphql"
    text = rd(path)
    gtype = {"f64": "Float", "u32": "Int", "bool": "Boolean", "String": "String"}
    for ent in ENT:
        rows = ENT[ent]["rows"]
        for nest in (1, 0):
            name = "En1994%s%sPatch" % (rows, "Actions" if nest else "")
            body = "type %s {\n  index: Int!\n" % name
            for f in patch_fields(ent, nest):
                body += "  steel: SteelSection\n" if f[3] == "whole" else "  %s: %s\n" % (camel(f[0]), gtype[f[2]])
            if not nest:
                body += "  actions: En1994%sActionsRows\n" % rows
            body += "}\n"
            text = re.sub(r"type %s \{.*?\n\}\n" % name, lambda m: body, text, count=1, flags=re.S)
    wr(path, text)

    path = DIFF + "/🛰️.proto"
    text = rd(path)
    ptype = {"f64": "double", "u32": "uint32", "bool": "bool", "String": "string"}
    for ent in ENT:
        rows = ENT[ent]["rows"]
        for nest in (1, 0):
            name = "En1994%s%sPatch" % (rows, "Actions" if nest else "")
            body = "message %s {\n  uint64 index = 1;\n" % name
            n = 2
            for f in patch_fields(ent, nest):
                if f[3] != "whole":
                    body += "  optional %s %s = %d;\n" % (ptype[f[2]], f[0], n)
                n += 1
            if not nest:
                body += "  optional En1994%sActionsRows actions = %d;\n" % (rows, n)
            body += "}\n"
            text = re.sub(r"message %s \{.*?\n\}\n" % name, lambda m: body, text, count=1, flags=re.S)
    wr(path, text)

    path = DIFF + "/🟦️.ts"
    text = rd(path)
    for ent in ENT:
        rows = ENT[ent]["rows"]
        for nest in (1, 0):
            name = "En1994%s%sPatch" % (rows, "Actions" if nest else "")
            body = "export interface %s {\n  index: number;\n" % name
            reader = "parse%s: NormWireReader<%s> = normWireObject<%s>({ index: normWireRequired(normWireRange(normWireInteger, {\"minimum\":0}))" % (name, name, name)
            readers = {"f64": "normWireNumber", "u32": "normWireRange(normWireInteger, {\"minimum\":0,\"maximum\":4294967295})", "bool": "normWireBoolean", "String": "normWireString"}
            for f in patch_fields(ent, nest):
                if f[3] == "whole":
                    body += "  steel: SteelSection | null;\n"
                    reader += ", steel: normWireRequired(normWireNullable(parseSteelSection))"
                else:
                    body += "  %s: %s | null;\n" % (camel(f[0]), camel_type(f[2]))
                    reader += ", %s: normWireRequired(normWireNullable(%s))" % (camel(f[0]), readers[f[2]])
            if not nest:
                body += "  actions: En1994%sActionsRows | null;\n" % rows
                reader += ", actions: normWireRequired(normWireNullable(normWireRef(() => parseEn1994%sActionsRows)))" % rows
            body += "}\n"
            reader += " });"
            text = re.sub(r"export interface %s \{.*?\n\}\n" % name, lambda m: body, text, count=1, flags=re.S)
            text = re.sub(r"export const parse%s: .*?\}\);" % name, lambda m: "export const " + reader, text, count=1, flags=re.S)
    if "normWireBoolean" not in text.split("\n")[3]:
        text = rep(text, "normWireArray, normWireDefault,", "normWireArray, normWireBoolean, normWireDefault,")
    if "parseSteelSection" not in text.split("\n")[4]:
        text = rep(text, "import { type AnnexChoice, type CompositeBeam, type CompositeColumn, type CompositeSlab, parseAnnexChoice, parseCompositeBeam, parseCompositeColumn, parseCompositeSlab }", "import { type AnnexChoice, type CompositeBeam, type CompositeColumn, type CompositeSlab, type SteelSection, parseAnnexChoice, parseCompositeBeam, parseCompositeColumn, parseCompositeSlab, parseSteelSection }")
    wr(path, text)


def normalise_diff_json(diff):
    for ent in ENT:
        rows = diff.get(ENT[ent]["coll"])
        if not rows:
            continue
        for patch in rows["modified"]:
            new = {"index": patch["index"]}
            for f in patch_fields(ent, 0):
                new[camel(f[0])] = patch.get(camel(f[0]))
            actions = patch.get("actions")
            if actions:
                for ap in actions["modified"]:
                    nap = {"index": ap["index"]}
                    for f in patch_fields(ent, 1):
                        nap[camel(f[0])] = ap.get(camel(f[0]))
                    ap.clear()
                    ap.update(nap)
            new["actions"] = actions
            patch.clear()
            patch.update(new)
    return diff


def catalogue_table():
    text = rd(ANY + "/🧬️schema/📸️snapshot/🦀️.rs")
    table = {}
    pattern = re.compile(r'"(HEB\d+)" => Some\(Self \{\s*designation: "HEB\d+"\.into\(\), height_m: ([\d.]+), width_m: ([\d.]+), tw_m: ([\d.]+), tf_m: ([\d.]+),\s*a_m2: ([\d.]+), w_pl_y_m3: ([\d.e-]+), i_y_m4: ([\d.e-]+), a_v_m2: ([\d.]+),')
    for m in pattern.finditer(text):
        v = [float(x) for x in m.groups()[1:]]
        table[m.group(1)] = dict(designation=m.group(1), heightM=v[0], widthM=v[1], twM=v[2], tfM=v[3], aM2=v[4], wPlYM3=v[5], iYM4=v[6], aVM2=v[7])
    return table


CATALOGUE = catalogue_table()
assert "HEB320" in CATALOGUE


def schema_ui(l):
    ui = {"widget": "stepper", "role": "value", "label": {"en": l["en"][:1].upper() + l["en"][1:], "de": l["de"]}}
    if l["ty"] == "String":
        ui["widget"] = "text"
    elif l["ty"] == "bool":
        ui["widget"] = "toggle"
    else:
        ui.update(UNITS[l["unit"]])
    ui["group"] = l["group"]
    return ui


def leaf_schema(l, existing):
    ent = ENT[l["ent"]]
    props = {"index": copy.deepcopy(existing[l["ent"]]["index"])}
    required = ["index"]
    order = 20
    if l["nest"]:
        props["actionIndex"] = copy.deepcopy(existing["action"])
        required.append("actionIndex")
        order = 30
    ui = schema_ui(l)
    ui["order"] = order
    if l["ty"] == "f64":
        prop = {"type": "number"}
    elif l["ty"] == "u32":
        prop = {"type": "integer"}
    elif l["ty"] == "bool":
        prop = {"type": "boolean"}
    else:
        prop = {"type": "string"}
    if l["ty"] in ("f64", "u32") and l["minimum"]:
        prop["minimum"] = 0
    prop["x-semio-ui"] = ui
    props[l["payload_json"]] = prop
    required.append(l["payload_json"])
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": SCHEMA_ID % l["kind"], "title": l["record"], "type": "object", "additionalProperties": False, "required": required, "properties": props}


def schema_ts(l):
    ent = ENT[l["ent"]]
    imports = {"normWireInteger", "normWireObject", "normWireRange", "normWireRequired", "NormWireReader"}
    members = "  index: number;\n" + ("  actionIndex: number;\n" if l["nest"] else "")
    reader = "index: normWireRequired(normWireRange(normWireInteger, {\"minimum\":0}))" + (", actionIndex: normWireRequired(normWireRange(normWireInteger, {\"minimum\":0}))" if l["nest"] else "")
    ty = l["ty"]
    if ty == "f64":
        value = "normWireRange(normWireNumber, {\"minimum\":0})" if l["minimum"] else "normWireNumber"
        imports.add("normWireNumber")
    elif ty == "u32":
        value = "normWireRange(normWireInteger, {\"minimum\":0})"
    elif ty == "bool":
        value = "normWireBoolean"
        imports.add("normWireBoolean")
    else:
        value = "normWireString"
        imports.add("normWireString")
    members += "  %s: %s;\n" % (l["payload_json"], camel_type(ty))
    reader += ", %s: normWireRequired(%s)" % (l["payload_json"], value)
    ordered = sorted(imports, key=lambda x: x.lower())
    imp = ", ".join(("type " + x) if x == "NormWireReader" else x for x in ordered)
    return ('/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`\n * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema\'s bounds stay Ajv\'s.\n * @see ./🔣️.json */\nimport { %s } from "@semio-tech/norm-js/registry/contract";\n\nexport interface %s {\n%s}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });\n') % (l["emoji"], l["kind"], l["record"], imp, l["record"], members, l["record"], l["record"], l["record"], reader)


def catalog_json(l, tag):
    return {"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + l["dirname"], "semanticKind": l["kind"], "displayName": l["record"], "emoji": l["emoji"], "aggregateVariant": l["record"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}


STEEL_LEAVES = [l for l in LEAVES if l["kind"].startswith("change-beam-steel-") and l["kind"] != "change-beam-steel-designation"]


def extra_bounds(l, crate_module):
    if l["kind"] != "change-beam-steel-designation":
        return ""
    return "".join(" + From<crate::%s::%s>" % (s["module"], s["record"]) for s in STEEL_LEAVES)


def leaf_rs(l):
    ent = ENT[l["ent"]]
    fields = "    pub index: usize,\n" + ("    pub action_index: usize,\n" if l["nest"] else "") + "    pub %s: %s,\n" % (l["payload"], rust_ty(l["ty"]))
    extra = extra_bounds(l, l["module"])
    label_en = "Change " + l["en"]
    label_de = l["de"] + " ändern"
    return '''//! %(emoji)s `%(kind)s` mutation leaf.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct %(record)s {
%(fields)s}

impl<Op> protocol::MutationKind<En1994Snapshot, Op> for %(record)s where Op: protocol::Mutation<En1994Snapshot, Diff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::%(module)s::%(record)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "%(entity)s", kind: "%(kind)s", record: "Changed%(rest)s" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("%(en)s", "%(de)s")
    }
}

impl<Op> protocol::ColdMutationKind<En1994Snapshot, Op> for %(record)s where Op: protocol::ColdMutation<En1994Snapshot, ReferenceDiff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::%(module)s::%(record)s>%(extra)s {
    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1994Snapshot>>::ReferenceDiff> {
        crate::%(module)s::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::%(module)s::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
''' % dict(emoji=l["emoji"], kind=l["kind"], record=l["record"], fields=fields, module=l["module"], entity=l["kind"][len("change-"):], rest=l["record"][len("Change"):], en=label_en, de=label_de, extra=extra)


def eq_expr(l, current, new):
    if l["ty"] == "f64":
        return "(%s - %s).abs() < f64::EPSILON" % (current, new)
    return "%s == %s" % (current, new)


def diff_rs(l):
    ent = ENT[l["ent"]]
    word = ent["word"]
    rows = ent["rows"]
    coll = ent["coll"]
    if l["patch"] == "steel":
        return '''//! 🔺️ Diff for `%(kind)s`.
use crate::%(module)s::%(record)s;
use semio_s_artifact_norm_en1994_model::{En1994Snapshot, SteelSection};
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &%(record)s, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    let next = SteelSection::from_catalogue(&payload.new_designation).unwrap_or_else(|| SteelSection { designation: payload.new_designation.clone(), ..beam.steel.clone() });
    if beam.steel == next {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows { modified: vec![En1994BeamsPatch { index: payload.index, steel: Some(next), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
''' % dict(kind=l["kind"], module=l["module"], record=l["record"])
    imports = "En1994Diff, En1994%sRows, En1994%sPatch" % (rows, rows) + (", En1994%sActionsRows, En1994%sActionsPatch" % (rows, rows) if l["nest"] else "")
    s = "//! 🔺️ Diff for `%s`.\nuse crate::%s::%s;\nuse semio_s_artifact_norm_en1994_model::En1994Snapshot;\nuse semio_s_artifact_norm_en1994_diff::{%s};\n\n" % (l["kind"], l["module"], l["record"], imports)
    s += "pub fn diff(payload: &%s, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {\n" % l["record"]
    if l["ty"] == "f64":
        s += "    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal(\"mutation.invariant\", \"must be finite\", [payload.index.to_string()]);\n    }\n" % l["payload"]
    s += "    let Some(%s) = base.%s.get(payload.index) else {\n        return protocol::MutationOutcome::error(\"mutation.target-missing\", \"%s missing\", [payload.index.to_string()]);\n    };\n" % (word, coll, word)
    holder = word
    if l["nest"]:
        s += "    let Some(action) = %s.actions.get(payload.action_index) else {\n        return protocol::MutationOutcome::error(\"mutation.target-missing\", \"action missing\", [payload.action_index.to_string()]);\n    };\n" % word
        holder = "action"
    current = "%s.%s" % (holder, l["access"])
    s += "    if %s {\n        return protocol::MutationOutcome::empty().warning(\"mutation.no-op\", \"unchanged\");\n    }\n" % eq_expr(l, current, "payload." + l["payload"])
    value = "payload.%s%s" % (l["payload"], clone(l["ty"]))
    if l["nest"]:
        s += "    protocol::MutationOutcome::new(En1994Diff {\n        %s: Some(En1994%sRows {\n            modified: vec![En1994%sPatch {\n                index: payload.index,\n                actions: Some(En1994%sActionsRows { modified: vec![En1994%sActionsPatch { index: payload.action_index, %s: Some(%s), ..Default::default() }] }),\n                ..Default::default()\n            }],\n            ..Default::default()\n        }),\n        ..Default::default()\n    })\n}\n" % (coll, rows, rows, rows, rows, l["patch"], value)
    else:
        s += "    protocol::MutationOutcome::new(En1994Diff {\n        %s: Some(En1994%sRows { modified: vec![En1994%sPatch { index: payload.index, %s: Some(%s), ..Default::default() }], ..Default::default() }),\n        ..Default::default()\n    })\n}\n" % (coll, rows, rows, l["patch"], value)
    return s


def inverse_rs(l):
    ent = ENT[l["ent"]]
    word = ent["word"]
    coll = ent["coll"]
    if l["patch"] == "steel":
        restores = "".join("        Op::from(%s { index: payload.index, %s: beam.steel.%s }),\n" % (s["record"], s["payload"], s["access"].split(".")[-1]) for s in STEEL_LEAVES)
        bounds = "Op: From<crate::%s::%s>%s" % (l["module"], l["record"], "".join(" + From<crate::%s::%s>" % (s["module"], s["record"]) for s in STEEL_LEAVES))
        return '''//! ↩️ Inverse for `%(kind)s`: restores the designation and every geometry leaf, so a custom section returns exactly.
use crate::%(module)s::%(record)s;
%(uses)suse semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &%(record)s, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where %(bounds)s {
    let Some(beam) = base.beams.get(payload.index) else { return Ok(Vec::new()); };
    Ok(vec![
        Op::from(%(record)s { index: payload.index, new_designation: beam.steel.designation.clone() }),
%(restores)s    ])
}
''' % dict(kind=l["kind"], module=l["module"], record=l["record"], bounds=bounds, restores=restores, uses="".join("use crate::%s::%s;\n" % (s["module"], s["record"]) for s in STEEL_LEAVES))
    s = "//! ↩️ Inverse for `%s`.\nuse crate::%s::%s;\nuse semio_s_artifact_norm_en1994_model::En1994Snapshot;\n" % (l["kind"], l["module"], l["record"])
    s += "pub fn inverse<Op>(payload: &%s, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%s::%s> {\n" % (l["record"], l["module"], l["record"])
    s += "    let Some(%s) = base.%s.get(payload.index) else { return Ok(Vec::new()); };\n" % (word, coll)
    holder = word
    selectors = "index: payload.index, "
    if l["nest"]:
        s += "    let Some(action) = %s.actions.get(payload.action_index) else { return Ok(Vec::new()); };\n" % word
        holder = "action"
        selectors += "action_index: payload.action_index, "
    s += "    Ok(vec![Op::from(%s { %s%s: %s.%s%s })])\n}\n" % (l["record"], selectors, l["payload"], holder, l["access"], clone(l["ty"]))
    return s


def crate_cargo(name, description):
    template = rd(MUT + "/📌️studs/📦️packages/🦀️rust/Cargo.toml")
    template = rep(template, "semio-s-artifact-norm-en1994-mut-studs", CRATE_PREFIX + name)
    template = rep(template, "EN 1994 studs mutation leaves", description)
    return template


def crate_lib(name, leaves):
    head = rd(MUT + "/🌉️beams/🦀️.rs").split("//#region 🪢️Mounts")[0]
    head = head.replace("//! 🌉️ EN 1994 beams mutation leaves.", "//! %s %s." % (CRATES[name][0][:2], CRATES[name][1]))
    mounts = "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (l["dirname"], l["module"]) for l in sorted(leaves, key=lambda x: x["module"]))
    return head + "//#region 🪢️Mounts\n" + mounts + "//#endregion 🪢️Mounts\n"


def sample_value(l, old):
    if l["ty"] == "f64":
        if not old:
            return 1500.0 if "n" == l["unit"] or l["unit"] == "nm" else 2.5e6 if l["unit"] == "pa" else 0.5
        return float("%.6g" % (old * 1.25))
    if l["ty"] == "u32":
        return old + 1
    if l["ty"] == "bool":
        return not old
    return l["sample"] if l["sample"] != old else old + "-2"


def get_target(snap, l, idx, action):
    row = snap[ENT[l["ent"]]["coll"]][idx]
    if l["nest"]:
        row = row["actions"][action]
    return row


def build_fixture(l, tag):
    ent = ENT[l["ent"]]
    fx = ent["action_fx"] if l["nest"] else ent["base_fx"]
    before = json.loads(rd(FIX + "/" + fx + "/✅apply/🔣️.before.json"))
    after = copy.deepcopy(before)
    row = get_target(after, l, 0, 0)
    parts = [camel(p) for p in l["access"].split(".")]
    holder = row
    for p in parts[:-1]:
        holder = holder[p]
    old = holder[parts[-1]]
    if l["patch"] == "steel":
        new = "HEB320"
        row["steel"] = copy.deepcopy(CATALOGUE[new])
        patch_value = copy.deepcopy(CATALOGUE[new])
    else:
        new = sample_value(l, old)
        holder[parts[-1]] = new
        patch_value = new
    mutation = {l["record"]: {"index": 0}}
    if l["nest"]:
        mutation[l["record"]]["actionIndex"] = 0
    mutation[l["record"]][l["payload_json"]] = new
    diff = {"annex": None, "structureKind": None, "steelFYPa": None, "beams": None, "columns": None, "slabs": None, "fireRating": None, "insulationThicknessM": None, "fatigueDetail": None}
    patch = {"index": 0}
    for f in patch_fields(l["ent"], 0):
        patch[camel(f[0])] = None
    patch["actions"] = None
    if l["nest"]:
        ap = {"index": 0}
        for f in patch_fields(l["ent"], 1):
            ap[camel(f[0])] = None
        ap[camel(l["patch"])] = patch_value
        patch["actions"] = {"modified": [ap]}
    else:
        patch[camel(l["patch"])] = patch_value
    diff[ent["coll"]] = {"removed": [], "inserted": [], "modified": [patch]}
    return before, mutation, after, diff


def read_existing_schemas():
    out = {}
    for name, ent in ENT.items():
        d = json.loads(rd(MUT + "/" + ent["schema_fx"] + "/🧬️schema/🔣️.json"))
        out[name] = {"index": d["properties"]["index"]}
    action = json.loads(rd(MUT + "/" + ENT["beam"]["action_fx"] + "/🧬️schema/🔣️.json"))
    out["action"] = action["properties"]["actionIndex"]
    return out


def edit_rule_line(l):
    ent = ENT[l["ent"]]
    path = "%s[].%s%s" % (ent["coll"], "actions[]." if l["nest"] else "", ".".join(camel(p) for p in l["access"].split(".")))
    selectors = 'SelectorKey::Index("index")' + (', SelectorKey::Index("actionIndex")' if l["nest"] else "")
    return '        SetFieldRule { path: "%s", kind: "%s", selectors: &[%s], value: "%s" },\n' % (path, l["kind"], selectors, l["payload_json"])


def testkit_line(l):
    fields = "index: 0, " + ("action_index: 0, " if l["nest"] else "")
    ty = l["ty"]
    if l["patch"] == "steel":
        value = '"HEB320".into()'
    elif ty == "f64":
        value = repr(float(sample_value(l, 1.0))) if False else "1.5"
    elif ty == "u32":
        value = "2"
    elif ty == "bool":
        value = "true"
    else:
        value = '"%s".into()' % l["sample"]
    return "        En1994Mutation::%s(%s::%s { %s%s: %s }),\n" % (l["record"], l["module"], l["record"], fields, l["payload"], value)


def register_workspace(todo_crates):
    cargo = rd(ART + "/Cargo.toml")
    for name in todo_crates:
        member = '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/📦️packages/🦀️rust",\n' % CRATES[name][0]
        if member not in cargo:
            cargo = rep(cargo, '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust",\n', member + '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust",\n')
        dep = "[workspace.dependencies.%s%s]\npath = \"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/📦️packages/🦀️rust\"\n\n" % (CRATE_PREFIX, name, CRATES[name][0])
        if dep not in cargo:
            anchor = "[workspace.dependencies.semio-s-artifact-norm-en1994-mutations]"
            cargo = rep(cargo, anchor, dep + anchor)
    wr(ART + "/Cargo.toml", cargo)
    script_path = ART + "/📦️packages/🦀️rust/📜️script.ts"
    script = rd(script_path)
    for name in todo_crates:
        entry = '"%s%s",' % (CRATE_PREFIX, name)
        if entry not in script:
            script = rep(script, '"%sbeams",' % CRATE_PREFIX, entry + '"%sbeams",' % CRATE_PREFIX)
    wr(script_path, script)
    for consumer in (MUT + "/📦️packages/🦀️rust/Cargo.toml", ANY + "/🚪️io/📦️packages/🦀️rust/Cargo.toml"):
        text = rd(consumer)
        for name in todo_crates:
            line = "%s%s = { workspace = true }\n" % (CRATE_PREFIX, name)
            if line not in text:
                text = rep(text, "%sbeams = { workspace = true }\n" % CRATE_PREFIX, line + "%sbeams = { workspace = true }\n" % CRATE_PREFIX)
        wr(consumer, text)
    owners = rd(OWNERS)
    data = json.loads(owners)
    names = {o["name"] for o in data["owners"]}
    for name in todo_crates:
        full = CRATE_PREFIX + name
        if full in names:
            continue
        root = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/📦️packages/🦀️rust" % CRATES[name][0]
        entry = {"name": full, "root": root, "manifest": root + "/Cargo.toml", "packages": []}
        index = next(i for i, o in enumerate(data["owners"]) if o["name"] == CRATE_PREFIX + "beams")
        data["owners"].insert(index, entry)
    text = json.dumps(data, indent=2, ensure_ascii=False) + "\n"
    wr(OWNERS, text if json.dumps(json.loads(owners), indent=2, ensure_ascii=False) + "\n" == owners else owners)
    if json.dumps(json.loads(owners), indent=2, ensure_ascii=False) + "\n" != owners:
        print("WARNING: build-owners.json does not round-trip; entries NOT added, add them by hand")


def main():
    root_rs = rd(MUT + "/🦀️.rs")
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', root_rs, re.M))
    todo = [l for l in LEAVES if l["kind"] not in existing]
    print("to generate:", len(todo))
    if os.environ.get("REGEN"):
        update_diff_rust()
        update_diff_schema()
        for path in sorted(os.popen("find '%s' -path '*/🔣️.diff.json' -not -path '*/target/*'" % FIX).read().split("\n")):
            if path:
                wr(path, dumps(normalise_diff_json(json.loads(rd(path)))))
        print("regenerated diff layer")
    if not todo:
        return
    existing_schemas = read_existing_schemas()
    proto_path = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    proto = rd(proto_path)
    tag = max(int(x) for x in re.findall(r"tag=(\d+)", proto)) + 1
    by_crate = {}
    for l in todo:
        l["tag"] = tag
        tag += 1
        by_crate.setdefault(l["crate"], []).append(l)
    for name, leaves in by_crate.items():
        folder = MUT + "/" + CRATES[name][0]
        wr(folder + "/📦️packages/🦀️rust/Cargo.toml", crate_cargo(name, CRATES[name][1]))
        wr(folder + "/🦀️.rs", crate_lib(name, leaves))
    for l in todo:
        d = MUT + "/" + l["dirname"]
        wr(d + "/🦀️.rs", leaf_rs(l))
        wr(d + "/🔺️diff/🦀️.rs", diff_rs(l))
        wr(d + "/↩️inverse/🦀️.rs", inverse_rs(l))
        wr(d + "/🧬️schema/🔣️.json", dumps(leaf_schema(l, existing_schemas)))
        wr(d + "/🧬️schema/🟦️.ts", schema_ts(l))
        wr(d + "/🔣️.json", dumps(catalog_json(l, l["tag"])))
        before, mutation, after, diff = build_fixture(l, l["tag"])
        fx = FIX + "/" + l["dirname"] + "/✅apply"
        wr(fx + "/🔣️.before.json", dumps(before))
        wr(fx + "/🔣️.mutation.json", dumps(mutation))
        wr(fx + "/🔣️.after.json", dumps(after))
        wr(fx + "/🔣️.outcome.json", dumps({"status": "applied"}))
        wr(fx + "/🔣️.diff.json", dumps(diff))
    update_diff_rust()
    update_diff_schema()
    for path in sorted(os.popen("find '%s' -path '*/🔣️.diff.json' -not -path '*/target/*'" % FIX).read().split("\n")):
        if path:
            wr(path, dumps(normalise_diff_json(json.loads(rd(path)))))
    register_workspace(list(by_crate))
    uses = "".join("use semio_s_artifact_norm_en1994_mut_%s::%s;\n" % (l["crate"].replace("-", "_"), l["module"]) for l in todo)
    s = rep(root_rs, "//#endregion 🔖️Leaves", uses + "//#endregion 🔖️Leaves")
    s = rep(s, "\n}\n\npub const KINDS", "\n" + "".join("    %s(%s::%s),\n" % (l["record"], l["module"], l["record"]) for l in todo).rstrip("\n") + "\n}\n\npub const KINDS")
    s = rep(s, "\n];\n//#endregion 🔖️Mutations", "\n" + "".join('    "%s",\n' % l["kind"] for l in todo).rstrip("\n") + "\n];\n//#endregion 🔖️Mutations")
    wr(MUT + "/🦀️.rs", s)
    p = MUT + "/🟦️.ts"
    s = rd(p)
    s = rep(s, "\nexport type En1994Mutation =", "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (l["record"], l["record"], l["dirname"]) for l in todo) + "\nexport type En1994Mutation =")
    s = rep(s, " };\n\nexport const parseEn1994Mutation", " }" + "".join("\n  | { %s: %s }" % (l["record"], l["record"]) for l in todo) + ";\n\nexport const parseEn1994Mutation")
    s = rep(s, "\n});", "\n" + "".join("  %s: parse%s,\n" % (l["record"], l["record"]) for l in todo).rstrip("\n") + "\n});")
    wr(p, s)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    for l in todo:
        j["oneOf"].append({"type": "object", "additionalProperties": False, "required": [l["record"]], "properties": {l["record"]: {"$ref": SCHEMA_ID % l["kind"]}}})
    wr(p, dumps(j))
    p = MUT + "/🧰️testkit/🦀️.rs"
    s = rd(p)
    s = rep(s, "    ]\n}", "".join(testkit_line(l) for l in todo) + "    ]\n}") if s.rstrip().endswith("]\n}") else rep(s, "    ]\n}\n", "".join(testkit_line(l) for l in todo) + "    ]\n}\n")
    wr(p, s)
    p = MUT + "/🧭️edit-rules/🦀️.rs"
    s = rd(p)
    s = rep(s, "    ],\n    insert_item:", "".join(edit_rule_line(l) for l in todo) + "    ],\n    insert_item:")
    wr(p, s)
    p = ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = rd(p)
    s = rep(s, "#[cfg(test)]\nuse semio_s_artifact_norm_en1994_mutations::demo_mutation_cases;", "".join("use semio_s_artifact_norm_en1994_mut_%s::%s;\n" % (l["crate"].replace("-", "_"), l["module"]) for l in todo) + "#[cfg(test)]\nuse semio_s_artifact_norm_en1994_mutations::demo_mutation_cases;")
    wr(p, s)
    wr(proto_path, proto.rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (l["kind"], l["tag"]) for l in todo))
    p = HARNESS + "/🦀️.rs"
    s = rd(p)
    s = rep(s, "\n];\n\n/// ⛔️ The refusal witnesses", "\n" + "".join('    "%s",\n' % l["kind"] for l in todo).rstrip("\n") + "\n];\n\n/// ⛔️ The refusal witnesses")
    arms = ""
    for l in todo:
        arms += '        "%s" => (\n' % l["kind"] + "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (l["dirname"], part) for part in ("before", "mutation", "after", "outcome")) + "        ),\n"
    s = rep(s, '        other => panic!("mutate-en1994-1: no committed fixture', arms + '        other => panic!("mutate-en1994-1: no committed fixture')
    wr(p, s)
    p = HARNESS + "/🐍️.py"
    s = rd(p)
    s = rep(s, "\n]\n\n#: 🧫️ The committed specification vectors", "\n" + "".join('    "%s",\n' % l["kind"] for l in todo).rstrip("\n") + "\n]\n\n#: 🧫️ The committed specification vectors")
    s = insert_after_last(s, r'^    "[a-z0-9-]+": \("[^"]+", "✅apply"\),\n', "".join('    "%s": ("%s", "✅apply"),\n' % (l["kind"], l["dirname"]) for l in todo))
    wr(p, s)
    p = HARNESS + "/🥒️.feature"
    s = rd(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (l["kind"], l["dirname"]) for l in todo)
    s = re.sub(r"((?:      \| [a-z0-9-]+ +\| [^|]+\| ✅apply  \|\n)+)(?=\s*\n|      \| remove)", lambda m: m.group(1) + rows, s)
    wr(p, s)
    p = ANY + "/🔮️oracles/🔣️.json"
    oj = json.loads(rd(p))
    catalog = oj["mutationCatalogs"][0]
    manifest = oj["mutationManifests"][0]
    vec_t = [v for v in catalog["vectors"] if v["mutationId"] == "change-beam-stud-count"][0]
    man_t = [m for m in manifest["mutations"] if m["id"] == "change-beam-stud-count"][0]
    for l in todo:
        v = copy.deepcopy(vec_t)
        v["mutationId"] = l["kind"]
        v["sourceMutationDirectoryName"] = l["dirname"]
        v["mutationDirectoryName"] = l["dirname"]
        catalog["vectors"].append(v)
        catalog["kinds"].append(l["kind"])
        m = copy.deepcopy(man_t)
        m["id"] = l["kind"]
        m["productionDispatch"]["operation"] = l["kind"]
        m["productionDispatch"]["variant"] = l["record"]
        manifest["mutations"].append(m)
    wr(p, dumps(oj))
    tax = json.loads(rd(TAX))
    new_rows = [{"aggregateVariant": l["record"], "artifact": "🧩️en1994", "entity": l["kind"][len("change-"):], "kind": l["kind"], "module": l["dirname"], "physicalLayout": "direct", "record": "Changed" + l["record"][len("Change"):], "source": "🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/🦀️.rs" % l["dirname"], "standard": "🔖️1", "subset": "✳️any", "type": l["record"], "verb": "change"} for l in todo]
    indices = [i for i, r in enumerate(tax["rows"]) if r["artifact"] == "🧩️en1994"]
    block = sorted([tax["rows"][i] for i in indices] + new_rows, key=lambda r: r["module"])
    tax["rows"][indices[0]: indices[-1] + 1] = block
    wr(TAX, dumps(tax))
    print("generated", len(todo), "leaves in", len(by_crate), "crates")


if __name__ == "__main__":
    main()
