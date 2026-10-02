# Audit — requirements coverage, accessibility, internationalisation, content

Ticket `2026/10/02/QUIZ-PETS`. Read-only audit; only this file was written. Read between 06:56 and 07:05 host time on 2026-10-02
(`date` in Git Bash). I re-checked the modification times of every pets file I cite at 07:04: none changed after my first
read (stage `🟦️.ts` 06:21, behaviour 06:06, terrain 05:30, survey 05:42, layer 05:43, quiz glue 05:22, preferences 05:23, i18n
04:47, ensemble 05:41, species 04:45–05:09, `AP/README.md` 06:24). The tuning and art agents had not touched them in that
window, so everything below describes the tree as of 07:04. Re-run the checks that matter after they land.

Abbreviations as in `📓️design.md`: `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`,
`Q` = `🧰️framework/🛍️products/❓️quiz`, `QR` = `Q/🎯️targets/⚛️react`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`,
`AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder. Stage = `P/🔨️modules/🎪️stage/🟦️.ts`,
behaviour = `P/🔨️modules/🧠️behavior/🟦️.ts`, layer = `PR/🔨️modules/🫧️layer/🟦️.tsx`, survey = `PR/🔨️modules/📡️survey/🟦️.ts`,
glue = `QR/🔨️modules/🐾️pets/🟦️.tsx`, prefs = `QR/🔨️modules/🎛️preferences/🟦️.tsx`, i18n = `QR/🔨️modules/🌐️i18n/🟦️.ts`.
Screenshots are in `TK/🗑️generated/wp-j/{walk,stories}/`; they are generated output and go away when the ticket's generated
folder is deleted (see N9).

Facts I computed myself with `bun -e` one-liners (read-only): 122 ids in the four quiz files (4 quizzes, 9 tasks, 109 items),
0 unresolved grounds of the 20 species; the 67 bonds and the 4 quiz casts of the ensemble equal `📓️explore-topic-pets.md`
§4.1 and §5.2 pair by pair; body-colour contrast ratios.

## 0. Verdict at a glance

| Owner's phrase | Verdict | One line |
|---|---|---|
| "Add pets to the ui" (quizzes) | met | one decorative layer mounted once, lazy, behind a preference |
| "Pets have a skeleton and are animated" | met | every species is a rig (4–15 bones, 5–23 parts, 9–12 clips), forward kinematics tested against numpy/gl-matrix |
| "By default they are slightly active" | met | default `calm`; measured idle 84–87 %, a fidget about once a minute per pet, an encounter about every two minutes |
| "When standing still they follow the cursor (with the eyes etc)" | eyes met, "etc" partly | pupils follow; wake-up near the pointer and a greeting on a click on the pet exist; no head or body turn; pupil travel is only 0.9–1.55 px |
| "small motions like blinking" | met | blink 2–6 s (double 1 in 6), idle breathing under every activity, two fidget clips per species |
| "they walk on top of ui elements" | **partly / not met as seen** | home: pets float about 26 px above the visible card edge; quiz pages and runs: only the window floor; phone: nothing on home |
| "they interact (like each other, small disputes)" | met in the model, not verifiable visually from the reports | greet / cuddle / squabble then sulk then mend, 67 authored bonds; no browser evidence of an encounter in any report |
| "always fitting to the topics" | met | casts per scene, build-time test that every cast member is grounded in its quiz; edge cases in §1 |
| the nine named pets | met | exact ids, right things, owner's order first |
| "etc" | met | eleven additions, each grounded in quiz items |

Top findings (details below): B1 pets do not stand on the UI they are meant to walk on; S1 the pause/stop/hide control is not on
the screens where the pets move; S2 the cursor-following is barely visible and has no "etc"; S3 no browser test or report
shows a pet standing on a card or an encounter; S4 no pets on a phone's home list; S6 the cast line on home names eleven pets
that can never show there; S7 `servy`'s English and German thing differ.

## 1. Requirement trace

### 1.1 Table

