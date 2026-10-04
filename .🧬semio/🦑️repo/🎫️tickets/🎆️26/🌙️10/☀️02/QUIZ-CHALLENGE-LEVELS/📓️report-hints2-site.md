# Report: site pass after hint revision rounds 2 and 3 (design §8.4a, §8.4b)

Agent: site, hint revision round 2. This was the final site pass.

Mid-task, the coordinator extended the brief:
- Keep the spec robust to round 3 (§8.4b): no exact full-sentence matches.
- Take the screenshots only once `📓️report-hints3-{client,core,content}.md` exist.
- Re-run the challenge spec once more after that.

Abbreviations:
- `S` = `🎓️teaching/🏛️architecture/❓️quiz`
- `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`
- `T` = this ticket folder

## Changed files

### Updated

**`S/🧪️tests/⛰️challenge-levels/🟦️.ts`**
- Docstring rewritten.
- Helpers replaced:
  - removed: `names`, `namesAnother`, `together`, `all`, `expectUValueQuestions`;
  - added: `nameOf`, `cited`, `referenceIn`, `unquoted`, `briefCounts`, `namesQuantity`, `orderAsks`, `adds`, `exchanged`, `expectCappedPowers`, `expectExchangeQuestions(device, task)`, `profileAsks`.
- New task constant `loads` (heating `heating-load-and-demand`).
- Easy sorting test: now also checks the cap.
- Easy matching test (EN): also covers the two-quantity matching.
- German matching test: also covers the two-quantity matching ("in puncto"). Renamed to "in German a matching, one of two quantities named in puncto, and a classification with spider profiles ask their easy hints in German".
- Still 13 tests.

**`S/🎭️e2e/🚶️learner/🟦️.ts`**
- `SourceQuantity.short?: Text`.

**`S/README.md`**
- `⛰️challenge-levels` row describes the tightened spec.
- The easy-row examples use the authored short labels ("Burning tea light", "Person sitting still").

**`T/challenge_shots.spec.ts`**
- `easyHint`: two largest exchanged (count questions) for the main shot; the reversed extremes go to `-tail`, or to the main shot when the pair lies within reach.
- New `questioned`, `matchingHint` (heating, two quantities), `profileHint` (demand).
- Two new German tests.

**`T/📸️shots/`**
- Replaced: `desktop-2-easy-hint`, `desktop-2-easy-hint-de`, `phone-2-easy-hint`, `phone-2-easy-hint-de`.
- Added: `desktop-2-easy-hint-matching-de`, `desktop-2-easy-hint-profile-de`, `phone-2-easy-hint-matching-de`, `phone-2-easy-hint-profile-de`.
- All are round-3 texts.

### Created
Probes, kept in `T`:
- `hints2_site_tasks_probe.ts`: task shapes, shorts, familiar flags.
- `hints2_site_reversal_probe.ts`: misses of a powers sorting placed upside down, over all 1001 draws: always 6, 8 or 10, so always more than the cap.
- `hints2_site_drop_count.ts`: a one-off cut of a block with U+00A0 that the Edit tool cannot match.

### Removed
- `T/🗑️generated/hints2-site`, `T/🗑️generated/integration`, after use.

### Not changed
- `R/🎨️.css`: no visible problem needed a fix (see Screenshots).

## What the spec asserts now (robust to §8.4a and §8.4b)

The spec checks structure and does not compare whole sentences. Exact wording and numbers are left to the React and core tests.

Every hint:
- has the expected kind;
- stands beside exactly the expected items;
- is one of at most 3 per task;
- opens with the hint opening;
- names its own item by its `short` label where it has one. A long label no longer satisfies the check.
- Nothing general is said anywhere in the task.

