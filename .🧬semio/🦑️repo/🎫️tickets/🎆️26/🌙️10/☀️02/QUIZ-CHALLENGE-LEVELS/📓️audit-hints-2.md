# Audit — easy hints after revision round 2 (design §8.4a, reports `📓️report-hints2-*.md`)

Auditor, 2026-10-04. No repo file changed. Probe scripts (kept in the ticket folder): `hint_audit2_probe.ts` (corpus; env `MODE` =
`live` | `nofamiliar` | `fulllabels`, env `SEED_FROM`), `hint_audit2_analyse.ts` (before/after figures), `hint_audit2_sample.ts`
(stratified sample), `hint_audit2_extra.ts` (cross-checks). The generated corpus (`🗑️generated/hint-audit-2/`) is deleted at the end.

## Method

- Same probe as `📓️audit-hints.md`: `sheetOf(quiz, seed, "easy")`, seeds 1–30, four live quizzes (physics, heating, cooling, demand), all 9 tasks,
  six flawed answers per task and seed (adjacent swap, far swap, extremes, one item to the opposite end, shift by 3, random; classification: category
  swaps / one item moved / random); same generator and seeds. Hints from the core `hintsOf` (round 2: verdict, `short`, `familiar`, cap 3, profile
  `other`/`above`), rendered by the React target's own `compareText` (names by `hintName`; the quantity named when a matching has several dimensions)
  and `classificationHintText` with `quizText("en"|"de")`.
- Corpus: **2,619 hints** (was 3,152; the cap), **846 distinct questions** (1,915 compare: 1,586 reversed, 208 under, 121 over; 406 group; 298 profile;
  0 `category`, 0 group "apart": still never reached by the live tasks, so those two wordings are unexercised).
- Fresh sample: a second corpus with seeds 101–130 (2,601 hints, 871 distinct questions) → **144 distinct questions**, stratified by task and form
  (reversed / under / over, plain, scale word, power of ten, sum, dimension, profile relative / value, group), read in EN and DE (below).
- Counterfactuals on the same quizzes: `nofamiliar` (the `familiar` flags stripped; reference rule otherwise as now) and `fulllabels` (names by
  `label`). The `fulllabels` rerun reproduces the first audit's label figures exactly (34 labels > 60 characters, 49 with parentheses, 3 digit-leading),
  which validates the method.
- Independent checks: verdict recomputed from claim and truth (0 disagreements with the core over all 1,915 compare hints); the item named first in a
  reversed question is always the one the learner's keys make larger (0 violations in 3,182 reversed hints of both corpora); truth side of every under/over
  agrees (0 disagreements).

## The nine issues, before / after

