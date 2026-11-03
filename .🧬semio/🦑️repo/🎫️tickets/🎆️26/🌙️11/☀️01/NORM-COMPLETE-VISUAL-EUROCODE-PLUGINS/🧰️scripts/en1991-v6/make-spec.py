import json, pathlib
def f(name, ty, en, de, unit=None, step=None, precision=None):
    r = {"name": name, "ty": ty, "en": en, "de": de}
    if unit: r["unit"] = unit
    if step is not None: r["step"] = step
    if precision is not None: r["precision"] = precision
    return r
def c(camel, entity, table, after, emoji, doc, le, ld, ie, idd, fields, kit):
    return {"camel": camel, "entity": entity, "table": table, "after": after, "emoji": emoji, "doc": doc, "label_en": le, "label_de": ld, "id_en": ie, "id_de": idd, "fields": fields, "kit": kit, "kit_after": after}
X = lambda: f("x", "f64", "Plan position x", "Position x", "m", 0.5)
Y = lambda: f("y", "f64", "Plan position y", "Position y", "m", 0.5)
cols = [
 c("helicopterLandings", "HelicopterLanding", "helicopter_landing", "cases", "🚁", "Hard landing of a helicopter on a roof pad by its take-off mass (EN 1991-1-7, 4.7).", "Helicopter landings", "Hubschrauberlandungen", "Helicopter landing id", "Hubschrauberlandung-Id",
   [f("mass", "f64", "Take-off mass m", "Startmasse m", "kg"), X(), Y(), f("height", "f64", "Height of the pad above ground", "Höhe des Landeplatzes über Gelände", "m", 0.5), f("assumed_force", "f64", "Assumed equivalent static force", "Angenommene statische Ersatzkraft", "N")],
   {"id": "helicopterLanding", "en": "Helicopter landing", "de": "Hubschrauberlandung", "icon": "plane", "scene_kind": "Load"}),
 c("tieRuns", "TieRun", "tie_run", "helicopterLandings", "🪢", "Horizontal tie of a floor with its spacing, span and loads for the robustness design (EN 1991-1-7, Annex A).", "Tie runs", "Zugglieder", "Tie id", "Zugglied-Id",
   [f("kind", "u8", "Tie kind (0 peripheral, 1 internal)", "Zuggliedart (0 Randzugglied, 1 Innenzugglied)"), f("spacing", "f64", "Tie spacing s", "Zuggliedabstand s", "m", 0.5), f("span", "f64", "Tie span L", "Zuggliedlänge L", "m", 0.5), f("dead_load", "f64", "Permanent floor load g_k", "Ständige Geschosslast g_k", "Pa"), f("imposed_load", "f64", "Variable floor load q_k", "Veränderliche Geschosslast q_k", "Pa"), f("psi", "f64", "Combination factor psi (psi_1 or psi_2)", "Kombinationsbeiwert psi (psi_1 oder psi_2)", None, 0.05, 2), X(), Y(), f("provided_force", "f64", "Provided tie force T_prov", "Vorhandene Zugkraft T_prov", "N")],
   {"id": "tieRun", "en": "Tie run", "de": "Zugglied", "icon": "link", "scene_kind": "Load"}),
 c("constructionLoads", "ConstructionLoad", "construction_load", "tieRuns", "🧱", "Construction load Q_c,i of a construction stage, summed per stage for the combination of EN 1991-1-6 (4.11, Annex A.2).", "Construction loads", "Bauzustandslasten", "Construction load id", "Bauzustandslast-Id",
   [f("stage", "String", "Construction stage id", "Bauzustand-Id"), f("kind", "u8", "Load kind (0 Q_ca, 1 Q_cb, 2 Q_cc, 3 Q_cd, 4 Q_ce, 5 Q_cf)", "Lastart (0 Q_ca, 1 Q_cb, 2 Q_cc, 3 Q_cd, 4 Q_ce, 5 Q_cf)"), f("magnitude", "f64", "Load Q_c,i as area load", "Last Q_c,i als Flächenlast", "Pa")],
   {"id": "constructionLoad", "en": "Construction load", "de": "Bauzustandslast", "icon": "hammer", "scene_kind": "Load"}),
 c("fallingLoads", "FallingLoad", "falling_load", "constructionLoads", "🪨", "Accidental falling load or crane impact during execution by its mass, drop height and stopping distance (EN 1991-1-6, accidental construction loads).", "Falling loads", "Herabfallende Lasten", "Falling load id", "Herabfallende-Last-Id",
   [f("mass", "f64", "Falling mass m", "Fallende Masse m", "kg"), f("drop_height", "f64", "Drop height h", "Fallhöhe h", "m", 0.5), f("stopping_distance", "f64", "Stopping distance s", "Bremsweg s", "m", 0.01), X(), Y(), f("assumed_force", "f64", "Assumed equivalent static force", "Angenommene statische Ersatzkraft", "N")],
   {"id": "fallingLoad", "en": "Falling load", "de": "Herabfallende Last", "icon": "arrow-down", "scene_kind": "Load"}),
 c("executionImperfections", "ExecutionImperfection", "execution_imperfection", "fallingLoads", "📐", "Geometric imperfection of a member during execution as equivalent horizontal force from its tolerance and vertical load (EN 1991-1-6, imperfections).", "Execution imperfections", "Ausführungsimperfektionen", "Imperfection id", "Imperfektion-Id",
   [f("height", "f64", "Member height h", "Bauteilhöhe h", "m", 0.5), f("tolerance", "f64", "Tolerance Delta", "Toleranz Delta", "m", 0.005), f("vertical_load", "f64", "Vertical load N", "Vertikallast N", "N"), X(), Y(), f("assumed_force", "f64", "Assumed equivalent horizontal force", "Angenommene horizontale Ersatzkraft", "N")],
   {"id": "executionImperfection", "en": "Execution imperfection", "de": "Ausführungsimperfektion", "icon": "ruler", "scene_kind": "Load"}),
 c("derailmentCases", "DerailmentCase", "derailment_case", "executionImperfections", "🚆", "Derailment design situation next to a track by its distance from the track axis and the design forces F_dx, F_dy entered from the applicable table (EN 1991-1-7, 4.4).", "Derailment cases", "Entgleisungsfälle", "Derailment case id", "Entgleisungsfall-Id",
   [f("distance", "f64", "Distance d from the track axis", "Abstand d von der Gleisachse", "m", 0.5), f("design_force_x", "f64", "Design force F_dx along the track (entered)", "Bemessungskraft F_dx in Gleisrichtung (eingegeben)", "N"), f("design_force_y", "f64", "Design force F_dy across the track (entered)", "Bemessungskraft F_dy quer zum Gleis (eingegeben)", "N"), X(), Y(), f("assumed_force", "f64", "Assumed resistance in track direction", "Angenommener Widerstand in Gleisrichtung", "N")],
   {"id": "derailmentCase", "en": "Derailment case", "de": "Entgleisungsfall", "icon": "train", "scene_kind": "Load"}),
]
spec = {"artifact": "en1991", "collections": cols}
p = pathlib.Path(__file__).parent.parent / "field-adder/specs/en1991-v6.json"
p.write_text(json.dumps(spec, ensure_ascii=False, indent=2) + "\n")
print(p)
