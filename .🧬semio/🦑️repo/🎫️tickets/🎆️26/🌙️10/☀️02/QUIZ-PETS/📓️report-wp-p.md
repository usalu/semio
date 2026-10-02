# 📓️ Report — work package P (follow-ups from the audits)

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-02. Paths: `P = 🧰️framework/🛍️products/🐾️pets`, `PR = P/🎯️targets/⚛️react`,
`Q = 🧰️framework/🛍️products/❓️quiz`, `QR = Q/🎯️targets/⚛️react`, `S = 🎓️teaching/🏛️architecture/❓️quiz`,
`AP = 🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder, `EV = TK/🗑️generated/wp-p` (tool output: logs and
screenshots, 22 MB; the heavy scratch — stacks, the private build, Playwright traces — is deleted).

**State: every item of the brief is done or decided and recorded. The TypeScript core is final since 10:45 host time
(§2); the pets suites, the site suite, the Protocol v2 oracle and TypeScript subject phases, the taxonomy reports of the
pets and teaching scopes and the release budget are green on the final code. The browser proof passes three times in a
row in both topologies; beside the other desktop specs the pet spec passed in every one of three two-topology runs
after its last fix (§3.4 lists the three failures those runs had and whose they are). Parity against Rust was not run:
it fails until work package M has mirrored §2. Red at the time of writing and not mine: the quiz package's own suite
and typecheck (58 tests, 9 errors: other tickets' leaderboard and navigation work in flight, §4).**

Times: the host shows local time (UTC+2); the session transcript stamps UTC. §3 gives both.

## 1. What was done, per item

### A. Behaviour

| Item | Done | Evidence |
|---|---|---|
| **A1a** pupil travel | `pupilReach(eye) = radius − pupil − 0.25` (never below 0) in `P/🔨️modules/🦴️rig`; `frameOf`, the gallery specimen (`PR/📖️stories`) and `TK/render_species_preview.mjs` use the one rule. `GAZE_REACH` 96 → 32, so a pointer 150 px away already pulls the pupil four fifths of the way out | reach per species 1.5 (flamy) … 2.05 px (cloudy); on screen, desktop: cloudy 1.85–1.90 px, waly 1.52 px (`EV/attention-desktop.txt`, `EV/attention-desktop-final.txt`); phone at 0.8: flamy 1.0–1.08 px (`EV/attention-phone.txt`). Audit measured 0.9–1.55 px before |
| **A1b** turning to the pointer | an idle actor that stands by itself on a perch turns to a pointer more than `width/2 + 12` px beyond its middle, at most once per 56 ticks after a turn. Turning is rewritten around `Actor.faced`: the actor faces the new way at once, the drawing sweeps over the 8 ticks before `faced`, a turn back takes as long as the turn has run (§2.5) | unit block "attending to the pointer" and the rewritten turning tests (stage suite, both companies); trace script `pointer-attention`; screenshots looked at: `EV/attention-desktop-final/01-far-left-5.png` (waly faces left), `02-far-right-4.png` (faces right), `EV/attention-phone/01-far-left-5.png`, `02-far-right-5.png` (flamy) |
| **A1c** lean | `leant`: the bone of the first eye turns 5° and shifts 1.5 px per unit of the gaze across and sinks or rises 1 px per unit down; driven by the gaze spring, so it eases | head matrix in the logs above: rotation 4.6° at full gaze; `03-above-1.png` (both series): pupils and head up |
| **A1d** perking up | a pointer that has rested for half a second within 1.5 heights beside — not on — an idle actor makes it greet (`hail`) when its curiosity need is at least 0.5; the greeting costs 0.6 of it, so the need is the cooldown and `temperament.curiosity` sets how often | `EV/attention-phone.txt`: flamy `05-beside-1…5 greet`, looked at `05-beside-1.png`; `EV/attention-desktop.txt`: cloudy greets; waly (curiosity 0.25) does not — as intended |
| `still` and quiet | `still`: nothing of the above. Quiet (a run): pupils follow, no turn, no lean, no greeting, and a poke is ignored (audit N7) | stage tests "is ignored in a time of concentration: hushed actors rest", "stays as it is for a pointer in a time of concentration, on a still stage, …", "leans after its eyes … not at all in a time of concentration or on a still stage", "does not perk up … in a time of concentration"; spec: nothing is written into the layer under reduced motion while the pointer crosses the window |
| **A2** `ActorFrame.activity` | three schema twins (`🟦️.ts`, `🔣️.json`; the Rust twin already had it from M), `data-pet-activity` on the pet `<svg>` (written only when it changes), `PetLayerProps.onCast` (who is on stage, in menagerie order, whenever that changes; nobody when the show ends; once for a new listener) | `schema generate` + `schema docs` (§4); suites `🖌️pet-depiction`, `🫥️decorative-layer` |
| **A3** `castOf` | one seat for a rotation visitor whenever a rotation exists and capacity ≥ 2; a core that does not fit takes turns; two rings from seeded starts, moved on by one per epoch (§2.3). The preferences line names who is on stage right now ("Here right now: …" / "Gerade hier: …") | behaviour unit tests; case `🧠️behavior-choice` (Python oracle restates it with `numpy.roll`); `EV/attention-desktop-final/page.png`: five of the owner's nine and the visitor thermy at home |
| **A4** audit F3 | `unitOf`, `weightedIndex`, `STAGE_STREAM`, `CAST_STREAM`, `ROTATION_STREAM` in `🎲️randomness`; `randomUnit`, `randomPick` built on them; behaviour, stage and the React layer import them; no second `TWO_POW_32` | randomness unit tests; `🎲️counter-randomness` fixture untouched |
| **A5** first visit | newcomers take the perches that hold the fewest, the ground last (`quarters`); over new ground the company spreads to perches that hold nobody (`spread`); an actor whose surface vanished in a survey that brought new ground leaves and arrives anew instead of falling (`uprooted`) | desktop: within 2 s of the overview five of six on tabs and edges, one on the footer line (`EV/look3/desktop-first-03-home-fresh-*.png`, `EV/attention-desktop-final/page.png`, looked at); phone: one on the card tab, one floating beside the body edge (`EV/attention-phone/page.png`, `EV/look2`); spec test "a learner who has just arrived …" |
| **A5** other desktop pages | measured on the real DOM (`EV/look3-desktop-pages.txt`): the first card's tab and edge lie 35 and 57 px under a 29 px navigation bar — less headroom than a pet is tall (40–56 px plus hover) — and every later card begins 30 px below the card above. No rule defect found; a real lack of room, recorded in design §13.4 | |

### B. Accessibility and UX

| Item | Done | Evidence |
|---|---|---|
| **B6** WCAG 2.2.2 | `PetsSwitch`: a native checkbox with its label ("Show pets" / "Tierchen anzeigen") on the footer line of every screen, outside the legal `nav`, first in the row; nothing on a site without pets. It switches between `off` and the learner's last other choice (`QuizPreferences.petsLiveliness`, `withPets`, `switchedPets`), persisted with the preferences | spec test "a switch on the footer of every screen …, by keyboard": focus + Space on the introduction, the identity step, the overview, the settings page, a run and the results, no horizontal overflow, kept over a reload; quiz suite (three switch tests); phone width: one row with "What is stored" (`EV/attention-phone/page.png`) |
| **B7** reduced motion | under the choice: "Your device asks for less motion, so the pets stay still." / "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos." (`quiz.preferences.petsReduced`, `usePetsReduced`) | quiz suite, spec test "a learner who prefers reduced motion …" (also that the note goes when the device stops asking) |
| **B8** S6 cast line | names who is on stage, not the scene's cast | above |
| **B8** N6 forced colours | new: under the choice "Your device uses its own contrast colours, so no pets show." / "Dein Gerät verwendet eigene Kontrastfarben, deshalb erscheinen keine Tierchen." (`petsForced`, `usePetsForced`); it replaces the motion note | quiz suite (new test), spec test "a device that forces its own colours …" |
| **B8** N5 keep-outs | `PET_TEXTS` gained `code, output, progress, meter, [role="alert"], [role="status"]`; bare text is marked `data-pet-keepout` (documented). The quiz's read-outs are `<progress>` elements and are covered | survey vectors regenerated (15 scenes, 42 surfaces, 76 keep-outs), suite `📡️surface-survey` |
| **B8** N7 | a click on a pet does nothing while quiet | stage test |
| **B8** N8 / F23 | gallery wording "Tierchen" throughout | `PR/📖️stories` |
| **B8** N10 | "display name" instead of "accessible name" in `AP/README.md`; the design says so in §13 | |
| **B8** N12 | a "React target" section in `P/README.md` (props, defaults, what the layer never does, pacing, gallery) — the quiz documents its target in the product README too, so no separate file | |
| decided, not changed | S9 results are not quiet (a learner who reads a score is done concentrating; the switch is on that screen) · N1 the unknown-scene fallback stays (`petScene` only names `home` or a catalogued quiz, and `🐾️pet-cast` fails when a quiz has no cast) · N4 the focus margin stays 8 px (the quiz's focus ring is at most 4 px, every control is a keep-out anyway; 24 px would clear the perches beside every focused control) · N11 the group label stays "Pets" (sibling groups are nouns; the options say what is set) · S7/F22 `servy` skipped as briefed | recorded in design §13.5, §13.6 |

### C. Browser proof (`S/🧪️tests/🐕️pet-walk/🟦️.ts`, 13 tests)

New or rewritten: a learner who has just arrived finds walkers whose drawing ends within 3 px of the top of a tab or a
body (`getBoundingClientRect` of the parts against the card parts); pupils follow the pointer to the left edge, the
right edge, the left edge again and upwards, and the pets turn round to it; a lid shuts and opens again; in `lively` a
pet's feet move at least 4 px while `data-pet-activity="walk"` and two pets carry `greet|cuddle|squabble` at the same
time; the footer switch by keyboard on every screen; the reduced-motion and the forced-colours notes; the cast line
equals the pets on screen. No `waitForTimeout` is left: every wait is on `data-pet`, `data-pet-activity`, a style or a
count of animation frames.

Test seam (documented in the spec header, the quiz module and `Q/README.md`): `data-pets-tempo` on the document root
(`QUIZ_PETS_TEMPO`), read once per mounted layer and passed to `PetLayer` as `tempo` (clamped to ⅛…8, it scales the
pacer's clock). Only the walk-and-meet test sets it (8): a first encounter took 52–89 s of real time in four lively
sessions. Nothing in the client sets it.

### D. Hygiene

| Item | Done |
|---|---|
| **D9** F2 | unique docstring emojis in every pets file (checker `…/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/domain_docstring_emojis.py` over 54 files: only its two known false positives, glob strings in the two vitest configs); `QR/⚖️legal` too, which I touched |
| F3, F8, F10, F11, F12, F13, F26, F28 | done in the core (§2); F13 reproduced before fixing: 2 112 of 36 005 one-decimal pairs a whole number of turns apart were falsely reported |
| F5, F14 | design corrected (§D11) |
| F7 | the survey fixture's key is `$comment`; the registry's `_comment` stays (it is the harness schema's own field) |
| F9 | `🦴️rig` header says that `compose`, `invert`, `IDENTITY` are API for tools and tests |
| F15, F16 | `rest(hidden)`; a running show sets `pointer-events: none` on the layer itself and removes it when it ends (test without stylesheet) |
| F17 | `PetLayer` shows nothing of a menagerie with validation issues (`menagerieIssues`, once per menagerie); test with a `url(…)` palette |
| F18, F19 | docstrings already said it (twice under a development double mount; React reports what the boundary caught) |
| F21 | `AP/🟦️.ts` has one named export; `🐾️pet-cast` asserts that; the gallery finds named exports |
| F24 | `react-dom` is a devDependency of `@semio-tech/pets-react`; `bun.lock` edited to match and checked with `bun install --frozen-lockfile --dry-run` (exit 0, nothing installed) |
| F25 | `@semio-tech/pets:typecheck`: `P/📦️packages/🟦️typescript/tsconfig.json` (core, unit suites, Protocol v2 adapters: 32 files), script, nx target, root script `typecheck:pets`, launch row `🛠️dev🐾️pets🟦️🪁️typecheck` (213.715; `.vscode/launch.json` regenerated, `check-generated` exit 0) |
| not mine | F4, F6, F20 are Rust (M did F4: `Facing`, `Rate` enums); F22 skipped as briefed; F27 recorded as residual risk in design §13.8 |
| **D10** schema catalog | `schema generate` and `schema docs`: `framework.product.pets` is in `📓️schema-catalog.md` (3651 scopes), hashes of the three twins current |
| `🔒️dependencies.json` | **generator not run.** `verify dependencies` reports 75 new entries repository-wide; five name a pets manifest among their users (`@types/aria-query@5.0.4`, `@types/d3-ease@3.0.2`, `aria-query@5.3.0`, `d3-ease@3.0.1`, `polygon-clipping@0.15.7`, each also used by `ui-react` or `quiz-react`). `write-baseline` would approve all 75 (other tickets', `temp/brepkit` among them), not only these (`EV/verify-dependencies.txt`) |
| taxonomy | the stale names `🖌️pet-depiction` and `🐾️pet-companions` are gone from `members-of-fixtures` (they stay in `members-of-tests`) |
| nx inputs | `@semio-tech/quiz-react` `namedInputs.default` names the pets sources the quiz bundles (schema, modules, both packages, the React target) |
| docs | `P/README.md` (randomness, rig, `castOf`, the pointer, spreading, the React target, the new command), `Q/README.md` `### Pets` (switch, notes, on-stage names, quiet, tempo), `AP/README.md` (visitor at home, casts, display name, the export, the spec); no file of pets, quiz or teaching names port 6071 or 6072 any more except the design's history note |
| **D11** design | `📓️design.md`: wrong statements corrected in place in §2.4, §4.2, §4.3, §4.5–§4.8, §5.1, §5.2, §5.7, §6.1–§6.4, §7, §8.1, §8.2; new **§13 As built** (nine tables: every deviation of every `📓️report-wp-*.md` and of this package, with its reason and its owner) |

