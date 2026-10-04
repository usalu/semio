# 📓️ Report B5 — Mischief in the stage

`TK` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS`, `P` = `🧰️framework/🛍️products/🐾️pets`, `S` =
`🎓️teaching/🏛️architecture/❓️quiz`. Repo MCP unreachable (CONNECTION_CLOSED) the whole time: no ticket bookkeeping.
The as-built decisions are also in `TK/📓️design-v2.md` §24.3 (normative table).

## 1. What was built

A pet now lifts a copy of a topic-fitting element of the page, holds it out of its stack, puts it back, or is thrown
off when the learner takes the element back. The document never changes. The shell only makes the original
transparent and draws the copy (C1b). The stage reads nothing of the host except a fixture's `key` and box (rule 7).
A Proxy test checks that only `id`, `key`, `x`, `y`, `width` and `height` are read.

| Phase | As built (TypeScript core) |
|---|---|
| Fixtures | `Surveyed.fixtures` land in `Stage.fixtures`. A survey that loses the lifted fixture, or finds it more than `FIXTURE_SLACK` = 0.5 px away from where it was (`astray`), ends the prank quietly (`unlifted`). |
| Gates (§20) | `circumstancesOf(stage, tick)` → A8 `allowedFrom`. The conditions: `permitted.mischief`; `fine: true` in the stage, because the layer folds `(pointer: fine)` into `permitted.mischief` (`PET_FINE_POINTER`); width ≥ `MISCHIEF_WIDTH` 1024 — the shell sends `Surveyed.width`, which is the layer's viewport width in stage units (px ÷ size, and size is 1 at ≥ 768 px); mode is not still; no lift in progress; the learner has been idle ≥ `MISCHIEF_PATIENCE` 768 ticks (12 s) or, while quiet, `…_QUIET` 1920 (30 s), counting from the latest of `stirred`, `pointed` (moves, drags, releases) and `scrolled`; the mode's cooldown since `rested` has passed (calm 11 520 = 180 s, lively 2 880 = 45 s). `rested` is 0 when the stage opens, so the first prank also waits one cooldown. A try that finds nobody able to go also sets `rested`. |
| Prospects | `prospectsOf`: sociable, whole, idle pets on a perch with no trip, whose grounds cover a fixture's key (A8 `fixtureFor`). Each has a post (`postOf`): perch stations first (body clear of the fixture), then, for climbers, the wall stretch beside the fixture with feet at the element's lower edge, held between rim and foot. The best post has the most room (≤ the pet's width), then is the nearest. There are no prospects while the mode's movers (walk/hop/trip) are at `MODE_LIMITS`. |
| Pick | Word 0 of `[seed, MISCHIEF_STREAM = 0xfffffffa, tick, 0]` selects one prospect (`chosenFixture`). If that pet cannot go, the others are tried round-robin. Word 1 is the copy's travel unit, and `[…, tick, 1]` is the throw. The stream is new, so no other draw changes. |
| Errand | `postTo`. To a wall post: the first clear B4 outing that keeps the grip for the lift (`holding`: grip ≥ `GRIP_HANG` × span, no reserve). To a post on its own perch: a claimed walk trip, but only when it is clear (a pet never passes another on its perch). Otherwise **slip over** (`popTo`): a dust puff where it stood (not counted as a poof), and it appears at its post at once at opacity 0 and fades in. On a wall it has a full grip and rests in its climb pose; it only arrives where the body is inside the stage, off claims, and a comfort gap from every body. `Stage.lift.since` = arrival + 1. |
| Push | At `age 0` `pushAt`: it faces the shove side, plays its push clip until `since + LIFT_TICKS + 1`, and on a wall needs the grip to hang until then. `PUSH_HOLD = LIFT_TICKS + 2`. The copy follows A8's `liftAt` (brace 24, shove 40, wobble 48, hold 512, return 56, fade 8 → 688 ticks = 10.75 s; at most 20 s per `LIFT_LIMIT`). |
| Put back | At `age = LIFT_TICKS` the stage ends the push itself (`unpush`: on a wall it rests and then moves on — slides down and lets go where the wall ends, under the parachute rule). The pet feels `tricked` (playful), and `rested` = now. If the pusher stops while the copy is out (picked up, summoned away, its wall gone): during brace, shove and wobble (the first 112 ticks) the copy is gone at once; during the hold, `since` shifts to `now − LIFT_RETURNS` so the copy slides home by itself. |
| Reclaim | `reclaimed` while the copy is out moves `since` past the copy's end (the copy is gone at once). The pusher is thrown (`thrownOff` away from the fixture's box, or `−side × THROW_SPEED, −THROW_LIFT` when the stage no longer has that box), gets `evicted` (scared 0.4), and goes into `unfoot(…, "tumble")` with B1's parachute rule. The prank stays open until the pusher is down and its fright has calmed (`calmsAt`). Then it feels an impulse of sad `SHEEPISH` = 0.3 and `rested` = now. |
| Quiet ends | No throw-off when the survey loses the fixture or it moves (> 0.5 px), when permission is withdrawn (`permitted` with `mischief: false` → `unlifted`), when the stage goes still (`freeze` → `unlifted`), or when an errand is cut (pusher picked up or leaving, trip gone). |
| Horizons | `prankTick(menagerie, stage, from)` is in the clock's `lull` (the earliest of gate opening, push start, lift end, put-back, sheepish), in the projection's `wake`, and blocks `pass`'s empty-stage jump while a lift exists. The chunked determinism test agrees. |
| Graph | `climb → push`; `push →` idle, walk, fall, hang, tumble, climb, mantle, slide, scoot (`AFTER_PUSH`; mirrored in Rust and the oracle's `FOLLOWERS`; behavior-choice fixture regenerated). |

### Shell (React target)

- `PET_FINE_POINTER = "(pointer: fine)"`: mischief is permitted to the stage only while that media query matches. A
  change sends `permitted` again, and when the pointer stops being fine, every lift is given back first.
- **Give-backs come before the survey of the same step** (new in this round, `🫧️layer` `step`). A survey drops a
  fixture that is under the pointer or holds the focus. Before, the stage heard the survey first, ended the prank quietly
  through `astray`, and then ignored `reclaimed`. Four of the six earlier dev e2e runs failed the throw-off assertion.
  The one that printed the trail showed `push/wall → climb/wall → slide/wall → fall/air → …` instead of a tumble. Now the order is ticked → reclaimed → survey → other
  waiting events. A give-back of a fixture that a survey found moved happens inside `measure`, so it still comes after
  that survey and ends quietly, as designed. The new React test runs against the old order and fails there (mutant
  run, then the fix restored).

## 2. Tests added or changed

- `P/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts`: a `describe.each(COMPANIES)` "mischief of …" block with 8 cases for each
  of the two companies. It covers the pick, trip and lift timeline; the gates; the pick's independence (noise fixtures,
  seeds, the Proxy read-set); routing (a floater walks along its ledge, floaters and hoppers take only perch posts, a
  climber on the floor slips over: puff, `since = tick + 1`, wall, opacity 0, full grip, fades in); the reclaim throw
  (vx = `THROW_SPEED + THROW_SPREAD × unit`, scared, touch ≤ `HARD_LANDING`, sad at the end); the put-back (in the hold →
  `since` shift; during the shove → gone); the quiet ends (row gone, moved 2 px, permission withdrawn, still; 0.25 px
  keeps the lift; an errand is cut by a click); and determinism (chunked, plus `wake` at the gate's tick).
- `P/🔨️modules/🎲️randomness` (TS + Rust unit tests): six streams, 9 distinct words.
- `P/🧪️tests/🤏️pet-handling/🟦️.tsx`: one test for the fine pointer, and the new one "tells the stage of a fixture the
  learner took back before the survey of the same step…". The fake core now records each `advance` call (`steps`) and
  serves `lifts`.
- `S/🧪️tests/🐕️pet-walk/🟦️.ts`: the mischief test is no longer fixme. It is now "a pet lifts a copy of a task of a quiz's
  page out of its place, beside its transparent original; the original is back at once on hover and on focus, and the
  pet is thrown off". It runs at tempo 1 on quiz `heating`. It polls until the copy stands > 1 px from the original, then
  reclaims by hover and in a second round by focus. Each time it expects the original back and the copy gone within
  6 s, and the pusher's trail to read `push/… → tumble/air/…`. The failure message prints the trail.
- `TK/generate_behavior_vectors.py`: three new scripts (§5).
- `TK/stage_fuzz.ts`: `--mischief` (implies `--walls`) marks card rows with keys, has idle and busy learner phases,
  reclaims at 1/150 per tick while a copy is out, and adds a mischief table. It exits 1 on any stray (a copy on its way
  out without a pushing pusher). New column: reclaims of a copy whose pusher no longer pushed.

## 3. What was seen in the browser

Real quiz page `#heating` at 1440 × 900, private stack, `TK/b5_probe.mjs --tempo 1` (`🗑️generated/b5/probe-light-4`,
`probe-dark-1`). I looked at every screenshot with Read.

