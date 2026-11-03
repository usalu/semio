"""Spec of the EN 1993 `HollowJoint` entity (RHS K/N gap joints, EN 1993-1-8 7.4; V-440). Executed by en1993-entity-gen.py."""
ENT = "HollowJoint"
ENT_DE = "Hohlprofilknoten"
SNAKE = "hollow_joint"
SNAKE_PL = "hollow_joints"
KEBAB_PL = "hollow-joints"
TABLE = "hollow_joint"
TS_TYPE = "HollowJoint"
EMOJI = "🕳"
DOC = "🕳 Welded RHS truss joint with a gap (K and N joints): chord face failure and brace effective width (EN 1993-1-8 7.4)."
COLL_EN, COLL_DE = "Hollow section joints", "Hohlprofilknoten"
NTABLES = 31
WORDS = "Thirty-one"
HAS_MEMBER = True
TIER = 0
KIT = ("hollowJoint", "Hollow section joint", "Hohlprofilknoten", "link", "Group")
PREV = dict(ent="ParametricSection", snake_pl="parametric_sections", snake="parametric_section", table="en1993_parametric_section", ts_type="En1993SqliteParametric", t_index=29, field_id=21, words="Thirty", ntables=30, kit="parametricSection", kebab_pl="parametric-sections")
CHOICE_CONST = {}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("member_id", "String", "", "", "", "Chord member id", "Gurt-Bauteil-Id"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("chord_width", "f64", 0.0, 0.12, "m", "Chord width b_0", "Gurtbreite b_0"),
    f("chord_height", "f64", 0.0, 0.12, "m", "Chord height h_0", "Gurthöhe h_0"),
    f("chord_thickness", "f64", 0.0, 0.008, "m", "Chord wall thickness t_0", "Gurtwanddicke t_0"),
    f("chord_fy", "f64", 0.0, 355.0e6, "Pa", "Chord yield strength f_y0", "Gurtstreckgrenze f_y0"),
    f("chord_stress", "f64", 0.0, 0.0, "Pa", "Chord stress σ_0,Ed (compression negative)", "Gurtspannung σ_0,Ed (Druck negativ)"),
    f("brace1_width", "f64", 0.0, 0.08, "m", "Brace 1 width b_1", "Streben-1-Breite b_1"),
    f("brace1_height", "f64", 0.0, 0.08, "m", "Brace 1 height h_1", "Streben-1-Höhe h_1"),
    f("brace1_thickness", "f64", 0.0, 0.005, "m", "Brace 1 wall thickness t_1", "Streben-1-Wanddicke t_1"),
    f("brace1_angle", "f64", 0.0, 45.0, "deg", "Brace 1 angle θ_1", "Streben-1-Winkel θ_1"),
    f("brace1_force", "f64", 0.0, 60000.0, "N", "Brace 1 force N_1,Ed", "Streben-1-Kraft N_1,Ed"),
    f("brace2_width", "f64", 0.0, 0.08, "m", "Brace 2 width b_2", "Streben-2-Breite b_2"),
    f("brace2_height", "f64", 0.0, 0.08, "m", "Brace 2 height h_2", "Streben-2-Höhe h_2"),
    f("brace2_thickness", "f64", 0.0, 0.005, "m", "Brace 2 wall thickness t_2", "Streben-2-Wanddicke t_2"),
    f("brace2_angle", "f64", 0.0, 45.0, "deg", "Brace 2 angle θ_2", "Streben-2-Winkel θ_2"),
    f("brace2_force", "f64", 0.0, 60000.0, "N", "Brace 2 force N_2,Ed", "Streben-2-Kraft N_2,Ed"),
    f("gap", "f64", 0.0, 0.03, "m", "Gap g between the braces", "Spalt g zwischen den Streben"),
    f("brace_fy", "f64", 0.0, 355.0e6, "Pa", "Brace yield strength f_yi", "Strebenstreckgrenze f_yi"),
]
SHOW_TEXT = {"id": "hollow-1", "member_id": "member-b1", "label": "K gap joint"}