## 2. What the Rust twins must mirror (TypeScript core, final since 10:45 host time)

Everything below is in the TypeScript with green suites (`bun ./📜️script.ts test` in `P/📦️packages/🟦️typescript`: 8 files,
510 tests) and regenerated vectors. Constants are literals; expressions are written in the order shown.

### 2.1 `P/🧬️schema` (three twins; the Rust twin already carries both fields)

| Type | Change | Why |
|---|---|---|
| `Actor` | new required `faced: Ticks`, listed after `facing` | the tick from which the actor faces `facing` squarely; it turns round over the 8 ticks before (§2.5) |
| `ActorFrame` | new required `activity: Activity`, listed after `facing` | observability: a render target shows what an actor does (`data-pet-activity`) |

### 2.2 `P/🔨️modules/🎲️randomness/🟦️.ts`

| Name | Change | Why |
|---|---|---|
| `STAGE_STREAM = 0xffffffff`, `CAST_STREAM = 0xfffffffe`, `ROTATION_STREAM = 0xfffffffd` | new exported constants | one place for the reserved streams (code audit F3); stage, behaviour and the React layer import them |
| `unitOf(word)` | new export: `word / 4294967296` (moved here from the stage) | one unit conversion |
| `randomUnit(key)` | now `unitOf(randomWords(key, 1)[0])` | same value as before |
| `weightedIndex(weights, unit)` | moved here from behaviour, body unchanged (−1 for no positive weight) | one weighted pick |
| `randomPick(key, weights)` | now `weightedIndex(weights, randomUnit(key))` | same value as before |

