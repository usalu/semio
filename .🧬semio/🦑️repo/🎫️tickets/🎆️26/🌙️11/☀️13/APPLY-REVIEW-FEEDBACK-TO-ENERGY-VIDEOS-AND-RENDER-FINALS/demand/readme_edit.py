import sys
from pathlib import Path

P = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\README.md")
s = P.read_text(encoding="utf-8")


def sub(old, new):
    global s
    if old not in s:
        sys.exit(f"missing {old[:70]!r}")
    s = s.replace(old, new)


sub("| Bilanz | `Q_h = Q_Senke − η_h · Q_Quelle` | Kapitel 1, from the Heating/Cooling flows |",
    "| Bilanz | `Q_h = Q_l − η · Q_g` (`Q_l = Q_T + Q_V`, `Q_g = Q_S + Q_i`, DIN V 4108-6) | Kapitel 1, from the Heating/Cooling flows |")
sub("| Verfahren | 12 Monatsbilanzen, Klimadaten, `G_t` | Kapitel 2 |",
    "| Verfahren | 12 Monatsbilanzen, Klimadaten, `G_t20/15` (bezogen auf `θ_i = 20 °C`, Heizgrenze `θ_HG = 15 °C`), `F_Gt = 0,024 · G_t` | Kapitel 2 |")
sub("| Leistung ≠ Energie | `Φ_HL [kW]` vs. `Q_h [kWh/a]` | Kapitel 3 |",
    "| Leistung ≠ Energie | `Q̇_H in kW` (Norm: `Φ_HL`) vs. `Q_h in kWh/a` | Kapitel 3 |")
sub("| Anlagenkette | `Q_E = Q_h · e_p` | Kapitel 4 |", "| Anlagenkette | Nutzenergie `Q_h` → Endenergie `Q_E = Q_h · e_E` | Kapitel 4 |")
sub("| Vorkette | `Q_P = Q_E · f_p` | Kapitel 5 |", "| Vorkette | Endenergie `Q_E` → Primärenergie `Q_P = Q_E · f_P` (`e_P = e_E · f_P`) | Kapitel 5 |")
sub("The building blocks of `Q_V` (airflow `V̇ = V·n`, `c_Luft`, `Δθ`, `η_WRG`)",
    "The building blocks of `Q_V` (airflow `V̇ = V·n`, `ρ·c_p = 0,34 Wh/(m³K)` for `ρ ≈ 1,2 kg/m³` at 20 °C, `Δθ`, `η_WRG`)")
sub("Q_h  = 100 kWh/(m²·a)\n  · e_p = 1,04 · 1,08 · 1,06 · 1,18 ≈ 1,40 (Gaskessel)  → Q_E = 140 → · f_p = 1,1 → Q_P = 154 → Klasse E\n  · e_p = 1,04 · 1,08 · 1,06 · 0,26 ≈ 0,31 (Wärmepumpe) → Q_E =  31 → · f_p = 1,8 → Q_P ≈  56 → Klasse A",
    "Q_h  = 100 kWh/(m²·a)   (Nutzenergie)\n  · e_E = 1,04 · 1,08 · 1,06 · 1,18 ≈ 1,40 (Gaskessel)  → Q_E = 140 (Endenergie) → · f_P = 1,1 → Q_P = 154 (Primärenergie) → Klasse E\n  · e_E = 1,04 · 1,08 · 1,06 · 0,26 ≈ 0,31 (Wärmepumpe) → Q_E =  31 (Endenergie) → · f_P = 1,8 → Q_P ≈  56 (Primärenergie) → Klasse A")
sub("| `e_ü`, `e_d`, `e_s`, `e_g` and `Q_E` | `♨️system-losses/🐍️.py` (`E_*`, `Q_E_GAS`, `Q_E_WP`) | `🏭️primary-energy/🐍️.py` (`E_P_*`) |",
    "| `e_ce`, `e_d`, `e_s`, `e_g` (DIN V 4701-10: Übergabe, Verteilung, Speicherung, Erzeugung) and `Q_E` | `♨️system-losses/🐍️.py` (`E_*`, `E_E_*`, `Q_E_GAS`, `Q_E_WP`) | `🏭️primary-energy/🐍️.py` (`E_E_*`) |")
