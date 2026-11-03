"""Spec of the EN 1993 `FoundationBlock` entity (concrete block, grout and anchor bolts under a base plate: f_jd, k_j and the anchor T-stub of EN 1993-1-8 6.2.5, 6.2.6.11, EN 1992-1-1 6.7; V-440). Executed by en1993-entity-gen.py."""
ENT = "FoundationBlock"
ENT_DE = "Fundamentblock"
SNAKE = "foundation_block"
SNAKE_PL = "foundation_blocks"
KEBAB_PL = "foundation-blocks"
TABLE = "foundation_block"
TS_TYPE = "FoundationBlock"
EMOJI = "🧱"
DOC = "🧱 Concrete block, grout layer and anchor bolts under a base plate joint: block size and strength, grout thickness and strength, anchors in tension."
COLL_EN, COLL_DE = "Foundation blocks", "Fundamentblöcke"
NTABLES = 37
WORDS = "Thirty-seven"
HAS_MEMBER = False
TIER = 0
KIT = ("foundationBlock", "Foundation block", "Fundamentblock", "layers", "Plate")
PREV = dict(ent="JointPlate", snake_pl="joint_plates", snake="joint_plate", table="en1993_joint_plate", ts_type="En1993SqliteJointPlate", t_index=35, field_id=27, words="Thirty-six", ntables=36, kit="jointPlate", kebab_pl="joint-plates")
EPREV = PREV
BOLT_CLASS = [("4.6", "4.6", "4.6"), ("5.6", "5.6", "5.6"), ("8.8", "8.8", "8.8"), ("10.9", "10.9", "10.9")]
CHOICE_CONST = {"anchor_class": "FOUNDATION_ANCHOR_CLASS"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("joint_id", "String", "", "", "", "Base plate joint id", "Fußplatten-Anschluss-Id"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("block_length", "f64", 0.0, 0.9, "m", "Block length a_2 (along the plate length)", "Blocklänge a_2 (entlang der Plattenlänge)"),
    f("block_width", "f64", 0.0, 0.9, "m", "Block width b_2", "Blockbreite b_2"),
    f("block_depth", "f64", 0.0, 0.8, "m", "Block depth h (load spread height)", "Blockhöhe h (Lastausbreitungshöhe)"),
    f("concrete_fck", "f64", 0.0, 30000000.0, "Pa", "Concrete characteristic strength f_ck", "Charakteristische Betondruckfestigkeit f_ck"),
    f("grout_fck", "f64", 0.0, 40000000.0, "Pa", "Grout characteristic strength", "Charakteristische Festigkeit des Vergussmörtels"),
    f("grout_thickness", "f64", 0.0, 0.03, "m", "Grout thickness t_g", "Dicke des Vergussmörtels t_g"),
    f("anchor_count", "u32", 0, 2, "", "Anchors in tension", "Zugbeanspruchte Anker"),
    f("anchor_class", "String", "", "8.8", "", "Anchor class", "Festigkeitsklasse der Anker", BOLT_CLASS),
    f("anchor_diameter", "f64", 0.0, 0.024, "m", "Anchor diameter d", "Ankerdurchmesser d"),
    f("anchor_stress_area", "f64", 0.0, 0.000353, "m2", "Anchor stress area A_s", "Ankerspannungsquerschnitt A_s"),
    f("anchor_embedment", "f64", 0.0, 0.4, "m", "Anchor embedment depth h_ef", "Verankerungstiefe h_ef"),
    f("anchor_edge_distance", "f64", 0.0, 0.2, "m", "Anchor edge distance to the block edge", "Randabstand der Anker zum Blockrand"),
]
SHOW_TEXT = {"id": "block-1", "joint_id": "joint-bp", "label": "Foundation block", "anchor_class": "8.8"}