### 2.3 `P/🔨️modules/🧠️behavior/🟦️.ts`

- `weightedIndex` and the private `CAST_STREAM` are gone (imported from randomness); `encounterOf` calls the imported one.
- **`castOf(cast, capacity, epoch, seed)` has a new rule** (owner: every species of a cast must be able to appear):

```
room   = capacity > 0 ? floor(capacity) : 0
core   = cast.core without repeats;  pool = cast.rotation without repeats and without core members
spare  = room − core.length                      (signed)
least  = room >= 2 ? 1 : 0
open   = spare > least ? spare : least
guests = open < pool.length ? open : pool.length
seats  = room − guests
words  = randomWords([seed, CAST_STREAM, 0], 2)   (word 0: the core's ring, word 1: the rotation's ring)
troupe = core.length <= seats ? core : turnsOf(core, seats, words[0], epoch)
troupe ++ turnsOf(pool, guests, words[1], epoch)
```

  New private `turnsOf(ring, seats, word, epoch)`: empty for an empty ring; `turn = floor(epoch) % count` made
  non-negative (`rem_euclid`); `start = (word % count) + turn`; the members `ring[(start + i) % count]` for
  `i < count && i < seats`. Home (nine core, eleven rotation) at capacity 6 → five of the core in turn and one visitor;
  a quiz cast (3 + 4) at capacity 2 → one core member in turn and one visitor; capacity 1 → one core member.
