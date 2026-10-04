# Report — hint revision round 3: re-authored short labels (content, design §8.4b item 4)

Agent: quiz content, round 3, 2026-10-04. Input: `📓️audit-hints-2.md` worst 7, 8 and 10 plus the nits (straight apostrophes,
"World's" without article), `📓️report-hints2-content.md` (every short), `📓️report-hints2-client.md` (templates).

## Method

`hints3_shorts_probe.ts` (ticket folder, kept) prints every item's effective name (`short ?? label`) of all nine tasks inside
"1,000 × “…” together only add up to 1 × “…”" / "1.000 × „…“ zusammen nur 1 × „…“ ergeben" and "“…” is larger than “…”" /
"„…“ größer ist als „…“", every quantity and axis inside "in …" / "in puncto …", and checks the content rules (≤ 40, no
leading digit or parenthesis, distinct per task, and the new noun-phrase rule below, which it also self-tests against the
rejected old forms and accepted ones such as "Burning tea light"). Every line was read in both languages before and after.

## Before → after

Only `short` members changed or were added; no label, value, `familiar`, explanation or icon changed.

| Quiz / task | Item | Before EN | After EN | Before DE | After DE | Why |
|---|---|---|---|---|---|---|
| physics / power-or-energy | `phone-charge` | One full smartphone charge (label) | Full smartphone charge | Eine volle Smartphone-Ladung (label) | Volle Smartphone-Ladung | leading article; same as the energies task |
| physics / powers | `germany-electricity` | Germany's average electricity use | Germany’s average electricity use | Deutschlands mittlerer Stromverbrauch | unchanged | straight apostrophe inside curly quotes |
| physics / powers | `world-primary-power` | Humanity's primary energy use | Humanity’s primary energy use | Primärenergieverbrauch der Menschheit | unchanged | straight apostrophe |
| physics / powers | `sun` | The Sun | Sun | Sonne | unchanged | "1 × “The Sun”", capital article mid-sentence |
| physics / energies | `boil-water` | Boiling a litre of water | Litre of water brought to the boil | Aufkochen eines Liters Wasser | Aufgekochter Liter Wasser | verbal phrase after "N ×" (audit worst 7) |
| physics / energies | `daily-food` | Daily food energy of an adult | unchanged | Tagesnahrung eines Erwachsenen | Tagesration eines Erwachsenen | stilted, uncountable German (worst 8) |
| physics / energies | `germany-primary-energy` | Germany's annual primary energy use | Germany’s primary energy use per year | Jährliche Primärenergie Deutschlands | Deutschlands Primärenergie pro Jahr | stilted genitive (worst 8); EN parallel, apostrophe |
| physics / energies | `world-primary-energy` | World's annual primary energy use | Global primary energy use per year | Jährliche Primärenergie der Welt | Weltweite Primärenergie pro Jahr | no article in EN, stilted DE (worst 8) |
| heating / u-values | `front-door-geg` | Front door of the GEG reference building (label) | GEG reference front door | Haustür des GEG-Referenzgebäudes (label) | Referenzhaustür nach GEG | jargon repeated in both names (worst 10) |
| heating / u-values | `window-geg` | Window of the GEG reference building (label) | GEG reference window | Fenster des GEG-Referenzgebäudes (label) | Referenzfenster nach GEG | as above |
| heating / u-values | `floor-geg` | Floor of the GEG reference building | GEG reference floor slab | Bodenplatte des GEG-Referenzgebäudes | Referenz-Bodenplatte nach GEG | as above |
| heating / u-values | `wall-geg` | Wall of the GEG reference building | GEG reference external wall | Außenwand des GEG-Referenzgebäudes | Referenz-Außenwand nach GEG | as above |
| heating / u-values | `roof-geg` | Roof of the GEG reference building (label) | GEG reference roof | Dach des GEG-Referenzgebäudes (label) | Referenzdach nach GEG | as above |
| heating / u-values | `wall-passive-house` | External wall of a passive house (label) | Passive-house external wall | Außenwand eines Passivhauses (label) | Passivhaus-Außenwand | parallel to "Passive-house window", shorter |
| heating / u-values | `roof-passive-house` | Roof of a passive house (label) | Passive-house roof | Dach eines Passivhauses (label) | Passivhausdach | as above |
| cooling / air-change-rates | `apartment` | Apartment, nominal ventilation (label) | Apartment at nominal ventilation | Wohnung, Nennlüftung (label) | Wohnung bei Nennlüftung | comma list inside a name |
| cooling / air-change-rates | `cinema` | Cinema auditorium, sold out (label) | Sold-out cinema auditorium | Kinosaal, ausverkauft (label) | Ausverkaufter Kinosaal | comma list |
| cooling / air-change-rates | `operating-room` | Operating room, room class Ib (label) | Class Ib operating room | Operationssaal, Raumklasse Ib (label) | Operationssaal der Raumklasse Ib | comma list |
| cooling / cooling-load-and-demand | `school-new-build` | New school building, cooled | Cooled new school building | Neues Schulgebäude, gekühlt | Gekühlter Schulneubau | comma list |

