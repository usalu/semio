"""Spec of the EN 1993 `ParametricSection` entity (welded I, RHS and CHS sections by dimensions, compared with the section row they describe). Executed by en1993-entity-gen.py."""
ENT = "ParametricSection"
ENT_DE = "Parametrischer Querschnitt"
SNAKE = "parametric_section"
SNAKE_PL = "parametric_sections"
KEBAB_PL = "parametric-sections"
TABLE = "parametric_section"
TS_TYPE = "Parametric"
EMOJI = "📐"
DOC = "📐 Parametric section: welded I, rectangular and circular hollow sections by their dimensions, compared with the section row (A, I_y, I_z, W_el,y, W_pl,y)."
COLL_EN, COLL_DE = "Parametric sections", "Parametrische Querschnitte"
NTABLES = 30
WORDS = "Thirty"
HAS_MEMBER = False
TIER = 0
KIT = ("parametricSection", "Parametric section", "Parametrischer Querschnitt", "ruler", "Group")
PREV = dict(ent="WeldGroup", snake_pl="weld_groups", snake="weld_group", table="en1993_weld_group", ts_type="En1993SqliteWeldGroup", t_index=28, field_id=20, words="Twenty-nine", ntables=29, kit="weldGroup", kebab_pl="weld-groups")
SHAPE = [("weldedI", "Welded I section", "Geschweißter I-Querschnitt"), ("rhs", "Rectangular hollow section", "Rechteckhohlprofil"), ("chs", "Circular hollow section", "Kreishohlprofil")]
CHOICE_CONST = {"shape": "PARAMETRIC_SHAPE"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("section_id", "String", "", "", "", "Described section id", "Id des beschriebenen Querschnitts"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("shape", "String", "", "weldedI", "", "Shape", "Form", SHAPE),
    f("height", "f64", 0.0, 0.24, "m", "Height h (diameter d for CHS)", "Höhe h (Durchmesser d bei CHS)"),
    f("width", "f64", 0.0, 0.24, "m", "Width b", "Breite b"),
    f("web_thickness", "f64", 0.0, 0.01, "m", "Web or wall thickness t_w", "Steg- oder Wanddicke t_w"),
    f("flange_thickness", "f64", 0.0, 0.017, "m", "Flange thickness t_f", "Flanschdicke t_f"),
    f("tolerance", "f64", 0.05, 0.05, "1", "Tolerance against the section row (fillets)", "Toleranz gegen den Querschnitt (Ausrundungen)"),
]
SHOW_TEXT = {"id": "param-1", "section_id": "sec-heb240", "label": "Parametric HEB 240"}
