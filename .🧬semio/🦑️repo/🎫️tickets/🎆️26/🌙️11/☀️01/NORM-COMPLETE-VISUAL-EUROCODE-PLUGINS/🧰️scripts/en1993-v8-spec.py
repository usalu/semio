"""Field spec of EN 1993 wave 2 v8: the warping support of a member for the bimoment row (V-066). Imported by `en1993-special-gen.py en1993-v8-spec.py`."""
import pathlib as _pl
exec(compile((_pl.Path(__file__).resolve().parent / "en1993-special-spec.py").read_text(encoding="utf8").split("CH = {")[0], "units", "exec")) if "__file__" in globals() else None
TAG = "V8"
CH = {
    "WARPING_SUPPORT": [("fork", "Fork supports (warping free)", "Gabellagerung (Verwölbung frei)"), ("fixed", "Both ends warping-restrained", "Beidseitig wölbbehindert"), ("fixed-fork", "Warping-restrained at the start, fork at the end", "Am Anfang wölbbehindert, am Ende Gabel"), ("cantilever", "Cantilever (restrained at the start, free end)", "Kragarm (am Anfang eingespannt, Ende frei)")],
}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
SPEC = {
    "SteelMember": dict(collection="members", patch="En1993MemberPatch", fields=[
        f("warping_support", "String", "fork", "fork", "", "Torsion and warping end conditions of the member for the bimoment row (EN 1993-1-1 6.2.7): fork, both ends restrained, restrained at the start with fork end, or cantilever", "Torsions- und Wölblagerung der Bauteilenden für die Bimomentzeile (EN 1993-1-1 6.2.7): Gabel, beidseitig eingespannt, Anfang eingespannt mit Gabelende oder Kragarm", "WARPING_SUPPORT"),
    ]),
}