- The `//#region 🔖️Adapters` lines around the imports are gone (also in terrain and stage; cosmetic).

### 2.4 `P/🔨️modules/🦴️rig`, `📐️trigonometry`, `🏞️terrain`, `✅️validation`

| File | Change | Why |
|---|---|---|
| `🦴️rig/🟦️.ts` | new export `pupilReach(eye)`: `reach = eye.radius − eye.pupil − 0.25; reach > 0 ? reach : 0` | A1(a): the pupil travels up to the outline of the white; one rule for the stage, the stories and the preview tool |
| `📐️trigonometry/🟦️.ts` | `sinTurns(turns: Turns)`, `cosTurns(turns: Turns)` (type only; `Turns` of the schema is used now) | code audit F10 |
| `🏞️terrain/🟦️.ts` | docstring of `hopOf` says "within" like the twin; no code change | F12 |
| `✅️validation/🟦️.ts` | `loop-seam` on the rotation channel: `revolutions = (to − from) / 360; apart = revolutions − floor(revolutions + 0.5)`; a seam when `channel ≠ rotation` or `apart > 1e-9` or `apart < −1e-9` (was `floor(revolutions) ≠ revolutions`) | F13 reproduces: 2 112 of 36 005 pairs of one-decimal angles a whole number of turns apart were reported (−359.8 → −719.8 gives −0.9999999999999999). New vectors: accepted `decimal-turn-loop`, `decimal-two-turns-loop`; rejected `almost-a-turn-loop` (0 → 359.999). The Python oracle of the case restates the same line |

### 2.5 `P/🔨️modules/🎪️stage/🟦️.ts` — turning round, rewritten around `Actor.faced`

Before, an actor kept its old facing for the first 8 ticks of an activity (clock: `since`) and flipped then. Now an
actor that must turn **faces the new way at once** and its drawing follows over the 8 ticks up to `faced`. One clock
for every turn makes a turn in the middle of an activity (the pointer) eased as well, and nothing can pop.

| Function | Now |
|---|---|
| constants | removed `STAGE_STREAM`, `TWO_POW_32` (imported); `GAZE_REACH` **96 → 32**; new `TURN_REST = 56`, `TURN_CLEAR = 12`, `LEAN_TURN = 5`, `LEAN_REACH = 1.5`, `LEAN_NOD = 1`, `PERK_LINGER = 32`, `PERK_URGE = 0.5`, `PERK_COST = 0.6` |
| `unitOf` | removed (imported from randomness) |
| `breathOf(kind)`, `facingTo(towards, x)`, `blinkAt(now, unit)` | new private helpers with the expressions the twin already factors out (F8); every former inline use calls them |
| `headingOf(stage, actors, kinds, index, now)` | new signature. Walk/hop and partner cases as before. New last case — `activity == idle`, no partner, `perch != null`, not leaving, `!stage.quiet`, `stage.pointer != null`, `now − stage.pointed < POINTER_TICKS`, `now >= actor.faced + TURN_REST`: `across = pointer.x − actor.x; clear = width / 2 + TURN_CLEAR; across > clear → 1; across < 0 − clear → −1; else 0` |
| `turning` | removed (`tick < actor.faced` says it) |
| `turn(body, way, now)` (new) | nothing when `way == body.facing`; else `left = body.faced > now ? body.faced − now : 0; body.facing = way; body.faced = now + TURN_TICKS − left` (a turn back takes as long as the turn has run) |
| `swivel(draft, index, now)` | `now < faced → true`; `now == faced → (walk: since = now) → true`; else `heading = headingOf(…, now)`; `heading == 0 or == facing → false`; else `turn(body, heading, now) → true` |
| `stroll`, `attend`, `sulk`, `meet` | unchanged (they never set `facing`) |
| `arrive` | body gets `faced: now` |
| `freeze` | sets `body.faced = now` (after `shift`) |
| `poseOf(kind, actor, tick, stream)` | the `turns` parameter is gone: `weight = tick < actor.faced && activity == walk ? 0 : weightOf(…)` |
| `squeezeOf(actor, tick)` | `tick < actor.faced ? 2 × smoothstep((TURN_TICKS − (actor.faced − tick)) / TURN_TICKS) − 1 : 1` (from −1, the old way, through 0 to 1) |
| `paceOf` | `turning(…)` became `tick < actor.faced` |
| `lull` | per actor: `next <= body.faced → 0`; `heading = headingOf(…, next)`, `heading != 0 && heading != body.facing → 0`; after the `until` horizon: `pointer != null && next < body.faced + TURN_REST → horizon = min(horizon, body.faced + TURN_REST)`; after the loop, before the pointer-interest horizon: `pointer != null && next <= pointed + PERK_LINGER → horizon = min(horizon, pointed + PERK_LINGER)` |
| `frameOf` wake | per actor (after the blink horizon): `stage.pointer != null && tick + 1 < actor.faced + TURN_REST → min(horizon, actor.faced + TURN_REST)`; before the pointer-interest horizon: `actors.length > 0 && pointer != null && tick + 1 <= pointed + PERK_LINGER → min(horizon, pointed + PERK_LINGER)` |