- **Light** (`1-lifted-close.png`): insuly (fluffy) clings to the left side of the "Tasks in this quiz" card at the
  height of row 2. The copy of "2 🔥 Heating Load and Heating Demand" stands some 45 px to the right of its place and
  over the card's right edge, with its own border. The original's slot shows only the card behind it.
  `1-reclaimed.png`: the row is back, the pointer is on it, and insuly tumbles in the air left of the card.
- **Dark** (`1-lifted.png`, `1-lifted-close.png`): radiatory (radiator with heat waves) clings to the card's right side
  and has pushed row 2 about 32 px out to the left, past the card's edge. The copy keeps the dark theme's colours, and
  the rest of the cast stands on the footer.
- Probe log: the lift came 40.6 s and 64.2 s after the page opened (light) and 40.6 s and 42.3 s (dark). The hover
  reclaim restored the original within 80 ms and 8 ms, the focus reclaim within 3 ms and 2 ms. Afterwards insuly went
  tumble/air (scared) → glide/chute, and radiatory went tumble/air (scared) → land → dizzy.
- Why slip over: on this page the cards are packed. The task rows sit at y≈351–410, no card top carries a perch for
  the cast's tallest, and every pet lives on the footer at y≈874. Ladders (≤ 3.6 heights) and ropes (≤ 3.2) do not
  reach. Without the puff, mischief would never happen on a quiz page.

