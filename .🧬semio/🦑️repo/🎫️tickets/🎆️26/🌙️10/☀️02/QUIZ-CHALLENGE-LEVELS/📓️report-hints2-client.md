# Report — React rendering of hint revision round 2 (design §8.4a items 1, 2, 3, 4, 6, 7)

Agent: client hints, round 2. `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `M` = `R/🔨️modules`,
`Q` = `🧰️framework/🛍️products/❓️quiz`. Coded against §8.4a, then checked against the "Round 2" section of
`📓️hints-schema-landed.md` once the TS twin landed (`verdict`, `ProfileHint.other/above`, `short: ShortText`): types match.

## Changed files

Updated (small `Edit` hunks only)
- `M/📏️quantity/🟦️.ts` — new `Toward = "down" | "up"`, `formatCount(count, locale, toward)`, `formatTimes(count, locale, toward)`,
  `withMinusSign(number)`; private tables `SCALE_WORDS`, `ABOUT`, `TIMES` and `counted`. `formatFactor` unchanged (results keep using it).
- `M/🧩️task/🟦️.tsx` — new `hintName(named, locale)` (`short ?? label`); `compareText(hint, quantity, label, text, locale, named?)`
  renders `verdict` (under/over/reversed) and, with `named`, the quantity; `hintKey` keys on `verdict`, `other`, `above`.
- `M/↕️sorting/🟦️.tsx`, `M/🃏️matching/🟦️.tsx` — hints name items by `hintName`; matching passes the dimension quantity's
  `hintName` when the task has more than one dimension. Rows, selects and buttons keep the full label.
- `M/🗂️classification/🟦️.tsx` — `classificationHintText` names items, categories and axes by `hintName`; `ProfileHint` with
  `other` renders the relative question (`above`/`below`), without it the value form with U+2212 minus.
- `M/🌐️i18n/🟦️.ts` (EN + DE) — changed `quiz.task.ratioUnder|ratioOver` (`{{times}}` instead of `{{count}}`, "really" /
  "tatsächlich"), `aboveOver` ("really lies" / "tatsächlich"); added `quiz.task.sumUnderIn|sumOverIn|ratioUnderIn|ratioOverIn|aboveUnderIn|aboveOverIn|larger|largerIn|higher|higherIn`,
  `quiz.classification.above|below`. No key became unused (all round-1 keys still render).
- `R/🎨️.css` — `.quiz-hint-question` gains `hyphens: auto; text-wrap: pretty`.
- `R/🟦️.tsx` — reexports `formatCount`, `formatTimes`, `withMinusSign`, type `Toward`, `hintName`.
- Tests: `Q/🧪️tests/🪜️challenge-views/🟦️.tsx` (fixtures `CLIMATES` rebuilt with long labels + shorts and a sub-zero axis, new `POWERS`,
  `HOUSES`; 68 exact-string template cases EN/DE (34 per language); short-form test; multi-dimension test; German reversed announcement test; old
  `under` shapes → `verdict`), `Q/🧪️tests/📐️quantity-formatting/🟦️.tsx` (count words table, Intl compact-long oracle sweep,
  minus sign), `Q/🧪️tests/🚶️learner-journey/🟦️.tsx` (one `under: true` → `verdict: "under"`).

Created: this report; probes `TICKET/hints2_labels_probe.ts`, `TICKET/hints2_render_probe.ts`. Removed: nothing.

## Final templates (EN / DE)

Single quantity (sorting, one-dimension matching):

| key | EN | DE |
|---|---|---|
| sumUnder | Are you sure {{count}} × “{{small}}” together only add up to 1 × “{{large}}”? | Bist du sicher, dass {{count}} × „{{small}}“ zusammen nur 1 × „{{large}}“ ergeben? |
| sumOver | Are you sure it takes {{count}} × “{{small}}” to add up to 1 × “{{large}}”? | Bist du sicher, dass es {{count}} × „{{small}}“ braucht, um 1 × „{{large}}“ zu ergeben? |
| ratioUnder | Are you sure “{{large}}” is only {{times}} as large as “{{small}}”? | Bist du sicher, dass „{{large}}“ nur {{times}} so groß ist wie „{{small}}“? |
| ratioOver | Are you sure “{{large}}” is really {{times}} as large as “{{small}}”? | Bist du sicher, dass „{{large}}“ tatsächlich {{times}} so groß ist wie „{{small}}“? |
| aboveUnder | Are you sure “{{large}}” lies only {{difference}} above “{{small}}”? | Bist du sicher, dass „{{large}}“ nur {{difference}} über „{{small}}“ liegt? |
| aboveOver | Are you sure “{{large}}” really lies {{difference}} above “{{small}}”? | Bist du sicher, dass „{{large}}“ tatsächlich {{difference}} über „{{small}}“ liegt? |
| larger (reversed, log) | Are you sure “{{larger}}” is larger than “{{smaller}}”? | Bist du sicher, dass „{{larger}}“ größer ist als „{{smaller}}“? |
| higher (reversed, linear) | Are you sure “{{larger}}” lies above “{{smaller}}”? | Bist du sicher, dass „{{larger}}“ über „{{smaller}}“ liegt? |

Several dimensions (`…In`): EN appends " in {{quantity}}" before the "?" in every template; DE inserts "in puncto {{quantity}}":
sumUnderIn "dass {{count}} × „{{small}}“ in puncto {{quantity}} zusammen nur 1 × „{{large}}“ ergeben?"; sumOverIn "dass es in puncto
{{quantity}} {{count}} × „{{small}}“ braucht, um …"; ratio/above/larger/higher "dass „{{large}}“ in puncto {{quantity}} nur / tatsächlich
… so groß ist wie / über … liegt / größer ist als …".

Classification: `fits` unchanged wording ("Are you sure “{{item}}” fits {{category}}, with {{axis}} at about {{value}}?" / "… zu
{{category}} passt, mit {{axis}} bei rund {{value}}?"); new `above`/`below`: EN "Are you sure “{{item}}” lies above / below “{{other}}” in
{{axis}}?", DE "Bist du sicher, dass „{{item}}“ in puncto {{axis}} über / unter „{{other}}“ liegt?"; `together`, `apart`, `belongs`,
`belongsBare` unchanged.

Numbers (`formatCount`; `{{times}}` = `formatTimes`): rounded value < 10⁶ → two significant digits with locale grouping ("1,200" /
"1.200"); < 10¹⁵ → two significant digits + NBSP + scale word ("1.2 million", "1 Million", "1,5 Millionen", "1 Milliarde", "4,9
Billionen"); ≥ 10¹⁵ → "roughly 10ⁿ" / "rund 10ⁿ" (superscript digits). Times: EN "N times"; DE "N-mal" after digits and powers ("2,9-mal",
"rund 10²⁶-mal"), "N Mal" after a scale word ("1,2 Millionen Mal").

## Gates (in `R/📦️packages/🟦️typescript`)

- `bun ./📜️script.ts typecheck`: exit 0, 0 errors (after the TS schema twin landed; before, only the test fixtures' `under` failed).
- `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test challenge-views quantity-formatting`: **203 passed** (2 files); verbose listing
  shows every new case.
- `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test` (full React suite, includes translation-completeness, live-regions,
  learner-journey): **909 passed, 0 failed** (22 files).
- Docstring emoji probe (`TICKET/docstring_emoji_probe.ts`) on all touched source and test files: clean.
- Runtime probe `bun TICKET/hints2_render_probe.ts` (core `hintsOf` on random easy answers, 4 live quizzes × 40 seeds, rendered by the
  React functions): 957 hints, 567 distinct questions, longest EN 136 characters (was 216), **0** rule breaks (no digit run > 6, no
  hyphen minus, no "as much as"/"ganze", no raw keys); verdicts: reversed 643, under 53, over 39; profile relative 36, value 67.
- Not run: browser check of the dev site (the Rust proctor must first speak `WireVersion` 4; behaviour observed in jsdom and the
  probe only); site tests and e2e (not mine).

## Deviations and decisions

1. **DE quantity/axis placement uses "in puncto", not "beim".** "beim" needs a masculine/neuter noun without adjective; the live
   labels include feminine ("Spezifische Heizlast", "Heizlast") and plural ("Netto-Energiekosten") forms. "in puncto" + nominative
   label is grammatical for every gender, number and adjective. EN uses "in {{quantity}}" at the end of each question.
2. **EN "roughly 10ⁿ", not "about 10ⁿ"**: "Are you sure about 10²⁶ × …" reads as "sure about". DE stays "rund".
3. **Exponent cut by direction, not rounded**: `down` (under) takes ⌊log10⌋, `up` (over) ⌈log10⌉ (exact via `toExponential`), keeping the
   round-1 invariant that a shown number never tips the claim over to the truth. Thresholds apply to the value after two-digit
   cutting (999,999 up → "1 million"; 9.99·10¹⁴ up → "roughly 10¹⁵").
4. **Linear reversed uses "lies above" (`higher`)** instead of "is larger than": "Is Tea larger than Ice?" is wrong for a temperature;
   the wording matches the linear under/over and profile questions. Log scales (additive or not) use `larger`.
5. **EN over-difference "really lies {{difference}} above"** (adverb before the verb) rather than "lies really".
6. **Scale words in the formatter, not in i18n**: number grammar per locale (`SCALE_WORDS`, `TIMES`, `ABOUT`), with Intl's long
   compact notation as third-party oracle over 10⁶…10¹⁵ (both directions, both locales, > 1,400 comparisons). Own table instead of
   Intl at runtime for control over thresholds and ICU-version independence.
7. **NBSP** between mantissa and scale word (no line break inside "12 million").
8. U+2212 only in hint numbers (profile value); `formatNumber` stays hyphen-minus so guess fields round-trip.

## Notes for the next agent

- Signatures: `hintName(named: {label: Text; short?: Text}, locale)`, `compareText(hint, quantity: Pick<Quantity,"unit"|"prefixed"|"additive">,
  label, text, locale, named?: string)`, `formatCount(count, locale, toward: "down"|"up")`, `formatTimes(…)`, `withMinusSign(s)`.
- Site content: quantity and axis `short` forms are read mid-sentence ("… in heating load?"); author English shorts in lower case
  (the probe shows "in Cooling demand", "in Heating demand" from current labels). Multi-dimension questions need a quantity `short`.
- Site spec/e2e still assert round-1 strings if any ("as much as", "ganze"); selectors unchanged (`.quiz-hint[data-hint]`,
  `.quiz-hint-question`, `.quiz-hint-symbol`).
- The results screen's `×N` deviation still uses `formatFactor` (plain digits); a 20-decade miss there prints all digits — not in
  this brief.
