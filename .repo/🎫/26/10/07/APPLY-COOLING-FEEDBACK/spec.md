# Apply Cooling Review Feedback — Work Spec

Ticket: `2026/10/07/APPLY-COOLING-FEEDBACK` (folder `.repo/🎫/26/10/07/APPLY-COOLING-FEEDBACK/`).
Repo root: `/Users/niloufarghandehariyoon/Developer/semio`.

The dev's instruction: **correct the video and animations scene by scene — finish one scene before starting the next, not all together**. Apply only the supervisor's feedback; everything the feedback does not mention stays unchanged (same beats, same order, same visuals, same narration unless a feedback item touches it).

Repo MCP was unavailable (`CONNECTION_CLOSED`), so `repo://goals`, `ticket_open` and `ticket_close` could not be called. Ticket created and logged here by hand, same as `APPLY-HEATING-FEEDBACK` and `APPLY-PHYSICAL-FUNDAMENTALS-FEEDBACK`.

## Supervisor feedback (verbatim, German)

Regeln:
- Alles muss visuell sein (erklären, visualisieren, morphen etc meinen alle das gleiche)
- Ideen müssen ineinander morphen

Allgemein:
- Alle Faktoren sollen Animation bekommen von 0-1, viele fehlen noch.
- Alle Teilwerte der Gleichungen müssen Resultat von der physischen Repräsentation sein (Flächen, Faktoren, etc)
- Noch viel unwichtige und unverknüpfte Informationen anstelle von zusammenhängend und spannend
- Text übertrieben und unnötig wuchtig
	- "Architektur formt Raum", "und sie hört nie auf", etc
- Architektur und Bauphysik sind nicht getrennt, sondern sollten eine Einheit bilden
- Latex muss richtig dargestellt werden (Formeln, Einheiten, etc - momentan sind subscript, superscript, brüche etc nicht richtig dargestellt)

Kühlen:
- (Mechanische Lüftung) am Anfang ist zu rudimentär animiert, wird nicht klar
- Körperwärme bei verschiedenen Aktivitäten (alle animiert) in Skala morphen von Schlafen bis zu Hochleistungssport
- f_n für Geräteaktivität erklären anhand von verschiedenen Nutzungen (jeweils animiert)
- Spitzenlast tritt selten erst am späten Abend auf, aber später Nachmittag ist realistisch
- Erkläre wie man mit physikalischen Eigenschenschaften engineeren kann am Beispiel der Sorptionsgestützte Kühlung über Entfeuchtung, Plattentauscher, Verdunstungskühlung, Regeneration durch die Sonne etc - Nachteile zeigen: Mikrobiotisch schlecht, da Mikroben unter Druck gesetzt warden durch krasse Temperatur und Feuchtigkeitswechsel und dann mutieren mit Tendenz, dass die "Bösen" überleben
- Entwickle die Sonnenleistungskurven der Himmelsrichtungen anhand von 3D Sonnenvisualisierung komplett animiert, etc
- Natürlich Lüftung und Passivhaus ist ein Wiederspruch. Genau natürliche Lüftung ist das was man aufgeben muss, da man Wärmerückgewinnung braucht, da sonst Lüftungswärmeverluste viel zu hoch sind.

## Scene mapping

| Feedback item | Scene |
|---|---|
| Mechanische Lüftung am Anfang zu rudimentär | Teil 1 `1_heating_vs_cooling/scene_1.py` |
| Körperwärme-Aktivitätsskala Schlafen → Hochleistungssport | Teil 2 `2_internal_gains/scene_2.py` |
| f_n Geräteaktivität je Nutzung animiert | Teil 2 `2_internal_gains/scene_2.py` |
| Spitzenlast später Nachmittag statt später Abend | Teil 5 `5_systemauslegung/scene_5.py` (+ wherever the daily load curve peaks) |
| Sorptionsgestützte Kühlung + mikrobieller Nachteil | Teil 6 `6_lueftungssysteme/scene_6.py` |
| Sonnenleistungskurven aus 3D-Sonnenvisualisierung | Teil 4 `4_solar_radiation/scene_4.py` |
| Natürliche Lüftung vs. Passivhaus Widerspruch | Teil 6 `6_lueftungssysteme/scene_6.py` |
| Allgemein (0→1 Faktoren, Teilwerte aus Zeichnung, Latex, Text, morphen) | alle Teile 1–6 |

## Working order (dev instruction: one scene at a time)

1. Teil 1 — Heizwärmebedarf vs. Kühllast → fix, render `-q l`, verify, then next
2. Teil 2 — Interne Wärmegewinne
3. Teil 3 — Transmission & Feuchte (general feedback only)
4. Teil 4 — Solare Einstrahlung
5. Teil 5 — Systemauslegung
6. Teil 6 — Lüftungssysteme

## Shared infrastructure

Reuse the `#region Math typesetting` and `#region Typeset readouts` helpers in `tutorial/manim_visuals.py` (`de_num`, `place_math`, `math_label`, `math_readout`, `math_panel`) that the Heating ticket introduced — same fix for the broken subscript/superscript/fraction rendering.

Render check per scene (repo root):

```bash
.venv/bin/manim -ql tutorial/energy/demand/Cooling/full_cooling_video.py Cooling_0N_<Name>
```

Temporary logs carry the `[DEBUG] ` prefix. All temp renders/logs land in `checks/` inside this ticket folder.
