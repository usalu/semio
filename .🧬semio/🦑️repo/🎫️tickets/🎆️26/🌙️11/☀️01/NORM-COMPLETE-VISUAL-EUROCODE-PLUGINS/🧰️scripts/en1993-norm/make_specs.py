import json, pathlib
OUT = pathlib.Path(__file__).resolve().parents[1] / "schema-evolve" / "specs"

def L(en, de):
    return {"en": en, "de": de}

def f(name, ty, en, de, doc, sample, unit=None, enum=None, **extra):
    d = {"name": name, "type": ty, "label": L(en, de), "doc": doc, "sample": sample}
    if unit is not None:
        d["unit"] = unit
    if enum:
        d["enum"] = enum
    d.update(extra)
    return d

def enum(name, doc, values):
    return {"name": name, "doc": doc, "values": [{"token": t, "label": L(e, d)} for t, e, d in values]}

def end(name):
    return enum(name, "⛓ End condition of a column at one node of a frame: framed, fixed or pinned.", [("Framed", "Framed", "Eingebunden"), ("Fixed", "Fixed", "Eingespannt"), ("Pinned", "Pinned", "Gelenkig")])

def collection(name, entity, emoji, doc, label, idl, singular, slug, fields):
    return {"artifact": "en1993", "reference": "frameMemberBases", "collection": {"name": name, "entity": entity, "emoji": emoji, "doc": doc, "label": L(*label), "id": L(*idl), "singular": L(*singular), "kindSlug": slug, "fields": fields}}

def write(name, spec):
    (OUT / name).write_text(json.dumps(spec, ensure_ascii=False, indent=2) + "\n", encoding="utf8")

write("en1993-joint-plate-group-method.json", {"artifact": "en1993", "leaf": {"kindSlug": "joint-plate", "default": "Elastic", "field": {
    "name": "group_method", "type": "enum", "label": L("Bolt group method", "Verfahren der Schraubengruppe"),
    "doc": "🔩 Distribution of the load over the bolt group of the plate (EN 1993-1-8 3.12): Elastic is the vector method with the polar moment of the bolts, InstantaneousCentre is the plastic method with the load-deformation law of the bolts about the instantaneous centre of rotation.",
    "enum": enum("BoltGroupMethod", "🔩 Method for the force on each bolt of a bolt group: elastic vector method or instantaneous centre of rotation.", [
        ("Elastic", "Elastic (polar moment)", "Elastisch (polares Moment)"),
        ("InstantaneousCentre", "Instantaneous centre", "Momentanpol")])}}})

write("en1993-fatigue-stresses.json", collection(
    "fatigueStresses", "FatigueStress", "⏳",
    "⏳ Stress range of one fatigue detail from the fatigue load model (EN 1993-1-9 5.1 to 5.4): the stresses at the detail fibre from the member actions of the highest and the lowest load case, the reduction of the compressive part for non-welded or stress-relieved details, the geometric stress concentration, the plate thickness for the size effect and the number of cycles of the range.",
    ("Stress ranges", "Spannungsschwingbreiten"), ("Stress range ID", "Schwingbreiten-ID"), ("stress range", "Spannungsschwingbreite"), "fatigue-stress", [
        f("detail_id", "String", "Fatigue detail", "Kerbfall-Detail", "🔗 Id of the fatigue detail (category, method) the range belongs to.", "fatigue-1"),
        f("max_case_id", "String", "Load case of the maximum", "Lastfall des Maximums", "⬆ Id of the load case whose member actions give the largest stress at the detail fibre (fatigue load model in its unfavourable position).", "lm-max"),
        f("min_case_id", "String", "Load case of the minimum", "Lastfall des Minimums", "⬇ Id of the load case whose member actions give the smallest stress at the detail fibre.", "lm-min"),
        f("fibre_z", "f64", "Fibre distance z", "Faserabstand z", "📍 Distance z of the detail fibre from the y axis, positive in the direction of positive bending moment M_y compression: sigma = N/A + M_y z/I_y - M_z y/I_z.", 0.1, unit="m"),
        f("fibre_y", "f64", "Fibre distance y", "Faserabstand y", "📍 Distance y of the detail fibre from the z axis.", 0.0, unit="m"),
        f("relief", "enum", "Residual stress state", "Eigenspannungszustand", "🧯 As-welded details take the full range; stress-relieved welded and non-welded details take the compressive part of the cycle reduced (EN 1993-1-9 5.4).", "AsWelded", enum=enum("StressRelief", "🧯 Residual stress state of a fatigue detail: as welded, welded and stress relieved or non-welded.", [
            ("AsWelded", "As welded", "Geschweißt, nicht spannungsarm"),
            ("StressRelieved", "Welded, stress relieved", "Geschweißt, spannungsarm geglüht"),
            ("NonWelded", "Non-welded", "Nicht geschweißt")])),
        f("concentration", "f64", "Stress concentration k_f", "Spannungskonzentration k_f", "📐 Geometric stress concentration factor k_f of the nominal stress at the detail (EN 1993-1-9 5.2); 0 = not given, the nominal stress is used unchanged.", 0.0, unit=""),
        f("thickness", "f64", "Plate thickness t", "Blechdicke t", "📏 Thickness t of the cracked plate for the size effect k_s = (25/t)^n of the detail strength (EN 1993-1-9 7.1); 0 = not given, no size effect is applied.", 0.0, unit="m"),
        f("cycles", "f64", "Number of cycles", "Lastspielzahl", "🔁 Number of cycles of this stress range in the design life.", 2000000.0, unit="")]))