### 2.6 Stage — attending to the pointer (A1)

| Function | Now |
|---|---|
| `look` | the sleeper's special case is gone: a sleeper's gaze springs to the centre like any other (the lean follows the gaze, so nothing may jump) |
| `hail(draft, index, x, now)` (new) | `words = randomWords(actorKey, 2)`; `shift(greet)`; `turn(body, facingTo(x, body.x), now)`; `until = now + dwellOf(greet, mode, unitOf(words[0]))`; `clip = clipAt(kind, greet, unitOf(words[1]))`; `goal = body.x`; `partner = null` — the tail of the old `poke` |
| `poke` | returns when `mode == still` **or `quiet`**; after the cheer and the reconcile it calls `hail` (the facing is no longer set at once: eased) |
| `perk(draft, index, now)` (new) | returns unless `!quiet`, `pointer != null`, `now == pointed + PERK_LINGER`, no partner, not leaving, `perch != null`, `watched(pointer, body, kind)`, `presenceOf(pointer, body, kind) == 1` and `needsAfter(needs, idle, now − since, temperament).curiosity >= PERK_URGE`; then `hail(draft, index, pointer.x, now)` and `needs.curiosity = max(needs.curiosity − PERK_COST, 0)` (energy and sociability as `shift` left them) |
| `act` | the motion branch ends `… else if (activity == sleep) { if (watched) settle } else if (activity == idle) perk(draft, index, now)` |
| `leant(kind, pose, across, down)` (new) | no eyes → the pose; else the bone whose id is `face.eyes[0].bone` gets `x + LEAN_REACH × across`, `y + LEAN_NOD × down`, `rotation + LEAN_TURN × across`, scales unchanged |
| `frameOf` per actor | `across = forward ? gaze.x : 0 − gaze.x`; eyes `{ x: across × pupilReach(eye), y: gaze.y × pupilReach(eye), lid }`; `pose = still ? restPose : poseOf(…)`; `solveRig(kind, still || stage.quiet ? pose : leant(kind, pose, across, gaze.y))`; the frame carries `activity: actor.activity`; the comparator of the order answers 0 for equal species (shape only) |

### 2.7 Stage — spreading out (A5)

| Function | Now |
|---|---|
| `crowdOn(draft, perch)` (new) | actors with `perch == perch.surface` and `perch.x0 <= x <= perch.x1` |
| `roomsFor` | every room carries `crowd: crowdOn(draft, perch)` |
| `quarters(draft, rooms)` (new) | `fewest` = least `crowd` among the rooms; `ground` = the greatest `y` of **all perches of the stage**; the rooms with `crowd == fewest` and `perch.y < ground` when there are any, else all rooms with `crowd == fewest` (order kept) |
| `arrive` | chooses among `quarters(draft, roomsFor(draft, kind))` (same three stage words: perch, place, facing) |
| `widened(draft, before)` (new) | whether a surface id of `draft.surfaces` is not in `before` |
| `carry(draft, index, before, uprooted, now)` | without a perch on its surface or under its feet: `uprooted || mode == still → vanish, −1`; else `drop` as before |
| `ride(draft, before, uprooted)` | passes `uprooted` on; `summon` calls `ride(draft, draft.surfaces, false)` |
| `spread(draft)` (new) | nothing when `quiet`. `vacant` = perches with `crowdOn == 0`, in perch order. Per perch, actors in order that are not leaving and stand on it: the first stays; each further one that is `idle` without a partner takes the first vacant perch at least as wide as its body (`x1 − x0 >= width`), which leaves the list; it then `leave`s — on a still stage it is collected and removed (highest index first) |
| `survey` | `uprooted = widened(draft, before)` (after the new surfaces are stored); `measure`; `ride(draft, before, uprooted)`; `spawn`; when `uprooted`: `spread(draft)`; `spawn` |

### 2.8 Adapters, vectors, tests

