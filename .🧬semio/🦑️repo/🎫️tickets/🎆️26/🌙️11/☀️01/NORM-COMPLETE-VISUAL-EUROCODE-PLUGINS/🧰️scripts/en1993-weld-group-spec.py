"""Spec of the EN 1993 `WeldGroup` entity (eccentrically loaded weld group, elastic vector method, EN 1993-1-8 4.5.3.3). Executed by en1993-entity-gen.py."""
ENT = "WeldGroup"
ENT_DE = "Schweißnahtgruppe"
SNAKE = "weld_group"
SNAKE_PL = "weld_groups"
KEBAB_PL = "weld-groups"
TABLE = "weld_group"
TS_TYPE = "WeldGroup"
EMOJI = "🧵"
DOC = "🧵 Weld group under in-plane shear and torsion: elastic vector method (EN 1993-1-8 4.5.3.3)."
COLL_EN, COLL_DE = "Weld groups", "Schweißnahtgruppen"
NTABLES = 29
WORDS = "Twenty-nine"
HAS_MEMBER = True
TIER = 0
KIT = ("weldGroup", "Weld group", "Schweißnahtgruppe", "link", "Group")
PREV = dict(ent="BuiltUpMember", snake_pl="built_up_members", snake="built_up_member", table="en1993_built_up_member", ts_type="En1993SqliteBuiltUp", t_index=27, field_id=19, words="Twenty-eight", ntables=28, kit="builtUpMember", kebab_pl="built-up-members")
PATTERN = [("line", "Single line", "Einzelnaht"), ("twoLines", "Two parallel lines", "Zwei parallele Nähte"), ("box", "Box (four sides)", "Rechteckumfang (vier Seiten)")]
CHOICE_CONST = {"pattern": "WELD_GROUP_PATTERN"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("member_id", "String", "", "", "", "Member id", "Bauteil-Id"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("pattern", "String", "", "twoLines", "", "Weld pattern", "Nahtanordnung", PATTERN),
    f("throat", "f64", 0.0, 0.005, "m", "Throat thickness a", "Nahtdicke a"),
    f("width", "f64", 0.0, 0.10, "m", "Group width b", "Gruppenbreite b"),
    f("depth", "f64", 0.0, 0.20, "m", "Group depth d", "Gruppentiefe d"),
    f("shear_force", "f64", 0.0, 100000.0, "N", "Shear force F_Ed along the depth", "Querkraft F_Ed in Tiefenrichtung"),
    f("eccentricity", "f64", 0.0, 0.15, "m", "Eccentricity e of the force", "Ausmitte e der Kraft"),
    f("steel_fu", "f64", 0.0, 510.0e6, "Pa", "Ultimate strength f_u of the weaker part", "Zugfestigkeit f_u des schwächeren Teils"),
    f("beta_w", "f64", 0.9, 0.9, "1", "Correlation factor β_w", "Korrelationsbeiwert β_w"),
]
SHOW_TEXT = {"id": "weld-group-1", "member_id": "member-b1", "label": "Weld group 1"}