sub("| `f_p` (GEG Anlage 4), `Q_P`, example GEG limit |", "| `f_P` (GEG Anlage 4), `Q_P`, example GEG limit |")
sub("Standards cited per beat via the `_din_ref()` chip, with editions where it\nmatters: GEG (§ 15/16, § 20, § 60c, § 71, § 79–87, Anlagen 1/4/10),\nDIN V 18599:2018-09 (Teile 1/2/5/6/10) and its 2025 continuation DIN/TS 18599,\nDIN EN 12831-1:2017-09 / DIN/TS 12831-1:2020-04, DIN 4108-2/-6, VDI 2078,\nVDI 3807, VDI 4645, VDI 4650-1:2024, VOB/C DIN 18380.",
    "Standards cited per beat via the `_din_ref()` chip, always with edition and locator (verified 2026-11-13):\n"
    "GEG, since 29 July 2026 Gebäudemodernisierungsgesetz (GModG): § 15 Abs. 1 and Anlage 1 (reference building), § 20 Abs. 1 (DIN V 18599:2018-09), § 22 Abs. 1 and Anlage 4 (primary energy factors), §§ 79–87 and Anlage 10 (Energieausweis); the 65-%-rule of §§ 71–73 is repealed.\n"
    "DIN V 4108-6:2003-06: 3.2 symbols, 5.5 balance methods, 6.1 transmission, 6.2 ventilation, 6.3 internal, 6.4 solar, 6.5 utilisation, Anhang A meteorological data, D.2 Heizperiodenbilanz (`Q_l = 0,024 · (H_T + H_V) · G_t`), D.5 Referenzklima.\n"
    "DIN V 18599:2018-09 (Teile 1/2/5/6/10; 2.: 5.5.3 Ausnutzungsgrad) with its successor DIN/TS 18599:2025-10; DIN V 4701-10:2003-08 (symbols `Q_h`, `Q_E`, `Q_P`, `e_P`); DIN EN 12831-1:2017-09 Abschn. 6 with DIN/TS 12831-1:2020-04; "
    "VDI 2078:2015-06; DIN 4108-2:2026-05 (replaces 2013-02); VDI 4645:2023-04; VDI 4650 Blatt 1:2024-02; VOB/C DIN 18380:2019-09.")
sub("| 1 | `Beat2_Verlustseite` | `Q_T + Q_V`, Wärmebrückenzuschlag |", "| 1 | `Beat2_Verlustseite` | `Q_l = Q_T + Q_V` (Übergangsmonat, `Δθ = 10 K`), Wärmebrückenzuschlag |")
sub("| 1 | `Beat3_Gewinnseite` | `Q_S + Q_I`, Ausnutzungsgrad `η_h` |", "| 1 | `Beat3_Gewinnseite` | `Q_g = Q_S + Q_i`, Ausnutzungsgrad `η` |")
sub("| 1 | `Beat4_WinterUndSommer` | dieselbe Bilanz, umgekehrtes Vorzeichen |", "| 1 | `Beat4_WinterUndSommer` | dieselbe Bilanz, `Q_h = Q_l − η · Q_g` im Januar, `Q_c = Q_g − η_c · Q_l` im Juli |")
sub("| 2 | `Beat3_Klimadaten` | `θ_e`, `I_S`, Testreferenzjahr |", "| 2 | `Beat3_Klimadaten` | `θ_e`, `I_s`, Testreferenzjahr |")
sub("| 2 | `Beat4_Gradtagszahl` | Heizgrenze, `G_t`, Witterungsbereinigung |", "| 2 | `Beat4_Gradtagszahl` | Innentemperatur `θ_i`, Heizgrenze, `G_t20/15`, `F_Gt` |")
sub("| 3 | `Beat2_Normheizlast` | DIN EN 12831-1, Gewinne bewusst ignoriert |", "| 3 | `Beat2_Normheizlast` | `Q̇_H` (Norm: `Φ_HL`) aus `(H_T + H_V) · Δθ`, Gewinne bewusst ignoriert |")
sub("| 4 | `Beat4_Anlagenaufwandszahl` | `e_p = e_ü · e_d · e_s · e_g` |", "| 4 | `Beat4_Anlagenaufwandszahl` | `e_E = e_ce · e_d · e_s · e_g` |")
sub("| 5 | `Beat2_Primaerenergiefaktor` | `f_p` je Energieträger |", "| 5 | `Beat2_Primaerenergiefaktor` | `f_P` je Energieträger |")
sub("a **schematic** reference-building value of 120 (GEG § 15).", "a **schematic** reference-building value of 120 (GEG § 15 Abs. 1, Anlage 1).")
P.write_text(s, encoding="utf-8")
print("ok")