- `P/🧪️tests/🎪️stage-trace/🟦️.ts`: the digest folds **the index of the activity in `ACTIVITIES`** after `facing` and before
  `opacity`. Two new scripts: `pointer-attention` (150 s: a pointer that rests beside the pets, crosses behind them, jitters
  from side to side, and a hushed stretch) and `new-ground` (120 s: a company on a floor that gets cards, loses them, gets
  other ones, hushed, still). 13 scripts now.
- `P/🧪️tests/🧠️behavior-choice/🟦️.ts` imports `weightedIndex` from randomness; its Python oracle restates the new `castOf`
  (`numpy.roll` for both rings, `generate_state(2)`); scenario `casts` has a new title.
- Regenerated (generators in `TK`, second runs "unchanged"): `🧠️behavior-choice` (casts), `🎪️stage-trace` (all traces: pupil
  travel, lean, activity, arrival places), `🧬️schema-conformance` (three new vectors). `🤝️bond-dynamics` is unchanged.
- TypeScript unit suites: stage 183 → 191 tests per the two companies (turning rewritten, new `attending to the pointer`
  block, spreading, uprooting, poke in quiet, `activity` in frames), behaviour (new `castOf` cases), randomness (streams,
  `unitOf`, `weightedIndex`). The Rust unit suites need the same cases.
- Parity against Rust fails for `🧠️behavior-choice`, `🎪️stage-trace` and `🧬️schema-conformance` until the twins follow
  (expected; not run by me).

### 2.9 Later changes of the core

No code changed after 10:45 (`🎪️stage/🟦️.ts` and `🧠️behavior/🟦️.ts` were last written at 10:38 and 10:37). One comment:
the header of `🦴️rig/🟦️.ts` now says that `compose`, `invert` and `IDENTITY` are API (F9); the twin's header may say
the same. TypeScript only: the core package got `tsconfig.json` and a `typecheck` target.

## 3. The gate failures of the morning

### 3.1 Cause

The five tests failed because **my change of `castOf` (a rotation visitor at home) reached the working tree an hour
before the spec was changed to allow one**, and gates of another ticket ran in that hour.

| UTC (host time) | What |
|---|---|
| 07:36 (09:36) | `castOf` gets the visitor seat (A3) |
| 08:02, 08:11 (10:02, 10:11) | `onCast` in the layer, the footer switch |
| 08:41 (10:41) | the spec's `expectCast` allows the rotation and no longer asks for a whole core |
| 08:55 (10:55) | the coordinator's message arrives |

The spec as it stood until 08:41 (`expectCast`, its line 120–132) allowed at home `cast.core` only and failed with
`strangers: [<visitor>]` at line 131. Four of its tests call `expectCast(…, HOME, …)`: 224 (line 232), 246 (line 256),
311 (line 316) and 355 (line 358). In 246 the call comes after a journey, when the visitor is long there — every run.
In 355 it is the first thing after the pets appear, and a poll passes at the first moment its value fits: when it looked
before the visitor had arrived (arrivals are one per whole second) it passed — "in some runs". A gate with
`TEACHING_ARCHITECTURE_QUIZ_WATCH=off` serves the sources as they were when it started; the "08:00–09:00" of the report
are therefore UTC, the hour between the two changes. Load, four workers and the proctor played no part in these four.

### 3.2 Proof

The old spec is recovered from the session transcript (`TK/wp_p_pet_walk_before.ts`, only its two paths rewritten) and
run against today's code on the private dev stack (`TK/wp_p_before.config.ts`, `--repeat-each 2`,
`EV/before-1/playwright.txt`): **224, 246, 311 and 355 fail in both repeats, seven of the eight at line 131** with
`strangers` = kettly, flamy, shady, thermy, venty (twice), windy — one rotation visitor each; 274, 327 and 373 pass.
(The eighth, a repeat of 224, passed the cast check before the visitor arrived and then failed on a control another
ticket has since renamed.)

**373 (the phone) is not explained by this**: it only checks a quiz's cast, which always allowed the rotation, and it
passes against today's code. Candidates I cannot tell apart without the gate's logs: my stage edits in flight during
that hour (pets that fell to the footer when new ground appeared, fixed by the `uprooted` rule), or the host
(`net::ERR_NO_BUFFER_SPACE`, §3.4).

### 3.3 What changed in the spec

- `expectCast(device, scene, present)`: core and rotation are allowed in every scene, nobody of another cast, nobody
  twice, and — where asked — somebody of the cast; never the whole cast.
- Every wall-clock sleep is gone; stillness is "nothing is written into the layer for 90 animation frames", motion is
  a changed drawing, a blink is a lid that shuts and opens, a walk is feet that move while the depiction says `walk`.
- Timeouts that depend on a cold dev server or on a change of cast are explicit and generous (`APPEAR_MS` 30 s,
  `RECAST_MS` 45 s, `BLINK_MS` 30 s, `MEET_MS` 120 s at tempo 8); pointer checks nudge the pointer and retry, because a
  pointer is worth a look for four seconds only.
- Found by the gate-like runs and fixed: the pupil check measured the offset across only; a pet on the footer line
  looks mostly *up* at a pointer in the middle of the window's edge (sunny: 0.65 px across of 1.85). It now measures
  along the line from the eye to the pointer (`lookTowards`). 24 repeats of that test on both topologies afterwards: no
  assertion failed.
- The spec stays in its own project `pets` of `S/🎭️e2e/🎚️config/🟦️.ts`; I did not edit that file.

