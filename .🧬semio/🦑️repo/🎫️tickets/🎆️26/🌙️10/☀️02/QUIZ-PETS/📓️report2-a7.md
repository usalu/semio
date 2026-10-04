# 📓️ Work package A7 (feeling: moods, species states, tricks, chemistry) — report

Ticket `2026/10/02/QUIZ-PETS`, second round. `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Design: `📓️design-v2.md` §19/§21, `📓️design.md` §2.4, MECH §8, CONTENT §A1/§7/§8.

## 1. What exists

| File | What |
|---|---|
| `P/🔨️modules/💗️feeling/🟦️.ts` | the pure module (725 lines, 88 docstrings, every constant and table exported and documented) |
| `P/🔨️modules/💗️feeling/🧪️tests/🔬️unit/🟦️.ts` | 71 vitest cases (laws, sweeps via `sampled`, every committed vector of both cases, robustness, purity, forbidden maths) |
| `P/🧪️tests/💗️feeling-dynamics/{🥒️.feature, 🐍️.py, 🟦️.ts}` | 10 scenarios, oracle `pets-numpy` |
| `P/🧪️tests/⚗️chemistry-rules/{🥒️.feature, 🐍️.py, 🟦️.ts}` | 8 scenarios, oracle `pets-scipy` (numpy + `scipy.sparse.csgraph`) |
| `P/🧫️fixtures/💗️feeling-dynamics/🔣️.json` (676 KB), `P/🧫️fixtures/⚗️chemistry-rules/🔣️.json` (616 KB) | generated vectors; the chemistry file carries its own menagerie (4 species from the reference rig, 13 reactions, valid: jsonschema in the generator, `menagerieIssues` → 0 findings) |
| `TK/generate_feeling_vectors.py` | generator of both fixtures (expected values = the oracles' own handlers; a second run writes nothing) |
| `TK/a7_docstring_emojis.mjs`, `TK/a7_validate_menagerie.ts`, `TK/a7_probe_sample.py` | checks: docstring emojis/console/comments; both menageries against the product validator; what the sample menagerie carries |

Anchored edits in shared files: `P/🔮️oracles/🔣️.json` (capability `pets-feeling-dynamics` on `pets-numpy`, `pets-chemistry-rules` on `pets-scipy`, one sentence each in both rationales); `P/📦️packages/🟦️typescript/🟦️.ts` (`export * from "../../🔨️modules/💗️feeling/🟦️.ts"` — no export name of the module collides with the schema or any other module, checked over all `export` lines of the package). Nothing in `🧠️behavior`, the schema or the stage was touched. No Rust file (phase D).

## 2. Exports

Runtime types (become schema types when the integrator puts them on `Actor`/`Stage`): `Feeling = { mood, intensity, since }`, `Countenance = { bend, lid, slant, drop }`, `Stir = { occasion, mood, amount }`, `Character = Pick<Species, "mood" | "temperament">`, `Leaning = { affinity, shares: [greet, cuddle, squabble], show: -1 | 0 | 1 }`, `Standing = { state, since }`, `Sighting = { species, state, mood, intensity, activity, x (middle), y (feet), width, height }`, `Cooling = { reaction, when, near, until }`, `Trial = { reaction, when, near, chancy }` (indices), `Consequence = { reaction, on, other, state|null, mood|null, amount, rapport, encounter|null, trick|null }`, `Chemistry = { consequences, coolings, drawn }`, `Occasion`.

| Area | Signature |
|---|---|
| moods | `atRest(resting, tick): Feeling` · `shownMood(mood, intensity): Mood` · `impulse(feeling, mood, amount, tick): Feeling` · `settled(feeling, resting, tick): Feeling` · `settlesAt(feeling, resting): Ticks` |
| face | `valenceOf(mood)` · `spiritsOf(feeling)` · `faceOf(feeling): Countenance` |
| appraisal | `pronenessOf(character, mood)` · `stirred(feeling, mood, amount, character, tick)` · `appraised(feeling, occasion, character, tick)` · `performed(feeling, trick, character, tick)` · `drowsed(feeling, energy, tick)` |
| wishes | `moodWeights(mood, intensity): number[]` (ACTIVITIES order) · `encounterBias(first: Feeling, second: Feeling): Leaning` · `swayedShares(shares, leaning)` |
| contagion | `caught(mine, theirs, affinity, sociability, tick): Feeling` |
| states | `lastingTicks(state)` · `stateAt(species, state, since, tick): Standing` · `stateEnds(species, state, since): Ticks \| null` · `ladderOf(species)` · `rungsOf(species, trick)` · `stepRung(rungs, state, direction)` · `stepState(species, state, direction)` · `stateAfterTrick(species, state, trick)` |
| tricks | `tricksFor(species, cue, state, feeling): Trick[]` · `clickTrick(species, state, feeling, index)` · `whimTrick(species, state, feeling, unit)` · `showTrick(species, state, feeling, unit)` |
| chemistry | `nearby(first, second, reach)` · `seen(first, second, where)` · `matches(trait, sighting)` · `everyTicks(reaction)` · `trialsOf(menagerie, sightings, coolings, tick): Trial[]` · `drawsOf(trials)` · `reactionsOf(menagerie, sightings, tick, coolings, units): Chemistry` |

Constants: `MOOD_PRIORITIES MOOD_HOLD MOOD_FAINT MOOD_REST MOOD_RISE MOOD_OVERRIDE MOOD_DECAYS PRONE_DECAY MOOD_VALENCES MOOD_LIDS MOOD_SLANTS MOOD_DROPS OCCASIONS APPRAISALS PRONENESS PRONE_GAIN TRICK_AMOUNT DROWSY_ENERGY MOOD_WEIGHTS MOOD_AFFINITIES MOOD_SHARES MOOD_SPREADS CONTAGION_BEAT CONTAGION_REACH WILLING_MOODS WHIM_FAVOR CHEMISTRY_BEAT EFFECT_AMOUNT LEAN_TICKS`.

## 3. The model and its tables (all arrays in `MOODS` order: content, happy, playful, curious, proud, sleepy, grumpy, sad, scared)

**Feeling as an anchor.** `intensity` is how strongly the mood was felt at `since`. The stage stores only what `impulse`/`atRest` return and reads the present with `settled` (closed form: settling at t1 then t2 equals settling at t2 — exact for dyadic inputs, proven by the `jumps` vectors and a 200…20 000-feeling sweep). Views are anchored so the same future follows: `since = tick − HOLD` while fading, `tick` while rising, and the tick it came to rest (`settlesAt`) once at rest.

**Rest.** A pet that nothing stirs feels the resting mood of its species (`Species.mood`) at `MOOD_REST = 0.25` — visible as a quarter of that mood's face, a quarter of its weights. Another mood fades linearly after a hold of `MOOD_HOLD = 128` ticks; below `MOOD_FAINT = 0.05` it is over and the resting mood rises back from 0 by `MOOD_RISE = 1/32` per s (8 s). The resting mood stirred above 0.25 fades at half its rate (`PRONE_DECAY`) back to 0.25; soothed below, it rises back.

| | content | happy | playful | curious | proud | sleepy | grumpy | sad | scared |
|---|---|---|---|---|---|---|---|---|---|
| rank (`MOOD_PRIORITIES`) | 0 | 3 | 3 | 2 | 3 | 1 | 5 | 4 | 6 |
| fade per s (`MOOD_DECAYS`) | 1/32 | 1/64 | 1/32 | 1/16 | 1/32 | 1/128 | 1/64 | 1/128 | 5/32 |
| seconds from full strength | 32 | 64 | 32 | 16 | 32 | 128 | 64 | 128 | 6.4 |
| mouth = valence | 0.3 | 0.8 | 0.6 | 0.2 | 0.6 | 0.1 | −0.5 | −0.8 | −0.4 |
| lid | 0.1 | 0.12 | 0 | 0 | 0.15 | 0.55 | 0.35 | 0.25 | 0 |
| slant ° | 0 | 0 | −10 | −10 | 6 | 4 | 12 | −10 | −14 |
| drop px | 0 | −1.5 | 0 | −1 | −1.5 | 1.5 | 0 | 2 | 2 |
| spread (contagion) | 0 | 0.5 | 0.6 | 0.4 | 0 | 0.8 | 0.2 | 0 | 0.3 |
| affinity shift (encounters) | 0 | +0.15 | +0.1 | +0.05 | +0.05 | −0.05 | −0.25 | −0.1 | −0.05 |
| shares ×[greet, cuddle, squabble] | 1,1,1 | 1.25,1.25,1 | 1.25,1.25,1 | 1.25,1,1 | 1.5,1,1 | 1,1,0.5 | 0.5,0.5,1.5 | 1,2,0.5 | 1,2,0.25 |
| performs by itself (`WILLING_MOODS`) | yes | yes | yes | yes | yes | no | no | no | no |

Every face channel and `spiritsOf` = content's value moved towards the mood's by the intensity (no jump when a mood fades: the last faint face differs from content's by < 0.06 in the mouth).

**Impulse** `(feeling, mood, amount, tick)` on the present feeling: amount ≤ 0 → nothing; same mood → `min(1, i + amount)`, hold anew; `content` → **soothes** a mood worse than content (curious, sleepy, grumpy, sad, scared: `i − amount`, over below 0.05), never takes joy (happy, playful, proud) away; another mood (amount ≥ 0.05) replaces when the present feeling is calm (≤ 0.25), or it outranks it, or — hold over — it is of the same rank or `amount > 1.2 × i`; otherwise it is lost.

**Appraisal** (`APPRAISALS`, applied in order, each × `pronenessOf`, clamped to [0, 1]):

| occasion | impulses | occasion | impulses |
|---|---|---|---|
| greeted (hello) | content 0.2, happy 0.3 | welcomed (a pet greets it) | happy 0.3 |
| tricked (no `Trick.mood`) | playful 0.4 (`TRICK_AMOUNT`) | cuddled | content 1, happy 0.5 |
| purred | content 0.5, happy 0.3 | squabbled | grumpy 0.5 |
| lifted (picked up) | scared 0.3 | mended (sulk ended) | content 0.3 |
| dangled (still held) | curious 0.4 | drenched (rained on) | sad 0.4 |
| shaken | scared 0.5 | pestered ("enough") | grumpy 0.4 |
| dropped (hard landing) | grumpy 0.4 | evicted (thrown off a fixture) | scared 0.4 |
| floated (soft, parachute) | content 1, proud 0.4 | watched (pointer lingers) | curious 0.4 |
| landed (soft, no parachute) | content 0.5, happy 0.3 | startled | scared 0.4 |
| trampled (landed on) | grumpy 0.3 | woken | content 1 |
| | | entertained (shown a trick) | happy 0.3 |

`performed(feeling, trick, …)`: the trick's `mood` at 0.4, else the `tricked` row. Proneness (`PRONENESS`, `[base, energy, sociability, curiosity]`): happy and sad `0.5 + sociability`, playful `0.5 + energy`, curious `0.5 + curiosity`, sleepy `1.5 − energy`, grumpy `1.5 − sociability`, content/proud/scared 1; × `PRONE_GAIN = 1.5` for the species' resting mood. `drowsed(feeling, energy, tick)`: `tired = clamp((0.6 − energy)/0.6)`, an impulse of sleepy `tired − present sleepiness` (takes calm pets; a stirred one only after its hold and when > 1.2 × as tired).

**Weights** (`MOOD_WEIGHTS`, 9 × 26, multiplier `1 + (row − 1) × intensity`; only self-started activities differ from 1): happy fidget 1.5, walk/hop/aim/climb/carry/push 1.25, sleep 0.5, trick 1.5 · playful fidget 2, walk 1.5, hop 2, sleep 0.25, aim/climb/carry 1.5, trick 3, push 2 · curious walk 2, hop 1.5, sleep 0.25, aim/climb 2, carry/push 1.5 · proud fidget 1.5, sleep 0.5, trick 2 · sleepy fidget/hop/aim/climb/carry/trick/push 0.25, walk 0.5, sleep 4 · grumpy walk 1.5, hop/sleep/aim/climb/carry/push 0.5, trick 0.25 · sad fidget/hop/trick/push 0.25, walk/aim/climb/carry 0.5, sleep 1.5 · scared fidget/sleep/aim/climb/carry/trick/push 0, walk/hop 0.25.

**Encounters** `encounterBias(a, b)`: affinity `+ (A[a]·i_a + A[b]·i_b)/2`; shares × `(1 + (S[a]−1)·i_a)·(1 + (S[b]−1)·i_b)`; `show` = the prouder of two proud pets above 0.25 (the first of equals). Two grumpy pets at 0.9 on a neutral bond squabble more than twice as often; a scared or sad pet with a friend is cuddled twice as often (cuddle is 0 for non-friends, so only friends comfort).

**Contagion** `caught(mine, theirs, affinity, sociability, tick)` (MECH §8.3): only a stirred mood travels (> 0.25); `gain = spread × sociability_mine × (0.5 + 0.5·affinity)` ≤ 0.8; same mood → `mine + gain·(theirs − mine)⁺`, hold untouched (so neighbours never stall each other's fading); a calm pet (≤ 0.25) adopts at `gain·theirs` if ≥ 0.05, fresh hold; anyone else keeps its mood. Beat `CONTAGION_BEAT = 32`, reach `CONTAGION_REACH = 3` body widths; neighbours applied in species order, each as it stood when the beat began (the crowds of the case: no intensity leaves [0, 1], nobody ends above the strongest of its mood — asserted by the oracle beat by beat).

**States.** `stateAt`: a state with `lasts` gives way to `then` (resting state when `then` names nothing known); chains are walked, a circle of states is reduced by whole laps after `#states` steps (an hour of a 17.5 s circle, or 1.4·10¹¹ ticks in the unit suite, cost a dozen steps). `ladderOf` = authored order; `stepState` holds at both ends (a further turn is a flourish). `stateAfterTrick`: `to` when the species has it, else a circling trick steps one rung up (`circle`) or down (`countercircle`) along **its rungs** — the states it lists in `from`, in that order, or the whole ladder. This is how a resting state can sit in the middle of a ladder (sunny: `from: [sunset, dim, shining, blazing]` on both circling tricks while `shining` is the first state).

