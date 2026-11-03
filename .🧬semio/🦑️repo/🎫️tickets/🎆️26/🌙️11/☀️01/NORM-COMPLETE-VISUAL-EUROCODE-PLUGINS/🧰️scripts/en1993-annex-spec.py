"""Field spec of the EN 1993 annex blocks (v4): ring-stiffened shells under external pressure (1-6), corrugated webs (1-5 Annex D), orthotropic decks (EN 1993-2 9.4) and purlins restrained by sheeting (1-3). Appended snapshot fields per existing entity; imported by en1993-special-gen.py (python3 -I en1993-special-gen.py en1993-annex-spec.py)."""
import pathlib as _pl
exec(compile((_pl.Path(__file__).resolve().parent / "en1993-special-spec.py").read_text(encoding="utf8").split("CH = {")[0], "units", "exec")) if "__file__" in globals() else None
TAG = "ANNEX"
CH = {}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
SPEC = {
    "SiloShell": dict(collection="siloShells", patch="En1993SiloShellPatch", fields=[
        f("ring_spacing", "f64", 0.0, 1.5, "m", "Ring stiffener spacing l_r (0 = unstiffened, L governs)", "Ringsteifenabstand l_r (0 = unausgesteift, L maßgebend)"),
        f("ring_area", "f64", 0.0, 0.0015, "m2", "Ring stiffener area A_r", "Ringsteifenfläche A_r"),
        f("ring_inertia", "f64", 0.0, 2.0e-6, "m4", "Ring stiffener second moment I_r about its own centroid", "Ringsteifen-Flächenträgheitsmoment I_r um den eigenen Schwerpunkt"),
        f("ring_offset", "f64", 0.0, 0.08, "m", "Ring centroid offset e_r from the shell mid-surface (outwards positive)", "Ringschwerpunktabstand e_r von der Schalenmittelfläche (nach außen positiv)"),
    ]),
    "PlatedPanel": dict(collection="platedPanels", patch="En1993PlatedPanelPatch", fields=[
        f("corrugation_flat", "f64", 0.0, 0.14, "m", "Corrugated web: flat fold width a_0 (0 = plane web)", "Trapezstegblech: ebene Faltenbreite a_0 (0 = ebener Steg)"),
        f("corrugation_projection", "f64", 0.0, 0.04, "m", "Corrugated web: horizontal projection a_1 of the inclined fold", "Trapezstegblech: waagerechte Projektion a_1 der geneigten Falte"),
        f("corrugation_depth", "f64", 0.0, 0.05, "m", "Corrugated web: corrugation depth d", "Trapezstegblech: Wellentiefe d"),
        f("web_height", "f64", 0.0, 1.0, "m", "Corrugated web height h_w", "Trapezstegblech: Steghöhe h_w"),
    ]),
    "BridgeFatigue": dict(collection="bridgeFatigue", patch="En1993BridgeFatiguePatch", fields=[
        f("deck_thickness", "f64", 0.0, 0.014, "m", "Orthotropic deck plate thickness t_d (0 = no deck)", "Fahrbahnblechdicke t_d (0 = keine Fahrbahnplatte)"),
        f("trough_spacing", "f64", 0.0, 0.6, "m", "Trough spacing e (centre to centre)", "Trogabstand e (Mitte zu Mitte)"),
        f("trough_width", "f64", 0.0, 0.3, "m", "Trough width between the deck welds", "Trogbreite zwischen den Deckblechnähten"),
        f("trough_thickness", "f64", 0.0, 0.006, "m", "Trough wall thickness t_t", "Trogwanddicke t_t"),
        f("crossbeam_spacing", "f64", 0.0, 3.0, "m", "Cross beam spacing a", "Querträgerabstand a"),
        f("surfacing_thickness", "f64", 0.0, 0.07, "m", "Surfacing thickness t_s", "Belagsdicke t_s"),
        f("wheel_load", "f64", 0.0, 60000.0, "N", "Wheel load Q_w", "Radlast Q_w"),
        f("wheel_patch", "f64", 0.0, 0.4, "m", "Wheel contact patch side", "Radaufstandsseite"),
    ]),
    "ColdFormedMember": dict(collection="coldFormedMembers", patch="En1993ColdFormedMemberPatch", fields=[
        f("purlin_span", "f64", 0.0, 6.0, "m", "Purlin span L (0 = not a purlin)", "Pfettenspannweite L (0 = keine Pfette)"),
        f("purlin_load", "f64", 0.0, 2500.0, "N/m", "Purlin vertical design load q_Ed", "Pfetten-Bemessungsstreckenlast q_Ed"),
        f("purlin_lateral_ratio", "f64", 0.0, 0.15, "1", "Lateral to vertical load ratio k_h on the free flange", "Verhältnis k_h seitliche zu lotrechter Last am freien Gurt"),
    ]),
}