### 3.4 Proof under gate-like conditions

Private stacks, never the gate's ports: dev 6193/8923 (`dev-site` with `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`),
rehearsal 6194/8924 (the release build of the working tree, proctor in production mode). `TK/wp_p_gate.sh` runs
`🪪️first-visit`, `🥞️layered-home`, `🎯️quiz-runs`, `🏆️live-leaderboard`, `🗣️both-languages` and `🐕️pet-walk` side by side
(`fullyParallel`) with the phone project beside them, against one proctor, both topologies at the same time with two
workers each — what the gate does when two topologies share the machine.

| Run (both topologies at once) | dev | rehearsal | Failures |
|---|---|---|---|
| `final-shared` | 29 of 30 | **30 of 30** | dev: `🪪️first-visit` — `net::ERR_NO_BUFFER_SPACE` on the request for housy's document |
| `final-shared-2` | **30 of 30** | 29 of 30 | rehearsal: the pet spec's pupil check (the geometry above; fixed) |
| `final-shared-3` (final spec) | 29 of 30 | **30 of 30** | dev: `🗣️both-languages` — 25 controls counted on the overview in one language, 30 in the other |

The pet spec itself: 13 of 13 in five of the six, 12 of 13 once (fixed). Alone, four workers, three times in a row
with the final spec: **dev 14, 14, 14 passed; rehearsal 14, 14, 14 passed** (13 pet tests and the phone spec).

What the three failures are:

1. `ERR_NO_BUFFER_SPACE` is the host running out of socket buffers while several sessions run stacks and gates; the
   other ticket's closing summary records the same. It hit a pets document once and a font once (`EV/pupils-dev`). It
   is not a pets defect, but pets raise the exposure in the dev topology: a fresh learner there causes 265 requests,
   40 of them for the pets (22 documents of the menagerie, 17 modules of the product; `TK/wp_p_requests.mjs`); in the
   release build the pets are three chunks.
2. The pupil geometry: mine, fixed.
3. `🗣️both-languages` reads the overview twice and compares the number of controls; the overview changed in between
   (other specs submit runs to the same proctor). Pets contribute the same one label to both readings. Not mine; it
   passed in the other five runs.

## 4. Commands and real results (final code unless a time is given)

| Command | Result |
|---|---|
| `bun ./📜️script.ts test` in `P/📦️packages/🟦️typescript` | 8 files, **510 passed** |
| `bun ./📜️script.ts typecheck` there (new) · `bun nx run @semio-tech/pets:typecheck` | exit 0 · exit 0 |
| `bun ./📜️script.ts test` in `PR/📦️packages/🟦️typescript` | 4 files, **82 passed** |
| `bun ./📜️script.ts typecheck` there | exit 0 |
| `bun ./📜️script.ts test` in `QR/📦️packages/🟦️typescript` | 11:41: 18 files, 496 passed, 3 failed (`🏠️home-grid`: hash navigation, another ticket in flight). 13:40: 19 files, **541 passed, 58 failed** — `🏠️home-grid` 52, `📇️learner-pages` 3, `🚶️learner-journey` 3: `state.leaderboards[boardKey(…)]`, sortable column heads, renamed regions; none in a pets test |
| `bun ./📜️script.ts test "🐾️pet-companions"` there | 1 file, **28 passed** |
| `bun ./📜️script.ts typecheck` there | 11:43: 4 errors (`trail` missing in four test fixtures). 13:40: 9 errors, all in `🧪️tests/🏠️home-grid/🟦️.tsx` (`boardKey`, `leaderboard`); **none in a pets file, none added by me** |
| `bun ./📜️script.ts test` in `S/📦️packages/🟦️typescript` | 5 files, **132 passed** (`🐾️pet-cast` among them) |
| `bun ./📜️script.ts typecheck` there | 1 error, `QR/🔨️modules/🛂️proctor/🟦️.ts(409)` (leaderboard query, another ticket); none in the pet spec |
| `.venv/Scripts/python.exe TK/generate_{behavior,schema,survey}_vectors.py` (second runs) | every fixture "unchanged"; 13 scripts; accepted=21 structural=26 rules=63; 15 scenes, 42 surfaces, 76 keep-outs |
| `oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case …` (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) | `🧠️behavior-choice` executed=9 passed=9 · `🤝️bond-dynamics` 6/6 · `🧬️schema-conformance` 3/3 · `🎪️stage-trace` not exercised (no-oracle decision) |
| `subject exhaustive --implementation typescript --owner … --case …` | `🧠️behavior-choice` 9/9 · `🤝️bond-dynamics` 6/6 · `🧬️schema-conformance` 3/3 · `🎪️stage-trace` 3/3 |
| parity against Rust | **not run**; fails until M has mirrored §2 |
| `NX_PLUGIN_NO_TIMEOUTS=true PROCTOR_URL=http://127.0.0.1:8924 bun ./📜️script.ts build --outDir EV/site-rehearsal --emptyOutDir` in `S/📦️packages/🟦️typescript` | exit 0; entry `assets/🌐️-C5RXcULX.js` 764 800 B, **gzip 216 519 B < 260 000**; lazy: menagerie 331 822 B (gzip 36 199), layer and core 54 156 B (20 163), validation 8 979 B (3 267); stylesheet gzip 37 944; 62 files, 2 556 999 B |
| `bun ./📜️script.ts verify taxonomy report --scope …` (repository root) | pets: `clean=true errors=0 warnings=0` · teaching: `clean=true errors=0 warnings=0` · quiz: `errors=5` — four unregistered directories of other tickets (`📊️plot`, `🚏️navigation` three times) and one file that changed during the scan; none in a pets directory |
| `bun ./📜️script.ts schema generate` · `schema docs` · `schema verify` | 3651 scopes, exit 0 · exit 0 · exit 1 with 13 findings, all `framework.schema` Rust entries (`rust-entry-format-absent` 10, `rust-scope-unknown` 3), none for pets |
| `bun ./📜️script.ts verify dependencies` | exit 1: 75 new entries (§1 D10); baseline not rewritten |
| `bun install --frozen-lockfile --dry-run` | exit 0 |
| `bun nx run @semio-tech/plugin-registry:generate` · `:check-generated` | exit 0 (launch row added) · exit 0 |
| browser proof | §3.4 |