## 4. Commands and real results (final code)

| Where | Command | Result |
|---|---|---|
| `P/📦️packages/🟦️typescript` | `NX_PLUGIN_NO_TIMEOUTS=true bun ./📜️script.ts test` | 16 files, **1119 passed**, 5.80 s (`test-6.log`) |
| same | `… test quick` | 1119 passed, 17.4 s (`test-quick-1.log`) |
| same | `… typecheck` | exit 0, no output (`typecheck-5.log`) |
| `P/🎯️targets/⚛️react/📦️packages/🟦️typescript` | `… test` / `… typecheck` | 8 files, **173 passed** / exit 0 (`react-test-3.log`, `react-typecheck-3.log`) |
| `S/📦️packages/🟦️typescript` | `… typecheck` (includes `🧪️tests/**`) | exit 0 (`site-typecheck-2.log`) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` | `… oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🎪️stage-trace"` | executed 0, not-exercised 1 (recorded no-oracle decision, as in B4) |
| same | `… subject exhaustive --implementation typescript … --case "🎪️stage-trace"` | **3/3 passed** (traces, laws, determinism) |
| same | oracle / subject (typescript) `--case "🪄️mischief-choice"` | **8/8** / **8/8** |
| same | oracle / subject (typescript) `--case "🧠️behavior-choice"` (its fixture changed) | **8/8** / **8/8** |
| repo root | `.venv/Scripts/python.exe TK/generate_behavior_vectors.py` (second run) | "traces as committed: **32 of 32** scripts", every fixture `unchanged` (`gen-check-2.log`) |
| `P/📦️packages/🦀️rust` | `cargo check --tests` / `cargo test --lib -- behavior:: randomness:: schema::` | exit 0, 0 warnings / **63 passed** |
| repo root | `node TK/b5_code_rules.mjs` | problems: **0** (emoji docstrings unique per file, no console, no `[DEBUG]`, no inner comments, no banned stems) |

