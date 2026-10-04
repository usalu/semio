# Audit 2 — requirement coverage, accessibility, internationalisation, content (second round)

Ticket `2026/10/02/QUIZ-PETS`. Read-only audit; only this file was written. Read between 18:36 and 18:52 host time on
2026-10-03 (`date` in Git Bash). F1 was working in the ticket folder while I read (`f1_*.sh`, `f1_survey*.mjs`,
`stage_fuzz.ts` changed 18:42–18:50); a `find -newermt` at 18:50 showed **no product, menagerie or site-test file newer than
18:10** (README of the pets product 18:10, layer 18:08, locomotion 18:00, pet-walk 18:03, stage unit suite 17:39, choice
17:39, preferences/glue/i18n older). Everything below describes the tree as of 18:50. I ran nothing (no build, test, server);
every number I did not compute myself is marked "reported" and names the report it comes from. Numbers marked "computed" are
read-only `python` one-liners over the committed JSON.

Abbreviations as in `📓️design-v2.md`: `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`,
`Q` = `🧰️framework/🛍️products/❓️quiz`, `QR` = `Q/🎯️targets/⚛️react`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`,
`AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder. `M` = `P/🔨️modules`. Test names in the traces are
shortened; the file:line is the `it(...)`/`test(...)` line.

---

## 0. Verdict at a glance

| Owner's phrase (second request) | Verdict | One line |
|---|---|---|
| "pet system must be way more sophisticated" | met in the model, partly as seen | twenty species with states, tricks, purr, emitters, gear, chemistry, moods; much of it needs a gesture the learner is never told about (S9) |
| "start to climb up ui elements" | met | walls: stage suites, traces, fuzz **and** the real browser (pusher trail `push/wall → tumble/air`, B5 probes on `#heating`) |
| "use grappling gun" | **partly — not verifiable on a real page** | stage suites, Protocol `🎣️grapple-reach`, traces, gallery; no browser run ever saw a rope; gear e2e is still `fixme` (B1) |
| "use parachutes" | met | unit + Protocol `🪂️parachute-descent` + e2e `pet-walk:709` + 0 hard landings with a parachute in 12 M fuzz actor-ticks |
| "use a ladder" | **partly — not verifiable on a real page** | same as the rope; B5 says ladders (≤ 3.6 heights) cannot reach on the real cards (B1) |
| "richer interaction, draggable in free space, hang, fall, parachute when too high" | met | `hold/letGo/decisionOf`, `pet-walk:709`, stage suite 2214–2361; "too high" = predicted impact above `HARD_LANDING` 600 px/s |
| "dont collide with each other" | met (strong evidence), with a blind spot on the escape hatch | invariant in every stage test, 36 M fuzz actor-ticks 0 overlaps, 13 reported real-browser lively minutes 0 (C4 ×6, B5 ×6, P1 ×1); poofs counted only partly (S12) |
| "different tricks … circling around the sun" | met, thing-specific for all twenty | 5–7 tricks each, 3–4 circling rungs; but several states are reachable by circling only (S2) |
| "states (strong shining)" / "moods" | met | 3–4 states per species (sunny `blazing`/Gleißend), nine moods with intensity on the face |
| "interaction of pets with each other depends on mood and state" | met in data and unit tests, **thin as seen** | 125 reactions (104 read a state, 13 a mood); in the default `calm` mode 0–4 reactions fire in 15 simulated minutes (S1) |
| "left clicking … hello, then tricks or purr" | met | `heat` bucket: hello → click tricks in order → purr → enough (shrug, 8 s glance); e2e `pet-walk:675` 6 of 6 runs |
| "alter ui elements … house pet pushes a question out of the stack" | met | lifted copies of marked task rows/items; housy can push `heating/heating-load-and-demand` and `demand/*` rows; e2e `pet-walk:1111` |
| "never change the document … thrown out, element returns" | met in intent; DOM-level caveat noted | original keeps place and content, gets `opacity:0 !important` through CSSOM and is restored exactly; restore within 2–80 ms of hover/focus (reported) |

Top findings (details in §7): **B1** the gear spec is still `fixme` with a stale reason and nothing proves ladder or rope on a
real page; **S1** chemistry hardly shows in the default mode; **S2** circling is a path gesture and the only way to many
states, so "every gesture trick is also reachable by clicks" (design §14.5, quiz README) is false in the data; **S3** one
control under the ring kills a circle; **S4** the lifted copy's swept box is checked against nothing (controls, focus);
**S5** mischief in runs is on by default; **S6** the Rust stage twin is not ported; **S7** several README/design claims are
stale.

---

## 1. Requirement trace

### 1.1 Table (one row per phrase)

