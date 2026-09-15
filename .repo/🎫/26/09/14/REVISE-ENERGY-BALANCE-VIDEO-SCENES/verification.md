
---

# 2026-09-15 — Continuation

## Context found on resume

A second Claude session (`b09d9a70`, driven by Niloufar) edited Chapters 1–4 and
`manim_visuals.py` between 09:28 and 10:44, applying three rounds of feedback:
removed the sun-to-window wave (C1 B3); standards wording with editions
(DIN V 18599:2018-09, DIN/TS 18599 2025), "Referenz-Heizgrenze", schematic-data
labels (C2, C3); canonical `e`-factors in C4 (gas e_p 1,35→1,40 / Q_E 135→140,
heat pump 0,29→0,31 / 29→31); tank loss waves rotated (C4 B2). All of it was kept.

## Cross-chapter consistency (the main work)

C4's corrected numbers had not reached C5–C7, which still showed 135 / 29 / 149 / 52.

| Where | Change |
|-------|--------|
| C4 B1 | staircase segments were typed (4+8+6+17 = 135) while B4 printed 140; now derived from the factor chain → +4 +8 +7 +21 = 140; narration "rund vierzig Prozent"; legend rounds instead of truncating |
| C5 | `E_P_*` mirror C4's factors; Q_E 140/31 → Q_P 154/56; `_eq()` prints "=" only for exact products (140·1,1 = 154, 31·1,8 ≈ 56) |
| C5 B4 | at Q_P = 56 the heat pump would have *failed* the old example limit (55). Limit is now 55 % (GEG § 15) of a **schematic** reference value 120 → 66, labelled as an example on screen; verdict "im Beispiel"; gas misses by ×2,33 → "mehr als das Doppelte" |
| C6 | 140 → E, 31 → **A** (was A+; A+ is < 30); letters looked up via `_class_letter`, never typed |
| C7 B3 | chain 100 → 140 → 154 → Klasse E from `CHAIN_*` constants |
| README | worked example and a sources-of-truth table |

## Accuracy corrections, same standard as the feedback on C1–C4

- C6 B3: GEG § 87 (1) requires Ausweisart, **Endenergie**kennwert, wesentlicher
  Energieträger, Baujahr and Effizienzklasse (Wohngebäude) — **no Primärenergie
  field**. Listing row and "beide Energiekennwerte" were wrong; removed. The
  unsupported "Bank / Beleihungswert" reader removed.
- C6 B1: "Neubau & größere Sanierung: Bedarfsausweis zwingend" → Neubau stets,
  Bestand meist Wahlrecht mit Ausnahmen (§ 80). Scale noted as Wohngebäude.
- C5: f_p described as a fixed calculation value (GEG Anlage 4), not grid losses;
  GEG limits primary energy *and* thermal protection; new clause on § 16 and the
  § 71 65 % rule with transition periods.
- C7: 20 °C as setpoint (not "all rooms"); prebound/rebound as averages; scatter
  labelled schematic; § 60c "für viele Anlagen" instead of "Pflicht & förderfähig".

## Defects found and fixed

- `5_primary_energy/scene_5.py` began with a stray `<` (and a double space in
  `TITLE_DE`), last modified 21:52 on 09-14 — after this ticket's final edit to it
  and not by the other session. Looks like accidental keystrokes; the chapter would
  not have imported.
- Guard-invisible collisions, found by frame inspection: C2 B1 revision note on the
  spoke arrows; C3 B4 capacity line through its card (card now opaque, static);
  C5 B3 header cards over the first row and emphasis stroke smearing card text;
  C5 B4 verdict note over the house roofs and gas bar through the limit label;
  C6 B1 and C7 B1 card columns crowding the caption.
- Wrapping text: two notes (C2 B4, C3 B2) and two of this ticket's own C7
  captions ran over one / two lines. `note_line` now warns on wrap, like
  `caption_bar`; all 127 captions and every note checked.

## Verification

- `sweep4-2026-09-15-full.log`: all 26 beats, zero `[LAYOUT]` reports, zero errors;
  only the two C7 caption warnings, then fixed.
- `sweep5-2026-09-15-changed-beats.log`: the 8 beats changed afterwards — clean.
- Frames inspected for every changed beat (`s3_*`, `f4_*`).
- Playlist imports: 7 chapters, 26 beats, 127 clauses.

Ticket left open: the repo MCP server was disconnected, so `ticket_close` could not run.