Not run, as told: cargo, the end-to-end gate, deploy-check, other owners' parity.

## 5. Files of this package

Core: `P/🔨️modules/{🎲️randomness,🧠️behavior,🦴️rig,📐️trigonometry,🏞️terrain,✅️validation,🎪️stage}/🟦️.ts` and the unit suites of
stage, behaviour and randomness; `P/🧬️schema/{🟦️.ts,🔣️.json}`; `P/🧪️tests/{🎪️stage-trace,🧠️behavior-choice,🧬️schema-conformance}`
adapters; four fixtures; `P/📦️packages/🟦️typescript/{tsconfig.json,📜️script.ts,📋️project.json}`; `P/README.md`.
React target: `PR/🔨️modules/{🖌️depiction,🫧️layer,📡️survey,⏲️pacing}`, `PR/📖️stories`, `PR/📦️packages/🟦️typescript/package.json`,
suites `🖌️pet-depiction`, `🫥️decorative-layer`, `📡️surface-survey`. Quiz: `QR/🔨️modules/{🐾️pets,🎛️preferences,🌐️i18n,⚖️legal}`,
`QR/🟦️.tsx`, `QR/📦️packages/🟦️typescript/📋️project.json`, suites `🐾️pet-companions`, `🏠️home-grid`, `📡️presence-client`
(`petsLiveliness` in their fixtures), `Q/README.md`. Site: `AP/🟦️.ts`, `AP/README.md`, `S/🧪️tests/{🐕️pet-walk,🐾️pet-cast}`.
Shared, by small anchored edits: taxonomy, schema catalog (generated), `bun.lock`, root `package.json`,
`.vscode/🧩️launch.seed.jsonc` (+ generated `launch.json`). Ticket: `📓️design.md`, this report, the three generators,
`render_species_preview.mjs`, `wp_j_private_stack.ts` (ports from the environment), and new tools `wp_p_stack.sh`,
`wp_p_gate.sh`, `wp_p_attention.mjs`, `wp_p_summary.mjs`, `wp_p_requests.mjs`, `wp_p_typecheck.tsconfig.json`,
`wp_p_before.config.ts`, `wp_p_pet_walk_before.ts`.

## 6. Open

1. **Rust twins** (M): everything of §2; until then parity fails for three cases. M's agent was still running when I
   last looked; the Rust schema twin already has `faced` and `activity`.
2. **Other tickets' work in flight** makes the quiz suite, the quiz and site typechecks and the quiz taxonomy scope
   red (§4). Nothing of it is in a pets file, but whoever gates next should expect it.
3. **`ERR_NO_BUFFER_SPACE`** in the dev topology while several sessions run stacks: a host condition that fails
   whichever spec's request it hits (the learner fixture counts every failed request). Running the two topologies one
   after the other, or fewer sessions at once, is the only remedy I see; the pets' share of the dev requests is 15 %.
4. **`🗣️both-languages`** compares two readings of a live overview (§3.4, failure 3) — its owner's.
5. **The phone test's failures of the morning** (old line 373) are not reproduced and not explained (§3.2).
6. **Desktop pages other than the overview** show pets on the footer line only: no room above the cards (§1 A5). A
   layout decision, not a pets rule.
7. **`🔒️dependencies.json`**: five entries with a pets manifest among their users wait for a deliberate
   `write-baseline` by whoever owns the baseline (§1 D10).
8. Recorded, not changed: with legal links configured a phone's footer wraps into two rows (switch above the links);
   the season flip `windowy – sunny` and signature scenes of the roster brief are not built; `🎪️stage-trace` remains a
   recorded no-oracle case whose traces come from the TypeScript subject (the Rust twin is the independent check).

## 7. Housekeeping

Started and stopped by me only: the private stacks on 6193/8923 and 6194/8924 (restarted for the final code, stopped at
the end; all four ports answer nothing now) and, earlier, one hung vitest of my own. Ports 6061, 6161, 6162, 8791,
8891, 8892 were never used. No git command that modifies anything, no cargo, no formatter; repository files were
changed with Write and Edit only. `bun install` ran only as `--frozen-lockfile --dry-run`. The repo MCP server was down
all session: no ticket was opened or closed. `EV` keeps the logs and the screenshot series this report cites; delete
it with `🗑️generated` when the ticket closes.