**Tricks.** `tricksFor`: cue listed, `from` absent or naming the state, and for the pet's own cues (`whim`, `show`) the shown mood is willing. `clickTrick(…, index)`: click tricks in authored order, round and round (negative indices wrap). `whimTrick`/`showTrick`: `weightedIndex` over 1 per trick, `WHIM_FAVOR = 3` for a trick whose `mood` is the shown mood.

**Shown mood.** `shownMood(mood, intensity)` = the mood above 0.25, `content` at or below. Trick gating and chemistry traits ask this — a sleepy-by-nature pet at rest still does whims, and a trait `mood: grumpy` means a pet stirred into grumpiness.

**Chemistry.** `nearby`: the gap between the two boxes (0 when touching or overlapping), `gapX² + gapY² ≤ within²`. `seen(first, second, where)` (the `when` actor seen from the `near` actor): `above`/`below` = the two share a column (`|dx| < (w1 + w2)/2`) and the middle of the first is higher/lower; `beside` = they share a row but no column; `any` = always (default). `trialsOf`: actors ordered by their species' place in `menagerie.species` (input order breaks ties; unknown species skipped), then for every ordered pair, every reaction in authored order whose traits match, which is near and seen as asked and not cooling for `(reaction, when species, near species)`. `reactionsOf` (one beat, `CHEMISTRY_BEAT = 32`): every trial cools for `everyTicks = max(1, floor(every·64 + 0.5))` whether it happens or not (so `every` is both the cooldown per pair and the retry period of a chance); a reaction with `chance` takes the next unit (`unit < chance`; a missing unit fails); effects become consequences in authored order with `amount` defaulting to `EFFECT_AMOUNT = 0.6`; everybody is judged on the snapshot of the beat; nothing twice per beat (first state, first mood, first trick per actor; first rapport shift and first encounter promise per unordered pair); an unknown state, a trick the species lacks or does not offer in the actor's state is dropped; an effect with nothing left is dropped; returned coolings = still running ones, then the new ones in trial order.

