# Verifikation: Physikalische Grundlagen (final, de) – Feedback F00–F29 (Teilmenge)

Read-only Prüfung. Keine Quelldatei geändert, nichts gerendert.

- Video: `.🧬semio/🎓️teaching/⚡️cache/videos/final/de/speaker/rendered/Full_Physical_Fundamentals_speaker_de_1080p30.mp4` (1920x1080, 30 fps, Dauer 18:12.37, mit Audio)
- Quelle: `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (3231 Zeilen, Stand 02:35, älter als das Video mit 08:01)
- Referenzen: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📚️references/🔣️.json`
- Roadmap-Szene: `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🗺️roadmap/🐍️.py`
- Frames (volle Auflösung): `TK/🗑️generated/fix-heat/verify-phys/t_<sek>.png`
- Kontaktbögen (alle 20 s): `TK/🗑️generated/verify-phys/sheets/sheet_01.png` bis `sheet_04.png`
- Ton nicht geprüft: Es gibt kein Transkript, deshalb sind gesprochene Formulierungen nicht belegt.

## Ergebnis je Feedback-Punkt

| ID | Verdict | Kurzbegründung |
|----|---------|----------------|
| F00 | APPLIED | Keine Klammer-Einheiten im Quelltext oder in den Bildern. Achsen und Werte zeigen `P in kW`, `θ in °C`, `Q in kJ`. |
| F01 | APPLIED | "Bohne 2022" ist weg. Schlussfolie nennt "Bohne, Bohne (2026), 13. Aufl.". JAZ ist über "VDI 4650 Blatt 1:2024-02" auffindbar. Einschränkung unten. |
| F02 | APPLIED | Endenergie (Brennstoff, Strom am Zähler) und Nutzwärme sind getrennt beschriftet. Balkendiagramm zeigt "Nutzwärme Q_N aus Q_E = 1kWh Endenergie". |
| F03 | APPLIED | Einziger Verweis ist die Schlussfolie: "Strahlung · Thermische Masse · Feuchte". Kein Kapitel oder Abschnitt, nur Stichworte. Einschränkung unten. |
| F06(1) | APPLIED | Das Video beginnt mit "Der Fahrplan des Videos" (t=0 und t=10 s). |
| F10/F22 | APPLIED (Stichprobe) | Alle geprüften Gleichungen tragen Einheiten an jeder Zahl. Brüche mit Bruchstrich. Ein kosmetischer Punkt bei `576W`. Nicht jedes Einzelbild geprüft. |
| F11 | APPLIED | Kein "Delta-Theta" und kein "Temperaturdifferenz" im Quelltext oder Bild. Sichtbar ist nur `Δθ` (t=880 s, t=1 s). Gesprochener Ton nicht geprüft. |

Nicht im Physik-Video betroffen: F04, F05, F07, F08, F09, F12–F21, F23–F29. Nicht geprüft.

## Einzelbelege

### F00 – Einheiten (global)
- Quelle: `grep` nach `[W]`, `[kWh]`, `\,[\mathrm{`, `\;[` liefert keine Treffer im Physik-Quelltext. Die einzigen Treffer sind Python-Code.
- Quelle Beispiele für die erlaubte Form: Zeile 640 `P\;\text{in}\;\;\mathrm{W}`, Zeile 772 `P\;\text{in}\;\;\mathrm{W}`, Zeile 2123 `I_{\mathrm{s}} = ... \frac{\mathrm{W}}{\mathrm{m^{2}}}`.
- Bild `t_120.png`: "E = P · t   P = E/t" und Achse "P in kW", "t in h".
- Bild `t_100.png`: "P = 0,7 kW", "t = 1,43 h = 86 min", "E = P · t = 1,00 kWh".
- Bild `t_580.png`: "Q_E = 1,34 kWh", "COP ≈ 3,0". Keine Klammer-Einheit.
- Achsenbeschriftungen aus den Kontaktbögen (Sheet 1 bis 4): "θ in °C", "Q in kJ", "Q̇_h in kW", "t in h".

### F01 – JAZ-Referenz
- Quelle `🧲️fundamentals/🐍️.py`:
  - Zeile 1710-1712 (Szene `Beat4_Waermepumpe`, Schlüssel `jaz`): "Übers Jahr gemittelt ergibt sich die Jahresarbeitszahl, kurz JAZ … rund 3."
  - Zeile 1713-1714 (Schlüssel `jaz_ref`): "Du findest sie als Jahresarbeitszahl in der Richtlinie VDI 4650 Blatt 1; das europäische Label nennt sie SCOP, DIN EN 14825."
  - Zeile 1921: `_references.cite('vdi-4650-1', 'din-en-14825')` mit "(SCOP)".
  - Zeile 3187: `willems, bohne = ... ("willems-2024", "bohne-2026")`, Schlusskarte.