| # | Issue (first audit) | Before | After | State |
|---|---|---|---|---|
| 1 | Reversed pairs worded as an overshoot | 1,851 of 2,280 compare hints (81 %) "as much as N times"; 472 of them with a harmless-looking count < 2 | 1,586 of 1,915 (82.8 %) are `reversed`, **100 %** of them worded as an order question ("is larger than" / "größer ist als"; "lies above" for linear), **0** with any number; the 107 old same-direction overstatements are now 121 `over` ("really" / "tatsächlich"); "as much as" / "ganze": 0 | fixed |
| 2 | Giant counts | 253 hints (11 %) with a count of 10+ digits, max 26 digits (34 characters DE) | max digit run **6** (EN and DE); 75 hints with a scale word, 28 with "roughly 10ⁿ" / "rund 10ⁿ"; 0 hints with a run ≥ 7 | fixed (see worst 3 for the power-of-ten rounding) |
| 3 | Labels too long / inconsistent | EN question median 140, p90 181, max 216 characters; 20 % > 160; 34 of 210 labels > 60 characters, 49 (23 %) with parentheses, 3 digit-leading | EN median **100**, p90 **121**, max **139**; DE median 110, p90 130, max 154; 0 % > 160; the 104 names used are all ≤ 40 characters, 0 with parentheses, 0 digit-leading; same hints with full labels: EN median 123 / max 220, DE median 132 / max 243, 13 % (EN) and 22 % (DE) > 160 | fixed (a few shorts are still awkward, see worst 8 and 10) |
| 4 | Reference familiarity / concentration | 9–17 references per task; one item serves 15–20 % of a task's hints; 38 % of multi-hint answers reuse one reference | familiar references: **64.4 %** of compare hints (counterfactual without the flags: 30.0 %); 9–17 references per task; the top reference serves **20–36 %** of a task's hints (without flags 11–23 %); multi-hint answers reusing a reference **42.3 %** (without flags, same cap: 21.0 %); 13 of 576 three-hint answers use one reference three times; group hints: 0 familiar (classification items cannot carry the flag) | partly fixed, partly traded for concentration |
| 5 | Profile hints | 18 distinct questions / 322 hints; no item named; 22-word axis label with parenthesis; capital mid-sentence; hyphen-minus | 46 distinct questions / 298 hints; **67.4 %** (201) name another item ("lies above / below … in …"), 32.6 % (97) keep the value form; axis labels short, "−2" with U+2212, 0 hyphen-minus; the value form still recurs (`303` in 37 % of value-form hints over both corpora, `135`, `43`); 44 % of profile hints are on the ventilation or cooling axis | partly fixed |
| 6 | Two-dimension matching | 40 of 267 answers (15 %) with the identical question in both columns; dimension not named | **0 of 618** answers with an identical question; the quantity is named in 676 of 676 two-dimension hints (EN "… in Heating demand?", DE "… in puncto Heizwärmebedarf …") | fixed (EN style, see worst 4) |
| 7 | Magnitude leak | 29 of 2,280 hints (1.3 %) show a number within 10 % of the true ratio inverted | **0** of 329 number-bearing hints within 10 % of the truth or its inverse; reversed hints carry no number; error claim/truth over all hints: min 3.65, 10 % 11.9, median 167 | fixed |
| 8 | Grammar and phrasing | "ganze 1,7-mal", "as much as 1.4 times as large as" (double "as"); 44 % of questions start with the reference | 0 old wordings; EN/DE read fully grammatical in the 144-sample (see below); 43.9 % still start with the reference (reversed hints start with the item the keys make larger, the hint stands beside the other one in about half of them) | fixed, 44 % unchanged |
| 9 | Density | far swap 1.7–2.4/2–4, extremes 2–4, opposite end 0.9–2.5/1–4, shift 0–1.1; **random: 4.8–5.5 (max 8–9)** | per answer: adjacent 0.50, far swap 1.96, extremes 2.28, opposite end 1.40, shift3 0.94, **random 2.63**; **max 3** everywhere (histogram over 1,620 answers: 0 hints 24.9 %, 1: 13.4 %, 2: 36.7 %, 3: 24.9 %); 73.7 % of the random answers sit at the cap | fixed |

Reading notes. (1) `reversed` is the dominant verdict (83 %): every flawed answer of 4 or more wrong cells has mostly pairs the wrong way round, so the
cap keeps the order questions with the largest error. (9) 89 shift3 and 10 opposite-end answers with 4 or more wrong cells get no hint at all (largest
case: 12 wrong cells, cooling-load-and-demand seed 6): by design, since the moved item's key does not miss; there is no "n more doubts" line either.

## Grammar, helpfulness, genericity — the fresh sample (144 questions)

Grammar: **0 errors** found.
- German: all dass-clauses verb-final ("Bist du sicher, dass … ist wie …?", "… braucht, um 1 × „…“ zu ergeben?", "… zusammen nur … ergeben?"; plural verb after
  "N ×" fits); "in puncto" sits after the subject, takes the bare nominative noun and works for the feminine "Heizlast / Kühllast", the plural
  "Netto-Energiekosten" and the neuter "Heizwärmebedarf"; "tatsächlich N-mal so groß ist wie" and "nur N-mal" fine. Quotes „…“ balanced in all 5,220 questions of both
  corpora, no straight quotes, no double spaces.
- Number words: DE "1 Million" (singular) vs "100 Millionen", "1,2 Milliarden", "1,1 Billionen", "16 Millionen" — **0 agreement violations** over both locales and both
  corpora; EN million / billion / trillion map one-to-one to DE Million / Milliarde / Billion (0 mismatches); NBSP between number and word; separators per locale
  ("22,000" / "22.000"); the "-mal" form only after digits ("1.000-mal", 155 times), "N Mal" never needed. "1 ×" never appears (smallest sum count 1.6 ×).