## 4. What the `Reaction` shape cannot say (CONTENT §7, 64 + 6 rules)

**Directly expressible** (one row, or one row per alternative state/mood of "A or B", "any lit state", "not shaded" by enumerating the complement): R01, R02, R04, R07, R08, R11, R13–R15, R16 (one of its two moods), R17, R18, R19 (Effect `trick` of the circling trick: one rung up), R20, R21, R23–R25, R27–R31, R37, R39, R40, R44, R48, R49, R52–R54, R56, R59, R61–R64. CONTENT's "in front of" must become `above` (bodies never overlap) or `beside` with a small `within`; "×1.5/×2 share" becomes `encounter` (a promise of that encounter, see §6).

**Expressible with species data or a decomposition** (author can do it today):
- R03, R35: three parties → pairwise chain (cloud → sun `dim`; sun `dim` near panel → panel `shaded`; battery near panel `sad` → `content` soothing).
- R06 ("peak for 40 s, alone"), the "for N s" outcomes and "else mild" of R57: `lasts`/`then` states.
- R10, R41–R43 ("plays trick X", "insuly tuck-in"): the trick leaves a short state (`to: gusting`, `lasts: 2`) and the reaction keys on it.
- R38 ("raises its blind for 5 s"): a `peeking` state with `lasts: 5, then: lowered`.
- R57 (hottest/coldest): eleven rows; precedence is then the menagerie's species order, not "hottest".
- R58 ("a bar every 6 s"): one row per rung with `every: 6`.
- R33 (thermy mediates): `thermy near {chilly, activity: squabble}` → soothing + `encounter: greet` — it promises the next encounter, it cannot convert the running one.
- R05, R12, R22, R26, R46, R55 (delays, "after 20 s", "within 6 s", "for 12 s"): only approximated by `chance` per `every` (a geometric waiting time).

