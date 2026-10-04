# 📓️ Work package R0 (second round) — the runtime contract

Ticket `2026/10/02/QUIZ-PETS`, second round, design-v2 §15.3 and §21. `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Tool output went to `TK/🗑️generated/r0/` and was deleted at 13:09; the results are quoted below.

## Contract landed

**State: landed 2026-10-03 ~02:05 (WEST) in all three twins** (`P/🧬️schema/{🟦️.ts, 🔣️.json, 🦀️.rs}`, 111 `$defs`), with the TypeScript and Rust stage folding it neutrally (behaviour unchanged). TypeScript `typecheck` of `P` and `PR` was green on the landed contract (02:05). From ~02:15 the integrators built on it and changed it (table below); everything was cut off at ~03:30 and resumed at 12:19.

**Exact state at 13:07 (after the resume; every number below was run, logs were in `TK/🗑️generated/r0/`):**

- **Schema = R0's contract + the integrators' changes, consistent in all three twins** (117 `$defs`; Rust twin test `every_definition_of_the_normative_file_has_a_twin_under_its_name` green). R0 mirrored B2's two changes into Rust (`Actor.spirits` gone, `Stage.over` added) — schema, façade (`open_stage` `over: Free`, `pointed` stores `over`), population (arrive/freeze), attention (`cheer` and its constants removed), clock (`lull` no longer waits for spirits), projection (frame `mood`/`intensity`/`spirits` from `settled` feeling, `pace_of` restless until `settles_at`) — and extended the Rust feeling twin by exactly what that needs (`settled`, `settles_at`, `spirits_of`, `valence_of`, `shown_mood` and their constants, term for term, with its own unit suite against the feeling-dynamics vectors). B1 mirrors its own fields (`host`, `tilt`, `puffs`, `claims`, `courses`, `origin`) itself.
- **TypeScript `P`** (`📦️packages/🟦️typescript`, `bun ./📜️script.ts typecheck`): **exit 0, 0 errors** (13:00). `test` (13:01, `SEMIO_TEST_BUDGET_MS=300000` — the default 15 s budget was exceeded on the loaded machine): `Test Files 2 failed | 14 passed (16)`, `Tests 14 failed | 1062 passed (1076)` — 10 in B3's new projection suite (tilt about the scruff, climber lean, plumes, pacing with plumes/ladders/lifts) and 4 in the stage suite's mind blocks (state hand-over with emitters, chemistry beat, trick on a whim, circling trick = B2). `test quick` at 12:54: same picture (then `8 failed | 1024 passed`, all B1/B2). **None of R0's tests fails** (`openStage` with every field, footing ⇔ perch, the frame fields, the reclaimed no-op, permissions/inputs/scrolls, walls/fixtures, the mid-turn test).
- **Rust** (`NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/pets-rs:test`, 13:02): compiles; `195 passed; 3 failed` — the 3 read the stage-trace fixture, which still holds the first round's event shapes (`poked`, `pointed` without `over`, `surveyed` without walls/fixtures); B2 has taken over its generator and must re-record it. `cargo clippy -p semio-framework-pets --all-targets --offline -- -D warnings` (13:03): **exit 0**.
- **React `PR`** (13:06): `typecheck` exit 0; `test` `Test Files 8 passed (8)`, `Tests 166 passed (166)`.

| Changed after landing (by) | What | Why / consequence |
|---|---|---|
| B2 | `Actor.spirits` removed; the mouth, the frame's `mood`, `intensity` and `spirits` are read off the feeling (`settled` → `spiritsOf`); `cheer` gone | the first-round scalar is redundant with A7's feeling; **every trace digest moves** (frames fold `spirits`) |
| B2 | `Stage.over` (`"free" \| "control"`, from `Pointed.over`) | the `control` guard of the gesture recognisers |
| B2 | frame `state` = `stateAt(…)`, wake gains `stateEnds` and the chemistry/contagion beat, `paceOf` 64 while `hoverBusy` | B2's horizons (Rust twin still lacks them: phase D / B2) |
| B1 | `Actor.host` (`Slug \| null`, the actor it stands on), `Actor.tilt` (`Turns`); `Stage.puffs` (`Puff[]`), `claims` (`Claim[]` of `Extent`/`Slice`), `courses` (`Course[]` of `Waypoint`), `origin` (`Point \| null`); footing `head` in use | clearance, planned flights, poofs |

Owners: **B1** body (footing, hand, tools, clearance, ladders) · **B2** mind (feeling, state, trick, heat, gestures, chemistry, permissions, played) · **B3** projection (frame fields, particles, lifts) · **B4** gear (rope, wall routes) · **B5** mischief (fixtures, lift).

### Scalars (closed sets)

| Name | Values | Meaning | Owner |
|---|---|---|---|
| `POINTERS` / `Pointer` | `mouse pen touch` | what presses (`Pressed.pointer`) | B2 |
| `PRESS_PHASES` / `PressPhase` | `idle armed holding lifted` | where the stage's press stands (A6) | B2 |
| `TIERS` / `Tier` | `hello trick purr enough` | the answer tier of a warmth (A6) | B2 |
| inline `over` (Rust `Over`, `OVERS`) | `free control` | what lies under the pointer (`Pointed.over`) | B2 |
| inline `reel` (Rust `Reeling`, `REELINGS`) | `zip swing` | how a shot is reeled (`Shot.reel`) | B4 |
| `side` / `facing` `1 \| -1` (Rust `Facing`) | | `Wall.side`, `Shot.facing`, `Ladder.side`, `Prank.side` | B1/B4/B5 |

### Events (`StageEvent`; `poked` is gone)

| Event | Fields | Folded today | Owner |
|---|---|---|---|
| `Pointed` | `x, y, over: "free" \| "control"` | pointer + tick as before; `over` not read yet | B2 |
| `Surveyed` | `+ walls: Wall[], fixtures: Fixture[]` | stored on the stage (`walls`, `fixtures`) | B1/B4 (walls), B5 (fixtures) |
| `Pressed` | `x, y, pointer` | nothing yet | B2 (press machine), B1 (pick-up) |
| `Dragged` | `x, y` | nothing yet | B1/B2 |
| `Released` | `x, y` | nothing yet | B1/B2 |
| `Cancelled` | — | nothing yet | B1/B2 |
| `Reclaimed` | `fixture` | nothing yet | B5 |
| `Stirred` | — | `stage.stirred = tick` | B2/B5 |
| `Scrolled` | — | `stage.scrolled = tick` (A6's `scrolled` guard: `tick − scrolled < SCROLL_TICKS`) | B2 |
| `Played` | `species, deed: Deed` | nothing yet | B2 |
| `Permitted` | `play, mischief` | `stage.play`, `stage.mischief` | B2 (play), B5 (mischief) |

`Wall = { id, surface, side, x, y0, y1 }` (a side edge of a surface's element; `-1` left edge); `Fixture = { id, key, x, y, width, height }` (key = a ground of the menagerie).

### Runtime state — `Actor`

| Field | Type | Meaning / neutral value at arrival | Owner |
|---|---|---|---|
| `footing` | `Footing` | what carries it; **kept consistent with `perch`**: `perch` exactly while `perch !== null`, `air` otherwise (arrive/touch → `perch`; hop launch, drop, crowd-out → `air`). Every remaining `perch === null` read still tests `perch` — B1 switches them to `footing`. | B1 |
| ~~`spirits`~~ | — | landed as the renamed first-round `Actor.mood` scalar; **removed by B2** (the spirits are read off `feeling`) | B2 |
| `feeling` | `Feeling {mood, intensity, since}` | A7's anchor; arrival `atRest(species.mood, tick)` | B2 |
| `state`, `stateSince` | `Slug`, `Ticks` | the species state and the tick it was entered; arrival `species.states[0].id`, arrival tick | B2 |
| `trick` | `Slug \| null` | the trick being performed; `null` | B2 |
| `warmth` | `Warmth {heat, since, until, tier, run, tricks}` | A6's heat bucket; `COLD` | B2 |
| `hover` | `Hover {circling: Circling, stroking: Stroking}` | A6's free-pointer recognisers; `noHover(0)` | B2 |
| `hang` | `Hang {grip: Grip, bob, previous} \| null` | A3's held pendulum; `null` | B1 |
| `chute` | `Parachute {since, open, opening, canopy: Canopy} \| null` | the parachute while out (`open` 0…1 may overshoot, `opening` its rate per second for a spring, `canopy` A3's dynamics); `null` | B1 |
| `rope` | `Rope {shot: Shot, since, length, hand, before} \| null` | the grappling rope while out (A4's `Shot` and the reel state of A4's `Haul` without the feet: rope left, hands now / a tick ago); `null` | B4 |
| `emitters` | `Plume[] = {emitter, since, until: Ticks \| null}[]` | emitters it runs (A8: `until` = the tick the state/trick/purr ended, `null` while it lasts); `[]` | B3 (B2 starts/stops them) |

### Runtime state — `Stage`

| Field | Type | Meaning / neutral value at `openStage` | Owner |
|---|---|---|---|
| `walls`, `fixtures` | `Wall[]`, `Fixture[]` | as surveyed; `[]` | B1/B4, B5 |
| `play`, `mischief` | boolean | what the learner permits; **`false` until the shell sends `permitted`** (safe by default; the layer's C1 must send it at mount) | B2, B5 |
| `stirred` | `Ticks` | the tick of the learner's last input; `0` | B2/B5 |
| `scrolled` | `Ticks` | the tick of the last scroll; `0` | B2 |
| `press` | `Press {phase, x, y, since, slop}` | A6's press machine; `IDLE` | B2 |
| `touched` | `Slug \| null` | the actor the open press belongs to; `null` | B2/B1 |
| `shaking` | `Shaking` | A6's shake recogniser of the held pet; `noShaking(0)` | B2 |
| `trail` | `Point[]` | the last pointer samples while a pet is held (A3: seven, newest last); `[]` | B1 |
| `coolings` | `Cooling {reaction, when, near, until}[]` | A7's chemistry cooldowns; `[]` | B2 |
| `pledges` | `Pledge {between, encounter, until}[]` | encounters the chemistry promised (A7 `LEAN_TICKS`); `[]` | B2 |
| `ladders` | `Ladder {owner, wall, surface, side, foot, top, since, until, rider}[]` | ladders that stand (A4's private `Ladder` fields + owner/lifetime/rider; `foot`, never `base` — banned stem); `[]` | B1/B4 (A4 may refine) |
| `lift` | `Prank {fixture, pusher, since, side, room, span, unit} \| null` | the one lifted fixture: everything `liftAt(since, tick, side, room, span, unit)` needs, and the pusher to throw off on `reclaimed`; `null` | B5 |
| `rested` | `Ticks` | the tick the last lift ended (A8 `allowedFrom`); `0` | B5 |
| `poofs` | integer ≥ 0 | pets that vanished in a puff (design §18); `0` | B1 |

### Frame

| Field | Type | Neutral value today | Owner |
|---|---|---|---|
| `ActorFrame.footing`, `.state` | `Footing`, `Slug` | the actor's | B1, B2 |
| `ActorFrame.mood`, `.intensity` | `Mood`, 0…1 | landed as the sign of `moodOf(activity)` and `|spirits|`; now (B2) the settled feeling's mood and intensity | B2/B3 |
| `ActorFrame.spirits` | −1…1 | the first round's `ActorFrame.mood` number, renamed — the React mouth reads it; now (B2) `spiritsOf(settled feeling)` | B2/B3 |
| `Stage.over` | `"free" \| "control"` | added by B2: what the pointer is over (from `Pointed.over`); `free` at `openStage` | B2 |
| `ActorFrame.tilt`, `.pivot` | `Turns`, `Point` | `0`, `{x: 0, y: −species.grip}` (A9: rig coordinates, x mirrored) | B1/B3 |
| `ActorFrame.tools` | `ToolFrame[]` | `[]` | B3 (B1/B4 feed) |
| `ActorFrame.body` | `Rect` | the size box at the feet `{x − w/2, y − h, w, h}` | B1 (clearance body) / B3 |
| `Frame.ladders` | `LadderFrame[]` | `[]` | B3/B4 |
| `Frame.particles` | `ParticleFrame[]` | `[]` | B3 |
| `Frame.lifts` | `LiftFrame[]` | `[]` | B3/B5 |
| `Frame.held` | `Slug \| null` | `null` | B1/B3 |

`ToolFrame = ChuteTool {kind:"chute", open, sway} | RopeTool {kind:"rope", x, y, slack} | HookTool {kind:"hook", x, y} | GunTool {kind:"gun", aim} | LadderTool {kind:"ladder", lean, length}`, `LadderFrame {x0, y0, x1, y1, rungs, opacity}`, `ParticleFrame {species, emitter, x, y, scale, rotation, opacity}`, `LiftFrame {fixture, dx, dy, tilt, opacity}` — exactly §21, so A9's `GearTool`, `GearLadder`, `EffectParticle` stay assignable (no change to A9's modules).

### Where the types came from (pure modules now import them from the schema)

`👆️gesture`: `Pointer`, `PressPhase`, `Press`, `TIERS`/`Tier`, `Warmth`, `Circling`, `Stroking`, `Shaking`, `Hover` · `🪢️swing`: `Grip`, `Hang`, `Canopy` · `💗️feeling`: `Feeling`, `Cooling` · `🪄️mischief`: its private stand-in `Fixture` replaced by the schema's. Step results and inputs stay in their modules (`PressInput`, `PressSignal`, `Guards`, `Chute`, `Reel`, `Standing`, `Sighting`, `Lift`, `Station` …). **A4 (`🧗️climbing`, still running)**: its private `Ladder` and `Shot` and `🏞️terrain`'s private `Wall` now exist as schema types with the same fields — import them from the schema instead of stating them (they are private, so nothing collides today).

### Rust specifics

Closed sets are enums (`Pointer`, `PressPhase`, `Tier`, inline `Over`, `Reeling`; sides are `Facing`). Ticks are `i64`; the recogniser counters and signs (`sx sy turn quarters steps against way count run tricks`) are `i64`, every other number `f64`; `LadderFrame.rungs` `u32`, `Stage.poofs` `u32`. `Pledge.encounter` is an `Activity` that only decodes greet/cuddle/squabble (`meeting`, which `Effect.encounter` now shares). Wire names are camelCase (`stateSince`, `topSince`, `bottomSince`). The neutral values the Rust stage needs live in **partial Rust twins** `P/🔨️modules/👆️gesture/🦀️.rs` (`IDLE`, `COLD`, `no_circling`, `no_stroking`, `no_shaking`, `no_hover`) and `P/🔨️modules/💗️feeling/🦀️.rs` (`MOOD_HOLD`, `MOOD_FAINT`, `MOOD_REST`, `MOOD_RISE`, `MOOD_DECAYS`, `PRONE_DECAY`, `MOOD_VALENCES`; `at_rest`, `shown_mood`, `settled`, `settles_at`, `valence_of`, `spirits_of`; unit suite `💗️feeling/🧪️tests/🔬️unit/🦀️.rs`: every decay and jump vector bit for bit, every face vector within 1e-9 like the TypeScript suite, since the oracle interpolates with `numpy.interp`), mounted `pub mod gesture; pub mod feeling;` and re-exported by the façade — phase D completes them.

## How it landed

1. **Batch A (schema, ~01:40–02:05):** the three twins of `P/🧬️schema` in one go, prepared in `TK/🗑️generated/r0/` and written back to back; in the same batch `👆️gesture`, `🪢️swing`, `💗️feeling` and `🪄️mischief` dropped their own declarations and import the schema's (otherwise the façade's `export *` collides), their suites and adapters followed.
2. **Batch B (stage + targets, ~02:05):** both stages (`openStage`/`open_stage`, `apply`, `draftOf`/`sealed`, arrive/survey, footing assignments, frame fields, `poke` removed with its constants), the Rust partial twins, the React survey/layer/depiction/stories, every suite and the TK long-run tools.
3. **After the resume (12:19–13:07):** mirrored B2's `spirits`/`over` changes into Rust (see above), adapted R0's own tests to what B1/B2 had added meanwhile (the `openStage` expectation, footing `head`, B2's press handling — the hand no-op test now uses `reclaimed`), regenerated the schema catalog.

## Decisions

- **`play` and `mischief` start `false`**: nothing the learner did not permit happens; the layer (C1) sends `permitted` at mount.
- **`Plume.until: Ticks | null`** instead of a lifetime: emitters end when the state/trick/purr ends, which is not known when they start.
- **`Parachute.opening`** next to `open`: the canopy opens on a spring that may overshoot, so its rate is state.
- **`Prank`** for the lifted fixture (`Stage.lift`): `Lift` is `🪄️mischief`'s step result; **`foot`** of a ladder, never `base` (banned stem).
- **Partial Rust twins** of gesture and feeling instead of literal neutral values in the stage: the neutral values have exactly one home in each language.
- **`hail` stays exported in TypeScript** (private in Rust): the TS stage suite greets mid-turn through a draft (`draftOf` → `hail` → `sealed`) now that `poke`, its only public trigger, is gone; the Rust test does the same through `greeted()`.
- **Frame neutral values** (`mood` = sign of `moodOf(activity)`, `intensity` = `|spirits|`) were superseded by B2 within the hour; R0 adopted B2's reading in Rust instead of keeping its own.

## Verification beyond typecheck and unit tests (12:45–13:05)

| Check | Result | Owner of any red |
|---|---|---|
| parity `schema-conformance` | `executed=9 passed=9 parity=9/9` | — |
| parity `bond-dynamics` | `executed=18 passed=18 parity=18/18` | — |
| parity `behavior-choice` | exit 1: the Rust adapter imports `pets::mood_of`, the TS adapter `moodOf` — both removed by B2 with `cheer` | B2 (adapters) |
| parity `stage-trace` | `executed=6 passed=2 failed=4 parity=0/3` — the fixture still holds first-round events | B2 (re-record) |
| long run (`TK/long_trace_digest.ts` → `stage_scratch.ts prepare` → `rust_scratch.sh wp-m … --example long_trace_digest`) | TS: 48 sessions, 1279 events, 2880 checkpoints; Rust against TS: **2798 of 2880 mismatch**, first `troupe-seed-1-still` second 1; probe: first difference at **tick 0, `$.stage.actors[4].x`** (TS 878.0040082183866, Rust 877.9899151201269) — TS seats arrivals through B1's `🚧️clearance` (`seatOf`, `freeAt`), which has no Rust twin yet | B1 / phase D |
| quiz `pet-companions` | 33 passed | — |
| site `pet-cast` | 80 passed | — |
| schema catalog (`schema generate`, `schema docs`, repo root) | regenerated; check `findings=9345`, no stale catalog, no pets findings | — |

## Digests

Every stage-trace and long-run digest moves, and an R0-only digest proof on the live tree is impossible: the frame digest now folds every §21 frame field (`spirits`, footing, state, mood, intensity, tilt, pivot, tools, body, ladders, particles, lifts, held), B2 derives `spirits` from the feeling, and B1/B2 changed TypeScript behaviour (seating, press machine, states, chemistry). By construction R0's own step moves only the scripts that sent `poked`: `pokes-and-glances` (the poke became pressed + released, which R0 folded as no-ops; B2's press machine gives them a meaning since); the poke of `still-and-back` hit a still stage and was a no-op already. The fixture is re-recorded by B2 with the new scripts once B1's trace laws hold (B2's last run: "moving-card: the recorded trace breaks the law perched").

## Open items for the integrators

- **B2:** re-record `P/🧫️fixtures/🎪️stage-trace/🔣️.json` (`TK/generate_behavior_vectors.py`, `TK/record_stage_trace.ts --rerecord`) — 3 Rust tests and stage-trace parity stay red until then; fix the behavior-choice adapters (`moodOf`/`mood_of`); mirror the press machine, states, horizons and chemistry into Rust (or leave them to phase D).
- **B1:** Rust twin of `🚧️clearance` (the long run diverges at arrival seating); switch the remaining `perch === null` reads to `footing`.
- **B3:** `🎥️projection/🟦️.ts` has the docstring emojis 🎒️ and 💨️ twice each (seen 12:51, still at 13:08; in-flight edits); 10 failing tests in the new projection suite.
- **A4:** import `Ladder`, `Shot` and `Wall` from the schema instead of the private copies in `🧗️climbing` and `🏞️terrain`.
- **C1:** send `permitted` at mount (play and mischief are `false` until then) and real walls/fixtures in `surveyed` (empty today).

## Files (R0)

- Schema: `P/🧬️schema/{🟦️.ts, 🔣️.json, 🦀️.rs}`, `P/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`; the repo schema catalog (regenerated).
- Pure modules: `P/🔨️modules/{👆️gesture, 🪢️swing, 💗️feeling, 🪄️mischief}/🟦️.ts`; new `P/🔨️modules/👆️gesture/🦀️.rs`, `P/🔨️modules/💗️feeling/🦀️.rs`, `P/🔨️modules/💗️feeling/🧪️tests/🔬️unit/🦀️.rs`; unit suites of gesture, feeling and swing; adapters `P/🧪️tests/{💗️feeling-dynamics, ⚗️chemistry-rules, 🪢️swing-dynamics, 🪂️parachute-descent}`.
- Glue: `P/🦀️.rs`, `P/📦️packages/🦀️rust/🦀️.rs`.
- Stage (TypeScript and Rust): `P/🔨️modules/{🎪️stage, 📝️draft, 👥️population, 🚶️locomotion, 🎯️choice, 💞️sociability, 🎥️projection, 👀️attention, 🕰️clock}`; suites `🎪️stage`, `👀️attention`, `👥️population`, `🕰️clock`, `🧠️behavior`, `💞️sociability` (`🧪️tests/🔬️unit`); `P/🧪️tests/🎪️stage-trace/{🟦️.ts, 🦀️.rs}`.
- React: `PR/🔨️modules/{📡️survey, 🫧️layer, 🖌️depiction}`, `PR/📖️stories/🟦️.tsx`; adapters `P/🧪️tests/{🖌️pet-depiction, 🧰️gear-depiction, 📡️surface-survey}`.
- Ticket tools: `TK/long_trace_digest.{ts, rs}`, `TK/stage_scratch.ts`; this report.