Lengths: all ≤ 34 code points. Effective names stay distinct per task in both languages (probe: "problems: none"). No short
holds a value, a range or a category name; "Class Ib", "WSchVO 1982", "GEG" are names, not values.

Kept on purpose (read fine in every template): "Food energy of a 100 g chocolate bar" (the mass is not the answer and makes
the quantity well-defined), "Annual electricity use of two people" / „Jahresstromverbrauch zweier Personen“, "Person sitting
still", "Wall box charging an electric car" (participle modifier, a noun phrase), "Masonry wall to WSchVO 1982", "House to
EnEV 2002" (British "built to"), the demand quiz (no awkward name found), all quantity and axis shorts.

## Site content test

`🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🧪️catalog/🟦️.ts`: new per-quiz case "names every item in hints by a noun phrase
that reads after "N ×": no leading article, no verb with an object, no comma, no straight apostrophe" over `short ?? label`
of every item of every task, `COUNTABLE` = EN `/^(?!(?:The|A|An|One) )(?!\p{L}+ing (?:(?:an?|the)\b|\d))[^,']*$/u`, DE
`/^(?!(?:Der|Die|Das|Ein|Eine) )(?!\p{Lu}\p{Ll}+en (?:eines|einer|des|der) )[^,']*$/u`; docstring extended.

## Runs

- `bun hints3_shorts_probe.ts` → "problems: none" (rule self-test included: it rejects "Boiling a litre of water",
  "Heating 1 litre of water", "The Sun", "World's …", "Apartment, nominal ventilation", "Aufkochen eines Liters Wasser", …).
- `NX_PLUGIN_NO_TIMEOUTS=true SEMIO_TEST_BUDGET_MS=900000 bun nx run @teaching/architecture-quiz:test --skip-nx-cache` → 5 files, **247 passed**.
- `bunx vitest run --config 🧪️tests/🎚️config/🟦️.ts --reporter=verbose catalog` (site root) → **38 passed**, the 4 new cases among them.
- `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:typecheck --skip-nx-cache` → exit 0.
- `NX_PLUGIN_NO_TIMEOUTS=true SEMIO_TEST_BUDGET_MS=900000 bun nx run @semio-tech/quiz:test --skip-nx-cache` → 11 files, **450 passed**.
- `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @teaching/architecture-quiz:check --skip-nx-cache` → exit 0; catalog architecture, 4 quizzes,
  7 badges; physics 3 tasks, heating/cooling/demand 2 each, all accepted (revisions changed: physics `efa4477c…`, heating `59fd45e5…`,
  cooling `b4a4a1bb…`, demand `12ca5b83…`).
- Runtime: `bun hints2_render_probe.ts` (core `hintsOf` + React renderers over the live quizzes) shows the new shorts in real
  hints, e.g. "Are you sure it takes 1.1 × 10¹⁶ × “Annual heating demand of an old house” to add up to the energy of 1 ×
  “Germany’s primary energy use per year”?".

## Deviations and decisions

- **EN keeps "Annual …" where it reads naturally**, "per year" only where it parallels the German re-authoring (the two
  primary-energy items), so the pair reads alike in a hint.
- **GEG context kept in the shorts** ("GEG reference roof", „Referenzdach nach GEG“) instead of the audit's bare "Reference roof":
  the u-values task also holds passive-house parts, so "reference" alone is unclear. The jargon stays but each name is short.
- **Typographic apostrophe (U+2019) in shorts**; labels (display, not mine) keep "'".
- Comma labels below 40 characters got a `short` although the length rule does not require one; the new test makes the comma
  rule binding for every item name.

## Notes for the next agent

- The `⛰️challenge-rules` fixture (`🧰️framework/🛍️products/❓️quiz/🧫️fixtures/⛰️challenge-rules/🔣️.json`) embeds copies of the
  live quizzes with the old shorts ("Boiling a litre of water", "GEG reference building", …). Nothing checks them against the
  live files (the quiz suite passes), but a vectors regeneration (`.venv/Scripts/python.exe TICKET/regenerate_challenge_vectors.py challenge`)
  picks up the new names — not run here, the fixtures belong to the vectors agent.
- The client's mid-sentence lower-casing (task `🟦️.tsx`) only lowers when the second letter is lower case, so "U-value" stays.