- English: "is really N times as large as", "is only", "together only add up to", "it takes … to add up to"; no doubled "as", no leftover "as much as".
- Nits: straight apostrophes inside curly quotes ("Germany's", "Humanity's", "World's"); EN "World's annual primary energy use" has no article.

Helpfulness (sample of 144):
- **Never generic in the sense of the first audit:** every question names two concrete items; 18 of the 144 are group questions, which are the most template-like
  ("… and … belong to the same category?"), still item-specific.
- 45 of 144 (31 %) are numeric questions of a task whose quantity is not named although "larger" reads as physical size for their items (windows, walls, car parks, houses): see worst 1–2.
- 48 of 144 EN questions (33 %) carry a capitalised quantity/axis in the middle of a sentence ("… in Cooling load?", "… with Heating demand at about …").
- 9 of the 144 show a power of ten, 4 of those more than 3 times off the learner's own claim.
- Sums with an event or action label read oddly after "N ×" ("100 million × “Boiling a litre of water”").
- Reversed questions give the direction doubt and nothing else (by design); 83 % of the numeric hints share one skeleton, so a learner with three of them sees three
  near-identical sentences with different names.

## Worst 10 remaining (verbatim)

Numbers in brackets: claim = what the learner's keys say for item/other, truth = real item/other.

1. Quantity not named; "larger" reads as physical size (u-values, air-change-rates, final-energy: 639 of 1,915 compare hints, 33 %). claim 0.0483, truth 5.37.
   EN: Are you sure “Passive-house window” is larger than “Old aluminium or steel window”?
   DE: Bist du sicher, dass „Passivhausfenster“ größer ist als „Altes Aluminium- oder Stahlfenster“?
   Fix: single-dimension matchings and sortings name their quantity as well: give every `Quantity` a lower-case `short` ("U-value" / "U-Wert", "air change rate" /
   "Luftwechselrate", "final energy" / "Endenergie") and always use the `…In` templates when a quantity short exists; for the log reversed form
   say "has a larger U-value than" / "hat einen größeren U-Wert als".
2. Same cause, obviously true on the surface. claim 50, truth 0.0542.
   EN: Are you sure “High-bay warehouse” is larger than “Underground car park”?
   DE: Bist du sicher, dass „Hochregallager“ größer ist als „Tiefgarage“?
   Fix: as 1 ("… has a higher air change rate than …" / "… hat eine höhere Luftwechselrate als …"); today the learner can answer "yes, a warehouse is bigger" and move on.
3. Power of ten misstates the learner's own claim (29 % of the 58 power-of-ten hints of both corpora are more than 3 times off, median 2.6, max 9.1). claim 1.1 × 10¹⁶, shown 10¹⁷.
   EN: Are you sure it takes roughly 10¹⁷ × “Food energy of a 100 g chocolate bar” to add up to 1 × “Litre of heating oil”?
   DE: Bist du sicher, dass es rund 10¹⁷ × „Brennwert einer 100-g-Tafel Schokolade“ braucht, um 1 × „Liter Heizöl“ zu ergeben?
   Fix: round the exponent to the nearest instead of cutting toward the claim's side (still safe: the claim is at least 3.65 times off the truth, so rounding by at most 3.16 never
   tips it over); the second example, claim 1.09 × 10²⁵ shown as "roughly 10²⁶", is 9.2 times off.
4. Capital letter mid-sentence and a tail that attaches to the wrong noun (EN, 37 % of the EN questions of both corpora: all 676 two-dimension plus all profile questions).
   EN: Are you sure “Hospital with operating rooms” is really 20 times as large as “Attic flat under an uninsulated roof” in Cooling load?
   DE: Bist du sicher, dass „Krankenhaus mit OP-Sälen“ in puncto Kühllast tatsächlich 20-mal so groß ist wie „Dachwohnung unter ungedämmtem Dach“?
   Fix: lower-case the English quantity and axis shorts ("cooling load", "heating demand", "net energy costs") and move the phrase to the subject as in German, or write
   "in terms of cooling load".
