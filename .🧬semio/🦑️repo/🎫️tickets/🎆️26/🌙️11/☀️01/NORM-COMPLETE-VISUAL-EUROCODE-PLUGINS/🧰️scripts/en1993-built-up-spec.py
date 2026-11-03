"""Spec of the EN 1993 `BuiltUpMember` entity (laced, battened and closely spaced built-up compression members, V-130..V-133, EN 1993-1-1 6.4). Executed by en1993-entity-gen.py."""
ENT = "BuiltUpMember"
ENT_DE = "Mehrteiliger Stab"
SNAKE = "built_up_member"
SNAKE_PL = "built_up_members"
KEBAB_PL = "built-up-members"
TABLE = "built_up_member"
TS_TYPE = "BuiltUp"
EMOJI = "🪜"
DOC = "🪜 Built-up compression member: laced, battened or closely spaced chords (EN 1993-1-1 6.4)."
COLL_EN, COLL_DE = "Built-up members", "Mehrteilige Stäbe"
NTABLES = 28
WORDS = "Twenty-eight"
HAS_MEMBER = True
TIER = 0
KIT = ("builtUpMember", "Built-up member", "Mehrteiliger Stab", "layers", "Group")
PREV = dict(ent="Restraint", snake_pl="restraints", snake="restraint", table="en1993_restraint", ts_type="En1993SqliteRestraint", t_index=26, field_id=18, words="Twenty-seven", ntables=27, kit="restraint", kebab_pl="restraints")
KIND = [("laced", "Laced (single bracing system)", "Gitterstab (einfaches Strebenfachwerk)"), ("battened", "Battened", "Rahmenstab"), ("closely", "Closely spaced", "Eng gestellter Stab")]
CHOICE_CONST = {"kind": "BUILT_UP_KIND"}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
FIELDS = [
    f("id", "String", "", "", "", "Id", "Id"),
    f("member_id", "String", "", "", "", "Member id", "Bauteil-Id"),
    f("label", "String", "", "", "", "Label", "Bezeichnung"),
    f("kind", "String", "", "battened", "", "Kind of built-up member", "Art des mehrteiligen Stabes", KIND),
    f("chord_area", "f64", 0.0, 0.00538, "m2", "Chord area A_ch", "Gurtfläche A_ch"),
    f("chord_inertia", "f64", 0.0, 1.336e-5, "m4", "Chord second moment I_ch (in plane)", "Gurt-Flächenmoment I_ch (in der Ebene)"),
    f("chord_distance", "f64", 0.0, 0.30, "m", "Chord distance h_0", "Gurtabstand h_0"),
    f("panel_spacing", "f64", 0.0, 0.80, "m", "Connection spacing a", "Abstand der Bindebleche a"),
    f("planes", "u32", 0, 2, "", "Planes of lacing or battens n", "Ebenen der Vergitterung oder Bindebleche n"),
    f("batten_inertia", "f64", 0.0, 5.0e-6, "m4", "Batten second moment I_b", "Bindeblech-Flächenmoment I_b"),
    f("diagonal_area", "f64", 0.0, 0.0, "m2", "Diagonal area A_d", "Diagonalenfläche A_d"),
    f("diagonal_length", "f64", 0.0, 0.0, "m", "Diagonal length d", "Diagonalenlänge d"),
    f("design_axial", "f64", 0.0, 400000.0, "N", "Design axial force N_Ed", "Bemessungsnormalkraft N_Ed"),
    f("design_moment", "f64", 0.0, 5000.0, "Nm", "First-order moment M_Ed", "Moment nach Theorie I. Ordnung M_Ed"),
    f("back_to_back", "u32", 0, 0, "", "Back-to-back angles (0/1)", "Rücken an Rücken gestellte Winkel (0/1)"),
    f("min_radius", "f64", 0.0, 0.0, "m", "Minimum radius of gyration i_min", "Kleinster Trägheitsradius i_min"),
    f("chain_spacing", "f64", 0.0, 0.0, "m", "Connection spacing of closely spaced members", "Verbindungsabstand eng gestellter Stäbe"),
]
SHOW_TEXT = {"id": "built-up-1", "member_id": "member-b1", "label": "Built-up 1"}