write("en1993-fastened-joints.json", collection(
    "fastenedJoints", "FastenedJoint", "🪛",
    "🪛 Mechanically fastened joint of cold-formed sheets (EN 1993-1-3 8.3): self-tapping screws or blind rivets in shear and tension with the sheet thicknesses and strengths, the fastener spacing and the shear resistance of the fastener from tests.",
    ("Fastened joints", "Befestigungsverbindungen"), ("Fastened joint ID", "Verbindungs-ID"), ("fastened joint", "Befestigungsverbindung"), "fastened-joint", [
        f("member_id", "String", "Cold-formed section", "Kaltprofil", "🥫 Id of the cold-formed member the joint belongs to.", "cold-1"),
        f("kind", "enum", "Fastener kind", "Art des Verbindungsmittels", "🪛 Self-tapping screw or blind rivet.", "SelfTappingScrew", enum=enum("FastenerKind", "🪛 Mechanical fastener of cold-formed sheets: self-tapping screw or blind rivet.", [
            ("SelfTappingScrew", "Self-tapping screw", "Selbstbohrende Schraube"),
            ("BlindRivet", "Blind rivet", "Blindniet")])),
        f("diameter", "f64", "Fastener diameter d", "Durchmesser d", "⌀ Nominal diameter d of the screw or rivet.", 0.0055, unit="m", precision=4, step=0.0005, range=[0.002, 0.02]),
        f("washer_diameter", "f64", "Washer or head diameter d_w", "Scheiben- oder Kopfdurchmesser d_w", "⭕ Diameter d_w of the washer or head that bears on the sheet in tension (pull-over); 0 = not given.", 0.016, unit="m", precision=4, step=0.0005, range=[0, 0.05]),
        f("count", "u32", "Number of fasteners", "Anzahl der Verbindungsmittel", "🔢 Number of fasteners n that share the force.", 4),
        f("per_row", "u32", "Fasteners across the section", "Verbindungsmittel im Schnitt", "🔢 Number of fasteners in the net section across the width.", 2),
        f("head_thickness", "f64", "Sheet thickness t_1", "Blechdicke t_1", "📏 Thickness t_1 of the sheet on the head side.", 0.00075, unit="m", precision=5, step=0.00005, range=[0.0004, 0.01]),
        f("base_thickness", "f64", "Sheet thickness t_2", "Blechdicke t_2", "📏 Thickness t_2 of the sheet or support the fastener ends in.", 0.0015, unit="m", precision=5, step=0.00005, range=[0.0004, 0.02]),
        f("head_strength", "f64", "Ultimate strength f_u,1", "Zugfestigkeit f_u,1", "💪 Ultimate tensile strength of the sheet t_1.", 360.0e6, unit="Pa"),
        f("base_strength", "f64", "Ultimate strength f_u,2", "Zugfestigkeit f_u,2", "💪 Ultimate tensile strength of the sheet t_2.", 360.0e6, unit="Pa"),
        f("end_distance", "f64", "End distance e_1", "Randabstand e_1", "↦ Distance e_1 from the fastener to the end of the sheet in the direction of the force.", 0.03, unit="m", precision=4, step=0.0005, range=[0.005, 0.2]),
        f("edge_distance", "f64", "Edge distance e_2", "Randabstand e_2", "↥ Distance e_2 from the fastener to the edge of the sheet perpendicular to the force.", 0.015, unit="m", precision=4, step=0.0005, range=[0.005, 0.2]),
        f("pitch", "f64", "Pitch p_1", "Lochabstand p_1", "↔ Spacing p_1 of the fasteners in the direction of the force.", 0.03, unit="m", precision=4, step=0.0005, range=[0.005, 0.3]),
        f("gauge", "f64", "Gauge p_2", "Rissabstand p_2", "↕ Spacing p_2 of the fastener rows perpendicular to the force.", 0.03, unit="m", precision=4, step=0.0005, range=[0.005, 0.3]),
        f("sheet_width", "f64", "Sheet width b", "Blechbreite b", "↔ Width b of the sheet in the net section across the fasteners.", 0.1, unit="m", precision=4, step=0.001, range=[0.01, 1.0]),
        f("shear_force", "f64", "Shear force F_v,Ed", "Querkraft F_v,Ed", "➡ Shear force on the fastened joint, shared by the n fasteners.", 4000.0, unit="N"),
        f("tension_force", "f64", "Tension force F_t,Ed", "Zugkraft F_t,Ed", "⬆ Tension force on the fastened joint (pull-out and pull-over), shared by the n fasteners.", 0.0, unit="N"),
        f("shear_resistance", "f64", "Fastener shear resistance F_v,Rd", "Abscherwiderstand F_v,Rd", "🧪 Design shear resistance of one fastener from the manufacturer or tests; 0 = not given, only the sheet resistances are checked.", 0.0, unit="N")]))