5. Profile value form that is neither relative nor item-specific (97 of 298 profile hints; "303 kWh/(m²·a)" in 37 % of them over both corpora).
   EN: Are you sure “New build to EnEV 2014” fits Profile B, with Heating demand at about 303 kWh/(m²·a)?
   DE: Bist du sicher, dass „Neubau nach EnEV 2014“ zu Profil B passt, mit Heizwärmebedarf bei rund 303 kWh/(m²·a)?
   Fix: before falling back to the value, try the other axes for a qualifying anchor, and prefer the axes a layperson can judge (heating demand, energy costs) over ventilation heat loss
   and cooling demand (44 % of the profile hints), e.g. "Are you sure “New build to EnEV 2014” lies above “Passive house” in heating demand?".
6. Group hint between two abstract items (classification items cannot carry `familiar`, the reference is the first in sheet order).
   EN: Are you sure “Output of a nuclear power plant unit” and “Primary energy in an energy certificate” belong to the same category?
   DE: Bist du sicher, dass „Leistung eines Kernkraftwerksblocks“ und „Primärenergiebedarf im Energieausweis“ in dieselbe Kategorie gehören?
   Fix: allow `familiar` on classification items and prefer a familiar item (tea light, kettle, heating oil) as the group partner; the physics group questions are 806 of the 5,220 hints.
7. Event label as multiplicand.
   EN: Are you sure 100 million × “Boiling a litre of water” together only add up to 1 × “World's annual primary energy use”?
   DE: Bist du sicher, dass 100 Millionen × „Aufkochen eines Liters Wasser“ zusammen nur 1 × „Jährliche Primärenergie der Welt“ ergeben?
   Fix: noun-phrase short "A boiled litre of water" / „Ein aufgekochter Liter Wasser“ (or a note in the style guide: after "N ×" a short is a countable thing).
8. Stilted German short (also „Jährliche Primärenergie Deutschlands“, „Tagesnahrung eines Erwachsenen“).
   EN: Are you sure 160,000 × “Full electric car battery” together only add up to 1 × “Germany's annual primary energy use”?
   DE: Bist du sicher, dass 160.000 × „Voller E-Auto-Akku“ zusammen nur 1 × „Jährliche Primärenergie Deutschlands“ ergeben?
   Fix: „Primärenergie Deutschlands pro Jahr“ (35 characters) and „Primärenergie der Welt pro Jahr“; "Jahresbedarf Nahrung eines Erwachsenen".
9. One reference three times in one answer (13 of 576 three-hint answers, 42.3 % of multi-hint answers repeat one; heating/u-values seed 13, random).
   EN: Are you sure “Roof of a passive house” is larger than “Uninsulated half-timbered wall”?
   EN: Are you sure “External wall of a passive house” is larger than “Uninsulated half-timbered wall”?
   EN: Are you sure “Floor of the GEG reference building” is larger than “Uninsulated half-timbered wall”?
   DE: Bist du sicher, dass „Dach eines Passivhauses“ größer ist als „Ungedämmte Fachwerkwand“? / … „Außenwand eines Passivhauses“ … / … „Bodenplatte des GEG-Referenzgebäudes“ …
   Fix: in the reference choice, after the tie window prefer a reference not yet used by the task's other hints (twin-safe: the choice then depends on the hints made so far in
   emission order), before `familiar`; the familiar flag concentrates the top reference to 20–36 % of a task's hints.
10. Jargon repeated in both names of one question.
    EN: Are you sure “Roof of the GEG reference building” is larger than “Front door of the GEG reference building”?
    DE: Bist du sicher, dass „Dach des GEG-Referenzgebäudes“ größer ist als „Haustür des GEG-Referenzgebäudes“?
    Fix: shorts "Reference roof" / „Referenzdach“ and "Reference front door" / „Referenzhaustür“ (the GEG context is in the task), plus issue 1 (a roof is not "larger" than a door).

## Verdict

Issues 1, 2, 3, 6, 7, 8, 9 of the first audit are fixed with numbers; issue 4 and 5 are improved but not closed. No grammar defect and no leak found. What remains is
content and template tuning, ranked in the answer below.
