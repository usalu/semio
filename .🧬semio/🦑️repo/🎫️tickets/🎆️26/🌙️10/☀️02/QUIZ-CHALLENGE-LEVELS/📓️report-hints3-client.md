# Report — client wording of hint revision round 3 (design §8.4b items 1 and 2)

Agent: client hints, round 3. `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `R/🔨️modules`,
`Q` = `🧰️framework/🛍️products/❓️quiz`.

## Changed files

Updated (small `Edit` hunks only)
- `M/🧩️task/🟦️.tsx` — new `hintTerm(named, locale)` (the `hintName` written as the language writes a quantity/axis name
  mid-sentence; private table `MID_SENTENCE`); `compareText(hint, quantity, label, text, locale)` loses its `named?`
  parameter and always names the quantity (`quantity` now `Pick<Quantity, "label" | "short" | "unit" | "prefixed" | "additive">`);
  one reversed key for both scales.
- `M/📏️quantity/🟦️.ts` — `counted`/`formatCount`/`formatTimes`: from 10¹⁵ a two-significant-digit mantissa × a superscript power
  of ten, cut toward the claim's side as before; table `ABOUT` ("roughly"/"rund") removed.
- `M/🌐️i18n/🟦️.ts` (EN + DE) — `quiz.task.sumUnder|sumOver|ratioUnder|ratioOver|aboveUnder|aboveOver|higher` reworded with `{{quantity}}`;
  removed `quiz.task.*In` (8 keys) and `quiz.task.larger`; `quiz.classification.above|below` reworded.
- `M/🗂️classification/🟦️.tsx` — axis names through `hintTerm`; docstrings.
- `M/🃏️matching/🟦️.tsx` — calls `compareText` without the dimension-count switch; docstring. `M/↕️sorting/🟦️.tsx` — docstring only.
- `R/🟦️.tsx` — reexports `hintTerm`.
- `Q/🧪️tests/🪜️challenge-views/🟦️.tsx` — all template cases rewritten (EN/DE); fixtures `HOUSES` ("Indoor temperature") and
  `CLIMATES` ("Rainfall", "Coldest month mean") exercise the lower-casing; new fixtures `U_VALUES`, `AIR_CHANGES`, `ENERGIES`,
  `COOLING` with the audit's regressions 1–4 (EN + DE); new tests for `hintTerm` and for the audit's profile regression 5
  (value and relative form, EN + DE); the announcement tests' strings updated.
- `Q/🧪️tests/📐️quantity-formatting/🟦️.tsx` — count table from 10¹⁵ (incl. the audit's 1.09·10²⁵ and 1.06·10¹⁶, carry 9.96·10¹⁵ → 1 × 10¹⁶);
  new oracle sweep against `Intl.NumberFormat` scientific notation (1,804 comparisons).
- Prose: `Q/README.md` (two examples), `Q/🧬️schema/🔣️.json` (one `description`, prose only → fingerprint unchanged),
  `🎓️teaching/🏛️architecture/❓️quiz/README.md` (one sentence of hint examples).

Created: this report; probes `TICKET/hints3_quantity_names_probe.ts`, `TICKET/hints3_render_probe.ts`. Removed: nothing.

## Final templates

`{{quantity}}` = `hintTerm(quantity)`: `short ?? label`; EN lower-cases the first letter when the second is a lower-case letter.

| key | EN | DE |
|---|---|---|
| sumUnder | Are you sure {{count}} × “{{small}}” together only add up to the {{quantity}} of 1 × “{{large}}”? | Bist du sicher, dass {{count}} × „{{small}}“ in puncto {{quantity}} zusammen nur 1 × „{{large}}“ ergeben? |
| sumOver | Are you sure it takes {{count}} × “{{small}}” to add up to the {{quantity}} of 1 × “{{large}}”? | Bist du sicher, dass es in puncto {{quantity}} {{count}} × „{{small}}“ braucht, um 1 × „{{large}}“ zu ergeben? |
| ratioUnder | Are you sure “{{large}}” is only {{times}} as high in {{quantity}} as “{{small}}”? | Bist du sicher, dass „{{large}}“ in puncto {{quantity}} nur {{times}} so hoch ist wie „{{small}}“? |
| ratioOver | Are you sure “{{large}}” is really {{times}} as high in {{quantity}} as “{{small}}”? | Bist du sicher, dass „{{large}}“ in puncto {{quantity}} tatsächlich {{times}} so hoch ist wie „{{small}}“? |
| aboveUnder | Are you sure “{{large}}” is only {{difference}} higher in {{quantity}} than “{{small}}”? | Bist du sicher, dass „{{large}}“ in puncto {{quantity}} nur {{difference}} höher ist als „{{small}}“? |
| aboveOver | Are you sure “{{large}}” is really {{difference}} higher in {{quantity}} than “{{small}}”? | Bist du sicher, dass „{{large}}“ in puncto {{quantity}} tatsächlich {{difference}} höher ist als „{{small}}“? |
| higher (reversed, both scales) | Are you sure “{{larger}}” is higher in {{quantity}} than “{{smaller}}”? | Bist du sicher, dass „{{larger}}“ in puncto {{quantity}} höher ist als „{{smaller}}“? |
| classification.above | Are you sure “{{item}}” is higher in {{axis}} than “{{other}}”? | Bist du sicher, dass „{{item}}“ in puncto {{axis}} höher ist als „{{other}}“? |
| classification.below | Are you sure “{{item}}” is lower in {{axis}} than “{{other}}”? | Bist du sicher, dass „{{item}}“ in puncto {{axis}} niedriger ist als „{{other}}“? |
| classification.fits (unchanged; axis now lower-case EN) | Are you sure “{{item}}” fits {{category}}, with {{axis}} at about {{value}}? | Bist du sicher, dass „{{item}}“ zu {{category}} passt, mit {{axis}} bei rund {{value}}? |

Numbers: < 10⁶ plain ("1,200" / "1.200"); < 10¹⁵ scale word ("1.2 million" / "1,2 Millionen"); ≥ 10¹⁵ "1.1 × 10¹⁶" / "1,1 × 10¹⁶"
(NBSP on both sides of ×; mantissa 1 → "1 × 10¹⁵"). Times: "1.1 × 10¹⁶ times" / "1,1 × 10¹⁶-mal".

Audit regressions, now: (1) "Are you sure “Passive-house window” is higher in U-value than “Old aluminium or steel window”?" / „… in puncto
U-Wert höher ist als …“; (2) "… “High-bay warehouse” is higher in air change rate than “Underground car park”?"; (3) "Are you sure it takes
1.1 × 10¹⁶ × “Food energy of a 100 g chocolate bar” to add up to the energy of 1 × “Litre of heating oil”?"; (4) "… “Hospital with operating
rooms” is really 21 times as high in cooling load as “Attic flat under an uninsulated roof”?"; (5) "… fits Profile B, with heating demand at
about 303 kWh/(m²·a)?".

## Gates (in `R/📦️packages/🟦️typescript`)

- `bun ./📜️script.ts typecheck`: exit 0.
- `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test challenge-views quantity-formatting`: **214 passed** (2 files; was 203).
- `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` (full React suite): **920 passed, 0 failed** (22 files).
- Docstring emoji probe on the touched source and test files: clean (after renaming one duplicate 🏷️ → 📛️).
- Runtime probe `bun TICKET/hints3_render_probe.ts` (core `hintsOf` on random easy answers, 4 live quizzes × 60 seeds, rendered by the
  React functions, `[DEBUG]` output): 1,434 hints, 700 distinct questions (564 compare); **0** compare/profile questions without their
  quantity/axis; **0** EN terms capitalised mid-sentence; **0** bare powers, "roughly", "rund 10", digit runs ≥ 7, hyphen minus or old
  wordings; 28 questions with "N × 10ⁿ" (all over-sums); EN length median 105 / p90 128 / max 147, DE 117 / 141 / 164. Terms seen: energy,
  power, U-value, heating load, heating demand, air change rate, cooling load, cooling demand, net energy costs, final energy demand,
  ventilation heat loss.
- Not run: browser check of the dev site (behaviour observed in jsdom and the probe); site spec and e2e (another agent's;
  `🎓️teaching/…/🧪️tests/⛰️challenge-levels/🟦️.ts` is already written wording-agnostic — "is (larger|higher) … than", "höher ist als",
  "niedriger ist als", quantity names case-insensitive — and accepts these templates by reading).

## Deviations and decisions

1. **"higher" for every compare and profile question** (EN "is higher in … than", "as high in … as"; DE "höher ist als", "so hoch ist wie",
   "niedriger ist als"). The live non-additive quantities are all intensities (U-value, rates, loads, demands, costs) where "larger"/"groß"
   reads as physical size; the reversed question is now one template for both scales (`quiz.task.larger` removed).
2. **EN places the quantity right after the comparative** ("higher in U-value than", "as high in cooling load as") instead of a tail
   ("… in Cooling load?"), so it never attaches to the second item (audit worst 4). Additive sums use "add up to the {{quantity}} of 1 × …":
   EN "the" is gender-free; it assumes a singular mass noun (all additive quantities are power/energy).
3. **DE keeps "in puncto"** after the subject: article-free and grammatical for feminine, neuter and plural names; "hinsichtlich" or "was …
   angeht" need an article.
4. **EN lower-casing rule:** only when the second character is a lower-case letter (`\p{Ll}`), so "U-value", "CO₂ …", "PV yield" stay;
   applied to quantities and axes only (not items, not categories). A name starting with a proper noun would be lower-cased — none live.
5. **Mantissa cut toward the claim's side:** the mantissa is cut toward the claim's side (floor for under, ceil for over) as all
   other counts, not rounded to nearest.

## Notes for the next agent

- Signatures: `hintTerm(named: {label: Text; short?: Text}, locale): string`; `compareText(hint, quantity: Pick<Quantity, "label" | "short" |
  "unit" | "prefixed" | "additive">, label, text, locale): string` (no `named` argument any more).
- Content: quantity/axis shorts may be authored capitalised; the client lower-cases them in English. Keep additive quantity names singular
  mass nouns ("the {{quantity}} of").
- Site spec/e2e: no string with "roughly 10" / "rund 10" or "lies above" may remain in their assertions.
