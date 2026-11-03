"""Spec of the EN 1993 `Restraint` entity (supports and restraints of a member: V-113b Mcr by FE, V-123 general method, V-124 plastic hinge stable length). Executed by en1993-entity-gen.py (U is in scope)."""
ENT = "Restraint"
ENT_DE = "Halterung"
SNAKE = "restraint"
SNAKE_PL = "restraints"
KEBAB_PL = "restraints"
TABLE = "restraint"
TS_TYPE = "Restraint"
EMOJI = "🧷"
DOC = "🧷 Supports and restraints of a member: end supports, an intermediate support, continuous torsional and discrete lateral springs, load height; M_cr by finite elements (EN 1993-1-1 6.3.2, 6.3.4, 6.3.5.3)."
COLL_EN, COLL_DE = "Restraints", "Halterungen"
NTABLES = 27
WORDS = "Twenty-seven"
HAS_MEMBER = True
TIER = 1
KIT = ("restraint", "Restraint", "Halterung", "anchor", "Group")
PREV = dict(ent="Storey", snake_pl="storeys", snake="storey", table="en1993_storey", ts_type="En1993SqliteStorey", t_index=25, field_id=17, words="Twenty-six", ntables=26, kit="storey", kebab_pl="storeys")
SUPPORT = [("fork", "Fork (laterally and torsionally held, warping free)", "Gabellagerung (seitlich und gegen Verdrehen gehalten, Wölbung frei)"), ("fixed", "Fixed (warping and lateral rotation restrained)", "Einspannung (Wölbung und seitliche Verdrehung behindert)")]
DISTRIBUTION = [("uniform", "Uniform moment", "Konstantes Moment"), ("udl", "Uniformly distributed load", "Gleichstreckenlast"), ("linear", "Linear moment (end moment ratio of the member)", "Linearer Momentenverlauf (Randmomentenverhältnis des Stabes)")]
CHOICE_CONST = {"start_support": "RESTRAINT_SUPPORT", "end_support": "RESTRAINT_SUPPORT", "load_distribution": "RESTRAINT_DISTRIBUTION"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("member_id", "String", "", "", "", "Member id", "Bauteil-Id"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("start_support", "String", "", "fork", "", "Support at the start", "Lagerung am Anfang", SUPPORT),
    f("end_support", "String", "", "fork", "", "Support at the end", "Lagerung am Ende", SUPPORT),
    f("intermediate_position", "f64", 0.0, 0.0, "m", "Intermediate lateral support at x (0 = none)", "Zwischenlagerung bei x (0 = keine)"),
    f("torsional_spring", "f64", 0.0, 0.0, "Nmm", "Continuous torsional spring c_θ", "Kontinuierliche Drehbettung c_θ"),
    f("lateral_spring_position", "f64", 0.0, 0.0, "m", "Lateral spring at x (0 = none)", "Seitliche Feder bei x (0 = keine)"),
    f("lateral_spring_stiffness", "f64", 0.0, 0.0, "N/m", "Lateral spring stiffness k", "Federsteifigkeit k"),
    f("load_height", "f64", 0.0, 0.0, "m", "Load height z_g above the shear centre", "Lastangriffshöhe z_g über dem Schubmittelpunkt"),
    f("load_distribution", "String", "", "udl", "", "Load and moment distribution", "Last- und Momentenverlauf", DISTRIBUTION),
    f("design_moment", "f64", 0.0, 100000.0, "Nm", "Maximum design moment M_y,Ed", "Maximales Bemessungsmoment M_y,Ed"),
    f("design_axial", "f64", 0.0, 0.0, "N", "Design axial force N_Ed", "Bemessungsnormalkraft N_Ed"),
    f("general_method", "u32", 0, 0, "", "General method 6.3.4 (0/1)", "Allgemeines Verfahren 6.3.4 (0/1)"),
    f("hinge_to_restraint", "f64", 0.0, 0.0, "m", "Plastic hinge to next restraint (0 = none)", "Fließgelenk bis zur nächsten Halterung (0 = keines)"),
]
SHOW_TEXT = {"id": "restraint-1", "member_id": "member-b1", "label": "Restraint 1"}
