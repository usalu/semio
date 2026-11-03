"""Field spec of EN 1993 wave 2 v7: the stiffener outline of a plated panel (I_t, I_w, I_p by the mechanics kernel), the lateral plate pressure (V-805), the web material of a hybrid girder (V-034), the torsion load of a member (V-066 bimoment) and the stress history of a fatigue detail (V-504 rainflow). Appended snapshot fields per existing entity; imported by `en1993-special-gen.py en1993-v7-spec.py`."""
import pathlib as _pl
exec(compile((_pl.Path(__file__).resolve().parent / "en1993-special-spec.py").read_text(encoding="utf8").split("CH = {")[0], "units", "exec")) if "__file__" in globals() else None
TAG = "V7"
CH = {
    "STIFFENER_SHAPE": [("", "None (no outline)", "Keine (keine Kontur)"), ("flat", "Flat bar", "Flachstahl"), ("tee", "T section", "T-Profil"), ("angle", "Angle", "Winkel")],
    "TORSION_LOAD": [("point", "Point torque", "Einzeltorsionsmoment"), ("udl", "Uniform torque", "Gleichmäßig verteiltes Torsionsmoment")],
}
f = lambda name, ty, default, show, unit, en, de, choices=None: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=choices)
SPEC = {
    "PlatedPanel": dict(collection="platedPanels", patch="En1993PlatedPanelPatch", fields=[
        f("stiffener_shape", "String", "", "tee", "", "Transverse stiffener outline shape (centreline walls for I_T, I_w and I_p of the stiffener)", "Konturform der Quersteife (Mittellinienwände für I_T, I_w und I_p der Steife)", "STIFFENER_SHAPE"),
        f("stiffener_depth", "f64", 0.0, 0.12, "m", "Stiffener web or leg height h_s, measured from the plate surface", "Steghöhe bzw. Schenkellänge h_s der Steife, ab Blechoberfläche"),
        f("stiffener_thickness", "f64", 0.0, 0.010, "m", "Stiffener web or leg thickness t_s", "Stegdicke bzw. Schenkeldicke t_s der Steife"),
        f("stiffener_flange_width", "f64", 0.0, 0.08, "m", "Stiffener flange width or second leg length b_f", "Flanschbreite bzw. zweite Schenkellänge b_f der Steife"),
        f("stiffener_flange_thickness", "f64", 0.0, 0.012, "m", "Stiffener flange or second leg thickness t_f", "Flanschdicke bzw. zweite Schenkeldicke t_f der Steife"),
        f("lateral_pressure", "f64", 0.0, 20000.0, "Pa", "Uniform lateral pressure p on the panel (EN 1993-1-7, perpendicular to the plate)", "Gleichmäßiger Querdruck p auf das Blech (EN 1993-1-7, senkrecht zur Blechebene)"),
        f("lateral_pressure_coefficient", "f64", 0.0, 0.5, "1", "Plate bending coefficient k of sigma = k p b²/t² for the panel aspect ratio and edge conditions (Timoshenko or Roark table, entered; table values not verified)", "Plattenbeiwert k von sigma = k p b²/t² für Seitenverhältnis und Lagerung (Tabelle nach Timoshenko oder Roark, eingegeben; Tabellenwerte ungeprüft)"),
    ]),
    "SteelMember": dict(collection="members", patch="En1993MemberPatch", fields=[
        f("web_material_id", "String", "", "", "", "Web material of a hybrid girder (empty = the member material; the flanges keep the member material)", "Stegwerkstoff eines Hybridträgers (leer = Bauteilwerkstoff; die Gurte behalten den Bauteilwerkstoff)"),
        f("torsion_load", "String", "point", "point", "", "Application of the torsion action on a fork-supported member (point torque at the position below or uniform torque)", "Anordnung des Torsionsmoments am gabelgelagerten Bauteil (Einzeltorsionsmoment an der Stelle unten oder gleichmäßig verteilt)", "TORSION_LOAD"),
        f("torque_position", "f64", 0.5, 0.5, "1", "Position of the point torque as a fraction of the member length", "Lage des Einzeltorsionsmoments als Bruchteil der Bauteillänge"),
    ]),
    "FatigueDetail": dict(collection="fatigueDetails", patch="En1993FatigueDetailPatch", sqlite=False, fields=[
        f("stress_history", "String", "", "", "", "Stress history Δσ in N/mm² as numbers separated by spaces; rainflow counted into the blocks of the damage sum when the spectrum is empty", "Spannungsgeschichte σ in N/mm² als durch Leerzeichen getrennte Zahlen; wird bei leerem Kollektiv mit Rainflow in die Blöcke der Schadenssumme gezählt"),
    ]),
}
