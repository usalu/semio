"""Field spec of the EN 1993-1-5 Annex A stiffened plate (wave 2 v5): the spacing of equally spaced longitudinal stiffeners of a plated panel. Appended snapshot field per existing entity; imported by `en1993-special-gen.py en1993-stiffened-spec.py`."""
import pathlib as _pl
exec(compile((_pl.Path(__file__).resolve().parent / "en1993-special-spec.py").read_text(encoding="utf8").split("CH = {")[0], "units", "exec")) if "__file__" in globals() else None
TAG = "STIFFENED"
CH = {}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
SPEC = {
    "PlatedPanel": dict(collection="platedPanels", patch="En1993PlatedPanelPatch", fields=[
        f("stiffener_spacing", "f64", 0.0, 0.25, "m", "Longitudinal stiffener spacing s (equal subpanels, n = b/s - 1 stiffeners; 0 = the single stiffener of the stiffener fields)", "Längssteifenabstand s (gleiche Teilfelder, n = b/s - 1 Steifen; 0 = die einzelne Steife der Steifenfelder)"),
    ]),
}
