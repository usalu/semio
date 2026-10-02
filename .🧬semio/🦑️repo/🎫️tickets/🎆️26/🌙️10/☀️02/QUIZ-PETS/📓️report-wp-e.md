# 📓️ Report — work package E (behaviour and stage)

Ticket `2026/10/02/QUIZ-PETS`. Paths: `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.

## 1. What exists

| File | What |
|---|---|
| `P/🔨️modules/🧠️behavior/🟦️.ts` | limits of the modes, weights of an idle pet, dwells, moods, the activity graph, encounter shares, bonds and rapport, needs, casting |
| `P/🔨️modules/🎪️stage/🟦️.ts` | `openStage`, `advance`, `frameOf` — the pure fold (about 1 300 lines; the normative order and every draw are documented in the file head) |
| `P/🔨️modules/🧠️behavior/🧪️tests/🔬️unit/🟦️.ts` | 55 vitest cases |
| `P/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts` | 119 vitest cases, each played on the sample menagerie of package A and on the trace troupe |
| `P/🧪️tests/🧠️behavior-choice/{🥒️.feature, 🐍️.py, 🟦️.ts}` | 9 scenarios, oracle `pets-scipy` (numpy `cumsum`/`searchsorted`/`SeedSequence`/`roll`, `scipy.sparse.csgraph`) |
| `P/🧪️tests/🤝️bond-dynamics/{🥒️.feature, 🐍️.py, 🟦️.ts}` | 6 scenarios, oracle `pets-numpy` |
| `P/🧪️tests/🎪️stage-trace/{🥒️.feature, 🐍️.py, 🟦️.ts}` | 3 scenarios, no-oracle decision `pets-stage-trace` (8 scripts, 81 920 ticks) |
| `P/🧫️fixtures/{🧠️behavior-choice, 🤝️bond-dynamics, 🎪️stage-trace}/🔣️.json` | generated vectors; the stage-trace file carries the self-contained menagerie (five blobs) |
| `TK/generate_behavior_vectors.py` | generator of the three fixtures (`--troupe` writes only the menagerie) |
| `TK/record_stage_trace.ts` | records the traces from the TypeScript subject for the generator |
| `TK/stage_storyboard.ts` | the tuning tool: timeline and statistics of a scripted session |

No Rust file was created. No file of a sibling package, of the quiz product or of the schema was edited. The scratch output under `TK/🗑️generated/wp-e/` was removed at the end; the generator recreates what it needs there.

## 2. The behaviour rules as implemented

**State and time.** Actors are kept in `menagerie.species` order. One tick, per actor in that order: fade → motion → gaze → blink → mood → end of activity; then the stage: a waiting pair begins its encounter, and on every whole second a new pair may be drawn. Ticks in which nothing can change are jumped over (see 2.9).

**2.1 Slightly active by default.** Only `idle` decides freely: when its dwell is over, one `randomPick` over `activityWeights` chooses idle again, fidget, walk, hop or sleep. Every other activity ends in idle (or where the activity graph says). Weights (idle = 1):

- `fidget = limits.fidget × (0.5 + 0.5·energy) × (0.5 + curiosity)` while fewer than `limits.fidgeters` others fidget and the species has a fidget clip
- `walk = limits.walk × (0.5 + 0.5·energy) × (0.25 + curiosity)` while the perch has room and fewer than `limits.movers` others walk or hop
- `hop = limits.hop × energy × (0.25 + curiosity)` while a perch is in reach (`hopOf` grants it and `hopLanding` ends on it, the place is free)
- `sleep = limits.sleep × tired²`, `tired = (0.6 − energy) ÷ 0.6` in [0, 1]; never while the pointer is within 1.5 heights
- quiet: fidget, walk, hop = 0, sleep × 3; still: only idle

A walk goes to a goal on the own perch: uniformly drawn, at most `limits.stroll` body widths away, at least ¾ of a width, half a body from the perch ends and clear of the other actors on that surface.

**2.2 Eyes follow the pointer when standing still.** Gaze goal, in screen orientation, looked at from 0.6 of the height: the pointer while it moved within the last 4 s and the actor neither walks nor flies → else its partner → else the nearest glance point → else straight ahead (0.3 towards its facing; down while falling; ahead and down while sulking). `lookOffset(…, reach 96)` per goal, `springStep` per axis with C's `GAZE_STIFFNESS`/`GAZE_DAMPING`; within 1/4096 and slower than 1/64 the pupils snap onto the goal and rest. The frame mirrors the x offset with the actor. A sleeper's pupils rest in the centre at once.

**2.3 Blinking.** A blink lasts `BLINK_TICKS`; when it is over, one draw schedules the next 2…6 s ahead, or (one in six) 7 ticks after the end — a double blink 19 ticks after the first began. Sleepers keep their lids shut and draw nothing.

**2.4 Walking, riding.** `strideTo` at the species' speed (hop gaits play their hop clip in a loop, float gaits glide at hover height). On `surveyed` the perches are cut anew (`clearance` = tallest of everyone on stage or wanted plus hover, `minimum` = 1.5 × widest; also on `summoned`), and every grounded actor moves with its surface (by the change of `x0` — an unchanged survey changes nothing, bit for bit — and to the new `y`), is held inside the nearest perch of that surface, and fades in anew when that carries it farther than its own width. Goals ride along.

**2.5 Falling and landing.** No perch left on its surface: if a perch of another surface lies exactly under its feet (an element replaced by its like) it stays; otherwise `fall` (`fallStep` + swept `landingOf`), then `land` for the landing clip or 0.3 s, then idle. Below the stage box it is put on the nearest perch (fade-in); without any perch it is gone and arrives anew with the first survey that has one. Hops: ballistic by `hopStep`, landing on the first perch crossed on the way down, touching down on the aimed perch on the last tick, falling when that perch is gone. A floating gait glides between perches in a straight line at twice its speed.

**2.6 Encounters.** On every whole second, when the mode has encounters, it is not quiet, nobody has a partner, the warm-up (20 s before the first) or the mode's gap since the last change of a rapport has passed and the mover budget is not used up: all pairs of sociable actors (idle, whole, grounded) within 12 mean widths (and 3 in height) are candidates; one stage draw picks one, weighted `(0.25 + |authored affinity|) × mean sociability`, and lets it happen with the chance `limits.encounterRate × mean sociability`. The pair approaches until the bodies are 2 px apart, each on its own perch: both walk when the mode has room for two movers, else the more sociable one walks and the other waits, facing it. When both wait, one stage draw decides the kind from the current affinity (`encounterOf`), the common span (2…5 s) and a clip each; they face each other and the rapport moves. A squabble ends in a sulk for both (3…6 s each, backs turned); when the second sulk ends the rapport mends by 0.1. A partner that falls, leaves or is frozen releases the other.

`encounterShares(affinity)` → `[greet, cuddle, squabble]`: friends (≥ 0.4) cuddle with `0.5 + 0.4·a` (0.66…0.9), never squabble; rivals (≤ −0.3) squabble with `0.55 − 0.5·a` (0.7…0.85), never cuddle; in between mostly greet, cuddle `0.5·a` for a > 0, squabble `0.08 − 0.5·a` (never below 0).

**2.7 Quiet, still, summon, leave, poke.**

- `hushed`: actors finish what they do (also an approach in progress), then only idle and sleep; no new encounter.
- `tuned still`: leavers are gone, whoever is in the air stands on the nearest perch, everyone idle with rest pose, open eyes, centred gaze; `ticked` jumps; pokes are ignored; summons swap the company at once. Leaving still gives every actor a fresh idle dwell, clip and blink. A mode with fewer movers lets the walkers beyond its limit come to rest.
- `summoned`: the wanted arrive on a perch with room (one stage draw: perch uniformly among perches with room, place, facing; spaced from the others by half of both widths + 8 px when any perch allows), fading in over 16 ticks; without a perch they wait. Whoever is no longer wanted lets go of its partner, walks to the nearer perch end when that is within 3 of its widths, and fades out (16 ticks); a leaver that is wanted again stays.
- `poked`: the nearest grounded actor within 1.2 heights of the tap (from the middle of its body) gains 0.3 mood and greets towards the tap for 2…5 s; an actor busy with its partner only cheers up; a sulker is reconciled first; a sleeper wakes.

**2.8 Needs and bonds.** Needs are settled lazily over the whole span of an activity when it ends (`needsAfter`), rapports fade lazily when one of them changes (`rapportFaded` since `stage.met`); so nothing depends on how time is cut into `ticked` events. Affinity = authored bond + drift, held in [−0.6, 1].

**2.9 Frames, rate, wake.** Pose = the first idle clip of the species looping on the clock of the stage (staggered 37 ticks per stream) + the clip of the activity, weighted in over its first 8 ticks and out over its last 8 (of its span, or of the way to the goal of a walk); gaits, flights, landing and sleep cross-fade the idle loop out, everything else lies on top of it (offsets add, scales multiply). Rate per actor, the stage reports the highest: 64 while fading, walking, flying, landing, playing a clip that does not loop or blending; 32 while a loop, a blink, pupils or a mood move; 16 while it only sleeps; 0 when nothing of it moves (a still stage, an empty one, or a species without any loop between blinks — then `wake` is the earliest of next blink, end of an activity, the pointer losing interest, the next pairing second). `advance` jumps to that tick in one step: an actor at rest has only scheduled changes.

**2.10 Determinism.** One key per draw, `[seed, stream, counter]`: actor stream = species index, counter = `actor.draws` (starts at the tick of its arrival, so a second life does not replay the first); stage stream = 0xffffffff, counter = `stage.draws`. Word order per draw: file head of `🎪️stage/🟦️.ts`. Only `+ − × ÷`, `sqrt`, `abs`, `floor`, `min`, `max`, comparisons and the siblings' functions are used; a unit test greps both modules for the forbidden functions and for `console`.

## 3. Tuned constants

`MODE_LIMITS` (ticks at 64/s):

| | `still` | `calm` | `lively` |
|---|---|---|---|
| movers / fidgeters at once | 0 / 0 | 1 / 1 | 2 / 2 |
| idle dwell | – | 384…1280 (6…20 s) | 192…640 (3…10 s) |
| weight fidget / walk / hop / sleep | 0 | 0.4 / 0.2 / 0.06 / 6 | 1 / 0.6 / 0.3 / 3 |
| longest walk (body widths) | 0 | 4 | 6 |
| encounter gap | never | 5760 (90 s) | 1920 (30 s) |
| encounter chance per second (× mean sociability) | 0 | 0.04 | 0.12 |

Dwells: fidget = its clip (1.5 s when the clip loops); land = its clip or 19 ticks; sleep 20…60 s; greet, cuddle, squabble 2…5 s; sulk 3…6 s; patience with a walk 30 s, with a fall 10 s. Moods: cuddle 1, greet 0.7, fidget and hop 0.5, walk 0.4, idle 0.3, land 0.2, sleep 0.1, fall −0.2, sulk −0.6, squabble −0.8. Rapport: greet +0.05, cuddle +0.1, squabble −0.15, end of sulk +0.1, span ±0.5, fade 0.1 per 38 400 ticks; affinity floor −0.6. Needs per second: energy idle −0.001, fidget −0.01, walk −0.012, hop −0.02, greet −0.006, cuddle −0.004, squabble −0.012, sleep +0.02 (drains × (1.5 − temperament.energy)); sociability +0.002 × (0.5 + temperament) alone, −0.12 in an encounter, −0.02 sulking; curiosity +0.01 × (0.5 + temperament) at rest, walk −0.06, fidget −0.08, hop −0.1. Arrival: energy 0.5 + 0.5 × temperament, sociability and curiosity as authored.

Stage: fade 1/16 per tick; pointer interest 256 ticks; gaze reach 96 px, ahead 0.3; blink gap 128…384 ticks, double 1 in 6; mood ease 1/32 per tick; poke reach 1.2 h, cheer 0.3; wake reach 1.5 h; blend 8 ticks; warm-up 1280 ticks; patience of a waiting partner 1920 ticks; encounter reach 12 widths; meeting gap 2 px; arrival gap 8 px; leave reach 3 widths; least stroll 0.75 width.

What the storyboard measured with these numbers (trace troupe, 5 actors, 10 minutes, seeds 1–3):

| | `calm` | `lively` |
|---|---|---|
| share of time idle / asleep | 84…87 % / 8 % | 68…72 % / 13…14 % |
| fidgets per minute (all actors) | 4.0…4.9 | 11.2…13.5 |
| walks per minute (all actors, approaches included) | 2.0…2.7 | 5.6…6.1 |
| encounters per minute | 0.4…0.6 | 0.8…1.3 |
| somebody walks or hops | 13…21 % of the time, never 2 at once | 28…44 %, never 3 at once |
| frames at rate 64 / 32 | 21…28 % / 72…79 % | 46…56 % / 44…54 % |

Per pet that is: in `calm` a fidget about once a minute, a walk every two to three minutes, an encounter somewhere on stage every two minutes; in `lively` a fidget every 25 s and a walk every 50 s per pet, an encounter every minute.

The architecture menagerie already runs on it (`--menagerie "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts"`, calm, 10 min, capacity 6, seeds 1, 2, 4, 5): idle 83…85 %, sleep 9…11 %, 0.4…0.5 encounters per minute, never two movers. Seed 5: housy and radiatory cuddle at 0:26 and 5:46, pumpy and windowy greet at 3:12, cloudy and solary squabble at 8:19, sulk, and mend from −0.150 to −0.049. Seed 1: radiatory and windowy squabble at 0:50 and greet at 2:29. With `--epoch 90` the nine seeds of the home cast take turns, one swap per epoch.

Cost (10 actors, 31 surfaces, 120 keep-outs, this machine under load): 4…8 µs per tick, 20…32 µs per frame, 35…65 µs per survey, 1…2 µs per pointer event.

## 4. Verification — commands and real results

All run on the final code and the final fixtures.

**Vectors** (repository root): `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_behavior_vectors.py` → the eight scripts recorded (81 920 ticks), the laws of every recorded trace checked by `🧪️tests/🎪️stage-trace/🐍️.py`, three fixtures written; a second run reports all three `unchanged` (the recording is reproducible).

**Unit suites** (from `P/📦️packages/🟦️typescript`):

| Command | Result |
|---|---|
| `bun ./📜️script.ts test` (fundamental, 15 s budget for the whole package) | exit 0 — `Test Files 8 passed (8)`, `Tests 435 passed (435)`, 4.93 s |
| `bun ./📜️script.ts test quick` (the long statistical sessions in full) | exit 0 — `Test Files 8 passed (8)`, `Tests 435 passed (435)`, 6.48 s |
| `bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🎪️stage" "🧠️behavior"` | `Test Files 2 passed (2)`, `Tests 174 passed (174)` (119 stage + 55 behaviour), 2.06 s of tests at fundamental, 4.87 s with `SEMIO_TEST_LEVEL=quick` |