| Phrase | Where implemented (rule) | Proof (test / evidence) | Verdict |
|---|---|---|---|
| "Add pets to the ui" | `QR/🟦️.tsx:483` `<QuizPets />` beside the presence overlay; `QR/🟦️.tsx:530-532` provider around `Client`; `S/🟦️.ts:29` passes `pets: () => import(...)`; layer `PR/🔨️modules/🫧️layer/🟦️.tsx:367` | `S/🧪️tests/🐕️pet-walk/🟦️.ts:132-152` (pets within 10 s of the overview); 24 jsdom tests `Q/🧪️tests/🐾️pet-companions`; shots `walk/desktop-light-3b-home-returning.png` | met |
| "skeleton … animated" | rig + clips in each `AP/<dir>/🔣️.json`; `solveRig` forward kinematics; stage `frameOf` (`🎪️stage/🟦️.ts:1267-1299`) | Protocol v2 `🦴️rig-solving` (numpy, gl-matrix), `🎞️animation-sampling` (scipy), `🖌️pet-depiction` (gl-matrix); `S/🧪️tests/🐾️pet-cast/🟦️.ts:65-82` (ajv + owned validator); `stories/roster-light.png` shows all twenty rigs | met |
| "by default slightly active" | default `calm`: prefs `🎛️preferences/🟦️.tsx:69`; `MODE_LIMITS.calm` behaviour `🧠️behavior/🟦️.ts:43` (1 mover, idle 6…20 s, fidget 0.4, walk 0.2, hop 0.06, encounter gap 90 s); weights `:73-88` | stage unit test `🎪️stage/🧪️tests/🔬️unit/🟦️.ts:222`; `E` report §3 storyboard numbers (calm: idle 84–87 %, 4.0–4.9 fidgets/min, 2.0–2.7 walks/min, 0.4–0.6 encounters/min); `pet-walk:132-152` `expectMoving`, `data-pets="calm"` | met |
| "follow the cursor (with the eyes …)" | stage `gazeGoal` `🎪️stage/🟦️.ts:237-265`: pointer if it moved within 4 s (`POINTER_TICKS = 256`, `:47`) and the pet neither walks nor flies, else partner, else the nearest glance point (other learners' cursors, `glue:72-77`), else straight ahead; `lookOffset` + `springStep`; pupil offset = gaze × (radius − pupil − 0.75) (`:1297`). Beyond pupils: a sleeper wakes when the pointer comes within 1.5 × its height (`:219-227`, behaviour `:86`); `poked` greets towards the click (`:1158-1187`); mouth bends with mood; sulking pets look down (`:241`) | unit tests `:280-345`; Protocol v2 `👀️gaze-tracking` (numpy `norm`, `matrix_power`); `G` report probes (100 of 104 gaze samples at the pointer; pupils up-right with pointer top right, down-left bottom left; only seen clearly in 4× close-ups). No site spec asserts it (`pet-walk` moves the mouse only to prove stillness, `:226-228`) | eyes met; "etc" partly (S2) |
| "small motions like blinking" | blink schedule 128…384 ticks, one in six doubled (`:55-57`), `lidAt`; idle loop under every activity (`poseOf`, `:1235`); two `fidget` clips per species | unit tests `:360-386`; `🎞️animation-sampling`; `H1` engine-frame sheets | met |
| "walk on top of ui elements" | surfaces = `#quiz-main [data-card]` (`glue:64`; `data-card` is set once, in `QR/🔨️modules/🪟️chrome/🟦️.tsx:51`, so every `QuizCard`); survey turns top edges into surfaces, keep-outs cut them into perches (`survey:143-176`); stage walks, hops (`windowy`, `insuly` gait `hop`), floats, falls | stage unit tests `🎪️stage/🧪️tests/🔬️unit/🟦️.ts:402-530`; Protocol v2 `🏞️terrain-walking`, `🦘️hop-ballistics`, `📡️surface-survey`; **screenshots contradict the intent**: `walk/desktop-light-3b-home-returning.png` (pets float 30–100 px above card tops, the house hangs in empty space above Heating); `walk/desktop-light-4-quiz-page.png` and `walk/rehearsal-light-5-run.png` (everyone on the window floor, nobody on a card); `walk/desktop-dark-5-run.png` (all six piled in the bottom-left corner); `walk/phone-light-3b-home-returning.png` (none) | partly; see B1, S4, S5 |
| "interact: like each other / small disputes" | encounters: `pair` `🎪️stage/🟦️.ts:752-786`, `meet` `:816-838`; shares `encounterShares` (behaviour `:187-191`: friends ≥ 0.4 cuddle and never squabble, rivals ≤ −0.3 squabble and never cuddle, others mostly greet, a neutral pair squabbles 8 % of the time); a squabble always ends in `sulk` (backs turned, gaze down) and mends (+0.1); rapport drifts, effective affinity floor −0.6 (`:202`) | unit tests `:558-626` (friends, rivals, sulk, mending), `:626-682` (pairing, one at a time, gap and warm-up); Protocol v2 `🤝️bond-dynamics`, `🧠️behavior-choice`. **No report records a squabble or cuddle seen in a browser**; `E` says "judged by numbers only so far" (§6) | model met; visual not verifiable (S3) |
| "always fitting to the topics" | casts per scene (`AP/🔣️.json:97-107`); scene by `petScene` (`glue:43-47`); `castOf` (behaviour `:288-304`) only draws from the scene's cast; layer falls back to `home` for an unknown scene (`layer:59-61`, `glue:51-52`) | `S/🧪️tests/🐾️pet-cast/🟦️.ts:131-160` (a cast for `home` + every catalog quiz and nothing else; every member of a quiz cast has a ground in that quiz; core holds a bonded pair); `pet-walk:154-180` (strangers = []). My own re-check: all 109 ground lookups resolve, every quiz-cast member has 1–8 grounds in its quiz | met |

### 1.2 The questions asked

**"Follow the cursor (with the eyes etc)": is there anything beyond pupils?** Yes, but little, and none of it is the "etc" a
reader of the sentence expects (head or body turning towards the pointer):
- wake-up: a sleeping pet wakes when the pointer comes within 1.5 × its height, and a pet near the pointer never dozes off
  (`🎪️stage/🟦️.ts:219-227`, behaviour `:86`);
- click or tap on a pet in a non-interactive spot: it greets towards the point, gains mood (`:1158-1187`);
- partner and other learners' cursors are looked at when the pointer is idle (`:245-263`);
- mood bends the mouth and blinks continue.
There is no turn of `facing`, no lean, no head bone aimed at the pointer: `facing` changes only for goals, partners, pokes and a random facing at arrival
(`:393, 406, 417, 570, 834-835, 1011, 1182`). Pupil travel is `radius − pupil − 0.75` = 0.9–1.55 px per species (computed from the
twenty `face.eyes`; shady 0.9, cloudy 1.55), 0.72–1.24 px at the phone scale 0.8. That is smaller than one pupil diameter
(2.7–3.8 px). It is correct and tested, but on a 1440 px screen it is a detail the owner may miss.

**"Walk on top of ui elements": which elements on which screens?** By code: the top edge of every `QuizCard` section inside
`#quiz-main` (home grid cards, quiz page cards, run header and task cards, results cards, badges, leaderboard, preferences) and the
window floor. Not buttons, chips, the navbar, headings, list rows or drag items. By screenshots (06:34–06:54):

| Screen | What the pets stood on |
|---|---|
| introduction, identity (first visit) | floor only; they stay there until a cast change or a reload (`report-wp-j` §5, §6.4) |
| home, returning learner, 1440×900 | card edges, but about 26 px above the visible edge: the section starts with the 26 px title-chip row and the perch is cut beside the chip (`report-wp-j` §6.1; `walk/desktop-light-3b-home-returning.png`) |
| quiz page | floor |
| run | floor; with a tall task six pets pile into the strip left of the card at x 42–124 (`report-wp-j` §6.2; `walk/desktop-dark-5-run.png`) |
| results | not captured by any report |
| phone 375×812 | none on the home list; two at scale 0.8 on the footer of a quiz page; none in a run (`report-wp-j` §5, §6.3) |

**"Small disputes": what exactly plays?** Two pets within 12 mean widths on the same or a near perch, both idle, are paired on a
whole second (calm: first after 20 s, then at least 90 s apart, 4 % per second × mean sociability; lively 30 s, 12 %). They walk
(one or both) until their bodies are 2 px apart, face each other, and play their own `squabble` clip (`flare`, `bluster`, `huff`,
`glare`, `rumble`, `boil-over` …, 0.5–0.9 s loops) for 2–5 s with mood −0.8 (frowning mouth). Then both `sulk` for 3–6 s with
backs turned, eyes down, a droop clip (`dim`, `drizzle`, `cool-off` …), and the rapport mends by 0.1. No sound, no text, no
speech bubble, nothing that carries meaning (N5 of the prior-constraints list is respected). Never during a run, never in
`still`. Rivals are the pairs at or below −0.3 (18 bonds, strongest `chilly–radiatory`, `solary–cloudy` −0.7); the effective
affinity is floored at −0.6.

**"Always fitting to the topics": how enforced, can a pet appear where it has no ground?**
- Enforcement: build-time. `S/🧪️tests/🐾️pet-cast/🟦️.ts:131-155` requires casts for `home` and exactly the catalog's quizzes, and
  that every member of a quiz's cast has a ground `<quiz>` or `<quiz>/…` in that quiz; `:118-125` that every ground exists in
  the quiz files. Run-time: `castOf` can only return members of the scene's cast.
- Home cast is not topical on purpose (all twenty are in `home`); home also covers preferences, leaderboard, badges, learner
  page, introduction and identity.
- Rotation: only members of the same cast. Grounded but not cast in other quizzes (e.g. `roofy` is grounded in cooling, demand,
  physics but cast only in heating): never shown there. Safe.
- Unknown scene: falls back to the `home` cast, i.e. all twenty species including pets with no ground in that quiz. Reachable only
  if a catalog quiz has no cast; the test above prevents that for the shipped catalog. For a future quiz the fallback is wrong
  rather than empty (nit N1).
- Transition: when the scene changes, the old pets walk to the nearest perch end (only within 3 widths) and fade over 16 ticks,
  or fade where they stand (`🎪️stage/🟦️.ts:1077-1099`), while the new cast fades in. `report-wp-j` §5 measured about six
  seconds before the old cast was gone; the spec tolerates 45 s (`pet-walk:37`). For a few seconds a quiz page can show a
  home pet that is not grounded there.
- On home the eleven additions never show at all (see 2.4).

## 2. Roster and content

### 2.1 The nine, with the owner's ids

All ids, directory names and things match the owner's list; the order of `AP/🔣️.json:6-27` is the owner's order.

| id | Directory | thing en / de | name en / de | Verdict |
|---|---|---|---|---|
| `sunny` | `☀️sunny` | sun / Sonne | Sunny, the sun / Sunny, die Sonne | ok |
| `cloudy` | `☁️cloudy` | cloud / Wolke | Cloudy, the cloud / Cloudy, die Wolke | ok |
| `housy` | `🏠️housy` | house / Haus | Housy, the house / Housy, das Haus | ok |
| `solary` | `🔆️solary` | solar panel / Solarmodul | Solary, the solar panel / Solary, das Solarmodul | ok |
| `radiatory` | `♨️radiatory` | radiator / Heizkörper | Radiatory, the radiator / Radiatory, der Heizkörper | ok |
| `pumpy` | `🌀️pumpy` | heat pump / Wärmepumpe | Pumpy, the heat pump / Pumpy, die Wärmepumpe | ok |
| `windowy` | `🪟️windowy` | window / Fenster | Windowy, the window / Windowy, das Fenster | ok |
| `waly` | `🧱️waly` | wall / Wand | Waly, the wall / Waly, die Wand | ok |
| `battery` | `🔋️battery` | battery / Akku | Battery, the battery / Battery, der Akku | ok ("Akku" is what the German quiz texts use) |

The pattern is consistent over all twenty: `Nickname, the <thing>` / `Nickname, <Artikel> <Ding>`; `thing` has no article. Gender
and article of every German noun is right (die Sonne, die Wolke, das Haus, das Solarmodul, der Heizkörper, die Wärmepumpe, das
Fenster, die Wand, der Akku, die Windenergieanlage, der Heizkessel, das Dach, die Dämmung, der Sonnenschutz, das Lüftungsgerät,
die Kältemaschine, der Wasserkocher, das Teelicht, das Thermometer).

**S7 / content defect, `servy`**: `AP/🖥️servy/🔣️.json:4-5` English "server rack" but German "Rechenzentrum" (data centre). The two
languages name different things. The brief offered "das Rechenzentrum (der Serverschrank)"; pick one referent: either
`Servy, the server rack` / `Servy, der Serverschrank`, or `Servy, the data centre` / `Servy, das Rechenzentrum`. The drawn
figure is a rack (`stories/roster-light.png`), so the first. The same pairing sits in `AP/README.md:48` and the roster
table. `windy` ("Windenergieanlage") is correct but long next to "das Windrad"; no change needed.

### 2.2 The eleven additions

`windy boily roofy insuly shady venty chilly kettly flamy thermy servy` equal the brief; each is a recognisable object in
`stories/roster-light.png` (turbine, boiler with flame window, roof, pink wool, blind with sunglasses, ventilation unit, chiller
with frost, kettle, tea light, thermometer, server rack). Grounds: `windy` 3 items; `boily` 10; `roofy` 7; `insuly` 9;
`shady` 8; `venty` 7; `chilly` 6; `kettly` 4; `flamy` 3 (two items and the task `physics/powers`); `thermy` 6, of which three are topic or
task level (`heating`, `cooling`, `cooling/cooling-load-and-demand`) and three are items (`heating/…/sfh-2000s`,
`cooling/…/attic-flat`, `physics/energies/boil-water`); `servy` exactly one item
(`cooling/cooling-load-and-demand/data-centre`). Weakest grounding as the brief predicted: `cloudy` (topic and task level plus
`sunlight-square-metre` and `pv-annual-yield`), `thermy`, `servy`. All pass the test; I would keep them.

### 2.3 Grounds, palette, temperament, repertoire, gait

- **Grounds**: 122 ids collected from the four quiz files; every ground of every species resolves (0 unresolved). Every
  species is in at least one quiz cast and in `home`. The README claim "every species in the cast of a quiz carries at least one
  ground in that quiz" holds (I recomputed it).
- **Palette** (`palette.body/accent/detail`): all 19 species equal `📓️explore-topic-pets.md` §6.3. `thermy` swaps roles
  (accent `#ff344f` = liquid, detail `#e8e2c8` = glass; the brief had accent glass, detail `#7b827d`, and a gradient the schema
  cannot express): acceptable. Contrast of the body (computed): ≥ 3.05:1 against both `#f7f3e3` and `#001117` for 19 of 20;
  `sunny` 1.34:1 on cream (relies on the 2 px ink outline, as the brief says). The ink outline against the glass panels is
  11.42:1 (light) and 13.12:1 (dark). Pets are decoration, so 1.4.11 does not apply, but the numbers hold.
- **Temperament**: all traits in [0.2, 0.9] (min `boily` curiosity 0.2, `shady` energy 0.25, `waly` curiosity 0.25; max 0.9).
  Sensible differences: `windy`, `battery` energetic; `shady`, `insuly`, `waly` sluggish; `servy`, `windowy`, `thermy` curious.
- **Size**: 28×45 (`flamy`) to 58×40 (`cloudy`); 19 of 20 are 40–56 px tall as design §9 demands; `venty` is 36 px tall
  (`AP/🌬️venty/🔣️.json:15`), the brief's own S size (nit N2).
- **Repertoire**: all twenty map `idle`, `fidget` (two clips each), `walk`, `land`, `sleep`, `greet`, `cuddle`, `squabble`,
  `sulk`. Missing clips per activity:

  | Activity | Species without a clip | Effect |
  |---|---|---|
  | `hop` | 15 of 20 (all but `windy`, `flamy`, `kettly` with `leap`, and `insuly`, `windowy` whose gait is `hop`) | `clipsOf` falls back to the idle loop (`🎪️stage/🟦️.ts:179-187`): a walker that hops between perches flies its arc while breathing. Floaters glide, so `sunny`, `cloudy`, `venty` need none |
  | `fall` | 17 of 20 (all but `windy`, `flamy`, `kettly` with `tumble`) | idle loop while falling after a perch vanished |

  Not a defect; add `leap`/`tumble` to the other species when the eye asks for it. Gait values: walkers 20–46 px/s, hoppers
  22 and 40, floaters 16–28; stride per clip cycle 10–31 px against 28–58 px widths. Plausible; foot sliding cannot be judged
  from the JSON.
- **Gait vs the brief's character sheets**: `flamy` and `kettly` walk (the sheets said hop on a base); `windowy`, `insuly` hop;
  `sunny`, `cloudy`, `venty` float with the sheet's hover heights 10, 14, 6. Fine.

### 2.4 Bonds and casts against the brief

- **Bonds**: 67 in the ensemble and 67 in the brief; every pair, sign and value identical (0 missing, 0 extra, 0 different).
  47 positive, 20 negative, 18 at or below −0.3, 35 at or above 0.4. No self-bond, no duplicate, every species has 4–11 bonds
  (`battery`, `windy`, `flamy`, `servy` the fewest). No obviously wrong pair; `housy` has no rival as the brief wanted.
  Not carried over (by design): the modulation rules of brief §4.4 (season flip for `windowy–sunny`, learner events) and the
  vignettes of §4.5; nit N3, and the absence of learner events is what the owner's cognitive-load rule wants.
- **Casts**: `physics`, `heating`, `cooling`, `demand` equal §5.2 exactly (core 3, rotation 4); `home` core = the nine in the
  owner's order, rotation = the eleven.
- **Home really shows only the nine, and at most six of them.** `castOf` (behaviour `:288-304`): a core that does not fit the
  capacity takes turns itself and "the rotation stays away". Capacity is 6 from 1024 px, 4 from 768, 2 below (`layer:49-51`), so
  at home the eleven additions never appear at any viewport (they need capacity ≥ 10). The brief's "nine seeds plus at most one
  visitor" (§5.4) is not built. The `pet-cast` test title at `:136` ("lets every other species take turns there") describes the
  data, not what the layer does. Each of the eleven is on screen on at least one quiz page, so nothing is orphaned.
  - **S6**: because of that, the preferences line on home, "At home here: …" / "Hier zu Hause: …", lists all twenty names
    (`glue:51-58` takes core + rotation of the scene) although eleven of them cannot appear there. Make it list the core when
    the core does not fit, or word it "These pets live here".

## 3. Accessibility (WCAG 2.2)

| Criterion | What the code does | Verdict |
|---|---|---|
| 2.2.2 Pause, Stop, Hide (A) | Pets move automatically and last longer than 5 s next to other content; the mechanism is the preference `Pets: Off / Still / Calm / Lively` (`prefs:153-159`, `Segments` = `role="group"` with `aria-label`, native `<button aria-pressed>`, so keyboard operable and announced, `QR/🔨️modules/🪟️chrome/🟦️.tsx:163-181`), persisted (below), and `prefers-reduced-motion` forces `still` (`glue:31-34, 120`). **Where it is reachable**: the first-visit card (shown while `learner === undefined`, i.e. introduction and identity, `QR/🟦️.tsx:319`) and the preferences page (overview card "Settings" → "Open"). It is not on quiz pages, runs, results, leaderboard or badges, where the pets are: the reader is two navigations away (Overview, then Settings). The existing "Animate task icons" switch has the same limitation | **S1 should-fix**. Add a one-control toggle in the always-visible header (next to the language switch) or the footer next to "What is stored", operating the same preference |
| 2.3.3 Animation from interactions (AAA) | Pointer-driven motion (gaze, wake, poke greeting) is dropped in `still`: `layer:264` returns before queuing, stage `poke` returns for `still` (`:1159`), `frameOf` shows rest pose, open eyes, centred pupils (`:1289-1299`) | met |
| 2.4.11 Focus Not Obscured (min.) | The focused element is a keep-out grown by 8 px (`survey:40, 170-174`), also a focused card itself; the ring is ≤ 4 px beyond the box (`QR/🎨️.css:245-248, 504-506`); `focusin`/`focusout` mark the survey stale and wake the pacer (`survey:225-226`), so pets leave within the next survey (≤ 8 ticks ≈ 125 ms) or are re-homed at once in `still`. The layer is `pointer-events: none`, so a covering pet never blocks input. Design M12 asked for ≥ 24 px; 8 px is what exists. Z-index 35 is below the skip link (`z-50`), the dialog backdrop (`z-50`) and the presence layer (`z-40`, `👥️presence/🟦️.tsx:1010`) | met (nit N4: 125 ms window, margin 8 px not 24) |
| 1.4.11 / text under pets | Pets are opaque (`opacity` 1 except the 16-tick fades) and sit above the page (`z-index` 35), so overlap would hide text, not lower its contrast. They can overlap text only when the perch logic fails: keep-outs cover controls, `p li h1–h6 label figcaption td th legend dt dd blockquote pre` and `[data-quiz-item] [data-quiz-drop]` (`survey:21-28`, `glue:68`), re-surveyed every 8 ticks while moving but only about once a second while standing (`G` §3.4), and not when a bare `div`/`span` carries the text (e.g. the run progress `QR/🔨️modules/▶️run/🟦️.tsx:130-133`). Evidence: `G` and `J` sampled 100 + 180 frames with 0 overlaps of a pet with text or a control; the pile-up case (B1/S5) puts six pets into one narrow strip | met in practice, no hard guarantee (N5) |
| Reduced motion | `prefers-reduced-motion: reduce` → mode `still` for any choice but `off`, also at run time (`useMediaQuery`). Nothing moves: no blink, breathing, gaze, walk, fidget, encounter, fade (they are simply there), rotation. What still changes position: pets ride a scrolling card or a rearranged layout (`carry`, `:905-941`), which is the page moving, not animation. Test: `pet-walk:219-233` (three identical drawings a second apart, also while the mouse moves; moving again once the media feature is off) | met |
| Forced colours / print | `@media (forced-colors: active)` and `print` hide the layer (`PR/🎨️.css:72-82`), and the layer renders empty and starts no show (`layer:335-337, 356`); it watches a runtime change | met (nit N6: the preferences still offer pets and list names) |
| Screen readers | `<div class="pet-layer" aria-hidden="true">` (`layer:367`), each `<svg focusable="false">` (`depiction:92`), no `role`, no `id`, no `tabindex`, no live region, no console output, last child of `.quiz-app` so no tab-order or reading-order change. Test: `pet-walk:135-143` (computed `pointer-events: none`, no focusable/role/id/style/script inside); `🫥️decorative-layer` (aria-query: no role, `isInaccessible`) | met |
| Touch | No hover dependency. A touch `pointerdown` sets the "look here" point for 4 s and, on a pet outside any interactive element, greets (`survey:266-273`); a finger leaving does not clear the point (`:274-276`); `pointer-events: none` so a tap always reaches the page; no gesture. A pet cannot be operated by keyboard, nor needs to be | met |
| Cognitive load during a run | `quiet = (step.screen === "run")` (`glue:139`) → stage `hushed`: no walks, hops, fidgets, no new encounters (`behaviour:73-88`, stage `:739`), sleep ×3; the cast and the rotation freeze (`layer:198-201`); `pet-walk:171-177` proves nobody changes place for three seconds. Pets get no answer, score, timer or result: the provider receives `Pick<QuizState, "step" \| "runs" \| "catalog">` and reads only `runs[id].quiz` (`glue:43-44, 114`); the only live input is other learners' cursor positions (already on screen). So pets do not react to answers or scores | met, with three leftovers below |

Leftovers on cognitive load (S9, N7): (1) **results are not quiet** (`glue:139` only the `run` screen): the pets walk, fidget and
may squabble while a learner reads a score. Add `results` to the quiet condition. (2) In a run the pets still breathe (32 fps
loops), blink and follow the pointer; with the pointer in constant use during a task the six figures at the bottom of the window
keep their pupils moving. Acceptable and requested ("when standing still they follow the cursor"), but consider dropping gaze
under `quiet`. (3) A click on a pet during a run still triggers a greeting (`poke` ignores `quiet`, `:1158-1187`), against the
doc "quiet actors only idle and sleep".

## 4. Internationalisation

| Item | Finding |
|---|---|
| New quiz strings | `quiz.preferences.pets`, `petsOff`, `petsStill`, `petsCalm`, `petsLively`, `petsCast`, and the changed `summary`: EN `i18n:101-109`, DE `:443-451`. Both bundles complete, three-segment keys, used literally (`PET_CHOICE_LABELS` `prefs:44` is a typed literal map; `text("quiz.preferences.petsCast", { names })` `prefs:158`). No default language is introduced: nothing in the pets code picks a language; names go through `localized(species.name, locale)` (`glue:56`) |
| German wording | informal register is not even exercised (no addressing). "Tierchen", "Aus / Reglos / Ruhig / Lebhaft", "Hier zu Hause: {{names}}". "Reglos" was chosen over "Still" because `🗣️both-languages` rejects a button worded the same in both languages and "Still"/"Ruhig" are near-synonyms (I report §5.5): good |
| One German word for "pets" | UI, ensemble title and e2e constant use **"Tierchen"** (`i18n:446`, `AP/🔣️.json:5`, `pet-walk:31`). The stories gallery (dev page) says "Tiergeschichten" and "Tiere laufen auf der Oberkante einer Karte" (`PR/📖️stories/🟦️.tsx:68, 104`; `🌐️.html:6`): inconsistent (N8). The READMEs are English. Note that "Tierchen" also means microbes in German ("Aufgusstierchen"); "Haustiere" or "Begleiter" would avoid it, but consistency matters more than the choice |
| Accessible names of species | They are data (`Species.name: Text`, required in both languages by the schema). They are used in exactly one place of the UI: the cast line under the Pets row, localised and updated on a language switch (`glue:140`, memo depends on `locale`). `Species.thing` is used only by the stories page (`PR/📖️stories/🟦️.tsx:336`). The pets themselves are `aria-hidden`, so "accessible name" in `AP/README.md:50` is a misnomer: these are display names (N10) |
| Mixed-language names | The nickname is English in both languages ("Battery, der Akku", "Windy, die Windenergieanlage"). By design (brief §7). The names sit in a `<p>` without a `lang` for the English nicknames; harmless for a screen reader reading German text |
| Content defects | `servy` en/de mismatch (S7); the cast line on home (S6) |
| Minor wording | English "Still" can be read as "yet"; the group label "Pets" does not say it sets how lively they are (N11). The separator " · " is read as "middle dot" by some readers |

## 5. Customisation and state lanes

| Question | Answer |
|---|---|
| What can the learner choose? | one thing: liveliness `off / still / calm / lively` (`prefs:44, 153-159`). Not which pets, how many, how big; text size does not scale them (px, scale 0.8 below 768 px, `layer:53-56`); `Animate task icons` is a separate switch. A host can recolour via `--pet-body/-accent/-detail/-ink/-paper/-pupil` and pass `capacity`, `scale`, `zIndex`, `surfaces`, `keepouts`, but the quiz exposes none of it |
| Where stored | persisted local-only: slice `preferences` of `semio.quiz.<tenant>.*` (`QR/🟦️.tsx:525-526`, `writePreferences` `prefs:74-76`), no personal data |
| Cross-tab sync | yes: `store.watch` re-reads the slice on a `storage` event (`QR/🟦️.tsx:522`, `💾️persistence/🟦️.ts:69`), `data-pets` follows (`:459`); the provider reloads or unmounts |
| Default | `calm` (`prefs:69`), also when the stored value is missing or unknown (`PET_CHOICES.some(...)`), so a stored `"bogus"` silently becomes `calm` |
| Reduced motion | overrides every choice but `off`; `data-pets` still says the choice, not the effective mode (`glue:31-34`); the UI never says that the system setting is why `Lively` does nothing (S8) |
| Lanes | stage ephemeral local-only (per device, random seed per mount, `layer:357`); other learners' cursors ephemeral shared read-only via `peerGlances` (`glue:72-77`); nothing travels over the wire; nothing persisted shared. Matches design §2.3 and `P/README.md:125-132` |
| Load | nothing fetched when `off` or when a site passes no source; a failed fetch is silent and retried when pets are chosen again (`glue:122-137`) |

## 6. Docs spot check (five concrete claims each)

| Document | Claim | Verdict |
|---|---|---|
| `P/README.md` | events are `ticked, pointed, unpointed, glanced, surveyed, summoned, tuned, hushed, poked` (`:112-114`) | true, `🎪️stage/🟦️.ts:1190-1201` |
| | `frameOf` reports a rate of 64, 32, 16 or 0 (`:115`) | true, tests `:991-1008` |
| | the sample menagerie has `blobby`, `hoppy`, `floaty`, three bonds, scenes `home` and `meadow` (`:136-141`) | true (computed from the fixture) |
| | root scripts `test:pets`, `test:pets:react`, `test:pets:rs`, `typecheck:pets:react`, `dev:pets:stories` (`:44-48`) | true, `package.json:228-232` |
| | "the same seed … bit for bit in TypeScript and Rust" (`:7-8`) | Rust twins exist (`P/🔨️modules/🎪️stage/🦀️.rs` 1 649 lines, 06:54, still being written); parity not verifiable from files |
| | no section on the React target: layer rules (aria-hidden, forced colours, z-index 35, capacities 6/4/2, survey margins) live only in docstrings | gap (N12) |
| `Q/README.md` `### Pets` (`:312-330`) | `effectivePetMode`: reduced motion gives motionless pets whatever was chosen | true, `glue:31-34` |
| | `data-pets` on `.quiz-app` | true, `QR/🟦️.tsx:459` |
| | scene = quiz id for page, run, results, else `home` | true, `glue:43-47` |
| | "stand on the top edges of the cards of the screen" | code true (`glue:64`), visible outcome not (B1) |
| | "while a run is on screen the pets rest" | partly: they stop walking, fidgeting and meeting; they still breathe, blink, follow the pointer, and a click greets |
| `AP/README.md` | capacities 6 from 1024, 4 from 768, 2 below | true, `layer:49-51` |
| | "the home screen shows the owner's nine" | true only as "up to six of them at a time, never the other eleven" (2.4) |
| | all 67 bonds listed with reasons | true, equal to the ensemble and the brief |
| | every quiz-cast member has a ground in its quiz; each core holds a bonded pair | true (recomputed; `pet-cast:150-160`) |
| | gallery on 6074, `architecture-pets-stories` | true, `.claude/launch.json:596-607` (the design text still says 6072) |
| | "the accessible name of a pet is …" | misleading (N10) |
| `S/README.md` `### Pets` (`:111-132`) | `🟦️.ts` passes `pets: () => import("../🐾️pets/🟦️.ts")` | true, `S/🟦️.ts:29` |
| | three lazy script chunks and one lazy stylesheet, entry budget not charged | true per `J` §4 (entry 215 896 B gzip of 260 000; chunks 35 401, 17 399, 167 B, css 370 B); not re-measured by me |
| | rest during a run, never take a click, keep clear of text and controls | true as far as selectors reach (3, N5) |
| | CSP needs nothing new | verified by `I` and `J` on the rehearsal topology |

## 7. Findings, by severity

### Blocker

**B1. Pets do not walk on top of the UI they were asked to walk on.** (a) Home: the perch is cut beside the 26 px title-chip row
of every card, so pets hover about 26 px above the visible edge (`QUIZ_PET_SURFACES = "#quiz-main [data-card]"` at
`glue:64`; section box vs body box in `report-wp-j` §6.1; `walk/desktop-light-3b-home-returning.png`). (b) Quiz pages and runs: the
first text line of every card sits directly under its top edge, its keep-out (4 px margin) blocks the edge, so the floor is the
only perch (`walk/desktop-light-4-quiz-page.png`, `walk/rehearsal-light-5-run.png`). The owner's "walk on top of ui elements"
is then visible only as "walk along the footer". Suggest: make the card's visible edge the surface (the body surface
`[data-slot="window-chrome-body-surface"]`) and let a surface's own first text line not count as a keep-out for the edge it
stands on (the survey already exempts the surface and its ancestors, `survey:164-169`; extend to the first block inside), or
treat the title chip as part of the silhouette; also offer chips and the progress bar as secondary surfaces. This is owned
by the tuning agent (survey/layer/quiz module); re-judge after it lands.

