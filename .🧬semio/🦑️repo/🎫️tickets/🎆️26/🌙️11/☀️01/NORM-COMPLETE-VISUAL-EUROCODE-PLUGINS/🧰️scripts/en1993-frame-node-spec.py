"""Spec of the EN 1993 `FrameNode` entity (node, support and nodal load of a general planar frame for the mechanics-kernel eigenvalue alpha_cr, EN 1993-1-1 5.2.1; V-020..V-027). Executed by en1993-entity-gen.py."""
ENT = "FrameNode"
ENT_DE = "Rahmenknoten"
SNAKE = "frame_node"
SNAKE_PL = "frame_nodes"
KEBAB_PL = "frame-nodes"
TABLE = "frame_node"
TS_TYPE = "FrameNode"
EMOJI = "🕸"
DOC = "🕸 Node of a general planar frame in the x-z plane (z up) with its support and nodal design loads: input of the mechanics-kernel eigenvalue α_cr (EN 1993-1-1 5.2.1)."
COLL_EN, COLL_DE = "Frame nodes", "Rahmenknoten"
NTABLES = 33
WORDS = "Thirty-three"
HAS_MEMBER = False
TIER = 0
KIT = ("frameNode", "Frame node", "Rahmenknoten", "anchor", "Group")
PREV = dict(ent="JointRow", snake_pl="joint_rows", snake="joint_row", table="en1993_joint_row", ts_type="En1993SqliteJointRow", t_index=31, field_id=23, words="Thirty-two", ntables=32, kit="jointRow", kebab_pl="joint-rows")
SUPPORT = [("free", "Free", "Frei"), ("pinned", "Pinned (translations fixed, rotation free)", "Gelenkig (Verschiebungen gehalten, Drehung frei)"), ("fixed", "Fixed (translations and rotation fixed)", "Eingespannt (Verschiebungen und Drehung gehalten)"), ("roller", "Roller (vertical translation fixed)", "Rollenlager (vertikale Verschiebung gehalten)")]
CHOICE_CONST = {"support": "FRAME_NODE_SUPPORT"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("x", "f64", 0.0, 0.0, "m", "Coordinate x", "Koordinate x"),
    f("z", "f64", 0.0, 0.0, "m", "Coordinate z (up)", "Koordinate z (nach oben)"),
    f("support", "String", "", "fixed", "", "Support", "Lagerung", SUPPORT),
    f("load_x", "f64", 0.0, 0.0, "N", "Design load in x", "Bemessungslast in x"),
    f("load_z", "f64", 0.0, 0.0, "N", "Design load in z (gravity negative)", "Bemessungslast in z (Schwerkraft negativ)"),
]
SHOW_TEXT = {"id": "node-a", "label": "Base left", "support": "fixed"}