**Not expressible:**
1. Any species (G1, G4, G6 as rules, generic "any raining cloud"): `Trait.species` is required → 380 rows per G rule.
2. A state held for a time as a precondition (R05 "for 30 s", R26 "open for 12 s", R46 "for 20 s").
3. A negated third party (R34 "no lowered shady within 120 px"), shelter/precedence between reactions (R45 "stays fluffy", "a scared pet ignores all rules except G4").
4. The scene (R36, R47: heating vs cooling scene) — the stage does not know its scene.
5. Behaviour effects (R09 drift ×1.6, R60 look-up odds ×3, G5 "falls asleep") and two moods at once (R16 "proud and sheepish").
6. Affinity conditions (G3 "a friend, affinity ≥ 0.4") and memory ("repeats its last trick", G6).

**Built in instead of rules:** G1 (grumpy bias −0.25, squabble ×1.5 each), G2 (contagion; greet and cuddle ×1.25), G3 in part (sad → cuddle ×2 for friends; `content` effects soothe), G6 (`Leaning.show` + `showTrick`).

**Smallest shape changes proposed** (to A2 / the coordinator; none made here):
1. `Trait.species` optional ("any species"); evaluator: `trait.species === undefined || …` (one line), cooldown key unchanged (species pair). Covers G1, G4, G6 and generic rules.
2. `Trait.held?: number` — seconds the actor has been in its state; needs `Sighting.since`. Covers R05, R26, R46 exactly.
3. `Reaction.unless?: Trait` — no actor matching it within `within` of the `near` actor. Covers R34, R45.
4. `Trait.trick?: Slug` — the trick being performed; needs `Sighting.trick`. Removes the short-state workaround of R10, R41–R43.
5. Scene: `Reaction.scenes?: Slug[]` plus the scene in `Summoned` — only if R36/R47 matter; otherwise author the cooling-scene reading.

