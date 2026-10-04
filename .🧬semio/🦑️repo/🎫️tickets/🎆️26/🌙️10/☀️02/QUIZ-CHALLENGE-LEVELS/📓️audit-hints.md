# Audit — specific easy hints (design §8, client `📓️report-hints-client.md`)

Auditor, 2026-10-04. No repo file changed. Probe scripts live in the ticket folder (`hint_audit_probe.ts`, `hint_audit_analyse.ts`,
`hint_audit_extra.ts`, `hint_audit_sample.ts`, `hint_audit_peek.ts`, `hint_audit_dup.ts`, `hint_audit_dup2.ts`, `hint_audit_leak.ts`,
`hint_audit_rev.ts`, `hint_audit_pick.ts`); the generated corpus (`🗑️generated/hint-audit/`) is deleted at the end.

## Method

- `sheetOf(quiz, seed, "easy")` for seeds 1–30 over the four live energy quizzes (physics, heating, cooling, demand), every task (9 tasks,
  two of them two-dimension matchings; no linear-scale task exists in the live quizzes, so the "difference" wording is not exercised).
- Per task and seed six flawed answers: adjacent swap, swap of two far-apart ranks (distance >= n/2), extremes swapped, one item moved to the
  opposite end, one item shifted by 3 places, fully random. Matching: the same flaw on every dimension (card ranks); classification:
  one/two/three category swaps between items of different categories, one item moved, or random.
- Hints from the core `hintsOf`, rendered by the React target's own `compareText` and `classificationHintText` (the exact string `HintNote`
  puts beside the item), with `quizText("en"|"de")`.
- Corpus: 3,152 hints, **1,693 distinct questions** (1,548 compare, 127 group, 18 profile; 0 `category`, 0 `apart` — the live tasks never reach them).

## What is right (checked, not assumed)

- Never generic: every question names concrete items; compare hints also carry a number. 0 bare directions or counts.
- Truth side: `under` agrees with claim/truth for all 2,280 compare hints (0 disagreements). The claim is always clearly wrong
  (error claim/truth: min 3.3, median 155), so no hint fires on a near miss. No equal keys, no equal true values, no label contains a quote.