The stage suite reads `SEMIO_TEST_LEVEL`: at the fundamental level the statistical sessions are short (5 instead of 10 minutes, fewer seeds) so that the package stays inside its 15 s budget; an earlier, heavier version of the suite was killed by that budget once while the machine was loaded.

**Protocol v2** (from `TEST`; `--case` needs the emoji name):

| Command | Summary line |
|---|---|
| `bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🧠️behavior-choice"` | `[test] level=exhaustive cases=1 executed=9 passed=9 failed=0 errored=0 parity=0/0` |
| `… parity exhaustive … --case "🧠️behavior-choice"` | `[test] level=exhaustive cases=1 executed=18 passed=18 failed=0 errored=0 parity=9/9` |
| `… oracle exhaustive … --case "🤝️bond-dynamics"` | `[test] level=exhaustive cases=1 executed=6 passed=6 failed=0 errored=0 parity=0/0` |
| `… parity exhaustive … --case "🤝️bond-dynamics"` | `[test] level=exhaustive cases=1 executed=12 passed=12 failed=0 errored=0 parity=6/6` |
| `… oracle exhaustive … --case "🎪️stage-trace"` | `[test] not-exercised … (recorded no-oracle decision pets-stage-trace — its evidence is discharged by the subject phase)` · `executed=0 … not-exercised=1` |
| `… parity exhaustive … --case "🎪️stage-trace"` | `[test] level=exhaustive cases=1 executed=3 passed=3 failed=0 errored=0 parity=0/0` |
| `… parity fundamental … --case "🎪️stage-trace"` | `[test] level=fundamental cases=1 executed=2 passed=2 failed=0 errored=0 parity=0/0` |
| `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1 with `5865 high-priority breach(es) across 4 rule(s)` — all of them in other owners (`temp/…`, `✏️s/…`); **0 lines name the pets product** (`grep -c -e '🐾' -e 'pets'` on the output → 0) |

TypeScript subject only; the Rust adapters are phase 2. `parity=0/0` for stage-trace is expected: one implementation, conformance and property scenarios; the subject itself fails with an `AssertionError` when its trace differs from the committed one.

**Other checks.** `tsc --noEmit --strict --noUncheckedIndexedAccess` over the two modules, both suites, the three adapters and the two ticket tools: no finding in an owned file. Docstrings: every one starts with an emoji, unique per file (checked by script). No `console`, no `[DEBUG]`, none of the forbidden math functions in the modules (also a unit test). `advance` never mutates its input (unit test over deep-frozen stages and events). Cutting time differently never changes the result (unit test and the `determinism` scenario: `ticked(n)` equals n × `ticked(1)`).

## 5. Deviations from the design, and decisions

1. **`affinityOf` is held inside [−0.6, 1]**, not [−1, 1]: §5.5 wants the effective affinity never below −0.6; the floor lives in the one function everybody reads (`AFFINITY_FLOOR`).
2. **Needs and rapports are lazy** (§5.2 step 5 says per tick): settled over the span of an activity at its end, faded since `stage.met` when a rapport changes. `stage.met` is therefore the tick of the last change of any rapport (an encounter beginning, or a sulk mending), and the encounter gap counts from it. Before the first encounter a warm-up of 20 s applies instead of the gap (`met = 0` means "never"), so a learner who stays a minute can see one.
3. **Who meets**: candidates are pairs within 12 mean widths horizontally and 3 vertically (§5.5: same perch, or perches less than 3 widths apart); the pair is picked with a weight that prefers pairs with an authored bond and whoever has not just had company; the kind is decided when both have arrived; in `calm` only one of the two walks (two walkers would break "one mover at once"); partners on different perches come as close as their own perch allows and act across the gap; sulks have individual spans and the mending happens when the second one ends.
4. **Pose** (§4.7 says: clip blended in over 8 ticks from the rest pose): the idle loop keeps running underneath on the stage clock and the activity clip fades in and out, so nothing pops when an activity begins or ends and "a pet that stands still is never frozen" (§5.3) also holds under a greet or a sulk.
5. **Rate 16** while every actor only sleeps (the schema's `Frame.rate` has it; G's pacer handles any rate). **Rate 0 with a wake tick** occurs for species without an idle loop.
6. **Hop gaits** advance uniformly at their speed with the hop clip looping; goals are not snapped to whole hops.
7. **Leaving**: the walk to the perch end happens only within 3 body widths, else the actor fades where it stands (a scene change would otherwise be a long exodus). On a still stage summons take effect at once.
8. **`castOf`**: a core that does not fit into the capacity takes turns itself (rotating by one per epoch from a seeded start) instead of always showing its first members — otherwise the last seeds of the home cast would never appear at capacity 6. A core that fits is unchanged: all of it, then the rotation.
9. **Actor counters** start at the tick of arrival (not 0), so a species that returns does not replay its first life.
10. **Extra exports** beyond §4.6/§4.7 (all unique across the package): `Limits`, `Situation`, `weightedIndex`, `moodOf`, `followersOf`, `ENCOUNTERS`, `Encounter`, `encounterShares`, `AFFINITY_FLOOR`, `RAPPORT_SPAN`, `rapportFaded`, `needsOf`.
11. **Schema**: no field was added; `Actor` and `Stage` as written suffice.
12. **Siblings**: every name of §4 was shipped as designed. Used in addition: `hopStep`, `hopLanding`, `HOP_DISTANCE`, `HOP_HEIGHT`, `HOP_TICKS` (terrain), `GAZE_STIFFNESS`, `GAZE_DAMPING` (animation).
13. **Unit suites carry no JavaScript oracle**: registering one means editing `P/🔮️oracles/🔣️.json` (package A's file); the third-party evidence is in the two Protocol v2 cases.
14. **stage-trace**: the Python file registers its handlers in the oracle role, but a no-oracle case has no oracle phase, so the harness never dispatches it; its checks run inside the generator, which refuses to write a trace that breaks a law. The scenario `determinism` is `@level-quick` (it replays every script three more times); `traces` and `laws` are fundamental.

## 6. Open

- Rust twins (phase 2). Things to mirror exactly: the order in the file head of `🎪️stage/🟦️.ts`; `Infinity` as the initial "least"; removal and insertion by index while iterating; counters wrapping at 2³² (`>>> 0`); the frame order (`y`, then species id by code unit).
- Judged by numbers only so far; the eye has to confirm `calm` and `lively` in the stories gallery. The knobs are in `MODE_LIMITS`; after a change: rerun `generate_behavior_vectors.py` (the Python table in `🧪️tests/🧠️behavior-choice/🐍️.py` restates the limits and must be changed with it).
- Not modelled: turning around is an instant mirror; walkers pass through each other (only goals keep clear of others); hop gaits end a walk mid-hop (the clip fades out); a pair on perches of different height acts across the gap.
- The stage works in species units (px at scale 1); a layer drawing at another scale leaves clearance and spacing conservative.
- `--case` of the harness needs the directory name with its emoji (`--case "🎪️stage-trace"`); the bare slug selects nothing.