## 5. Verification — exact commands and real results

Repository root unless noted; `G` = `TK/🗑️generated/a7` (removed at the end).

| Command | Result |
|---|---|
| `bun TK/taxonomy_name_probe_v2.ts` | `💗️feeling`, `💗️feeling-dynamics`, `⚗️chemistry-rules` already registered (by A2), `validateTaxonomy problems baseline: 0 after: 0` |
| `.venv/Scripts/python.exe TK/generate_feeling_vectors.py` | `feeling-dynamics: 29 stories, 58 decays, 48 jumps, 15 crowds (33 moods adopted on the way), 45 faces, 5 characters, 86 leanings` · `chemistry-rules: 54 relations {above 11, below 6, beside 12, any 54}, 22 states, 72 stages (52 with trials, 131 trials), 23 stories (189 consequences {state 67, mood 107, rapport 41, encounter 8, trick 18}), reactions that never happen in a story: []`; second run: both `unchanged` |
| `bun TK/a7_validate_menagerie.ts` | `⚗️chemistry-rules: 0 finding(s)` · `🧬️schema-conformance: 0 finding(s)` |
| `bun ./📜️script.ts typecheck` (in `P/📦️packages/🟦️typescript`, after the glue edit) | exit 0 |
| `bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "💗️feeling"` (same directory) | `Tests 71 passed (71)`, tests 208 ms (final code) |
| `bun ./📜️script.ts test` (fundamental) | first run: `14 files, 1 failed / 930 passed` — the one failure in `🚧️clearance` (A5, in flight); second run: `15 files, 5 failed / 962 passed` — all five in `👆️gesture` (A6, in flight); 12.4 s / 12.8 s wall. The feeling suite passed in both. |
| `bun ./📜️script.ts test quick` | exit 0, `Test Files 14 passed (14)`, `Tests 931 passed (931)`, 19.8 s wall |
| (in `TEST`) `bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "💗️feeling-dynamics"` | `[test] level=exhaustive cases=1 executed=10 passed=10 failed=0 errored=0 parity=0/0` |
| `… subject exhaustive --implementation typescript … --case "💗️feeling-dynamics"` | `executed=10 passed=10 failed=0 errored=0` |
| `… oracle exhaustive … --case "⚗️chemistry-rules"` | `executed=8 passed=8 failed=0 errored=0` |
| `… subject exhaustive --implementation typescript … --case "⚗️chemistry-rules"` | `executed=8 passed=8 failed=0 errored=0` |
| the four runs again on the final files (the sample menagerie now carrying states, tricks and chemistry) | the same four summary lines (85 s, 59 s, 56 s, 63 s) |
| `… contract --owner "🧰️framework/🛍️products/🐾️pets"` | `6245 high-priority breach(es) across 5 rule(s)` after 15 min (loaded host), all of other owners (`temp/…` and the like): **no line names the pets tree** (`grep -c 🐾` → 0, nothing mentions `feeling` or `chemistry`) |
| `node TK/a7_docstring_emojis.mjs` | exit 0: every docstring of the eight A7 files starts with an emoji + U+FE0F, none repeated per file, no console, no `[DEBUG]`, no comment inside a definition |

