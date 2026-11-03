"""Spec of the EN 1993 `FrameBar` entity (member of a general planar frame between two frame nodes with the section and material of a steel member, EN 1993-1-1 5.2.1). Executed by en1993-entity-gen.py."""
ENT = "FrameBar"
ENT_DE = "Rahmenstab"
SNAKE = "frame_bar"
SNAKE_PL = "frame_bars"
KEBAB_PL = "frame-bars"
TABLE = "frame_bar"
TS_TYPE = "FrameBar"
EMOJI = "🪢"
DOC = "🪢 Bar of a general planar frame between two frame nodes; section and material come from the referenced steel member, end releases make hinges."
COLL_EN, COLL_DE = "Frame bars", "Rahmenstäbe"
NTABLES = 34
WORDS = "Thirty-four"
HAS_MEMBER = True
TIER = 0
KIT = ("frameBar", "Frame bar", "Rahmenstab", "minus", "Member")
PREV = dict(ent="FrameNode", snake_pl="frame_nodes", snake="frame_node", table="en1993_frame_node", ts_type="En1993SqliteFrameNode", t_index=32, field_id=24, words="Thirty-three", ntables=33, kit="frameNode", kebab_pl="frame-nodes")
RELEASE = [("rigid", "Rigid", "Biegesteif"), ("pinned", "Hinge", "Gelenk")]
CHOICE_CONST = {"release_start": "FRAME_BAR_RELEASE", "release_end": "FRAME_BAR_RELEASE"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("member_id", "String", "", "", "", "Steel member id (section, material)", "Stahlbauteil-Id (Querschnitt, Werkstoff)"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("start_node_id", "String", "", "", "", "Start node id", "Anfangsknoten-Id"),
    f("end_node_id", "String", "", "", "", "End node id", "Endknoten-Id"),
    f("release_start", "String", "", "rigid", "", "Start connection", "Anschluss am Anfang", RELEASE),
    f("release_end", "String", "", "rigid", "", "End connection", "Anschluss am Ende", RELEASE),
]
SHOW_TEXT = {"id": "bar-1", "member_id": "member-1", "label": "Left column", "start_node_id": "node-a", "end_node_id": "node-b", "release_start": "rigid", "release_end": "rigid"}