### Should-fix

- **S1. Pause/stop/hide is not where the motion is** (2.2.2): see §3. A header or footer toggle for the same preference, keyboard
  operable, is the smallest fix.
- **S2. Cursor-following is subtle and has no "etc".** Pupil travel 0.9–1.55 px; no head or body turn. Suggest eyes of radius
  4.5–5.5 with a pupil travel of 2–2.5 px, and a small lean or a mirror of `facing` towards the pointer for pets that stand
  still, shaped as an additive rig channel so it stays inside `still` = nothing moves.
- **S3. No evidence of the two most visible behaviours in a browser.** No spec asserts that a pet's feet stand on a card top
  (suggest: in `pet-walk`, read the `translate()` of each pet and compare with the card rect, ±2 px) or that pupils move
  with the pointer (move the mouse, compare two pupil `cx/cy`); no report records an encounter. The layer takes a random seed
  per mount (`layer:357`) and the quiz glue passes none; a test seam (`?pets-seed=` read by the site build, or an optional
  `seed` in `QuizPetsSource`) would let an e2e force an encounter.
- **S4. Phone: no pets on the home list** because three clipped keep-outs block the floor (`report-wp-j` §6.3, survey clips
  surfaces by scrolling ancestors but not keep-outs). Mobile is priority two in `AGENTS.md`.