The binding TypeScript-against-numpy comparison is the unit suite: it holds every committed vector (computed by the oracles) to the module within 1e-9 and exactly for moods, ticks, ids and orders. The sample menagerie (rehearsed by the oracle): every state of blobby, hoppy and floaty is reachable, each of its three reactions happens when its species stand as it asks.

## 6. Notes for the stage integrator (B2)

- **Actor** gains `feeling: Feeling` (start `atRest(species.mood, tick)`), `state: Slug` + `stateSince: Ticks` (start: `species.states[0].id`, the arrival tick). **Stage** gains `coolings: Cooling[]` and the promised encounters `{ between, encounter, until: tick + LEAN_TICKS }`.
- Read through `settled(actor.feeling, species.mood, tick)` and `stateAt(species, state, stateSince, tick)`; store only at event ticks (`impulse`/`stirred`/`appraised` results, consequences). Never store a view on a plain `ticked` — then any cut of time gives the same frames.
- Horizons (`lull`, `paceOf`, `wake`): a feeling moves until `settlesAt(feeling, species.mood)`; a state changes at `stateEnds(...)`; chemistry can change at the next `CHEMISTRY_BEAT` while `trialsOf` is non-empty, else at the earliest `cooling.until` or the next event.
- **Draws:** `drawsOf(trialsOf(...))` units per beat. Recommended: a new stream `CHEMISTRY_STREAM` in `🎲️randomness` with `counter = tick / CHEMISTRY_BEAT` (`randomWords([seed, CHEMISTRY_STREAM, beat], count).map(unitOf)`): no existing path gains a draw, nothing to store.
- **Consequences:** `state` → `state = s, stateSince = tick` (renews); `mood` → `stirred(feeling, mood, amount, species, tick)`; `rapport` → `clamp(drift + delta, −RAPPORT_SPAN, RAPPORT_SPAN)` on the pair (and `stage.met`); `encounter` → promise for `LEAN_TICKS`: when that pair is paired, the kind is the promised one; `trick` → the trick activity with that trick; when a trick ends: `stateAfterTrick`, `performed`.
- **Sighting:** `x = actor.x`, `y = actor.y` (feet, hover included), `width/height = species.size` (or the clearance body), `mood/intensity` from the settled feeling, `state` from `stateAt`.
- **Weights:** multiply `activityWeights(...)` element by element with `moodWeights(feeling.mood, feeling.intensity)`.
- **Encounters:** `encounterBias(a, b)` → `encounterShares(clamp(affinity + leaning.affinity, AFFINITY_FLOOR, 1))` → `swayedShares` → `weightedIndex`; `leaning.show >= 0` → that side plays `showTrick(...)` instead of its greet clip, the other gets `entertained`.
- **Contagion:** every `CONTAGION_BEAT`, pairs within `CONTAGION_REACH` mean widths and adjacent levels, snapshot of the beat, catchers in species order, givers in species order (exactly the loop of the `contagion` vectors).
- **Clicks:** hello → `appraised(…, "greeted")` + greet; trick tier → `clickTrick(species, state, feeling, n)`; purr → `"purred"`; enough → `"pestered"`; circling → `tricksFor(species, "circle" | "countercircle", state, feeling)[0]`; held → `"lifted"`, later `"dangled"`; shake → `"shaken"` and the `shake` trick; landings → `"dropped"`, `"floated"`, `"landed"`, `"trampled"`; sleep end → `"woken"`; energy → `drowsed` at decisions.
- **Frame:** `mood` = `shownMood(feeling.mood, feeling.intensity)` is the better enum for one-sided marks and tremor; `spirits` = `spiritsOf(feeling)`; the face numbers from `faceOf`.
- The first round's `Actor.mood` scalar and `moodOf(activity)` easing become redundant with `spiritsOf`; `🧠️behavior` was not touched.

