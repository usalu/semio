"""Spec of the EN 1993 `JointPlate` entity (fin plate, splice flange cover or splice web cover of a joint: plate, bolt pattern and weld for EN 1993-1-8 3.4, 3.6, 3.10, 4.5; V-451, V-452). Executed by en1993-entity-gen.py."""
ENT = "JointPlate"
ENT_DE = "Anschlussblech"
SNAKE = "joint_plate"
SNAKE_PL = "joint_plates"
KEBAB_PL = "joint-plates"
TABLE = "joint_plate"
TS_TYPE = "JointPlate"
EMOJI = "🧩"
DOC = "🧩 Plate of a fin plate or splice joint: role, plate size and steel, bolt pattern, eccentricity of the bolt line to the support and weld to the support or member."
COLL_EN, COLL_DE = "Joint plates", "Anschlussbleche"
NTABLES = 36
WORDS = "Thirty-six"
HAS_MEMBER = False
TIER = 0
KIT = ("jointPlate", "Joint plate", "Anschlussblech", "plate", "Plate")
PREV = dict(ent="Placement", snake_pl="placements", snake="placement", table="en1993_placement", ts_type="En1993SqlitePlacement", t_index=34, field_id=26, words="Thirty-five", ntables=35, kit="", kebab_pl="placements")
EPREV = dict(ent="FrameBar", snake_pl="frame_bars", snake="frame_bar", table="en1993_frame_bar", ts_type="En1993SqliteFrameBar", t_index=33, field_id=25, words="Thirty-four", ntables=34, kit="frameBar", kebab_pl="frame-bars")
PLATE_ROLE = [("finPlate", "Fin plate (shear tab)", "Fahnenblech"), ("flangeCover", "Splice flange cover plate", "Stoßlasche am Flansch"), ("webCover", "Splice web cover plate", "Stoßlasche am Steg")]
BOLT_CLASS = [("4.6", "4.6", "4.6"), ("5.6", "5.6", "5.6"), ("8.8", "8.8", "8.8"), ("10.9", "10.9", "10.9")]
CHOICE_CONST = {"role": "JOINT_PLATE_ROLE", "bolt_class": "JOINT_PLATE_BOLT_CLASS"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("joint_id", "String", "", "", "", "Joint id", "Anschluss-Id"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("role", "String", "", "finPlate", "", "Plate role", "Blechrolle", PLATE_ROLE),
    f("plate_height", "f64", 0.0, 0.24, "m", "Plate height h_p (along the bolt line)", "Blechhöhe h_p (entlang der Schraubenlinie)"),
    f("plate_width", "f64", 0.0, 0.1, "m", "Plate width b_p", "Blechbreite b_p"),
    f("plate_thickness", "f64", 0.0, 0.01, "m", "Plate thickness t_p", "Blechdicke t_p"),
    f("plate_count", "u32", 0, 1, "", "Plates acting in parallel (1 or 2)", "Parallel wirkende Bleche (1 oder 2)"),
    f("plate_fy", "f64", 0.0, 355000000.0, "Pa", "Plate yield strength f_y,p", "Blechstreckgrenze f_y,p"),
    f("plate_fu", "f64", 0.0, 470000000.0, "Pa", "Plate tensile strength f_u,p", "Zugfestigkeit des Blechs f_u,p"),
    f("bolt_class", "String", "", "8.8", "", "Bolt class", "Festigkeitsklasse", BOLT_CLASS),
    f("bolt_diameter", "f64", 0.0, 0.02, "m", "Bolt diameter d", "Schraubendurchmesser d"),
    f("hole_diameter", "f64", 0.0, 0.022, "m", "Hole diameter d_0", "Lochdurchmesser d_0"),
    f("bolt_rows", "u32", 0, 3, "", "Bolt rows n_1 along the line of action", "Schraubenreihen n_1 in Kraftrichtung"),
    f("bolts_per_row", "u32", 0, 1, "", "Bolts per row n_2", "Schrauben je Reihe n_2"),
    f("pitch", "f64", 0.0, 0.07, "m", "Pitch p_1", "Lochabstand p_1"),
    f("gauge", "f64", 0.0, 0.0, "m", "Gauge p_2", "Lochabstand quer p_2"),
    f("end_distance", "f64", 0.0, 0.04, "m", "End distance e_1", "Randabstand e_1"),
    f("edge_distance", "f64", 0.0, 0.04, "m", "Edge distance e_2", "Randabstand e_2"),
    f("shear_planes", "u32", 0, 1, "", "Shear planes per bolt", "Scherfugen je Schraube"),
    f("eccentricity", "f64", 0.0, 0.06, "m", "Eccentricity z of the bolt line to the support", "Exzentrizität z der Schraubenlinie zum Auflager"),
    f("weld_throat", "f64", 0.0, 0.005, "m", "Weld throat thickness a", "Nahtdicke a"),
    f("weld_fu", "f64", 0.0, 470000000.0, "Pa", "Weld reference tensile strength f_u", "Bezugs-Zugfestigkeit f_u der Naht"),
    f("weld_beta_w", "f64", 0.0, 0.9, "1", "Correlation factor β_w (Table 4.1)", "Korrelationsbeiwert β_w (Tabelle 4.1)"),
]
SHOW_TEXT = {"id": "plate-1", "joint_id": "joint-fin", "label": "Fin plate", "role": "finPlate", "bolt_class": "8.8"}