- Referenzen-JSON: Eintrag `vdi-4650-1` mit Lokator `"jaz": "Jahresarbeitszahl (JAZ)"`, Ausgabe 2024-02. Eintrag `bohne-2026`: `editionNumber 13`, Jahr 2026.
- Bild `t_600.png`: Kopf rechts "Jahresarbeitszahl JAZ ≈ 3" und "VDI 4650 Blatt 1:2024-02 · DIN EN 14825:2023-10 (SCOP)".
- Bild `t_1085.png`: "Bohne, Bohne (2026), 13. Aufl." mit "Wärmepumpe · Lüftung · Anlagentechnik". "VDI 4650 Blatt 1:2024-02 · DIN EN 14825:2023-10 · DIN EN 410:2011-04", "Jahresarbeitszahl · g-Wert · Sonnenschutz · Wärmespeicherung".
- Einschränkung: Auf dem JAZ-Bild steht der Lokator "Jahresarbeitszahl (JAZ)" nicht. Die Richtlinie ist mit Nummer und Ausgabe genannt, die Stelle im Buch aber nicht. Das ist für eine Norm ausreichend, aber nicht ganz so wie die Konvention in §6 es formuliert.

### F02 – Endenergie vs. Wärme
- Quelle `🧲️fundamentals/🐍️.py`:
  - Zeile 1718-1721: "Endenergie ist, was ans Haus geliefert und bezahlt wird … Was die Räume tatsächlich erwärmt, ist die Nutzwärme … Endenergie und Nutzwärme sind nicht dasselbe."
  - Zeile 1724: "Je 1 kWh Endenergie $Q_{\mathrm{E}}$ liefert Holz rund 0,85 kWh Nutzwärme $Q_{\mathrm{N}}$, Öl 0,94, Erdgas 0,96."
  - Zeile 1981: "Nutzwärme Q_N aus Q_E = 1 kWh Endenergie" (Achsentitel).
  - Zeile 1968: Balkenbeschriftungen "Luft-WP (JAZ)", "Erd-WP (JAZ)".
- Bild `t_620.png`: Gebäudegrenze, "Endenergie Q_E" über "Brennstoff im Tank" und "Strom am Zähler", "Nutzwärme Q_N" im Haus. Untertitel: "Endenergie und Nutzwärme sind nicht dasselbe."
- Bild `t_640.png`: Balken 0,85 / 0,94 / 0,96 / 1,00 mit Legende "Brennstoff (Endenergie)", "Strom (Endenergie)".
- Bild `t_652.png`: "Abgas am Haus" und "Abgas im Kraftwerk".
- Bild `t_664.png`: Wärmepumpen-Balken mit "Umweltwärme" (3 und 4). Die Zahlen sind ohne Einheit, stehen aber unter dem Titel "aus Q_E = 1 kWh".

### F03 – "Willems et al. 2024"
- Quelle `🧲️fundamentals/🐍️.py` Zeilen 3187-3194: Buchkarte mit `tr("Radiation · Thermal mass · Moisture", "Strahlung · Thermische Masse · Feuchte")`.
- Referenzen-JSON `willems-2024`: `"locators": {}`. Kein Kapitel, keine Abschnittsnummer hinterlegt. Ausgabe `editionNumber 8`, 2024. Die ISBN-Verwechslung ist im JSON-Hinweis dokumentiert und behoben.
- Bild `t_1085.png`: "Willems, Schild, Stricker (2024), 8. Aufl." / "Formeln und Tabellen Bauphysik" / "Strahlung · Thermische Masse · Feuchte" / DOI-Link.
- Einschränkung: Das Stichwort sagt, was dort steht. Kapitel oder Abschnitt fehlt. Im Physik-Video gibt es keine weitere Stelle mit "Willems et al. 2024".

### F06(1) – Fahrplan am Anfang
- Quelle `🗺️roadmap/🐍️.py` Zeile 54: `HEADING = ("The roadmap of this video", "Der Fahrplan des Videos")`.
- Bild `t_10.png`: "Der Fahrplan des Videos", sechs Karten: Leistung/Energie, Erhaltung, Wärmepumpe, Strahlung, Therm. Masse, Sensibel/latent. Abblendung läuft noch.
- Kontaktbogen `sheet_01.png` Kachel 1 (t=0): Fahrplan ist das erste Bild.

