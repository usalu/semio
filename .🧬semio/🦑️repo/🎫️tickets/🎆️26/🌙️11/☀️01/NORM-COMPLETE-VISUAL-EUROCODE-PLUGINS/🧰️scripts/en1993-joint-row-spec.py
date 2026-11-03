"""Spec of the EN 1993 `JointRow` entity (one bolt row of an end plate joint for the EN 1993-1-8 component method; V-420, V-421, V-423, V-430..V-433). Executed by en1993-entity-gen.py."""
ENT = "JointRow"
ENT_DE = "Anschlussschraubenreihe"
SNAKE = "joint_row"
SNAKE_PL = "joint_rows"
KEBAB_PL = "joint-rows"
TABLE = "joint_row"
TS_TYPE = "JointRow"
EMOJI = "🔩"
DOC = "🔩 Bolt row of an end plate joint: lever arm, T-stub geometry (m, e, m_x, e_x, alpha), pitch to the previous row, bolts and group effect for the EN 1993-1-8 component method."
COLL_EN, COLL_DE = "Joint bolt rows", "Anschluss-Schraubenreihen"
NTABLES = 32
WORDS = "Thirty-two"
HAS_MEMBER = False
TIER = 0
KIT = ("jointRow", "Joint bolt row", "Anschluss-Schraubenreihe", "bolt", "Group")
PREV = dict(ent="HollowJoint", snake_pl="hollow_joints", snake="hollow_joint", table="en1993_hollow_joint", ts_type="En1993SqliteHollowJoint", t_index=30, field_id=22, words="Thirty-one", ntables=31, kit="hollowJoint", kebab_pl="hollow-joints")
ROW_TYPE = [("extended", "Extended part above the beam flange", "Überstand oberhalb des Trägerflanschs"), ("firstBelow", "First row below the tension flange", "Erste Reihe unter dem Zugflansch"), ("inner", "Other inner row", "Weitere innere Reihe")]
BOLT_CLASS = [("8.8", "8.8", "8.8"), ("10.9", "10.9", "10.9")]
CHOICE_CONST = {"row_type": "JOINT_ROW_TYPE", "bolt_class": "JOINT_ROW_BOLT_CLASS"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("joint_id", "String", "", "", "", "Joint id", "Anschluss-Id"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("row_type", "String", "", "extended", "", "Row position", "Reihenlage", ROW_TYPE),
    f("lever", "f64", 0.0, 0.27, "m", "Lever arm h_r to the compression centre", "Hebelarm h_r zum Druckpunkt"),
    f("bolt_to_web", "f64", 0.0, 0.04, "m", "Bolt to web distance m", "Schraubenabstand zum Steg m"),
    f("edge_distance", "f64", 0.0, 0.04, "m", "Edge distance e", "Randabstand e"),
    f("extension_to_flange", "f64", 0.0, 0.04, "m", "Extension bolt to flange m_x", "Überstand Schraube bis Flansch m_x"),
    f("extension_edge", "f64", 0.0, 0.04, "m", "Extension edge distance e_x", "Überstand Randabstand e_x"),
    f("alpha_factor", "f64", 0.0, 6.5, "1", "Yield line factor α (Fig. 6.11)", "Fließgelenkfaktor α (Bild 6.11)"),
    f("pitch_to_previous", "f64", 0.0, 0.0, "m", "Pitch p to the previous row", "Teilung p zur vorigen Reihe"),
    f("bolts", "u32", 0, 2, "", "Bolts in the row", "Schrauben in der Reihe"),
    f("bolt_diameter", "f64", 0.0, 0.02, "m", "Bolt diameter d", "Schraubendurchmesser d"),
    f("bolt_stress_area", "f64", 0.0, 0.000245, "m2", "Bolt stress area A_s", "Schraubenspannungsquerschnitt A_s"),
    f("grip_length", "f64", 0.0, 0.05, "m", "Grip length L_b", "Klemmlänge L_b"),
    f("bolt_class", "String", "", "10.9", "", "Bolt class", "Festigkeitsklasse", BOLT_CLASS),
    f("group_with_previous", "u32", 0, 0, "", "Group effect with the previous row (0/1)", "Gruppenwirkung mit voriger Reihe (0/1)"),
]
SHOW_TEXT = {"id": "row-1", "joint_id": "joint-ep", "label": "Row 1 (extended)", "row_type": "extended", "bolt_class": "10.9"}