- **S5. Pile-up in a tall run**: six actors clamped into one strip stand in each other (`walk/desktop-dark-5-run.png`,
  `report-wp-j` §6.2). Space actors when a perch shrinks, or let surplus actors leave.
- **S6. Cast line on home names eleven pets that never show** (2.4).
- **S7. `servy` thing en/de differ** (2.1).
- **S8. Reduced motion is silent**: a learner on `Lively` under `prefers-reduced-motion` sees still pets and no reason. Add a
  line under the row, e.g. "Your system asks for less motion: the pets stay still." (EN/DE, one key).
- **S9. Results are not a quiet time** (§3 leftovers).
- **S10. First-time learner sees pets only on the floor** until a reload or a cast change (`report-wp-j` §6.4). Re-seat on
  the first home survey.

### Nit

- N1. An unknown scene falls back to the whole `home` cast, which includes pets with no ground there; for a future quiz an empty
  or core-only fallback would be safer (`layer:59-61`, `glue:51-52`).
- N2. `venty` is 36 px tall, design says 40…56 (`AP/🌬️venty/🔣️.json:15`).
- N3. Brief features not built: season flip `windowy–sunny`, "one visitor" at home, signature scenes; fine, but the README says
  nothing about them.
- N4. Focus keep-out margin is 8 px, the prior-constraints list said 24 px (M12); with the 125 ms survey latency a pet can sit
  under a freshly focused control for a few frames.