### F10/F22 – Einheiten in Gleichungen, Brüche
- Quelle Beispiele mit Einheit an jeder Zahl: Zeile 1142-1143 `m=30 kg · c=4,19 kJ/(kgK) · ΔT=28 K = 3520 kJ`; Zeile 2396-2399 `V = 100 cm · 100 cm · 10 cm = 0,1 m³`, `m = ρ · V = 2400 kg/m³ · 0,1 m³ = 240 kg`.
- Bild `t_100.png`: "E = P · t = 1,00 kWh".
- Bild `t_240.png`: "1 kW · 1 h = 1 kWh".
- Bild `t_300.png`: "1kWh = 1kW · 3600s = 1 kJ/s · 3600s = 3600kJ" (Bruch kJ/s), und "3 520 kJ geteilt durch 3 600 kJ pro kWh".
- Bild `t_720.png`: "Q̇_S = g⊥ · A · I_s = 0,60 · 1,92 m² · 500 W/m² = 576W" (Bruch W/m²).
- Bild `t_780.png`: "m = ρ · V = 2400 kg/m³ · 0,1 m³ = 240 kg" (Bruch kg/m³).
- Bild `t_880.png`: "Q = m · c · Δθ = 1 kg · 4,19 kJ/kgK · 100 K ≈ 419 kJ" (Bruch kJ/kgK).
- Bild `t_860.png`: "Q = 210 kJ", "θ = 50 °C", "m = 1 kg" untereinander, jeweils mit Einheit.
- Kleiner Mangel: "576W" ohne Leerzeichen. Kein Verstoß gegen F10.

### F11 – "Δθ" statt "Delta-Theta"
- Quelle: `grep -i delta` und `grep -i theta` im Physik-Quelltext ergibt nur TeX-Ausdrücke (`\Delta\theta`, Zeilen 2401, 2430, 2812, 3159). Keine ausgeschriebene Form.
- Quelle: Das Wort "Temperaturdifferenz" kommt im Physik-Quelltext nicht vor. Das Feedback F11 betrifft deshalb hier keine Formulierung. Nächster Hinweis steht in Zeile 2338: "Temperaturänderung".
- Bild `t_880.png`: rot "Δθ" in der Formel.
- Bild `t_10.png`: Fahrplan-Karte 5 "c, m, Δθ".
- Nicht geprüft: gesprochener Ton.

## Layout-Beobachtungen in gehaltenen Bildern (keine Überlappung mit Text)
- `t_720.png`: Formelkasten liegt direkt unter der Hausbodenlinie, Abstand fast null, aber nicht überlappend.
- `t_664.png`: Obere Kante des Erd-WP-Balkens (Wert "4") liegt nahe an der Legende "Umweltwärme". Kein Überlappen.
- `t_680.png`: Beschriftung "DIN EN 410:2011-04 · g⊥" steht nahe am rechten Bildrand, bleibt aber innerhalb.
- `t_1085.png`: Schlusskarte gut lesbar; die Zeile "VDI 4650 Blatt 1:2024-02 · …" ist die breiteste Zeile.

## Ausserhalb der Liste, aber aufgefallen
- Das Symbol φ steht im Physik-Video für relative Luftfeuchte ("φ = 50 %", "φ = 57 %", Bilder `sheet_04.png` Kachel 7 bis 11 bzw. `sheet_03.png`, Kachel 13 bis 16). Das ist nicht der Wärmestrom und nicht falsch. Es ist aber dasselbe Zeichen, das F06(2) für den Wärmestrom in der Heizung kritisiert. Bitte bewusst entscheiden. Gefunden in den Taupunkt-Szenen um 900 bis 940 s (Kontaktbogen `sheet_03.png`, `sheet_04.png`).

## Zusammenfassung der offenen Punkte
1. F01: Auf dem JAZ-Bild fehlt ein Kapitel- oder Stichwort-Lokator. Die Richtlinie ist aber genannt und auffindbar.
2. F03: Nur Stichworte, kein Kapitel oder Abschnitt. Konvention §6 erlaubt das nur mit dem Stichwort, das ist hier vorhanden.
3. F10/F22: Stichprobe, nicht jedes Einzelbild. Kein Verstoß gefunden.
4. F11: Ton nicht geprüft.
5. Kosmetik: "576W" ohne Leerzeichen; Formelkasten nah am Haus.