**Extremes exchanged** (powers sorting, U-value matching, heating-load matching): the order question.
- Matches `is larger|higher … than` / `größer|höher ist als`.
- Quotes exactly one item that is placed right as the reference.
- The item the keys make larger is named first.
- Outside the quoted names, the question has no digit at all, so the order is asked for alone.
- It names the quantity: case-insensitive, and in German prefixed with "in puncto".
  - Round 3 names the quantity in every compare question, so this applies to every compare hint.
  - On the two-quantity matching it is the dimension's short: "in puncto Heizlast".

**Two largest powers exchanged** (when they lie farther apart than the reach):
- The largest is asked "together only add up to" / "zusammen nur". The other is asked "it takes" / "braucht, um".
- `× “reference”` comes before `1 × “item”`, and the reference is an item placed right.
- No run of 4 or more digits outside the quoted names, so counts are in words or mantissa × 10ⁿ.
- The quantity is named.
- When the Sun is drawn, it is checked by its short label ("The Sun"/"Sonne" in round 2, "Sun"/"Sonne" after round 3; the spec reads the catalog live).

**Cap:**
- A powers sorting placed upside down (6 to 10 misses) shows exactly 3 hints.
- Each of the 3 stands beside a missing item and opens as a question.

**Profile (demand):**
- If a correctly placed standard lies strictly between the assigned and the own value on the questioned axis, the question has the form `“X” … lies above|is higher / below|is lower … “R”` (German `über|höher ist als` / `unter|niedriger ist als`).
  - R is the standard farthest from X's own value, and the side is the claimed one.
  - The axis is named: EN " in <axis>", DE "in puncto <axis>", case-insensitive.
- Otherwise the value form: "fits … with … at about" / "passt, mit … bei rund", naming the profile and the axis.

**Not pinned** (round 3 changed these, or may still change them): the reference identity (familiar and diversity rules), the number formats, and the exact templates.

## Gates

### Exact commands
All run in `S/📦️packages/🟦️typescript` with `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true`:
- Challenge spec in dev: `bun ./📜️script.ts test-e2e dev --project=desktop --no-deps challenge-levels`
- Full dev gate: `bun ./📜️script.ts test-e2e dev`
- Rehearsal for desktop: `bun ./📜️script.ts test-e2e rehearsal --project=desktop`
- Typecheck: `bun ./📜️script.ts typecheck`
- Site node tests, from the repo root: `bun nx run @teaching/architecture-quiz:test --skip-nx-cache`

### Results, in order

**Challenge spec in dev, run 1** (round-2 code; at that point the spec still asserted exact round-2 sentences, with the reference predicted by the tie window, familiar first and the smallest claim)
- **12 passed, 1 failed.**
- The failure was `hard hides the keys` at the home-card click: `playQuiz` timed out waiting for navigation. This is the same load failure as before and is not hint-related.
- So, against round 2, every exact sentence held in EN and DE: the reversed order question, the "together only add up to" and "it takes" counts in words, "in puncto Heizlast", and the profile relative form.

**Full dev gate** (spec version before the round-3 adaptation; round 3 was landing while it ran)
- **30 passed, 6 failed, 22 did not run** (18.5 min).
- 5 of the failures are `⛰️challenge-levels`:
  - 3 failed on the new wording: "is higher in U-value than", "höher ist als", and "is higher in Heating demand than" in the profile question.
  - 2 (EN and DE powers sorting) showed **no hints at all** after the extremes exchange, while the matching hints in the same run rendered. This was round-3 code in flight. The same tests passed in the rehearsal and in the final run.
- 1 failure is `🎯️quiz-runs` (demand card click timeout): the load failure listed in `📓️report-site-final.md`.
- presence, shortage, away, faults and pets did not run because desktop failed.
- From the list output, every other desktop test passed, and phone and layout passed.

**Rehearsal for desktop** (spec adapted to round 3; release build on the round-3 code)
- **30/30 passed** (11.7 min), including all 13 `⛰️challenge-levels` tests.