- Floor/ceil never tips the claim over to the truth (the shown number is always on the claim's side).
- `adjacent` swaps give no hint on sortings/matchings (good); 1–2 hints for a far swap / extremes / opposite-end move.

## Ranked issues

1. **Reversed direction is worded as an overshoot (worst).** 1,851 of 2,280 compare hints (81 %) are "over" hints whose truth lies on the
   other side of 1 (the learner has the pair the wrong way round); that is 95 % of all "over" hints (89–92 % in the four ratio quizzes).
   "as much as 3.9 times as large" / "ganze 3,9-mal so groß" says "too big", so the learner shrinks the factor instead of reconsidering the
   order. 472 of the 1,345 ratio-form ones show a count below 2 ("1.3 times"), where the number looks harmless. Fix: the core needs a
   third verdict, `side: "under" | "over" | "reversed"` (replace `under: boolean`; wire/vectors/Rust twin), with its own wording that drops
   the number: EN "Are you sure “X” is larger than “Y” at all?", DE "Bist du sicher, dass „X“ überhaupt größer ist als „Y“?"; additive:
   "Are you sure “Y” adds up to more than “X”?" / "…„Y“ zusammen mehr ergibt als „X“?". Keep "as much as" only for the 107 same-direction overstatements and
   say "up to" / "bis zu" (never "ganze N-mal").
2. **Giant counts.** 253 hints (11 %) print a count of 10+ digits, up to `11.000.000.000.000.000.000.000.000 ×` (26 digits, 34 characters in
   DE). `formatFactor` uses `Intl` with 2 significant digits and no exponent. Fix: counts < 10,000 as today; 10,000 .. 10^12 with the
   locale's long compact form (EN "16 million", DE "16 Millionen", via `notation: "compact", compactDisplay: "long"`, so the long/short scale
   is the locale's own); >= 10^12 as `{{m}} × 10^{{e}}` with superscript digits and one decimal at most. Resolved for the reversed case by issue 1 (no number), but 189
   remaining "under" claims > 10^9 still need it.
3. **Labels are too long and inconsistent for a sentence.** EN question median 140 characters, p90 181, max 216; 20 % exceed 160. 34 of 210
   labels exceed 60 characters, 49 (23 %) contain parentheses (nested inside the quotes), 3 start with a digit (`1 × „1 Liter Wasser…“`,
   `“1970s office tower…”`), the "heating" labels mix `1860–1918,` and `(1860–1918)` styles and make near-twins (unrenovated vs retrofitted
   Gründerzeit) hard to tell apart. Fix: author an optional `short` label (<= 40 characters, no parentheses, noun-first, no leading digit) next to `label` in
   the schema and use `short ?? label` in hints; rename the water item to a noun phrase ("Erhitzen von 1 Liter Wasser auf Kochtemperatur" / "Boiling 1 litre of water").
4. **Reference choice: right size of absurdity, wrong familiarity.** The rule (largest error, then smallest oriented claim) reliably yields an
   absurd claim, but the reference is whichever extreme anchor wins: 9–17 distinct references per task, one item serves 15–20 % of a task's
   hints (cooling load: "1970s office tower, fully glazed, without external shading" 20 %), 38 % of multi-hint answers reuse one reference, and
   many references are items a layperson cannot size ("Humanity's average primary energy use", "Primary energy demand stated in an energy performance
   certificate"). Fix: authored flag `everyday: boolean` on items (kettle, heating oil, chocolate bar, wall box, passive house vs Gründerzeit);
   inside the tie window prefer `everyday` anchors, then the smallest claim, then sheet order. For group hints prefer an `everyday` anchor over the first in sheet order.
5. **Profile hints are not relative to the others and read badly.** Only 18 distinct questions for 322 hints; one fixed number recurs ("Heating
   demand at about 303 kWh/(m²·a)" 7×). The axis is chosen by gap/reach, so it is often a non-obvious one ("Cooling demand … about 2", "Ventilation heat
   loss … about 43"); the axis label is a full catalogue label with a parenthetical and a capital letter mid-sentence ("with Ventilation heat loss
   (air leakage plus ventilation after heat recovery)"); negative values use a hyphen ("-2 €/(m²·a)"). This is the one kind that breaks the
   owner's "relative to the others". Fix: name another item the learner put into that profile ("Are you sure “Passive house” and “Unrenovated old
   building” both fit Profile B — heating demand about 303?") and use a `short` axis label in lower case, value with U+2212.
6. **Two-dimension matching.** No dimension is named: in 40 of 267 two-dimension answers (15 %; 21/131 heating-load-and-demand, 19/136
   cooling-load-and-demand) the identical question stands beside the item in both columns, and a single announced hint ("Are you sure “X” is
   … ?") does not say which quantity. Fix: merge identical dimension hints into one ("… in both columns"), otherwise append the dimension label
   (“— heating load” / „— Heizlast“) to the announcement text (visual placement already disambiguates).
7. **Magnitude leak when the learner's claim is the inverse of the truth.** 29 of 2,280 hints (1.3 %) show a number within 10 % of the true ratio
   (inverted): "280 ×" where it is 282, "7,500 ×" where it is 8,000. The wording stays wrong-way, but the figure is the answer. Solved by issue 1 (no number in
   the reversed case).
8. **Grammar and phrasing.** DE "ganze 1,7-mal" is unidiomatic (use "bis zu"); EN "is as much as 1.4 times as large as" doubles "as" (use "up to
   1.4 times as large as"); "ist nur 1,2-mal so groß" is fine. 44 % of the questions (1,001) start with the reference, not the hinted item (roles swapped
   when claim < 1), while the hint stands beside the hinted item: only the pair makes the link clear. Fix (low priority): always start with the hinted item and phrase
   the inverse claim from its side ("“X” is only a fifth of “Y”" is not an option, since it needs a fraction), or accept it and rely on the placement.
9. **Density.** Hints per answer (mean/max): far swap 1.7–2.4/2–4, extremes 2–4, one item at the opposite end 0.9–2.5/1–4, shift by 3 0–1.1 (often none),
   adjacent swap 0 on sorting/matching. Fully random answers: powers 5.5 of 10 (max 8), energies 4.8 (9), heating-load-and-demand 5.2 of 16 cells (9),
   power-or-energy 5.4 of 12 (8). Normal flawed answers are fine; random ones are a wall. Fix: show at most 4 at a time (largest error first), the rest
   as "and n more doubts" — optional.

## Worst 15 examples (verbatim)

Numbers in brackets are claim = what the learner's keys say for item/other, truth = real item/other.

1. Reversed, small count, "as much as" (issues 1, 8). claim 1.21, truth 0.105 (really 10× smaller).
   EN: Are you sure “New passive house” is as much as 1.3 times as large as “Gründerzeit apartment building 1860–1918, unrenovated”?
   DE: Bist du sicher, dass „Neues Passivhaus“ ganze 1,3-mal so groß ist wie „Gründerzeit-Mehrfamilienhaus 1860–1918, unsaniert“?
   Fix: reversed wording without number: "Are you sure “New passive house” is larger than “Gründerzeit apartment building 1860–1918, unrenovated” at all?"
2. Reversed, "ganze 1,7-mal". claim 1.61, truth 0.056.
   EN: Are you sure “Roof of the GEG reference building” is as much as 1.7 times as large as “Uninsulated roller-shutter box, before 1995”?
   DE: Bist du sicher, dass „Dach des GEG-Referenzgebäudes“ ganze 1,7-mal so groß ist wie „Ungedämmter Rollladenkasten, vor 1995“?
   Fix: as 1; "Dach des GEG-Referenzgebäudes" vs "Rollladenkasten" needs the short labels ("Dach (GEG-Referenz)", "Rollladenkasten").
3. Reversed, 1,000 times. claim 1000, truth 0.033; the reference label starts with a digit and runs 70 characters.
   EN: Are you sure “Passive-house home with external shading” is as much as 1,000 times as large as “1970s office tower, fully glazed, without external shading”?
   DE: Bist du sicher, dass „Passivhaus-Wohngebäude mit außenliegendem Sonnenschutz“ ganze 1.000-mal so groß ist wie „Bürohochhaus der 1970er-Jahre, vollverglast, ohne außenliegenden Sonnenschutz“?
   Fix: reversed wording; short labels "Passivhaus mit Sonnenschutz" / "Bürohochhaus, vollverglast".
4. Reversed, long labels with parentheses and "z. B.". claim 0.032, truth 225.
   EN: Are you sure “Underground car park of a residential building, mechanically ventilated” is as much as 32 times as large as “Cleanroom ISO class 5 with unidirectional airflow (e.g. chip fab)”?
   DE: Bist du sicher, dass „Tiefgarage eines Wohngebäudes, maschinell belüftet“ ganze 32-mal so groß ist wie „Reinraum ISO-Klasse 5 mit turbulenzarmer Verdrängungsströmung (z. B. Chipfabrik)“?
   Fix: short labels "Tiefgarage" / "Reinraum ISO 5"; reversed wording.
5. 26-digit count (issue 2). claim 1.1e25, truth 0.0056.
   EN: Are you sure it takes 11,000,000,000,000,000,000,000,000 × “Design heating load of an unrenovated 1960s single-family house” to add up to 1 × “Person sitting still (body heat)”?
   DE: Bist du sicher, dass es 11.000.000.000.000.000.000.000.000 × „Heizlast eines unsanierten Einfamilienhauses der 1960er-Jahre“ braucht, um 1 × „Ruhig sitzender Mensch (Körperwärme)“ zu ergeben?
   Fix: reversed wording without number; where a number stays: "11 × 10²⁴".
6. 16-digit count in an "only" hint (still needs the number rule). claim 4.97e15, truth 1.09e25.
   EN: Are you sure 4,900,000,000,000,000 × “Burning tea light (heat)” together only add up to 1 × “Total radiant power of the Sun”?
   DE: Bist du sicher, dass 4.900.000.000.000.000 × „Brennendes Teelicht (Wärme)“ zusammen nur 1 × „Gesamte Strahlungsleistung der Sonne“ ergeben?
   Fix: compact long form ("4,9 Billiarden × „Brennendes Teelicht“" / "4.9 quadrillion ×"), or pick an everyday reference closer in size (issue 4).
7. Magnitude leak (issue 7). claim 0.0036, truth 282: the shown 280 is the real ratio, inverted.
   EN: Are you sure it takes 280 × “Nuclear power plant unit (electrical, Isar 2)” to add up to 1 × “Modern onshore wind turbine at rated wind speed”?
   DE: Bist du sicher, dass es 280 × „Kernkraftwerksblock (elektrisch, Isar 2)“ braucht, um 1 × „Moderne Windenergieanlage an Land bei Nennwind“ zu ergeben?
   Fix: reversed wording without number.
8. Magnitude leak. claim 7500, truth 0.000125 (really 8,000 the other way round).
   EN: Are you sure it takes 7,500 × “ICE 3 high-speed train at full power” to add up to 1 × “Sunlight on 1 m² at noon on a clear summer day”?
   DE: Bist du sicher, dass es 7.500 × „ICE-3-Hochgeschwindigkeitszug unter Volllast“ braucht, um 1 × „Sonnenlicht auf 1 m² an einem klaren Sommermittag“ zu ergeben?
   Fix: as 7.
9. Longest question (216 characters), near-twin labels, reversed. claim 7.58, truth 0.4.
   EN: Are you sure “Gründerzeit apartment building (1860–1918) retrofitted with passive-house components” is as much as 7.6 times as large as “New single-family house to GEG 2024 with the minimum envelope and a heat pump”?
   DE: Bist du sicher, dass „Gründerzeit-Mehrfamilienhaus (1860–1918), mit Passivhaus-Komponenten saniert“ ganze 7,6-mal so groß ist wie „Neues Einfamilienhaus nach GEG 2024 mit Mindest-Gebäudehülle und Wärmepumpe“?
   Fix: short labels "Gründerzeit-Haus, saniert" / "Neubau GEG 2024"; an `everyday` anchor as reference.
10. Near-twin labels in one sentence (different punctuation style for the same year range). claim 1.3, truth 9.85 (under).
    EN: Are you sure “Gründerzeit apartment building 1860–1918, unrenovated” is only 1.3 times as large as “Gründerzeit apartment building (1860–1918) retrofitted with passive-house components”?
    DE: Bist du sicher, dass „Gründerzeit-Mehrfamilienhaus 1860–1918, unsaniert“ nur 1,3-mal so groß ist wie „Gründerzeit-Mehrfamilienhaus (1860–1918), mit Passivhaus-Komponenten saniert“?
    Fix: short labels "Gründerzeithaus, unsaniert" / "Gründerzeithaus, saniert" (the doubt is then visible at a glance).
11. Digit-leading label after "1 ×", huge abstract reference, "it takes … to add up" for a reversed claim. claim 56, truth 3e-14.
    EN: Are you sure it takes 57 × “Germany's annual primary energy consumption” to add up to 1 × “Heating 1 litre of water from 20 °C to boiling”?
    DE: Bist du sicher, dass es 57 × „Jährlicher Primärenergieverbrauch Deutschlands“ braucht, um 1 × „1 Liter Wasser von 20 °C zum Kochen bringen“ zu ergeben?
    Fix: noun-first label "Erhitzen von 1 Liter Wasser auf Kochtemperatur"; reversed wording.
12. Profile: non-relative, 22-word axis label, capital letter, hyphen-minus. 
    EN: Are you sure “Unrenovated old building (1960s single-family house)” fits Profile C, with Net energy costs for heating, hot water and auxiliary energy (after PV credit) at about -2 €/(m²·a)?
    DE: Bist du sicher, dass „Unsanierter Altbau (Einfamilienhaus der 1960er-Jahre)“ zu Profil C passt, mit Netto-Energiekosten für Heizung, Warmwasser und Hilfsenergie (nach PV-Gutschrift) bei rund -2 €/(m²·a)?
    Fix: "Are you sure “Unrenovated old building” and “Plus-energy house” both fit Profile C — energy costs about −2 €/(m²·a)?" with a `short` axis label "energy costs" and U+2212.
13. Profile: the named axis is not the one a layperson can judge. 
    EN: Are you sure “Plus-energy house” fits Profile E, with Cooling demand to keep 26 °C at about 2 kWh/(m²·a)?
    DE: Bist du sicher, dass „Plusenergiehaus“ zu Profil E passt, mit Kühlbedarf für 26 °C bei rund 2 kWh/(m²·a)?
    Fix: choose among axes above the threshold the one with a `salient` flag (heating demand, energy costs) before the largest gap/reach; or name an item of that profile.
14. Group: both items obscure, symmetric doubt, reference = first in sheet order (issue 4).
    EN: Are you sure “Primary energy demand stated in an energy performance certificate” and “Charging power of a home wall box” belong to the same category?
    DE: Bist du sicher, dass „Primärenergiebedarf im Energieausweis“ und „Ladeleistung einer Wallbox“ in dieselbe Kategorie gehören?
    Fix: reference = an `everyday` anchor of the assigned category ("Wasserkocher", "Heizöl"); short labels.
15. Identical question twice (two dimensions, issue 6), seed 1, `extremes`, heating-load-and-demand: both columns show
    EN: Are you sure “Gründerzeit apartment building (1860–1918) retrofitted with passive-house components” is as much as 1.4 times as large as “Single-family house 1969–1978, unrenovated”?
    DE: Bist du sicher, dass „Gründerzeit-Mehrfamilienhaus (1860–1918), mit Passivhaus-Komponenten saniert“ ganze 1,4-mal so groß ist wie „Einfamilienhaus 1969–1978, unsaniert“?
    Fix: merge ("in beiden Spalten") or append the dimension label; plus issues 1 and 3.

## Rules to adopt (summary)

- Verdict `under | over | reversed`; reversed = no number, direction question.
- Counts: plain < 10^4, locale compact long up to 10^12, `m × 10^e` beyond; never more than 6 digits in a sentence.
- Authored per item: `short` label (<= 40 characters, noun-first, no parentheses, no leading digit) and `everyday` flag; reference = everyday anchor inside the tie window.
- Profile hints: name a co-placed item and a short, lower-case axis label; minus sign U+2212.
- Matching with several dimensions: merge identical dimension hints, name the dimension in the announcement.
- DE "bis zu" instead of "ganze"; EN "up to".
- Optional cap of 4 simultaneous hints.
