"""Field spec of the EN 1993-1-1 6.4 built-up member chord outline (wave 2 v6): the chord cross-section as a dimensioned outline so the mechanics kernel `analyse_section` computes I_t, I_w and the shear centre of the chord. Appended snapshot fields on `BuiltUpMember`; imported by `en1993-special-gen.py en1993-built-up-outline-spec.py`."""
import pathlib as _pl
exec(compile((_pl.Path(__file__).resolve().parent / "en1993-special-spec.py").read_text(encoding="utf8").split("CH = {")[0], "units", "exec")) if "__file__" in globals() else None
TAG = "BUILTUPOUTLINE"
CH = {
    "CHORD_SHAPE": [("", "None (no outline)", "Keine (keine Kontur)"), ("channel", "Channel (U)", "U-Profil"), ("i", "I section", "I-Profil"), ("rhs", "Rectangular hollow section", "Rechteckhohlprofil"), ("chs", "Circular hollow section", "Kreishohlprofil")],
}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
SPEC = {
    "BuiltUpMember": dict(collection="builtUpMembers", patch="En1993BuiltUpMemberPatch", fields=[
        f("chord_shape", "String", "", "channel", "", "Chord section outline shape (sharp-corner outline for I_t and I_w of the chord)", "Konturform des Gurtquerschnitts (scharfkantige Kontur für I_t und I_w des Gurts)", "CHORD_SHAPE"),
        f("chord_depth", "f64", 0.0, 0.30, "m", "Chord outline depth h", "Konturhöhe h des Gurts"),
        f("chord_width", "f64", 0.0, 0.10, "m", "Chord outline width b", "Konturbreite b des Gurts"),
        f("chord_flange_thickness", "f64", 0.0, 0.0165, "m", "Chord outline flange thickness t_f (wall thickness of hollow sections)", "Flanschdicke t_f der Gurtkontur (Wanddicke bei Hohlprofilen)"),
        f("chord_web_thickness", "f64", 0.0, 0.010, "m", "Chord outline web thickness t_w", "Stegdicke t_w der Gurtkontur"),
    ]),
}