## 7. Notes for the Rust twin (phase D)

- Straight-line arithmetic in the written order: `(rate * wait) / 64`, `(rate * PRONE_DECAY * wait) / 64` (left to right), `(MOOD_RISE * span) / 64`, `calm + (table[i] − calm) * intensity`, `row[0] + row[1]*e + row[2]*s + row[3]*c`, `spread * sociability * (0.5 + 0.5 * affinity)`, gap `across*across + tall*tall <= reach*reach`. Comparisons as written (`>=`, `>`, `<`, `!(x > 0)` for NaN).
- `fadesAt`/`settlesAt` start from `floor(...)` and adjust with two short `while` loops; mirror them exactly (they define the tick where a mood ends, the anchor of the resting view). Their guards (`!(i >= 0 && i <= 1)` → at rest from `since`; `fadesAt`: `!(i >= FAINT && i <= 1)`) keep NaN and infinite intensities from looping — keep them, NaN-semantics included. `clickTrick` returns `null` for a NaN index.
- `stateAt`: a bounded loop of `2·count + 2` steps; the lap reduction at step `count` uses integer ticks (`floor((tick − began) / lap) * lap` on i64 is exact).
- Arrays in `MOODS`/`ACTIVITIES`/`[greet, cuddle, squabble]` order as `const` slices; `includes`/`indexOf` are linear scans; `clamp` from trigonometry; `weightedIndex` from randomness; no `f64::max/min` semantics needed (all ternaries).
- The schema twin names `Reaction.where` `place` and `Effect.encounter` an `Option<Activity>` (A2): `Consequence.encounter: Option<Activity>`. `Trial` holds `usize` indices. Projections: `null` for absent state/mood/encounter/trick, `amount 0` without a mood, `rapport 0` without a shift.
- Adapters: the TypeScript adapters in both cases are the template (each scenario a projection keyed by vector id); `sample` reads `shared://🧬️schema-conformance/🔣️.json` and skips species without states.