write("en1993-buckling-lengths.json", collection(
    "bucklingLengths", "BucklingLength", "🪜",
    "🪜 Buckling length of a column in a frame from the end restraint of the adjacent members (EN 1993-1-1 6.3.1.3 with the nomogram of the effective length factor): sway or non-sway mode, the stiffness sums of columns and beams at both ends, and the member whose buckling length it checks.",
    ("Buckling lengths", "Knicklängen"), ("Buckling length ID", "Knicklängen-ID"), ("buckling length", "Knicklänge"), "buckling-length", [
        f("member_id", "String", "Column", "Stütze", "🧱 Id of the column whose buckling length is derived.", "member-1"),
        f("plane", "enum", "Buckling plane", "Knickebene", "🧭 Plane of the frame: buckling about the major axis (length L_cr,y) or the minor axis (L_cr,z).", "MajorAxis", enum=enum("BucklingPlane", "🧭 Buckling plane of a column: about the major or about the minor axis.", [
            ("MajorAxis", "About the major axis", "Um die starke Achse"),
            ("MinorAxis", "About the minor axis", "Um die schwache Achse")])),
        f("sway", "enum", "Frame mode", "Rahmenverformung", "↔ Non-sway or sway buckling mode of the frame.", "NonSway", enum=enum("FrameSway", "↔ Buckling mode of a frame: nodes held (non-sway) or free to translate (sway).", [
            ("NonSway", "Non-sway", "Unverschieblich"),
            ("Sway", "Sway", "Verschieblich")])),
        f("restraint_a", "enum", "Node A", "Knoten A", "⛓ End condition at node A (the lower end of a column): framed into columns and beams, fixed (rigid) or pinned.", "Framed", enum=end("NodeARestraint")),
        f("column_stiffness_a", "f64", "Node A: column stiffness sum", "Knoten A: Stützensteifigkeit", "🧮 Sum of I_c/L_c of the columns at node A, including the column itself.", 3.6e-5, unit="m3", precision=7, step=1e-6, range=[0, 0.01]),
        f("beam_stiffness_a", "f64", "Node A: beam stiffness sum", "Knoten A: Riegelsteifigkeit", "🧮 Sum of alpha I_b/L_b of the beams at node A with the far-end factor alpha of each beam.", 7.2e-5, unit="m3", precision=7, step=1e-6, range=[0, 0.01]),
        f("restraint_b", "enum", "Node B", "Knoten B", "⛓ End condition at node B (the upper end of a column): framed into columns and beams, fixed (rigid) or pinned.", "Framed", enum=end("NodeBRestraint")),
        f("column_stiffness_b", "f64", "Node B: column stiffness sum", "Knoten B: Stützensteifigkeit", "🧮 Sum of I_c/L_c of the columns at node B, including the column itself.", 3.6e-5, unit="m3", precision=7, step=1e-6, range=[0, 0.01]),
        f("beam_stiffness_b", "f64", "Node B: beam stiffness sum", "Knoten B: Riegelsteifigkeit", "🧮 Sum of alpha I_b/L_b of the beams at node B with the far-end factor alpha of each beam.", 7.2e-5, unit="m3", precision=7, step=1e-6, range=[0, 0.01])]))