### Fuzz (`bun TK/stage_fuzz.ts --seeds 48 --ticks 30000 --mischief --menagerie both`, `fuzz-mischief-48.log`)

| menagerie | actor-ticks | overlaps | outside | hard / with parachute | order breaks | pranks | copy ticks | reclaims / of a non-pushing pusher / thrown | sheepish | ended early | slid home | strays |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| sample | 4 305 607 | **0** | **0** | 0 / **0** | 0 | 162 | 19 405 | 123 / 0 / 123 | 121 | 35 | 3 | **0** |
| architecture | 7 750 986 | **0** | **0** | 247 / **0** | 2 | 147 | 14 587 | 107 / 1 / 106 | 99 | 38 | 3 | **0** |

Before the "never past another on its own perch" fix, the same run showed 4 / 8 order breaks. Floaters slipped over to
a post on their own perch past a neighbour. Now: 0 / 2, and the first remaining break follows a `surveyed` (B4's known
re-seat at the stage edge; B4 reported 0–2). Every reclaim of a pushing pusher threw it off. I did not trace the thrown
pushers that were not counted sheepish (2 and 7). The counter only counts a sheepish end while the pusher is still on
stage, so a pusher summoned away, or a session that ends during the thrown phase, is not counted.

### End-to-end (`pets` project of `S/🎭️e2e/🎚️config/🟦️.ts`, `--no-deps`, 4 workers, private stack 6247/8947, watch off, scratch proctor data)

Runs via `bash TK/b5_gate.sh <topology> <label> 4`. The stack comes from `TK/b5_stack.sh` (rehearsal is built first by
`b5_stack.sh build`).

| Topology | Run | Result | Mischief annotation | Lively minute |
|---|---|---|---|---|
| dev | round3-1 | **14 passed**, 1 skipped (gear spec, B4 fixme), 152 s | hover: radiatory push/wall → tumble/air; focus: insuly push/wall → tumble/air | 720 frames, 0 overlaps |
| dev | round3-2 | **14 passed**, 1 skipped, 180 s | radiatory ×2 → tumble/air | 697 frames, 0 overlaps |
| dev | round3-3 | **14 passed**, 1 skipped, 154 s | insuly / radiatory → tumble/air | 586 frames, 0 overlaps |
| rehearsal | round3-1 | **14 passed**, 1 skipped, 120 s | radiatory ×2 → tumble/air | 820 frames, 0 overlaps |
| rehearsal | round3-2 | **14 passed**, 1 skipped, 127 s | insuly ×2 → tumble/air | 700 frames, 0 overlaps |
| rehearsal | round3-3 | **14 passed**, 1 skipped, 125 s | radiatory ×2 → tumble/air | 874 frames, 0 overlaps |

Earlier failing runs, kept in `gate-dev-final-*` and `gate-dev-round2-*` (traces removed), led to three fixes:

1. The shell's survey-before-reclaim order (trail `push → climb/wall → slide …`), fixed in the layer as described above.
2. The spec measured the copy's offset in the brace phase (0.57 px). It now polls for the offset.
3. Tempo 2 → 1, so a copy is out for about 10 s of wall clock while four browsers share the machine.

One `ERR_NO_BUFFER_SPACE` (Windows sockets) failed "the cast follows the learner" once. One B4 overlap appeared once
(round2-1: held radiatory over windowy resting on a wall on the home screen, where no fixtures exist, so not mischief).
Neither appeared in the six final runs.

## 5. Stage traces

The generator gained the `grounds` of the troupe's species, a task card with three marked rows (`row-moss`,
`row-thorns`, `row-rain`) and three scripts:

| Script | Story (replayed with `TK/b5_trace_story.ts`) | Digest |
|---|---|---|
| `idle-learner-prank` (seed 301) | The learner is idle. At 45.00 s thorny picks `row-thorns`, slips over to the card's right side and pushes from 45.02 s. At 55.77 s the copy is gone and the prank is over. Thorny then slides down and lands on the floor (69 s). | 3968230086 |
| `reclaimed-prank` (seed 302) | Mossy pushes `row-moss` from 45.44 s. At 50.02 s `reclaimed`: the copy is gone, mossy tumbles, opens its parachute (50.38 s glide) and lands. The prank ends at 55.09 s with mossy sad 0.39 (sheepish). | 1502840347 |
| `scene-change-prank` (seed 303) | Thorny climbs over the rim and down the left side and pushes `row-rain` from 48.23 s. At 50.02 s `reclaimed` and `summoned` [pebble, misty] arrive together: thrown, and leaves. At 50.14 s the survey has no fixture, and the prank is over at 50.25 s. | 1668566665 |

The 29 earlier digests did not move (12 cross-checked against B4's). A second run of the generator leaves every fixture
unchanged.

## 6. TypeScript changes the Rust twins must mirror

Already mirrored: `🎲️randomness` `MISCHIEF_STREAM` (and its unit test), `🧠️behavior` `AFTER_CLIMB += push` /
`AFTER_PUSH` / `followers_of`, the `🧬️schema` docstrings of `Prank` and `Stage` (`.rs`, `.json`).

To port with the stage (the following stage-port package). Until then the Rust `🎪️stage-trace` adapter cannot replay the
three new scripts:

- `🗓️schedule`: `Post`, `Prospect`, `postOf`, `prospectsOf`, `circumstancesOf`, `prankTick` (all branches: errand, push
  start, hold, put-back, thrown phase with `settled`/`calmsAt`, airborne → −1).
- `🚶️locomotion`: `POST_UNITS`, `holding`, `postTo` (no slip over to a post on its own perch), `popTo` (puff not
  counted, opacity 0, wall: `rest` + `until = now + span`), `pushAt`, `unpush`, `unlifted`.
- `🎯️choice`: `SHEEPISH`, `PUSH_HOLD`, `prank` (two words, round-robin tries, `rested` on failure), `mischief`
  (sheepish, put back at `LIFT_TICKS`, push start, put-back shift, cut errand), `reclaimed`.
- `🕰️clock`: the `lull` horizon from `prankTick`; `mischief` after `pair` and before spawn in `step`; the empty-stage
  jump of `pass` only without a lift.
- `🎥️projection`: `wake` includes `prankTick(…, tick + 1)`.
- `👥️population`: `FIXTURE_SLACK`, `astray`, the survey's quiet end, and `freeze` ending the lift.
- `🎪️stage`: `permitted` without mischief → `unlifted`; `reclaimed` → `choice.reclaimed`.
- The shell order (give-backs before the survey) belongs to the React target and has no Rust twin.

## 7. Decisions and deviations

- **Slip over** (§3) instead of "it goes there with its gear" only. This was needed for the real page.
- The first prank of a session waits a full cooldown (45 s lively, 3 min calm), counted from the stage's opening.
- Sheepishness is sad 0.3, felt only after the fright calms, because the nine shared moods have no sheepish one.
- Reasons the shell gives back for (hidden, scene, still, mischief withdrawn) end as reclaims, because the shell tells
  the stage of the give-back before the reason. A moved fixture ends quietly.
- **Rule deviation:** earlier in this package I used `sed -i` once on my own ticket tool `TK/b5_explore.ts`. No
  repository file was edited that way.
- Docs updated: `P/README.md` (mischief paragraph, module rows), `TK/📓️design-v2.md` §24.3, the layer's `startShow`
  docstring, and the stage-trace `🥒️.feature` prose.

## 8. Open

- Rust port of the list in §6 (stage-port package).
- After a prank, a wall pusher always moves on by sliding or falling from its wall. It never climbs back to a perch,
  because its grip was spent.
- The gear spec of `🐕️pet-walk` stays fixme (B4).
- Seen once and not reproduced: the B4 hand-vs-wall overlap on the home screen (§4).
- `TK/🗑️generated/b5/` keeps the cited logs, probe screenshots and e2e reports (4.8 MB). The stack's scratch data, the
  rehearsal build and Playwright traces were removed. Remove the folder at ticket close. The stack is stopped by its
  PIDs, and ports 6247 and 8947 are free.