## 8. Notes for the art and content agents (H1…H4, C4)

- `Species.mood` is felt at a quarter of its strength whenever nothing happens (the face shows it faintly, its weights apply a quarter), impulses of it are 1.5 × as strong and it fades half as fast. `content` is the neutral choice; `sleepy`/`grumpy`/`sad`/`scared` as a resting mood do not stop whims (the shown mood at rest is content) but do bias encounters a little.
- States: the first is the resting one; every state should be reachable (a trick, a `lasts`/`then`, or a reaction) — the case's reachability scenario is the model check. To put the resting state in the middle of a circling ladder, list the rungs low → high in `from` of both circling tricks (no `to`). One trick per step with `from`/`to` (as the sample's blobby does) works as well.
- Chemistry: `within` is the gap between bodies (0 = touching); the `when` actor is seen from the `near` actor; `every` is both the cooldown per pair and the retry period of a `chance`; within one beat the first state/mood/trick per actor wins — across pairs, the species order of the ensemble decides. A mood effect needs an amount above 0.25 to show (default 0.6; seconds ≈ amount ÷ fade + 2); `mood: content` soothes (−amount on worse-than-content moods). Mood traits match the shown mood. See §4 for what needs a workaround.

## 9. Deviations from the brief and the design, and decisions

1. **Resting level 0.25 and a rise back** (MECH §8.2 returns to "baseline" without a level): needed so a non-content resting mood (the trace troupe has happy, playful, grumpy, sleepy) is felt at all without blocking anything. MECH's "half-intensity residual" of a replaced mood is not kept (one mood per feeling).
2. **`content` impulses soothe** instead of competing by rank (rank 0 would never win): "calms to content in 4 s", "the sad ends 20 s sooner" become amounts.
3. **Ranks:** sleepy gets rank 1 (between content and curious, as CONTENT §A1 orders it); design-v2 only says it follows the energy need — `drowsed` does that.
4. **Rates** follow CONTENT's durations (seconds), not MECH's per-second numbers (which give 10 s happiness); all are dyadic so the vectors are exact.
5. **Signatures:** `tricksFor`, `clickTrick`, `whimTrick`, `showTrick` take the `Feeling` (not the mood) and `encounterBias` takes two feelings (intensity counts); `caught` takes `tick` (the adopted mood's hold); appraisal functions take a `Character` (every `Species` is one); `reactionsOf(menagerie, sightings, tick, coolings, units)` returns `{ consequences, coolings, drawn }`. Extra exports listed in §2.
6. **Trait moods match the shown mood** (above 0.25; `content` = calm), not the stored label.
7. **`Effect.encounter`** is applied as a promise of the next encounter of the pair for `LEAN_TICKS` (A2's docstring: "makes their next encounter the given one"); CONTENT's share factors (×1.5, ×2 for 30 s) are not representable.
8. **Rungs from `from`** for circling tricks without `to` (§3) — an extension of "the order of states as authored" that keeps that order as the default.
9. **One resting mood** instead of CONTENT's 2–3 prone moods per species (the schema has one `mood`).
10. `moodWeights` only moves self-started activities; encounter activities are steered by `encounterBias`.

## 10. Open

- Phase D: Rust module twin, Rust adapters for both cases, then `parity`.
- B2: the integration of §6; A2/coordinator: the shape changes of §4 if the chemistry of CONTENT §7 is to be authored without workarounds.
- Tuning by eye (stories gallery): the rest level 0.25, the rates, the appraisal amounts and the weights are starting values; after a change rerun `TK/generate_feeling_vectors.py` (the oracles restate the tables and must change with them).
- The package's fundamental run was red during this work only through sibling suites in flight (`🚧️clearance` once, `👆️gesture` once); `test quick` was green with all 931 tests.