write("en1993-forming-gains.json", collection(
    "formingGains", "FormingGain", "🥫",
    "🥫 Enhanced yield strength of a cold-formed member from the work hardening of the bends (EN 1993-1-3 3.2.2): process, ultimate strength of the base material, gross area and number of 90 degree bends with a small radius.",
    ("Forming gains", "Kaltverfestigungen"), ("Forming gain ID", "Kaltverfestigungs-ID"), ("forming gain", "Kaltverfestigung"), "forming-gain", [
        f("member_id", "String", "Cold-formed section", "Kaltprofil", "🥫 Id of the cold-formed member whose f_y is checked against the enhanced yield strength.", "cold-1"),
        f("forming", "enum", "Forming process", "Umformverfahren", "🏭 Cold roll forming or other forming (brake pressing) selects the coefficient k of f_ya.", "RollForming", enum=enum("FormingProcess", "🏭 Forming process of a cold-formed section: cold roll forming or other forming.", [
            ("RollForming", "Cold roll forming", "Rollprofilieren"),
            ("OtherForming", "Other forming", "Sonstiges Umformen")])),
        f("ultimate_strength", "f64", "Ultimate strength f_u", "Zugfestigkeit f_u", "💪 Ultimate tensile strength of the base material before forming.", 360.0e6, unit="Pa"),
        f("gross_area", "f64", "Gross area A_g", "Bruttoquerschnitt A_g", "▭ Gross cross-section area A_g of the member.", 0.0006, unit="m2", precision=6, step=1e-5, range=[0, 0.05]),
        f("bends", "u32", "Bends n", "Anzahl der Biegungen n", "🔢 Number of 90 degree bends with an inner radius r <= 5 t (bends of other angles count proportionally).", 4)]))

write("en1993-material-ductility.json", collection(
    "materialDuctility", "MaterialDuctility", "🧪",
    "🧪 Elongation at failure and ultimate strain of a steel material (EN 1993-1-1 3.2.2): the values of the material certificate that, with f_u/f_y, decide whether a material is ductile enough for plastic global analysis.",
    ("Material ductility", "Werkstoffduktilität"), ("Ductility ID", "Duktilitäts-ID"), ("material ductility", "Werkstoffduktilität"), "material-ductility", [
        f("material_id", "String", "Material", "Werkstoff", "🧪 Id of the steel material the certificate values belong to.", "steel-s355"),
        f("elongation", "f64", "Elongation at failure", "Bruchdehnung", "📏 Elongation at failure of the material as a fraction (0.15 = 15 %).", 0.20, unit="", precision=3, step=0.005, range=[0, 1]),
        f("ultimate_strain", "f64", "Ultimate strain eps_u", "Gleichmaßdehnung eps_u", "📏 Strain eps_u at the ultimate strength f_u as a fraction.", 0.15, unit="", precision=3, step=0.005, range=[0, 1])]))

write("en1993-forming-gain-used-yield.json", {"artifact": "en1993", "leaf": {"kindSlug": "forming-gain", "default": 0.0, "field": {
    "name": "used_yield", "type": "f64", "unit": "Pa", "label": L("Yield strength used f_y,used", "Angesetzte Streckgrenze f_y,used"),
    "doc": "🥫 Yield strength the design uses for the member: the basic f_yb, or an enhanced value that must not exceed the average yield strength f_ya of the cold forming (EN 1993-1-3 3.2.2); 0 = the basic yield strength of the member is used.",
    "precision": 1, "step": 1000000.0, "range": [0, 1.0e9]}}})