**Challenge spec in dev, final run** (after all three round-3 reports existed)
- **12 passed, 1 failed** (4.4 min). Every hint test passed on the round-3 code, in EN and DE.
- The failure was `hard hides the keys`: the same home-card click timeout in `playQuiz`, not hint-related.
- Re-run with `-g "hard hides the keys"`: **1/1 passed** (44.5 s).

**Typecheck:** exit 0, both after the spec edits and after the round-3 adaptation.

**Site node tests:** **5 files, 243/243 passed.**

**React suite**, in `R/📦️packages/🟦️typescript`, `SEMIO_TEST_BUDGET_MS=900000 bun ./📜️script.ts test`
- **22 files, 920/920 passed**, run once at the end.

**Not run:**
- the full dev gate again after the round-3 adaptation: the rehearsal and the final challenge run cover the spec, and the other failures belong to other areas;
- the rehearsal for phone and layout.

**Screenshots:** taken after all three round-3 reports existed, so every shot in `📸️shots` named above shows round-3 texts.

## Screenshots

The questions shown (round 3) are logged by the shot spec.

**Desktop (1440 wide):**
- Every question fits on one line under its label, left of the key or card column. There is no overlap with keys, selects or buttons, and the page width equals the window.
- Examples:
  - EN: "Are you sure 180 million × “Car engine at full throttle” together only add up to the power of 1 × “Sun”?", and "…it takes 3.9 × 10²⁴ × “Person sitting still” to add up to the power of 1 × “Humanity’s primary energy use”?"
  - DE: "Bist du sicher, dass 1,7 Billionen × „Automotor unter Volllast“ in puncto Leistung zusammen nur 1 × „Sonne“ ergeben?"
  - Matching: "…, dass „Neues Passivhaus“ in puncto Heizlast höher ist als „Unsanierter Gründerzeitbau“?"
- The profile hint wraps to two lines inside its chip. One blemish: "Netto-Ener-giekosten" is hyphenated inside an already-hyphenated compound (from `hyphens: auto`). Left as is, because `manual` would break long German compounds without a hyphen on phones.

**Phone (375 wide):**
- Questions wrap to 3–4 lines inside the row, above the key and the buttons, with no horizontal scroll.
- The pets overlay covers part of the footer in the sorting shot. That is decoration and not mine.

**No CSS change was needed.**

## Deviations and decisions

1. **Exact sentences, then structure.** The brief asked for exact text where round 2 made it deterministic, and run 1 verified that against round 2. The coordinator then asked for robustness to round 3, so the final spec asserts structure instead: order of the names, markers, the reference among the anchors, the quantity named, and digit runs.
2. **The quantity is asserted on every compare question.** That is §8.4b.1. It is checked case-insensitively, because EN lower-cases the name and DE keeps the capitals.
3. **The cap check is EN only.** It costs about 45 move clicks, and the German test already covers the rest.
4. **The main sorting shot now shows the count questions.** They are the longest text. The reversed questions are kept as `-tail`, generated only.
5. **The `familiar` field was not added to the driver.** The final spec no longer predicts the reference.
6. **The extra phone shots for matching and profile were copied into `📸️shots`.** They show the narrow layout of the new texts.

## Notes for the next agent

- `expectExchangeQuestions(device, task)` handles any easy matching with a first dimension that is not additive. `profileAsks(item, category, axis, placed, locale)` predicts the relative or the value form.
- Re-shoot with:
  - `PLAYWRIGHT_BASE_URL=http://127.0.0.1:6063 bunx playwright test --config T/challenge_shots.config.ts -g easy`
  - run from `S/📦️packages/🟦️typescript`, with the `architektur-und-technologie-quizze-beside` launch row running.
- Observed (not mine): the radar diagrams still label their axes with the full long labels ("Netto-Energiekosten für Heizung, Warmwasser und Hilfsenergie (nach PV-Gutschrift)"), which wrap to 7 lines on desktop. The axis `short` could serve there too.