| Phrase | Where implemented | Proof | Verdict |
|---|---|---|---|
| "climb up ui elements" | survey `walls` (`PR/🔨️modules/📡️survey`), `M/🏞️terrain` `wallsOf`, `M/🧗️climbing` (grip, `wallPath`, `chainOf`, lunge across 30 px gaps), `M/🚶️locomotion/🟦️.ts:800-1450` Gear region (`outingsTo` 1154, `setOut` 1191, `embark` 1216, `travel` 1252, `rest` 1324), `M/🎯️choice/🟦️.ts:31,117` (`CLIMB_SHARE` 0.4 in lively), `VENTURE_SHARES` calm 0.25 / lively 0.6 (`:30,146`) | stage unit `🎪️stage/🧪️tests/🔬️unit/🟦️.ts:2435` (climbs, rests, climbs off), `:2464` (wall that a survey moves), `:2530` (gap crossing), `:2577` (lively page whose cards have walls, bodies kept apart), `:2591` (nothing in quiet/still/floaters); Protocol `🧗️wall-climbing` 13/13 (reported B4); traces `gutter-life`, `gap-crossing`, `scrolled-climber`; **browser:** `pet-walk:1111` records `radiatory/insuly push/wall → tumble/air` in 6 of 6 reported runs; B5 screenshots (`#heating`, light and dark): a pet clinging to the side of "Tasks in this quiz" | **met** |
| "use grappling gun" | `M/🧗️climbing` `shotFor`, `hookStep`, `zipStep`, `swayStep`; locomotion `ropeOuting`; activities `aim`, `reel`, `shrug`; projection `toolsOf` (gun/rope/hook); gear: housy, battery, boily, kettly, thermy, servy | stage unit `:2510` (miss flies past and is shrugged off, hit reels up); Protocol `🎣️grapple-reach` 8/8; traces `grappling-rope`, `missed-hook`; fuzz: rope ticks 12 (sample, walls) / 155–195 (plain); gallery (C2) draws gun, rope, hook per species; **no browser run saw `rope`**: e2e `pet-walk:1146` is `test.fixme` | **partly — not verifiable on a real page** (B1) |
| "use parachutes" | `M/🪢️swing` `chuteOpens` (`:280`: `vy ≥ 240`, headroom `≥ 56`, impact `> HARD_LANDING 600`), `M/🚶️locomotion` `decisionOf` (`:315`), `wayDown` braking (`BRACES` `:59`); eleven species own one | stage unit `:2257`, `:2289` (with: floats, lands idle; without: lands hard); swing unit `:404` (boundary); Protocol `🪂️parachute-descent` 9/9; fuzz "hard landings with a parachute" 0 in 12 M (B1), 12.2 M (B4), 12 M (B5) actor-ticks; **browser** `pet-walk:709` `hang/hand → glide/chute → idle/perch`, 6 of 6 runs | **met** |
| "use a ladder" | `M/🧗️climbing` `ladderFor`, `LadderStand`, `spillOf`; locomotion `ladderOuting`/`ladderWays`; `carry` activity; eight species own one | stage unit `:2477` (raises its own, climbs, steps off, ladder stands then goes), `:2495` (topples, never lands hard with a parachute); Protocol `🪜️ladder-geometry` 5/5; traces `ladder-up`, `toppled-ladder`; fuzz architecture+walls: **2 raised, 1 toppled in 3.96 M actor-ticks** (B4 §4); **no browser run saw `ladder`**; B5 §3: "Ladders (≤ 3.6 heights) and ropes (≤ 3.2) do not reach" on the real cards | **partly — not verifiable on a real page** (B1) |
| "interaction … richer, draggable in the free space" | `PR/🔨️modules/🤏️grasp` (window capture listeners), `M/👆️gesture` `pressStep`, `M/👀️attention` `grasp`/`handle`, locomotion `lift` 701 / `hold` 726 / `letGo` 748 / `giveBack` 754 / `toss` 793 | stage unit `:2214` (pick-up beyond the slop only), `:2228` (dangles on a spring, 64 ticks/s), `:2243` (pushed out of bodies, never through), `:2277` (Escape glides back); `P/🧪️tests/🤏️pet-handling` 26 tests (user-event as oracle); Protocol `🪢️swing-dynamics` 13/13; **browser** `pet-walk:709` | **met** |
| "(then they hang and fall afterwards — when too high … parachute to land soft)" | `hold` → `hang/hand`; release → `tumble/air` with the pointer's velocity (7 samples); parachute only where impact would be hard (rule above) | `:2257`, `:2289`; `pet-walk:709` releases 420 px above the lowest edge; negative case (short drop, no chute) = pure boundary test only (`swing:404`), no stage-level or browser test with a drop < 56 px | **met** (negative case thin) |
| "Make sure that they dont collide with each other" | `M/🚧️clearance` (`overlaps` 153, `claimClear` 295, `seatOf` 414, `mustPoof` 553), `M/📏️spacing` `extentOf`, locomotion `poof` 106 / `vanish` 99, `scoot`, heads (`headUnder`) | every stage test advances through `heldApart` (`:111`); Protocol `🚧️clearance-proof` 11/11; fuzz **0 overlap ticks**: B1 11.98 M, B4 12.2 M, B5 12.06 M actor-ticks (reported); browser `pet-walk:748` (60 s, ≥ 3 throws, > 300 frames, bodies ≥ 8 px apart, tolerance 0.5 px): 0 overlaps in all 13 reported runs (525–874 frames each; C4 ×6, B5 ×6, P1 ×1) | **met** (see 1.3 for the hatch) |
| "tricks … circling around the sun something happens … based on what the pet is" | species `tricks` (cues `click circle countercircle stroke shake whim show`), `M/💗️feeling/🟦️.ts:480-549` (`rungsOf`, `stepRung`, `stateAfterTrick`, `tricksFor`, `clickTrick`), `M/👀️attention` hovers → `cued` | stage unit `:2051` (circling asks a trick and climbs a rung), `:2084` (stroke purrs); `pet-cast:246` (circling one way or the other changes the resting state, **per species**); browser `pet-walk:785` (`housy circle: cosy → trick → wrapped`, `thermy: mild → trick → hot`, 5 + runs) | **met**; thing-specific for all 20 (§2) |
| "different states (e.g. strong shining)" | species `states` with `tint`, overlay `clip`, `emitter`, `lasts`/`then`; `STATE_BLEND` 16 (`🎥️projection:33`) | `pet-cast:231,239` (every state names existing clips/emitters, reachable and returns); stage unit `:2073`; gallery C2 screenshots (sunny blazing/dim/sunset look different) | **met** (3–4 states each, §2) |
| "different moods" | nine moods + intensity (`M/💗️feeling`), `faceOf` (mouth, lid, slant, drop), `MOOD_WEIGHTS`, resting `mood` per species | Protocol `💗️feeling-dynamics` 10/10; stage unit `:2109` (contagion), `:1985` block; trace `chemistry-and-moods` | **met** |
| "interaction of the pets with each other is dependant on the mood and state" | `chemistry` (125 reactions, §1.5), `M/🎯️choice` `beat`/`react`, `M/💞️sociability` `meet` with `encounterBias`/`swayedShares`, contagion `caught` | Protocol `⚗️chemistry-rules` 8/8 (27 stories, 299 consequences); stage unit `:2128`; `pet-cast:266-335` (rule ids, sides, effects, distances, "every scene has a reaction"); trace `chemistry-and-moods`; **no browser evidence** | **met in data**, **thin as seen** (S1) |
| "user can interact with the pets with left clicking" | `PR/🤏️grasp:176` (`button === 0`, primary pointer, no modifier, no control, no selection, point in a visible pet's body), stage `clicked` | `pet-handling` (17 hand cases); `pet-walk:675` | **met** |
| "first time hello, then … tricks or purr" | `M/👆️gesture/🟦️.ts:102-159` (`HEAT_CLICK` 1, leak 0.5/s, hello ≤ 1, trick ≤ 3, purr < 7, enough ≥ 7, forgiven at 2), `M/💞️sociability/🟦️.ts:296-330` | stage unit `:2013` (hello, click tricks in order, purr drawn out, enough: shrug then glance), `:2040`; browser `pet-walk:675` "hello → trick → purr", 6 of 6 runs, trails recorded | **met** |
| "pets can start to alter ui elements … house pet … push a question out of the stack" | `M/🪄️mischief` (`fits` 143, `fixtureFor` 149, `stationFor` 209, `liftAt` 249), `M/🎯️choice` `prank`/`mischief`, locomotion Mischief region 1450+, `PR/🪞️lifting`, survey `fixtures`; quiz marks: `QR/🔨️modules/📖️quiz-page:219` (task rows `<quiz>/<task>`), `▶️run:344` + `🗂️classification:68` + `↕️sorting:159` + `🃏️matching:132` (run items), `🏁️results:210,371` (true order) | stage unit `:2673-2846` (8 cases × 2 companies); `P/🧪️tests/🪞️fixture-lifting` 22; Protocol `🪄️mischief-choice` 8/8; traces `idle-learner-prank`, `reclaimed-prank`, `scene-change-prank`; browser `pet-walk:1111` (6 of 6 reported runs, hover and focus) | **met** (1.7) |
| "These interactions never change the document" | lifting contract (`PR/🪞️lifting`): original keeps place, size, content, attributes, listeners; only inline `opacity:0 !important` + `transition-property:none !important` by CSSOM, given back exactly; copy is `inert` + `aria-hidden`, ids/names/`data-*`/`aria-*` stripped | fixture-lifting: `outerHTML` identical afterwards (style declarations too), CSP `style-src-attr 'none'` run (C1b §6), fuzz `--mischief` strays 0 | **met** — an inline `style` attribute exists on the original while lifted (a MutationObserver sees it) and the copy is DOM inside the layer; content, order, focus and listeners are untouched |
| "As soon as the user interacts again … thrown out, element returns to the original location" | layer: capture-phase reclaim on `pointerover pointerdown focusin keydown input change dragstart selectstart click` (target or 12 ancestors), `reclaimed` is folded in **before** the survey of the step (`🫧️layer` `step`); core `thrownOff`, `evicted`, `SHEEPISH` | pet-handling "tells the stage of a fixture the learner took back before the survey…"; stage unit `:2787`; browser: original back within 80/8 ms (hover), 3/2 ms (focus) (reported B5 §3), pusher trail `push/wall → tumble/air` | **met** |

### 1.2 Gear on the real quiz pages — what a strict reading finds

- **Exists:** all four gear items are species data (`climb` 6 species, `ladder` 8, `grapple` 6, `parachute` 11; sunny, cloudy and
  venty are floaters and own none by design) and each has clips, tools drawn in the depiction (`PR/🧰️gear`) and a route in the
  stage. Computed from the twenty `🔣️.json`.
- **Used and seen in the real browser:** `wall` (via mischief) and `chute` (via the hand). **Not seen:** `ladder`, `rope`.
  `pet-walk:1146` ("a pet that cannot hop to a perch above climbs a card's side, carries its ladder or shoots its grapple
  there") is `test.fixme(true, "pets take no route with their gear yet: … work package B4 … have not landed")` — B4 landed
  (`📓️report2-b4.md`), the reason is stale, and the body would even pass on `wall` alone, which would still prove nothing about
  ladders and ropes.
- **Why it matters:** B5 §3 measured the real heating page at 1440 × 900: task rows at y ≈ 351–410, no card top carries a
  perch for the cast's tallest, every pet lives on the footer at y ≈ 874; ladders and ropes do not reach. B4's 7–33 trips per
  5–10 lively minutes were measured on synthetic terrain (`b4_explore.ts quiz/roomy`), not on the real DOM. In **calm** (the
  default) `VENTURE_SHARES` is 0.25 and `CLIMB_SHARE` is lively-only.
- **Verdict:** the code and the art exist and are well tested at stage level; "use a ladder / use a grappling gun" has no
  evidence on a page the owner opens. See B1 for what to ask.

### 1.3 Non-collision — evidence and escape hatches

- **Invariant:** bodies (size box + hover + margin 4, tilted about the scruff, plus canopy) pairwise disjoint after every
  `advance`; `heldApart` runs inside every stage test that advances time (`:111-121`), the stage-trace adapter and the Python
  keeper check `apart` on every checkpoint.
- **Fuzz (reported):** B1 24 seeds × 30 000 ticks, B4 120 seeds × 6 000 with and without walls, B5 48 seeds × 30 000 with
  mischief — **0 overlaps, 0 outside the stage box, 0 hard landings with a parachute** in all of them. Order breaks 0–2 per run
  (a pet pushed against the stage edge by a survey).
- **Browser:** `pet-walk:748`, 0 overlaps in all 13 reported lively minutes (dev and rehearsal topologies). The definition is the species' size box mapped by
  the drawing's CTM (≥ 8 px apart expected, 0.5 px tolerance) — tools, canopies and encounter poses are not in it (C4 §5).
- **Hatches, in order:** planned paths + claims → guarded strides (`guardedStride`) → head landings (`slide`) → `scoot`
  → fall steering (`columnOver`, `STEERINGS`) → wait `DELAY` 32 ticks while the place is free → **`poof`** (`locomotion:106`,
  counted, `puffs` drawn). Two more puffs are **not counted**: the bottom-edge exit (B4 §2.2) and the mischief slip-over
  (`popTo`, B5 §1). Fuzz with walls (B4 §4): 233 bottom exits in 2.15 M actor-ticks (sample) and 380 in 3.96 M (architecture),
  i.e. ≈ 100 per million against 3.7–14.6 counted poofs per million. "Poofs must stay rare" (design §18) is therefore measured
  on the smaller half of the teleports. Nothing in the browser spec counts puffs (the gallery readout does). See S12.

### 1.4 Tricks, states, moods

Per-species table in §2. All twenty have ≥ 3 states (computed: 3 for radiatory, boily, shady, thermy; 4 for the rest), 5–7
tricks (2–3 click tricks, 2–3 circling tricks, 7 species with a cue-less chemistry-only "signature duet": sunny `rainbow`,
cloudy `snow`, waly `u-value-duel`, battery `power-vs-energy`, boily `relight`, shady `sun-block`, flamy `look-up`), a purr clip
with its own emitter, 4–8 emitters, gear that fits the nature, a resting mood. States tint, loop an overlay clip and run an
emitter (63 palette and tint body colours checked against both pages, §6 Menagerie README, claim 5). `pet-cast` pins that every state is reachable from the resting one and returns
(`:239`), that circling changes the resting state (`:246`), and that every clip/emitter/state a trick names exists (`:231`).
It does **not** pin "≥ 2 states", "≥ 3 tricks", "≥ 1 click trick" or "both languages non-empty" (the schema's `Text` carries
both languages; the other three are only true of today's data).

### 1.5 Chemistry — count and coverage (computed)

- 125 reactions = 63 physical rules (R01–R64 but R06) + 5 mood rules (G1, G3–G6). 125 unique ids, 0 unresolved
  species/states/tricks, every id appears in the AP README table (script check). `every` 20 s for 101 rows; `within` 90 px for
  102 rows. Rows that read a state: 104; a mood: 13; an activity: 4; effects: 166 mood, 71 state.
- **Reactions that can fire per scene** (both species can be on one stage, the home pool counted as 5 core + 1 visitor):
  home 99, physics 46, heating 38, cooling 30, demand 22. **Eight rows can never fire in the shipped product**: R15 (×2),
  R20 (×2), R57 kettly/boily/flamy (×3), R62 — the README says "gallery only".
- **How often they fire** (reported, C4 §2 storyboards, 15 simulated minutes, seed 7, with clicks and circles, **before** B4/B5
  landed): calm — home `g1 ×2`, physics `g1 ×4`, heating `r28 ×1 r24-shut ×3`, **cooling none**, demand `r48 ×1`; lively — home
  `r36-shining ×7 r46-sunny ×6 g3 ×1`, physics `r04-glad ×16 g1 ×6`, heating `r24-shut ×9 r28 ×2 g1 ×2 g5 ×2`, cooling `r40 ×1`,
  demand `g1 ×22 r49 ×6 r48 ×5`. The default is calm. See S1.

### 1.6 Click escalation

Heat bucket: +1 per click, −0.5/s leak; hello ≤ 1, trick ≤ 3 (the n-th trick click plays the n-th click trick in authored
order, round and round), purr below 7, **enough** from 7: `shrug` and turn away, then 8 s (`ENOUGH_TICKS` 512) of glances, then
forgiven at heat 2 (the next click is a trick). No counter on screen, nothing punished (`gesture:102-159`,
`sociability:296-330`). A sulker is reconciled, a sleeper wakes, a busy pet only feels it. Keyboard deeds map onto the same
state (`played` `hello/trick/pet`, `toss` is the hand's). Matches the owner's sequence; spacing matters: two clicks more than
2 s apart are two hellos (leak), which is the intended "leaky" behaviour.

### 1.7 Mischief — which element, which screen

- **Marks** (`data-pet-prop`): opened quiz page task rows `<quiz>/<task>` (`📖️quiz-page:219`, the "Tasks in this quiz" stack);
  run: classification chips, sorting rows, matching item rows (`<quiz>/<task>/<item>`; value cards and table rows never);
  results: the true order of a sorting. Cards of the overview carry `data-pet-topic` (matched, never lifted).
- **Who can push what** (computed from grounds ∩ cast): heating page — `heating/heating-load-and-demand` by **radiatory, housy,
  insuly**, `heating` by thermy; demand page — `demand/standard-profiles` by **housy**, venty, `demand/final-energy` by
  **housy**, insuly, radiatory; cooling — chilly, cloudy, thermy, venty (`cooling/air-change-rates`), sunny; physics — only
  flamy (`physics/powers`) and cloudy (`physics`); servy has only an item-level ground and can only play in a run. So the
  house pet pushing a house question out of the stack is possible on the heating and demand pages, **on 1024 px and wider**,
  and only when housy is in the (rotating) cast. The e2e does not pin housy: over its six reported runs (12 reclaims) the annotations name radiatory ×8 and insuly ×4.
- **Gates:** `petsMischief` on, `(pointer: fine)`, stage ≥ 1024 px, not still, learner idle 12 s (30 s in a run; a pointer
  move, input and scroll all count), cooldown since the last prank (calm 180 s, lively 45 s, **also counted from the stage's
  opening**), no prank under way (`mischief/🟦️.ts:33-45,173-190`). A calm learner sees the first prank after ≥ 3 minutes.
- **Choice never reveals an answer:** two inputs only (ground ↔ key, one stream word); a Proxy test shows only `id/key/x/y/width/height` are read.
- **How it gets there:** on the real pages the cards are packed, so the pet usually does **not** walk or climb: it **slips
  over in a puff** to the wall post beside the element (`popTo`, B5 §1 and §7). The push itself is then real (brace 24, shove 40,
  wobble 48, hold 512, return 56, fade 8 ticks = 10.75 s).
- **Restore/throw-off:** see the last two rows of 1.1. It is also restored on hidden, pause (`rest`), scene change, `still`,
  mischief withdrawn, coarse pointer, unmount and when the box moves > 0.5 px (`layer` 506–512, 568–590; B5 §1).

---

## 2. Content quality

### 2.1 Per species (computed from `AP/*/🔣️.json`)

| Species | States | Tricks | Emitters | Gear | Resting mood, grip, reach |
|---|---|---|---:|---|---|
| sunny | 4 (3 tinted) | 5 (2 click, 2 circling, cue-less `rainbow`) | 6 | none (float) | content, 48, 6 |
| cloudy | 4 (3) | 6 (2, 2, `snow`) | 8 | none (float) | content, 37, 7 |
| housy | 4 (3) | 7 (2, 3, —) | 7 | ladder, grapple | content, 49, 8 |
| solary | 4 (3) | 5 (2, 2) | 5 | climb, ladder, parachute | happy, 49, 7, canopy |
| radiatory | 3 (2) | 5 (2, 2) | 4 | climb, ladder | content, 39, 4.5 |
| pumpy | 4 (3) | 6 (2, 2) | 7 | parachute | content, 43.5, 9, canopy |
| windowy | 4 (1) | 5 (2, 2) | 5 | climb, parachute | curious, 49, 8, canopy |
| waly | 4 (3) | 7 (2, 3, `u-value-duel`) | 5 | ladder | content, 41.5, 8 |
| battery | 4 (3) | 6 (2, 2, `power-vs-energy`) | 7 | climb, ladder, grapple | playful, 49, 8 |
| windy | 4 (3) | 5 (2, 2) | 4 | parachute | content, 43, 6.5 |
| boily | 3 (2) | 6 (2, 2, `relight`) | 4 | ladder, grapple | content, 41.5, 2.5 |
| roofy | 4 (2) | 6 (2, 2) | 7 | ladder, parachute | content, 40, 3, canopy |
| insuly | 4 (1) | 6 (3, 3) | 5 | climb, parachute | content, 36, 4, canopy |
| shady | 3 (0) | 7 (2, 2, `sun-block`) | 5 | ladder, parachute | content, 43, 9, canopy |
| venty | 4 (3) | 5 (2, 2) | 4 | none (float) | proud, 35, 4.5 |
| chilly | 4 (3) | 5 (2, 2) | 7 | climb, parachute | grumpy, 38, 11.5, canopy |
| kettly | 4 (3) | 6 (2, 2) | 7 | grapple, parachute | content, 39.5, 8, canopy |
| flamy | 4 (3) | 7 (3, 2, `look-up`) | 5 | parachute | curious, 16, 5, canopy |
| thermy | 3 (2) | 5 (2, 2) | 6 | grapple, parachute | content, 48, 1.5, canopy |
| servy | 4 (2) | 5 (2, 2) | 5 | grapple | curious, 53, 5 |

Reading: every species meets ≥ 2 states and ≥ 3 tricks; the "signature" is cue-less for seven only, but for the others the
two click tricks are the signature (sunny `corona`, cloudy `rain`, kettly `boil`/`whistle`, …). Gear is coherent: heavy
wall/box things (waly, servy, boily, housy) get ladder/grapple, light ones (insuly, flamy, shady) a canopy, windy (a turbine)
only a parachute, floaters nothing. Sunny shows its states visibly (C2 screenshots: blazing with heat waves and corona, dim
muted, sunset flat and orange; trick sheet: corona rays, prominence arcs, afterglow orbit, rainbow). Physical content matches
the quiz facts (R02 1 000 W/m², R31 303 → 15 kWh/(m²·a), R42 U 1.0 → 0.28 → 0.15 — as in the bond table).

German (all computed, informal where addressed): 0 missing `de`, 0 formal pronouns (`Sie/Ihr…`) in species data; identical
en/de only where the word is the same (`Warm`, `Mild`, `thermometer`); articles of the twenty `Species.name` are right (the
`servy` defect of the first audit is fixed: "Serverschrank"). Spot reading of every German state and trick: all natural
("Gleißend", "Abendrot", "Hauchdünn", "Vorlauftemperatur", "Stoßlüften", "Klopfprobe", "Passivhaus-gedämmt"); the only
anglicism is "Cooler Auftritt" (shady `pose`), idiomatic. Nit: the three different "Cool down" tricks (waly, chilly, kettly) share
one English name — harmless, names are not shown to learners today (§4).

### 2.2 Chemistry — physical sense, 15 rules spot-checked (README table, data checked)

| Rule | Reading | Verdict |
|---|---|---|
| R01 cloudy heavy/raining 24 px from sunny → sunny `dim`, grumpy | a thick cloud cuts 1 000 → 100–300 W/m² | sound |
| R02 sunny blazing near solary generating → `peak` | 1 000 W/m² is the module's test irradiance | sound |
| R05 sunny blazing 20 s near fluffy cloud → `heavy` | convection builds towering clouds | sound (generalised) |
| R07 raining cloud + sunny → `rainbow` duet (rest 180 s) | refraction in drops | sound |
| R08 wispy/fluffy cloud 24 px from sunny → sunny grumpy 0.4 | "a thin cloud lets 90 % through: only teasing" | weak: the reason says it hardly matters, the effect says the sun minds |
| R13/R14 rain / gale snuffs flamy; damp air → `ember` | wind fans, then blows out | sound |
| R16 kettly heating/boiling + snuffed flamy → flamy `burning` | "an electric spark relights a wick" | weak: a kettle sparks nothing; the rule only exists because the brief wanted the duet |
| R19 boiling kettle + cloud → `condense` | steam looks like a baby cloud | sound (playful) |
| R21 radiatory warm + open window → squabble | an open window throws away the radiator's heat | sound |
| R23 radiatory **cold** + shut window → `fogged`, sad | "warm moist air condenses on cold glass" | weak: the cause is the cold *glass*, not the cold radiator |
| R26 window open 12 s + cosy house → `cold` | ventilation heat loss | sound |
| R36/R47 sunny `shining` = winter gains, `blazing` = summer gains | the scene is unknown to the stage, so the sun's state stands for the season | disclosed proxy; physically contrived (a clear noon in January is `shining`) |
| R44/R45 rain above insuly soaks her unless a roof is within 120 px | wet insulation insulates badly; the roof keeps it dry | sound |
| R48 pumpy heating + radiatory warm → radiatory **`hot`**, happy | "low flow temperatures with a heat pump" | **wrong sign**: the reason says low flow temperature; the effect heats the radiator to `hot` (that is the boiler's R29 story) |
| R58/R59 solary peak / windy rated + battery → `charging`; full battery → solary sad "curtailed" | the battery stores the surplus; a full store takes no more | sound |

Everything resolves (0 unresolved ids), distances respect `reach` (pet-cast `:314`). R48 and R23 are the two to fix.

---

## 3. Accessibility (WCAG 2.2)

| SC | Finding | Evidence | Verdict |
|---|---|---|---|
| 2.1.1 Keyboard | A "Play with the pets" group in the settings: one named group per pet on stage, four native buttons (Hello, Trick, Pet, Toss), Enter/Space, focus stays on the button, one polite atomic status line that says only what was asked. Present only while pets are calm or lively, play is allowed and somebody is on stage | `🐾️pets/🎪️stage:168-204`, `🎛️preferences:240-253`; `pet-walk:885` (German, 4 deeds by Enter/Space); C3 browser drive | met — but the group is in the settings pane, not beside the pets; and see 2.5.1 |
| 2.5.1 Pointer gestures | Path-based gestures: circling (state ladder tricks), stroking (purr), shaking (shake tricks), dragging. Alternatives: `pet` (= stroke), `toss` (= throw), `trick` (= **click** tricks only) | `feeling:480-526`; computed: 19 of 20 species have ≥ 1 state reachable only by a gesture (circle, countercircle, shake) or the chemistry (`servy` is the exception); 50 tricks have only gesture cues (circle/countercircle/shake) and 7 more only the chemistry | **partly (S2)**: design §14.5 and quiz README ("everything the hand does") are not true in the data |
| 2.5.2 Pointer cancellation | A press only arms; hello/trick/lift fire on release or after the slop; Escape, `pointercancel`, lost capture, window blur, hidden document, play off, still stage and a second finger abort; the Escape is kept from the page only while a press is open; the click a taken press leads to is swallowed, a keyboard click always passes | `grasp:170-241`; `pet-handling` 17 hand cases | met. (A hold purrs on the down side after `HOLD_TICKS` 28 — cosmetic, no function) |
| 2.5.7 Dragging movements | `toss` lifts to the top and lets go (fall/parachute) without a drag. "Place the pet where I want" has no alternative | `stage unit :2289` | met for the function (fall/parachute); decoration otherwise |
| 2.5.8 Target size | Controls of the group: `.quiz-target` min-height 24 px (`QR/🎨️.css:179`), labels wide. Touch pads = the pet's body: Flamy is 28 px wide → 22.4 px at the phone scale 0.8; pets are decoration with a keyboard-reachable equivalent | `PR/🫧️layer:336-371` | met for controls; nit for pads |
| 2.2.2 Pause, stop, hide | Footer switch "Show pets" on every screen (native checkbox, stored with the preferences), `still`, `off`. Switching off unmounts the layer → `stop()` ends the hand (capture released, `data-pet-held` removed), `lifting.end()` restores every original, scenery struck. Mode → `still` or play off or hidden document calls a held pet off and gives lifts back (`direct` 576–582, `rest` 506–512) | `layer:545-560,568-590`; `pet-walk:1061` (switch by keyboard); no e2e of "switch while held / lifted" | met in code, thin in tests (nit) |
| 2.3.3 Animation from interactions / reduced motion | `prefers-reduced-motion` decides the default only: motionless pets, no hand (no press taken, no cursor), no lifts, until the learner explicitly chooses a liveliness or the switch (then it holds on every device); `petsPlay` and `petsMischief` can be turned off separately; the layer forbids transitions inside itself | `🐾️pets/🟦️.tsx:58-60,182-184`; `pet-walk:982` (no grab cursor, click changes nothing, play group absent, calm walks after choosing, also after reload) | met |
| 2.4.11 Focus not obscured | The focused element grows an 8 px keep-out for perched pets and is never a fixture (`survey:294-297`, `FOCUS_MARGIN` 8). **Not covered:** an airborne/held pet, a canopy and the lifted copy's swept box can pass in front of a focused control; design §17's "see-through while airborne over a control or text" was not built (C4 §7.3, `presenceOf` only reacts to the pointer) | `survey`, `attention:63-70` | **partly (S4)** |
| 1.4.13 Content on hover or focus | Pets add no tooltips, popovers or hover content; the only hover effect is a cursor and a see-through under the resting pointer | `grasp:134-157` | n/a / met |
| 4.1.2 / screen readers | Layer `aria-hidden="true"`, nothing focusable, no role/id/style/script inside (asserted by `pet-walk:648`); the lifted copy is `inert` + `aria-hidden`, its `id/name/for/form/tabindex/role/title/aria-*/data-*/href` are stripped (`getRoles` empty, `isInaccessible` true for every node — fixture-lifting); **the original is not hidden from the tree** (opacity 0, no `visibility`/`inert`), so a screen reader still reads it where it was and its focus order is unchanged | `PR/🪞️lifting`; `layer:682` | met |
| Cognitive load in runs | `quiet` in a run: no walks, whims, encounters, chemistry, rotation; clicks and pick-up still answered; mischief only after 30 s without pointer, input or scroll, and only with `petsMischief` (default **on**); C4 §7.1: on the physics run at 1440 × 900 no pet fits at all | `choice`, `mischief:36-39`; `QR/🐾️pets/🟦️.tsx:206` | **partly (S5)** |
| Touch | Taps work everywhere; dragging needs pads on `(pointer: coarse)` while play is permitted, only for `perch` pets, clipped at the feet, `touch-action: none`; hover gestures and mischief are fine-pointer only; the phone shows ≤ 2 pets at scale 0.8 | `layer:336-371,519`; `pet-walk:1165`; pads only verified in the C1a harness (CDP touch), not in the gate | met; pads not in the e2e (nit) |
| Flashing (2.3.1) | `thunder` flashes twice in 2 s, particle lives ≤ 1.6 s, no per-part opacity | content brief, species data | met (not measured) |

---

## 4. Internationalisation

- **Learner strings:** 12 new keys in `QUIZ_BUNDLE_EN` and `DE` (`🌐️i18n/🟦️.ts:127-138` / `:580-593`): `petsPlay`,
  `petsMischief`, `petsResting`, `petsPlayWith`, `petsHello/Trick/Pet/Toss`, four `…Said` lines. Used literally, so
  `🗣️translation-completeness` (every used key is a leaf, DE keys equal EN keys, informal German) covers them. The deed buttons
  are worded differently in the two languages (Hello/Hallo is the only near-pair; `🗣️both-languages` compares wording), and
  German is informal ("Dein Gerät bittet um weniger Bewegung …").
- **"Tierchen" consistently:** grep of the quiz and site sources finds no `Haustier`, `Maskottchen`, `Begleiter`; the pets key
  group is "Tierchen" throughout ("Mit den Tierchen spielen", "Tierchen anzeigen", "Gerade hier: …").
- **No default language:** every species `name`, `thing`, state name and trick name carries `en` and `de` (computed: 0 missing,
  0 formal); the layer itself draws no text; the gallery has an en/de switch.
- **Names that learners never see:** state and trick names (en/de) are authored, validated and shown only in the stories
  gallery. The play group's status line does not say *which* trick or state happened ("Sunny, the sun does a trick."), only the
  deed asked for. No finding — an option for accessibility ("what did my click do?").
- **Findings:** (a) the status line interpolates the full display name, which already contains a comma and the thing:
  "Sunny, the sun says hello." / "Sunny, die Sonne sagt Hallo." — readable in German, clumsy in English; the quiz README quotes
  "Sunny says hello." (nit, S10). (b) `🗣️both-languages` counts controls per page in both languages (`:60`); the settings page now
  holds 4 buttons per pet on stage and the cast rotates every 60–120 s — C3 §11 warned, the test file is unchanged since
  2026-10-02 17:55, so a rotation between the two readings flips the count (flake risk, S9). (c) The English "Pet" on a button is
  ambiguous without the group name ("Sunny, the sun" + "Pet" is fine for AT, a bare "Pet" is not) — it is always inside a named
  group.

---

## 5. Customisation and state lanes

| What the learner controls | Default | Persisted / synced |
|---|---|---|
| Pets: `off` / `still` / `calm` / `lively` (segmented control in settings) | `calm`; on a reduced-motion device `still` until chosen (`petsChosen`) | persisted local-only (`semio.quiz.<tenant>.preferences`), synced across tabs by the `storage` watch (`QR/🟦️.tsx:551`) |
| "Show pets" switch on every footer | follows the choice; brings back the last non-`off` liveliness | same record |
| `petsPlay` — react to clicks / can be picked up | on; disabled+unchecked with a described reason while off or still | same record; only an explicit `false` turns it off |
| `petsMischief` — may play with the page | on; same disabling | same record; **one switch for page and run** |
| Play with the pets (keyboard deeds) | present while play is allowed | ephemeral |
| Not customisable | which pets are on stage (scene cast), size (host prop `scale`), speed (test seam `data-pets-tempo`), mischief in runs separately, which trick/state a pet does, hiding one species | — |

State lanes (`AGENTS.md`): persisted local-only = the preferences above; **ephemeral local-only** = the whole stage (positions,
rapport, warmth, states, lifts, ladders), every tab simulates its own pets, seeded randomly per mount; **ephemeral shared,
read-only** = other learners' cursors, fed to the stage as `glanced` points; **persisted shared** = none (nothing of the pets
travels). The pets README's table lists only "whether shown / how lively" under persisted local-only (S7); the quiz README's
state-class table does not name the pets preferences either.

---

## 6. Docs — five claims each

**Pets README (`P/README.md`)**
1. "twenty-six activities … in an order that never changes" — **true** (`ACTIVITIES`, 26, schema `:55`).
2. "mischief gates: 1024 px, 12 s (30 s quiet), cooldown, a stage that just opened rests a cooldown too" — **true** (`mischief:33-45`, B5).
3. "the four reserved streams (`STAGE_STREAM`, `CAST_STREAM`, `ROTATION_STREAM`, `CHEMISTRY_STREAM`)" (line 19) — **stale**: `GEAR_STREAM` 0xfffffffb and `MISCHIEF_STREAM` 0xfffffffa exist (`randomness:35-39`).
4. "`advance` folds … `ticked`, `pointed`, `unpointed`, `glanced`, `surveyed`, `summoned`, `tuned`, `hushed`, `poked`" (lines 189-191) and "a poke is ignored" (216) — **stale**: `poked` is gone; `Pressed Dragged Released Cancelled Reclaimed Stirred Scrolled Played Permitted` are missing (`StageEvent`, schema `:372`).
5. "focused element keeps `FOCUS_MARGIN` (8 px) free" — **true** (`survey:43`); "State classes … persisted local-only: whether pets are shown and how lively" — **incomplete** (also `petsPlay`, `petsMischief`).

**Menagerie README (`AP/README.md`)**
1. Casts table equals the ensemble JSON (all five scenes, core and rotation) — **true** (computed).
2. "All 67 bonds" — **true**; "chemistry … 125 rows for R01–R64 but R06 and G1, G3–G6" — **true**, every id is in the table (script check).
3. States/tricks/gear table rows (sunny, cloudy, solary spot-checked) — **true** against the JSON (cues, `lasts → then`, `to`).
4. "Mischief on lifted copies … wait for the stage (`fixme`)" (line 373) — **stale**: the mischief test is live (`pet-walk:1111`); the gear test is still `fixme`, for a stale reason.
5. "State: … a `tint` (colours that keep 3:1 against both pages)" (line 335) — **not true of the data**: 17 of 63 palette/tint body colours are below 3:1 on at least one page (computed; e.g. sunny blazing 1.14 on light, cloudy raining 2.43 on dark, solary shaded 2.06 on dark). The 2 px ink outline carries them (H1/H3/H4 say so) and nothing in `pet-cast` pins the claim.

**Quiz README (`Q/README.md` `### Pets`)**
1. "every screen carries a 'Show pets' checkbox on its footer line" — **true** (`QR/🟦️.tsx:508`, inside `LegalFooter`).
2. "Only an explicit `false` in storage turns one off; neither sets `petsChosen`" — **true** (`preferences:98-99,129-130`).
3. "keyboard and single-pointer way to everything the hand does (WCAG 2.1.1, 2.5.1, 2.5.7)" — **overstated** (S2).
4. Status line example "Sunny says hello." — **inexact**: the line is "{{name}} says hello." with the display name "Sunny, the sun".
5. Run behaviour ("they play with the page only after the learner has been idle for a long while") — **true**; "Where a screen leaves no room … no pet shows" — true and **also on desktop**: C4 §7.1 found no pet in the physics run at 1440 × 900. The State-class table omits the pets preferences.

**Design-v2 §24 / the contract**
1. §24.1 `STATE_BLEND` 16, `WALL_LEAN` 0.04 — **true** (`projection:33,36`).
2. §24.2 `PUFF_CLOUDS` 8, `PUFF_RING` 7, `PUFF_RISE` ¼ — **true** (`effects:209-215`).
3. §24.3 cooldowns calm 3 min / lively 45 s, patience 12/30 s, `SHEEPISH` 0.3, `FIXTURE_SLACK` 0.5, stream 0xfffffffa — **true** (`mischief:36-45`, `choice:34`, `population:29`, `randomness:39`).
4. §21 contract types — **stale**: `Trait` lacks `held`, `trick` and an optional `species`; `Effect` lacks `activity`; `Reaction` lacks `unless`, `affinity` (schema `:252-258`); `Wall` is `Pitch` (`:313`). The document calls itself normative.
5. §24 records only B3, C2 and B5; B1 (footings, the hand, claims), B2 (mind), B4 (trips), C1, C3 (lazy half), C4 deviations live only in their reports (e.g. B1 `Course`, `Claim`, `Puff`; B4 `Trip`, `Foothold`; C3 `data-pet-topic`). §22's e2e promise "a pet climbs a card wall; a ladder and a rope are used" is not marked as deferred.

---

## 7. Gaps and risks (severity, evidence, suggestion)

**B1 — blocker (acceptance): ladder and grappling gun have no proof on a real page; the gear spec is `fixme` with a stale reason.**
`S/🧪️tests/🐕️pet-walk/🟦️.ts:1146-1147`. The only browser-seen gear is `wall` (mischief) and `chute` (hand). B5 §3 says ladders and ropes cannot reach on the real heating page at 1440 × 900; B4's trip counts are from synthetic terrain; in `calm` gear ventures are rare. Suggest: (1) delete the `fixme`, rewrite it to require each of `ladder` and `rope` (not "any of") at tempo 8 on the pages where a route exists — probe first which page/viewport that is (home overview and results at 1024–1440, quiz page, a `roomy` layout) with a one-off count of footing trails over 10 lively minutes; (2) if the count is 0 on every real layout, say so to the owner and adjust either the geometry (a strip of room above the footer cards, longer ladders `≤ 3.6 → 5` heights, rope reach `3.2`) or the claim; (3) keep the wall assertion separate so a green wall cannot hide a missing ladder.

**S1 — should-fix: chemistry is almost invisible in the default mode.** Reported storyboards (15 min, with clicks and circles, before B4/B5): calm cooling 0 reactions, demand 1, home 2 (`g1`), heating 4, physics 4 (`g1`); lively cooling 1. Of the 125 rows, the mood-only `g1` bicker is the most frequent firing in several scenes. The owner's "interaction depends on mood and state" is true in the model and rarely seen. Suggest: re-run `stage_storyboard.ts` with B4/B5 in, per scene/mode, and set a floor (for example ≥ 3 physical-rule firings in 15 calm minutes per quiz scene) by letting pets with a due reaction partner walk to each other (a `visit` venture in `venturesOf`), shortening `every` for the top 20 rules in calm, or raising the beat; add a threshold test on the storyboard digest.

**S2 — should-fix: gestures vs. alternatives (2.5.1, design drift).** 19 of 20 species have states reachable only by circling, shaking or the chemistry (e.g. sunny `blazing/dim/sunset`, solary `shaded/peak/hot`, waly `warm/wrapped/passive`); 50 tricks have only circle/countercircle/shake cues (7 more are chemistry-only); the "Trick" deed plays only click tricks (`clickTrick`) or a whim trick. Design §14.5 and the quiz README claim every gesture trick is reachable by clicks. Suggest: either extend `DEEDS` with "Warm up / Cool down" (= `circle` / `countercircle`) and "Shake" for the play group (schema `DEEDS`, `PET_DEEDS`, two i18n keys each), or make the "Trick" deed cycle all tricks on offer in authored order, and correct the claim; add a policy test in `pet-cast` ("every state reachable from the resting state through deeds the play group offers").

**S3 — should-fix: one control under the ring kills the circle.** `gesture:230` returns `noCircling` while the pointer is over a control, and the band reaches 3.4 × the pet's larger side; on a card top that crosses "Open" buttons. C4 had to carry a pet to the footer line to test it and the home screen failed (C4 §5.1). Suggest: let a lap survive a short crossing (`CIRCLE_OUT_TICKS`-style grace, not a reset), or judge `control` only at the press, and add an e2e that circles a pet standing on a home card.

**S4 — should-fix: nothing checks where the lifted copy and an airborne pet go (2.4.11).** `liftAt`'s travel is `room × (…)` along x only (`mischief:236-252`), `room` is the distance to the stage edge capped at the pusher's width; the post must be beside a free pitch, the destination is not checked against controls, text or the focused element, and a focus on any element *other than the lifted one* does not reclaim (`lifting` reclaims only by the original). B5 shows the copy over the card's right edge. Suggest: clip the travel against `keepouts` (the survey already has them) or reclaim when focus lands in the copy's swept box; same for the unbuilt "see-through while airborne over a control" (design §17, C4 §7.3).

**S5 — should-fix: mischief in a run is allowed by default.** One checkbox (`petsMischief`, default on) covers pages and runs; the design says "only if the learner allows it". In a (timed, challenge) run an item chip the learner may reach for turns transparent after 30 s of stillness. Suggest: a separate "also during a run" opt-in (default off), or no lifts of `data-quiz-item` while a challenge timer runs.

**S6 — should-fix (project rule): the Rust stage twin is not ported.** TS/Rust line counts (computed): `🚶️locomotion` 1572 / 332, `🎯️choice` 318 / 135, `🗓️schedule` 201 / 62, `🎥️projection` 502 / 242, `💞️sociability` 356 / 223; the pure modules (`climbing`, `clearance`, `feeling`, `mischief`, `swing`, `effects`, `gesture`) are ported with bit-exact checks (D1–D3). `the_calm_home_replays_into_its_committed_trace` fails in Rust (B1/B2 report), and design §18 promises "TypeScript and Rust with equal digests". `AGENTS.md` says multi-implementation. Suggest: the "stage-port" package B5 §6 names, before closing the ticket, or record the deferral in the closing summary.

**S7 — should-fix (docs):** items listed in §6 (pets README events/streams/state classes; AP README `fixme` sentence and 3:1 claim; quiz README equivalence claim; design-v2 §21 and §24).

**S8 — nit/should-fix (content):** R48 `hot` contradicts its reason; R23 blames the wrong party; R08 and R16 are weak; 8 rows never fire in the product (R15 ×2, R20 ×2, R57 ×3, R62) and 17 of 63 tints are below 3:1 although the how-to promises 3:1. Suggest: R48 keeps radiatory `warm` (and pumpy proud, radiatory happy) instead of `hot`, which is the boiler's story (R29); R23 either swaps its trigger to "windowy shut next to a cold *wall or chiller*" or its reason to "cold air keeps the pane cold"; reword R08 and R16 or drop them; test the 3:1 claim with an allowlist of the outlined tints, or soften the how-to.

**S9 — should-fix (discoverability):** nothing tells the learner that clicks escalate, that circling or stroking does anything, that a shaken pet gets dizzy or that tossing exists; the only cues are the grab cursor and the settings checkbox "react to clicks and can be picked up". Also flake risk: `🗣️both-languages` counts controls with the play group present (C3 §11, test untouched). Suggest: one sentence of help under "Play with the pets" ("Click a pet, circle it with the pointer, stroke it …") in both languages; make the test count outside `[data-pets-play]`.

**S10 — nit:** status line reads "Sunny, the sun says hello."; use the nickname (first token) or put the thing in parentheses; fix the quiz README example.

**S11 — nit (tests):** `pet-cast` lacks "≥ 2 states / ≥ 3 tricks / ≥ 1 click trick"; no browser test of "switch off while a pet is held / a copy is lifted"; touch pads and the 22 px pad of Flamy at phone scale are only checked in a harness.

**S12 — nit (metrics):** poofs counted 3.7–14.6 per million actor-ticks, but bottom-edge puffs (~100 per million) and mischief slip-overs are not counted; the gate never counts puffs. Count all three and print them in the lively-minute annotation.

**S13 — note (behaviour):** on the real pages the mischief post is reached by teleport (`popTo`), not by climbing; the first prank of a session waits one cooldown (3 min in calm), so the owner will not see it without switching to `lively` and waiting ≥ 57 s idle. Worth saying in the closing summary.

What I could not verify from files: how the pets look and feel on the owner's machine (P1 measured 12–13 fps for the home overview with pets off in a software-composited browser), whether a ladder or rope ever appears on the live pages, and the appearance of the other fifteen species' states and tricks (I looked at sunny's sheets only).