- N5. Keep-out selectors miss text in a bare `div`/`span`/`output`/`progress`; a visual sweep or `data-pet-keepout` on the few
  cases (run progress) would close it.
- N6. Under forced colours the preferences still offer pets and list their names.
- N7. A click on a pet greets even while quiet.
- N8. Dev gallery uses "Tiergeschichten" and "Tiere" beside "Tierchen" in the product.
- N9. The proof screenshots live in `TK/🗑️generated/`, which `AGENTS.md` says to delete when the ticket closes. Keep five of
  them (`desktop-light-3b-home-returning`, `desktop-light-4-quiz-page`, `rehearsal-light-5-run`, `desktop-dark-5-run`,
  `roster-light`) as inputs or reports in the ticket, otherwise B1/S5 lose their evidence.
- N10. "Accessible name" wording in `AP/README.md:50` and `design.md`: the pets are decorative; call it the display name.
- N11. Group label "Pets" does not say what it sets ("Pets: how lively").
- N12. No README for `PR` (layer rules and defaults are only in docstrings).

## 8. Gaps: asked and not built, and built but perhaps not wanted

**Asked, not built or not seen working**
- Walking on top of the interface elements as the owner pictures it (B1, S4, S5, S10).
- "etc" for the cursor: nothing turns towards the pointer except pupils (S2).
- A visible squabble in a real browser (S3). The model does it; nobody has watched it.
- Pets "always fitting" on the home screen: only the nine seeds ever appear there (2.4); the eleven additions live only on quiz pages.

**Built, owner may not want (reasoning)**
- Pets start moving on first visit before the learner has seen the introduction, with the control one card away. The owner asked
  for "by default slightly active", so this is right; just be aware of the 2.2.2 argument (S1).
- Six pets on a 1440 px desktop is the most; with `lively` and two movers it is busier than "slightly active". The owner can
  choose.
- The `still` and `off` modes and the lazy-load machinery are not in the request but are required by `AGENTS.md`
  (accessible, customizable) and WCAG 2.2.2; keep.
- Eleven extra pets, 67 bonds, rapport, needs, sleep: considerably more than "etc", but each is grounded and cheap at run
  time (4–8 µs per tick per `E`). The owner asked for a world, not a few figures.
- The deterministic, two-language (TypeScript and Rust) core is mandated by `AGENTS.md`, not by the owner; invisible to the
  learner.
